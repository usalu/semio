//! ↩️ Inverse for `CreateHandleKind`.

use crate::Block2dSnapshot;
use crate::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateHandleKind, _base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::delete_handle_kind::delete_handle_kind(payload.handle_kind.id.clone())]
}
//#endregion 🔖️Inverse
