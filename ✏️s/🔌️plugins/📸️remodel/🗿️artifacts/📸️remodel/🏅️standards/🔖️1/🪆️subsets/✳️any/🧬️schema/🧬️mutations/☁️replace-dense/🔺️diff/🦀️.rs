//! 🔺️ Sparse diff builder for `ReplaceDense` — a whole-value swap of `results.dense`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceDense, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.dense == base.results.dense {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Dense results already have this value.");
    }
    let mut results = base.results.clone();
    results.dense = payload.dense.clone();
    protocol::MutationOutcome::new(RemodelDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
