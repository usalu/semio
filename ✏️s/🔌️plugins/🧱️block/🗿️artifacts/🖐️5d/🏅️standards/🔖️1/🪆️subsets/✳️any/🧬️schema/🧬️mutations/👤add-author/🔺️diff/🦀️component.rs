//! 🔺️ Sparse diff builder for `AddAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dAuthorList};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddAuthor, base: &Block5dSnapshot) -> Block5dDiff {
    let mut values = base.authors.clone();
    values.push(payload.author.clone());
    Block5dDiff { authors: Some(Block5dAuthorList { values }), ..Default::default() }
}
//#endregion 🔖️Diff
