//! ↩️ Inverse for `MoveGrip3d`.

use crate::Block5dSnapshot;
use crate::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveGrip3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::move_grip_3d::move_grip_3d(payload.id.clone(), existing.position, existing.direction)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
