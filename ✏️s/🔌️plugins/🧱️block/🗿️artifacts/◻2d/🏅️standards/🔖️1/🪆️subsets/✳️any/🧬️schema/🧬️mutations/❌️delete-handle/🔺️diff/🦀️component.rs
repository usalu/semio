//! 🔺️ Sparse diff builder for `DeleteHandle` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandlesDelta};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteHandle, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { handles: Some(Block2dHandlesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
