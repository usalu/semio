//! 🔺️ Sparse diff builder for `ReplaceGeoProducts`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceGeoProducts, base: &RemodelSnapshot) -> RemodelDiff {
    let mut results = base.results.clone();
    results.geo = payload.geo.clone();
    RemodelDiff { results: Some(results), ..Default::default() }
}
//#endregion 🔖️Diff
