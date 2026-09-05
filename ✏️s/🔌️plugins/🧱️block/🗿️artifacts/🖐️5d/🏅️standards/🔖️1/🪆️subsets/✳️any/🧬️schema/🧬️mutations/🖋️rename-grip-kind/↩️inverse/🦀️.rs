//! ↩️ Inverse for `RenameGripKind`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameGripKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::rename_grip_kind::rename_grip_kind(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
