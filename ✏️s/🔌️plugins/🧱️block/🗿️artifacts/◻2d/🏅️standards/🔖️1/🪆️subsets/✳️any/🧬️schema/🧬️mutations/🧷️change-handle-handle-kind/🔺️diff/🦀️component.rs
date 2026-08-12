//! 🔺️ Sparse diff builder for `ChangeHandleHandleKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dHandlesDelta, Block2dHandlesPatch, Block2dHandlesPatchEntry};
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::{Block2dHandleTemplate};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeHandleHandleKind, base: &Block2dSnapshot) -> Block2dDiff {
    let Some(existing) = base.handles.iter().find(|item| item.id == payload.id) else { return Block2dDiff::default(); };
    let replacement = Block2dHandleTemplate { handle_kind: payload.new_handle_kind.clone(), ..existing.clone() };
    Block2dDiff { handles: Some(Block2dHandlesDelta { patched: vec![Block2dHandlesPatchEntry { id: payload.id.clone(), patch: Block2dHandlesPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
