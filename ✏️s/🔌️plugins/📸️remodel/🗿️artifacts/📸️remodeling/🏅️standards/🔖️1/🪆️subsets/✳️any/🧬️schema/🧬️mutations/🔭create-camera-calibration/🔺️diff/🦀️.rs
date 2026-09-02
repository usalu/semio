//! 🔺️ Sparse diff builder for `CreateCameraCalibration`. Duplicate `camera.id` ⇒ Fatal.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateCameraCalibration, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A camera calibration with id \"{}\" already exists.", payload.camera.id), [payload.camera.id.clone()]);
    }
    let mut calibration = base.calibration.clone();
    calibration.cameras.push(payload.camera.clone());
    protocol::MutationOutcome::new(RemodelingDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
