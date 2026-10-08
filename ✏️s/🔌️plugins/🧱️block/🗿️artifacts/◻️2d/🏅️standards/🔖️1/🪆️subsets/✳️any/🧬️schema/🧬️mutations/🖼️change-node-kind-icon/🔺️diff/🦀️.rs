//! 🔺️ Diff for `ChangeNodeKindIcon`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeNodeKindIcon, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_icon == base.node_kind.icon {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Node kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { node_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { icon: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_icon.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
