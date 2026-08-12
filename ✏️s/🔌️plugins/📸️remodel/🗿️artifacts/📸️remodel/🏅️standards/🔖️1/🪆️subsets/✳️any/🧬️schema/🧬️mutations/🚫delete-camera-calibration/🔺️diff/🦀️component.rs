//! 🔺️ Sparse diff builder for `DeleteCameraCalibration`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteCameraCalibration, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.calibration.cameras.iter().any(|camera| camera.id == payload.camera_id) {
        return RemodelDiff::default();
    }
    let mut calibration = base.calibration.clone();
    calibration.cameras.retain(|camera| camera.id != payload.camera_id);
    RemodelDiff { calibration: Some(calibration), ..Default::default() }
}
//#endregion 🔖️Diff
