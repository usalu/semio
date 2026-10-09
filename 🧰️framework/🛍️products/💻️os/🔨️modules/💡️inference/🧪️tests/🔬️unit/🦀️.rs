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
    type Dependency = Vec<u8>;
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
    type Dependency = Vec<u8>;
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

struct OwnedWeightSum;
impl InferredField<DagSnapshot> for OwnedWeightSum {
    type Key=String;
    type Value=i64;
    type Dependency=Vec<i64>;
    const FIELD_ID:&'static str="test.dag.owned-weight-sum";
    const SCHEMA_VERSION:u32=1;
    fn reads()->&'static[&'static str]{WeightSum::reads()}
    fn plan(snapshot:&DagSnapshot)->Vec<InferenceStep<Self::Key>>{WeightSum::plan(snapshot)}
    fn dep_input(snapshot:&DagSnapshot,key:&Self::Key,_parents:&[Self::Key])->Self::Dependency{vec![snapshot.weights.get(key.as_str()).copied().unwrap_or(0)]}
    fn compute(snapshot:&DagSnapshot,key:&Self::Key,parents:&[Self::Value])->Self::Value{WeightSum::compute(snapshot,key,parents)}
}
#[semio_framework_async_macros::async_test]
async fn inference_owned_dependency_values_match_neutral_and_serde_oracle(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🌱️owned-dependencies/🔣️.json")).unwrap();
    let snapshot=base_snapshot().await;
    let dependency=OwnedWeightSum::dep_input(&snapshot,&"leaf_a".into(),&[]);
    assert_eq!(serde_json::to_value(&dependency).unwrap(),fixture["dependency"]);
    assert_eq!(semio_framework_pack_json::to_json_string(&dependency),serde_json::to_string(&dependency).unwrap());
    let expected:BTreeMap<String,i64>=serde_json::from_value(fixture["expected"].clone()).unwrap();
    let mut cache=InferenceCache::new(InferenceCacheConfig{enabled:true,record_stats:true,..Default::default()}).await;
    assert_eq!(infer_field::<DagSnapshot,OwnedWeightSum>(&snapshot,None),expected);
    assert_eq!(infer_field::<DagSnapshot,OwnedWeightSum>(&snapshot,Some(&mut cache)),expected);
    assert_eq!(infer_field::<DagSnapshot,OwnedWeightSum>(&snapshot,Some(&mut cache)),expected);
    assert!(cache.stats().await.hits>0);
    eprintln!("[DEBUG] Inference owned dependency values and cache transparency oracle=serde_json");
}

//#region 🧪️SteppedDriver
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
struct SlowSnapshot {
    nodes: Vec<(String, i64, Vec<String>)>,
    slices: usize,
    computes: Arc<AtomicUsize>,
    cancelled: Arc<AtomicUsize>,
}

impl SlowSnapshot {
    fn from_fixture(fixture: &serde_json::Value) -> Self {
        let nodes = fixture["nodes"].as_array().unwrap().iter().map(|node| (node["key"].as_str().unwrap().to_string(), node["weight"].as_i64().unwrap(), node["parents"].as_array().unwrap().iter().map(|parent| parent.as_str().unwrap().to_string()).collect())).collect();
        Self { nodes, slices: fixture["slices"].as_u64().unwrap() as usize, computes: Arc::default(), cancelled: Arc::default() }
    }

    fn weight(&self, key: &str) -> i64 {
        self.nodes.iter().find(|(name, _, _)| name == key).map_or(0, |(_, weight, _)| *weight)
    }
}

struct SlowJob {
    remaining: usize,
    total: usize,
    cancelled: Arc<AtomicUsize>,
}

impl InferencePending for SlowJob {
    fn cancel(&mut self) {
        self.cancelled.fetch_add(1, Ordering::SeqCst);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct SlowWeightSum;
impl InferredField<SlowSnapshot> for SlowWeightSum {
    type Key = String;
    type Value = i64;
    type Dependency = Vec<i64>;
    const FIELD_ID: &'static str = "test.dag.slow-weight-sum";
    const SCHEMA_VERSION: u32 = 1;
    fn reads() -> &'static [&'static str] {
        &["nodes"]
    }
    fn plan(snapshot: &SlowSnapshot) -> Vec<InferenceStep<Self::Key>> {
        snapshot.nodes.iter().map(|(key, _, parents)| InferenceStep { key: key.clone(), parents: parents.clone() }).collect()
    }
    fn dep_input(snapshot: &SlowSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Self::Dependency {
        vec![snapshot.weight(key)]
    }
    fn compute(snapshot: &SlowSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        snapshot.computes.fetch_add(1, Ordering::SeqCst);
        snapshot.weight(key) + parents.iter().sum::<i64>()
    }
    fn compute_step(snapshot: &SlowSnapshot, key: &Self::Key, parents: &[Self::Value], pending: &mut Option<Box<dyn InferencePending>>, fuel: usize) -> Result<ComputeStep<Self::Value>, InferenceFault> {
        let job = pending.get_or_insert_with(|| Box::new(SlowJob { remaining: snapshot.slices, total: snapshot.slices, cancelled: snapshot.cancelled.clone() }));
        let job = job.as_any_mut().downcast_mut::<SlowJob>().ok_or_else(|| InferenceFault::new("test.pending-type", "the pending slot holds another job"))?;
        let spent = fuel.min(job.remaining);
        job.remaining -= spent;
        if job.remaining > 0 {
            return Ok(ComputeStep::Working { fuel_used: spent, progress: 1.0 - job.remaining as f32 / job.total as f32 });
        }
        *pending = None;
        Ok(ComputeStep::Done { value: Self::compute(snapshot, key, parents), fuel_used: spent })
    }
}

fn stepped_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪜️stepped-driver/🔣️.json")).unwrap()
}

fn expected_values(fixture: &serde_json::Value) -> BTreeMap<String, i64> {
    serde_json::from_value(fixture["expected"].clone()).unwrap()
}

fn run_stepped(snapshot: &SlowSnapshot, mut cache: Option<&mut InferenceCache>, fuel: usize) -> (BTreeMap<String, i64>, usize, usize) {
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    let (mut calls, mut spent) = (0, 0);
    loop {
        let report = infer_field_step::<SlowSnapshot, SlowWeightSum>(snapshot, cache.as_deref_mut(), &mut cursor, &mut values, fuel).unwrap();
        calls += 1;
        spent += report.fuel_used;
        assert!(calls < 1000, "a stepped run must terminate");
        if report.done {
            return (values, calls, spent);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn stepped_driver_equals_the_unbounded_driver_for_every_fuel() {
    let fixture = stepped_fixture();
    let snapshot = SlowSnapshot::from_fixture(&fixture);
    let unbounded = try_infer_field::<SlowSnapshot, SlowWeightSum>(&snapshot, None).unwrap();
    assert_eq!(unbounded, expected_values(&fixture));
    let total = snapshot.nodes.len() * snapshot.slices;
    for fuel in 1..=total + 3 {
        let (stepped, _, spent) = run_stepped(&snapshot, None, fuel);
        assert_eq!(stepped, unbounded, "fuel {fuel}");
        assert_eq!(spent, total, "fuel {fuel}: every slice is paid for exactly once");
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
        let (cached, _, _) = run_stepped(&snapshot, Some(&mut cache), fuel);
        assert_eq!(cached, unbounded, "fuel {fuel} with a cold cache");
        let (warm, calls, warm_spent) = run_stepped(&snapshot, Some(&mut cache), fuel);
        assert_eq!(warm, unbounded, "fuel {fuel} with a warm cache");
        assert_eq!((calls, warm_spent), (1, 0), "a warm run is one free call");
    }
}

#[semio_framework_async_macros::async_test]
async fn stepped_driver_call_counts_match_the_language_agnostic_fixture() {
    let fixture = stepped_fixture();
    let snapshot = SlowSnapshot::from_fixture(&fixture);
    let by_fuel: BTreeMap<String, usize> = serde_json::from_value(fixture["stepsByFuel"].clone()).unwrap();
    for (fuel, steps) in by_fuel {
        let (_, calls, _) = run_stepped(&snapshot, None, fuel.parse().unwrap());
        assert_eq!(calls, steps, "fuel {fuel}");
    }
}

#[semio_framework_async_macros::async_test]
async fn default_compute_step_charges_one_unit_per_computed_entity_and_nothing_per_hit() {
    let snapshot = base_snapshot().await;
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    let report = infer_field_step::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache), &mut cursor, &mut values, 2).unwrap();
    assert_eq!((report.done, report.computed, report.fuel_used), (false, 2, 2));
    assert_eq!((cursor.completed(), cursor.total()), (2, 4));
    let report = infer_field_step::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache), &mut cursor, &mut values, 2).unwrap();
    assert_eq!((report.done, report.computed, report.fuel_used), (true, 2, 2));
    assert_eq!(values, infer_field::<DagSnapshot, WeightSum>(&snapshot, None));
    let mut warm_cursor = InferenceCursor::new();
    let mut warm_values = BTreeMap::new();
    let report = infer_field_step::<DagSnapshot, WeightSum>(&snapshot, Some(&mut cache), &mut warm_cursor, &mut warm_values, 1).unwrap();
    assert_eq!((report.done, report.hits, report.fuel_used), (true, 4, 0));
}

#[semio_framework_async_macros::async_test]
async fn cancelling_mid_run_keeps_finished_values_cancels_the_pending_compute_and_resumes_from_the_warm_cache() {
    let fixture = stepped_fixture();
    let snapshot = SlowSnapshot::from_fixture(&fixture);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    for _ in 0..fixture["cancelAfterSteps"].as_u64().unwrap() {
        assert!(!infer_field_step::<SlowSnapshot, SlowWeightSum>(&snapshot, Some(&mut cache), &mut cursor, &mut values, 1).unwrap().done);
    }
    cursor.cancel();
    assert!(cursor.is_cancelled());
    assert_eq!(snapshot.cancelled.load(Ordering::SeqCst), 1, "the in-flight job is cancelled exactly once");
    assert_eq!(infer_field_step::<SlowSnapshot, SlowWeightSum>(&snapshot, Some(&mut cache), &mut cursor, &mut values, 1), Err(InferenceError::Cancelled));
    let finished: BTreeMap<String, i64> = serde_json::from_value(fixture["cancelledValues"].clone()).unwrap();
    assert_eq!(values, finished, "only finished entities are committed");
    assert_eq!(cache.len(), finished.len(), "no partial value reaches the cache");
    let before = snapshot.computes.load(Ordering::SeqCst);
    let (resumed, _, _) = run_stepped(&snapshot, Some(&mut cache), 2);
    assert_eq!(resumed, expected_values(&fixture));
    assert_eq!(snapshot.computes.load(Ordering::SeqCst) - before, fixture["resumeComputes"].as_u64().unwrap() as usize, "the resumed run recomputes only the unfinished entities");
}

struct FailingField;
impl InferredField<SlowSnapshot> for FailingField {
    type Key = String;
    type Value = i64;
    type Dependency = Vec<i64>;
    const FIELD_ID: &'static str = "test.dag.failing";
    const SCHEMA_VERSION: u32 = 1;
    fn reads() -> &'static [&'static str] {
        &["nodes"]
    }
    fn plan(snapshot: &SlowSnapshot) -> Vec<InferenceStep<Self::Key>> {
        SlowWeightSum::plan(snapshot)
    }
    fn dep_input(snapshot: &SlowSnapshot, key: &Self::Key, parents: &[Self::Key]) -> Self::Dependency {
        SlowWeightSum::dep_input(snapshot, key, parents)
    }
    fn compute(snapshot: &SlowSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        SlowWeightSum::compute(snapshot, key, parents)
    }
    fn compute_step(snapshot: &SlowSnapshot, key: &Self::Key, parents: &[Self::Value], pending: &mut Option<Box<dyn InferencePending>>, fuel: usize) -> Result<ComputeStep<Self::Value>, InferenceFault> {
        if key == "leaf_b" {
            return Err(InferenceFault::new("test.fail", "leaf_b refuses"));
        }
        SlowWeightSum::compute_step(snapshot, key, parents, pending, fuel)
    }
}

#[semio_framework_async_macros::async_test]
async fn a_failing_compute_is_an_error_naming_the_entity_and_keeps_the_values_before_it() {
    let snapshot = SlowSnapshot::from_fixture(&stepped_fixture());
    let mut cursor = InferenceCursor::new();
    let mut values = BTreeMap::new();
    let error = infer_field_step::<SlowSnapshot, FailingField>(&snapshot, None, &mut cursor, &mut values, usize::MAX).unwrap_err();
    assert_eq!(error, InferenceError::Compute { key: "\"leaf_b\"".into(), fault: InferenceFault::new("test.fail", "leaf_b refuses") });
    assert_eq!(values.keys().cloned().collect::<Vec<_>>(), vec!["leaf_a".to_string(), "root".to_string()]);
    assert!(try_infer_field::<SlowSnapshot, FailingField>(&snapshot, None).is_err());
}
//#endregion 🧪️SteppedDriver

//#region 🧪️PlanContract
struct MisorderedField;
impl InferredField<DagSnapshot> for MisorderedField {
    type Key = String;
    type Value = i64;
    type Dependency = Vec<u8>;
    const FIELD_ID: &'static str = "test.dag.misordered";
    const SCHEMA_VERSION: u32 = 1;
    fn reads() -> &'static [&'static str] {
        &["weights"]
    }
    fn plan(_snapshot: &DagSnapshot) -> Vec<InferenceStep<Self::Key>> {
        vec![InferenceStep { key: "leaf_a".to_string(), parents: vec!["root".to_string()] }, InferenceStep { key: "root".to_string(), parents: vec![] }]
    }
    fn dep_input(_snapshot: &DagSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        Vec::new()
    }
    fn compute(_snapshot: &DagSnapshot, _key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        parents.iter().sum::<i64>() + 1
    }
}

#[semio_framework_async_macros::async_test]
async fn a_parent_named_before_it_is_computed_is_a_loud_error_not_a_dropped_parent() {
    let snapshot = base_snapshot().await;
    assert_eq!(try_infer_field::<DagSnapshot, MisorderedField>(&snapshot, None), Err(InferenceError::MissingParent { key: "\"leaf_a\"".into(), parent: "\"root\"".into() }));
}
//#endregion 🧪️PlanContract

//#region 🧪️TypedCache
#[derive(Clone, Debug, PartialEq)]
struct Opaque(Arc<Vec<i64>>);

struct OpaqueSum;
impl InferredField<DagSnapshot> for OpaqueSum {
    type Key = String;
    type Value = Opaque;
    type Dependency = Vec<u8>;
    const FIELD_ID: &'static str = "test.dag.opaque-sum";
    const SCHEMA_VERSION: u32 = 1;
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
        let mut chain: Vec<i64> = parents.iter().flat_map(|parent| parent.0.iter().copied()).collect();
        chain.push(snapshot.weights.get(key.as_str()).copied().unwrap_or(0));
        Opaque(Arc::new(chain))
    }
    fn value_bytes(value: &Self::Value) -> usize {
        value.0.len() * size_of::<i64>()
    }
}

#[semio_framework_async_macros::async_test]
async fn values_that_cannot_be_encoded_are_cached_typed_and_cold_equals_warm_equals_pure() {
    let snapshot = base_snapshot().await;
    let pure = infer_field::<DagSnapshot, OpaqueSum>(&snapshot, None);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    assert_eq!(infer_field::<DagSnapshot, OpaqueSum>(&snapshot, Some(&mut cache)), pure);
    assert_eq!(infer_field::<DagSnapshot, OpaqueSum>(&snapshot, Some(&mut cache)), pure);
    assert_eq!(cache.stats().await.hits, 4);
    assert_eq!(cache.used_bytes(), pure.values().map(|value| value.0.len() * 8).sum::<usize>());
}

#[semio_framework_async_macros::async_test]
async fn a_hit_hands_out_the_stored_allocation_not_a_re_decoded_copy() {
    let snapshot = base_snapshot().await;
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
    let first = infer_field::<DagSnapshot, OpaqueSum>(&snapshot, Some(&mut cache));
    let second = infer_field::<DagSnapshot, OpaqueSum>(&snapshot, Some(&mut cache));
    for (key, value) in &first {
        assert!(Arc::ptr_eq(&value.0, &second[key].0), "{key}: the warm run shares the cached allocation");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_cache_never_exceeds_its_budget_evicts_least_recently_used_first_and_reconciles_a_re_insert() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, budget_bytes: 100, record_stats: true, ..Default::default() }).await;
    let key = |name: &str| DepHash::root("test.cache", 1, name.as_bytes());
    cache.insert(key("a"), 1_i64, 40);
    cache.insert(key("b"), 2_i64, 40);
    assert_eq!(cache.get::<i64>(key("a")), Some(1), "a hit refreshes recency");
    cache.insert(key("c"), 3_i64, 40);
    assert_eq!(cache.get::<i64>(key("b")), None, "the least recently used entry is the one evicted");
    assert_eq!((cache.get::<i64>(key("a")), cache.get::<i64>(key("c"))), (Some(1), Some(3)));
    assert_eq!(cache.used_bytes(), 80);
    cache.insert(key("a"), 9_i64, 50);
    assert_eq!((cache.used_bytes(), cache.len()), (90, 2), "re-inserting a key replaces its accounting");
    assert_eq!(cache.get::<i64>(key("a")), Some(9));
    cache.insert(key("huge"), 0_i64, 101);
    assert_eq!(cache.get::<i64>(key("huge")), None, "an entry above the whole budget is never stored");
    assert!(cache.used_bytes() <= 100);
    assert_eq!(cache.get::<String>(key("a")), None, "another value type under the same hash is a miss, never a mis-cast");
    assert_eq!(cache.stats().await.evictions, 1);
}

#[semio_framework_async_macros::async_test]
async fn an_eviction_storm_with_a_budget_of_one_entry_stays_correct_and_within_budget() {
    let snapshot = base_snapshot().await;
    let pure = infer_field::<DagSnapshot, OpaqueSum>(&snapshot, None);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, budget_bytes: 8, record_stats: true, ..Default::default() }).await;
    for _ in 0..3 {
        assert_eq!(infer_field::<DagSnapshot, OpaqueSum>(&snapshot, Some(&mut cache)), pure);
        assert!(cache.used_bytes() <= 8);
    }
}
//#endregion 🧪️TypedCache

//#region 🧪️SessionGate
struct Touching(&'static [&'static str]);
impl crate::os_spr::command::DiffRegions for Touching {
    fn touches(&self) -> crate::os_spr::command::TouchedPaths {
        crate::os_spr::command::TouchedPaths::new(self.0.iter().copied())
    }
}

#[semio_framework_async_macros::async_test]
async fn the_session_root_moves_exactly_when_an_entity_dependency_chain_moves() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
    let mut session = InferenceSession::new().await;
    let base = base_snapshot().await;
    assert_eq!(session.root(WeightSum::FIELD_ID), None);
    let first = infer_field_after_diff::<DagSnapshot, WeightSum, _>(&base, &Touching(&["weights"]), &mut session, &mut cache).await;
    let root = session.root(WeightSum::FIELD_ID).expect("a run stores its root");
    let unchanged = infer_field_after_diff::<DagSnapshot, WeightSum, _>(&base, &Touching(&["weights"]), &mut session, &mut cache).await;
    assert_eq!((unchanged, session.root(WeightSum::FIELD_ID)), (first.clone(), Some(root)), "a touching diff that changed nothing keeps the root");
    let mut changed = base.clone();
    changed.weights.insert("leaf_b", 30);
    let second = infer_field_after_diff::<DagSnapshot, WeightSum, _>(&changed, &Touching(&["weights"]), &mut session, &mut cache).await;
    assert_ne!(session.root(WeightSum::FIELD_ID), Some(root), "a changed chain moves the root");
    assert_eq!(second["leaf_b"], 31);
    assert_ne!(second, first);
}

#[semio_framework_async_macros::async_test]
async fn a_diff_outside_the_reads_serves_the_stored_result_without_walking_the_plan() {
    let fixture = stepped_fixture();
    let snapshot = SlowSnapshot::from_fixture(&fixture);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let mut session = InferenceSession::new().await;
    let first = infer_field_after_diff::<SlowSnapshot, SlowWeightSum, _>(&snapshot, &Touching(&["nodes"]), &mut session, &mut cache).await;
    let computed = snapshot.computes.load(Ordering::SeqCst);
    assert_eq!(first, expected_values(&fixture));
    let gated = infer_field_after_diff::<SlowSnapshot, SlowWeightSum, _>(&snapshot, &Touching(&["camera"]), &mut session, &mut cache).await;
    assert_eq!(gated, first);
    assert_eq!(snapshot.computes.load(Ordering::SeqCst), computed, "the tier-1 gate computes nothing");
    let walked = infer_field_after_diff::<SlowSnapshot, SlowWeightSum, _>(&snapshot, &Touching(&["nodes/root"]), &mut session, &mut cache).await;
    assert_eq!(walked, first);
    assert_eq!(snapshot.computes.load(Ordering::SeqCst), computed * 2, "a diff inside the reads walks the plan again (the cache is disabled)");
}
//#endregion 🧪️SessionGate

//#region 🧪️HashingOnlyWithACache
#[derive(Clone, Debug, Default)]
struct CountedSnapshot {
    inputs: Arc<AtomicUsize>,
}

struct CountedSum;
impl InferredField<CountedSnapshot> for CountedSum {
    type Dependency = Vec<u8>;
    type Key = String;
    type Value = i64;
    const FIELD_ID: &'static str = "test.dag.counted-sum";
    const SCHEMA_VERSION: u32 = 1;
    fn reads() -> &'static [&'static str] {
        &["weights"]
    }
    fn plan(_snapshot: &CountedSnapshot) -> Vec<InferenceStep<Self::Key>> {
        vec![InferenceStep { key: "root".to_string(), parents: vec![] }, InferenceStep { key: "leaf".to_string(), parents: vec!["root".to_string()] }]
    }
    fn dep_input(snapshot: &CountedSnapshot, _key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        snapshot.inputs.fetch_add(1, Ordering::SeqCst);
        vec![1]
    }
    fn compute(_snapshot: &CountedSnapshot, _key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        1 + parents.iter().sum::<i64>()
    }
}

#[semio_framework_async_macros::async_test]
async fn dependencies_are_hashed_only_when_a_cache_can_use_them() {
    let snapshot = CountedSnapshot::default();
    let pure = infer_field::<CountedSnapshot, CountedSum>(&snapshot, None);
    assert_eq!(snapshot.inputs.load(Ordering::SeqCst), 0, "no cache: dep_input is never evaluated");
    let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    assert_eq!(infer_field::<CountedSnapshot, CountedSum>(&snapshot, Some(&mut disabled)), pure);
    assert_eq!(snapshot.inputs.load(Ordering::SeqCst), 0, "a disabled cache cannot use dependency hashes either");
    let mut enabled = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
    assert_eq!(infer_field::<CountedSnapshot, CountedSum>(&snapshot, Some(&mut enabled)), pure);
    assert_eq!(snapshot.inputs.load(Ordering::SeqCst), 2, "an enabled cache hashes every entity once");
    let mut session = InferenceSession::new().await;
    let mut idle = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let gated = infer_field_after_diff::<CountedSnapshot, CountedSum, _>(&snapshot, &Touching(&["weights"]), &mut session, &mut idle).await;
    assert_eq!(gated, pure);
    assert!(session.root(CountedSum::FIELD_ID).is_some(), "the session root is a fold of dependency hashes, so the gated driver always hashes");
}
//#endregion 🧪️HashingOnlyWithACache

//#region 🧪️DiffUpdate
#[derive(Clone)]
struct EditSnapshot {
    weights: BTreeMap<String, i64>,
    nodes: Vec<(String, Vec<String>)>,
    inputs: Arc<AtomicUsize>,
}

fn strings_of(value: &serde_json::Value) -> Vec<String> {
    value.as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_string()).collect()
}

impl EditSnapshot {
    fn from_fixture(fixture: &serde_json::Value) -> Self {
        let nodes = fixture["nodes"].as_array().unwrap();
        Self { weights: nodes.iter().map(|node| (node["key"].as_str().unwrap().to_string(), node["weight"].as_i64().unwrap())).collect(), nodes: nodes.iter().map(|node| (node["key"].as_str().unwrap().to_string(), strings_of(&node["parents"]))).collect(), inputs: Arc::default() }
    }

    fn edit(&mut self, step: &serde_json::Value) {
        for (key, weight) in step["setWeights"].as_object().into_iter().flatten() {
            self.weights.insert(key.clone(), weight.as_i64().unwrap());
        }
        for key in step["drop"].as_array().into_iter().flatten() {
            let key = key.as_str().unwrap();
            self.weights.remove(key);
            self.nodes.retain(|(name, _)| name != key);
        }
        for node in step["add"].as_array().into_iter().flatten() {
            let key = node["key"].as_str().unwrap().to_string();
            self.weights.insert(key.clone(), node["weight"].as_i64().unwrap());
            self.nodes.push((key, strings_of(&node["parents"])));
        }
        for (key, parents) in step["setParents"].as_object().into_iter().flatten() {
            self.nodes.iter_mut().filter(|(name, _)| name == key).for_each(|(_, row)| *row = strings_of(parents));
        }
    }
}

struct EditSum;
impl InferredField<EditSnapshot> for EditSum {
    type Key = String;
    type Value = i64;
    type Dependency = Vec<i64>;
    const FIELD_ID: &'static str = "test.dag.edit-sum";
    const SCHEMA_VERSION: u32 = 1;
    fn reads() -> &'static [&'static str] {
        &["weights", "nodes"]
    }
    fn plan(snapshot: &EditSnapshot) -> Vec<InferenceStep<Self::Key>> {
        snapshot.nodes.iter().map(|(key, parents)| InferenceStep { key: key.clone(), parents: parents.clone() }).collect()
    }
    fn dep_input(snapshot: &EditSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Self::Dependency {
        snapshot.inputs.fetch_add(1, Ordering::SeqCst);
        vec![snapshot.weights.get(key).copied().unwrap_or(0)]
    }
    fn compute(snapshot: &EditSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        snapshot.weights.get(key).copied().unwrap_or(0) + parents.iter().sum::<i64>()
    }
    fn touched_by(_snapshot: &EditSnapshot, key: &Self::Key, touched: &crate::os_spr::command::TouchedPaths) -> bool {
        touched.intersects_parts(&["weights", key])
    }
}

struct Strings(Vec<String>);
impl crate::os_spr::command::DiffRegions for Strings {
    fn touches(&self) -> crate::os_spr::command::TouchedPaths {
        crate::os_spr::command::TouchedPaths::new(self.0.iter().cloned())
    }
}

fn touches_of(step: &serde_json::Value) -> Strings {
    Strings(strings_of(&step["touches"]))
}

fn values_of(expected: &serde_json::Value) -> BTreeMap<String, i64> {
    serde_json::from_value(expected.clone()).unwrap()
}

fn sorted_keys(delta: &FieldDelta<String, i64>) -> Vec<String> {
    let mut keys: Vec<String> = delta.changes.iter().map(|change| change.key.clone()).collect();
    keys.sort();
    keys
}

fn diff_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔄️diff-update/🔣️.json")).unwrap()
}

fn count_of(row: &serde_json::Value, name: &str) -> usize {
    row[name].as_u64().unwrap() as usize
}

#[semio_framework_async_macros::async_test]
async fn a_diff_update_serves_exactly_the_entities_the_diff_and_their_parents_moved_and_equals_a_pure_recompute() {
    let fixture = diff_fixture();
    let mut snapshot = EditSnapshot::from_fixture(&fixture);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
    let mut session = InferenceSession::new().await;
    let first = infer_field_delta::<EditSnapshot, EditSum, _>(&snapshot, &Strings(vec!["weights".into()]), &mut session, &mut cache).unwrap();
    assert_eq!(session.field_values::<EditSnapshot, EditSum>().unwrap(), &values_of(&fixture["first"]["values"]));
    assert_eq!((first.computed, first.carried, first.confirmed), (count_of(&fixture["first"], "computed"), count_of(&fixture["first"], "carried"), count_of(&fixture["first"], "confirmed")));
    for step in fixture["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        snapshot.edit(step);
        let before = snapshot.inputs.load(Ordering::SeqCst);
        let delta = infer_field_delta::<EditSnapshot, EditSum, _>(&snapshot, &touches_of(step), &mut session, &mut cache).unwrap();
        let expected = values_of(&step["values"]);
        assert_eq!(session.field_values::<EditSnapshot, EditSum>().unwrap(), &expected, "{name}");
        assert_eq!(expected, infer_field::<EditSnapshot, EditSum>(&snapshot, None), "{name}: the update equals a pure recompute");
        let mut changed = strings_of(&step["changed"]);
        changed.sort();
        assert_eq!(sorted_keys(&delta), changed, "{name}");
        assert_eq!((delta.computed, delta.carried, delta.confirmed, delta.gated, delta.done), (count_of(step, "computed"), count_of(step, "carried"), count_of(step, "confirmed"), step["gated"].as_bool().unwrap(), true), "{name}");
        let evaluated = snapshot.inputs.load(Ordering::SeqCst) - before;
        assert_eq!(evaluated, delta.computed + delta.confirmed, "{name}: only entities that are not carried evaluate their dependency");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_cancelled_diff_update_leaves_the_session_distrusting_its_values_until_a_whole_update_ran() {
    let fixture = diff_fixture();
    let mut snapshot = EditSnapshot::from_fixture(&fixture);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, ..Default::default() }).await;
    let mut session = InferenceSession::new().await;
    for step in fixture["steps"].as_array().unwrap() {
        snapshot.edit(step);
        infer_field_delta::<EditSnapshot, EditSum, _>(&snapshot, &touches_of(step), &mut session, &mut cache).unwrap();
    }
    let cancel = &fixture["cancel"];
    snapshot.edit(cancel);
    let mut update = session.begin_update::<EditSnapshot, EditSum, _>(&touches_of(cancel), true);
    let report = step_field_update::<EditSnapshot, EditSum>(&snapshot, &mut cache, &mut update, count_of(cancel, "fuel")).unwrap();
    assert_eq!(report.done, cancel["doneAfterFuel"].as_bool().unwrap());
    update.cancel();
    assert!(matches!(step_field_update::<EditSnapshot, EditSum>(&snapshot, &mut cache, &mut update, 1), Err(InferenceError::Cancelled)));
    let partial = session.finish_update(update);
    assert!(!partial.done);
    let held = session.field_values::<EditSnapshot, EditSum>().unwrap();
    assert!(values_of(&cancel["finished"]).iter().all(|(key, value)| held[key] == *value), "what the update finished stays");
    let then = &cancel["then"];
    let delta = infer_field_delta::<EditSnapshot, EditSum, _>(&snapshot, &touches_of(then), &mut session, &mut cache).unwrap();
    assert_eq!(session.field_values::<EditSnapshot, EditSum>().unwrap(), &values_of(&then["values"]));
    assert_eq!((delta.computed, delta.carried, delta.confirmed, delta.gated), (count_of(then, "computed"), count_of(then, "carried"), count_of(then, "confirmed"), then["gated"].as_bool().unwrap()));
}

#[semio_framework_async_macros::async_test]
async fn a_diff_update_without_an_enabled_cache_is_a_plain_recompute_of_every_entity() {
    let fixture = diff_fixture();
    let mut snapshot = EditSnapshot::from_fixture(&fixture);
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let mut session = InferenceSession::new().await;
    infer_field_delta::<EditSnapshot, EditSum, _>(&snapshot, &Strings(vec!["weights".into()]), &mut session, &mut cache).unwrap();
    let step = &fixture["steps"][0];
    snapshot.edit(step);
    let delta = infer_field_delta::<EditSnapshot, EditSum, _>(&snapshot, &touches_of(step), &mut session, &mut cache).unwrap();
    assert_eq!((delta.computed, delta.carried, delta.confirmed), (6, 0, 0));
    assert_eq!(session.field_values::<EditSnapshot, EditSum>().unwrap(), &values_of(&step["values"]));
}

#[semio_framework_async_macros::async_test]
async fn the_chain_hash_equals_the_merkle_node_it_replaced_and_the_default_touched_by_says_yes() {
    let (a, b) = (DepHash::root("f", 1, b"a"), DepHash::root("f", 1, b"b"));
    let mut own = b"f".to_vec();
    own.push(0);
    own.extend_from_slice(&1u32.to_le_bytes());
    own.push(0);
    own.extend_from_slice(b"input");
    let hex = |bytes: &[u8; 32]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let mut children = vec![hex(&a.0), hex(&b.0)];
    children.sort();
    let folded = semio_framework_hash::merkle_node(&[&semio_framework_hash::hash(&own).to_hex()], children);
    assert_eq!(hex(&DepHash::chain("f", 1, b"input", &[b, a]).0), folded, "the chain hash equals the merkle node over the hex of the sorted parents");
    let snapshot = base_snapshot().await;
    assert!(WeightSum::touched_by(&snapshot, &"root".to_string(), &crate::os_spr::command::TouchedPaths::default()));
}

#[semio_framework_async_macros::async_test]
async fn touched_paths_match_a_part_with_slashes_and_ancestors_without_splitting() {
    let touched = crate::os_spr::command::TouchedPaths::new(["walls/a/b/axis", "storeys"]);
    assert!(touched.intersects_parts(&["walls", "a/b"]) && touched.intersects_parts(&["walls", "a", "b"]) && touched.intersects_parts(&["walls"]));
    assert!(touched.intersects_parts(&["storeys", "st-1"]), "a coarse write region covers every row beneath it");
    assert!(!touched.intersects_parts(&["walls", "a/c"]) && !touched.intersects_parts(&["wall"]) && !touched.intersects_parts(&["walls", "a/bb"]));
    assert_eq!(touched.intersects_parts(&["walls", "a/b"]), touched.intersects_prefix("walls/a/b"));
}
//#endregion 🧪️DiffUpdate
