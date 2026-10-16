//! 📦️ `pack_value` — DSL schema-aware wire encoding/decoding of `semio_framework_dsl_record::RecordValue`
//! documents into the `pack_format` binary container. Implements every wire tag (0x00-0x16),
//! canonical-mode determinism (sorted field ids, omitted `Absent`, sorted map keys, minimal
//! varints, exact IEEE words, deterministic string interning, mandatory packed numeric
//! forms), `TableSoA` columnar encoding for `Shape::Table`, unknown-field preservation via
//! `DecodeReport`, `schema_hash`, and the top-level `encode_document`/`decode_document` entry
//! points every other `pack_*`/`vcs`/`dsl_derive` crate calls through.
//!
//! See the `## pack_value` section of the wave-0 contract at
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md` for the binding
//! byte layout this module implements against.

use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordSpecProducer, RecordValue, Shape, WireEdgeLabel, WireNode, WireValue};
use semio_framework_replication_codec::{write_varint_i64, write_varint_u64, ByteReader, ChunkId, CodecId, PackLimits};
use std::collections::{HashMap, HashSet};
use semio_framework_pack_record_schema::{ELEM_BOOL, ELEM_ENUM, ELEM_F64, ELEM_INT, ELEM_STR, ELEM_UINT, NumKind, TAG_ABSENT, TAG_BLOCK, TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_EXPR, TAG_F64, TAG_FALSE, TAG_INT, TAG_LIST, TAG_MAP, TAG_NULL, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_RECORD, TAG_STATEMENTS, TAG_STR, TAG_STR_INLINE, TAG_TABLE_SOA, TAG_TRUE, TAG_TUPLE, TAG_UINT, TAG_VALUE, TAG_WIRE, block_inner_shape, elem_shape_of, elem_tag_for_shape, map_inner_shape, schema_hash, schema_hash_controlled, statements_variants, table_spec_of};
#[cfg(test)]
use semio_framework_pack_record_encode::{encode_record_body_controlled, encode_value_record_body_controlled};

//#region 🔖️Canonical

/// 🧾️ Resolves a `Record` field's nested spec, if `shape` is `Record`.
fn record_spec_of(shape: Option<&Shape>) -> Option<RecordSpec> {
    match shape {
        Some(Shape::Record(spec_fn)) => Some((spec_fn.ordinary)()),
        _ => None,
    }
}

/// 🚧️ Every shape whose `FieldValue` representation is `Tuple` rather than `List` — the
/// packed-numeric-array fast path (`TAG_PACKED_F64`/`TAG_PACKED_VARINT`) collapses both to the
/// same bytes on the wire (a run of numbers has no other distinguishing feature), so `shape` is
/// the ONLY signal decode has left to reconstruct the right `FieldValue` variant. Every shape here
/// is a fixed-arity number tuple by construction (`Coord`/`Dir`/`Dim`/`Range`'s own parsers in
/// `dsl_schema` never produce anything else), so this can never rebuild the wrong shape.
fn is_tuple_shape(shape: Option<&Shape>) -> bool {
    matches!(shape, Some(Shape::Tuple(_, _)) | Some(Shape::Coord(_)) | Some(Shape::Dir) | Some(Shape::Dim(_)) | Some(Shape::Range))
}

/// 🛡️ Depth-limit check shared by every recursive encode/decode entry point.
fn check_depth(max_depth: u16, depth: u16) -> Result<(), PackRefusal> {
    if depth > max_depth {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"max_depth exceeded"});
    }
    Ok(())
}

/// 🧮️ A sequence is packed-eligible iff every element is the same numeric `FieldValue`
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

/// ✒️ Mutable state threaded through one `encode_document` call: the precomputed symbol
/// table (built by [`build_symbols`] in a deterministic pre-pass), the live segment/chunk writer,
/// and the caller's options.
struct EncCtx<'a> {
    symbol_index: HashMap<String, u64>,
    writer: &'a mut semio_framework_pack_format::PackWriter<Vec<u8>>,
    options: &'a EncodeOptions,
}

/// 📖️ Encodes a string using the precomputed interning decision: `TAG_STR` + symref if
/// `s` made it into the symbol table, else `TAG_STR_INLINE` + length-prefixed UTF-8 bytes.
fn encode_string(ctx: &mut EncCtx<'_>, s: &str, out: &mut Vec<u8>) {
    if let Some(&idx) = ctx.symbol_index.get(s) {
        out.push(TAG_STR);
        write_varint_u64(out, idx);
    } else {
        encode_string_inline(s, out);
    }
}

/// 📌️ Forces `TAG_STR_INLINE` regardless of the interning decision — the wire rule for
/// `Value`/`DslValue::Object` keys, which are never symrefs.
fn encode_string_inline(s: &str, out: &mut Vec<u8>) {
    out.push(TAG_STR_INLINE);
    write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

/// 🔗️ Writes a bare symref varint with NO leading tag — the wire rule for `Statements`
/// keywords and `TableSoA` `Str` columns, both of which are unconditionally interned.
fn write_symref_forced(ctx: &mut EncCtx<'_>, s: &str, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
    let idx = *ctx.symbol_index.get(s).ok_or_else(|| PackRefusal::from(ValueError::new(ValueRefusalKind::InvariantViolated,format!("symbol {s:?} missing from precomputed table"))))?;
    write_varint_u64(out, idx);
    Ok(())
}

/// 🔎️ Deterministic string-interning pre-pass: walks the whole document once (shape-aware
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

/// 🔁️ Mutually recursive with `walk_value_for_symbols`; both walkers are synchronous because they
/// only inspect already-resident values.
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
                let element_spec = (spec_fn.ordinary)();
                for row in items {
                    let FieldValue::Record(r) = row else { continue };
                    for field in &element_spec.fields {
                        let Some(v) = r.fields.get(&field.id) else { continue };
                        if matches!(v, FieldValue::Absent) {
                            continue;
                        }
                        if matches!(field.shape, Shape::Text | Shape::Ref(_)) {
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
                let spec = variants.and_then(|vs| vs.iter().find(|(k, _)| k == keyword)).map(|(_, f)| (f.ordinary)());
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

/// 🌱️ `DslValue::Object` keys are always inline (never interned) per the wire contract, so
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
/// 🧾️ Encodes one record's fields as `field_count varint, (field_id varint, value)*` —
/// the shared body used both for the top-level document and for nested `FieldValue::Record`
/// (behind its own `0x0D` tag). Always sorts by field id (the purity LAW: byte-identical output
/// regardless of `HashMap` iteration order) and always omits `Absent`. `spec` is `None` for a
/// genuinely schema-less context (an unrecognized `Statements` variant, a shape/value mismatch);
/// fields are then encoded generically. `options.preserve_unknown == false` drops fields whose id
/// isn't found in `spec` instead of encoding them.
fn encode_record_fields(ctx: &mut EncCtx<'_>, spec: Option<&RecordSpec>, record: &RecordValue, depth: u16) -> Result<Vec<u8>, PackRefusal> {
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

/// ✍️ Encodes one field value, tag-prefixed and self-describing. `shape` is the field's
/// declared `Shape` when known (disambiguates `Tuple` vs `List`, selects `TableSoA` for
/// `Shape::Table`, and resolves nested `Record`/`Block`/`Statements`/`Map` sub-shapes); `None`
/// encodes the value generically from its runtime `FieldValue` variant alone — the path used for
/// field ids absent from the caller's `RecordSpec`, which is what makes unknown-field
/// preservation possible without ever having seen their original schema.
// 🔁️ Mutually recursive with `encode_record_fields`/`encode_seq`/`encode_map` (and directly
// self-recursive for `Block`) — every edge in that cycle is `Box::pin(...).await` (R10 residue
// shape 3): an `async fn`'s own opaque `Future` type cannot embed itself or a cycle-partner's
// opaque type at an unboxed, unbounded size.
fn encode_value(ctx: &mut EncCtx<'_>, shape: Option<&Shape>, value: &FieldValue, depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
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
            out.extend_from_slice(&f.to_le_bytes());
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
            encode_string(ctx, &semio_framework_dsl_record::print_expr(expr), out);
        }
    }
    Ok(())
}

/// 🧱️ Encodes a `Bytes64` payload direct (`TAG_BYTES`) or, once it reaches
/// `options.chunk_threshold`, split into `options.chunk_size`-sized chunks written through the
/// live `PackWriter` (`TAG_BYTES_CHUNKED` + the resulting `ChunkId`s).
fn encode_bytes(ctx: &mut EncCtx<'_>, bytes: &[u8], out: &mut Vec<u8>) -> Result<(), PackRefusal> {
    if (bytes.len() as u64) >= ctx.options.chunk_threshold {
        let chunk_size = ctx.options.chunk_size.max(1) as usize;
        let mut ids = Vec::new();
        for piece in bytes.chunks(chunk_size) {
            let mut chunk = ::semio_framework_async::poll::resolve_ready(ctx.writer.begin_identity_chunk(piece.len()))?;
            for fragment in piece.chunks(4096) {
                if let Err(error) = ::semio_framework_async::poll::resolve_ready(chunk.write_fragment(fragment)) {
                    chunk.close();
                    return Err(error);
                }
            }
            ids.push(::semio_framework_async::poll::resolve_ready(chunk.finish())?);
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

/// 📚️ Encodes a `Tuple`/`List` sequence: the mandatory packed `0x15`/`0x16` form when
/// every element is the same numeric kind, else the plain self-describing `0x0B`/`0x0C` form.
fn encode_seq(ctx: &mut EncCtx<'_>, items: &[FieldValue], elem_shape: Option<&Shape>, is_tuple: bool, depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
    if let Some(kind) = homogeneous_numeric_kind(items) {
        match kind {
            NumKind::F64 => {
                out.push(TAG_PACKED_F64);
                write_varint_u64(out, items.len() as u64);
                for it in items {
                    if let FieldValue::Float(f) = it {
                        out.extend_from_slice(&f.to_le_bytes());
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

/// 🗺️ Encodes `Map`/object entries sorted by key bytes (canonical, always — not just when
/// `options.canonical`, per the purity LAW), each key using the conditional interning rule.
fn encode_map(ctx: &mut EncCtx<'_>, entries: &[(String, FieldValue)], inner_shape: Option<&Shape>, depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
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

/// 📜️ Encodes `Statements`: `count, (keyword symref, Record-payload)*`. The keyword is
/// always a bare forced symref (never a self-describing string tag) per the wire contract.
fn encode_statements(ctx: &mut EncCtx<'_>, variants: Option<&Vec<(String, RecordSpecProducer)>>, items: &[(String, RecordValue)], depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    out.push(TAG_STATEMENTS);
    write_varint_u64(out, items.len() as u64);
    for (keyword, record) in items {
        write_symref_forced(ctx, keyword, out)?;
        let spec = variants.and_then(|vs| vs.iter().find(|(k, _)| k == keyword)).map(|(_, f)| (f.ordinary)());
        let fields = encode_record_fields(ctx, spec.as_ref(), record, depth + 1)?;
        out.extend_from_slice(&fields);
    }
    Ok(())
}

/// 🌱️ Encodes a `DslValue` using the same self-describing tag set recursively; object
/// entries retain owner occurrence order with keys FORCED inline (`encode_string_inline`, never a symref) —
/// the one deliberate carve-out from the general conditional-interning rule.
///
/// `Number` writes its own variant's tag — `TAG_UINT`/`TAG_INT` carry the exact 64-bit magnitude
/// as a canonical unsigned/zig-zag LEB128, `TAG_F64` the complete little-endian IEEE word. Widening
/// through `as_f64` is not injective past 2^53, so the tag, not the reader, is what preserves an
/// integer; see the `🎒️pack-dynamic-integer-v1` corpus under `💻️os/🧫️fixtures`.
fn encode_dsl_value(ctx: &mut EncCtx<'_>, v: &DslValue, depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    match v {
        DslValue::Null => out.push(TAG_NULL),
        DslValue::Bool(b) => out.push(if *b { TAG_TRUE } else { TAG_FALSE }),
        DslValue::Number(Number::UInt(u)) => {
            out.push(TAG_UINT);
            write_varint_u64(out, *u);
        }
        DslValue::Number(Number::Int(i)) => {
            out.push(TAG_INT);
            write_varint_i64(out, *i);
        }
        DslValue::Number(Number::Float(f)) => {
            out.push(TAG_F64);
            out.extend_from_slice(&f.to_le_bytes());
        }
        DslValue::String(s) => encode_string(ctx, s, out),
        DslValue::Bytes(bytes)=>encode_bytes(ctx,bytes,out)?,
        DslValue::Array(items) => {
            out.push(TAG_LIST);
            write_varint_u64(out, items.len() as u64);
            for it in items {
                encode_dsl_value(ctx, it, depth + 1, out)?;
            }
        }
        DslValue::Object(entries) => {
            out.push(TAG_MAP);
            write_varint_u64(out, entries.len() as u64);
            for (k, val) in entries {
                encode_string_inline(k, out);
                encode_dsl_value(ctx, val, depth + 1, out)?;
            }
        }
    }
    Ok(())
}

/// 🕸️ Encodes a `Wire` literal. Wire sub-format (presence bitmask + node layout) is this
/// crate's own choice — the contract pins only the outer `0x13` tag and the constituent parts
/// (`from`, optional `to`, `props`); everything here just needs to round-trip, which it does.
fn encode_wire(ctx: &mut EncCtx<'_>, w: &WireValue, depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
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
/// 🧭️ Where symrefs and chunk ids resolve during one decode: a full opened `PackFile`
/// (the `decode_document` path) or a container-less inline symbol table (the
/// `decode_record_body` path, which has no chunk table by construction).
enum DecSource<'a> {
    File(&'a semio_framework_pack_format::PackFile<&'a [u8]>),
    Inline { symbols: Vec<String> },
}

/// 📖️ Mutable state threaded through one `decode_document`/`decode_record_body` call: the
/// symref/chunk resolution source, the caller's limits/verification/preserve-unknown choices, and
/// the accumulated unknown-field-id report.
struct DecCtx<'a> {
    source: DecSource<'a>,
    limits: PackLimits,
    verification: semio_framework_pack_format::VerificationLevel,
    preserve_unknown: bool,
    unknown_field_ids: Vec<u16>,
    materialization: &'a dyn MaterializationAdmission,
}

/// 🧮️ Cumulative concrete requested storage for owned UTF-8 and typed collection slots.
struct ValueMaterialization {
    used: std::cell::Cell<u64>,
    maximum: u64,
}

impl ValueMaterialization {
    fn charge(&self, bytes: u64) -> Result<(), PackRefusal> {
        let used = self.used.get().checked_add(bytes).filter(|used| *used <= self.maximum).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire value materialization exceeds max_total_alloc" })?;
        self.used.set(used);
        Ok(())
    }
}

/// 🧾️ Transfers octets after controlled validation of the same immutable source range.
/// Both private admission owners use the exact span copy; the immutable borrow prevents its
/// validated scalar words from changing between validation and owned transfer.
fn copy_validated_span_utf8<M:MaterializationAdmission+?Sized>(materialization:&M,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
    materialization.check_span_utf8(bytes,offset,what)?;
    let bytes=materialization.copy_span_bytes(bytes)?;
    Ok(unsafe{String::from_utf8_unchecked(bytes)})
}

trait MaterializationAdmission {
    fn schema(&self,producer:&RecordSpecProducer)->Result<RecordSpec,PackRefusal>{Ok((producer.ordinary)())}
    fn charge(&self,bytes:u64)->Result<(),PackRefusal>;
    fn step(&self)->Result<(),PackRefusal>{Ok(())}
    fn controlled(&self)->bool{false}
    fn copy_text(&self,text:&str)->Result<String,PackRefusal>{self.charge(text.len() as u64)?;let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded string allocation" })?;output.push_str(text);Ok(output)}
    fn copy_bytes(&self,bytes:&[u8])->Result<Vec<u8>,PackRefusal>{self.charge(bytes.len() as u64)?;let mut output=Vec::new();output.try_reserve_exact(bytes.len()).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded octet allocation" })?;output.extend_from_slice(bytes);Ok(output)}
    fn octet_buffer(&self,count:usize)->Result<Vec<u8>,PackRefusal>{self.charge(count as u64)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded octet allocation" })?;Ok(output)}
    fn copy_span_bytes(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>)->Result<Vec<u8>,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_bytes(bytes)}
        let mut output=self.octet_buffer(bytes.len())?;
        for byte in bytes.iter(){self.step()?;output.push(byte);}
        Ok(output)
    }
    fn check_span_utf8(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<(),PackRefusal>{
        let invalid=||PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what,offset,detail:"invalid utf8"};
        let mut position=0;
        while position<bytes.len(){
            self.step()?;
            let first=bytes[position];
            let length=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>return Err(invalid())};
            if position+length>bytes.len(){return Err(invalid())}
            let mut word=[0;4];
            for(index,byte)in word[..length].iter_mut().enumerate(){*byte=bytes[position+index];}
            std::str::from_utf8(&word[..length]).map_err(|_|invalid())?;
            position+=length;
        }
        Ok(())
    }
    fn copy_span_utf8(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_utf8(bytes,offset,what)}
        copy_validated_span_utf8(self,bytes,offset,what)
    }
    fn append_chunk(&self,_file:&semio_framework_pack_format::PackFile<&[u8]>,_id:ChunkId,_verification:semio_framework_pack_format::VerificationLevel,_output:&mut Vec<u8>)->Result<(),PackRefusal>{Err(PackRefusal::from(ValueError::new(ValueRefusalKind::UnsupportedOwner,"chunk owner has no controlled admission")))}
    fn expression(&self,text:&str,_maximum_depth:u16)->Result<semio_framework_dsl_record::ExprValue,PackRefusal>{semio_framework_dsl_record::parse_expr_text(text).map_err(PackRefusal::from)}
    fn copy_utf8(&self,bytes:&[u8],offset:u64,what:&'static str)->Result<String,PackRefusal>{let value=std::str::from_utf8(bytes).map_err(|_|PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what,offset,detail:"invalid utf8".into()})?;self.copy_text(value)}
}

impl MaterializationAdmission for ValueMaterialization {fn charge(&self,bytes:u64)->Result<(),PackRefusal>{ValueMaterialization::charge(self,bytes)}}

struct ControlledMaterialization<'a,'callback>{control:std::cell::RefCell<&'a mut semio_framework_value::native_decoding::NativeDecodeControl<'callback>>,maximum:u64}
impl ControlledMaterialization<'_,'_>{
    fn check(&self,bytes:u64)->Result<(),PackRefusal>{if (self.control.borrow().owned_bytes() as u64).checked_add(bytes).filter(|next|*next<=self.maximum).is_none(){return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"controlled native materialization exceeds max_total_alloc")));}Ok(())}
}
impl MaterializationAdmission for ControlledMaterialization<'_,'_>{
    fn schema(&self,producer:&RecordSpecProducer)->Result<RecordSpec,PackRefusal>{let maximum=usize::try_from(self.maximum).unwrap_or(usize::MAX);self.control.borrow_mut().scoped_maximum(maximum,|control|producer.decode(control)).map_err(PackRefusal::from)}
    fn charge(&self,bytes:u64)->Result<(),PackRefusal>{self.check(bytes)?;self.control.borrow_mut().charge(usize::try_from(bytes).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "native allocation exceeds address space" })?).map_err(PackRefusal::from)}
    fn step(&self)->Result<(),PackRefusal>{self.control.borrow_mut().step().map_err(PackRefusal::from)}
    fn controlled(&self)->bool{true}
    fn copy_text(&self,text:&str)->Result<String,PackRefusal>{self.check(text.len() as u64)?;self.control.borrow_mut().copy_text(text).map_err(PackRefusal::from)}
    fn copy_bytes(&self,bytes:&[u8])->Result<Vec<u8>,PackRefusal>{self.check(bytes.len() as u64)?;self.control.borrow_mut().copy_bytes(bytes).map_err(PackRefusal::from)}
    fn octet_buffer(&self,count:usize)->Result<Vec<u8>,PackRefusal>{self.check(count as u64)?;self.control.borrow_mut().allocate_vec(count).map_err(PackRefusal::from)}
    fn append_chunk(&self,file:&semio_framework_pack_format::PackFile<&[u8]>,id:ChunkId,verification:semio_framework_pack_format::VerificationLevel,output:&mut Vec<u8>)->Result<(),PackRefusal>{self.control.borrow_mut().scoped_stage(|control|::semio_framework_async::poll::resolve_ready(file.append_chunk_controlled(id,verification,output,control)))}
    fn expression(&self,text:&str,maximum_depth:u16)->Result<semio_framework_dsl_record::ExprValue,PackRefusal>{let maximum=usize::try_from(self.maximum).unwrap_or(usize::MAX);let mut limits=semio_framework_diagnostic::Limits::default();limits.max_depth=usize::from(maximum_depth);self.control.borrow_mut().scoped_maximum(maximum,|control|Ok::<_,ValueError>(semio_framework_dsl_record::parse_expr_text_controlled(text,&limits,control).map_err(PackRefusal::from))).map_err(PackRefusal::from)?}
    fn copy_utf8(&self,bytes:&[u8],_offset:u64,_what:&'static str)->Result<String,PackRefusal>{self.check(bytes.len() as u64)?;let mut control=self.control.borrow_mut();if bytes.len()>control.maximum_bytes().saturating_sub(control.owned_bytes()){return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text exceeds caller allowance")));}let value=control.borrow_text(bytes).map_err(PackRefusal::from)?;control.copy_text(value).map_err(PackRefusal::from)}
    fn copy_span_utf8(&self,bytes:semio_framework_replication_codec::ByteSpan<'_>,offset:u64,what:&'static str)->Result<String,PackRefusal>{
        if let Some(bytes)=bytes.contiguous(){return self.copy_utf8(bytes,offset,what)}
        self.check(bytes.len()as u64)?;
        {let control=self.control.borrow();if bytes.len()>control.maximum_bytes().saturating_sub(control.owned_bytes()){return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"native text exceeds caller allowance")))}}
        copy_validated_span_utf8(self,bytes,offset,what)
    }
}

fn copy_decoded_string(value:&str,materialization:&dyn MaterializationAdmission)->Result<String,PackRefusal>{materialization.copy_text(value)}

impl DecCtx<'_> {
    fn check_items(&self,n:u64)->Result<(),PackRefusal>{if n>self.limits.max_items{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "item count exceeds max_items" });}Ok(())}
    fn value_slots<T>(&self,count:u64)->Result<Vec<T>,PackRefusal>{self.check_items(count)?;self.materialization.charge(count.checked_mul(size_of::<T>() as u64).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire value slot overflow" })?)?;let mut output=Vec::new();output.try_reserve_exact(usize::try_from(count).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire value slot count" })?).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "wire value slot allocation" })?;Ok(output)}
    fn record_slots(&self,record:&mut RecordValue,count:u64)->Result<(),PackRefusal>{self.check_items(count)?;let count=usize::try_from(count).map_err(|_|PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"record slot count exceeds address space")))?.max(record.fields.len());if count<=record.fields.capacity(){return Ok(());}let bytes=count.checked_mul(size_of::<(u16,FieldValue)>()).ok_or_else(||PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"record slot storage overflow")))?;self.materialization.charge(bytes as u64)?;let mut entries=Vec::new();entries.try_reserve_exact(count).map_err(|_|PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::AllocationFailed,"record slot allocation failed")))?;record.fields.replace_empty_slots(entries);Ok(())}
}

fn resolve_symref(ctx: &DecCtx<'_>, symref: u64) -> Result<String, PackRefusal> {
    let value = match &ctx.source {
        DecSource::File(pack_file) => pack_file.symbol(symref)?,
        DecSource::Inline { symbols } => symbols.get(usize::try_from(symref).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvalidValue, limit: "symbol reference index" })?).map(String::as_str).ok_or_else(|| PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue,
            what: "symref",
            offset: 0,
            detail: format!("symref {symref} out of range for inline table of {}", symbols.len()),
        })?,
    };
    copy_decoded_string(value, ctx.materialization)
}

/// 📏️ Reads a `varint` length then that many raw bytes, rejecting an oversized length
/// against `limits.max_segment_len` BEFORE allocating/slicing.
fn read_len_prefixed_bytes<'b>(reader: &mut ByteReader<'b>, limits: &PackLimits) -> Result<semio_framework_replication_codec::ByteSpan<'b>, PackRefusal> {
    let len = reader.read_varint_u64()?;
    if len > limits.max_segment_len {
        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "inline blob length exceeds max_segment_len" });
    }
    reader.read_span(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "inline blob byte length" })?)
}

fn read_inline_string(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<String, PackRefusal> {
    let bytes = read_len_prefixed_bytes(reader, &ctx.limits)?;
    // 🔁️ `reader.position()` is async now; `map_err`'s closure is sync (R10 residue shape 1), so
    // the position is read up front rather than awaited inside the closure.
    let offset = reader.position() as u64;
    ctx.materialization.copy_span_utf8(bytes,offset,"text")
}

fn read_inline_bytes(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<Vec<u8>, PackRefusal> {
    let bytes=read_len_prefixed_bytes(reader,&ctx.limits)?;
    if bytes.len()as u64>ctx.limits.max_total_alloc{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octets exceed max_total_alloc" });}
    ctx.materialization.copy_span_bytes(bytes)
}

/// 🧱️ Reads `count` chunk ids and concatenates their decoded (and, per `verification`,
/// integrity-checked) content via the open `PackFile`'s chunk table.
fn read_chunked_bytes(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<Vec<u8>, PackRefusal> {
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    if ctx.materialization.controlled(){
        let DecSource::File(file)=&ctx.source else{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::UnsupportedOwner,what:"chunk_id",offset:reader.position() as u64,detail:"chunked octets require an owned chunk catalog".into()});};let mut sizes=reader.fork();let mut length=0u64;
        for _ in 0..count{ctx.materialization.step()?;let raw_id=sizes.read_varint_u64()?;let id=u32::try_from(raw_id).map_err(|_|PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"chunk_id",offset:sizes.position() as u64,detail:"chunk id exceeds u32".into()})?;let bytes=file.chunk_decoded_len(ChunkId(id))?;if bytes>ctx.limits.max_segment_len{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "chunk length exceeds max_segment_len" });}length=length.checked_add(bytes).filter(|length|*length<=ctx.limits.max_total_alloc).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "controlled octet length exceeds caller limits" })?;}
        let length=usize::try_from(length).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "chunk octets exceed address space" })?;let mut output=ctx.materialization.octet_buffer(length)?;
        for _ in 0..count{ctx.materialization.step()?;let id=u32::try_from(reader.read_varint_u64()?).map_err(|_|PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"chunk_id",offset:reader.position() as u64,detail:"chunk id exceeds u32".into()})?;ctx.materialization.append_chunk(file,ChunkId(id),ctx.verification,&mut output)?;}
        if output.len()!=length{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvariantViolated,what:"chunks",offset:reader.position() as u64,detail:"chunk output differs from admitted length".into()});}return Ok(output);
    }
    let mut out = Vec::new();
    for _ in 0..count {
        let id = reader.read_varint_u64()?;
        if id > u32::MAX as u64 {
            return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "chunk_id", offset: reader.position() as u64, detail: "chunk id exceeds u32".to_string() });
        }
        let piece = match &ctx.source {
            DecSource::File(pack_file) => {
                let length=pack_file.chunk_decoded_len(ChunkId(id as u32))?;
                let total=(out.len()as u64).checked_add(length).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octet length overflow" })?;
                if total>ctx.limits.max_total_alloc||usize::try_from(total).is_err(){return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octets exceed max_total_alloc" });}
                ctx.materialization.charge(length)?;
                ::semio_framework_async::poll::resolve_ready(pack_file.read_chunk(ChunkId(id as u32),ctx.verification))?
            },
            DecSource::Inline { .. } => {
                return Err(PackRefusal::Malformed { kind: ValueRefusalKind::UnsupportedOwner, what: "chunk_id", offset: reader.position() as u64, detail: "chunked bytes are not representable in a container-less record body".to_string() });
            }
        };
        let next=out.len().checked_add(piece.len()).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octet length overflow" })?;
        if next as u64>ctx.limits.max_total_alloc{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "decoded octets exceed max_total_alloc" });}
        out.try_reserve_exact(piece.len()).map_err(|_|PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "decoded octet allocation" })?;
        out.extend_from_slice(&piece);
    }
    Ok(out)
}

/// 📖️ Reads one self-describing string value (`TAG_STR` or `TAG_STR_INLINE`) — used for
/// `Map`/object keys and `DslValue::String`, where the tag itself (not any external shape) is
/// what disambiguates interned vs inline.
fn decode_string(reader: &mut ByteReader<'_>, ctx: &DecCtx<'_>) -> Result<String, PackRefusal> {
    let tag = reader.read_u8()?;
    match tag {
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            resolve_symref(ctx, idx)
        }
        TAG_STR_INLINE => read_inline_string(reader, ctx),
        other => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "string", offset: reader.position() as u64, detail: format!("expected a string tag, found {other:#04x}") }),
    }
}

/// 🧾️ Decodes one record's fields: `field_count, (field_id, value)*`. Any field id not
/// found in `spec` is decoded generically (`shape = None`) and reported into
/// `ctx.unknown_field_ids`; when `ctx.preserve_unknown` is `false` it is still consumed (to stay
/// byte-aligned) but dropped from the returned `RecordValue`. Every `spec` field not seen on the
/// wire is inserted as `Absent` — the decode-side half of canonical mode's "omit `Absent`" rule.
fn decode_record_fields(reader: &mut ByteReader<'_>, spec: Option<&RecordSpec>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<RecordValue, PackRefusal> {
    ctx.materialization.step()?;
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut record = RecordValue::default();
    ctx.record_slots(&mut record,count)?;
    for _ in 0..count {
        let id_raw = reader.read_varint_u64()?;
        if id_raw > u16::MAX as u64 {
            return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "field_id", offset: reader.position() as u64, detail: "field id exceeds u16".to_string() });
        }
        let id = id_raw as u16;
        let field_shape = spec.and_then(|s| s.fields.iter().find(|f| f.id == id)).map(|f| &f.shape);
        let value = decode_value(reader, field_shape, ctx, depth + 1)?;
        if field_shape.is_none() {
            ctx.materialization.charge(8)?;
            ctx.unknown_field_ids.try_reserve_exact(1).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "unknown field report allocation" })?;
            ctx.unknown_field_ids.push(id);
            if ctx.preserve_unknown {
                record.fields.insert(id, value);
            }
        } else {
            record.fields.insert(id, value);
        }
    }
    if let Some(spec) = spec {
        let missing=spec.fields.iter().filter(|field|!record.fields.contains_key(&field.id)).count();
        let count=record.fields.len().checked_add(missing).ok_or_else(||PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"native record default storage overflow")))?;
        ctx.record_slots(&mut record,count as u64)?;
        for field in &spec.fields {
            if !record.fields.contains_key(&field.id){record.fields.insert(field.id,FieldValue::Absent);}
        }
    }
    Ok(record)
}

/// 📖️ Decodes one tag-prefixed value. `shape`, when known, disambiguates `Tuple` vs
/// `List`, resolves nested `Record`/`Block`/`Statements`/`Map` sub-shapes, and reinterprets
/// `PackedVarint` payloads as `UInt`/`Enum` where the shape says so; `None` decodes generically
/// straight from the wire tag — every tag is self-describing enough for this to always succeed,
/// which is what makes unknown-field decode possible without the original schema.
// 🔁️ Mutually recursive with `decode_record_fields`/`decode_seq_body`/`decode_map` (and directly
// self-recursive for `TAG_BLOCK`) — same `Box::pin(...).await` requirement as `encode_value`.
fn decode_value(reader: &mut ByteReader<'_>, shape: Option<&Shape>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    ctx.materialization.step()?;
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
            let nested_spec = match shape{Some(Shape::Record(producer))=>Some(ctx.materialization.schema(producer)?),_=>None};
            Ok(FieldValue::Record(decode_record_fields(reader, nested_spec.as_ref(), ctx, depth + 1)?))
        }
        TAG_BLOCK => {
            ctx.materialization.charge(size_of::<FieldValue>() as u64)?;
            Ok(FieldValue::Block(Box::new(decode_value(reader, block_inner_shape(shape), ctx, depth + 1)?)))
        },
        TAG_STATEMENTS => decode_statements(reader, statements_variants(shape), ctx, depth),
        TAG_MAP => decode_map(reader, map_inner_shape(shape), ctx, depth),
        TAG_VALUE => Ok(FieldValue::Value(decode_dsl_value(reader, ctx, depth + 1)?)),
        TAG_WIRE => Ok(FieldValue::Wire(decode_wire(reader, ctx, depth + 1)?)),
        TAG_EXPR => {
            let text = decode_string(reader, ctx)?;
            // 🔁️ Same closure constraint as `read_inline_string`: read the position before the
            // sync `map_err` closure, don't `.await` inside it.
            let _offset = reader.position() as u64;
            ctx.materialization.expression(&text,ctx.limits.max_depth).map(FieldValue::Expr)
        }
        TAG_TABLE_SOA => Ok(FieldValue::List(decode_table_soa(reader, table_spec_of(shape), ctx, depth)?)),
        TAG_PACKED_F64 => decode_packed_f64_body(reader, is_tuple_shape(shape), ctx),
        TAG_PACKED_VARINT => decode_packed_varint_body(reader, elem_shape_of(shape).or(shape.filter(|s| !matches!(s, Shape::Tuple(_, _)))), is_tuple_shape(shape), ctx),
        TAG_NULL => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire_tag", offset: reader.position() as u64, detail: "TAG_NULL is only valid inside a DslValue".to_string() }),
        other => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire_tag", offset: reader.position() as u64, detail: format!("unrecognized tag {other:#04x}") }),
    }
}

/// 📚️ Decodes a plain (non-packed) `Tuple`/`List` body: `count, values*`.
fn decode_seq_body(reader: &mut ByteReader<'_>, elem_shape: Option<&Shape>, is_tuple: bool, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        items.push(decode_value(reader, elem_shape, ctx, depth + 1)?);
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

fn decode_packed_f64_body(reader: &mut ByteReader<'_>, is_tuple: bool, ctx: &DecCtx<'_>) -> Result<FieldValue, PackRefusal> {
    let count = reader.read_varint_u64()?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        ctx.materialization.step()?;
        items.push(FieldValue::Float(reader.read_f64_le()?));
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

/// 🔢️ Decodes a `PackedVarint` body. `elem_shape` (the field's `List(UInt)`/`List(Enum)`/
/// `Tuple(..)` element shape, when known) picks the reconstruction type; unknown context always
/// defaults to `Int`, which is also what makes an unknown field's homogeneous-`Int` list
/// re-encode to the exact same bytes (round-trip preserved even without the original schema).
fn decode_packed_varint_body(reader: &mut ByteReader<'_>, elem_shape: Option<&Shape>, is_tuple: bool, ctx: &DecCtx<'_>) -> Result<FieldValue, PackRefusal> {
    let count = reader.read_varint_u64()?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        ctx.materialization.step()?;
        let v = reader.read_varint_i64()?;
        let fv = match elem_shape {
            Some(Shape::UInt) => {
                if v < 0 {
                    return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "packed_varint", offset: reader.position() as u64, detail: "negative value under UInt shape".to_string() });
                }
                FieldValue::UInt(v as u64)
            }
            Some(Shape::Enum(_)) => {
                if v < 0 {
                    return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "packed_varint", offset: reader.position() as u64, detail: "negative value under Enum shape".to_string() });
                }
                FieldValue::Enum(v as u32)
            }
            _ => FieldValue::Int(v),
        };
        items.push(fv);
    }
    Ok(if is_tuple { FieldValue::Tuple(items) } else { FieldValue::List(items) })
}

fn decode_map(reader: &mut ByteReader<'_>, inner_shape: Option<&Shape>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut entries = ctx.value_slots(count)?;
    for _ in 0..count {
        let key = decode_string(reader, ctx)?;
        let value = decode_value(reader, inner_shape, ctx, depth + 1)?;
        entries.push((key, value));
    }
    Ok(FieldValue::Map(entries))
}

fn decode_statements(reader: &mut ByteReader<'_>, variants: Option<&Vec<(String, RecordSpecProducer)>>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<FieldValue, PackRefusal> {
    check_depth(ctx.limits.max_depth, depth)?;
    let count = reader.read_varint_u64()?;
    ctx.check_items(count)?;
    let mut items = ctx.value_slots(count)?;
    for _ in 0..count {
        let symref = reader.read_varint_u64()?;
        let keyword = resolve_symref(ctx, symref)?;
        let spec = variants.and_then(|vs| vs.iter().find(|(k, _)| *k == keyword)).map(|(_, f)| ctx.materialization.schema(f)).transpose()?;
        let record = decode_record_fields(reader, spec.as_ref(), ctx, depth + 1)?;
        items.push((keyword, record));
    }
    Ok(FieldValue::Statements(items))
}

// 🔁️ Self-recursive (`TAG_LIST`/`TAG_MAP` arms) — boxed for the same reason as `decode_value`.
fn decode_dsl_value(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<DslValue, PackRefusal> {
    ctx.materialization.step()?;
    check_depth(ctx.limits.max_depth, depth)?;
    let tag = reader.read_u8()?;
    match tag {
        TAG_NULL => Ok(DslValue::Null),
        TAG_FALSE => Ok(DslValue::Bool(false)),
        TAG_TRUE => Ok(DslValue::Bool(true)),
        TAG_UINT => Ok(DslValue::Number(Number::UInt(reader.read_varint_u64()?))),
        TAG_INT => Ok(DslValue::Number(Number::Int(reader.read_varint_i64()?))),
        TAG_F64 => Ok(DslValue::Number(Number::Float(reader.read_f64_le()?))),
        TAG_STR => {
            let idx = reader.read_varint_u64()?;
            Ok(DslValue::String(resolve_symref(ctx, idx)?))
        }
        TAG_STR_INLINE => Ok(DslValue::String(read_inline_string(reader, ctx)?)),
        TAG_BYTES=>Ok(DslValue::Bytes(read_inline_bytes(reader,ctx)?)),
        TAG_BYTES_CHUNKED=>Ok(DslValue::Bytes(read_chunked_bytes(reader,ctx)?)),
        TAG_LIST => {
            let count = reader.read_varint_u64()?;
            let mut items = ctx.value_slots(count)?;
            for _ in 0..count {
                items.push(decode_dsl_value(reader, ctx, depth + 1)?);
            }
            Ok(DslValue::Array(items))
        }
        TAG_MAP => {
            let count = reader.read_varint_u64()?;
            let mut entries = ctx.value_slots(count)?;
            for _ in 0..count {
                let key = decode_string(reader, ctx)?;
                let value = decode_dsl_value(reader, ctx, depth + 1)?;
                entries.push((key, value));
            }
            Ok(DslValue::Object(entries))
        }
        other => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "dsl_value", offset: reader.position() as u64, detail: format!("unexpected tag {other:#04x}") }),
    }
}

fn decode_wire(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<WireValue, PackRefusal> {
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

fn decode_wire_node(reader: &mut ByteReader<'_>, ctx: &mut DecCtx<'_>) -> Result<WireNode, PackRefusal> {
    ctx.materialization.step()?;
    let presence = reader.read_u8()?;
    let id = decode_string(reader, ctx)?;
    let kind = if presence & 0b01 != 0 { Some(decode_string(reader, ctx)?) } else { None };
    let port = if presence & 0b10 != 0 { Some(decode_string(reader, ctx)?) } else { None };
    Ok(WireNode { id, kind, port })
}
//#endregion 🔖️Decode

//#region 🔖️Table

/// 📊️ Encodes `Shape::Table`'s `List(Record)` value as columnar `TableSoA`: `row_count,
/// col_count`, then per column (sorted by field id) `field_id, presence (0=dense/1=sparse+bitmap),
/// elem_tag, packed payload`. Fixed-width/varint columns write only present-row values
/// (compacted, in row order); `Bool` columns instead write one ceil(rows/8)-byte row-aligned value
/// bitmap unconditionally (simpler than compacting individual bits). `Text` columns are always
/// interned (forced symrefs, matching `build_symbols`'s pre-pass); every other shape falls back to
/// self-describing per-present-row values.
fn encode_table(ctx: &mut EncCtx<'_>, spec_fn: RecordSpecProducer, items: &[FieldValue], depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
    check_depth(ctx.options.limits.max_depth, depth)?;
    let element_spec = (spec_fn.ordinary)();
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
                            out.extend_from_slice(&f.to_le_bytes());
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

/// 📖️ Decodes `TableSoA` fully self-describing — `field_id`/`presence`/`elem_tag` are
/// stored per column on the wire, so no `RecordSpec` is ever required to reconstruct the rows
/// (this is what lets an unknown `Table`-shaped field still round-trip). When the caller DOES
/// know the table's element `RecordSpec` (`spec_fn` is `Some`), it is threaded into the
/// fallback (non-primitive) column branch so a nested `Record` column's own `Absent` sub-fields
/// get backfilled correctly instead of merely reflecting what was present on the wire.
fn decode_table_soa(reader: &mut ByteReader<'_>, spec_fn: Option<RecordSpecProducer>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<Vec<FieldValue>, PackRefusal> {
    check_depth(ctx.limits.max_depth, depth)?;
    let element_spec = spec_fn.map(|producer|ctx.materialization.schema(&producer)).transpose()?;
    let row_count_raw = reader.read_varint_u64()?;
    ctx.check_items(row_count_raw)?;
    let col_count = reader.read_varint_u64()?;
    ctx.check_items(col_count)?;
    let row_count = usize::try_from(row_count_raw).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "table row count" })?;

    let mut values = ctx.value_slots(row_count_raw)?;
    let mut rows: Vec<RecordValue> = ctx.value_slots(row_count_raw)?;
    for _ in 0..row_count {
        let mut row = RecordValue::default();
        ctx.record_slots(&mut row,col_count)?;
        rows.push(row);
    }
    for _ in 0..col_count {
        let field_id = reader.read_varint_u64()? as u16;
        let presence = reader.read_u8()?;
        let dense = presence == 0;
        let bitmap = if dense { None } else { Some(reader.read_span(row_count.div_ceil(8))?) };
        let present = |index: usize| bitmap.is_none_or(|bitmap| bitmap[index / 8] & (1 << (index % 8)) != 0);
        let elem_tag = reader.read_u8()?;
        match elem_tag {
            ELEM_F64 => {
                for i in 0..row_count {
                    ctx.materialization.step()?;
                    if present(i) {
                        let f = reader.read_f64_le()?;
                        rows[i].fields.insert(field_id, FieldValue::Float(f));
                    }
                }
            }
            ELEM_INT => {
                for i in 0..row_count {
                    ctx.materialization.step()?;
                    if present(i) {
                        let v = reader.read_varint_i64()?;
                        rows[i].fields.insert(field_id, FieldValue::Int(v));
                    }
                }
            }
            ELEM_UINT => {
                for i in 0..row_count {
                    ctx.materialization.step()?;
                    if present(i) {
                        let v = reader.read_varint_u64()?;
                        rows[i].fields.insert(field_id, FieldValue::UInt(v));
                    }
                }
            }
            ELEM_ENUM => {
                for i in 0..row_count {
                    ctx.materialization.step()?;
                    if present(i) {
                        let v = reader.read_varint_u64()?;
                        rows[i].fields.insert(field_id, FieldValue::Enum(v as u32));
                    }
                }
            }
            ELEM_BOOL => {
                let bitmap = reader.read_span(row_count.div_ceil(8))?;
                for i in 0..row_count {
                    ctx.materialization.step()?;
                    if present(i) {
                        let b = bitmap[i / 8] & (1 << (i % 8)) != 0;
                        rows[i].fields.insert(field_id, FieldValue::Bool(b));
                    }
                }
            }
            ELEM_STR => {
                for i in 0..row_count {
                    ctx.materialization.step()?;
                    if present(i) {
                        let symref = reader.read_varint_u64()?;
                        let s = resolve_symref(ctx, symref)?;
                        rows[i].fields.insert(field_id, FieldValue::Text(s));
                    }
                }
            }
            _ => {
                let field_shape = element_spec.as_ref().and_then(|s| s.fields.iter().find(|f| f.id == field_id)).map(|f| &f.shape);
                for i in 0..row_count {
                    ctx.materialization.step()?;
                    if present(i) {
                        let v = decode_value(reader, field_shape, ctx, depth + 1)?;
                        rows[i].fields.insert(field_id, v);
                    }
                }
            }
        }
        for i in 0..row_count {
                    ctx.materialization.step()?;
            if !present(i) {
                if !rows[i].fields.contains_key(&field_id){rows[i].fields.insert(field_id,FieldValue::Absent);}
            }
        }
    }
    values.extend(rows.into_iter().map(FieldValue::Record));
    Ok(values)
}
//#endregion 🔖️Table

//#region 🔖️Document
/// 🎛️ First-party Pack policy is owned by the shared transport contract.
pub use semio_framework_replication_codec::{PackEncodeOptions as EncodeOptions,PackDecodeOptions as DecodeOptions};

/// 🩺️ What [`decode_document`] observed beyond the plain `RecordValue`: field ids present
/// on the wire but absent from the caller's `RecordSpec`, any unrecognized segment kinds,
/// whether the manifest's stored `schema_hash` disagrees with the caller's `spec`, and the
/// verification level actually applied.
#[derive(Clone, Debug)]
pub struct DecodeReport {
    pub unknown_field_ids: Vec<u16>,
    pub unknown_segments: Vec<u8>,
    pub schema_drift: bool,
    pub verified: semio_framework_pack_format::VerificationLevel,
}

/// 🚪️ The single entry point every other `pack_*`/`vcs`/`dsl_derive` crate encodes a
/// `RecordValue` through. Pre-pass computes the deterministic symbol table, then writes
/// `Symbols`, one-or-more `Document` frames (split at `options.frame_size`), any `Bytes64` chunks
/// produced along the way, and finally the `Manifest`/`End`/`Footer` via `PackWriter::finish`.
pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackRefusal> {
    let symbols = build_symbols(spec, record);
    let mut symbol_index = HashMap::with_capacity(symbols.len());
    for (i, s) in symbols.iter().enumerate() {
        symbol_index.insert(s.clone(), i as u64);
    }

    let write_options = semio_framework_pack_format::WriteOptions { required_flags: 0, optional_flags: if options.canonical { semio_framework_pack_format::OPTIONAL_CANONICAL } else { 0 }, codec: options.codec };
    let mut writer = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackWriter::begin(Vec::new(), &write_options))?;

    let symbols_payload = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::encode_symbols(&symbols));
    ::semio_framework_async::poll::resolve_ready(writer.write_segment(semio_framework_replication_codec::KIND_SYMBOLS, &symbols_payload))?;

    let field_count = record.fields.values().filter(|v| !matches!(v, FieldValue::Absent)).count() as u64;
    let doc_payload = {
        let mut enc_ctx = EncCtx { symbol_index, writer: &mut writer, options };
        encode_record_fields(&mut enc_ctx, Some(spec), record, 0)?
    };

    let frame_size = options.frame_size.max(1) as usize;
    let doc_start = ::semio_framework_async::poll::resolve_ready(writer.position());
    let mut frame_count: u64 = 0;
    for frame in doc_payload.chunks(frame_size) {
        ::semio_framework_async::poll::resolve_ready(writer.write_segment(semio_framework_replication_codec::KIND_DOCUMENT, frame))?;
        frame_count += 1;
    }
    let doc_end = ::semio_framework_async::poll::resolve_ready(writer.position());

    let manifest = semio_framework_pack_format::Manifest {
        schema_name: String::new(),
        schema_hash: schema_hash(spec),
        doc_span: semio_framework_replication_codec::ByteRange { offset: doc_start, len: doc_end - doc_start },
        doc_frame_count: frame_count,
        symbols_span: semio_framework_replication_codec::ByteRange { offset: 0, len: 0 },
        chunk_table_span: semio_framework_replication_codec::ByteRange { offset: 0, len: 0 },
        field_index_span: semio_framework_replication_codec::ByteRange { offset: 0, len: 0 },
        uncompressed_body_len: doc_payload.len() as u64,
        field_count,
        chunk_count: 0,
        symbol_count: symbols.len() as u64,
    };
    ::semio_framework_async::poll::resolve_ready(writer.finish(&manifest))
}

/// 🚪️ The single entry point every other `pack_*`/`vcs`/`dsl_derive` crate decodes a
/// `RecordValue` through. Opens the pack file at manifest level, reads and concatenates the
/// `Document` frame(s), then decodes the top-level record body against `spec` — self-describing
/// enough that any field id `spec` doesn't recognize still decodes and is preserved (subject to
/// `options.preserve_unknown`) and reported.
pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<(RecordValue, DecodeReport), PackRefusal> {
    let pack_file = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackFile::open_manifest(bytes, &options.limits, options.verification))?;
    let manifest = pack_file.manifest().ok_or_else(|| PackRefusal::from(ValueError::new(ValueRefusalKind::InvariantViolated,"manifest not loaded")))?;
    let schema_drift = manifest.schema_hash != schema_hash(spec);
    let body = ::semio_framework_async::poll::resolve_ready(pack_file.body_bytes(options.verification))?;

    let mut reader = ByteReader::new(&body);
    let retained=pack_file.owned_catalog_bytes()?.checked_add(body.capacity() as u64).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "document materialization allocation overflow" })?;
    if retained>options.limits.max_total_alloc{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "document materialization exceeds max_total_alloc" });}
    let budget=ValueMaterialization{used:std::cell::Cell::new(retained),maximum:options.limits.max_total_alloc};
    let mut dec_ctx = DecCtx { source: DecSource::File(&pack_file), limits: options.limits.clone(), verification: options.verification, preserve_unknown: options.preserve_unknown, unknown_field_ids: Vec::new(), materialization: &budget };
    let record = decode_record_fields(&mut reader, Some(spec), &mut dec_ctx, 0)?;

    let report = DecodeReport { unknown_field_ids: dec_ctx.unknown_field_ids, unknown_segments: Vec::new(), schema_drift, verified: options.verification };
    Ok((record, report))
}

/// 🛬️ Controls catalog/source/inflation/primitive ownership using the caller's typed-construction budget.
pub fn decode_document_controlled(bytes:&[u8],spec:&RecordSpec,options:&DecodeOptions,control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<(RecordValue,DecodeReport),PackRefusal>{
    let file=::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackFile::open_manifest_controlled(bytes,&options.limits,options.verification,control))?;
    let manifest=file.manifest().ok_or_else(||PackRefusal::from(ValueError::new(ValueRefusalKind::InvariantViolated,"controlled manifest absent")))?;let schema_drift=manifest.schema_hash!=schema_hash_controlled(spec,control)?;
    let body=::semio_framework_async::poll::resolve_ready(file.body_bytes_controlled(options.verification,control))?;
    control.begin_stage(0).map_err(PackRefusal::from)?;let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};
    let mut context=DecCtx{source:DecSource::File(&file),limits:options.limits.clone(),verification:options.verification,preserve_unknown:options.preserve_unknown,unknown_field_ids:Vec::new(),materialization:&materialization};let mut reader=ByteReader::new(&body);
    let record=decode_record_fields(&mut reader,Some(spec),&mut context,0)?;
    if reader.remaining()!=0{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"document",offset:reader.position() as u64,detail:"trailing document body bytes".into()});}
    Ok((record,DecodeReport{unknown_field_ids:context.unknown_field_ids,unknown_segments:Vec::new(),schema_drift,verified:options.verification}))
}

/// 🎯️ Container-less twin of [`encode_document`] for small payloads (operation/command
/// records): `symbol_count varint, (len varint, utf8)*, record fields` — no header, segments,
/// manifest, or footer, and never any `Bytes64` chunking (oversized bytes stay inline via
/// `TAG_BYTES`). Deterministic by the same purity rules as the document path: byte-identical
/// output for equal `(spec, record)` regardless of map iteration order.
pub fn encode_record_body(spec: &RecordSpec, record: &RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackRefusal> {
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
    let write_options = semio_framework_pack_format::WriteOptions { required_flags: 0, optional_flags: 0, codec: CodecId(0) };
    let mut writer = ::semio_framework_async::poll::resolve_ready(semio_framework_pack_format::PackWriter::begin(Vec::new(), &write_options))?;
    let fields = {
        let mut enc_ctx = EncCtx { symbol_index, writer: &mut writer, options: &body_options };
        encode_record_fields(&mut enc_ctx, Some(spec), record, 0)?
    };
    out.extend_from_slice(&fields);
    Ok(out)
}

/// 🎯️ Decodes an [`encode_record_body`] payload against `spec`. Unknown fields decode,
/// are preserved (subject to `options.preserve_unknown`), and are reported exactly like the
/// document path; a `TAG_BYTES_CHUNKED` value is malformed here by construction.
pub fn decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<(RecordValue, DecodeReport), PackRefusal> {
    decode_record_body_inner(bytes, spec, options, false)
}

/// 🔒️ Decodes exactly one terminal record body, rejecting unknown fields and trailing bytes.
pub fn decode_record_body_exact(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions) -> Result<RecordValue, PackRefusal> {
    decode_record_body_inner(bytes, spec, options, true).map(|(record, _)| record)
}

/// 🛬️ Parses a terminal container-less record under the same control used by its typed constructor.
pub fn decode_record_body_exact_controlled(bytes:&[u8],spec:&RecordSpec,options:&DecodeOptions,control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<RecordValue,PackRefusal>{
    if bytes.len() as u64>options.limits.max_file_len{return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record body exceeds max_file_len" });}
    control.begin_stage(0).map_err(PackRefusal::from)?;
    let mut reader=ByteReader::new(bytes);
    let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};
    let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;
    let mut ctx=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};
    let record=decode_record_fields(&mut reader,Some(spec),&mut ctx,0)?;
    if reader.position()!=bytes.len(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position() as u64,detail:"trailing bytes after terminal record".into()});}
    if !ctx.unknown_field_ids.is_empty(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position() as u64,detail:"unknown terminal record fields".into()});}
    Ok(record)
}

/// 📖️ Decodes one terminal Record directly from a checked source range under the caller's control.
pub fn decode_record_body_span_exact_controlled(bytes:semio_framework_replication_codec::ByteSpan<'_>,spec:&RecordSpec,options:&DecodeOptions,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RecordValue,PackRefusal>{
    if bytes.len()as u64>options.limits.max_file_len{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"record body exceeds max_file_len"})}
    control.begin_stage(0).map_err(PackRefusal::from)?;
    let mut reader=ByteReader::from_span(bytes);
    let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};
    let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;
    let mut context=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};
    let record=decode_record_fields(&mut reader,Some(spec),&mut context,0)?;
    if reader.position()!=bytes.len(){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position()as u64,detail:"trailing bytes after terminal record"})}
    if !context.unknown_field_ids.is_empty(){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position()as u64,detail:"unknown terminal record fields"})}
    Ok(record)
}

fn decode_inline_symbols(reader: &mut ByteReader<'_>, limits: &PackLimits, materialization: &dyn MaterializationAdmission) -> Result<Vec<String>, PackRefusal> {
    let symbol_count = reader.read_varint_u64()?;
    if symbol_count > u64::from(limits.max_symbols) {
        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record-body symbol count exceeds max_symbols" });
    }
    materialization.charge(symbol_count.checked_mul(size_of::<String>() as u64).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol slot overflow" })?)?;
    let count = usize::try_from(symbol_count).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol slot count" })?;
    let mut symbols = Vec::new();
    symbols.try_reserve_exact(count).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::AllocationFailed, limit: "wire symbol slot allocation" })?;
    for _ in 0..symbol_count {
        materialization.step()?;
        let len = reader.read_varint_u64()?;
        if len > limits.max_segment_len {
            return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record-body symbol length exceeds max_segment_len" });
        }
        let raw = reader.read_span(usize::try_from(len).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "wire symbol byte length" })?)?;
        let symbol_offset = reader.position() as u64;
        symbols.push(materialization.copy_span_utf8(raw,symbol_offset,"symbol")?);
    }
    Ok(symbols)
}

/// 🛡️ Decodes one exact intrinsic field with cumulative concrete storage admission.
/// Owned UTF-8 and typed symbol, list and member slots are admitted before allocation.
pub fn decode_value_record_body_exact(bytes: &[u8], field_id: u16, limits: &PackLimits) -> Result<DslValue, PackRefusal> {
    if bytes.len() as u64 > limits.max_file_len {
        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "wire value exceeds max_file_len" });
    }
    let mut reader = ByteReader::new(bytes);
    let budget = ValueMaterialization { used: std::cell::Cell::new(0), maximum: limits.max_total_alloc };
    let symbols = decode_inline_symbols(&mut reader, limits, &budget)?;
    if reader.read_varint_u64()? != 1 || reader.read_varint_u64()? != u64::from(field_id) || reader.read_u8()? != TAG_VALUE {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire value", offset: reader.position() as u64, detail: "expected exactly one declared Value field".into() });
    }
    let mut ctx = DecCtx { source: DecSource::Inline { symbols }, limits: limits.clone(), verification: semio_framework_pack_format::VerificationLevel::Standard, preserve_unknown: false, unknown_field_ids: Vec::new(), materialization: &budget };
    let value = decode_dsl_value(&mut reader, &mut ctx, 0)?;
    if reader.position() != bytes.len() {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "wire value", offset: reader.position() as u64, detail: "trailing bytes after the terminal value".into() });
    }
    Ok(value)
}

/// 🎞️ Decodes one terminal intrinsic field under cumulative ownership and cancellation.
pub fn decode_value_record_body_exact_controlled(bytes:&[u8],field_id:u16,options:&DecodeOptions,control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    if bytes.len() as u64>options.limits.max_file_len{return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::WorkLimit,"intrinsic wire exceeds max_file_len")));}
    control.scoped_stage(|control|{control.begin_stage(0).map_err(PackRefusal::ValueRefusal)?;let mut reader=ByteReader::new(bytes);let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;if reader.read_varint_u64()?!=1||reader.read_varint_u64()?!=u64::from(field_id)||reader.read_u8()?!=TAG_VALUE{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"expected exactly one declared Value field".into()});}let mut ctx=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};let value=decode_dsl_value(&mut reader,&mut ctx,0)?;if reader.position()!=bytes.len(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"trailing bytes after terminal intrinsic value".into()});}materialization.control.borrow_mut().checkpoint().map_err(PackRefusal::ValueRefusal)?;Ok(value)})
}

/// 🎞️ Decodes one exact intrinsic body by borrowing the caller's complete immutable byte source.
pub fn decode_value_record_body_span_exact_controlled(bytes:semio_framework_replication_codec::ByteSpan<'_>,field_id:u16,options:&DecodeOptions,control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    if bytes.len() as u64>options.limits.max_file_len{return Err(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::WorkLimit,"intrinsic wire exceeds max_file_len")));}
    control.scoped_stage(|control|{control.begin_stage(0).map_err(PackRefusal::ValueRefusal)?;let mut reader=ByteReader::from_span(bytes);let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;if reader.read_varint_u64()?!=1||reader.read_varint_u64()?!=u64::from(field_id)||reader.read_u8()?!=TAG_VALUE{return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"expected exactly one declared Value field".into()});}let mut ctx=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};let value=decode_dsl_value(&mut reader,&mut ctx,0)?;if reader.position()!=bytes.len(){return Err(PackRefusal::Malformed{ kind: ValueRefusalKind::InvalidValue,what:"intrinsic wire",offset:reader.position() as u64,detail:"trailing bytes after terminal intrinsic value".into()});}materialization.control.borrow_mut().checkpoint().map_err(PackRefusal::ValueRefusal)?;Ok(value)})
}

fn decode_record_body_inner(bytes: &[u8], spec: &RecordSpec, options: &DecodeOptions, exact: bool) -> Result<(RecordValue, DecodeReport), PackRefusal> {
    if bytes.len() as u64 > options.limits.max_file_len { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "record body exceeds max_file_len" }); }
    let mut reader = ByteReader::new(bytes);
    let budget = ValueMaterialization { used: std::cell::Cell::new(0), maximum: options.limits.max_total_alloc };
    let symbols = decode_inline_symbols(&mut reader, &options.limits, &budget)?;
    let mut dec_ctx = DecCtx { source: DecSource::Inline { symbols }, limits: options.limits.clone(), verification: options.verification, preserve_unknown: options.preserve_unknown, unknown_field_ids: Vec::new(), materialization: &budget };
    let record = decode_record_fields(&mut reader, Some(spec), &mut dec_ctx, 0)?;
    if exact && reader.position() != bytes.len() {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "record body", offset: reader.position() as u64, detail: "trailing bytes after the terminal record".into() });
    }
    if exact && !dec_ctx.unknown_field_ids.is_empty() {
        return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "record body", offset: reader.position() as u64, detail: format!("{} unknown terminal record fields, first {}", dec_ctx.unknown_field_ids.len(), dec_ctx.unknown_field_ids[0]) });
    }
    let report = DecodeReport { unknown_field_ids: dec_ctx.unknown_field_ids, unknown_segments: Vec::new(), schema_drift: false, verified: options.verification };
    Ok((record, report))
}
//#endregion 🔖️Document

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

#[path = "🔎️scalar-witness/🦀️.rs"]
pub mod scalar_witness;

#[path="🌳️intrinsic/🦀️.rs"]
pub mod intrinsic;

