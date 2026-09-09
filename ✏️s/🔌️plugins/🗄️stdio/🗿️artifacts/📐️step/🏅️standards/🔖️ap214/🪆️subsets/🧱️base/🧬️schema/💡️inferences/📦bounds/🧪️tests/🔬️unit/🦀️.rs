use super::*;
use crate::schema::snapshot::StepEntity;
use crate::STDIO_STEP_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn point_entity(id: u64, x: f64, y: f64, z: f64) -> StepEntity {
    StepEntity { id, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String(String::new()), StepValue::Aggregate(vec![StepValue::Real(x), StepValue::Real(y), StepValue::Real(z)])], complex: Vec::new() }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_entity_extent() {
    let snapshot = StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: Default::default(),
        entities: vec![point_entity(1, 0.0, 0.0, 0.0), point_entity(2, -5.0, 2.0, 7.0), point_entity(3, 10.0, 3.0, -1.0), StepEntity { id: 4, name: "DIRECTION".into(), args: vec![StepValue::String(String::new())], complex: Vec::new() }],
    };
    let bounds = compute_step_bounds(&snapshot);
    assert_eq!(bounds.min, [-5.0, 0.0, -1.0]);
    assert_eq!(bounds.max, [10.0, 3.0, 7.0]);
    assert_eq!(bounds.point_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = StepSnapshot { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), header: Default::default(), entities: vec![point_entity(1, 1.0, 1.0, 1.0)] };
    assert_eq!(compute_step_bounds(&snapshot), compute_step_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_step_bounds(&StepSnapshot::default()), StepBounds::default());
}
