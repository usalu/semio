//! ↩️ Inverse for `ChangeGripKindColor`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeGripKindColor, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_grip_kind_color::change_grip_kind_color(payload.id.clone(), existing.color.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
