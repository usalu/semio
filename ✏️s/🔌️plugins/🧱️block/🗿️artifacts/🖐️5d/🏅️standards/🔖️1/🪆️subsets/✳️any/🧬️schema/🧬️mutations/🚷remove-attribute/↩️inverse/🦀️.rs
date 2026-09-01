//! ↩️ Inverse for `RemoveAttribute`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::RemoveAttribute, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.attributes.iter().find(|item| item.key == payload.key) {
        Some(existing) => vec![super::super::add_attribute::add_attribute(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
