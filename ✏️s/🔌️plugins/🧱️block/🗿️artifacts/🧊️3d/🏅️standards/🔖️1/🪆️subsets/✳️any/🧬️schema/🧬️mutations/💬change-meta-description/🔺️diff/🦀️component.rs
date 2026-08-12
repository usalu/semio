//! 🔺️ Sparse diff builder for `ChangeMetaDescription` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::{BlockMeta};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeMetaDescription, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { meta: Some(BlockMeta { description: payload.new_description.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
