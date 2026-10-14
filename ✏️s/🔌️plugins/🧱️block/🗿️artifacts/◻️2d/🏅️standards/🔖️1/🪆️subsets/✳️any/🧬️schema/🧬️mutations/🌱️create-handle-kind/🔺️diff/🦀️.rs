//! 🔺️ Diff for `CreateHandleKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::{Block2dDiff, Block2dHandleKindsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::create_handle_kind::CreateHandleKind, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.handle_kinds.iter().any(|item| item.id == payload.handle_kind.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "handle-kind", payload.handle_kind.id), vec![payload.handle_kind.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handle_kinds: Block2dHandleKindsDelta::insertion(semio_s_plugin_block::block_insert_index(base.handle_kinds.len(), payload.index), payload.handle_kind.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
