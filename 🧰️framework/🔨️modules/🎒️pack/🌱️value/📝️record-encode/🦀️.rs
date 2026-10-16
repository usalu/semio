//! 📝️ Wire encoding of `FieldValue`, `DslValue` and `Wire` literals: tag-prefixed, self-describing and canonical.

use semio_framework_value::{DslValue, Number};
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldValue, RecordSpec, RecordSpecProducer, RecordValue, Shape, WireNode, WireValue};
use semio_framework_replication_codec::{write_varint_i64, write_varint_u64};
use semio_framework_pack_record_schema::{NumKind, TAG_ABSENT, TAG_BLOCK, TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_EXPR, TAG_F64, TAG_FALSE, TAG_INT, TAG_LIST, TAG_MAP, TAG_NULL, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_RECORD, TAG_STATEMENTS, TAG_TRUE, TAG_TUPLE, TAG_UINT, TAG_VALUE, TAG_WIRE, block_inner_shape, elem_shape_of, map_inner_shape, statements_variants, table_spec_of};
use super::{EncCtx, check_depth, encode_string, encode_string_inline, encode_table, homogeneous_numeric_kind, record_spec_of, write_symref_forced};

//#region 🔖️Encode
/// 🧾️ Encodes one record's fields as `field_count varint, (field_id varint, value)*` —
/// the shared body used both for the top-level document and for nested `FieldValue::Record`
/// (behind its own `0x0D` tag). Always sorts by field id (the purity LAW: byte-identical output
/// regardless of `HashMap` iteration order) and always omits `Absent`. `spec` is `None` for a
/// genuinely schema-less context (an unrecognized `Statements` variant, a shape/value mismatch);
/// fields are then encoded generically. `options.preserve_unknown == false` drops fields whose id
/// isn't found in `spec` instead of encoding them.
pub(super) fn encode_record_fields(ctx: &mut EncCtx<'_>, spec: Option<&RecordSpec>, record: &RecordValue, depth: u16) -> Result<Vec<u8>, PackRefusal> {
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
pub(super) fn encode_value(ctx: &mut EncCtx<'_>, shape: Option<&Shape>, value: &FieldValue, depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
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
