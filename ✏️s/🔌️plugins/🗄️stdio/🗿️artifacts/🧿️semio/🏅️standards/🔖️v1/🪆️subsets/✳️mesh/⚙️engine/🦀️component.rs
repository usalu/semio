//! ⚙️ Semio mesh engine — demo snapshot construction and semio DSL/pack wire helpers for the
//! `s.stdio.semio/v1/mesh` owning subset. Geometry primitives (`SemioPoint3`, `SemioUv`, `SemioRgba`)
//! remain shared under `✳️any/⚙️engine/🧮️geometry`; this module owns mesh-specific demo + wire.

use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{
    SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology,
    STDIO_SEMIOMESH_DOCUMENT_SCHEMA,
};

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.mesh` document — single source of truth for
/// `📚️examples/🧊️cube/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` and conformance laws.
pub fn demo_mesh_snapshot() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(),
        meshes: vec![SemioMesh {
            id: "mesh-1".into(),
            primitives: vec![SemioPrimitive {
                id: "prim-1".into(),
                topology: SemioTopology::Triangles,
                positions: vec![
                    SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 },
                    SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 },
                    SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 },
                ],
                normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3],
                uvs: vec![
                    SemioUv { u: 0.0, v: 0.0 },
                    SemioUv { u: 1.0, v: 0.0 },
                    SemioUv { u: 0.0, v: 1.0 },
                ],
                colors: vec![SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }; 3],
                indices: vec![0, 1, 2],
                material_id: Some("mat-1".into()),
            }],
        }],
        materials: vec![SemioMaterial {
            id: "mat-1".into(),
            base_color: SemioRgba { r: 0.8, g: 0.2, b: 0.2, a: 1.0 },
            metallic: 0.1,
            roughness: 0.6,
        }],
        textures: vec![SemioTexture {
            id: "tex-1".into(),
            mime: "image/png".into(),
            bytes: vec![0x89, 0x50, 0x4e, 0x47],
        }],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Wire
/// 📝 Parse mesh subset DSL text into a `SemioMeshSnapshot`.
pub fn parse_mesh_dsl(text: &str) -> Result<SemioMeshSnapshot, store::TextError> {
    <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 📝 Render a `SemioMeshSnapshot` as mesh subset DSL text.
pub fn print_mesh_dsl(snapshot: &SemioMeshSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦 Encode a `SemioMeshSnapshot` as a semio pack envelope.
pub fn encode_mesh_pack(snapshot: &SemioMeshSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📦 Decode a semio pack envelope into a `SemioMeshSnapshot`.
pub fn decode_mesh_pack(bytes: &[u8]) -> Result<SemioMeshSnapshot, store::PackError> {
    <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
//#endregion 🔖️Wire
