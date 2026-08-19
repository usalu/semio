//! 🔺️ Sparse diff builder for `DeleteHandle` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandlesDelta};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.handles.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handles: Some(Block2dHandlesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
