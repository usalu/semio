use super::*;
use semio_framework_value::retained_clone::ordered_map::{BoundedOrdGrant, RetainedOrderedMap, RetainedOrderedMapInsertCursor, RetainedOrderedMapInsertGrant, RetainedOrderedMapInsertStep};
use semio_framework_value::retained_clone::*;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneStep};
use semio_framework_value::retirement::{OwnedValueRetirementFactory, RetireOwned};
use semio_framework_value_derive::{RetainedClone, RetireOwned};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, RetainedClone, RetireOwned)]
#[serde(rename_all = "camelCase")]
struct NeutralRow {
    id: u64,
    text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, RetainedClone, RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum NeutralChoice {
    Unit,
    Text { text: String },
    Nested { rows: Vec<NeutralRow> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, RetainedClone, RetireOwned)]
#[serde(rename_all = "camelCase")]
struct NeutralSnapshot {
    title: String,
    payload: Vec<u8>,
    optional: Option<String>,
    choice: NeutralChoice,
    choices: Vec<NeutralChoice>,
    fixed_array: [u32; 4],
    pair: (String, u64),
    triple: (String, u32, String),
    labels: RetainedOrderedMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, RetainedClone, RetireOwned)]
struct UnitRecord;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, RetainedClone, RetireOwned)]
struct RecursiveRecord {
    text: String,
    next: Option<Box<RecursiveRecord>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, RetainedClone, RetireOwned)]
struct GenericRecord<T> {
    value: T,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, RetainedClone, RetireOwned)]
enum GenericChoice<T> {
    Unit,
    Tuple(T, String),
    Named { value: T },
}

#[derive(RetainedClone, RetireOwned)]
enum EmptyRecord {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, RetainedClone, RetireOwned)]
struct TupleRecord(String, u64);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    source: FixtureSource,
    payload_byte_length: usize,
    payload_modulo: usize,
    recursive_depth: usize,
    grant: FixtureGrant,
    insufficient_capacity_bytes: usize,
    immutable_lease: ImmutableLease,
    cancellation_stops: Vec<usize>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImmutableLease {
    captured: String,
    external_after_capture: String,
    expected: String,
}

#[derive(Deserialize)]
struct FixtureSource {
    title: String,
    optional: Option<String>,
    choice: NeutralChoice,
    choices: Vec<NeutralChoice>,
    #[serde(rename = "fixedArray")]
    fixed_array: [u32; 4],
    pair: (String, u64),
    triple: (String, u32, String),
    labels: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureGrant {
    maximum_items: usize,
    maximum_copy_bytes: usize,
    maximum_capacity_bytes: usize,
    maximum_depth: usize,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🧫️fixtures/📦️nested/🔣️.json")).expect("retained clone fixture")
}

fn source(fixture: &Fixture) -> NeutralSnapshot {
    NeutralSnapshot {
        title: fixture.source.title.clone(),
        payload: (0..fixture.payload_byte_length).map(|index| (index % fixture.payload_modulo) as u8).collect(),
        optional: fixture.source.optional.clone(),
        choice: fixture.source.choice.clone(),
        choices: fixture.source.choices.clone(),
        fixed_array: fixture.source.fixed_array,
        pair: fixture.source.pair.clone(),
        triple: fixture.source.triple.clone(),
        labels: fixture_labels(fixture),
    }
}

fn fixture_labels(fixture: &Fixture) -> RetainedOrderedMap<String, String> {
    let turn = &fixture.grant;
    let grant = RetainedOrderedMapInsertGrant {
        comparison: BoundedOrdGrant { maximum_items: turn.maximum_items, maximum_bytes: turn.maximum_copy_bytes },
        maximum_moved_items: turn.maximum_items,
        maximum_moved_bytes: turn.maximum_copy_bytes,
        maximum_capacity_bytes: turn.maximum_capacity_bytes,
    };
    let mut labels = RetainedOrderedMap::default();
    for (key, value) in &fixture.source.labels {
        let mut cursor = RetainedOrderedMapInsertCursor::new(labels, key.clone(), value.clone());
        for _ in 0..10000 {
            let step = cursor.advance(grant).expect("controlled fixture labels insertion");
            match step {
                RetainedOrderedMapInsertStep::Progress(progress) => assert!(progress.fits(grant)),
                RetainedOrderedMapInsertStep::Complete { progress, .. } => {
                    assert!(progress.fits(grant));
                    break;
                }
            }
        }
        labels = cursor.take().expect("controlled fixture labels completed");
        assert!(cursor.begin_close());
        for _ in 0..10000 {
            if cursor.close_step(turn.maximum_items, turn.maximum_copy_bytes).expect("fixture labels cursor closes") == SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(cursor.terminal_is_empty());
    }
    labels
}

fn grant(fixture: &Fixture) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: fixture.grant.maximum_items, maximum_copy_bytes: fixture.grant.maximum_copy_bytes, maximum_capacity_bytes: fixture.grant.maximum_capacity_bytes, maximum_depth: fixture.grant.maximum_depth }
}

fn assert_progress(progress: RetainedCloneProgress, grant: RetainedCloneGrant) {
    assert!(progress.fits(grant));
}

fn close_cursor<T: RetainedClone>(cursor: &mut T::Cursor) {
    assert!(cursor.begin_close());
    assert!(!cursor.begin_close());
    let mut turns = 0usize;
    while !cursor.terminal_is_empty() {
        let step = cursor.close_step(64, 4_096).expect("retained clone close");
        if step == SnapshotRetirementStep::Complete {
            assert!(cursor.terminal_is_empty());
        }
        turns += 1;
        assert!(turns < 100_000, "retained clone close did not terminate for {}", std::any::type_name::<T>());
    }
}

fn retained_copy<T>(source: &T, grant: RetainedCloneGrant) -> T
where
    T: RetainedClone + Clone,
{
    let source = retained_source(source.clone());
    let mut cursor = T::retained_clone_cursor();
    let mut turns = 0usize;
    let output = loop {
        turns += 1;
        let step = cursor.advance(source.borrow(), grant).expect("retained copy");
        assert_progress(step.progress(), grant);
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break cursor.take().expect("retained copy output");
        }
        assert!(turns < 100_000);
    };
    assert!(cursor.advance(source.borrow(), grant).is_err());
    close_cursor::<T>(&mut cursor);
    output
}

fn recursive_record(depth: usize) -> RecursiveRecord {
    let mut value = RecursiveRecord { text: "leaf".into(), next: None };
    for index in 0..depth {
        value = RecursiveRecord { text: format!("node-{index}-β"), next: Some(Box::new(value)) };
    }
    value
}

#[test]
fn derived_clone_matches_clone_and_serde_oracles_with_distinct_capacity_credits() {
    let fixture = fixture();
    let source = source(&fixture);
    let retained_source = retained_source(source.clone());
    let clone_oracle = source.clone();
    let serde_oracle = serde_json::to_value(&source).expect("serde oracle");
    let grant = grant(&fixture);
    let mut cursor = NeutralSnapshot::retained_clone_cursor();
    let mut turns = 0usize;
    let output = loop {
        turns += 1;
        let step = cursor.advance(retained_source.borrow(), grant).expect("retained clone step");
        assert_progress(step.progress(), grant);
        if let RetainedCloneStep::Complete(_) = step {
            break cursor.take().expect("completed retained clone");
        }
        assert!(turns < 10_000);
    };
    assert!(turns > fixture.payload_byte_length / fixture.grant.maximum_copy_bytes);
    assert_eq!(output, clone_oracle);
    assert_eq!(serde_json::to_value(&output).expect("retained serde output"), serde_oracle);
    close_cursor::<NeutralSnapshot>(&mut cursor);
}

#[test]
fn capacity_refusal_precedes_large_vector_allocation() {
    let fixture = fixture();
    let source = source(&fixture);
    let retained_owner = retained_source(source);
    let mut cursor = NeutralSnapshot::retained_clone_cursor();
    let admitted = grant(&fixture);
    let refused = RetainedCloneGrant { maximum_capacity_bytes: fixture.insufficient_capacity_bytes, ..admitted };
    let mut reached_refusal = false;
    for _ in 0..32 {
        let step = cursor.advance(retained_owner.borrow(), refused).expect("capacity probe");
        let progress = step.progress();
        assert_progress(progress, refused);
        if progress == RetainedCloneProgress::default() {
            reached_refusal = true;
            break;
        }
    }
    assert!(reached_refusal);
    close_cursor::<NeutralSnapshot>(&mut cursor);
}

#[test]
fn cancellation_retires_every_partial_owner_through_bounded_steps() {
    let fixture = fixture();
    let source = source(&fixture);
    let retained_owner = retained_source(source);
    let grant = grant(&fixture);
    for stop in &fixture.cancellation_stops {
        let mut cursor = NeutralSnapshot::retained_clone_cursor();
        for _ in 0..*stop {
            if matches!(cursor.advance(retained_owner.borrow(), grant).expect("retained clone before cancel"), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(cursor.begin_close());
        let mut close_turns = 0usize;
        while !cursor.terminal_is_empty() {
            let step = cursor.close_step(64, 4_096).expect("retained clone close");
            if step == SnapshotRetirementStep::Complete {
                assert!(cursor.terminal_is_empty());
            }
            if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                assert!(released_items <= 64);
                assert!(released_bytes <= 4_096);
            }
            close_turns += 1;
            assert!(close_turns < 100_000);
        }
    }
}

#[test]
fn utf8_copy_is_boundary_paged_and_source_lease_captures_one_value() {
    let fixture = fixture();
    let source = "βeta 🧬 Deutsch 🇯🇵".repeat(512);
    let grant = RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 7, maximum_capacity_bytes: source.len(), maximum_depth: 512 };
    assert_eq!(retained_copy(&source, grant), source);

    let mut external = fixture.immutable_lease.captured.clone();
    let retained = retained_source(external.clone());
    let mut cursor = String::retained_clone_cursor();
    let reserve = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 2, maximum_capacity_bytes: external.len(), maximum_depth: 512 };
    cursor.advance(retained.borrow(), reserve).expect("string reserve");
    cursor.advance(retained.borrow(), reserve).expect("first immutable page");
    external = fixture.immutable_lease.external_after_capture.clone();
    let copied = loop {
        if matches!(cursor.advance(retained.borrow(), reserve).expect("captured string copy"), RetainedCloneStep::Complete(_)) {
            break cursor.take().expect("captured string output");
        }
    };
    assert_eq!(copied, fixture.immutable_lease.expected);
    assert_ne!(copied, external);
    close_cursor::<String>(&mut cursor);

    let siblings = retained_source(("same".to_string(), "same".to_string()));
    let mut sibling_cursor = String::retained_clone_cursor();
    sibling_cursor.advance(siblings.borrow().project(1, |value| &value.0), reserve).expect("first sibling projection");
    let error = sibling_cursor.advance(siblings.borrow().project(2, |value| &value.1), reserve).expect_err("crossed sibling projection must fail");
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::InvariantViolated);
    assert!(error.message.contains("projected path changed"));
    close_cursor::<String>(&mut sibling_cursor);
}

#[test]
fn production_snapshot_read_lease_survives_multiturn_copy_and_bounded_cancellation() {
    let captured = fixture().immutable_lease.captured;
    let registry = Arc::new(super::SnapshotReadLeaseRegistry::new());
    let owner = Arc::new(captured.clone());
    let lease = registry.try_issue(Arc::clone(&owner)).expect("production snapshot read lease");
    let retained_source = RetainedCloneSource::from_authority(Arc::clone(&owner), super::SnapshotRead::new(Arc::clone(&owner), lease));
    let copy_grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: captured.len(), maximum_depth: 64 };
    let mut cursor = String::retained_clone_cursor();
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        if matches!(cursor.advance(retained_source.borrow(), copy_grant).expect("production snapshot read copy"), RetainedCloneStep::Complete(_)) {
            break cursor.take().expect("production snapshot read output");
        }
    };
    assert!(turns > 2);
    assert_eq!(copied, captured);
    close_cursor::<String>(&mut cursor);
    drop(retained_source);
    assert!(registry.terminal_is_empty());
    drop(owner);

    let fixture = fixture();
    let owner = Arc::new(source(&fixture));
    let lease = registry.try_issue(Arc::clone(&owner)).expect("production cancellation snapshot read lease");
    let retained_source = RetainedCloneSource::from_authority(Arc::clone(&owner), super::SnapshotRead::new(owner, lease));
    let grant = grant(&fixture);
    let mut cursor = NeutralSnapshot::retained_clone_cursor();
    cursor.advance(retained_source.borrow(), grant).expect("production cancellation reserve");
    cursor.advance(retained_source.borrow(), grant).expect("production cancellation prefix");
    drop(retained_source);
    assert!(!registry.terminal_is_empty(), "active cursor binding retains the exact production snapshot read");
    close_cursor::<NeutralSnapshot>(&mut cursor);
    assert!(registry.has_returned(), "the last large owner returns to the bounded registry pump");
    let factory: Arc<dyn super::ArtifactOwnedValueRetirementFactory<NeutralSnapshot>> = Arc::new(OwnedValueRetirementFactory::<NeutralSnapshot>::default());
    let mut active = None;
    let mut retirement_turns = 0usize;
    loop {
        retirement_turns += 1;
        let step = super::advance_returned_snapshot_read(&registry, &mut active, &factory, 64, fixture.grant.maximum_capacity_bytes).expect("returned large snapshot retirement");
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 64);
            assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
        }
        if step == SnapshotRetirementStep::Complete {
            break;
        }
        assert!(retirement_turns < 100_000, "returned large snapshot retirement remains bounded");
    }
    assert!(retirement_turns > 1, "large nested ownership is drained by the registry over multiple turns");
    assert!(registry.terminal_is_empty());
}

#[test]
fn retained_vector_and_completed_optional_preserve_their_captured_owner() {
    let mut values = vec![1u64, 2, 3, 4];
    let retained_values = retained_source(values.clone());
    let grant = RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 64, maximum_capacity_bytes: 1_024, maximum_depth: 512 };
    let mut vector = Vec::<u64>::retained_clone_cursor();
    vector.advance(retained_values.borrow(), grant).expect("vector reserve");
    values.pop();
    let copied = loop {
        if matches!(vector.advance(retained_values.borrow(), grant).expect("captured vector"), RetainedCloneStep::Complete(_)) {
            break vector.take().expect("captured vector output");
        }
    };
    assert_eq!(copied, vec![1, 2, 3, 4]);
    close_cursor::<Vec<u64>>(&mut vector);

    let source = Some("owner".repeat(1_024));
    let retained_source = retained_source(source.clone());
    let mut optional = Option::<String>::retained_clone_cursor();
    let optional_grant = RetainedCloneGrant { maximum_capacity_bytes: source.as_ref().expect("optional source").len(), ..grant };
    loop {
        if matches!(optional.advance(retained_source.borrow(), optional_grant).expect("optional copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(matches!(optional.advance(retained_source.borrow(), optional_grant).expect("completed optional"), RetainedCloneStep::Complete(_)));
    assert_eq!(optional.take(), Some(source.clone()));
    assert!(optional.advance(retained_source.borrow(), grant).is_err());
    close_cursor::<Option<String>>(&mut optional);
}

#[test]
fn unit_generic_enum_and_recursive_records_remain_grant_bounded() {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 16, maximum_capacity_bytes: 65_536, maximum_depth: 512 };
    let units = vec![UnitRecord; 256];
    let retained_units = retained_source(units.clone());
    let mut cursor = Vec::<UnitRecord>::retained_clone_cursor();
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        let step = cursor.advance(retained_units.borrow(), grant).expect("unit vector clone");
        assert!(step.progress().copied_items <= 1);
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break cursor.take().expect("unit vector output");
        }
        assert!(turns < 10_000);
    };
    assert_eq!(copied, units);
    assert!(turns > units.len());
    close_cursor::<Vec<UnitRecord>>(&mut cursor);

    let generic = GenericRecord { value: "generic".repeat(128) };
    assert_eq!(retained_copy(&generic, grant), generic);
    for choice in [GenericChoice::Unit, GenericChoice::Tuple("tuple".to_string(), "text".to_string()), GenericChoice::Named { value: "named".to_string() }] {
        assert_eq!(retained_copy(&choice, grant), choice);
    }

    let recursive = recursive_record(fixture().recursive_depth);
    let copied = retained_copy(&recursive, grant);
    assert_eq!(copied, recursive);
    assert_eq!(serde_json::to_value(copied).expect("recursive retained serde"), serde_json::to_value(recursive).expect("recursive clone serde"));

    fn assert_retained<T: RetainedClone>() {}
    assert_retained::<EmptyRecord>();
}

#[test]
fn recursive_depth_envelope_accepts_boundary_and_rejects_the_next_box() {
    let maximum_depth = 64usize;
    let grant = RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 64, maximum_capacity_bytes: 65_536, maximum_depth };
    let accepted = recursive_record(maximum_depth);
    assert_eq!(retained_copy(&accepted, grant), accepted);

    let refused = recursive_record(maximum_depth + 1);
    let retained_refused = retained_source(refused);
    let mut cursor = RecursiveRecord::retained_clone_cursor();
    let mut turns = 0usize;
    let error = loop {
        turns += 1;
        match cursor.advance(retained_refused.borrow(), grant) {
            Ok(RetainedCloneStep::Progress(progress)) => assert_progress(progress, grant),
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-depth recursive owner completed"),
            Err(error) => break error,
        }
        assert!(turns < 100_000);
    };
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::DepthLimit);
    assert!(error.message.contains("depth limit"));
    close_cursor::<RecursiveRecord>(&mut cursor);
}

#[test]
fn recursive_partial_copy_cancels_within_the_declared_depth_envelope() {
    let maximum_depth = 64usize;
    let source = recursive_record(maximum_depth);
    let retained_owner = retained_source(source);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 2, maximum_capacity_bytes: 65_536, maximum_depth };
    let mut cursor = RecursiveRecord::retained_clone_cursor();
    for _ in 0..256 {
        assert!(matches!(cursor.advance(retained_owner.borrow(), grant).expect("recursive prefix copy"), RetainedCloneStep::Progress(_)));
    }
    assert!(cursor.begin_close());
    let mut turns = 0usize;
    while !cursor.terminal_is_empty() {
        let step = cursor.close_step(1, 2).expect("recursive prefix close");
        if step == SnapshotRetirementStep::Complete {
            assert!(cursor.terminal_is_empty());
        }
        turns += 1;
        assert!(turns < 100_000);
    }
    assert!(turns > maximum_depth);
}

#[test]
fn fixed_arrays_standard_tuples_and_tuple_records_preserve_native_shape() {
    let array = std::array::from_fn::<_, 128, _>(|index| index as u32);
    let refused = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: std::mem::size_of_val(&array) - 1, maximum_capacity_bytes: 0, maximum_depth: 512 };
    let mut cursor = <[u32; 128]>::retained_clone_cursor();
    let retained_array = retained_source(array);
    assert_eq!(cursor.advance(retained_array.borrow(), refused).expect("array refusal").progress(), RetainedCloneProgress::default());
    close_cursor::<[u32; 128]>(&mut cursor);
    let admitted = RetainedCloneGrant { maximum_copy_bytes: std::mem::size_of_val(&array), ..refused };
    assert_eq!(retained_copy(&array, admitted), array);

    let pair = ("βeta".repeat(128), 42u64);
    let triple = ("Deutsch".to_string(), 7u32, "日本語".to_string());
    let record = TupleRecord("tuple record".repeat(64), 9);
    let grant = RetainedCloneGrant { maximum_items: 2, maximum_copy_bytes: 32, maximum_capacity_bytes: 65_536, maximum_depth: 512 };
    assert_eq!(retained_copy(&pair, grant), pair);
    assert_eq!(retained_copy(&triple, grant), triple);
    assert_eq!(retained_copy(&record, grant), record);
}

#[test]
fn zero_grants_never_construct_or_complete_an_owner() {
    let zero = RetainedCloneGrant::default();
    let mut scalar = u64::retained_clone_cursor();
    let scalar_source = retained_source(7u64);
    assert_eq!(scalar.advance(scalar_source.borrow(), zero).expect("zero scalar").progress(), RetainedCloneProgress::default());
    assert!(scalar.take().is_none());
    close_cursor::<u64>(&mut scalar);

    let text = "β".repeat(64);
    let retained_text = retained_source(text);
    let mut string = String::retained_clone_cursor();
    assert_eq!(string.advance(retained_text.borrow(), zero).expect("zero string").progress(), RetainedCloneProgress::default());
    assert!(string.take().is_none());
    close_cursor::<String>(&mut string);

    let values = vec![UnitRecord; 16];
    let retained_values = retained_source(values);
    let mut vector = Vec::<UnitRecord>::retained_clone_cursor();
    assert_eq!(vector.advance(retained_values.borrow(), zero).expect("zero vector").progress(), RetainedCloneProgress::default());
    assert!(vector.take().is_none());
    close_cursor::<Vec<UnitRecord>>(&mut vector);

    let source = Some("owner".to_string());
    let retained_owner = retained_source(source);
    let mut optional = Option::<String>::retained_clone_cursor();
    assert_eq!(optional.advance(retained_owner.borrow(), zero).expect("zero optional").progress(), RetainedCloneProgress::default());
    assert!(optional.take().is_none());
    close_cursor::<Option<String>>(&mut optional);

    let unit = UnitRecord;
    let retained_unit = retained_source(unit);
    let mut derived = UnitRecord::retained_clone_cursor();
    assert_eq!(derived.advance(retained_unit.borrow(), zero).expect("zero derived").progress(), RetainedCloneProgress::default());
    assert!(derived.take().is_none());
    close_cursor::<UnitRecord>(&mut derived);
}

fn retained_source<T: Send + Sync + 'static>(owner: T) -> RetainedCloneSource<T> {
    RetainedCloneSource::from_authority(Arc::new(owner), ())
}
