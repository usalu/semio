//! ↩️ Inverse for `MoveCamera3d`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::MoveCamera3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::move_camera3d::mutation::move_camera3d(base.camera3d.position, base.camera3d.target)]
}
//#endregion 🔖️Inverse
