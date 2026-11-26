//! 🧮️ Exercises real original cache publications and retirements against a neutral dependency graph.
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, StepContext};
use semio_framework_trace_memory_testing::observe_backing;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
use semio_os_inference::*;
use std::collections::BTreeMap;

#[derive(serde::Deserialize)]
struct Node { key: String, weight: i64, parents: Vec<String> }

struct Snapshot { nodes: Vec<Node>, links: Vec<(String, String)> }
struct Field;

impl InferredField<Snapshot> for Field {
    type Key = String;
    type Value = i64;
    type Dependency = i64;
    const FIELD_ID: &'static str = "test.original-inference-cache";
    const SCHEMA_VERSION: u32 = 1;
    fn reads() -> &'static [&'static str] { &["nodes"] }
    fn plan(source: &Snapshot) -> Vec<InferenceStep<String>> {
        source.nodes.iter().map(|node| InferenceStep { key: node.key.clone(), parents: node.parents.clone() }).collect()
    }
    fn dep_input(source: &Snapshot, key: &String, _: &[String]) -> i64 {
        source.nodes.iter().find(|node| &node.key == key).unwrap().weight
    }
    fn compute(source: &Snapshot, key: &String, parents: &[i64]) -> i64 {
        Self::dep_input(source, key, &[]) + parents.iter().sum::<i64>()
    }
}

fn funded(cx: &mut StepContext<'_>, copy: usize) -> Result<bool, InferenceWorkFailure> {
    let grant = cx.retained_grant();
    if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(false); }
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })?;
    cx.consume_fuel(1);
    Ok(true)
}

impl OriginalInferredField<Snapshot> for Field {
    type Source = Snapshot;
    fn validate_source(snapshot: &Snapshot, source: &Self::Source, cx: &StepContext<'_>) -> Result<(), InferenceWorkFailure> {
        if !std::ptr::eq(snapshot, source) { return Err(ValueError::invariant("fixture inference source changed").into()); }
        Ok(cx.original_source_parent()?.validate_borrowed_context(cx)?)
    }
    fn plan_source_extent(_: &Snapshot, source: &Self::Source) -> Option<(usize, usize)> { Some((source.nodes.len(), source.links.len())) }
    fn plan_source_node<'a>(_: &'a Snapshot, source: &'a Self::Source, ordinal: usize) -> Option<&'a str> {
        source.nodes.get(ordinal).map(|node| node.key.as_str())
    }
    fn plan_source_key<'a>(_: &'a Snapshot, source: &'a Self::Source, ordinal: usize) -> Option<&'a String> {
        source.nodes.get(ordinal).map(|node| &node.key)
    }
    fn plan_source_link<'a>(_: &'a Snapshot, source: &'a Self::Source, ordinal: usize) -> Option<(&'a str, &'a str)> {
        source.links.get(ordinal).map(|(from, to)| (from.as_str(), to.as_str()))
    }
    fn original_dependency_step(_: &Snapshot, source: &Self::Source, ordinal: usize, _: &mut Option<Box<dyn InferencePending>>, cx: &mut StepContext<'_>) -> Result<OriginalDependencyStep, InferenceWorkFailure> {
        if !funded(cx, 48)? { return Ok(OriginalDependencyStep::Pending); }
        let mut hash = OriginalDependencyHasher::new(Self::FIELD_ID, Self::SCHEMA_VERSION);
        hash.append(&(ordinal as u64).to_le_bytes());
        hash.append(&source.nodes[ordinal].weight.to_le_bytes());
        Ok(OriginalDependencyStep::Ready(hash.finish()))
    }
    fn original_cache_weight(_: &i64) -> Option<usize> { Some(size_of::<i64>()) }
    fn clone_original_parent(value: &i64, output: &mut Option<i64>, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !funded(cx, size_of::<i64>())? { return Ok(false); }
        *output = Some(*value);
        Ok(true)
    }
    fn compute_step(snapshot: &Snapshot, _: &Self::Source, key: &String, parents: &[i64], _: &mut Option<Box<dyn InferencePending>>, cx: &mut StepContext<'_>) -> Result<ComputeStep<i64>, InferenceWorkFailure> {
        if !funded(cx, size_of::<i64>())? { return Ok(ComputeStep::Working { progress: 0.0, fuel_used: 0 }); }
        Ok(ComputeStep::Done { value: Self::compute(snapshot, key, parents), fuel_used: 1 })
    }
    fn compute_cyclic_step(_: &Snapshot, _: &Self::Source, _: &String, _: &mut Option<Box<dyn InferencePending>>, _: &mut StepContext<'_>) -> Result<ComputeStep<i64>, InferenceWorkFailure> {
        Err(ValueError::invariant("fixture is acyclic").into())
    }
}

fn run(caller: &mut OriginalJobFixtureCaller, grant: RetainedCloneGrant, snapshot: &Snapshot, cache: &mut OriginalInferenceCache<i64>, fingerprints: &serde_json::Value) -> (BTreeMap<String, i64>, usize, usize) {
    let mut cursor = InferenceCursor::new();
    let mut values = InferenceValues::new();
    let (mut computed, mut hits) = (0, 0);
    let mut projected = None;
    for _ in 0..2048 {
        let (report, born, freed) = observe_backing(|| caller.turn(grant, |cx| Ok(infer_field_step::<_, Field>(snapshot, snapshot, Some(cache), &mut cursor, &mut values, cx))).unwrap().unwrap());
        assert_eq!((born, freed), (caller.receipt().retained_capacity_bytes, caller.receipt().released_bytes));
        assert!(report.fuel_used > 0 || report.done);
        computed += report.computed;
        hits += report.hits;
        if report.done {
            projected = Some(values.iter().map(|(ordinal, value)| {
                let id = &snapshot.nodes[ordinal].key;
                let bytes = values.fingerprint(ordinal).unwrap().0;
                let digest = bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
                assert_eq!(digest, fingerprints[id].as_str().unwrap());
                (id.clone(), *value)
            }).collect());
            break;
        }
    }
    let projected = projected.expect("original inference failed to finish");
    for _ in 0..2048 {
        let (done, born, freed) = observe_backing(|| caller.turn(grant, |cx| {
            if !values.close_step(cx).unwrap() { return Ok(false); }
            Ok(cursor.close_original_plan_step(cx).unwrap())
        }).unwrap());
        assert_eq!((born, freed), (caller.receipt().retained_capacity_bytes, caller.receipt().released_bytes));
        if done { return (projected, computed, hits); }
    }
    panic!("original fixture values and plan failed to close");
}

fn clear(caller: &mut OriginalJobFixtureCaller, grant: RetainedCloneGrant, cache: &mut OriginalInferenceCache<i64>) {
    for _ in 0..100 {
        let (done, born, freed) = observe_backing(|| caller.turn(grant, |cx| Ok(cache.clear_step(cx).unwrap())).unwrap());
        assert_eq!((born, freed), (caller.receipt().retained_capacity_bytes, caller.receipt().released_bytes));
        if done {
            assert!(cache.terminal_is_empty());
            return;
        }
    }
    panic!("original cache failed to clear");
}

#[test]
fn cache_reuses_only_unchanged_dependencies_under_one_item_turns() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪜️stepped-driver/🔣️.json")).unwrap();
    let nodes: Vec<Node> = serde_json::from_value(fixture["nodes"].clone()).unwrap();
    let links = nodes.iter().flat_map(|node| node.parents.iter().map(|parent| (parent.clone(), node.key.clone()))).collect();
    let mut snapshot = Snapshot { nodes, links };
    let recipe = &fixture["cache"];
    let mut cache = OriginalInferenceCache::new(recipe["budgetBytes"].as_u64().unwrap() as usize);
    let stock = RetainedCloneGrant { maximum_items: 32768, maximum_copy_bytes: 1048576, maximum_capacity_bytes: 16777216, maximum_release_bytes: 16777216, maximum_depth: 64 };
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(971), Generation(19), stock, || Some(1), 1000000, 65536);
    for (index, expected) in recipe["runs"].as_array().unwrap().iter().enumerate() {
        if index == 2 {
            let key = recipe["edit"]["key"].as_str().unwrap();
            let node = snapshot.nodes.iter_mut().find(|node| node.key == key).unwrap();
            node.weight = recipe["edit"]["weight"].as_i64().unwrap();
        }
        let (values, computed, hits) = run(&mut caller, grant, &snapshot, &mut cache, &recipe[if index < 2 { "initialFingerprints" } else { "editedFingerprints" }]);
        assert_eq!(serde_json::to_value(values).unwrap(), if index < 2 { &fixture["expected"] } else { &recipe["edited"] }.clone());
        assert_eq!(computed as u64, expected["computed"].as_u64().unwrap());
        assert_eq!(hits as u64, expected["hits"].as_u64().unwrap());
    }
    let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
    let used = cache.used_bytes();
    assert!(!caller.turn(denied, |cx| Ok(cache.clear_step(cx).unwrap())).unwrap());
    assert_eq!(cache.used_bytes(), used);
    assert_eq!(caller.receipt(), Default::default());
    clear(&mut caller, grant, &mut cache);
    let budget = recipe["boundedBudgetBytes"].as_u64().unwrap() as usize;
    let mut small_cache = OriginalInferenceCache::new(budget);
    let (values, computed, hits) = run(&mut caller, grant, &snapshot, &mut small_cache, &recipe["editedFingerprints"]);
    assert_eq!(serde_json::to_value(values).unwrap(), recipe["edited"]);
    assert_eq!((computed, hits), (4, 0));
    assert!(small_cache.used_bytes() <= budget);
    assert!(small_cache.stats().evictions > 0);
    clear(&mut caller, grant, &mut small_cache);
    caller.retire(grant);
}
