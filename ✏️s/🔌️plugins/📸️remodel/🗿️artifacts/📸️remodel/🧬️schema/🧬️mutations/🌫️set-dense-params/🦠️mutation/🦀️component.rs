//! 🌫️ Remodel mutation — `SetDenseParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::DenseParams) {
    next.params.dense = params.clone();
}
//#endregion 🔖️Mutation
