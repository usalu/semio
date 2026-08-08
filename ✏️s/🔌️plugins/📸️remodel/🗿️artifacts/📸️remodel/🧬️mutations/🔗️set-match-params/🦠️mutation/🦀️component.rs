//! 🔗️ Remodel mutation — `SetMatchParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::MatchParams) {
    next.params.matching = params.clone();
}
//#endregion 🔖️Mutation
