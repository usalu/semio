//! 🔺️ Sparse diff builder for `CreateGrip` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dGripsDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateGrip, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { grips: Some(Block5dGripsDelta { added: vec![payload.grip.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
