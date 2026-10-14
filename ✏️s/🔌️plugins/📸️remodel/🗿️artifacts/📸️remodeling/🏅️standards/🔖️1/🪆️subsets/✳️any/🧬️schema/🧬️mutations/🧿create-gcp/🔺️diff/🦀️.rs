//! 🔺️ Sparse diff builder for `CreateGcp` — inserts at the point's canonical `id` position so
//! `delete-gcp` puts it back exactly where it was. Duplicate `gcp.id` ⇒ Fatal `mutation.duplicate-id`.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateGcp, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if base.gcps.iter().any(|gcp| gcp.id == payload.gcp.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A GCP with id \"{}\" already exists.", payload.gcp.id), [payload.gcp.id.clone()]);
    }
    protocol::MutationOutcome::new(RemodelingDiff::gcp_rows(vec![RemodelingRow::Insert { entity: payload.gcp.clone() }]))
}
//#endregion 🔖️Diff
