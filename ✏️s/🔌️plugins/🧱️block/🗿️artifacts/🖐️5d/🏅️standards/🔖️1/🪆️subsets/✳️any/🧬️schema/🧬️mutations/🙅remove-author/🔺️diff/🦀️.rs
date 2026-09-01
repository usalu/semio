//! 🔺️ Diff for `RemoveAuthor`.

use crate::BlockAuthor;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dAuthorList, Block5dDiff};

//#region 🔖️Diff
pub async fn diff(payload: &super::RemoveAuthor, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.authors.iter().any(|author| author.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "author", payload.id), vec![payload.id.clone()]);
    }
    let values: Vec<BlockAuthor> = base.authors.iter().filter(|author| author.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(Block5dDiff { authors: Some(Block5dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
