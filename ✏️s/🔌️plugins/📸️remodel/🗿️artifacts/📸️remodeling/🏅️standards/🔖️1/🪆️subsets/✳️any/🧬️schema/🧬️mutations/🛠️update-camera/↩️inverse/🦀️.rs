//! ↩️ Inverse for `UpdateCameraCalibration` — the OLD full record looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::update_camera_calibration::component::UpdateCameraCalibration;
use crate::standards::v1::subsets::any::schema::mutations::update_camera_calibration::component::update_camera_calibration;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateCameraCalibration>>(payload: &super::UpdateCameraCalibration, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera.id) {
        Some(old) => vec![super::update_camera_calibration(old.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
