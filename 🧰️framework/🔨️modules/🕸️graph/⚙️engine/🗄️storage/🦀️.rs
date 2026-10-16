//! 🧩️ Graph storage: the adjacency-map `Storage`, its view traits and weight sources, and the CSR snapshot hot algorithms read.

use super::*;

// #region 🔖️Storage
/// 📦️ Per-node record: attribute bag plus, for ported storages, the handles anchored on it (stays empty for `Normal` storages).
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`; `PropertyBag`/`PropertyValue` (`🛂️manifest`) are already covered.
#[derive(Clone, Debug, Default, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct NodeRecord {
    pub attrs: PropertyBag,
    pub handles: Vec<HandleId>,
}

/// 📦️ Per-edge record: typed endpoints (node ids for `Normal`, handle ids for `Ported`) plus attribute bag.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive, see `NodeRecord` above; `E` is always `u64` in
// practice (see `CoreEdge`'s note).
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`. Every `BTreeMap` here is keyed by `NodeId`/`EdgeId`/`HandleId`
// (all bare `u64`), not `String`, so each names a stringified-key `with` bridge (precedent: actor's
// `ShardTable`/`actor_shard_map_to_value`) rather than the `BTreeMap<String, _>`-only blanket impl.
// `#[value(bound = "...")]` replaces the derive's per-type-param auto bound (`P`/`D` themselves are
// never touched — both live only behind a skipped `PhantomData` field) with the ONE bound the
// `edges` bridge actually needs: `P::Endpoint` (always `NodeId`/`HandleId` = `u64` in practice).
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(bound = "P::Endpoint: semio_framework_value::ToValue + semio_framework_value::FromValue")]
pub struct Storage<P: PortModel, D: Directedness> {
    #[value(with = "storage_nodes_bridge")]
    pub(crate) nodes: BTreeMap<NodeId, NodeRecord>,
    #[value(with = "storage_edges_bridge")]
    pub(crate) edges: BTreeMap<EdgeId, EdgeRecord<P::Endpoint>>,
    #[value(with = "storage_adjacency_bridge")]
    successors: BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>,
    #[value(with = "storage_adjacency_bridge")]
    predecessors: BTreeMap<NodeId, BTreeMap<NodeId, Vec<EdgeId>>>,
    #[value(with = "storage_handle_owner_bridge")]
    handle_owner: BTreeMap<HandleId, NodeId>,
    graph_attrs: PropertyBag,
    next_node_id: NodeId,
    next_edge_id: EdgeId,
    next_handle_id: HandleId,
    #[value(skip)]
    _directedness: std::marker::PhantomData<D>,
    #[value(skip)]
    _port_model: std::marker::PhantomData<P>,
}

/// 🌉️ `Storage::nodes` bridge — `BTreeMap<NodeId, _>` is `u64`-keyed, not `String`-keyed, so it
/// cannot use the `BTreeMap<String, T>`-only blanket `ToValue`/`FromValue` impl (`🌱️value/🔁️codec`);
/// stringifies the key the same way `serde_json` itself would for an integer map key.
mod storage_nodes_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, super::NodeRecord>) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), semio_framework_value::ToValue::to_value(v))))
    }
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, super::NodeRecord>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage::nodes"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::NodeId = k.parse().map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k}`")))?;
                let record = <super::NodeRecord as semio_framework_value::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, record))
            })
            .collect()
    }
}

/// 🌉️ `Storage::edges` bridge — generic over `E = P::Endpoint` (always `u64` in practice), same
/// `u64`-key stringification as `storage_nodes_bridge`.
mod storage_edges_bridge {
    pub fn to_value<E: semio_framework_value::ToValue>(map: &std::collections::BTreeMap<super::EdgeId, super::EdgeRecord<E>>) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), semio_framework_value::ToValue::to_value(v))))
    }
    pub fn from_value<E: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<std::collections::BTreeMap<super::EdgeId, super::EdgeRecord<E>>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage::edges"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::EdgeId = k.parse().map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid EdgeId key `{k}`")))?;
                let record = <super::EdgeRecord<E> as semio_framework_value::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, record))
            })
            .collect()
    }
}

/// 🌉️ `Storage::successors`/`predecessors` bridge — `NodeId -> NodeId -> [EdgeId]`, both map levels
/// `u64`-keyed.
mod storage_adjacency_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, std::collections::BTreeMap<super::NodeId, Vec<super::EdgeId>>>) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(map.iter().map(|(k, inner)| {
            (k.to_string(), semio_framework_value::DslValue::object(inner.iter().map(|(k2, v)| (k2.to_string(), semio_framework_value::ToValue::to_value(v)))))
        }))
    }
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, std::collections::BTreeMap<super::NodeId, Vec<super::EdgeId>>>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage adjacency"));
        };
        entries
            .into_iter()
            .map(|(k, inner)| {
                let node_id: super::NodeId = k.parse().map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k}`")))?;
                let semio_framework_value::DslValue::Object(inner_entries) = inner else {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object").under(&k));
                };
                let inner_map = inner_entries
                    .into_iter()
                    .map(|(k2, v)| {
                        let node_id2: super::NodeId = k2.parse().map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k2}`")))?;
                        let edges = <Vec<super::EdgeId> as semio_framework_value::FromValue>::from_value(v).map_err(|e| e.under(&k2))?;
                        Ok((node_id2, edges))
                    })
                    .collect::<Result<std::collections::BTreeMap<_, _>, semio_framework_value::ValueError>>()?;
                Ok((node_id, inner_map))
            })
            .collect()
    }
}

/// 🌉️ `Storage::handle_owner` bridge — `HandleId -> NodeId`, `u64`-keyed.
mod storage_handle_owner_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::HandleId, super::NodeId>) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), semio_framework_value::ToValue::to_value(v))))
    }
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<std::collections::BTreeMap<super::HandleId, super::NodeId>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Storage::handle_owner"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let handle_id: super::HandleId = k.parse().map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid HandleId key `{k}`")))?;
                let node_id = <super::NodeId as semio_framework_value::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((handle_id, node_id))
            })
            .collect()
    }
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
        // 🚨️ all four branches were dropped-future no-ops: none of these `unlink_one` calls were
        // ever awaited, so `unlink_adjacency` silently did nothing — the outer `.await` fixed at
        // its own call sites (`remove_edge`) was necessary but not sufficient.
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
            Some(record) => {
                for (key, value) in attrs {
                    if let Some(previous) = record.attrs.insert(key, value) {
                        <PropertyValue as semio_framework_value::FromValue>::retire_decoded(previous);
                    }
                }
            }
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
            // 🚨️ was a dropped-future no-op: incident edges were never actually removed when
            // their node was — `remove_edge`'s side effects on `self.edges`/adjacency never ran.
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
                    for (key, value) in attrs {
                        if let Some(previous) = record.attrs.insert(key, value) {
                            <PropertyValue as semio_framework_value::FromValue>::retire_decoded(previous);
                        }
                    }
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
        self.link_adjacency(un, vn, id);
        id
    }

    pub fn remove_edge(&mut self, id: EdgeId) -> bool {
        let Some(record) = self.edges.remove(&id) else { return false };
        let (u, v) = (self.endpoint_node(record.source), self.endpoint_node(record.target));
        // 🚨️ was a dropped-future no-op: `unlink_adjacency` was never awaited, so a removed edge's
        // adjacency entries were silently left in place. Same class as `add_edge_with` above.
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
        <PropertyBag as semio_framework_value::FromValue>::retire_decoded(std::mem::take(&mut self.graph_attrs));
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
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
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
// 🧬️ Hand-written ToValue/FromValue (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), not derived — see `Directed`'s note (⚙️engine top) on why a semicolon-terminated unit
// struct can't use `#[derive(ToValue, FromValue)]`.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnitWeight;

impl semio_framework_value::ToValue for UnitWeight {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Null
    }
}
impl semio_framework_value::FromValue for UnitWeight {
    fn from_value(_value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(UnitWeight)
    }
}

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
        // 🔀️ Rewritten from `.map(..)` — `endpoint_node` is async and cannot be called inside the
        // sync closure that used to build each `EdgeRef` (R10 residue shape #1).
        let mut out = Vec::with_capacity(self.edges.len());
        for (&id, record) in &self.edges {
            let u = self.endpoint_node(record.source);
            let v = self.endpoint_node(record.target);
            out.push(EdgeRef { id, u, v });
        }
        out.into_iter()
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
/// 🌉️ `Csr::node_index` bridge — `BTreeMap<NodeId, usize>` is `u64`-keyed, see `storage_nodes_bridge` above.
mod csr_node_index_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, usize>) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), semio_framework_value::ToValue::to_value(v))))
    }
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, usize>, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Csr::node_index"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::NodeId = k.parse().map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid NodeId key `{k}`")))?;
                let index = <usize as semio_framework_value::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, index))
            })
            .collect()
    }
}

/// 🧊️ Frozen, index-based CSR adjacency snapshot for hot algorithms; supersedes the ad-hoc `algorithms::Adjacency` for NEW code (that type is left untouched — old call sites keep using it). Node index assignment is `0..n` in sorted `NodeId` order, so two snapshots of the same graph always assign the same indices.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct Csr {
    node_ids: Vec<NodeId>,
    #[value(with = "csr_node_index_bridge")]
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
