//! ↩️ Inverse for `SetMeshParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMeshParams { params: base.params.mesh.clone() }]
}
//#endregion 🔖️Inverse
