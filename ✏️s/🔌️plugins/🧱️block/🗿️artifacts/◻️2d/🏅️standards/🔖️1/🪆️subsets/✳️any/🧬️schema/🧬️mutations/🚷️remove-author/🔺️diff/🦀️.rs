//! 🔺️ Diff for `RemoveAuthor`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_plugin_block::BlockAuthorsDelta;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_author::RemoveAuthor, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.authors.iter().any(|author| author.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "author", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { authors: BlockAuthorsDelta::removal(&base.authors, base.authors.iter().position(|author| author.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
