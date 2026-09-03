//! ↩️ Inverse for `ScaleCamera2d`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ScaleCamera2d, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::scale_camera2d::mutation::scale_camera2d(base.camera2d.zoom)]
}
//#endregion 🔖️Inverse
