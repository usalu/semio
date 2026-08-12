//! ↩️ Inverse for `DeleteHandle` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteHandle, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handles.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::create_handle::mutation::create_handle(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
