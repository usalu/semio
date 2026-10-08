//! 🔺️ Sparse diff builder for `DragSelection3d` — every unlocked addressed part's world origin and target volume's
//! origin moves by the payload offset, read off the BASE origin, so the leaf replays on any base. A part's board pin
//! tracks the same ground-plane motion through [`PUZZLE5D_FLAT_TO_WORLD`] (board y points the other way), so the two
//! poses of a part never drift apart.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle5dDiff, Puzzle5dPart2dPatch, Puzzle5dPart3dPatch, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dTargetVolumePatch, Puzzle5dTargetVolumePatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection, puzzle5d_selection_outcome, PUZZLE5D_FLAT_TO_WORLD};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DragSelection3d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !payload.offset.iter().all(|value| value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let selection = match puzzle5d_selection(base, &payload.targets, true) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let [dx, dy, dz] = payload.offset;
    let moved = |origin: [f64; 3]| [origin[0] + dx, origin[1] + dy, origin[2] + dz];
    let parts = selection
        .parts
        .iter()
        .map(|part| {
            let board = Puzzle5dPart2dPatch { x: Some(part.part_2d.x + dx / PUZZLE5D_FLAT_TO_WORLD).filter(|x| *x != part.part_2d.x), y: Some(part.part_2d.y - dy / PUZZLE5D_FLAT_TO_WORLD).filter(|y| *y != part.part_2d.y), ..Default::default() };
            let world = Puzzle5dPart3dPatch { origin: Some(moved(part.part_3d.origin)).filter(|origin| *origin != part.part_3d.origin), ..Default::default() };
            Puzzle5dPartPatchEntry { id: part.id.clone(), patch: Puzzle5dPartPatch { part_2d: Some(board).filter(|board| !board.is_empty()), part_3d: Some(world).filter(|world| !world.is_empty()), ..Default::default() } }
        })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    let volumes = selection
        .volumes
        .iter()
        .map(|volume| Puzzle5dTargetVolumePatchEntry { id: volume.id.clone(), patch: Puzzle5dTargetVolumePatch { origin: Some(moved(volume.origin)).filter(|origin| *origin != volume.origin), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle5d_selection_outcome(selection, &payload.targets, parts, volumes)
}
//#endregion 🔖️Diff
