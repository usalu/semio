//! 🔗️ Connectivity and reachability constraints, checked exactly at completion via a small
//! hand-rolled union-find (`mathematical_graph`'s own union-find lives in a private region of that
//! crate, so this crate owns a minimal one rather than reaching into it).

use crate::bitset::PatternSet;
use crate::constraint::{AdjacencyView, Constraint, Exactness, PatternSelector};
use crate::domain::DomainStore;
use crate::error::ConstraintError;
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::weights::WeightTable;

// #region 🔖️UnionFind
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
    }
}
// #endregion 🔖️UnionFind

// #region 🔖️Connectivity
/// 🔗️ Requires that every node whose assigned pattern matches `selector` forms exactly one
/// connected component (using the solver's own adjacency — two selected nodes are connected iff
/// there is a path between them through other selected nodes).
#[derive(Clone, Debug)]
pub struct ConnectivityConstraint {
    pub selector: PatternSelector,
    model: CompiledModel,
}

impl ConnectivityConstraint {
    pub fn new(model: CompiledModel, selector: PatternSelector) -> Self {
        Self { selector, model }
    }
}

impl Constraint for ConnectivityConstraint {
    fn name(&self) -> &'static str {
        "connectivity"
    }

    fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    fn initialize(&self, _domains: &DomainStore, _weights: &WeightTable, _adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        Ok(Vec::new())
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        let selected: Vec<usize> = (0..assignment.len()).filter(|&n| self.selector.matches(&self.model, assignment[n])).collect();
        if selected.len() <= 1 {
            return Ok(());
        }
        let mut uf = UnionFind::new(assignment.len());
        for &n in &selected {
            for &m in adjacency.neighbors(NodeId::from_index(n)) {
                if self.selector.matches(&self.model, assignment[m.index()]) {
                    uf.union(n, m.index());
                }
            }
        }
        let root = uf.find(selected[0]);
        if selected.iter().all(|&n| uf.find(n) == root) {
            Ok(())
        } else {
            Err(format!("connectivity constraint: {} selected nodes do not form one connected component", selected.len()))
        }
    }
}
// #endregion 🔖️Connectivity

// #region 🔖️Reachability
/// 🔗️ Requires that every node in `to` is reachable from every node in `from`, moving only through
/// nodes whose assigned pattern matches `selector` (endpoints themselves must also match).
#[derive(Clone, Debug)]
pub struct ReachabilityConstraint {
    pub from: Vec<NodeId>,
    pub to: Vec<NodeId>,
    pub selector: PatternSelector,
    model: CompiledModel,
}

impl ReachabilityConstraint {
    pub fn new(model: CompiledModel, from: Vec<NodeId>, to: Vec<NodeId>, selector: PatternSelector) -> Self {
        Self { from, to, selector, model }
    }
}

impl Constraint for ReachabilityConstraint {
    fn name(&self) -> &'static str {
        "reachability"
    }

    fn exactness(&self) -> Exactness {
        Exactness::Exact
    }

    fn initialize(&self, _domains: &DomainStore, _weights: &WeightTable, _adjacency: &AdjacencyView) -> Result<Vec<(NodeId, PatternSet)>, ConstraintError> {
        Ok(Vec::new())
    }

    fn validate_complete(&self, assignment: &[PatternId], adjacency: &AdjacencyView) -> Result<(), String> {
        for &start in &self.from {
            if !self.selector.matches(&self.model, assignment[start.index()]) {
                return Err(format!("reachability constraint: source node {start} is not itself selected"));
            }
            let mut visited = vec![false; assignment.len()];
            let mut stack = vec![start];
            visited[start.index()] = true;
            while let Some(n) = stack.pop() {
                for &m in adjacency.neighbors(n) {
                    if !visited[m.index()] && self.selector.matches(&self.model, assignment[m.index()]) {
                        visited[m.index()] = true;
                        stack.push(m);
                    }
                }
            }
            for &target in &self.to {
                if !visited[target.index()] {
                    return Err(format!("reachability constraint: {target} is not reachable from {start}"));
                }
            }
        }
        Ok(())
    }
}
// #endregion 🔖️Reachability

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
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

    /// A 5-node "H" shape: 0-1-2, 2-3, 2-4 (so node2 is a semio_hub).
    fn hub_adjacency() -> AdjacencyView {
        let edges = [(0, 1), (1, 2), (2, 3), (2, 4)];
        let mut neighbors = vec![Vec::new(); 5];
        for &(a, b) in &edges {
            neighbors[a].push(NodeId::from_index(b));
            neighbors[b].push(NodeId::from_index(a));
        }
        AdjacencyView::new(neighbors, vec![crate::ids::RegionId(0); 5])
    }

    #[test]
    fn connectivity_accepts_single_connected_component() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
        // floor at 0,1,2 (connected through the semio_hub), wall elsewhere.
        let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(1), PatternId(1)];
        assert!(c.validate_complete(&assignment, &adjacency).is_ok());
    }

    #[test]
    fn connectivity_rejects_split_components() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
        // floor at 0 and at 3+4, split by wall at the semio_hub (node2) and node1.
        let assignment = vec![PatternId(0), PatternId(1), PatternId(1), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&assignment, &adjacency).is_err());
    }

    #[test]
    fn connectivity_trivially_accepts_zero_or_one_selected() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
        let all_wall = vec![PatternId(1); 5];
        assert!(c.validate_complete(&all_wall, &adjacency).is_ok());
    }

    #[test]
    fn reachability_accepts_connected_path() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ReachabilityConstraint::new(model, vec![NodeId(0)], vec![NodeId(3), NodeId(4)], PatternSelector::Pattern(PatternId(0)));
        let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&assignment, &adjacency).is_ok());
    }

    #[test]
    fn reachability_rejects_blocked_path() {
        let model = floor_wall_model();
        let adjacency = hub_adjacency();
        let c = ReachabilityConstraint::new(model, vec![NodeId(0)], vec![NodeId(3)], PatternSelector::Pattern(PatternId(0)));
        // Wall at the semio_hub (node2) blocks the only path from 0 to 3.
        let assignment = vec![PatternId(0), PatternId(0), PatternId(1), PatternId(0), PatternId(0)];
        assert!(c.validate_complete(&assignment, &adjacency).is_err());
    }
}
// #endregion 🔖️Tests
