//! 🧭️ Remodel mutation — `SetSfmParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::SfmParams) {
    next.params.sfm = params.clone();
}
//#endregion 🔖️Mutation
