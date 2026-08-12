//! 🔺️ Sparse diff builder for `RemoveCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dCompatibilityDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveCompatibilityRule, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { compatibility: Some(Block3dCompatibilityDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
