//! 🔺️ Sparse diff builder for `RotateSelection` — every unlocked addressed object and target volume turns
//! about its own origin: its BASE orientation (identity when absent) is pre-multiplied by the payload's
//! axis-angle quaternion, so the leaf replays on any base. The attraction graph is re-solved: attracted
//! objects are re-placed from their turned parents, other touched attractions re-derive.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle3dDiff, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumePatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection, puzzle3d_selection_follow, puzzle3d_selection_outcome, quat_from_axis_angle, quat_mul, Puzzle3dPose, Puzzle3dSelectionFollow, PUZZLE3D_IDENTITY_QUATERNION};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RotateSelection, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !(payload.axis.iter().all(|value| value.is_finite()) && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation axis and angle must be finite", payload.targets.clone());
    }
    let selection = match puzzle3d_selection(base, &payload.targets) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let turn = quat_from_axis_angle(payload.axis[0], payload.axis[1], payload.axis[2], payload.angle);
    let turned = |orientation: Option<[f64; 4]>| Some(quat_mul(turn, orientation.unwrap_or(PUZZLE3D_IDENTITY_QUATERNION)));
    let identity = payload.angle == 0.0 || turn == PUZZLE3D_IDENTITY_QUATERNION;
    let solved = if identity { Puzzle3dSelectionFollow::default() } else { puzzle3d_selection_follow(base, &payload.targets, &selection, &|object| Puzzle3dPose { origin: object.origin, orientation: turned(object.orientation), scale: object.scale }, true) };
    let volumes = selection
        .volumes
        .iter()
        .filter(|_| !identity)
        .map(|volume| Puzzle3dTargetVolumePatchEntry { id: volume.id.clone(), patch: Puzzle3dTargetVolumePatch { orientation: Some(turned(volume.orientation)).filter(|orientation| *orientation != volume.orientation), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle3d_selection_outcome(selection, &payload.targets, solved, base, volumes)
}
//#endregion 🔖️Diff
