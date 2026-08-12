//! ↩️ Inverse for `RemoveAuthor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RemoveAuthor, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.authors.iter().find(|author| author.id == payload.id) { Some(existing) => vec![super::super::add_author::mutation::add_author(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
