//! 🔺️ Diff for `ChangeObjectKindIcon`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_object_kind_icon::ChangeObjectKindIcon, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_icon == base.object_kind.icon {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Object kind icon is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { icon: Some(semio_s_plugin_block::BlockOptionalText { value: payload.new_icon.clone() }), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
