//! 🔺️ Sparse diff builder for `RemoveAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dAuthorList};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockAuthor};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveAuthor, base: &Block3dSnapshot) -> Block3dDiff {
    let values: Vec<BlockAuthor> = base.authors.iter().filter(|author| author.id != payload.id).cloned().collect();
    Block3dDiff { authors: Some(Block3dAuthorList { values }), ..Default::default() }
}
//#endregion 🔖️Diff
