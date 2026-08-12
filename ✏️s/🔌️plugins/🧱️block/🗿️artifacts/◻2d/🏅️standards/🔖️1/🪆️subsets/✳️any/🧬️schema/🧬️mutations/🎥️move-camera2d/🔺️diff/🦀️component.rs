//! 🔺️ Sparse diff builder for `MoveCamera2d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockCamera2d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveCamera2d, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { camera2d: Some(BlockCamera2d { x: payload.new_x, y: payload.new_y, ..base.camera2d.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
