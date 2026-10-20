//! 🔺️ Sparse diff builder for `ReplaceDense` — a whole-value swap of `results.dense`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingAssigned;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingResultsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::replace_dense::component::ReplaceDense, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.dense == base.results.dense {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Dense results already have this value.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { dense: Some(RemodelingAssigned::new(payload.dense.clone())), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
