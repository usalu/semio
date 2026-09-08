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
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`; `E` is always `P::Endpoint` (`NodeId`/`HandleId`, both `u64`) in
// practice, so the derive's auto-synthesized `E: ToValue + FromValue` bound is always satisfied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
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
// 🧬️ ToValue/FromValue additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), hand-written rather than derived: `#[derive(ToValue, FromValue)]` rejects a
// SEMICOLON-terminated unit struct (`Fields::Unit`, distinct from an empty-brace `struct Foo {}`,
// which IS `Fields::Named`) — changing to brace form would ripple into every value-level
// construction site of this marker type. Zero-field, so the wire shape is `null` (serde's own
// default for a unit struct, had this type ever derived `Serialize`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Directed;

impl dsl_core::ToValue for Directed {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Directed {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Directed)
    }
}

impl Directedness for Directed {
    const DIRECTED: bool = true;
}

/// ↔ Undirected edges store ordered endpoint pair.
// 🧬️ Hand-written ToValue/FromValue, see `Directed` above.
#[derive(Clone, Copy, Debug, Default)]
pub struct Undirected;

impl dsl_core::ToValue for Undirected {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Undirected {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Undirected)
    }
}

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
// 🧬️ Hand-written ToValue/FromValue (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), not derived — see `Directed`'s note on why a semicolon-terminated unit struct can't
// use `#[derive(ToValue, FromValue)]`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl dsl_core::ToValue for Normal {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Normal {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Normal)
    }
}

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
// 🧬️ Hand-written ToValue/FromValue, see `Normal` above.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ported;

impl dsl_core::ToValue for Ported {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for Ported {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
        Ok(Ported)
    }
}

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
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`; `PropertyBag`/`PropertyValue` (`🛂️manifest`) are already covered.
#[derive(Clone, Debug, Default, value_derive::ToValue, value_derive::FromValue)]
pub struct NodeRecord {
    pub attrs: PropertyBag,
    pub handles: Vec<HandleId>,
}

/// 📦️ Per-edge record: typed endpoints (node ids for `Normal`, handle ids for `Ported`) plus attribute bag.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeRecord` above; `E` is always `u64` in
// practice (see `CoreEdge`'s note).
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
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
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`. Every `BTreeMap` here is keyed by `NodeId`/`EdgeId`/`HandleId`
// (all bare `u64`), not `String`, so each names a stringified-key `with` bridge (precedent: actor's
// `ShardTable`/`actor_shard_map_to_value`) rather than the `BTreeMap<String, _>`-only blanket impl.
// `#[value(bound = "...")]` replaces the derive's per-type-param auto bound (`P`/`D` themselves are
// never touched — both live only behind a skipped `PhantomData` field) with the ONE bound the
// `edges` bridge actually needs: `P::Endpoint` (always `NodeId`/`HandleId` = `u64` in practice).
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
#[value(bound = "P::Endpoint: dsl_core::ToValue + dsl_core::FromValue")]
pub struct Storage<P: PortModel, D: Directedness> {
    #[value(with = "storage_nodes_bridge")]
    nodes: BTreeMap<NodeId, NodeRecord>,
    #[value(with = "storage_edges_bridge")]
    edges: BTreeMap<EdgeId, EdgeRecord<P::Endpoint>>,
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
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, super::NodeRecord>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, super::NodeRecord>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for Storage::nodes"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::NodeId = k.parse().map_err(|_| dsl_core::ValueError::new(format!("invalid NodeId key `{k}`")))?;
                let record = <super::NodeRecord as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, record))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🌉️ `Storage::edges` bridge — generic over `E = P::Endpoint` (always `u64` in practice), same
/// `u64`-key stringification as `storage_nodes_bridge`.
mod storage_edges_bridge {
    pub fn to_value<E: dsl_core::ToValue>(map: &std::collections::BTreeMap<super::EdgeId, super::EdgeRecord<E>>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value<E: dsl_core::FromValue>(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::EdgeId, super::EdgeRecord<E>>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for Storage::edges"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::EdgeId = k.parse().map_err(|_| dsl_core::ValueError::new(format!("invalid EdgeId key `{k}`")))?;
                let record = <super::EdgeRecord<E> as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, record))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🌉️ `Storage::successors`/`predecessors` bridge — `NodeId -> NodeId -> [EdgeId]`, both map levels
/// `u64`-keyed.
mod storage_adjacency_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, std::collections::BTreeMap<super::NodeId, Vec<super::EdgeId>>>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, inner)| {
            (k.to_string(), dsl_core::DslValue::object(inner.iter().map(|(k2, v)| (k2.to_string(), dsl_core::ToValue::to_value(v)))))
        }))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, std::collections::BTreeMap<super::NodeId, Vec<super::EdgeId>>>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for Storage adjacency"));
        };
        entries
            .into_iter()
            .map(|(k, inner)| {
                let node_id: super::NodeId = k.parse().map_err(|_| dsl_core::ValueError::new(format!("invalid NodeId key `{k}`")))?;
                let dsl_core::DslValue::Object(inner_entries) = inner else {
                    return Err(dsl_core::ValueError::new("expected an object").under(&k));
                };
                let inner_map = inner_entries
                    .into_iter()
                    .map(|(k2, v)| {
                        let node_id2: super::NodeId = k2.parse().map_err(|_| dsl_core::ValueError::new(format!("invalid NodeId key `{k2}`")))?;
                        let edges = <Vec<super::EdgeId> as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k2))?;
                        Ok((node_id2, edges))
                    })
                    .collect::<Result<std::collections::BTreeMap<_, _>, dsl_core::ValueError>>()?;
                Ok((node_id, inner_map))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🌉️ `Storage::handle_owner` bridge — `HandleId -> NodeId`, `u64`-keyed.
mod storage_handle_owner_bridge {
    pub fn to_value(map: &std::collections::BTreeMap<super::HandleId, super::NodeId>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::HandleId, super::NodeId>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for Storage::handle_owner"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let handle_id: super::HandleId = k.parse().map_err(|_| dsl_core::ValueError::new(format!("invalid HandleId key `{k}`")))?;
                let node_id = <super::NodeId as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((handle_id, node_id))
            })
            .collect()
    }
    use crate::dsl_core;
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
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
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

impl dsl_core::ToValue for UnitWeight {
    fn to_value(&self) -> dsl_core::DslValue {
        dsl_core::DslValue::Null
    }
}
impl dsl_core::FromValue for UnitWeight {
    fn from_value(_value: dsl_core::DslValue) -> Result<Self, dsl_core::ValueError> {
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
    pub fn to_value(map: &std::collections::BTreeMap<super::NodeId, usize>) -> dsl_core::DslValue {
        dsl_core::DslValue::object(map.iter().map(|(k, v)| (k.to_string(), dsl_core::ToValue::to_value(v))))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<std::collections::BTreeMap<super::NodeId, usize>, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for Csr::node_index"));
        };
        entries
            .into_iter()
            .map(|(k, v)| {
                let id: super::NodeId = k.parse().map_err(|_| dsl_core::ValueError::new(format!("invalid NodeId key `{k}`")))?;
                let index = <usize as dsl_core::FromValue>::from_value(v).map_err(|e| e.under(&k))?;
                Ok((id, index))
            })
            .collect()
    }
    use crate::dsl_core;
}

/// 🧊️ Frozen, index-based CSR adjacency snapshot for hot algorithms; supersedes the ad-hoc `algorithms::Adjacency` for NEW code (that type is left untouched — old call sites keep using it). Node index assignment is `0..n` in sorted `NodeId` order, so two snapshots of the same graph always assign the same indices.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
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

// #region 🔖️Interner
/// 🔤️ Generalized, bidirectional label<->`NodeId` map — the generic successor to the string-only `algorithms::IdIndex` (which stays untouched for old call sites). `intern` is idempotent: the same label always maps to the same id.
// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01): `L` is arbitrary (`Ord + Clone + Hash` only — the ONLY instantiation anywhere in the
// repo is `Interner<String>`, confined to this file's own `#[cfg(test)] mod tests`, never consumed
// by any other module). `by_label: HashMap<L, NodeId>` needs the `BTreeMap`/`HashMap`-blanket's
// `L: ToString` bound (`🌱️value/🔁️codec`), which the derive's auto-synthesized `L: ToValue +
// FromValue` bound does not provide — forcing a `#[value(bound = "L: ToString + FromStr + …")]`
// override would newly constrain every FUTURE `L`, an API change with zero present benefit for a
// generic type nothing outside its own tests instantiates.
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
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`, so no `#[value(...)]` rename is needed and the derive's default
// externally-tagged shape (unit variant -> bare wire-name string, data-carrying variant ->
// `{"VariantName": payload}`) is simply the new wire shape. `NotImplementedForKind`'s two fields
// moved `&'static str` -> `String`: `FromValue` needs an OWNED `Self`, and no runtime-decoded
// string can honestly become `&'static str` without leaking memory — a greenfield, no-legacy-API
// repo (CLAUDE.md) fixes the field type instead of working around it (both call sites already
// pass a string literal, which `.into()`s into `String` for free).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    NotImplementedForKind { algorithm: String, kind: String },
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
// 🧬️ ToValue/FromValue coverage deliberately SKIPPED (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01), same rationale as `Interner` above: `K`/`V` are arbitrary, nothing outside this file's
// own `#[cfg(test)] mod tests` ever instantiates `MappedHeap` (there, `MappedHeap<i64, &str>` — a
// BORROWED `V` that structurally CANNOT implement `FromValue`, which needs an owned `Self`),
// `position: HashMap<V, usize>` needs `V: ToString` the auto bound does not provide, and no
// external consumer exists to benefit from the bound override this would force.
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

    // 🚨️ was a dropped-future no-op throughout: `sift_up`/`sift_down`/`swap` were never awaited at
    // ANY of their call sites in this struct (including here, sift_up/sift_down calling their own
    // `swap`), so the whole `MappedHeap` never actually maintained the heap invariant — every push,
    // decrease-key, and pop silently left `self.heap` in insertion order. See R10 header.
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

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️PropertyValue
/// 🧾️ Converts a fixture `userData` value into a typed property bag.
pub fn property_bag_from_value(value: &dsl_core::DslValue) -> PropertyBag {
    let dsl_core::DslValue::Object(entries) = value.clone() else {
        return PropertyBag::default();
    };
    entries.into_iter().filter_map(|(k, v)| dsl_core::FromValue::from_value(v).ok().map(|pv| (k, pv))).collect()
}

/// 🧾️ Serializes a property bag back to a value for fixture export.
pub fn property_bag_to_value(bag: &PropertyBag) -> Option<dsl_core::DslValue> {
    if bag.is_empty() {
        None
    } else {
        let entries: Vec<(String, dsl_core::DslValue)> = bag.iter().map(|(k, v)| (k.clone(), dsl_core::ToValue::to_value(v))).collect();
        Some(dsl_core::DslValue::Object(entries))
    }
}
// #endregion 🔖️PropertyValue

// #region 🔖️Kinds
use geometry::Point;

/// 🌉️ `geometry::Point` bridge (`📐️geometry` is a different owner's module — RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01 splits this ticket by module, so `Point` itself is out of scope here); hand-written
/// rather than a derive since we cannot add `#[derive(ToValue, FromValue)]` to `Point`'s own
/// definition. `Node::center`/`Handle` below name this via `#[value(with = "point_bridge")]`.
mod point_bridge {
    pub fn to_value(p: &super::Point) -> dsl_core::DslValue {
        dsl_core::DslValue::object([
            ("x".to_string(), dsl_core::ToValue::to_value(&p.x)),
            ("y".to_string(), dsl_core::ToValue::to_value(&p.y)),
        ])
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<super::Point, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new("expected an object for Point"));
        };
        let x = entries.iter().find(|(k, _)| k == "x").map(|(_, v)| v.clone()).ok_or_else(|| dsl_core::ValueError::new("missing field `x`"))?;
        let y = entries.iter().find(|(k, _)| k == "y").map(|(_, v)| v.clone()).ok_or_else(|| dsl_core::ValueError::new("missing field `y`"))?;
        Ok(super::Point {
            x: <f64 as dsl_core::FromValue>::from_value(x).map_err(|e| e.under("x"))?,
            y: <f64 as dsl_core::FromValue>::from_value(y).map_err(|e| e.under("y"))?,
        })
    }
    use crate::dsl_core;
}

/// 🔵️ Circle or axis-aligned rectangle node body.
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum NodeShape {
    #[default]
    Circle,
    Rectangle,
}

/// 🪝️ Port direction for directed edge wiring.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeShape` above.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum HandleRole {
    Source,
    Target,
    #[default]
    Any,
}

/// 🏷️ Semantic kind and property payload shared by graph elements.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeShape` above; `PropertyBag` (`🛂️manifest`) already covered.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ElementSemantics {
    pub kind: Option<String>,
    pub properties: PropertyBag,
}

/// 🟠️ Retained node state with world-space center and shape extents.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `NodeShape` above; `center` bridges through
// `point_bridge` (`geometry::Point` is a different owner's module).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Node {
    pub id: NodeId,
    #[value(with = "point_bridge")]
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
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `Node` above.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
// 🧬️ `value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, value_derive::ToValue, value_derive::FromValue)]
struct FlowEdge {
    to: u32,
    capacity: f64,
}

/// 🌊️ Capacitated directed flow network on a `u32`-indexed arena, for [Dinic's algorithm](https://doi.org/10.1016/0898-1221(74)90074-0) (CLRS ch. 26). Edges are stored as forward/reverse residual pairs at adjacent slots so augmenting a path only ever touches two `Vec` entries; adjacency is `Vec<Vec<u32>>`, never a hash map, so traversal order — and therefore `min_cut`'s result — is fixed by construction order alone.
// 🧬️ `value_derive::{ToValue, FromValue}` additive, see `FlowEdge` above.
#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
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
            // 🚨️ was a dropped-future no-op: `bfs_levels` was never awaited, so `max_flow` never
            // actually built a level graph — the whole Dinic's-algorithm loop was silently inert.
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
