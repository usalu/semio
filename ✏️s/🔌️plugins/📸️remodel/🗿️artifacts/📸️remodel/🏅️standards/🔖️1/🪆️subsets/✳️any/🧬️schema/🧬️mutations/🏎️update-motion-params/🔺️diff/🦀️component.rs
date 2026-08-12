//! 🔺️ Sparse diff builder for `UpdateMotionParams` — always present, no existence check needed.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateMotionParams, base: &RemodelSnapshot) -> RemodelDiff {
    let mut params = base.params.clone();
    params.motion = payload.params.clone();
    RemodelDiff { params: Some(params), ..Default::default() }
}
//#endregion 🔖️Diff
