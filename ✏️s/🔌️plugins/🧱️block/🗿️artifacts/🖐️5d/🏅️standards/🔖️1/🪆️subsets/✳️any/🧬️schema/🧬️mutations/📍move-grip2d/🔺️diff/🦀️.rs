//! 🔺️ Diff for `MoveGrip2d`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Block5dGripsDelta, Block5dGripsPatchEntry, Block5dGripTemplatePatch};

//#region 🔖️Diff
pub fn diff(payload: &super::MoveGrip2d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.grips.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip", payload.id), vec![payload.id.clone()]);
    };
    if existing.angle == payload.new_angle && existing.radius_2d == payload.new_radius_2d {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block5dGripTemplatePatch { angle: Some(payload.new_angle), radius_2d: Some(payload.new_radius_2d), ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { grips: Block5dGripsDelta { modified: vec![Block5dGripsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, ..Default::default() })
}
//#endregion 🔖️Diff
