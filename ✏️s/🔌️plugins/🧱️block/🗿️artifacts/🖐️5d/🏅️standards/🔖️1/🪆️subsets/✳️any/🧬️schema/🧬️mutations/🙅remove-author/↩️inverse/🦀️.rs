//! ↩️ Inverse for `RemoveAuthor`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::RemoveAuthor, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.authors.iter().find(|author| author.id == payload.id) {
        Some(existing) => vec![super::super::add_author::add_author(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
