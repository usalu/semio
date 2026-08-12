//! ↩️ Inverse for `AddAuthor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddAuthor, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_author::mutation::remove_author(payload.author.id.clone())]
}
//#endregion 🔖️Inverse
