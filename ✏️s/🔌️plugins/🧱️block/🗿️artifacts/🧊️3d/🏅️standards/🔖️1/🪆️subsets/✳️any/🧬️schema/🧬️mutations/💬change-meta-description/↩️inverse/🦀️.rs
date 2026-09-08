//! ↩️ Inverse for `ChangeMetaDescription`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeMetaDescription, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::change_meta_description::change_meta_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
