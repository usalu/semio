//! 🔺️ Diff for `ChangeMetaDescription`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_meta_description::ChangeMetaDescription, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { meta: Some(semio_s_plugin_block::BlockMetaPatch { description: Some(payload.new_description.clone()) }), ..Default::default() })
}
//#endregion 🔖️Diff
