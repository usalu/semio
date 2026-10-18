//! 🔺️ Sparse diff builder for `CreateCameraCalibration` — inserts at the camera's canonical `id`
//! position so `delete-camera-calibration` puts it back exactly where it was. Duplicate `camera.id` ⇒
//! Fatal.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::create_camera_calibration::component::CreateCameraCalibration;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateCameraCalibration, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A camera calibration with id \"{}\" already exists.", payload.camera.id), [payload.camera.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::camera_rows(vec![RemodelingRow::Insert { entity: payload.camera.clone() }]))
}
//#endregion 🔖️Diff
