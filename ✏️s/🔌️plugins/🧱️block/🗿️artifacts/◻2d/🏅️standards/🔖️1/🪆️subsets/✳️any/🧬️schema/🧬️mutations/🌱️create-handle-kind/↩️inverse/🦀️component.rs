//! ↩️ Inverse for `CreateHandleKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateHandleKind, _base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::delete_handle_kind::mutation::delete_handle_kind(payload.handle_kind.id.clone())]
}
//#endregion 🔖️Inverse
