//! 🔺️ Sparse diff builder for `DragSelection2d` — every unlocked addressed part's board projection moves by the
//! payload offset, read off the BASE position, so the leaf replays on any base. Target volumes live in the world.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dPart2dPatch, Puzzle5dPartPatch, Puzzle5dPartModification};
use protocol::list_delta::RowPatch;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection, puzzle5d_selection_outcome};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DragSelection2d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let selection = match puzzle5d_selection(base, &payload.targets, false) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let (dx, dy) = (payload.dx, payload.dy);
    let parts = selection
        .parts
        .iter()
        .map(|part| {
            let board = Puzzle5dPart2dPatch { x: Some(part.part_2d.x + dx).filter(|x| *x != part.part_2d.x), y: Some(part.part_2d.y + dy).filter(|y| *y != part.part_2d.y), ..Default::default() };
            Puzzle5dPartModification { id: part.id.clone(), patch: Puzzle5dPartPatch { part_2d: Some(board).filter(|board| !board.is_empty()), ..Default::default() } }
        })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle5d_selection_outcome(selection, &payload.targets, parts, Vec::new())
}
//#endregion 🔖️Diff
