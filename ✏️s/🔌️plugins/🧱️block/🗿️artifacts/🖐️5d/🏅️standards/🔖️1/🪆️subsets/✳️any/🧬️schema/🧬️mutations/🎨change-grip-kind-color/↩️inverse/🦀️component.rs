//! ↩️ Inverse for `ChangeGripKindColor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeGripKindColor, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grip_kinds.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_grip_kind_color::mutation::change_grip_kind_color(payload.id.clone(), existing.color.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
