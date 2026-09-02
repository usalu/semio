//! 🔺️ Sparse diff builder for `CreateGcp`. Duplicate `gcp.id` ⇒ Fatal `mutation.duplicate-id`.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingGcpList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateGcp, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A GCP with id \"{}\" already exists.", payload.gcp.id), [payload.gcp.id.clone()]);
    }
    let mut gcps = base.gcps.clone();
    gcps.push(payload.gcp.clone());
    protocol::MutationOutcome::new(RemodelingDiff { gcps: Some(RemodelingGcpList { values: gcps }), ..Default::default() })
}
//#endregion 🔖️Diff
