//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture).
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateNode, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    if scene.nodes.iter().any(|node| node.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    let mut nodes = scene.nodes;
    nodes.push(payload.node.clone());
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges))
}
//#endregion 🔖️Diff
