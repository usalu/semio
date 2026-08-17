//! ↩️ Inverse for `MoveCamera2d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::MoveCamera2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::move_camera2d::mutation::move_camera2d(base.camera2d.x, base.camera2d.y)]
}
//#endregion 🔖️Inverse
