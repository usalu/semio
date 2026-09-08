//! ↩️ Inverse for `MoveCamera2d`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::MoveCamera2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::move_camera2d::move_camera2d(base.camera2d.x, base.camera2d.y)]
}
//#endregion 🔖️Inverse
