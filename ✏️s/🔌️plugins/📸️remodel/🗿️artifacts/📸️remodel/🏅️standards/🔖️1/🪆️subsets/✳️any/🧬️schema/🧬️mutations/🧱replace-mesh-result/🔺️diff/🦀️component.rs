//! 🔺️ Sparse diff builder for `ReplaceMeshResult`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceMeshResult, base: &RemodelSnapshot) -> RemodelDiff {
    let mut results = base.results.clone();
    results.mesh = (*payload.mesh).clone();
    RemodelDiff { results: Some(results), ..Default::default() }
}
//#endregion 🔖️Diff
