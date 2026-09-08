
use super::*;
use crate::schema::snapshot::DxfBlock;

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_entity_extent() {
    let snapshot = DxfSnapshot {
        schema: "s.stdio.dxf".into(),
        header_vars: Vec::new(),
        tables: Default::default(),
        other_tables: Vec::new(),
        blocks: vec![DxfBlock { name: "b1".into(), base_point: [0.0, 0.0, 0.0], entities: vec![DxfEntity::Circle { center: [5.0, 7.5, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] }],
        entities: vec![DxfEntity::Line { start: [-2.0, 1.0, 0.0], end: [0.0, 2.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }, DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![] }],
    };
    let bounds = compute_dxf_bounds(&snapshot);
    assert_eq!(bounds.min, [-2.0, 1.0, -1.0]);
    assert_eq!(bounds.max, [6.0, 8.5, 1.0]);
    assert_eq!(bounds.entity_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DxfSnapshot {
        schema: "s.stdio.dxf".into(),
        header_vars: Vec::new(),
        tables: Default::default(),
        other_tables: Vec::new(),
        blocks: Vec::new(),
        entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 1.0], layer: "0".into(), unknown_group_codes: vec![] }],
    };
    assert_eq!(compute_dxf_bounds(&snapshot), compute_dxf_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_dxf_bounds(&DxfSnapshot::default()), DxfBounds::default());
}
