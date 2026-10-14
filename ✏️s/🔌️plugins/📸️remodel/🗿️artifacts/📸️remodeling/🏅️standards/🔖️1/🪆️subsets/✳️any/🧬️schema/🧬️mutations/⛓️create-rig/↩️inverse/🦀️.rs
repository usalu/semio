//! ↩️ Inverse for `CreateRigExtrinsic` — `delete-rig-extrinsic` of the camera id it created.
//! A duplicate create was a no-op, so its inverse is too.
use crate::standards::v1::subsets::any::schema::mutations::delete_rig_extrinsic::DeleteRigExtrinsic;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteRigExtrinsic>>(payload: &super::CreateRigExtrinsic, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        return Vec::new();
    }
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_rig_extrinsic::delete_rig_extrinsic(payload.extrinsic.camera_id.clone())]

    })())
}
//#endregion 🔖️Inverse
