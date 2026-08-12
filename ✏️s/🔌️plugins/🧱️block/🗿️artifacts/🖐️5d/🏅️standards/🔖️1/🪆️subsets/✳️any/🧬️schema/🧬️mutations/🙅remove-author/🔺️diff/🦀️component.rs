//! 🔺️ Sparse diff builder for `RemoveAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dAuthorList};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockAuthor};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveAuthor, base: &Block5dSnapshot) -> Block5dDiff {
    let values: Vec<BlockAuthor> = base.authors.iter().filter(|author| author.id != payload.id).cloned().collect();
    Block5dDiff { authors: Some(Block5dAuthorList { values }), ..Default::default() }
}
//#endregion 🔖️Diff
