//! 🔺️ Sparse diff builder for `ReplaceNodeKind`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceNodeKind, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    let Some(existing) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" already has that kind.", payload.id));
    }
    let mut nodes = scene.nodes;
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.kind = payload.new_kind.clone();
    }
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges))
}
//#endregion 🔖️Diff
