//! 🔺️ Sparse diff builder for `UpdateMatchParams` — always present, no existence check needed.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateMatchParams, base: &RemodelSnapshot) -> RemodelDiff {
    let mut params = base.params.clone();
    params.matching = payload.params.clone();
    RemodelDiff { params: Some(params), ..Default::default() }
}
//#endregion 🔖️Diff
