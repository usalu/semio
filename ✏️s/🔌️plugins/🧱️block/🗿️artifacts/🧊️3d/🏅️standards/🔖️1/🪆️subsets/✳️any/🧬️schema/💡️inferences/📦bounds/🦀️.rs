//! 📦 `bounds` — one named inference: geometric bounding box + vertex count over an object kind's
//! rim vortex templates (`Block3dSnapshot::vortices`). Block3d has no parent/child object graph
//! (unlike puzzle3d's `flatPosition`) — it is a single flat catalog of rim placements — so this is
//! a plain whole-snapshot derivation, not a per-entity `InferredField` chain: every vortex
//! contributes independently to one aggregate box, there is nothing to invalidate incrementally.

use crate::Block3dSnapshot;

//#region 🔖️Bounds
/// 📦️ Axis-aligned bounding box in the object kind's local space.
#[derive(Clone, Copy, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BoundingBox3d {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 📦️ Aggregate geometric stats over `Block3dSnapshot::vortices` — `None` bounding box for an
/// empty catalog (matches `Block3dSnapshot::default()`, satisfying the inference default law).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dBounds {
    pub bounding_box: Option<BoundingBox3d>,
    pub vertex_count: u32,
}

/// 📦️ Computes `bounds` from a block3d snapshot — each rim vortex template contributes its
/// `position` inflated by its own `radius` (the rim placement's physical footprint) to the
/// running min/max; an empty vortex catalog yields `Block3dBounds::default()`.
pub fn compute_block3d_bounds(snapshot: &Block3dSnapshot) -> Block3dBounds {
    let Some(first) = snapshot.vortices.first() else {
        return Block3dBounds::default();
    };
    let mut min = [first.position[0] - first.radius, first.position[1] - first.radius, first.position[2] - first.radius];
    let mut max = [first.position[0] + first.radius, first.position[1] + first.radius, first.position[2] + first.radius];
    for vortex in &snapshot.vortices[1..] {
        for axis in 0..3 {
            min[axis] = min[axis].min(vortex.position[axis] - vortex.radius);
            max[axis] = max[axis].max(vortex.position[axis] + vortex.radius);
        }
    }
    Block3dBounds { bounding_box: Some(BoundingBox3d { min, max }), vertex_count: snapshot.vortices.len() as u32 }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
