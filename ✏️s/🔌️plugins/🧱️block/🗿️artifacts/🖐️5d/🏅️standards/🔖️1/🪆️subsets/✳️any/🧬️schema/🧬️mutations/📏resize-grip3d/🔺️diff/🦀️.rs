//! 🔺️ Diff for `ResizeGrip3d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;
use semio_s_artifact_block_5d_diff::{Block5dGripsDelta, Block5dGripsPatchEntry, Block5dGripTemplatePatch};

//#region 🔖️Diff
pub fn diff(payload: &crate::resize_grip_3d::ResizeGrip3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let Some(existing) = base.grips.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip", payload.id), vec![payload.id.clone()]);
    };
    if existing.radius_3d == payload.new_radius_3d {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block5dGripTemplatePatch { radius_3d: Some(payload.new_radius_3d), ..Default::default() };
    protocol::MutationOutcome::new(Block5dDiff { grips: Block5dGripsDelta { modified: vec![Block5dGripsPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, ..Default::default() })
}
//#endregion 🔖️Diff
