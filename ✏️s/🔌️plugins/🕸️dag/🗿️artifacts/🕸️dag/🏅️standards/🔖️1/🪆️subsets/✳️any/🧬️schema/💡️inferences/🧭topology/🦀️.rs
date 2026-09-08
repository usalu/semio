//! 🧭 `topology` — one named inference: execution-order topology stats derived from the DAG's
//! own node/edge graph (topological order, per-node longest-path depth, cycle-freedom, node count).

use crate::{DagFixtureEdge, DagNodeSpec};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

//#region 🔖️Topology
/// 🧭 Whole-snapshot topology summary — a plain scalar inference (no per-entity `InferredField`
/// caching: recomputing a full topological sort over the node/edge graph on every read is cheap
/// at pilot scale, and the graph has no natural per-entity dependency-hash boundary the way
/// puzzle3d's flatten chain does).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagTopology {
    pub topo_order: Vec<String>,
    pub depth: BTreeMap<String, u32>,
    pub cycle_free: bool,
    pub node_count: u32,
}

impl Default for DagTopology {
    fn default() -> Self {
        Self { topo_order: Vec::new(), depth: BTreeMap::new(), cycle_free: true, node_count: 0 }
    }
}

/// 🧭 Kahn's algorithm over `nodes`/`edges`, deterministic via `BTreeMap`/sorted-adjacency
/// iteration order; nodes left over after the queue drains (a cycle) are appended in id order so
/// `topo_order` always stays a total permutation of every node id.
pub fn compute_dag_topology(nodes: &[DagNodeSpec], edges: &[DagFixtureEdge]) -> DagTopology {
    let ids: BTreeSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut indegree: BTreeMap<String, u32> = ids.iter().cloned().map(|id| (id, 0)).collect();
    let mut adjacency: BTreeMap<String, Vec<String>> = ids.iter().cloned().map(|id| (id, Vec::new())).collect();
    for edge in edges {
        if ids.contains(&edge.source) && ids.contains(&edge.target) {
            adjacency.get_mut(&edge.source).expect("source tracked in adjacency").push(edge.target.clone());
            *indegree.get_mut(&edge.target).expect("target tracked in indegree") += 1;
        }
    }
    for children in adjacency.values_mut() {
        children.sort();
    }

    let mut remaining = indegree.clone();
    let mut queue: VecDeque<String> = indegree.iter().filter(|(_, &degree)| degree == 0).map(|(id, _)| id.clone()).collect();
    let mut depth: BTreeMap<String, u32> = queue.iter().map(|id| (id.clone(), 0)).collect();
    let mut topo_order = Vec::new();

    while let Some(id) = queue.pop_front() {
        topo_order.push(id.clone());
        let current_depth = *depth.get(&id).unwrap_or(&0);
        if let Some(children) = adjacency.get(&id) {
            for child in children {
                let entry = remaining.get_mut(child).expect("child tracked in remaining");
                *entry -= 1;
                let slot = depth.entry(child.clone()).or_insert(0);
                if current_depth + 1 > *slot {
                    *slot = current_depth + 1;
                }
                if *entry == 0 {
                    queue.push_back(child.clone());
                }
            }
        }
    }

    let cycle_free = topo_order.len() == ids.len();
    if !cycle_free {
        let visited: BTreeSet<String> = topo_order.iter().cloned().collect();
        for id in &ids {
            if !visited.contains(id) {
                depth.entry(id.clone()).or_insert(0);
                topo_order.push(id.clone());
            }
        }
    }

    DagTopology { topo_order, depth, cycle_free, node_count: ids.len() as u32 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
