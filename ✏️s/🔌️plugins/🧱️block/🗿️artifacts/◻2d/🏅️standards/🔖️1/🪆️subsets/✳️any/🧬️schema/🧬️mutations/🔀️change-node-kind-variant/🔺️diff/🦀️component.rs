//! 🔺️ Sparse diff builder for `ChangeNodeKindVariant` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockKindIdentity};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeKindVariant, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_variant == base.node_kind.variant {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Node kind variant is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { variant: payload.new_variant.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
