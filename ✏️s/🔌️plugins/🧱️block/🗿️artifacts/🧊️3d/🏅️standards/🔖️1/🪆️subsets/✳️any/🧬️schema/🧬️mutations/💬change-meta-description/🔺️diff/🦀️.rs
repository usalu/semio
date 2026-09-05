//! 🔺️ Diff for `ChangeMetaDescription`.

use crate::BlockMeta;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMetaDescription, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
