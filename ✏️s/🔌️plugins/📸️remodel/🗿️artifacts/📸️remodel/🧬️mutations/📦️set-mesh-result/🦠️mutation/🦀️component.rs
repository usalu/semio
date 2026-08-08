//! 📦️ Remodel mutation — `SetMeshResult` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, mesh: &Box<crate::artifacts::remodel::RemodelMesh>) {
    next.results.mesh = mesh.as_ref().clone();
}
//#endregion 🔖️Mutation
