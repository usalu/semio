//! 🔺️ Sparse diff builder for `RemoveGcpObservation`. A missing GCP or an out-of-range index ⇒
//! Error `mutation.target-missing`.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::GroundControlPointPatch;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingMembers;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::remove_gcp_observation::component::RemoveGcpObservation, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.observation_index as usize >= gcp.observations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" has no observation at index {}.", payload.id, payload.observation_index), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::gcp_rows(vec![RemodelingRow::Patch { key: payload.id.clone(), patch: GroundControlPointPatch { observations: Some(RemodelingMembers { removed: vec![gcp.observations[payload.observation_index as usize].clone()], added: Vec::new() }) } }]))
}
//#endregion 🔖️Diff
