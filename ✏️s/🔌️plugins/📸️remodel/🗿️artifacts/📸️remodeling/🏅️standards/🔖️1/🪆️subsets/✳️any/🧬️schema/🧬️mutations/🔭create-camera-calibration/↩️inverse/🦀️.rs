//! ↩️ Inverse for `CreateCameraCalibration` — `delete-camera-calibration` of the id it created.
//! A duplicate create was a no-op, so its inverse is too.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateCameraCalibration, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return Vec::new();
    }
    vec![crate::artifacts::remodeling::mutations::delete_camera_calibration::delete_camera_calibration(payload.camera.id.clone())]
}
//#endregion 🔖️Inverse
