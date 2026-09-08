
use super::*;
use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio_mesh() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: "stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "pyramid".into(),
            primitives: vec![SemioPrimitive {
                id: "pyramid-prim-0".into(),
                topology: SemioTopology::Triangles,
                positions: vec![
                    SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 },
                    SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 },
                    SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 },
                    SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 },
                    SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                    SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 },
                ],
                normals: vec![
                    SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                    SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                    SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
                    SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 },
                    SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 },
                    SemioPoint3 { x: 0.0, y: -1.0, z: 0.0 },
                ],
                uvs: Vec::new(),
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
    let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&original)).expect("serialize");
    assert_eq!(stl.solid_name, "pyramid");
    assert_eq!(stl.triangles.len(), 2);
    assert_eq!(stl.triangles[0].normal, [0.0, 0.0, 1.0]);
    let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromStl::deserialize(&stl)).expect("deserialize");
    assert_eq!(original, round_tripped, "uniform per-triangle normals must average back exactly");
}

#[semio_framework_async_macros::async_test]
async fn non_triangle_topology_is_a_hard_error() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].topology = SemioTopology::Lines;
    let err = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&semio)).expect_err("Lines topology must error");
    assert!(format!("{err:?}").contains("Triangles"), "got {err:?}");
}

#[semio_framework_async_macros::async_test]
async fn indexed_triangles_export_correctly() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].positions = vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }];
    semio.meshes[0].primitives[0].normals = vec![SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }; 3];
    semio.meshes[0].primitives[0].indices = vec![0, 1, 2];
    let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&semio)).expect("serialize");
    assert_eq!(stl.triangles.len(), 1);
    assert_eq!(stl.triangles[0].vertices, [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
}
