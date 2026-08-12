//! 🔺️ Sparse diff builder for `UpdateSfmParams` — always present, no existence check needed.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateSfmParams, base: &RemodelSnapshot) -> RemodelDiff {
    let mut params = base.params.clone();
    params.sfm = payload.params.clone();
    RemodelDiff { params: Some(params), ..Default::default() }
}
//#endregion 🔖️Diff
