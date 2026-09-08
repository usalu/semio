//! ↩️ Inverse for `AddRepresentationAttribute`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationAttribute, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_representation_attribute::remove_representation_attribute(payload.id.clone(), payload.attribute.key.clone())]
}
//#endregion 🔖️Inverse
