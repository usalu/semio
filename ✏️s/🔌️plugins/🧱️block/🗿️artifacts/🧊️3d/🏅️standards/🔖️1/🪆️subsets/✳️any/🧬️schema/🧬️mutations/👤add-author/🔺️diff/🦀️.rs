//! 🔺️ Diff for `AddAuthor`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_plugin_block::BlockAuthorsDelta;
use semio_s_artifact_block_3d_diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::add_author::AddAuthor, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff { authors: BlockAuthorsDelta::insertion(semio_s_plugin_block::block_insert_index(base.authors.len(), payload.index), payload.author.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
