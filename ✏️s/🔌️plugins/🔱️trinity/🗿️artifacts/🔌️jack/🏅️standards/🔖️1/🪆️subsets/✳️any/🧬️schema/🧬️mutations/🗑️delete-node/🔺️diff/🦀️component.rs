//! 🔺️ Sparse diff builder for `DeleteNode` — removes the node AND every edge severed by its
//! removal (real cascade capture, never apply-then-capture).
use crate::artifacts::jack::diff::{diff_delete_node, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteNode, base: &JackSnapshot) -> JackDiff {
    let severed: Vec<String> = base
        .edges
        .iter()
        .filter(|edge| crate::artifacts::jack::port_node_id(&edge.source) == Some(payload.id.as_str()) || crate::artifacts::jack::port_node_id(&edge.target) == Some(payload.id.as_str()))
        .map(|edge| edge.id.clone())
        .collect();
    diff_delete_node(payload.id.clone(), severed)
}
//#endregion 🔖️Diff
