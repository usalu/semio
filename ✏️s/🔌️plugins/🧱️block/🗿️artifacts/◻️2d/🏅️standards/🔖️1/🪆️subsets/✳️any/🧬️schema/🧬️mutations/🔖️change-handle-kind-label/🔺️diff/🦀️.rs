//! 🔺️ Diff for `ChangeHandleKindLabel`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;
use semio_s_artifact_block_2d_diff::{Block2dHandleKindsDelta, Block2dHandleKindsPatchEntry, Block2dHandleKindPatch};

//#region 🔖️Diff
pub fn diff(payload: &crate::change_handle_kind_label::ChangeHandleKindLabel, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    let Some(existing) = base.handle_kinds.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle-kind", payload.id), vec![payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block2dHandleKindPatch { label: Some(payload.new_label.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Block2dDiff { handle_kinds: Block2dHandleKindsDelta { modified: vec![Block2dHandleKindsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, ..Default::default() })
}
//#endregion 🔖️Diff
