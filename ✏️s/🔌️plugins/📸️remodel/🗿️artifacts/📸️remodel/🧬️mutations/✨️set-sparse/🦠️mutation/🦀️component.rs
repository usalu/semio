//! ✨️ Remodel mutation — `SetSparse` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, sparse: &Option<crate::artifacts::remodel::SparseCloud>) {
    next.results.sparse = sparse.clone();
}
//#endregion 🔖️Mutation
