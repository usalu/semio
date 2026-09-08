
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba};
use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioPrimitive;
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
                normals: Vec::new(),
                uvs: Vec::new(),
                colors: vec![SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }, SemioRgba { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }, SemioRgba { r: 1.0, g: 1.0, b: 0.0, a: 1.0 }],
                indices: vec![0, 1, 2, 0, 2, 3],
                material_id: None,
            }],
        }],
        materials: Vec::new(),
        textures: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn serialize_then_deserialize_round_trips_at_the_semio_level() {
    let original = sample_semio_mesh();
    let ply = semio_framework_plugin::resolve_ready(SemioMeshToPly::serialize(&original)).expect("serialize");
    assert_eq!(ply.elements[0].name, "vertex");
    assert_eq!(ply.elements[0].rows.len(), 4);
    assert_eq!(ply.elements[1].name, "face");
    assert_eq!(ply.elements[1].rows.len(), 2);
    let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromPly::deserialize(&ply)).expect("deserialize");
    assert_eq!(original.meshes[0].primitives[0].positions, round_tripped.meshes[0].primitives[0].positions);
    assert_eq!(original.meshes[0].primitives[0].colors, round_tripped.meshes[0].primitives[0].colors);
    assert_eq!(original.meshes[0].primitives[0].indices, round_tripped.meshes[0].primitives[0].indices);
}

#[semio_framework_async_macros::async_test]
async fn non_uniform_color_presence_is_a_hard_error() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives.push(SemioPrimitive {
        id: "prim-no-color".into(),
        topology: SemioTopology::Points,
        positions: vec![SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }],
        normals: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        indices: Vec::new(),
        material_id: None,
    });
    let err = semio_framework_plugin::resolve_ready(SemioMeshToPly::serialize(&semio)).expect_err("mixed color presence must error");
    assert!(format!("{err:?}").contains("colors"), "got {err:?}");
}

#[semio_framework_async_macros::async_test]
async fn non_triangle_non_points_topology_is_a_hard_error() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].topology = SemioTopology::LineStrip;
    let err = semio_framework_plugin::resolve_ready(SemioMeshToPly::serialize(&semio)).expect_err("LineStrip must error");
    assert!(format!("{err:?}").contains("Triangles/Points"), "got {err:?}");
}
