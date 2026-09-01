//! ↩️ Inverse for `CreateGripKind`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateGripKind, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_grip_kind::delete_grip_kind(payload.grip_kind.id.clone())]
}
//#endregion 🔖️Inverse
