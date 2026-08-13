//! 🔺️ Sparse diff builder for `DeleteNode` — removes the node AND every edge severed by its
//! removal (real cascade capture, never apply-then-capture) against the current scene off `base`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteNode, base: &JackSnapshot) -> JackDiff {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    scene.nodes.retain(|node| node.id != payload.id);
    scene.edges.retain(|edge| {
        crate::artifacts::jack::port_node_id(&edge.source) != Some(payload.id.as_str()) && crate::artifacts::jack::port_node_id(&edge.target) != Some(payload.id.as_str())
    });
    diff_replace_content(scene.nodes, scene.edges)
}
//#endregion 🔖️Diff
