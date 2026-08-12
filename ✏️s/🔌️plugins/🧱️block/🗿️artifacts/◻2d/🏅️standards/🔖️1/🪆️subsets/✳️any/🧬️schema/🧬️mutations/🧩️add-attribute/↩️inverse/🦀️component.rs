//! ↩️ Inverse for `AddAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddAttribute, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::remove_attribute::mutation::remove_attribute(payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
