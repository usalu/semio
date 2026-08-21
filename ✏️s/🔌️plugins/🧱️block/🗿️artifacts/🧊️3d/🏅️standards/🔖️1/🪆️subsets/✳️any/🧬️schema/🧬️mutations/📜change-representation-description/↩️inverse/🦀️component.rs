//! ↩️ Inverse for `ChangeRepresentationDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeRepresentationDescription, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_description::mutation::change_representation_description(payload.id.clone(), existing.description.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
