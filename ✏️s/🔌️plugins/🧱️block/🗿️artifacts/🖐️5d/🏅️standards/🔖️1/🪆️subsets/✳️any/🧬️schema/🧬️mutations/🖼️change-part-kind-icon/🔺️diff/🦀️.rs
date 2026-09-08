//! 🔺️ Diff for `ChangePartKindIcon`.

use crate::BlockKindIdentity;
use crate::Block5dSnapshot;
use crate::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePartKindIcon, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_icon == base.part_kind.icon {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Part kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(BlockKindIdentity { icon: payload.new_icon.clone(), ..base.part_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
