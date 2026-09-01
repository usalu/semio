//! ↩️ Inverse for `ResizeVortex`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ResizeVortex, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::resize_vortex::resize_vortex(payload.id.clone(), existing.radius)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
