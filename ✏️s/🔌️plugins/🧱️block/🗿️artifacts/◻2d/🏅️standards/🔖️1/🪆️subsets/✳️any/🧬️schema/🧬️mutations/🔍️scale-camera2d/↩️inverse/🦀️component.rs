//! ↩️ Inverse for `ScaleCamera2d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ScaleCamera2d, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::scale_camera2d::mutation::scale_camera2d(base.camera2d.zoom)]
}
//#endregion 🔖️Inverse
