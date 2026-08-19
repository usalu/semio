//! 🔺️ Sparse diff builder for `ReplaceSparse` — a whole-value swap of `results.sparse`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceSparse, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.sparse == base.results.sparse {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Sparse results already have this value.");
    }
    let mut results = base.results.clone();
    results.sparse = payload.sparse.clone();
    protocol::MutationOutcome::new(RemodelDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
