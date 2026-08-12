//! 🔺️ Sparse diff builder for `ScaleCamera3d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockCamera3d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleCamera3d, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { camera3d: Some(BlockCamera3d { zoom: payload.new_zoom, ..base.camera3d.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
