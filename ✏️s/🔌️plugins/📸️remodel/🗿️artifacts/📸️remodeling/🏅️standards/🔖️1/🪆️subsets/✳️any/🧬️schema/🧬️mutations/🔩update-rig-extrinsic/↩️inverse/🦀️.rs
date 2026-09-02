//! ↩️ Inverse for `UpdateRigExtrinsic` — the OLD full record looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::UpdateRigExtrinsic, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        Some(old) => vec![super::update_rig_extrinsic(old.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
