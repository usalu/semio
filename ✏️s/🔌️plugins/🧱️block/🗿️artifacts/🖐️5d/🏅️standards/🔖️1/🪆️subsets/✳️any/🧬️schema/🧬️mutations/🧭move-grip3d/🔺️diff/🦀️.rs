//! 🔺️ Diff for `MoveGrip3d`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Block5dGripsDelta, Block5dGripsPatchEntry, Block5dGripTemplatePatch};

//#region 🔖️Diff
pub fn diff(payload: &super::MoveGrip3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.grips.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip", payload.id), vec![payload.id.clone()]);
    };
    if existing.position == payload.new_position && existing.direction == payload.new_direction {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block5dGripTemplatePatch { position: Some(payload.new_position), direction: Some(payload.new_direction), ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { grips: Some(Block5dGripsDelta { patched: vec![Block5dGripsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
