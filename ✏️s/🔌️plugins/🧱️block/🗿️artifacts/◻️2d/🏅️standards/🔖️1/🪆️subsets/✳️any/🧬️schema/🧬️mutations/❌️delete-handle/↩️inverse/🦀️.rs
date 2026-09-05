//! ↩️ Inverse for `DeleteHandle`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteHandle, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handles.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_handle::create_handle(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
