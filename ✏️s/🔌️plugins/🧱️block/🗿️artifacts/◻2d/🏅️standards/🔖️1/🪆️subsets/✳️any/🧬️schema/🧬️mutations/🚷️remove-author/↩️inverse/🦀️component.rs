//! ↩️ Inverse for `RemoveAuthor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RemoveAuthor, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.authors.iter().find(|author| author.id == payload.id) { Some(existing) => vec![super::super::add_author::mutation::add_author(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
