//! 🔺️ Sparse diff builder for `ChangeMetaDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::BlockMeta;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeMetaDescription, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
