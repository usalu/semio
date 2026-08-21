//! ↩️ Inverse for `RenameRepresentation` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RenameRepresentation, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::rename_representation::mutation::rename_representation(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
