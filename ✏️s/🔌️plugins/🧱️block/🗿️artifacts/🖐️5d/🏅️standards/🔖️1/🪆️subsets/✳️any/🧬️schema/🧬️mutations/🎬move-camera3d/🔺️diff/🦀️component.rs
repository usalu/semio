//! 🔺️ Sparse diff builder for `MoveCamera3d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockCamera3d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveCamera3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    protocol::MutationOutcome::new(Block5dDiff { camera3d: Some(BlockCamera3d { position: payload.new_position, target: payload.new_target, ..base.camera3d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
