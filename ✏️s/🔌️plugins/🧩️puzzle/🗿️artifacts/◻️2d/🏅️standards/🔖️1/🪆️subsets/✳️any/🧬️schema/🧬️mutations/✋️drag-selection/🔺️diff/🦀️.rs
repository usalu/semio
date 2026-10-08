//! 🔺️ Sparse diff builder for `DragSelection` — every unlocked addressed node and target region moves
//! by the payload offset, read off the BASE position, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dTargetRegionPatch, Puzzle2dTargetRegionPatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection, puzzle2d_selection_outcome};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DragSelection, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.iter().map(|id| id.to_string_owner()).collect::<Vec<_>>());
    }
    let selection = match puzzle2d_selection(base, &payload.targets, true) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let (dx, dy) = (payload.dx, payload.dy);
    let nodes = selection
        .nodes
        .iter()
        .map(|node| Puzzle2dNodePatchEntry { id: node.id.clone(), patch: Puzzle2dNodePatch { x: Some(node.x + dx).filter(|x| *x != node.x), y: Some(node.y + dy).filter(|y| *y != node.y), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    let regions = selection
        .regions
        .iter()
        .map(|region| Puzzle2dTargetRegionPatchEntry { id: region.id.clone(), patch: Puzzle2dTargetRegionPatch { x: Some(region.x + dx).filter(|x| *x != region.x), y: Some(region.y + dy).filter(|y| *y != region.y), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle2d_selection_outcome(selection, &payload.targets, nodes, regions)
}
//#endregion 🔖️Diff
