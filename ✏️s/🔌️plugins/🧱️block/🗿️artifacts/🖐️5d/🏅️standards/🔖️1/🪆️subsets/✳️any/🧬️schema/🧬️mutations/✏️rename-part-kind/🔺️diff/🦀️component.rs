//! 🔺️ Sparse diff builder for `RenamePartKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenamePartKind, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { part_kind: Some(BlockKindIdentity { name: payload.new_name.clone(), ..base.part_kind.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
