//! ↩️ Inverse for `SetMeshResult`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMeshResult { mesh: Box::new(base.results.mesh.clone()) }]
}
//#endregion 🔖️Inverse
