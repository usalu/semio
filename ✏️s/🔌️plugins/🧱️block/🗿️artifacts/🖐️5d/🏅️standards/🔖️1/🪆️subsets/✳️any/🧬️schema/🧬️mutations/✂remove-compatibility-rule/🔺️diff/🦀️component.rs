//! 🔺️ Sparse diff builder for `RemoveCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dCompatibilityDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveCompatibilityRule, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { compatibility: Some(Block5dCompatibilityDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
