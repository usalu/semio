//! 🔺️ Diff for `RenameNodeKind`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::rename_node_kind::RenameNodeKind, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_name == base.node_kind.name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node kind name is already \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { name: Some(payload.new_name.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
