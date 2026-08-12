//! 🔺️ Sparse diff builder for `CreateCameraCalibration`. Duplicate `camera.id` ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateCameraCalibration, base: &RemodelSnapshot) -> RemodelDiff {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return RemodelDiff::default();
    }
    let mut calibration = base.calibration.clone();
    calibration.cameras.push(payload.camera.clone());
    RemodelDiff { calibration: Some(calibration), ..Default::default() }
}
//#endregion 🔖️Diff
