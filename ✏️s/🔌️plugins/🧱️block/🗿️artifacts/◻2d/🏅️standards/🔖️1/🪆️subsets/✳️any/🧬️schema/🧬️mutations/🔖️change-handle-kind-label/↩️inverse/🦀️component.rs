//! ↩️ Inverse for `ChangeHandleKindLabel` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeHandleKindLabel, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_handle_kind_label::mutation::change_handle_kind_label(payload.id.clone(), existing.label.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
