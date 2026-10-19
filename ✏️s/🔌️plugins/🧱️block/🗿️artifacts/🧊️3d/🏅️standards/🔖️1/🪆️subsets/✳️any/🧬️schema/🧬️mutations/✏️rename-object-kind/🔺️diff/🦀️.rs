//! 🔺️ Diff for `RenameObjectKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::rename_object_kind::RenameObjectKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_name == base.object_kind.name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Object kind name is already \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { name: Some(payload.new_name.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
