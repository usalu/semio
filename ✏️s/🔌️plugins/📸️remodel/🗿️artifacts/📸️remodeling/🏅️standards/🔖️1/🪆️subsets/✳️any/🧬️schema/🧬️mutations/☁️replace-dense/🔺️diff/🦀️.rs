//! 🔺️ Sparse diff builder for `ReplaceDense` — a whole-value swap of `results.dense`, which is
//! always present on the snapshot, so there is no missing-target case to detect.
use crate::diff::{RemodelingAssigned, RemodelingDiff, RemodelingResultsDiff};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceDense, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.dense == base.results.dense {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Dense results already have this value.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { dense: Some(RemodelingAssigned::new(payload.dense.clone())), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
