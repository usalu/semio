//! ↩️ Inverse for `MoveVortex`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveVortex, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::move_vortex::move_vortex(payload.id.clone(), existing.position, existing.direction)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
