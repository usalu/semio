//! 🔺️ Sparse diff builder for `ScaleCamera2d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockCamera2d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleCamera2d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    protocol::MutationOutcome::new(Block5dDiff { camera2d: Some(BlockCamera2d { zoom: payload.new_zoom, ..base.camera2d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
