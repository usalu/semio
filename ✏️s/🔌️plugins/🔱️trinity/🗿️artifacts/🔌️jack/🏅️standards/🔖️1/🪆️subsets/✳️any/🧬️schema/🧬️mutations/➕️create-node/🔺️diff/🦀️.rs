//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture). Reads the current scene off `base`, appends the new node to a clone, and replaces the
//! composed content child wholesale via `diff_replace_content`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateNode, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let scene = crate::artifacts::jack::jack_working_scene(base);
    if scene.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    let mut nodes = scene.nodes;
    nodes.push(payload.node.clone());
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges))
}
//#endregion 🔖️Diff
