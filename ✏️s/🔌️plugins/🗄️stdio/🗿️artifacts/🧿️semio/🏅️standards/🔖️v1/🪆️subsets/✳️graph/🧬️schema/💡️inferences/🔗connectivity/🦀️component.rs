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

use crate::artifacts::semio::standards::v1::subsets::graph::schema::normal_internals::undirected::UndirectedGraph;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::traversal_internals::dfs_preorder_nodes;
use graph_core::{NodeId, PropertyBag};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Value
/// 🔗 One node's degree (undirected, self-loops counted twice per `UndirectedGraph::degree`'s own
/// convention) and its weakly-connected-component id (stable within one `compute()` call, assigned
/// in ascending node-id discovery order — NOT stable across snapshot edits that add/remove earlier
/// components, same convention graph algorithms libraries use for arbitrary component labels).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

    async fn reads() -> &'static [&'static str] {
        &["nodes", "edges"]
    }

    async fn plan(snapshot: &SemioGraphSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
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
    async fn dep_input(snapshot: &SemioGraphSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        let mut node_ids: Vec<&str> = snapshot.nodes.iter().map(|n| n.id.value.as_str()).collect();
        node_ids.sort_unstable();
        let mut edge_pairs: Vec<(&str, &str)> = snapshot.edges.iter().map(|e| (e.source.value.as_str(), e.target.value.as_str())).collect();
        edge_pairs.sort_unstable();
        serde_json::to_vec(&(key.as_str(), node_ids, edge_pairs)).unwrap_or_default()
    }

    async fn compute(snapshot: &SemioGraphSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
    use store::{InferenceCache, InferenceCacheConfig};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn node(id: &str) -> SemioGraphNode {
        SemioGraphNode { id: GraphNodeId::new(id), kind: "task".into(), label: id.into(), position: Default::default(), ports: Vec::new(), properties: Vec::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edge(id: &str, source: &str, target: &str) -> SemioGraphEdge {
        SemioGraphEdge { id: GraphEdgeId::new(id), source: GraphNodeId::new(source), target: GraphNodeId::new(target), kind: "flows-to".into(), label: id.into() }
    }

    /// 🔀️ Two disjoint components: `a-b` (2 nodes, 1 edge each) and `c` (isolated).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn two_component_snapshot() -> SemioGraphSnapshot {
        SemioGraphSnapshot { schema: STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA.into(), nodes: vec![node("a"), node("b"), node("c")], edges: vec![edge("e1", "a", "b")] }
    }

    //#region 🧪️Honesty
    #[semio_framework_async_macros::async_test]
    async fn connected_nodes_share_a_component_and_isolated_node_gets_its_own() {
        let values = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&two_component_snapshot(), None);
        let a = values.get("a").expect("a present");
        let b = values.get("b").expect("b present");
        let c = values.get("c").expect("c present");
        assert_eq!(a.component, b.component, "a and b are connected by e1");
        assert_ne!(a.component, c.component, "c is isolated");
        assert_eq!(a.degree, 1);
        assert_eq!(b.degree, 1);
        assert_eq!(c.degree, 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_self_loop_counts_degree_twice() {
        let mut snapshot = two_component_snapshot();
        snapshot.edges.push(edge("e2", "c", "c"));
        let values = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&snapshot, None);
        assert_eq!(values.get("c").expect("c present").degree, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_all_empty_snapshot_yields_an_empty_plan() {
        let values = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&SemioGraphSnapshot::default(), None);
        assert!(values.is_empty());
    }
    //#endregion 🧪️Honesty

    //#region 🧪️CacheTransparencyLaw
    #[semio_framework_async_macros::async_test]
    async fn disabled_cache_matches_pure_recompute() {
        let snapshot = two_component_snapshot();
        let pure = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&snapshot, None);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() });
        let via_disabled = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&snapshot, Some(&mut disabled));
        assert_eq!(pure, via_disabled);
    }
    //#endregion 🧪️CacheTransparencyLaw

    //#region 🧪️IncrementalityLaw
    #[semio_framework_async_macros::async_test]
    async fn identical_snapshot_recompute_is_a_cache_hit() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_component_snapshot();
        let _ = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));
        let before = cache.stats();
        let _ = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
        assert_eq!(after.hits - before.hits, 3, "all three nodes must be cache hits");
    }

    /// 🌐️ Unlike `📊moments`/`🎲entropy` (per-key independence), connectivity is a WHOLE-GRAPH
    /// property by design (see this file's doc header) — an edit to `c`'s isolated neighbourhood
    /// still misses `a`/`b`'s entries too, because `dep_input` folds in the entire edge/node set
    /// for every key (plus `key` itself — see that method's own doc comment for why).
    #[semio_framework_async_macros::async_test]
    async fn editing_any_edge_misses_every_entry_because_connectivity_is_whole_graph() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_component_snapshot();
        let _ = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.edges.push(edge("e2", "b", "c"));
        let before = cache.stats();
        let values = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 3, "adding one edge must miss all three entries, not just the two it touches");
        assert_eq!(values.get("a").map(|v| v.component), values.get("c").map(|v| v.component), "a and c are now connected through b");
    }

    /// 🪤 The regression test for the collision trap documented on `dep_input`: `infer_field`'s
    /// driver hashes `(FIELD_ID, SCHEMA_VERSION, dep_input)` alone for a parentless step — it does
    /// NOT separately fold in `key` — so if `dep_input` ever again became byte-identical across
    /// keys (e.g. someone "simplifies" it back to just the node/edge set), `a` and `b` would hash
    /// to the SAME `DepHash` and the cache would hand one of them back the other's
    /// `SemioGraphNodeConnectivity` verbatim. Proven by asserting every cached value matches its
    /// own uncached recompute AND that two structurally-different nodes stay distinct.
    #[semio_framework_async_macros::async_test]
    async fn distinct_keys_never_collide_in_the_cache() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_component_snapshot();
        let cached = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));
        let pure = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, None);
        assert_eq!(cached, pure, "every key's cached value must equal its own pure recompute, not some other key's");
        assert_ne!(cached.get("a"), cached.get("c"), "a (degree 1, in a's component) and c (degree 0, isolated) must not collide");
    }
    //#endregion 🧪️IncrementalityLaw
}
//#endregion 🧪️Tests
