//! ↩️ Inverse for `MoveCamera3d`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::MoveCamera3d, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::move_camera3d::move_camera3d(base.camera3d.position, base.camera3d.target)]
}
//#endregion 🔖️Inverse
