//! ↩️ Inverse for `AddAttribute` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddAttribute, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_attribute::mutation::remove_attribute(payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
