//! 🔺️ Sparse diff builder for `ChangeMetaDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockMeta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeMetaDescription, base: &Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
