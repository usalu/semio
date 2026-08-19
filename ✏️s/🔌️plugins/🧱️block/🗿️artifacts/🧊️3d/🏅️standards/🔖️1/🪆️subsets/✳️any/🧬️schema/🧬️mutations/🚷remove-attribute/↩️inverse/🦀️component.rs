//! ↩️ Inverse for `RemoveAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveAttribute, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.attributes.iter().find(|item| item.key == payload.key) { Some(existing) => vec![super::super::add_attribute::mutation::add_attribute(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
