//! ↩️ Inverse for `AddAuthor`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::AddAuthor, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_author::remove_author(payload.author.id.clone())]
}
//#endregion 🔖️Inverse
