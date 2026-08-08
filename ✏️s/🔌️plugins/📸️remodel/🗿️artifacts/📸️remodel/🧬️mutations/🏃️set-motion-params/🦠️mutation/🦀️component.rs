//! 🏃️ Remodel mutation — `SetMotionParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::MotionParams) {
    next.params.motion = params.clone();
}
//#endregion 🔖️Mutation
