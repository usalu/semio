//! 🔺️ Diff for `ChangeGripKindColor`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;
use semio_s_artifact_block_5d_diff::{Block5dGripKindsDelta, Block5dGripKindsPatchEntry, Block5dGripKindPatch};

//#region 🔖️Diff
pub fn diff(payload: &crate::change_grip_kind_color::ChangeGripKindColor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.grip_kinds.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip-kind", payload.id), vec![payload.id.clone()]);
    };
    if existing.color == payload.new_color {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block5dGripKindPatch { color: Some(payload.new_color.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { grip_kinds: Block5dGripKindsDelta { modified: vec![Block5dGripKindsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, ..Default::default() })
}
//#endregion 🔖️Diff
