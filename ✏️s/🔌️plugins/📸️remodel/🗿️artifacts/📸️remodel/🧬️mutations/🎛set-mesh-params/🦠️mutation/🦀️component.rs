//! 🎛 Remodel mutation — `SetMeshParams` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, params: &crate::artifacts::remodel::MeshParams) {
    next.params.mesh = params.clone();
}
//#endregion 🔖️Mutation
