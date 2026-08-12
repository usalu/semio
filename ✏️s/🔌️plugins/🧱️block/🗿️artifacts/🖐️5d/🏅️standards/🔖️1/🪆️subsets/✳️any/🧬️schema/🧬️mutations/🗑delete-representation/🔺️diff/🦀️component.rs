//! 🔺️ Sparse diff builder for `DeleteRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dRepresentationsDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteRepresentation, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { representations: Some(Block5dRepresentationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
