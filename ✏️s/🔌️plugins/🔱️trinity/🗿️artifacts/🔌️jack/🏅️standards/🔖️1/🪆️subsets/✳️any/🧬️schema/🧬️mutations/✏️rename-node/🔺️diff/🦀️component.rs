//! 🔺️ Sparse diff builder for `RenameNode`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameNode, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    let Some(existing) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    if let Some(node) = scene.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, scene.edges))
}
//#endregion 🔖️Diff
