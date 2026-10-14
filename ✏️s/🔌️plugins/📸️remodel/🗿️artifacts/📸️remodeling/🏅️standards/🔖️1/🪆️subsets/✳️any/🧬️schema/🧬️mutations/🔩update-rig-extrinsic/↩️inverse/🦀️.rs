//! ↩️ Inverse for `UpdateRigExtrinsic` — the OLD full record looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::update_rig_extrinsic::UpdateRigExtrinsic;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateRigExtrinsic>>(payload: &super::UpdateRigExtrinsic, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.extrinsic.camera_id) {
        Some(old) => vec![super::update_rig_extrinsic(old.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
