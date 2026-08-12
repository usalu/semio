//! 🔺️ Sparse diff builder for `DeleteRigExtrinsic`. Missing target ⇒ `RemodelDiff::default()`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteRigExtrinsic, base: &RemodelSnapshot) -> RemodelDiff {
    if !base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        return RemodelDiff::default();
    }
    let mut calibration = base.calibration.clone();
    calibration.rig.retain(|extrinsic| extrinsic.camera_id != payload.camera_id);
    RemodelDiff { calibration: Some(calibration), ..Default::default() }
}
//#endregion 🔖️Diff
