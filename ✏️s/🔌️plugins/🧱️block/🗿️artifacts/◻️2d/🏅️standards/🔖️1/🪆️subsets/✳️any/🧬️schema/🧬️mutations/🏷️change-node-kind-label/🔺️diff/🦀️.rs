//! 🔺️ Diff for `ChangeNodeKindLabel`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_node_kind_label::ChangeNodeKindLabel, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_label == base.node_kind.label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node kind label is already \"{}\".", payload.new_label));
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { label: Some(payload.new_label.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
