//! 🔺️ Sparse diff builder for `RemoveAuthor` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dAuthorList};
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockAuthor};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveAuthor, base: &Block2dSnapshot) -> Block2dDiff {
    let values: Vec<BlockAuthor> = base.authors.iter().filter(|author| author.id != payload.id).cloned().collect();
    Block2dDiff { authors: Some(Block2dAuthorList { values }), ..Default::default() }
}
//#endregion 🔖️Diff
