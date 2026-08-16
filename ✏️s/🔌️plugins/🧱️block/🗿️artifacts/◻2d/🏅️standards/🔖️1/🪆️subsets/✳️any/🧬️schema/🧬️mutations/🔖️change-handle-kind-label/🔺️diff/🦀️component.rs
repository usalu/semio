//! 🔺️ Sparse diff builder for `ChangeHandleKindLabel` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandleKindsDelta, Block2dHandleKindsPatch, Block2dHandleKindsPatchEntry};
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::{Block2dHandleKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeHandleKindLabel, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    let Some(existing) = base.handle_kinds.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle-kind", payload.id), vec![payload.id.clone()]);
    };
    let replacement = Block2dHandleKind { label: payload.new_label.clone(), ..existing.clone() };
    if replacement == *existing {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block2dDiff { handle_kinds: Some(Block2dHandleKindsDelta { patched: vec![Block2dHandleKindsPatchEntry { id: payload.id.clone(), patch: Block2dHandleKindsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
