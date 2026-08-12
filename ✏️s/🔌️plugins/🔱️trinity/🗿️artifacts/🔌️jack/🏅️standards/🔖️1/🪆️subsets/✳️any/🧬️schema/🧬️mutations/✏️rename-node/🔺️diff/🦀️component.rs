//! 🔺️ Sparse diff builder for `RenameNode`.
use crate::artifacts::jack::diff::{diff_nodes_patched, JackDiff, JackNodePatch, JackNodePatchEntry};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameNode, _base: &JackSnapshot) -> JackDiff {
    diff_nodes_patched(vec![JackNodePatchEntry { id: payload.id.clone(), patch: JackNodePatch { name: Some(payload.new_name.clone()), ..Default::default() } }])
}
//#endregion 🔖️Diff
