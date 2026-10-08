//! 🔺️ Sparse diff builder for `RemoveGcpObservation`. A missing GCP or an out-of-range index ⇒
//! Error `mutation.target-missing`.
use crate::diff::{RemodelingDiff, RemodelingRow, GroundControlPointPatch, RemodelingMembers};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveGcpObservation, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.observation_index as usize >= gcp.observations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" has no observation at index {}.", payload.id, payload.observation_index), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::gcp_rows(vec![RemodelingRow::Patch { key: payload.id.clone(), patch: GroundControlPointPatch { observations: Some(RemodelingMembers { removed: vec![gcp.observations[payload.observation_index as usize].clone()], added: Vec::new() }) } }]))
}
//#endregion 🔖️Diff
