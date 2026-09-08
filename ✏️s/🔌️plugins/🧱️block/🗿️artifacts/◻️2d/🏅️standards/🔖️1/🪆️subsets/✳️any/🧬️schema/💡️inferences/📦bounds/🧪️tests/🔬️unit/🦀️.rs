
use super::*;
use crate::Block2dHandleTemplate;
use std::f64::consts::PI;

fn handle(id: &str, angle: f64, radius: f64) -> Block2dHandleTemplate {
    Block2dHandleTemplate { id: id.into(), handle_kind: "wire".into(), angle, radius }
}

#[semio_framework_async_macros::async_test]
async fn empty_catalog_yields_default_bounds() {
    let snapshot = Block2dSnapshot::default();
    assert_eq!(compute_block2d_bounds(&snapshot), Block2dBounds::default());
}

#[semio_framework_async_macros::async_test]
async fn single_handle_bounds_equal_its_own_cartesian_point() {
    let mut snapshot = Block2dSnapshot::default();
    snapshot.handles.push(handle("h0", 0.0, 2.0));
    let bounds = compute_block2d_bounds(&snapshot);
    let point = bounds.bounding_box.expect("one handle produces a bounding box");
    assert!((point.min[0] - 2.0).abs() < 1e-9);
    assert!((point.max[0] - 2.0).abs() < 1e-9);
    assert!(point.min[1].abs() < 1e-9);
    assert!(point.max[1].abs() < 1e-9);
    assert_eq!(bounds.vertex_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn opposite_handles_span_the_full_diameter() {
    let mut snapshot = Block2dSnapshot::default();
    snapshot.handles.push(handle("h0", 0.0, 1.0));
    snapshot.handles.push(handle("h1", PI, 1.0));
    let bounds = compute_block2d_bounds(&snapshot);
    let box_ = bounds.bounding_box.expect("two handles produce a bounding box");
    assert!((box_.min[0] + 1.0).abs() < 1e-9);
    assert!((box_.max[0] - 1.0).abs() < 1e-9);
    assert_eq!(bounds.vertex_count, 2);
}
