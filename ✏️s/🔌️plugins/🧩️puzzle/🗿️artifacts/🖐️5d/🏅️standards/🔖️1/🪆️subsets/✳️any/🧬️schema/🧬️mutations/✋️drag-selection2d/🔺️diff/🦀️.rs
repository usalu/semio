//! 🔺️ Sparse diff builder for `DragSelection2d` — every unlocked addressed part's board projection moves by the
//! payload offset, read off the BASE position, so the leaf replays on any base. Target volumes live in the world.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::puzzle5d_selection_diff;
use crate::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::DragSelection2d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let (dx, dy) = (payload.dx, payload.dy);
    puzzle5d_selection_diff(base, &payload.targets, dx == 0.0 && dy == 0.0, |entry: &Puzzle5dPart| Puzzle5dPart { part_2d: Puzzle5dPart2d { x: entry.part_2d.x + dx, y: entry.part_2d.y + dy, ..entry.part_2d.clone() }, ..entry.clone() }, None)
}
//#endregion 🔖️Diff
