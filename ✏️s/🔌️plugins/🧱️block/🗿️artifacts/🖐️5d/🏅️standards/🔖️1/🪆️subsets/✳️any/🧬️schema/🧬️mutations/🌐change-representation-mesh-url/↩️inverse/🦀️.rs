//! ↩️ Inverse for `ChangeRepresentationMeshUrl`.

use crate::Block5dSnapshot;
use crate::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeRepresentationMeshUrl, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_mesh_url::change_representation_mesh_url(payload.id.clone(), existing.mesh_url.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
