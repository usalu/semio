//! 🔺️ Sparse diff builder for `UpdateRigExtrinsic`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateRigExtrinsic, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        return RemodelDiff::default();
    }
    let mut calibration = base.calibration.clone();
    if let Some(existing) = calibration.rig.iter_mut().find(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        *existing = payload.extrinsic.clone();
    }
    RemodelDiff { calibration: Some(calibration), ..Default::default() }
}
//#endregion 🔖️Diff
