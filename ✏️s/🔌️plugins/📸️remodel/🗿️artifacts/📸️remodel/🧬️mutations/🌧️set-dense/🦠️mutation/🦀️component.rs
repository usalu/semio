//! 🌧️ Remodel mutation — `SetDense` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, dense: &Option<crate::artifacts::remodel::DenseCloud>) {
    next.results.dense = dense.clone();
}
//#endregion 🔖️Mutation
