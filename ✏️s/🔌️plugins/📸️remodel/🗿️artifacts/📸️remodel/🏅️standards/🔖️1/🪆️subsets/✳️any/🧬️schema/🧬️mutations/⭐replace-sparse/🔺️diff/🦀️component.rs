//! 🔺️ Sparse diff builder for `ReplaceSparse`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceSparse, base: &RemodelSnapshot) -> RemodelDiff {
    let mut results = base.results.clone();
    results.sparse = payload.sparse.clone();
    RemodelDiff { results: Some(results), ..Default::default() }
}
//#endregion 🔖️Diff
