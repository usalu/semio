//! ↩️ Inverse for `AddAttribute`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::AddAttribute, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_attribute::remove_attribute(payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
