//! 🔺️ Sparse diff builder for `ScaleSelection` — every unlocked addressed node spreads from the pivot
//! (its own `scale` untouched) and every unlocked addressed target region scales corner and extent,
//! all read off the BASE, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_selection_diff;
use crate::{Puzzle2dNode, Puzzle2dSnapshot, Puzzle2dTargetRegion};

//#region 🔖️Diff
pub fn diff(payload: &super::ScaleSelection, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.factor.is_finite() && payload.factor > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a scale pivot must be finite and its factor finite and positive", payload.targets.iter().map(|id| id.to_string_owner()).collect::<Vec<_>>());
    }
    let (cx, cy, factor) = (payload.pivot_x, payload.pivot_y, payload.factor);
    let region: &dyn Fn(&Puzzle2dTargetRegion) -> Puzzle2dTargetRegion = &|entry| Puzzle2dTargetRegion { x: cx + (entry.x - cx) * factor, y: cy + (entry.y - cy) * factor, width: entry.width * factor, height: entry.height * factor, ..entry.clone() };
    puzzle2d_selection_diff(base, &payload.targets, factor == 1.0, |entry: &Puzzle2dNode| Puzzle2dNode { x: cx + (entry.x - cx) * factor, y: cy + (entry.y - cy) * factor, ..entry.clone() }, Some(region))
}
//#endregion 🔖️Diff
