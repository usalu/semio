//! 🔺️ Sparse diff builder for `ReorderNodes`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReorderNodes, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    let mut seen = std::collections::BTreeSet::new();
    let mut duplicates = Vec::new();
    for id in &payload.order {
        if !seen.insert(id.clone()) {
            duplicates.push(id.clone());
        }
    }
    if !duplicates.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Reorder list contains duplicate id(s): {}.", duplicates.join(", ")), duplicates);
    }
    let current_order: Vec<String> = scene.nodes.iter().map(|node| node.id.clone()).collect();
    let unknown: Vec<String> = payload.order.iter().filter(|id| !current_order.contains(id)).cloned().collect();
    let mut by_id: std::collections::BTreeMap<_, _> = scene.nodes.into_iter().map(|node| (node.id.clone(), node)).collect();
    let mut ordered = Vec::with_capacity(payload.order.len());
    for id in &payload.order {
        if let Some(node) = by_id.remove(id) {
            ordered.push(node);
        }
    }
    ordered.extend(by_id.into_values());
    let new_order: Vec<String> = ordered.iter().map(|node| node.id.clone()).collect();
    if new_order == current_order {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Nodes are already in the requested order.");
    }
    let mut outcome = protocol::MutationOutcome::new(diff_replace_content(ordered, scene.edges));
    if !unknown.is_empty() {
        outcome = outcome.absorb_messages([protocol::MutationMessage::error("mutation.target-missing", format!("Unknown node id(s) ignored: {}.", unknown.join(", "))).at(unknown)]);
    }
    outcome
}
//#endregion 🔖️Diff
