//! ↩️ Inverse for `ChangeRepresentationMeshUrl`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeRepresentationMeshUrl, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_mesh_url::change_representation_mesh_url(payload.id.clone(), existing.mesh_url.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
