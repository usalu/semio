//! ↩️ Inverse for `ScaleCamera3d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ScaleCamera3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::scale_camera3d::mutation::scale_camera3d(base.camera3d.zoom)]
}
//#endregion 🔖️Inverse
