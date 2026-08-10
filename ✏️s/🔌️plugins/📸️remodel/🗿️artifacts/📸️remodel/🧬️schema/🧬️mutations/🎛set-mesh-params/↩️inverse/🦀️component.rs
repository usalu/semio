//! ↩️ Inverse for `SetMeshParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMeshParams { params: base.params.mesh.clone() }]
}
//#endregion 🔖️Inverse
