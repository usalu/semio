//! ↩️ Inverse for `DeleteRigExtrinsic` — recreates the captured BASE record.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteRigExtrinsic, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        Some(extrinsic) => vec![crate::artifacts::remodeling::mutations::create_rig_extrinsic::create_rig_extrinsic(extrinsic.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
