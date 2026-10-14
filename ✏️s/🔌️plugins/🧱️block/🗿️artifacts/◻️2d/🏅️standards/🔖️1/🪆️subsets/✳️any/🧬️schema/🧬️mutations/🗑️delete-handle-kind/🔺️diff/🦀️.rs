//! 🔺️ Diff for `DeleteHandleKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::{Block2dDiff, Block2dHandleKindsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_handle_kind::DeleteHandleKind, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.handle_kinds.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle-kind", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handle_kinds: Block2dHandleKindsDelta::removal(&base.handle_kinds, base.handle_kinds.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
