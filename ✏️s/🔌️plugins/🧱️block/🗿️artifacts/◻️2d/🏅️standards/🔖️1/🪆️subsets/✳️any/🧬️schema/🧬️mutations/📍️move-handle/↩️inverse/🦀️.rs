//! ↩️ Inverse for `MoveHandle`.

use crate::artifacts::block2d::{Block2dHandleTemplate, Block2dSnapshot};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::MoveHandle, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handles.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::move_handle::move_handle(payload.id.clone(), existing.angle, existing.radius)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
