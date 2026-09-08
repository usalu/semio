
use super::*;
use semio_s_artifact_stdio_ply::schema::snapshot::{PlyElement, PlyFormat, PlyRow};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_ply() -> PlySnapshot {
    let vertex = PlyElement {
        name: "vertex".into(),
        count: 4,
        properties: vec![
            PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float },
            PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float },
            PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float },
            PlyProperty::Scalar { name: "red".into(), kind: PlyScalarType::UChar },
            PlyProperty::Scalar { name: "green".into(), kind: PlyScalarType::UChar },
            PlyProperty::Scalar { name: "blue".into(), kind: PlyScalarType::UChar },
        ],
        rows: vec![
            PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::UChar(255), PlyValue::UChar(0), PlyValue::UChar(0)] },
            PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::UChar(0), PlyValue::UChar(255), PlyValue::UChar(0)] },
            PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(1.0), PlyValue::Float(0.0), PlyValue::UChar(0), PlyValue::UChar(0), PlyValue::UChar(255)] },
            PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(1.0), PlyValue::Float(0.0), PlyValue::UChar(255), PlyValue::UChar(255), PlyValue::UChar(0)] },
        ],
    };
    let face = PlyElement {
        name: "face".into(),
        count: 1,
        properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
        rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2), PlyValue::Int(3)])] }],
    };
    PlySnapshot { schema: "stdio.ply".into(), format: PlyFormat::Ascii, comments: Vec::new(), elements: vec![vertex, face] }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_builds_a_real_indexed_mesh_with_colors() {
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromPly::deserialize(&sample_ply())).expect("deserialize");
    let prim = &semio.meshes[0].primitives[0];
    assert_eq!(prim.topology, SemioTopology::Triangles);
    assert_eq!(prim.positions.len(), 4, "vertex pool stays 4 entries -- a real shared index space");
    assert_eq!(prim.colors.len(), 4);
    assert_eq!(prim.colors[0], SemioRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    assert_eq!(prim.indices, vec![0, 1, 2, 0, 2, 3], "quad face fan-triangulated, referencing the shared vertex pool");
    assert!(prim.normals.is_empty() && prim.uvs.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn no_face_element_yields_a_points_primitive() {
    let mut ply = sample_ply();
    ply.elements.retain(|e| e.name != "face");
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromPly::deserialize(&ply)).expect("deserialize");
    let prim = &semio.meshes[0].primitives[0];
    assert_eq!(prim.topology, SemioTopology::Points);
    assert!(prim.indices.is_empty());
    assert_eq!(prim.positions.len(), 4);
}
