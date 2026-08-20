//! 🔀️ The directed port (multi-edge) graph family — NetworkX `MultiDiGraph` parity facade.

use std::collections::{BTreeMap, BTreeSet};

use graph_core::{AttrView, Directed, EdgeId, EdgeRef, EdgeWeights, GraphView, HandleId, NodeId, Normal, Ported, PropertyBag, PropertyValue, Storage, Undirected};

// #region 🔖️PortDirectedGraph
/// 🔀️ NetworkX `MultiDiGraph` parity facade: wraps `Storage<Ported, Directed>` and hides handle bookkeeping behind a
/// lazily-created "default handle" per node — `Storage<Ported, _>` requires `HandleId` endpoints, but callers here only
/// ever deal in `NodeId`s, exactly like `networkx.MultiDiGraph`.
#[derive(Clone, Debug)]
pub struct PortDirectedGraph {
    storage: Storage<Ported, Directed>,
    default_handle: BTreeMap<NodeId, HandleId>,
    name: String,
}

impl Default for PortDirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl PortDirectedGraph {
    // #subregion 🔖️Construction
    /// 🆕️ Empty directed multigraph.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self { storage: Storage::new(), default_handle: BTreeMap::new(), name: String::new() }
    }
    // #endsubregion

    // #subregion 🔖️Handles
    /// 🪝️ Returns `node`'s default handle, lazily allocating one the first time it's needed. `node` MUST already exist.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn handle_of(&mut self, node: NodeId) -> HandleId {
        if let Some(&h) = self.default_handle.get(&node) {
            return h;
        }
        let h = self.storage.add_handle(node).expect("node exists before a handle is requested for it");
        self.default_handle.insert(node, h);
        h
    }
    // #endsubregion

    // #subregion 🔖️NodeOperations
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
            self.storage.add_node_with_id(id, PropertyBag::new());
        }
    }

    /// 🗑️ Removes a node, cascading incident edges (any handle-node they route through), then drops its default handle entry.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_node(&mut self, node: NodeId) -> bool {
        let removed = self.storage.remove_node(node);
        if removed {
            self.default_handle.remove(&node);
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
    pub fn has_node(&self, node: NodeId) -> bool {
        self.storage.contains_node(node)
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
    // #endsubregion

    // #subregion 🔖️EdgeOperations
    /// ➕️ Auto-creates missing endpoints, then always adds a fresh parallel `source -> target` edge (NetworkX
    /// `MultiDiGraph.add_edge` semantics — never an upsert, direction matters).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edge(&mut self, source: NodeId, target: NodeId) -> EdgeId {
        self.add_edge_with(source, target, PropertyBag::new())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edge_with(&mut self, source: NodeId, target: NodeId, attrs: PropertyBag) -> EdgeId {
        if !self.storage.contains_node(source) {
            self.storage.add_node_with_id(source, PropertyBag::new());
        }
        if !self.storage.contains_node(target) {
            self.storage.add_node_with_id(target, PropertyBag::new());
        }
        let hs = self.handle_of(source);
        let ht = self.handle_of(target);
        self.storage.add_edge_with(hs, ht, attrs)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId)>) {
        for (source, target) in edges {
            self.add_edge(source, target);
        }
    }

    /// ⚖️ Adds `(source, target, weight)` triples, storing `weight` under the `"weight"` attribute key.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_weighted_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId, f64)>) {
        for (source, target, weight) in edges {
            let mut attrs = PropertyBag::new();
            attrs.insert("weight".to_string(), PropertyValue::Number(weight));
            self.add_edge_with(source, target, attrs);
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_edge(&mut self, id: EdgeId) -> bool {
        self.storage.remove_edge(id)
    }

    /// 🗑️ Removes an arbitrary one of the parallel `source -> target` edges — the smallest `EdgeId`, for determinism.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn remove_one_edge(&mut self, source: NodeId, target: NodeId) -> bool {
        let Some(id) = self.edges_between(source, target).next() else { return false };
        self.storage.remove_edge(id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn has_edge(&self, source: NodeId, target: NodeId) -> bool {
        self.storage.edges_between(source, target).next().is_some()
    }

    /// 🔢️ Counts edges: both endpoints given counts parallel `source -> target` edges; one endpoint given counts every
    /// edge touching it (in + out); neither given counts the whole graph.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_edges(&self, source: Option<NodeId>, target: Option<NodeId>) -> usize {
        match (source, target) {
            (Some(u), Some(v)) => self.storage.edges_between(u, v).count(),
            (Some(u), None) => self.degree(u),
            (None, Some(v)) => self.degree(v),
            (None, None) => self.storage.edge_count(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get_edge_data(&self, id: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn edges_between(&self, source: NodeId, target: NodeId) -> impl Iterator<Item = EdgeId> + '_ {
        self.storage.edges_between(source, target).map(|e| e.id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn in_edges(&self, node: NodeId) -> impl Iterator<Item = EdgeRef> + '_ {
        self.storage.in_neighbors(node).flat_map(move |u| self.storage.edges_between(u, node))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn out_edges(&self, node: NodeId) -> impl Iterator<Item = EdgeRef> + '_ {
        self.storage.out_neighbors(node).flat_map(move |v| self.storage.edges_between(node, v))
    }

    /// 🐍️ Adds a directed path `nodes[0] -> nodes[1] -> ... -> nodes[n-1]`, auto-creating nodes as needed.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_path(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        let ids: Vec<NodeId> = nodes.into_iter().collect();
        for pair in ids.windows(2) {
            self.add_edge(pair[0], pair[1]);
        }
    }

    /// 🔁️ Adds a directed cycle over `nodes` in order, closing back to the first.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_cycle(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        let ids: Vec<NodeId> = nodes.into_iter().collect();
        if ids.len() < 2 {
            if let Some(&only) = ids.first() {
                self.add_edge(only, only);
            }
            return;
        }
        for pair in ids.windows(2) {
            self.add_edge(pair[0], pair[1]);
        }
        self.add_edge(*ids.last().expect("checked len >= 2 above"), ids[0]);
    }

    /// ⭐️ Adds directed edges from `nodes[0]` (the semio_hub) to every other node in `nodes`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add_star(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        let ids: Vec<NodeId> = nodes.into_iter().collect();
        let Some(&semio_hub) = ids.first() else { return };
        for &leaf in &ids[1..] {
            self.add_edge(semio_hub, leaf);
        }
    }
    // #endsubregion

    // #subregion 🔖️Queries
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn successors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.storage.out_neighbors(node).collect::<BTreeSet<_>>().into_iter()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn predecessors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.storage.in_neighbors(node).collect::<BTreeSet<_>>().into_iter()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn in_degree(&self, node: NodeId) -> usize {
        GraphView::in_degree(&self.storage, node)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn out_degree(&self, node: NodeId) -> usize {
        GraphView::out_degree(&self.storage, node)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn degree(&self, node: NodeId) -> usize {
        GraphView::degree(&self.storage, node)
    }

    /// ⚖️ Sums the edge weights of every incident edge (in + out) using `weights`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn weighted_degree(&self, node: NodeId, weights: &impl EdgeWeights) -> f64 {
        let out: f64 = self.storage.out_neighbors(node).flat_map(|v| self.storage.edges_between(node, v)).map(|e| weights.weight(e)).sum();
        let inn: f64 = self.storage.in_neighbors(node).flat_map(|u| self.storage.edges_between(u, node)).map(|e| weights.weight(e)).sum();
        out + inn
    }

    /// 📐️ Directed multigraph density `m / (n * (n - 1))`; unlike a simple graph this can exceed `1.0` because parallel
    /// edges are not deduped — matches NetworkX `density()` applied to a `MultiDiGraph`. `0.0` for `n < 2`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn density(&self) -> f64 {
        let n = self.storage.node_count() as f64;
        if n < 2.0 {
            return 0.0;
        }
        self.storage.edge_count() as f64 / (n * (n - 1.0))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.storage.edge_count() == 0
    }

    /// 🔗️ Union of predecessors and successors, deduped.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn all_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.storage.out_neighbors(node).chain(self.storage.in_neighbors(node)).collect::<BTreeSet<_>>().into_iter()
    }
    // #endsubregion

    // #subregion 🔖️Transforms
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// 🔎️ Owned copy restricted to `nodes`; an edge is kept only when both endpoints are in the subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn subgraph(&self, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let keep: BTreeSet<NodeId> = nodes.into_iter().filter(|&n| self.storage.contains_node(n)).collect();
        let mut out = Self::new();
        for &n in &keep {
            let attrs = self.storage.node_attrs(n).cloned().unwrap_or_default();
            out.storage.add_node_with_id(n, attrs);
        }
        for e in self.storage.edges() {
            if keep.contains(&e.u) && keep.contains(&e.v) {
                let attrs = self.storage.edge_attrs(e.id).cloned().unwrap_or_default();
                out.add_edge_with(e.u, e.v, attrs);
            }
        }
        *out.storage.graph_attrs_mut() = self.storage.graph_attrs().clone();
        out.name = self.name.clone();
        out
    }

    /// 🔎️ Owned copy restricted to `edges`; nodes are exactly the endpoints of the included edges.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn edge_subgraph(&self, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let keep: BTreeSet<EdgeId> = edges.into_iter().collect();
        let mut out = Self::new();
        for e in self.storage.edges() {
            if !keep.contains(&e.id) {
                continue;
            }
            for n in [e.u, e.v] {
                if !out.storage.contains_node(n) {
                    let attrs = self.storage.node_attrs(n).cloned().unwrap_or_default();
                    out.storage.add_node_with_id(n, attrs);
                }
            }
            let attrs = self.storage.edge_attrs(e.id).cloned().unwrap_or_default();
            out.add_edge_with(e.u, e.v, attrs);
        }
        *out.storage.graph_attrs_mut() = self.storage.graph_attrs().clone();
        out.name = self.name.clone();
        out
    }

    /// ↩️ Owned copy with every edge's source/target swapped (NetworkX `MultiDiGraph.reverse()`); rebuilt explicitly
    /// (rather than wrapping with `ReversedView`) since the facade owns its storage and must hand back a `Self`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn reverse(&self) -> Self {
        let mut out = Self::new();
        for n in self.storage.nodes() {
            let attrs = self.storage.node_attrs(n).cloned().unwrap_or_default();
            out.storage.add_node_with_id(n, attrs);
        }
        for e in self.storage.edges() {
            let attrs = self.storage.edge_attrs(e.id).cloned().unwrap_or_default();
            out.add_edge_with(e.v, e.u, attrs);
        }
        *out.storage.graph_attrs_mut() = self.storage.graph_attrs().clone();
        out.name = self.name.clone();
        out
    }

    /// 🔀️ NetworkX `MultiDiGraph.to_undirected(reciprocal=...)`. When `reciprocal` is `false`, every directed edge
    /// (each `EdgeId`) becomes its own parallel undirected edge — `u->v` and `v->u` both individually survive as two
    /// separate parallel undirected edges, since a multigraph never dedupes distinct keyed edges (this is the real
    /// semantic difference from the simple-graph `DirectedGraph.to_undirected`, which collapses to one edge). When
    /// `reciprocal` is `true`, an edge `u->v` is kept only if `v->u` also exists in the original directed graph — one
    /// undirected edge is emitted per surviving directed edge (so a mutual pair emits two parallel undirected edges,
    /// mirroring NetworkX's behavior of keeping both directed edges' data). Builds fresh default-handle bookkeeping —
    /// this new `Storage<Ported, Undirected>`'s handle ids are unrelated to `self`'s.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_undirected(&self, reciprocal: bool) -> Storage<Ported, Undirected> {
        let mut out = Storage::<Ported, Undirected>::new();
        let mut handles: BTreeMap<NodeId, HandleId> = BTreeMap::new();
        for n in self.storage.nodes() {
            let attrs = self.storage.node_attrs(n).cloned().unwrap_or_default();
            out.add_node_with_id(n, attrs);
        }
        let mut handle_of = |out: &mut Storage<Ported, Undirected>, node: NodeId| -> HandleId {
            if let Some(&h) = handles.get(&node) {
                return h;
            }
            let h = semio_framework_plugin::resolve_ready(out.add_handle(node)).expect("node was inserted above before any handle is requested");
            handles.insert(node, h);
            h
        };
        for e in self.storage.edges() {
            if reciprocal && self.storage.edges_between(e.v, e.u).next().is_none() {
                continue;
            }
            let attrs = self.storage.edge_attrs(e.id).cloned().unwrap_or_default();
            let hu = handle_of(&mut out, e.u);
            let hv = handle_of(&mut out, e.v);
            out.add_edge_with(hu, hv, attrs);
        }
        *out.graph_attrs_mut() = self.storage.graph_attrs().clone();
        out
    }

    /// 🧵️ Collapses parallel `(source, target)` directed edges into one, summing each edge's `"weight"` attribute
    /// (defaulting to `1.0` per edge when absent) — NetworkX `MultiDiGraph` -> `DiGraph` simplification.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_simple(&self) -> Storage<Normal, Directed> {
        let mut out = Storage::<Normal, Directed>::new();
        for n in self.storage.nodes() {
            let attrs = self.storage.node_attrs(n).cloned().unwrap_or_default();
            out.add_node_with_id(n, attrs);
        }
        let mut weight_sum: BTreeMap<(NodeId, NodeId), f64> = BTreeMap::new();
        for e in self.storage.edges() {
            let w = self.storage.edge_attrs(e.id).and_then(|a| a.get("weight")).and_then(PropertyValue::as_f64).unwrap_or(1.0);
            *weight_sum.entry((e.u, e.v)).or_insert(0.0) += w;
        }
        for ((u, v), w) in weight_sum {
            let mut attrs = PropertyBag::new();
            attrs.insert("weight".to_string(), PropertyValue::Number(w));
            out.add_edge_with(u, v, attrs);
        }
        out
    }

    /// 🧹️ Removes every node, edge, and handle-owner mapping; the `default_handle` cache is cleared too.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn clear(&mut self) {
        self.storage.clear();
        self.default_handle.clear();
    }

    /// 🧹️ Removes every edge but keeps nodes; default handles persist (same documented choice as the undirected sibling
    /// facade — a node's default handle is a stable identity, not edge-scoped).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn clear_edges(&mut self) {
        self.storage.clear_edges();
    }
    // #endsubregion

    // #subregion 🔖️Attributes
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
    pub fn name(&self) -> &str {
        &self.name
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
    // #endsubregion

    // #subregion 🔖️SelfLoops
    /// 🔁️ Every self-loop edge ref (`u == v`), in `EdgeId` order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn selfloop_edges(&self) -> impl Iterator<Item = EdgeRef> + '_ {
        self.storage.edges().filter(|e| e.u == e.v)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn number_of_selfloops(&self) -> usize {
        self.selfloop_edges().count()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nodes_with_selfloops(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.selfloop_edges().map(|e| e.u).collect::<BTreeSet<_>>().into_iter()
    }
    // #endsubregion
}
// #endregion 🔖️PortDirectedGraph

// #region 🔖️ViewDelegation
impl GraphView for PortDirectedGraph {
    async fn node_count(&self) -> usize {
        self.storage.node_count().await
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.storage.nodes()
    }
    async fn contains_node(&self, node: NodeId) -> bool {
        self.storage.contains_node(node).await
    }
    async fn edge_count(&self) -> usize {
        self.storage.edge_count().await
    }
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.storage.edges()
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.neighbors(node)
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.out_neighbors(node)
    }
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.storage.in_neighbors(node)
    }
    async fn degree(&self, node: NodeId) -> usize {
        self.storage.degree(node).await
    }
    async fn out_degree(&self, node: NodeId) -> usize {
        self.storage.out_degree(node).await
    }
    async fn in_degree(&self, node: NodeId) -> usize {
        self.storage.in_degree(node).await
    }
    async fn is_directed(&self) -> bool {
        self.storage.is_directed().await
    }
    async fn is_multigraph(&self) -> bool {
        self.storage.is_multigraph().await
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.storage.edges_between(u, v)
    }
}

impl AttrView for PortDirectedGraph {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.storage.node_attrs(node).await
    }
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.storage.edge_attrs(edge).await
    }
    async fn graph_attrs(&self) -> &PropertyBag {
        self.storage.graph_attrs().await
    }
}

impl EdgeWeights for PortDirectedGraph {
    async fn weight(&self, edge: EdgeRef) -> f64 {
        self.storage.weight(edge).await
    }
}
// #endregion 🔖️ViewDelegation

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn parallel_directed_edges_are_never_upserted() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e1 = g.add_edge(a, b);
        let e2 = g.add_edge(a, b);
        assert_ne!(e1, e2);
        assert_eq!(g.number_of_edges(Some(a), Some(b)), 2);
        assert_eq!(g.edges_between(a, b).collect::<Vec<_>>(), vec![e1, e2]);
    }

    #[semio_framework_async_macros::async_test]
    async fn reverse_pair_is_independent() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        assert!(g.has_edge(a, b));
        assert!(!g.has_edge(b, a));
        g.add_edge(b, a);
        assert!(g.has_edge(a, b));
        assert!(g.has_edge(b, a));
        assert_eq!(g.number_of_edges(Some(a), Some(b)), 1);
        assert_eq!(g.number_of_edges(Some(b), Some(a)), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn successors_and_predecessors_dedupe_despite_parallels() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, b);
        g.add_edge(a, b);
        assert_eq!(g.successors(a).collect::<Vec<_>>(), vec![b]);
        assert_eq!(g.predecessors(b).collect::<Vec<_>>(), vec![a]);
    }

    #[semio_framework_async_macros::async_test]
    async fn in_out_degree_count_every_parallel_edge() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, b);
        g.add_edge(b, a);
        assert_eq!(g.out_degree(a), 2);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn selfloop_counts_twice_towards_degree() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.number_of_selfloops(), 1);
        assert_eq!(g.nodes_with_selfloops().collect::<Vec<_>>(), vec![a]);
        assert_eq!(g.out_degree(a), 1);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_one_edge_leaves_the_rest() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e1 = g.add_edge(a, b);
        let e2 = g.add_edge(a, b);
        assert!(g.remove_one_edge(a, b));
        assert_eq!(g.edges_between(a, b).collect::<Vec<_>>(), vec![e2]);
        assert_ne!(e1, e2);
        assert_eq!(g.number_of_edges(Some(a), Some(b)), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn to_undirected_non_reciprocal_keeps_both_directions_as_separate_parallel_edges() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, a);
        let u = g.to_undirected(false);
        assert_eq!(u.edge_count(), 2);
        assert_eq!(GraphView::degree(&u, a), 2);
        assert_eq!(GraphView::degree(&u, b), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn to_undirected_reciprocal_keeps_only_mutual_pairs() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, a);
        g.add_edge(a, c); // not reciprocated
        let u = g.to_undirected(true);
        assert_eq!(u.edge_count(), 2); // a<->b pair survives as two edges, a-c dropped
        assert_eq!(GraphView::degree(&u, a), 2);
        assert_eq!(GraphView::degree(&u, c), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn to_simple_sums_weights() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs1 = PropertyBag::new();
        attrs1.insert("weight".to_string(), PropertyValue::Number(2.0));
        g.add_edge_with(a, b, attrs1);
        let mut attrs2 = PropertyBag::new();
        attrs2.insert("weight".to_string(), PropertyValue::Number(3.5));
        g.add_edge_with(a, b, attrs2);
        g.add_edge(b, a); // unweighted -> defaults to 1.0, separate pair
        let s = g.to_simple();
        assert_eq!(s.edge_count(), 2);
        let ab = s.edges_between(a, b).next().expect("a->b collapsed edge exists");
        assert_eq!(s.edge_attrs(ab.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64), Some(5.5));
        let ba = s.edges_between(b, a).next().expect("b->a collapsed edge exists");
        assert_eq!(s.edge_attrs(ba.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64), Some(1.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn reverse_flips_every_edge() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, b);
        let r = g.reverse();
        assert!(!r.has_edge(a, b));
        assert_eq!(r.number_of_edges(Some(b), Some(a)), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_edge_auto_creates_missing_nodes() {
        let mut g = PortDirectedGraph::new();
        assert!(!g.has_node(0));
        assert!(!g.has_node(1));
        g.add_edge(0, 1);
        assert!(g.has_node(0));
        assert!(g.has_node(1));
        assert!(g.has_edge(0, 1));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_path_cycle_star_build_directed_shapes() {
        let mut g = PortDirectedGraph::new();
        g.add_path([0, 1, 2]);
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 2));
        assert!(!g.has_edge(2, 0));

        let mut cyc = PortDirectedGraph::new();
        cyc.add_cycle([0, 1, 2]);
        assert!(cyc.has_edge(0, 1));
        assert!(cyc.has_edge(1, 2));
        assert!(cyc.has_edge(2, 0));

        let mut star = PortDirectedGraph::new();
        star.add_star([0, 1, 2, 3]);
        assert!(star.has_edge(0, 1));
        assert!(star.has_edge(0, 2));
        assert!(star.has_edge(0, 3));
        assert!(!star.has_edge(1, 2));
    }

    #[semio_framework_async_macros::async_test]
    async fn subgraph_and_edge_subgraph_restrict_correctly() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        let ac = g.add_edge(a, c);

        let sub = g.subgraph([a, b]);
        assert_eq!(sub.number_of_nodes(), 2);
        assert!(sub.has_edge(a, b));
        assert!(!sub.has_node(c));

        let esub = g.edge_subgraph([ac]);
        assert_eq!(esub.number_of_nodes(), 2);
        assert!(esub.has_edge(a, c));
        assert!(!esub.has_node(b));
    }

    #[semio_framework_async_macros::async_test]
    async fn density_can_exceed_one_for_multigraph() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        for _ in 0..5 {
            g.add_edge(a, b);
        }
        assert!(g.density() > 1.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn clear_and_clear_edges_behave() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.clear_edges();
        assert_eq!(g.number_of_edges(None, None), 0);
        assert_eq!(g.number_of_nodes(), 2);
        g.clear();
        assert_eq!(g.number_of_nodes(), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn add_node_with_and_with_id_apply_attrs() {
        let mut g = PortDirectedGraph::new();
        let mut attrs = PropertyBag::new();
        attrs.insert("color".to_string(), PropertyValue::String("red".to_string()));
        let a = g.add_node_with(attrs.clone());
        assert_eq!(g.get_node_attributes(a), Some(&attrs));
        let b = g.add_node_with_id(42, attrs.clone());
        assert_eq!(b, 42);
        assert!(g.has_node(42));
        assert_eq!(g.get_node_attributes(42), Some(&attrs));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_nodes_from_and_remove_nodes_from() {
        let mut g = PortDirectedGraph::new();
        g.add_nodes_from([1, 2, 3]);
        assert_eq!(g.number_of_nodes(), 3);
        assert_eq!(g.order(), 3);
        assert_eq!(g.nodes().collect::<Vec<_>>(), vec![1, 2, 3]);
        g.remove_nodes_from([1, 3]);
        assert!(!g.has_node(1));
        assert!(g.has_node(2));
        assert!(!g.has_node(3));
        assert_eq!(g.number_of_nodes(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_node_cascades_incident_edges_and_forgets_handle() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        assert!(g.remove_node(a));
        assert!(!g.has_node(a));
        assert_eq!(g.number_of_edges(None, None), 0);
        assert!(!g.remove_node(a));
        let a2 = g.add_node_with_id(a, PropertyBag::new());
        assert_eq!(a2, a);
        assert_eq!(g.out_degree(a), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_edge_reports_existence() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        assert!(g.remove_edge(e));
        assert!(!g.has_edge(a, b));
        assert!(!g.remove_edge(e));
        assert!(!g.remove_one_edge(a, b));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_edges_from_and_weighted_edges_from() {
        let mut g = PortDirectedGraph::new();
        g.add_edges_from([(0, 1), (1, 2)]);
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 2));

        let mut w = PortDirectedGraph::new();
        w.add_weighted_edges_from([(0, 1, 2.5), (1, 2, 4.0)]);
        let e = w.edges_between(0, 1).next().expect("edge exists");
        assert_eq!(w.get_edge_data(e).and_then(|a| a.get("weight")).and_then(PropertyValue::as_f64), Some(2.5));
    }

    #[semio_framework_async_macros::async_test]
    async fn number_of_edges_single_endpoint_counts_in_and_out() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(c, a);
        assert_eq!(g.number_of_edges(Some(a), None), 2);
        assert_eq!(g.number_of_edges(None, Some(a)), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn in_edges_and_out_edges_yield_correct_refs() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        let e1 = g.add_edge(a, b);
        let e2 = g.add_edge(c, a);
        assert_eq!(g.out_edges(a).map(|e| e.id).collect::<Vec<_>>(), vec![e1]);
        assert_eq!(g.in_edges(a).map(|e| e.id).collect::<Vec<_>>(), vec![e2]);
    }

    #[semio_framework_async_macros::async_test]
    async fn all_neighbors_unions_predecessors_and_successors() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(c, a);
        assert_eq!(g.all_neighbors(a).collect::<Vec<_>>(), vec![b, c]);
    }

    #[semio_framework_async_macros::async_test]
    async fn weighted_degree_sums_in_and_out_weights() {
        struct UnitWeights;
        impl EdgeWeights for UnitWeights {
            async fn weight(&self, edge: EdgeRef) -> f64 {
                edge.id as f64
            }
        }
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let out = g.add_edge(a, b);
        let inn = g.add_edge(b, a);
        let weights = UnitWeights;
        assert_eq!(g.weighted_degree(a, &weights), (out + inn) as f64);
    }

    #[semio_framework_async_macros::async_test]
    async fn density_is_zero_below_two_nodes() {
        let mut g = PortDirectedGraph::new();
        assert_eq!(g.density(), 0.0);
        g.add_node();
        assert_eq!(g.density(), 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn is_empty_tracks_edge_presence_not_node_presence() {
        let mut g = PortDirectedGraph::new();
        assert!(g.is_empty());
        g.add_node();
        assert!(g.is_empty());
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        assert!(!g.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_produces_an_independent_clone() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let mut c = g.copy();
        c.add_edge(b, a);
        assert!(!g.has_edge(b, a));
        assert!(c.has_edge(b, a));
    }

    #[semio_framework_async_macros::async_test]
    async fn edge_subgraph_skips_edges_not_in_the_keep_set() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let esub = g.edge_subgraph([]);
        assert_eq!(esub.number_of_nodes(), 0);
        assert_eq!(esub.number_of_edges(None, None), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn subgraph_ignores_nonexistent_node_ids() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let sub = g.subgraph([a, 999]);
        assert_eq!(sub.number_of_nodes(), 1);
        assert!(!sub.has_node(999));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_and_get_node_and_edge_attributes() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let mut node_attrs = PropertyBag::new();
        node_attrs.insert("label".to_string(), PropertyValue::String("A".to_string()));
        g.set_node_attributes(a, node_attrs.clone());
        assert_eq!(g.get_node_attributes(a), Some(&node_attrs));

        let mut edge_attrs = PropertyBag::new();
        edge_attrs.insert("weight".to_string(), PropertyValue::Number(1.5));
        g.set_edge_attributes(e, edge_attrs.clone());
        assert_eq!(g.get_edge_attributes(e), Some(&edge_attrs));

        g.set_node_attributes(999, node_attrs);
        assert_eq!(g.get_node_attributes(999), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn name_defaults_empty_and_is_settable() {
        let mut g = PortDirectedGraph::new();
        assert_eq!(g.name(), "");
        g.set_name("my-graph");
        assert_eq!(g.name(), "my-graph");
    }

    #[semio_framework_async_macros::async_test]
    async fn default_matches_new() {
        let g = PortDirectedGraph::default();
        assert_eq!(g.number_of_nodes(), 0);
        assert_eq!(g.number_of_edges(None, None), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_view_and_attr_view_delegate_correctly() {
        let mut g = PortDirectedGraph::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge_with(a, b, {
            let mut attrs = PropertyBag::new();
            attrs.insert("weight".to_string(), PropertyValue::Number(3.0));
            attrs
        });
        assert_eq!(GraphView::node_count(&g), 2);
        assert_eq!(GraphView::edge_count(&g), 1);
        assert!(GraphView::contains_node(&g, a));
        assert_eq!(GraphView::nodes(&g).collect::<Vec<_>>(), vec![a, b]);
        assert_eq!(GraphView::edges(&g).map(|e| e.id).collect::<Vec<_>>(), vec![e]);
        assert_eq!(GraphView::neighbors(&g, a).collect::<Vec<_>>(), vec![b]);
        assert_eq!(GraphView::out_neighbors(&g, a).collect::<Vec<_>>(), vec![b]);
        assert_eq!(GraphView::in_neighbors(&g, b).collect::<Vec<_>>(), vec![a]);
        assert_eq!(GraphView::degree(&g, a), 1);
        assert_eq!(GraphView::out_degree(&g, a), 1);
        assert_eq!(GraphView::in_degree(&g, b), 1);
        assert!(GraphView::is_directed(&g));
        assert!(GraphView::is_multigraph(&g));
        assert_eq!(GraphView::edges_between(&g, a, b).map(|e| e.id).collect::<Vec<_>>(), vec![e]);

        assert!(AttrView::node_attrs(&g, a).is_some());
        let edge_ref = GraphView::edges(&g).next().expect("edge exists");
        assert_eq!(AttrView::edge_attrs(&g, e).and_then(|a| a.get("weight")).and_then(PropertyValue::as_f64), Some(3.0));
        assert!(AttrView::graph_attrs(&g).is_empty());
        assert_eq!(EdgeWeights::weight(&g, edge_ref), 3.0);
    }
}
// #endregion 🔖️Tests
