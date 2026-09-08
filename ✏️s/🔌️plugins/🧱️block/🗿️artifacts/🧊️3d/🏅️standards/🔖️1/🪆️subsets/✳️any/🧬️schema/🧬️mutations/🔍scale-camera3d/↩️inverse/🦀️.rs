//! ↩️ Inverse for `ScaleCamera3d`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ScaleCamera3d, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::scale_camera3d::scale_camera3d(base.camera3d.zoom)]
}
//#endregion 🔖️Inverse
