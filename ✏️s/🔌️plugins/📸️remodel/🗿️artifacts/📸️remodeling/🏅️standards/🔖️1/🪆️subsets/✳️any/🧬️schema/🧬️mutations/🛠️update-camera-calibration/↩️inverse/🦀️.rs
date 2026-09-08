//! ↩️ Inverse for `UpdateCameraCalibration` — the OLD full record looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::UpdateCameraCalibration, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera.id) {
        Some(old) => vec![super::update_camera_calibration(old.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
