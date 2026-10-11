//! 🧬️ SemioMeshSnapshot — meshes -> primitives{topology, positions/normals/uvs/colors, indices,
//! material} + materials (PBR base_color/metallic/roughness) + textures{mime, bytes}. Informed by
//! gltf 2.0's `GltfMesh`/`GltfPrimitive`/`GltfAccessor`/`GltfMaterial`, per the master plan's
//! "Subset snapshot cores" table. Owned types (w1b-type-ownership.md): `SemioMesh`,
//! `SemioPrimitive`, `SemioMaterial`, `SemioTexture` (`SemioPrimitive` was RESERVED at W1b —
//! this file is where it lands).

use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};



//#region 🔖️Topology
/// 🔺️ Primitive draw mode — the gltf 2.0 `mode` enumeration, named (never a bare integer tag).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(Default, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SemioTopology {
    Points,
    Lines,
    LineStrip,
    #[default]
    Triangles,
    TriangleStrip,
    TriangleFan,
}

//#endregion 🔖️Topology

//#region 🔖️Primitive
/// 🔷️ One drawable primitive inside a `SemioMesh` — id-keyed (the strong entity gltf's
/// `mesh.primitives` array lacks; every W2 subset id-keys its repeating structures per the
/// schema-design.md recipe). `positions`/`normals`/`uvs`/`colors`/`indices` are weak, parallel
/// buffer-shaped data — whole-value replaced in diffs, never sub-diffed per vertex.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioPrimitive {
    pub id: String,
    #[value(default)]
    pub topology: SemioTopology,
    #[value(default)]
    pub positions: Vec<SemioPoint3>,
    #[value(default)]
    pub normals: Vec<SemioPoint3>,
    #[value(default)]
    pub uvs: Vec<SemioUv>,
    #[value(default)]
    pub colors: Vec<SemioRgba>,
    #[value(default)]
    pub indices: Vec<u32>,
    #[value(default)]
    pub material_id: Option<String>,
}
//#endregion 🔖️Primitive

//#region 🔖️Mesh
/// 🕸️ A mesh is an id-keyed collection of `SemioPrimitive`s (gltf's `mesh.primitives`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioMesh {
    pub id: String,
    #[value(default)]
    pub primitives: Vec<SemioPrimitive>,
}
//#endregion 🔖️Mesh

//#region 🔖️Material
/// 🎨️ PBR metallic-roughness material (gltf's `material.pbrMetallicRoughness`, the spec-mandated
/// field set per the master plan's row: "materials (PBR base_color/metallic/roughness)").
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioMaterial {
    pub id: String,
    #[value(default)]
    pub base_color: SemioRgba,
    #[value(default)]
    pub metallic: f32,
    #[value(default)]
    pub roughness: f32,
    #[value(default)]
    pub base_color_texture: Option<String>,
    #[value(default)]
    pub metallic_roughness_texture: Option<String>,
    #[value(default)]
    pub normal_texture: Option<String>,
    #[value(default)]
    pub occlusion_texture: Option<String>,
    #[value(default)]
    pub emissive_texture: Option<String>,
}
//#endregion 🔖️Material

//#region 🔖️Texture
/// 🖼️ Raw texture payload (gltf's `image` + embedded `bufferView`/data-uri collapsed into one
/// typed-raw-retention entity — mime + bytes, per the master plan's row: "textures{mime, bytes}").
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, value_derive::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct SemioTexture {
    pub id: String,
    #[value(default)]
    pub mime: String,
    #[value(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Texture

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOMESH_DOCUMENT_SCHEMA: &str = "stdio.semio.mesh";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, value_derive::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.mesh")]
pub struct SemioMeshSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub meshes: Vec<SemioMesh>,
    #[state(artifact)]
    #[value(default)]
    pub materials: Vec<SemioMaterial>,
    #[state(artifact)]
    #[value(default)]
    pub textures: Vec<SemioTexture>,
}

impl Default for SemioMeshSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: Default::default(), materials: Default::default(), textures: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives




































//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives

















//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Demo
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES) — pure snapshot constructor, no codec/IO concern.
///
/// 🌱 The demo `s.stdio.semio.mesh` document — single source of truth for
/// `📚️examples/🧊️cube/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and conformance laws.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_mesh_snapshot() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(),
        meshes: vec![SemioMesh {
            id: "mesh-1".into(),
            primitives: vec![SemioPrimitive {
                id: "prim-1".into(),
                topology: SemioTopology::Triangles,
                positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3],
                uvs: vec![SemioUv { u: 0.0, v: 0.0 }, SemioUv { u: 1.0, v: 0.0 }, SemioUv { u: 0.0, v: 1.0 }],
                colors: vec![SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }; 3],
                indices: vec![0, 1, 2],
                material_id: Some("mat-1".into()),
            }],
        }],
        materials: vec![SemioMaterial { id: "mat-1".into(), base_color: SemioRgba { r: 0.8, g: 0.2, b: 0.2, a: 1.0 }, metallic: 0.1, roughness: 0.6, ..Default::default() }],
        textures: vec![SemioTexture { id: "tex-1".into(), mime: "image/png".into(), bytes: vec![0x89, 0x50, 0x4e, 0x47] }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests







