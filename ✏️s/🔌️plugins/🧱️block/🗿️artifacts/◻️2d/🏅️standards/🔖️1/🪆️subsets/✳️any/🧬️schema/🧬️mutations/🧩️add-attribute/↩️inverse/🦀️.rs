//! ↩️ Inverse for `AddAttribute`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAttribute, _base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::remove_attribute::remove_attribute(payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
