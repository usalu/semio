//! 🔺️ Sparse diff builder for `DragSelection` — every unlocked addressed node and target region moves
//! by the payload offset, read off the BASE position, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_selection_diff;
use crate::{Puzzle2dNode, Puzzle2dSnapshot, Puzzle2dTargetRegion};

//#region 🔖️Diff
pub fn diff(payload: &super::DragSelection, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let (dx, dy) = (payload.dx, payload.dy);
    let region: &dyn Fn(&Puzzle2dTargetRegion) -> Puzzle2dTargetRegion = &|entry| Puzzle2dTargetRegion { x: entry.x + dx, y: entry.y + dy, ..entry.clone() };
    puzzle2d_selection_diff(base, &payload.targets, dx == 0.0 && dy == 0.0, |entry: &Puzzle2dNode| Puzzle2dNode { x: entry.x + dx, y: entry.y + dy, ..entry.clone() }, Some(region))
}
//#endregion 🔖️Diff
