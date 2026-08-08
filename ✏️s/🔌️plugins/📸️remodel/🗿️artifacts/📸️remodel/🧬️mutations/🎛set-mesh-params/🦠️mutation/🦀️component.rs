//! 🎛 Remodel mutation — `SetMeshParams` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, params: &crate::artifacts::remodel::MeshParams) {
    next.params.mesh = params.clone();
}
//#endregion 🔖️Mutation
