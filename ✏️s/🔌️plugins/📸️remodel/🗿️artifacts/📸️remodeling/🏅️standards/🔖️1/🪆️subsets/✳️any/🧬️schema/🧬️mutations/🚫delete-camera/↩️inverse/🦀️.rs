//! ↩️ Inverse for `DeleteCameraCalibration` — recreates the captured BASE record at its canonical
//! `id` position, which restores the document exactly: the delete refuses while anything references
//! the camera, so it removes nothing else.
//! Missing target ⇒ `Vec::new()`.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteCameraCalibration, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera_id) {
        Some(camera) => vec![crate::mutations::create_camera_calibration::create_camera_calibration(camera.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
