//! ↩️ Inverse for `DeleteHandleKind`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::DeleteHandleKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_handle_kind::create_handle_kind(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
