//! 🔺️ Diff for `ChangeNodeKindIcon`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_node_kind_icon::ChangeNodeKindIcon, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_icon == base.node_kind.icon {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Node kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { icon: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_icon.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
