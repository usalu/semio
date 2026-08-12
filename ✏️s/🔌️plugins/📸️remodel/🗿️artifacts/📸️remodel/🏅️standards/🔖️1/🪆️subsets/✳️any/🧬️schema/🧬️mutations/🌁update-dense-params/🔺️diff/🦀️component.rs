//! 🔺️ Sparse diff builder for `UpdateDenseParams` — always present, no existence check needed.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateDenseParams, base: &RemodelSnapshot) -> RemodelDiff {
    let mut params = base.params.clone();
    params.dense = payload.params.clone();
    RemodelDiff { params: Some(params), ..Default::default() }
}
//#endregion 🔖️Diff
