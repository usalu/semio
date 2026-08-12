//! ↩️ Inverse for `UpdateCameraCalibration` — the OLD full record looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::UpdateCameraCalibration, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera.id) {
        Some(old) => vec![super::mutation::update_camera_calibration(old.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
