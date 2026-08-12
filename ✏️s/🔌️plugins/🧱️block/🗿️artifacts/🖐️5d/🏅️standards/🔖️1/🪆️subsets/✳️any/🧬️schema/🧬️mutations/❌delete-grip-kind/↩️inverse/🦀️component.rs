//! ↩️ Inverse for `DeleteGripKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteGripKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::create_grip_kind::mutation::create_grip_kind(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
