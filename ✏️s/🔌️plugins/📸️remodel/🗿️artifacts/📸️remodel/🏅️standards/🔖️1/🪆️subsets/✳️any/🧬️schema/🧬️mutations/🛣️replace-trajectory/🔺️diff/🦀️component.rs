//! 🔺️ Sparse diff builder for `ReplaceTrajectory`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceTrajectory, base: &RemodelSnapshot) -> RemodelDiff {
    let mut results = base.results.clone();
    results.trajectory = payload.trajectory.clone();
    RemodelDiff { results: Some(results), ..Default::default() }
}
//#endregion 🔖️Diff
