//! 🧮️ `TableSoA` columnar encode and decode for `Shape::Table`.

use semio_framework_value::ValueRefusalKind;
use semio_framework_pack_error::PackRefusal;
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordSpecProducer, RecordValue};
use semio_framework_replication_codec::{write_varint_i64, write_varint_u64, ByteReader};
use semio_framework_pack_record_schema::{ELEM_BOOL, ELEM_ENUM, ELEM_F64, ELEM_INT, ELEM_STR, ELEM_UINT, TAG_TABLE_SOA, elem_tag_for_shape};
use super::{DecCtx, EncCtx, check_depth, decode_value, encode_value, resolve_symref, write_symref_forced};

//#region 🔖️Table

/// 📊️ Encodes `Shape::Table`'s `List(Record)` value as columnar `TableSoA`: `row_count,
/// col_count`, then per column (sorted by field id) `field_id, presence (0=dense/1=sparse+bitmap),
/// elem_tag, packed payload`. Fixed-width/varint columns write only present-row values
/// (compacted, in row order); `Bool` columns instead write one ceil(rows/8)-byte row-aligned value
/// bitmap unconditionally (simpler than compacting individual bits). `Text` columns are always
/// interned (forced symrefs, matching `build_symbols`'s pre-pass); every other shape falls back to
/// self-describing per-present-row values.
pub(super) fn encode_table(ctx: &mut EncCtx<'_>, spec_fn: RecordSpecProducer, items: &[FieldValue], depth: u16, out: &mut Vec<u8>) -> Result<(), PackRefusal> {
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
pub(super) fn decode_table_soa(reader: &mut ByteReader<'_>, spec_fn: Option<RecordSpecProducer>, ctx: &mut DecCtx<'_>, depth: u16) -> Result<Vec<FieldValue>, PackRefusal> {
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
