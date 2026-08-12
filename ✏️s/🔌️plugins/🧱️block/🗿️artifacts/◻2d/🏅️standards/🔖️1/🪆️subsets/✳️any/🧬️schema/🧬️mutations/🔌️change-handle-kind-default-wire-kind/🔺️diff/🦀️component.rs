//! 🔺️ Sparse diff builder for `ChangeHandleKindDefaultWireKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandleKindsDelta, Block2dHandleKindsPatch, Block2dHandleKindsPatchEntry};
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::{Block2dHandleKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeHandleKindDefaultWireKind, base: &Block2dSnapshot) -> Block2dDiff {
    let Some(existing) = base.handle_kinds.iter().find(|item| item.id == payload.id) else { return Block2dDiff::default(); };
    let replacement = Block2dHandleKind { default_wire_kind: payload.new_default_wire_kind.clone(), ..existing.clone() };
    Block2dDiff { handle_kinds: Some(Block2dHandleKindsDelta { patched: vec![Block2dHandleKindsPatchEntry { id: payload.id.clone(), patch: Block2dHandleKindsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
