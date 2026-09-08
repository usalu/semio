//! ↩️ Inverse for `AddAuthor`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAuthor, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::remove_author::remove_author(payload.author.id.clone())]
}
//#endregion 🔖️Inverse
