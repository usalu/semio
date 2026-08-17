//! ↩️ Inverse for `ScaleCamera2d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ScaleCamera2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::scale_camera2d::mutation::scale_camera2d(base.camera2d.zoom)]
}
//#endregion 🔖️Inverse
