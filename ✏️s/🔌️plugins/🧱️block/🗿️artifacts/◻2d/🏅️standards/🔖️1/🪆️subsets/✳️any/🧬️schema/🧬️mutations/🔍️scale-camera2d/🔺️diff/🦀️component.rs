//! 🔺️ Sparse diff builder for `ScaleCamera2d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockCamera2d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleCamera2d, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    protocol::MutationOutcome::new(Block2dDiff { camera2d: Some(BlockCamera2d { zoom: payload.new_zoom, ..base.camera2d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
