//! ↩️ Inverse for `AddAttribute`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAttribute, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_attribute::remove_attribute(payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
