//! ↩️ Inverse for `DeleteCameraCalibration` — recreates the captured BASE record.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DeleteCameraCalibration, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera_id) {
        Some(camera) => vec![crate::artifacts::remodel::mutations::create_camera_calibration::mutation::create_camera_calibration(camera.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
