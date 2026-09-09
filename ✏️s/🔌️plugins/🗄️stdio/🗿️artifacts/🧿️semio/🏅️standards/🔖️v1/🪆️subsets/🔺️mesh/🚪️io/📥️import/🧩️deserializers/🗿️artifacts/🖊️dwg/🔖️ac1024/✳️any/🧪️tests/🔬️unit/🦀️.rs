use super::*;
use semio_s_artifact_stdio_dwg::schema::snapshot::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg::{DwgColor, DwgEntity};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_dwg() -> DwgSnapshot {
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("walls");
    drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] } });
    DwgSnapshot { version: "AC1015".into(), drawing: DwgLogicalDrawing::from_native(&drawing).expect("valid sample drawing"), ..DwgSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn groups_polyface_mesh_by_layer_name() {
    let semio = semio_framework_plugin::resolve_ready(SemioMeshFromDwg::deserialize(&sample_dwg())).expect("deserialize");
    assert_eq!(semio.meshes.len(), 1);
    assert_eq!(semio.meshes[0].id, "walls");
    let prim = &semio.meshes[0].primitives[0];
    assert_eq!(prim.positions.len(), 4);
    assert_eq!(prim.indices.len(), 6, "quad face splits into 2 triangles");
    assert_eq!(prim.topology, SemioTopology::Triangles);
}

#[semio_framework_async_macros::async_test]
async fn rejects_malformed_payload() {
    let bad = DwgSnapshot { drawing: DwgLogicalDrawing { extmax: vec![0.0], ..Default::default() }, ..DwgSnapshot::default() };
    assert!(semio_framework_plugin::resolve_ready(SemioMeshFromDwg::deserialize(&bad)).is_err());
}
