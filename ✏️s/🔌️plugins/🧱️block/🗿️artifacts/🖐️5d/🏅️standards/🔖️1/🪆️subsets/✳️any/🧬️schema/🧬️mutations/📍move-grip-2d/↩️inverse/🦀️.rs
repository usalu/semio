//! ↩️ Inverse for `MoveGrip2d`.

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::MoveGrip2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::move_grip_2d::mutation::move_grip_2d(payload.id.clone(), existing.angle, existing.radius_2d)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
