//! ↩️ Inverse for `ChangeGripGripKind`.

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeGripGripKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_grip_grip_kind::change_grip_grip_kind(payload.id.clone(), existing.grip_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
