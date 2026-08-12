//! 🔺️ Sparse diff builder for `ChangeDataProperty` — encodes the new value as JSON on the
//! addressed node's or edge's `key`/`valueJson` patch slot.
use crate::artifacts::jack::diff::{diff_edges_patched, diff_nodes_patched, JackDiff, JackEdgePatch, JackEdgePatchEntry, JackNodePatch, JackNodePatchEntry};
use crate::artifacts::jack::{EntityRef, JackSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeDataProperty, _base: &JackSnapshot) -> JackDiff {
    let json = serde_json::to_string(&payload.new_value).unwrap_or_else(|_| "null".to_string());
    match &payload.entity {
        EntityRef::Node(id) => diff_nodes_patched(vec![JackNodePatchEntry { id: id.clone(), patch: JackNodePatch { key: Some(payload.key.clone()), value_json: Some(Some(json)), ..Default::default() } }]),
        EntityRef::Edge(id) => diff_edges_patched(vec![JackEdgePatchEntry { id: id.clone(), patch: JackEdgePatch { key: Some(payload.key.clone()), value_json: Some(Some(json)) } }]),
    }
}
//#endregion 🔖️Diff
