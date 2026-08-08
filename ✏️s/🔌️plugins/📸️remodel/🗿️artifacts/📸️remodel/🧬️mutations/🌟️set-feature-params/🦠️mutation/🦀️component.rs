//! 🌟️ Remodel mutation — `SetFeatureParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::FeatureParams) {
    next.params.feature = params.clone();
}
//#endregion 🔖️Mutation
