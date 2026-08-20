//! 🧭 `topology` — one named inference: the typed property graph's topological order, per-node
//! depth, and cycle-freedom, computed by Kahn's algorithm over `nodes`/`edges`. Node/edge
//! identity is `GraphNodeId{value}`/`GraphEdgeId{value}` (named single-field structs, never bare
//! strings) — this walk dispatches on `.value`. A plain whole-snapshot scalar — no
//! `InferredField`/incremental caching needed for a single BFS pass (same ruling trinity's own
//! `jack` and sibling `✳️flow` topology facets reach for their own node/edge graphs).

use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

//#region 🔖️Topology
/// 🧭 Topological shape of the semio graph's node/edge structure.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioGraphTopology {
    /// 🥇️ Node id values in Kahn topological order — only nodes reachable by repeatedly removing
    /// zero-indegree nodes; nodes stuck in a cycle are omitted (see `cycle_free`).
    pub topo_order: Vec<String>,
    /// 📏️ Longest-path depth from any zero-indegree root, keyed by node id value — only defined
    /// for nodes present in `topo_order`.
    pub depth: BTreeMap<String, u32>,
    /// ✅️ Whether every node was reachable by the topological sort (`topo_order.len() == nodeCount`).
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🩹 Hand-rolled: the naive derive would default `cycle_free` to `false`, but an empty graph
/// (`node_count: 0`) is honestly cycle-free — `compute_semio_graph_topology(&SemioGraphSnapshot::default())`
/// must equal `SemioGraphTopology::default()` (the inference-default law), so this matches that
/// zero case (same fix jack's own `JackTopology::default()` and sibling `✳️flow`'s
/// `SemioFlowTopology::default()` document).
impl Default for SemioGraphTopology {
    fn default() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}

/// 📐️ Computes `topology` directly from `nodes`/`edges` via Kahn's algorithm — deterministic
/// because both the root frontier and each frontier's children are drained in node-id sort order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_graph_topology(snapshot: &SemioGraphSnapshot) -> SemioGraphTopology {
    let node_count = snapshot.nodes.len() as u32;
    let mut adjacency: BTreeMap<String, Vec<String>> = snapshot.nodes.iter().map(|node| (node.id.value.clone(), Vec::new())).collect();
    let mut indegree: BTreeMap<String, u32> = snapshot.nodes.iter().map(|node| (node.id.value.clone(), 0u32)).collect();
    for edge in &snapshot.edges {
        let (source_id, target_id) = (&edge.source.value, &edge.target.value);
        if !indegree.contains_key(source_id) || !indegree.contains_key(target_id) {
            continue;
        }
        adjacency.get_mut(source_id).expect("known node").push(target_id.clone());
        *indegree.get_mut(target_id).expect("known node") += 1;
    }

    let mut remaining_indegree = indegree.clone();
    let mut queue: VecDeque<String> = indegree.iter().filter(|(_, &degree)| degree == 0).map(|(id, _)| id.clone()).collect();
    let mut depth: BTreeMap<String, u32> = queue.iter().map(|id| (id.clone(), 0u32)).collect();
    let mut topo_order = Vec::new();

    while let Some(node_id) = queue.pop_front() {
        topo_order.push(node_id.clone());
        let node_depth = depth.get(&node_id).copied().unwrap_or(0);
        let Some(children) = adjacency.get(&node_id).cloned() else { continue };
        let mut newly_zero = Vec::new();
        for child_id in children {
            let candidate_depth = node_depth + 1;
            let entry = depth.entry(child_id.clone()).or_insert(0);
            if candidate_depth > *entry {
                *entry = candidate_depth;
            }
            let remaining = remaining_indegree.get_mut(&child_id).expect("known node");
            *remaining -= 1;
            if *remaining == 0 {
                newly_zero.push(child_id);
            }
        }
        newly_zero.sort();
        for child_id in newly_zero {
            queue.push_back(child_id);
        }
    }

    let cycle_free = topo_order.len() as u32 == node_count;
    depth.retain(|id, _| topo_order.iter().any(|ordered| ordered == id));
    SemioGraphTopology { topo_order, depth, cycle_free, node_count }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn node(id: &str) -> SemioGraphNode {
        SemioGraphNode { id: GraphNodeId::new(id), kind: "task".into(), label: id.into(), position: Default::default(), ports: Vec::new(), properties: Vec::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edge(id: &str, source: &str, target: &str) -> SemioGraphEdge {
        SemioGraphEdge { id: GraphEdgeId::new(id), source: GraphNodeId::new(source), target: GraphNodeId::new(target), kind: "flows-to".into(), label: id.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn chain_snapshot() -> SemioGraphSnapshot {
        // root -e1- mid -e2- leaf: a 3-node chain.
        SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: vec![node("root"), node("mid"), node("leaf")], edges: vec![edge("e1", "root", "mid"), edge("e2", "mid", "leaf")] }
    }

    #[semio_framework_async_macros::async_test]
    async fn chain_is_cycle_free_with_increasing_depth() {
        let topology = compute_semio_graph_topology(&chain_snapshot());
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 3);
        assert_eq!(topology.topo_order, vec!["root".to_string(), "mid".to_string(), "leaf".to_string()]);
        assert_eq!(topology.depth.get("root"), Some(&0));
        assert_eq!(topology.depth.get("mid"), Some(&1));
        assert_eq!(topology.depth.get("leaf"), Some(&2));
    }

    #[semio_framework_async_macros::async_test]
    async fn a_cycle_is_reported_as_not_cycle_free() {
        let mut snapshot = chain_snapshot();
        snapshot.edges.push(edge("e3", "leaf", "root"));
        let topology = compute_semio_graph_topology(&snapshot);
        assert!(!topology.cycle_free);
        assert!(topology.topo_order.is_empty(), "every node in the 3-cycle has nonzero indegree");
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(compute_semio_graph_topology(&snapshot), compute_semio_graph_topology(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_graph_topology(&SemioGraphSnapshot::default()), SemioGraphTopology::default());
    }
}
//#endregion 🧪️Tests
