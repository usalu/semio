//! ↩️ Inverse for `DeleteCameraCalibration` — recreates the captured BASE record at its canonical
//! `id` position, which restores the document exactly: the delete refuses while anything references
//! the camera, so it removes nothing else.
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::create_camera_calibration::CreateCameraCalibration;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateCameraCalibration>>(payload: &super::DeleteCameraCalibration, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.calibration.cameras.iter().find(|camera| camera.id == payload.camera_id) {
        Some(camera) => vec![crate::standards::v1::subsets::any::schema::mutations::create_camera_calibration::create_camera_calibration(camera.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
