//! 🔺️ Diff for `RemoveAuthor`.

use crate::Block5dSnapshot;
use semio_s_plugin_block::BlockAuthorsDelta;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAuthor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.authors.iter().any(|author| author.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "author", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { authors: Some(BlockAuthorsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
