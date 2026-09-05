//! ↩️ Inverse for `AddRepresentationTag`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationTag, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_representation_tag::remove_representation_tag(payload.id.clone(), payload.tag.clone())]
}
//#endregion 🔖️Inverse
