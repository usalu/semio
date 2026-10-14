//! 🔺️ Diff for `ChangeMetaDescription`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_meta_description::ChangeMetaDescription, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block3dDiff { meta: Some(semio_s_plugin_block::BlockMetaPatch { description: Some(payload.new_description.clone()) }), ..Default::default() })
}
//#endregion 🔖️Diff
