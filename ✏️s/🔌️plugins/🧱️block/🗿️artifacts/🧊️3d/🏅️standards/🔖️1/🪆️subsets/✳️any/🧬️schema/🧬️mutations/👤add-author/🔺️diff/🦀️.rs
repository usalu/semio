//! 🔺️ Diff for `AddAuthor`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dAuthorList, Block3dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::AddAuthor, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "author", payload.author.id)).at(vec![payload.author.id.clone()])]);
    }
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    protocol::MutationOutcome::new(Block3dDiff { authors: Some(Block3dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
