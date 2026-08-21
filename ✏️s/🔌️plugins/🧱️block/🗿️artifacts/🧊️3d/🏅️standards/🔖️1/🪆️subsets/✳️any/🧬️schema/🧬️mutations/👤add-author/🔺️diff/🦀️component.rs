//! 🔺️ Sparse diff builder for `AddAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dAuthorList;
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::AddAuthor, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    protocol::MutationOutcome::new(Block3dDiff { authors: Some(Block3dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
