//! 🌧️ Remodel mutation — `SetDense` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, dense: &Option<crate::artifacts::remodel::DenseCloud>) {
    next.results.dense = dense.clone();
}
//#endregion 🔖️Mutation
