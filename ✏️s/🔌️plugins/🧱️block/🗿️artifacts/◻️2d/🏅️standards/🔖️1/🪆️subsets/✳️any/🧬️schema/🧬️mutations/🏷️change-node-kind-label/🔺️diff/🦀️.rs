//! 🔺️ Diff for `ChangeNodeKindLabel`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKindLabel, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_label == base.node_kind.label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node kind label is already \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { label: Some(payload.new_label.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
