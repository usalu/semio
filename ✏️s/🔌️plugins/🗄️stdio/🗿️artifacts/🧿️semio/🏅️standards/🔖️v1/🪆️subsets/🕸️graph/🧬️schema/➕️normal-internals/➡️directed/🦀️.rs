//! ➡️ The directed simple graph family — NetworkX `DiGraph` parity facade over `graph_core::Storage<Normal, Directed>`.
//! 📐️ Design notes: `reverse`/`to_undirected`/`subgraph`/`edge_subgraph` rebuild a fresh owned `Storage` (explicit copies, not aliasing borrowed views — see the core crate's `SubgraphView`/`ReversedView` doc) rather than wrapping `ReversedView`; `to_undirected` returns the raw `graph_core::Storage<Normal, Undirected>` instead of the sibling facade type to avoid a circular crate dependency; `all_neighbors` chains predecessors then successors WITHOUT deduping, mirroring NetworkX's `all_neighbors` (`itertools.chain`, duplicates included when a node is both a predecessor and a successor).

use graph_core::{AttrView, Directed, EdgeId, EdgeRef, EdgeWeights, GraphView, NodeId, Normal, Storage, Undirected};
use graph_manifest::{PropertyBag, PropertyValue};
use std::collections::{BTreeMap, BTreeSet};

// #region 🔖️DirectedGraph
/// ➡️ NetworkX `DiGraph` parity facade: a simple directed graph (no parallel edges; self-loops allowed).
#[derive(Clone, Debug, Default)]
pub struct DirectedGraph(Storage<Normal, Directed>);

impl DirectedGraph {
    // #subregion Construction
    /// 🆕️ Empty directed graph.
    pub fn new() -> Self {
        Self(Storage::new())
    }
    // #endsubregion

    // #subregion NodeOperations
    /// ➕️ Adds a fresh node with no attributes, returning its auto-assigned id.
    pub fn add_node(&mut self) -> NodeId {
        self.0.add_node()
    }

    /// ➕️ Adds a fresh node with `attrs`, returning its auto-assigned id.
    pub fn add_node_with(&mut self, attrs: PropertyBag) -> NodeId {
        self.0.add_node_with(attrs)
    }

    /// 🆔️ Inserts a node at `id` (or merges `attrs` into it if already present) — NetworkX `add_node(id, **attrs)`.
    pub fn add_node_with_id(&mut self, id: NodeId, attrs: PropertyBag) -> NodeId {
        self.0.add_node_with_id(id, attrs)
    }

    /// ➕️ Adds every id in `nodes` (no attrs) — NetworkX `add_nodes_from`.
    pub fn add_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        for id in nodes {
            self.ensure_node(id);
        }
    }

    /// 🗑️ Removes a node and every incident edge; returns whether it was present.
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        self.0.remove_node(id)
    }

    /// 🗑️ Removes every id in `nodes` — NetworkX `remove_nodes_from`.
    pub fn remove_nodes_from(&mut self, nodes: impl IntoIterator<Item = NodeId>) {
        for id in nodes {
            self.0.remove_node(id);
        }
    }

    pub fn has_node(&self, id: NodeId) -> bool {
        self.0.contains_node(id)
    }

    pub fn number_of_nodes(&self) -> usize {
        self.0.node_count()
    }

    /// 🔢️ Alias of `number_of_nodes` — NetworkX `order()`.
    pub fn order(&self) -> usize {
        self.0.node_count()
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.0.nodes()
    }
    // #endsubregion

    // #subregion EdgeOperations
    /// 🔗️ Ensures `id` exists (inserting an empty-attr node if absent), matching NetworkX's implicit node creation on `add_edge`.
    fn ensure_node(&mut self, id: NodeId) {
        self.0.add_node_with_id(id, PropertyBag::new());
    }

    /// ➕️ Adds a directed `source -> target` edge (auto-creating missing endpoints), returning its id; upserts (merges no attrs) if the pair already exists.
    pub fn add_edge(&mut self, source: NodeId, target: NodeId) -> EdgeId {
        self.add_edge_with(source, target, PropertyBag::new())
    }

    /// ➕️ Adds a directed `source -> target` edge with `attrs` (auto-creating missing endpoints); upserts (merges `attrs`) if the pair already exists.
    pub fn add_edge_with(&mut self, source: NodeId, target: NodeId, attrs: PropertyBag) -> EdgeId {
        self.ensure_node(source);
        self.ensure_node(target);
        self.0.add_edge_with(source, target, attrs)
    }

    /// ➕️ Adds every `(source, target)` pair — NetworkX `add_edges_from`.
    pub fn add_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId)>) -> Vec<EdgeId> {
        edges.into_iter().map(|(u, v)| self.add_edge(u, v)).collect()
    }

    /// ➕️ Adds every `(source, target, weight)` triple, storing `weight` under the `"weight"` attribute — NetworkX `add_weighted_edges_from`.
    pub fn add_weighted_edges_from(&mut self, edges: impl IntoIterator<Item = (NodeId, NodeId, f64)>) -> Vec<EdgeId> {
        edges
            .into_iter()
            .map(|(u, v, w)| {
                let mut attrs = PropertyBag::new();
                attrs.insert("weight".to_string(), PropertyValue::Number(w));
                self.add_edge_with(u, v, attrs)
            })
            .collect()
    }

    /// 🗑️ Removes the `source -> target` edge only — the reverse edge (if any) is untouched. Returns whether it was present.
    pub fn remove_edge(&mut self, source: NodeId, target: NodeId) -> bool {
        let Some(id) = self.0.edges_between(source, target).next().map(|e| e.id) else { return false };
        self.0.remove_edge(id)
    }

    /// ❓️ Whether `source -> target` exists (direction matters).
    pub fn has_edge(&self, source: NodeId, target: NodeId) -> bool {
        self.0.edges_between(source, target).next().is_some()
    }

    pub fn number_of_edges(&self) -> usize {
        self.0.edge_count()
    }

    /// 📏️ Total edge count, or total edge weight when `weighted` — NetworkX `size(weight=...)`.
    pub fn size(&self, weighted: bool) -> f64 {
        if weighted {
            self.0.edges().map(|e| self.0.weight(e)).sum()
        } else {
            self.0.edge_count() as f64
        }
    }

    /// 🏷️ Attribute bag of the `source -> target` edge, if present.
    pub fn get_edge_data(&self, source: NodeId, target: NodeId) -> Option<&PropertyBag> {
        let id = self.0.edges_between(source, target).next()?.id;
        self.0.edge_attrs(id)
    }

    /// ➡️ Adds edges `nodes[0] -> nodes[1] -> ... -> nodes[n-1]`; every listed node is added even if isolated (e.g. a single-node path).
    pub fn add_path(&mut self, nodes: &[NodeId]) -> Vec<EdgeId> {
        for &n in nodes {
            self.ensure_node(n);
        }
        nodes.windows(2).map(|w| self.add_edge(w[0], w[1])).collect()
    }

    /// 🔁️ `add_path` plus a closing edge `nodes[n-1] -> nodes[0]`; a single-node cycle yields a self-loop, matching NetworkX `add_cycle`.
    pub fn add_cycle(&mut self, nodes: &[NodeId]) -> Vec<EdgeId> {
        if nodes.is_empty() {
            return Vec::new();
        }
        let mut ids = self.add_path(nodes);
        let last = *nodes.last().expect("checked non-empty above");
        ids.push(self.add_edge(last, nodes[0]));
        ids
    }

    /// ⭐️ Adds edges `center -> leaf` for every leaf, `center = nodes[0]` — NetworkX directed `add_star` convention. The center is added even with zero leaves.
    pub fn add_star(&mut self, nodes: &[NodeId]) -> Vec<EdgeId> {
        let mut ids = Vec::new();
        if let Some((&center, leaves)) = nodes.split_first() {
            self.ensure_node(center);
            for &leaf in leaves {
                ids.push(self.add_edge(center, leaf));
            }
        }
        ids
    }
    // #endsubregion

    // #subregion Queries
    /// ➡️ Out-neighbors of `node` — NetworkX `successors`.
    pub fn successors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.0.out_neighbors(node)
    }

    /// ⬅️ In-neighbors of `node` — NetworkX `predecessors`.
    pub fn predecessors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.0.in_neighbors(node)
    }

    /// ⬅️ Edges landing on `node` (`v == node`).
    pub fn in_edges(&self, node: NodeId) -> impl Iterator<Item = EdgeRef> + '_ {
        self.0.edges().filter(move |e| e.v == node)
    }

    /// ➡️ Edges leaving `node` (`u == node`).
    pub fn out_edges(&self, node: NodeId) -> impl Iterator<Item = EdgeRef> + '_ {
        self.0.edges().filter(move |e| e.u == node)
    }

    pub fn in_degree(&self, node: NodeId) -> usize {
        self.0.in_degree(node)
    }

    pub fn out_degree(&self, node: NodeId) -> usize {
        self.0.out_degree(node)
    }

    /// 🔢️ `in_degree + out_degree` — NetworkX directed `degree`.
    pub fn degree(&self, node: NodeId) -> usize {
        self.0.degree(node)
    }

    /// 📐️ `m / (n * (n - 1))`, `0.0` for `n < 2` — NetworkX directed `density`.
    pub fn density(&self) -> f64 {
        let n = self.0.node_count() as f64;
        if n < 2.0 {
            return 0.0;
        }
        self.0.edge_count() as f64 / (n * (n - 1.0))
    }

    /// ❓️ Whether the graph has zero edges (isolated nodes don't count) — NetworkX `is_empty`.
    pub fn is_empty(&self) -> bool {
        self.0.edge_count() == 0
    }
    // #endsubregion

    // #subregion Transforms
    /// 📋️ Deep, independent copy.
    pub fn copy(&self) -> Self {
        Self(self.0.clone())
    }

    /// 🔎️ Owned copy restricted to `nodes`; an edge is included only when both endpoints survive.
    pub fn subgraph(&self, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let keep: BTreeSet<NodeId> = nodes.into_iter().filter(|&n| self.0.contains_node(n)).collect();
        let mut out = Storage::<Normal, Directed>::new();
        for &id in &keep {
            out.add_node_with_id(id, self.0.node_attrs(id).cloned().unwrap_or_default());
        }
        for e in self.0.edges() {
            if keep.contains(&e.u) && keep.contains(&e.v) {
                out.add_edge_with(e.u, e.v, self.0.edge_attrs(e.id).cloned().unwrap_or_default());
            }
        }
        *out.graph_attrs_mut() = self.0.graph_attrs().clone();
        Self(out)
    }

    /// 🔎️ Owned copy restricted to `edges`; nodes are exactly the endpoints of the survivors.
    pub fn edge_subgraph(&self, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let keep: BTreeSet<EdgeId> = edges.into_iter().collect();
        let mut nodes: BTreeSet<NodeId> = BTreeSet::new();
        for e in self.0.edges() {
            if keep.contains(&e.id) {
                nodes.insert(e.u);
                nodes.insert(e.v);
            }
        }
        let mut out = Storage::<Normal, Directed>::new();
        for &id in &nodes {
            out.add_node_with_id(id, self.0.node_attrs(id).cloned().unwrap_or_default());
        }
        for e in self.0.edges() {
            if keep.contains(&e.id) {
                out.add_edge_with(e.u, e.v, self.0.edge_attrs(e.id).cloned().unwrap_or_default());
            }
        }
        *out.graph_attrs_mut() = self.0.graph_attrs().clone();
        Self(out)
    }

    /// ↩️ Owned copy with every edge's source/target swapped (rebuilt, not a borrowed `ReversedView`, since callers need an owned `Self`).
    pub fn reverse(&self) -> Self {
        let mut out = Storage::<Normal, Directed>::new();
        for id in self.0.nodes() {
            out.add_node_with_id(id, self.0.node_attrs(id).cloned().unwrap_or_default());
        }
        for e in self.0.edges() {
            out.add_edge_with(e.v, e.u, self.0.edge_attrs(e.id).cloned().unwrap_or_default());
        }
        *out.graph_attrs_mut() = self.0.graph_attrs().clone();
        Self(out)
    }

    /// 🔀️ NetworkX `to_undirected(reciprocal=...)`: `false` collapses every directed edge into one undirected edge (attrs from both directions merged, later-processed direction wins on key conflicts); `true` keeps an undirected edge only where BOTH `u -> v` and `v -> u` exist in `self`. Returns the raw core `Storage` (not the undirected sibling facade) to avoid a circular crate dependency between the two direction facades.
    pub fn to_undirected(&self, reciprocal: bool) -> Storage<Normal, Undirected> {
        let mut out = Storage::<Normal, Undirected>::new();
        for id in self.0.nodes() {
            out.add_node_with_id(id, self.0.node_attrs(id).cloned().unwrap_or_default());
        }
        let mut done: BTreeSet<(NodeId, NodeId)> = BTreeSet::new();
        for e in self.0.edges() {
            let key = if e.u <= e.v { (e.u, e.v) } else { (e.v, e.u) };
            if !done.insert(key) {
                continue;
            }
            let backward = self.has_edge(e.v, e.u);
            if reciprocal && !backward {
                continue;
            }
            let mut attrs = self.0.edge_attrs(e.id).cloned().unwrap_or_default();
            if backward {
                if let Some(rev) = self.get_edge_data(e.v, e.u) {
                    for (key,value) in rev.clone() {
                if let Some(previous)=attrs.insert(key,value){<PropertyValue as semio_framework_value::FromValue>::retire_decoded(previous);}
            }
                }
            }
            out.add_edge_with(e.u, e.v, attrs);
        }
        *out.graph_attrs_mut() = self.0.graph_attrs().clone();
        out
    }

    /// 🧹️ Removes every node, edge, and graph-level attribute.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// 🧹️ Removes every edge; nodes and graph-level attrs survive.
    pub fn clear_edges(&mut self) {
        self.0.clear_edges();
    }
    // #endsubregion

    // #subregion Attributes
    /// 🏷️ Sets `attrs[name]` on every listed node that exists; missing ids are skipped — NetworkX `set_node_attributes`.
    pub fn set_node_attributes(&mut self, name: &str, values: impl IntoIterator<Item = (NodeId, PropertyValue)>) {
        for (id, value) in values {
            if let Some(attrs) = self.0.node_attrs_mut(id) {
                attrs.insert(name.to_string(), value);
            }
        }
    }

    /// 🏷️ Reads `attrs[name]` off every node that has it — NetworkX `get_node_attributes`.
    pub fn get_node_attributes(&self, name: &str) -> BTreeMap<NodeId, PropertyValue> {
        self.0.nodes().filter_map(|id| self.0.node_attrs(id).and_then(|a| a.get(name)).cloned().map(|v| (id, v))).collect()
    }

    /// 🏷️ Sets `attrs[name]` on every listed `(source, target)` edge that exists; missing pairs are skipped — NetworkX `set_edge_attributes`, keyed by direction.
    pub fn set_edge_attributes(&mut self, name: &str, values: impl IntoIterator<Item = ((NodeId, NodeId), PropertyValue)>) {
        for ((u, v), value) in values {
            let edge_id = self.0.edges_between(u, v).next().map(|e| e.id);
            if let Some(id) = edge_id {
                if let Some(attrs) = self.0.edge_attrs_mut(id) {
                    attrs.insert(name.to_string(), value);
                }
            }
        }
    }

    /// 🏷️ Reads `attrs[name]` off every `(source, target)` edge that has it — NetworkX `get_edge_attributes`, keyed by direction.
    pub fn get_edge_attributes(&self, name: &str) -> BTreeMap<(NodeId, NodeId), PropertyValue> {
        self.0.edges().filter_map(|e| self.0.edge_attrs(e.id).and_then(|a| a.get(name)).cloned().map(|v| ((e.u, e.v), v))).collect()
    }

    /// 🏷️ Graph-level `"name"` attribute, or `""` if unset.
    pub fn name(&self) -> String {
        self.0.graph_attrs().get("name").and_then(PropertyValue::as_str).unwrap_or("").to_string()
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.0.graph_attrs_mut().insert("name".to_string(), PropertyValue::String(name.into()));
    }
    // #endsubregion

    // #subregion SelfLoops
    /// 🔁️ Every edge where `u == v`.
    pub fn selfloop_edges(&self) -> impl Iterator<Item = EdgeRef> + '_ {
        self.0.edges().filter(|e| e.u == e.v)
    }

    pub fn number_of_selfloops(&self) -> usize {
        self.selfloop_edges().count()
    }

    /// 🔁️ Distinct nodes that carry at least one self-loop.
    pub fn nodes_with_selfloops(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.selfloop_edges().map(|e| e.u).collect::<BTreeSet<_>>().into_iter()
    }
    // #endsubregion

    // #subregion PathHelpers
    /// 🛤️ Whether every listed node exists and every consecutive pair is a forward (`source -> target`) edge.
    pub fn is_path(&self, nodes: &[NodeId]) -> bool {
        nodes.iter().all(|&n| self.0.contains_node(n)) && nodes.windows(2).all(|w| self.has_edge(w[0], w[1]))
    }

    /// ⚖️ Sum of edge weights along `nodes` (via the graph's own `"weight"` attribute, defaulting to `1.0` per edge), or `None` if `nodes` isn't a valid path.
    pub fn path_weight(&self, nodes: &[NodeId]) -> Option<f64> {
        if !self.is_path(nodes) {
            return None;
        }
        Some(
            nodes
                .windows(2)
                .map(|w| {
                    let edge = self.0.edges_between(w[0], w[1]).next().expect("edge exists, checked by is_path");
                    self.0.weight(edge)
                })
                .sum(),
        )
    }

    /// 🤝️ Nodes reachable as a successor of both `u` and `v` — NetworkX digraph `common_neighbors` (defined via successors, i.e. `neighbors == successors` on a `DiGraph`).
    pub fn common_neighbors(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let from_v: BTreeSet<NodeId> = self.0.out_neighbors(v).collect();
        self.0.out_neighbors(u).filter(move |n| from_v.contains(n)).collect::<Vec<_>>().into_iter()
    }

    /// 🙅️ Nodes that are neither `node` itself nor a successor of `node`.
    pub fn non_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        let neighbors: BTreeSet<NodeId> = self.0.out_neighbors(node).collect();
        self.0.nodes().filter(move |&n| n != node && !neighbors.contains(&n))
    }

    /// 🙅️ Every ordered pair `(u, v)` with `u != v` that is NOT a `u -> v` edge.
    pub fn non_edges(&self) -> impl Iterator<Item = (NodeId, NodeId)> + '_ {
        let nodes: Vec<NodeId> = self.0.nodes().collect();
        let mut pairs = Vec::new();
        for &u in &nodes {
            for &v in &nodes {
                if u != v && !self.has_edge(u, v) {
                    pairs.push((u, v));
                }
            }
        }
        pairs.into_iter()
    }
    // #endsubregion

    // #subregion DirectedOnlyHelpers
    /// 🔀️ Predecessors chained with successors, duplicates included when a node is both — NetworkX `all_neighbors` on a directed graph; useful for algorithms that want undirected-style adjacency without discarding direction elsewhere.
    pub fn all_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.0.in_neighbors(node).chain(self.0.out_neighbors(node))
    }
    // #endsubregion
}
// #endregion 🔖️DirectedGraph

// #region 🔖️ViewImpls
impl GraphView for DirectedGraph {
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

impl AttrView for DirectedGraph {
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

impl EdgeWeights for DirectedGraph {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.0.weight(edge)
    }
}
// #endregion 🔖️ViewImpls

// #region 🔖️Tests

// #endregion 🔖️Tests
