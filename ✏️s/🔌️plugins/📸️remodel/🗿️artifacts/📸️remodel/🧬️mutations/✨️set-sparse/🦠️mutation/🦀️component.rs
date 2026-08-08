//! ✨️ Remodel mutation — `SetSparse` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, sparse: &Option<crate::artifacts::remodel::SparseCloud>) {
    next.results.sparse = sparse.clone();
}
//#endregion 🔖️Mutation
