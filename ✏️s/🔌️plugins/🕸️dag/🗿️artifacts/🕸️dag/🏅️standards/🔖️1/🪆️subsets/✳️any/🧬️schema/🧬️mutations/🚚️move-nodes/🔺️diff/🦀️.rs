//! 🔺️ Sparse diff builder for `MoveNodes` — every addressed node moves by the payload offset from its BASE position; a
//! node the scene lacks is skipped (`mutation.partial`).
use crate::diff::DagDiff;
use crate::mutations::{dag_partial, dag_targets_invariant};
use crate::schema::diff::diff_replace_content;
use crate::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
/// 🏗️ A malformed target list or a non-finite offset is `mutation.invariant`; none left to move is
/// `mutation.target-missing`; a zero offset is `mutation.no-op`.
pub fn diff(payload: &super::mutation::MoveNodes, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    if let Err(reason) = dag_targets_invariant(&payload.ids) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, payload.ids.clone());
    }
    if !payload.dx.is_finite() || !payload.dy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node offset must be finite", payload.ids.clone());
    }
    let scene = dag_working_scene(base);
    let missing: Vec<String> = payload.ids.iter().filter(|id| !scene.nodes.iter().any(|node| &node.id == *id)).cloned().collect();
    if missing.len() == payload.ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} node(s) exists", payload.ids.len()), missing);
    }
    let messages: Vec<protocol::MutationMessage> = dag_partial(missing, payload.ids.len(), "no such node").into_iter().collect();
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::empty().absorb_messages(messages.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "the drag offset is zero").at(payload.ids.clone())]));
    }
    let mut nodes = scene.nodes;
    for node in nodes.iter_mut().filter(|node| payload.ids.contains(&node.id)) {
        node.x += payload.dx;
        node.y += payload.dy;
    }
    if nodes.iter().filter(|node| payload.ids.contains(&node.id)).any(|node| !node.x.is_finite() || !node.y.is_finite()) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "the moved position leaves the finite canvas", payload.ids.clone());
    }
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges)).absorb_messages(messages)
}
//#endregion 🔖️Diff
