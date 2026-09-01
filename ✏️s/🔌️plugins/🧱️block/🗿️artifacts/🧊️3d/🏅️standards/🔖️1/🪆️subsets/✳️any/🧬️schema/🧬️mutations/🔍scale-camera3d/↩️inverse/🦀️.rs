//! ↩️ Inverse for `ScaleCamera3d`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ScaleCamera3d, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::scale_camera3d::mutation::scale_camera3d(base.camera3d.zoom)]
}
//#endregion 🔖️Inverse
