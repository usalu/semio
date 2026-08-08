//! ↩️ Inverse for `SetMeshResult`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMeshResult { mesh: Box::new(base.results.mesh.clone()) }]
}
//#endregion 🔖️Inverse
