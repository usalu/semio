//! 🔺️ Sparse diff builder for `CreateRigExtrinsic`. A duplicate `camera_id` ⇒ Fatal
//! `mutation.duplicate-id`; a `camera_id` referencing an unknown camera ⇒ Fatal
//! `mutation.invariant`.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRigExtrinsic, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A rig extrinsic for camera \"{}\" already exists.", payload.extrinsic.camera_id), [payload.extrinsic.camera_id.clone()]);
    }
    if !base.calibration.cameras.iter().any(|camera| camera.id == payload.extrinsic.camera_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rig extrinsic references unknown camera \"{}\".", payload.extrinsic.camera_id), [payload.extrinsic.camera_id.clone()]);
    }
    let mut calibration = base.calibration.clone();
    calibration.rig.push(payload.extrinsic.clone());
    protocol::MutationOutcome::new(RemodelingDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
