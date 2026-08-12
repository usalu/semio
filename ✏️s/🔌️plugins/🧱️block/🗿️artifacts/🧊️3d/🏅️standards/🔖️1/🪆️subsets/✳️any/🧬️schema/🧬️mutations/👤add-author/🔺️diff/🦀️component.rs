//! 🔺️ Sparse diff builder for `AddAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dAuthorList};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddAuthor, base: &Block3dSnapshot) -> Block3dDiff {
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    Block3dDiff { authors: Some(Block3dAuthorList { values }), ..Default::default() }
}
//#endregion 🔖️Diff
