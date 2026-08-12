//! 🔺️ Sparse diff builder for `RemoveAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dAttributesDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveAttribute, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { attributes: Some(Block5dAttributesDelta { removed: vec![payload.key.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
