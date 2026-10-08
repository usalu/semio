//! 🔺️ Diff for `AddAuthor`.

use crate::Block2dSnapshot;
use semio_s_plugin_block::BlockAuthorsDelta;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddAuthor, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block2dDiff { authors: Some(BlockAuthorsDelta { added: vec![payload.author.clone()], reordered: semio_s_plugin_block::block_insert_order(base.authors.iter().map(|item| item.id.as_str()), &payload.author.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
