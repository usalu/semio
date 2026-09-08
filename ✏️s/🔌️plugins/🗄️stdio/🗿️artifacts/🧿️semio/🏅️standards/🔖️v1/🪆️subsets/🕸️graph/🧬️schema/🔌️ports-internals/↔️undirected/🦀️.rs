//! 🔀️ The undirected port (multi-edge) graph family — NetworkX `MultiGraph` parity facade over `graph_core::Storage<Ported, Undirected>`.

use std::collections::{BTreeMap, BTreeSet};

use graph_core::{pairwise, AttrView, AttrWeight, Directed, EdgeId, EdgeRef, EdgeSubgraphView, EdgeWeights, GraphView, HandleId, NodeId, Normal, Ported, PropertyBag, PropertyValue, Storage, SubgraphView, Undirected};

// #region 🔖️Construction
/// 🕸️ NetworkX `MultiGraph` parity facade: an undirected multigraph wrapping `Storage<Ported, Undirected>`. Every node gets a lazily-allocated "default handle" (see `handle_of`) so callers work at plain `NodeId` level, matching NetworkX's handle-free `MultiGraph` API — the `Ported` port model is only an internal storage detail here. `Storage<Ported,_>::add_edge_with` always mints a fresh `EdgeId`, which directly plays the role of NetworkX's per-pair insertion "key" (`G.add_edge(u, v)` returns a key); this is a deliberate simplification since `EdgeId` is globally unique/monotone rather than scoped per node pair like NetworkX's keys.
#[derive(Clone, Debug)]
pub struct PortUndirectedGraph {
    storage: Storage<Ported, Undirected>,
    default_handle: BTreeMap<NodeId, HandleId>,
}

impl Default for PortUndirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl PortUndirectedGraph {
    /// 🆕️ Empty multigraph.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self { storage: Storage::new(), default_handle: BTreeMap::new() }
    }

    /// 🪝️ Looks up, or lazily allocates via `Storage::add_handle`, the single default handle every node routes all of its edges through. Panics if `node` isn't already present in `storage` — every call site uses `ensure_node` first, so this never fires in practice.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn handle_of(&mut self, node: NodeId) -> HandleId {
        match self.default_handle.get(&node) {
            Some(&handle) => handle,
            None => {
                let handle = self.storage.add_handle(node).expect("handle_of is only called after the node is known to exist");
                self.default_handle.insert(node, handle);
                handle
            }
        }
    }

    /// 🌱️ Creates `id` with empty attrs if not already present (NetworkX `MultiGraph.add_edge` auto-creates endpoint nodes).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn ensure_node(&mut self, id: NodeId) {
        if !self.storage.contains_node(id) {
            self.storage.add_node_with_id(id, PropertyBag::default());
        }
    }
}
// #endregion 🔖️Construction

// #region 🔖️NodeOperations
impl PortUndirectedGraph {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_node(&mut self) -> NodeId {
        self.storage.add_node()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_node_with(&mut self, attrs: PropertyBag) -> NodeId {
        self.storage.add_node_with(attrs)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_node_with_id(&mut self, id: NodeId, attrs: PropertyBag) -> NodeId {
        self.storage.add_node_with_id(id, attrs)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        for id in nodes {
            self.add_node_with_id(id, PropertyBag::default());
        }
    }

    /// 🗑️ Removes `id` and cascades (edges, handles) via `Storage::remove_node`, then drops this facade's own `default_handle` bookkeeping entry for it.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        let removed = self.storage.remove_node(id);
        if removed {
            self.default_handle.remove(&id);
        }
        removed
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        for id in nodes {
            self.remove_node(id);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn has_node(&self, id: NodeId) -> bool {
        self.storage.contains_node(id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_nodes(&self) -> usize {
        self.storage.node_count()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn order(&self) -> usize {
        self.storage.node_count()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.storage.nodes()
    }
}
// #endregion 🔖️NodeOperations

// #region 🔖️EdgeOperations
impl PortUndirectedGraph {
    /// ➕️ Always creates a NEW parallel edge (NetworkX `MultiGraph.add_edge`); auto-creates `u`/`v` if unseen.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edge(&mut self, u: NodeId, v: NodeId) -> EdgeId {
        self.add_edge_with(u, v, PropertyBag::default())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edge_with(&mut self, u: NodeId, v: NodeId, attrs: PropertyBag) -> EdgeId {
        self.ensure_node(u);
        self.ensure_node(v);
        let hu = self.handle_of(u);
        let hv = self.handle_of(v);
        self.storage.add_edge_with(hu, hv, attrs)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId)>) -> Vec<EdgeId> {
        edges.into_iter().map(|(u, v)| self.add_edge(u, v)).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_weighted_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId, f64)>) -> Vec<EdgeId> {
        edges
            .into_iter()
            .map(|(u, v, weight)| {
                let mut attrs = PropertyBag::default();
                attrs.insert("weight".to_string(), PropertyValue::Number(weight));
                self.add_edge_with(u, v, attrs)
            })
            .collect()
    }

    /// 🗑️ Keyed removal — NetworkX `remove_edge(u, v, key)`. Here `EdgeId` IS the key (see the crate doc's simplification note), so this facade's `remove_edge` takes just the id rather than the simple-graph facades' `(u, v)`-keyed removal.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_edge(&mut self, id: EdgeId) -> bool {
        self.storage.remove_edge(id)
    }

    /// 🗑️ NetworkX's convenience `remove_edge(u, v)` without a key removes an arbitrary parallel edge; this picks the smallest `EdgeId` for determinism.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_one_edge(&mut self, u: NodeId, v: NodeId) -> bool {
        match self.edges_between(u, v).min() {
            Some(id) => self.storage.remove_edge(id),
            None => false,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn has_edge(&self, u: NodeId, v: NodeId) -> bool {
        self.storage.edges_between(u, v).next().is_some()
    }

    /// 🔢️ Total edge count if both `u`/`v` are `None`; edge count between a specific pair if both are `Some` (the mixed one-`Some` NetworkX form is unsupported here — it falls back to the total).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_edges(&self, u: Option<NodeId>, v: Option<NodeId>) -> usize {
        match (u, v) {
            (Some(u), Some(v)) => self.storage.edges_between(u, v).count(),
            _ => self.storage.edge_count(),
        }
    }

    /// 🏷️ Keyed attribute lookup — several edges can share a `(u, v)` pair, so lookup is always by `EdgeId`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_edge_data(&self, id: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeId> + '_ {
        self.storage.edges_between(u, v).map(|e| e.id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_path(&mut self, nodes: impl IntoIterator<Item = NodeId>) -> Vec<EdgeId> {
        let nodes: Vec<NodeId> = nodes.into_iter().collect();
        pairwise(&nodes).map(|(a, b)| self.add_edge(a, b)).collect()
    }

    /// 🔁️ Mirrors NetworkX `add_cycle`: a single node produces one self-loop, an empty input produces no edges, everything else wraps the last node back to the first.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_cycle(&mut self, nodes: impl IntoIterator<Item = NodeId>) -> Vec<EdgeId> {
        let nodes: Vec<NodeId> = nodes.into_iter().collect();
        match nodes.len() {
            0 => Vec::new(),
            1 => vec![self.add_edge(nodes[0], nodes[0])],
            _ => {
                let mut ids: Vec<EdgeId> = pairwise(&nodes).map(|(a, b)| self.add_edge(a, b)).collect();
                ids.push(self.add_edge(nodes[nodes.len() - 1], nodes[0]));
                ids
            }
        }
    }

    /// ⭐️ First node is the semio_hub, the rest are leaves connected to it (NetworkX `add_star`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_star(&mut self, nodes: impl IntoIterator<Item = NodeId>) -> Vec<EdgeId> {
        let nodes: Vec<NodeId> = nodes.into_iter().collect();
        let Some(&semio_hub) = nodes.first() else { return Vec::new() };
        nodes[1..].iter().map(|&leaf| self.add_edge(semio_hub, leaf)).collect()
    }
}
// #endregion 🔖️EdgeOperations

// #region 🔖️Queries
impl PortUndirectedGraph {
    /// 🎯️ Distinct neighbor node ids; a node with several parallel edges to the same neighbor still yields it once (`Storage`'s adjacency map is already keyed by neighbor id, so this is deterministic and deduplicated for free).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.storage.neighbors(node)
    }

    /// 🔢️ Counts every parallel edge; a self-loop counts twice (delegates straight to `Storage::degree`, which already encodes both conventions).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn degree(&self, node: NodeId) -> usize {
        self.storage.degree(node)
    }

    /// ⚖️ Sums the named numeric attribute (default `1.0` per edge) over every incident edge, counting a self-loop's weight twice like `degree`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn weighted_degree(&self, node: NodeId, weight_name: &str) -> f64 {
        let weight = AttrWeight { graph: &self.storage, name: weight_name, default: 1.0 };
        let mut total = 0.0;
        for neighbor in self.storage.neighbors(node) {
            for edge in self.storage.edges_between(node, neighbor) {
                total += weight.weight(edge);
            }
        }
        total
    }

    /// 📐️ `2*m/(n*(n-1))` using the multi-edge count `m`, matching NetworkX's `density` formula for `MultiGraph` — deliberately NOT clamped to `1.0`, since parallel edges can push it arbitrarily high.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn density(&self) -> f64 {
        let n = self.storage.node_count() as f64;
        let m = self.storage.edge_count() as f64;
        if n <= 1.0 {
            0.0
        } else {
            2.0 * m / (n * (n - 1.0))
        }
    }

    /// 🕳️ NetworkX `is_empty`: true when the graph has no EDGES (nodes may still be present).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.storage.edge_count() == 0
    }
}
// #endregion 🔖️Queries

// #region 🔖️Transforms
impl PortUndirectedGraph {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// 🔎️ Owned copy restricted to `nodes`; rebuilt via a `SubgraphView` over `self`, so it gets fresh `NodeId`-stable but new `EdgeId`s (see the core `Views` module doc on why views never mutate in place).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn subgraph(&self, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let view = SubgraphView::new(&self.storage, nodes);
        let mut out = Self::new();
        for node in view.nodes() {
            let attrs = view.node_attrs(node).cloned().unwrap_or_default();
            out.storage.add_node_with_id(node, attrs);
        }
        for edge in view.edges() {
            let attrs = view.edge_attrs(edge.id).cloned().unwrap_or_default();
            out.add_edge_with(edge.u, edge.v, attrs);
        }
        out.storage.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        out
    }

    /// 🔎️ Owned copy restricted to `edges`; nodes are exactly those edges' endpoints (`EdgeSubgraphView`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn edge_subgraph(&self, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let view = EdgeSubgraphView::new(&self.storage, edges);
        let mut out = Self::new();
        for node in view.nodes() {
            let attrs = view.node_attrs(node).cloned().unwrap_or_default();
            out.storage.add_node_with_id(node, attrs);
        }
        for edge in view.edges() {
            let attrs = view.edge_attrs(edge.id).cloned().unwrap_or_default();
            out.add_edge_with(edge.u, edge.v, attrs);
        }
        out.storage.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        out
    }

    /// 🧵️ Collapses parallel edges into one simple edge per node pair, summing each collapsed group's `"weight"` attribute (missing weight defaults to `1.0` per parallel edge, matching `EdgeWeights`'s convention). Returns the raw `Storage`, not a sibling facade type, to avoid a circular dependency on the not-yet-built `graph_core::normal::undirected` crate — not a literal NetworkX method, but a common multigraph-to-simple-graph convenience.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_simple(&self) -> Storage<Normal, Undirected> {
        let mut simple = Storage::<Normal, Undirected>::new();
        for node in self.storage.nodes() {
            let attrs = self.storage.node_attrs(node).cloned().unwrap_or_default();
            simple.add_node_with_id(node, attrs);
        }
        let mut weight_sums: BTreeMap<(NodeId, NodeId), f64> = BTreeMap::new();
        for edge in self.storage.edges() {
            let pair = if edge.u <= edge.v { (edge.u, edge.v) } else { (edge.v, edge.u) };
            let weight = self.storage.edge_attrs(edge.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64).unwrap_or(1.0);
            *weight_sums.entry(pair).or_insert(0.0) += weight;
        }
        for ((u, v), weight) in weight_sums {
            let mut attrs = PropertyBag::default();
            attrs.insert("weight".to_string(), PropertyValue::Number(weight));
            simple.add_edge_with(u, v, attrs);
        }
        simple.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        simple
    }

    /// ➡️ Each undirected parallel edge becomes two directed parallel edges (one each way), attributes cloned onto both; an undirected self-loop becomes exactly ONE directed self-loop, since NetworkX's own `to_directed` walks the (already-collapsed) adjacency structure where a self-loop appears only once. Builds a fresh handle-bookkeeping map for the new `Directed` storage — the old `default_handle` handle ids belong to a different `Storage` instance and can't be reused.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_directed(&self) -> Storage<Ported, Directed> {
        let mut directed = Storage::<Ported, Directed>::new();
        for node in self.storage.nodes() {
            let attrs = self.storage.node_attrs(node).cloned().unwrap_or_default();
            directed.add_node_with_id(node, attrs);
        }
        let mut handles: BTreeMap<NodeId, HandleId> = BTreeMap::new();
        for edge in self.storage.edges() {
            let attrs = self.storage.edge_attrs(edge.id).cloned().unwrap_or_default();
            let hu = *handles.entry(edge.u).or_insert_with(|| directed.add_handle(edge.u).expect("node added above"));
            let hv = *handles.entry(edge.v).or_insert_with(|| directed.add_handle(edge.v).expect("node added above"));
            directed.add_edge_with(hu, hv, attrs.clone());
            if edge.u != edge.v {
                directed.add_edge_with(hv, hu, attrs);
            }
        }
        directed.graph_attrs_mut().extend(self.storage.graph_attrs().clone());
        directed
    }

    /// 🧹️ Removes every node, edge, and handle; also drops this facade's `default_handle` bookkeeping.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn clear(&mut self) {
        self.storage.clear();
        self.default_handle.clear();
    }

    /// 🧹️ Removes every edge but keeps nodes AND their handles — `default_handle` is intentionally left untouched, since handles are anchored on nodes, not edges, and only get dropped when their owning node is removed.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn clear_edges(&mut self) {
        self.storage.clear_edges();
    }
}
// #endregion 🔖️Transforms

// #region 🔖️Attributes
impl PortUndirectedGraph {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_node_attributes(&mut self, node: NodeId, attrs: PropertyBag) {
        if let Some(bag) = self.storage.node_attrs_mut(node) {
            bag.extend(attrs);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_node_attributes(&self, node: NodeId) -> Option<&PropertyBag> {
        self.storage.node_attrs(node)
    }

    /// 🏷️ Keyed by `EdgeId` (not `(u, v)`), since several parallel edges can share a node pair.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_edge_attributes(&mut self, edge: EdgeId, attrs: PropertyBag) {
        if let Some(bag) = self.storage.edge_attrs_mut(edge) {
            bag.extend(attrs);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_edge_attributes(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(edge)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn name(&self) -> Option<&str> {
        self.storage.graph_attrs().get("name").and_then(PropertyValue::as_str)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.storage.graph_attrs_mut().insert("name".to_string(), PropertyValue::String(name.into()));
    }
}
// #endregion 🔖️Attributes

// #region 🔖️SelfLoops
impl PortUndirectedGraph {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn selfloop_edges(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.storage.edges().filter(|edge| edge.u == edge.v).map(|edge| edge.id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_selfloops(&self) -> usize {
        self.selfloop_edges().count()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nodes_with_selfloops(&self) -> impl Iterator<Item = NodeId> + '_ {
        let nodes: BTreeSet<NodeId> = self.storage.edges().filter(|edge| edge.u == edge.v).map(|edge| edge.u).collect();
        nodes.into_iter()
    }
}
// #endregion 🔖️SelfLoops

// #region 🔖️Views
/// 🪟️ Structural view delegation — every method borrows `self.storage`'s own `GraphView` impl, which already operates at node level regardless of the `Ported` model underneath.
impl GraphView for PortUndirectedGraph {
    fn node_count(&self) -> usize {
        self.storage.node_count()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.storage.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.storage.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.storage.edge_count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.storage.edges()
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.out_neighbors(node)
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.in_neighbors(node)
    }
    fn degree(&self, node: NodeId) -> usize {
        self.storage.degree(node)
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.storage.out_degree(node)
    }
    fn in_degree(&self, node: NodeId) -> usize {
        self.storage.in_degree(node)
    }
    fn is_directed(&self) -> bool {
        self.storage.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.storage.is_multigraph()
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.storage.edges_between(u, v)
    }
}

impl AttrView for PortUndirectedGraph {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.storage.node_attrs(node)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.storage.graph_attrs()
    }
}

impl EdgeWeights for PortUndirectedGraph {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.storage.weight(edge)
    }
}
// #endregion 🔖️Views

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
