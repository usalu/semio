//! ↩️ Inverse for `CreateHandle` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateHandle, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::delete_handle::mutation::delete_handle(payload.handle.id.clone())]
}
//#endregion 🔖️Inverse
