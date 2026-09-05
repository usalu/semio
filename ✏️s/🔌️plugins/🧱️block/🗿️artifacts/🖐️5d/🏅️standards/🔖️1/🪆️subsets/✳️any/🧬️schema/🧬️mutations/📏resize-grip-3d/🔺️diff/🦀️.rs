//! 🔺️ Diff for `ResizeGrip3d`.

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripsDelta, Block5dGripsPatch, Block5dGripsPatchEntry};

//#region 🔖️Diff
pub fn diff(payload: &super::ResizeGrip3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.grips.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip", payload.id), vec![payload.id.clone()]);
    };
    let replacement = Block5dGripTemplate { radius_3d: payload.new_radius_3d, ..existing.clone() };
    if replacement == *existing {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff {
        grips: Some(Block5dGripsDelta { patched: vec![Block5dGripsPatchEntry { id: payload.id.clone(), patch: Block5dGripsPatch { replacement: Some(replacement) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
