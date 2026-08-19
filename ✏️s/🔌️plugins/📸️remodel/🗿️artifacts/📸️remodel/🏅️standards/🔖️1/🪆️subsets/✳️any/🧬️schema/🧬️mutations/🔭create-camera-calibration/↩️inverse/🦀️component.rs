//! ↩️ Inverse for `CreateCameraCalibration` — `delete-camera-calibration` of the id it created.
//! A duplicate create was a no-op, so its inverse is too.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateCameraCalibration, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return Vec::new();
    }
    vec![crate::artifacts::remodel::mutations::delete_camera_calibration::mutation::delete_camera_calibration(payload.camera.id.clone())]
}
//#endregion 🔖️Inverse
