//! ↩️ Inverse for `RemoveAuthor`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAuthor, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.authors.iter().find(|author| author.id == payload.id) {
        Some(existing) => vec![super::super::add_author::add_author(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
