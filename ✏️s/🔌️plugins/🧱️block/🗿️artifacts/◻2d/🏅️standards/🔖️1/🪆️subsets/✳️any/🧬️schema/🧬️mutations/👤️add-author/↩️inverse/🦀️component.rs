//! ↩️ Inverse for `AddAuthor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddAuthor, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::remove_author::mutation::remove_author(payload.author.id.clone())]
}
//#endregion 🔖️Inverse
