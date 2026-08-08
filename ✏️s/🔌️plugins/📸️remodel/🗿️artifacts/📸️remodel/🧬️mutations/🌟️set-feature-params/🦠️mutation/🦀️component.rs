//! 🌟️ Remodel mutation — `SetFeatureParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::FeatureParams) {
    next.params.feature = params.clone();
}
//#endregion 🔖️Mutation
