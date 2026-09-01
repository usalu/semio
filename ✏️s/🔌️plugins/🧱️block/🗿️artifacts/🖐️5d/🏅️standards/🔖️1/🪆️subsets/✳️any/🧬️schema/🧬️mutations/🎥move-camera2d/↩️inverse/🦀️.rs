//! ↩️ Inverse for `MoveCamera2d`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::MoveCamera2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::move_camera2d::mutation::move_camera2d(base.camera2d.x, base.camera2d.y)]
}
//#endregion 🔖️Inverse
