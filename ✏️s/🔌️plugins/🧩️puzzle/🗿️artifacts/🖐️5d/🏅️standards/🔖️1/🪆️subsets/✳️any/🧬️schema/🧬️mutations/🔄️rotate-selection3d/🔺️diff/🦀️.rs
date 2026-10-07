//! 🔺️ Sparse diff builder for `RotateSelection3d` — every unlocked addressed part and target volume turns about its
//! own world origin: its BASE orientation (identity when absent) is pre-multiplied by the payload's axis-angle
//! quaternion, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_diff,quat_from_axis_angle,quat_mul};

use crate::{Puzzle5dPart, Puzzle5dPart3d, Puzzle5dSnapshot, Puzzle5dTargetVolume};

//#region 🔖️Diff
pub fn diff(payload: &super::RotateSelection3d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !(payload.axis.iter().all(|value| value.is_finite()) && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation axis and angle must be finite", payload.targets.clone());
    }
    let turn = quat_from_axis_angle(payload.axis[0], payload.axis[1], payload.axis[2], payload.angle);
    let turned = |orientation: Option<[f64; 4]>| Some(quat_mul(turn, orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
    let volume = |entry: &Puzzle5dTargetVolume| Puzzle5dTargetVolume { orientation: turned(entry.orientation), ..entry.clone() };
    let identity = payload.angle == 0.0 || turn == [0.0, 0.0, 0.0, 1.0];
    puzzle5d_selection_diff(base, &payload.targets, identity, |entry: &Puzzle5dPart| Puzzle5dPart { part_3d: Puzzle5dPart3d { orientation: turned(entry.part_3d.orientation), ..entry.part_3d.clone() }, ..entry.clone() }, Some(&volume))
}
//#endregion 🔖️Diff
