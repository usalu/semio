//! 🔺️ Diff for `ChangeNodeKindVariant`.

use crate::BlockKindIdentity;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;

//#region 🔖️Diff
pub async fn diff(payload: &super::ChangeNodeKindVariant, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_variant == base.node_kind.variant {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Node kind variant is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { variant: payload.new_variant.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
