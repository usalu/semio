
use super::*;
use crate::standards::v1::subsets::cad::schema::snapshot::{CadBlock, CadEntityRecord, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn point(x: f64, y: f64) -> SemioPoint2 {
    SemioPoint2 { x, y }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn record(handle: &str, layer: &str, entity: CadEntity) -> CadEntityRecord {
    CadEntityRecord { handle: handle.into(), layer: layer.into(), entity }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_entity_extent() {
    let snapshot = SemioCadSnapshot {
        schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: Vec::new(),
        blocks: vec![CadBlock { name: "b1".into(), base_point: point(0.0, 0.0), entities: vec![record("h2", "0", CadEntity::Circle { center: point(5.0, 7.5), radius: 1.0 })] }],
        entities: vec![record("h0", "0", CadEntity::Line { a: point(-2.0, 1.0), b: point(0.0, 2.0) }), record("h1", "0", CadEntity::Polyline { vertices: vec![point(1.0, 1.0), point(3.0, 4.0)], closed: false })],
    };
    let bounds = compute_semio_cad_bounds(&snapshot);
    assert_eq!(bounds.min, point(-2.0, 1.0));
    assert_eq!(bounds.max, point(6.0, 8.5));
    assert_eq!(bounds.entity_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SemioCadSnapshot { schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(), layers: Vec::new(), blocks: Vec::new(), entities: vec![record("h0", "0", CadEntity::Line { a: point(0.0, 0.0), b: point(1.0, 1.0) })] };
    assert_eq!(compute_semio_cad_bounds(&snapshot), compute_semio_cad_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_cad_bounds(&SemioCadSnapshot::default()), SemioCadBounds::default());
}
