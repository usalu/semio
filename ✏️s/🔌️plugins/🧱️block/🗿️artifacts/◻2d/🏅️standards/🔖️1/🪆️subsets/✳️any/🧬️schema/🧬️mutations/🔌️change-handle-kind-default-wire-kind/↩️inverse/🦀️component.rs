//! ↩️ Inverse for `ChangeHandleKindDefaultWireKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeHandleKindDefaultWireKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_handle_kind_default_wire_kind::mutation::change_handle_kind_default_wire_kind(payload.id.clone(), existing.default_wire_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
