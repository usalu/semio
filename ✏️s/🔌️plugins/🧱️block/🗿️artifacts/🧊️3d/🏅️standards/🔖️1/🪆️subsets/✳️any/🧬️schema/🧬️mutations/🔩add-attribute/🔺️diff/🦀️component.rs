//! 🔺️ Sparse diff builder for `AddAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dAttributesDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddAttribute, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { attributes: Some(Block3dAttributesDelta { added: vec![payload.attribute.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
