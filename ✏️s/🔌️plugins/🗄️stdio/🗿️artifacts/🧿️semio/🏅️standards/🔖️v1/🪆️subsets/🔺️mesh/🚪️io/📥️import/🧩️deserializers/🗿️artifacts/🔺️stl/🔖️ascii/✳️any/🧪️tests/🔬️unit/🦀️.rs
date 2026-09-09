use super::*;
use semio_s_artifact_stdio_stl::schema::snapshot::StlTriangle;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_stl() -> StlSnapshot {
    StlSnapshot {
        schema: "stdio.stl".into(),
        solid_name: "pyramid".into(),
        triangles: vec![StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }, StlTriangle { normal: [0.0, -1.0, 0.0], vertices: [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]] }],
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_expands_face_normals_and_flattens_triangle_soup() {
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromStl::deserialize(&sample_stl())).expect("deserialize");
    assert_eq!(semio.meshes.len(), 1);
    assert_eq!(semio.meshes[0].id, "pyramid");
    let prim = &semio.meshes[0].primitives[0];
    assert_eq!(prim.topology, SemioTopology::Triangles);
    assert_eq!(prim.positions.len(), 6);
    assert_eq!(prim.normals.len(), 6);
    assert_eq!(prim.normals[0], SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 });
    assert_eq!(prim.normals[1], prim.normals[0], "per-facet normal expanded identically to every vertex of that facet");
    assert!(prim.indices.is_empty(), "STL has no shared-index concept; empty indices means sequential draw");
    assert!(prim.uvs.is_empty() && prim.colors.is_empty() && prim.material_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn empty_solid_name_falls_back_to_a_generated_mesh_id() {
    let mut stl = sample_stl();
    stl.solid_name.clear();
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromStl::deserialize(&stl)).expect("deserialize");
    assert_eq!(semio.meshes[0].id, "mesh-0");
}
