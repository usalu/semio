//! ↩️ Inverse for `ChangeHandleHandleKind`.

use crate::artifacts::block2d::{Block2dHandleTemplate, Block2dSnapshot};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeHandleHandleKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handles.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_handle_handle_kind::change_handle_handle_kind(payload.id.clone(), existing.handle_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
