//! 📝️ Text representation codec surface for `s.stdio.semio.mesh` (mutations) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

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
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_rgba};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_rgba};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_point3};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_point3};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_list};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_list};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{dec_str};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{enc_str};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{hex_decode};
use crate::standards::v1::subsets::mesh::io::text::snapshot::{hex_encode};
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

/// 📥️ Decodes this facet's own externally-tagged (`{"<VariantName>": {<snake_case payload>}}`)
/// JSON projection — no `#[value(rename_all)]` sits on this enum or its payload structs, which is
/// exactly the shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors
/// carry — into a real [`SemioMeshMutation`]. `create-primitive`'s payload embeds a whole `MeshPrimitive`, so decoding it from the committed
/// vector is the only way the adapter can exercise that kind without restating every coordinate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_mesh_mutation_json(text: &str) -> Result<SemioMeshMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// ⚡️ Real hand-rolled `OpText`, grammar `keyword arg=value ...` — reusing the sibling `🔺️diff`
/// facet's `pub(crate)` hex/value primitives (one source of truth for entity encoding, same
/// convention this file's pre-rewrite version and `🧊️brep`/`🌊️flow`'s mutations facets establish).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_semio_mesh_mutation(m: &SemioMeshMutation) -> String {
    match m {
        SemioMeshMutation::PatchSnapshot(payload) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(&payload.patch),
        SemioMeshMutation::SetSnapshot(p) => format!("set-snapshot snapshot={}", hex_encode(semio_framework_pack_json::to_json_string(&p.snapshot).as_bytes())),
        SemioMeshMutation::CreateMesh(p) => format!("create-mesh mesh={}", enc_mesh(&p.mesh)),
        SemioMeshMutation::DeleteMesh(p) => format!("delete-mesh id={}", enc_str(&p.id)),
        SemioMeshMutation::CreatePrimitive(p) => format!("create-primitive mesh-id={} primitive={}", enc_str(&p.mesh_id), enc_primitive(&p.primitive)),
        SemioMeshMutation::DeletePrimitive(p) => format!("delete-primitive mesh-id={} primitive-id={}", enc_str(&p.mesh_id), enc_str(&p.primitive_id)),
        SemioMeshMutation::SetPrimitiveTopology(p) => format!("set-primitive-topology mesh-id={} primitive-id={} topology={}", enc_str(&p.mesh_id), enc_str(&p.primitive_id), enc_topology(&p.topology)),
        SemioMeshMutation::ReplacePrimitiveGeometry(p) => format!(
            "replace-primitive-geometry mesh-id={} primitive-id={} positions={} normals={} uvs={} colors={} indices={}",
            enc_str(&p.mesh_id),
            enc_str(&p.primitive_id),
            enc_list(&p.positions, enc_point3),
            enc_list(&p.normals, enc_point3),
            enc_list(&p.uvs, enc_uv),
            enc_list(&p.colors, enc_rgba),
            enc_list(&p.indices, |v: &u32| v.to_string()),
        ),
        SemioMeshMutation::SetPrimitiveMaterial(p) => format!("set-primitive-material mesh-id={} primitive-id={} material-id={}", enc_str(&p.mesh_id), enc_str(&p.primitive_id), encode_option(&p.material_id, |v: &String| enc_str(v))),
        SemioMeshMutation::CreateMaterial(p) => format!("create-material material={}", enc_material(&p.material)),
        SemioMeshMutation::DeleteMaterial(p) => format!("delete-material id={}", enc_str(&p.id)),
        SemioMeshMutation::ChangeMaterialBaseColor(p) => format!("change-material-base-color id={} new-base-color={}", enc_str(&p.id), enc_rgba(&p.new_base_color)),
        SemioMeshMutation::ChangeMaterialMetallic(p) => format!("change-material-metallic id={} new-metallic={}", enc_str(&p.id), p.new_metallic),
        SemioMeshMutation::ChangeMaterialRoughness(p) => format!("change-material-roughness id={} new-roughness={}", enc_str(&p.id), p.new_roughness),
        SemioMeshMutation::CreateTexture(p) => format!("create-texture texture={}", enc_texture(&p.texture)),
        SemioMeshMutation::DeleteTexture(p) => format!("delete-texture id={}", enc_str(&p.id)),
        SemioMeshMutation::ChangeTextureMime(p) => format!("change-texture-mime id={} new-mime={}", enc_str(&p.id), enc_str(&p.new_mime)),
        SemioMeshMutation::ReplaceTextureBytes(p) => format!("replace-texture-bytes id={} new-bytes={}", enc_str(&p.id), hex_encode(&p.new_bytes)),
        SemioMeshMutation::MoveVertex(p) => format!("move-vertex mesh-id={} primitive-id={} vertex-index={} new-point={}", enc_str(&p.mesh_id), enc_str(&p.primitive_id), p.vertex_index, enc_point3(&p.new_point)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_semio_mesh_mutation(line: &str) -> Result<SemioMeshMutation, String> {
    if let Some(source) = line.strip_prefix("patch-snapshot patch=") {
        let patch = semio_s_artifact_stdio_contract::editing::snapshot_patch_from_hex(source)?;
        return Ok(SemioMeshMutation::PatchSnapshot(crate::standards::v1::subsets::mesh::schema::mutations::patch_snapshot::PatchSnapshot { patch }));
    }
    if let Some(payload) = line.strip_prefix("set-snapshot snapshot=") {
        let bytes = hex_decode(payload)?;
        let json = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        let parsed = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let snapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
        return Ok(SemioMeshMutation::SetSnapshot(SetSnapshot { snapshot }));
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("semio mesh mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("semio mesh mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "create-mesh" => Ok(SemioMeshMutation::CreateMesh(create_mesh::CreateMesh { mesh: dec_mesh(arg("mesh")?)? })),
        "delete-mesh" => Ok(SemioMeshMutation::DeleteMesh(delete_mesh::DeleteMesh { id: dec_str(arg("id")?)? })),
        "create-primitive" => Ok(SemioMeshMutation::CreatePrimitive(create_primitive::CreatePrimitive { mesh_id: dec_str(arg("mesh-id")?)?, primitive: dec_primitive(arg("primitive")?)? })),
        "delete-primitive" => Ok(SemioMeshMutation::DeletePrimitive(delete_primitive::DeletePrimitive { mesh_id: dec_str(arg("mesh-id")?)?, primitive_id: dec_str(arg("primitive-id")?)? })),
        "set-primitive-topology" => {
            Ok(SemioMeshMutation::SetPrimitiveTopology(set_primitive_topology::SetPrimitiveTopology { mesh_id: dec_str(arg("mesh-id")?)?, primitive_id: dec_str(arg("primitive-id")?)?, topology: dec_topology(arg("topology")?)? }))
        }
        "replace-primitive-geometry" => Ok(SemioMeshMutation::ReplacePrimitiveGeometry(replace_primitive_geometry::ReplacePrimitiveGeometry {
            mesh_id: dec_str(arg("mesh-id")?)?,
            primitive_id: dec_str(arg("primitive-id")?)?,
            positions: dec_list(arg("positions")?, dec_point3)?,
            normals: dec_list(arg("normals")?, dec_point3)?,
            uvs: dec_list(arg("uvs")?, dec_uv)?,
            colors: dec_list(arg("colors")?, dec_rgba)?,
            indices: dec_list(arg("indices")?, |t| t.parse::<u32>().map_err(|e: std::num::ParseIntError| e.to_string()))?,
        })),
        "set-primitive-material" => {
            Ok(SemioMeshMutation::SetPrimitiveMaterial(set_primitive_material::SetPrimitiveMaterial { mesh_id: dec_str(arg("mesh-id")?)?, primitive_id: dec_str(arg("primitive-id")?)?, material_id: decode_option(arg("material-id")?, dec_str)? }))
        }
        "create-material" => Ok(SemioMeshMutation::CreateMaterial(create_material::CreateMaterial { material: dec_material(arg("material")?)? })),
        "delete-material" => Ok(SemioMeshMutation::DeleteMaterial(delete_material::DeleteMaterial { id: dec_str(arg("id")?)? })),
        "change-material-base-color" => Ok(SemioMeshMutation::ChangeMaterialBaseColor(change_material_base_color::ChangeMaterialBaseColor { id: dec_str(arg("id")?)?, new_base_color: dec_rgba(arg("new-base-color")?)? })),
        "change-material-metallic" => {
            Ok(SemioMeshMutation::ChangeMaterialMetallic(change_material_metallic::ChangeMaterialMetallic { id: dec_str(arg("id")?)?, new_metallic: arg("new-metallic")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? }))
        }
        "change-material-roughness" => {
            Ok(SemioMeshMutation::ChangeMaterialRoughness(change_material_roughness::ChangeMaterialRoughness { id: dec_str(arg("id")?)?, new_roughness: arg("new-roughness")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? }))
        }
        "create-texture" => Ok(SemioMeshMutation::CreateTexture(create_texture::CreateTexture { texture: dec_texture(arg("texture")?)? })),
        "delete-texture" => Ok(SemioMeshMutation::DeleteTexture(delete_texture::DeleteTexture { id: dec_str(arg("id")?)? })),
        "change-texture-mime" => Ok(SemioMeshMutation::ChangeTextureMime(change_texture_mime::ChangeTextureMime { id: dec_str(arg("id")?)?, new_mime: dec_str(arg("new-mime")?)? })),
        "replace-texture-bytes" => Ok(SemioMeshMutation::ReplaceTextureBytes(replace_texture_bytes::ReplaceTextureBytes { id: dec_str(arg("id")?)?, new_bytes: hex_decode(arg("new-bytes")?)? })),
        "move-vertex" => Ok(SemioMeshMutation::MoveVertex(move_vertex::MoveVertex {
            mesh_id: dec_str(arg("mesh-id")?)?,
            primitive_id: dec_str(arg("primitive-id")?)?,
            vertex_index: arg("vertex-index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            new_point: dec_point3(arg("new-point")?)?,
        })),
        other => Err(format!("semio mesh mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioMeshMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_semio_mesh_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_semio_mesh_mutation(self)
    }
}
}
pub use mutations_codec::*;
