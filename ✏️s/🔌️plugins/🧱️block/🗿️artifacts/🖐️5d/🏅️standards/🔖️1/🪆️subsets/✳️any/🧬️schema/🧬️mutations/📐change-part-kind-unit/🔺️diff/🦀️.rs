//! 🔺️ Diff for `ChangePartKindUnit`.

use crate::BlockKindIdentity;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;

//#region 🔖️Diff
pub async fn diff(payload: &super::ChangePartKindUnit, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_unit == base.part_kind.unit {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Part kind unit is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { unit: payload.new_unit.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
