//! ↩️ Inverse for `UpdateMeshParams` — the OLD `MeshParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::UpdateMeshParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::update_mesh_params(base.params.mesh.clone())]
}
//#endregion 🔖️Inverse
