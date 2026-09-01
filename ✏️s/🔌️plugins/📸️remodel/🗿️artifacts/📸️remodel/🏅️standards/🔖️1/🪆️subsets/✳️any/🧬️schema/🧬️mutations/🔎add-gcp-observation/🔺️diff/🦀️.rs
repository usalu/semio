//! 🔺️ Sparse diff builder for `AddGcpObservation`. Missing owner GCP ⇒ Error; the exact observation
//! already present ⇒ Warning `mutation.no-op`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddGcpObservation, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if gcp.observations.contains(&payload.observation) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("GCP \"{}\" already has this observation.", payload.id));
    }
    let mut gcps = base.gcps.clone();
    if let Some(gcp) = gcps.iter_mut().find(|gcp| gcp.id == payload.id) {
        gcp.observations.push(payload.observation.clone());
    }
    protocol::MutationOutcome::new(RemodelDiff { gcps: Some(RemodelGcpList { values: gcps }), ..Default::default() })
}
//#endregion 🔖️Diff
