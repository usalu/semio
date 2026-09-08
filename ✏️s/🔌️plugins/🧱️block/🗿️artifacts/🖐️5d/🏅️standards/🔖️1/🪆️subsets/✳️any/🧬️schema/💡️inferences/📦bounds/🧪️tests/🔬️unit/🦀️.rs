
use super::*;
use crate::Block5dGripTemplate;

fn grip(id: &str, position: [f64; 3], radius_3d: f64) -> Block5dGripTemplate {
    Block5dGripTemplate { id: id.into(), grip_kind: "rope".into(), angle: 0.0, radius_2d: 0.0, position, direction: [0.0, 1.0, 0.0], radius_3d }
}

#[semio_framework_async_macros::async_test]
async fn empty_catalog_yields_default_bounds() {
    let snapshot = Block5dSnapshot::default();
    assert_eq!(compute_block5d_bounds(&snapshot), Block5dBounds::default());
}

#[semio_framework_async_macros::async_test]
async fn single_grip_bounds_equal_its_own_inflated_footprint() {
    let mut snapshot = Block5dSnapshot::default();
    snapshot.grips.push(grip("g0", [1.0, 1.0, 1.0], 0.5));
    let bounds = compute_block5d_bounds(&snapshot);
    assert_eq!(bounds.bounding_box, Some(BoundingBox3d { min: [0.5, 0.5, 0.5], max: [1.5, 1.5, 1.5] }));
    assert_eq!(bounds.vertex_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn multiple_grips_union_their_footprints() {
    let mut snapshot = Block5dSnapshot::default();
    snapshot.grips.push(grip("g0", [1.0, 2.0, 3.0], 0.5));
    snapshot.grips.push(grip("g1", [-1.0, 0.0, 4.0], 0.25));
    let bounds = compute_block5d_bounds(&snapshot);
    assert_eq!(bounds.bounding_box, Some(BoundingBox3d { min: [-1.25, -0.25, 2.5], max: [1.5, 2.5, 4.25] }));
    assert_eq!(bounds.vertex_count, 2);
}
