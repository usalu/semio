//! 🔺️ Sparse diff builder for `ScaleSelection` — every unlocked addressed object and target volume keeps
//! its origin and multiplies its BASE scale (uniform broadcast, absent reads as one) by the payload's
//! per-axis factors, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle3dDiff, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumePatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_scaled, puzzle3d_selection, puzzle3d_selection_follow, puzzle3d_selection_outcome, Puzzle3dPose, Puzzle3dSelectionFollow};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleSelection, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !payload.factors.iter().all(|value| value.is_finite() && *value > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "every scale factor must be a finite number greater than 0", payload.targets.clone());
    }
    let selection = match puzzle3d_selection(base, &payload.targets) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let factors = payload.factors;
    let identity = factors == [1.0; 3];
    let solved = if identity { Puzzle3dSelectionFollow::default() } else { puzzle3d_selection_follow(base, &payload.targets, &selection, &|object| Puzzle3dPose { origin: object.origin, orientation: object.orientation, scale: Some(puzzle3d_scaled(object.scale, factors)) }, false) };
    let volumes = selection
        .volumes
        .iter()
        .filter(|_| !identity)
        .map(|volume| Puzzle3dTargetVolumePatchEntry { id: volume.id.clone(), patch: Puzzle3dTargetVolumePatch { scale: Some(Some(puzzle3d_scaled(volume.scale, factors))).filter(|scale| *scale != volume.scale), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle3d_selection_outcome(selection, &payload.targets, solved, base, volumes)
}
//#endregion 🔖️Diff
