//! 🔺️ Sparse diff builder for `RemoveDataProperty` — `valueJson: Some(None)` signals a clear.
use crate::artifacts::jack::diff::{diff_edges_patched, diff_nodes_patched, JackDiff, JackEdgePatch, JackEdgePatchEntry, JackNodePatch, JackNodePatchEntry};
use crate::artifacts::jack::{EntityRef, JackSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveDataProperty, _base: &JackSnapshot) -> JackDiff {
    match &payload.entity {
        EntityRef::Node(id) => diff_nodes_patched(vec![JackNodePatchEntry { id: id.clone(), patch: JackNodePatch { key: Some(payload.key.clone()), value_json: Some(None), ..Default::default() } }]),
        EntityRef::Edge(id) => diff_edges_patched(vec![JackEdgePatchEntry { id: id.clone(), patch: JackEdgePatch { key: Some(payload.key.clone()), value_json: Some(None) } }]),
    }
}
//#endregion 🔖️Diff
