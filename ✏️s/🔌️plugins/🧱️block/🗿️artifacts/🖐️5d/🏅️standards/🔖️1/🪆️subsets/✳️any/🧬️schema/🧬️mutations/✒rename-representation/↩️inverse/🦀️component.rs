//! ↩️ Inverse for `RenameRepresentation` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RenameRepresentation, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::rename_representation::mutation::rename_representation(payload.id.clone(), existing.name.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
