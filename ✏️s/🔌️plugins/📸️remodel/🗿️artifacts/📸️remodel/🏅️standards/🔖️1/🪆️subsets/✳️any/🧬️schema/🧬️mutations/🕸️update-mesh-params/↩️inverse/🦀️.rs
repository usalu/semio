//! ↩️ Inverse for `UpdateMeshParams` — the OLD `MeshParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMeshParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::update_mesh_params(base.params.mesh.clone())]
}
//#endregion 🔖️Inverse
