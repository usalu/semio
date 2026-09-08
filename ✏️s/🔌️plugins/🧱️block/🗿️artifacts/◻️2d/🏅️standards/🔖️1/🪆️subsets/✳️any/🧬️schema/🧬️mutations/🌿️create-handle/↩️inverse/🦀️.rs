//! ↩️ Inverse for `CreateHandle`.

use crate::Block2dSnapshot;
use crate::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateHandle, _base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::delete_handle::delete_handle(payload.handle.id.clone())]
}
//#endregion 🔖️Inverse
