//! ↩️ Inverse for `MoveGrip2d`.

use crate::Block5dSnapshot;
use crate::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveGrip2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::move_grip_2d::move_grip_2d(payload.id.clone(), existing.angle, existing.radius_2d)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
