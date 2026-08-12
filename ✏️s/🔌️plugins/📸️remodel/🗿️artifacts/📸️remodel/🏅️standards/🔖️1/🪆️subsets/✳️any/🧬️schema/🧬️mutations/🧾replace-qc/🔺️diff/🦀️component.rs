//! 🔺️ Sparse diff builder for `ReplaceQc`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceQc, base: &RemodelSnapshot) -> RemodelDiff {
    let mut results = base.results.clone();
    results.qc = payload.qc.clone();
    RemodelDiff { results: Some(results), ..Default::default() }
}
//#endregion 🔖️Diff
