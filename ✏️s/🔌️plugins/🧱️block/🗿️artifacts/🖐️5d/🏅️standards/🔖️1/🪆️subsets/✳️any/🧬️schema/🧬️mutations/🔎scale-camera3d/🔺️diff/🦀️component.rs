//! 🔺️ Sparse diff builder for `ScaleCamera3d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockCamera3d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleCamera3d, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { camera3d: Some(BlockCamera3d { zoom: payload.new_zoom, ..base.camera3d.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
