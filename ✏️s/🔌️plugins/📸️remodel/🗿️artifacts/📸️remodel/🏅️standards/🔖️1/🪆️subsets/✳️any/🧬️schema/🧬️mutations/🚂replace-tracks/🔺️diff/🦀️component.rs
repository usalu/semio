//! 🔺️ Sparse diff builder for `ReplaceTracks`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceTracks, base: &RemodelSnapshot) -> RemodelDiff {
    let mut results = base.results.clone();
    results.tracks = payload.tracks.clone();
    RemodelDiff { results: Some(results), ..Default::default() }
}
//#endregion 🔖️Diff
