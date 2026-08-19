//! ↩️ Inverse for `RemoveAuthor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveAuthor, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.authors.iter().find(|author| author.id == payload.id) { Some(existing) => vec![super::super::add_author::mutation::add_author(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
