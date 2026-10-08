//! 🔺️ Diff for `AddAuthor`.

use crate::Block5dSnapshot;
use semio_s_plugin_block::BlockAuthorsDelta;
use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddAuthor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff { authors: Some(BlockAuthorsDelta { added: vec![payload.author.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
