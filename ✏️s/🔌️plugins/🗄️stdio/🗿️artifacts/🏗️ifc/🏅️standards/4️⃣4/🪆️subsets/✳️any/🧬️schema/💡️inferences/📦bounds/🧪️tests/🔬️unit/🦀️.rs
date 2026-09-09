use super::*;
use crate::schema::snapshot::IfcEntity;
use crate::STDIO_IFC_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn point_entity(id: u64, x: f64, y: f64, z: f64) -> IfcEntity {
    IfcEntity { id, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(x), IfcValue::Real(y), IfcValue::Real(z)])], complex: Vec::new() }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_entity_extent() {
    let snapshot = IfcSnapshot {
        schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        header: Default::default(),
        entities: vec![point_entity(1, 0.0, 0.0, 0.0), point_entity(2, -3.0, 6.0, 12.0), point_entity(3, 9.0, -1.0, 4.0), IfcEntity { id: 4, name: "IFCOWNERHISTORY".into(), args: vec![IfcValue::Unset], complex: Vec::new() }],
    };
    let bounds = compute_ifc_bounds(&snapshot);
    assert_eq!(bounds.min, [-3.0, -1.0, 0.0]);
    assert_eq!(bounds.max, [9.0, 6.0, 12.0]);
    assert_eq!(bounds.point_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = IfcSnapshot { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), header: Default::default(), entities: vec![point_entity(1, 1.0, 1.0, 1.0)] };
    assert_eq!(compute_ifc_bounds(&snapshot), compute_ifc_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_ifc_bounds(&IfcSnapshot::default()), IfcBounds::default());
}
