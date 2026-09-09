use super::*;
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::STDIO_LAS_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot_with(header: LasHeader, points: Vec<LasPoint>) -> LasSnapshot {
    LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), header, vlrs: Vec::<LasVlr>::new(), points }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_header_extent() {
    let header = LasHeader { min_x: -12.5, min_y: 0.0, min_z: -3.25, max_x: 100.0, max_y: 88.75, max_z: 15.0, number_of_point_records: 3, ..LasHeader::default() };
    let snapshot = snapshot_with(header, vec![LasPoint::default(), LasPoint::default(), LasPoint::default()]);
    let bounds = compute_las_bounds(&snapshot);
    assert_eq!(bounds.min_x, -12.5);
    assert_eq!(bounds.min_y, 0.0);
    assert_eq!(bounds.min_z, -3.25);
    assert_eq!(bounds.max_x, 100.0);
    assert_eq!(bounds.max_y, 88.75);
    assert_eq!(bounds.max_z, 15.0);
    assert_eq!(bounds.point_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot_with(LasHeader { number_of_point_records: 1, ..LasHeader::default() }, vec![LasPoint::default()]);
    assert_eq!(compute_las_bounds(&snapshot), compute_las_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_las_bounds(&LasSnapshot::default()), LasBounds::default());
}
