//! ↩️ Inverse for `UpdateMeshParams` — the OLD `MeshParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMeshParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_mesh_params(base.params.mesh.clone())]
}
//#endregion 🔖️Inverse
