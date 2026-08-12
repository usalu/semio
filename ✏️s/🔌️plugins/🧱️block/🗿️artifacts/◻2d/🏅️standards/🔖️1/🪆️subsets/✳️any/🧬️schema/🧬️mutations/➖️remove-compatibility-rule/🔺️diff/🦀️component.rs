//! 🔺️ Sparse diff builder for `RemoveCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dCompatibilityDelta};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveCompatibilityRule, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { compatibility: Some(Block2dCompatibilityDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
