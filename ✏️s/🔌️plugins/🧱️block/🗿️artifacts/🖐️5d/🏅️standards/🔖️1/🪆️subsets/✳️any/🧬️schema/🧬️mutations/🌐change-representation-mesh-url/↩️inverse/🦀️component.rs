//! ↩️ Inverse for `ChangeRepresentationMeshUrl` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeRepresentationMeshUrl, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_mesh_url::mutation::change_representation_mesh_url(payload.id.clone(), existing.mesh_url.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
