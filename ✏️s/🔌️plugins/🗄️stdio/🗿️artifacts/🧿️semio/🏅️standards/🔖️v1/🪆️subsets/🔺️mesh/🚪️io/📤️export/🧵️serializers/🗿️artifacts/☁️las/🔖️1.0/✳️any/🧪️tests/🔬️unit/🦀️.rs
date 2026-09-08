
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba};
use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;
use crate::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioPrimitive, SemioTopology};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio_mesh() -> SemioMeshSnapshot {
    SemioMeshSnapshot {
        schema: "stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "cloud".into(),
            primitives: vec![SemioPrimitive {
                id: "cloud-prim-0".into(),
                topology: SemioTopology::Points,
                positions: vec![SemioPoint3 { x: 1.23, y: 4.56, z: 7.89 }, SemioPoint3 { x: -2.5, y: 0.0, z: 10.0 }],
                normals: Vec::new(),
                uvs: Vec::new(),
                colors: vec![SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, SemioRgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }],
                indices: Vec::new(),
                material_id: None,
            }],
        }],
        materials: Vec::new(),
        textures: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn serialize_then_deserialize_round_trips_positions_and_colors_within_las_quantization() {
    let original = sample_semio_mesh();
    let las = semio_framework_plugin::resolve_ready(SemioMeshToLas::serialize(&original)).expect("serialize");
    assert_eq!(las.points.len(), 2);
    assert_eq!(las.points[0].rgb, Some((65535, 0, 0)));
    let round_tripped = semio_framework_plugin::resolve_ready(SemioMeshFromLas::deserialize(&las)).expect("deserialize");
    let orig_prim = &original.meshes[0].primitives[0];
    let rt_prim = &round_tripped.meshes[0].primitives[0];
    assert_eq!(rt_prim.topology, SemioTopology::Points);
    assert_eq!(rt_prim.positions.len(), orig_prim.positions.len());
    for (a, b) in orig_prim.positions.iter().zip(&rt_prim.positions) {
        assert!((a.x - b.x).abs() < 1e-6, "x drifted beyond LAS's documented scale quantization: {a:?} vs {b:?}");
        assert!((a.y - b.y).abs() < 1e-6);
        assert!((a.z - b.z).abs() < 1e-6);
    }
    assert_eq!(orig_prim.colors, rt_prim.colors, "0.0/1.0 channel extremes must survive the u16 round trip exactly");
}

#[semio_framework_async_macros::async_test]
async fn triangle_primitive_flattens_to_a_point_cloud_no_error() {
    let mut semio = sample_semio_mesh();
    semio.meshes[0].primitives[0].topology = SemioTopology::Triangles;
    semio.meshes[0].primitives[0].indices = vec![0, 1, 0];
    let las = semio_framework_plugin::resolve_ready(SemioMeshToLas::serialize(&semio)).expect("triangle -> point-cloud flatten must succeed, not error");
    assert_eq!(las.points.len(), 2, "connectivity dropped, positions kept");
}
