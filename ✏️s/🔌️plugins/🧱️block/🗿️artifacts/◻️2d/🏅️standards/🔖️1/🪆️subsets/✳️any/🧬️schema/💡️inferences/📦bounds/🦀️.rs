//! 📦 `bounds` — one named inference: geometric bounding box + vertex count over a node kind's rim
//! handle templates (`Block2dSnapshot::handles`), converting each handle's polar `angle`/`radius`
//! placement into cartesian rim coordinates. Block2d has no parent/child object graph (unlike
//! puzzle2d's flatten pipeline) — it is a single flat catalog of rim placements — so this is a
//! plain whole-snapshot derivation, not a per-entity `InferredField` chain.

use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Bounds
/// 📦️ Axis-aligned bounding box in the node kind's local (rim) space.
#[derive(Clone, Copy, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BoundingBox2d {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

/// 📦️ Aggregate geometric stats over `Block2dSnapshot::handles` — `None` bounding box for an
/// empty catalog (matches `Block2dSnapshot::default()`, satisfying the inference default law).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block2dBounds {
    pub bounding_box: Option<BoundingBox2d>,
    pub vertex_count: u32,
}

/// 📦️ Computes `bounds` from a block2d snapshot — each rim handle template's polar `angle`
/// (radians) / `radius` placement is converted to cartesian `(radius·cos(angle), radius·sin(angle))`
/// and folded into the running min/max; an empty handle catalog yields `Block2dBounds::default()`.
pub async fn compute_block2d_bounds(snapshot: &Block2dSnapshot) -> Block2dBounds {
    if snapshot.handles.is_empty() {
        return Block2dBounds::default();
    }
    let points: Vec<[f64; 2]> = snapshot.handles.iter().map(|handle| [handle.radius * handle.angle.cos(), handle.radius * handle.angle.sin()]).collect();
    let mut min = points[0];
    let mut max = points[0];
    for point in &points[1..] {
        for axis in 0..2 {
            min[axis] = min[axis].min(point[axis]);
            max[axis] = max[axis].max(point[axis]);
        }
    }
    Block2dBounds { bounding_box: Some(BoundingBox2d { min, max }), vertex_count: snapshot.handles.len() as u32 }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::block2d::Block2dHandleTemplate;
    use std::f64::consts::PI;

    async fn handle(id: &str, angle: f64, radius: f64) -> Block2dHandleTemplate {
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
}
//#endregion 🧪️Tests
