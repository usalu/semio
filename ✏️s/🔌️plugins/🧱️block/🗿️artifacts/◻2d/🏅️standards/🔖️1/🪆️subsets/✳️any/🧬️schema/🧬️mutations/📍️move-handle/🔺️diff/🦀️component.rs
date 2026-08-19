//! 🔺️ Sparse diff builder for `MoveHandle` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandlesDelta, Block2dHandlesPatch, Block2dHandlesPatchEntry};
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::{Block2dHandleTemplate};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MoveHandle, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    let Some(existing) = base.handles.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle", payload.id), vec![payload.id.clone()]);
    };
    let replacement = Block2dHandleTemplate { angle: payload.new_angle, radius: payload.new_radius, ..existing.clone() };
    if replacement == *existing {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handles: Some(Block2dHandlesDelta { patched: vec![Block2dHandlesPatchEntry { id: payload.id.clone(), patch: Block2dHandlesPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
