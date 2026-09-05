//! ↩️ Inverse for `ChangeObjectKindLabel`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeObjectKindLabel, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_label::change_object_kind_label(base.object_kind.label.clone())]
}
//#endregion 🔖️Inverse
