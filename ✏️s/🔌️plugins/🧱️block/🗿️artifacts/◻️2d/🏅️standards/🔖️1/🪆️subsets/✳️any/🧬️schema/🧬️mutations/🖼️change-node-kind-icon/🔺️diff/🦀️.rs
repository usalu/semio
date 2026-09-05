//! 🔺️ Diff for `ChangeNodeKindIcon`.

use crate::BlockKindIdentity;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKindIcon, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_icon == base.node_kind.icon {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Node kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { icon: payload.new_icon.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
