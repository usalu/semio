//! 🧭 `topology` — one named inference: the node/edge graph's topological order, per-node depth,
//! and cycle-freedom, computed by Kahn's algorithm over `nodes`/`edges`. Edge endpoints are
//! `nodeId@portId` port keys (`crate::port_node_id` parses the node id out); a
//! plain whole-snapshot scalar (per the family root's own "simple whole-snapshot scalars"
//! guidance) — no `InferredField`/incremental caching needed for a single BFS pass.

use crate::{port_node_id, JackSnapshot};
use std::collections::{BTreeMap, VecDeque};

//#region 🔖️Topology
/// 🧭 Topological shape of the node/edge graph.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct JackTopology {
    /// 🥇️ Node ids in Kahn topological order — only nodes reachable by repeatedly removing
    /// zero-indegree nodes; nodes stuck in a cycle are omitted (see `cycle_free`).
    pub topo_order: Vec<String>,
    /// 📏️ Longest-path depth from any zero-indegree root, keyed by node id — only defined for
    /// nodes present in `topo_order` (nodes inside a cycle have no well-defined DAG depth).
    pub depth: BTreeMap<String, u32>,
    /// ✅️ Whether every node was reachable by the topological sort (`topo_order.len() == nodeCount`).
    pub cycle_free: bool,
    pub node_count: u32,
}

/// 🩹 Hand-rolled: the naive derive would default `cycle_free` to `false`, but an empty graph
/// (`node_count: 0`) is honestly cycle-free — `compute_topology(&JackSnapshot::default())` must
/// equal `JackTopology::default()` (the inference-default law), so this matches that zero case.
impl Default for JackTopology {
    fn default() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}

/// 📐️ Computes `topology` directly from `nodes`/`edges` via Kahn's algorithm — deterministic
/// because both the root frontier and each frontier's children are drained in node-id sort order.
pub fn compute_topology(snapshot: &JackSnapshot) -> JackTopology {
    let scene = crate::jack_working_scene(snapshot);
    let node_count = scene.nodes.len() as u32;
    let mut adjacency: BTreeMap<String, Vec<String>> = scene.nodes.iter().map(|node| (node.id.clone(), Vec::new())).collect();
    let mut indegree: BTreeMap<String, u32> = scene.nodes.iter().map(|node| (node.id.clone(), 0u32)).collect();
    for edge in &scene.edges {
        let Some(source_id) = port_node_id(&edge.source) else { continue };
        let Some(target_id) = port_node_id(&edge.target) else { continue };
        if !indegree.contains_key(source_id) || !indegree.contains_key(target_id) {
            continue;
        }
        adjacency.get_mut(source_id).expect("known node").push(target_id.to_string());
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
    JackTopology { topo_order, depth, cycle_free, node_count }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
