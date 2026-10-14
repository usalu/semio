//! 🔺️ Sparse diff builder for `ScaleSelection` — every unlocked addressed object and target volume keeps
//! its origin and multiplies its BASE scale (uniform broadcast, absent reads as one) by the payload's
//! per-axis factors, so the leaf replays on any base.
use semio_s_artifact_puzzle_3d_diff::{Puzzle3dDiff, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumeModification};
use {semio_s_artifact_puzzle_3d_model::puzzle3d_scaled, crate::puzzle3d_selection, crate::puzzle3d_selection_follow, crate::puzzle3d_selection_outcome, semio_s_artifact_puzzle_3d_model::Puzzle3dPose, crate::Puzzle3dSelectionFollow};
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

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
        .map(|volume| Puzzle3dTargetVolumeModification { id: volume.id.clone(), patch: Puzzle3dTargetVolumePatch { scale: Some(Some(puzzle3d_scaled(volume.scale, factors))).filter(|scale| *scale != volume.scale), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle3d_selection_outcome(selection, &payload.targets, solved, base, volumes)
}
//#endregion 🔖️Diff
