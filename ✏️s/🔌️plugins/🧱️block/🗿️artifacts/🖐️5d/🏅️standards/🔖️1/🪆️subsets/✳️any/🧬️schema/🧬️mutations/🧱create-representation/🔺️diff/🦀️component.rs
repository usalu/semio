//! 🔺️ Sparse diff builder for `CreateRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dRepresentationsDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateRepresentation, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { representations: Some(Block5dRepresentationsDelta { added: vec![payload.representation.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
