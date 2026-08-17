//! 🔺️ Sparse diff builder for `ChangePartKindIcon` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangePartKindIcon, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_icon == base.part_kind.icon {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Part kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { icon: payload.new_icon.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
