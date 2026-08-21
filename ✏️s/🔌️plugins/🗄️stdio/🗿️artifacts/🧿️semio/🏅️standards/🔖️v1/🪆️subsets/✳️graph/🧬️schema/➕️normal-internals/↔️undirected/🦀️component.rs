//! ⚪️ The undirected simple graph family — NetworkX `Graph` parity facade.

use graph_core::{pairwise, AttrView, AttrWeight, Directed, EdgeId, EdgeRef, EdgeSubgraphView, EdgeWeights, GraphView, NodeId, Normal, PropertyBag, PropertyValue, Storage, SubgraphView, Undirected};
use std::collections::BTreeMap;

// #region 🔖️Construction
/// ⚪️ NetworkX `Graph` parity facade wrapping `Storage<Normal, Undirected>` — a simple (no parallel edges), undirected graph with upsert-on-`add_edge` semantics.
#[derive(Clone, Debug, Default)]
pub struct UndirectedGraph(Storage<Normal, Undirected>);

impl UndirectedGraph {
    /// 🆕️ Empty undirected graph; id allocators start at `0` and are monotone.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self(Storage::new())
    }

    /// 🏗️ Materializes an owned `UndirectedGraph` by copying every node/edge/graph attribute out of a borrowed view — used by `subgraph`/`edge_subgraph` to avoid exposing the borrowed view types in the public API.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from_view<V: GraphView + AttrView>(view: &V) -> Self {
        let mut storage = Storage::<Normal, Undirected>::new();
        for node in view.nodes() {
            let attrs = view.node_attrs(node).cloned().unwrap_or_default();
            storage.add_node_with_id(node, attrs);
        }
        for edge in view.edges() {
            let attrs = view.edge_attrs(edge.id).cloned().unwrap_or_default();
            storage.add_edge_with(edge.u, edge.v, attrs);
        }
        storage.graph_attrs_mut().extend(view.graph_attrs().clone());
        Self(storage)
    }
}
// #endregion 🔖️Construction

// #region 🔖️NodeOperations
impl UndirectedGraph {
    /// ➕️ Allocates a fresh node with no attributes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_node(&mut self) -> NodeId {
        self.0.add_node()
    }

    /// ➕️ Allocates a fresh node with the given attributes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_node_with(&mut self, attrs: PropertyBag) -> NodeId {
        self.0.add_node_with(attrs)
    }

    /// 🆔️ Inserts (or upserts attrs into) a node at a caller-supplied id.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_node_with_id(&mut self, id: NodeId, attrs: PropertyBag) -> NodeId {
        self.0.add_node_with_id(id, attrs)
    }

    /// 📦️ NetworkX `add_nodes_from`: ensures every id exists, leaving already-present nodes' attrs untouched.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) -> Vec<NodeId> {
        nodes.into_iter().map(|id| self.0.add_node_with_id(id, PropertyBag::new())).collect()
    }

    /// 🗑️ Removes a node, cascading to its incident edges.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        self.0.remove_node(id)
    }

    /// 🗑️ NetworkX `remove_nodes_from`: removes every given id, ignoring ones that don't exist.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        for id in nodes {
            self.0.remove_node(id);
        }
    }

    /// 🔎️ Whether `id` is a live node.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn has_node(&self, id: NodeId) -> bool {
        self.0.contains_node(id)
    }

    /// 🔢️ Node count.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_nodes(&self) -> usize {
        self.0.node_count()
    }

    /// 📐️ Alias for `number_of_nodes` (NetworkX `G.order()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn order(&self) -> usize {
        self.number_of_nodes()
    }

    /// 📇️ Every node id, in ascending order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.0.nodes()
    }
}
// #endregion 🔖️NodeOperations

// #region 🔖️EdgeOperations
impl UndirectedGraph {
    /// ➕️ Adds (or, if the pair is already connected, upserts) an edge with no attributes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edge(&mut self, u: NodeId, v: NodeId) -> EdgeId {
        self.0.add_edge(u, v)
    }

    /// ➕️ Adds (or upserts attrs into) an edge between `u` and `v`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edge_with(&mut self, u: NodeId, v: NodeId, attrs: PropertyBag) -> EdgeId {
        self.0.add_edge_with(u, v, attrs)
    }

    /// 📦️ NetworkX `add_edges_from`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId)>) -> Vec<EdgeId> {
        edges.into_iter().map(|(u, v)| self.0.add_edge(u, v)).collect()
    }

    /// ⚖️ NetworkX `add_weighted_edges_from`: sets the `"weight"` attribute on each edge.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_weighted_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId, f64)>) -> Vec<EdgeId> {
        edges
            .into_iter()
            .map(|(u, v, weight)| {
                let mut attrs = PropertyBag::new();
                attrs.insert("weight".to_string(), PropertyValue::Number(weight));
                self.0.add_edge_with(u, v, attrs)
            })
            .collect()
    }

    /// 🗑️ NetworkX `remove_edge(u, v)`: looks the edge id up by endpoints first, since simple graphs address edges by their pair.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_edge(&mut self, u: NodeId, v: NodeId) -> bool {
        let existing = self.0.edges_between(u, v).next().map(|edge| edge.id);
        match existing {
            Some(id) => self.0.remove_edge(id),
            None => false,
        }
    }

    /// 🔎️ Whether `u` and `v` are connected by an edge.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn has_edge(&self, u: NodeId, v: NodeId) -> bool {
        self.0.edges_between(u, v).next().is_some()
    }

    /// 🔢️ Edge count.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_edges(&self) -> usize {
        self.0.edge_count()
    }

    /// 📏️ NetworkX `size(weight=...)`: unweighted is the edge count, weighted is the sum of edge weights.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn size(&self, weighted: bool) -> f64 {
        if weighted {
            self.0.edges().map(|e| self.0.weight(e)).sum()
        } else {
            self.0.edge_count() as f64
        }
    }

    /// 🏷️ Attribute bag of the edge between `u` and `v`, if any.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_edge_data(&self, u: NodeId, v: NodeId) -> Option<&PropertyBag> {
        self.0.edges_between(u, v).next().and_then(|e| self.0.edge_attrs(e.id))
    }

    /// 🛤️ Adds an edge between every consecutive pair of `nodes`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_path(&mut self, nodes: &[NodeId]) {
        for (a, b) in pairwise(nodes) {
            self.0.add_edge(a, b);
        }
    }

    /// 🔁️ Adds a path through `nodes` and closes it into a cycle; a single node produces a self-loop (matching NetworkX `add_cycle`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_cycle(&mut self, nodes: &[NodeId]) {
        match nodes.len() {
            0 => {}
            1 => {
                self.0.add_edge(nodes[0], nodes[0]);
            }
            n => {
                self.add_path(nodes);
                self.0.add_edge(nodes[n - 1], nodes[0]);
            }
        }
    }

    /// ⭐️ Connects `center` to every node in `leaves`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_star(&mut self, center: NodeId, leaves: &[NodeId]) {
        for &leaf in leaves {
            self.0.add_edge(center, leaf);
        }
    }
}
// #endregion 🔖️EdgeOperations

// #region 🔖️Queries
impl UndirectedGraph {
    /// 👥️ Neighbors of `node`, deterministically ordered.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.0.neighbors(node)
    }

    /// 🗺️ NetworkX `G.adjacency()`: every node paired with its neighbor iterator.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn adjacency(&self) -> impl Iterator<Item = (NodeId, impl Iterator<Item = NodeId> + '_)> + '_ {
        self.0.nodes().map(|n| (n, self.0.neighbors(n)))
    }

    /// 🔢️ Degree of `node`; a self-loop counts twice, matching NetworkX.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn degree(&self, node: NodeId) -> usize {
        self.0.degree(node)
    }

    /// ⚖️ Sum of the named attribute over every incident edge, defaulting to `1.0` per edge when the attribute is missing (a self-loop is summed twice, matching `degree`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn weighted_degree(&self, node: NodeId, weight_name: &str) -> f64 {
        let weights = AttrWeight { graph: &self.0, name: weight_name, default: 1.0 };
        self.0.neighbors(node).map(|nb| self.0.edges_between(node, nb).map(|e| weights.weight(e)).sum::<f64>()).sum()
    }

    /// 📐️ NetworkX density `2*m / (n*(n-1))`; defined as `0.0` for `n < 2` (including the empty graph) rather than dividing by zero.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn density(&self) -> f64 {
        let n = self.0.node_count();
        if n < 2 {
            return 0.0;
        }
        let m = self.0.edge_count();
        (2.0 * m as f64) / (n as f64 * (n as f64 - 1.0))
    }

    /// 🕳️ NetworkX `is_empty`: true when there are no edges, regardless of node count — distinct from `number_of_nodes() == 0`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.0.edge_count() == 0
    }
}
// #endregion 🔖️Queries

// #region 🔖️Transforms
impl UndirectedGraph {
    /// 🧬️ Independent full clone.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn copy(&self) -> Self {
        Self(self.0.clone())
    }

    /// ✂️ Owned copy restricted to `nodes` (an edge survives only when both endpoints are kept) — an explicit copy rather than NetworkX's aliasing view.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn subgraph(&self, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let view = SubgraphView::new(&self.0, nodes);
        Self::from_view(&view)
    }

    /// ✂️ Owned copy restricted to `edges` (nodes become exactly those edges' endpoints) — an explicit copy rather than NetworkX's aliasing view.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn edge_subgraph(&self, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let view = EdgeSubgraphView::new(&self.0, edges);
        Self::from_view(&view)
    }

    /// ➡️ NetworkX `to_directed`: each undirected edge becomes two directed edges (one per direction); a self-loop becomes a single directed self-loop since both directions coincide. Returns the raw `Storage` — the `DirectedGraph` facade lives in a sibling crate this crate deliberately doesn't depend on, to avoid a circular dependency.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_directed(&self) -> Storage<Normal, Directed> {
        let mut storage = Storage::<Normal, Directed>::new();
        for node in self.0.nodes() {
            let attrs = self.0.node_attrs(node).cloned().unwrap_or_default();
            storage.add_node_with_id(node, attrs);
        }
        for edge in self.0.edges() {
            let attrs = self.0.edge_attrs(edge.id).cloned().unwrap_or_default();
            storage.add_edge_with(edge.u, edge.v, attrs.clone());
            if edge.u != edge.v {
                storage.add_edge_with(edge.v, edge.u, attrs);
            }
        }
        storage.graph_attrs_mut().extend(self.0.graph_attrs().clone());
        storage
    }

    /// 🧹️ Removes every node, edge, and graph-level attribute.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// 🧹️ Removes every edge, keeping nodes and graph-level attributes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn clear_edges(&mut self) {
        self.0.clear_edges();
    }
}
// #endregion 🔖️Transforms

// #region 🔖️Attributes
impl UndirectedGraph {
    /// 🏷️ NetworkX `set_node_attributes`: merges `attrs` into each named node; ids absent from the graph are silently skipped.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_node_attributes(&mut self, values: impl IntoIterator<Item = (NodeId, PropertyBag)>) {
        for (node, attrs) in values {
            if let Some(existing) = self.0.node_attrs_mut(node) {
                existing.extend(attrs);
            }
        }
    }

    /// 🏷️ NetworkX `get_node_attributes(name)`: every node carrying `name`, mapped to its value.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_node_attributes(&self, name: &str) -> BTreeMap<NodeId, PropertyValue> {
        self.0.nodes().filter_map(|node| semio_framework_plugin::resolve_ready(self.0.node_attrs(node)).and_then(|attrs| attrs.get(name)).map(|value| (node, value.clone()))).collect()
    }

    /// 🏷️ NetworkX `set_edge_attributes`: merges `attrs` into the edge between each `(u, v)`; pairs without an edge are silently skipped.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_edge_attributes(&mut self, values: impl IntoIterator<Item = (NodeId, NodeId, PropertyBag)>) {
        for (u, v, attrs) in values {
            let edge_id = self.0.edges_between(u, v).next().map(|edge| edge.id);
            if let Some(id) = edge_id {
                if let Some(existing) = self.0.edge_attrs_mut(id) {
                    existing.extend(attrs);
                }
            }
        }
    }

    /// 🏷️ NetworkX `get_edge_attributes(name)`: every edge carrying `name`, keyed by its endpoints sorted ascending.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_edge_attributes(&self, name: &str) -> BTreeMap<(NodeId, NodeId), PropertyValue> {
        self.0
            .edges()
            .filter_map(|edge| {
                semio_framework_plugin::resolve_ready(self.0.edge_attrs(edge.id)).and_then(|attrs| attrs.get(name)).map(|value| {
                    let key = if edge.u <= edge.v { (edge.u, edge.v) } else { (edge.v, edge.u) };
                    (key, value.clone())
                })
            })
            .collect()
    }

    /// 🏷️ NetworkX graph `name` attribute, read from `graph_attrs["name"]`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn name(&self) -> Option<String> {
        self.0.graph_attrs().get("name").and_then(PropertyValue::as_str).map(str::to_owned)
    }

    /// 🏷️ Sets the NetworkX graph `name` attribute.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_name(&mut self, name: String) {
        self.0.graph_attrs_mut().insert("name".to_string(), PropertyValue::String(name));
    }
}
// #endregion 🔖️Attributes

// #region 🔖️SelfLoops
impl UndirectedGraph {
    /// 🔂️ Every self-loop edge id.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn selfloop_edges(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.0.edges().filter(|e| e.u == e.v).map(|e| e.id)
    }

    /// 🔢️ Self-loop count.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_selfloops(&self) -> usize {
        self.selfloop_edges().count()
    }

    /// 🔂️ Every node carrying a self-loop.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nodes_with_selfloops(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.0.edges().filter(|e| e.u == e.v).map(|e| e.u)
    }
}
// #endregion 🔖️SelfLoops

// #region 🔖️PathHelpers
impl UndirectedGraph {
    /// 🛤️ Whether every node in `nodes` exists and every consecutive pair is an edge.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_path(&self, nodes: &[NodeId]) -> bool {
        nodes.iter().all(|&n| self.0.contains_node(n)) && pairwise(nodes).all(|(a, b)| self.has_edge(a, b))
    }

    /// ⚖️ Sum of the named weight along consecutive pairs of `nodes`; `None` if any consecutive pair isn't an edge.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn path_weight(&self, nodes: &[NodeId], weight_name: &str) -> Option<f64> {
        let weights = AttrWeight { graph: &self.0, name: weight_name, default: 1.0 };
        let mut total = 0.0;
        for (a, b) in pairwise(nodes) {
            let edge = self.0.edges_between(a, b).next()?;
            total += weights.weight(edge);
        }
        Some(total)
    }

    /// 🤝️ Neighbors shared by both `u` and `v` (excluding `u` and `v` themselves).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn common_neighbors(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let v_neighbors: std::collections::BTreeSet<NodeId> = self.0.neighbors(v).collect();
        self.0.neighbors(u).filter(move |&n| n != u && n != v && v_neighbors.contains(&n))
    }

    /// 🚫️ Every node other than `u` that isn't a neighbor of `u`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn non_neighbors(&self, u: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let neighbors: std::collections::BTreeSet<NodeId> = self.0.neighbors(u).collect();
        self.0.nodes().filter(move |&n| n != u && !neighbors.contains(&n))
    }

    /// 🚫️ Every unordered node pair with no edge between them.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn non_edges(&self) -> impl Iterator<Item = (NodeId, NodeId)> + '_ {
        let nodes: Vec<NodeId> = self.0.nodes().collect();
        let mut pairs = Vec::new();
        for i in 0..nodes.len() {
            for &v in &nodes[(i + 1)..] {
                let u = nodes[i];
                if !self.has_edge(u, v) {
                    pairs.push((u, v));
                }
            }
        }
        pairs.into_iter()
    }
}
// #endregion 🔖️PathHelpers

// #region 🔖️ViewDelegation
/// 🪟️ Delegates the structural view to the inner `Storage` so later-wave algorithm crates can operate on `&UndirectedGraph` directly.
impl GraphView for UndirectedGraph {
    fn node_count(&self) -> usize {
        self.0.node_count()
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.0.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.0.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.0.edge_count()
    }
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.0.edges()
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.0.neighbors(node)
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.0.out_neighbors(node)
    }
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.0.in_neighbors(node)
    }
    fn degree(&self, node: NodeId) -> usize {
        self.0.degree(node)
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.0.out_degree(node)
    }
    fn in_degree(&self, node: NodeId) -> usize {
        self.0.in_degree(node)
    }
    fn is_directed(&self) -> bool {
        self.0.is_directed()
    }
    fn is_multigraph(&self) -> bool {
        self.0.is_multigraph()
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.0.edges_between(u, v)
    }
}

/// 🏷️ Delegates attribute lookup to the inner `Storage`.
impl AttrView for UndirectedGraph {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.0.node_attrs(node)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.0.edge_attrs(edge)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        self.0.graph_attrs()
    }
}

/// ⚖️ Delegates edge weight lookup (`PropertyBag["weight"]`, defaulting to `1.0`) to the inner `Storage`.
impl EdgeWeights for UndirectedGraph {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.0.weight(edge)
    }
}
// #endregion 🔖️ViewDelegation

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn crud_round_trip() {
        let mut g = UndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        assert_eq!(g.number_of_nodes(), 2);
        assert!(g.has_node(a) && g.has_node(b));
        g.add_edge(a, b);
        assert_eq!(g.number_of_edges(), 1);
        assert!(g.has_edge(a, b) && g.has_edge(b, a));
        assert!(g.remove_edge(a, b));
        assert_eq!(g.number_of_edges(), 0);
        assert!(g.remove_node(a));
        assert_eq!(g.number_of_nodes(), 1);
        assert!(!g.has_node(a));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_edge_upsert_semantics() {
        let mut g = UndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs1 = PropertyBag::new();
        attrs1.insert("weight".to_string(), PropertyValue::Number(1.0));
        let e1 = g.add_edge_with(a, b, attrs1);
        let mut attrs2 = PropertyBag::new();
        attrs2.insert("color".to_string(), PropertyValue::String("red".to_string()));
        let e2 = g.add_edge_with(a, b, attrs2);
        assert_eq!(e1, e2, "upserting an existing pair must not allocate a new edge id");
        assert_eq!(g.number_of_edges(), 1);
        let data = g.get_edge_data(a, b).expect("edge data");
        assert_eq!(data.get("weight").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(data.get("color").and_then(PropertyValue::as_str), Some("red"));
    }

    #[semio_framework_async_macros::async_test]
    async fn selfloop_counts_double_degree() {
        let mut g = UndirectedGraph::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.degree(a), 2, "NetworkX counts a self-loop twice towards degree");
        assert_eq!(g.number_of_selfloops(), 1);
        assert_eq!(g.nodes_with_selfloops().collect::<Vec<_>>(), vec![a]);
        assert_eq!(g.selfloop_edges().count(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn density_edge_cases() {
        let empty = UndirectedGraph::new();
        assert_eq!(empty.density(), 0.0, "empty graph is a documented n < 2 case, not a panic");

        let mut singleton = UndirectedGraph::new();
        singleton.add_node();
        assert_eq!(singleton.density(), 0.0);

        let mut k4 = UndirectedGraph::new();
        let nodes: Vec<NodeId> = (0..4).map(|_| k4.add_node()).collect();
        for i in 0..nodes.len() {
            for &v in &nodes[(i + 1)..] {
                k4.add_edge(nodes[i], v);
            }
        }
        assert!((k4.density() - 1.0).abs() < 1e-9, "K4 is a complete graph, density == 1.0");
    }

    #[semio_framework_async_macros::async_test]
    async fn is_empty_vs_number_of_nodes() {
        let mut g = UndirectedGraph::new();
        assert!(g.is_empty());
        assert_eq!(g.number_of_nodes(), 0);
        let a = g.add_node();
        let b = g.add_node();
        assert!(g.is_empty(), "nodes without edges is still empty per NetworkX convention");
        assert_eq!(g.number_of_nodes(), 2);
        g.add_edge(a, b);
        assert!(!g.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn subgraph_and_edge_subgraph_are_independent_copies() {
        let mut g = UndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);

        let mut sub = g.subgraph([a, b]);
        assert_eq!(sub.number_of_nodes(), 2);
        assert_eq!(sub.number_of_edges(), 1);
        sub.remove_edge(a, b);
        assert!(g.has_edge(a, b), "mutating the subgraph copy must not affect the original");

        let mut edge_sub = g.edge_subgraph([e_ab]);
        assert_eq!(edge_sub.number_of_nodes(), 2);
        assert_eq!(edge_sub.number_of_edges(), 1);
        edge_sub.add_node();
        assert_eq!(g.number_of_nodes(), 3, "mutating the edge_subgraph copy must not affect the original");
    }

    #[semio_framework_async_macros::async_test]
    async fn to_directed_doubles_edge_count() {
        let mut g = UndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let directed = g.to_directed();
        assert_eq!(GraphView::edge_count(&directed), 4);
        assert!(GraphView::is_directed(&directed));

        let mut looped = UndirectedGraph::new();
        let n = looped.add_node();
        looped.add_edge(n, n);
        let looped_directed = looped.to_directed();
        assert_eq!(GraphView::edge_count(&looped_directed), 1, "a self-loop has only one direction, so it must not double");
    }

    #[semio_framework_async_macros::async_test]
    async fn path_cycle_star_builders() {
        let mut g = UndirectedGraph::new();
        let nodes: Vec<NodeId> = (0..4).map(|_| g.add_node()).collect();
        g.add_path(&nodes);
        assert!(g.is_path(&nodes));
        assert_eq!(g.number_of_edges(), 3);

        let mut cyc = UndirectedGraph::new();
        let cnodes: Vec<NodeId> = (0..4).map(|_| cyc.add_node()).collect();
        cyc.add_cycle(&cnodes);
        assert_eq!(cyc.number_of_edges(), 4);

        let mut looped_cycle = UndirectedGraph::new();
        let solo = looped_cycle.add_node();
        looped_cycle.add_cycle(&[solo]);
        assert_eq!(looped_cycle.number_of_selfloops(), 1);

        let mut star = UndirectedGraph::new();
        let center = star.add_node();
        let leaves: Vec<NodeId> = (0..3).map(|_| star.add_node()).collect();
        star.add_star(center, &leaves);
        assert_eq!(star.degree(center), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn attributes_round_trip() {
        let mut g = UndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);

        let mut node_attrs = PropertyBag::new();
        node_attrs.insert("label".to_string(), PropertyValue::String("A".to_string()));
        g.set_node_attributes([(a, node_attrs)]);
        let got_nodes = g.get_node_attributes("label");
        assert_eq!(got_nodes.get(&a).and_then(PropertyValue::as_str), Some("A"));

        let mut edge_attrs = PropertyBag::new();
        edge_attrs.insert("weight".to_string(), PropertyValue::Number(2.5));
        g.set_edge_attributes([(a, b, edge_attrs)]);
        let got_edges = g.get_edge_attributes("weight");
        let key = if a <= b { (a, b) } else { (b, a) };
        assert_eq!(got_edges.get(&key).and_then(PropertyValue::as_f64), Some(2.5));

        assert_eq!(g.name(), None);
        g.set_name("test-graph".to_string());
        assert_eq!(g.name().as_deref(), Some("test-graph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn path_helpers() {
        let mut g = UndirectedGraph::new();
        let nodes: Vec<NodeId> = (0..4).map(|_| g.add_node()).collect();
        g.add_weighted_edges_from([(nodes[0], nodes[1], 2.0), (nodes[1], nodes[2], 3.0)]);
        assert_eq!(g.path_weight(&nodes[0..3], "weight"), Some(5.0));
        assert_eq!(g.path_weight(&[nodes[0], nodes[3]], "weight"), None);

        assert_eq!(g.common_neighbors(nodes[0], nodes[2]).collect::<Vec<_>>(), vec![nodes[1]]);
        assert!(g.non_neighbors(nodes[0]).collect::<Vec<_>>().contains(&nodes[3]));
        assert!(g.non_edges().any(|(u, v)| (u, v) == (nodes[0], nodes[2]) || (u, v) == (nodes[2], nodes[0])));
    }

    #[semio_framework_async_macros::async_test]
    async fn weighted_degree_and_size() {
        let mut g = UndirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_weighted_edges_from([(a, b, 4.0)]);
        assert_eq!(g.weighted_degree(a, "weight"), 4.0);
        assert_eq!(g.size(false), 1.0);
        assert_eq!(g.size(true), 4.0);
    }
}
// #endregion 🔖️Tests
