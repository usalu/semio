//! 🔺️ Sparse diff builder for `ChangePartKindIcon` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangePartKindIcon, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { icon: payload.new_icon.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
