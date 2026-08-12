//! ↩️ Inverse for `ChangeObjectKindIcon` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeObjectKindIcon, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_icon::mutation::change_object_kind_icon(base.object_kind.icon.clone())]
}
//#endregion 🔖️Inverse
