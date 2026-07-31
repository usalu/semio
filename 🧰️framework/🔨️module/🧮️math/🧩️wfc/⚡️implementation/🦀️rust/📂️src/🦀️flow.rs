//! 🚰️ Resource-flow constraint: requires at least `min_flow` edge-disjoint paths (uniform
//! capacity 1 per adjacency edge — per-node/per-pattern-weighted capacities are deferred, see
//! this module's tests for the exact contract) from a source set to a sink set, moving only
//! through nodes whose assigned pattern matches a selector. Computed exactly at completion via a
//! hand-rolled Edmonds-Karp (BFS augmenting-path) max-flow over a virtual super-source/super-sink
//! network — no external graph/flow crate, matching this crate's zero-dependency convention.

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
use crate::domain::DomainStore;
use crate::error::ConstraintError;
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::weights::WeightTable;
use std::collections::VecDeque;

// #region 🔖️Network
/// 🚰️ A minimal adjacency-list max-flow network: paired forward/reverse edges at consecutive
/// indices `(2k, 2k+1)`, so `e ^ 1` always gives an edge's reverse without extra bookkeeping.
struct FlowNetwork {
    adj: Vec<Vec<usize>>,
    to: Vec<usize>,
    cap: Vec<u32>,
}

impl FlowNetwork {
    fn new(n: usize) -> Self {
        Self { adj: vec![Vec::new(); n], to: Vec::new(), cap: Vec::new() }
    }

    fn add_edge(&mut self, u: usize, v: usize, capacity: u32) {
        self.adj[u].push(self.to.len());
        self.to.push(v);
        self.cap.push(capacity);
        self.adj[v].push(self.to.len());
        self.to.push(u);
        self.cap.push(0);
    }

    /// 🚰️ One BFS augmenting-path step: finds the shortest (fewest-edges) path from `s` to `t`
    /// with remaining capacity, pushes the bottleneck amount along it, and returns that amount
    /// (`0` once no augmenting path remains).
    fn bfs_augment(&mut self, s: usize, t: usize) -> u32 {
        let mut parent_edge: Vec<Option<usize>> = vec![None; self.adj.len()];
        let mut visited = vec![false; self.adj.len()];
        visited[s] = true;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        'bfs: while let Some(u) = queue.pop_front() {
            for &e in &self.adj[u] {
                let v = self.to[e];
                if !visited[v] && self.cap[e] > 0 {
                    visited[v] = true;
                    parent_edge[v] = Some(e);
                    if v == t {
                        break 'bfs;
                    }
                    queue.push_back(v);
                }
            }
        }
        if !visited[t] {
            return 0;
        }
        let mut bottleneck = u32::MAX;
        let mut v = t;
        while v != s {
            let e = parent_edge[v].expect("visited[t] guarantees a parent chain back to s");
            bottleneck = bottleneck.min(self.cap[e]);
            v = self.to[e ^ 1];
        }
        v = t;
        while v != s {
            let e = parent_edge[v].expect("same parent chain as above");
            self.cap[e] -= bottleneck;
            self.cap[e ^ 1] += bottleneck;
            v = self.to[e ^ 1];
        }
        bottleneck
    }

    fn max_flow(&mut self, s: usize, t: usize) -> u32 {
        let mut total = 0u32;
        loop {
            let f = self.bfs_augment(s, t);
            if f == 0 {
                return total;
            }
            total += f;
        }
    }
}
// #endregion 🔖️Network

// #region 🔖️Constraint
/// 🚰️ Requires at least `min_flow` edge-disjoint paths from `sources` to `sinks`, through nodes
/// whose assigned pattern matches `selector` (source/sink endpoints must match too).
#[derive(Clone, Debug)]
pub struct FlowConstraint {
    pub selector: PatternSelector,
    pub sources: Vec<NodeId>,
    pub sinks: Vec<NodeId>,
    pub min_flow: u32,
    model: CompiledModel,
}

impl FlowConstraint {
    pub fn new(model: CompiledModel, selector: PatternSelector, sources: Vec<NodeId>, sinks: Vec<NodeId>, min_flow: u32) -> Self {
        Self { selector, sources, sinks, min_flow, model }
    }

    fn compute_max_flow(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> u32 {
        let n = assignment.len();
        let super_source = n;
        let super_sink = n + 1;
        let mut net = FlowNetwork::new(n + 2);

        for i in 0..n {
            if !self.selector.matches(&self.model, assignment[i]) {
                continue;
            }
            for &m in adjacency.neighbors(NodeId::from_index(i)) {
                if self.selector.matches(&self.model, assignment[m.index()]) {
                    net.add_edge(i, m.index(), 1);
                }
            }
        }
        for &s in &self.sources {
            if self.selector.matches(&self.model, assignment[s.index()]) {
                net.add_edge(super_source, s.index(), u32::MAX);
            }
        }
        for &t in &self.sinks {
            if self.selector.matches(&self.model, assignment[t.index()]) {
                net.add_edge(t.index(), super_sink, u32::MAX);
            }
        }
        net.max_flow(super_source, super_sink)
    }
}

impl Constraint for FlowConstraint {
    fn name(&self) -> &'static str {
        "flow"
    }

    fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    fn initialize(&self, _domains: &DomainStore, _weights: &WeightTable, _adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        Ok(Vec::new())
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        let flow = self.compute_max_flow(assignment, adjacency);
        if flow < self.min_flow { Err(format!("flow constraint: max flow {flow} is below the required minimum {}", self.min_flow)) } else { Ok(()) }
    }
}
// #endregion 🔖️Constraint

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::RegionId;
    use crate::model::ModelBuilder;

    fn floor_wall_model() -> CompiledModel {
        let mut b = ModelBuilder::new();
        let floor = b.add_pattern(1.0);
        let wall = b.add_pattern(1.0);
        let r = b.add_relation("adj");
        b.allow_mirrored(r, floor, floor);
        b.allow_mirrored(r, floor, wall);
        b.allow_mirrored(r, wall, wall);
        b.compile().unwrap()
    }

    fn line_adjacency(n: usize) -> AdjacencyView {
        let mut neighbors = vec![Vec::new(); n];
        for i in 0..n.saturating_sub(1) {
            neighbors[i].push(NodeId::from_index(i + 1));
            neighbors[i + 1].push(NodeId::from_index(i));
        }
        AdjacencyView::new(neighbors, vec![RegionId(0); n])
    }

    /// A 2-wide, 3-long ladder: two parallel floor corridors from node0/node1 (side by side) to
    /// node4/node5, letting up to 2 edge-disjoint paths exist when everything is floor.
    fn ladder_adjacency() -> AdjacencyView {
        let edges = [(0, 2), (2, 4), (1, 3), (3, 5)];
        let mut neighbors = vec![Vec::new(); 6];
        for &(a, b) in &edges {
            neighbors[a].push(NodeId::from_index(b));
            neighbors[b].push(NodeId::from_index(a));
        }
        AdjacencyView::new(neighbors, vec![RegionId(0); 6])
    }

    #[test]
    fn single_path_meets_flow_of_one() {
        let model = floor_wall_model();
        let adjacency = line_adjacency(4);
        let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(3)], 1);
        let all_floor = vec![PatternId(0); 4];
        assert!(c.validate_complete(&all_floor, &adjacency).is_ok());
    }

    #[test]
    fn a_wall_blocking_the_only_path_fails_flow_of_one() {
        let model = floor_wall_model();
        let adjacency = line_adjacency(4);
        let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(3)], 1);
        let blocked = vec![PatternId(0), PatternId(0), PatternId(1), PatternId(0)];
        assert!(c.validate_complete(&blocked, &adjacency).is_err());
    }

    #[test]
    fn ladder_with_both_corridors_open_meets_flow_of_two() {
        let model = floor_wall_model();
        let adjacency = ladder_adjacency();
        let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0), NodeId(1)], vec![NodeId(4), NodeId(5)], 2);
        let all_floor = vec![PatternId(0); 6];
        assert!(c.validate_complete(&all_floor, &adjacency).is_ok());
    }

    #[test]
    fn ladder_with_one_corridor_walled_off_fails_flow_of_two() {
        let model = floor_wall_model();
        let adjacency = ladder_adjacency();
        let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0), NodeId(1)], vec![NodeId(4), NodeId(5)], 2);
        // node2 (the middle of the top corridor) is wall: only 1 edge-disjoint path survives.
        let one_corridor = vec![PatternId(0), PatternId(0), PatternId(1), PatternId(0), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&one_corridor, &adjacency).is_err());
    }

    #[test]
    fn source_not_selected_yields_zero_flow() {
        let model = floor_wall_model();
        let adjacency = line_adjacency(3);
        let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(2)], 1);
        let source_is_wall = vec![PatternId(1), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&source_is_wall, &adjacency).is_err());
    }

    #[test]
    fn max_flow_never_exceeds_the_number_of_distinct_source_sink_edges() {
        // A direct single-edge line: max possible flow is 1, regardless of how large min_flow's
        // check demands — this just exercises the network construction terminates and is exact.
        let model = floor_wall_model();
        let adjacency = line_adjacency(2);
        let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(1)], 1);
        let assignment = vec![PatternId(0), PatternId(0)];
        assert_eq!(c.compute_max_flow(&assignment, &adjacency), 1);
    }
}
// #endregion 🔖️Tests
