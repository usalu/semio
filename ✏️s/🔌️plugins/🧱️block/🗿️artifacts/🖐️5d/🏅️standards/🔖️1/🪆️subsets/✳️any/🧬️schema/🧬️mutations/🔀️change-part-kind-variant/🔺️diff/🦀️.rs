//! 🔺️ Diff for `ChangePartKindVariant`.

use crate::BlockKindIdentity;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePartKindVariant, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_variant == base.part_kind.variant {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Part kind variant is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { variant: payload.new_variant.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
