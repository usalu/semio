//! binary rep for stdio.step 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ap214::subsets::base::schema::diff::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use crate::StepSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use crate::schema::snapshot::StepComplexType;
use crate::schema::snapshot::StepEntity;
use crate::schema::snapshot::StepFileDescription;
use crate::schema::snapshot::StepFileName;
use crate::schema::snapshot::StepFileSchema;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::{StepTypedValue, StepValue};

/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives backing the upgraded `OpBinary`
/// (`../🧬️mutations/🦀️.rs`) and `DiffCodec` (below) frames — mirrors md/dxf's own
/// `write_str_bin`/`read_str_bin`/`write_option_bin`/`read_option_bin` shape, reusing
/// `store::pack_rt::write_varint_u64`/`store::write_varint_i64`/`store::ByteReader` rather than
/// reinventing varint encode/decode. `pub(crate)` so the mutations sibling can reuse these rather
/// than duplicating them a second time in that file (same intra-artifact-reuse split the TEXT
/// codec primitives above use).
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
pub(crate) fn write_f64_bin(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {
    reader.read_f64_le().map_err(|e| e.to_string())
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_list_bin(out: &mut Vec<u8>, list: &[String]) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for s in list {
        write_str_bin(out, s);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<String>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| read_str_bin(reader)).collect()
}

/// 🧪️ P2-FG1: real recursive binary twin of [`enc_value`]/[`dec_value`] — same 0-8 ordinal order
/// as `StepValue`'s own declaration (`Unset`,`Derived`,`Integer`,`Real`,`String`,`Enum`,
/// `Reference`,`Aggregate`,`TypedValue`), backing the upgraded `OpBinary`/`DiffCodec` frames below.
/// `Aggregate`/`TypedValue` recurse via plain Rust function recursion — the DSL derive machinery's
/// `Prim::Ref` protocol-dialect blocker (cited on the sibling `.protocol.semio` files) constrains
/// only the DECLARATIVE description, never hand-written Rust, which recurses here exactly like
/// `enc_value`'s own text twin does two regions up.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_bin(v: &StepValue, out: &mut Vec<u8>) {
    match v {
        StepValue::Unset => out.push(0),
        StepValue::Derived => out.push(1),
        StepValue::Integer(i) => {
            out.push(2);
            store::write_varint_i64(out, *i);
        }
        StepValue::Real(r) => {
            out.push(3);
            write_f64_bin(out, *r);
        }
        StepValue::String(s) => {
            out.push(4);
            write_str_bin(out, s);
        }
        StepValue::Enum(s) => {
            out.push(5);
            write_str_bin(out, s);
        }
        StepValue::Reference(id) => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, *id);
        }
        StepValue::Aggregate(items) => {
            out.push(7);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                enc_value_bin(item, out);
            }
        }
        StepValue::TypedValue(StepTypedValue { type_name, value }) => {
            out.push(8);
            write_str_bin(out, type_name);
            enc_value_bin(value, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_bin(reader: &mut store::ByteReader<'_>) -> Result<StepValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(StepValue::Unset),
        1 => Ok(StepValue::Derived),
        2 => Ok(StepValue::Integer(reader.read_varint_i64().map_err(|e| e.to_string())?)),
        3 => Ok(StepValue::Real(read_f64_bin(reader)?)),
        4 => Ok(StepValue::String(read_str_bin(reader)?)),
        5 => Ok(StepValue::Enum(read_str_bin(reader)?)),
        6 => Ok(StepValue::Reference(reader.read_varint_u64().map_err(|e| e.to_string())?)),
        7 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let items = (0..count).map(|_| dec_value_bin(reader)).collect::<Result<Vec<_>, String>>()?;
            Ok(StepValue::Aggregate(items))
        }
        8 => {
            let type_name = read_str_bin(reader)?;
            let value = Box::new(dec_value_bin(reader)?);
            Ok(StepValue::TypedValue(StepTypedValue { type_name, value }))
        }
        other => Err(format!("step value binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_complex_bin(c: &StepComplexType, out: &mut Vec<u8>) {
    write_str_bin(out, &c.name);
    store::pack_rt::write_varint_u64(out, c.args.len() as u64);
    for a in &c.args {
        enc_value_bin(a, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_complex_bin(reader: &mut store::ByteReader<'_>) -> Result<StepComplexType, String> {
    let name = read_str_bin(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let args = (0..count).map(|_| dec_value_bin(reader)).collect::<Result<Vec<_>, String>>()?;
    Ok(StepComplexType { name, args })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_bin(e: &StepEntity, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, e.id);
    write_str_bin(out, &e.name);
    store::pack_rt::write_varint_u64(out, e.args.len() as u64);
    for a in &e.args {
        enc_value_bin(a, out);
    }
    store::pack_rt::write_varint_u64(out, e.complex.len() as u64);
    for c in &e.complex {
        enc_complex_bin(c, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_bin(reader: &mut store::ByteReader<'_>) -> Result<StepEntity, String> {
    let id = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let name = read_str_bin(reader)?;
    let args_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let args = (0..args_count).map(|_| dec_value_bin(reader)).collect::<Result<Vec<_>, String>>()?;
    let complex_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let complex = (0..complex_count).map(|_| dec_complex_bin(reader)).collect::<Result<Vec<_>, String>>()?;
    Ok(StepEntity { id, name, args, complex })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_file_description_bin(d: &StepFileDescription, out: &mut Vec<u8>) {
    write_str_list_bin(out, &d.description);
    write_str_bin(out, &d.implementation_level);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_file_description_bin(reader: &mut store::ByteReader<'_>) -> Result<StepFileDescription, String> {
    let description = read_str_list_bin(reader)?;
    let implementation_level = read_str_bin(reader)?;
    Ok(StepFileDescription { description, implementation_level })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_file_name_bin(f: &StepFileName, out: &mut Vec<u8>) {
    write_str_bin(out, &f.name);
    write_str_bin(out, &f.timestamp);
    write_str_list_bin(out, &f.author);
    write_str_list_bin(out, &f.organization);
    write_str_bin(out, &f.preprocessor_version);
    write_str_bin(out, &f.originating_system);
    write_str_bin(out, &f.authorization);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_file_name_bin(reader: &mut store::ByteReader<'_>) -> Result<StepFileName, String> {
    Ok(StepFileName {
        name: read_str_bin(reader)?,
        timestamp: read_str_bin(reader)?,
        author: read_str_list_bin(reader)?,
        organization: read_str_list_bin(reader)?,
        preprocessor_version: read_str_bin(reader)?,
        originating_system: read_str_bin(reader)?,
        authorization: read_str_bin(reader)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_file_schema_bin(s: &StepFileSchema, out: &mut Vec<u8>) {
    write_str_list_bin(out, &s.schemas);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_file_schema_bin(reader: &mut store::ByteReader<'_>) -> Result<StepFileSchema, String> {
    Ok(StepFileSchema { schemas: read_str_list_bin(reader)? })
}

/// 🧪️ P2-FG1: real recursive binary twins of [`enc_args_diff`]/[`enc_entity_diff`]/
/// [`enc_entities_diff`] — same three-section (removed/modified/added) collection-triple shape,
/// backing the upgraded `DiffBinary::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_args_diff_bin(d: &StepArgsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for i in &d.removed {
        store::pack_rt::write_varint_u64(out, *i as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for m in &d.modified {
        store::pack_rt::write_varint_u64(out, m.index as u64);
        enc_value_bin(&m.value, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for a in &d.added {
        store::pack_rt::write_varint_u64(out, a.index as u64);
        enc_value_bin(&a.value, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_args_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<StepArgsDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let value = dec_value_bin(reader)?;
        modified.push(StepArgModified { index, value });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let value = dec_value_bin(reader)?;
        added.push(StepArgAdded { index, value });
    }
    Ok(StepArgsDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entity_diff_bin(d: &StepEntityDiff, out: &mut Vec<u8>) {
    write_option_bin(out, &d.name, |v, o| write_str_bin(o, v));
    write_option_bin(out, &d.args, enc_args_diff_bin);
    write_option_bin(out, &d.complex, |v, o| {
        store::pack_rt::write_varint_u64(o, v.len() as u64);
        for c in v {
            enc_complex_bin(c, o);
        }
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_entity_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<StepEntityDiff, String> {
    let name = read_option_bin(reader, read_str_bin)?;
    let args = read_option_bin(reader, dec_args_diff_bin)?;
    let complex = read_option_bin(reader, |r| {
        let count = r.read_varint_u64().map_err(|e| e.to_string())?;
        (0..count).map(|_| dec_complex_bin(r)).collect::<Result<Vec<_>, String>>()
    })?;
    Ok(StepEntityDiff { name, args, complex })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_entities_diff_bin(d: &StepEntitiesDiff, out: &mut Vec<u8>) {
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
pub(crate) fn dec_entities_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<StepEntitiesDiff, String> {
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
        modified.push(StepEntityModified { id, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let entity = dec_entity_bin(reader)?;
        added.push(StepEntityAdded { index, entity });
    }
    Ok(StepEntitiesDiff { removed, modified, added })
}

impl protocol::DiffBinary for StepDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | present-field payloads`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut. `flags` is a 4-bit
/// presence mask (bit0=`file_description`, bit1=`file_name`, bit2=`file_schema`,
/// bit3=`entities`) — `StepDiff` has FOUR independently optional top-level fields, same shape
/// dxf's own `DxfDiff` (also four) upgraded to this same wave, unlike md/json's single
/// `has_value` byte.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let flags: u8 = (self.file_description.is_some() as u8) | ((self.file_name.is_some() as u8) << 1) | ((self.file_schema.is_some() as u8) << 2) | ((self.entities.is_some() as u8) << 3);
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(v) = &self.file_description {
        enc_file_description_bin(v, &mut out);
    }
    if let Some(v) = &self.file_name {
        enc_file_name_bin(v, &mut out);
    }
    if let Some(v) = &self.file_schema {
        enc_file_schema_bin(v, &mut out);
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
    let file_description = if flags & 1 != 0 { Some(dec_file_description_bin(&mut reader).map_err(|e| malformed("diff file_description", reader.position(), e))?) } else { None };
    let file_name = if flags & 2 != 0 { Some(dec_file_name_bin(&mut reader).map_err(|e| malformed("diff file_name", reader.position(), e))?) } else { None };
    let file_schema = if flags & 4 != 0 { Some(dec_file_schema_bin(&mut reader).map_err(|e| malformed("diff file_schema", reader.position(), e))?) } else { None };
    let entities = if flags & 8 != 0 { Some(dec_entities_diff_bin(&mut reader).map_err(|e| malformed("diff entities", reader.position(), e))?) } else { None };
    Ok(StepDiff { file_description, file_name, file_schema, entities })
}
}
}
pub use diff_codec::*;
