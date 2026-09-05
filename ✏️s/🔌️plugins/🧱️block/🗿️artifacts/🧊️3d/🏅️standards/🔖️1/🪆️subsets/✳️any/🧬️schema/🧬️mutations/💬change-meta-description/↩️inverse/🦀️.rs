//! ↩️ Inverse for `ChangeMetaDescription`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeMetaDescription, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_meta_description::change_meta_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
