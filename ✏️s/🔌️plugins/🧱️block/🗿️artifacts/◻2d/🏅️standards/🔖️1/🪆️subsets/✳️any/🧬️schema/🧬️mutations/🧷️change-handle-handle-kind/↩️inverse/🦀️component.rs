//! ↩️ Inverse for `ChangeHandleHandleKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeHandleHandleKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handles.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::change_handle_handle_kind::mutation::change_handle_handle_kind(payload.id.clone(), existing.handle_kind.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
