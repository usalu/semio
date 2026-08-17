//! ↩️ Inverse for `MoveCamera3d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::MoveCamera3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::move_camera3d::mutation::move_camera3d(base.camera3d.position, base.camera3d.target)]
}
//#endregion 🔖️Inverse
