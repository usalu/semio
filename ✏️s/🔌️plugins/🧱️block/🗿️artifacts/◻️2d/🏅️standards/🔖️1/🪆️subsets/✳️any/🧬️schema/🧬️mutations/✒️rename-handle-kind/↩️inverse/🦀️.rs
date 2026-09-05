//! ↩️ Inverse for `RenameHandleKind`.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dSnapshot};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameHandleKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::rename_handle_kind::rename_handle_kind(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
