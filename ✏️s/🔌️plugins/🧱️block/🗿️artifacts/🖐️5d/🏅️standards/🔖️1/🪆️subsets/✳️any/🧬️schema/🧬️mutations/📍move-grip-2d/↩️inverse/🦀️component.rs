//! ↩️ Inverse for `MoveGrip2d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::MoveGrip2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::move_grip_2d::mutation::move_grip_2d(payload.id.clone(), existing.angle, existing.radius_2d)], None => Vec::new() }
}
//#endregion 🔖️Inverse
