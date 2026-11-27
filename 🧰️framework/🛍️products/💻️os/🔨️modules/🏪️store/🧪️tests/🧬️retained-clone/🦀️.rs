use super::*;
use semio_framework_value::retained_clone::ordered_map::{BoundedOrdGrant, RetainedOrderedMap, RetainedOrderedMapInsertCursor, RetainedOrderedMapInsertGrant, RetainedOrderedMapInsertStep};
use semio_framework_value::retained_clone::*;
use semio_framework_value::RetirementDemand;
use semio_framework_value::{OriginalRetirementFailure, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, ValueError};
use semio_framework_value::retained_clone::{OriginalEffectReply, OriginalEffectReplyComplete};
use semio_framework_value_clone_source_testing::drive;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneStep};
use semio_framework_value::factory::OwnedValueRetirementFactory;
use semio_framework_value::retirement::RetireOwned;
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
    retirement_grant: FixtureGrant,
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
    maximum_release_bytes: usize,
    maximum_depth: usize,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🧪️tests/🧫️fixtures/📦️nested/🔣️.json")).expect("retained clone fixture")
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

const WORK: u64 = 1 << 20;

fn receipt_of(reply: &Result<RetainedCloneStep, OriginalRetirementFailure>) -> RetainedCloneProgress {
    match reply { Ok(step) => step.progress(), Err(error) => error.retained_progress() }
}

fn drive_insert(mut call: impl for<'r> FnMut(&mut RetainedAllocationEffect<'_>, OriginalEffectReply<'r, RetainedCloneStep, OriginalRetirementFailure>, bool) -> Result<OriginalEffectReplyComplete<'r, RetainedCloneStep, OriginalRetirementFailure>, ValueError>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep, OriginalRetirementFailure> {
    drive::warm();
    drive::drive_reply::<RetainedCloneStep, OriginalRetirementFailure>(grant, WORK, receipt_of, &mut call)
}

/// 🎞️ Turns of one clone cursor under a real original Work action; every turn proves its heap equals its receipt.
trait CursorTurns<T: RetainedClone>: RetainedCloneCursor<T> + Sized {
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, body: usize, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep, OriginalRetirementFailure> {
        drive_insert(|effect, reply, recover| if recover { self.recover_advance_into(source, body, WORK, effect, reply) } else { self.advance_into(source, body, WORK, effect, reply) }, grant)
    }
    fn advance_funded(&mut self, source: RetainedCloneRef<'_, T>, body: usize, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep, OriginalRetirementFailure> {
        let demand = self.advance_demands(source, body, WORK).expect("retained clone advance quote");
        assert!(demand.depth <= grant.maximum_depth.max(1), "depth envelope {} is below the quoted minimal step {demand:?}", grant.maximum_depth);
        let turn = funded(grant, demand);
        let outcome = self.advance(source, body, turn);
        if let Ok(step) = &outcome.result {
            assert!(step.progress().fits(turn), "turn receipt {:?} exceeds its funded grant {turn:?}", step.progress());
        }
        outcome
    }
    fn taken(&mut self, grant: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(T, RetainedCloneProgress)>, ValueError> {
        let mut output = None;
        drive::warm();
        let outcome = drive::drive_reply::<RetainedCloneStep, ValueError>(grant, WORK, |reply| match reply { Ok(step) => step.progress(), Err(error) => error.retained_progress() }, |effect, reply, recover| if recover { self.recover_take(&mut output, effect, reply) } else { self.take(&mut output, effect, reply) });
        RetainedWorkOutcome { result: outcome.result.map(|step| output.map(|value| (value, step.progress()))), work: outcome.work }
    }
    fn began(&mut self) -> bool {
        if self.close_started() {
            return false;
        }
        let demand = self.begin_close_demand().expect("retained clone begin-close demand");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        drive_insert(|effect, reply, recover| if recover { self.recover_begin_close_into(effect, reply) } else { self.begin_close_into(effect, reply) }, grant).result.expect("retained clone begin-close");
        true
    }
    fn closed(&mut self, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep, OriginalRetirementFailure> {
        let body = grant.maximum_copy_bytes;
        drive_insert(|effect, reply, recover| if recover { self.recover_close_into(body, effect, reply) } else { self.close_into(body, effect, reply) }, grant)
    }
}
impl<T: RetainedClone, C: RetainedCloneCursor<T>> CursorTurns<T> for C {}

fn fixture_labels(fixture: &Fixture) -> RetainedOrderedMap<String, String> {
    let turn = &fixture.grant;
    let retirement = physical_grant();
    let grant = RetainedOrderedMapInsertGrant {
        retirement,
        comparison: BoundedOrdGrant { maximum_items: turn.maximum_items, maximum_bytes: turn.maximum_copy_bytes },
        maximum_moved_items: turn.maximum_items,
        maximum_moved_bytes: turn.maximum_copy_bytes,
        maximum_capacity_bytes: turn.maximum_capacity_bytes,
    };
    let mut labels = RetainedOrderedMap::default();
    for (key, value) in &fixture.source.labels {
        let key = key.clone();
        let value = value.clone();
        let (mut cursor, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| RetainedOrderedMapInsertCursor::new(labels, key, value));
        assert_eq!((allocated, released), (0, 0));
        let body = turn.maximum_copy_bytes;
        for _ in 0..10000 {
            let step = drive_insert(|effect, reply, recover| if recover { cursor.recover_advance_into(body, grant, effect, reply) } else { cursor.advance_into(body, grant, effect, reply) }, retirement).result.expect("controlled fixture labels insertion");
            assert!(step.progress().fits(retirement));
            if matches!(step, RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        let mut output = None;
        drive_insert(|effect, reply, _| cursor.take_into(&mut output, effect, reply), retirement).result.expect("controlled fixture labels taken");
        labels = output.expect("controlled fixture labels completed");
        drive_insert(|effect, reply, _| cursor.begin_close_into(effect, reply), retirement).result.expect("fixture labels cursor begins closing");
        for _ in 0..10000 {
            if cursor.terminal_is_empty() {
                break;
            }
            let demand = cursor.close_demands(body).expect("fixture labels close demand");
            let close = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            let step = drive_insert(|effect, reply, recover| if recover { cursor.recover_close_into(body, effect, reply) } else { cursor.close_into(body, effect, reply) }, close).result.expect("fixture labels cursor closes");
            assert!(step.progress().fits(close));
        }
        assert!(cursor.terminal_is_empty());
    }
    labels
}

fn grant(fixture: &Fixture) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: fixture.grant.maximum_items, maximum_copy_bytes: fixture.grant.maximum_copy_bytes, maximum_capacity_bytes: fixture.grant.maximum_capacity_bytes, maximum_depth: fixture.grant.maximum_depth, maximum_release_bytes: fixture.grant.maximum_release_bytes }
}

/// 💰️ Lifts a deliberately small per-turn grant to the cursor's quoted minimal step, so the turn pays its alias and body credit and always makes progress; the payload page stays bounded by `body`.
fn funded(grant: RetainedCloneGrant, demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: grant.maximum_items.max(1), maximum_copy_bytes: grant.maximum_copy_bytes.max(demand.copy_bytes), maximum_capacity_bytes: grant.maximum_capacity_bytes.max(demand.capacity_bytes), maximum_release_bytes: grant.maximum_release_bytes.max(demand.release_bytes), maximum_depth: grant.maximum_depth.max(demand.depth).max(1) }
}

/// 🔗️ The first turn of every clone cursor binds its source alias, which spends one sealed alias handle of copy credit; this lifts a deliberately tiny per-turn payload grant to that floor for the bind turn alone.
fn bound(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: grant.maximum_items.max(1), maximum_copy_bytes: grant.maximum_copy_bytes.max(RetainedCloneBinding::alias_copy_bytes()), maximum_depth: grant.maximum_depth.max(1), ..grant }
}

/// 🧱️ The smallest per-turn copy credit under which a composite cursor can transfer its inline child cursor.
const COMPOSITE_COPY_BYTES: usize = 4096;

fn assert_progress(progress: RetainedCloneProgress, grant: RetainedCloneGrant) {
    assert!(progress.fits(grant));
}

fn quoted_close_grant<T: RetainedClone>(cursor: &T::Cursor, maximum_items: usize, copy_floor: usize) -> RetainedCloneGrant {
    let copy = cursor.next_close_copy_byte_demand().expect("retained clone copy demand").max(copy_floor);
    RetainedCloneGrant { maximum_items, maximum_copy_bytes: copy, maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(copy).expect("retained clone capacity demand"), maximum_release_bytes: cursor.next_close_release_byte_demand().expect("retained clone release demand"), maximum_depth: cursor.next_close_depth_demand().expect("retained clone depth demand").max(1) }
}

fn close_cursor<T: RetainedClone>(cursor: &mut T::Cursor) {
    assert!(cursor.began());
    assert!(!cursor.began());
    let mut turns = 0usize;
    let grant = physical_grant();
    while !cursor.terminal_is_empty() {
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| cursor.closed(grant).result.expect("retained clone close"));
        assert!(step.progress().fits(grant));
        assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(cursor.terminal_is_empty());
        }
        turns += 1;
        assert!(turns < 100_000, "retained clone close did not terminate for {}", std::any::type_name::<T>());
    }
}

fn retained_copy<T>(source: &T, body:usize, grant: RetainedCloneGrant) -> T
where
    T: RetainedClone + Clone,
{
    let mut source = retained_source(source.clone());
    let mut cursor = T::retained_clone_cursor();
    let mut turns = 0usize;
    let output = loop {
        turns += 1;
        let turn = if turns == 1 { bound(grant) } else { grant };
        let step = cursor.advance_funded(source.borrow(), body, turn).result.expect("retained copy");
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break cursor.taken(physical_grant()).result.expect("retained take").expect("retained copy output").0;
        }
        assert!(turns < 100_000, "retained copy stalled under {grant:?}; last turn {step:?}; next demand {:?}", cursor.advance_demands(source.borrow(), body, WORK));
    };
    assert!(cursor.advance(source.borrow(), body, grant).result.is_err());
    close_cursor::<T>(&mut cursor);
    close_source(&mut source);
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
    let mut retained_source = retained_source(source.clone());
    let clone_oracle = source.clone();
    let serde_oracle = serde_json::to_value(&source).expect("serde oracle");
    let grant = grant(&fixture);
    let mut cursor = NeutralSnapshot::retained_clone_cursor();
    let mut turns = 0usize;
    let output = loop {
        turns += 1;
        let step = cursor.advance_funded(retained_source.borrow(), fixture.grant.maximum_copy_bytes, grant).result.expect("retained clone step");
        if let RetainedCloneStep::Complete(_) = step {
            break cursor.taken(physical_grant()).result.expect("retained take").expect("completed retained clone").0;
        }
        assert!(turns < 10_000);
    };
    assert!(turns > fixture.payload_byte_length / fixture.grant.maximum_copy_bytes);
    assert_eq!(output, clone_oracle);
    assert_eq!(serde_json::to_value(&output).expect("retained serde output"), serde_oracle);
    close_cursor::<NeutralSnapshot>(&mut cursor);
    close_source(&mut retained_source);
}

#[test]
fn capacity_refusal_precedes_large_vector_allocation() {
    let fixture = fixture();
    let source = source(&fixture);
    let mut retained_owner = retained_source(source);
    let mut cursor = NeutralSnapshot::retained_clone_cursor();
    let admitted = grant(&fixture);
    let refused = RetainedCloneGrant { maximum_capacity_bytes: fixture.insufficient_capacity_bytes, ..admitted };
    let mut reached_refusal = false;
    for _ in 0..32 {
        let step = cursor.advance(retained_owner.borrow(), fixture.grant.maximum_copy_bytes, refused).result.expect("capacity probe");
        let progress = step.progress();
        assert_progress(progress, refused);
        if progress == RetainedCloneProgress::default() {
            reached_refusal = true;
            break;
        }
    }
    assert!(reached_refusal);
    close_cursor::<NeutralSnapshot>(&mut cursor);
    close_source(&mut retained_owner);
}

#[test]
fn cancellation_retires_every_partial_owner_through_bounded_steps() {
    let fixture = fixture();
    let source = source(&fixture);
    let mut retained_owner = retained_source(source);
    let grant = grant(&fixture);
    for stop in &fixture.cancellation_stops {
        let mut cursor = NeutralSnapshot::retained_clone_cursor();
        for _ in 0..*stop {
            if matches!(cursor.advance(retained_owner.borrow(), fixture.grant.maximum_copy_bytes, grant).result.expect("retained clone before cancel"), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(cursor.began());
        let mut close_turns = 0usize;
        while !cursor.terminal_is_empty() {
            let step = cursor.closed(physical_grant()).result.expect("retained clone close");
            if matches!(step, RetainedCloneStep::Complete(_)) {
                assert!(cursor.terminal_is_empty());
            }
            assert!(step.progress().fits(physical_grant()));
            close_turns += 1;
            assert!(close_turns < 100_000);
        }
    }
    close_source(&mut retained_owner);
}

#[test]
fn utf8_copy_is_boundary_paged_and_source_lease_captures_one_value() {
    let fixture = fixture();
    let source = "βeta 🧬 Deutsch 🇯🇵".repeat(512);
    let grant = RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 7, maximum_capacity_bytes: source.len(), maximum_depth: 512, maximum_release_bytes: source.len() };
    assert_eq!(retained_copy(&source, 7, grant), source);

    let mut external = fixture.immutable_lease.captured.clone();
    let mut retained = retained_source(external.clone());
    let mut cursor = String::retained_clone_cursor();
    let reserve = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 7, maximum_capacity_bytes: external.len(), maximum_depth: 512, maximum_release_bytes: external.len() };
    cursor.advance_funded(retained.borrow(), 7, bound(reserve)).result.expect("string alias bind");
    cursor.advance_funded(retained.borrow(), 7, reserve).result.expect("string reserve");
    cursor.advance_funded(retained.borrow(), 7, reserve).result.expect("first immutable page");
    external = fixture.immutable_lease.external_after_capture.clone();
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        assert!(turns < 10_000, "captured string copy stalled under {reserve:?}");
        if matches!(cursor.advance_funded(retained.borrow(), 7, reserve).result.expect("captured string copy"), RetainedCloneStep::Complete(_)) {
            break cursor.taken(physical_grant()).result.expect("retained take").expect("captured string output").0;
        }
    };
    assert_eq!(copied, fixture.immutable_lease.expected);
    assert_ne!(copied, external);
    close_cursor::<String>(&mut cursor);
    close_source(&mut retained);

    let mut siblings = retained_source(("same".to_string(), "same".to_string()));
    let mut sibling_cursor = String::retained_clone_cursor();
    sibling_cursor.advance_funded(siblings.borrow().project(1, |value| &value.0), 2, bound(reserve)).result.expect("first sibling projection");
    let error = sibling_cursor.advance(siblings.borrow().project(2, |value| &value.1), 2, reserve).result.expect_err("crossed sibling projection must fail").into_value_error();
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::InvariantViolated);
    assert!(error.message.contains("projected path changed"));
    close_cursor::<String>(&mut sibling_cursor);
    close_source(&mut siblings);
}

/// 🎟️ Drives one staged source admission to its published source under repeated identical grants.
fn seal_read<T: semio_framework_value::retirement::RetireOwned + Sync + 'static>(read: snapshot_read::SnapshotRead<T>, grant: RetainedCloneGrant) -> RetainedCloneSource<T, snapshot_read::SnapshotRead<T>> {
    drive::warm();
    let (source, progress) = drive::with_bank(|bank| semio_framework_value_clone_source_testing::admit_projected::<T, snapshot_read::SnapshotRead<T>, ()>(bank, read, (), grant).result.unwrap_or_else(|_| panic!("staged snapshot source admission refused")));
    assert!(progress.fits(grant));
    source
}

#[test]
fn production_snapshot_read_lease_survives_multiturn_copy_and_bounded_cancellation() {
    let captured = fixture().immutable_lease.captured;
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(captured.clone());
    let lease = registry.try_issue(Arc::clone(&owner)).expect("production snapshot read lease");
    let mut retained_source = seal_read(snapshot_read::SnapshotRead::new(Arc::clone(&owner), lease), SOURCE_STOCK);
    let copy_grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: captured.len(), maximum_depth: 64, maximum_release_bytes: captured.len() };
    let mut cursor = String::retained_clone_cursor();
    cursor.advance_funded(retained_source.borrow(), 1, bound(copy_grant)).result.expect("production snapshot read alias bind");
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        assert!(turns < 100_000, "production snapshot read copy stalled under {copy_grant:?}");
        if matches!(cursor.advance_funded(retained_source.borrow(), 1, copy_grant).result.expect("production snapshot read copy"), RetainedCloneStep::Complete(_)) {
            break cursor.taken(physical_grant()).result.expect("retained take").expect("production snapshot read output").0;
        }
    };
    assert!(turns > 2);
    assert_eq!(copied, captured);
    close_cursor::<String>(&mut cursor);
    close_source(&mut retained_source);
    assert!(registry.terminal_is_empty());
    close_registry(registry);
    drop(owner);

    let fixture = fixture();
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(source(&fixture));
    let lease = registry.try_issue(Arc::clone(&owner)).expect("production cancellation snapshot read lease");
    let mut retained_source = seal_read(snapshot_read::SnapshotRead::new(owner, lease), SOURCE_STOCK);
    let grant = grant(&fixture);
    let mut cursor = NeutralSnapshot::retained_clone_cursor();
    cursor.advance_funded(retained_source.borrow(), 1, grant).result.expect("production cancellation reserve");
    cursor.advance_funded(retained_source.borrow(), 1, grant).result.expect("production cancellation prefix");
    close_source(&mut retained_source);
    assert!(!registry.terminal_is_empty(), "active cursor binding retains the exact production snapshot read");
    close_cursor::<NeutralSnapshot>(&mut cursor);
    assert!(registry.has_returned(), "the last large owner returns to the bounded registry pump");
    let factory: Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<NeutralSnapshot>> = Arc::new(OwnedValueRetirementFactory::<NeutralSnapshot>::default());
    let mut active = None;
    let mut retirement_turns = 0usize;
    loop {
        retirement_turns += 1;
        let step = drive::drive(physical_grant(), WORK, |effect| snapshot_read::advance_returned_snapshot_read(&registry, &mut active, &factory, effect)).result.expect("returned large snapshot retirement");
        assert!(step.progress().fits(physical_grant()));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break;
        }
        assert!(retirement_turns < 100_000, "returned large snapshot retirement remains bounded");
    }
    assert!(retirement_turns > 1, "large nested ownership is drained by the registry over multiple turns");
    assert!(registry.terminal_is_empty());
    close_registry(registry);
}

#[test]
fn retained_vector_and_completed_optional_preserve_their_captured_owner() {
    let mut values = vec![1u64, 2, 3, 4];
    let mut retained_values = retained_source(values.clone());
    let grant = RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: 64, maximum_capacity_bytes: 1_024, maximum_depth: 512, maximum_release_bytes: 1_024 };
    let mut vector = Vec::<u64>::retained_clone_cursor();
    vector.advance_funded(retained_values.borrow(), 64, bound(grant)).result.expect("vector alias bind");
    vector.advance_funded(retained_values.borrow(), 64, grant).result.expect("vector reserve");
    values.pop();
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        assert!(turns < 10_000, "captured vector copy stalled under {grant:?}");
        if matches!(vector.advance_funded(retained_values.borrow(), 64, grant).result.expect("captured vector"), RetainedCloneStep::Complete(_)) {
            break vector.taken(physical_grant()).result.expect("retained take").expect("captured vector output").0;
        }
    };
    assert_eq!(copied, vec![1, 2, 3, 4]);
    close_cursor::<Vec<u64>>(&mut vector);
    close_source(&mut retained_values);

    let source = Some("owner".repeat(1_024));
    let mut retained_source = retained_source(source.clone());
    let mut optional = Option::<String>::retained_clone_cursor();
    let optional_grant = RetainedCloneGrant { maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_copy_bytes: COMPOSITE_COPY_BYTES, ..grant };
    let mut turns = 0usize;
    loop {
        turns += 1;
        assert!(turns < 10_000, "optional copy stalled under {optional_grant:?}");
        if matches!(optional.advance_funded(retained_source.borrow(), COMPOSITE_COPY_BYTES, optional_grant).result.expect("optional copy"), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(matches!(optional.advance_funded(retained_source.borrow(), COMPOSITE_COPY_BYTES, optional_grant).result.expect("completed optional"), RetainedCloneStep::Complete(_)));
    assert_eq!(optional.taken(physical_grant()).result.unwrap().map(|(value,_)|value), Some(source.clone()));
    assert!(optional.advance(retained_source.borrow(), COMPOSITE_COPY_BYTES, grant).result.is_err());
    close_cursor::<Option<String>>(&mut optional);
    close_source(&mut retained_source);
}

#[test]
fn unit_generic_enum_and_recursive_records_remain_grant_bounded() {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: COMPOSITE_COPY_BYTES, maximum_capacity_bytes: 65_536, maximum_depth: 512, maximum_release_bytes: 65_536 };
    let units = vec![UnitRecord; 256];
    let mut retained_units = retained_source(units.clone());
    let mut cursor = Vec::<UnitRecord>::retained_clone_cursor();
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        let step = cursor.advance_funded(retained_units.borrow(), COMPOSITE_COPY_BYTES, grant).result.expect("unit vector clone");
        assert!(step.progress().copied_items <= 1);
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break cursor.taken(physical_grant()).result.expect("retained take").expect("unit vector output").0;
        }
        assert!(turns < 10_000);
    };
    assert_eq!(copied, units);
    assert!(turns > units.len());
    close_cursor::<Vec<UnitRecord>>(&mut cursor);
    close_source(&mut retained_units);

    let generic = GenericRecord { value: "generic".repeat(128) };
    assert_eq!(retained_copy(&generic, COMPOSITE_COPY_BYTES, grant), generic);
    for choice in [GenericChoice::Unit, GenericChoice::Tuple("tuple".to_string(), "text".to_string()), GenericChoice::Named { value: "named".to_string() }] {
        assert_eq!(retained_copy(&choice, COMPOSITE_COPY_BYTES, grant), choice);
    }

    let recursive = recursive_record(fixture().recursive_depth);
    let copied = retained_copy(&recursive, COMPOSITE_COPY_BYTES, grant);
    assert_eq!(copied, recursive);
    assert_eq!(serde_json::to_value(copied).expect("recursive retained serde"), serde_json::to_value(recursive).expect("recursive clone serde"));

    fn assert_retained<T: RetainedClone>() {}
    assert_retained::<EmptyRecord>();
}

/// 🌲️ Clones one recursive record under a fixed depth envelope; `None` when the envelope refuses it or the cursor can no longer advance.
fn clone_under_depth(source: &RecursiveRecord, depth: usize) -> Option<RecursiveRecord> {
    let grant = RetainedCloneGrant { maximum_items: 4, maximum_copy_bytes: COMPOSITE_COPY_BYTES + RetainedCloneBinding::alias_copy_bytes(), maximum_capacity_bytes: 1 << 20, maximum_depth: depth, maximum_release_bytes: 1 << 20 };
    let mut retained = retained_source(source.clone());
    let mut cursor = RecursiveRecord::retained_clone_cursor();
    let mut idle = 0usize;
    let mut output = None;
    for turn in 0..400_000usize {
        let step = cursor.advance(retained.borrow(), COMPOSITE_COPY_BYTES, grant).result;
        match step {
            Err(_) => break,
            Ok(RetainedCloneStep::Complete(_)) => {
                output = cursor.taken(physical_grant()).result.unwrap().map(|(value,_)|value);
                break;
            }
            Ok(step) => {
                idle = if step.progress() == RetainedCloneProgress::default() { idle + 1 } else { 0 };
                if idle >= 8 {
                    break;
                }
            }
        }
    }
    close_cursor::<RecursiveRecord>(&mut cursor);
    close_source(&mut retained);
    output
}

#[test]
fn recursive_depth_envelope_accepts_boundary_and_rejects_the_next_box() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🧪️tests/🧫️fixtures/🌳️structural-depth/🔣️.json")).unwrap();
    let mut boxes = law["cases"].as_array().unwrap().iter().map(|row| row["boxes"].as_u64().unwrap() as usize).collect::<Vec<_>>();
    boxes.sort_unstable();
    boxes.dedup();
    let mut measured = Vec::new();
    for boxes in boxes {
        let source = recursive_record(boxes);
        let (mut low, mut high) = (1usize, 1024usize);
        assert!(clone_under_depth(&source, high).is_some(), "{boxes} boxes fit a finite depth envelope");
        while low < high {
            let middle = low + (high - low) / 2;
            if clone_under_depth(&source, middle).is_some() { high = middle } else { low = middle + 1 }
        }
        let copied = clone_under_depth(&source, low).expect("the boundary depth accepts the record");
        assert_eq!(copied, source);
        assert_eq!(serde_json::to_value(&copied).unwrap(), serde_json::to_value(&source).unwrap());
        assert!(low == 1 || clone_under_depth(&source, low - 1).is_none(), "one level below the boundary refuses {boxes} boxes");
        measured.push((boxes, low));
    }
    let slope = (measured[measured.len() - 1].1 - measured[0].1) as f64 / (measured[measured.len() - 1].0 - measured[0].0) as f64;
    for pair in measured.windows(2) {
        let step = (pair[1].1 - pair[0].1) as f64 / (pair[1].0 - pair[0].0) as f64;
        assert!((step - slope).abs() < 1e-9, "the depth envelope grows by one constant amount per box: {measured:?}");
    }
}

#[test]
fn recursive_partial_copy_cancels_within_the_declared_depth_envelope() {
    let maximum_depth = 64usize;
    let source = recursive_record((maximum_depth - 1) / 2);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: COMPOSITE_COPY_BYTES, maximum_capacity_bytes: 65_536, maximum_depth, maximum_release_bytes: 65_536 };
    let mut probe_source = retained_source(source.clone());
    let mut probe = RecursiveRecord::retained_clone_cursor();
    let mut complete_turns = 0usize;
    let started = std::time::Instant::now();
    loop {
        let step = probe.advance_funded(probe_source.borrow(), COMPOSITE_COPY_BYTES, grant).result.expect("recursive probe copy");
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
        complete_turns += 1;
        if complete_turns % 100 == 0 { eprintln!("[DEBUG] recursive probe turn {complete_turns} after {:?}: {step:?} next {:?}", started.elapsed(), probe.advance_demands(probe_source.borrow(), COMPOSITE_COPY_BYTES, WORK)); }
        assert!(complete_turns < 3_000, "recursive probe copy stalled under {grant:?}; last turn {step:?}");
    }
    probe.taken(physical_grant()).result.expect("recursive probe take");
    close_cursor::<RecursiveRecord>(&mut probe);
    close_source(&mut probe_source);
    let mut retained_owner = retained_source(source);
    let mut cursor = RecursiveRecord::retained_clone_cursor();
    for _ in 0..complete_turns {
        assert!(matches!(cursor.advance_funded(retained_owner.borrow(), COMPOSITE_COPY_BYTES, grant).result.expect("recursive prefix copy"), RetainedCloneStep::Progress(_)));
    }
    assert!(cursor.began());
    let mut turns = 0usize;
    while !cursor.terminal_is_empty() {
        let step = cursor.closed(grant).result.expect("recursive prefix physical close");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(cursor.terminal_is_empty());
        }
        turns += 1;
        assert!(turns < 100_000);
    }
    assert!(turns > (maximum_depth - 1) / 2, "closing the partial nesting takes at least one turn per live nested owner");
    close_source(&mut retained_owner);
}

#[test]
fn fixed_arrays_standard_tuples_and_tuple_records_preserve_native_shape() {
    let array = std::array::from_fn::<_, 128, _>(|index| index as u32);
    let refused = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: std::mem::size_of_val(&array) - 1, maximum_capacity_bytes: 0, maximum_depth: 512, maximum_release_bytes: 0 };
    let mut cursor = <[u32; 128]>::retained_clone_cursor();
    let mut retained_array = retained_source(array);
    cursor.advance(retained_array.borrow(), std::mem::size_of_val(&array), bound(refused)).result.expect("array alias bind");
    assert_eq!(cursor.advance(retained_array.borrow(), std::mem::size_of_val(&array), refused).result.expect("array refusal").progress(), RetainedCloneProgress::default());
    close_cursor::<[u32; 128]>(&mut cursor);
    close_source(&mut retained_array);
    let admitted = RetainedCloneGrant { maximum_copy_bytes: std::mem::size_of_val(&array), ..refused };
    assert_eq!(retained_copy(&array, std::mem::size_of_val(&array), admitted), array);

    let pair = ("βeta".repeat(128), 42u64);
    let triple = ("Deutsch".to_string(), 7u32, "日本語".to_string());
    let record = TupleRecord("tuple record".repeat(64), 9);
    let grant = RetainedCloneGrant { maximum_items: 2, maximum_copy_bytes: COMPOSITE_COPY_BYTES, maximum_capacity_bytes: 65_536, maximum_depth: 512, maximum_release_bytes: 65_536 };
    assert_eq!(retained_copy(&pair, COMPOSITE_COPY_BYTES, grant), pair);
    assert_eq!(retained_copy(&triple, COMPOSITE_COPY_BYTES, grant), triple);
    assert_eq!(retained_copy(&record, COMPOSITE_COPY_BYTES, grant), record);
}

#[test]
fn zero_grants_never_construct_or_complete_an_owner() {
    let zero = RetainedCloneGrant::default();
    let mut scalar = u64::retained_clone_cursor();
    let mut scalar_source = retained_source(7u64);
    assert_eq!(scalar.advance(scalar_source.borrow(), 0, zero).result.expect("zero scalar").progress(), RetainedCloneProgress::default());
    assert!(scalar.taken(physical_grant()).result.unwrap().is_none());
    close_cursor::<u64>(&mut scalar);
    close_source(&mut scalar_source);

    let text = "β".repeat(64);
    let mut retained_text = retained_source(text);
    let mut string = String::retained_clone_cursor();
    assert_eq!(string.advance(retained_text.borrow(), 0, zero).result.expect("zero string").progress(), RetainedCloneProgress::default());
    assert!(string.taken(physical_grant()).result.unwrap().is_none());
    close_cursor::<String>(&mut string);
    close_source(&mut retained_text);

    let values = vec![UnitRecord; 16];
    let mut retained_values = retained_source(values);
    let mut vector = Vec::<UnitRecord>::retained_clone_cursor();
    assert_eq!(vector.advance(retained_values.borrow(), 0, zero).result.expect("zero vector").progress(), RetainedCloneProgress::default());
    assert!(vector.taken(physical_grant()).result.unwrap().is_none());
    close_cursor::<Vec<UnitRecord>>(&mut vector);
    close_source(&mut retained_values);

    let source = Some("owner".to_string());
    let mut retained_owner = retained_source(source);
    let mut optional = Option::<String>::retained_clone_cursor();
    assert_eq!(optional.advance(retained_owner.borrow(), 0, zero).result.expect("zero optional").progress(), RetainedCloneProgress::default());
    assert!(optional.taken(physical_grant()).result.unwrap().is_none());
    close_cursor::<Option<String>>(&mut optional);
    close_source(&mut retained_owner);

    let unit = UnitRecord;
    let mut retained_unit = retained_source(unit);
    let mut derived = UnitRecord::retained_clone_cursor();
    assert_eq!(derived.advance(retained_unit.borrow(), 0, zero).result.expect("zero derived").progress(), RetainedCloneProgress::default());
    assert!(derived.taken(physical_grant()).result.unwrap().is_none());
    close_cursor::<UnitRecord>(&mut derived);
    close_source(&mut retained_unit);
}

fn physical_grant() -> RetainedCloneGrant {
    let policy = fixture().retirement_grant;
    RetainedCloneGrant { maximum_items: policy.maximum_items, maximum_copy_bytes: policy.maximum_copy_bytes, maximum_capacity_bytes: policy.maximum_capacity_bytes, maximum_release_bytes: policy.maximum_release_bytes, maximum_depth: policy.maximum_depth }
}

const SOURCE_STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 25, maximum_depth: 256 };

fn retained_source<T: RetireOwned + Sync + 'static>(owner: T) -> RetainedCloneSource<T> {
    drive::warm();
    let (source, progress) = drive::with_bank(|bank| semio_framework_value_clone_source_testing::admit_owned(bank, owner, (), SOURCE_STOCK).result.unwrap_or_else(|_| panic!("original Source admission refused")));
    assert!(progress.fits(SOURCE_STOCK));
    source
}

fn close_source<T: Sync + 'static, A: RetireOwned + Sync>(source: &mut RetainedCloneSource<T, A>) {
    let grant = physical_grant();
    for _ in 0..100_000 {
        if source.terminal_is_empty() { return; }
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| { let body = grant.maximum_copy_bytes; drive::drive(grant, WORK, |effect| source.close_step(body, effect)).result.expect("fixed source retirement policy") });
        assert!(step.progress().fits(grant));
        assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    panic!("original source did not close under its independent policy");
}

fn close_registry(registry: semio_framework_value_snapshot_read::SnapshotReadRegistryHandle) {
    let mut original = Some(semio_framework_value_snapshot_read::SnapshotReadRegistryAliasRetirement::new(registry));
    let grant = physical_grant();
    for _ in 0..100_000 {
        if original.is_none() { return; }
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drive::drive(grant, WORK, |effect| semio_framework_value_snapshot_read::snapshot_registry_alias_close_step(&mut original, effect)).result.expect("fixed registry backing policy"));
        assert!(step.progress().fits(grant));
        assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
    }
    panic!("original registry backing did not close under its independent policy");
}

fn child_fixture() -> (ArtifactChild<()>, Arc<String>) {
    let owner = Arc::new("local materialization".to_string());
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "artifact-β".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "semio.test.child".into(), standard: "standard".into(), subset: "*".into() } };
    (ArtifactChild::new("child-α".into(), target).with_local_owner(owner.clone()), owner)
}

#[test]
fn artifact_child_retained_clone_matches_clone_and_value_oracles_and_aliases_the_local_owner() {
    let (child, owner) = child_fixture();
    let oracle = child.clone();
    let copy = retained_copy(&child, fixture().retirement_grant.maximum_copy_bytes, physical_grant());
    assert_eq!(copy, oracle);
    assert_eq!(semio_framework_value::ToValue::to_value(&copy), semio_framework_value::ToValue::to_value(&oracle));
    assert!(Arc::ptr_eq(&copy.require_local_owner::<String>().expect("cloned child aliases its local owner"), &owner));
    let bare = ArtifactChild::<()>::new("bare".into(), child.target.clone());
    let bare_copy = retained_copy(&bare, fixture().retirement_grant.maximum_copy_bytes, physical_grant());
    assert_eq!(bare_copy, bare);
    assert!(bare_copy.local_owner::<String>().is_none());
    drop(copy);
    drop(bare_copy);
    drop(child);
    drop(oracle);
    assert_eq!(Arc::strong_count(&owner), 1);
}

#[test]
fn artifact_child_retained_clone_retires_every_partial_cursor_under_cancellation() {
    let (child, owner) = child_fixture();
    let mut source = retained_source(child);
    let grant = physical_grant();
    for stop in 0..16usize {
        let mut cursor = <ArtifactChild<()> as RetainedClone>::retained_clone_cursor();
        let mut completed = false;
        for _ in 0..stop {
            if matches!(cursor.advance(source.borrow(), fixture().retirement_grant.maximum_copy_bytes, grant).result.expect("child clone before cancel"), RetainedCloneStep::Complete(_)) {
                completed = true;
                break;
            }
        }
        if completed {
            drop(cursor.taken(physical_grant()));
        }
        close_cursor::<ArtifactChild<()>>(&mut cursor);
    }
    close_source(&mut source);
    assert_eq!(Arc::strong_count(&owner), 1);
}
