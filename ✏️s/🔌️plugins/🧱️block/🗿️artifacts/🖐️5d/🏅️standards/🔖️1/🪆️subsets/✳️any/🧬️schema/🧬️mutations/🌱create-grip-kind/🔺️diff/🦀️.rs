//! 🔺️ Diff for `CreateGripKind`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::{Block5dDiff, Block5dGripKindsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::create_grip_kind::CreateGripKind, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.grip_kinds.iter().any(|item| item.id == payload.grip_kind.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "grip-kind", payload.grip_kind.id), vec![payload.grip_kind.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grip_kinds: Block5dGripKindsDelta::insertion(semio_s_plugin_block::block_insert_index(base.grip_kinds.len(), payload.index), payload.grip_kind.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
