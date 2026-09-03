//! 🔺️ Diff for `ChangeNodeKindLabel`.

use crate::BlockKindIdentity;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;

//#region 🔖️Diff
pub async fn diff(payload: &super::ChangeNodeKindLabel, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_label == base.node_kind.label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node kind label is already \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { label: payload.new_label.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
