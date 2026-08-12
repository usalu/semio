//! ↩️ Inverse for `ChangeGripGripKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeGripGripKind, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_grip_grip_kind::mutation::change_grip_grip_kind(payload.id.clone(), existing.grip_kind.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
