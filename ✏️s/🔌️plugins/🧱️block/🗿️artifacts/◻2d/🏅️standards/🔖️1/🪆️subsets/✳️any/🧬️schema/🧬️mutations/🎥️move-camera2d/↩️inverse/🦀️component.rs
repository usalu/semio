//! ↩️ Inverse for `MoveCamera2d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::MoveCamera2d, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::move_camera2d::mutation::move_camera2d(base.camera2d.x, base.camera2d.y)]
}
//#endregion 🔖️Inverse
