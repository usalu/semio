//! 🔺️ Sparse diff builder for `ReplaceSparse` — a whole-value swap of `results.sparse`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingAssigned;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingResultsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::replace_sparse::component::ReplaceSparse, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.sparse == base.results.sparse {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Sparse results already have this value.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { sparse: Some(RemodelingAssigned::new(payload.sparse.clone())), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
