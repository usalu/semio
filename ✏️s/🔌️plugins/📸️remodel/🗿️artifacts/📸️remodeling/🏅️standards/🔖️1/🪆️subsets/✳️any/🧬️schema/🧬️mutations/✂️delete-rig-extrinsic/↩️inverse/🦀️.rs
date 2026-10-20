//! ↩️ Inverse for `DeleteRigExtrinsic` — recreates the captured BASE record at its canonical
//! `camera_id` position.
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::create_rig_extrinsic::CreateRigExtrinsic;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateRigExtrinsic>>(payload: &crate::standards::v1::subsets::any::schema::mutations::delete_rig_extrinsic::component::DeleteRigExtrinsic, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.calibration.rig.iter().find(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        Some(extrinsic) => vec![crate::standards::v1::subsets::any::schema::mutations::create_rig_extrinsic::create_rig_extrinsic(extrinsic.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
