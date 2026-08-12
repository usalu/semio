//! ↩️ Inverse for `ResizeGrip3d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ResizeGrip3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::resize_grip_3d::mutation::resize_grip_3d(payload.id.clone(), existing.radius_3d)], None => Vec::new() }
}
//#endregion 🔖️Inverse
