//! ❄️ N cold edits saved in order and reopened from the base replay through `ColdMutationApplyFactory` to the same snapshot as an independent `serde_json` fold, and every inverse restores its pre-state.
use super::*;
use semio_os_store_replay_operation::{ArtifactMutationSource, ArtifactReplayPreparation, ArtifactReplayPreparationContext, ArtifactReplayPreparationFactory, ArtifactReplayPreparationMode, ArtifactReplayPreparationRequest, ArtifactReplayPreparationStep};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, RetirementDemand};

fn retained(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

fn one_item(grant: RetainedCloneGrant) -> ArtifactStoreOneItemGrant {
    ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth }
}

fn replay_step(base: Arc<PlainDocument>, mutation: DocumentMutation, index: u32) -> (Arc<PlainDocument>, Vec<DocumentMutation>) {
    let (next, inverse, _) = replay_step_with(paged::ColdMutationApplyFactory::<PlainDocument, DocumentMutation>::default(), base, mutation, index);
    (next, inverse)
}

fn replay_step_with(factory: paged::ColdMutationApplyFactory<PlainDocument, DocumentMutation>, base: Arc<PlainDocument>, mutation: DocumentMutation, index: u32) -> (Arc<PlainDocument>, Vec<DocumentMutation>, usize) {
    let (next, inverse, messages, copied) = replay_outcome_with(factory, base, mutation, index);
    assert!(messages.is_empty());
    (next.expect("cold replay produces its post snapshot"), inverse, copied)
}

fn replay_outcome_with(factory: paged::ColdMutationApplyFactory<PlainDocument, DocumentMutation>, base: Arc<PlainDocument>, mutation: DocumentMutation, index: u32) -> (Option<Arc<PlainDocument>>, Vec<DocumentMutation>, Vec<protocol::MutationMessage>, usize) {
    let snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<PlainDocument>> = Arc::new(OwnedValueRetirementFactory::<PlainDocument>::default());
    let mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>> = Arc::new(OwnedValueRetirementFactory::<DocumentMutation>::default());
    let mut original = Some(mutation);
    let (mut source, _) = ArtifactMutationSource::admit(&mut original, &mutations, retained(ArtifactMutationSource::<DocumentMutation>::constructor_demand())).unwrap().unwrap();
    let mut input = None;
    for _ in 0..64 {
        if source.ready() { source.take(&mut input, retained(source.demands(4096).unwrap())).unwrap(); break; }
        {let grant=retained(source.demands(4096).unwrap());effect::with_effect(grant,effect::CPU,|e|source.advance(e)).0.result.unwrap();}
    }
    let registry = SnapshotReadRegistryHandle::new();
    let lease = registry.try_issue(Arc::clone(&base)).unwrap_or_else(|_| panic!("replay registry admission"));
    let context = ArtifactReplayPreparationContext { mode: ArtifactReplayPreparationMode::Prefix, operation_index: index, generation: 1, base_revision: [index as u8; 32] };
    let request = ArtifactReplayPreparationRequest { base: SnapshotRead::new(Arc::clone(&base), lease), input: input.take().unwrap(), context, snapshots: Arc::clone(&snapshots), mutations: Arc::clone(&mutations) };
    let mut receiver = Receiver::new(retained(ArtifactReplayPreparationFactory::preparation_birth_demand(&factory)));
    let birth = retained(ArtifactReplayPreparationFactory::preparation_birth_demand(&factory));
    let mut cursor = receiver.born(birth, |control| { control.admit_turn_capacity(birth.maximum_capacity_bytes)?; control.charge(birth.maximum_capacity_bytes)?; ArtifactReplayPreparationFactory::begin(&factory, request, birth).map_err(|(error, _)| error) }).unwrap();
    let mut prepared = None;
    let mut copied = 0;
    for _ in 0..10_000 {
        let grant = retained(cursor.advance_demands(4096, effect::CPU).unwrap());
        let step = receiver.run(grant, |control| cursor.advance(4096, one_item(grant), control)).unwrap();
        copied += step.progress().copied_bytes;
        if let ArtifactReplayPreparationStep::Prepared(_) = step { prepared = cursor.take_prepared(); break; }
    }
    let prepared = prepared.expect("cold replay reaches its prepared owners");
    assert!(prepared.apply_refusal.is_none() && prepared.input_refusal.is_none());
    let next = prepared.next;
    let inverse: Vec<DocumentMutation> = prepared.inverse.as_ref().map_or_else(|_| Vec::new(), |rows| rows.iter().cloned().collect());
    drain_fixture(prepared.inverse);
    let messages = prepared.messages;
    cursor.cancel();
    cursor.begin_close();
    for _ in 0..10_000 {
        if cursor.terminal_is_empty() { break; }
        let grant = retained(cursor.close_demands(4096).unwrap());
        receiver.retire(grant, |effect, receiving| cursor.close_step(effect, receiving)).unwrap();
    }
    assert!(cursor.terminal_is_empty());
    drop(cursor);
    let mut active: Option<Box<dyn ErasedSnapshotRetirement>> = None;
    for _ in 0..10_000 {
        if !registry.has_returned() && active.is_none() { break; }
        let demand = if let Some(active) = active.as_ref() { semio_framework_value_snapshot_read::artifact_retirement_box_demands(active, 4096).unwrap() } else { registry.returned_admission_demands::<PlainDocument>(|_| RetirementDemand { capacity_bytes: semio_framework_value::factory::FactorySharedRetirement::<PlainDocument>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).unwrap() };
        effect::settled(effect::with_effect(retained(demand), effect::CPU, |e| semio_framework_value_snapshot_read::advance_returned_snapshot_read(&registry, &mut active, &snapshots, e)).0).result.unwrap();
    }
    assert!(!registry.has_returned() && active.is_none());
    receiver.finish();
    source.begin_close();
    for _ in 0..10_000 {
        if source.terminal_is_empty() { break; }
        let grant = retained(source.close_demand(4096).unwrap());
        effect::settled(effect::with_effect(grant, effect::CPU, |effect| source.close_step(effect)).0).result.unwrap();
    }
    assert!(source.terminal_is_empty());
    drain_fixture(base);
    (next, inverse, messages, copied)
}

fn replay(base: &PlainDocument, edits: &[DocumentMutation]) -> PlainDocument {
    let mut current = Arc::new(base.clone());
    for (index, edit) in edits.iter().enumerate() { current = replay_step(current, edit.clone(), index as u32).0; }
    let post = (*current).clone();
    drain_fixture(current);
    post
}

fn pairs_of(document: &PlainDocument) -> Value { Value::Array(document.nodes.iter().map(|node| json!([node.id, node.text])).collect()) }

fn oracle(base: &Value, edit: &Value, nodes: &mut Vec<Value>) {
    let text = |key: &str| edit[key].as_str().unwrap().to_owned();
    let at = |key: &str| edit[key].as_u64().unwrap() as usize;
    let _ = base;
    match edit["kind"].as_str().unwrap() {
        "insert" => nodes.insert(at("index"), json!([text("id"), text("text")])),
        "remove" => { nodes.remove(at("index")); }
        "set-text" => nodes[at("index")][1] = Value::String(text("text")),
        "remove-range" => { nodes.drain(at("start")..at("start") + at("count")); }
        other => panic!("unknown oracle edit {other}"),
    }
}

#[test]
fn cold_replay_of_n_saved_edits_reopens_to_the_same_snapshot() {
    let law: Value = serde_json::from_str(include_str!("../🧫️fixtures/❄️cold-replay/🔣️.json")).unwrap();
    let base = plain_document_json(&law["base"]);
    let edits = rows(&law["edits"]);
    let mut folded: Vec<Value> = pairs_of(&base).as_array().unwrap().clone();
    for (count, edit) in law["edits"].as_array().unwrap().iter().enumerate() {
        oracle(&law["base"], edit, &mut folded);
        assert_eq!(Value::Array(folded.clone()), law["posts"][count], "independent oracle disagrees with the fixture after edit {count}");
        let first = replay(&base, &edits[..=count]);
        let reopened = replay(&base, &edits[..=count]);
        assert_eq!(pairs_of(&first), law["posts"][count], "cold replay of {} saved edits", count + 1);
        assert_eq!(first, reopened, "reopening the saved history replays to the same snapshot");
        assert_eq!(first.title, base.title);
    }
}

#[test]
fn cold_replay_inverse_rows_restore_every_pre_state() {
    let law: Value = serde_json::from_str(include_str!("../🧫️fixtures/❄️cold-replay/🔣️.json")).unwrap();
    let mut current = Arc::new(plain_document_json(&law["base"]));
    for (index, edit) in rows(&law["edits"]).into_iter().enumerate() {
        let before = (*current).clone();
        let (next, inverse) = replay_step(current, edit, index as u32);
        let restored = replay(&next, &inverse);
        assert_eq!(restored, before, "inverse rows of edit {index} restore its pre-state");
        current = next;
    }
    drain_fixture(current);
}

fn snapshot_priced_at_four_kib(_: &PlainDocument) -> usize { 4096 }

#[test]
fn cold_replay_meters_the_base_snapshot_clone_as_the_bounded_path_does() {
    let law: Value = serde_json::from_str(include_str!("../🧫️fixtures/❄️cold-replay/🔣️.json")).unwrap();
    let edit = rows(&law["edits"]).remove(0);
    let unbounded = replay_step_with(paged::ColdMutationApplyFactory::default(), Arc::new(plain_document_json(&law["base"])), edit.clone(), 0);
    let bounds = paged::ColdPreparationBounds { maximum_bytes: 1 << 20, mutation_bytes: |_| 64, snapshot_bytes: snapshot_priced_at_four_kib };
    let bounded = replay_step_with(paged::ColdMutationApplyFactory::bounded(bounds), Arc::new(plain_document_json(&law["base"])), edit, 0);
    assert_eq!(pairs_of(&unbounded.0), pairs_of(&bounded.0), "metering never changes the replayed snapshot");
    assert!(bounded.2 >= unbounded.2 + 4096 - std::mem::size_of::<PlainDocument>(), "the base clone is charged at its measured size: {} vs {}", bounded.2, unbounded.2);
    drain_fixture(unbounded.0);
    drain_fixture(bounded.0);
}

/// 🚦️ Metadata-free preview preserves native diagnostic admission before applying or inverting a candidate.
#[test]
fn cold_replay_private_semantics_preserve_diagnostics_and_refuse_before_apply() {
    let law: Value = serde_json::from_str(include_str!("../../🧬️mutation-apply/🧪️tests/🧫️fixtures/🚦️admission/🔣️.json")).unwrap();
    let original = plain_document_json(&fixture()["base"]);
    for index in [0, 7] {
        for case in law["cases"].as_array().unwrap() {
            let operation = DocumentMutation::Diagnostic { level: case["level"].as_str().unwrap().into(), code: case["code"].as_str().unwrap().into(), message: case["message"].as_str().unwrap().into() };
            APPLY_CALLS.set(0); INVERSE_CALLS.set(0); DIFF_CALLS.set(0); DIFF_RETIRES.set(0);
            let (next, inverse, messages, _) = replay_outcome_with(paged::ColdMutationApplyFactory::default(), Arc::new(original.clone()), operation, index);
            let applied = case["applied"].as_bool().unwrap();
            assert_eq!(next.is_some(), applied, "native refusal must not yield a candidate");
            assert_eq!(DIFF_CALLS.get(), 1);
            assert_eq!(DIFF_RETIRES.get(), 1, "original cold delta retires exactly once on admission and refusal");
            assert_eq!(APPLY_CALLS.get(), case["applyCalls"].as_u64().unwrap() as usize);
            assert_eq!(INVERSE_CALLS.get(), case["inverseCalls"].as_u64().unwrap() as usize);
            assert_eq!(messages.len(), 1, "cold preview must preserve the native diagnostic");
            let actual: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&messages[0])).unwrap();
            let mut expected = case.as_object().unwrap().clone();
            for key in ["applied", "post", "applyCalls", "inverseCalls"] { expected.remove(key); }
            expected.insert("opIndex".into(), json!(index));
            assert_eq!(actual, Value::Object(expected));
            if let Some(post) = next {
                let actual = serde_json::to_value(post.as_ref()).unwrap();
                let mut expected = serde_json::to_value(&original).unwrap();
                expected["nodes"][0]["text"] = Value::String("diagnostic applied".into());
                assert_eq!(actual, expected);
                assert_eq!(inverse, vec![DocumentMutation::SetText { index: 0, text: original.nodes[0].text.clone() }]);
                drain_fixture(post);
            } else { assert!(inverse.is_empty()); }
            drain_fixture(inverse);
            let mut ledger = protocol::MutationMessageLedgerRetirement::new(String::new(), messages);
            while !matches!(ledger.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: WIDE, maximum_capacity_bytes: WIDE, maximum_release_bytes: WIDE, maximum_depth: 4096 }).unwrap(), semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {}
        }
    }
}
