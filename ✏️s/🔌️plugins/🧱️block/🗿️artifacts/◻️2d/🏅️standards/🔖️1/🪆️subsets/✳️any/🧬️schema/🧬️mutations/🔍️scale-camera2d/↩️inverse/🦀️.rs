//! ↩️ Inverse for `ScaleCamera2d`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ScaleCamera2d, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![crate::artifacts::block2d::mutations::scale_camera2d::scale_camera2d(base.camera2d.zoom)]
}
//#endregion 🔖️Inverse
