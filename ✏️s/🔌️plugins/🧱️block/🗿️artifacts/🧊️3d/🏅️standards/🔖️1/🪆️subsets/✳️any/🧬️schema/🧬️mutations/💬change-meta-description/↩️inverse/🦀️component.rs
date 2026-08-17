//! ↩️ Inverse for `ChangeMetaDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeMetaDescription, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_meta_description::mutation::change_meta_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
