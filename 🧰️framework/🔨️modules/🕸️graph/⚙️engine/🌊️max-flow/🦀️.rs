//! 🧩️ Graph max-flow: the flow network over index-based edges.

use super::*;

// #region 🔖️MaxFlow
/// 🎚️ Residual-capacity noise guard: capacities at or below this are treated as exhausted (fractional alpha-expansion graph-cut costs are not exact).
const FLOW_EPS: f64 = 1e-9;

/// 🌊️ Directed residual-graph edge; its paired reverse edge always lives at the adjacent arena slot (`id ^ 1`).
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct FlowEdge {
    to: u32,
    capacity: f64,
}

/// 🌊️ Capacitated directed flow network on a `u32`-indexed arena, for [Dinic's algorithm](https://doi.org/10.1016/0898-1221(74)90074-0) (CLRS ch. 26). Edges are stored as forward/reverse residual pairs at adjacent slots so augmenting a path only ever touches two `Vec` entries; adjacency is `Vec<Vec<u32>>`, never a hash map, so traversal order — and therefore `min_cut`'s result — is fixed by construction order alone.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive, see `FlowEdge` above.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct FlowNetwork {
    node_count: u32,
    pub(crate) edges: Vec<FlowEdge>,
    adjacency: Vec<Vec<u32>>,
}

impl FlowNetwork {
    /// 🆕️ Empty network over nodes `0..node_count`, no edges yet.
    pub fn new(node_count: u32) -> Self {
        Self { node_count, edges: Vec::new(), adjacency: vec![Vec::new(); node_count as usize] }
    }

    /// ➕️ Adds a directed edge `from -> to` with `capacity`, plus a zero-capacity reverse residual edge; returns the forward edge's id (its reverse is always `id ^ 1`).
    pub fn add_edge(&mut self, from: u32, to: u32, capacity: f64) -> u32 {
        let forward_id = self.edges.len() as u32;
        self.edges.push(FlowEdge { to, capacity });
        self.adjacency[from as usize].push(forward_id);
        let reverse_id = self.edges.len() as u32;
        self.edges.push(FlowEdge { to: from, capacity: 0.0 });
        self.adjacency[to as usize].push(reverse_id);
        forward_id
    }

    /// 🌊️ BFS level graph from `source`, restricted to edges with residual capacity above `FLOW_EPS`; `None` marks nodes unreached this phase.
    fn bfs_levels(&self, source: u32) -> Vec<Option<u32>> {
        let mut level = vec![None; self.node_count as usize];
        level[source as usize] = Some(0);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            let du = level[u as usize].expect("every queued node has a level assigned before being pushed");
            for &edge_id in &self.adjacency[u as usize] {
                let edge = self.edges[edge_id as usize];
                if edge.capacity > FLOW_EPS && level[edge.to as usize].is_none() {
                    level[edge.to as usize] = Some(du + 1);
                    queue.push_back(edge.to);
                }
            }
        }
        level
    }

    /// 🌊️ DFS blocking flow along the level graph; `cursor` is the current-arc optimization, skipping adjacency entries already exhausted within this blocking-flow phase.
    fn dfs_blocking_flow(&mut self, u: u32, sink: u32, pushed: f64, level: &[Option<u32>], cursor: &mut [usize]) -> f64 {
        if u == sink || pushed <= FLOW_EPS {
            return pushed;
        }
        while cursor[u as usize] < self.adjacency[u as usize].len() {
            let edge_id = self.adjacency[u as usize][cursor[u as usize]];
            let edge = self.edges[edge_id as usize];
            let advances = edge.capacity > FLOW_EPS && level[edge.to as usize] == level[u as usize].map(|l| l + 1);
            if advances {
                let sent = self.dfs_blocking_flow(edge.to, sink, pushed.min(edge.capacity), level, cursor);
                if sent > FLOW_EPS {
                    self.edges[edge_id as usize].capacity -= sent;
                    self.edges[(edge_id ^ 1) as usize].capacity += sent;
                    return sent;
                }
            }
            cursor[u as usize] += 1;
        }
        0.0
    }

    /// 🏔️ Dinic's max flow: alternates BFS level-graph construction with DFS blocking-flow phases (current-arc optimized) until `sink` is unreachable from `source` in the residual graph; returns the total flow value pushed. `source == sink` short-circuits to `0.0`.
    pub fn max_flow(&mut self, source: u32, sink: u32) -> f64 {
        if source == sink {
            return 0.0;
        }
        let mut total = 0.0;
        loop {
            // 🚨️ was a dropped-future no-op: `bfs_levels` was never awaited, so `max_flow` never
            // actually built a level graph — the whole Dinic's-algorithm loop was silently inert.
            let level = self.bfs_levels(source);
            if level[sink as usize].is_none() {
                break;
            }
            let mut cursor = vec![0usize; self.node_count as usize];
            loop {
                let pushed = self.dfs_blocking_flow(source, sink, f64::INFINITY, &level, &mut cursor);
                if pushed <= FLOW_EPS {
                    break;
                }
                total += pushed;
            }
        }
        total
    }

    /// ✂️ Source side of the minimum cut, valid only after `max_flow` has run: nodes reachable from `source` over edges whose residual capacity still exceeds `FLOW_EPS`, visited in ascending id order via `Vec`-backed BFS — fully deterministic.
    pub fn min_cut(&self, source: u32) -> Vec<u32> {
        let mut reachable = vec![false; self.node_count as usize];
        reachable[source as usize] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            for &edge_id in &self.adjacency[u as usize] {
                let edge = self.edges[edge_id as usize];
                if edge.capacity > FLOW_EPS && !reachable[edge.to as usize] {
                    reachable[edge.to as usize] = true;
                    queue.push_back(edge.to);
                }
            }
        }
        (0..self.node_count).filter(|&i| reachable[i as usize]).collect()
    }
}
// #endregion 🔖️MaxFlow
