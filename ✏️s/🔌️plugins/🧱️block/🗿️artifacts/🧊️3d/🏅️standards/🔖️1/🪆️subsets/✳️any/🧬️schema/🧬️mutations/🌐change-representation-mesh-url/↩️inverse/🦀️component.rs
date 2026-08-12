//! ↩️ Inverse for `ChangeRepresentationMeshUrl` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeRepresentationMeshUrl, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_representation_mesh_url::mutation::change_representation_mesh_url(payload.id.clone(), existing.mesh_url.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
