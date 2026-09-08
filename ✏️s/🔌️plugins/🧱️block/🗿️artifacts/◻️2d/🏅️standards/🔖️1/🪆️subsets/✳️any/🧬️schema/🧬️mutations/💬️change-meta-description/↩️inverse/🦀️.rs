//! ↩️ Inverse for `ChangeMetaDescription`.

use crate::Block2dSnapshot;
use crate::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeMetaDescription, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_meta_description::change_meta_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
