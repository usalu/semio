//! 🔺️ Sparse diff builder for `ReplaceDense`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceDense, base: &RemodelSnapshot) -> RemodelDiff {
    let mut results = base.results.clone();
    results.dense = payload.dense.clone();
    RemodelDiff { results: Some(results), ..Default::default() }
}
//#endregion 🔖️Diff
