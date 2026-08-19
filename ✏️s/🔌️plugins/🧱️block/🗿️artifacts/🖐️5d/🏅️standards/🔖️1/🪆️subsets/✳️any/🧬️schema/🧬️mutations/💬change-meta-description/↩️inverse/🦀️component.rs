//! ↩️ Inverse for `ChangeMetaDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangeMetaDescription, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::change_meta_description::mutation::change_meta_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
