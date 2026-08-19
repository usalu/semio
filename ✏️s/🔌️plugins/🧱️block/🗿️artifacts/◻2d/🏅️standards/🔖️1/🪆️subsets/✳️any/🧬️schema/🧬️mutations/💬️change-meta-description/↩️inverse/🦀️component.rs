//! ↩️ Inverse for `ChangeMetaDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangeMetaDescription, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_meta_description::mutation::change_meta_description(base.meta.description.clone())]
}
//#endregion 🔖️Inverse
