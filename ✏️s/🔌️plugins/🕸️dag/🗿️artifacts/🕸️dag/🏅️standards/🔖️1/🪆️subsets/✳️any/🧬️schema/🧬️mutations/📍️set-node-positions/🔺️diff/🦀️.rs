//! 🔺️ Sparse diff builder for `SetNodePositions` — every addressed node lands at its payload position; a node the scene
//! lacks is skipped (`mutation.partial`).
use crate::diff::DagDiff;
use crate::mutations::{dag_partial, dag_targets_invariant};
use crate::schema::diff::diff_replace_content;
use crate::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
/// 🏗️ A malformed position list is `mutation.invariant`; none left to place is `mutation.target-missing`; every node
/// already there is `mutation.no-op`.
pub fn diff(payload: &super::mutation::SetNodePositions, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let ids = payload.ids();
    if let Err(reason) = dag_targets_invariant(&ids) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, ids);
    }
    if payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node position must be finite", ids);
    }
    let scene = dag_working_scene(base);
    let missing: Vec<String> = ids.iter().filter(|id| !scene.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if missing.len() == ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} node(s) exists", ids.len()), missing);
    }
    let messages: Vec<protocol::MutationMessage> = dag_partial(missing, ids.len(), "no such node").into_iter().collect();
    let mut nodes = scene.nodes;
    let mut changed = false;
    for position in &payload.positions {
        if let Some(node) = nodes.iter_mut().find(|node| node.id == position.id) {
            changed |= (node.x, node.y) != (position.x, position.y);
            node.x = position.x;
            node.y = position.y;
        }
    }
    if !changed {
        return protocol::MutationOutcome::empty().absorb_messages(messages.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "every node already sits at its position").at(ids)]));
    }
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges)).absorb_messages(messages)
}
//#endregion 🔖️Diff
