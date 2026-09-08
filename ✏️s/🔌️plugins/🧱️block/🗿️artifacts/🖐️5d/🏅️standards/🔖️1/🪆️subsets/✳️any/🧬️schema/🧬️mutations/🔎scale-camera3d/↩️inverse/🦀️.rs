//! ↩️ Inverse for `ScaleCamera3d`.

use crate::Block5dSnapshot;
use crate::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ScaleCamera3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::scale_camera3d::scale_camera3d(base.camera3d.zoom)]
}
//#endregion 🔖️Inverse
