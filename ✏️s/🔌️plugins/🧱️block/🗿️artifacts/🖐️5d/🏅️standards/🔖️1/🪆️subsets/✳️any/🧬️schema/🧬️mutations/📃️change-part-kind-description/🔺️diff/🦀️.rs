//! 🔺️ Diff for `ChangePartKindDescription`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePartKindDescription, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_description == base.part_kind.description {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Part kind description is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { description: Some(payload.new_description.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
