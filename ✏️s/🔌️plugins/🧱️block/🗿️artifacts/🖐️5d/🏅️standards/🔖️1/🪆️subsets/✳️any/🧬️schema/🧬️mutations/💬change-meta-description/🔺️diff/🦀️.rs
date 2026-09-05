//! 🔺️ Diff for `ChangeMetaDescription`.

use crate::BlockMeta;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMetaDescription, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
