//! ↩️ Inverse for `DeleteGripKind`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteGripKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_grip_kind::create_grip_kind(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
