//! 🔺️ Sparse diff builder for `RemoveGcpObservation`. A missing GCP or an out-of-range index ⇒
//! Error `mutation.target-missing`.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingGcpList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveGcpObservation, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.observation_index as usize >= gcp.observations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" has no observation at index {}.", payload.id, payload.observation_index), [payload.id.clone()]);
    }
    let mut gcps = base.gcps.clone();
    if let Some(gcp) = gcps.iter_mut().find(|gcp| gcp.id == payload.id) {
        gcp.observations.remove(payload.observation_index as usize);
    }
    protocol::MutationOutcome::new(RemodelingDiff { gcps: Some(RemodelingGcpList { values: gcps }), ..Default::default() })
}
//#endregion 🔖️Diff
