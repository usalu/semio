//! binary rep for stdio.json 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_rfc8259::subsets::base::schema::diff::*;
use crate::schema::snapshot::JsonMember;
use crate::JsonSnapshot;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use framework_schema::ArtifactSchema;
use protocol::os_spr::command::DiffAlgebra;
use std::collections::{HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::JsonValue;

/// 🧪️ P2-P1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing the
/// upgraded `OpBinary`/`DiffCodec` frames (see `../🧬️mutations/🦀️.rs`'s `#region OpCodecs`
/// and `#region 🔖️HandcraftedDiffCodec` below) — reuses `store::pack_rt::write_varint_u64` /
/// `store::ByteReader` rather than reinventing varint encode/decode.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

/// 🧪️ P2-P1: real recursive binary twin of [`enc_json_value`]/[`dec_json_value`] above — a 1-byte
/// kind tag (`0`=Null/`1`=Bool/`2`=Number/`3`=String/`4`=Array/`5`=Object, distinct numbering from
/// the text codec's letter tags, chosen to read cleanly as a match arm) followed by the real payload
/// (length-prefixed bytes for scalars, a varint COUNT then that many recursively-encoded elements
/// for `Array`/`Object` — genuinely recursive, not text-as-bytes). Backs the upgraded `OpBinary`
/// frame (`../🧬️mutations/🦀️.rs`) and the `Replace`/added-item payloads inside
/// [`enc_value_diff_bin`] below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_json_value_bin(value: &JsonValue, out: &mut Vec<u8>) {
    match value {
        JsonValue::Null => out.push(0),
        JsonValue::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        JsonValue::Number { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        JsonValue::String { value } => {
            out.push(3);
            write_str_lp(out, value);
        }
        JsonValue::Array { items } => {
            out.push(4);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                enc_json_value_bin(item, out);
            }
        }
        JsonValue::Object { members } => {
            out.push(5);
            store::pack_rt::write_varint_u64(out, members.len() as u64);
            for member in members {
                write_str_lp(out, &member.key);
                enc_json_value_bin(&member.value, out);
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_json_value_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(JsonValue::Null),
        1 => Ok(JsonValue::Bool { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        2 => Ok(JsonValue::Number { lexeme: read_str_lp(reader)? }),
        3 => Ok(JsonValue::String { value: read_str_lp(reader)? }),
        4 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut items = Vec::with_capacity(count as usize);
            for _ in 0..count {
                items.push(dec_json_value_bin(reader)?);
            }
            Ok(JsonValue::Array { items })
        }
        5 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut members = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let key = read_str_lp(reader)?;
                let value = dec_json_value_bin(reader)?;
                members.push(JsonMember { key, value });
            }
            Ok(JsonValue::Object { members })
        }
        other => Err(format!("json value binary: unknown tag {other}")),
    }
}

/// 🧪️ P2-P1: real recursive binary twin of [`enc_value_diff`]/[`dec_value_diff`] — same 1-byte tag
/// numbering scheme as [`enc_json_value_bin`] plus `6`=`Replace` (needs its own arm since `Replace`
/// wraps a whole [`JsonValue`], not a bare scalar payload). `Array`/`Object` collection triples
/// encode as three varint-counted, recursively-encoded lists (removed/modified/added) — genuinely
/// structured binary, backing the upgraded `DiffBinary::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_diff_bin(diff: &JsonValueDiff, out: &mut Vec<u8>) {
    match diff {
        JsonValueDiff::Replace { value } => {
            out.push(6);
            enc_json_value_bin(value, out);
        }
        JsonValueDiff::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        JsonValueDiff::Number { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        JsonValueDiff::String { value } => {
            out.push(3);
            write_str_lp(out, value);
        }
        JsonValueDiff::Array { diff } => {
            out.push(4);
            enc_array_diff_bin(diff, out);
        }
        JsonValueDiff::Object { diff } => {
            out.push(5);
            enc_object_diff_bin(diff, out);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonValueDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        6 => Ok(JsonValueDiff::Replace { value: dec_json_value_bin(reader)? }),
        1 => Ok(JsonValueDiff::Bool { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        2 => Ok(JsonValueDiff::Number { lexeme: read_str_lp(reader)? }),
        3 => Ok(JsonValueDiff::String { value: read_str_lp(reader)? }),
        4 => Ok(JsonValueDiff::Array { diff: dec_array_diff_bin(reader)? }),
        5 => Ok(JsonValueDiff::Object { diff: dec_object_diff_bin(reader)? }),
        other => Err(format!("json value diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_array_diff_bin(diff: &JsonArrayDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_value_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_json_value_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_array_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonArrayDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_value_diff_bin(reader)?;
        modified.push(JsonArrayModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_json_value_bin(reader)?;
        added.push(JsonArrayAdded { index, item });
    }
    Ok(JsonArrayDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_object_diff_bin(diff: &JsonObjectDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for key in &diff.removed {
        write_str_lp(out, key);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.key);
        enc_value_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        write_str_lp(out, &entry.key);
        enc_json_value_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_object_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<JsonObjectDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(read_str_lp(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = read_str_lp(reader)?;
        let diff = dec_value_diff_bin(reader)?;
        modified.push(JsonObjectModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let key = read_str_lp(reader)?;
        let item = dec_json_value_bin(reader)?;
        added.push(JsonObjectAdded { index, key, item });
    }
    Ok(JsonObjectDiff { removed, modified, added })
}

impl protocol::DiffBinary for JsonDiff {
/// 🧪️ P2-P1: REAL binary frame (`format u8 | has_value u8 | value-diff payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of stdio's
/// `DiffCodec` impls were still on that shortcut per the P2-W0 census; this is the first real
/// upgrade, per the ticket's own "be the good example" framing).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, if self.value.is_some() { 1 } else { 0 }];
    if let Some(value) = &self.value {
        enc_value_diff_bin(value, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let _format = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
    let has_value = reader.read_u8().map_err(|e| protocol::ProtocolError::Malformed { what: "diff has_value", offset: 1, detail: e.to_string() })?;
    let value = if has_value != 0 { Some(dec_value_diff_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff value", offset: reader.position() as u64, detail: e })?) } else { None };
    Ok(JsonDiff { value })
}
}
}
pub use diff_codec::*;
