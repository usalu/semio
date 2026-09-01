//! 🔺️ Sparse diff builder for `CreateGcp`. Duplicate `gcp.id` ⇒ Fatal `mutation.duplicate-id`.
use crate::artifacts::remodel::diff::{RemodelDiff, RemodelGcpList};
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateGcp, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A GCP with id \"{}\" already exists.", payload.gcp.id), [payload.gcp.id.clone()]);
    }
    let mut gcps = base.gcps.clone();
    gcps.push(payload.gcp.clone());
    protocol::MutationOutcome::new(RemodelDiff { gcps: Some(RemodelGcpList { values: gcps }), ..Default::default() })
}
//#endregion 🔖️Diff
