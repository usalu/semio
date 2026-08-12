//! 🔺️ Sparse diff builder for `CreateVortex` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dVorticesDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateVortex, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { vortices: Some(Block3dVorticesDelta { added: vec![payload.vortex.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
