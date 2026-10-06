//! 💾️ Binary representation codec surface for `stdio.semio.value` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::value::schema::diff::*;
use crate::standards::v1::subsets::base::schema::triples::{dec_indexed_triple, dec_named_triple, enc_indexed_triple, enc_named_triple, IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueNode, ValueId};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

/// 🧪️ Real length-prefixed binary primitives (`store::pack_rt::write_varint_u64`/
/// `store::ByteReader`), the genuinely-recursive twin of `hex_encode`/`hex_decode` above —
/// template copied verbatim from `json`'s own `write_bytes_lp`/`read_bytes_lp`/`write_str_lp`/
/// `read_str_lp` (`🔣️json/…/🔺️diff/🦀️.rs`).
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

/// 🌳️ Real recursive binary twin of [`enc_semio_value`]/[`dec_semio_value`] — a 1-byte kind tag
/// (`0`=Null/`1`=Bool/`2`=Int/`3`=Float/`4`=Str/`5`=Bytes/`6`=List/`7`=Map/`8`=Ref) followed by the
/// real payload (length-prefixed bytes for scalars, a varint COUNT then that many recursively
/// encoded elements for `List`/`Map` — genuinely recursive, not text-as-bytes). Template copied
/// from json's `enc_json_value_bin`/`dec_json_value_bin`. Backs the upgraded `DiffCodec`/`OpBinary`
/// frames (this file's own `encode_diff`/`decode_diff`, and the sibling `🧬️mutations/🦀️.rs`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_bin(value: &SemioValue, out: &mut Vec<u8>) {
    match value {
        SemioValue::Null => out.push(0),
        SemioValue::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        SemioValue::Int { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        SemioValue::Float { lexeme } => {
            out.push(3);
            write_str_lp(out, lexeme);
        }
        SemioValue::Str { value } => {
            out.push(4);
            write_str_lp(out, value);
        }
        SemioValue::Bytes { value } => {
            out.push(5);
            write_bytes_lp(out, value);
        }
        SemioValue::List { items } => {
            out.push(6);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                enc_semio_value_bin(item, out);
            }
        }
        SemioValue::Map { entries } => {
            out.push(7);
            store::pack_rt::write_varint_u64(out, entries.len() as u64);
            for entry in entries {
                write_str_lp(out, &entry.key);
                enc_semio_value_bin(&entry.value, out);
            }
        }
        SemioValue::Ref { id } => {
            out.push(8);
            write_str_lp(out, &id.value);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValue, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(SemioValue::Null),
        1 => Ok(SemioValue::Bool { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        2 => Ok(SemioValue::Int { lexeme: read_str_lp(reader)? }),
        3 => Ok(SemioValue::Float { lexeme: read_str_lp(reader)? }),
        4 => Ok(SemioValue::Str { value: read_str_lp(reader)? }),
        5 => Ok(SemioValue::Bytes { value: read_bytes_lp(reader)? }),
        6 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut items = Vec::with_capacity(count as usize);
            for _ in 0..count {
                items.push(dec_semio_value_bin(reader)?);
            }
            Ok(SemioValue::List { items })
        }
        7 => {
            let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut entries = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let key = read_str_lp(reader)?;
                let value = dec_semio_value_bin(reader)?;
                entries.push(SemioValueEntry { key, value });
            }
            Ok(SemioValue::Map { entries })
        }
        8 => Ok(SemioValue::Ref { id: ValueId::new(read_str_lp(reader)?) }),
        other => Err(format!("semio value binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_value_node_bin(node: &SemioValueNode, out: &mut Vec<u8>) {
    write_str_lp(out, &node.id.value);
    enc_semio_value_bin(&node.value, out);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_value_node_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValueNode, String> {
    let id = ValueId::new(read_str_lp(reader)?);
    let value = dec_semio_value_bin(reader)?;
    Ok(SemioValueNode { id, value })
}

/// 🌳️ Real recursive binary twin of [`enc_value_diff`]/[`dec_value_diff`] — tag numbering distinct
/// from the text codec's letter tags (`0`=Replace/`1`=Bool/`2`=Int/`3`=Float/`4`=Str/`5`=Bytes/
/// `6`=List/`7`=Map/`8`=Ref), same shape json's `enc_value_diff_bin`/`dec_value_diff_bin` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value_diff_bin(d: &SemioValueDiff, out: &mut Vec<u8>) {
    match d {
        SemioValueDiff::Replace { value } => {
            out.push(0);
            enc_semio_value_bin(value, out);
        }
        SemioValueDiff::Bool { value } => {
            out.push(1);
            out.push(if *value { 1 } else { 0 });
        }
        SemioValueDiff::Int { lexeme } => {
            out.push(2);
            write_str_lp(out, lexeme);
        }
        SemioValueDiff::Float { lexeme } => {
            out.push(3);
            write_str_lp(out, lexeme);
        }
        SemioValueDiff::Str { value } => {
            out.push(4);
            write_str_lp(out, value);
        }
        SemioValueDiff::Bytes { value } => {
            out.push(5);
            write_bytes_lp(out, value);
        }
        SemioValueDiff::List { diff } => {
            out.push(6);
            enc_indexed_diff_bin(diff, out);
        }
        SemioValueDiff::Map { diff } => {
            out.push(7);
            enc_map_diff_bin(diff, out);
        }
        SemioValueDiff::Ref { id } => {
            out.push(8);
            write_str_lp(out, &id.value);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<SemioValueDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(SemioValueDiff::Replace { value: dec_semio_value_bin(reader)? }),
        1 => Ok(SemioValueDiff::Bool { value: reader.read_u8().map_err(|e| e.to_string())? != 0 }),
        2 => Ok(SemioValueDiff::Int { lexeme: read_str_lp(reader)? }),
        3 => Ok(SemioValueDiff::Float { lexeme: read_str_lp(reader)? }),
        4 => Ok(SemioValueDiff::Str { value: read_str_lp(reader)? }),
        5 => Ok(SemioValueDiff::Bytes { value: read_bytes_lp(reader)? }),
        6 => Ok(SemioValueDiff::List { diff: dec_indexed_diff_bin(reader)? }),
        7 => Ok(SemioValueDiff::Map { diff: dec_map_diff_bin(reader)? }),
        8 => Ok(SemioValueDiff::Ref { id: ValueId::new(read_str_lp(reader)?) }),
        other => Err(format!("semio value diff binary: unknown tag {other}")),
    }
}

/// 🌳️ `List`'s `IndexedTripleDiff<SemioValueDiff, SemioValue>` — varint COUNT then that many
/// entries per section, same shape json's `enc_array_diff_bin`/`dec_array_diff_bin` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_indexed_diff_bin(diff: &IndexedTripleDiff<SemioValueDiff, SemioValue>, out: &mut Vec<u8>) {
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
        enc_semio_value_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_indexed_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<IndexedTripleDiff<SemioValueDiff, SemioValue>, String> {
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
        modified.push(IndexModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_semio_value_bin(reader)?;
        added.push(IndexAdded { index, item });
    }
    Ok(IndexedTripleDiff { removed, modified, added })
}

/// 🌳️ `Map`'s `NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>` — `added`
/// entries carry their own `index` (see [`NamedAdded`]'s doc comment), same shape json's
/// `enc_value_diff_bin`/`dec_value_diff_bin` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_map_diff_bin(diff: &NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>, out: &mut Vec<u8>) {
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
        write_str_lp(out, &entry.item.key);
        enc_semio_value_bin(&entry.item.value, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_map_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>, String> {
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
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let key = read_str_lp(reader)?;
        let value = dec_semio_value_bin(reader)?;
        added.push(NamedAdded { index, item: SemioValueEntry { key, value } });
    }
    Ok(NamedTripleDiff { removed, modified, added })
}

/// 🌳️ The top-level `nodes` GRAPH's `NamedTripleDiff<ValueId, SemioValueDiff,
/// NamedAdded<SemioValueNode>>` — same shape as [`enc_map_diff_bin`]/[`dec_map_diff_bin`], keyed
/// by [`ValueId`] instead of a plain map-entry key.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_nodes_diff_bin(diff: &NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for id in &diff.removed {
        write_str_lp(out, &id.value);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.key.value);
        enc_value_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_semio_value_node_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_nodes_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(ValueId::new(read_str_lp(reader)?));
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = ValueId::new(read_str_lp(reader)?);
        let diff = dec_value_diff_bin(reader)?;
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_semio_value_node_bin(reader)?;
        added.push(NamedAdded { index, item });
    }
    Ok(NamedTripleDiff { removed, modified, added })
}

impl protocol::DiffBinary for SemioValueTreeDiff {
/// 🧪️ Real binary frame (`format u8 | presence u8 | root? | nodes?`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the `print_diff().into_bytes()` text-as-binary shortcut this facet started
/// with. `presence` bit0=`root` present, bit1=`nodes` present (both real, individually
/// protocol-walkable fixed fields); the recursive `SemioValueDiff`/`NamedTripleDiff` payloads
/// are real LEB128-varint-framed binary ([`enc_value_diff_bin`]/[`enc_nodes_diff_bin`]
/// above), honestly opaque only at the PROTOCOL-DESCRIPTION layer (`Prim::Ref` can't recurse —
/// recipe §5's `protocol-prim-ref-recursion` gap), genuinely structured and round-trip tested
/// at the Rust layer — same treatment json's own `JsonDiff::encode_diff` uses.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let presence: u8 = (if self.root.is_some() { 1 } else { 0 }) | (if self.nodes.is_some() { 2 } else { 0 });
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, presence];
    if let Some(root) = &self.root {
        enc_value_diff_bin(root, &mut out);
    }
    if let Some(nodes) = &self.nodes {
        enc_nodes_diff_bin(nodes, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let presence = reader.read_u8().map_err(|e| malformed("diff presence", 1, e.to_string()))?;
    let root = if presence & 1 != 0 { Some(dec_value_diff_bin(&mut reader).map_err(|e| malformed("diff root", reader.position(), e))?) } else { None };
    let nodes = if presence & 2 != 0 { Some(dec_nodes_diff_bin(&mut reader).map_err(|e| malformed("diff nodes", reader.position(), e))?) } else { None };
    Ok(SemioValueTreeDiff { root, nodes })
}
}
}
pub use diff_codec::*;
