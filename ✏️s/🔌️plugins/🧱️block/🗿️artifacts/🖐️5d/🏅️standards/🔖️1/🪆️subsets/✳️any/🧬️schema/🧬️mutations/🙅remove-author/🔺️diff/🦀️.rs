//! 🔺️ Diff for `RemoveAuthor`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_plugin_block::BlockAuthorsDelta;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_author::RemoveAuthor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.authors.iter().any(|author| author.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "author", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { authors: BlockAuthorsDelta::removal(&base.authors, base.authors.iter().position(|author| author.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
