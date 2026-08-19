//! ↩️ Inverse for `RemoveAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveAttribute, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.attributes.iter().find(|item| item.key == payload.key) { Some(existing) => vec![super::super::add_attribute::mutation::add_attribute(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
