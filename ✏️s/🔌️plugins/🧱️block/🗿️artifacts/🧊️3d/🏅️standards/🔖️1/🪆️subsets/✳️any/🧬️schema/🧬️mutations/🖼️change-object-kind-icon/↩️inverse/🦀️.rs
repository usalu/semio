//! ↩️ Inverse for `ChangeObjectKindIcon`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindIcon, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_icon::change_object_kind_icon(base.object_kind.icon.clone())]
}
//#endregion 🔖️Inverse
