//! ↩️ Inverse for `CreateGripKind`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateGripKind, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_grip_kind::delete_grip_kind(payload.grip_kind.id.clone())]
}
//#endregion 🔖️Inverse
