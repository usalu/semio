//! 🔗️ Remodel mutation — `SetMatchParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::MatchParams) {
    next.params.matching = params.clone();
}
//#endregion 🔖️Mutation
