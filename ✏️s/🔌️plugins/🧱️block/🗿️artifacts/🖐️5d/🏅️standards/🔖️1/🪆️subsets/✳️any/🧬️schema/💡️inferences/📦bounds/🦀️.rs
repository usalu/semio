//! 📦 `bounds` — one named inference: geometric bounding box + vertex count over a part kind's rim
//! grip templates' 3d placement (`Block5dGripTemplate::position`/`radius_3d` — the part's
//! 3d-projection rim geometry; the parallel 2d `angle`/`radius_2d` half is left for a future
//! `bounds2d` inference should the board projection need its own footprint). Block5d has no
//! parent/child object graph — it is a single flat catalog of rim placements — so this is a plain
//! whole-snapshot derivation, not a per-entity `InferredField` chain.

use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Bounds
/// 📦️ Axis-aligned bounding box in the part kind's local 3d space.
#[derive(Clone, Copy, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct BoundingBox3d {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 📦️ Aggregate geometric stats over `Block5dSnapshot::grips`' 3d placements — `None` bounding box
/// for an empty catalog (matches `Block5dSnapshot::default()`, satisfying the inference default law).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block5dBounds {
    pub bounding_box: Option<BoundingBox3d>,
    pub vertex_count: u32,
}

/// 📦️ Computes `bounds` from a block5d snapshot — each rim grip template contributes its 3d
/// `position` inflated by its own `radius_3d` (the rim placement's physical footprint in the 3d
/// projection) to the running min/max; an empty grip catalog yields `Block5dBounds::default()`.
pub fn compute_block5d_bounds(snapshot: &Block5dSnapshot) -> Block5dBounds {
    let Some(first) = snapshot.grips.first() else {
        return Block5dBounds::default();
    };
    let mut min = [first.position[0] - first.radius_3d, first.position[1] - first.radius_3d, first.position[2] - first.radius_3d];
    let mut max = [first.position[0] + first.radius_3d, first.position[1] + first.radius_3d, first.position[2] + first.radius_3d];
    for grip in &snapshot.grips[1..] {
        for axis in 0..3 {
            min[axis] = min[axis].min(grip.position[axis] - grip.radius_3d);
            max[axis] = max[axis].max(grip.position[axis] + grip.radius_3d);
        }
    }
    Block5dBounds { bounding_box: Some(BoundingBox3d { min, max }), vertex_count: snapshot.grips.len() as u32 }
}
//#endregion 🔖️Bounds

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::block5d::Block5dGripTemplate;

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
}
//#endregion 🧪️Tests
