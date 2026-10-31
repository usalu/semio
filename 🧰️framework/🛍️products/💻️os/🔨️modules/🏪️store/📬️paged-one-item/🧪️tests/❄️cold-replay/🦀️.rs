//! ❄️ N cold edits saved in order and reopened from the base replay through `ColdMutationApplyFactory` to the same snapshot as an independent `serde_json` fold, and every inverse restores its pre-state.
use super::*;
use semio_framework_os_store_replay_operation::{ArtifactMutationSource, ArtifactReplayPreparation, ArtifactReplayPreparationContext, ArtifactReplayPreparationFactory, ArtifactReplayPreparationMode, ArtifactReplayPreparationRequest, ArtifactReplayPreparationStep};
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
    let snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<PlainDocument>> = Arc::new(OwnedValueRetirementFactory::<PlainDocument>::default());
    let mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<DocumentMutation>> = Arc::new(OwnedValueRetirementFactory::<DocumentMutation>::default());
    let mut original = Some(mutation);
    let (mut source, _) = ArtifactMutationSource::admit(&mut original, &mutations, retained(ArtifactMutationSource::<DocumentMutation>::constructor_demand())).unwrap().unwrap();
    let mut input = None;
    for _ in 0..64 {
        if source.ready() { source.take(&mut input, retained(source.demands(4096).unwrap())).unwrap(); break; }
        source.advance(retained(source.demands(4096).unwrap())).unwrap();
    }
    let registry = SnapshotReadRegistryHandle::new();
    let lease = registry.try_issue(Arc::clone(&base)).unwrap_or_else(|_| panic!("replay registry admission"));
    let context = ArtifactReplayPreparationContext { mode: ArtifactReplayPreparationMode::Prefix, operation_index: index, generation: 1, base_revision: [index as u8; 32] };
    let request = ArtifactReplayPreparationRequest { base: SnapshotRead::new(Arc::clone(&base), lease), input: input.take().unwrap(), context, snapshots: Arc::clone(&snapshots), mutations: Arc::clone(&mutations) };
    let mut receiver = Receiver::new(retained(ArtifactReplayPreparationFactory::preparation_birth_demand(&factory)));
    let birth = retained(ArtifactReplayPreparationFactory::preparation_birth_demand(&factory));
    let mut cursor = receiver.run(birth, |control| { control.admit_turn_capacity(birth.maximum_capacity_bytes)?; control.charge(birth.maximum_capacity_bytes)?; ArtifactReplayPreparationFactory::begin(&factory, request, birth).map(|(cursor, _)| cursor).map_err(|(error, _)| error) }).unwrap();
    let mut prepared = None;
    let mut copied = 0;
    for _ in 0..10_000 {
        let grant = retained(cursor.advance_demands(4096).unwrap());
        let step = receiver.run(grant, |control| cursor.advance(4096, one_item(grant), control)).unwrap();
        copied += step.progress().copied_bytes;
        if let ArtifactReplayPreparationStep::Prepared(_) = step { prepared = cursor.take_prepared(); break; }
    }
    let prepared = prepared.expect("cold replay reaches its prepared owners");
    assert!(prepared.apply_refusal.is_none() && prepared.input_refusal.is_none() && prepared.messages.is_empty());
    let next = prepared.next.expect("cold replay produces its post snapshot");
    let inverse: Vec<DocumentMutation> = prepared.inverse.as_ref().unwrap().iter().cloned().collect();
    drain_fixture(prepared.inverse.unwrap());
    cursor.cancel();
    cursor.begin_close();
    for _ in 0..10_000 {
        if cursor.terminal_is_empty() { break; }
        let grant = retained(cursor.close_demands(4096).unwrap());
        receiver.run(grant, |control| control.with_retirement(grant, |receiving| cursor.close_step(grant, receiving))).unwrap();
    }
    assert!(cursor.terminal_is_empty());
    drop(cursor);
    let mut active: Option<Box<dyn ErasedSnapshotRetirement>> = None;
    for _ in 0..10_000 {
        if !registry.has_returned() && active.is_none() { break; }
        let demand = if let Some(active) = active.as_ref() { semio_framework_value_snapshot_read::artifact_retirement_box_demands(active, 4096).unwrap() } else { registry.returned_admission_demands::<PlainDocument>(|_| RetirementDemand { capacity_bytes: semio_framework_value::factory::FactorySharedRetirement::<PlainDocument>::constructor_capacity_bytes(), depth: 1, ..Default::default() }).unwrap() };
        semio_framework_value_snapshot_read::advance_returned_snapshot_read(&registry, &mut active, &snapshots, retained(demand)).unwrap();
    }
    assert!(!registry.has_returned() && active.is_none());
    receiver.finish();
    source.begin_close();
    for _ in 0..10_000 {
        if source.terminal_is_empty() { break; }
        let grant = retained(source.close_demand(4096).unwrap());
        source.close_step(grant).unwrap();
    }
    assert!(source.terminal_is_empty());
    drain_fixture(base);
    (next, inverse, copied)
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
