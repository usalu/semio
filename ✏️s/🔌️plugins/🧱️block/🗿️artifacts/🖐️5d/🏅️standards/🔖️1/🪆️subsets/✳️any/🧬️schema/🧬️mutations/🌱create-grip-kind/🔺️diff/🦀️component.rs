//! 🔺️ Sparse diff builder for `CreateGripKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dGripKindsDelta};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::{Block5dGripKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateGripKind, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { grip_kinds: Some(Block5dGripKindsDelta { added: vec![payload.grip_kind.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
