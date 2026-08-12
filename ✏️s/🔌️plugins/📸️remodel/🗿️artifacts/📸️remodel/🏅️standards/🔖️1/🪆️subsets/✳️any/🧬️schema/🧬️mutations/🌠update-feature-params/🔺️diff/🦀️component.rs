//! 🔺️ Sparse diff builder for `UpdateFeatureParams` — always present, no existence check needed.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateFeatureParams, base: &RemodelSnapshot) -> RemodelDiff {
    let mut params = base.params.clone();
    params.feature = payload.params.clone();
    RemodelDiff { params: Some(params), ..Default::default() }
}
//#endregion 🔖️Diff
