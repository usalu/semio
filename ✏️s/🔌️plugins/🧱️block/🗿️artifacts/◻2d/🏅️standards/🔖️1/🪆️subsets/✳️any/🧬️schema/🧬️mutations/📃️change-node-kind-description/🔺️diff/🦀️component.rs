//! 🔺️ Sparse diff builder for `ChangeNodeKindDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeKindDescription, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_description == base.node_kind.description {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Node kind description is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { description: payload.new_description.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
