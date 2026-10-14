//! 🔺️ Diff for `DeleteHandle`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::{Block2dDiff, Block2dHandlesDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_handle::DeleteHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.handles.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handles: Block2dHandlesDelta::removal(&base.handles, base.handles.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
