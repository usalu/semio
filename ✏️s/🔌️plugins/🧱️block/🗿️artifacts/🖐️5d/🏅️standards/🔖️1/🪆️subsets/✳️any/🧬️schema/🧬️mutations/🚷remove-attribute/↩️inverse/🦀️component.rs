//! ↩️ Inverse for `RemoveAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RemoveAttribute, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.attributes.iter().find(|item| item.key == payload.key) { Some(existing) => vec![super::super::add_attribute::mutation::add_attribute(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
