//! ↩️ Inverse for `UpdateRigExtrinsic` — the OLD full record looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::UpdateRigExtrinsic, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        Some(old) => vec![super::mutation::update_rig_extrinsic(old.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
