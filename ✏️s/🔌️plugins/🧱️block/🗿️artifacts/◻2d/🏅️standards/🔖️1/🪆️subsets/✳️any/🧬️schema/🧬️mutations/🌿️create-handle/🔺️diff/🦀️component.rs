//! 🔺️ Sparse diff builder for `CreateHandle` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandlesDelta};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateHandle, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { handles: Some(Block2dHandlesDelta { added: vec![payload.handle.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
