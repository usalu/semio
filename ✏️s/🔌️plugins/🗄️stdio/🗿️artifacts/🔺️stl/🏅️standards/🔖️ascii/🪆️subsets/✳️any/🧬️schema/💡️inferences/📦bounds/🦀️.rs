//! 📦 `bounds` — the stl snapshot's triangle-soup bounding box and triangle count. Real STL has
//! no shared vertex index space (each `StlTriangle` owns its own 3 vertices), so this is a direct
//! min/max fold over every triangle's own `vertices` — no per-triangle incremental decomposition,
//! a pure whole-snapshot scalar. No `InferredField` needed.

use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;

//#region 🔖️Bounds
/// 📦️ Stl triangle-soup bounding box and triangle count.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
