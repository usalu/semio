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
    assert_eq!(values["leaf_a"], 3);
    assert_eq!(values["leaf_b"], 4);
    assert_eq!(values["leaf_both"], 4 + 3 + 4);
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
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌱️owned-dependencies/🔣️.json")).unwrap();
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
    cancelled: Option<Arc<AtomicUsize>>,
}

impl InferencePending for SlowJob {
    fn cancel(&mut self) {
        if let Some(cancelled)=self.cancelled.as_ref(){cancelled.fetch_add(1, Ordering::SeqCst);}
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn close_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let Some(original)=self.cancelled.as_ref()else{return Ok(true)};let grant=cx.retained_grant();
        if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes < size_of::<Arc<AtomicUsize>>(){return Ok(false)}
        if Arc::strong_count(original)<=1{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"fixture cancellation witness must retain its actual original").into())}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Arc<AtomicUsize>>(),..Default::default()})?;cx.consume_fuel(1);self.cancelled=None;Ok(true)
    }
    fn original_box_release_bytes(&self)->Option<usize>{self.cancelled.is_none().then_some(size_of::<Self>())}
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

}

fn stepped_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🪜️stepped-driver/🔣️.json")).unwrap()
}

fn expected_values(fixture: &serde_json::Value) -> BTreeMap<String, i64> {
    serde_json::from_value(fixture["expected"].clone()).unwrap()
}

fn original_inference_fixture_caller(items:usize)->(semio_framework_job::OriginalJobFixtureCaller,semio_framework_value::RetainedCloneGrant){
 let law=stepped_fixture();let p=&law["caller"];let raw=p["stock"].as_array().unwrap();let stock=semio_framework_value::RetainedCloneGrant{maximum_items:raw[0].as_u64().unwrap()as usize,maximum_copy_bytes:raw[1].as_u64().unwrap()as usize,maximum_capacity_bytes:raw[2].as_u64().unwrap()as usize,maximum_release_bytes:raw[3].as_u64().unwrap()as usize,maximum_depth:raw[4].as_u64().unwrap()as usize};
 (semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(p["operation"].as_u64().unwrap()),semio_framework_job::Generation(p["generation"].as_u64().unwrap()),stock,||Some(1),p["deadlineUs"].as_u64().unwrap(),p["fuel"].as_u64().unwrap()),semio_framework_value::RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64})
}

fn close_stepped(caller:&mut semio_framework_job::OriginalJobFixtureCaller,grant:semio_framework_value::RetainedCloneGrant,cursor:&mut InferenceCursor<String>,values:&mut InferenceValues<i64>){
 for _ in 0..10000{
  let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|{let turn=|failure:InferenceWorkFailure|match failure{InferenceWorkFailure::Original(failure)=>failure,other=>panic!("{other:?}")};if !cursor.close_original_pending_step(cx).map_err(turn)?{return Ok(false)}if !values.close_step(cx).map_err(turn)?{return Ok(false)}cursor.close_original_plan_step(cx).map_err(turn)}).unwrap());
  assert_eq!((born,freed),(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));if result{return}
 }panic!("same original driver owners did not finish paid close");
}

fn stepped_projection(snapshot:&SlowSnapshot,values:&InferenceValues<i64>)->BTreeMap<String,i64>{values.iter().map(|(ordinal,value)|(snapshot.nodes[ordinal].0.clone(),*value)).collect()}

fn run_stepped(snapshot:&SlowSnapshot,mut cache:Option<&mut InferenceCache>,items:usize)->(BTreeMap<String,i64>,usize,usize){
 let mut cursor=InferenceCursor::new();let mut values=InferenceValues::new();let(mut calls,mut spent)=(0,0);let mut progress=0.0;let(mut caller,grant)=original_inference_fixture_caller(items);
 loop{
  let(report,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|Ok(infer_field_step::<SlowSnapshot,SlowWeightSum>(snapshot,snapshot,cache.as_deref_mut(),&mut cursor,&mut values,cx))).unwrap().unwrap());
  assert_eq!((born,freed),(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));assert!(report.fuel_used<=items);assert!(report.progress>=progress);progress=report.progress;calls+=1;spent+=report.fuel_used;assert!(calls<10000);
  if report.done{let projection=stepped_projection(snapshot,&values);close_stepped(&mut caller,grant,&mut cursor,&mut values);caller.retire(grant);return(projection,calls,spent)}
 }
}

#[semio_framework_async_macros::async_test]
async fn original_stepped_driver_matches_neutral_dag_for_each_finite_turn(){
 let fixture=stepped_fixture();let snapshot=SlowSnapshot::from_fixture(&fixture);let expected=expected_values(&fixture);assert_eq!(try_infer_field::<SlowSnapshot,SlowWeightSum>(&snapshot,None).unwrap(),expected);
 for items in fixture["turnItems"].as_array().unwrap(){
  let items=items.as_u64().unwrap()as usize;let(values,calls,spent)=run_stepped(&snapshot,None,items);assert_eq!(values,expected);assert!(spent>=fixture["minimumComputeSteps"].as_u64().unwrap()as usize);assert!(calls>=snapshot.nodes.len());
 }
}

#[semio_framework_async_macros::async_test]
async fn original_stepped_cache_option_preserves_values_without_unfunded_cache_publication(){
 let fixture=stepped_fixture();let snapshot=SlowSnapshot::from_fixture(&fixture);let mut cache=InferenceCache::new(InferenceCacheConfig{enabled:true,record_stats:true,..Default::default()}).await;
 for _ in 0..2{let(values,_,spent)=run_stepped(&snapshot,Some(&mut cache),1);assert_eq!(values,expected_values(&fixture));assert!(spent>0);assert_eq!(cache.len(),0)}
}

#[semio_framework_async_macros::async_test]
async fn original_stepped_cancel_retains_finished_rows_and_closes_pending_before_source(){
 let fixture=stepped_fixture();let snapshot=SlowSnapshot::from_fixture(&fixture);let mut cursor=InferenceCursor::new();let mut values=InferenceValues::new();let(mut caller,grant)=original_inference_fixture_caller(1);
 for turn in 0..10000{
  let report=caller.turn(grant,|cx|Ok(infer_field_step::<SlowSnapshot,SlowWeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap().unwrap();assert!(!report.done);assert!(turn<9999);
  if cursor.completed()==1&&cursor.pending.is_some(){break}
 }
 cursor.cancel();assert!(cursor.is_cancelled());assert_eq!(snapshot.cancelled.load(Ordering::SeqCst),1);let finished:BTreeMap<String,i64>=serde_json::from_value(fixture["cancelledValues"].clone()).unwrap();assert_eq!(stepped_projection(&snapshot,&values),finished);
 let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|Ok(infer_field_step::<SlowSnapshot,SlowWeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());assert!(matches!(result,Err(InferenceWorkFailure::Driver(InferenceError::Cancelled))));assert_eq!((born,freed),(0,0));
 caller.cancel().cancel().await;close_stepped(&mut caller,grant,&mut cursor,&mut values);caller.retire(grant);assert!(values.terminal_is_empty());
 let before=snapshot.computes.load(Ordering::SeqCst);let(resumed,_,_)=run_stepped(&snapshot,None,1);assert_eq!(resumed,expected_values(&fixture));assert_eq!(snapshot.computes.load(Ordering::SeqCst)-before,fixture["resumeComputes"].as_u64().unwrap()as usize);
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

}

#[semio_framework_async_macros::async_test]
async fn a_failing_compute_is_an_error_naming_the_entity_and_keeps_the_values_before_it(){
 let snapshot=SlowSnapshot::from_fixture(&stepped_fixture());let mut cursor=InferenceCursor::new();let mut values=InferenceValues::new();let(mut caller,grant)=original_inference_fixture_caller(1);
 for turn in 0..10000{
  let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|Ok(infer_field_step::<SlowSnapshot,FailingField>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());assert_eq!((born,freed),(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));assert!(turn<9999);
  match result{Ok(report)=>assert!(!report.done),Err(failure)=>{assert!(matches!(&failure,InferenceWorkFailure::Original(original)if original.primary_value().unwrap().message=="leaf_b refuses"));break}}
 }
 assert_eq!(cursor.completed(),2);assert_eq!(stepped_projection(&snapshot,&values).keys().cloned().collect::<Vec<_>>(),vec!["leaf_a".to_string(),"root".to_string()]);close_stepped(&mut caller,grant,&mut cursor,&mut values);caller.retire(grant);
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
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
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
impl semio_framework_os_command::DiffRegions for Touching {
    fn touches(&self) -> semio_framework_os_command::TouchedPaths {
        semio_framework_os_command::TouchedPaths::new(self.0.iter().copied())
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
    fn touched_by(_snapshot: &EditSnapshot, key: &Self::Key, touched: &semio_framework_os_command::TouchedPaths) -> bool {
        touched.intersects_parts(&["weights", key])
    }
}

struct Strings(Vec<String>);
impl semio_framework_os_command::DiffRegions for Strings {
    fn touches(&self) -> semio_framework_os_command::TouchedPaths {
        semio_framework_os_command::TouchedPaths::new(self.0.iter().cloned())
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
    serde_json::from_str(include_str!("../🧫️fixtures/🔄️diff-update/🔣️.json")).unwrap()
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
    let (mut caller,grant)=original_inference_fixture_caller(count_of(cancel,"fuel"));
    let report=caller.turn(grant,|cx|Ok(update_field_cold::<EditSnapshot,EditSum>(&snapshot,&mut cache,&mut update))).unwrap().unwrap();
    assert_eq!(report.done, cancel["doneAfterFuel"].as_bool().unwrap());
    update.cancel();
    assert!(matches!(caller.turn(grant,|cx|Ok(update_field_cold::<EditSnapshot,EditSum>(&snapshot,&mut cache,&mut update))).unwrap(),Err(InferenceWorkFailure::Driver(InferenceError::Cancelled))));
    caller.retire(grant);
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
    assert!(WeightSum::touched_by(&snapshot, &"root".to_string(), &semio_framework_os_command::TouchedPaths::default()));
}

#[semio_framework_async_macros::async_test]
async fn touched_paths_match_a_part_with_slashes_and_ancestors_without_splitting() {
    let touched = semio_framework_os_command::TouchedPaths::new(["walls/a/b/axis", "storeys"]);
    assert!(touched.intersects_parts(&["walls", "a/b"]) && touched.intersects_parts(&["walls", "a", "b"]) && touched.intersects_parts(&["walls"]));
    assert!(touched.intersects_parts(&["storeys", "st-1"]), "a coarse write region covers every row beneath it");
    assert!(!touched.intersects_parts(&["walls", "a/c"]) && !touched.intersects_parts(&["wall"]) && !touched.intersects_parts(&["walls", "a/bb"]));
    assert_eq!(touched.intersects_parts(&["walls", "a/b"]), touched.intersects_prefix("walls/a/b"));
}
//#endregion 🧪️DiffUpdate


#[semio_framework_async_macros::async_test]
async fn original_inferred_source_binding_preserves_actual_caller_before_unfunded_plan(){
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️caller/🔣️.json")).unwrap();let p=&law["caller"];let v=p["sourceGrant"].as_array().unwrap();let source=RetainedCloneGrant{maximum_items:v[0].as_u64().unwrap()as usize,maximum_copy_bytes:v[1].as_u64().unwrap()as usize,maximum_capacity_bytes:v[2].as_u64().unwrap()as usize,maximum_release_bytes:v[3].as_u64().unwrap()as usize,maximum_depth:v[4].as_u64().unwrap()as usize};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(p["operation"].as_u64().unwrap()),Generation(p["generation"].as_u64().unwrap()),source,||Some(1),p["deadlineUs"].as_u64().unwrap(),p["fuel"].as_u64().unwrap());let snapshot=base_snapshot().await;let original=&snapshot as*const _ as usize;let mut cursor=InferenceCursor::new();let mut values=InferenceValues::new();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(denied,|cx|Ok(infer_field_step::<DagSnapshot,WeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());let step=result.unwrap();assert_eq!(step.fuel_used,0);assert_eq!((born,freed),(0,0));assert_eq!(caller.receipt(),Default::default());assert!(cursor.original_caller.is_none()&&cursor.plan.is_none()&&values.is_empty());}
 let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|Ok(infer_field_step::<DagSnapshot,WeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());assert_eq!(result.unwrap().fuel_used,1);assert_eq!((born,freed),(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!(cursor.original_caller,Some((961,13,original)));assert!(cursor.plan.is_none()&&values.is_empty());
 let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|Ok(infer_field_step::<DagSnapshot,WeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());let failure=result.unwrap_err();assert_eq!((born,freed),(0,0));assert_eq!(caller.receipt(),Default::default());assert!(matches!(&failure,InferenceWorkFailure::Original(original) if original.primary_value().unwrap().kind==semio_framework_value::ValueRefusalKind::UnsupportedOwner));assert_eq!(cursor.original_caller,Some((961,13,original)));assert!(cursor.plan.is_none()&&values.is_empty());drop(failure);
 assert!(!caller.turn(grant,|cx|Ok(cursor.close_original_plan_step(cx))).unwrap().unwrap());assert!(caller.turn(grant,|cx|Ok(cursor.close_original_plan_step(cx))).unwrap().unwrap());caller.retire(source);let(_,born,freed)=semio_framework_trace_memory_testing::observe_backing(||drop((cursor,values)));assert_eq!((born,freed),(0,0));
}

struct OriginalPlanSnapshot{nodes:Vec<String>,links:Vec<(String,String)>}
struct OriginalPlanField;
impl InferredField<OriginalPlanSnapshot>for OriginalPlanField{
 type Key=String;type Value=i64;type Dependency=Vec<u8>;
 const FIELD_ID:&'static str="test.original.inference.plan";const SCHEMA_VERSION:u32=1;
 fn reads()->&'static[&'static str]{&["nodes","links"]}
 fn plan(_snapshot:&OriginalPlanSnapshot)->Vec<InferenceStep<String>>{panic!("whole planning is not the original funded route")}



 fn dep_input(_snapshot:&OriginalPlanSnapshot,_key:&String,_parents:&[String])->Vec<u8>{panic!("dependency is outside this planning law")}
 fn compute(_snapshot:&OriginalPlanSnapshot,_key:&String,_parents:&[i64])->i64{panic!("compute is outside this planning law")}
}
#[semio_framework_async_macros::async_test]
async fn original_inferred_graph_planning_quotes_each_borrowed_source_and_page(){
 use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️plan/🔣️.json")).unwrap();let snapshot=OriginalPlanSnapshot{nodes:serde_json::from_value(law["nodes"].clone()).unwrap(),links:serde_json::from_value(law["links"].clone()).unwrap()};
 let original=snapshot.nodes.as_ptr();let link_original=snapshot.links.as_ptr();let expected:Vec<String>=serde_json::from_value(law["expected"].clone()).unwrap();
 for stop in [0usize,1,13,31,83,usize::MAX]{
  let(mut caller,source)=original_inference_fixture_caller(1);let mut cursor=InferenceCursor::new();let mut values=InferenceValues::new();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};let(mut turns,mut births,mut releases)=(0usize,0usize,0usize);
  loop{
   if turns==stop{cursor.cancel();caller.cancel().cancel().await;break}
   let position=cursor.original_plan().map(OriginalInferencePlan::position);let demand=cursor.original_plan().map(|plan|plan.demands::<OriginalPlanSnapshot,OriginalPlanField>(&snapshot,&snapshot).unwrap()).unwrap_or(semio_framework_value::RetirementDemand{depth:1,..Default::default()});
   let mut denials=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}];if demand.copy_bytes!=0{denials.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant})}if demand.capacity_bytes!=0{denials.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})}
   for denied in denials{let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(denied,|cx|Ok(infer_field_step::<OriginalPlanSnapshot,OriginalPlanField>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());assert_eq!(result.unwrap().fuel_used,0);assert_eq!((born,freed),(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(cursor.original_plan().map(OriginalInferencePlan::position),position);}
   let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|Ok(infer_field_step::<OriginalPlanSnapshot,OriginalPlanField>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());
   let receipt=caller.receipt();assert_eq!(born,receipt.retained_capacity_bytes);assert_eq!(freed,receipt.released_bytes);births+=born;releases+=freed;assert!(receipt.fits(grant));turns+=1;assert!(turns<10000);
   match result{Ok(report)=>{assert!(report.fuel_used<=1);if cursor.original_plan().is_some_and(OriginalInferencePlan::complete){break}},Err(error)=>panic!("original graph planning refused: {error:?}")}
  }
  assert_eq!(snapshot.nodes.as_ptr(),original);assert_eq!(snapshot.links.as_ptr(),link_original);assert!(values.is_empty());
  if stop==usize::MAX{let plan=cursor.original_plan().unwrap();assert_eq!((0..plan.len()).map(|i|snapshot.nodes[plan.source_ordinal(i).unwrap()].clone()).collect::<Vec<_>>(),expected);assert_eq!(plan.edge_count(),law["expectedDistinctEdges"].as_u64().unwrap()as usize);assert_eq!(plan.cyclic_start(),5);}
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(denied,|cx|Ok(cursor.close_original_plan_step(cx))).unwrap());assert_eq!(result.unwrap(),cursor.original_caller.is_none());assert_eq!((born,freed),(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress::default());}
  loop{
   if let Some(plan)=cursor.original_plan(){let demand=plan.close_demands().unwrap();let position=plan.position();let mut denied=vec![];for body in [0,1,3]{if body<demand.copy_bytes{denied.push(RetainedCloneGrant{maximum_copy_bytes:body,..grant})}}if demand.release_bytes!=0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})}for denied in denied{let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(denied,|cx|Ok(cursor.close_original_plan_step(cx))).unwrap());assert!(!result.unwrap());assert_eq!((born,freed),(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(cursor.original_plan().unwrap().position(),position);}}
   let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|Ok(cursor.close_original_plan_step(cx))).unwrap());let receipt=caller.receipt();assert_eq!(born,receipt.retained_capacity_bytes);assert_eq!(freed,receipt.released_bytes);births+=born;releases+=freed;if result.unwrap(){break}}
  assert_eq!(births,releases);assert!(cursor.original_plan().is_none()&&cursor.original_caller.is_none());caller.retire(source);let(_,born,freed)=semio_framework_trace_memory_testing::observe_backing(||drop((cursor,values)));assert_eq!((born,freed),(0,0));
 }
}


fn original_inference_scope_clock()->Option<u64>{Some(1)}

#[semio_framework_async_macros::async_test]
async fn original_inferred_scope_refuses_foreign_clock_binding_on_same_parent_and_function(){
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueRefusalKind};
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️caller/🔣️.json")).unwrap();let input=&law["caller"];let raw=law["sourceIdentity"]["nativeStock"].as_array().unwrap();let stock=RetainedCloneGrant{maximum_items:raw[0].as_u64().unwrap()as usize,maximum_copy_bytes:raw[1].as_u64().unwrap()as usize,maximum_capacity_bytes:raw[2].as_u64().unwrap()as usize,maximum_release_bytes:raw[3].as_u64().unwrap()as usize,maximum_depth:raw[4].as_u64().unwrap()as usize};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(input["operation"].as_u64().unwrap()),Generation(input["generation"].as_u64().unwrap()),stock,original_inference_scope_clock,input["deadlineUs"].as_u64().unwrap(),law["sourceIdentity"]["nativeFuel"].as_u64().unwrap());let original=caller.original_clock_binding();let snapshot=base_snapshot().await;let mut cursor=InferenceCursor::new();let mut values=InferenceValues::new();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 caller.turn(grant,|cx|{cx.bind_original_clock_binding(original)?;Ok(infer_field_step::<DagSnapshot,WeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))}).unwrap().unwrap();
 let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(grant,|cx|{Ok(match cx.bind_original_clock_binding(original+1){Err(error)=>Err(InferenceWorkFailure::from(error)),Ok(())=>infer_field_step::<DagSnapshot,WeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx)})}).unwrap());assert!(matches!(&result,Err(InferenceWorkFailure::Original(failure))if failure.primary_value().unwrap().kind==ValueRefusalKind::InvariantViolated));assert_eq!((born,freed),(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress::default());drop(result);
 assert!(!caller.turn(grant,|cx|{cx.bind_original_clock_binding(original)?;Ok(cursor.close_original_plan_step(cx))}).unwrap().unwrap());assert!(caller.turn(grant,|cx|{cx.bind_original_clock_binding(original)?;Ok(cursor.close_original_plan_step(cx))}).unwrap().unwrap());caller.retire(stock);
}

#[semio_framework_async_macros::async_test]
async fn original_inferred_source_scope_refuses_foreign_root_with_same_business_identity(){
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueRefusalKind};
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️caller/🔣️.json")).unwrap();let declared=&law["caller"];let raw=law["sourceIdentity"]["nativeStock"].as_array().unwrap();let stock=RetainedCloneGrant{maximum_items:raw[0].as_u64().unwrap()as usize,maximum_copy_bytes:raw[1].as_u64().unwrap()as usize,maximum_capacity_bytes:raw[2].as_u64().unwrap()as usize,maximum_release_bytes:raw[3].as_u64().unwrap()as usize,maximum_depth:raw[4].as_u64().unwrap()as usize};
 let operation=OperationId(declared["operation"].as_u64().unwrap());let generation=Generation(declared["generation"].as_u64().unwrap());let deadline=declared["deadlineUs"].as_u64().unwrap();let fuel=law["sourceIdentity"]["nativeFuel"].as_u64().unwrap();let snapshot=base_snapshot().await;let mut cursor=InferenceCursor::new();let mut values=InferenceValues::new();
 let mut caller=OriginalJobFixtureCaller::admit(operation,generation,stock,original_inference_scope_clock,deadline,fuel);
 let metadata=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 let first=caller.turn(metadata,|cx|Ok(infer_field_step::<DagSnapshot,WeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap().unwrap();assert_eq!(first.fuel_used,1);
 let mut foreign=OriginalJobFixtureCaller::admit(operation,generation,stock,original_inference_scope_clock,deadline,fuel);
 let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||foreign.turn(metadata,|cx|Ok(infer_field_step::<DagSnapshot,WeightSum>(&snapshot,&snapshot,None,&mut cursor,&mut values,cx))).unwrap());let failure=result.unwrap_err();assert!(matches!(&failure,InferenceWorkFailure::Original(original)if original.primary_value().unwrap().kind==ValueRefusalKind::InvariantViolated));assert_eq!((born,freed),(0,0));assert_eq!(foreign.receipt(),RetainedCloneProgress::default());assert!(values.is_empty());drop(failure);foreign.retire(stock);
 assert!(!caller.turn(metadata,|cx|Ok(cursor.close_original_plan_step(cx))).unwrap().unwrap());assert!(caller.turn(metadata,|cx|Ok(cursor.close_original_plan_step(cx))).unwrap().unwrap());assert!(cursor.original_scope.is_none()&&cursor.original_caller.is_none());caller.retire(stock);
}

impl OriginalInferredField<DagSnapshot> for WeightSum {
 type Source=DagSnapshot;
 fn validate_source(snapshot:&DagSnapshot,source:&Self::Source,cx:&semio_framework_job::StepContext<'_>)->Result<(),InferenceWorkFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if !std::ptr::eq(snapshot,source){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native test inference changed its original input owner").into())}Ok(())}
fn compute_step(_snapshot:&DagSnapshot,_source:&Self::Source,_key:&Self::Key,_parents:&[Self::Value],_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"authored field has no original compute_step producer").into())}
fn compute_cyclic_step(_snapshot:&DagSnapshot,_source:&Self::Source,_key:&Self::Key,_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"authored field has no original compute_cyclic_step producer").into())}
}

impl OriginalInferredField<SlowSnapshot> for SlowWeightSum {
 type Source=SlowSnapshot;
 fn validate_source(snapshot:&SlowSnapshot,source:&Self::Source,cx:&semio_framework_job::StepContext<'_>)->Result<(),InferenceWorkFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if !std::ptr::eq(snapshot,source){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native test inference changed its original input owner").into())}Ok(())}
fn plan_source_extent(snapshot:&SlowSnapshot,_source:&Self::Source)->Option<(usize,usize)>{Some((snapshot.nodes.len(),snapshot.nodes.iter().map(|row|row.2.len()).sum()))}
fn plan_source_node<'view>(snapshot:&'view SlowSnapshot,_source:&'view Self::Source,ordinal:usize)->Option<&'view str>{snapshot.nodes.get(ordinal).map(|row|row.0.as_str())}
fn plan_source_key<'view>(snapshot:&'view SlowSnapshot,_source:&'view Self::Source,ordinal:usize)->Option<&'view String>{snapshot.nodes.get(ordinal).map(|row|&row.0)}
fn plan_source_link<'view>(snapshot:&'view SlowSnapshot,_source:&'view Self::Source,mut ordinal:usize)->Option<(&'view str,&'view str)>{for row in &snapshot.nodes{if ordinal<row.2.len(){return Some((row.2[ordinal].as_str(),row.0.as_str()))}ordinal-=row.2.len()}None}
fn clone_original_parent(original:&i64,output:&mut Option<i64>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        if output.is_some(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"fixture parent donor is occupied").into())}
        if !original_field_metadata(cx,size_of::<i64>())?{return Ok(false)}*output=Some(*original);Ok(true)
    }
fn compute_step(snapshot: &SlowSnapshot,_source:&Self::Source, key: &Self::Key, parents: &[Self::Value], pending:&mut Option<Box<dyn InferencePending>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure> {
        if pending.is_none(){
            let grant=cx.retained_grant();let copy=size_of::<SlowJob>()+size_of::<Arc<AtomicUsize>>();let capacity=size_of::<SlowJob>();
            if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity{return Ok(ComputeStep::Working{fuel_used:0,progress:0.0})}
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);
            *pending=Some(Box::new(SlowJob{remaining:snapshot.slices,total:snapshot.slices,cancelled:Some(Arc::clone(&snapshot.cancelled))}));return Ok(ComputeStep::Working{fuel_used:1,progress:0.0})
        }
        let job = pending.as_mut().unwrap();
        let job = job.as_any_mut().downcast_mut::<SlowJob>().ok_or_else(|| InferenceFault::new("test.pending-type", "the pending slot holds another job"))?;
        let copy=if job.remaining==1{size_of::<i64>()}else{0};if !original_field_metadata(cx,copy)?{return Ok(ComputeStep::Working{fuel_used:0,progress:1.0-job.remaining as f32/job.total as f32})}
        let spent=1;job.remaining=job.remaining.checked_sub(1).expect("fixture pending producer advances once");
        if job.remaining > 0 {
            return Ok(ComputeStep::Working { fuel_used: spent, progress: 1.0 - job.remaining as f32 / job.total as f32 });
        }
        Ok(ComputeStep::Done { value: Self::compute(snapshot, key, parents), fuel_used: spent })
    }
fn compute_cyclic_step(_snapshot:&SlowSnapshot,_source:&Self::Source,_key:&Self::Key,_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"authored field has no original compute_cyclic_step producer").into())}
}

impl OriginalInferredField<SlowSnapshot> for FailingField {
 type Source=SlowSnapshot;
 fn validate_source(snapshot:&SlowSnapshot,source:&Self::Source,cx:&semio_framework_job::StepContext<'_>)->Result<(),InferenceWorkFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if !std::ptr::eq(snapshot,source){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native test inference changed its original input owner").into())}Ok(())}
fn plan_source_extent(snapshot:&SlowSnapshot,_source:&Self::Source)->Option<(usize,usize)>{SlowWeightSum::plan_source_extent(snapshot,_source)}
fn plan_source_node<'view>(snapshot:&'view SlowSnapshot,_source:&'view Self::Source,ordinal:usize)->Option<&'view str>{SlowWeightSum::plan_source_node(snapshot,_source,ordinal)}
fn plan_source_key<'view>(snapshot:&'view SlowSnapshot,_source:&'view Self::Source,ordinal:usize)->Option<&'view String>{SlowWeightSum::plan_source_key(snapshot,_source,ordinal)}
fn plan_source_link<'view>(snapshot:&'view SlowSnapshot,_source:&'view Self::Source,ordinal:usize)->Option<(&'view str,&'view str)>{SlowWeightSum::plan_source_link(snapshot,_source,ordinal)}
fn clone_original_parent(original:&i64,output:&mut Option<i64>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,InferenceWorkFailure>{SlowWeightSum::clone_original_parent(original,output,cx)}
fn compute_step(snapshot: &SlowSnapshot,_source:&Self::Source, key: &Self::Key, parents: &[Self::Value], pending:&mut Option<Box<dyn InferencePending>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure> {
        if key == "leaf_b" {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"leaf_b refuses").into());
        }
        SlowWeightSum::compute_step(snapshot,_source,key,parents,pending,cx)
    }
fn compute_cyclic_step(_snapshot:&SlowSnapshot,_source:&Self::Source,_key:&Self::Key,_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"authored field has no original compute_cyclic_step producer").into())}
}

impl OriginalInferredField<OriginalPlanSnapshot>for OriginalPlanField{
 type Source=OriginalPlanSnapshot;
 fn validate_source(snapshot:&OriginalPlanSnapshot,source:&Self::Source,cx:&semio_framework_job::StepContext<'_>)->Result<(),InferenceWorkFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if !std::ptr::eq(snapshot,source){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native test inference changed its original input owner").into())}Ok(())}
fn plan_source_extent(snapshot:&OriginalPlanSnapshot,_source:&Self::Source)->Option<(usize,usize)>{Some((snapshot.nodes.len(),snapshot.links.len()))}
fn plan_source_node<'view>(snapshot:&'view OriginalPlanSnapshot,_source:&'view Self::Source,index:usize)->Option<&'view str>{snapshot.nodes.get(index).map(String::as_str)}
fn plan_source_link<'view>(snapshot:&'view OriginalPlanSnapshot,_source:&'view Self::Source,index:usize)->Option<(&'view str,&'view str)>{snapshot.links.get(index).map(|(from,to)|(from.as_str(),to.as_str()))}
fn compute_step(_snapshot:&OriginalPlanSnapshot,_source:&Self::Source,_key:&Self::Key,_parents:&[Self::Value],_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"authored field has no original compute_step producer").into())}
fn compute_cyclic_step(_snapshot:&OriginalPlanSnapshot,_source:&Self::Source,_key:&Self::Key,_pending:&mut Option<Box<dyn InferencePending>>,_cx:&mut semio_framework_job::StepContext<'_>)->Result<ComputeStep<Self::Value>,InferenceWorkFailure>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"authored field has no original compute_cyclic_step producer").into())}
}
