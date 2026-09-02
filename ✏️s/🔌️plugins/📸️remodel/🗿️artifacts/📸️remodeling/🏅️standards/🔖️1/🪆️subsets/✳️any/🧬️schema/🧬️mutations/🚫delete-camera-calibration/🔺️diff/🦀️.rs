//! 🔺️ Sparse diff builder for `DeleteCameraCalibration`. Missing target ⇒ Error.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteCameraCalibration, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !base.calibration.cameras.iter().any(|camera| camera.id == payload.camera_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Camera calibration \"{}\" does not exist.", payload.camera_id), [payload.camera_id.clone()]);
    }
    let mut calibration = base.calibration.clone();
    calibration.cameras.retain(|camera| camera.id != payload.camera_id);
    protocol::MutationOutcome::new(RemodelingDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
