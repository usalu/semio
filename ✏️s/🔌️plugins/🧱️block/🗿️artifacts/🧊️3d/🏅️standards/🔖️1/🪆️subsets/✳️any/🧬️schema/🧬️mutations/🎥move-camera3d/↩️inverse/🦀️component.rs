//! ↩️ Inverse for `MoveCamera3d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::MoveCamera3d, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::move_camera3d::mutation::move_camera3d(base.camera3d.position, base.camera3d.target)]
}
//#endregion 🔖️Inverse
