//! 📦 `bounds` — the stl snapshot's triangle-soup bounding box and triangle count. Real STL has
//! no shared vertex index space (each `StlTriangle` owns its own 3 vertices), so this is a direct
//! min/max fold over every triangle's own `vertices` — no per-triangle incremental decomposition,
//! a pure whole-snapshot scalar. No `InferredField` needed.

use crate::artifacts::stl::schema::snapshot::StlTriangle;
use crate::artifacts::stl::StlSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Stl triangle-soup bounding box and triangle count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StlBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub triangle_count: u32,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, p: [f64; 3]) {
    if !*seen {
        *min = p;
        *max = p;
        *seen = true;
        return;
    }
    for i in 0..3 {
        min[i] = min[i].min(p[i]);
        max[i] = max[i].max(p[i]);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_triangle(min: &mut [f64; 3], max: &mut [f64; 3], seen: &mut bool, triangle: &StlTriangle) {
    for vertex in &triangle.vertices {
        expand(min, max, seen, *vertex);
    }
}

/// 📦️ Computes [`StlBounds`] over every triangle's own 3 vertices.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_stl_bounds(snapshot: &StlSnapshot) -> StlBounds {
    let mut min = [0.0f64; 3];
    let mut max = [0.0f64; 3];
    let mut seen = false;

    for triangle in &snapshot.triangles {
        expand_triangle(&mut min, &mut max, &mut seen, triangle);
    }

    StlBounds { min, max, triangle_count: snapshot.triangles.len() as u32 }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::stl::STDIO_STL_DOCUMENT_SCHEMA;

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
}
//#endregion 🧪️Tests
