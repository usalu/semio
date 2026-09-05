//! ↩️ Inverse for `RenameRepresentation`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameRepresentation, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::rename_representation::rename_representation(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
