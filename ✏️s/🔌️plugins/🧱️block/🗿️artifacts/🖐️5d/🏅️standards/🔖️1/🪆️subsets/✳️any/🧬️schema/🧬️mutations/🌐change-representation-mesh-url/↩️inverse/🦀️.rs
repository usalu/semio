//! ↩️ Inverse for `ChangeRepresentationMeshUrl`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeRepresentationMeshUrl, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_mesh_url::change_representation_mesh_url(payload.id.clone(), existing.mesh_url.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
