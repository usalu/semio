//! ↩️ Inverse for `ChangeHandleKindDefaultWireKind`.

use crate::artifacts::block2d::{Block2dHandleKind, Block2dSnapshot};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeHandleKindDefaultWireKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    match base.handle_kinds.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_handle_kind_default_wire_kind::change_handle_kind_default_wire_kind(payload.id.clone(), existing.default_wire_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
