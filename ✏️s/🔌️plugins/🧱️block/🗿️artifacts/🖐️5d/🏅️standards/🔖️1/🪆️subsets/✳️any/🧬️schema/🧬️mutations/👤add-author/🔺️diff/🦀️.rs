//! 🔺️ Diff for `AddAuthor`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dAuthorList, Block5dDiff};

//#region 🔖️Diff
pub async fn diff(payload: &super::AddAuthor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    protocol::MutationOutcome::new(Block5dDiff { authors: Some(Block5dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
