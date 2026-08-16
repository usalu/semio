//! 🔺️ Sparse diff builder for `ChangePartKindDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangePartKindDescription, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { description: payload.new_description.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
