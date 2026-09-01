//! ↩️ Inverse for `RemoveAttribute`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::RemoveAttribute, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.attributes.iter().find(|item| item.key == payload.key) {
        Some(existing) => vec![super::super::add_attribute::add_attribute(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
