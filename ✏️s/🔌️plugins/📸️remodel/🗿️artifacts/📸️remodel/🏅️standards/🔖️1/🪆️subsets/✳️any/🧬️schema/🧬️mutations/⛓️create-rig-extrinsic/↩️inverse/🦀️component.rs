//! ↩️ Inverse for `CreateRigExtrinsic` — `delete-rig-extrinsic` of the camera id it created.
//! A duplicate create was a no-op, so its inverse is too.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateRigExtrinsic, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    if base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        return Vec::new();
    }
    vec![crate::artifacts::remodel::mutations::delete_rig_extrinsic::mutation::delete_rig_extrinsic(payload.extrinsic.camera_id.clone())]
}
//#endregion 🔖️Inverse
