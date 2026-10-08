//! 🔺️ Diff for `RenameGripKind`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Block5dGripKindsDelta, Block5dGripKindsPatchEntry, Block5dGripKindPatch};

//#region 🔖️Diff
pub fn diff(payload: &super::RenameGripKind, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.grip_kinds.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip-kind", payload.id), vec![payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block5dGripKindPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { grip_kinds: Some(Block5dGripKindsDelta { patched: vec![Block5dGripKindsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
