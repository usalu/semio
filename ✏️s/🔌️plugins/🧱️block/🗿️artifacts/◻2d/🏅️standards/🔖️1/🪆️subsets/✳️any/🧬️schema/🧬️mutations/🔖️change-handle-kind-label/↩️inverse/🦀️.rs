//! ↩️ Inverse for `ChangeHandleKindLabel`.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dSnapshot};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeHandleKindLabel, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_handle_kind_label::change_handle_kind_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
