//! ↩️ Inverse for `ChangeMetaDescription`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangeMetaDescription, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_meta_description::change_meta_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
