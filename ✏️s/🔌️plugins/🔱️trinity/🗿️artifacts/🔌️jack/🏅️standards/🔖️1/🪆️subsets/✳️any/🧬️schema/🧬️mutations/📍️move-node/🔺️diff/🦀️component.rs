//! 🔺️ Sparse diff builder for `MoveNode`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveNode, base: &JackSnapshot) -> JackDiff {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    if let Some(node) = scene.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.x = payload.x;
        node.y = payload.y;
    }
    diff_replace_content(scene.nodes, scene.edges)
}
//#endregion 🔖️Diff
