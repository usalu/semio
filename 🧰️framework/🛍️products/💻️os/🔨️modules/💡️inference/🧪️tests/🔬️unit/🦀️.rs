
use super::*;

//#region 🧸️Fixtures
// Synthetic 3-node DAG: a "root" entity feeding two "leaf" entities, each Value = i64 sum of
// its own weight plus every ancestor's weight — smallest fixture that exercises root vs. chain
// hashing, multi-parent folding (leaf_both has two parents), and per-entity incrementality.
#[derive(Clone, Debug, Default)]
struct DagSnapshot {
    weights: BTreeMap<&'static str, i64>,
}

struct WeightSum;
impl InferredField<DagSnapshot> for WeightSum {
    // 🔑️ String, not &'static str: FromValue (needed for the session's whole-result cache in
    // `infer_field_after_diff`, via `decode_map`) can't be satisfied by a borrowed str —
    // matches real usage (e.g. Puzzle3dFlatPlane's Key = object id String).
    type Key = String;
    type Value = i64;
    const FIELD_ID: &'static str = "test.dag.weight-sum";
    const SCHEMA_VERSION: u32 = 1;
    fn reads() -> &'static [&'static str] {
        &["weights"]
    }
    fn plan(_snapshot: &DagSnapshot) -> Vec<InferenceStep<Self::Key>> {
        vec![
            InferenceStep { key: "root".to_string(), parents: vec![] },
            InferenceStep { key: "leaf_a".to_string(), parents: vec!["root".to_string()] },
            InferenceStep { key: "leaf_b".to_string(), parents: vec!["root".to_string()] },
            InferenceStep { key: "leaf_both".to_string(), parents: vec!["leaf_a".to_string(), "leaf_b".to_string()] },
        ]
    }
    fn dep_input(snapshot: &DagSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        snapshot.weights.get(key.as_str()).copied().unwrap_or(0).to_le_bytes().to_vec()
    }
    fn compute(snapshot: &DagSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        snapshot.weights.get(key.as_str()).copied().unwrap_or(0) + parents.iter().sum::<i64>()
    }
}

async fn base_snapshot() -> DagSnapshot {
    DagSnapshot { weights: BTreeMap::from([("root", 1), ("leaf_a", 2), ("leaf_b", 3), ("leaf_both", 4)]) }
}
//#endregion 🧸️Fixtures

//#region 🧪️PlanShape
#[semio_framework_async_macros::async_test]
async fn infer_field_computes_expected_values_over_the_dag() {
    let snapshot = base_snapshot().await;
    let values = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);
    assert_eq!(values["root"], 1);
    assert_eq!(values["leaf_a"], 3); // 2 + root(1)
    assert_eq!(values["leaf_b"], 4); // 3 + root(1)
    assert_eq!(values["leaf_both"], 4 + 3 + 4); // 4 + leaf_a(3) + leaf_b(4)
}
//#endregion 🧪️PlanShape

//#region 🧪️CacheTransparencyLaw
#[semio_framework_async_macros::async_test]
async fn disabled_cache_matches_pure_recompute() {
    let snapshot = base_snapshot().await;
    let pure = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);

    let mut disabled_cache = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let via_disabled_cache = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut disabled_cache));
    assert_eq!(pure, via_disabled_cache);
}

#[semio_framework_async_macros::async_test]
async fn cold_and_warm_cache_match_pure_recompute() {
    let snapshot = base_snapshot().await;
    let pure = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);

    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let cold = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache));
    assert_eq!(pure, cold);
    assert_eq!(cache.stats().await.hits, 0);
    assert!(cache.stats().await.misses > 0);

    let warm = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache));
    assert_eq!(pure, warm);
    assert!(cache.stats().await.hits > 0, "second run over the same snapshot must hit the warm cache");
}

#[semio_framework_async_macros::async_test]
async fn tiny_budget_eviction_storm_still_matches_pure_recompute() {
    let snapshot = base_snapshot().await;
    let pure = infer_field::<DagSnapshot, WeightSum>(&snapshot, None);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, budget_bytes: 1, ..Default::default() }).await;
    let via_tiny_cache = infer_field::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache));
    assert_eq!(pure, via_tiny_cache, "an eviction storm must never change the computed result, only cache hit rate");
}
//#endregion 🧪️CacheTransparencyLaw

//#region 🧪️IncrementalityLaw
#[semio_framework_async_macros::async_test]
async fn changing_a_leaf_weight_only_recomputes_that_leaf_and_its_descendants() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = base_snapshot().await;
    let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.weights.insert("leaf_a", 99);
    let before = cache.stats().await;
    let values = infer_field::<DagSnapshot, WeightSum>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    // root is untouched by leaf_a's weight change (identical dep chain) => cache hit.
    // leaf_a changed directly => miss. leaf_b's own dep chain (root's hash) is unchanged => hit.
    // leaf_both depends on leaf_a's NEW value => its dep_input is unaffected but its parent
    // chain folds leaf_a's (changed) hash, so it also misses.
    assert_eq!(after.misses - before.misses, 2, "only leaf_a and leaf_both (its descendant) may miss");
    assert_eq!(values["leaf_a"], 99 + 1);
    assert_eq!(values["leaf_b"], 3 + 1, "leaf_b must be unaffected by leaf_a's weight change");
    assert_eq!(values["root"], 1);
}

#[semio_framework_async_macros::async_test]
async fn changing_the_root_weight_recomputes_the_entire_subtree() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = base_snapshot().await;
    let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.weights.insert("root", 999);
    let before = cache.stats().await;
    let _ = infer_field::<DagSnapshot, WeightSum>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    assert_eq!(after.misses - before.misses, 4, "changing the root must miss for every entity in the DAG (all four are its descendants, root included)");
}

#[semio_framework_async_macros::async_test]
async fn identical_snapshot_recompute_is_all_cache_hits() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = base_snapshot().await;
    let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));
    let before = cache.stats().await;
    let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
    assert_eq!(after.hits - before.hits, 4);
}
//#endregion 🧪️IncrementalityLaw

//#region 🧪️VersionSaltLaw
struct WeightSumV2;
impl InferredField<DagSnapshot> for WeightSumV2 {
    type Key = String;
    type Value = i64;
    const FIELD_ID: &'static str = "test.dag.weight-sum";
    const SCHEMA_VERSION: u32 = 2;
    fn reads() -> &'static [&'static str] {
        WeightSum::reads()
    }
    fn plan(snapshot: &DagSnapshot) -> Vec<InferenceStep<Self::Key>> {
        WeightSum::plan(snapshot)
    }
    fn dep_input(snapshot: &DagSnapshot, key: &Self::Key, parents: &[Self::Key]) -> Vec<u8> {
        WeightSum::dep_input(snapshot, key, parents)
    }
    fn compute(snapshot: &DagSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        WeightSum::compute(snapshot, key, parents)
    }
}

#[semio_framework_async_macros::async_test]
async fn schema_version_bump_yields_zero_hits_on_an_otherwise_warm_cache() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = base_snapshot().await;
    let _ = infer_field::<DagSnapshot, WeightSum>(&base, Some(&mut cache));
    let before = cache.stats().await;
    let _ = infer_field::<DagSnapshot, WeightSumV2>(&base, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.hits, before.hits, "a version-salted key must never collide with the prior version's entries");
    assert_eq!(after.misses - before.misses, 4);
}
//#endregion 🧪️VersionSaltLaw

//#region 🧪️DepHash
#[semio_framework_async_macros::async_test]
async fn dep_hash_root_is_deterministic_and_input_sensitive() {
    let a = DepHash::root("field", 1, b"input-a");
    let b = DepHash::root("field", 1, b"input-a");
    let c = DepHash::root("field", 1, b"input-b");
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[semio_framework_async_macros::async_test]
async fn dep_hash_chain_is_order_independent_over_parent_set() {
    let p1 = DepHash::root("f", 1, b"p1");
    let p2 = DepHash::root("f", 1, b"p2");
    let forward = DepHash::chain("f", 1, b"self", &[p1, p2]);
    let backward = DepHash::chain("f", 1, b"self", &[p2, p1]);
    assert_eq!(forward, backward, "two entities with the same parent SET in different orders must hash identically");
}

#[semio_framework_async_macros::async_test]
async fn dep_hash_chain_differs_from_root_for_the_same_input() {
    let root = DepHash::root("f", 1, b"same");
    let chained = DepHash::chain("f", 1, b"same", &[DepHash::root("f", 1, b"parent")]);
    assert_ne!(root, chained);
}
//#endregion 🧪️DepHash

//#region 🧪️Config
#[semio_framework_async_macros::async_test]
async fn default_config_is_disabled() {
    assert!(!InferenceCacheConfig::default().enabled);
}

#[semio_framework_async_macros::async_test]
async fn clear_drops_every_entry_and_resets_used_bytes() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
    let _ = infer_field::<DagSnapshot, WeightSum>(&base_snapshot().await, Some(&mut cache));
    assert!(!cache.entries.is_empty());
    cache.clear().await;
    assert!(cache.entries.is_empty());
    assert_eq!(cache.used_bytes, 0);
}
//#endregion 🧪️Config
