//! 🔺️ Sparse diff builder for `ChangeGripKindDefaultRopeKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dGripKindsDelta, Block5dGripKindsPatch, Block5dGripKindsPatchEntry};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::{Block5dGripKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeGripKindDefaultRopeKind, base: &Block5dSnapshot) -> Block5dDiff {
    let Some(existing) = base.grip_kinds.iter().find(|item| item.id == payload.id) else { return Block5dDiff::default(); };
    let replacement = Block5dGripKind { default_rope_kind: payload.new_default_rope_kind.clone(), ..existing.clone() };
    Block5dDiff { grip_kinds: Some(Block5dGripKindsDelta { patched: vec![Block5dGripKindsPatchEntry { id: payload.id.clone(), patch: Block5dGripKindsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
