//! 🔺️ Sparse diff builder for `MoveCamera2d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockCamera2d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveCamera2d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    protocol::MutationOutcome::new(Block5dDiff { camera2d: Some(BlockCamera2d { x: payload.new_x, y: payload.new_y, ..base.camera2d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
