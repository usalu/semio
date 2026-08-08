//! 📦️ Remodel mutation — `SetMeshResult` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, mesh: &Box<crate::artifacts::remodel::RemodelMesh>) {
    next.results.mesh = mesh.as_ref().clone();
}
//#endregion 🔖️Mutation
