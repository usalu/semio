//! 🔺️ Diff for `RemoveAuthor`.

use crate::BlockAuthor;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dAuthorList, Block3dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAuthor, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.authors.iter().any(|author| author.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "author", payload.id), vec![payload.id.clone()]);
    }
    let values: Vec<BlockAuthor> = base.authors.iter().filter(|author| author.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(Block3dDiff { authors: Some(Block3dAuthorList { values }), ..Default::default() })
}
//#endregion 🔖️Diff
