//! 🔺️ Sparse diff builder for `AddAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dAuthorList};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddAuthor, base: &Block2dSnapshot) -> Block2dDiff {
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    Block2dDiff { authors: Some(Block2dAuthorList { values }), ..Default::default() }
}
//#endregion 🔖️Diff
