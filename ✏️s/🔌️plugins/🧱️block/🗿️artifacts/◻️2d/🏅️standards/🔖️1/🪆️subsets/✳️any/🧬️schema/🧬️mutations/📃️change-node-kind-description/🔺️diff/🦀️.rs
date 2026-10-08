//! 🔺️ Diff for `ChangeNodeKindDescription`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKindDescription, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_description == base.node_kind.description {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Node kind description is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { description: Some(payload.new_description.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
