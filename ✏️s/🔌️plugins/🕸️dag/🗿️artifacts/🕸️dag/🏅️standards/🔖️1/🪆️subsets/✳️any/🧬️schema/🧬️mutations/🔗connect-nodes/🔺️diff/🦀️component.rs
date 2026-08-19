//! 🔺️ Sparse diff builder for `ConnectNodes`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagFixtureEdge, DagSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ConnectNodes, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    if scene.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let (source_node, _) = crate::artifacts::dag::schema::split_endpoint(&payload.source);
    let (target_node, _) = crate::artifacts::dag::schema::split_endpoint(&payload.target);
    if !scene.nodes.iter().any(|node| node.id == source_node) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Source node \"{}\" does not exist.", source_node), [source_node]);
    }
    if !scene.nodes.iter().any(|node| node.id == target_node) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Target node \"{}\" does not exist.", target_node), [target_node]);
    }
    if source_node == target_node {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" cannot connect to itself.", source_node), [source_node]);
    }
    if scene.edges.iter().any(|edge| {
        let (edge_source, _) = crate::artifacts::dag::schema::split_endpoint(&edge.source);
        let (edge_target, _) = crate::artifacts::dag::schema::split_endpoint(&edge.target);
        edge_source == source_node && edge_target == target_node
    }) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("\"{}\" is already connected to \"{}\"; parallel edges are not allowed.", source_node, target_node));
    }
    if would_create_cycle(&scene.edges, &source_node, &target_node) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Connecting \"{}\" to \"{}\" would create a cycle.", source_node, target_node), [source_node, target_node]);
    }
    let edge = DagFixtureEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), route_style: payload.route_style, properties: payload.properties.clone() };
    let mut edges = scene.edges;
    edges.push(edge);
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, edges))
}

/// 🔁️ Whether adding a `source -> target` edge would create a cycle — true iff `target` can already
/// reach `source` through the existing edge set (a DAG's core invariant).
async fn would_create_cycle(edges: &[DagFixtureEdge], source: &str, target: &str) -> bool {
    let mut visited = std::collections::BTreeSet::new();
    let mut stack = vec![target.to_string()];
    while let Some(node) = stack.pop() {
        if node == source {
            return true;
        }
        if !visited.insert(node.clone()) {
            continue;
        }
        for edge in edges {
            let (from, _) = crate::artifacts::dag::schema::split_endpoint(&edge.source);
            let (to, _) = crate::artifacts::dag::schema::split_endpoint(&edge.target);
            if from == node {
                stack.push(to);
            }
        }
    }
    false
}
//#endregion 🔖️Diff
