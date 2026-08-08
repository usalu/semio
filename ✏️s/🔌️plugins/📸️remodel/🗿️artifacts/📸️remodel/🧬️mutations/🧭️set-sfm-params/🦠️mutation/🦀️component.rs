//! 🧭️ Remodel mutation — `SetSfmParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::SfmParams) {
    next.params.sfm = params.clone();
}
//#endregion 🔖️Mutation
