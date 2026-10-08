//! 🔺️ Sparse diff builder for `ScaleSelection` — every unlocked addressed node spreads from the pivot
//! (its own `scale` untouched) and every unlocked addressed target region scales corner and extent,
//! all read off the BASE, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodeModification, Puzzle2dTargetRegionPatch, Puzzle2dTargetRegionModification};
use protocol::list_delta::RowPatch;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection, puzzle2d_selection_outcome};
use crate::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ScaleSelection, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.factor.is_finite() && payload.factor > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a scale pivot must be finite and its factor finite and positive", payload.targets.iter().map(|id| id.to_string_owner()).collect::<Vec<_>>());
    }
    let selection = match puzzle2d_selection(base, &payload.targets, true) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let (cx, cy, factor) = (payload.pivot_x, payload.pivot_y, payload.factor);
    let nodes = selection
        .nodes
        .iter()
        .map(|node| Puzzle2dNodeModification { id: node.id.clone(), patch: Puzzle2dNodePatch { x: Some(cx + (node.x - cx) * factor).filter(|x| *x != node.x), y: Some(cy + (node.y - cy) * factor).filter(|y| *y != node.y), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    let regions = selection
        .regions
        .iter()
        .map(|region| {
            let patch = Puzzle2dTargetRegionPatch {
                x: Some(cx + (region.x - cx) * factor).filter(|x| *x != region.x),
                y: Some(cy + (region.y - cy) * factor).filter(|y| *y != region.y),
                width: Some(region.width * factor).filter(|width| *width != region.width),
                height: Some(region.height * factor).filter(|height| *height != region.height),
                ..Default::default()
            };
            Puzzle2dTargetRegionModification { id: region.id.clone(), patch }
        })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle2d_selection_outcome(selection, &payload.targets, nodes, regions)
}
//#endregion 🔖️Diff
