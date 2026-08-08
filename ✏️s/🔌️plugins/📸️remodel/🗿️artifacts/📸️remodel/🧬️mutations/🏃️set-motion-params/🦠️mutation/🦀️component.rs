//! 🏃️ Remodel mutation — `SetMotionParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::MotionParams) {
    next.params.motion = params.clone();
}
//#endregion 🔖️Mutation
