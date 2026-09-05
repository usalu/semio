//! ↩️ Inverse for `ChangeObjectKindDescription`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindDescription, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_description::change_object_kind_description(base.object_kind.description.clone())]
}
//#endregion 🔖️Inverse
