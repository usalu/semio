//! 🔺️ Diff for `ChangeNodeKindUnit`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKindUnit, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_unit == base.node_kind.unit {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Node kind unit is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { unit: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_unit.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
