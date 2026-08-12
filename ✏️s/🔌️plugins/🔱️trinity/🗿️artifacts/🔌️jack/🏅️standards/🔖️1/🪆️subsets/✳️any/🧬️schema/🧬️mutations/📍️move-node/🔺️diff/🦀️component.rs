//! 🔺️ Sparse diff builder for `MoveNode`.
use crate::artifacts::jack::diff::{diff_nodes_patched, JackDiff, JackNodePatch, JackNodePatchEntry};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveNode, _base: &JackSnapshot) -> JackDiff {
    diff_nodes_patched(vec![JackNodePatchEntry { id: payload.id.clone(), patch: JackNodePatch { x: Some(payload.x), y: Some(payload.y), ..Default::default() } }])
}
//#endregion 🔖️Diff
