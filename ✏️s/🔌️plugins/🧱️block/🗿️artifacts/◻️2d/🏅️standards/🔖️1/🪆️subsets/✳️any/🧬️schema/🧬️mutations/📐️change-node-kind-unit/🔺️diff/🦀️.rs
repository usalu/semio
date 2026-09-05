//! 🔺️ Diff for `ChangeNodeKindUnit`.

use crate::BlockKindIdentity;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKindUnit, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_unit == base.node_kind.unit {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Node kind unit is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(BlockKindIdentity { unit: payload.new_unit.clone(), ..base.node_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
