//! ↩️ Inverse for `RemoveAuthor`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAuthor, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.authors.iter().find(|author| author.id == payload.id) {
        Some(existing) => vec![super::super::add_author::add_author(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
