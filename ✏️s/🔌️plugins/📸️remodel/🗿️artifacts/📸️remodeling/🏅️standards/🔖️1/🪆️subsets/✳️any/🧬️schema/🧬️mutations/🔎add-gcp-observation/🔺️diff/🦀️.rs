//! 🔺️ Sparse diff builder for `AddGcpObservation`, in the vocabulary's one guard order: a missing
//! owner GCP ⇒ Error `mutation.target-missing`, an observation naming a stream the document does not
//! carry ⇒ Error `mutation.target-missing` too (the referential half of `delete-stream`'s own guard), the exact
//! observation already present ⇒ Warning `mutation.no-op`. The observation lands at its canonical
//! `(stream_id, frame_index)` position so `remove-gcp-observation` puts it back where it was.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::GroundControlPointPatch;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingMembers;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::add_gcp_observation::component::AddGcpObservation, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.streams.iter().any(|stream| stream.id == payload.observation.stream_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" cannot be observed in unknown stream \"{}\".", payload.id, payload.observation.stream_id), [payload.observation.stream_id.clone()]);
    }
    if gcp.observations.contains(&payload.observation) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("GCP \"{}\" already has this observation.", payload.id));
    }
    protocol::MutationOutcome::new(RemodelingDiff::gcp_rows(vec![RemodelingRow::Patch { key: payload.id.clone(), patch: GroundControlPointPatch { observations: Some(RemodelingMembers { removed: Vec::new(), added: vec![payload.observation.clone()] }) } }]))
}
//#endregion 🔖️Diff
