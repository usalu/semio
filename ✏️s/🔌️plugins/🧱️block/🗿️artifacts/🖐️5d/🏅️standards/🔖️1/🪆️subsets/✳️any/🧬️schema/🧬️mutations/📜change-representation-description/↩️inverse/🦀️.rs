//! ↩️ Inverse for `ChangeRepresentationDescription`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeRepresentationDescription, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_representation_description::change_representation_description(payload.id.clone(), existing.description.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
