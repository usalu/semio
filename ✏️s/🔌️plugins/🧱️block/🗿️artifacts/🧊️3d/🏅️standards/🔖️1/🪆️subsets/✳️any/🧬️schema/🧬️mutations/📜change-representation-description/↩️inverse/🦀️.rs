//! ↩️ Inverse for `ChangeRepresentationDescription`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeRepresentationDescription, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_description::change_representation_description(payload.id.clone(), existing.description.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
