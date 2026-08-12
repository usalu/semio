//! 🔺️ Sparse diff builder for `DeleteRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dRepresentationsDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteRepresentation, base: &Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { representations: Some(Block3dRepresentationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
