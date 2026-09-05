//! 🔺️ Diff for `ChangePartKindDescription`.

use crate::BlockKindIdentity;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePartKindDescription, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_description == base.part_kind.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Part kind description is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { description: payload.new_description.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
