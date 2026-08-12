//! 🔺️ Sparse diff builder for `ChangeMetaDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockMeta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeMetaDescription, base: &Block2dSnapshot) -> Block2dDiff {
    Block2dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
