//! 💾️ Binary representation codec surface for `s.stdio.semio.mesh` (mutations) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::mesh::schema::mutations::*;
#[cfg(test)]
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::mesh::schema::diff::{SemioMeshDiff};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_texture};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_texture};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_material};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_material};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_mesh};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_mesh};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_primitive};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_primitive};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_topology};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_topology};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_uv};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_uv};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_rgba};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_rgba};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_point3};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_point3};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_decode};
use crate::standards::v1::subsets::drawing::io::text::snapshot::{hex_encode};
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
#[cfg(test)]
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioPrimitive, SemioTexture, SemioTopology};
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioMeshMutation` block below
/// calls `self.print_op()` via method syntax, which needs `OpText` in scope in production code
/// too, not merely under `#[cfg(test)]` (same fix `🧊️brep`/`🌊️flow`'s own facets document).
use protocol::{OpBinary, OpText};
use crate::standards::v1::subsets::mesh::schema::mutations::change_material_base_color;
use crate::standards::v1::subsets::mesh::schema::mutations::change_material_metallic;
use crate::standards::v1::subsets::mesh::schema::mutations::change_material_roughness;
use crate::standards::v1::subsets::mesh::schema::mutations::change_texture_mime;
use crate::standards::v1::subsets::mesh::schema::mutations::create_material;
use crate::standards::v1::subsets::mesh::schema::mutations::create_mesh;
use crate::standards::v1::subsets::mesh::schema::mutations::create_primitive;
use crate::standards::v1::subsets::mesh::schema::mutations::create_texture;
use crate::standards::v1::subsets::mesh::schema::mutations::delete_material;
use crate::standards::v1::subsets::mesh::schema::mutations::delete_mesh;
use crate::standards::v1::subsets::mesh::schema::mutations::delete_primitive;
use crate::standards::v1::subsets::mesh::schema::mutations::delete_texture;
use crate::standards::v1::subsets::mesh::schema::mutations::move_vertex;
use crate::standards::v1::subsets::mesh::schema::mutations::replace_primitive_geometry;
use crate::standards::v1::subsets::mesh::schema::mutations::replace_texture_bytes;
use crate::standards::v1::subsets::mesh::schema::mutations::set_primitive_material;
use crate::standards::v1::subsets::mesh::schema::mutations::set_primitive_topology;
/// 🧬️ Every variant wraps exactly one `protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation>`
/// payload struct declared in the corresponding triad leaf's `🦠️mutation/🦀️.rs`. Seventeen
/// triads: mesh lifecycle, primitive lifecycle + topology/geometry/material, material lifecycle +
/// base-color/metallic/roughness, texture lifecycle + mime/bytes, then the one scalar reposition
/// (`move-vertex`).
use crate::standards::v1::subsets::mesh::schema::mutations::set_snapshot::SetSnapshot;
use crate::standards::v1::subsets::mesh::io::text::mutations::{print_semio_mesh_mutation};
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn wire_tag(m: &SemioMeshMutation) -> u8 {
    match m {
        SemioMeshMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
        SemioMeshMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
        SemioMeshMutation::CreateMesh(_) => TAG_CREATE_MESH,
        SemioMeshMutation::DeleteMesh(_) => TAG_DELETE_MESH,
        SemioMeshMutation::CreatePrimitive(_) => TAG_CREATE_PRIMITIVE,
        SemioMeshMutation::DeletePrimitive(_) => TAG_DELETE_PRIMITIVE,
        SemioMeshMutation::SetPrimitiveTopology(_) => TAG_SET_PRIMITIVE_TOPOLOGY,
        SemioMeshMutation::ReplacePrimitiveGeometry(_) => TAG_REPLACE_PRIMITIVE_GEOMETRY,
        SemioMeshMutation::SetPrimitiveMaterial(_) => TAG_SET_PRIMITIVE_MATERIAL,
        SemioMeshMutation::CreateMaterial(_) => TAG_CREATE_MATERIAL,
        SemioMeshMutation::DeleteMaterial(_) => TAG_DELETE_MATERIAL,
        SemioMeshMutation::ChangeMaterialBaseColor(_) => TAG_CHANGE_MATERIAL_BASE_COLOR,
        SemioMeshMutation::ChangeMaterialMetallic(_) => TAG_CHANGE_MATERIAL_METALLIC,
        SemioMeshMutation::ChangeMaterialRoughness(_) => TAG_CHANGE_MATERIAL_ROUGHNESS,
        SemioMeshMutation::CreateTexture(_) => TAG_CREATE_TEXTURE,
        SemioMeshMutation::DeleteTexture(_) => TAG_DELETE_TEXTURE,
        SemioMeshMutation::ChangeTextureMime(_) => TAG_CHANGE_TEXTURE_MIME,
        SemioMeshMutation::ReplaceTextureBytes(_) => TAG_REPLACE_TEXTURE_BYTES,
        SemioMeshMutation::MoveVertex(_) => TAG_MOVE_VERTEX,
    }
}

/// ✂️ Just the `key=value ...` argument tail of `print_semio_mesh_mutation` — the binary frame's
/// `tag` byte already carries the keyword, so the text keyword itself is redundant in the binary
/// payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_mesh_mutation_args(m: &SemioMeshMutation) -> String {
    match print_semio_mesh_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame: `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (its kind's record tag in
/// `💾️binary/📡️.protocol.semio`) as two REAL fixed fields, then the variant's own `key=value ...`
/// argument payload as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_semio_mesh_mutation`/`parse_semio_mesh_mutation` text codec rather than re-deriving a
/// second independent encoding.
impl OpBinary for SemioMeshMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        if let Self::PatchSnapshot(payload) = self {
            let mut out = vec![1, TAG_PATCH_SNAPSHOT];
            out.extend(protocol::OpBinary::encode_op(&payload.patch)?);
            return Ok(out);
        }
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(print_semio_mesh_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        if bytes[1] == TAG_PATCH_SNAPSHOT {
            return Ok(Self::PatchSnapshot(crate::standards::v1::subsets::mesh::schema::mutations::patch_snapshot::PatchSnapshot { patch: protocol::OpBinary::decode_op(&bytes[2..])? }));
        }
        let tag = bytes[1];
        let keyword = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(tag)).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} names no record of 📡️.protocol.semio") })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioMeshMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_CREATE_MESH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-mesh");
const TAG_DELETE_MESH: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-mesh");
const TAG_CREATE_PRIMITIVE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-primitive");
const TAG_DELETE_PRIMITIVE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-primitive");
const TAG_SET_PRIMITIVE_TOPOLOGY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-primitive-topology");
const TAG_REPLACE_PRIMITIVE_GEOMETRY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-primitive-geometry");
const TAG_SET_PRIMITIVE_MATERIAL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-primitive-material");
const TAG_CREATE_MATERIAL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-material");
const TAG_DELETE_MATERIAL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-material");
const TAG_CHANGE_MATERIAL_BASE_COLOR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-material-base-color");
const TAG_CHANGE_MATERIAL_METALLIC: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-material-metallic");
const TAG_CHANGE_MATERIAL_ROUGHNESS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-material-roughness");
const TAG_CREATE_TEXTURE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "create-texture");
const TAG_DELETE_TEXTURE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "delete-texture");
const TAG_CHANGE_TEXTURE_MIME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-texture-mime");
const TAG_REPLACE_TEXTURE_BYTES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-texture-bytes");
const TAG_MOVE_VERTEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "move-vertex");
//#endregion 🏷️WireTags
