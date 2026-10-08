//! 🔺️ Diff for `CreateHandle`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::{Block2dDiff, Block2dHandlesDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.handles.iter().any(|item| item.id == payload.handle.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "handle", payload.handle.id), vec![payload.handle.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handles: Block2dHandlesDelta::insertion(semio_s_plugin_block::block_insert_index(base.handles.len(), payload.index), payload.handle.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
