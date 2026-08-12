//! 🔺️ Sparse diff builder for `UpdateCameraCalibration`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateCameraCalibration, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return RemodelDiff::default();
    }
    let mut calibration = base.calibration.clone();
    if let Some(existing) = calibration.cameras.iter_mut().find(|camera| camera.id == payload.camera.id) {
        *existing = payload.camera.clone();
    }
    RemodelDiff { calibration: Some(calibration), ..Default::default() }
}
//#endregion 🔖️Diff
