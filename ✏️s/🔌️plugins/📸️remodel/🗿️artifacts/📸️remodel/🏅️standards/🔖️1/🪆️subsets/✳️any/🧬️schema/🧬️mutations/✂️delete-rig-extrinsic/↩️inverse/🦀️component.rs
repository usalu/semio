//! ↩️ Inverse for `DeleteRigExtrinsic` — recreates the captured BASE record.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteRigExtrinsic, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        Some(extrinsic) => vec![crate::artifacts::remodel::mutations::create_rig_extrinsic::mutation::create_rig_extrinsic(extrinsic.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
