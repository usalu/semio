//! 🔺️ Sparse diff builder for `ReplaceSparse` — a whole-value swap of `results.sparse`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use crate::diff::{RemodelingAssigned, RemodelingDiff, RemodelingResultsDiff};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceSparse, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.sparse == base.results.sparse {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Sparse results already have this value.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { sparse: Some(RemodelingAssigned::new(payload.sparse.clone())), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
