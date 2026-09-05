//! ↩️ Inverse for `ResizeGrip3d`.

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ResizeGrip3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    match base.grips.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::resize_grip_3d::resize_grip_3d(payload.id.clone(), existing.radius_3d)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
