//! 🔺️ Sparse diff builder for `DeleteGripKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dGripKindsDelta};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::{Block5dGripKind};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteGripKind, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { grip_kinds: Some(Block5dGripKindsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
