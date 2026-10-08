//! 🔺️ Sparse diff builder for `DeleteRigExtrinsic`. Missing target ⇒ Error.
use crate::diff::{RemodelingDiff, RemodelingRow};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteRigExtrinsic, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rig extrinsic for camera \"{}\" does not exist.", payload.camera_id), [payload.camera_id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::rig_rows(vec![RemodelingRow::Remove { key: payload.camera_id.clone() }]))
}
//#endregion 🔖️Diff
