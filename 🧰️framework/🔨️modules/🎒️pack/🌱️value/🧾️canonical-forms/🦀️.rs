//! 🧾️ Canonical-mode helpers: record specs, depth limits, packed-numeric eligibility, the encode context and the deterministic string-interning pre-pass.

use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldValue, RecordSpec, RecordValue, Shape};
use semio_framework_replication_codec::write_varint_u64;
use std::collections::{HashMap, HashSet};
use semio_framework_pack_record_schema::{NumKind, TAG_STR, TAG_STR_INLINE, block_inner_shape, elem_shape_of, map_inner_shape, statements_variants, table_spec_of};
use super::EncodeOptions;

//#region 🔖️Canonical

/// 🧾️ Resolves a `Record` field's nested spec, if `shape` is `Record`.
pub(super) fn record_spec_of(shape: Option<&Shape>) -> Option<RecordSpec> {
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
pub(super) fn is_tuple_shape(shape: Option<&Shape>) -> bool {
    matches!(shape, Some(Shape::Tuple(_, _)) | Some(Shape::Coord(_)) | Some(Shape::Dir) | Some(Shape::Dim(_)) | Some(Shape::Range))
}

/// 🛡️ Depth-limit check shared by every recursive encode/decode entry point.
pub(super) fn check_depth(max_depth: u16, depth: u16) -> Result<(), PackRefusal> {
    if depth > max_depth {
        return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"max_depth exceeded"});
    }
    Ok(())
}

/// 🧮️ A sequence is packed-eligible iff every element is the same numeric `FieldValue`
/// variant (`Float`, `Int`, `Enum`) or every element is `UInt` and fits in `i64` (so the zigzag
/// round trip through `PackedVarint` is lossless). Empty sequences are never eligible — there is
/// no element to infer a kind from, so they fall through to the plain `0x0B`/`0x0C` forms.
pub(super) fn homogeneous_numeric_kind(items: &[FieldValue]) -> Option<NumKind> {
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
pub(super) struct EncCtx<'a> {
    pub(super) symbol_index: HashMap<String, u64>,
    pub(super) writer: &'a mut semio_framework_pack_format::PackWriter<Vec<u8>>,
    pub(super) options: &'a EncodeOptions,
}

/// 📖️ Encodes a string using the precomputed interning decision: `TAG_STR` + symref if
/// `s` made it into the symbol table, else `TAG_STR_INLINE` + length-prefixed UTF-8 bytes.
pub(super) fn encode_string(ctx: &mut EncCtx<'_>, s: &str, out: &mut Vec<u8>) {
    if let Some(&idx) = ctx.symbol_index.get(s) {
        out.push(TAG_STR);
        write_varint_u64(out, idx);
    } else {
        encode_string_inline(s, out);
    }
}

/// 📌️ Forces `TAG_STR_INLINE` regardless of the interning decision — the wire rule for
/// `Value`/`DslValue::Object` keys, which are never symrefs.
pub(super) fn encode_string_inline(s: &str, out: &mut Vec<u8>) {
    out.push(TAG_STR_INLINE);
    write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

/// 🔗️ Writes a bare symref varint with NO leading tag — the wire rule for `Statements`
/// keywords and `TableSoA` `Str` columns, both of which are unconditionally interned.
pub(super) fn write_symref_forced(ctx: &mut EncCtx<'_>, s: &str, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
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
pub(super) fn build_symbols(spec: &RecordSpec, record: &RecordValue) -> Vec<String> {
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
