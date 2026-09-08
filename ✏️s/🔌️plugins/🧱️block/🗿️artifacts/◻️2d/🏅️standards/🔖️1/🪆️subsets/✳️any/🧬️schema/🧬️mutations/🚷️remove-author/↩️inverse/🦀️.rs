//! ↩️ Inverse for `RemoveAuthor`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAuthor, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.authors.iter().find(|author| author.id == payload.id) {
        Some(existing) => vec![super::super::add_author::add_author(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
