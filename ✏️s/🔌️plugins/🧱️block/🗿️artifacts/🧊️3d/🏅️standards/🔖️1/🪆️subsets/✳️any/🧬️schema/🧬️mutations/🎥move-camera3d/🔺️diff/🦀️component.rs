//! 🔺️ Sparse diff builder for `MoveCamera3d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockCamera3d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveCamera3d, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { camera3d: Some(BlockCamera3d { position: payload.new_position, target: payload.new_target, ..base.camera3d.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
