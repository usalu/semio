use super::*;
use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::dwg::v_ac1024::any::SemioMeshFromDwg;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio_mesh() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: "stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "box".into(),
            primitives: vec![SemioPrimitive {
                id: "box-prim-0".into(),
                topology: SemioTopology::Triangles,
                positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 1.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                indices: vec![0, 1, 2, 0, 2, 3],
                ..SemioPrimitive::default()
            }],
        }],
        ..SemioMeshSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn serialize_then_deserialize_round_trips_triangle_and_vertex_counts() {
    let original = sample_semio_mesh();
    let dwg = semio_framework_plugin::resolve_ready(SemioMeshToDwg::serialize(&original)).expect("serialize");
    assert_eq!(dwg.version, DWG_CODEC_VERSION);
    let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromDwg::deserialize(&dwg)).expect("deserialize");
    assert_eq!(round_tripped.meshes.len(), 1);
    assert_eq!(round_tripped.meshes[0].id, "box");
    let prim = &round_tripped.meshes[0].primitives[0];
    assert_eq!(prim.indices.len(), 6, "quad fan-splits into 2 triangles == 6 indices");
    assert_eq!(prim.positions.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn non_triangle_topology_is_a_hard_error() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].topology = SemioTopology::TriangleFan;
    let err = semio_framework_plugin::resolve_ready(SemioMeshToDwg::serialize(&semio)).expect_err("TriangleFan must error");
    assert!(format!("{err:?}").contains("Triangles"), "got {err:?}");
}
