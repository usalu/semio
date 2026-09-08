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
        self.0.nodes().filter_map(|node| self.0.node_attrs(node).and_then(|attrs| attrs.get(name)).map(|value| (node, value.clone()))).collect()
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
                self.0.edge_attrs(edge.id).and_then(|attrs| attrs.get(name)).map(|value| {
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
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.0.nodes()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.0.contains_node(node)
    }
    fn edge_count(&self) -> usize {
        self.0.edge_count()
    }
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.0.edges()
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.0.neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.0.out_neighbors(node)
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
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
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
