//! 🔺️ Sparse diff builder for `RenameNode`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameNode, base: &JackSnapshot) -> JackDiff {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    if let Some(node) = scene.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.name = payload.new_name.clone();
    }
    diff_replace_content(scene.nodes, scene.edges)
}
//#endregion 🔖️Diff
