//! 🔺️ Sparse diff builder for `CreateCameraCalibration`. Duplicate `camera.id` ⇒ Fatal.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::CreateCameraCalibration, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if base.calibration.cameras.iter().any(|camera| camera.id == payload.camera.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A camera calibration with id \"{}\" already exists.", payload.camera.id), [payload.camera.id.clone()]);
    }
    let mut calibration = base.calibration.clone();
    calibration.cameras.push(payload.camera.clone());
    protocol::MutationOutcome::new(RemodelDiff { calibration: Some(calibration), ..Default::default() })
}
//#endregion 🔖️Diff
