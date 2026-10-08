//! 🔺️ Sparse diff builder for `RotateSelection3d` — every unlocked addressed part and target volume turns about its
//! own world origin: its BASE orientation (identity when absent) is pre-multiplied by the payload's axis-angle
//! quaternion, so the leaf replays on any base.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle5dDiff, Puzzle5dPart3dPatch, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dTargetVolumePatch, Puzzle5dTargetVolumePatchEntry};
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection, puzzle5d_selection_outcome, quat_from_axis_angle, quat_mul};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RotateSelection3d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if !(payload.axis.iter().all(|value| value.is_finite()) && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation axis and angle must be finite", payload.targets.clone());
    }
    let selection = match puzzle5d_selection(base, &payload.targets, true) {
        Ok(selection) => selection,
        Err(outcome) => return outcome,
    };
    let turn = quat_from_axis_angle(payload.axis[0], payload.axis[1], payload.axis[2], payload.angle);
    let turned = |orientation: Option<[f64; 4]>| Some(quat_mul(turn, orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
    let identity = payload.angle == 0.0 || turn == [0.0, 0.0, 0.0, 1.0];
    let parts = selection
        .parts
        .iter()
        .filter(|_| !identity)
        .map(|part| {
            let world = Puzzle5dPart3dPatch { orientation: Some(turned(part.part_3d.orientation)).filter(|orientation| *orientation != part.part_3d.orientation), ..Default::default() };
            Puzzle5dPartPatchEntry { id: part.id.clone(), patch: Puzzle5dPartPatch { part_3d: Some(world).filter(|world| !world.is_empty()), ..Default::default() } }
        })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    let volumes = selection
        .volumes
        .iter()
        .filter(|_| !identity)
        .map(|volume| Puzzle5dTargetVolumePatchEntry { id: volume.id.clone(), patch: Puzzle5dTargetVolumePatch { orientation: Some(turned(volume.orientation)).filter(|orientation| *orientation != volume.orientation), ..Default::default() } })
        .filter(|entry| !entry.patch.is_empty())
        .collect();
    puzzle5d_selection_outcome(selection, &payload.targets, parts, volumes)
}
//#endregion 🔖️Diff
