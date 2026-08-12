//! 🔺️ Sparse diff builder for `AddAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dAttributesDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddAttribute, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { attributes: Some(Block5dAttributesDelta { added: vec![payload.attribute.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
