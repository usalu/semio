//! 🔺️ Sparse diff builder for `CreateGcp` — inserts at the point's canonical `id` position so
//! `delete-gcp` puts it back exactly where it was. Duplicate `gcp.id` ⇒ Fatal `mutation.duplicate-id`.
use crate::diff::{RemodelingDiff, RemodelingRow};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateGcp, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A GCP with id \"{}\" already exists.", payload.gcp.id), [payload.gcp.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::gcp_rows(vec![RemodelingRow::Insert { entity: payload.gcp.clone() }]))
}
//#endregion 🔖️Diff
