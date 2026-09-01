//! ↩️ Inverse for `CreateGrip`.

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateGrip, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_grip::delete_grip(payload.grip.id.clone())]
}
//#endregion 🔖️Inverse
