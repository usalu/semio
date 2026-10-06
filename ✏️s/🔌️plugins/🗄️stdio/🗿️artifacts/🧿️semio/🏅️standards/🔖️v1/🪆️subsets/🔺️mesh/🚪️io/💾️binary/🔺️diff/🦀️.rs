//! 💾️ Binary representation codec surface for `s.stdio.semio.mesh` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::mesh::schema::diff::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;













/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`) backing the real `DiffBinary::encode_diff`/`decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec();
    String::from_utf8(bytes).map_err(|e| e.to_string())
}







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

















































































impl protocol::DiffBinary for SemioMeshDiff {
/// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
/// shortcut (per this wave's brief item 5). `format u8` + `presence u8` (bit0 = `meshes`
/// present, bit1 = `materials` present, bit2 = `textures` present) are two REAL fixed fields;
/// each present collection then follows as its own varint-length-prefixed opaque blob (the
/// same `enc_meshes_diff`/`enc_materials_diff`/`enc_textures_diff` bracket/hex text
/// `print_diff` already produces) — independently-delimited segments rather than one bare
/// trailing `bytes` because there can be 0-3 of them (chaining a `Cond` per-segment hits the
/// `protocol-cond-cannot-chain` gap: a second `if`-guard on a field that was itself only
/// conditionally decoded hard-errors `eval_cond` — see `🌊️flow`'s pilot report).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    let mut presence = 0u8;
    if self.meshes.is_some() {
        presence |= 0b001;
    }
    if self.materials.is_some() {
        presence |= 0b010;
    }
    if self.textures.is_some() {
        presence |= 0b100;
    }
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(v) = &self.meshes {
        write_str_lp(&mut out, &enc_meshes_diff(v));
    }
    if let Some(v) = &self.materials {
        write_str_lp(&mut out, &enc_materials_diff(v));
    }
    if let Some(v) = &self.textures {
        write_str_lp(&mut out, &enc_textures_diff(v));
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
    let meshes = if presence & 0b001 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff meshes blob", offset: 2, detail: e })?;
        Some(dec_meshes_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff meshes text", offset: 2, detail: e })?)
    } else {
        None
    };
    let materials = if presence & 0b010 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff materials blob", offset: 2, detail: e })?;
        Some(dec_materials_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff materials text", offset: 2, detail: e })?)
    } else {
        None
    };
    let textures = if presence & 0b100 != 0 {
        let text = read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff textures blob", offset: 2, detail: e })?;
        Some(dec_textures_diff(&text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff textures text", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioMeshDiff { meshes, materials, textures })
}
}

use crate::mesh::io::text::diff::{hex_encode, hex_decode, enc_str, dec_str, enc_bytes, dec_bytes, parse_f32, parse_f64, parse_u32, enc_list, dec_list, enc_point3, dec_point3, enc_uv, dec_uv, enc_rgba, dec_rgba, enc_topology, dec_topology, enc_primitive, dec_primitive, enc_mesh, dec_mesh, enc_material, dec_material, enc_texture, dec_texture, enc_named_added_mesh, dec_named_added_mesh, enc_named_added_primitive, dec_named_added_primitive, enc_named_added_material, dec_named_added_material, enc_named_added_texture, dec_named_added_texture, enc_primitive_diff, dec_primitive_diff, enc_mesh_item_diff, dec_mesh_item_diff, enc_material_diff, dec_material_diff, enc_texture_diff, dec_texture_diff, enc_meshes_diff, dec_meshes_diff, enc_materials_diff, dec_materials_diff, enc_textures_diff, dec_textures_diff};
}
pub use diff_codec::*;
