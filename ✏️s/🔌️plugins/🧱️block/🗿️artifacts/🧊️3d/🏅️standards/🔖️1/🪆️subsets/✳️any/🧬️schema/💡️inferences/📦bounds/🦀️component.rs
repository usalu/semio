//! 📦 `bounds` — one named inference: geometric bounding box + vertex count over an object kind's
//! rim vortex templates (`Block3dSnapshot::vortices`). Block3d has no parent/child object graph
//! (unlike puzzle3d's `flatPosition`) — it is a single flat catalog of rim placements — so this is
//! a plain whole-snapshot derivation, not a per-entity `InferredField` chain: every vortex
//! contributes independently to one aggregate box, there is nothing to invalidate incrementally.

use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Bounds
/// 📦️ Axis-aligned bounding box in the object kind's local space.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox3d {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 📦️ Aggregate geometric stats over `Block3dSnapshot::vortices` — `None` bounding box for an
/// empty catalog (matches `Block3dSnapshot::default()`, satisfying the inference default law).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use crate::artifacts::block3d::Block3dVortexTemplate;

    fn vortex(id: &str, position: [f64; 3], radius: f64) -> Block3dVortexTemplate {
        Block3dVortexTemplate { id: id.into(), vortex_kind: "door".into(), position, direction: [0.0, 1.0, 0.0], radius, label: None }
    }

    #[test]
    fn empty_catalog_yields_default_bounds() {
        let snapshot = Block3dSnapshot::default();
        assert_eq!(compute_block3d_bounds(&snapshot), Block3dBounds::default());
    }

    #[test]
    fn single_vortex_bounds_equal_its_own_inflated_footprint() {
        let mut snapshot = Block3dSnapshot::default();
        snapshot.vortices.push(vortex("v0", [1.0, 1.0, 1.0], 0.5));
        let bounds = compute_block3d_bounds(&snapshot);
        assert_eq!(bounds.bounding_box, Some(BoundingBox3d { min: [0.5, 0.5, 0.5], max: [1.5, 1.5, 1.5] }));
        assert_eq!(bounds.vertex_count, 1);
    }

    #[test]
    fn multiple_vortices_union_their_footprints() {
        let mut snapshot = Block3dSnapshot::default();
        snapshot.vortices.push(vortex("v0", [1.0, 2.0, 3.0], 0.5));
        snapshot.vortices.push(vortex("v1", [-1.0, 0.0, 4.0], 0.25));
        let bounds = compute_block3d_bounds(&snapshot);
        assert_eq!(bounds.bounding_box, Some(BoundingBox3d { min: [-1.25, -0.5, 2.5], max: [1.5, 2.5, 4.25] }));
        assert_eq!(bounds.vertex_count, 2);
    }
}
//#endregion 🧪️Tests
