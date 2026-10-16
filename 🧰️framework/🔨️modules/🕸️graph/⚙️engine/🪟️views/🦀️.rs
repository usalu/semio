//! 🧩️ Graph views: subgraph, edge-subgraph, reversed, filtered and undirected adapters over any `GraphView`.

use super::*;

// #region 🔖️Views
/// 🪟️ Read-only borrowed views over any `GraphView`. Deliberately excluded: NetworkX's mutable attribute-sharing views (`G.subgraph()` et al. alias the parent's attribute dicts) — that aliasing pattern doesn't fit Rust ownership, so every view here only ever borrows. Callers who need an owned, mutated copy build one explicitly (a `.copy()`-style constructor lives on the per-kind facade crates from a later wave); these types just leave that seam open.
///
/// 🔎️ Restricts a graph to a node subset; an edge is included only when both endpoints are in the subset.
pub struct SubgraphView<'g, G: GraphView> {
    pub(crate) graph: &'g G,
    pub(crate) nodes: BTreeSet<NodeId>,
}

impl<'g, G: GraphView> SubgraphView<'g, G> {
    pub fn new(graph: &'g G, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let mut kept = BTreeSet::new();
        for n in nodes {
            if graph.contains_node(n) {
                kept.insert(n);
            }
        }
        Self { graph, nodes: kept }
    }
}

impl<'g, G: GraphView> GraphView for SubgraphView<'g, G> {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.iter().copied()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }
    fn edge_count(&self) -> usize {
        self.edges().count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().filter(|e| self.nodes.contains(&e.u) && self.nodes.contains(&e.v))
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.neighbors(node).filter(|n| self.nodes.contains(n))
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node).filter(|n| self.nodes.contains(n))
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.in_neighbors(node).filter(|n| self.nodes.contains(n))
    }
    fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            let mut total = 0usize;
            for nb in self.in_neighbors(node) {
                total += self.edges_between(nb, node).count();
            }
            total
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        let keep = self.nodes.contains(&u) && self.nodes.contains(&v);
        self.graph.edges_between(u, v).filter(move |_| keep)
    }
}

impl<'g, G: GraphView + AttrView> AttrView for SubgraphView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if self.nodes.contains(&node) {
            self.graph.node_attrs(node)
        } else {
            None
        }
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// 🔎️ Restricts a graph to an edge subset; nodes are exactly the endpoints of the included edges.
pub struct EdgeSubgraphView<'g, G: GraphView> {
    pub(crate) graph: &'g G,
    pub(crate) edges: BTreeSet<EdgeId>,
    pub(crate) nodes: BTreeSet<NodeId>,
}

impl<'g, G: GraphView> EdgeSubgraphView<'g, G> {
    pub fn new(graph: &'g G, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let edge_set: BTreeSet<EdgeId> = edges.into_iter().collect();
        let mut nodes = BTreeSet::new();
        for e in graph.edges() {
            if edge_set.contains(&e.id) {
                nodes.insert(e.u);
                nodes.insert(e.v);
            }
        }
        Self { graph, edges: edge_set, nodes }
    }
}

impl<'g, G: GraphView> GraphView for EdgeSubgraphView<'g, G> {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.iter().copied()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }
    fn edge_count(&self) -> usize {
        self.edges.len()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().filter(|e| self.edges.contains(&e.id))
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let directed = self.graph.is_directed();
        self.edges()
            .filter_map(move |e| {
                if e.u == node {
                    Some(e.v)
                } else if !directed && e.v == node {
                    Some(e.u)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        if self.graph.is_directed() {
            self.edges().filter_map(move |e| if e.v == node { Some(e.u) } else { None }).collect::<BTreeSet<_>>().into_iter()
        } else {
            self.out_neighbors(node).collect::<BTreeSet<_>>().into_iter()
        }
    }
    fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            let mut total = 0usize;
            for nb in self.in_neighbors(node) {
                total += self.edges_between(nb, node).count();
            }
            total
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(u, v).filter(|e| self.edges.contains(&e.id))
    }
}

impl<'g, G: GraphView + AttrView> AttrView for EdgeSubgraphView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if self.nodes.contains(&node) {
            self.graph.node_attrs(node)
        } else {
            None
        }
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        if self.edges.contains(&edge) {
            self.graph.edge_attrs(edge)
        } else {
            None
        }
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// ↩️ Swaps successors and predecessors; only meaningful when the wrapped view is directed — on an undirected view this is a documented no-operation (not a panic), since successors already equal predecessors there.
pub struct ReversedView<'g, G: GraphView> {
    pub(crate) graph: &'g G,
}

impl<'g, G: GraphView> ReversedView<'g, G> {
    pub fn new(graph: &'g G) -> Self {
        Self { graph }
    }
}

impl<'g, G: GraphView> GraphView for ReversedView<'g, G> {
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().map(|e| EdgeRef { id: e.id, u: e.v, v: e.u })
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.in_neighbors(node)
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node)
    }
    fn degree(&self, node: NodeId) -> usize {
        self.graph.degree(node)
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.graph.in_degree(node)
    }
    fn in_degree(&self, node: NodeId) -> usize {
        self.graph.out_degree(node)
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(v, u).map(|e| EdgeRef { id: e.id, u: e.v, v: e.u })
    }
}

impl<'g, G: GraphView + AttrView> AttrView for ReversedView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.graph.node_attrs(node)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// 🎛️ NetworkX `restricted_view`/`hide_nodes`/`hide_edges` equivalent: predicates return `true` to KEEP an element, so a "hide" caller just inverts its predicate.
pub struct FilteredView<'g, G: GraphView, FN, FE> {
    pub(crate) graph: &'g G,
    keep_node: FN,
    keep_edge: FE,
}

impl<'g, G: GraphView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> FilteredView<'g, G, FN, FE> {
    pub fn new(graph: &'g G, keep_node: FN, keep_edge: FE) -> Self {
        Self { graph, keep_node, keep_edge }
    }

    fn keep(&self, edge: EdgeRef) -> bool {
        (self.keep_node)(edge.u) && (self.keep_node)(edge.v) && (self.keep_edge)(edge)
    }
}

impl<'g, G: GraphView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> GraphView for FilteredView<'g, G, FN, FE> {
    fn node_count(&self) -> usize {
        self.nodes().count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes().filter(|&n| (self.keep_node)(n))
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node) && (self.keep_node)(node)
    }
    fn edge_count(&self) -> usize {
        self.edges().count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        // 🔀️ Rewritten from `.filter(move |&e| self.keep(e))` — `keep` is async and cannot be
        // called inside a sync closure (R10 residue shape #1).
        let mut out = Vec::new();
        for e in self.graph.edges() {
            if self.keep(e) {
                out.push(e);
            }
        }
        out.into_iter()
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        // 🔀️ Rewritten — `edges_between` is async and cannot be called inside the sync `.filter`
        // predicate that used to guard this (R10 residue shape #1).
        let node_ok = (self.keep_node)(node);
        let mut out = Vec::new();
        if node_ok {
            for nb in self.graph.out_neighbors(node) {
                if self.edges_between(node, nb).next().is_some() {
                    out.push(nb);
                }
            }
        }
        out.into_iter()
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let node_ok = (self.keep_node)(node);
        let mut out = Vec::new();
        if node_ok {
            for nb in self.graph.in_neighbors(node) {
                if self.edges_between(nb, node).next().is_some() {
                    out.push(nb);
                }
            }
        }
        out.into_iter()
    }
    fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            let mut total = 0usize;
            for nb in self.in_neighbors(node) {
                total += self.edges_between(nb, node).count();
            }
            total
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        let keep_u = (self.keep_node)(u);
        let keep_v = (self.keep_node)(v);
        self.graph.edges_between(u, v).filter(move |&e| keep_u && keep_v && (self.keep_edge)(e))
    }
}

impl<'g, G: GraphView + AttrView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> AttrView for FilteredView<'g, G, FN, FE> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if (self.keep_node)(node) {
            self.graph.node_attrs(node)
        } else {
            None
        }
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}

/// 🔀️ Presents a directed graph's edges as undirected — merges successor and predecessor sets into one neighbor view without materializing storage. Querying `edges_between(u, u)` on a directed self-loop yields it twice, mirroring the same "self-loop counts twice" convention `Storage` applies natively to undirected adjacency.
pub struct UndirectedView<'g, G: GraphView> {
    pub(crate) graph: &'g G,
}

impl<'g, G: GraphView> UndirectedView<'g, G> {
    pub fn new(graph: &'g G) -> Self {
        Self { graph }
    }
}

impl<'g, G: GraphView> GraphView for UndirectedView<'g, G> {
    fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().map(|e| if e.u <= e.v { e } else { EdgeRef { id: e.id, u: e.v, v: e.u } })
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node).chain(self.graph.in_neighbors(node)).collect::<BTreeSet<_>>().into_iter()
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.neighbors(node)
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.neighbors(node)
    }
    fn degree(&self, node: NodeId) -> usize {
        self.out_degree(node)
    }
    fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.neighbors(node) {
            total += self.edges_between(node, nb).count();
        }
        total
    }
    fn in_degree(&self, node: NodeId) -> usize {
        self.out_degree(node)
    }
    fn is_directed(&self) -> bool {
        false
    }
    fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(u, v).chain(self.graph.edges_between(v, u))
    }
}

impl<'g, G: GraphView + AttrView> AttrView for UndirectedView<'g, G> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.graph.node_attrs(node)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs()
    }
}
// #endregion 🔖️Views
