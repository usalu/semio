//! ↩️ Inverse for `CreateGrip`.

use crate::Block5dSnapshot;
use crate::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateGrip, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_grip::delete_grip(payload.grip.id.clone())]
}
//#endregion 🔖️Inverse
