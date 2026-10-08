//! 🔺️ Sparse diff builder for `DragSelection` — every unlocked addressed object and target volume moves
//! by the payload offset, read off the BASE origin, so the leaf replays on any base. The attraction graph is
//! re-solved: attracted objects are re-placed from their moved parents, other touched attractions re-derive.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle3dDiff, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumePatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection, puzzle3d_selection_follow, puzzle3d_selection_outcome, Puzzle3dPose, Puzzle3dSelectionFollow};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DragSelection, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !payload.offset.iter().all(|value| value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    let selection = match puzzle3d_selection(base, &payload.targets) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let identity = payload.offset == [0.0; 3];
    let [dx, dy, dz] = payload.offset;
    let moved = |origin: [f64; 3]| [origin[0] + dx, origin[1] + dy, origin[2] + dz];
    let solved = if identity { Puzzle3dSelectionFollow::default() } else { puzzle3d_selection_follow(base, &payload.targets, &selection, &|object| Puzzle3dPose { origin: moved(object.origin), orientation: object.orientation, scale: object.scale }, true) };
    let volumes = selection
        .volumes
        .iter()
        .filter(|_| !identity)
        .map(|volume| Puzzle3dTargetVolumePatchEntry { id: volume.id.clone(), patch: Puzzle3dTargetVolumePatch { origin: Some(moved(volume.origin)).filter(|origin| *origin != volume.origin), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle3d_selection_outcome(selection, &payload.targets, solved, base, volumes)
}
//#endregion 🔖️Diff
