//! ↩️ Inverse for `AddAuthor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddAuthor, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_author::mutation::remove_author(payload.author.id.clone())]
}
//#endregion 🔖️Inverse
