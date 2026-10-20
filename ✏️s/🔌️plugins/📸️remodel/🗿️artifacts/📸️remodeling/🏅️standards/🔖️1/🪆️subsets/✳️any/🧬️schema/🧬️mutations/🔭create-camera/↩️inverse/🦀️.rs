//! ↩️ Inverse for `CreateCameraCalibration` — `delete-camera-calibration` of the id it created.
//! A duplicate create was a no-op, so its inverse is too.
use crate::standards::v1::subsets::any::schema::mutations::delete_camera_calibration::DeleteCameraCalibration;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::create_camera_calibration::component::CreateCameraCalibration;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteCameraCalibration>>(payload: &crate::standards::v1::subsets::any::schema::mutations::create_camera_calibration::component::CreateCameraCalibration, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return Vec::new();
    }
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_camera_calibration::delete_camera_calibration(payload.camera.id.clone())]

    })())
}
//#endregion 🔖️Inverse
