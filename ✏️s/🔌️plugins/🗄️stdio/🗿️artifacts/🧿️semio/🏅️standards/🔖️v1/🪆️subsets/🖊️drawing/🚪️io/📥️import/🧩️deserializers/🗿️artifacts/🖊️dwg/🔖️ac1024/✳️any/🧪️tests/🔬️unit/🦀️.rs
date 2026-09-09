use super::*;
use semio_s_artifact_stdio_dwg::schema::snapshot::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg::{DwgColor, DwgEntity};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_dwg() -> DwgSnapshot {
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("annotations");
    drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: false, elevation: 0.0, vertices: vec![[0.0, 0.0], [5.0, 0.0]], bulges: vec![0.0, 0.0] } });
    drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Text { at: [1.0, 1.0, 0.0], height: 1.0, rotation: 0.0, content: "hi".into() } });
    DwgSnapshot { version: "AC1015".into(), drawing: DwgLogicalDrawing::from_native(&drawing).expect("valid sample drawing"), ..DwgSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn buckets_entities_by_layer_in_entity_order() {
    let drawing = semio_framework_plugin::resolve_ready(SemioDrawingFromDwg::deserialize(&sample_dwg())).expect("deserialize");
    assert_eq!(drawing.layers.len(), 1);
    assert_eq!(drawing.layers[0].id, "annotations");
    match &drawing.layers[0].root {
        DrawNode::Group { children, .. } => {
            assert_eq!(children.len(), 2);
            assert!(matches!(children[0], DrawNode::Path { .. }));
            assert!(matches!(children[1], DrawNode::Text { .. }));
        }
        other => panic!("expected Group, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn rejects_malformed_payload() {
    let bad = DwgSnapshot { drawing: DwgLogicalDrawing { extmin: vec![0.0], ..Default::default() }, ..DwgSnapshot::default() };
    assert!(semio_framework_plugin::resolve_ready(SemioDrawingFromDwg::deserialize(&bad)).is_err());
}
