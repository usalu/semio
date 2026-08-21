//! 🔺️ Sparse diff builder for `AddAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dAuthorList;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::AddAuthor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    protocol::MutationOutcome::new(Block5dDiff { authors: Some(Block5dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
