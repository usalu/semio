
use super::*;
use crate::Block3dVortexTemplate;

fn vortex(id: &str, position: [f64; 3], radius: f64) -> Block3dVortexTemplate {
    Block3dVortexTemplate { id: id.into(), vortex_kind: "door".into(), position, direction: [0.0, 1.0, 0.0], radius, label: None }
}

#[semio_framework_async_macros::async_test]
async fn empty_catalog_yields_default_bounds() {
    let snapshot = Block3dSnapshot::default();
    assert_eq!(compute_block3d_bounds(&snapshot), Block3dBounds::default());
}

#[semio_framework_async_macros::async_test]
async fn single_vortex_bounds_equal_its_own_inflated_footprint() {
    let mut snapshot = Block3dSnapshot::default();
    snapshot.vortices.push(vortex("v0", [1.0, 1.0, 1.0], 0.5));
    let bounds = compute_block3d_bounds(&snapshot);
    assert_eq!(bounds.bounding_box, Some(BoundingBox3d { min: [0.5, 0.5, 0.5], max: [1.5, 1.5, 1.5] }));
    assert_eq!(bounds.vertex_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn multiple_vortices_union_their_footprints() {
    let mut snapshot = Block3dSnapshot::default();
    snapshot.vortices.push(vortex("v0", [1.0, 2.0, 3.0], 0.5));
    snapshot.vortices.push(vortex("v1", [-1.0, 0.0, 4.0], 0.25));
    let bounds = compute_block3d_bounds(&snapshot);
    assert_eq!(bounds.bounding_box, Some(BoundingBox3d { min: [-1.25, -0.25, 2.5], max: [1.5, 2.5, 4.25] }));
    assert_eq!(bounds.vertex_count, 2);
}
