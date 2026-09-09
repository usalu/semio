use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioPrimitive, SemioTexture};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio_mesh() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: "stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "quad".into(),
            primitives: vec![SemioPrimitive {
                id: "quad-prim-0".into(),
                topology: SemioTopology::Triangles,
                positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 1.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 4],
                uvs: vec![SemioUv { u: 0.0, v: 0.0 }, SemioUv { u: 1.0, v: 0.0 }, SemioUv { u: 1.0, v: 1.0 }, SemioUv { u: 0.0, v: 1.0 }],
                colors: vec![SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }; 4],
                indices: vec![0, 1, 2, 0, 2, 3],
                material_id: Some("mat-0".into()),
            }],
        }],
        materials: vec![SemioMaterial { id: "mat-0".into(), base_color: SemioRgba { r: 0.8, g: 0.1, b: 0.1, a: 1.0 }, metallic: 0.2, roughness: 0.7 }],
        textures: vec![SemioTexture { id: "tex-0".into(), mime: "image/png".into(), bytes: vec![0x89, 0x50, 0x4e, 0x47] }],
    }
}

#[semio_framework_async_macros::async_test]
async fn serialize_then_deserialize_round_trips_at_the_semio_level() {
    let original = sample_semio_mesh();
    let gltf = semio_framework_plugin::resolve_ready(SemioMeshToGltf::serialize(&original)).expect("serialize");
    assert_eq!(gltf.document.meshes.len(), 1);
    assert_eq!(gltf.document.meshes[0].primitives[0].mode, Some(4));
    let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).expect("deserialize");
    assert_eq!(original, round_tripped, "semio mesh -> gltf -> semio mesh must be stable (documented lossy fields excepted, none apply here)");
}

#[semio_framework_async_macros::async_test]
async fn unknown_material_reference_is_a_hard_error() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].material_id = Some("does-not-exist".into());
    let err = semio_framework_plugin::resolve_ready(SemioMeshToGltf::serialize(&semio)).expect_err("dangling material ref must error");
    assert!(format!("{err:?}").contains("does-not-exist"), "got {err:?}");
}

#[semio_framework_async_macros::async_test]
async fn empty_positions_is_a_hard_error_not_a_fabricated_accessor() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].positions.clear();
    let err = semio_framework_plugin::resolve_ready(SemioMeshToGltf::serialize(&semio)).expect_err("empty positions must error");
    assert!(format!("{err:?}").contains("positions"), "got {err:?}");
}
