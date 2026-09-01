//! ↩️ Inverse for `DeleteCameraCalibration` — recreates the captured BASE record.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteCameraCalibration, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera_id) {
        Some(camera) => vec![crate::artifacts::remodel::mutations::create_camera_calibration::create_camera_calibration(camera.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
