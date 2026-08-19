//! ↩️ Inverse for `RenameHandleKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RenameHandleKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::rename_handle_kind::mutation::rename_handle_kind(payload.id.clone(), existing.name.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
