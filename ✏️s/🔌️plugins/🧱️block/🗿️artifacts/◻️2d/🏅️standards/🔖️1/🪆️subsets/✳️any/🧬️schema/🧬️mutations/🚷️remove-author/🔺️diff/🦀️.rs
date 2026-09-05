//! 🔺️ Diff for `RemoveAuthor`.

use crate::BlockAuthor;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dAuthorList, Block2dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAuthor, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.authors.iter().any(|author| author.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "author", payload.id), vec![payload.id.clone()]);
    }
    let values: Vec<BlockAuthor> = base.authors.iter().filter(|author| author.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(Block2dDiff { authors: Some(Block2dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
