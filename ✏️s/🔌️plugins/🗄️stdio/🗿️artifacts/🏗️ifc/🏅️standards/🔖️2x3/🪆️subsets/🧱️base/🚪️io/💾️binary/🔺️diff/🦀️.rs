//! binary rep for stdio.ifc.2x3 diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Ifc2x3DiffBinary = Vec<u8>;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::diff::*;
use crate::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::part21::{Part21Decimal, Part21Header, Part21Instance, Part21Value};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

/// 🧪️ Real varint/length-prefixed binary primitives (own local copy, never shared cross-artifact —
/// same convention the text primitives above use) — reuses `store::pack_rt::write_varint_u64`/
/// `store::ByteReader` rather than reinventing varint encode/decode. `pub(crate)` so the mutations
/// sibling can reuse these too.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edm_preamble_bin(preamble: &Ifc2x3EdmPreamble, out: &mut Vec<u8>) {
    for value in [
        &preamble.producer,
        &preamble.module,
        &preamble.creation_date,
        &preamble.host,
        &preamble.database,
        &preamble.database_version,
        &preamble.database_creation_date,
        &preamble.schema,
        &preamble.model,
        &preamble.model_creation_date,
        &preamble.header_model,
        &preamble.header_model_creation_date,
        &preamble.user,
        &preamble.group,
        &preamble.license,
        &preamble.options,
    ] {
        write_str_bin(out, value);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edm_preamble_bin(reader: &mut store::ByteReader<'_>) -> Result<Ifc2x3EdmPreamble, String> {
    Ok(Ifc2x3EdmPreamble {
        producer: read_str_bin(reader)?,
        module: read_str_bin(reader)?,
        creation_date: read_str_bin(reader)?,
        host: read_str_bin(reader)?,
        database: read_str_bin(reader)?,
        database_version: read_str_bin(reader)?,
        database_creation_date: read_str_bin(reader)?,
        schema: read_str_bin(reader)?,
        model: read_str_bin(reader)?,
        model_creation_date: read_str_bin(reader)?,
        header_model: read_str_bin(reader)?,
        header_model_creation_date: read_str_bin(reader)?,
        user: read_str_bin(reader)?,
        group: read_str_bin(reader)?,
        license: read_str_bin(reader)?,
        options: read_str_bin(reader)?,
    })
}

/// ➡️ Zigzag-encodes `value` into `store::pack_rt::write_varint_u64`'s unsigned domain — own local
/// copy (`store::pack_rt` only ships the unsigned writer; the read side's zigzag decode is already
/// built into `store::ByteReader::read_varint_i64`), same convention `4`'s own diff module uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_varint_i64(out: &mut Vec<u8>, value: i64) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    store::pack_rt::write_varint_u64(out, zigzag);
}

/// 🧪️ Real recursive binary twin of [`enc_part21_value`]/[`dec_part21_value`] above — same 0-8
/// ordinal order as the text codec's `U`-`T` tag range (matching `4`'s own `enc_ifc_value_bin`
/// ordinal choice for the isomorphic shape). `List`/`Typed` recurse into
/// [`enc_part21_value_list_bin`] exactly like their text-codec twins — genuine field-by-field
/// binary all the way down, no opaque tail needed (`Part21Value` itself is fully flat/
/// spec-expressible per variant, only the top-level `Ifc2x3Diff`/`Ifc2x3Mutation` frames stop at
/// this recursion's OWN entry point rather than an opaque byte chain).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_value_bin(v: &Part21Value, out: &mut Vec<u8>) {
    match v {
        Part21Value::Unset => out.push(0),
        Part21Value::Derived => out.push(1),
        Part21Value::Int(i) => {
            out.push(2);
            write_varint_i64(out, *i);
        }
        Part21Value::Real(r) => {
            out.push(3);
            out.push(r.negative as u8);
            write_str_bin(out, &r.coefficient);
            store::pack_rt::write_varint_u64(out, r.scale as u64);
            match r.exponent {
                None => out.push(0),
                Some(exponent) => {
                    out.push(1);
                    write_varint_i64(out, exponent as i64);
                }
            }
        }
        Part21Value::Str(s) => {
            out.push(4);
            write_str_bin(out, s);
        }
        Part21Value::Enum(s) => {
            out.push(5);
            write_str_bin(out, s);
        }
        Part21Value::Ref(id) => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, *id);
        }
        Part21Value::List(items) => {
            out.push(7);
            enc_part21_value_list_bin(items, out);
        }
        Part21Value::Typed { name, items } => {
            out.push(8);
            write_str_bin(out, name);
            enc_part21_value_list_bin(items, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_value_bin(reader: &mut store::ByteReader<'_>) -> Result<Part21Value, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(Part21Value::Unset),
        1 => Ok(Part21Value::Derived),
        2 => Ok(Part21Value::Int(reader.read_varint_i64().map_err(|e| e.to_string())?)),
        3 => {
            let negative = reader.read_u8().map_err(|e| e.to_string())? != 0;
            let coefficient = read_str_bin(reader)?;
            let scale = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
            let exponent = match reader.read_u8().map_err(|e| e.to_string())? {
                0 => None,
                1 => Some(reader.read_varint_i64().map_err(|e| e.to_string())? as i32),
                tag => return Err(format!("Part21Decimal exponent presence: unknown tag {tag}")),
            };
            Ok(Part21Value::Real(Part21Decimal { negative, coefficient, scale, exponent }))
        }
        4 => Ok(Part21Value::Str(read_str_bin(reader)?)),
        5 => Ok(Part21Value::Enum(read_str_bin(reader)?)),
        6 => Ok(Part21Value::Ref(reader.read_varint_u64().map_err(|e| e.to_string())?)),
        7 => Ok(Part21Value::List(dec_part21_value_list_bin(reader)?)),
        8 => {
            let name = read_str_bin(reader)?;
            let items = dec_part21_value_list_bin(reader)?;
            Ok(Part21Value::Typed { name, items })
        }
        other => Err(format!("part21 value binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_value_list_bin(vs: &[Part21Value], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, vs.len() as u64);
    for v in vs {
        enc_part21_value_bin(v, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_value_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<Part21Value>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_part21_value_bin(reader)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_header_bin(h: &Part21Header, out: &mut Vec<u8>) {
    enc_part21_value_list_bin(&h.file_description, out);
    enc_part21_value_list_bin(&h.file_name, out);
    enc_part21_value_list_bin(&h.file_schema, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_header_bin(reader: &mut store::ByteReader<'_>) -> Result<Part21Header, String> {
    let file_description = dec_part21_value_list_bin(reader)?;
    let file_name = dec_part21_value_list_bin(reader)?;
    let file_schema = dec_part21_value_list_bin(reader)?;
    Ok(Part21Header { file_description, file_name, file_schema })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part21_instance_bin(inst: &Part21Instance, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, inst.id);
    store::pack_rt::write_varint_u64(out, inst.entities.len() as u64);
    for (name, args) in &inst.entities {
        write_str_bin(out, name);
        enc_part21_value_list_bin(args, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part21_instance_bin(reader: &mut store::ByteReader<'_>) -> Result<Part21Instance, String> {
    let id = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut entities = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let name = read_str_bin(reader)?;
        let args = dec_part21_value_list_bin(reader)?;
        entities.push((name, args));
    }
    Ok(Part21Instance { id, entities })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_instance_list_bin(list: &[Part21Instance], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for inst in list {
        enc_part21_instance_bin(inst, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_instance_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<Part21Instance>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_part21_instance_bin(reader)).collect()
}

impl protocol::DiffBinary for Ifc2x3Diff {
/// 🧪️ REAL binary frame (`format u8 | flags u8 | field payloads...`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// no F6/text-as-binary shortcut ever existed for this facet (there was no `DiffCodec` impl at
/// all before this wave). `flags` bit0..3 = `schema`/`header`/non-empty-`removed_instances`/
/// non-empty-`upserted_instances` presence (same order the text grammar's token list uses);
/// each present field's payload follows immediately in that order, real field-by-field binary
/// all the way down — only the innermost recursive `Part21Value::List`/`Typed` payload bottoms
/// out via `enc_part21_value_bin`'s own recursive call (not an opaque tail: `Part21Value` is
/// fully spec-expressible per variant).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let flags: u8 = (self.schema.is_some() as u8)
        | ((self.header.is_some() as u8) << 1)
        | ((!self.removed_instances.is_empty() as u8) << 2)
        | ((!self.upserted_instances.is_empty() as u8) << 3)
        | ((self.edm_preamble.is_some() as u8) << 4)
        | ((self.instance_order.is_some() as u8) << 5);
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(s) = &self.schema {
        write_str_bin(&mut out, s);
    }
    if let Some(h) = &self.header {
        enc_part21_header_bin(h, &mut out);
    }
    if !self.removed_instances.is_empty() {
        store::pack_rt::write_varint_u64(&mut out, self.removed_instances.len() as u64);
        for id in &self.removed_instances {
            store::pack_rt::write_varint_u64(&mut out, *id);
        }
    }
    if !self.upserted_instances.is_empty() {
        enc_instance_list_bin(&self.upserted_instances, &mut out);
    }
    if let Some(preamble) = &self.edm_preamble {
        match preamble {
            None => out.push(0),
            Some(value) => {
                out.push(1);
                enc_edm_preamble_bin(value, &mut out);
            }
        }
    }
    if let Some(order) = &self.instance_order {
        store::pack_rt::write_varint_u64(&mut out, order.len() as u64);
        for id in order {
            store::pack_rt::write_varint_u64(&mut out, *id);
        }
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    if format != store::pack_rt::OP_BINARY_FORMAT {
        return Err(malformed("diff format", 0, format!("unsupported format {format}")));
    }
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    if flags & !0b0011_1111 != 0 {
        return Err(malformed("diff flags", 1, "unknown flag bits".into()));
    }
    let schema = if flags & 1 != 0 { Some(read_str_bin(&mut reader).map_err(|e| malformed("diff schema", reader.position(), e))?) } else { None };
    let header = if flags & 2 != 0 { Some(dec_part21_header_bin(&mut reader).map_err(|e| malformed("diff header", reader.position(), e))?) } else { None };
    let removed_instances = if flags & 4 != 0 {
        let count = reader.read_varint_u64().map_err(|e| malformed("diff removed count", reader.position(), e.to_string()))?;
        let mut v = Vec::with_capacity(count as usize);
        for _ in 0..count {
            v.push(reader.read_varint_u64().map_err(|e| malformed("diff removed id", reader.position(), e.to_string()))?);
        }
        v
    } else {
        Vec::new()
    };
    let upserted_instances = if flags & 8 != 0 { dec_instance_list_bin(&mut reader).map_err(|e| malformed("diff upserted", reader.position(), e))? } else { Vec::new() };
    let edm_preamble = if flags & 16 != 0 {
        Some(match reader.read_u8().map_err(|e| malformed("diff EDM preamble presence", reader.position(), e.to_string()))? {
            0 => None,
            1 => Some(dec_edm_preamble_bin(&mut reader).map_err(|e| malformed("diff EDM preamble", reader.position(), e))?),
            tag => return Err(malformed("diff EDM preamble presence", reader.position(), format!("unknown tag {tag}"))),
        })
    } else {
        None
    };
    let instance_order = if flags & 32 != 0 {
        let count = reader.read_varint_u64().map_err(|error| malformed("diff instance order", reader.position(), error.to_string()))?;
        let mut order = Vec::with_capacity(count as usize);
        for _ in 0..count {
            order.push(reader.read_varint_u64().map_err(|error| malformed("diff instance order", reader.position(), error.to_string()))?);
        }
        Some(order)
    } else {
        None
    };
    if reader.remaining() != 0 {
        return Err(malformed("diff trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
    }
    Ok(Ifc2x3Diff { schema, header, removed_instances, upserted_instances, edm_preamble, instance_order })
}
}
}
pub use diff_codec::*;
