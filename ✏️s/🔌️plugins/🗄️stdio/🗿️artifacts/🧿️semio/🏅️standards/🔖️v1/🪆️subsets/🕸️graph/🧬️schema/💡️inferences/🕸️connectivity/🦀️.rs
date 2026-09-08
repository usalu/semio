//! 🔗 `connectivity` — per-node degree and weakly-connected-component id, computed as a genuine
//! `InferredField<SemioGraphSnapshot>` (not a bare pass-through): builds an undirected NetworkX-parity
//! graph via `normal_internals::undirected::UndirectedGraph` from `nodes`/`edges`, reads each node's
//! `degree()` off it, and assigns component ids by repeated `traversal_internals::dfs_preorder_nodes`
//! from the lowest-unvisited node in id order (deterministic).
//!
//! Wraps `🚶️traversal-internals`/`➕️normal-internals` — moved verbatim from
//! `🧰️framework/🔨️modules/🧮️math/🕸️graph/{🚶️traversal,➕️normal}` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MATHEND (zero consumers anywhere
//! in the repo; migrated under the "nothing deleted" rule — `🔧️operators-internals`/
//! `🔌️ports-internals` moved alongside but have no consumer here yet, same honest remainder
//! `📊moments`/`🎲entropy` document for their own unused siblings).
//!
//! Connectivity is a WHOLE-GRAPH property — a node's component id can depend on any edge reachable
//! from it, not just its own incident edges — so `dep_input` is deliberately the ENTIRE edge/node
//! set for every key, unlike `📊moments`/`🎲entropy`'s per-column slice. Editing ANY edge invalidates
//! EVERY cache entry; the incrementality law below proves that honestly (an edit disjoint from a
//! node's own component still misses that node's cache slot).

use crate::standards::v1::subsets::graph::schema::normal_internals::undirected::UndirectedGraph;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::graph::schema::traversal_internals::dfs_preorder_nodes;
use graph_core::{NodeId, PropertyBag};
use std::collections::BTreeMap;

//#region 🔖️Value
/// 🔗 One node's degree (undirected, self-loops counted twice per `UndirectedGraph::degree`'s own
/// convention) and its weakly-connected-component id (stable within one `compute()` call, assigned
/// in ascending node-id discovery order — NOT stable across snapshot edits that add/remove earlier
/// components, same convention graph algorithms libraries use for arbitrary component labels).
/// 🔀️ No longer dual-derives `serde`: `store::InferredField::Value` used to bound on `Serialize +
/// DeserializeOwned`, forcing every implementor onto serde regardless of its own fields — that
/// bound now reads `ToValue + FromValue` (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`), so this leaf drops the
/// serde half entirely.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioGraphNodeConnectivity {
    pub degree: u32,
    pub component: u32,
}
//#endregion 🔖️Value

//#region 🔖️Build
/// 🏗️ Builds an `UndirectedGraph` from the snapshot's `nodes`/`edges`, plus the id-value → `NodeId`
/// lookup needed to translate back. Node ids are assigned in `nodes` array order — deterministic
/// because that order is itself the persisted snapshot order, never a `HashMap` iteration order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_undirected(snapshot: &SemioGraphSnapshot) -> (UndirectedGraph, BTreeMap<String, NodeId>) {
    let mut graph = UndirectedGraph::new();
    let mut id_of: BTreeMap<String, NodeId> = BTreeMap::new();
    for (index, node) in snapshot.nodes.iter().enumerate() {
        let id = index as NodeId;
        id_of.insert(node.id.value.clone(), id);
        graph.add_node_with_id(id, PropertyBag::new());
    }
    for edge in &snapshot.edges {
        if let (Some(&u), Some(&v)) = (id_of.get(&edge.source.value), id_of.get(&edge.target.value)) {
            graph.add_edge(u, v);
        }
    }
    (graph, id_of)
}

/// 🧭️ Assigns a weakly-connected-component id to every node, discovering components by repeated
/// `dfs_preorder_nodes` seeded from the lowest-numbered unvisited node — deterministic because
/// `graph.nodes()` order follows insertion order (== `nodes` array order) and the seed scan always
/// picks the smallest remaining `NodeId`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn component_of(graph: &UndirectedGraph) -> BTreeMap<NodeId, u32> {
    let mut component: BTreeMap<NodeId, u32> = BTreeMap::new();
    let mut next_component: u32 = 0;
    let mut all_ids: Vec<NodeId> = graph.nodes().collect();
    all_ids.sort_unstable();
    for seed in all_ids {
        if component.contains_key(&seed) {
            continue;
        }
        for reached in dfs_preorder_nodes(graph, seed) {
            component.entry(reached).or_insert(next_component);
        }
        next_component += 1;
    }
    component
}
//#endregion 🔖️Build

//#region 🔖️DependencyHashChain
pub struct NodeConnectivity;

impl store::InferredField<SemioGraphSnapshot> for NodeConnectivity {
    type Key = String;
    type Value = SemioGraphNodeConnectivity;
    const FIELD_ID: &'static str = "s.stdio.semio.graph.inference.connectivity";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["nodes", "edges"]
    }

    fn plan(snapshot: &SemioGraphSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.nodes.iter().map(|n| store::InferenceStep { key: n.id.value.clone(), parents: Vec::new() }).collect()
    }

    /// 🔑 The WHOLE node/edge set (connectivity is a whole-graph property — see this file's doc
    /// header — so any edge anywhere can change any node's component id; a slice scoped to only
    /// `key`'s own incident edges would silently under-invalidate the cache), WITH `key` itself
    /// folded in. `infer_field`'s driver hashes `(FIELD_ID, SCHEMA_VERSION, dep_input)` alone for a
    /// parentless step — it does NOT separately fold in `key` — so two different keys sharing
    /// byte-identical `dep_input` would collide onto the SAME cache slot and silently hand one
    /// node's value back for another. Keying every entry with its own `key` up front is therefore
    /// load-bearing correctness, not a style choice; `changing_the_key_alone_produces_a_different_hash`
    /// below is the regression test for exactly this trap.
    fn dep_input(snapshot: &SemioGraphSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut node_ids: Vec<&str> = snapshot.nodes.iter().map(|n| n.id.value.as_str()).collect();
        node_ids.sort_unstable();
        let mut edge_pairs: Vec<(&str, &str)> = snapshot.edges.iter().map(|e| (e.source.value.as_str(), e.target.value.as_str())).collect();
        edge_pairs.sort_unstable();
        let node_ids_json: Vec<pack::JsonValue> = node_ids.iter().map(|s| pack::JsonValue::from(*s)).collect();
        let edge_pairs_json: Vec<pack::JsonValue> = edge_pairs.iter().map(|(a, b)| pack::JsonValue::Array(vec![pack::JsonValue::from(*a), pack::JsonValue::from(*b)])).collect();
        let value = pack::JsonValue::Array(vec![pack::JsonValue::from(key.as_str()), pack::JsonValue::Array(node_ids_json), pack::JsonValue::Array(edge_pairs_json)]);
        pack::json_to_string(&value).into_bytes()
    }

    fn compute(snapshot: &SemioGraphSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let (graph, id_of) = build_undirected(snapshot);
        let Some(&id) = id_of.get(key.as_str()) else {
            return SemioGraphNodeConnectivity::default();
        };
        let degree = graph.degree(id) as u32;
        let component = component_of(&graph).get(&id).copied().unwrap_or(0);
        SemioGraphNodeConnectivity { degree, component }
    }
}
//#endregion 🔖️DependencyHashChain

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
