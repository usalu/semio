//! ↩️ Inverse for `DeleteRepresentation`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteRepresentation, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.representations.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_representation::create_representation(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
