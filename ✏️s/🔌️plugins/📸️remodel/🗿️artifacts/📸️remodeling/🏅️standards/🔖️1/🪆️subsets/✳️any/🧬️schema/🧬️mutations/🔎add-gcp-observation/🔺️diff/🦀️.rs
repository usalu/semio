//! 🔺️ Sparse diff builder for `AddGcpObservation`, in the vocabulary's one guard order: a missing
//! owner GCP ⇒ Error `mutation.target-missing`, an observation naming a stream the document does not
//! carry ⇒ Fatal `mutation.invariant` (the referential half of `delete-stream`'s own guard), the exact
//! observation already present ⇒ Warning `mutation.no-op`. The observation lands at its canonical
//! `(stream_id, frame_index)` position so `remove-gcp-observation` puts it back where it was.
use crate::diff::{RemodelingDiff, RemodelingGcpList};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddGcpObservation, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.streams.iter().any(|stream| stream.id == payload.observation.stream_id) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("GCP \"{}\" cannot be observed in unknown stream \"{}\".", payload.id, payload.observation.stream_id), [payload.observation.stream_id.clone()]);
    }
    if gcp.observations.contains(&payload.observation) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("GCP \"{}\" already has this observation.", payload.id));
    }
    let mut gcps = base.gcps.clone();
    if let Some(gcp) = gcps.iter_mut().find(|gcp| gcp.id == payload.id) {
        let at = crate::mutations::ordered_index(&gcp.observations, &(payload.observation.stream_id.clone(), payload.observation.frame_index), |observation| (observation.stream_id.clone(), observation.frame_index));
        gcp.observations.insert(at, payload.observation.clone());
    }
    protocol::MutationOutcome::new(RemodelingDiff { gcps: Some(RemodelingGcpList { values: gcps }), ..Default::default() })
}
//#endregion 🔖️Diff
