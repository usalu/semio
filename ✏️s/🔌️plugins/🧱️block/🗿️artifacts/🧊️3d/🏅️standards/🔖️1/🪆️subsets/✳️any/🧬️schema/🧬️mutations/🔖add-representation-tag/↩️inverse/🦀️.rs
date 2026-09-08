//! ↩️ Inverse for `AddRepresentationTag`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationTag, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_representation_tag::remove_representation_tag(payload.id.clone(), payload.tag.clone())]
}
//#endregion 🔖️Inverse
