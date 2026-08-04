//! 🕸️ Pure graph foundation: topology markers, node/handle/edge kinds, and index-based algorithms; the interactive board engine lives in `infinite_board`.

use std::collections::{BTreeMap, BTreeSet};

pub use mathematical_graph_manifest::{PropertyBag, PropertyValue};

// #region 🔖️Ids
/// 🧩️ Stable node identifier.
pub type NodeId = u64;
/// 🪝️ Stable handle identifier.
pub type HandleId = u64;
/// 🪢️ Stable edge identifier.
pub type EdgeId = u64;
// #endregion 🔖️Ids

// #region 🔖️Edge
/// 🪢️ Edge with typed endpoints (node id or handle id).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoreEdge<E> {
    pub id: EdgeId,
    pub source: E,
    pub target: E,
}

impl<E: Copy + Ord> CoreEdge<E> {
    /// 📐️ Normalize endpoints for undirected storage.
    pub fn normalize_undirected(source: E, target: E) -> (E, E) {
        if source <= target {
            (source, target)
        } else {
            (target, source)
        }
    }
}
// #endregion 🔖️Edge

// #region 🔖️Directedness
/// ↔ Compile-time directed vs undirected graph axis.
pub trait Directedness {
    const DIRECTED: bool;
}

/// ➡️ Directed edges keep source→target order.
#[derive(Clone, Copy, Debug, Default)]
pub struct Directed;

impl Directedness for Directed {
    const DIRECTED: bool = true;
}

/// ↔ Undirected edges store ordered endpoint pair.
#[derive(Clone, Copy, Debug, Default)]
pub struct Undirected;

impl Directedness for Undirected {
    const DIRECTED: bool = false;
}

/// 📐️ Apply directedness when storing edge endpoints.
#[inline]
pub fn orient_endpoints<E: Copy + Ord, D: Directedness>(source: E, target: E) -> (E, E) {
    if D::DIRECTED {
        (source, target)
    } else {
        CoreEdge::<E>::normalize_undirected(source, target)
    }
}
// #endregion 🔖️Directedness

// #region 🔖️PortModel
/// 🔌️ Compile-time normal (node) vs ported (handle) graph axis.
pub trait PortModel {
    type Endpoint: Copy + Ord + std::fmt::Debug;
    const HAS_PORTS: bool;
    /// 🪢️ Whether this port model allows parallel edges between the same pair (the port axis IS the multi-edge axis: `Ported` ~ NetworkX `Multi(Di)Graph`, `Normal` ~ NetworkX `(Di)Graph`).
    const MULTI_EDGES: bool;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64;
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint>;
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId>;
}

/// 🟠️ Node-to-node edges without handles.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl PortModel for Normal {
    type Endpoint = NodeId;
    const HAS_PORTS: bool = false;
    const MULTI_EDGES: bool = false;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(_: HandleId) -> Option<Self::Endpoint> {
        None
    }
    fn endpoint_as_handle(_: Self::Endpoint) -> Option<HandleId> {
        None
    }
}

/// 🪝️ Handle-to-handle edges on nodes.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ported;

impl PortModel for Ported {
    type Endpoint = HandleId;
    const HAS_PORTS: bool = true;
    const MULTI_EDGES: bool = true;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint> {
        Some(handle_id)
    }
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId> {
        Some(endpoint)
    }
}
// #endregion 🔖️PortModel

// #region 🔖️Storage
/// 📦️ Per-node record: attribute bag plus, for ported storages, the handles anchored on it (stays empty for `Normal` storages).
#[derive(Clone, Debug, Default)]
pub struct NodeRecord {
    pub attrs: PropertyBag,
    pub handles: Vec<HandleId>,
}

/// 📦️ Per-edge record: typed endpoints (node ids for `Normal`, handle ids for `Ported`) plus attribute bag.
#[derive(Clone, Debug)]
pub struct EdgeRecord<E> {
    pub source: E,
    pub target: E,
    pub attrs: PropertyBag,
}

/// 🗑️ Removes one occurrence of `edge_id` from `map[u][v]`, dropping the inner entry once its edge list empties.
fn unlink_one(map: &mut BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>, u: NodeId, v: NodeId, edge_id: EdgeId) {
    if let Some(inner) = map.get_mut(&u) {
        if let Some(ids) = inner.get_mut(&v) {
            if let Some(pos) = ids.iter().position(|&e| e == edge_id) {
                ids.remove(pos);
            }
            if ids.is_empty() {
                inner.remove(&v);
            }
        }
    }
}

/// 🗄️ Shared adjacency-map storage behind every per-kind facade crate; `BTreeMap` everywhere keeps iteration deterministic. Node-level adjacency (`successors`/`predecessors`) is always keyed by `NodeId`, even for `Ported` storages — port/handle detail lives only in `EdgeRecord::source`/`target` and is resolved down to owning nodes via `handle_owner`. For undirected storages `successors` already holds both directions of every edge, so `predecessors` stays empty and is never consulted (documented at each call site); a self-loop on an undirected storage is recorded twice in `successors[u][u]`, matching NetworkX's convention of counting a self-loop twice towards degree.
#[derive(Clone, Debug)]
pub struct Storage<P: PortModel, D: Directedness> {
    nodes: BTreeMap<NodeId, NodeRecord>,
    edges: BTreeMap<EdgeId, EdgeRecord<P::Endpoint>>,
    successors: BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>,
    predecessors: BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>,
    handle_owner: BTreeMap<HandleId, NodeId>,
    graph_attrs: PropertyBag,
    next_node_id: NodeId,
    next_edge_id: EdgeId,
    next_handle_id: HandleId,
    _directedness: std::marker::PhantomData<D>,
    _port_model: std::marker::PhantomData<P>,
}

impl<P: PortModel, D: Directedness> Default for Storage<P, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: PortModel, D: Directedness> Storage<P, D> {
    /// 🆕️ Empty storage; every id allocator starts at `0` and is monotone — an id is never reused, even after removal.
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
            successors: BTreeMap::new(),
            predecessors: BTreeMap::new(),
            handle_owner: BTreeMap::new(),
            graph_attrs: PropertyBag::new(),
            next_node_id: 0,
            next_edge_id: 0,
            next_handle_id: 0,
            _directedness: std::marker::PhantomData,
            _port_model: std::marker::PhantomData,
        }
    }

    /// 🔗️ Resolves an edge endpoint down to the node it lives on: identity for `Normal` (`Endpoint == NodeId`), a `handle_owner` lookup for `Ported`.
    fn endpoint_node(&self, endpoint: P::Endpoint) -> NodeId {
        match P::endpoint_as_handle(endpoint) {
            Some(handle_id) => *self.handle_owner.get(&handle_id).expect("every live handle endpoint has a recorded owner node"),
            None => P::endpoint_as_u64(endpoint),
        }
    }

    fn link_adjacency(&mut self, u: NodeId, v: NodeId, edge_id: EdgeId) {
        self.successors.entry(u).or_default().entry(v).or_default().push(edge_id);
        if D::DIRECTED {
            self.predecessors.entry(v).or_default().entry(u).or_default().push(edge_id);
        } else if u == v {
            self.successors.entry(u).or_default().entry(v).or_default().push(edge_id);
        } else {
            self.successors.entry(v).or_default().entry(u).or_default().push(edge_id);
        }
    }

    fn unlink_adjacency(&mut self, u: NodeId, v: NodeId, edge_id: EdgeId) {
        unlink_one(&mut self.successors, u, v, edge_id);
        if D::DIRECTED {
            unlink_one(&mut self.predecessors, v, u, edge_id);
        } else if u == v {
            unlink_one(&mut self.successors, u, v, edge_id);
        } else {
            unlink_one(&mut self.successors, v, u, edge_id);
        }
    }

    // #subregion Nodes
    pub fn add_node(&mut self) -> NodeId {
        self.add_node_with(PropertyBag::new())
    }

    pub fn add_node_with(&mut self, attrs: PropertyBag) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        self.nodes.insert(id, NodeRecord { attrs, handles: Vec::new() });
        id
    }

    /// 🆔️ Inserts a node at a caller-supplied id, or merges `attrs` into it if already present (NetworkX `add_node(id, **attrs)` semantics); bumps the allocator past `id` so future auto-ids never collide with it.
    pub fn add_node_with_id(&mut self, id: NodeId, attrs: PropertyBag) -> NodeId {
        if self.next_node_id <= id {
            self.next_node_id = id + 1;
        }
        match self.nodes.get_mut(&id) {
            Some(record) => record.attrs.extend(attrs),
            None => {
                self.nodes.insert(id, NodeRecord { attrs, handles: Vec::new() });
            }
        }
        id
    }

    pub fn contains_node(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    /// 🗑️ Removes a node, cascading: every incident edge is removed first, then (for ported storages) every handle anchored on it.
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        if !self.nodes.contains_key(&id) {
            return false;
        }
        let mut incident: BTreeSet<EdgeId> = BTreeSet::new();
        if let Some(succ) = self.successors.get(&id) {
            for ids in succ.values() {
                incident.extend(ids.iter().copied());
            }
        }
        if let Some(pred) = self.predecessors.get(&id) {
            for ids in pred.values() {
                incident.extend(ids.iter().copied());
            }
        }
        for edge_id in incident {
            self.remove_edge(edge_id);
        }
        if let Some(record) = self.nodes.remove(&id) {
            for handle_id in record.handles {
                self.handle_owner.remove(&handle_id);
            }
        }
        self.successors.remove(&id);
        self.predecessors.remove(&id);
        true
    }

    pub fn node_attrs_mut(&mut self, id: NodeId) -> Option<&mut PropertyBag> {
        self.nodes.get_mut(&id).map(|r| &mut r.attrs)
    }
    // #endsubregion

    // #subregion Edges
    pub fn add_edge(&mut self, source: P::Endpoint, target: P::Endpoint) -> EdgeId {
        self.add_edge_with(source, target, PropertyBag::new())
    }

    /// 🔀️ `Normal` storages upsert: an edge already connecting this pair gets `attrs` merged into it and its existing id returned (NetworkX `Graph`/`DiGraph`). `Ported` storages always create a fresh parallel edge with a new `EdgeId` (NetworkX `MultiGraph`/`MultiDiGraph`).
    pub fn add_edge_with(&mut self, source: P::Endpoint, target: P::Endpoint, attrs: PropertyBag) -> EdgeId {
        let (un, vn) = (self.endpoint_node(source), self.endpoint_node(target));
        if !P::MULTI_EDGES {
            if let Some(&existing) = self.successors.get(&un).and_then(|m| m.get(&vn)).and_then(|ids| ids.first()) {
                if let Some(record) = self.edges.get_mut(&existing) {
                    record.attrs.extend(attrs);
                }
                return existing;
            }
        }
        let id = self.next_edge_id;
        self.next_edge_id += 1;
        self.edges.insert(id, EdgeRecord { source, target, attrs });
        self.link_adjacency(un, vn, id);
        id
    }

    pub fn remove_edge(&mut self, id: EdgeId) -> bool {
        let Some(record) = self.edges.remove(&id) else { return false };
        let (u, v) = (self.endpoint_node(record.source), self.endpoint_node(record.target));
        self.unlink_adjacency(u, v, id);
        true
    }

    pub fn edge_attrs_mut(&mut self, id: EdgeId) -> Option<&mut PropertyBag> {
        self.edges.get_mut(&id).map(|r| &mut r.attrs)
    }

    pub fn edge_endpoints(&self, id: EdgeId) -> Option<(P::Endpoint, P::Endpoint)> {
        self.edges.get(&id).map(|r| (r.source, r.target))
    }
    // #endsubregion

    // #subregion Handles
    /// 🪝️ Allocates a new handle anchored on `node`; only meaningful when `P::HAS_PORTS` — returns `None` otherwise (or if `node` doesn't exist), never panics.
    pub fn add_handle(&mut self, node: NodeId) -> Option<HandleId> {
        if !P::HAS_PORTS || !self.nodes.contains_key(&node) {
            return None;
        }
        let id = self.next_handle_id;
        self.next_handle_id += 1;
        self.handle_owner.insert(id, node);
        self.nodes.get_mut(&node).expect("presence checked above").handles.push(id);
        Some(id)
    }

    pub fn handles(&self, node: NodeId) -> &[HandleId] {
        self.nodes.get(&node).map_or(&[], |r| r.handles.as_slice())
    }

    pub fn handle_owner(&self, handle: HandleId) -> Option<NodeId> {
        self.handle_owner.get(&handle).copied()
    }
    // #endsubregion

    // #subregion Whole graph
    pub fn graph_attrs_mut(&mut self) -> &mut PropertyBag {
        &mut self.graph_attrs
    }

    /// 🧹️ Removes every node, edge, and handle; graph-level attrs are cleared too. Id allocators are NOT reset — ids are never reused, even across a clear.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.successors.clear();
        self.predecessors.clear();
        self.handle_owner.clear();
        self.graph_attrs.clear();
    }

    /// 🧹️ Removes every edge but keeps nodes (and their handles) and graph-level attrs.
    pub fn clear_edges(&mut self) {
        self.edges.clear();
        for adj in self.successors.values_mut() {
            adj.clear();
        }
        for adj in self.predecessors.values_mut() {
            adj.clear();
        }
    }
    // #endsubregion
}
// #endregion 🔖️Storage

// #region 🔖️View traits
/// 🪢️ Node-level edge reference; carries its own id plus both endpoint node ids. Port/handle detail is already resolved away — algorithms never see a `HandleId`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeRef {
    pub id: EdgeId,
    pub u: NodeId,
    pub v: NodeId,
}

/// 🪟️ Structural read-only view every future algorithm crate is written against — the single most important contract in this campaign; keep it minimal and stable.
pub trait GraphView {
    fn node_count(&self) -> usize;
    fn nodes(&self) -> impl Iterator<Item = NodeId>;
    fn contains_node(&self, node: NodeId) -> bool;
    fn edge_count(&self) -> usize;
    fn edges(&self) -> impl Iterator<Item = EdgeRef>;
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    /// ⬅️ Equals `out_neighbors` on an undirected view — there is only one adjacency direction, so predecessors and successors coincide.
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    fn degree(&self, node: NodeId) -> usize;
    fn out_degree(&self, node: NodeId) -> usize;
    /// ⬅️ Equals `out_degree` on an undirected view, for the same reason as `in_neighbors`.
    fn in_degree(&self, node: NodeId) -> usize;
    fn is_directed(&self) -> bool;
    fn is_multigraph(&self) -> bool;
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef>;
}

/// 🏷️ Attribute lookup companion to `GraphView`.
pub trait AttrView {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag>;
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag>;
    fn graph_attrs(&self) -> &PropertyBag;
}

/// ⚖️ Edge weight lookup, decoupled from attribute storage so algorithms take `impl EdgeWeights` instead of hardcoding a `"weight"` key.
pub trait EdgeWeights {
    fn weight(&self, edge: EdgeRef) -> f64;
}

/// 1⃣ Unweighted default: every edge costs `1.0` (NetworkX's unweighted-graph convention).
#[derive(Clone, Copy, Debug, Default)]
pub struct UnitWeight;

impl EdgeWeights for UnitWeight {
    fn weight(&self, _edge: EdgeRef) -> f64 {
        1.0
    }
}

/// 🏷️ Reads a named numeric attribute off any `AttrView`, falling back to `default` when the attribute is missing or non-numeric (NetworkX's named-weight-with-default convention, e.g. `weight="cost"`).
pub struct AttrWeight<'g, G> {
    pub graph: &'g G,
    pub name: &'g str,
    pub default: f64,
}

impl<'g, G: AttrView> EdgeWeights for AttrWeight<'g, G> {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.graph.edge_attrs(edge.id).and_then(|attrs| attrs.get(self.name)).and_then(PropertyValue::as_f64).unwrap_or(self.default)
    }
}

impl<F: Fn(EdgeRef) -> f64> EdgeWeights for F {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self(edge)
    }
}

impl<P: PortModel, D: Directedness> GraphView for Storage<P, D> {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.keys().copied()
    }
    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains_key(&node)
    }
    fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// 📇️ One `EdgeRef` per stored edge, in `EdgeId` order — a self-loop appears once here even though it counts twice towards `degree`.
    fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.edges.iter().map(|(&id, record)| EdgeRef { id, u: self.endpoint_node(record.source), v: self.endpoint_node(record.target) })
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.successors.get(&node).into_iter().flat_map(|m| m.keys().copied())
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let map = if D::DIRECTED { &self.predecessors } else { &self.successors };
        map.get(&node).into_iter().flat_map(|m| m.keys().copied())
    }
    fn degree(&self, node: NodeId) -> usize {
        if D::DIRECTED {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.successors.get(&node).map_or(0, |m| m.values().map(Vec::len).sum())
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if D::DIRECTED {
            self.predecessors.get(&node).map_or(0, |m| m.values().map(Vec::len).sum())
        } else {
            self.out_degree(node)
        }
    }
    fn is_directed(&self) -> bool {
        D::DIRECTED
    }
    fn is_multigraph(&self) -> bool {
        P::MULTI_EDGES
    }
    fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.successors.get(&u).and_then(|m| m.get(&v)).into_iter().flatten().copied().map(move |id| EdgeRef { id, u, v })
    }
}

impl<P: PortModel, D: Directedness> AttrView for Storage<P, D> {
    fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.nodes.get(&node).map(|r| &r.attrs)
    }
    fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.edges.get(&edge).map(|r| &r.attrs)
    }
    fn graph_attrs(&self) -> &PropertyBag {
        &self.graph_attrs
    }
}

/// ⚖️ Reads the graph's own `PropertyBag["weight"]` on each edge, defaulting to `1.0` — the common case; use `AttrWeight`/`UnitWeight`/a closure for anything else.
impl<P: PortModel, D: Directedness> EdgeWeights for Storage<P, D> {
    fn weight(&self, edge: EdgeRef) -> f64 {
        self.edge_attrs(edge.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64).unwrap_or(1.0)
    }
}
// #endregion 🔖️View traits

// #region 🔖️Csr
/// 🧊️ Frozen, index-based CSR adjacency snapshot for hot algorithms; supersedes the ad-hoc `algorithms::Adjacency` for NEW code (that type is left untouched — old call sites keep using it). Node index assignment is `0..n` in sorted `NodeId` order, so two snapshots of the same graph always assign the same indices.
#[derive(Clone, Debug)]
pub struct Csr {
    node_ids: Vec<NodeId>,
    node_index: BTreeMap<NodeId, usize>,
    out_starts: Vec<usize>,
    out_targets: Vec<usize>,
    out_edge_ids: Vec<EdgeId>,
    in_starts: Vec<usize>,
    in_targets: Vec<usize>,
}

impl Csr {
    /// 🏗️ Builds a CSR snapshot from any `GraphView`; each node's out-neighbor slot is sorted by `(target index, edge id)` for determinism under parallel edges. `in_neighbors` is populated only for directed views (empty slots otherwise).
    pub fn from_view(view: &impl GraphView) -> Self {
        let mut node_ids: Vec<NodeId> = view.nodes().collect();
        node_ids.sort_unstable();
        let node_index: BTreeMap<NodeId, usize> = node_ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();
        let n = node_ids.len();
        let directed = view.is_directed();

        let mut out_buckets: Vec<Vec<(usize, EdgeId)>> = vec![Vec::new(); n];
        let mut in_buckets: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in view.edges() {
            let (Some(&ui), Some(&vi)) = (node_index.get(&edge.u), node_index.get(&edge.v)) else {
                continue;
            };
            out_buckets[ui].push((vi, edge.id));
            if directed {
                in_buckets[vi].push(ui);
            } else if ui != vi {
                out_buckets[vi].push((ui, edge.id));
            }
        }
        for bucket in &mut out_buckets {
            bucket.sort_unstable();
        }
        for bucket in &mut in_buckets {
            bucket.sort_unstable();
        }

        let mut out_starts = Vec::with_capacity(n + 1);
        let mut out_targets = Vec::new();
        let mut out_edge_ids = Vec::new();
        out_starts.push(0);
        for bucket in &out_buckets {
            for &(target, edge_id) in bucket {
                out_targets.push(target);
                out_edge_ids.push(edge_id);
            }
            out_starts.push(out_targets.len());
        }

        let mut in_starts = Vec::with_capacity(n + 1);
        let mut in_targets = Vec::new();
        in_starts.push(0);
        for bucket in &in_buckets {
            in_targets.extend(bucket.iter().copied());
            in_starts.push(in_targets.len());
        }

        Self { node_ids, node_index, out_starts, out_targets, out_edge_ids, in_starts, in_targets }
    }

    pub fn node_count(&self) -> usize {
        self.node_ids.len()
    }

    pub fn out_neighbors(&self, i: usize) -> &[usize] {
        &self.out_targets[self.out_starts[i]..self.out_starts[i + 1]]
    }

    pub fn in_neighbors(&self, i: usize) -> &[usize] {
        &self.in_targets[self.in_starts[i]..self.in_starts[i + 1]]
    }

    pub fn out_edges(&self, i: usize) -> &[EdgeId] {
        &self.out_edge_ids[self.out_starts[i]..self.out_starts[i + 1]]
    }

    pub fn node_of(&self, i: usize) -> Option<NodeId> {
        self.node_ids.get(i).copied()
    }

    pub fn index_of(&self, id: NodeId) -> Option<usize> {
        self.node_index.get(&id).copied()
    }
}
// #endregion 🔖️Csr

// #region 🔖️Views
/// 🪟️ Read-only borrowed views over any `GraphView`. Deliberately excluded: NetworkX's mutable attribute-sharing views (`G.subgraph()` et al. alias the parent's attribute dicts) — that aliasing pattern doesn't fit Rust ownership, so every view here only ever borrows. Callers who need an owned, mutated copy build one explicitly (a `.copy()`-style constructor lives on the per-kind facade crates from a later wave); these types just leave that seam open.
///
/// 🔎️ Restricts a graph to a node subset; an edge is included only when both endpoints are in the subset.
pub struct SubgraphView<'g, G: GraphView> {
    graph: &'g G,
    nodes: BTreeSet<NodeId>,
}

impl<'g, G: GraphView> SubgraphView<'g, G> {
    pub fn new(graph: &'g G, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        Self { graph, nodes: nodes.into_iter().filter(|&n| graph.contains_node(n)).collect() }
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
        self.out_neighbors(node).map(|nb| self.edges_between(node, nb).count()).sum()
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.in_neighbors(node).map(|nb| self.edges_between(nb, node).count()).sum()
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
    graph: &'g G,
    edges: BTreeSet<EdgeId>,
    nodes: BTreeSet<NodeId>,
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
        self.out_neighbors(node).map(|nb| self.edges_between(node, nb).count()).sum()
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.in_neighbors(node).map(|nb| self.edges_between(nb, node).count()).sum()
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
    graph: &'g G,
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
    graph: &'g G,
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
        self.graph.edges().filter(move |&e| self.keep(e))
    }
    fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node)
    }
    fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let node_ok = (self.keep_node)(node);
        self.graph.out_neighbors(node).filter(move |&nb| node_ok && self.edges_between(node, nb).next().is_some())
    }
    fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let node_ok = (self.keep_node)(node);
        self.graph.in_neighbors(node).filter(move |&nb| node_ok && self.edges_between(nb, node).next().is_some())
    }
    fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.out_degree(node) + self.in_degree(node)
        } else {
            self.out_degree(node)
        }
    }
    fn out_degree(&self, node: NodeId) -> usize {
        self.out_neighbors(node).map(|nb| self.edges_between(node, nb).count()).sum()
    }
    fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed() {
            self.in_neighbors(node).map(|nb| self.edges_between(nb, node).count()).sum()
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
    graph: &'g G,
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
        self.neighbors(node).map(|nb| self.edges_between(node, nb).count()).sum()
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

// #region 🔖️Interner
/// 🔤️ Generalized, bidirectional label<->`NodeId` map — the generic successor to the string-only `algorithms::IdIndex` (which stays untouched for old call sites). `intern` is idempotent: the same label always maps to the same id.
#[derive(Clone, Debug, Default)]
pub struct Interner<L: Ord + Clone + std::hash::Hash> {
    labels: Vec<L>,
    by_label: std::collections::HashMap<L, NodeId>,
}

impl<L: Ord + Clone + std::hash::Hash> Interner<L> {
    pub fn new() -> Self {
        Self { labels: Vec::new(), by_label: std::collections::HashMap::new() }
    }

    /// 🏗️ Builds an interner from labels sorted for deterministic id assignment; duplicate labels collapse to one id.
    pub fn from_labels(labels: impl IntoIterator<Item = L>) -> Self {
        let mut sorted: Vec<L> = labels.into_iter().collect();
        sorted.sort();
        sorted.dedup();
        let mut interner = Self::new();
        for label in sorted {
            interner.intern(label);
        }
        interner
    }

    /// ➕️ Returns the existing id for `label` if already interned, otherwise allocates the next sequential id.
    pub fn intern(&mut self, label: L) -> NodeId {
        if let Some(&id) = self.by_label.get(&label) {
            return id;
        }
        let id = self.labels.len() as NodeId;
        self.labels.push(label.clone());
        self.by_label.insert(label, id);
        id
    }

    pub fn label_of(&self, id: NodeId) -> Option<&L> {
        self.labels.get(id as usize)
    }

    pub fn id_of(&self, label: &L) -> Option<NodeId> {
        self.by_label.get(label).copied()
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}
// #endregion 🔖️Interner

// #region 🔖️GraphError
/// 🚨️ Flat, non-generic error enum mirroring the NetworkX exception hierarchy; every downstream algorithm crate returns `Result<_, GraphError>`. Nothing here is generic over node/edge label types — everything is `NodeId`/`EdgeId`/`u64`/`String` — so this shape stays stable across the whole family.
#[derive(Clone, Debug, PartialEq)]
pub enum GraphError {
    NodeNotFound(NodeId),
    EdgeNotFound(EdgeId),
    NoPath { source: NodeId, target: NodeId },
    HasACycle,
    NoCycle,
    Unfeasible(String),
    Unbounded(String),
    NotATree,
    NotAForest,
    NotBipartite,
    NotPlanar,
    NotEulerian,
    NotConnected,
    NotStronglyConnected,
    AmbiguousSolution(String),
    ExceededMaxIterations { iterations: usize },
    PowerIterationFailedConvergence { iterations: usize },
    NegativeCycle,
    NotGraphical(String),
    NotImplementedForKind { algorithm: &'static str, kind: &'static str },
    Io(String),
    Parse { line: usize, message: String },
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(id) => write!(f, "node {id} not found"),
            GraphError::EdgeNotFound(id) => write!(f, "edge {id} not found"),
            GraphError::NoPath { source, target } => write!(f, "no path from node {source} to node {target}"),
            GraphError::HasACycle => write!(f, "graph has a cycle"),
            GraphError::NoCycle => write!(f, "graph has no cycle"),
            GraphError::Unfeasible(msg) => write!(f, "unfeasible: {msg}"),
            GraphError::Unbounded(msg) => write!(f, "unbounded: {msg}"),
            GraphError::NotATree => write!(f, "graph is not a tree"),
            GraphError::NotAForest => write!(f, "graph is not a forest"),
            GraphError::NotBipartite => write!(f, "graph is not bipartite"),
            GraphError::NotPlanar => write!(f, "graph is not planar"),
            GraphError::NotEulerian => write!(f, "graph is not eulerian"),
            GraphError::NotConnected => write!(f, "graph is not connected"),
            GraphError::NotStronglyConnected => write!(f, "graph is not strongly connected"),
            GraphError::AmbiguousSolution(msg) => write!(f, "ambiguous solution: {msg}"),
            GraphError::ExceededMaxIterations { iterations } => write!(f, "exceeded max iterations ({iterations})"),
            GraphError::PowerIterationFailedConvergence { iterations } => {
                write!(f, "power iteration failed to converge after {iterations} iterations")
            }
            GraphError::NegativeCycle => write!(f, "graph has a negative cycle"),
            GraphError::NotGraphical(msg) => write!(f, "not a graphical degree sequence: {msg}"),
            GraphError::NotImplementedForKind { algorithm, kind } => write!(f, "{algorithm} is not implemented for {kind}"),
            GraphError::Io(msg) => write!(f, "io error: {msg}"),
            GraphError::Parse { line, message } => write!(f, "parse error at line {line}: {message}"),
        }
    }
}

impl std::error::Error for GraphError {}
// #endregion 🔖️GraphError

// #region 🔖️Utils
/// 🎚️ Strict numeric tolerance for exact-equality-sensitive comparisons (e.g. verifying a closed-form result).
pub const TOL_STRICT: f64 = 1e-9;
/// 🎚️ Loose numeric tolerance for iterative/approximate algorithm convergence checks.
pub const TOL_LOOSE: f64 = 1e-6;

/// 🔗️ Consecutive-pair iterator: `[a, b, c] -> [(a, b), (b, c)]`.
pub fn pairwise<T: Copy>(items: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    items.windows(2).map(|w| (w[0], w[1]))
}

/// 🎯️ Deterministic representative element (the first one) from a slice.
pub fn arbitrary_element<T: Copy>(items: &[T]) -> Option<T> {
    items.first().copied()
}

/// 🗳️ Binary-heap priority queue with `decrease_key`, ordered by `K` and keyed by `V` identity; a position index makes membership/decrease `O(log n)` instead of the `O(n)` a plain `BinaryHeap` needs for those operations.
#[derive(Clone, Debug)]
pub struct MappedHeap<K: Ord, V: Eq + std::hash::Hash + Clone> {
    heap: Vec<(K, V)>,
    position: std::collections::HashMap<V, usize>,
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> Default for MappedHeap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> MappedHeap<K, V> {
    pub fn new() -> Self {
        Self { heap: Vec::new(), position: std::collections::HashMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn contains(&self, item: &V) -> bool {
        self.position.contains_key(item)
    }

    /// ➕️ Pushes `item` at `priority` if absent, or decreases its priority if `priority` is lower than its current one; no-operation if `item` is present with an already-lower-or-equal priority.
    pub fn push_or_decrease(&mut self, item: V, priority: K) {
        if let Some(&i) = self.position.get(&item) {
            if priority < self.heap[i].0 {
                self.heap[i].0 = priority;
                self.sift_up(i);
            }
        } else {
            self.heap.push((priority, item.clone()));
            let i = self.heap.len() - 1;
            self.position.insert(item, i);
            self.sift_up(i);
        }
    }

    /// 🔽️ Lowers `item`'s priority; returns `false` (no-operation) if `item` isn't present or `priority` isn't lower than its current one.
    pub fn decrease_key(&mut self, item: &V, priority: K) -> bool {
        let Some(&i) = self.position.get(item) else { return false };
        if priority < self.heap[i].0 {
            self.heap[i].0 = priority;
            self.sift_up(i);
            true
        } else {
            false
        }
    }

    pub fn pop_min(&mut self) -> Option<(K, V)> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.swap(0, last);
        let (priority, item) = self.heap.pop().expect("heap checked non-empty above");
        self.position.remove(&item);
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        Some((priority, item))
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.position.insert(self.heap[i].1.clone(), i);
        self.position.insert(self.heap[j].1.clone(), j);
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i].0 < self.heap[parent].0 {
                self.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut smallest = i;
            if l < n && self.heap[l].0 < self.heap[smallest].0 {
                smallest = l;
            }
            if r < n && self.heap[r].0 < self.heap[smallest].0 {
                smallest = r;
            }
            if smallest == i {
                break;
            }
            self.swap(i, smallest);
            i = smallest;
        }
    }
}
// #endregion 🔖️Utils

// #region 🔖️Algorithms
pub mod algorithms {
    //! 🧮️ Index-based graph algorithms: traversal, ordering, cycles, components, shortest paths.

    use std::collections::HashMap;

    // #region 🔖️Adjacency
    /// 🧮️ Compact adjacency built once per query batch.
    #[derive(Clone, Debug)]
    pub struct Adjacency {
        n: usize,
        out: Vec<Vec<usize>>,
        inc: Vec<Vec<usize>>,
    }

    impl Adjacency {
        pub fn node_count(&self) -> usize {
            self.n
        }
        pub fn out_neighbors(&self, i: usize) -> &[usize] {
            &self.out[i]
        }
        pub fn in_neighbors(&self, i: usize) -> &[usize] {
            &self.inc[i]
        }
    }

    /// 🧮️ Builds adjacency lists from index edges; `directed` controls whether reverse edges are also recorded as out-edges.
    pub fn adjacency(node_count: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
        let mut out = vec![Vec::new(); node_count];
        let mut inc = vec![Vec::new(); node_count];
        for &(a, b) in edges {
            if a >= node_count || b >= node_count {
                continue;
            }
            out[a].push(b);
            inc[b].push(a);
            if !directed {
                out[b].push(a);
                inc[a].push(b);
            }
        }
        Adjacency { n: node_count, out, inc }
    }
    // #endregion 🔖️Adjacency

    // #region 🔖️IdIndex
    /// 🔤️ Deterministic string-id <-> index bridge (ids sorted for reproducible ordering).
    #[derive(Clone, Debug, Default)]
    pub struct IdIndex {
        ids: Vec<String>,
        index: HashMap<String, usize>,
    }

    impl IdIndex {
        pub fn from_ids<'a>(ids: impl Iterator<Item = &'a str>) -> Self {
            let mut sorted: Vec<String> = ids.map(|s| s.to_string()).collect();
            sorted.sort();
            sorted.dedup();
            let index = sorted.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
            Self { ids: sorted, index }
        }

        pub fn from_edges<'a>(edges: impl Iterator<Item = (&'a str, &'a str)>) -> Self {
            let mut all: Vec<String> = Vec::new();
            for (a, b) in edges {
                all.push(a.to_string());
                all.push(b.to_string());
            }
            Self::from_ids(all.iter().map(|s| s.as_str()))
        }

        pub fn index_of(&self, id: &str) -> Option<usize> {
            self.index.get(id).copied()
        }

        pub fn id_of(&self, index: usize) -> Option<&str> {
            self.ids.get(index).map(|s| s.as_str())
        }

        pub fn len(&self) -> usize {
            self.ids.len()
        }

        pub fn is_empty(&self) -> bool {
            self.ids.is_empty()
        }

        pub fn edges_to_indices(&self, edges: &[(String, String)]) -> Vec<(usize, usize)> {
            edges.iter().filter_map(|(a, b)| Some((self.index_of(a)?, self.index_of(b)?))).collect()
        }
    }
    // #endregion 🔖️IdIndex

    // #region 🔖️Traversal
    /// 🌊️ Breadth-first visitation order from the given seeds.
    pub fn bfs_order(adj: &Adjacency, seeds: &[usize]) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        for &s in seeds {
            if s < adj.n && !visited[s] {
                visited[s] = true;
                queue.push_back(s);
            }
        }
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj.out[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        order
    }

    /// 🌊️ Breadth-first layers (distance bands) from the given seeds.
    pub fn bfs_layers(adj: &Adjacency, seeds: &[usize]) -> Vec<Vec<usize>> {
        let mut visited = vec![false; adj.n];
        let mut layers = Vec::new();
        let mut frontier: Vec<usize> = seeds.iter().copied().filter(|&s| s < adj.n).collect();
        for &s in &frontier {
            visited[s] = true;
        }
        while !frontier.is_empty() {
            layers.push(frontier.clone());
            let mut next = Vec::new();
            for &u in &frontier {
                for &v in &adj.out[u] {
                    if !visited[v] {
                        visited[v] = true;
                        next.push(v);
                    }
                }
            }
            frontier = next;
        }
        layers
    }

    /// 📏️ Unweighted BFS distance from a single seed to every reachable node.
    pub fn bfs_distances(adj: &Adjacency, seed: usize) -> Vec<Option<u32>> {
        let mut dist = vec![None; adj.n];
        if seed >= adj.n {
            return dist;
        }
        dist[seed] = Some(0);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(seed);
        while let Some(u) = queue.pop_front() {
            let du = dist[u].expect("every queued node was assigned a distance before being pushed");
            for &v in &adj.out[u] {
                if dist[v].is_none() {
                    dist[v] = Some(du + 1);
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    /// 🌲️ Depth-first preorder from a single seed.
    pub fn dfs_preorder(adj: &Adjacency, seed: usize) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        if seed >= adj.n {
            return order;
        }
        let mut stack = vec![seed];
        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            order.push(u);
            for &v in adj.out[u].iter().rev() {
                if !visited[v] {
                    stack.push(v);
                }
            }
        }
        order
    }

    /// 🌲️ Depth-first postorder from a single seed.
    pub fn dfs_postorder(adj: &Adjacency, seed: usize) -> Vec<usize> {
        let mut visited = vec![false; adj.n];
        let mut order = Vec::new();
        if seed >= adj.n {
            return order;
        }
        fn visit(u: usize, adj: &Adjacency, visited: &mut [bool], order: &mut Vec<usize>) {
            visited[u] = true;
            for &v in &adj.out[u] {
                if !visited[v] {
                    visit(v, adj, visited, order);
                }
            }
            order.push(u);
        }
        visit(seed, adj, &mut visited, &mut order);
        order
    }
    // #endregion 🔖️Traversal

    // #region 🔖️Ordering
    /// ⚠️ A cycle was found where a DAG was required; `cycle` lists the node indices on the cycle.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct CycleError {
        pub cycle: Vec<usize>,
    }

    /// 🔢️ Kahn's algorithm topological sort; index-ascending tie-break for determinism.
    pub fn topo_sort(adj: &Adjacency) -> Result<Vec<usize>, CycleError> {
        let mut in_deg = vec![0usize; adj.n];
        for list in &adj.out {
            for &v in list {
                in_deg[v] += 1;
            }
        }
        let mut heap = std::collections::BinaryHeap::new();
        for (i, &deg) in in_deg.iter().enumerate() {
            if deg == 0 {
                heap.push(std::cmp::Reverse(i));
            }
        }
        let mut order = Vec::with_capacity(adj.n);
        while let Some(std::cmp::Reverse(u)) = heap.pop() {
            order.push(u);
            for &v in &adj.out[u] {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    heap.push(std::cmp::Reverse(v));
                }
            }
        }
        if order.len() == adj.n {
            Ok(order)
        } else {
            let remaining: Vec<usize> = (0..adj.n).filter(|&i| in_deg[i] > 0).collect();
            Err(CycleError { cycle: find_cycle_among(adj, &remaining).unwrap_or(remaining) })
        }
    }

    /// 🪜️ Topological levels: each level contains nodes whose dependencies are all in earlier levels.
    pub fn topo_levels(adj: &Adjacency) -> Result<Vec<Vec<usize>>, CycleError> {
        let mut in_deg = vec![0usize; adj.n];
        for list in &adj.out {
            for &v in list {
                in_deg[v] += 1;
            }
        }
        let mut levels = Vec::new();
        let mut remaining = in_deg.clone();
        let mut placed = vec![false; adj.n];
        let mut placed_count = 0;
        loop {
            let mut frontier: Vec<usize> = (0..adj.n).filter(|&i| !placed[i] && remaining[i] == 0).collect();
            if frontier.is_empty() {
                break;
            }
            frontier.sort_unstable();
            for &u in &frontier {
                placed[u] = true;
                placed_count += 1;
            }
            for &u in &frontier {
                for &v in &adj.out[u] {
                    remaining[v] -= 1;
                }
            }
            levels.push(frontier);
        }
        if placed_count == adj.n {
            Ok(levels)
        } else {
            let unplaced: Vec<usize> = (0..adj.n).filter(|&i| !placed[i]).collect();
            Err(CycleError { cycle: find_cycle_among(adj, &unplaced).unwrap_or(unplaced) })
        }
    }

    /// 🪜️ Longest-path layer index per node (DAG layering for hierarchical drawing); layer 0 = roots.
    pub fn longest_path_layers(adj: &Adjacency) -> Result<Vec<u32>, CycleError> {
        let levels = topo_levels(adj)?;
        let mut layer = vec![0u32; adj.n];
        for (li, level) in levels.iter().enumerate() {
            for &u in level {
                layer[u] = li as u32;
            }
        }
        Ok(layer)
    }
    // #endregion 🔖️Ordering

    // #region 🔖️Cycles
    /// 🔎️ Whether `to` is reachable from `from` following out-edges.
    pub fn is_reachable(adj: &Adjacency, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }
        bfs_order(adj, &[from]).contains(&to)
    }

    /// ➕️ Whether adding an edge `source -> target` would create a cycle (i.e. `target` can already reach `source`).
    pub fn would_create_cycle(adj: &Adjacency, source: usize, target: usize) -> bool {
        source == target || is_reachable(adj, target, source)
    }

    /// ➕️ String-id convenience: whether adding `source -> target` to `existing` directed edges would create a cycle.
    pub fn would_create_cycle_ids(existing: &[(String, String)], source: &str, target: &str) -> bool {
        if source == target {
            return true;
        }
        let index = IdIndex::from_edges(existing.iter().map(|(a, b)| (a.as_str(), b.as_str())));
        let (Some(s), Some(t)) = (index.index_of(source), index.index_of(target)) else {
            return false;
        };
        let adj = adjacency(index.len(), &index.edges_to_indices(existing), true);
        would_create_cycle(&adj, s, t)
    }

    /// ➕️ Batched acyclic filter: for each `candidates[i]`, whether adding it to `existing` (+ prior accepted candidates) keeps the graph acyclic.
    pub fn acyclic_edge_subset(existing: &[(String, String)], candidates: &[(String, String)]) -> Vec<bool> {
        let all_ids = existing.iter().chain(candidates.iter()).flat_map(|(a, b)| [a.as_str(), b.as_str()]);
        let index = IdIndex::from_ids(all_ids);
        let mut edges = index.edges_to_indices(existing);
        let mut accepted = Vec::with_capacity(candidates.len());
        for (a, b) in candidates {
            let (Some(s), Some(t)) = (index.index_of(a), index.index_of(b)) else {
                accepted.push(false);
                continue;
            };
            let adj = adjacency(index.len(), &edges, true);
            if would_create_cycle(&adj, s, t) {
                accepted.push(false);
            } else {
                edges.push((s, t));
                accepted.push(true);
            }
        }
        accepted
    }

    fn find_cycle_among(adj: &Adjacency, candidates: &[usize]) -> Option<Vec<usize>> {
        let mut color = vec![0u8; adj.n];
        let mut path = Vec::new();
        fn dfs(u: usize, adj: &Adjacency, color: &mut [u8], path: &mut Vec<usize>) -> Option<Vec<usize>> {
            color[u] = 1;
            path.push(u);
            for &v in &adj.out[u] {
                if color[v] == 1 {
                    let start = path.iter().position(|&x| x == v).expect("color[v] == 1 means v is currently on the open dfs path");
                    return Some(path[start..].to_vec());
                }
                if color[v] == 0 {
                    if let Some(cycle) = dfs(v, adj, color, path) {
                        return Some(cycle);
                    }
                }
            }
            path.pop();
            color[u] = 2;
            None
        }
        for &start in candidates {
            if color[start] == 0 {
                if let Some(cycle) = dfs(start, adj, &mut color, &mut path) {
                    return Some(cycle);
                }
            }
        }
        None
    }

    /// 🔎️ Finds one cycle in the graph, if any exist.
    pub fn find_cycle(adj: &Adjacency) -> Option<Vec<usize>> {
        let all: Vec<usize> = (0..adj.n).collect();
        find_cycle_among(adj, &all)
    }
    // #endregion 🔖️Cycles

    // #region 🔖️Components
    /// 🧮️ Union-find (disjoint-set) with path compression and union-by-rank.
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<u8>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self { parent: (0..n).collect(), rank: vec![0; n] }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]);
            }
            self.parent[x]
        }

        pub fn union(&mut self, a: usize, b: usize) {
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

        pub fn same_set(&mut self, a: usize, b: usize) -> bool {
            self.find(a) == self.find(b)
        }
    }

    /// 🧩️ Weak connected-component id per node (undirected reachability, works for directed adjacency too).
    pub fn connected_components(adj: &Adjacency) -> Vec<usize> {
        let mut uf = UnionFind::new(adj.n);
        for u in 0..adj.n {
            for &v in &adj.out[u] {
                uf.union(u, v);
            }
        }
        let mut root_to_component: HashMap<usize, usize> = HashMap::new();
        let mut labels = vec![0usize; adj.n];
        for (u, label) in labels.iter_mut().enumerate() {
            let root = uf.find(u);
            let next_id = root_to_component.len();
            let id = *root_to_component.entry(root).or_insert(next_id);
            *label = id;
        }
        labels
    }

    /// 🧩️ Tarjan's strongly connected components; returned in reverse-topological order, nodes sorted within each.
    pub fn strongly_connected_components(adj: &Adjacency) -> Vec<Vec<usize>> {
        struct State {
            index: Vec<Option<u32>>,
            lowlink: Vec<u32>,
            on_stack: Vec<bool>,
            stack: Vec<usize>,
            counter: u32,
            out: Vec<Vec<usize>>,
        }
        fn strongconnect(u: usize, adj: &Adjacency, st: &mut State) {
            st.index[u] = Some(st.counter);
            st.lowlink[u] = st.counter;
            st.counter += 1;
            st.stack.push(u);
            st.on_stack[u] = true;
            for &v in &adj.out[u] {
                if st.index[v].is_none() {
                    strongconnect(v, adj, st);
                    st.lowlink[u] = st.lowlink[u].min(st.lowlink[v]);
                } else if st.on_stack[v] {
                    st.lowlink[u] = st.lowlink[u].min(st.index[v].expect("on_stack[v] implies index[v] was assigned when v was first visited"));
                }
            }
            if st.lowlink[u] == st.index[u].expect("index[u] was assigned at the start of this strongconnect call") {
                let mut component = Vec::new();
                loop {
                    let w = st.stack.pop().expect("u is still on the tarjan stack until its own component is popped");
                    st.on_stack[w] = false;
                    component.push(w);
                    if w == u {
                        break;
                    }
                }
                component.sort_unstable();
                st.out.push(component);
            }
        }
        let mut st = State { index: vec![None; adj.n], lowlink: vec![0; adj.n], on_stack: vec![false; adj.n], stack: Vec::new(), counter: 0, out: Vec::new() };
        for u in 0..adj.n {
            if st.index[u].is_none() {
                strongconnect(u, adj, &mut st);
            }
        }
        st.out
    }

    /// ⬇️ In-degree per node.
    pub fn in_degrees(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).map(|i| adj.inc[i].len()).collect()
    }

    /// ⬆️ Out-degree per node.
    pub fn out_degrees(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).map(|i| adj.out[i].len()).collect()
    }

    /// 🌱️ Indices of nodes with in-degree 0 (DAG roots).
    pub fn root_indices(adj: &Adjacency) -> Vec<usize> {
        (0..adj.n).filter(|&i| adj.inc[i].is_empty()).collect()
    }
    // #endregion 🔖️Components

    // #region 🔖️Paths
    /// 📏️ Shortest path (by hop count) between two nodes, if reachable.
    pub fn shortest_path_unweighted(adj: &Adjacency, from: usize, to: usize) -> Option<Vec<usize>> {
        if from >= adj.n || to >= adj.n {
            return None;
        }
        let mut visited = vec![false; adj.n];
        let mut parent = vec![usize::MAX; adj.n];
        visited[from] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(from);
        while let Some(u) = queue.pop_front() {
            if u == to {
                let mut path = vec![to];
                let mut cur = to;
                while cur != from {
                    cur = parent[cur];
                    path.push(cur);
                }
                path.reverse();
                return Some(path);
            }
            for &v in &adj.out[u] {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }
        None
    }

    /// 📏️ Dijkstra shortest distances from `from` to every node, given non-negative edge weights parallel to adjacency out-edges.
    pub fn dijkstra(adj: &Adjacency, weights: &HashMap<(usize, usize), f64>, from: usize) -> Vec<Option<f64>> {
        let mut dist = vec![None; adj.n];
        if from >= adj.n {
            return dist;
        }
        dist[from] = Some(0.0);
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(OrderedFloat(0.0, from)));
        while let Some(std::cmp::Reverse(OrderedFloat(d, u))) = heap.pop() {
            if dist[u].is_none_or(|cur| d > cur) {
                continue;
            }
            for &v in &adj.out[u] {
                let w = weights.get(&(u, v)).copied().unwrap_or(1.0);
                let nd = d + w;
                if dist[v].is_none_or(|cur| nd < cur) {
                    dist[v] = Some(nd);
                    heap.push(std::cmp::Reverse(OrderedFloat(nd, v)));
                }
            }
        }
        dist
    }

    /// 📏️ Dijkstra shortest path and distance between two nodes, if reachable.
    pub fn dijkstra_path(adj: &Adjacency, weights: &HashMap<(usize, usize), f64>, from: usize, to: usize) -> Option<(Vec<usize>, f64)> {
        if from >= adj.n || to >= adj.n {
            return None;
        }
        let mut dist = vec![None; adj.n];
        let mut parent = vec![usize::MAX; adj.n];
        dist[from] = Some(0.0);
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse(OrderedFloat(0.0, from)));
        while let Some(std::cmp::Reverse(OrderedFloat(d, u))) = heap.pop() {
            if dist[u].is_none_or(|cur| d > cur) {
                continue;
            }
            if u == to {
                let mut path = vec![to];
                let mut cur = to;
                while cur != from {
                    cur = parent[cur];
                    path.push(cur);
                }
                path.reverse();
                return Some((path, d));
            }
            for &v in &adj.out[u] {
                let w = weights.get(&(u, v)).copied().unwrap_or(1.0);
                let nd = d + w;
                if dist[v].is_none_or(|cur| nd < cur) {
                    dist[v] = Some(nd);
                    parent[v] = u;
                    heap.push(std::cmp::Reverse(OrderedFloat(nd, v)));
                }
            }
        }
        None
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct OrderedFloat(f64, usize);
    impl Eq for OrderedFloat {}
    impl PartialOrd for OrderedFloat {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for OrderedFloat {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal).then(self.1.cmp(&other.1))
        }
    }

    /// 🌲️ Kruskal minimum spanning tree; returns the indices (into `edges`) of the selected edges.
    pub fn minimum_spanning_tree(node_count: usize, edges: &[(usize, usize, f64)]) -> Vec<usize> {
        let mut order: Vec<usize> = (0..edges.len()).collect();
        order.sort_by(|&a, &b| edges[a].2.partial_cmp(&edges[b].2).unwrap_or(std::cmp::Ordering::Equal));
        let mut uf = UnionFind::new(node_count);
        let mut selected = Vec::new();
        for i in order {
            let (a, b, _) = edges[i];
            if a >= node_count || b >= node_count {
                continue;
            }
            if !uf.same_set(a, b) {
                uf.union(a, b);
                selected.push(i);
            }
        }
        selected
    }
    // #endregion 🔖️Paths

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        fn adj_from(n: usize, edges: &[(usize, usize)], directed: bool) -> Adjacency {
            adjacency(n, edges, directed)
        }

        #[test]
        fn bfs_order_visits_reachable_nodes_breadth_first() {
            let adj = adj_from(5, &[(0, 1), (0, 2), (1, 3), (2, 4)], true);
            let order = bfs_order(&adj, &[0]);
            assert_eq!(order, vec![0, 1, 2, 3, 4]);
        }

        #[test]
        fn bfs_layers_group_by_distance() {
            let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3)], true);
            let layers = bfs_layers(&adj, &[0]);
            assert_eq!(layers, vec![vec![0], vec![1, 2], vec![3]]);
        }

        #[test]
        fn bfs_distances_unreachable_is_none() {
            let adj = adj_from(3, &[(0, 1)], true);
            let dist = bfs_distances(&adj, 0);
            assert_eq!(dist, vec![Some(0), Some(1), None]);
        }

        #[test]
        fn dfs_preorder_and_postorder_agree_on_leaf_first_last() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert_eq!(dfs_preorder(&adj, 0), vec![0, 1, 2]);
            assert_eq!(dfs_postorder(&adj, 0), vec![2, 1, 0]);
        }

        #[test]
        fn topo_sort_orders_dependencies_before_dependents() {
            let adj = adj_from(4, &[(0, 1), (0, 2), (1, 3), (2, 3)], true);
            let order = topo_sort(&adj).expect("acyclic");
            let pos = |n: usize| order.iter().position(|&x| x == n).unwrap();
            assert!(pos(0) < pos(1));
            assert!(pos(1) < pos(3));
            assert!(pos(2) < pos(3));
        }

        #[test]
        fn topo_sort_detects_cycle() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            let err = topo_sort(&adj).unwrap_err();
            assert_eq!(err.cycle.len(), 3);
        }

        #[test]
        fn topo_levels_groups_independent_nodes() {
            let adj = adj_from(4, &[(0, 2), (1, 2), (2, 3)], true);
            let levels = topo_levels(&adj).expect("acyclic");
            assert_eq!(levels[0], vec![0, 1]);
            assert_eq!(levels[1], vec![2]);
            assert_eq!(levels[2], vec![3]);
        }

        #[test]
        fn longest_path_layers_assigns_root_layer_zero() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            let layers = longest_path_layers(&adj).expect("acyclic");
            assert_eq!(layers, vec![0, 1, 2]);
        }

        #[test]
        fn would_create_cycle_detects_back_edge() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert!(would_create_cycle(&adj, 2, 0));
            assert!(!would_create_cycle(&adj, 0, 2));
        }

        #[test]
        fn would_create_cycle_ids_matches_index_version() {
            let existing = vec![("a".to_string(), "b".to_string()), ("b".to_string(), "c".to_string())];
            assert!(would_create_cycle_ids(&existing, "c", "a"));
            assert!(!would_create_cycle_ids(&existing, "a", "c"));
        }

        #[test]
        fn acyclic_edge_subset_accumulates_accepted_candidates() {
            let existing = vec![("a".to_string(), "b".to_string())];
            let candidates = vec![("b".to_string(), "c".to_string()), ("c".to_string(), "a".to_string()), ("c".to_string(), "d".to_string())];
            let accepted = acyclic_edge_subset(&existing, &candidates);
            assert_eq!(accepted, vec![true, false, true]);
        }

        #[test]
        fn find_cycle_returns_none_for_dag() {
            let adj = adj_from(3, &[(0, 1), (1, 2)], true);
            assert!(find_cycle(&adj).is_none());
        }

        #[test]
        fn find_cycle_returns_some_for_cyclic_graph() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            assert!(find_cycle(&adj).is_some());
        }

        #[test]
        fn connected_components_groups_weak_components() {
            let adj = adj_from(5, &[(0, 1), (1, 2), (3, 4)], true);
            let labels = connected_components(&adj);
            assert_eq!(labels[0], labels[1]);
            assert_eq!(labels[1], labels[2]);
            assert_eq!(labels[3], labels[4]);
            assert_ne!(labels[0], labels[3]);
        }

        #[test]
        fn strongly_connected_components_finds_cycle_as_one_component() {
            let adj = adj_from(4, &[(0, 1), (1, 2), (2, 0), (2, 3)], true);
            let sccs = strongly_connected_components(&adj);
            let cyclic = sccs.iter().find(|c| c.contains(&0)).unwrap();
            assert_eq!(cyclic, &vec![0, 1, 2]);
            assert!(sccs.iter().any(|c| c == &vec![3]));
        }

        #[test]
        fn degrees_and_roots_match_edge_shape() {
            let adj = adj_from(3, &[(0, 1), (0, 2)], true);
            assert_eq!(out_degrees(&adj), vec![2, 0, 0]);
            assert_eq!(in_degrees(&adj), vec![0, 1, 1]);
            assert_eq!(root_indices(&adj), vec![0]);
        }

        #[test]
        fn union_find_unions_and_queries_sets() {
            let mut uf = UnionFind::new(4);
            uf.union(0, 1);
            uf.union(2, 3);
            assert!(uf.same_set(0, 1));
            assert!(!uf.same_set(0, 2));
        }

        #[test]
        fn shortest_path_unweighted_finds_hop_path() {
            let adj = adj_from(4, &[(0, 1), (1, 3), (0, 2), (2, 3)], true);
            let path = shortest_path_unweighted(&adj, 0, 3).expect("reachable");
            assert_eq!(path.len(), 3);
            assert_eq!(path[0], 0);
            assert_eq!(*path.last().unwrap(), 3);
        }

        #[test]
        fn shortest_path_unweighted_none_when_unreachable() {
            let adj = adj_from(3, &[(0, 1)], true);
            assert!(shortest_path_unweighted(&adj, 0, 2).is_none());
        }

        #[test]
        fn dijkstra_prefers_cheaper_longer_path() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
            let mut weights = HashMap::new();
            weights.insert((0, 1), 1.0);
            weights.insert((1, 2), 1.0);
            weights.insert((0, 2), 5.0);
            let dist = dijkstra(&adj, &weights, 0);
            assert_eq!(dist[2], Some(2.0));
        }

        #[test]
        fn dijkstra_path_reconstructs_cheapest_route() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (0, 2)], true);
            let mut weights = HashMap::new();
            weights.insert((0, 1), 1.0);
            weights.insert((1, 2), 1.0);
            weights.insert((0, 2), 5.0);
            let (path, dist) = dijkstra_path(&adj, &weights, 0, 2).expect("reachable");
            assert_eq!(path, vec![0, 1, 2]);
            assert_eq!(dist, 2.0);
        }

        #[test]
        fn minimum_spanning_tree_selects_cheapest_edges_without_cycles() {
            let edges = vec![(0, 1, 1.0), (1, 2, 2.0), (0, 2, 3.0)];
            let selected = minimum_spanning_tree(3, &edges);
            assert_eq!(selected.len(), 2);
            assert!(selected.contains(&0));
            assert!(selected.contains(&1));
        }

        #[test]
        fn id_index_is_deterministic_and_sorted() {
            let edges = [("c".to_string(), "a".to_string()), ("a".to_string(), "b".to_string())];
            let index = IdIndex::from_edges(edges.iter().map(|(a, b)| (a.as_str(), b.as_str())));
            assert_eq!(index.id_of(0), Some("a"));
            assert_eq!(index.id_of(1), Some("b"));
            assert_eq!(index.id_of(2), Some("c"));
        }

        #[test]
        fn id_index_from_ids_dedupes_and_reports_len() {
            let index = IdIndex::from_ids(["b", "a", "a"].into_iter());
            assert_eq!(index.len(), 2);
            assert!(!index.is_empty());
            assert_eq!(index.index_of("a"), Some(0));
            assert_eq!(index.index_of("z"), None);
            assert!(IdIndex::from_ids(std::iter::empty()).is_empty());
        }

        #[test]
        fn adjacency_accessors_expose_node_count_and_neighbor_lists() {
            let adj = adj_from(3, &[(0, 1), (0, 2)], true);
            assert_eq!(adj.node_count(), 3);
            assert_eq!(adj.out_neighbors(0), &[1, 2]);
            assert_eq!(adj.in_neighbors(1), &[0]);
            assert!(adj.in_neighbors(0).is_empty());
        }

        #[test]
        fn bfs_order_ignores_out_of_range_seeds() {
            let adj = adj_from(2, &[(0, 1)], true);
            assert_eq!(bfs_order(&adj, &[5]), Vec::<usize>::new());
        }

        #[test]
        fn dfs_preorder_and_postorder_out_of_range_seed_returns_empty() {
            let adj = adj_from(2, &[(0, 1)], true);
            assert!(dfs_preorder(&adj, 9).is_empty());
            assert!(dfs_postorder(&adj, 9).is_empty());
        }

        #[test]
        fn topo_levels_detects_cycle() {
            let adj = adj_from(3, &[(0, 1), (1, 2), (2, 0)], true);
            let err = topo_levels(&adj).unwrap_err();
            assert_eq!(err.cycle.len(), 3);
        }

        #[test]
        fn longest_path_layers_propagates_cycle_error() {
            let adj = adj_from(2, &[(0, 1), (1, 0)], true);
            assert!(longest_path_layers(&adj).is_err());
        }

        #[test]
        fn would_create_cycle_self_loop_is_always_a_cycle() {
            let adj = adj_from(2, &[(0, 1)], true);
            assert!(would_create_cycle(&adj, 1, 1));
        }

        #[test]
        fn is_reachable_from_equals_to_is_trivially_true() {
            let adj = adj_from(2, &[], true);
            assert!(is_reachable(&adj, 1, 1));
        }

        #[test]
        fn dijkstra_unreachable_node_stays_none() {
            let adj = adj_from(3, &[(0, 1)], true);
            let dist = dijkstra(&adj, &HashMap::new(), 0);
            assert_eq!(dist, vec![Some(0.0), Some(1.0), None]);
        }

        #[test]
        fn dijkstra_path_none_when_unreachable() {
            let adj = adj_from(2, &[], true);
            assert!(dijkstra_path(&adj, &HashMap::new(), 0, 1).is_none());
        }

        #[test]
        fn dijkstra_and_dijkstra_path_out_of_range_from_returns_empty_or_none() {
            let adj = adj_from(2, &[(0, 1)], true);
            assert_eq!(dijkstra(&adj, &HashMap::new(), 9), vec![None, None]);
            assert!(dijkstra_path(&adj, &HashMap::new(), 9, 0).is_none());
        }

        #[test]
        fn minimum_spanning_tree_skips_out_of_range_edges() {
            let edges = vec![(0, 1, 1.0), (0, 5, 2.0)];
            let selected = minimum_spanning_tree(2, &edges);
            assert_eq!(selected, vec![0]);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Algorithms

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    type NU = Storage<Normal, Undirected>;
    type ND = Storage<Normal, Directed>;
    type PU = Storage<Ported, Undirected>;
    type PD = Storage<Ported, Directed>;

    // #subregion Storage
    #[test]
    fn add_node_allocates_monotone_ids() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(g.node_count(), 2);
    }

    #[test]
    fn add_node_with_id_upserts_attrs_and_bumps_allocator() {
        let mut g = NU::new();
        let mut attrs = PropertyBag::new();
        attrs.insert("color".into(), PropertyValue::String("red".into()));
        g.add_node_with_id(5, attrs);
        assert!(g.contains_node(5));
        let next = g.add_node();
        assert_eq!(next, 6, "auto id must skip past the caller-supplied id");

        let mut more = PropertyBag::new();
        more.insert("size".into(), PropertyValue::Number(3.0));
        g.add_node_with_id(5, more);
        let record = g.node_attrs(5).expect("node 5 exists");
        assert_eq!(record.get("color").and_then(PropertyValue::as_str), Some("red"));
        assert_eq!(record.get("size").and_then(PropertyValue::as_f64), Some(3.0));
    }

    #[test]
    fn remove_node_cascades_edges_and_handles() {
        let mut g = PU::new();
        let a = g.add_node();
        let b = g.add_node();
        let ha = g.add_handle(a).expect("ported storage grants handles");
        let hb = g.add_handle(b).expect("ported storage grants handles");
        let e = g.add_edge(ha, hb);
        assert!(g.remove_node(a));
        assert!(!g.contains_node(a));
        assert!(g.edge_endpoints(e).is_none(), "incident edge must be cascaded away");
        assert!(g.handle_owner(ha).is_none(), "handle on the removed node must be cascaded away");
        assert_eq!(g.handles(b), &[hb]);
    }

    #[test]
    fn normal_add_edge_upserts_instead_of_duplicating() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut first = PropertyBag::new();
        first.insert("weight".into(), PropertyValue::Number(1.0));
        let e1 = g.add_edge_with(a, b, first);
        let mut second = PropertyBag::new();
        second.insert("label".into(), PropertyValue::String("x".into()));
        let e2 = g.add_edge_with(a, b, second);
        assert_eq!(e1, e2, "Normal storages upsert an existing pair instead of creating a parallel edge");
        assert_eq!(g.edge_count(), 1);
        let attrs = g.edge_attrs(e1).expect("edge exists");
        assert_eq!(attrs.get("weight").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(attrs.get("label").and_then(PropertyValue::as_str), Some("x"));
    }

    #[test]
    fn ported_add_edge_always_creates_parallel_edges() {
        let mut g = PD::new();
        let a = g.add_node();
        let b = g.add_node();
        let ha = g.add_handle(a).expect("ported");
        let hb = g.add_handle(b).expect("ported");
        let e1 = g.add_edge(ha, hb);
        let e2 = g.add_edge(ha, hb);
        assert_ne!(e1, e2, "Ported storages always create a fresh parallel edge");
        assert_eq!(g.edge_count(), 2);
        assert_eq!(g.out_degree(a), 2);
    }

    #[test]
    fn normal_storage_denies_handles() {
        let mut g = NU::new();
        let a = g.add_node();
        assert!(g.add_handle(a).is_none());
        assert!(g.handles(a).is_empty());
    }

    #[test]
    fn remove_edge_unlinks_adjacency_both_ways_when_undirected() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        assert!(g.remove_edge(e));
        assert_eq!(g.out_degree(a), 0);
        assert_eq!(g.out_degree(b), 0);
        assert!(g.edges_between(a, b).next().is_none());
    }

    #[test]
    fn clear_edges_keeps_nodes_clear_removes_everything() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.clear_edges();
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 0);
        g.clear();
        assert_eq!(g.node_count(), 0);
    }

    #[test]
    fn remove_edge_and_remove_node_return_false_for_unknown_ids() {
        let mut g = NU::new();
        assert!(!g.remove_edge(999), "removing a never-created edge id must fail cleanly");
        assert!(!g.remove_node(999), "removing a never-created node id must fail cleanly");
    }

    #[test]
    fn node_attrs_mut_and_edge_attrs_mut_edit_in_place_and_are_none_for_unknown_ids() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        g.node_attrs_mut(a).expect("node exists").insert("k".into(), PropertyValue::Number(1.0));
        g.edge_attrs_mut(e).expect("edge exists").insert("w".into(), PropertyValue::Number(2.0));
        assert_eq!(g.node_attrs(a).unwrap().get("k").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(g.edge_attrs(e).unwrap().get("w").and_then(PropertyValue::as_f64), Some(2.0));
        assert!(g.node_attrs_mut(999).is_none());
        assert!(g.edge_attrs_mut(999).is_none());
    }

    #[test]
    fn add_handle_denies_missing_node_and_handle_owner_is_none_for_unknown_handle() {
        let mut g = PU::new();
        assert!(g.add_handle(999).is_none(), "cannot anchor a handle on a node that doesn't exist");
        assert!(g.handle_owner(999).is_none());
    }

    #[test]
    fn core_edge_normalize_undirected_orders_the_pair() {
        assert_eq!(CoreEdge::<u64>::normalize_undirected(5, 2), (2, 5));
        assert_eq!(CoreEdge::<u64>::normalize_undirected(2, 5), (2, 5));
    }
    // #endsubregion

    // #subregion GraphView
    #[test]
    fn undirected_self_loop_counts_twice_towards_degree() {
        let mut g = NU::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.degree(a), 2);
        assert_eq!(g.edge_count(), 1, "edges() still lists the self-loop once");
        assert_eq!(g.edges_between(a, a).count(), 2);
    }

    #[test]
    fn directed_degree_is_in_plus_out() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(c, a);
        assert_eq!(g.out_degree(a), 1);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 2);
        assert_eq!(GraphView::neighbors(&g, a).collect::<Vec<_>>(), vec![b], "neighbors == out_neighbors for directed storages");
    }

    #[test]
    fn undirected_in_neighbors_equals_out_neighbors() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let out: Vec<_> = g.out_neighbors(a).collect();
        let inn: Vec<_> = g.in_neighbors(a).collect();
        assert_eq!(out, inn);
    }

    #[test]
    fn is_directed_and_is_multigraph_reflect_type_axes() {
        assert!(!NU::new().is_directed());
        assert!(ND::new().is_directed());
        assert!(!NU::new().is_multigraph());
        assert!(PU::new().is_multigraph());
    }

    #[test]
    fn directed_self_loop_counts_once_each_towards_out_and_in_degree() {
        let mut g = ND::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.out_degree(a), 1);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 2);
    }
    // #endsubregion

    // #subregion EdgeWeights
    #[test]
    fn unit_weight_is_always_one() {
        let w = UnitWeight;
        assert_eq!(w.weight(EdgeRef { id: 0, u: 0, v: 1 }), 1.0);
    }

    #[test]
    fn storage_default_weight_reads_weight_attr_with_fallback() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs = PropertyBag::new();
        attrs.insert("weight".into(), PropertyValue::Number(4.5));
        let e = g.add_edge_with(a, b, attrs);
        let edge_ref = EdgeRef { id: e, u: a, v: b };
        assert_eq!(g.weight(edge_ref), 4.5);

        let e2 = g.add_edge(b, a);
        assert_eq!(e2, e, "Normal upsert must keep returning the same edge id");

        let mut g2 = NU::new();
        let x = g2.add_node();
        let y = g2.add_node();
        let unweighted_edge = g2.add_edge(x, y);
        assert_eq!(g2.weight(EdgeRef { id: unweighted_edge, u: x, v: y }), 1.0);
    }

    #[test]
    fn attr_weight_falls_back_to_default_when_missing_or_non_numeric() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs = PropertyBag::new();
        attrs.insert("cost".into(), PropertyValue::String("not-a-number".into()));
        let e = g.add_edge_with(a, b, attrs);
        let aw = AttrWeight { graph: &g, name: "cost", default: 2.0 };
        assert_eq!(aw.weight(EdgeRef { id: e, u: a, v: b }), 2.0);
    }

    #[test]
    fn closure_implements_edge_weights() {
        let double = |edge: EdgeRef| (edge.id as f64) * 2.0;
        assert_eq!(double.weight(EdgeRef { id: 3, u: 0, v: 1 }), 6.0);
    }
    // #endsubregion

    // #subregion Csr
    #[test]
    fn csr_from_view_preserves_directed_adjacency() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, c);
        let csr = Csr::from_view(&g);
        assert_eq!(csr.node_count(), 3);
        let ia = csr.index_of(a).expect("a indexed");
        let ib = csr.index_of(b).expect("b indexed");
        let ic = csr.index_of(c).expect("c indexed");
        let mut out: Vec<usize> = csr.out_neighbors(ia).to_vec();
        out.sort_unstable();
        let mut expected = vec![ib, ic];
        expected.sort_unstable();
        assert_eq!(out, expected);
        assert_eq!(csr.node_of(ia), Some(a));
        assert!(csr.in_neighbors(ib).contains(&ia));
    }

    #[test]
    fn csr_from_view_mirrors_undirected_edges_both_ways() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let csr = Csr::from_view(&g);
        let ia = csr.index_of(a).unwrap();
        let ib = csr.index_of(b).unwrap();
        assert!(csr.out_neighbors(ia).contains(&ib));
        assert!(csr.out_neighbors(ib).contains(&ia));
    }

    #[test]
    fn csr_out_edges_and_unknown_ids_return_none() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let csr = Csr::from_view(&g);
        let ia = csr.index_of(a).unwrap();
        assert_eq!(csr.out_edges(ia), &[e]);
        assert_eq!(csr.node_of(999), None);
        assert_eq!(csr.index_of(999), None);
    }
    // #endsubregion

    // #subregion Views
    #[test]
    fn subgraph_view_drops_edges_leaving_the_subset() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let sub = SubgraphView::new(&g, [a, b]);
        assert_eq!(sub.node_count(), 2);
        assert_eq!(sub.edge_count(), 1);
        assert!(!sub.contains_node(c));
    }

    #[test]
    fn edge_subgraph_view_nodes_are_exactly_edge_endpoints() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_node(); // isolated node d, never referenced by an edge
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);
        let view = EdgeSubgraphView::new(&g, [e_ab]);
        let mut nodes: Vec<_> = view.nodes().collect();
        nodes.sort_unstable();
        assert_eq!(nodes, vec![a, b]);
        assert_eq!(view.edge_count(), 1);
    }

    #[test]
    fn subgraph_view_degree_counts_only_edges_within_subset() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, c);
        let sub = SubgraphView::new(&g, [a, b]);
        assert_eq!(sub.out_degree(a), 1, "the edge to c falls outside the node subset");
        assert_eq!(sub.in_degree(b), 1);
        assert_eq!(sub.degree(a), sub.out_degree(a) + sub.in_degree(a), "directed subgraph degree is out+in");
        assert!(sub.is_directed());
        assert!(!sub.is_multigraph());
    }

    #[test]
    fn subgraph_view_attr_view_hides_attrs_outside_the_node_subset() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let sub = SubgraphView::new(&g, [a]);
        assert!(sub.node_attrs(a).is_some());
        assert!(sub.node_attrs(b).is_none(), "b is outside the node subset");
        assert!(sub.edge_attrs(e).is_some(), "edge attrs are not filtered by SubgraphView");
        assert!(std::ptr::eq(sub.graph_attrs(), g.graph_attrs()));
    }

    #[test]
    fn edge_subgraph_view_degree_and_directed_flag() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);
        let view = EdgeSubgraphView::new(&g, [e_ab]);
        assert!(view.is_directed());
        assert_eq!(view.out_degree(a), 1);
        assert_eq!(view.in_degree(b), 1);
        assert_eq!(view.degree(a), 1);
        assert!(view.edge_attrs(e_ab).is_some());
    }

    #[test]
    fn edge_subgraph_view_undirected_in_neighbors_matches_out_neighbors() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = EdgeSubgraphView::new(&g, [e]);
        assert!(!view.is_directed());
        assert_eq!(view.in_neighbors(a).collect::<Vec<_>>(), view.out_neighbors(a).collect::<Vec<_>>());
        assert_eq!(view.degree(a), view.out_degree(a));
    }

    #[test]
    fn reversed_view_swaps_direction_on_directed_graph() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.out_neighbors(b).collect::<Vec<_>>(), vec![a]);
        assert_eq!(rev.in_neighbors(a).collect::<Vec<_>>(), vec![b]);
    }

    #[test]
    fn reversed_view_is_a_no_op_on_undirected_graph() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.out_neighbors(a).collect::<Vec<_>>(), g.out_neighbors(a).collect::<Vec<_>>());
    }

    #[test]
    fn reversed_view_edges_and_edges_between_swap_endpoints() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.edges().collect::<Vec<_>>(), vec![EdgeRef { id: e, u: b, v: a }]);
        assert_eq!(rev.edges_between(b, a).next(), Some(EdgeRef { id: e, u: b, v: a }));
        assert_eq!(rev.degree(a), g.degree(a));
        assert_eq!(rev.is_multigraph(), g.is_multigraph());
    }

    #[test]
    fn filtered_view_keep_predicate_hides_by_inversion() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let hidden: BTreeSet<NodeId> = [b].into_iter().collect();
        let view = FilteredView::new(&g, |n| !hidden.contains(&n), |_e| true);
        assert!(view.contains_node(a));
        assert!(!view.contains_node(b));
        assert_eq!(view.edge_count(), 0, "both edges touch the hidden node b");
    }

    #[test]
    fn filtered_view_keep_edge_predicate_hides_specific_edges_without_hiding_nodes() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e_bad = g.add_edge(a, b);
        let view = FilteredView::new(&g, |_n| true, move |e| e.id != e_bad);
        assert!(view.contains_node(a));
        assert!(view.contains_node(b));
        assert_eq!(view.edge_count(), 0);
        assert_eq!(view.out_degree(a), 0);
        assert_eq!(view.degree(a), 0);
    }

    #[test]
    fn filtered_view_attr_view_delegates_edge_and_graph_attrs() {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = FilteredView::new(&g, |_n| true, |_e| true);
        assert!(view.edge_attrs(e).is_some());
        assert!(std::ptr::eq(view.graph_attrs(), g.graph_attrs()));
    }

    #[test]
    fn undirected_view_merges_successors_and_predecessors() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let view = UndirectedView::new(&g);
        assert!(!view.is_directed());
        assert_eq!(view.neighbors(a).collect::<Vec<_>>(), vec![b]);
        assert_eq!(view.neighbors(b).collect::<Vec<_>>(), vec![a]);
    }

    #[test]
    fn undirected_view_degree_and_edges_between_merge_both_directions() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, a);
        let view = UndirectedView::new(&g);
        assert_eq!(view.degree(a), 2, "both directed edges count towards undirected degree");
        assert_eq!(view.edges_between(a, b).count(), 2);
        assert_eq!(view.is_multigraph(), g.is_multigraph());
    }

    #[test]
    fn undirected_view_edges_normalizes_endpoint_order() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(b, a);
        let view = UndirectedView::new(&g);
        assert_eq!(view.edges().collect::<Vec<_>>(), vec![EdgeRef { id: e, u: a, v: b }], "edges() orders endpoints u <= v regardless of storage direction");
    }

    #[test]
    fn undirected_view_attr_view_delegates_to_parent() {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = UndirectedView::new(&g);
        assert!(view.node_attrs(a).is_some());
        assert!(view.edge_attrs(e).is_some());
        assert!(std::ptr::eq(view.graph_attrs(), g.graph_attrs()));
    }
    // #endsubregion

    // #subregion Interner
    #[test]
    fn interner_intern_is_idempotent() {
        let mut interner: Interner<String> = Interner::new();
        let a1 = interner.intern("alpha".to_string());
        let a2 = interner.intern("alpha".to_string());
        let b = interner.intern("beta".to_string());
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        assert_eq!(interner.label_of(a1), Some(&"alpha".to_string()));
        assert_eq!(interner.id_of(&"beta".to_string()), Some(b));
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn interner_from_labels_is_sorted_and_deduplicated() {
        let interner: Interner<String> = Interner::from_labels(["c".to_string(), "a".to_string(), "a".to_string(), "b".to_string()]);
        assert_eq!(interner.len(), 3);
        assert_eq!(interner.label_of(0), Some(&"a".to_string()));
        assert_eq!(interner.label_of(1), Some(&"b".to_string()));
        assert_eq!(interner.label_of(2), Some(&"c".to_string()));
    }

    #[test]
    fn interner_is_empty_and_unknown_lookups_return_none() {
        let mut interner: Interner<String> = Interner::new();
        assert!(interner.is_empty());
        assert_eq!(interner.label_of(0), None);
        assert_eq!(interner.id_of(&"ghost".to_string()), None);
        interner.intern("alpha".to_string());
        assert!(!interner.is_empty());
    }
    // #endsubregion

    // #subregion GraphError
    #[test]
    fn graph_error_display_reads_clearly() {
        assert_eq!(GraphError::NodeNotFound(7).to_string(), "node 7 not found");
        assert_eq!(GraphError::NoPath { source: 1, target: 2 }.to_string(), "no path from node 1 to node 2");
        assert_eq!(GraphError::NotImplementedForKind { algorithm: "planarity", kind: "multigraph" }.to_string(), "planarity is not implemented for multigraph");
    }

    #[test]
    fn graph_error_is_a_std_error() {
        let err: Box<dyn std::error::Error> = Box::new(GraphError::HasACycle);
        assert_eq!(err.to_string(), "graph has a cycle");
    }

    #[test]
    fn graph_error_display_covers_remaining_variants() {
        assert_eq!(GraphError::EdgeNotFound(3).to_string(), "edge 3 not found");
        assert_eq!(GraphError::NoCycle.to_string(), "graph has no cycle");
        assert_eq!(GraphError::Unfeasible("x".into()).to_string(), "unfeasible: x");
        assert_eq!(GraphError::Unbounded("y".into()).to_string(), "unbounded: y");
        assert_eq!(GraphError::NotATree.to_string(), "graph is not a tree");
        assert_eq!(GraphError::NotAForest.to_string(), "graph is not a forest");
        assert_eq!(GraphError::NotBipartite.to_string(), "graph is not bipartite");
        assert_eq!(GraphError::NotPlanar.to_string(), "graph is not planar");
        assert_eq!(GraphError::NotEulerian.to_string(), "graph is not eulerian");
        assert_eq!(GraphError::NotConnected.to_string(), "graph is not connected");
        assert_eq!(GraphError::NotStronglyConnected.to_string(), "graph is not strongly connected");
        assert_eq!(GraphError::AmbiguousSolution("z".into()).to_string(), "ambiguous solution: z");
        assert_eq!(GraphError::ExceededMaxIterations { iterations: 5 }.to_string(), "exceeded max iterations (5)");
        assert_eq!(GraphError::PowerIterationFailedConvergence { iterations: 8 }.to_string(), "power iteration failed to converge after 8 iterations");
        assert_eq!(GraphError::NegativeCycle.to_string(), "graph has a negative cycle");
        assert_eq!(GraphError::NotGraphical("odd sum".into()).to_string(), "not a graphical degree sequence: odd sum");
        assert_eq!(GraphError::Io("disk full".into()).to_string(), "io error: disk full");
        assert_eq!(GraphError::Parse { line: 4, message: "bad token".into() }.to_string(), "parse error at line 4: bad token");
    }
    // #endsubregion

    // #subregion Utils
    #[test]
    fn pairwise_yields_consecutive_pairs() {
        let items = [1, 2, 3, 4];
        assert_eq!(pairwise(&items).collect::<Vec<_>>(), vec![(1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn arbitrary_element_is_deterministic() {
        assert_eq!(arbitrary_element(&[9, 1, 2]), Some(9));
        assert_eq!(arbitrary_element::<i32>(&[]), None);
    }

    #[test]
    fn tolerance_constants_are_ordered() {
        assert!(TOL_STRICT < TOL_LOOSE);
    }

    #[test]
    fn mapped_heap_pops_in_ascending_priority_order() {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("c", 30);
        heap.push_or_decrease("a", 10);
        heap.push_or_decrease("b", 20);
        assert_eq!(heap.pop_min(), Some((10, "a")));
        assert_eq!(heap.pop_min(), Some((20, "b")));
        assert_eq!(heap.pop_min(), Some((30, "c")));
        assert_eq!(heap.pop_min(), None);
    }

    #[test]
    fn mapped_heap_decrease_key_reorders() {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("a", 10);
        heap.push_or_decrease("b", 20);
        assert!(heap.decrease_key(&"b", 5));
        assert!(!heap.decrease_key(&"b", 100), "raising priority via decrease_key is a no-operation");
        assert_eq!(heap.pop_min(), Some((5, "b")));
        assert!(heap.contains(&"a"));
        assert!(!heap.contains(&"b"));
    }

    #[test]
    fn mapped_heap_len_and_is_empty_track_size() {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        heap.push_or_decrease("a", 5);
        assert!(!heap.is_empty());
        assert_eq!(heap.len(), 1);
    }

    #[test]
    fn mapped_heap_push_or_decrease_ignores_higher_or_equal_priority() {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("a", 5);
        heap.push_or_decrease("a", 10);
        assert_eq!(heap.len(), 1, "a higher priority for an already-present item must be a no-operation");
        heap.push_or_decrease("a", 5);
        assert_eq!(heap.pop_min(), Some((5, "a")), "priority must stay at the lowest value ever pushed");
    }

    #[test]
    fn decrease_key_returns_false_for_absent_item() {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        assert!(!heap.decrease_key(&"missing", 1));
    }
    // #endsubregion

    // #subregion Randomized consistency (expensive-ish; kept here since it's the one genuinely property-style check in this file)
    mod quick {
        use super::*;

        /// 🎲️ Tiny deterministic xorshift so this crate doesn't need `mathematical_random` as a dependency just for one fuzz test.
        fn xorshift(state: &mut u64) -> u64 {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
            *state
        }

        #[test]
        fn csr_out_degree_matches_storage_out_degree_under_random_directed_graphs() {
            let mut seed = 0x5eed_u64;
            for _ in 0..20 {
                let mut g = ND::new();
                let n = 3 + (xorshift(&mut seed) % 8) as usize;
                let nodes: Vec<NodeId> = (0..n).map(|_| g.add_node()).collect();
                let edge_attempts = n * 2;
                for _ in 0..edge_attempts {
                    let u = nodes[(xorshift(&mut seed) as usize) % n];
                    let v = nodes[(xorshift(&mut seed) as usize) % n];
                    g.add_edge(u, v);
                }
                let csr = Csr::from_view(&g);
                for &node in &nodes {
                    let i = csr.index_of(node).expect("every storage node is indexed");
                    assert_eq!(csr.out_neighbors(i).len(), g.out_degree(node), "csr out-degree must match storage out-degree for node {node}");
                }
            }
        }
    }
    // #endsubregion

    // #subregion MaxFlow
    /// 🏗️ The classic CLRS Ford-Fulkerson network (Fig. 26.1): six nodes `s=0, v1=1, v2=2, v3=3, v4=4, t=5`, known max flow `23`.
    fn clrs_flow_network() -> FlowNetwork {
        let mut net = FlowNetwork::new(6);
        net.add_edge(0, 1, 16.0);
        net.add_edge(0, 2, 13.0);
        net.add_edge(1, 3, 12.0);
        net.add_edge(2, 1, 4.0);
        net.add_edge(3, 2, 9.0);
        net.add_edge(2, 4, 14.0);
        net.add_edge(4, 3, 7.0);
        net.add_edge(3, 5, 20.0);
        net.add_edge(4, 5, 4.0);
        net
    }

    #[test]
    fn max_flow_matches_clrs_textbook_network() {
        let mut net = clrs_flow_network();
        assert_eq!(net.max_flow(0, 5), 23.0);
    }

    #[test]
    fn min_cut_capacity_matches_max_flow_value_duality() {
        let mut net = clrs_flow_network();
        let flow = net.max_flow(0, 5);
        let reachable: BTreeSet<u32> = net.min_cut(0).into_iter().collect();
        assert!(!reachable.contains(&5), "sink must land on the far side of a valid cut");
        let clrs_edges = [(0u32, 1u32, 16.0), (0, 2, 13.0), (1, 3, 12.0), (2, 1, 4.0), (3, 2, 9.0), (2, 4, 14.0), (4, 3, 7.0), (3, 5, 20.0), (4, 5, 4.0)];
        let crossing: f64 = clrs_edges.iter().filter(|&&(u, v, _)| reachable.contains(&u) && !reachable.contains(&v)).map(|&(_, _, cap)| cap).sum();
        assert_eq!(crossing, flow, "total capacity crossing the min cut must equal the max flow value");
    }

    #[test]
    fn max_flow_saturates_branching_level_graph() {
        let mut net = FlowNetwork::new(5);
        net.add_edge(0, 1, 10.0);
        net.add_edge(0, 2, 10.0);
        net.add_edge(0, 3, 10.0);
        net.add_edge(1, 2, 2.0);
        net.add_edge(2, 3, 2.0);
        net.add_edge(1, 4, 4.0);
        net.add_edge(2, 4, 4.0);
        net.add_edge(3, 4, 4.0);
        assert_eq!(net.max_flow(0, 4), 12.0, "sink in-degree 3 at capacity 4 each caps the flow at 12 regardless of source out-degree 3");
    }

    #[test]
    fn max_flow_is_zero_when_source_and_sink_are_disconnected() {
        let mut net = FlowNetwork::new(2);
        assert_eq!(net.max_flow(0, 1), 0.0);
        assert_eq!(net.min_cut(0), vec![0], "with no path at all, only the source itself is reachable");
    }

    #[test]
    fn max_flow_and_min_cut_are_deterministic_across_fresh_instances() {
        let mut first = clrs_flow_network();
        let mut second = clrs_flow_network();
        let flow_a = first.max_flow(0, 5);
        let flow_b = second.max_flow(0, 5);
        assert_eq!(flow_a, flow_b, "identically constructed networks must yield byte-identical flow values");
        assert_eq!(first.min_cut(0), second.min_cut(0), "identically constructed networks must yield byte-identical min-cut node sets");
    }
    // #endsubregion

    // #subregion PropertyJson
    #[test]
    fn property_bag_json_round_trips_and_empty_bag_serializes_to_none() {
        let mut bag = PropertyBag::new();
        bag.insert("label".into(), PropertyValue::String("hi".into()));
        bag.insert("count".into(), PropertyValue::Number(3.0));
        let json = property_bag_to_json(&bag).expect("non-empty bag serializes to Some");
        let round_tripped = property_bag_from_json(&json);
        assert_eq!(round_tripped.get("label").and_then(PropertyValue::as_str), Some("hi"));
        assert_eq!(round_tripped.get("count").and_then(PropertyValue::as_f64), Some(3.0));
        assert!(property_bag_to_json(&PropertyBag::new()).is_none(), "an empty bag serializes to None");
    }

    #[test]
    fn property_bag_from_json_falls_back_to_default_on_unparsable_shape() {
        let value = serde_json::json!("not-an-object-map");
        let bag = property_bag_from_json(&value);
        assert!(bag.is_empty(), "a JSON value that can't deserialize into a PropertyBag falls back to empty");
    }
    // #endsubregion
}
// #endregion 🔖️Tests

// #region 🔖️PropertyJson
/// 🧾️ Converts JSON fixture `userData` into a typed property bag.
pub fn property_bag_from_json(value: &serde_json::Value) -> PropertyBag {
    serde_json::from_value(value.clone()).unwrap_or_default()
}

/// 🧾️ Serializes a property bag back to JSON for fixture export.
pub fn property_bag_to_json(bag: &PropertyBag) -> Option<serde_json::Value> {
    if bag.is_empty() {
        None
    } else {
        serde_json::to_value(bag).ok()
    }
}
// #endregion 🔖️PropertyJson

// #region 🔖️Kinds
use mathematical_geometry::Point;

/// 🔵️ Circle or axis-aligned rectangle node body.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NodeShape {
    #[default]
    Circle,
    Rectangle,
}

/// 🪝️ Port direction for directed edge wiring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HandleRole {
    Source,
    Target,
    #[default]
    Any,
}

/// 🏷️ Semantic kind and property payload shared by graph elements.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElementSemantics {
    pub kind: Option<String>,
    pub properties: PropertyBag,
}

/// 🟠️ Retained node state with world-space center and shape extents.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub center: Point,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub shape: NodeShape,
    pub draggable: bool,
    pub kind: Option<String>,
    pub label: Option<String>,
    pub properties: PropertyBag,
}

/// 🟣️ Tangent handle anchored to a node at a polar angle.
#[derive(Clone, Debug, PartialEq)]
pub struct Handle {
    pub angle: f64,
    pub id: HandleId,
    pub node_id: NodeId,
    pub radius: f64,
    pub role: HandleRole,
    pub kind: Option<String>,
    pub properties: PropertyBag,
}

/// 🪢️ Retained edge with typed endpoints.
pub type GraphEdge<E> = CoreEdge<E>;
// #endregion 🔖️Kinds

// #region 🔖️MaxFlow
/// 🎚️ Residual-capacity noise guard: capacities at or below this are treated as exhausted (fractional alpha-expansion graph-cut costs are not exact).
const FLOW_EPS: f64 = 1e-9;

/// 🌊️ Directed residual-graph edge; its paired reverse edge always lives at the adjacent arena slot (`id ^ 1`).
#[derive(Clone, Copy, Debug)]
struct FlowEdge {
    to: u32,
    capacity: f64,
}

/// 🌊️ Capacitated directed flow network on a `u32`-indexed arena, for [Dinic's algorithm](https://doi.org/10.1016/0898-1221(74)90074-0) (CLRS ch. 26). Edges are stored as forward/reverse residual pairs at adjacent slots so augmenting a path only ever touches two `Vec` entries; adjacency is `Vec<Vec<u32>>`, never a hash map, so traversal order — and therefore `min_cut`'s result — is fixed by construction order alone.
#[derive(Clone, Debug)]
pub struct FlowNetwork {
    node_count: u32,
    edges: Vec<FlowEdge>,
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
