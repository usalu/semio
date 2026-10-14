//! 🔺️ Diff for `MoveHandle`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;
use semio_s_artifact_block_2d_diff::{Block2dHandlesDelta, Block2dHandlesPatchEntry, Block2dHandleTemplatePatch};

//#region 🔖️Diff
pub fn diff(payload: &crate::move_handle::MoveHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    let Some(existing) = base.handles.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle", payload.id), vec![payload.id.clone()]);
    };
    if existing.angle == payload.new_angle && existing.radius == payload.new_radius {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block2dHandleTemplatePatch { angle: Some(payload.new_angle), radius: Some(payload.new_radius), ..Default::default() };
    protocol::MutationOutcome::new(Block2dDiff { handles: Block2dHandlesDelta { modified: vec![Block2dHandlesPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }, ..Default::default() })
}
//#endregion 🔖️Diff
