//! ↩️ Inverse for `CreateRigExtrinsic` — `delete-rig-extrinsic` of the camera id it created.
//! A duplicate create was a no-op, so its inverse is too.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateRigExtrinsic, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    if base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        return Vec::new();
    }
    vec![crate::artifacts::remodeling::mutations::delete_rig_extrinsic::delete_rig_extrinsic(payload.extrinsic.camera_id.clone())]
}
//#endregion 🔖️Inverse
