//! 🔺️ Diff for `CreateHandle`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::{Block2dDiff, Block2dHandlesDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::create_handle::CreateHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.handles.iter().any(|item| item.id == payload.handle.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "handle", payload.handle.id), vec![payload.handle.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handles: Block2dHandlesDelta::insertion(semio_s_plugin_block::block_insert_index(base.handles.len(), payload.index), payload.handle.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
