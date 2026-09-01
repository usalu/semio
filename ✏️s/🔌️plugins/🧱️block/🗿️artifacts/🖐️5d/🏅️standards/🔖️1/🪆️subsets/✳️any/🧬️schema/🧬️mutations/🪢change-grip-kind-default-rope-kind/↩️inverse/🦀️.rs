//! ↩️ Inverse for `ChangeGripKindDefaultRopeKind`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeGripKindDefaultRopeKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_grip_kind_default_rope_kind::change_grip_kind_default_rope_kind(payload.id.clone(), existing.default_rope_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
