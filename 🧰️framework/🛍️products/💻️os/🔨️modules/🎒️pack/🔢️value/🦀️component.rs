//! 📦️ `pack_value` — DSL schema-aware wire encoding/decoding of `crate::os_dsl::schema::RecordValue`
//! documents into the `pack_format` binary container. Implements every wire tag (0x00-0x16),
//! canonical-mode determinism (sorted field ids, omitted `Absent`, sorted map keys, minimal
//! varints, `f64` normalization, deterministic string interning, mandatory packed numeric
//! forms), `TableSoA` columnar encoding for `Shape::Table`, unknown-field preservation via
//! `DecodeReport`, `schema_hash`, and the top-level `encode_document`/`decode_document` entry
//! points every other `pack_*`/`vcs`/`dsl_derive` crate calls through.
//!
//! See the `## pack_value` section of the wave-0 contract at
//! `.🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md` for the binding
//! byte layout this module implements against.

use crate::os_dsl::schema::{DslValue, FieldSpec, FieldValue, RecordSpec, RecordValue, Shape, WireEdgeLabel, WireNode, WireValue};
use crate::os_pack::{write_varint_i64, write_varint_u64, ByteReader, ChunkId, CodecId, PackError, PackLimits};
use std::collections::{HashMap, HashSet};

//#region 🔖️Tags
/// @emoji 🕳️ `FieldValue::Absent` — never written at record-field granularity (canonical mode
/// omits it entirely) but valid as a decode target, e.g. inside a sparse `TableSoA` fallback
/// column or a hand-crafted non-canonical file.
const TAG_ABSENT: u8 = 0x00;
const TAG_FALSE: u8 = 0x01;
const TAG_TRUE: u8 = 0x02;
const TAG_INT: u8 = 0x03;
const TAG_UINT: u8 = 0x04;
const TAG_F64: u8 = 0x05;
const TAG_STR: u8 = 0x06;
const TAG_STR_INLINE: u8 = 0x07;
const TAG_BYTES: u8 = 0x08;
const TAG_BYTES_CHUNKED: u8 = 0x09;
const TAG_ENUM: u8 = 0x0A;
const TAG_TUPLE: u8 = 0x0B;
const TAG_LIST: u8 = 0x0C;
const TAG_RECORD: u8 = 0x0D;
const TAG_BLOCK: u8 = 0x0E;
const TAG_STATEMENTS: u8 = 0x0F;
const TAG_MAP: u8 = 0x10;
const TAG_VALUE: u8 = 0x11;
const TAG_NULL: u8 = 0x12;
const TAG_WIRE: u8 = 0x13;
const TAG_TABLE_SOA: u8 = 0x14;
const TAG_PACKED_F64: u8 = 0x15;
const TAG_PACKED_VARINT: u8 = 0x16;
const TAG_EXPR: u8 = 0x17;
//#endregion 🔖️Tags

//#region 🔖️Canonical
/// @emoji ✂️ Extracts a `List`/`Tuple` field's element `Shape`, if `shape` is one of those two
/// variants — the seam shared by encode and decode so both walk exactly the same element type.
fn elem_shape_of(shape: Option<&Shape>) -> Option<&Shape> {
    match shape {
        Some(Shape::Tuple(elem, _)) | Some(Shape::List(elem)) => Some(elem),
        _ => None,
    }
}

/// @emoji 📊️ Extracts a `Table` field's lazy element-spec constructor, if `shape` is `Table`.
fn table_spec_of(shape: Option<&Shape>) -> Option<fn() -> RecordSpec> {
    match shape {
        Some(Shape::Table(spec_fn)) => Some(*spec_fn),
        _ => None,
    }
}

/// @emoji 🧾️ Resolves a `Record` field's nested spec, if `shape` is `Record`.
fn record_spec_of(shape: Option<&Shape>) -> Option<RecordSpec> {
    match shape {
        Some(Shape::Record(spec_fn)) => Some(spec_fn()),
        _ => None,
    }
}

fn block_inner_shape(shape: Option<&Shape>) -> Option<&Shape> {
    match shape {
        Some(Shape::Block(inner)) => Some(inner),
        _ => None,
    }
}

fn statements_variants(shape: Option<&Shape>) -> Option<&Vec<(String, fn() -> RecordSpec)>> {
    match shape {
        Some(Shape::Statements(variants)) => Some(variants),
        _ => None,
    }
}

fn map_inner_shape(shape: Option<&Shape>) -> Option<&Shape> {
    match shape {
        Some(Shape::Map(inner)) => Some(inner),
        _ => None,
    }
}

/// @emoji 🚧️ Every shape whose `FieldValue` representation is `Tuple` rather than `List` — the
/// packed-numeric-array fast path (`TAG_PACKED_F64`/`TAG_PACKED_VARINT`) collapses both to the
/// same bytes on the wire (a run of numbers has no other distinguishing feature), so `shape` is
/// the ONLY signal decode has left to reconstruct the right `FieldValue` variant. Every shape here
/// is a fixed-arity number tuple by construction (`Coord`/`Dir`/`Dim`/`Range`'s own parsers in
/// `dsl_schema` never produce anything else), so this can never rebuild the wrong shape.
fn is_tuple_shape(shape: Option<&Shape>) -> bool {
    matches!(shape, Some(Shape::Tuple(_, _)) | Some(Shape::Coord(_)) | Some(Shape::Dir) | Some(Shape::Dim(_)) | Some(Shape::Range))
}

/// @emoji 🛡️ Depth-limit check shared by every recursive encode/decode entry point.
fn check_depth(max_depth: u16, depth: u16) -> Result<(), PackError> {
    if depth > max_depth {
        return Err(PackError::LimitExceeded("max_depth exceeded"));
    }
    Ok(())
}

/// @emoji 🔢️ Canonical `f64` normalization preserves signed zero and maps any `NaN` to the
/// single quiet-NaN bit pattern `0x7ff8_0000_0000_0000`.
fn normalize_f64(value: f64) -> f64 {
    if value.is_nan() {
        f64::from_bits(0x7ff8_0000_0000_0000)
    } else {
        value
    }
}

/// @emoji 🔢️ Which packed form a homogeneous numeric sequence is eligible for.
enum NumKind {
    F64,
    Varint,
}

/// @emoji 🧮️ A sequence is packed-eligible iff every element is the same numeric `FieldValue`
/// variant (`Float`, `Int`, `Enum`) or every element is `UInt` and fits in `i64` (so the zigzag
/// round trip through `PackedVarint` is lossless). Empty sequences are never eligible — there is
/// no element to infer a kind from, so they fall through to the plain `0x0B`/`0x0C` forms.
fn homogeneous_numeric_kind(items: &[FieldValue]) -> Option<NumKind> {
    if items.is_empty() {
        return None;
    }
    if items.iter().all(|v| matches!(v, FieldValue::Float(_))) {
        return Some(NumKind::F64);
    }
    if items.iter().all(|v| matches!(v, FieldValue::Int(_))) {
        return Some(NumKind::Varint);
    }
    if items.iter().all(|v| matches!(v, FieldValue::Enum(_))) {
        return Some(NumKind::Varint);
    }
    if items.iter().all(|v| matches!(v, FieldValue::UInt(u) if *u <= i64::MAX as u64)) {
        return Some(NumKind::Varint);
    }
    None
}

/// @emoji ✒️ Mutable state threaded through one `encode_document` call: the precomputed symbol
/// table (built by [`build_symbols`] in a deterministic pre-pass), the live segment/chunk writer,
/// and the caller's options.
struct EncCtx<'a> {
    symbol_index: HashMap<String, u64>,
    writer: &'a mut crate::os_pack::format::PackWriter<Vec<u8>>,
    options: &'a EncodeOptions,
}

/// @emoji 📖️ Encodes a string using the precomputed interning decision: `TAG_STR` + symref if
/// `s` made it into the symbol table, else `TAG_STR_INLINE` + length-prefixed UTF-8 bytes.
fn encode_string(ctx: &mut EncCtx<'_>, s: &str, out: &mut Vec<u8>) {
    if let Some(&idx) = ctx.symbol_index.get(s) {
        out.push(TAG_STR);
        write_varint_u64(out, idx);
    } else {
        encode_string_inline(s, out);
    }
}

/// @emoji 📌️ Forces `TAG_STR_INLINE` regardless of the interning decision — the wire rule for
/// `Value`/`DslValue::Object` keys, which are never symrefs.
fn encode_string_inline(s: &str, out: &mut Vec<u8>) {
    out.push(TAG_STR_INLINE);
    write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

/// @emoji 🔗️ Writes a bare symref varint with NO leading tag — the wire rule for `Statements`
/// keywords and `TableSoA` `Str` columns, both of which are unconditionally interned.
fn write_symref_forced(ctx: &mut EncCtx<'_>, s: &str, out: &mut Vec<u8>) -> Result<(), PackError> {
    let idx = *ctx.symbol_index.get(s).ok_or_else(|| PackError::Schema(format!("symbol {s:?} missing from precomputed table")))?;
    write_varint_u64(out, idx);
    Ok(())
}

/// @emoji 🔎️ Deterministic string-interning pre-pass: walks the whole document once (shape-aware
/// where a shape is known, generically otherwise) counting string occurrences and marking forced
/// interns (`Statements` keywords, `TableSoA` `Text` columns), then returns the sorted symbol
/// table — a `len <= 128 || count >= 2` string, or any forced one, is interned; everything else
/// stays inline. Sorting (rather than first-occurrence order) is what keeps this a pure function
/// of `(spec, record)` regardless of `HashMap` iteration order.
fn build_symbols(spec: &RecordSpec, record: &RecordValue) -> Vec<String> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut forced: HashSet<String> = HashSet::new();
    walk_record_for_symbols(&mut counts, &mut forced, Some(spec), record);
    let mut set: HashSet<String> = forced;
    for (s, count) in &counts {
        if s.len() <= 128 || *count >= 2 {
            set.insert(s.clone());
        }
    }
    let mut symbols: Vec<String> = set.into_iter().collect();
    symbols.sort();
    symbols
}

fn note_symbol(counts: &mut HashMap<String, u64>, s: &str) {
    *counts.entry(s.to_string()).or_insert(0) += 1;
}

fn force_symbol(counts: &mut HashMap<String, u64>, forced: &mut HashSet<String>, s: &str) {
    forced.insert(s.to_string());
    note_symbol(counts, s);
}

fn walk_record_for_symbols(counts: &mut HashMap<String, u64>, forced: &mut HashSet<String>, spec: Option<&RecordSpec>, record: &RecordValue) {
    let mut ids: Vec<u16> = record.fields.keys().copied().collect();
    ids.sort_unstable();
    for id in ids {
        let value = record.fields.get(&id).expect("id came from this map's own keys");
        if matches!(value, FieldValue::Absent) {
            continue;
        }
        let shape = spec.and_then(|s| s.fields.iter().find(|f| f.id == id)).map(|f| &f.shape);
        walk_value_for_symbols(counts, forced, shape, value);
    }
}

fn walk_value_for_symbols(counts: &mut HashMap<String, u64>, forced: &mut HashSet<String>, shape: Option<&Shape>, value: &FieldValue) {
    match value {
        FieldValue::Text(s) => note_symbol(counts, s),
        FieldValue::Tuple(items) => {
            let elem = elem_shape_of(shape);
            for it in items {
                walk_value_for_symbols(counts, forced, elem, it);
            }
        }
        FieldValue::List(items) => {
            if let Some(spec_fn) = table_spec_of(shape) {
                // Walk each row's `TableSoA` columns, forcing `Text`-typed columns (they're
                // always symrefs on the wire) and recursing generically into every other column
                // shape for nested strings.
                let element_spec = spec_fn();
                for row in items {
                    let FieldValue::Record(r) = row else { continue };
                    for field in &element_spec.fields {
                        let Some(v) = r.fields.get(&field.id) else { continue };
                        if matches!(v, FieldValue::Absent) {
                            continue;
                        }
                        if matches!(field.shape, Shape::Text) {
                            if let FieldValue::Text(s) = v {
                                force_symbol(counts, forced, s);
                            }
                        } else {
                            walk_value_for_symbols(counts, forced, Some(&field.shape), v);
                        }
                    }
                }
            } else {
                let elem = elem_shape_of(shape);
                for it in items {
                    walk_value_for_symbols(counts, forced, elem, it);
                }
            }
        }
        FieldValue::Record(r) => {
            let spec = record_spec_of(shape);
            walk_record_for_symbols(counts, forced, spec.as_ref(), r);
        }
        FieldValue::Block(inner) => walk_value_for_symbols(counts, forced, block_inner_shape(shape), inner),
        FieldValue::Statements(items) => {
            let variants = statements_variants(shape);
            for (keyword, record) in items {
                force_symbol(counts, forced, keyword);
                let spec = variants.and_then(|vs| vs.iter().find(|(k, _)| k == keyword)).map(|(_, f)| f());
                walk_record_for_symbols(counts, forced, spec.as_ref(), record);
            }
        }
        FieldValue::Map(entries) => {
            let inner = map_inner_shape(shape);
            for (k, v) in entries {
                note_symbol(counts, k);
                walk_value_for_symbols(counts, forced, inner, v);
            }
        }
        FieldValue::Value(v) => walk_dsl_value_for_symbols(counts, v),
        FieldValue::Wire(w) => {
            note_symbol(counts, &w.from.id);
            if let Some(k) = &w.from.kind {
                note_symbol(counts, k);
            }
            if let Some(p) = &w.from.port {
                note_symbol(counts, p);
            }
            if let Some((_, to)) = &w.edge {
                note_symbol(counts, &to.id);
                if let Some(k) = &to.kind {
                    note_symbol(counts, k);
                }
                if let Some(p) = &to.port {
                    note_symbol(counts, p);
                }
            }
            walk_dsl_value_for_symbols(counts, &w.properties);
        }
        _ => {}
    }
}

/// @emoji 🌱️ `DslValue::Object` keys are always inline (never interned) per the wire contract, so
/// only `String` leaves and array/object values are walked here.
fn walk_dsl_value_for_symbols(counts: &mut HashMap<String, u64>, v: &DslValue) {
    match v {
        DslValue::String(s) => note_symbol(counts, s),
        DslValue::Array(items) => {
            for it in items {
                walk_dsl_value_for_symbols(counts, it);
            }
        }
        DslValue::Object(entries) => {
            for (_, v) in entries {
                walk_dsl_value_for_symbols(counts, v);
            }
        }
        _ => {}
    }
}
//#endregion 🔖️Canonical

//#region 🔖️Encode
/// @emoji 🧾️ Encodes one record's fields as `field_count varint, (field_id varint, value)*` —
/// the shared body used both for the top-level document and for nested `FieldValue::Record`
/// (behind its own `0x0D` tag). Always sorts by field id (the purity LAW: byte-identical output
/// regardless of `HashMap` iteration order) and always omits `Absent`. `spec` is `None` for a
/// genuinely schema-less context (an unrecognized `Statements` variant, a shape/value mismatch);
/// fields are then encoded generically. `options.preserve_unknown == false` drops fields whose id
/// isn't found in `spec` instead of encoding them.
fn encode_record_fields(ctx: &mut EncCtx<'_>, spec: Option<&RecordSpec>, record: &RecordValue, depth: u16) -> Result<Vec<u8>, PackError> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    let preserve_unknown = ctx.options.preserve_unknown;
    let mut ids: Vec<u16> = record.fields.iter().filter(|(_, v)| !matches!(v, FieldValue::Absent)).filter(|(id, _)| preserve_unknown || spec.is_some_and(|s| s.fields.iter().any(|f| f.id == **id))).map(|(id, _)| *id).collect();
    ids.sort_unstable();
    let mut buf = Vec::new();
    write_varint_u64(&mut buf, ids.len() as u64);
    for id in ids {
        let value = record.fields.get(&id).expect("id came from this map's own keys");
        write_varint_u64(&mut buf, id as u64);
        let field_shape = spec.and_then(|s| s.fields.iter().find(|f| f.id == id)).map(|f| &f.shape);
        encode_value(ctx, field_shape, value, depth + 1, &mut buf)?;
    }
    Ok(buf)
}

/// @emoji ✍️ Encodes one field value, tag-prefixed and self-describing. `shape` is the field's
/// declared `Shape` when known (disambiguates `Tuple` vs `List`, selects `TableSoA` for
/// `Shape::Table`, and resolves nested `Record`/`Block`/`Statements`/`Map` sub-shapes); `None`
/// encodes the value generically from its runtime `FieldValue` variant alone — the path used for
/// field ids absent from the caller's `RecordSpec`, which is what makes unknown-field
/// preservation possible without ever having seen their original schema.
fn encode_value(ctx: &mut EncCtx<'_>, shape: Option<&Shape>, value: &FieldValue, depth: u16, out: &mut Vec<u8>) -> Result<(), PackError> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    match value {
        FieldValue::Absent => out.push(TAG_ABSENT),
        FieldValue::Bool(b) => out.push(if *b { TAG_TRUE } else { TAG_FALSE }),
        FieldValue::Int(i) => {
            out.push(TAG_INT);
            write_varint_i64(out, *i);
        }
        FieldValue::UInt(u) => {
            out.push(TAG_UINT);
            write_varint_u64(out, *u);
        }
        FieldValue::Float(f) => {
            out.push(TAG_F64);
            out.extend_from_slice(&normalize_f64(*f).to_le_bytes());
        }
        FieldValue::Text(s) => encode_string(ctx, s, out),
        FieldValue::Bytes64(bytes) => encode_bytes(ctx, bytes, out)?,
        FieldValue::Enum(ordinal) => {
            out.push(TAG_ENUM);
            write_varint_u64(out, *ordinal as u64);
        }
        FieldValue::Tuple(items) => encode_seq(ctx, items, elem_shape_of(shape), true, depth, out)?,
        FieldValue::List(items) => {
            if let Some(spec_fn) = table_spec_of(shape) {
                encode_table(ctx, spec_fn, items, depth, out)?;
            } else {
                encode_seq(ctx, items, elem_shape_of(shape), false, depth, out)?;
            }
        }
        FieldValue::Record(record) => {
            let nested_spec = record_spec_of(shape);
            out.push(TAG_RECORD);
            let fields = encode_record_fields(ctx, nested_spec.as_ref(), record, depth + 1)?;
            out.extend_from_slice(&fields);
        }
        FieldValue::Block(inner) => {
            out.push(TAG_BLOCK);
            encode_value(ctx, block_inner_shape(shape), inner, depth + 1, out)?;
        }
        FieldValue::Statements(items) => encode_statements(ctx, statements_variants(shape), items, depth, out)?,
        FieldValue::Map(entries) => encode_map(ctx, entries, map_inner_shape(shape), depth, out)?,
        FieldValue::Value(v) => {
            out.push(TAG_VALUE);
            encode_dsl_value(ctx, v, depth + 1, out)?;
        }
        FieldValue::Wire(w) => {
            out.push(TAG_WIRE);
            encode_wire(ctx, w, depth + 1, out)?;
        }
        // Canonical `print_expr` text under the string codec — deterministic (the printer is
        // canonical), so `decode = parse_expr_text ∘ decode_string` inverts it exactly, and
        // pack ≡ dsl holds by construction rather than needing a bespoke binary AST encoding.
        FieldValue::Expr(expr) => {
            out.push(TAG_EXPR);
            encode_string(ctx, &crate::os_dsl::schema::print_expr(expr), out);
        }
    }
    Ok(())
}

/// @emoji 🧱️ Encodes a `Bytes64` payload direct (`TAG_BYTES`) or, once it reaches
/// `options.chunk_threshold`, split into `options.chunk_size`-sized chunks written through the
/// live `PackWriter` (`TAG_BYTES_CHUNKED` + the resulting `ChunkId`s).
fn encode_bytes(ctx: &mut EncCtx<'_>, bytes: &[u8], out: &mut Vec<u8>) -> Result<(), PackError> {
    if (bytes.len() as u64) >= ctx.options.chunk_threshold {
        let chunk_size = ctx.options.chunk_size.max(1) as usize;
        let mut ids = Vec::new();
        for piece in bytes.chunks(chunk_size) {
            ids.push(ctx.writer.write_chunk(piece)?);
        }
        out.push(TAG_BYTES_CHUNKED);
        write_varint_u64(out, ids.len() as u64);
        for id in ids {
            write_varint_u64(out, id.0 as u64);
        }
    } else {
        out.push(TAG_BYTES);
        write_varint_u64(out, bytes.len() as u64);
        out.extend_from_slice(bytes);
    }
    Ok(())
}

/// @emoji 📚️ Encodes a `Tuple`/`List` sequence: the mandatory packed `0x15`/`0x16` form when
/// every element is the same numeric kind, else the plain self-describing `0x0B`/`0x0C` form.
fn encode_seq(ctx: &mut EncCtx<'_>, items: &[FieldValue], elem_shape: Option<&Shape>, is_tuple: bool, depth: u16, out: &mut Vec<u8>) -> Result<(), PackError> {
    if let Some(kind) = homogeneous_numeric_kind(items) {
        match kind {
            NumKind::F64 => {
                out.push(TAG_PACKED_F64);
                write_varint_u64(out, items.len() as u64);
                for it in items {
                    if let FieldValue::Float(f) = it {
                        out.extend_from_slice(&normalize_f64(*f).to_le_bytes());
                    }
                }
            }
            NumKind::Varint => {
                out.push(TAG_PACKED_VARINT);
                write_varint_u64(out, items.len() as u64);
                for it in items {
                    let v: i64 = match it {
                        FieldValue::Int(i) => *i,
                        FieldValue::UInt(u) => *u as i64,
                        FieldValue::Enum(o) => *o as i64,
                        _ => 0,
                    };
                    write_varint_i64(out, v);
                }
            }
        }
        return Ok(());
    }
    out.push(if is_tuple { TAG_TUPLE } else { TAG_LIST });
    write_varint_u64(out, items.len() as u64);
    for it in items {
        encode_value(ctx, elem_shape, it, depth + 1, out)?;
    }
    Ok(())
}

/// @emoji 🗺️ Encodes `Map`/object entries sorted by key bytes (canonical, always — not just when
/// `options.canonical`, per the purity LAW), each key using the conditional interning rule.
fn encode_map(ctx: &mut EncCtx<'_>, entries: &[(String, FieldValue)], inner_shape: Option<&Shape>, depth: u16, out: &mut Vec<u8>) -> Result<(), PackError> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    out.push(TAG_MAP);
    let mut sorted: Vec<&(String, FieldValue)> = entries.iter().filter(|(_, v)| !matches!(v, FieldValue::Absent)).collect();
    sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    write_varint_u64(out, sorted.len() as u64);
    for (k, v) in sorted {
        encode_string(ctx, k, out);
        encode_value(ctx, inner_shape, v, depth + 1, out)?;
    }
    Ok(())
}

/// @emoji 📜️ Encodes `Statements`: `count, (keyword symref, Record-payload)*`. The keyword is
/// always a bare forced symref (never a self-describing string tag) per the wire contract.
fn encode_statements(ctx: &mut EncCtx<'_>, variants: Option<&Vec<(String, fn() -> RecordSpec)>>, items: &[(String, RecordValue)], depth: u16, out: &mut Vec<u8>) -> Result<(), PackError> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    out.push(TAG_STATEMENTS);
    write_varint_u64(out, items.len() as u64);
    for (keyword, record) in items {
        write_symref_forced(ctx, keyword, out)?;
        let spec = variants.and_then(|vs| vs.iter().find(|(k, _)| k == keyword)).map(|(_, f)| f());
        let fields = encode_record_fields(ctx, spec.as_ref(), record, depth + 1)?;
        out.extend_from_slice(&fields);
    }
    Ok(())
}

/// @emoji 🌱️ Encodes a `DslValue` using the same self-describing tag set recursively; object
/// entries sorted by key bytes with keys FORCED inline (`encode_string_inline`, never a symref) —
/// the one deliberate carve-out from the general conditional-interning rule.
fn encode_dsl_value(ctx: &mut EncCtx<'_>, v: &DslValue, depth: u16, out: &mut Vec<u8>) -> Result<(), PackError> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    match v {
        DslValue::Null => out.push(TAG_NULL),
        DslValue::Bool(b) => out.push(if *b { TAG_TRUE } else { TAG_FALSE }),
        DslValue::Number(n) => {
            out.push(TAG_F64);
            out.extend_from_slice(&normalize_f64(*n).to_le_bytes());
        }
        DslValue::String(s) => encode_string(ctx, s, out),
        DslValue::Array(items) => {
            out.push(TAG_LIST);
            write_varint_u64(out, items.len() as u64);
            for it in items {
                encode_dsl_value(ctx, it, depth + 1, out)?;
            }
        }
        DslValue::Object(entries) => {
            out.push(TAG_MAP);
            let mut sorted: Vec<&(String, DslValue)> = entries.iter().collect();
            sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
            write_varint_u64(out, sorted.len() as u64);
            for (k, val) in sorted {
                encode_string_inline(k, out);
                encode_dsl_value(ctx, val, depth + 1, out)?;
            }
        }
    }
    Ok(())
}

/// @emoji 🕸️ Encodes a `Wire` literal. Wire sub-format (presence bitmask + node layout) is this
/// crate's own choice — the contract pins only the outer `0x13` tag and the constituent parts
/// (`from`, optional `to`, `props`); everything here just needs to round-trip, which it does.
fn encode_wire(ctx: &mut EncCtx<'_>, w: &WireValue, depth: u16, out: &mut Vec<u8>) -> Result<(), PackError> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    let has_label = !w.edge_label.is_empty();
    let mut presence = 0u8;
    if w.edge.is_some() {
        presence |= 0b01;
    }
    if let Some((directed, _)) = &w.edge {
        if *directed {
            presence |= 0b10;
        }
    }
    if has_label {
        presence |= 0b100;
    }
    out.push(presence);
    encode_wire_node(ctx, &w.from, out);
    if let Some((_, to)) = &w.edge {
        encode_wire_node(ctx, to, out);
    }
    if has_label {
        let mut lp = 0u8;
        if w.edge_label.id.is_some() {
            lp |= 0b01;
        }
        if w.edge_label.kind.is_some() {
            lp |= 0b10;
        }
        out.push(lp);
        if let Some(id) = &w.edge_label.id {
            encode_string(ctx, id, out);
        }
        if let Some(kind) = &w.edge_label.kind {
            encode_string(ctx, kind, out);
        }
    }
    encode_dsl_value(ctx, &w.properties, depth + 1, out)?;
    Ok(())
}

fn encode_wire_node(ctx: &mut EncCtx<'_>, node: &WireNode, out: &mut Vec<u8>) {
    let mut presence = 0u8;
    if node.kind.is_some() {
        presence |= 0b01;
    }
    if node.port.is_some() {
        presence |= 0b10;
    }
    out.push(presence);
    encode_string(ctx, &node.id, out);
    if let Some(k) = &node.kind {
        encode_string(ctx, k, out);
    }
    if let Some(p) = &node.port {
        encode_string(ctx, p, out);
    }
}
//#endregion 🔖️Encode

//#region 🔖️Decode
/// @emoji 🧭️ Where symrefs and chunk ids resolve during one decode: a full opened `PackFile`
/// (the `decode_document` path) or a container-less inline symbol table (the
/// `decode_record_body` path, which has no chunk table by construction).
enum DecSource<'a> {
    File(&'a crate::os_pack::format::PackFile<&'a [u8]>),
    Inline { symbols: Vec<String> },
}

/// @emoji 📖️ Mutable state threaded through one `decode_document`/`decode_record_body` call: the
/// symref/chunk resolution source, the caller's limits/verification/preserve-unknown choices, and
/// the accumulated unknown-field-id report.
struct DecCtx<'a> {
    source: DecSource<'a>,
    limits: PackLimits,
    verification: crate::os_pack::format::VerificationLevel,
    preserve_unknown: bool,
    unknown_field_ids: Vec<u16>,
}

impl DecCtx<'_> {
    fn check_items(&self, n: u64) -> Result<(), PackError> {
        if n > self.limits.max_items {
            return Err(PackError::LimitExceeded("item count exceeds max_items"));
        }
        Ok(())
    }
}

fn resolve_symref(ctx: &DecCtx<'_>, symref: u64) -> Result<String, PackError> {
    match &ctx.source {
        DecSource::File(pack_file) => pack_file.symbol(symref).map(str::to_string),
        DecSource::Inline { symbols } => symbols.get(symref as usize).cloned().ok_or_else(|| PackError::Malformed { what: "symref", offset: 0, detail: format!("symref {symref} out of range for inline table of {}", symbols.len()) }),
    }
}

/// @emoji 📏️ Reads a `varint` length then that many raw bytes, rejecting an oversized length
/// against `limits.max_segment_len` BEFORE allocating/slicing.
fn read_len_prefixed_bytes<'b>(reader: &mut ByteReader<'b>, limits: &PackLimits) -> Result<&'b [u8], PackError> {
    let len = reader.read_varint_u64()?;
    if len > limits.max_segment_len {
        return Err(PackError::LimitExceeded("inline blob length exceeds max_segment_len"));
    }
    reader.read_bytes(len as usize)
}

fn read_inline_string(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<String, PackError> {
    let bytes = read_len_prefixed_bytes(reader, &ctx.limits)?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|_| PackError::Malformed { what: "text", offset: reader.position() as u64, detail: "invalid utf8".to_string() })
}

fn read_inline_bytes(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<Vec<u8>, PackError> {
    Ok(read_len_prefixed_bytes(reader, &ctx.limits)?.to_vec())
}

/// @emoji 🧱️ Reads `count` chunk ids and concatenates their decoded (and, per `verification`,
/// integrity-checked) content via the open `PackFile`'s chunk table.
fn read_chunked_bytes(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<Vec<u8>, PackError> {
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut out = Vec::new();
    for _ in 0..count {
        let id = reader.read_varint_u64()?;
        if id > u32::MAX as u64 {
            return Err(PackError::Malformed { what: "chunk_id", offset: reader.position() as u64, detail: "chunk id exceeds u32".to_string() });
        }
        let piece = match &ctx.source {
            DecSource::File(pack_file) => pack_file.read_chunk(ChunkId(id as u32), ctx.verification)?,
            DecSource::Inline { .. } => {
                return Err(PackError::Malformed { what: "chunk_id", offset: reader.position() as u64, detail: "chunked bytes are not representable in a container-less record body".to_string() });
            }
        };
        out.extend_from_slice(&piece);
    }
    Ok(out)
}

/// @emoji 📖️ Reads one self-describing string value (`TAG_STR` or `TAG_STR_INLINE`) — used for
/// `Map`/object keys and `DslValue::String`, where the tag itself (not any external shape) is
/// what disambiguates interned vs inline.
fn decode_string(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<String, PackError> {
    let tag = reader.read_u8()?;
    match tag {
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            resolve_symref(ctx, idx)
        }
        TAG_STR_INLINE => read_inline_string(reader, ctx),
        other => Err(PackError::Malformed { what: "string", offset: reader.position() as u64, detail: format!("expected a string tag, found {other:#04x}") }),
    }
}

/// @emoji 🧾️ Decodes one record's fields: `field_count, (field_id, value)*`. Any field id not
/// found in `spec` is decoded generically (`shape = None`) and reported into
/// `ctx.unknown_field_ids`; when `ctx.preserve_unknown` is `false` it is still consumed (to stay
/// byte-aligned) but dropped from the returned `RecordValue`. Every `spec` field not seen on the
/// wire is inserted as `Absent` — the decode-side half of canonical mode's "omit `Absent`" rule.
fn decode_record_fields(reader: &mut ByteReader<'_>, spec: Option<&RecordSpec>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<RecordValue, PackError> {
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut record = RecordValue::default();
    for _ in 0..count {
        let id_raw = reader.read_varint_u64()?;
        if id_raw > u16::MAX as u64 {
            return Err(PackError::Malformed { what: "field_id", offset: reader.position() as u64, detail: "field id exceeds u16".to_string() });
        }
        let id = id_raw as u16;
        let field_shape = spec.and_then(|s| s.fields.iter().find(|f| f.id == id)).map(|f| &f.shape);
        let value = decode_value(reader, field_shape, ctx, depth + 1)?;
        if field_shape.is_none() {
            ctx.unknown_field_ids.push(id);
            if ctx.preserve_unknown {
                record.fields.insert(id, value);
            }
        } else {
            record.fields.insert(id, value);
        }
    }
    if let Some(spec) = spec {
        for field in &spec.fields {
            record.fields.entry(field.id).or_insert(FieldValue::Absent);
        }
    }
    Ok(record)
}

/// @emoji 📖️ Decodes one tag-prefixed value. `shape`, when known, disambiguates `Tuple` vs
/// `List`, resolves nested `Record`/`Block`/`Statements`/`Map` sub-shapes, and reinterprets
/// `PackedVarint` payloads as `UInt`/`Enum` where the shape says so; `None` decodes generically
/// straight from the wire tag — every tag is self-describing enough for this to always succeed,
/// which is what makes unknown-field decode possible without the original schema.
fn decode_value(reader: &mut ByteReader<'_>, shape: Option<&Shape>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackError> {
    check_depth(ctx.limits.max_depth, depth)?;
    let tag = reader.read_u8()?;
    match tag {
        TAG_ABSENT => Ok(FieldValue::Absent),
        TAG_FALSE => Ok(FieldValue::Bool(false)),
        TAG_TRUE => Ok(FieldValue::Bool(true)),
        TAG_INT => Ok(FieldValue::Int(reader.read_varint_i64()?)),
        TAG_UINT => Ok(FieldValue::UInt(reader.read_varint_u64()?)),
        TAG_F64 => Ok(FieldValue::Float(reader.read_f64_le()?)),
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            Ok(FieldValue::Text(resolve_symref(ctx, idx)?))
        }
        TAG_STR_INLINE => Ok(FieldValue::Text(read_inline_string(reader, ctx)?)),
        TAG_BYTES => Ok(FieldValue::Bytes64(read_inline_bytes(reader, ctx)?)),
        TAG_BYTES_CHUNKED => Ok(FieldValue::Bytes64(read_chunked_bytes(reader, ctx)?)),
        TAG_ENUM => Ok(FieldValue::Enum(reader.read_varint_u64()? as u32)),
        TAG_TUPLE => decode_seq_body(reader, elem_shape_of(shape), true, ctx, depth),
        TAG_LIST => {
            if let Some(spec_fn) = table_spec_of(shape) {
                // Defensive: a verbose AoS list under a `Table` field. `Shape::Record(spec_fn)`
                // built inline lets us reuse `decode_seq_body` unchanged.
                let record_shape = Shape::Record(spec_fn);
                decode_seq_body(reader, Some(&record_shape), false, ctx, depth)
            } else {
                decode_seq_body(reader, elem_shape_of(shape), false, ctx, depth)
            }
        }
        TAG_RECORD => {
            let nested_spec = record_spec_of(shape);
            Ok(FieldValue::Record(decode_record_fields(reader, nested_spec.as_ref(), ctx, depth + 1)?))
        }
        TAG_BLOCK => Ok(FieldValue::Block(Box::new(decode_value(reader, block_inner_shape(shape), ctx, depth + 1)?))),
        TAG_STATEMENTS => decode_statements(reader, statements_variants(shape), ctx, depth),
        TAG_MAP => decode_map(reader, map_inner_shape(shape), ctx, depth),
        TAG_VALUE => Ok(FieldValue::Value(decode_dsl_value(reader, ctx, depth + 1)?)),
        TAG_WIRE => Ok(FieldValue::Wire(decode_wire(reader, ctx, depth + 1)?)),
        TAG_EXPR => {
            let text = decode_string(reader, ctx)?;
            crate::os_dsl::schema::parse_expr_text(&text).map(FieldValue::Expr).map_err(|e| PackError::Malformed { what: "expr", offset: reader.position() as u64, detail: e.message })
        }
        TAG_TABLE_SOA => Ok(FieldValue::List(decode_table_soa(reader, table_spec_of(shape), ctx, depth)?)),
        TAG_PACKED_F64 => decode_packed_f64_body(reader, is_tuple_shape(shape)),
        TAG_PACKED_VARINT => decode_packed_varint_body(reader, elem_shape_of(shape).or(shape.filter(|s| !matches!(s, Shape::Tuple(_, _)))), is_tuple_shape(shape)),
        TAG_NULL => Err(PackError::Malformed { what: "wire_tag", offset: reader.position() as u64, detail: "TAG_NULL is only valid inside a DslValue".to_string() }),
        other => Err(PackError::Malformed { what: "wire_tag", offset: reader.position() as u64, detail: format!("unrecognized tag {other:#04x}") }),
    }
}

/// @emoji 📚️ Decodes a plain (non-packed) `Tuple`/`List` body: `count, values*`.
fn decode_seq_body(reader: &mut ByteReader<'_>, elem_shape: Option<&Shape>, is_tuple: bool, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackError> {
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut items = Vec::with_capacity(count.min(4096) as usize);
    for _ in 0..count {
        items.push(decode_value(reader, elem_shape, ctx, depth + 1)?);
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

fn decode_packed_f64_body(reader: &mut ByteReader<'_>, is_tuple: bool) -> Result<FieldValue, PackError> {
    let count = reader.read_varint_u64()?;
    let mut items = Vec::with_capacity(count.min(4096) as usize);
    for _ in 0..count {
        items.push(FieldValue::Float(reader.read_f64_le()?));
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

/// @emoji 🔢️ Decodes a `PackedVarint` body. `elem_shape` (the field's `List(UInt)`/`List(Enum)`/
/// `Tuple(..)` element shape, when known) picks the reconstruction type; unknown context always
/// defaults to `Int`, which is also what makes an unknown field's homogeneous-`Int` list
/// re-encode to the exact same bytes (round-trip preserved even without the original schema).
fn decode_packed_varint_body(reader: &mut ByteReader<'_>, elem_shape: Option<&Shape>, is_tuple: bool) -> Result<FieldValue, PackError> {
    let count = reader.read_varint_u64()?;
    let mut items = Vec::with_capacity(count.min(4096) as usize);
    for _ in 0..count {
        let v = reader.read_varint_i64()?;
        let fv = match elem_shape {
            Some(Shape::UInt) => {
                if v < 0 {
                    return Err(PackError::Malformed { what: "packed_varint", offset: reader.position() as u64, detail: "negative value under UInt shape".to_string() });
                }
                FieldValue::UInt(v as u64)
            }
            Some(Shape::Enum(_)) => {
                if v < 0 {
                    return Err(PackError::Malformed { what: "packed_varint", offset: reader.position() as u64, detail: "negative value under Enum shape".to_string() });
                }
                FieldValue::Enum(v as u32)
            }
            _ => FieldValue::Int(v),
        };
        items.push(fv);
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

fn decode_map(reader: &mut ByteReader<'_>, inner_shape: Option<&Shape>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackError> {
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut entries = Vec::with_capacity(count.min(4096) as usize);
    for _ in 0..count {
        let key = decode_string(reader, ctx)?;
        let value = decode_value(reader, inner_shape, ctx, depth + 1)?;
        entries.push((key, value));
    }
    Ok(FieldValue::Map(entries))
}

fn decode_statements(reader: &mut ByteReader<'_>, variants: Option<&Vec<(String, fn() -> RecordSpec)>>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackError> {
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut items = Vec::with_capacity(count.min(4096) as usize);
    for _ in 0..count {
        let symref = reader.read_varint_u64()?;
        let keyword = resolve_symref(ctx, symref)?;
        let spec = variants.and_then(|vs| vs.iter().find(|(k, _)| *k == keyword)).map(|(_, f)| f());
        let record = decode_record_fields(reader, spec.as_ref(), ctx, depth + 1)?;
        items.push((keyword, record));
    }
    Ok(FieldValue::Statements(items))
}

fn decode_dsl_value(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<DslValue, PackError> {
    check_depth(ctx.limits.max_depth, depth)?;
    let tag = reader.read_u8()?;
    match tag {
        TAG_NULL => Ok(DslValue::Null),
        TAG_FALSE => Ok(DslValue::Bool(false)),
        TAG_TRUE => Ok(DslValue::Bool(true)),
        TAG_F64 => Ok(DslValue::Number(reader.read_f64_le()?)),
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            Ok(DslValue::String(resolve_symref(ctx, idx)?))
        }
        TAG_STR_INLINE => Ok(DslValue::String(read_inline_string(reader, ctx)?)),
        TAG_LIST => {
            let count = reader.read_varint_u64()?;
            ctx.check_items(count)?;
            let mut items = Vec::with_capacity(count.min(4096) as usize);
            for _ in 0..count {
                items.push(decode_dsl_value(reader, ctx, depth + 1)?);
            }
            Ok(DslValue::Array(items))
        }
        TAG_MAP => {
            let count = reader.read_varint_u64()?;
            ctx.check_items(count)?;
            let mut entries = Vec::with_capacity(count.min(4096) as usize);
            for _ in 0..count {
                let key = decode_string(reader, ctx)?;
                let value = decode_dsl_value(reader, ctx, depth + 1)?;
                entries.push((key, value));
            }
            Ok(DslValue::Object(entries))
        }
        other => Err(PackError::Malformed { what: "dsl_value", offset: reader.position() as u64, detail: format!("unexpected tag {other:#04x}") }),
    }
}

fn decode_wire(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<WireValue, PackError> {
    let presence = reader.read_u8()?;
    let from = decode_wire_node(reader, ctx)?;
    let edge = if presence & 0b01 != 0 {
        let directed = presence & 0b10 != 0;
        let to = decode_wire_node(reader, ctx)?;
        Some((directed, to))
    } else {
        None
    };
    let edge_label = if presence & 0b100 != 0 {
        let lp = reader.read_u8()?;
        let id = if lp & 0b01 != 0 { Some(decode_string(reader, ctx)?) } else { None };
        let kind = if lp & 0b10 != 0 { Some(decode_string(reader, ctx)?) } else { None };
        WireEdgeLabel { id, kind }
    } else {
        WireEdgeLabel::default()
    };
    let properties = decode_dsl_value(reader, ctx, depth + 1)?;
    Ok(WireValue { from, edge, edge_label, properties })
}

fn decode_wire_node(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>) -> Result<WireNode, PackError> {
    let presence = reader.read_u8()?;
    let id = decode_string(reader, ctx)?;
    let kind = if presence & 0b01 != 0 { Some(decode_string(reader, ctx)?) } else { None };
    let port = if presence & 0b10 != 0 { Some(decode_string(reader, ctx)?) } else { None };
    Ok(WireNode { id, kind, port })
}
//#endregion 🔖️Decode

//#region 🔖️Table
/// @emoji 🏷️ `TableSoA` per-column element-type tags — local to this crate's columnar encoding,
/// distinct from (and not overlapping the meaning of) the top-level wire tag space.
const ELEM_FALLBACK: u8 = 0;
const ELEM_BOOL: u8 = 1;
const ELEM_INT: u8 = 2;
const ELEM_UINT: u8 = 3;
const ELEM_F64: u8 = 4;
const ELEM_STR: u8 = 5;
const ELEM_ENUM: u8 = 6;

fn elem_tag_for_shape(shape: &Shape) -> u8 {
    match shape {
        Shape::Float | Shape::Quantity(_) | Shape::Angle(_) => ELEM_F64,
        Shape::Int => ELEM_INT,
        Shape::UInt | Shape::Count => ELEM_UINT,
        Shape::Enum(_) => ELEM_ENUM,
        Shape::Bool => ELEM_BOOL,
        Shape::Text | Shape::Ref(_) => ELEM_STR,
        _ => ELEM_FALLBACK,
    }
}

/// @emoji 📊️ Encodes `Shape::Table`'s `List(Record)` value as columnar `TableSoA`: `row_count,
/// col_count`, then per column (sorted by field id) `field_id, presence (0=dense/1=sparse+bitmap),
/// elem_tag, packed payload`. Fixed-width/varint columns write only present-row values
/// (compacted, in row order); `Bool` columns instead write one ceil(rows/8)-byte row-aligned value
/// bitmap unconditionally (simpler than compacting individual bits). `Text` columns are always
/// interned (forced symrefs, matching `build_symbols`'s pre-pass); every other shape falls back to
/// self-describing per-present-row values.
fn encode_table(ctx: &mut EncCtx<'_>, spec_fn: fn() -> RecordSpec, items: &[FieldValue], depth: u16, out: &mut Vec<u8>) -> Result<(), PackError> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    let element_spec = spec_fn();
    let mut columns: Vec<&FieldSpec> = element_spec.fields.iter().collect();
    columns.sort_by_key(|f| f.id);
    let row_count = items.len();
    out.push(TAG_TABLE_SOA);
    write_varint_u64(out, row_count as u64);
    write_varint_u64(out, columns.len() as u64);
    for field in &columns {
        let present: Vec<bool> = items.iter().map(|row| matches!(row, FieldValue::Record(r) if r.fields.get(&field.id).is_some_and(|v| !matches!(v, FieldValue::Absent)))).collect();
        let dense = present.iter().all(|p| *p);
        write_varint_u64(out, field.id as u64);
        out.push(if dense { 0 } else { 1 });
        if !dense {
            let mut bitmap = vec![0u8; row_count.div_ceil(8)];
            for (i, p) in present.iter().enumerate() {
                if *p {
                    bitmap[i / 8] |= 1 << (i % 8);
                }
            }
            out.extend_from_slice(&bitmap);
        }
        let elem_tag = elem_tag_for_shape(&field.shape);
        out.push(elem_tag);
        match elem_tag {
            ELEM_F64 => {
                for (row, p) in items.iter().zip(&present) {
                    if !*p {
                        continue;
                    }
                    if let FieldValue::Record(r) = row {
                        if let Some(FieldValue::Float(f)) = r.fields.get(&field.id) {
                            out.extend_from_slice(&normalize_f64(*f).to_le_bytes());
                        }
                    }
                }
            }
            ELEM_INT => {
                for (row, p) in items.iter().zip(&present) {
                    if !*p {
                        continue;
                    }
                    if let FieldValue::Record(r) = row {
                        if let Some(FieldValue::Int(v)) = r.fields.get(&field.id) {
                            write_varint_i64(out, *v);
                        }
                    }
                }
            }
            ELEM_UINT => {
                for (row, p) in items.iter().zip(&present) {
                    if !*p {
                        continue;
                    }
                    if let FieldValue::Record(r) = row {
                        if let Some(FieldValue::UInt(v)) = r.fields.get(&field.id) {
                            write_varint_u64(out, *v);
                        }
                    }
                }
            }
            ELEM_ENUM => {
                for (row, p) in items.iter().zip(&present) {
                    if !*p {
                        continue;
                    }
                    if let FieldValue::Record(r) = row {
                        if let Some(FieldValue::Enum(v)) = r.fields.get(&field.id) {
                            write_varint_u64(out, *v as u64);
                        }
                    }
                }
            }
            ELEM_BOOL => {
                let mut valbits = vec![0u8; row_count.div_ceil(8)];
                for (i, row) in items.iter().enumerate() {
                    if let FieldValue::Record(r) = row {
                        if let Some(FieldValue::Bool(b)) = r.fields.get(&field.id) {
                            if *b {
                                valbits[i / 8] |= 1 << (i % 8);
                            }
                        }
                    }
                }
                out.extend_from_slice(&valbits);
            }
            ELEM_STR => {
                for (row, p) in items.iter().zip(&present) {
                    if !*p {
                        continue;
                    }
                    if let FieldValue::Record(r) = row {
                        if let Some(FieldValue::Text(s)) = r.fields.get(&field.id) {
                            write_symref_forced(ctx, s, out)?;
                        }
                    }
                }
            }
            _ => {
                for (row, p) in items.iter().zip(&present) {
                    if !*p {
                        continue;
                    }
                    if let FieldValue::Record(r) = row {
                        if let Some(v) = r.fields.get(&field.id) {
                            encode_value(ctx, Some(&field.shape), v, depth + 1, out)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// @emoji 📖️ Decodes `TableSoA` fully self-describing — `field_id`/`presence`/`elem_tag` are
/// stored per column on the wire, so no `RecordSpec` is ever required to reconstruct the rows
/// (this is what lets an unknown `Table`-shaped field still round-trip). When the caller DOES
/// know the table's element `RecordSpec` (`spec_fn` is `Some`), it is threaded into the
/// fallback (non-primitive) column branch so a nested `Record` column's own `Absent` sub-fields
/// get backfilled correctly instead of merely reflecting what was present on the wire.
fn decode_table_soa(reader: &mut ByteReader<'_>, spec_fn: Option<fn() -> RecordSpec>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<Vec<FieldValue>, PackError> {
    check_depth(ctx.limits.max_depth, depth)?;
    let element_spec = spec_fn.map(|f| f());
    let row_count_raw = reader.read_varint_u64()?;
    ctx.check_items(row_count_raw)?;
    let col_count = reader.read_varint_u64()?;
    ctx.check_items(col_count)?;
    let row_count = row_count_raw as usize;
    let mut rows: Vec<RecordValue> = (0..row_count).map(|_| RecordValue::default()).collect();
    for _ in 0..col_count {
        let field_id = reader.read_varint_u64()? as u16;
        let presence = reader.read_u8()?;
        let dense = presence == 0;
        let present: Vec<bool> = if dense {
            vec![true; row_count]
        } else {
            let bitmap = reader.read_bytes(row_count.div_ceil(8))?.to_vec();
            (0..row_count).map(|i| bitmap[i / 8] & (1 << (i % 8)) != 0).collect()
        };
        let elem_tag = reader.read_u8()?;
        match elem_tag {
            ELEM_F64 => {
                for (i, p) in present.iter().enumerate() {
                    if *p {
                        let f = reader.read_f64_le()?;
                        rows[i].fields.insert(field_id, FieldValue::Float(f));
                    }
                }
            }
            ELEM_INT => {
                for (i, p) in present.iter().enumerate() {
                    if *p {
                        let v = reader.read_varint_i64()?;
                        rows[i].fields.insert(field_id, FieldValue::Int(v));
                    }
                }
            }
            ELEM_UINT => {
                for (i, p) in present.iter().enumerate() {
                    if *p {
                        let v = reader.read_varint_u64()?;
                        rows[i].fields.insert(field_id, FieldValue::UInt(v));
                    }
                }
            }
            ELEM_ENUM => {
                for (i, p) in present.iter().enumerate() {
                    if *p {
                        let v = reader.read_varint_u64()?;
                        rows[i].fields.insert(field_id, FieldValue::Enum(v as u32));
                    }
                }
            }
            ELEM_BOOL => {
                let bitmap = reader.read_bytes(row_count.div_ceil(8))?.to_vec();
                for (i, p) in present.iter().enumerate() {
                    if *p {
                        let b = bitmap[i / 8] & (1 << (i % 8)) != 0;
                        rows[i].fields.insert(field_id, FieldValue::Bool(b));
                    }
                }
            }
            ELEM_STR => {
                for (i, p) in present.iter().enumerate() {
                    if *p {
                        let symref = reader.read_varint_u64()?;
                        let s = resolve_symref(ctx, symref)?;
                        rows[i].fields.insert(field_id, FieldValue::Text(s));
                    }
                }
            }
            _ => {
                let field_shape = element_spec.as_ref().and_then(|s| s.fields.iter().find(|f| f.id == field_id)).map(|f| &f.shape);
                for (i, p) in present.iter().enumerate() {
                    if *p {
                        let v = decode_value(reader, field_shape, ctx, depth + 1)?;
                        rows[i].fields.insert(field_id, v);
                    }
                }
            }
        }
        for (i, p) in present.iter().enumerate() {
            if !*p {
                rows[i].fields.entry(field_id).or_insert(FieldValue::Absent);
            }
        }
    }
    Ok(rows.into_iter().map(FieldValue::Record).collect())
}
//#endregion 🔖️Table

//#region 🔖️SchemaHash
/// @emoji 🏷️ A fixed numeric tag per `Shape` variant, used only by [`schema_hash`]'s canonical
/// serialization — an internal id, not a wire tag.
fn shape_tag(shape: &Shape) -> u8 {
    match shape {
        Shape::Bool => 1,
        Shape::Int => 2,
        Shape::UInt => 3,
        Shape::Float => 4,
        Shape::Text => 5,
        Shape::Bytes64 => 6,
        Shape::Enum(_) => 7,
        Shape::Tuple(_, _) => 8,
        Shape::List(_) => 9,
        Shape::Record(_) => 10,
        Shape::Block(_) => 11,
        Shape::Statements(_) => 12,
        Shape::Map(_) => 13,
        Shape::Value => 14,
        Shape::Table(_) => 15,
        Shape::Wire => 16,
        Shape::Quantity(_) => 17,
        Shape::Angle(_) => 18,
        Shape::Ref(_) => 19,
        Shape::Coord(_) => 20,
        Shape::Dir => 21,
        Shape::Dim(_) => 22,
        Shape::Range => 23,
        Shape::Count => 24,
        Shape::Expr => 25,
        Shape::Embed(_) => 26,
        Shape::EmbedFrom(_) => 26,
    }
}

/// @emoji 🔑️ `blake3` over a canonical serialization of `spec`'s `(field id, key, shape-tag)`
/// tuples, sorted by id — stable regardless of `spec.fields`' declaration order, and independent
/// of any nested lazy `fn() -> RecordSpec` payload (only the shape's discriminant is hashed, not
/// its recursive contents, which is what keeps self-referential specs hashable at all).
pub fn schema_hash(spec: &RecordSpec) -> [u8; 32] {
    let mut fields: Vec<&FieldSpec> = spec.fields.iter().collect();
    fields.sort_by_key(|f| f.id);
    let mut buf = Vec::new();
    for f in fields {
        write_varint_u64(&mut buf, f.id as u64);
        write_varint_u64(&mut buf, f.key.len() as u64);
        buf.extend_from_slice(f.key.as_bytes());
        buf.push(shape_tag(&f.shape));
    }
    *blake3::hash(&buf).as_bytes()
}
//#endregion 🔖️SchemaHash

//#region 🔖️Document
/// @emoji ⚙️ Knobs for [`encode_document`]. `canonical` gates only the `OPTIONAL_CANONICAL`
/// header bit — the sorted-fields/omitted-Absent/sorted-map-keys/minimal-varint/normalized-f64/
/// interning/packed-numeric rules are applied unconditionally (the purity LAW demands determinism
/// regardless of `HashMap` iteration order, so there is no looser "non-canonical" code path).
#[derive(Clone, Debug)]
pub struct EncodeOptions {
    pub canonical: bool,
    pub codec: CodecId,
    pub chunk_threshold: u64,
    pub chunk_size: u64,
    pub frame_size: u64,
    pub preserve_unknown: bool,
    pub limits: PackLimits,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self { canonical: true, codec: CodecId(1), chunk_threshold: 256 * 1024, chunk_size: 1024 * 1024, frame_size: 1024 * 1024, preserve_unknown: true, limits: PackLimits::default() }
    }
}

/// @emoji ⚙️ Knobs for [`decode_document`].
#[derive(Clone, Debug)]
pub struct DecodeOptions {
    pub verification: crate::os_pack::format::VerificationLevel,
    pub preserve_unknown: bool,
    pub limits: PackLimits,
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self { verification: crate::os_pack::format::VerificationLevel::Standard, preserve_unknown: true, limits: PackLimits::default() }
    }
}

/// @emoji 🩺️ What [`decode_document`] observed beyond the plain `RecordValue`: field ids present
/// on the wire but absent from the caller's `RecordSpec`, any unrecognized segment kinds,
/// whether the manifest's stored `schema_hash` disagrees with the caller's `spec`, and the
/// verification level actually applied.
#[derive(Clone, Debug)]
pub struct DecodeReport {
    pub unknown_field_ids: Vec<u16>,
    pub unknown_segments: Vec<u8>,
    pub schema_drift: bool,
    pub verified: crate::os_pack::format::VerificationLevel,
}

/// @emoji 🚪️ The single entry point every other `pack_*`/`vcs`/`dsl_derive` crate encodes a
/// `RecordValue` through. Pre-pass computes the deterministic symbol table, then writes
/// `Symbols`, one-or-more `Document` frames (split at `options.frame_size`), any `Bytes64` chunks
/// produced along the way, and finally the `Manifest`/`End`/`Footer` via `PackWriter::finish`.
pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    let symbols = build_symbols(spec, record);
    let mut symbol_index = HashMap::with_capacity(symbols.len());
    for (i, s) in symbols.iter().enumerate() {
        symbol_index.insert(s.clone(), i as u64);
    }

    let write_options = crate::os_pack::format::WriteOptions { required_flags: 0, optional_flags: if options.canonical { crate::os_pack::format::OPTIONAL_CANONICAL } else { 0 }, codec: options.codec };
    let mut writer = crate::os_pack::format::PackWriter::begin(Vec::new(), &write_options)?;

    let symbols_payload = crate::os_pack::format::encode_symbols(&symbols);
    writer.write_segment(crate::KIND_SYMBOLS, &symbols_payload)?;

    let field_count = record.fields.values().filter(|v| !matches!(v, FieldValue::Absent)).count() as u64;
    let doc_payload = {
        let mut enc_ctx = EncCtx { symbol_index, writer: &mut writer, options };
        encode_record_fields(&mut enc_ctx, Some(spec), record, 0)?
    };

    let frame_size = options.frame_size.max(1) as usize;
    let doc_start = writer.position();
    let mut frame_count: u64 = 0;
    for frame in doc_payload.chunks(frame_size) {
        writer.write_segment(crate::KIND_DOCUMENT, frame)?;
        frame_count += 1;
    }
    let doc_end = writer.position();

    let manifest = crate::os_pack::format::Manifest {
        schema_name: String::new(),
        schema_hash: schema_hash(spec),
        doc_span: crate::os_pack::ByteRange { offset: doc_start, len: doc_end - doc_start },
        doc_frame_count: frame_count,
        symbols_span: crate::os_pack::ByteRange { offset: 0, len: 0 },
        chunk_table_span: crate::os_pack::ByteRange { offset: 0, len: 0 },
        field_index_span: crate::os_pack::ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: doc_payload.len() as u64,
        field_count,
        chunk_count: 0,
        symbol_count: symbols.len() as u64,
    };
    writer.finish(&manifest)
}

/// @emoji 🚪️ The single entry point every other `pack_*`/`vcs`/`dsl_derive` crate decodes a
/// `RecordValue` through. Opens the pack file at manifest level, reads and concatenates the
/// `Document` frame(s), then decodes the top-level record body against `spec` — self-describing
/// enough that any field id `spec` doesn't recognize still decodes and is preserved (subject to
/// `options.preserve_unknown`) and reported.
pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<(RecordValue, DecodeReport), PackError> {
    let pack_file = crate::os_pack::format::PackFile::open_manifest(bytes, &options.limits, options.verification)?;
    let manifest = pack_file.manifest().ok_or_else(|| PackError::Schema("manifest not loaded".to_string()))?;
    let schema_drift = manifest.schema_hash != schema_hash(spec);
    let body = pack_file.body_bytes(options.verification)?;

    let mut reader = ByteReader::new(&body);
    let mut dec_ctx = DecCtx { source: DecSource::File(&pack_file), limits: options.limits.clone(), verification: options.verification, preserve_unknown: options.preserve_unknown, unknown_field_ids: Vec::new() };
    let record = decode_record_fields(&mut reader, Some(spec), &mut dec_ctx, 0)?;

    let report = DecodeReport { unknown_field_ids: dec_ctx.unknown_field_ids, unknown_segments: Vec::new(), schema_drift, verified: options.verification };
    Ok((record, report))
}

/// @emoji 🎯️ Container-less twin of [`encode_document`] for small payloads (operation/command
/// records): `symbol_count varint, (len varint, utf8)*, record fields` — no header, segments,
/// manifest, or footer, and never any `Bytes64` chunking (oversized bytes stay inline via
/// `TAG_BYTES`). Deterministic by the same purity rules as the document path: byte-identical
/// output for equal `(spec, record)` regardless of map iteration order.
pub fn encode_record_body(spec: &RecordSpec, record: &RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    let symbols = build_symbols(spec, record);
    let mut symbol_index = HashMap::with_capacity(symbols.len());
    for (i, s) in symbols.iter().enumerate() {
        symbol_index.insert(s.clone(), i as u64);
    }
    let mut out = Vec::new();
    write_varint_u64(&mut out, symbols.len() as u64);
    for s in &symbols {
        write_varint_u64(&mut out, s.len() as u64);
        out.extend_from_slice(s.as_bytes());
    }
    let mut body_options = options.clone();
    body_options.chunk_threshold = u64::MAX;
    let write_options = crate::os_pack::format::WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
    let mut writer = crate::os_pack::format::PackWriter::begin(Vec::new(), &write_options)?;
    let fields = {
        let mut enc_ctx = EncCtx { symbol_index, writer: &mut writer, options: &body_options };
        encode_record_fields(&mut enc_ctx, Some(spec), record, 0)?
    };
    out.extend_from_slice(&fields);
    Ok(out)
}

/// @emoji 🎯️ Decodes an [`encode_record_body`] payload against `spec`. Unknown fields decode,
/// are preserved (subject to `options.preserve_unknown`), and are reported exactly like the
/// document path; a `TAG_BYTES_CHUNKED` value is malformed here by construction.
pub fn decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<(RecordValue, DecodeReport), PackError> {
    let mut reader = ByteReader::new(bytes);
    let symbol_count = reader.read_varint_u64()?;
    if symbol_count > u64::from(options.limits.max_symbols) {
        return Err(PackError::LimitExceeded("record-body symbol count exceeds max_symbols"));
    }
    let mut symbols = Vec::with_capacity(symbol_count as usize);
    for _ in 0..symbol_count {
        let len = reader.read_varint_u64()?;
        if len > options.limits.max_segment_len {
            return Err(PackError::LimitExceeded("record-body symbol length exceeds max_segment_len"));
        }
        let raw = reader.read_bytes(len as usize)?;
        let s = std::str::from_utf8(raw).map_err(|_| PackError::Malformed { what: "symbol", offset: reader.position() as u64, detail: "invalid utf8".to_string() })?;
        symbols.push(s.to_string());
    }
    let mut dec_ctx = DecCtx { source: DecSource::Inline { symbols }, limits: options.limits.clone(), verification: options.verification, preserve_unknown: options.preserve_unknown, unknown_field_ids: Vec::new() };
    let record = decode_record_fields(&mut reader, Some(spec), &mut dec_ctx, 0)?;
    let report = DecodeReport { unknown_field_ids: dec_ctx.unknown_field_ids, unknown_segments: Vec::new(), schema_drift: false, verified: options.verification };
    Ok((record, report))
}
//#endregion 🔖️Document

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_dsl::schema::{ExprOp, ExprValue};
    use crate::os_dsl::schema::{FieldSpec, RecordLayout};

    //#region 🔖️Fixtures
    fn nested_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Text).optional()])
    }

    fn table_row_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "id", Shape::UInt), FieldSpec::new(2, "name", Shape::Text), FieldSpec::new(3, "score", Shape::Float), FieldSpec::new(4, "active", Shape::Bool)])
    }

    fn header_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "name", Shape::Text), FieldSpec::new(2, "description", Shape::Text).optional()])
    }

    fn table_row_with_nested_record_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "id", Shape::UInt), FieldSpec::new(2, "header", Shape::Record(header_spec))])
    }

    fn table_row_with_tuple_spec() -> RecordSpec {
        RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "id", Shape::UInt), FieldSpec::new(2, "distortion", Shape::Tuple(Box::new(Shape::Float), Some(5)))])
    }

    fn stmt_foo_spec() -> RecordSpec {
        RecordSpec::new(Some("foo"), RecordLayout::Lines, vec![FieldSpec::new(1, "x", Shape::Int)])
    }

    fn stmt_bar_spec() -> RecordSpec {
        RecordSpec::new(Some("bar"), RecordLayout::Lines, vec![FieldSpec::new(1, "y", Shape::Text)])
    }

    /// @emoji 🧬️ One field of every `Shape` variant, exercising every wire tag in a single spec.
    fn full_spec() -> RecordSpec {
        RecordSpec::new(
            None,
            RecordLayout::Lines,
            vec![
                FieldSpec::new(1, "bool_field", Shape::Bool),
                FieldSpec::new(2, "int_field", Shape::Int),
                FieldSpec::new(3, "uint_field", Shape::UInt),
                FieldSpec::new(4, "float_field", Shape::Float),
                FieldSpec::new(5, "text_field", Shape::Text),
                FieldSpec::new(6, "bytes_field", Shape::Bytes64),
                FieldSpec::new(7, "enum_field", Shape::Enum(vec![("red".to_string(), 0), ("green".to_string(), 1), ("blue".to_string(), 2)])),
                FieldSpec::new(8, "tuple_field", Shape::Tuple(Box::new(Shape::Int), Some(3))),
                FieldSpec::new(9, "list_field", Shape::List(Box::new(Shape::Text))),
                FieldSpec::new(10, "record_field", Shape::Record(nested_spec)),
                FieldSpec::new(11, "block_field", Shape::Block(Box::new(Shape::Int))),
                FieldSpec::new(12, "statements_field", Shape::Statements(vec![("foo".to_string(), stmt_foo_spec), ("bar".to_string(), stmt_bar_spec)])),
                FieldSpec::new(13, "map_field", Shape::Map(Box::new(Shape::Int))),
                FieldSpec::new(14, "value_field", Shape::Value),
                FieldSpec::new(15, "table_field", Shape::Table(table_row_spec)),
                FieldSpec::new(16, "wire_field", Shape::Wire),
                FieldSpec::new(17, "quantity_field", Shape::Quantity(crate::os_dsl::unit_by_symbol("GPa").unwrap())),
                FieldSpec::new(18, "angle_field", Shape::Angle(crate::os_dsl::unit_by_symbol("deg").unwrap())),
                FieldSpec::new(19, "ref_field", Shape::Ref("material")),
                FieldSpec::new(20, "coord_field", Shape::Coord(3)),
                FieldSpec::new(21, "dir_field", Shape::Dir),
                FieldSpec::new(22, "dim_field", Shape::Dim(2)),
                FieldSpec::new(23, "range_field", Shape::Range),
                FieldSpec::new(24, "count_field", Shape::Count),
                FieldSpec::new(25, "expr_field", Shape::Expr),
                FieldSpec::new(26, "embed_field", Shape::Embed("jack")),
            ],
        )
    }

    fn full_record() -> RecordValue {
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Bool(true));
        fields.insert(2, FieldValue::Int(-42));
        fields.insert(3, FieldValue::UInt(9_999_999_999));
        fields.insert(4, FieldValue::Float(3.5));
        fields.insert(5, FieldValue::Text("hello café".to_string()));
        fields.insert(6, FieldValue::Bytes64(vec![1, 2, 3, 4, 5]));
        fields.insert(7, FieldValue::Enum(1));
        fields.insert(8, FieldValue::Tuple(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)]));
        fields.insert(9, FieldValue::List(vec![FieldValue::Text("x".to_string()), FieldValue::Text("y".to_string())]));
        let mut nested = HashMap::new();
        nested.insert(1, FieldValue::Int(7));
        nested.insert(2, FieldValue::Text("nested".to_string()));
        fields.insert(10, FieldValue::Record(RecordValue { fields: nested }));
        fields.insert(11, FieldValue::Block(Box::new(FieldValue::Int(99))));
        let mut foo_fields = HashMap::new();
        foo_fields.insert(1, FieldValue::Int(5));
        let mut bar_fields = HashMap::new();
        bar_fields.insert(1, FieldValue::Text("statement text".to_string()));
        fields.insert(12, FieldValue::Statements(vec![("foo".to_string(), RecordValue { fields: foo_fields }), ("bar".to_string(), RecordValue { fields: bar_fields })]));
        // `Map`/`DslValue::Object` are `Vec`-backed so `PartialEq` is order-sensitive; since
        // canonical encoding always sorts entries by key bytes, these fixtures are pre-sorted
        // ("aaa" < "zzz", "arr" < "k") so the round-trip equality check below holds exactly.
        fields.insert(13, FieldValue::Map(vec![("aaa".to_string(), FieldValue::Int(2)), ("zzz".to_string(), FieldValue::Int(1))]));
        fields.insert(14, FieldValue::Value(DslValue::Object(vec![("arr".to_string(), DslValue::Array(vec![DslValue::Bool(true), DslValue::Null])), ("k".to_string(), DslValue::Number(1.0))])));
        let table_rows: Vec<FieldValue> = (0..3)
            .map(|i| {
                let mut row = HashMap::new();
                row.insert(1, FieldValue::UInt(i as u64));
                row.insert(2, FieldValue::Text(format!("row{i}")));
                row.insert(3, FieldValue::Float(i as f64 * 1.5));
                row.insert(4, FieldValue::Bool(i % 2 == 0));
                FieldValue::Record(RecordValue { fields: row })
            })
            .collect();
        fields.insert(15, FieldValue::List(table_rows));
        fields.insert(
            16,
            FieldValue::Wire(WireValue {
                from: WireNode { id: "a".to_string(), kind: Some("Kind".to_string()), port: None },
                edge: Some((true, WireNode { id: "b".to_string(), kind: None, port: Some("out".to_string()) })),
                edge_label: WireEdgeLabel::default(),
                properties: DslValue::Object(vec![("weight".to_string(), DslValue::Number(2.0))]),
            }),
        );
        fields.insert(17, FieldValue::Float(210.0));
        fields.insert(18, FieldValue::Float(0.5));
        fields.insert(19, FieldValue::Text("s355".to_string()));
        fields.insert(20, FieldValue::Tuple(vec![FieldValue::Float(1.35), FieldValue::Float(0.0), FieldValue::Float(-2.4)]));
        fields.insert(21, FieldValue::Tuple(vec![FieldValue::Float(0.0), FieldValue::Float(1.0), FieldValue::Float(0.0)]));
        fields.insert(22, FieldValue::Tuple(vec![FieldValue::Float(2.4), FieldValue::Float(0.12)]));
        fields.insert(23, FieldValue::Tuple(vec![FieldValue::Float(0.0), FieldValue::Float(10.0), FieldValue::Float(0.5)]));
        fields.insert(24, FieldValue::UInt(24));
        fields.insert(
            25,
            FieldValue::Expr(ExprValue::Binary(
                ExprOp::Add,
                Box::new(ExprValue::Binary(ExprOp::Mul, Box::new(ExprValue::Num(1.35)), Box::new(ExprValue::Var("G".to_string())))),
                Box::new(ExprValue::Binary(ExprOp::Mul, Box::new(ExprValue::Num(1.5)), Box::new(ExprValue::Var("Q".to_string())))),
            )),
        );
        fields.insert(26, FieldValue::Text("MATCH (a) RETURN a".to_string()));
        RecordValue { fields }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️RoundTrip
    #[test]
    fn round_trips_every_shape_variant_in_one_document() {
        let spec = full_spec();
        let record = full_record();
        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, report) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        assert!(report.unknown_field_ids.is_empty());
        assert!(!report.schema_drift);
        assert_eq!(decoded, record);
    }

    #[test]
    fn round_trips_scalar_edge_cases() {
        let spec = RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![
                FieldSpec::new(1, "empty_text", Shape::Text),
                FieldSpec::new(2, "nan", Shape::Float),
                FieldSpec::new(3, "neg_zero", Shape::Float),
                FieldSpec::new(4, "big_int", Shape::Int),
                FieldSpec::new(5, "big_uint", Shape::UInt),
                FieldSpec::new(6, "unicode", Shape::Text),
                FieldSpec::new(7, "empty_list", Shape::List(Box::new(Shape::Int))),
                FieldSpec::new(8, "empty_map", Shape::Map(Box::new(Shape::Int))),
                FieldSpec::new(9, "empty_bytes", Shape::Bytes64),
            ],
        );
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Text(String::new()));
        fields.insert(2, FieldValue::Float(f64::NAN));
        fields.insert(3, FieldValue::Float(-0.0));
        fields.insert(4, FieldValue::Int(i64::MIN));
        fields.insert(5, FieldValue::UInt(u64::MAX));
        fields.insert(6, FieldValue::Text("héllo wörld 🔖️ 日本語".to_string()));
        fields.insert(7, FieldValue::List(vec![]));
        fields.insert(8, FieldValue::Map(vec![]));
        fields.insert(9, FieldValue::Bytes64(vec![]));
        let record = RecordValue { fields };

        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");

        assert_eq!(decoded.get(1), Some(&FieldValue::Text(String::new())));
        match decoded.get(2) {
            Some(FieldValue::Float(f)) => assert!(f.is_nan()),
            other => panic!("expected NaN float, got {other:?}"),
        }
        assert_eq!(decoded.get(3), Some(&FieldValue::Float(-0.0)));
        assert!(matches!(decoded.get(3), Some(FieldValue::Float(f)) if f.is_sign_negative()));
        assert_eq!(decoded.get(4), Some(&FieldValue::Int(i64::MIN)));
        assert_eq!(decoded.get(5), Some(&FieldValue::UInt(u64::MAX)));
        assert_eq!(decoded.get(6), Some(&FieldValue::Text("héllo wörld 🔖️ 日本語".to_string())));
        assert_eq!(decoded.get(7), Some(&FieldValue::List(vec![])));
        assert_eq!(decoded.get(8), Some(&FieldValue::Map(vec![])));
        assert_eq!(decoded.get(9), Some(&FieldValue::Bytes64(vec![])));
    }

    #[test]
    fn packed_numeric_list_and_tuple_round_trip_and_use_packed_tags() {
        let spec = RecordSpec::new(
            None,
            RecordLayout::Inline,
            vec![
                FieldSpec::new(1, "ints", Shape::List(Box::new(Shape::Int))),
                FieldSpec::new(2, "floats", Shape::List(Box::new(Shape::Float))),
                FieldSpec::new(3, "uints", Shape::Tuple(Box::new(Shape::UInt), None)),
                FieldSpec::new(4, "mixed", Shape::List(Box::new(Shape::Value))),
            ],
        );
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(-2), FieldValue::Int(3)]));
        fields.insert(2, FieldValue::List(vec![FieldValue::Float(1.5), FieldValue::Float(-2.5)]));
        fields.insert(3, FieldValue::Tuple(vec![FieldValue::UInt(1), FieldValue::UInt(2), FieldValue::UInt(3)]));
        fields.insert(4, FieldValue::List(vec![FieldValue::Value(DslValue::Bool(true)), FieldValue::Value(DslValue::Null)]));
        let record = RecordValue { fields };

        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        assert_eq!(decoded.get(1), record.get(1));
        assert_eq!(decoded.get(2), record.get(2));
        assert_eq!(decoded.get(3), record.get(3));
        assert_eq!(decoded.get(4), record.get(4));
    }

    #[test]
    fn table_soa_round_trips_with_sparse_columns() {
        let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "rows", Shape::Table(table_row_spec))]);
        let mut row0 = HashMap::new();
        row0.insert(1, FieldValue::UInt(10));
        row0.insert(2, FieldValue::Text("alpha".to_string()));
        row0.insert(3, FieldValue::Float(1.25));
        row0.insert(4, FieldValue::Bool(true));
        let mut row1 = HashMap::new();
        row1.insert(1, FieldValue::UInt(20));
        // row1 omits "name" (sparse Text column) and "active" (sparse Bool column).
        row1.insert(3, FieldValue::Float(-9.5));
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::List(vec![FieldValue::Record(RecordValue { fields: row0 }), FieldValue::Record(RecordValue { fields: row1 })]));
        let record = RecordValue { fields };

        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        let Some(FieldValue::List(rows)) = decoded.get(1) else { panic!("expected table rows") };
        assert_eq!(rows.len(), 2);
        let FieldValue::Record(r0) = &rows[0] else { panic!("row0 not a record") };
        assert_eq!(r0.get(2), Some(&FieldValue::Text("alpha".to_string())));
        assert_eq!(r0.get(4), Some(&FieldValue::Bool(true)));
        let FieldValue::Record(r1) = &rows[1] else { panic!("row1 not a record") };
        assert_eq!(r1.get(2), Some(&FieldValue::Absent));
        assert_eq!(r1.get(4), Some(&FieldValue::Absent));
        assert_eq!(r1.get(3), Some(&FieldValue::Float(-9.5)));
    }

    /// @emoji 🪟️ Regression for a `TableSoA` column whose element type is a nested (non-Option)
    /// `Record` with its own `Option` sub-field left absent — `decode_table_soa`'s fallback branch
    /// must thread the known column shape through so `decode_record_fields` still backfills that
    /// sub-field as `Absent` instead of leaving it missing from the decoded `RecordValue` map.
    #[test]
    fn table_soa_nested_record_column_backfills_absent_option_subfield() {
        let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "rows", Shape::Table(table_row_with_nested_record_spec))]);
        let mut header_fields = HashMap::new();
        header_fields.insert(1, FieldValue::Text("Stakeholder A".to_string()));
        // "description" (field 2, Option<Text>) is intentionally omitted from the fixture — it
        // encodes as `Absent` and canonical-mode compaction drops it from the wire entirely.
        let mut row = HashMap::new();
        row.insert(1, FieldValue::UInt(1));
        row.insert(2, FieldValue::Record(RecordValue { fields: header_fields }));
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::List(vec![FieldValue::Record(RecordValue { fields: row })]));
        let record = RecordValue { fields };

        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        let Some(FieldValue::List(rows)) = decoded.get(1) else { panic!("expected table rows") };
        let FieldValue::Record(row0) = &rows[0] else { panic!("row0 not a record") };
        let Some(FieldValue::Record(header)) = row0.get(2) else { panic!("expected header record") };
        assert_eq!(header.get(1), Some(&FieldValue::Text("Stakeholder A".to_string())));
        assert_eq!(header.get(2), Some(&FieldValue::Absent), "nested record's Option sub-field must backfill to Absent, not be missing");
    }

    /// @emoji 🎯️ Regression for a `TableSoA` column whose element type is a fixed-size `Tuple`
    /// (e.g. a `[f32; 5]` lens-distortion field) — because every element is the same numeric
    /// kind, `encode_seq` collapses it to the packed `TAG_PACKED_F64` wire form, which carries no
    /// tuple-vs-list marker of its own. `decode_table_soa`'s fallback branch must thread the
    /// known column `Shape::Tuple` through so `decode_value` reconstructs a `FieldValue::Tuple`,
    /// not a `FieldValue::List` — a `List` fails `[T; N]`'s `DslField::from_value` downstream.
    #[test]
    fn table_soa_tuple_column_round_trips_as_tuple_not_list() {
        let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "rows", Shape::Table(table_row_with_tuple_spec))]);
        let mut row = HashMap::new();
        row.insert(1, FieldValue::UInt(1));
        row.insert(2, FieldValue::Tuple(vec![FieldValue::Float(0.1), FieldValue::Float(0.2), FieldValue::Float(0.3), FieldValue::Float(0.4), FieldValue::Float(0.5)]));
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::List(vec![FieldValue::Record(RecordValue { fields: row })]));
        let record = RecordValue { fields };

        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        let Some(FieldValue::List(rows)) = decoded.get(1) else { panic!("expected table rows") };
        let FieldValue::Record(row0) = &rows[0] else { panic!("row0 not a record") };
        assert_eq!(
            row0.get(2),
            Some(&FieldValue::Tuple(vec![FieldValue::Float(0.1), FieldValue::Float(0.2), FieldValue::Float(0.3), FieldValue::Float(0.4), FieldValue::Float(0.5)])),
            "tuple-shaped table column must decode as FieldValue::Tuple, not FieldValue::List"
        );
    }

    #[test]
    fn wire_literal_round_trips_bare_node_and_undirected_edge() {
        let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "w", Shape::Wire)]);
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Wire(WireValue { from: WireNode { id: "solo".to_string(), kind: None, port: None }, edge: None, edge_label: WireEdgeLabel::default(), properties: DslValue::Object(vec![]) }));
        let record = RecordValue { fields };
        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        assert_eq!(decoded.get(1), record.get(1));
    }
    //#endregion 🔖️RoundTrip

    //#region 🔖️Canonical
    #[test]
    fn canonical_encoding_is_byte_stable_across_shuffled_insertion_order() {
        let spec = full_spec();
        let record_a = full_record();
        // Rebuild an equal `RecordValue` by inserting fields in a deliberately different order —
        // `HashMap` insertion order never affects iteration order anyway, but this at minimum
        // proves two independent builds of an equal value encode identically, twice in a row.
        let mut shuffled_fields = HashMap::new();
        let mut ids: Vec<u16> = record_a.fields.keys().copied().collect();
        ids.sort_unstable_by(|a, b| b.cmp(a));
        for id in ids {
            shuffled_fields.insert(id, record_a.fields.get(&id).unwrap().clone());
        }
        let record_b = RecordValue { fields: shuffled_fields };
        assert_eq!(record_a, record_b);

        let bytes_a = encode_document(&spec, &record_a, &EncodeOptions::default()).expect("encode a");
        let bytes_b = encode_document(&spec, &record_b, &EncodeOptions::default()).expect("encode b");
        assert_eq!(bytes_a, bytes_b, "canonical encoding must be byte-identical regardless of HashMap insertion order");

        let bytes_a_again = encode_document(&spec, &record_a, &EncodeOptions::default()).expect("encode a again");
        assert_eq!(bytes_a, bytes_a_again, "encoding the same document twice must be byte-identical");
    }

    #[test]
    fn schema_hash_is_order_independent_and_content_sensitive() {
        let spec_a = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(2, "b", Shape::Text), FieldSpec::new(1, "a", Shape::Int)]);
        let spec_b = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Text)]);
        assert_eq!(schema_hash(&spec_a), schema_hash(&spec_b));

        let spec_c = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Float)]);
        assert_ne!(schema_hash(&spec_a), schema_hash(&spec_c));
    }
    //#endregion 🔖️Canonical

    //#region 🔖️Unknown
    #[test]
    fn unknown_field_round_trips_through_decode_then_reencode() {
        let full = full_spec();
        let mut record = full_record();
        // Add a field id absent from `narrow_spec` below but present in `full` for the initial
        // encode, simulating "a writer on a newer schema version wrote an extra field".
        record.fields.insert(200, FieldValue::Text("extra field payload".to_string()));
        record.fields.insert(201, FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)]));

        let mut widened_fields = full.fields.clone();
        widened_fields.push(FieldSpec::new(200, "extra", Shape::Text));
        widened_fields.push(FieldSpec::new(201, "extra_list", Shape::List(Box::new(Shape::Int))));
        let widened_spec = RecordSpec::new(full.keyword.as_deref(), full.layout, widened_fields);

        let bytes = encode_document(&widened_spec, &record, &EncodeOptions::default()).expect("encode with widened spec");

        // Decode against the NARROW spec (doesn't know fields 200/201) — they must still decode
        // and be reported as unknown.
        let (decoded, report) = decode_document(&bytes, &full, &DecodeOptions::default()).expect("decode with narrow spec");
        let mut unknown_sorted = report.unknown_field_ids.clone();
        unknown_sorted.sort_unstable();
        assert_eq!(unknown_sorted, vec![200, 201]);
        assert_eq!(decoded.get(200), Some(&FieldValue::Text("extra field payload".to_string())));
        assert_eq!(decoded.get(201), Some(&FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)])));

        // Re-encode the decoded (narrow-spec) RecordValue — the unknown fields must survive.
        let reencoded = encode_document(&full, &decoded, &EncodeOptions::default()).expect("re-encode");
        let (decoded_again, report_again) = decode_document(&reencoded, &full, &DecodeOptions::default()).expect("decode again");
        assert_eq!(decoded_again.get(200), Some(&FieldValue::Text("extra field payload".to_string())));
        assert_eq!(decoded_again.get(201), Some(&FieldValue::List(vec![FieldValue::Int(1), FieldValue::Int(2), FieldValue::Int(3)])));
        let mut unknown_again_sorted = report_again.unknown_field_ids.clone();
        unknown_again_sorted.sort_unstable();
        assert_eq!(unknown_again_sorted, vec![200, 201]);
    }

    #[test]
    fn preserve_unknown_false_drops_unknown_fields_from_decoded_value_but_still_reports_them() {
        let narrow = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int)]);
        let wide = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(2, "b", Shape::Text)]);
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Int(1));
        fields.insert(2, FieldValue::Text("dropped on decode".to_string()));
        let record = RecordValue { fields };

        let bytes = encode_document(&wide, &record, &EncodeOptions::default()).expect("encode");
        let mut options = DecodeOptions::default();
        options.preserve_unknown = false;
        let (decoded, report) = decode_document(&bytes, &narrow, &options).expect("decode");
        assert_eq!(report.unknown_field_ids, vec![2]);
        assert_eq!(decoded.get(2), None);
        assert_eq!(decoded.get(1), Some(&FieldValue::Int(1)));
    }
    //#endregion 🔖️Unknown

    //#region 🔖️Chunking
    #[test]
    fn large_bytes_field_is_chunked_and_round_trips() {
        let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "blob", Shape::Bytes64)]);
        let payload: Vec<u8> = (0..600_000u32).map(|i| (i % 256) as u8).collect();
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Bytes64(payload.clone()));
        let record = RecordValue { fields };

        let mut options = EncodeOptions::default();
        options.chunk_threshold = 1024;
        options.chunk_size = 64 * 1024;
        let bytes = encode_document(&spec, &record, &options).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        assert_eq!(decoded.get(1), Some(&FieldValue::Bytes64(payload)));
    }

    #[test]
    fn document_body_splits_across_multiple_frames_when_frame_size_is_small() {
        let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "text", Shape::Text)]);
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Text("x".repeat(5000)));
        let record = RecordValue { fields };

        let mut options = EncodeOptions::default();
        options.frame_size = 64;
        let bytes = encode_document(&spec, &record, &options).expect("encode");
        let (decoded, _) = decode_document(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        assert_eq!(decoded.get(1), record.get(1));
    }
    //#endregion 🔖️Chunking

    //#region 🔖️RecordBody
    #[test]
    fn record_body_round_trips_every_shape() {
        let spec = full_spec();
        let record = full_record();
        let bytes = encode_record_body(&spec, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, report) = decode_record_body(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        assert_eq!(decoded, record);
        assert!(report.unknown_field_ids.is_empty());
    }

    #[test]
    fn record_body_is_deterministic_for_equal_inputs() {
        let spec = full_spec();
        let a = encode_record_body(&spec, &full_record(), &EncodeOptions::default()).expect("encode a");
        let b = encode_record_body(&spec, &full_record(), &EncodeOptions::default()).expect("encode b");
        assert_eq!(a, b);
    }

    #[test]
    fn record_body_keeps_oversized_bytes_inline_instead_of_chunking() {
        let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "blob", Shape::Bytes64)]);
        let payload: Vec<u8> = (0..600_000u32).map(|i| (i % 256) as u8).collect();
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Bytes64(payload.clone()));
        let record = RecordValue { fields };

        let mut options = EncodeOptions::default();
        options.chunk_threshold = 1024;
        let bytes = encode_record_body(&spec, &record, &options).expect("encode");
        let (decoded, _) = decode_record_body(&bytes, &spec, &DecodeOptions::default()).expect("decode");
        assert_eq!(decoded.get(1), Some(&FieldValue::Bytes64(payload)));
    }

    #[test]
    fn record_body_preserves_and_reports_unknown_fields() {
        let wide = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int), FieldSpec::new(9, "extra", Shape::Text)]);
        let narrow = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "a", Shape::Int)]);
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Int(3));
        fields.insert(9, FieldValue::Text("kept".to_string()));
        let record = RecordValue { fields };

        let bytes = encode_record_body(&wide, &record, &EncodeOptions::default()).expect("encode");
        let (decoded, report) = decode_record_body(&bytes, &narrow, &DecodeOptions::default()).expect("decode");
        assert_eq!(decoded.get(9), Some(&FieldValue::Text("kept".to_string())));
        assert_eq!(report.unknown_field_ids, vec![9]);
    }
    //#endregion 🔖️RecordBody
}
//#endregion 🧪️Tests
