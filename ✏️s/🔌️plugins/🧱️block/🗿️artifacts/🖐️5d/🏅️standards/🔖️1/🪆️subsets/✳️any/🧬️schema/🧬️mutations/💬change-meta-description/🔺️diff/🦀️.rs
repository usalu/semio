//! 🔺️ Diff for `ChangeMetaDescription`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_meta_description::ChangeMetaDescription, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_description == base.meta.description {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Meta description is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { meta: Some(semio_s_plugin_block::BlockMetaPatch { description: Some(payload.new_description.clone()) }), ..Default::default() })
}
//#endregion 🔖️Diff
