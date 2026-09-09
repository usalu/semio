use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_dxf() -> DxfSnapshot {
    DxfSnapshot {
        entities: vec![
            DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Circle { center: [2.0, 2.0, 0.0], radius: 1.0, layer: "walls".into(), unknown_group_codes: vec![] },
            DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![] },
        ],
        ..DxfSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn buckets_entities_by_layer_and_drops_unmodeled() {
    let drawing = semio_framework_plugin::resolve_ready(SemioDrawingFromDxf::deserialize(&sample_dxf())).expect("deserialize");
    assert_eq!(drawing.layers.len(), 2);
    assert_eq!(drawing.layers[0].id, "0");
    assert_eq!(drawing.layers[1].id, "walls");
    match &drawing.layers[0].root {
        DrawNode::Group { children, .. } => {
            assert_eq!(children.len(), 1);
            assert!(matches!(children[0], DrawNode::Path { .. }));
        }
        other => panic!("expected Group, got {other:?}"),
    }
}
