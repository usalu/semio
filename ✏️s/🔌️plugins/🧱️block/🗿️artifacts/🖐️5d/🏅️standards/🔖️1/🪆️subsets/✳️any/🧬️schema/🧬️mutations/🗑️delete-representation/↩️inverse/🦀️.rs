//! ↩️ Inverse for `DeleteRepresentation`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteRepresentation, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_representation::create_representation(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
