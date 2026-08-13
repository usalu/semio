//! 🧭 `topology` — one named inference: execution-order topology stats derived from the DAG's
//! own node/edge graph (topological order, per-node longest-path depth, cycle-freedom, node count).

use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

//#region 🔖️Topology
/// 🧭 Whole-snapshot topology summary — a plain scalar inference (no per-entity `InferredField`
/// caching: recomputing a full topological sort over the node/edge graph on every read is cheap
/// at pilot scale, and the graph has no natural per-entity dependency-hash boundary the way
/// puzzle3d's flatten chain does).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;

    fn node(id: &str) -> DagNodeSpec {
        DagNodeSpec { id: id.into(), ..Default::default() }
    }

    fn edge(id: &str, source: &str, target: &str) -> DagFixtureEdge {
        DagFixtureEdge { id: id.into(), source: source.into(), target: target.into(), ..Default::default() }
    }

    #[test]
    fn linear_chain_orders_roots_before_leaves_with_increasing_depth() {
        let nodes = vec![node("a"), node("b"), node("c")];
        let edges = vec![edge("e1", "a", "b"), edge("e2", "b", "c")];
        let topology = compute_dag_topology(&nodes, &edges);
        assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(topology.depth.get("a"), Some(&0));
        assert_eq!(topology.depth.get("b"), Some(&1));
        assert_eq!(topology.depth.get("c"), Some(&2));
        assert!(topology.cycle_free);
        assert_eq!(topology.node_count, 3);
    }

    #[test]
    fn a_cycle_is_reported_as_not_cycle_free_but_still_totals_every_node() {
        let nodes = vec![node("a"), node("b")];
        let edges = vec![edge("e1", "a", "b"), edge("e2", "b", "a")];
        let topology = compute_dag_topology(&nodes, &edges);
        assert!(!topology.cycle_free);
        assert_eq!(topology.topo_order.len(), 2);
        assert_eq!(topology.node_count, 2);
    }

    #[test]
    fn dangling_edge_endpoints_are_ignored() {
        let nodes = vec![node("a")];
        let edges = vec![edge("e1", "a", "missing")];
        let topology = compute_dag_topology(&nodes, &edges);
        assert!(topology.cycle_free);
        assert_eq!(topology.topo_order, vec!["a".to_string()]);
    }
}
//#endregion 🧪️Tests
