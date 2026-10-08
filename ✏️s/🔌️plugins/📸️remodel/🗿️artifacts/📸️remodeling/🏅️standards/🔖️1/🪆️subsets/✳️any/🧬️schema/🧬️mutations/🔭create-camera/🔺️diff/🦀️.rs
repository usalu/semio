//! 🔺️ Sparse diff builder for `CreateCameraCalibration` — inserts at the camera's canonical `id`
//! position so `delete-camera-calibration` puts it back exactly where it was. Duplicate `camera.id` ⇒
//! Fatal.
use crate::diff::{RemodelingDiff, RemodelingRow};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateCameraCalibration, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A camera calibration with id \"{}\" already exists.", payload.camera.id), [payload.camera.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::camera_rows(vec![RemodelingRow::Insert { entity: payload.camera.clone() }]))
}
//#endregion 🔖️Diff
