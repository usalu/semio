//! 🔺️ Sparse diff builder for `DeleteCameraCalibration`. Missing target ⇒ Error.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteCameraCalibration, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if !base.calibration.cameras.iter().any(|camera| camera.id == payload.camera_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Camera calibration \"{}\" does not exist.", payload.camera_id), [payload.camera_id.clone()]);
    }
    let mut calibration = base.calibration.clone();
    calibration.cameras.retain(|camera| camera.id != payload.camera_id);
    protocol::MutationOutcome::new(RemodelDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
