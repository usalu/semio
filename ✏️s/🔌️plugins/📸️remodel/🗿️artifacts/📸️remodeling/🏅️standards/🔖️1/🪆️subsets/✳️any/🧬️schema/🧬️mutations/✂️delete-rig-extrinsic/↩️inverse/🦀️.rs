//! ↩️ Inverse for `DeleteRigExtrinsic` — recreates the captured BASE record at its canonical
//! `camera_id` position.
//! Missing target ⇒ `Vec::new()`.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteRigExtrinsic, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        Some(extrinsic) => vec![crate::mutations::create_rig_extrinsic::create_rig_extrinsic(extrinsic.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
