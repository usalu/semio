use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioUv};
use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio_mesh() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: "stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "tri".into(),
            primitives: vec![SemioPrimitive {
                id: "tri-prim-0".into(),
                topology: SemioTopology::Triangles,
                positions: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }],
                normals: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3],
                uvs: vec![SemioUv { u: 0.0, v: 0.0 }, SemioUv { u: 1.0, v: 0.0 }, SemioUv { u: 0.0, v: 1.0 }],
                colors: Vec::new(),
                indices: Vec::new(),
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
    let obj = semio_framework_plugin::resolve_ready(SemioMeshToObj::serialize(&original)).expect("serialize");
    assert_eq!(obj.vertices.len(), 3);
    assert_eq!(obj.faces.len(), 1);
    assert_eq!(obj.objects.len(), 1);
    assert_eq!(obj.objects[0].name, "tri");
    let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromObj::deserialize(&obj)).expect("deserialize");
    assert_eq!(original, round_tripped);
}

#[semio_framework_async_macros::async_test]
async fn non_triangle_topology_is_a_hard_error() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].topology = SemioTopology::TriangleFan;
    let err = semio_framework_plugin::resolve_ready(SemioMeshToObj::serialize(&semio)).expect_err("TriangleFan must error");
    assert!(format!("{err:?}").contains("Triangles"), "got {err:?}");
}
