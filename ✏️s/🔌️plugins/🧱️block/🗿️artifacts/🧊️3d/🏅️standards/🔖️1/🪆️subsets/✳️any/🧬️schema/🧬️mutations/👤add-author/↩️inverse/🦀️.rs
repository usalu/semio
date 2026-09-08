//! ↩️ Inverse for `AddAuthor`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAuthor, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_author::remove_author(payload.author.id.clone())]
}
//#endregion 🔖️Inverse
