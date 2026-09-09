use super::*;
use crate::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphNode, STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA};
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
    let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let via_disabled = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&snapshot, Some(&mut disabled));
    assert_eq!(pure, via_disabled);
}
//#endregion 🧪️CacheTransparencyLaw

//#region 🧪️IncrementalityLaw
#[semio_framework_async_macros::async_test]
async fn identical_snapshot_recompute_is_a_cache_hit() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_component_snapshot();
    let _ = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
    assert_eq!(after.hits - before.hits, 3, "all three nodes must be cache hits");
}

/// 🌐️ Unlike `📊moments`/`🎲entropy` (per-key independence), connectivity is a WHOLE-GRAPH
/// property by design (see this file's doc header) — an edit to `c`'s isolated neighbourhood
/// still misses `a`/`b`'s entries too, because `dep_input` folds in the entire edge/node set
/// for every key (plus `key` itself — see that method's own doc comment for why).
#[semio_framework_async_macros::async_test]
async fn editing_any_edge_misses_every_entry_because_connectivity_is_whole_graph() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_component_snapshot();
    let _ = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.edges.push(edge("e2", "b", "c"));
    let before = cache.stats().await;
    let values = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&changed, Some(&mut cache));
    let after = cache.stats().await;

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
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_component_snapshot();
    let cached = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, Some(&mut cache));
    let pure = store::infer_field::<SemioGraphSnapshot, NodeConnectivity>(&base, None);
    assert_eq!(cached, pure, "every key's cached value must equal its own pure recompute, not some other key's");
    assert_ne!(cached.get("a"), cached.get("c"), "a (degree 1, in a's component) and c (degree 0, isolated) must not collide");
}
//#endregion 🧪️IncrementalityLaw
