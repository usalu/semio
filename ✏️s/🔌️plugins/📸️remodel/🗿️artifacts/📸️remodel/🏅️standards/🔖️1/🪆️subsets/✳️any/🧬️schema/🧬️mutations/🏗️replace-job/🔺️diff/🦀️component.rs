//! 🔺️ Sparse diff builder for `ReplaceJob`.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceJob, _base: &RemodelSnapshot) -> RemodelDiff {
    RemodelDiff { job: Some(payload.job.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
