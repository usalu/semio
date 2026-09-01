//! 🔺️ Diff for `ChangeGripKindColor`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripKindsDelta, Block5dGripKindsPatch, Block5dGripKindsPatchEntry};

//#region 🔖️Diff
pub async fn diff(payload: &super::ChangeGripKindColor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.grip_kinds.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip-kind", payload.id), vec![payload.id.clone()]);
    };
    let replacement = Block5dGripKind { color: payload.new_color.clone(), ..existing.clone() };
    if replacement == *existing {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff {
        grip_kinds: Some(Block5dGripKindsDelta { patched: vec![Block5dGripKindsPatchEntry { id: payload.id.clone(), patch: Block5dGripKindsPatch { replacement: Some(replacement) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
