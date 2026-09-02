//! 🔺️ Sparse diff builder for `DeleteRigExtrinsic`. Missing target ⇒ Error.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteRigExtrinsic, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rig extrinsic for camera \"{}\" does not exist.", payload.camera_id), [payload.camera_id.clone()]);
    }
    let mut calibration = base.calibration.clone();
    calibration.rig.retain(|extrinsic| extrinsic.camera_id != payload.camera_id);
    protocol::MutationOutcome::new(RemodelingDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
