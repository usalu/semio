//! 🔺️ Sparse diff builder for `DeleteVortex` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dVorticesDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteVortex, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { vortices: Some(Block3dVorticesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
