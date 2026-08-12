//! ↩️ Inverse for `ChangeHandleKindColor` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeHandleKindColor, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_handle_kind_color::mutation::change_handle_kind_color(payload.id.clone(), existing.color.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
