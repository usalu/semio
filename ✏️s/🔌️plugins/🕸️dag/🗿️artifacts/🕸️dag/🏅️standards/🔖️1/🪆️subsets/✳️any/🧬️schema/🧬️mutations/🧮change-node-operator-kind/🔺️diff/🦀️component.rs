//! 🔺️ Sparse diff builder for `ChangeNodeOperatorKind`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeNodeOperatorKind, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    let Some(existing) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.operator_kind == payload.new_operator_kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" already has that operator kind.", payload.id));
    }
    let mut nodes = scene.nodes;
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.operator_kind = payload.new_operator_kind.clone();
    }
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges))
}
//#endregion 🔖️Diff
