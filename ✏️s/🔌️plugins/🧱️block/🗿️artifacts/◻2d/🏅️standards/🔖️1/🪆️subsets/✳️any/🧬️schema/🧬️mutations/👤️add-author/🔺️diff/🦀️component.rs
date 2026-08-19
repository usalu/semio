//! 🔺️ Sparse diff builder for `AddAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dAuthorList};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::AddAuthor, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    protocol::MutationOutcome::new(Block2dDiff { authors: Some(Block2dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
