//! ↩️ Inverse for `MoveCamera2d`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::MoveCamera2d, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::move_camera2d::mutation::move_camera2d(base.camera2d.x, base.camera2d.y)]
}
//#endregion 🔖️Inverse
