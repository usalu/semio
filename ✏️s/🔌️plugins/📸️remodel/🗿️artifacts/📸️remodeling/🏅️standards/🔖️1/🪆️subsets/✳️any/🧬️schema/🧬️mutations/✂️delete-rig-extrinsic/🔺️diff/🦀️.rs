//! 🔺️ Sparse diff builder for `DeleteRigExtrinsic`. Missing target ⇒ Error.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::delete_rig_extrinsic::component::DeleteRigExtrinsic, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !base.calibration.rig.iter().any(|extrinsic| extrinsic.camera_id == payload.camera_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Rig extrinsic for camera \"{}\" does not exist.", payload.camera_id), [payload.camera_id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::rig_rows(vec![RemodelingRow::Remove { key: payload.camera_id.clone() }]))
}
//#endregion 🔖️Diff
