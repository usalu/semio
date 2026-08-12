//! 🔺️ Sparse diff builder for `RemoveAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dAttributesDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveAttribute, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { attributes: Some(Block3dAttributesDelta { removed: vec![payload.key.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
