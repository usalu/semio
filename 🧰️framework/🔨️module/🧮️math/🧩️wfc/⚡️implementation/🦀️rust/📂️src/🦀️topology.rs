//! 🗺️ The shared topology abstraction every kernel routine (`propagate`, `search`) is generic
//! over, plus [`GraphTopology`] — the CSR-backed arbitrary-graph implementation and semantic
//! reference for `Grid2dTopology`/`Grid3dTopology` (added in later phases). `Topology` is
//! `pub(crate)`: not public API, free to evolve, and never boxed as `dyn` — internal-iteration
//! methods take `impl FnMut` so every implementor's arc loop inlines into pure index arithmetic
//! (grids) or a CSR slice walk (graphs) with zero adjacency storage for grids and zero indirect
//! calls in the propagation hot path.

use crate::ids::{NodeId, RegionId, RelationId};

// #region 🔖️Trait
/// 🗺️ What the kernel needs from any topology: how many variables, how they connect, and where
/// each one's incoming arcs live in a dense per-arc-slot indexing scheme (used by AC-4's support
/// counters). Implementors are always monomorphized into the kernel — never called through `dyn`.
#[allow(dead_code)] // arc_count/region_of/for_each_in_arc/max_in_degree are consumed starting
// with the AC-4 propagator (P6) and region-scoped constraints (P7); the trait's full shape is
// fixed now so those phases never need to touch this sealed boundary.
pub(crate) trait Topology {
    fn node_count(&self) -> usize;
    fn arc_count(&self) -> usize;
    fn region_of(&self, n: NodeId) -> RegionId;
    /// 🗺️ Calls `f(target, relation)` once per outgoing arc of `n`, in a stable order.
    fn for_each_out_arc(&self, n: NodeId, f: impl FnMut(NodeId, RelationId));
    /// 🗺️ Calls `f(source, relation, slot)` once per incoming arc of `n`. `slot` is a dense id
    /// unique to this specific incoming arc, always `< node_count() * max_in_degree()` — AC-4
    /// keys its support counters by it. Bundling the slot into the same callback (rather than a
    /// separate `in_arc_slot(target, ordinal)` lookup) is deliberate: it is the only way to
    /// guarantee the slot a caller records for an arc is the same slot the topology itself means,
    /// since "ordinal" has no meaning independent of how a specific implementor enumerates arcs.
    fn for_each_in_arc(&self, n: NodeId, f: impl FnMut(NodeId, RelationId, usize));
    /// 🗺️ Upper bound on any single node's incoming-arc count, for sizing dense counter tables.
    fn max_in_degree(&self) -> usize;
}
// #endregion 🔖️Trait

// #region 🔖️Graph
/// 🗺️ Arbitrary directed graph topology: CSR-style outgoing and incoming arc storage. Supports
/// multiedges (repeated `(from, to)` under different or identical relations) and self-loops.
#[derive(Clone, Debug)]
#[allow(dead_code)] // in_sources/in_relations/regions back for_each_in_arc/in_arc_slot/region_of,
// unread until AC-4 (P6) and region-scoped constraints (P7) call those trait methods.
pub struct GraphTopology {
    node_count: usize,
    out_starts: Vec<u32>,
    out_targets: Vec<u32>,
    out_relations: Vec<u32>,
    in_starts: Vec<u32>,
    in_sources: Vec<u32>,
    in_relations: Vec<u32>,
    regions: Vec<u32>,
}

impl GraphTopology {
    #[inline]
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    #[inline]
    pub fn arc_count(&self) -> usize {
        self.out_targets.len()
    }

    pub fn out_degree(&self, n: NodeId) -> usize {
        (self.out_starts[n.index() + 1] - self.out_starts[n.index()]) as usize
    }

    pub fn in_degree(&self, n: NodeId) -> usize {
        (self.in_starts[n.index() + 1] - self.in_starts[n.index()]) as usize
    }
}

impl Topology for GraphTopology {
    #[inline]
    fn node_count(&self) -> usize {
        self.node_count
    }

    #[inline]
    fn arc_count(&self) -> usize {
        self.out_targets.len()
    }

    #[inline]
    fn region_of(&self, n: NodeId) -> RegionId {
        RegionId(self.regions[n.index()])
    }

    #[inline]
    fn for_each_out_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId)) {
        let start = self.out_starts[n.index()] as usize;
        let end = self.out_starts[n.index() + 1] as usize;
        for i in start..end {
            f(NodeId(self.out_targets[i]), RelationId(self.out_relations[i]));
        }
    }

    #[inline]
    fn for_each_in_arc(&self, n: NodeId, mut f: impl FnMut(NodeId, RelationId, usize)) {
        let start = self.in_starts[n.index()] as usize;
        let end = self.in_starts[n.index() + 1] as usize;
        for i in start..end {
            f(NodeId(self.in_sources[i]), RelationId(self.in_relations[i]), i);
        }
    }

    fn max_in_degree(&self) -> usize {
        (0..self.node_count).map(|i| (self.in_starts[i + 1] - self.in_starts[i]) as usize).max().unwrap_or(0)
    }
}
// #endregion 🔖️Graph

// #region 🔖️Builder
/// 🏗️ Accumulates directed arcs and per-node regions before [`GraphTopologyBuilder::build`]
/// buckets them into the two CSR arrays [`GraphTopology`] reads.
#[derive(Clone, Debug)]
pub struct GraphTopologyBuilder {
    node_count: usize,
    arcs: Vec<(NodeId, NodeId, RelationId)>,
    regions: Vec<RegionId>,
}

impl GraphTopologyBuilder {
    pub fn new(node_count: usize) -> Self {
        Self { node_count, arcs: Vec::new(), regions: vec![RegionId(0); node_count] }
    }

    pub fn arc(&mut self, from: NodeId, to: NodeId, relation: RelationId) -> &mut Self {
        self.arcs.push((from, to, relation));
        self
    }

    pub fn region(&mut self, n: NodeId, r: RegionId) -> &mut Self {
        self.regions[n.index()] = r;
        self
    }

    pub fn build(self) -> Result<GraphTopology, crate::error::TopologyError> {
        use crate::error::TopologyError;
        for &(from, to, _) in &self.arcs {
            if from.index() >= self.node_count {
                return Err(TopologyError::DanglingArc { from });
            }
            if to.index() >= self.node_count {
                return Err(TopologyError::DanglingArc { from: to });
            }
        }

        let mut out_sorted = self.arcs.clone();
        out_sorted.sort_by_key(|&(from, to, r)| (from.get(), to.get(), r.get()));
        let mut out_starts = vec![0u32; self.node_count + 1];
        for &(from, _, _) in &out_sorted {
            out_starts[from.index() + 1] += 1;
        }
        for i in 0..self.node_count {
            out_starts[i + 1] += out_starts[i];
        }
        let out_targets: Vec<u32> = out_sorted.iter().map(|&(_, to, _)| to.get()).collect();
        let out_relations: Vec<u32> = out_sorted.iter().map(|&(_, _, r)| r.get()).collect();

        let mut in_sorted = self.arcs.clone();
        in_sorted.sort_by_key(|&(from, to, r)| (to.get(), from.get(), r.get()));
        let mut in_starts = vec![0u32; self.node_count + 1];
        for &(_, to, _) in &in_sorted {
            in_starts[to.index() + 1] += 1;
        }
        for i in 0..self.node_count {
            in_starts[i + 1] += in_starts[i];
        }
        let in_sources: Vec<u32> = in_sorted.iter().map(|&(from, _, _)| from.get()).collect();
        let in_relations: Vec<u32> = in_sorted.iter().map(|&(_, _, r)| r.get()).collect();

        let regions: Vec<u32> = self.regions.iter().map(|r| r.get()).collect();

        Ok(GraphTopology { node_count: self.node_count, out_starts, out_targets, out_relations, in_starts, in_sources, in_relations, regions })
    }
}
// #endregion 🔖️Builder

// #region 🔖️FromGraphView
/// 🔁️ Builds a [`GraphTopology`] from any [`mathematical_graph::GraphView`]. Nodes are assigned
/// dense ids in ascending order of their `mathematical_graph::NodeId` (deterministic regardless of
/// the view's internal iteration order). Directed views get one arc per edge via `rel_of`;
/// undirected views get the same relation registered in both directions (the model relation is
/// expected to be self-inverse in that case, matching every other symmetric-adjacency convention
/// in this crate).
pub fn from_graph_view(view: &impl mathematical_graph::GraphView, rel_of: impl Fn(mathematical_graph::EdgeRef) -> RelationId) -> Result<GraphTopology, crate::error::TopologyError> {
    use crate::error::TopologyError;
    let mut sorted_nodes: Vec<mathematical_graph::NodeId> = view.nodes().collect();
    sorted_nodes.sort_unstable();
    if sorted_nodes.len() > u32::MAX as usize {
        return Err(TopologyError::TooManyNodes { count: sorted_nodes.len() as u64 });
    }
    let index_of: std::collections::HashMap<mathematical_graph::NodeId, usize> = sorted_nodes.iter().enumerate().map(|(i, &n)| (n, i)).collect();

    let mut builder = GraphTopologyBuilder::new(sorted_nodes.len());
    for edge in view.edges() {
        let from = NodeId::from_index(index_of[&edge.u]);
        let to = NodeId::from_index(index_of[&edge.v]);
        let r = rel_of(edge);
        builder.arc(from, to, r);
        if !view.is_directed() {
            builder.arc(to, from, r);
        }
    }
    builder.build()
}
// #endregion 🔖️FromGraphView

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_correct_out_and_in_arcs() {
        let mut b = GraphTopologyBuilder::new(3);
        b.arc(NodeId(0), NodeId(1), RelationId(0));
        b.arc(NodeId(1), NodeId(2), RelationId(0));
        b.arc(NodeId(0), NodeId(2), RelationId(1));
        let topo = b.build().unwrap();
        assert_eq!(topo.node_count(), 3);
        assert_eq!(topo.arc_count(), 3);

        let mut out0 = Vec::new();
        topo.for_each_out_arc(NodeId(0), |m, r| out0.push((m, r)));
        assert_eq!(out0, vec![(NodeId(1), RelationId(0)), (NodeId(2), RelationId(1))]);

        let mut in2 = Vec::new();
        topo.for_each_in_arc(NodeId(2), |m, r, _slot| in2.push((m, r)));
        assert_eq!(in2, vec![(NodeId(0), RelationId(1)), (NodeId(1), RelationId(0))]);
    }

    #[test]
    fn self_loops_and_multiedges_are_supported() {
        let mut b = GraphTopologyBuilder::new(2);
        b.arc(NodeId(0), NodeId(0), RelationId(0)); // self-loop
        b.arc(NodeId(0), NodeId(1), RelationId(0));
        b.arc(NodeId(0), NodeId(1), RelationId(1)); // multiedge under a different relation
        let topo = b.build().unwrap();
        assert_eq!(topo.arc_count(), 3);
        assert_eq!(topo.out_degree(NodeId(0)), 3);
        let mut out0 = Vec::new();
        topo.for_each_out_arc(NodeId(0), |m, r| out0.push((m, r)));
        assert!(out0.contains(&(NodeId(0), RelationId(0))));
        assert!(out0.contains(&(NodeId(1), RelationId(0))));
        assert!(out0.contains(&(NodeId(1), RelationId(1))));
    }

    #[test]
    fn dangling_arc_is_rejected() {
        let mut b = GraphTopologyBuilder::new(2);
        b.arc(NodeId(0), NodeId(5), RelationId(0));
        assert!(b.build().is_err());
    }

    #[test]
    fn in_arc_slots_are_dense_and_unique_per_node() {
        let mut b = GraphTopologyBuilder::new(3);
        b.arc(NodeId(0), NodeId(2), RelationId(0));
        b.arc(NodeId(1), NodeId(2), RelationId(0));
        let topo = b.build().unwrap();
        assert_eq!(topo.in_degree(NodeId(2)), 2);
        let mut slots = Vec::new();
        topo.for_each_in_arc(NodeId(2), |_, _, slot| slots.push(slot));
        assert_eq!(slots.len(), 2);
        assert_ne!(slots[0], slots[1]);
        for &slot in &slots {
            assert!(slot < topo.node_count() * topo.max_in_degree());
        }
        assert_eq!(topo.max_in_degree(), 2);
    }

    #[test]
    fn regions_default_to_zero_and_are_settable() {
        let mut b = GraphTopologyBuilder::new(2);
        b.region(NodeId(1), RegionId(7));
        let topo = b.build().unwrap();
        assert_eq!(topo.region_of(NodeId(0)), RegionId(0));
        assert_eq!(topo.region_of(NodeId(1)), RegionId(7));
    }
}
// #endregion 🔖️Tests
