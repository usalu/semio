//! 🔺️ Sparse diff builder for `CreateRigExtrinsic`. Duplicate `camera_id` ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateRigExtrinsic, base: &RemodelSnapshot) -> RemodelDiff {
    if base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        return RemodelDiff::default();
    }
    let mut calibration = base.calibration.clone();
    calibration.rig.push(payload.extrinsic.clone());
    RemodelDiff { calibration: Some(calibration), ..Default::default() }
}
//#endregion 🔖️Diff
