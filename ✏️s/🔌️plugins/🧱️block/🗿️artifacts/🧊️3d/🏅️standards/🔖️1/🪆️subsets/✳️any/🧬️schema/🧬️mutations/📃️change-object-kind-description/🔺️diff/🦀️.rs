//! 🔺️ Diff for `ChangeObjectKindDescription`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_object_kind_description::ChangeObjectKindDescription, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_description == base.object_kind.description {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Object kind description is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { object_kind: Some(semio_s_plugin_block::BlockKindIdentityPatch { description: Some(payload.new_description.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
