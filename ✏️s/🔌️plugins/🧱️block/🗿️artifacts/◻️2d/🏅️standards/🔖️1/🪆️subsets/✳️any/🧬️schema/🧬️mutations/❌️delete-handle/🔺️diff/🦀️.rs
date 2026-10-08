//! 🔺️ Diff for `DeleteHandle`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::{Block2dDiff, Block2dHandlesDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.handles.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handles: Block2dHandlesDelta::removal(&base.handles, base.handles.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
