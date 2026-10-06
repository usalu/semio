//! binary rep for stdio.ifc 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v4::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{IfcComplexType, IfcEntity, IfcValue};
use crate::IfcSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

/// 🧪️ P2-FG1: real varint/length-prefixed binary primitives (own local copy — never shared
/// cross-artifact, same convention the text primitives above use) backing the upgraded
/// `DiffBinary::encode_diff`/`decode_diff` frame below and `OpBinary::encode_op`/`decode_op`
/// (`../../🧬️mutations/🦀️.rs`) — reuses `store::pack_rt::write_varint_u64`/
/// `store::ByteReader` rather than reinventing varint encode/decode. `pub(crate)` so the
/// mutations sibling can reuse these rather than duplicating them a second time in that file (same
/// intra-artifact-reuse split the TEXT codec primitives above already use).
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
pub(crate) fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}

/// ➡️ Zigzag-encodes `value` into `store::pack_rt::write_varint_u64`'s unsigned domain — own local
/// copy (`store::pack_rt` only ships the unsigned writer; the read side's zigzag decode is already
/// built into `store::ByteReader::read_varint_i64`), same convention `zip`'s own diff module uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_varint_i64(out: &mut Vec<u8>, value: i64) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    store::pack_rt::write_varint_u64(out, zigzag);
}

/// 🧪️ P2-FG1: real recursive binary twin of [`enc_ifc_value`]/[`dec_ifc_value`] above — same 0-8
/// ordinal order as the text codec's `U`-`T` tag range, backing the upgraded `DiffCodec`
/// (below)/`OpBinary` (`../../🧬️mutations/🦀️.rs`) frames. `Aggregate`/`TypedValue` recurse
/// into [`enc_ifc_value_list_bin`] exactly like their text-codec twins recurse into
/// `enc_ifc_value_list` — genuine field-by-field binary all the way down, no opaque tail needed
/// here (unlike the top-level `IfcDiff`/`IfcMutation` frames, `IfcValue` itself is fully flat/
/// spec-expressible per variant).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifc_value_bin(v: &IfcValue, out: &mut Vec<u8>) {
    match v {
        IfcValue::Unset => out.push(0),
        IfcValue::Derived => out.push(1),
        IfcValue::Integer(i) => {
            out.push(2);
            write_varint_i64(out, *i);
        }
        IfcValue::Real(r) => {
            out.push(3);
            out.extend_from_slice(&r.to_le_bytes());
        }
        IfcValue::String(s) => {
            out.push(4);
            write_str_bin(out, s);
        }
        IfcValue::Enum(s) => {
            out.push(5);
            write_str_bin(out, s);
        }
        IfcValue::Reference(id) => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, *id);
        }
        IfcValue::Aggregate(items) => {
            out.push(7);
            enc_ifc_value_list_bin(items, out);
        }
        IfcValue::TypedValue { name, items } => {
            out.push(8);
            write_str_bin(out, name);
            enc_ifc_value_list_bin(items, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifc_value_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(IfcValue::Unset),
        1 => Ok(IfcValue::Derived),
        2 => Ok(IfcValue::Integer(reader.read_varint_i64().map_err(|e| e.to_string())?)),
        3 => Ok(IfcValue::Real(reader.read_f64_le().map_err(|e| e.to_string())?)),
        4 => Ok(IfcValue::String(read_str_bin(reader)?)),
        5 => Ok(IfcValue::Enum(read_str_bin(reader)?)),
        6 => Ok(IfcValue::Reference(reader.read_varint_u64().map_err(|e| e.to_string())?)),
        7 => Ok(IfcValue::Aggregate(dec_ifc_value_list_bin(reader)?)),
        8 => {
            let name = read_str_bin(reader)?;
            let items = dec_ifc_value_list_bin(reader)?;
            Ok(IfcValue::TypedValue { name, items })
        }
        other => Err(format!("ifc value binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_ifc_value_list_bin(vs: &[IfcValue], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, vs.len() as u64);
    for v in vs {
        enc_ifc_value_bin(v, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_ifc_value_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<IfcValue>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_ifc_value_bin(reader)).collect()
}

/// 🧪️ P2-FG1: real binary twin of [`enc_complex_type`]/[`dec_complex_type`] and
/// [`enc_entity`]/[`dec_entity`] above — `id | name | args | complex` field-by-field, `args`/
/// `complex` each length-prefixed lists of the recursive `enc_ifc_value_bin`/`enc_complex_type_bin`
/// shape. `pub(crate)` (entity + list variants) so the mutations sibling can reuse these for its
/// own `InsertEntity`/`SetSnapshot` payloads, same intra-artifact-reuse split the TEXT codec uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_complex_type_bin(c: &IfcComplexType, out: &mut Vec<u8>) {
    write_str_bin(out, &c.name);
    enc_ifc_value_list_bin(&c.args, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_complex_type_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcComplexType, String> {
    let name = read_str_bin(reader)?;
    let args = dec_ifc_value_list_bin(reader)?;
    Ok(IfcComplexType { name, args })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_complex_list_bin(list: &[IfcComplexType], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for c in list {
        enc_complex_type_bin(c, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_complex_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<IfcComplexType>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_complex_type_bin(reader)).collect()
}

/// 📦️ `id | name | args | complex` — positional, mirrors [`enc_entity`]'s own field order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_bin(e: &IfcEntity, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, e.id);
    write_str_bin(out, &e.name);
    enc_ifc_value_list_bin(&e.args, out);
    enc_complex_list_bin(&e.complex, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcEntity, String> {
    let id = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let name = read_str_bin(reader)?;
    let args = dec_ifc_value_list_bin(reader)?;
    let complex = dec_complex_list_bin(reader)?;
    Ok(IfcEntity { id, name, args, complex })
}

/// 🧪️ P2-FG1: real binary twin of [`enc_args_diff`]/[`dec_args_diff`],
/// [`enc_entity_diff`]/[`dec_entity_diff`], [`enc_entities_diff`]/[`dec_entities_diff`] above — each
/// collection triple becomes three varint-counted lists (removed/modified/added), the recursive
/// per-entry `value`/`diff`/`entity` payload the flat/recursive `IfcValue`/`IfcEntityDiff`/
/// `IfcEntity` binary shape already defined above — backing the upgraded
/// `DiffBinary::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_args_diff_bin(d: &IfcArgsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_ifc_value_bin(&m.value, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_ifc_value_bin(&a.value, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_args_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcArgsDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let value = dec_ifc_value_bin(reader)?;
        modified.push(IfcArgModified { index, value });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let value = dec_ifc_value_bin(reader)?;
        added.push(IfcArgAdded { index, value });
    }
    Ok(IfcArgsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff_bin(d: &IfcEntityDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.name, |v, o| write_str_bin(o, v));
    write_option_bin(out, &d.args, enc_args_diff_bin);
    write_option_bin(out, &d.complex, |v, o| enc_complex_list_bin(v, o));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcEntityDiff, String> {
    let name = read_option_bin(reader, read_str_bin)?;
    let args = read_option_bin(reader, dec_args_diff_bin)?;
    let complex = read_option_bin(reader, dec_complex_list_bin)?;
    Ok(IfcEntityDiff { name, args, complex })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff_bin(d: &IfcEntitiesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for id in &d.removed {
        store::pack_rt::write_varint_u64(out, *id);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.id);
        enc_entity_diff_bin(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_entity_bin(&a.entity, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entities_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcEntitiesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let id = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let diff = dec_entity_diff_bin(reader)?;
        modified.push(IfcEntityModified { id, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let entity = dec_entity_bin(reader)?;
        added.push(IfcEntityAdded { index, entity });
    }
    Ok(IfcEntitiesDiff { removed, modified, added })
}

impl protocol::DiffBinary for IfcDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | field payloads...`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (`IfcDiff` was one of
/// 4 of stdio's 7 FG1 standards still on that shortcut per this wave's own P2-FG1 census).
/// `flags` bit0..3 = `file_description`/`file_name`/`file_schema`/`entities` presence (same
/// order the text grammar's token list uses); each present field's payload follows immediately
/// in that order, real field-by-field binary all the way down for `file_description`/
/// `file_name`/`file_schema` (flat `Vec<IfcValue>`) and for `entities`' own flat `id`/`name`/
/// `index` fields — only the innermost recursive `IfcValue::Aggregate`/`TypedValue` payload
/// bottoms out via `enc_ifc_value_bin`'s own recursive call (not an opaque tail: `IfcValue` is
/// fully spec-expressible per variant, see that fn's own doc comment).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let flags: u8 = (self.file_description.is_some() as u8) | ((self.file_name.is_some() as u8) << 1) | ((self.file_schema.is_some() as u8) << 2) | ((self.entities.is_some() as u8) << 3);
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(v) = &self.file_description {
        enc_ifc_value_list_bin(v, &mut out);
    }
    if let Some(v) = &self.file_name {
        enc_ifc_value_list_bin(v, &mut out);
    }
    if let Some(v) = &self.file_schema {
        enc_ifc_value_list_bin(v, &mut out);
    }
    if let Some(v) = &self.entities {
        enc_entities_diff_bin(v, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let file_description = if flags & 1 != 0 { Some(dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("diff file_description", reader.position(), e))?) } else { None };
    let file_name = if flags & 2 != 0 { Some(dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("diff file_name", reader.position(), e))?) } else { None };
    let file_schema = if flags & 4 != 0 { Some(dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("diff file_schema", reader.position(), e))?) } else { None };
    let entities = if flags & 8 != 0 { Some(dec_entities_diff_bin(&mut reader).map_err(|e| malformed("diff entities", reader.position(), e))?) } else { None };
    Ok(IfcDiff { file_description, file_name, file_schema, entities })
}
}

}
pub use diff_codec::*;

#[allow(unused_imports)]
mod diff_wire_codec {
use super::*;
use crate::standards::v4::subsets::any::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::schema::snapshot::{IfcComplexType, IfcEntity, IfcValue};
use crate::IfcSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

// 🚫️async: EEEEhE️EntityBinaryCodecs
/// 🧪️ P2-FG1EEE EE1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_list_bin(list: &[IfcEntity], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for e in list {
        enc_entity_bin(e, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<IfcEntity>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_entity_bin(reader)).collect()
}
}
pub use diff_wire_codec::*;
