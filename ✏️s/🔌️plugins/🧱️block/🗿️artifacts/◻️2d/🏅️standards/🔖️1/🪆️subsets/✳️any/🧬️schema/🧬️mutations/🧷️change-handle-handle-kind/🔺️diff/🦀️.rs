//! 🔺️ Diff for `ChangeHandleHandleKind`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Block2dHandlesDelta, Block2dHandlesPatchEntry, Block2dHandleTemplatePatch};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeHandleHandleKind, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    let Some(existing) = base.handles.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "handle", payload.id), vec![payload.id.clone()]);
    };
    if existing.handle_kind == payload.new_handle_kind {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block2dHandleTemplatePatch { handle_kind: Some(payload.new_handle_kind.clone()), ..Default::default() };
    protocol::MutationOutcome::new(Block2dDiff { handles: Some(Block2dHandlesDelta { patched: vec![Block2dHandlesPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
