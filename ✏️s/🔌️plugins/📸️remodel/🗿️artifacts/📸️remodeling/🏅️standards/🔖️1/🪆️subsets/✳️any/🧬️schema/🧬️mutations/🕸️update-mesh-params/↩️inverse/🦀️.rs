//! ↩️ Inverse for `UpdateMeshParams` — the OLD `MeshParams` looked up from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMeshParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_mesh_params(base.params.mesh.clone())]
}
//#endregion 🔖️Inverse
