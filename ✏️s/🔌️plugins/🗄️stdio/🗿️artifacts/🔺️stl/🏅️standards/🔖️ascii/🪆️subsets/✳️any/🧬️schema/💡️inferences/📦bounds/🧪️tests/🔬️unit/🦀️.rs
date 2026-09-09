use super::*;
use crate::STDIO_STL_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn triangle(normal: [f64; 3], vertices: [[f64; 3]; 3]) -> StlTriangle {
    StlTriangle { normal, vertices }
}

#[semio_framework_async_macros::async_test]
async fn bounds_matches_hand_built_triangle_extent() {
    let snapshot = StlSnapshot {
        schema: STDIO_STL_DOCUMENT_SCHEMA.into(),
        solid_name: "cube_corner".into(),
        triangles: vec![triangle([0.0, 0.0, 1.0], [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]]), triangle([1.0, 0.0, 0.0], [[0.0, 0.0, 5.0], [0.0, 2.0, -3.0], [0.0, -4.0, 1.0]])],
    };
    let bounds = compute_stl_bounds(&snapshot);
    assert_eq!(bounds.min, [-1.0, -4.0, -3.0]);
    assert_eq!(bounds.max, [1.0, 2.0, 5.0]);
    assert_eq!(bounds.triangle_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: "solid".into(), triangles: vec![triangle([0.0, 0.0, 1.0], [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])] };
    assert_eq!(compute_stl_bounds(&snapshot), compute_stl_bounds(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_stl_bounds(&StlSnapshot::default()), StlBounds::default());
}
