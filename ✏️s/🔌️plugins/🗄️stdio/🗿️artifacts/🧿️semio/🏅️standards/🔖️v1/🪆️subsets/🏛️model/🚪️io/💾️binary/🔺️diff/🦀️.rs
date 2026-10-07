//! 💾️ Binary representation codec surface for `stdio.semio.model` (diff).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::model::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_named_triple, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, Property, PropertySet, PsetValue, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText, MutationDiff};











// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}





/// 🧪️ P2 pilot (model): real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::
/// write_varint_u64` / `store::ByteReader`, same helpers `stdio.flow`'s upgraded `DiffCodec`
/// reuses) backing the real `DiffBinary::encode_diff`/`decode_diff` below.
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













































































impl protocol::DiffBinary for SemioModelDiff {
/// ⚡️ P2 pilot (model): real binary diff frame, replacing the old `print_diff().into_bytes()`
/// text-as-binary shortcut. `format u8` + `presence u8` (bit0=`spatial`, bit1=`elements`,
/// bit2=`relations`) are two REAL fixed fields; each present collection then follows as its own
/// varint-length-prefixed opaque blob (the same `enc_spatial_diff`/`enc_elements_diff`/
/// `enc_relations_diff` bracket/hex text this type's `print_diff` already produces) — three
/// independently-delimited segments rather than one bare trailing `bytes` because there can be
/// 0-3 of them (chaining a `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a
/// second `if`-guard on a field that was itself only conditionally decoded hard-errors
/// `eval_cond` — same gap `stdio.semio.flow`'s own diff facet hit first).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.spatial.is_some() {
        presence |= 0b001;
    }
    if self.elements.is_some() {
        presence |= 0b010;
    }
    if self.relations.is_some() {
        presence |= 0b100;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.spatial {
        write_str_lp(&mut out, &enc_spatial_diff(v));
    }
    if let Some(v) = &self.elements {
        write_str_lp(&mut out, &enc_elements_diff(v));
    }
    if let Some(v) = &self.relations {
        write_str_lp(&mut out, &enc_relations_diff(v));
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let spatial = if presence & 0b001 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff spatial blob", offset: 2, detail: e })?;
        Some(dec_spatial_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff spatial text", offset: 2, detail: e })?)
    } else {
        None
    };
    let elements = if presence & 0b010 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff elements blob", offset: 2, detail: e })?;
        Some(dec_elements_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff elements text", offset: 2, detail: e })?)
    } else {
        None
    };
    let relations = if presence & 0b100 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff relations blob", offset: 2, detail: e })?;
        Some(dec_relations_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff relations text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioModelDiff { spatial, elements, relations })
}
}

use crate::standards::v1::subsets::model::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, parse_f64, enc_list, dec_list, enc_point3, dec_point3, enc_quat, dec_quat, enc_transform, dec_transform, enc_spatial_kind, dec_spatial_kind, enc_element_class, dec_element_class, enc_geometry_ref, dec_geometry_ref, enc_pset_value, dec_pset_value, enc_property, dec_property, enc_property_set, dec_property_set, enc_spatial_node, dec_spatial_node, enc_element, dec_element, enc_relation_kind, dec_relation_kind, enc_relation, dec_relation, enc_spatial_node_diff, dec_spatial_node_diff, enc_element_diff, dec_element_diff, enc_relation_diff, dec_relation_diff, enc_spatial_diff, dec_spatial_diff, enc_elements_diff, dec_elements_diff, enc_relations_diff, dec_relations_diff};
}
pub use diff_codec::*;
