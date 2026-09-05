//! ↩️ Inverse for `AddAuthor`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAuthor, _base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::remove_author::remove_author(payload.author.id.clone())]
}
//#endregion 🔖️Inverse
