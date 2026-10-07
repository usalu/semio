//! 🔺️ Sparse diff builder for `RotateSelection` — every unlocked addressed node orbits the pivot from
//! its BASE position and turns every handle angle by the same amount. A target region is axis-aligned
//! by construction, so a rotation skips it with `mutation.partial` instead of moving it.
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_selection_diff;
use crate::{Puzzle2dHandle, Puzzle2dNode, Puzzle2dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::RotateSelection, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation pivot and angle must be finite", payload.targets.iter().map(|id| id.to_string_owner()).collect::<Vec<_>>());
    }
    let (cx, cy, radians) = (payload.pivot_x, payload.pivot_y, payload.angle);
    let (sin, cos) = radians.sin_cos();
    let node = |entry: &Puzzle2dNode| Puzzle2dNode {
        x: cx + (entry.x - cx) * cos - (entry.y - cy) * sin,
        y: cy + (entry.x - cx) * sin + (entry.y - cy) * cos,
        handles: entry.handles.iter().map(|handle| Puzzle2dHandle { angle: handle.angle + radians, ..handle.clone() }).collect(),
        ..entry.clone()
    };
    puzzle2d_selection_diff(base, &payload.targets, radians == 0.0, node, None)
}
//#endregion 🔖️Diff
