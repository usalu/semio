//! 🕸️ Pure graph foundation: topology markers, node/handle/edge kinds, and index-based algorithms; the interactive board engine lives in `infinite_board`.

use std::collections::{BTreeMap, BTreeSet};

pub use crate::manifest::{PropertyBag, PropertyValue};

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
    pub async fn normalize_undirected(source: E, target: E) -> (E, E) {
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
pub async fn orient_endpoints<E: Copy + Ord, D: Directedness>(source: E, target: E) -> (E, E) {
    if D::DIRECTED {
        (source, target)
    } else {
        CoreEdge::<E>::normalize_undirected(source, target).await
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
    async fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64;
    async fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint>;
    async fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId>;
}

/// 🟠️ Node-to-node edges without handles.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl PortModel for Normal {
    type Endpoint = NodeId;
    const HAS_PORTS: bool = false;
    const MULTI_EDGES: bool = false;
    async fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    async fn try_handle_endpoint(_: HandleId) -> Option<Self::Endpoint> {
        None
    }
    async fn endpoint_as_handle(_: Self::Endpoint) -> Option<HandleId> {
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
    async fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    async fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint> {
        Some(handle_id)
    }
    async fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId> {
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
async fn unlink_one(map: &mut BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>, u: NodeId, v: NodeId, edge_id: EdgeId) {
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
    // 🚫️async: E1 impl of external trait `std::default::Default` — must stay sync. Mirrors `new()`'s
    // literal (I/O-free) body directly rather than calling it, since `new()` stays `async` for
    // call-site uniformity with the rest of this crate. See R9.
    fn default() -> Self {
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
}

impl<P: PortModel, D: Directedness> Storage<P, D> {
    /// 🆕️ Empty storage; every id allocator starts at `0` and is monotone — an id is never reused, even after removal.
    pub async fn new() -> Self {
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
    async fn endpoint_node(&self, endpoint: P::Endpoint) -> NodeId {
        match P::endpoint_as_handle(endpoint).await {
            Some(handle_id) => *self.handle_owner.get(&handle_id).expect("every live handle endpoint has a recorded owner node"),
            None => P::endpoint_as_u64(endpoint).await,
        }
    }

    async fn link_adjacency(&mut self, u: NodeId, v: NodeId, edge_id: EdgeId) {
        self.successors.entry(u).or_default().entry(v).or_default().push(edge_id);
        if D::DIRECTED {
            self.predecessors.entry(v).or_default().entry(u).or_default().push(edge_id);
        } else if u == v {
            self.successors.entry(u).or_default().entry(v).or_default().push(edge_id);
        } else {
            self.successors.entry(v).or_default().entry(u).or_default().push(edge_id);
        }
    }

    async fn unlink_adjacency(&mut self, u: NodeId, v: NodeId, edge_id: EdgeId) {
        // 🚨️ all four branches were dropped-future no-ops: none of these `unlink_one` calls were
        // ever awaited, so `unlink_adjacency` silently did nothing — the outer `.await` fixed at
        // its own call sites (`remove_edge`) was necessary but not sufficient.
        unlink_one(&mut self.successors, u, v, edge_id).await;
        if D::DIRECTED {
            unlink_one(&mut self.predecessors, v, u, edge_id).await;
        } else if u == v {
            unlink_one(&mut self.successors, u, v, edge_id).await;
        } else {
            unlink_one(&mut self.successors, v, u, edge_id).await;
        }
    }

    // #subregion Nodes
    pub async fn add_node(&mut self) -> NodeId {
        self.add_node_with(PropertyBag::new()).await
    }

    pub async fn add_node_with(&mut self, attrs: PropertyBag) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        self.nodes.insert(id, NodeRecord { attrs, handles: Vec::new() });
        id
    }

    /// 🆔️ Inserts a node at a caller-supplied id, or merges `attrs` into it if already present (NetworkX `add_node(id, **attrs)` semantics); bumps the allocator past `id` so future auto-ids never collide with it.
    pub async fn add_node_with_id(&mut self, id: NodeId, attrs: PropertyBag) -> NodeId {
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

    pub async fn contains_node(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    /// 🗑️ Removes a node, cascading: every incident edge is removed first, then (for ported storages) every handle anchored on it.
    pub async fn remove_node(&mut self, id: NodeId) -> bool {
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
            // 🚨️ was a dropped-future no-op: incident edges were never actually removed when
            // their node was — `remove_edge`'s side effects on `self.edges`/adjacency never ran.
            self.remove_edge(edge_id).await;
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

    pub async fn node_attrs_mut(&mut self, id: NodeId) -> Option<&mut PropertyBag> {
        self.nodes.get_mut(&id).map(|r| &mut r.attrs)
    }
    // #endsubregion

    // #subregion Edges
    pub async fn add_edge(&mut self, source: P::Endpoint, target: P::Endpoint) -> EdgeId {
        self.add_edge_with(source, target, PropertyBag::new()).await
    }

    /// 🔀️ `Normal` storages upsert: an edge already connecting this pair gets `attrs` merged into it and its existing id returned (NetworkX `Graph`/`DiGraph`). `Ported` storages always create a fresh parallel edge with a new `EdgeId` (NetworkX `MultiGraph`/`MultiDiGraph`).
    pub async fn add_edge_with(&mut self, source: P::Endpoint, target: P::Endpoint, attrs: PropertyBag) -> EdgeId {
        let (un, vn) = (self.endpoint_node(source).await, self.endpoint_node(target).await);
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
        // 🚨️ was a dropped-future no-op: `link_adjacency` (mutates `successors`/`predecessors`)
        // was never awaited, so every edge added through this path was silently absent from
        // adjacency — traversal/neighbor/degree queries would all have missed it. See R10 header.
        self.link_adjacency(un, vn, id).await;
        id
    }

    pub async fn remove_edge(&mut self, id: EdgeId) -> bool {
        let Some(record) = self.edges.remove(&id) else { return false };
        let (u, v) = (self.endpoint_node(record.source).await, self.endpoint_node(record.target).await);
        // 🚨️ was a dropped-future no-op: `unlink_adjacency` was never awaited, so a removed edge's
        // adjacency entries were silently left in place. Same class as `add_edge_with` above.
        self.unlink_adjacency(u, v, id).await;
        true
    }

    pub async fn edge_attrs_mut(&mut self, id: EdgeId) -> Option<&mut PropertyBag> {
        self.edges.get_mut(&id).map(|r| &mut r.attrs)
    }

    pub async fn edge_endpoints(&self, id: EdgeId) -> Option<(P::Endpoint, P::Endpoint)> {
        self.edges.get(&id).map(|r| (r.source, r.target))
    }
    // #endsubregion

    // #subregion Handles
    /// 🪝️ Allocates a new handle anchored on `node`; only meaningful when `P::HAS_PORTS` — returns `None` otherwise (or if `node` doesn't exist), never panics.
    pub async fn add_handle(&mut self, node: NodeId) -> Option<HandleId> {
        if !P::HAS_PORTS || !self.nodes.contains_key(&node) {
            return None;
        }
        let id = self.next_handle_id;
        self.next_handle_id += 1;
        self.handle_owner.insert(id, node);
        self.nodes.get_mut(&node).expect("presence checked above").handles.push(id);
        Some(id)
    }

    pub async fn handles(&self, node: NodeId) -> &[HandleId] {
        self.nodes.get(&node).map_or(&[], |r| r.handles.as_slice())
    }

    pub async fn handle_owner(&self, handle: HandleId) -> Option<NodeId> {
        self.handle_owner.get(&handle).copied()
    }
    // #endsubregion

    // #subregion Whole graph
    pub async fn graph_attrs_mut(&mut self) -> &mut PropertyBag {
        &mut self.graph_attrs
    }

    /// 🧹️ Removes every node, edge, and handle; graph-level attrs are cleared too. Id allocators are NOT reset — ids are never reused, even across a clear.
    pub async fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.successors.clear();
        self.predecessors.clear();
        self.handle_owner.clear();
        self.graph_attrs.clear();
    }

    /// 🧹️ Removes every edge but keeps nodes (and their handles) and graph-level attrs.
    pub async fn clear_edges(&mut self) {
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
    async fn node_count(&self) -> usize;
    async fn nodes(&self) -> impl Iterator<Item = NodeId>;
    async fn contains_node(&self, node: NodeId) -> bool;
    async fn edge_count(&self) -> usize;
    async fn edges(&self) -> impl Iterator<Item = EdgeRef>;
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    /// ⬅️ Equals `out_neighbors` on an undirected view — there is only one adjacency direction, so predecessors and successors coincide.
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId>;
    async fn degree(&self, node: NodeId) -> usize;
    async fn out_degree(&self, node: NodeId) -> usize;
    /// ⬅️ Equals `out_degree` on an undirected view, for the same reason as `in_neighbors`.
    async fn in_degree(&self, node: NodeId) -> usize;
    async fn is_directed(&self) -> bool;
    async fn is_multigraph(&self) -> bool;
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef>;
}

/// 🏷️ Attribute lookup companion to `GraphView`.
pub trait AttrView {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag>;
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag>;
    async fn graph_attrs(&self) -> &PropertyBag;
}

/// ⚖️ Edge weight lookup, decoupled from attribute storage so algorithms take `impl EdgeWeights` instead of hardcoding a `"weight"` key.
pub trait EdgeWeights {
    async fn weight(&self, edge: EdgeRef) -> f64;
}

/// 1⃣ Unweighted default: every edge costs `1.0` (NetworkX's unweighted-graph convention).
#[derive(Clone, Copy, Debug, Default)]
pub struct UnitWeight;

impl EdgeWeights for UnitWeight {
    async fn weight(&self, _edge: EdgeRef) -> f64 {
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
    async fn weight(&self, edge: EdgeRef) -> f64 {
        self.graph.edge_attrs(edge.id).await.and_then(|attrs| attrs.get(self.name)).and_then(PropertyValue::as_f64).unwrap_or(self.default)
    }
}

impl<F: Fn(EdgeRef) -> f64> EdgeWeights for F {
    async fn weight(&self, edge: EdgeRef) -> f64 {
        self(edge)
    }
}

impl<P: PortModel, D: Directedness> GraphView for Storage<P, D> {
    async fn node_count(&self) -> usize {
        self.nodes.len()
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.keys().copied()
    }
    async fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains_key(&node)
    }
    async fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// 📇️ One `EdgeRef` per stored edge, in `EdgeId` order — a self-loop appears once here even though it counts twice towards `degree`.
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        // 🔀️ Rewritten from `.map(..)` — `endpoint_node` is async and cannot be called inside the
        // sync closure that used to build each `EdgeRef` (R10 residue shape #1).
        let mut out = Vec::with_capacity(self.edges.len());
        for (&id, record) in &self.edges {
            let u = self.endpoint_node(record.source).await;
            let v = self.endpoint_node(record.target).await;
            out.push(EdgeRef { id, u, v });
        }
        out.into_iter()
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node).await
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.successors.get(&node).into_iter().flat_map(|m| m.keys().copied())
    }
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let map = if D::DIRECTED { &self.predecessors } else { &self.successors };
        map.get(&node).into_iter().flat_map(|m| m.keys().copied())
    }
    async fn degree(&self, node: NodeId) -> usize {
        if D::DIRECTED {
            self.out_degree(node).await + self.in_degree(node).await
        } else {
            self.out_degree(node).await
        }
    }
    async fn out_degree(&self, node: NodeId) -> usize {
        self.successors.get(&node).map_or(0, |m| m.values().map(Vec::len).sum())
    }
    async fn in_degree(&self, node: NodeId) -> usize {
        if D::DIRECTED {
            self.predecessors.get(&node).map_or(0, |m| m.values().map(Vec::len).sum())
        } else {
            self.out_degree(node).await
        }
    }
    async fn is_directed(&self) -> bool {
        D::DIRECTED
    }
    async fn is_multigraph(&self) -> bool {
        P::MULTI_EDGES
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.successors.get(&u).and_then(|m| m.get(&v)).into_iter().flatten().copied().map(move |id| EdgeRef { id, u, v })
    }
}

impl<P: PortModel, D: Directedness> AttrView for Storage<P, D> {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.nodes.get(&node).map(|r| &r.attrs)
    }
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.edges.get(&edge).map(|r| &r.attrs)
    }
    async fn graph_attrs(&self) -> &PropertyBag {
        &self.graph_attrs
    }
}

/// ⚖️ Reads the graph's own `PropertyBag["weight"]` on each edge, defaulting to `1.0` — the common case; use `AttrWeight`/`UnitWeight`/a closure for anything else.
impl<P: PortModel, D: Directedness> EdgeWeights for Storage<P, D> {
    async fn weight(&self, edge: EdgeRef) -> f64 {
        self.edge_attrs(edge.id).await.and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64).unwrap_or(1.0)
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
    pub async fn from_view(view: &impl GraphView) -> Self {
        let mut node_ids: Vec<NodeId> = view.nodes().await.collect();
        node_ids.sort_unstable();
        let node_index: BTreeMap<NodeId, usize> = node_ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();
        let n = node_ids.len();
        let directed = view.is_directed().await;

        let mut out_buckets: Vec<Vec<(usize, EdgeId)>> = vec![Vec::new(); n];
        let mut in_buckets: Vec<Vec<usize>> = vec![Vec::new(); n];
        for edge in view.edges().await {
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

    pub async fn node_count(&self) -> usize {
        self.node_ids.len()
    }

    pub async fn out_neighbors(&self, i: usize) -> &[usize] {
        &self.out_targets[self.out_starts[i]..self.out_starts[i + 1]]
    }

    pub async fn in_neighbors(&self, i: usize) -> &[usize] {
        &self.in_targets[self.in_starts[i]..self.in_starts[i + 1]]
    }

    pub async fn out_edges(&self, i: usize) -> &[EdgeId] {
        &self.out_edge_ids[self.out_starts[i]..self.out_starts[i + 1]]
    }

    pub async fn node_of(&self, i: usize) -> Option<NodeId> {
        self.node_ids.get(i).copied()
    }

    pub async fn index_of(&self, id: NodeId) -> Option<usize> {
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
    pub async fn new(graph: &'g G, nodes: impl IntoIterator<Item = NodeId>) -> Self {
        let mut kept = BTreeSet::new();
        for n in nodes {
            if graph.contains_node(n).await {
                kept.insert(n);
            }
        }
        Self { graph, nodes: kept }
    }
}

impl<'g, G: GraphView> GraphView for SubgraphView<'g, G> {
    async fn node_count(&self) -> usize {
        self.nodes.len()
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.iter().copied()
    }
    async fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }
    async fn edge_count(&self) -> usize {
        self.edges().await.count()
    }
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().await.filter(|e| self.nodes.contains(&e.u) && self.nodes.contains(&e.v))
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.neighbors(node).await.filter(|n| self.nodes.contains(n))
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node).await.filter(|n| self.nodes.contains(n))
    }
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.in_neighbors(node).await.filter(|n| self.nodes.contains(n))
    }
    async fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed().await {
            self.out_degree(node).await + self.in_degree(node).await
        } else {
            self.out_degree(node).await
        }
    }
    async fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node).await {
            total += self.edges_between(node, nb).await.count();
        }
        total
    }
    async fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed().await {
            let mut total = 0usize;
            for nb in self.in_neighbors(node).await {
                total += self.edges_between(nb, node).await.count();
            }
            total
        } else {
            self.out_degree(node).await
        }
    }
    async fn is_directed(&self) -> bool {
        self.graph.is_directed().await
    }
    async fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph().await
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        let keep = self.nodes.contains(&u) && self.nodes.contains(&v);
        self.graph.edges_between(u, v).await.filter(move |_| keep)
    }
}

impl<'g, G: GraphView + AttrView> AttrView for SubgraphView<'g, G> {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if self.nodes.contains(&node) {
            self.graph.node_attrs(node).await
        } else {
            None
        }
    }
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge).await
    }
    async fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs().await
    }
}

/// 🔎️ Restricts a graph to an edge subset; nodes are exactly the endpoints of the included edges.
pub struct EdgeSubgraphView<'g, G: GraphView> {
    graph: &'g G,
    edges: BTreeSet<EdgeId>,
    nodes: BTreeSet<NodeId>,
}

impl<'g, G: GraphView> EdgeSubgraphView<'g, G> {
    pub async fn new(graph: &'g G, edges: impl IntoIterator<Item = EdgeId>) -> Self {
        let edge_set: BTreeSet<EdgeId> = edges.into_iter().collect();
        let mut nodes = BTreeSet::new();
        for e in graph.edges().await {
            if edge_set.contains(&e.id) {
                nodes.insert(e.u);
                nodes.insert(e.v);
            }
        }
        Self { graph, edges: edge_set, nodes }
    }
}

impl<'g, G: GraphView> GraphView for EdgeSubgraphView<'g, G> {
    async fn node_count(&self) -> usize {
        self.nodes.len()
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.nodes.iter().copied()
    }
    async fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }
    async fn edge_count(&self) -> usize {
        self.edges.len()
    }
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().await.filter(|e| self.edges.contains(&e.id))
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node).await
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let directed = self.graph.is_directed().await;
        self.edges()
            .await
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
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        if self.graph.is_directed().await {
            self.edges().await.filter_map(move |e| if e.v == node { Some(e.u) } else { None }).collect::<BTreeSet<_>>().into_iter()
        } else {
            self.out_neighbors(node).await.collect::<BTreeSet<_>>().into_iter()
        }
    }
    async fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed().await {
            self.out_degree(node).await + self.in_degree(node).await
        } else {
            self.out_degree(node).await
        }
    }
    async fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node).await {
            total += self.edges_between(node, nb).await.count();
        }
        total
    }
    async fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed().await {
            let mut total = 0usize;
            for nb in self.in_neighbors(node).await {
                total += self.edges_between(nb, node).await.count();
            }
            total
        } else {
            self.out_degree(node).await
        }
    }
    async fn is_directed(&self) -> bool {
        self.graph.is_directed().await
    }
    async fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph().await
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(u, v).await.filter(|e| self.edges.contains(&e.id))
    }
}

impl<'g, G: GraphView + AttrView> AttrView for EdgeSubgraphView<'g, G> {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if self.nodes.contains(&node) {
            self.graph.node_attrs(node).await
        } else {
            None
        }
    }
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        if self.edges.contains(&edge) {
            self.graph.edge_attrs(edge).await
        } else {
            None
        }
    }
    async fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs().await
    }
}

/// ↩️ Swaps successors and predecessors; only meaningful when the wrapped view is directed — on an undirected view this is a documented no-operation (not a panic), since successors already equal predecessors there.
pub struct ReversedView<'g, G: GraphView> {
    graph: &'g G,
}

impl<'g, G: GraphView> ReversedView<'g, G> {
    pub async fn new(graph: &'g G) -> Self {
        Self { graph }
    }
}

impl<'g, G: GraphView> GraphView for ReversedView<'g, G> {
    async fn node_count(&self) -> usize {
        self.graph.node_count().await
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes().await
    }
    async fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node).await
    }
    async fn edge_count(&self) -> usize {
        self.graph.edge_count().await
    }
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().await.map(|e| EdgeRef { id: e.id, u: e.v, v: e.u })
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node).await
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.in_neighbors(node).await
    }
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node).await
    }
    async fn degree(&self, node: NodeId) -> usize {
        self.graph.degree(node).await
    }
    async fn out_degree(&self, node: NodeId) -> usize {
        self.graph.in_degree(node).await
    }
    async fn in_degree(&self, node: NodeId) -> usize {
        self.graph.out_degree(node).await
    }
    async fn is_directed(&self) -> bool {
        self.graph.is_directed().await
    }
    async fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph().await
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(v, u).await.map(|e| EdgeRef { id: e.id, u: e.v, v: e.u })
    }
}

impl<'g, G: GraphView + AttrView> AttrView for ReversedView<'g, G> {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.graph.node_attrs(node).await
    }
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge).await
    }
    async fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs().await
    }
}

/// 🎛️ NetworkX `restricted_view`/`hide_nodes`/`hide_edges` equivalent: predicates return `true` to KEEP an element, so a "hide" caller just inverts its predicate.
pub struct FilteredView<'g, G: GraphView, FN, FE> {
    graph: &'g G,
    keep_node: FN,
    keep_edge: FE,
}

impl<'g, G: GraphView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> FilteredView<'g, G, FN, FE> {
    pub async fn new(graph: &'g G, keep_node: FN, keep_edge: FE) -> Self {
        Self { graph, keep_node, keep_edge }
    }

    async fn keep(&self, edge: EdgeRef) -> bool {
        (self.keep_node)(edge.u) && (self.keep_node)(edge.v) && (self.keep_edge)(edge)
    }
}

impl<'g, G: GraphView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> GraphView for FilteredView<'g, G, FN, FE> {
    async fn node_count(&self) -> usize {
        self.nodes().await.count()
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes().await.filter(|&n| (self.keep_node)(n))
    }
    async fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node).await && (self.keep_node)(node)
    }
    async fn edge_count(&self) -> usize {
        self.edges().await.count()
    }
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        // 🔀️ Rewritten from `.filter(move |&e| self.keep(e))` — `keep` is async and cannot be
        // called inside a sync closure (R10 residue shape #1).
        let mut out = Vec::new();
        for e in self.graph.edges().await {
            if self.keep(e).await {
                out.push(e);
            }
        }
        out.into_iter()
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.out_neighbors(node).await
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        // 🔀️ Rewritten — `edges_between` is async and cannot be called inside the sync `.filter`
        // predicate that used to guard this (R10 residue shape #1).
        let node_ok = (self.keep_node)(node);
        let mut out = Vec::new();
        if node_ok {
            for nb in self.graph.out_neighbors(node).await {
                if self.edges_between(node, nb).await.next().is_some() {
                    out.push(nb);
                }
            }
        }
        out.into_iter()
    }
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        let node_ok = (self.keep_node)(node);
        let mut out = Vec::new();
        if node_ok {
            for nb in self.graph.in_neighbors(node).await {
                if self.edges_between(nb, node).await.next().is_some() {
                    out.push(nb);
                }
            }
        }
        out.into_iter()
    }
    async fn degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed().await {
            self.out_degree(node).await + self.in_degree(node).await
        } else {
            self.out_degree(node).await
        }
    }
    async fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.out_neighbors(node).await {
            total += self.edges_between(node, nb).await.count();
        }
        total
    }
    async fn in_degree(&self, node: NodeId) -> usize {
        if self.graph.is_directed().await {
            let mut total = 0usize;
            for nb in self.in_neighbors(node).await {
                total += self.edges_between(nb, node).await.count();
            }
            total
        } else {
            self.out_degree(node).await
        }
    }
    async fn is_directed(&self) -> bool {
        self.graph.is_directed().await
    }
    async fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph().await
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        let keep_u = (self.keep_node)(u);
        let keep_v = (self.keep_node)(v);
        self.graph.edges_between(u, v).await.filter(move |&e| keep_u && keep_v && (self.keep_edge)(e))
    }
}

impl<'g, G: GraphView + AttrView, FN: Fn(NodeId) -> bool, FE: Fn(EdgeRef) -> bool> AttrView for FilteredView<'g, G, FN, FE> {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        if (self.keep_node)(node) {
            self.graph.node_attrs(node).await
        } else {
            None
        }
    }
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge).await
    }
    async fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs().await
    }
}

/// 🔀️ Presents a directed graph's edges as undirected — merges successor and predecessor sets into one neighbor view without materializing storage. Querying `edges_between(u, u)` on a directed self-loop yields it twice, mirroring the same "self-loop counts twice" convention `Storage` applies natively to undirected adjacency.
pub struct UndirectedView<'g, G: GraphView> {
    graph: &'g G,
}

impl<'g, G: GraphView> UndirectedView<'g, G> {
    pub async fn new(graph: &'g G) -> Self {
        Self { graph }
    }
}

impl<'g, G: GraphView> GraphView for UndirectedView<'g, G> {
    async fn node_count(&self) -> usize {
        self.graph.node_count().await
    }
    async fn nodes(&self) -> impl Iterator<Item = NodeId> {
        self.graph.nodes().await
    }
    async fn contains_node(&self, node: NodeId) -> bool {
        self.graph.contains_node(node).await
    }
    async fn edge_count(&self) -> usize {
        self.graph.edge_count().await
    }
    async fn edges(&self) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges().await.map(|e| if e.u <= e.v { e } else { EdgeRef { id: e.id, u: e.v, v: e.u } })
    }
    async fn neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.graph.out_neighbors(node).await.chain(self.graph.in_neighbors(node).await).collect::<BTreeSet<_>>().into_iter()
    }
    async fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.neighbors(node).await
    }
    async fn in_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> {
        self.neighbors(node).await
    }
    async fn degree(&self, node: NodeId) -> usize {
        self.out_degree(node).await
    }
    async fn out_degree(&self, node: NodeId) -> usize {
        let mut total = 0usize;
        for nb in self.neighbors(node).await {
            total += self.edges_between(node, nb).await.count();
        }
        total
    }
    async fn in_degree(&self, node: NodeId) -> usize {
        self.out_degree(node).await
    }
    async fn is_directed(&self) -> bool {
        false
    }
    async fn is_multigraph(&self) -> bool {
        self.graph.is_multigraph().await
    }
    async fn edges_between(&self, u: NodeId, v: NodeId) -> impl Iterator<Item = EdgeRef> {
        self.graph.edges_between(u, v).await.chain(self.graph.edges_between(v, u).await)
    }
}

impl<'g, G: GraphView + AttrView> AttrView for UndirectedView<'g, G> {
    async fn node_attrs(&self, node: NodeId) -> Option<&PropertyBag> {
        self.graph.node_attrs(node).await
    }
    async fn edge_attrs(&self, edge: EdgeId) -> Option<&PropertyBag> {
        self.graph.edge_attrs(edge).await
    }
    async fn graph_attrs(&self) -> &PropertyBag {
        self.graph.graph_attrs().await
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
    pub async fn new() -> Self {
        Self { labels: Vec::new(), by_label: std::collections::HashMap::new() }
    }

    /// 🏗️ Builds an interner from labels sorted for deterministic id assignment; duplicate labels collapse to one id.
    pub async fn from_labels(labels: impl IntoIterator<Item = L>) -> Self {
        let mut sorted: Vec<L> = labels.into_iter().collect();
        sorted.sort();
        sorted.dedup();
        let mut interner = Self::new().await;
        for label in sorted {
            interner.intern(label).await;
        }
        interner
    }

    /// ➕️ Returns the existing id for `label` if already interned, otherwise allocates the next sequential id.
    pub async fn intern(&mut self, label: L) -> NodeId {
        if let Some(&id) = self.by_label.get(&label) {
            return id;
        }
        let id = self.labels.len() as NodeId;
        self.labels.push(label.clone());
        self.by_label.insert(label, id);
        id
    }

    pub async fn label_of(&self, id: NodeId) -> Option<&L> {
        self.labels.get(id as usize)
    }

    pub async fn id_of(&self, label: &L) -> Option<NodeId> {
        self.by_label.get(label).copied()
    }

    pub async fn len(&self) -> usize {
        self.labels.len()
    }

    pub async fn is_empty(&self) -> bool {
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
pub async fn arbitrary_element<T: Copy>(items: &[T]) -> Option<T> {
    items.first().copied()
}

/// 🗳️ Binary-heap priority queue with `decrease_key`, ordered by `K` and keyed by `V` identity; a position index makes membership/decrease `O(log n)` instead of the `O(n)` a plain `BinaryHeap` needs for those operations.
#[derive(Clone, Debug)]
pub struct MappedHeap<K: Ord, V: Eq + std::hash::Hash + Clone> {
    heap: Vec<(K, V)>,
    position: std::collections::HashMap<V, usize>,
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> Default for MappedHeap<K, V> {
    // 🚫️async: E1 impl of external trait `std::default::Default` — must stay sync; mirrors `new()`'s
    // literal (I/O-free) body directly. See R9, and the identical `Storage::default` fix above.
    fn default() -> Self {
        Self { heap: Vec::new(), position: std::collections::HashMap::new() }
    }
}

impl<K: Ord, V: Eq + std::hash::Hash + Clone> MappedHeap<K, V> {
    pub async fn new() -> Self {
        Self { heap: Vec::new(), position: std::collections::HashMap::new() }
    }

    pub async fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub async fn len(&self) -> usize {
        self.heap.len()
    }

    pub async fn contains(&self, item: &V) -> bool {
        self.position.contains_key(item)
    }

    /// ➕️ Pushes `item` at `priority` if absent, or decreases its priority if `priority` is lower than its current one; no-operation if `item` is present with an already-lower-or-equal priority.
    pub async fn push_or_decrease(&mut self, item: V, priority: K) {
        if let Some(&i) = self.position.get(&item) {
            if priority < self.heap[i].0 {
                self.heap[i].0 = priority;
                self.sift_up(i).await;
            }
        } else {
            self.heap.push((priority, item.clone()));
            let i = self.heap.len() - 1;
            self.position.insert(item, i);
            self.sift_up(i).await;
        }
    }

    /// 🔽️ Lowers `item`'s priority; returns `false` (no-operation) if `item` isn't present or `priority` isn't lower than its current one.
    pub async fn decrease_key(&mut self, item: &V, priority: K) -> bool {
        let Some(&i) = self.position.get(item) else { return false };
        if priority < self.heap[i].0 {
            self.heap[i].0 = priority;
            self.sift_up(i).await;
            true
        } else {
            false
        }
    }

    pub async fn pop_min(&mut self) -> Option<(K, V)> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.swap(0, last).await;
        let (priority, item) = self.heap.pop().expect("heap checked non-empty above");
        self.position.remove(&item);
        if !self.heap.is_empty() {
            self.sift_down(0).await;
        }
        Some((priority, item))
    }

    async fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.position.insert(self.heap[i].1.clone(), i);
        self.position.insert(self.heap[j].1.clone(), j);
    }

    // 🚨️ was a dropped-future no-op throughout: `sift_up`/`sift_down`/`swap` were never awaited at
    // ANY of their call sites in this struct (including here, sift_up/sift_down calling their own
    // `swap`), so the whole `MappedHeap` never actually maintained the heap invariant — every push,
    // decrease-key, and pop silently left `self.heap` in insertion order. See R10 header.
    async fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[i].0 < self.heap[parent].0 {
                self.swap(i, parent).await;
                i = parent;
            } else {
                break;
            }
        }
    }

    async fn sift_down(&mut self, mut i: usize) {
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
            self.swap(i, smallest).await;
            i = smallest;
        }
    }
}
// #endregion 🔖️Utils


// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
    // an `async fn` directly (std has no executor for it), so every async test body in this
    // module runs through this instead. Sound because this crate performs no real I/O: every
    // future here resolves on its first poll, so a single poll (never a spin-park loop) is
    // enough — panics loudly if that invariant is ever violated rather than hanging.
    fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn noop(_: *const ()) {}
        fn clone_raw(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
        let raw = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(raw) };
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
        }
    }


    type NU = Storage<Normal, Undirected>;
    type ND = Storage<Normal, Directed>;
    type PU = Storage<Ported, Undirected>;
    type PD = Storage<Ported, Directed>;

    // #subregion Storage
    #[test]
    fn add_node_allocates_monotone_ids() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            assert_eq!(a, 0);
            assert_eq!(b, 1);
            assert_eq!(g.node_count().await, 2);
        });
    }

    #[test]
    fn add_node_with_id_upserts_attrs_and_bumps_allocator() {
        block_on_test(async {
            let mut g = NU::new().await;
            let mut attrs = PropertyBag::new();
            attrs.insert("color".into(), PropertyValue::String("red".into()));
            g.add_node_with_id(5, attrs).await;
            assert!(g.contains_node(5).await);
            let next = g.add_node().await;
            assert_eq!(next, 6, "auto id must skip past the caller-supplied id");

            let mut more = PropertyBag::new();
            more.insert("size".into(), PropertyValue::Number(3.0));
            g.add_node_with_id(5, more).await;
            let record = g.node_attrs(5).await.expect("node 5 exists");
            assert_eq!(record.get("color").and_then(PropertyValue::as_str), Some("red"));
            assert_eq!(record.get("size").and_then(PropertyValue::as_f64), Some(3.0));
        });
    }

    #[test]
    fn remove_node_cascades_edges_and_handles() {
        block_on_test(async {
            let mut g = PU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let ha = g.add_handle(a).expect("ported storage grants handles");
            let hb = g.add_handle(b).expect("ported storage grants handles");
            let e = g.add_edge(ha, hb).await;
            assert!(g.remove_node(a).await);
            assert!(!g.contains_node(a).await);
            assert!(g.edge_endpoints(e).await.is_none(), "incident edge must be cascaded away");
            assert!(g.handle_owner(ha).await.is_none(), "handle on the removed node must be cascaded away");
            assert_eq!(g.handles(b).await, &[hb]);
        });
    }

    #[test]
    fn normal_add_edge_upserts_instead_of_duplicating() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let mut first = PropertyBag::new();
            first.insert("weight".into(), PropertyValue::Number(1.0));
            let e1 = g.add_edge_with(a, b, first).await;
            let mut second = PropertyBag::new();
            second.insert("label".into(), PropertyValue::String("x".into()));
            let e2 = g.add_edge_with(a, b, second).await;
            assert_eq!(e1, e2, "Normal storages upsert an existing pair instead of creating a parallel edge");
            assert_eq!(g.edge_count().await, 1);
            let attrs = g.edge_attrs(e1).await.expect("edge exists");
            assert_eq!(attrs.get("weight").and_then(PropertyValue::as_f64), Some(1.0));
            assert_eq!(attrs.get("label").and_then(PropertyValue::as_str), Some("x"));
        });
    }

    #[test]
    fn ported_add_edge_always_creates_parallel_edges() {
        block_on_test(async {
            let mut g = PD::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let ha = g.add_handle(a).expect("ported");
            let hb = g.add_handle(b).expect("ported");
            let e1 = g.add_edge(ha, hb).await;
            let e2 = g.add_edge(ha, hb).await;
            assert_ne!(e1, e2, "Ported storages always create a fresh parallel edge");
            assert_eq!(g.edge_count().await, 2);
            assert_eq!(g.out_degree(a).await, 2);
        });
    }

    #[test]
    fn normal_storage_denies_handles() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            assert!(g.add_handle(a).is_none());
            assert!(g.handles(a).await.is_empty());
        });
    }

    #[test]
    fn remove_edge_unlinks_adjacency_both_ways_when_undirected() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            assert!(g.remove_edge(e).await);
            assert_eq!(g.out_degree(a).await, 0);
            assert_eq!(g.out_degree(b).await, 0);
            assert!(g.edges_between(a, b).await.next().is_none());
        });
    }

    #[test]
    fn clear_edges_keeps_nodes_clear_removes_everything() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            g.add_edge(a, b).await;
            g.clear_edges().await;
            assert_eq!(g.node_count().await, 2);
            assert_eq!(g.edge_count().await, 0);
            g.clear().await;
            assert_eq!(g.node_count().await, 0);
        });
    }

    #[test]
    fn remove_edge_and_remove_node_return_false_for_unknown_ids() {
        block_on_test(async {
            let mut g = NU::new().await;
            assert!(!g.remove_edge(999).await, "removing a never-created edge id must fail cleanly");
            assert!(!g.remove_node(999).await, "removing a never-created node id must fail cleanly");
        });
    }

    #[test]
    fn node_attrs_mut_and_edge_attrs_mut_edit_in_place_and_are_none_for_unknown_ids() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            g.node_attrs_mut(a).await.expect("node exists").insert("k".into(), PropertyValue::Number(1.0));
            g.edge_attrs_mut(e).await.expect("edge exists").insert("w".into(), PropertyValue::Number(2.0));
            assert_eq!(g.node_attrs(a).await.unwrap().get("k").and_then(PropertyValue::as_f64), Some(1.0));
            assert_eq!(g.edge_attrs(e).await.unwrap().get("w").and_then(PropertyValue::as_f64), Some(2.0));
            assert!(g.node_attrs_mut(999).await.is_none());
            assert!(g.edge_attrs_mut(999).await.is_none());
        });
    }

    #[test]
    fn add_handle_denies_missing_node_and_handle_owner_is_none_for_unknown_handle() {
        block_on_test(async {
            let mut g = PU::new().await;
            assert!(g.add_handle(999).is_none(), "cannot anchor a handle on a node that doesn't exist");
            assert!(g.handle_owner(999).await.is_none());
        });
    }

    #[test]
    fn core_edge_normalize_undirected_orders_the_pair() {
        block_on_test(async {
            assert_eq!(CoreEdge::<u64>::normalize_undirected(5, 2).await, (2, 5));
            assert_eq!(CoreEdge::<u64>::normalize_undirected(2, 5).await, (2, 5));
        });
    }
    // #endsubregion

    // #subregion GraphView
    #[test]
    fn undirected_self_loop_counts_twice_towards_degree() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            g.add_edge(a, a).await;
            assert_eq!(g.degree(a).await, 2);
            assert_eq!(g.edge_count().await, 1, "edges() still lists the self-loop once");
            assert_eq!(g.edges_between(a, a).await.count(), 2);
        });
    }

    #[test]
    fn directed_degree_is_in_plus_out() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let c = g.add_node().await;
            g.add_edge(a, b).await;
            g.add_edge(c, a).await;
            assert_eq!(g.out_degree(a).await, 1);
            assert_eq!(g.in_degree(a).await, 1);
            assert_eq!(g.degree(a).await, 2);
            assert_eq!(GraphView::neighbors(&g, a).await.collect::<Vec<_>>(), vec![b], "neighbors == out_neighbors for directed storages");
        });
    }

    #[test]
    fn undirected_in_neighbors_equals_out_neighbors() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            g.add_edge(a, b).await;
            let out: Vec<_> = g.out_neighbors(a).await.collect();
            let inn: Vec<_> = g.in_neighbors(a).await.collect();
            assert_eq!(out, inn);
        });
    }

    #[test]
    fn is_directed_and_is_multigraph_reflect_type_axes() {
        block_on_test(async {
            assert!(!NU::new().await.is_directed().await);
            assert!(ND::new().await.is_directed().await);
            assert!(!NU::new().await.is_multigraph().await);
            assert!(PU::new().await.is_multigraph().await);
        });
    }

    #[test]
    fn directed_self_loop_counts_once_each_towards_out_and_in_degree() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            g.add_edge(a, a).await;
            assert_eq!(g.out_degree(a).await, 1);
            assert_eq!(g.in_degree(a).await, 1);
            assert_eq!(g.degree(a).await, 2);
        });
    }
    // #endsubregion

    // #subregion EdgeWeights
    #[test]
    fn unit_weight_is_always_one() {
        block_on_test(async {
            let w = UnitWeight;
            assert_eq!(w.weight(EdgeRef { id: 0, u: 0, v: 1 }).await, 1.0);
        });
    }

    #[test]
    fn storage_default_weight_reads_weight_attr_with_fallback() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let mut attrs = PropertyBag::new();
            attrs.insert("weight".into(), PropertyValue::Number(4.5));
            let e = g.add_edge_with(a, b, attrs).await;
            let edge_ref = EdgeRef { id: e, u: a, v: b };
            assert_eq!(g.weight(edge_ref).await, 4.5);

            let e2 = g.add_edge(b, a).await;
            assert_eq!(e2, e, "Normal upsert must keep returning the same edge id");

            let mut g2 = NU::new().await;
            let x = g2.add_node().await;
            let y = g2.add_node().await;
            let unweighted_edge = g2.add_edge(x, y).await;
            assert_eq!(g2.weight(EdgeRef { id: unweighted_edge, u: x, v: y }).await, 1.0);
        });
    }

    #[test]
    fn attr_weight_falls_back_to_default_when_missing_or_non_numeric() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let mut attrs = PropertyBag::new();
            attrs.insert("cost".into(), PropertyValue::String("not-a-number".into()));
            let e = g.add_edge_with(a, b, attrs).await;
            let aw = AttrWeight { graph: &g, name: "cost", default: 2.0 };
            assert_eq!(aw.weight(EdgeRef { id: e, u: a, v: b }).await, 2.0);
        });
    }

    #[test]
    fn closure_implements_edge_weights() {
        block_on_test(async {
            let double = |edge: EdgeRef| (edge.id as f64) * 2.0;
            assert_eq!(double.weight(EdgeRef { id: 3, u: 0, v: 1 }).await, 6.0);
        });
    }
    // #endsubregion

    // #subregion Csr
    #[test]
    fn csr_from_view_preserves_directed_adjacency() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let c = g.add_node().await;
            g.add_edge(a, b).await;
            g.add_edge(a, c).await;
            let csr = Csr::from_view(&g).await;
            assert_eq!(csr.node_count().await, 3);
            let ia = csr.index_of(a).await.expect("a indexed");
            let ib = csr.index_of(b).await.expect("b indexed");
            let ic = csr.index_of(c).await.expect("c indexed");
            let mut out: Vec<usize> = csr.out_neighbors(ia).await.to_vec();
            out.sort_unstable();
            let mut expected = vec![ib, ic];
            expected.sort_unstable();
            assert_eq!(out, expected);
            assert_eq!(csr.node_of(ia).await, Some(a));
            assert!(csr.in_neighbors(ib).await.contains(&ia));
        });
    }

    #[test]
    fn csr_from_view_mirrors_undirected_edges_both_ways() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            g.add_edge(a, b).await;
            let csr = Csr::from_view(&g).await;
            let ia = csr.index_of(a).await.unwrap();
            let ib = csr.index_of(b).await.unwrap();
            assert!(csr.out_neighbors(ia).await.contains(&ib));
            assert!(csr.out_neighbors(ib).await.contains(&ia));
        });
    }

    #[test]
    fn csr_out_edges_and_unknown_ids_return_none() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            let csr = Csr::from_view(&g).await;
            let ia = csr.index_of(a).await.unwrap();
            assert_eq!(csr.out_edges(ia).await, &[e]);
            assert_eq!(csr.node_of(999).await, None);
            assert_eq!(csr.index_of(999).await, None);
        });
    }
    // #endsubregion

    // #subregion Views
    #[test]
    fn subgraph_view_drops_edges_leaving_the_subset() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let c = g.add_node().await;
            g.add_edge(a, b).await;
            g.add_edge(b, c).await;
            let sub = SubgraphView::new(&g, [a, b]).await;
            assert_eq!(sub.node_count().await, 2);
            assert_eq!(sub.edge_count().await, 1);
            assert!(!sub.contains_node(c).await);
        });
    }

    #[test]
    fn edge_subgraph_view_nodes_are_exactly_edge_endpoints() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let c = g.add_node().await;
            g.add_node().await; // isolated node d, never referenced by an edge
            let e_ab = g.add_edge(a, b).await;
            g.add_edge(b, c).await;
            let view = EdgeSubgraphView::new(&g, [e_ab]).await;
            let mut nodes: Vec<_> = view.nodes().await.collect();
            nodes.sort_unstable();
            assert_eq!(nodes, vec![a, b]);
            assert_eq!(view.edge_count().await, 1);
        });
    }

    #[test]
    fn subgraph_view_degree_counts_only_edges_within_subset() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let c = g.add_node().await;
            g.add_edge(a, b).await;
            g.add_edge(a, c).await;
            let sub = SubgraphView::new(&g, [a, b]).await;
            assert_eq!(sub.out_degree(a).await, 1, "the edge to c falls outside the node subset");
            assert_eq!(sub.in_degree(b).await, 1);
            assert_eq!(sub.degree(a).await, sub.out_degree(a).await + sub.in_degree(a).await, "directed subgraph degree is out+in");
            assert!(sub.is_directed().await);
            assert!(!sub.is_multigraph().await);
        });
    }

    #[test]
    fn subgraph_view_attr_view_hides_attrs_outside_the_node_subset() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            let sub = SubgraphView::new(&g, [a]).await;
            assert!(sub.node_attrs(a).await.is_some());
            assert!(sub.node_attrs(b).await.is_none(), "b is outside the node subset");
            assert!(sub.edge_attrs(e).await.is_some(), "edge attrs are not filtered by SubgraphView");
            assert!(std::ptr::eq(sub.graph_attrs().await, g.graph_attrs().await));
        });
    }

    #[test]
    fn edge_subgraph_view_degree_and_directed_flag() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let c = g.add_node().await;
            let e_ab = g.add_edge(a, b).await;
            g.add_edge(b, c).await;
            let view = EdgeSubgraphView::new(&g, [e_ab]).await;
            assert!(view.is_directed().await);
            assert_eq!(view.out_degree(a).await, 1);
            assert_eq!(view.in_degree(b).await, 1);
            assert_eq!(view.degree(a).await, 1);
            assert!(view.edge_attrs(e_ab).await.is_some());
        });
    }

    #[test]
    fn edge_subgraph_view_undirected_in_neighbors_matches_out_neighbors() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            let view = EdgeSubgraphView::new(&g, [e]).await;
            assert!(!view.is_directed().await);
            assert_eq!(view.in_neighbors(a).await.collect::<Vec<_>>(), view.out_neighbors(a).await.collect::<Vec<_>>());
            assert_eq!(view.degree(a).await, view.out_degree(a).await);
        });
    }

    #[test]
    fn reversed_view_swaps_direction_on_directed_graph() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            g.add_edge(a, b).await;
            let rev = ReversedView::new(&g).await;
            assert_eq!(rev.out_neighbors(b).await.collect::<Vec<_>>(), vec![a]);
            assert_eq!(rev.in_neighbors(a).await.collect::<Vec<_>>(), vec![b]);
        });
    }

    #[test]
    fn reversed_view_is_a_no_op_on_undirected_graph() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            g.add_edge(a, b).await;
            let rev = ReversedView::new(&g).await;
            assert_eq!(rev.out_neighbors(a).await.collect::<Vec<_>>(), g.out_neighbors(a).await.collect::<Vec<_>>());
        });
    }

    #[test]
    fn reversed_view_edges_and_edges_between_swap_endpoints() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            let rev = ReversedView::new(&g).await;
            assert_eq!(rev.edges().await.collect::<Vec<_>>(), vec![EdgeRef { id: e, u: b, v: a }]);
            assert_eq!(rev.edges_between(b, a).await.next(), Some(EdgeRef { id: e, u: b, v: a }));
            assert_eq!(rev.degree(a).await, g.degree(a).await);
            assert_eq!(rev.is_multigraph().await, g.is_multigraph().await);
        });
    }

    #[test]
    fn filtered_view_keep_predicate_hides_by_inversion() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let c = g.add_node().await;
            g.add_edge(a, b).await;
            g.add_edge(b, c).await;
            let hidden: BTreeSet<NodeId> = [b].into_iter().collect();
            let view = FilteredView::new(&g, |n| !hidden.contains(&n), |_e| true).await;
            assert!(view.contains_node(a).await);
            assert!(!view.contains_node(b).await);
            assert_eq!(view.edge_count().await, 0, "both edges touch the hidden node b");
        });
    }

    #[test]
    fn filtered_view_keep_edge_predicate_hides_specific_edges_without_hiding_nodes() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e_bad = g.add_edge(a, b).await;
            let view = FilteredView::new(&g, |_n| true, move |e| e.id != e_bad).await;
            assert!(view.contains_node(a).await);
            assert!(view.contains_node(b).await);
            assert_eq!(view.edge_count().await, 0);
            assert_eq!(view.out_degree(a).await, 0);
            assert_eq!(view.degree(a).await, 0);
        });
    }

    #[test]
    fn filtered_view_attr_view_delegates_edge_and_graph_attrs() {
        block_on_test(async {
            let mut g = NU::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            let view = FilteredView::new(&g, |_n| true, |_e| true).await;
            assert!(view.edge_attrs(e).await.is_some());
            assert!(std::ptr::eq(view.graph_attrs().await, g.graph_attrs().await));
        });
    }

    #[test]
    fn undirected_view_merges_successors_and_predecessors() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            g.add_edge(a, b).await;
            let view = UndirectedView::new(&g).await;
            assert!(!view.is_directed().await);
            assert_eq!(view.neighbors(a).await.collect::<Vec<_>>(), vec![b]);
            assert_eq!(view.neighbors(b).await.collect::<Vec<_>>(), vec![a]);
        });
    }

    #[test]
    fn undirected_view_degree_and_edges_between_merge_both_directions() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            g.add_edge(a, b).await;
            g.add_edge(b, a).await;
            let view = UndirectedView::new(&g).await;
            assert_eq!(view.degree(a).await, 2, "both directed edges count towards undirected degree");
            assert_eq!(view.edges_between(a, b).await.count(), 2);
            assert_eq!(view.is_multigraph().await, g.is_multigraph().await);
        });
    }

    #[test]
    fn undirected_view_edges_normalizes_endpoint_order() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(b, a).await;
            let view = UndirectedView::new(&g).await;
            assert_eq!(view.edges().await.collect::<Vec<_>>(), vec![EdgeRef { id: e, u: a, v: b }], "edges() orders endpoints u <= v regardless of storage direction");
        });
    }

    #[test]
    fn undirected_view_attr_view_delegates_to_parent() {
        block_on_test(async {
            let mut g = ND::new().await;
            let a = g.add_node().await;
            let b = g.add_node().await;
            let e = g.add_edge(a, b).await;
            let view = UndirectedView::new(&g).await;
            assert!(view.node_attrs(a).await.is_some());
            assert!(view.edge_attrs(e).await.is_some());
            assert!(std::ptr::eq(view.graph_attrs().await, g.graph_attrs().await));
        });
    }
    // #endsubregion

    // #subregion Interner
    #[test]
    fn interner_intern_is_idempotent() {
        block_on_test(async {
            let mut interner: Interner<String> = Interner::new().await;
            let a1 = interner.intern("alpha".to_string()).await;
            let a2 = interner.intern("alpha".to_string()).await;
            let b = interner.intern("beta".to_string()).await;
            assert_eq!(a1, a2);
            assert_ne!(a1, b);
            assert_eq!(interner.label_of(a1).await, Some(&"alpha".to_string()));
            assert_eq!(interner.id_of(&"beta".to_string()).await, Some(b));
            assert_eq!(interner.len().await, 2);
        });
    }

    #[test]
    fn interner_from_labels_is_sorted_and_deduplicated() {
        block_on_test(async {
            let interner: Interner<String> = Interner::from_labels(["c".to_string(), "a".to_string(), "a".to_string(), "b".to_string()]).await;
            assert_eq!(interner.len().await, 3);
            assert_eq!(interner.label_of(0).await, Some(&"a".to_string()));
            assert_eq!(interner.label_of(1).await, Some(&"b".to_string()));
            assert_eq!(interner.label_of(2).await, Some(&"c".to_string()));
        });
    }

    #[test]
    fn interner_is_empty_and_unknown_lookups_return_none() {
        block_on_test(async {
            let mut interner: Interner<String> = Interner::new().await;
            assert!(interner.is_empty().await);
            assert_eq!(interner.label_of(0).await, None);
            assert_eq!(interner.id_of(&"ghost".to_string()).await, None);
            interner.intern("alpha".to_string()).await;
            assert!(!interner.is_empty().await);
        });
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
        block_on_test(async {
            assert_eq!(arbitrary_element(&[9, 1, 2]).await, Some(9));
            assert_eq!(arbitrary_element::<i32>(&[]).await, None);
        });
    }

    #[test]
    fn tolerance_constants_are_ordered() {
        const { assert!(TOL_STRICT < TOL_LOOSE) };
    }

    #[test]
    fn mapped_heap_pops_in_ascending_priority_order() {
        block_on_test(async {
            let mut heap: MappedHeap<i64, &str> = MappedHeap::new().await;
            heap.push_or_decrease("c", 30).await;
            heap.push_or_decrease("a", 10).await;
            heap.push_or_decrease("b", 20).await;
            assert_eq!(heap.pop_min().await, Some((10, "a")));
            assert_eq!(heap.pop_min().await, Some((20, "b")));
            assert_eq!(heap.pop_min().await, Some((30, "c")));
            assert_eq!(heap.pop_min().await, None);
        });
    }

    #[test]
    fn mapped_heap_decrease_key_reorders() {
        block_on_test(async {
            let mut heap: MappedHeap<i64, &str> = MappedHeap::new().await;
            heap.push_or_decrease("a", 10).await;
            heap.push_or_decrease("b", 20).await;
            assert!(heap.decrease_key(&"b", 5).await);
            assert!(!heap.decrease_key(&"b", 100).await, "raising priority via decrease_key is a no-operation");
            assert_eq!(heap.pop_min().await, Some((5, "b")));
            assert!(heap.contains(&"a").await);
            assert!(!heap.contains(&"b").await);
        });
    }

    #[test]
    fn mapped_heap_len_and_is_empty_track_size() {
        block_on_test(async {
            let mut heap: MappedHeap<i64, &str> = MappedHeap::new().await;
            assert!(heap.is_empty().await);
            assert_eq!(heap.len().await, 0);
            heap.push_or_decrease("a", 5).await;
            assert!(!heap.is_empty().await);
            assert_eq!(heap.len().await, 1);
        });
    }

    #[test]
    fn mapped_heap_push_or_decrease_ignores_higher_or_equal_priority() {
        block_on_test(async {
            let mut heap: MappedHeap<i64, &str> = MappedHeap::new().await;
            heap.push_or_decrease("a", 5).await;
            heap.push_or_decrease("a", 10).await;
            assert_eq!(heap.len().await, 1, "a higher priority for an already-present item must be a no-operation");
            heap.push_or_decrease("a", 5).await;
            assert_eq!(heap.pop_min().await, Some((5, "a")), "priority must stay at the lowest value ever pushed");
        });
    }

    #[test]
    fn decrease_key_returns_false_for_absent_item() {
        block_on_test(async {
            let mut heap: MappedHeap<i64, &str> = MappedHeap::new().await;
            assert!(!heap.decrease_key(&"missing", 1).await);
        });
    }
    // #endsubregion

    // #subregion Randomized consistency (expensive-ish; kept here since it's the one genuinely property-style check in this file)
    mod quick {
        use super::*;

        /// 🎲️ Tiny deterministic xorshift so this crate doesn't need `crate::random` as a dependency just for one fuzz test.
        async fn xorshift(state: &mut u64) -> u64 {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
            *state
        }

        #[test]
        fn csr_out_degree_matches_storage_out_degree_under_random_directed_graphs() {
            block_on_test(async {
                let mut seed = 0x5eed_u64;
                for _ in 0..20 {
                    let mut g = ND::new().await;
                    let n = 3 + (xorshift(&mut seed).await % 8) as usize;
                    // 🔀️ Rewritten from `.map(..)` — `add_node` is async and cannot be called inside
                    // the sync closure that used to build `nodes` (R10 residue shape #1).
                    let mut nodes: Vec<NodeId> = Vec::with_capacity(n);
                    for _ in 0..n {
                        nodes.push(g.add_node().await);
                    }
                    let edge_attempts = n * 2;
                    for _ in 0..edge_attempts {
                        let u = nodes[(xorshift(&mut seed).await as usize) % n];
                        let v = nodes[(xorshift(&mut seed).await as usize) % n];
                        g.add_edge(u, v).await;
                    }
                    let csr = Csr::from_view(&g).await;
                    for &node in &nodes {
                        let i = csr.index_of(node).await.expect("every storage node is indexed");
                        assert_eq!(csr.out_neighbors(i).await.len(), g.out_degree(node).await, "csr out-degree must match storage out-degree for node {node}");
                    }
                }
            });
        }
    }
    // #endsubregion

    // #subregion MaxFlow
    /// 🏗️ The classic CLRS Ford-Fulkerson network (Fig. 26.1): six nodes `s=0, v1=1, v2=2, v3=3, v4=4, t=5`, known max flow `23`.
    async fn clrs_flow_network() -> FlowNetwork {
        let mut net = FlowNetwork::new(6).await;
        net.add_edge(0, 1, 16.0).await;
        net.add_edge(0, 2, 13.0).await;
        net.add_edge(1, 3, 12.0).await;
        net.add_edge(2, 1, 4.0).await;
        net.add_edge(3, 2, 9.0).await;
        net.add_edge(2, 4, 14.0).await;
        net.add_edge(4, 3, 7.0).await;
        net.add_edge(3, 5, 20.0).await;
        net.add_edge(4, 5, 4.0).await;
        net
    }

    #[test]
    fn max_flow_matches_clrs_textbook_network() {
        block_on_test(async {
            let mut net = clrs_flow_network().await;
            assert_eq!(net.max_flow(0, 5).await, 23.0);
        });
    }

    #[test]
    fn min_cut_capacity_matches_max_flow_value_duality() {
        block_on_test(async {
            let mut net = clrs_flow_network().await;
            let flow = net.max_flow(0, 5).await;
            let reachable: BTreeSet<u32> = net.min_cut(0).await.into_iter().collect();
            assert!(!reachable.contains(&5), "sink must land on the far side of a valid cut");
            let clrs_edges = [(0u32, 1u32, 16.0), (0, 2, 13.0), (1, 3, 12.0), (2, 1, 4.0), (3, 2, 9.0), (2, 4, 14.0), (4, 3, 7.0), (3, 5, 20.0), (4, 5, 4.0)];
            let crossing: f64 = clrs_edges.iter().filter(|&&(u, v, _)| reachable.contains(&u) && !reachable.contains(&v)).map(|&(_, _, cap)| cap).sum();
            assert_eq!(crossing, flow, "total capacity crossing the min cut must equal the max flow value");
        });
    }

    #[test]
    fn max_flow_saturates_branching_level_graph() {
        block_on_test(async {
            let mut net = FlowNetwork::new(5).await;
            net.add_edge(0, 1, 10.0).await;
            net.add_edge(0, 2, 10.0).await;
            net.add_edge(0, 3, 10.0).await;
            net.add_edge(1, 2, 2.0).await;
            net.add_edge(2, 3, 2.0).await;
            net.add_edge(1, 4, 4.0).await;
            net.add_edge(2, 4, 4.0).await;
            net.add_edge(3, 4, 4.0).await;
            assert_eq!(net.max_flow(0, 4).await, 12.0, "sink in-degree 3 at capacity 4 each caps the flow at 12 regardless of source out-degree 3");
        });
    }

    #[test]
    fn max_flow_is_zero_when_source_and_sink_are_disconnected() {
        block_on_test(async {
            let mut net = FlowNetwork::new(2).await;
            assert_eq!(net.max_flow(0, 1).await, 0.0);
            assert_eq!(net.min_cut(0).await, vec![0], "with no path at all, only the source itself is reachable");
        });
    }

    #[test]
    fn max_flow_and_min_cut_are_deterministic_across_fresh_instances() {
        block_on_test(async {
            let mut first = clrs_flow_network().await;
            let mut second = clrs_flow_network().await;
            let flow_a = first.max_flow(0, 5).await;
            let flow_b = second.max_flow(0, 5).await;
            assert_eq!(flow_a, flow_b, "identically constructed networks must yield byte-identical flow values");
            assert_eq!(first.min_cut(0).await, second.min_cut(0).await, "identically constructed networks must yield byte-identical min-cut node sets");
        });
    }
    // #endsubregion

    // #subregion PropertyJson
    #[test]
    fn property_bag_json_round_trips_and_empty_bag_serializes_to_none() {
        block_on_test(async {
            let mut bag = PropertyBag::new();
            bag.insert("label".into(), PropertyValue::String("hi".into()));
            bag.insert("count".into(), PropertyValue::Number(3.0));
            let json = property_bag_to_json(&bag).await.expect("non-empty bag serializes to Some");
            let round_tripped = property_bag_from_json(&json).await;
            assert_eq!(round_tripped.get("label").and_then(PropertyValue::as_str), Some("hi"));
            assert_eq!(round_tripped.get("count").and_then(PropertyValue::as_f64), Some(3.0));
            assert!(property_bag_to_json(&PropertyBag::new()).await.is_none(), "an empty bag serializes to None");
        });
    }

    #[test]
    fn property_bag_from_json_falls_back_to_default_on_unparsable_shape() {
        block_on_test(async {
            let value = serde_json::json!("not-an-object-map");
            let bag = property_bag_from_json(&value);
            assert!(bag.await.is_empty(), "a JSON value that can't deserialize into a PropertyBag falls back to empty");
        });
    }
    // #endsubregion
}
// #endregion 🔖️Tests

// #region 🔖️PropertyJson
/// 🧾️ Converts JSON fixture `userData` into a typed property bag.
pub async fn property_bag_from_json(value: &serde_json::Value) -> PropertyBag {
    serde_json::from_value(value.clone()).unwrap_or_default()
}

/// 🧾️ Serializes a property bag back to JSON for fixture export.
pub async fn property_bag_to_json(bag: &PropertyBag) -> Option<serde_json::Value> {
    if bag.is_empty() {
        None
    } else {
        serde_json::to_value(bag).ok()
    }
}
// #endregion 🔖️PropertyJson

// #region 🔖️Kinds
use geometry::Point;

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
    pub async fn new(node_count: u32) -> Self {
        Self { node_count, edges: Vec::new(), adjacency: vec![Vec::new(); node_count as usize] }
    }

    /// ➕️ Adds a directed edge `from -> to` with `capacity`, plus a zero-capacity reverse residual edge; returns the forward edge's id (its reverse is always `id ^ 1`).
    pub async fn add_edge(&mut self, from: u32, to: u32, capacity: f64) -> u32 {
        let forward_id = self.edges.len() as u32;
        self.edges.push(FlowEdge { to, capacity });
        self.adjacency[from as usize].push(forward_id);
        let reverse_id = self.edges.len() as u32;
        self.edges.push(FlowEdge { to: from, capacity: 0.0 });
        self.adjacency[to as usize].push(reverse_id);
        forward_id
    }

    /// 🌊️ BFS level graph from `source`, restricted to edges with residual capacity above `FLOW_EPS`; `None` marks nodes unreached this phase.
    async fn bfs_levels(&self, source: u32) -> Vec<Option<u32>> {
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
    async fn dfs_blocking_flow(&mut self, u: u32, sink: u32, pushed: f64, level: &[Option<u32>], cursor: &mut [usize]) -> f64 {
        if u == sink || pushed <= FLOW_EPS {
            return pushed;
        }
        while cursor[u as usize] < self.adjacency[u as usize].len() {
            let edge_id = self.adjacency[u as usize][cursor[u as usize]];
            let edge = self.edges[edge_id as usize];
            let advances = edge.capacity > FLOW_EPS && level[edge.to as usize] == level[u as usize].map(|l| l + 1);
            if advances {
                // 🔀️ Box::pin(..) breaks the self-recursion (E0733) — see R10 residue shape #3.
                let sent = Box::pin(self.dfs_blocking_flow(edge.to, sink, pushed.min(edge.capacity), level, cursor)).await;
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
    pub async fn max_flow(&mut self, source: u32, sink: u32) -> f64 {
        if source == sink {
            return 0.0;
        }
        let mut total = 0.0;
        loop {
            // 🚨️ was a dropped-future no-op: `bfs_levels` was never awaited, so `max_flow` never
            // actually built a level graph — the whole Dinic's-algorithm loop was silently inert.
            let level = self.bfs_levels(source).await;
            if level[sink as usize].is_none() {
                break;
            }
            let mut cursor = vec![0usize; self.node_count as usize];
            loop {
                let pushed = self.dfs_blocking_flow(source, sink, f64::INFINITY, &level, &mut cursor).await;
                if pushed <= FLOW_EPS {
                    break;
                }
                total += pushed;
            }
        }
        total
    }

    /// ✂️ Source side of the minimum cut, valid only after `max_flow` has run: nodes reachable from `source` over edges whose residual capacity still exceeds `FLOW_EPS`, visited in ascending id order via `Vec`-backed BFS — fully deterministic.
    pub async fn min_cut(&self, source: u32) -> Vec<u32> {
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
