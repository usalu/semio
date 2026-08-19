//! ↩️ Inverse for `MoveHandle` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::MoveHandle, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handles.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::move_handle::mutation::move_handle(payload.id.clone(), existing.angle, existing.radius)], None => Vec::new() }
}
//#endregion 🔖️Inverse
