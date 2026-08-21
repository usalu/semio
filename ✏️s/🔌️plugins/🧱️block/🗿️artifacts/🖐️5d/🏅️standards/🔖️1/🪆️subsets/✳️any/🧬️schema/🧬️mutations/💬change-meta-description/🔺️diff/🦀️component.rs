//! 🔺️ Sparse diff builder for `ChangeMetaDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::BlockMeta;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeMetaDescription, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
