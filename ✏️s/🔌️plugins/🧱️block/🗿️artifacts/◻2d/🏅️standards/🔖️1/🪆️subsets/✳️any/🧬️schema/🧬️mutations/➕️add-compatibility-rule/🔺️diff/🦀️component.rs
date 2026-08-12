//! 🔺️ Sparse diff builder for `AddCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dCompatibilityDelta};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddCompatibilityRule, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { compatibility: Some(Block2dCompatibilityDelta { added: vec![payload.rule.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
