//! ↩️ Inverse for `ChangeObjectKindDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeObjectKindDescription, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_object_kind_description::mutation::change_object_kind_description(base.object_kind.description.clone())]
}
//#endregion 🔖️Inverse
