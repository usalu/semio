//! ↩️ Inverse for `DeleteCameraCalibration` — recreates the captured BASE record.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteCameraCalibration, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera_id) {
        Some(camera) => vec![crate::artifacts::remodeling::mutations::create_camera_calibration::create_camera_calibration(camera.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
