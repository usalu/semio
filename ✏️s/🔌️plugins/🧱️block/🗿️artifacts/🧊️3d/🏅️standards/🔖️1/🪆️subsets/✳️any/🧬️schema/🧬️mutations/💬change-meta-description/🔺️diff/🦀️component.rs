//! 🔺️ Sparse diff builder for `ChangeMetaDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockMeta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeMetaDescription, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
