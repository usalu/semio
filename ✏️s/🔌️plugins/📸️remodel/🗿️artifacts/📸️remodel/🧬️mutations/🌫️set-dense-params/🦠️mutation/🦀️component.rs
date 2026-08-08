//! 🌫️ Remodel mutation — `SetDenseParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::DenseParams) {
    next.params.dense = params.clone();
}
//#endregion 🔖️Mutation
