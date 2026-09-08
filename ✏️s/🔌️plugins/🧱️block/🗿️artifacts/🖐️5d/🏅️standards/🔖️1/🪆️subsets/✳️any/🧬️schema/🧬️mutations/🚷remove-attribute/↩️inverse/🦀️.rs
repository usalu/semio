//! ↩️ Inverse for `RemoveAttribute`.

use crate::Block5dSnapshot;
use crate::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAttribute, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.attributes.iter().find(|item| item.key == payload.key) {
        Some(existing) => vec![super::super::add_attribute::add_attribute(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
