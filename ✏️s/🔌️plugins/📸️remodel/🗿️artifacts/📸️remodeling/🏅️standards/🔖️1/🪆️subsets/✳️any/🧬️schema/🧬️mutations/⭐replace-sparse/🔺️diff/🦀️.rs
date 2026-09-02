//! 🔺️ Sparse diff builder for `ReplaceSparse` — a whole-value swap of `results.sparse`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceSparse, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.sparse == base.results.sparse {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Sparse results already have this value.");
    }
    let mut results = base.results.clone();
    results.sparse = payload.sparse.clone();
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
