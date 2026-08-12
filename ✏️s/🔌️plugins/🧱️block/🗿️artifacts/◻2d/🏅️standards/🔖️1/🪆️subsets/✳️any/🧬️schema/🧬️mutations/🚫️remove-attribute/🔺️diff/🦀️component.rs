//! 🔺️ Sparse diff builder for `RemoveAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dAttributesDelta};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveAttribute, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { attributes: Some(Block2dAttributesDelta { removed: vec![payload.key.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
