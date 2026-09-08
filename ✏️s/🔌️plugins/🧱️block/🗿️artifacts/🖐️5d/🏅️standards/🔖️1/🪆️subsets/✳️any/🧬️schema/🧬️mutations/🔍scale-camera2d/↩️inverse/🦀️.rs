//! ↩️ Inverse for `ScaleCamera2d`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ScaleCamera2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::scale_camera2d::scale_camera2d(base.camera2d.zoom)]
}
//#endregion 🔖️Inverse
