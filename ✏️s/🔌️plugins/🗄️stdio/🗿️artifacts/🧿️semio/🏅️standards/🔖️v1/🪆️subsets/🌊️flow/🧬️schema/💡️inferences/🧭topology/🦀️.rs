//! 🧭 `topology` — one named inference: the node/edge flow graph's topological order, per-node
//! depth, and cycle-freedom, computed by Kahn's algorithm over `nodes`/`edges`. Edge endpoints
//! are `PortRef{node,port}` — only `node` participates in graph dispatch, `port` is presentation
//! detail this inference ignores. A plain whole-snapshot scalar — no `InferredField`/incremental
//! caching needed for a single BFS pass (same ruling trinity's own `jack` topology facet reaches
//! for its own node/edge graph).

use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use std::collections::{BTreeMap, VecDeque};

//#region 🔖️Topology
/// 🧭 Topological shape of the semio flow node/edge graph.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioFlowTopology {
    /// 🥇️ Node ids in Kahn topological order — only nodes reachable by repeatedly removing
    /// zero-indegree nodes; nodes stuck in a cycle are omitted (see `cycle_free`).
    pub topo_order: Vec<String>,
    /// 📏️ Longest-path depth from any zero-indegree root, keyed by node id — only defined for
    /// nodes present in `topo_order`.
    pub depth: BTreeMap<String, u32>,
    /// ✅️ Whether every node was reachable by the topological sort (`topo_order.len() == nodeCount`).
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🩹 Hand-rolled: the naive derive would default `cycle_free` to `false`, but an empty graph
/// (`node_count: 0`) is honestly cycle-free — `compute_semio_flow_topology(&SemioFlowSnapshot::default())`
/// must equal `SemioFlowTopology::default()` (the inference-default law), so this matches that
/// zero case (same fix jack's own `JackTopology::default()` documents).
impl Default for SemioFlowTopology {
    fn default() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}

/// 📐️ Computes `topology` directly from `nodes`/`edges` via Kahn's algorithm — deterministic
/// because both the root frontier and each frontier's children are drained in node-id sort order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_flow_topology(snapshot: &SemioFlowSnapshot) -> SemioFlowTopology {
    let node_count = snapshot.nodes.len() as u32;
    let mut adjacency: BTreeMap<String, Vec<String>> = snapshot.nodes.iter().map(|node| (node.id.clone(), Vec::new())).collect();
    let mut indegree: BTreeMap<String, u32> = snapshot.nodes.iter().map(|node| (node.id.clone(), 0u32)).collect();
    for edge in &snapshot.edges {
        let (source_id, target_id) = (&edge.from.node, &edge.to.node);
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
    SemioFlowTopology { topo_order, depth, cycle_free, node_count }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
