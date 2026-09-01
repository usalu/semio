//! ↩️ Inverse for `ChangeGripKindLabel`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeGripKindLabel, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_grip_kind_label::change_grip_kind_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
