//! 🔺️ Sparse diff builder for `RotateSelection` — every unlocked addressed object and target volume turns
//! about its own origin: its BASE orientation (identity when absent) is pre-multiplied by the payload's
//! axis-angle quaternion, so the leaf replays on any base. The attraction graph is re-solved: attracted
//! objects are re-placed from their turned parents, other touched attractions re-derive.
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_diff, quat_from_axis_angle, quat_mul};
use crate::{Puzzle3dObject, Puzzle3dSnapshot, Puzzle3dTargetVolume};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RotateSelection, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !(payload.axis.iter().all(|value| value.is_finite()) && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation axis and angle must be finite", payload.targets.clone());
    }
    let turn = quat_from_axis_angle(payload.axis[0], payload.axis[1], payload.axis[2], payload.angle);
    let turned = |orientation: Option<[f64; 4]>| Some(quat_mul(turn, orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
    let identity = payload.angle == 0.0 || turn == [0.0, 0.0, 0.0, 1.0];
    puzzle3d_selection_diff(base, &payload.targets, identity, |entry: &Puzzle3dObject| Puzzle3dObject { orientation: turned(entry.orientation), ..entry.clone() }, |entry: &Puzzle3dTargetVolume| Puzzle3dTargetVolume { orientation: turned(entry.orientation), ..entry.clone() }, true)
}
//#endregion 🔖️Diff
