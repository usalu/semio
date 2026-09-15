//! 🧭 `topology` — one named inference: execution-order topology stats derived from the DAG's
//! own node/edge graph (topological order, per-node longest-path depth, cycle-freedom, node count).

use crate::{DagHostDocumentEdge, DagNodeSpec};
use std::collections::{BTreeMap, VecDeque};

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

/// 🧭 Kahn's algorithm over `nodes`/`edges` (endpoints `node@port` resolve to their node) on dense id-ordered indices,
/// O((V + E) log V): ties break by node id and children are visited in id order; nodes left over after the queue
/// drains (a cycle) are appended in id order so `topo_order` always stays a total permutation of every node id.
pub fn compute_dag_topology<'a>(nodes: &'a [DagNodeSpec], edges: &'a [DagHostDocumentEdge]) -> DagTopology {
    let mut ids: Vec<&str> = nodes.iter().map(|node| node.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    let index = |id: &str| ids.binary_search(&id).ok();
    let mut indegree = vec![0_u32; ids.len()];
    let mut adjacency: Vec<Vec<u32>> = vec![Vec::new(); ids.len()];
    for edge in edges {
        let node_of = |endpoint: &'a str| endpoint.split_once('@').map_or(endpoint, |(node, _)| node);
        if let (Some(source), Some(target)) = (index(node_of(&edge.source)), index(node_of(&edge.target))) {
            adjacency[source].push(target as u32);
            indegree[target] += 1;
        }
    }
    for children in &mut adjacency {
        children.sort_unstable();
    }
    let mut depth = vec![0_u32; ids.len()];
    let mut visited = vec![false; ids.len()];
    let mut queue: VecDeque<u32> = (0..ids.len() as u32).filter(|at| indegree[*at as usize] == 0).collect();
    let mut order: Vec<u32> = Vec::with_capacity(ids.len());
    while let Some(at) = queue.pop_front() {
        order.push(at);
        visited[at as usize] = true;
        for &child in &adjacency[at as usize] {
            let child = child as usize;
            indegree[child] -= 1;
            depth[child] = depth[child].max(depth[at as usize] + 1);
            if indegree[child] == 0 {
                queue.push_back(child as u32);
            }
        }
    }
    let cycle_free = order.len() == ids.len();
    order.extend((0..ids.len() as u32).filter(|at| !visited[*at as usize]));
    DagTopology { topo_order: order.iter().map(|at| ids[*at as usize].to_string()).collect(), depth: ids.iter().zip(&depth).map(|(id, depth)| ((*id).to_string(), *depth)).collect(), cycle_free, node_count: ids.len() as u32 }
}
//#endregion 🔖️Topology

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
