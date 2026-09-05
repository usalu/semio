//! ↩️ Inverse for `MoveCamera2d`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::MoveCamera2d, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![crate::artifacts::block2d::mutations::move_camera2d::move_camera2d(base.camera2d.x, base.camera2d.y)]
}
//#endregion 🔖️Inverse
