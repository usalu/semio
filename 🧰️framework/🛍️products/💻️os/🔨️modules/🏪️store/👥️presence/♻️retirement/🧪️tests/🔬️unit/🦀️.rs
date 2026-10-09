use super::fixture_mutations::{SetValue, ValueMutation};
use super::*;
use crate::os_store::component::presence_test_retirement::{Factory, CountedPresence, CLOSE_GRANT, observed_close, observed_step, finish, finish_box, admit_returned_string, finish_registry};
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, retirement::controlled::ControlledRetirement};

pub(super) fn assert_fixture_descriptor<T: crate::os_spr::MutationLeaf>(descriptor: &str) {
    assert_eq!(serde_json::Value::from(T::DESCRIPTOR.to_value()), serde_json::from_str::<serde_json::Value>(descriptor).unwrap());
    assert!(T::DESCRIPTOR.validate().is_ok());
}

/// 🧾️ The committed wire witness of `set-value` decodes through `ValueMutation`'s `FromValue` and re-encodes to exactly the
/// committed JSON: it is the canonical Rust wire of the leaf.
#[test]
fn committed_wire_witness_is_the_canonical_wire() {
    crate::os_store::test_support::assert_wire_witness::<ValueMutation>(include_str!("../../🧪️testing/🧬️mutations/🔢️set-value/🧫️fixtures/🧾️wire-witness/🦠️mutation/🔣️.json"));
}

#[test]
fn direct_presence_fixture_value_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["family"] == "presence").unwrap();
    let before = Value(serde_json::from_value(row["before"].clone()).unwrap());
    let mut wire = row["payload"].clone();
    wire["operation"] = row["operation"].clone();
    let op = serde_json::from_value::<ValueMutation>(wire).unwrap();
    let after = crate::os_spr::apply_diff(op.diff(&before).diff(), &before).unwrap();
    assert_eq!(after.0, serde_json::from_value::<i32>(row["after"].clone()).unwrap());
    assert_eq!(crate::os_spr::apply_diff(op.inverse(&before).expect("valid retained mutation inverse fixture")[0].diff(&after).diff(), &after).unwrap().0, before.0);
    assert_eq!(serde_json::from_value::<ValueMutation>(serde_json::to_value(&op).unwrap()).unwrap(), op);
    for json in ["{\"operation\":\"setValue\"}", "{\"operation\":\"setValue\",\"n\":null}", "{\"operation\":\"setValue\",\"n\":2147483648}", "{\"operation\":\"setValue\",\"n\":0.5}", "{\"operation\":\"setValue\",\"n\":7,\"unknown\":true}"] {
        assert!(serde_json::from_str::<ValueMutation>(json).is_err());
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::RetireOwned)]
pub(crate) struct Value(pub(super) i32);

/// 🔀️ Hand-written, not derived: tuple struct, not one of `#[derive(ToValue, FromValue)]`'s
/// supported shapes.
impl semio_framework_value::ToValue for Value {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&self.0)
    }
}
impl semio_framework_value::FromValue for Value {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self(semio_framework_value::FromValue::from_value(value)?))
    }
}

impl crate::os_spr::DiffAlgebra<Value> for Value {
    fn inverse(&self, base: &Value) -> Self {
        base.clone()
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl MutationDiff<Value> for Value {
    fn apply(&self, _base: &Value, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<Value> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl CountedPresence for Value { fn counted(self) -> bool { self.0 != 0 } }

struct CapturedLocalJob {
    read: Option<ControlledRetirement<SnapshotRead<Value>>>,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for CapturedLocalJob {
    fn step(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        assert_eq!(self.read.as_ref().unwrap().original().unwrap().get().0, 23);
        semio_framework_job::StepOutcome::Yield
    }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let Some(read) = self.read.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }; };
        match read.step(grant) {
            Err(error) => semio_framework_job::InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
            Ok(step) => {
                let progress = step.progress();
                if read.terminal_is_empty() { self.read.take(); }
                if self.terminal_is_empty() { semio_framework_job::InteractiveJobCloseStep::Complete { progress } }
                else { semio_framework_job::InteractiveJobCloseStep::Pending { progress } }
            }
        }
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.read.as_ref().map_or(Ok(0), |read| read.next_copy_byte_demand()) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, semio_framework_value::ValueError> { self.read.as_ref().map_or(Ok(0), |read| read.next_capacity_byte_demand(copy)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.read.as_ref().map_or(Ok(0), |read| read.next_release_byte_demand()) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.read.as_ref().map_or(Ok(0), |read| read.next_depth_demand()) }
    fn terminal_is_empty(&self) -> bool { self.closing && self.read.is_none() }
}

#[test]
fn retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧹️retirement.json")).unwrap();
    let law = &fixture["localCapture"];
    let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let factory = Arc::new(Factory(count.clone()));
    let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(law["value"].as_i64().unwrap() as i32));
    assert!(owner.local_read().is_err());
    owner.install_local_retirement_factory(factory).unwrap();
    let job = CapturedLocalJob { read: Some(ControlledRetirement::new(owner.local_read().unwrap()).unwrap_or_else(|_| panic!("original controlled read"))), closing: false };
    let cancel = semio_framework_job::root_cancel_token();
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::allocate_operation_id(),
        generation: semio_framework_job::Generation(0),
        cancel: cancel.clone(),
        config: semio_framework_job::BatchDriveConfig { retained: CLOSE_GRANT, site: "presence.local.capture", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 8000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = semio_framework_job::MountedWorkerJobSession::try_new(job, params).unwrap_or_else(|_| panic!("exact mounted capture slot"));
    let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)));
    for _ in 0..100_000 {
        session.pump_one(&pool, semio_framework_async::Lane::Interactive).unwrap_or_else(|_| panic!("mounted Presence capture must pump its exact worker"));
        if session.checked_out_outcome().is_some() {
            break;
        }
        std::thread::yield_now();
    }
    assert!(session.checked_out_outcome().is_some());
    cancel.cancel_now();
    session.begin_close();
    let mut observed_worker_page = false;
    let mut cancel_handbacks = 0;
    for _ in 0..4096 {
        let demand = session.retirement_demands(CLOSE_GRANT.maximum_copy_bytes).unwrap().release_bytes;
        if demand == law["workerPageBytes"].as_u64().unwrap() as usize {
            let phase = session.close_phase();
            let denied = RetainedCloneGrant { maximum_release_bytes: 4096, ..CLOSE_GRANT };
            let step = session.close_step(denied);
            assert!(matches!(step, semio_framework_job::WorkerJobCloseStep::Pending { .. }));
            let progress = step.progress();
            assert_eq!(serde_json::json!({"releasedItems": progress.copied_items, "releasedBytes": progress.released_bytes}), law["workerUndergrant"]);
            assert_eq!(session.close_phase(), phase);
            assert_eq!(session.retirement_demands(CLOSE_GRANT.maximum_copy_bytes).unwrap().release_bytes, demand);
            observed_worker_page = true;
        }
        let (returned, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| session.return_original_cancel_alias_step(&cancel, CLOSE_GRANT).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        if let Some(step) = returned {
            assert!(step.progress().fits(CLOSE_GRANT));
            assert_eq!(step.progress(), RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<semio_framework_async::CancelToken>(), ..Default::default() });
            assert!(matches!(step, RetainedCloneStep::Complete(_)));
            cancel_handbacks += 1;
        } else {
            let step = session.close_step(CLOSE_GRANT);
            assert!(step.progress().fits(CLOSE_GRANT));
            assert!(!matches!(step, semio_framework_job::WorkerJobCloseStep::Refused { .. }));
        }
        observed_step(CLOSE_GRANT, || owner.maintenance_local_reads_step(CLOSE_GRANT));
        if session.terminal_is_empty() && owner.local_read_maintenance_is_idle() {
            break;
        }
    }
    assert!(observed_worker_page);
    assert_eq!(session.terminal_is_empty(), law["expectedWorkerTerminal"].as_bool().unwrap());
    assert_eq!(cancel_handbacks, 1);
    let mut original_cancel = semio_framework_async::CancelTokenRetirement::from_token(cancel);
    let mut cancel_released = 0;
    for _ in 0..64 {
        if original_cancel.terminal_is_empty() { break; }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| original_cancel.close_step(CLOSE_GRANT).unwrap());
        assert!(step.progress().fits(CLOSE_GRANT));
        assert_eq!((step.progress().retained_capacity_bytes, step.progress().released_bytes), (heap.requested_bytes, heap.released_bytes));
        cancel_released += heap.released_bytes;
    }
    assert!(original_cancel.terminal_is_empty());
    assert!(cancel_released > 0);
    assert!(!owner.retirement_started());
    assert_eq!(serde_json::to_value(owner.local()).unwrap(), law["expectedValueWhileOpen"]);
    assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
    let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
    for _ in 0..2048 {
        if matches!(observed_close(&mut close, CLOSE_GRANT), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(close.terminal_is_empty());
    assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedFinalSnapshots"]);
    eprintln!("[DEBUG] original mounted Presence capture returns matching cancel alias exactly once without heap effects; caller root releases separately with exact System receipt; Store remains open until its own retirement");
}

#[test]
fn retained_presence_local_replacements_release_shared_aliases_and_retire_exact_final_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧹️retirement.json")).unwrap();
    let law = &fixture["localReplacements"];
    let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let factory = Arc::new(Factory(count.clone()));
    let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(23));
    owner.install_local_retirement_factory(factory).unwrap();
    let first = owner.local_read().unwrap();
    owner.apply_one(0, ValueMutation::SetValue(SetValue { n: 31 })).ok().unwrap();
    let second = owner.local_read().unwrap();
    owner.apply_one(1, ValueMutation::SetValue(SetValue { n: 47 })).ok().unwrap();
    assert_eq!(serde_json::json!([first.get().0, second.get().0]), law["capturedValues"]);
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let other = barrier.clone();
    let first = std::thread::spawn(move || {
        other.wait();
        drop(first);
    });
    let second = std::thread::spawn(move || {
        barrier.wait();
        drop(second);
    });
    first.join().unwrap();
    second.join().unwrap();
    let guard = owner.local_reads.as_ref().unwrap().state.try_lock().unwrap();
    assert_eq!(advance_returned_local(owner.local_reads.as_ref().unwrap(), &mut owner.active_returned_local, owner.local_retirement_factory.as_ref(), CLOSE_GRANT).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    drop(guard);
    for _ in 0..4096 {
        if matches!(observed_step(CLOSE_GRANT, || owner.maintenance_local_reads_step(CLOSE_GRANT)), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(owner.local_read_maintenance_is_idle());
    assert!(!owner.retirement_started());
    assert_eq!(owner.local().0, 47);
    assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedRetiredWhileOpen"]);
    let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
    for _ in 0..2048 {
        if matches!(observed_close(&mut close, CLOSE_GRANT), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(close.terminal_is_empty());
    assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedFinalSnapshots"]);
}

fn close_peer_root(mut retirement: PresencePeersRetirement<Value>) -> usize {
    let mut actor_bytes = 0;
    for _ in 0..2048 {
        let body = retirement.entry.as_ref().filter(|entry| entry.actor.capacity() != 0).map_or(0, |entry| entry.actor.len());
        let step = observed_close(&mut retirement, CLOSE_GRANT);
        if step.progress().released_bytes != 0 { actor_bytes += body; }
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(retirement.terminal_is_empty()); return actor_bytes; }
    }
    panic!("exact peer root failed bounded terminal progress")
}

fn peer_commit(owner: &PresenceStore<Value, ValueMutation>, peer: &serde_json::Value) -> PresencePeersCommit<Value> {
    let mut publication = owner.begin_peer_publication().unwrap();
    while publication.prune_one(|_| true).unwrap() {}
    publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
    while publication.release_created_one() {}
    let commit = publication.take_commit().unwrap();
    assert!(publication.terminal_is_empty());
    commit
}

#[test]
fn original_peer_publication_refuses_before_birth_and_settles_actual_original_receipts() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🫴️publication.json")).unwrap();
    let p = &law["grant"];
    let grant = RetainedCloneGrant { maximum_items: p["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: p["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: p["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: p["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: p["maximumDepth"].as_u64().unwrap() as usize };
    for refusal in law["prebirthRefusals"].as_array().unwrap() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory = Arc::new(Factory(count.clone()));
        let mut store = PresenceStore::<Value, ValueMutation>::new(Value(0));
        store.install_local_retirement_factory(factory.clone()).unwrap();
        store.install_peer_retirement_factory(factory).unwrap();
        let (begun, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| store.begin_peer_publication(grant));
        let (mut publication, progress) = begun.unwrap();
        assert!(progress.fits(grant));
        assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (heap.requested_bytes, heap.released_bytes));
        while publication.prune_one(|_| true, grant).unwrap().0 {}
        let actor_value = law["peer"]["actor"].as_str().unwrap();
        let mut actor = Some(String::with_capacity(65536));
        actor.as_mut().unwrap().push_str(actor_value);
        let actor_pointer = actor.as_ref().unwrap().as_ptr();
        let mut presence = Some(Value(law["peer"]["value"].as_i64().unwrap() as i32));
        let refused = match refusal.as_str().unwrap() { "items" => RetainedCloneGrant { maximum_items: 0, ..grant }, "copy" => RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, "capacity" => RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }, "depth" => RetainedCloneGrant { maximum_depth: 0, ..grant }, _ => grant };
        let running = std::cell::Cell::new(refusal != "nativeCancellation");
        let mut observer = |_: semio_framework_value::native_decoding::NativeDecodeProgress| running.get();
        let mut native = semio_framework_value::NativeDecodeControl::new(law["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut observer);
        let mut original = crate::io::control::NativeSnapshotDecodeOwner::new(&mut native, refused);
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| publication.adopt(&mut actor, &mut presence, law["receivedAtMilliseconds"].as_i64().unwrap(), &mut original));
        assert!(result.is_err());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (law["refusedRequestedBytes"].as_u64().unwrap() as usize, law["refusedReleasedBytes"].as_u64().unwrap() as usize));
        assert_eq!(original.progress(), RetainedCloneProgress::default());
        assert_eq!(original.native().owned_bytes(), 0);
        assert_eq!(actor.as_ref().unwrap().as_ptr(), actor_pointer);
        assert_eq!(presence.as_ref().unwrap().0, law["peer"]["value"].as_i64().unwrap() as i32);
        drop(original);
        running.set(true);
        let mut original = crate::io::control::NativeSnapshotDecodeOwner::new(&mut native, grant);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| publication.adopt(&mut actor, &mut presence, law["receivedAtMilliseconds"].as_i64().unwrap(), &mut original));
        let step = step.unwrap();
        assert!(step.progress().fits(grant));
        assert_eq!((step.progress().retained_capacity_bytes, step.progress().released_bytes), (heap.requested_bytes, heap.released_bytes));
        assert_eq!(original.progress(), step.progress());
        assert!(actor.is_none() && presence.is_none());
        while publication.release_created_one(grant).unwrap().0 {}
        let (commit, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| publication.take_commit(&mut original));
        let (commit, progress) = commit.unwrap();
        assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (heap.requested_bytes, heap.released_bytes));
        assert_eq!(original.native().owned_bytes(), original.progress().retained_capacity_bytes);
        let produced = commit.root.peers().map(|(actor, value)| serde_json::json!({ "actor": actor, "value": value.0 })).collect::<Vec<_>>();
        assert_eq!(produced, vec![law["peer"].clone()]);
        assert_eq!(commit.root.entries[0].as_ref().unwrap().actor.as_ptr(), actor_pointer);
        assert!(publication.terminal_is_empty());
        close_peer_root(store.publish_peer_commit(commit).ok().unwrap().unwrap());
        let mut close = store.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        finish(&mut close);
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
    eprintln!("[DEBUG] original peer actor pointer and typed presence survive every prebirth refusal; actual original native Arc birth and independent receipt conserve allocation and closure");
}

#[test]
fn retained_presence_peer_commit_rejects_foreign_and_stale_roots_without_losing_exact_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📌️peer-commit.json")).unwrap();
    for law in fixture["cases"].as_array().unwrap() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory: Arc<dyn SnapshotRetirementFactory<Value>> = Arc::new(Factory(count.clone()));
        let other_factory: Arc<dyn SnapshotRetirementFactory<Value>> = if law["sameFactory"] == true { factory.clone() } else { Arc::new(Factory(count.clone())) };
        let mut first = PresenceStore::<Value, ValueMutation>::new(Value(17));
        first.install_local_retirement_factory(factory.clone()).unwrap();
        first.install_peer_retirement_factory(factory.clone()).unwrap();
        let mut other = PresenceStore::<Value, ValueMutation>::new(Value(23));
        other.install_local_retirement_factory(other_factory.clone()).unwrap();
        other.install_peer_retirement_factory(other_factory).unwrap();
        let candidate = peer_commit(&first, &fixture["candidate"]);
        let mut displaced = if law["stale"] == true { Some(first.publish_peer_commit(peer_commit(&first, &fixture["winner"])).ok().unwrap().unwrap()) } else { None };
        let blocked = displaced.as_mut().map(|owner| observed_close(owner, CLOSE_GRANT).progress() == RetainedCloneProgress::default());
        let target = if law["sameStore"] == true { &mut first } else { &mut other };
        let before = target.peers_root();
        let result = target.publish_peer_commit(candidate);
        let accepted = result.is_ok();
        let unchanged = Arc::ptr_eq(&before, &target.peers_root());
        drop(before);
        match result {
            Ok(Some(retirement)) => {
                close_peer_root(retirement);
            }
            Ok(None) => panic!("peer commit must hand back its exact displaced root"),
            Err(commit) => {
                let mut retirement = commit.into_retirement();
                for _ in 0..2048 {
                    if matches!(observed_close(&mut retirement, CLOSE_GRANT), RetainedCloneStep::Complete(_)) {
                        break;
                    }
                }
                assert!(retirement.terminal_is_empty());
            }
        }
        if let Some(retirement) = displaced {
            close_peer_root(retirement);
        }
        let mut first = first.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        let mut other = other.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        for owner in [&mut first, &mut other] {
            for _ in 0..2048 {
                if matches!(observed_close(owner, CLOSE_GRANT), RetainedCloneStep::Complete(_)) {
                    break;
                }
            }
            assert!(owner.terminal_is_empty());
        }
        assert_eq!(accepted, law["accepted"].as_bool().unwrap(), "{}", law["name"]);
        assert_eq!(unchanged, !accepted);
        if law["stale"] == true {
            assert_eq!(blocked, Some(true));
        }
        assert_eq!(count.load(std::sync::atomic::Ordering::SeqCst) as u64, law["expectedSnapshots"].as_u64().unwrap());
        eprintln!("[DEBUG] exact peer commit case={} accepted={accepted}, unchanged={unchanged}, base-retirement-blocked={blocked:?}, snapshots={}", law["name"], count.load(std::sync::atomic::Ordering::SeqCst));
    }
}

#[test]
fn retained_presence_store_close_preserves_distinct_original_local_and_peer_factories() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧹️retirement.json")).unwrap();
    let law = &fixture["closeFactoryBinding"];
    let local = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let peer = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let foreign = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(law["local"].as_i64().unwrap() as i32));
    owner.install_local_retirement_factory(Arc::new(Factory(local.clone()))).unwrap();
    owner.install_peer_retirement_factory(Arc::new(Factory(peer.clone()))).unwrap();
    let read = owner.local_read().unwrap();
    let mut publication = owner.begin_peer_publication().unwrap();
    while publication.prune_one(|_| true).unwrap() {}
    publication.adopt(law["peer"]["actor"].as_str().unwrap().into(), Value(law["peer"]["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
    while publication.release_created_one() {}
    close_peer_root(owner.publish_peer_commit(publication.take_commit().unwrap()).ok().unwrap().unwrap());
    let foreign_factory = Arc::new(Factory(foreign.clone()));
    let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
    drop(read);
    for _ in 0..2048 {
        if matches!(observed_close(&mut close, CLOSE_GRANT), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(close.terminal_is_empty());
    let actual = serde_json::json!({ "local": local.load(std::sync::atomic::Ordering::SeqCst), "peer": peer.load(std::sync::atomic::Ordering::SeqCst), "foreign": foreign.load(std::sync::atomic::Ordering::SeqCst) });
    assert_eq!(actual, serde_json::json!({ "local": law["expectedLocal"], "peer": law["expectedPeer"], "foreign": law["expectedForeign"] }));
    assert_eq!(Arc::strong_count(&foreign_factory), 1);
}

#[test]
fn retained_presence_overlapping_rosters_retire_shared_entries_once_across_workers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧹️retirement.json")).unwrap();
    let law = &fixture["overlap"];
    for race in [false, true] {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory = Arc::new(Factory(count.clone()));
        let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(23));
        owner.install_local_retirement_factory(factory.clone()).unwrap();
        owner.install_peer_retirement_factory(factory.clone()).unwrap();
        let mut publication = owner.begin_peer_publication().unwrap();
        while publication.prune_one(|_| true).unwrap() {}
        for peer in law["first"].as_array().unwrap() {
            publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
        }
        while publication.release_created_one() {}
        let commit = publication.take_commit().unwrap();
        assert_eq!(close_peer_root(owner.publish_peer_commit(commit).ok().unwrap().unwrap()), 0);
        let reader = owner.peers_root();
        let mut publication = owner.begin_peer_publication().unwrap();
        while publication.prune_one(|_| true).unwrap() {}
        for peer in law["second"].as_array().unwrap().iter().skip(law["first"].as_array().unwrap().len()) {
            publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).ok().unwrap();
        }
        while publication.release_created_one() {}
        let mut first = owner.publish_peer_commit(publication.take_commit().unwrap()).ok().unwrap().unwrap();
        let mut publication = owner.begin_peer_publication().unwrap();
        while publication.prune_one(|_| false).unwrap() {}
        let second = owner.publish_peer_commit(publication.take_commit().unwrap()).ok().unwrap().unwrap();
        assert!(!owner.retirement_started());
        assert!(owner.peers_root().is_empty());
        let bytes = if race {
            drop(reader);
            let barrier = Arc::new(std::sync::Barrier::new(2));
            let first_barrier = barrier.clone();
            let first = std::thread::spawn(move || {
                first_barrier.wait();
                close_peer_root(first)
            });
            let second = std::thread::spawn(move || {
                barrier.wait();
                close_peer_root(second)
            });
            first.join().unwrap() + second.join().unwrap()
        } else {
            assert_eq!(observed_close(&mut first, CLOSE_GRANT), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            let second_bytes = std::thread::spawn(move || close_peer_root(second)).join().unwrap();
            assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 1);
            let observed = reader.peers().map(|(actor, value)| serde_json::json!({ "actor": actor, "value": value.0 })).collect::<Vec<_>>();
            assert_eq!(serde_json::to_value(observed).unwrap(), law["first"]);
            drop(reader);
            second_bytes + std::thread::spawn(move || close_peer_root(first)).join().unwrap()
        };
        assert_eq!(serde_json::json!(bytes), law["expectedActorBytes"]);
        assert_eq!(serde_json::json!(count.load(std::sync::atomic::Ordering::Relaxed)), law["expectedPeerSnapshots"]);
        let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        for _ in 0..256 {
            if matches!(observed_close(&mut close, CLOSE_GRANT), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(close.terminal_is_empty());
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 3);
        eprintln!("[DEBUG] overlapping immutable peer roots retired each payload once across workers race={race}, actor_bytes={bytes}");
    }
}

#[test]
fn retained_presence_read_return_releases_alias_before_cross_worker_reclamation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧹️retirement.json")).unwrap();
    let text = fixture["readerReturn"]["text"].as_str().unwrap();
    let expected: String = serde_json::from_value(fixture["readerReturn"]["text"].clone()).unwrap();
    assert_eq!(expected, text);
    for variant in fixture["readerReturn"]["variants"].as_array().unwrap() {
        let registry = crate::os_store::SnapshotReadRegistryHandle::new();
        let root = Arc::new(text.to_string());
        let lease = registry.try_issue(root.clone()).unwrap();
        let variant = variant.as_str().unwrap().to_string();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let worker_barrier = barrier.clone();
        let worker = std::thread::spawn(move || {
            worker_barrier.wait();
            match variant.as_str() {
                "typed-drop" => { drop(SnapshotRead::new(root, lease)); None }
                "erased-drop" => { drop(ErasedSnapshotRead::new(root, lease)); None }
                "explicit-return" => { assert!(SnapshotRead::new(root, lease).return_to_registry()); None }
                "witness-return" => Some(SnapshotRead::new(root, lease).return_to_registry_witness().unwrap()),
                _ => unreachable!(),
            }
        });
        barrier.wait();
        let mut final_owner = None;
        for _ in 0..100_000 {
            final_owner = admit_returned_string(&registry, text);
            if final_owner.is_some() { break; }
            std::thread::yield_now();
        }
        let witness = worker.join().unwrap();
        assert!(final_owner.is_some());
        finish_box(&mut final_owner);
        if let Some(witness) = witness {
            let mut close = ControlledRetirement::new(witness).unwrap_or_else(|_| panic!("original return witness"));
            finish(&mut close);
        }
        assert!(registry.terminal_is_empty());
        finish_registry(registry);
    }
}

#[test]
fn retained_presence_read_return_injected_alias_barrier_preserves_unreturned_guard() {
    let registry = crate::os_store::SnapshotReadRegistryHandle::new();
    let root = Arc::new(String::from("aä🧵"));
    let lease = registry.try_issue(root.clone()).unwrap();
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let worker_barrier = barrier.clone();
    let worker = std::thread::spawn(move || {
        let mut owner = Some(root);
        let mut lease = Some(lease);
        assert!(return_snapshot_read(&mut owner, &mut lease, || { worker_barrier.wait(); worker_barrier.wait(); }));
    });
    barrier.wait();
    assert!(!registry.has_returned());
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY { assert!(admit_returned_string(&registry, "aä🧵").is_none()); }
    assert!(!registry.terminal_is_empty());
    barrier.wait();
    worker.join().unwrap();
    let mut final_owner = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        final_owner = admit_returned_string(&registry, "aä🧵");
        if final_owner.is_some() { break; }
    }
    assert!(final_owner.is_some());
    finish_box(&mut final_owner);
    assert!(registry.terminal_is_empty());
    finish_registry(registry);
}

#[test]
fn retained_presence_read_transfer_contention_preserves_unreturned_capability() {
    let registry = crate::os_store::SnapshotReadRegistryHandle::new();
    let root = Arc::new(String::from("aä🧵"));
    let lease = registry.try_issue(root.clone()).unwrap();
    let read = ErasedSnapshotRead::new(root, lease);
    let held = registry.state.try_lock().unwrap();
    let worker_registry = registry.clone();
    let read = match std::thread::spawn(move || read.into_typed::<String>(&worker_registry)).join().unwrap() {
        Err(read) => read,
        Ok(_) => panic!("contended transfer cannot detach its unreturned guard"),
    };
    assert!(!registry.has_returned());
    let lease = read.lease.as_ref().unwrap();
    assert_eq!(registry.lease_generations[lease.index as usize].load(std::sync::atomic::Ordering::Acquire), lease.generation);
    drop(held);
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let worker_barrier = barrier.clone();
    let worker_registry = registry.clone();
    let worker = std::thread::spawn(move || {
        worker_barrier.wait();
        for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY { assert!(admit_returned_string(&worker_registry, "aä🧵").is_none()); }
        finish_registry(worker_registry);
    });
    barrier.wait();
    let mut read = read;
    let mut owner = None;
    for _ in 0..100_000 {
        match read.into_typed::<String>(&registry) {
            Ok(root) => { owner = Some(root); break; }
            Err(retained) => read = retained,
        }
        std::thread::yield_now();
    }
    worker.join().unwrap();
    assert!(!registry.has_returned());
    let original = owner.expect("exact transfer retries after bounded lock contention");
    assert_eq!(original.as_str(), "aä🧵");
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::retirement::shared::admit_shared_retirement(original, CLOSE_GRANT, false));
    let (owner, receipt) = result.unwrap_or_else(|_| panic!("original transferred String"));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
    finish_box(&mut Some(owner));
    assert!(registry.terminal_is_empty());
    finish_registry(registry);
}

#[test]
fn retained_presence_store_close_keeps_captured_readers_and_retires_nonempty_peers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧹️retirement.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory = Arc::new(Factory(count.clone()));
        let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(case["local"].as_i64().unwrap() as i32));
        owner.install_local_retirement_factory(factory.clone()).unwrap();
        owner.install_peer_retirement_factory(factory.clone()).unwrap();
        let mut publication = owner.begin_peer_publication().unwrap();
        assert!(!publication.prune_one(|_| true).unwrap());
        for peer in case["peers"].as_array().unwrap() {
            assert!(publication.adopt(peer["actor"].as_str().unwrap().into(), Value(peer["value"].as_i64().unwrap() as i32), 0).is_ok());
        }
        while publication.release_created_one() {}
        let commit = publication.take_commit().unwrap();
        assert_eq!(close_peer_root(owner.publish_peer_commit(commit).ok().unwrap().unwrap()), 0);
        assert!(publication.terminal_is_empty());
        let shared = case["sharedReaders"].as_bool().unwrap();
        let mut local_reader = shared.then(|| owner.local_read().unwrap());
        let mut peer_reader = shared.then(|| owner.peers_root());
        let local_pointer = Arc::as_ptr(owner.local.as_ref().unwrap());
        let peers_pointer = Arc::as_ptr(owner.peers.as_ref().unwrap());
        let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
        assert!(owner.retirement_started());
        assert!(owner.local.is_none() && owner.peers.is_none());
        assert_eq!(Arc::as_ptr(close.local.as_ref().unwrap()), local_pointer);
        assert_eq!(Arc::as_ptr(close.peers.as_ref().unwrap()), peers_pointer);
        assert!(owner.apply_one(0, ValueMutation::SetValue(SetValue { n: 1 })).is_err());
        assert!(owner.begin_peer_publication().is_err());
        assert_eq!(observed_close(&mut close, RetainedCloneGrant { maximum_items: 0, ..CLOSE_GRANT }), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        let mut released = 0;
        let mut blocked_local = false;
        let mut blocked_peers = false;
        for turn in 0..2048 {
            let step = observed_close(&mut close, CLOSE_GRANT);
            released += step.progress().released_bytes;
            if matches!(step, RetainedCloneStep::Complete(_)) { break; }
            if step.progress() == RetainedCloneProgress::default() {
                if let Some(reader) = local_reader.take() {
                    assert_eq!(reader.get().0, case["local"].as_i64().unwrap() as i32);
                    drop(reader);
                    blocked_local = true;
                } else if let Some(reader) = peer_reader.take() {
                    let actual = serde_json::to_value(reader.peers().map(|(actor, value)| serde_json::json!({ "actor": actor, "value": value.0 })).collect::<Vec<_>>()).unwrap();
                    assert_eq!(actual, case["peers"]);
                    drop(reader);
                    blocked_peers = true;
                } else { panic!("unshared presence root failed to progress"); }
            }
            assert!(turn < 2047);
        }
        assert!(close.terminal_is_empty());
        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), case["expectedSnapshots"].as_u64().unwrap() as usize);
        let actor_bytes = case["peers"].as_array().unwrap().iter().map(|peer| serde_json::from_value::<String>(peer["actor"].clone()).unwrap().len()).sum::<usize>();
        assert_eq!(actor_bytes, case["expectedActorBytes"].as_u64().unwrap() as usize);
        assert!(released > actor_bytes);
        assert_eq!((blocked_local, blocked_peers), (shared, shared));
        eprintln!("[DEBUG] original Presence roots transfer allocation identity into retirement; Store detaches both Options and rejects late writes; captured readers retain original values until physical close");
    }
}

#[test]
fn retained_presence_store_close_rejects_nonempty_terminal_and_late_commit_without_drop() {
    let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let factory = Arc::new(Factory(count.clone()));
    let mut owner = PresenceStore::<Value, ValueMutation>::new(Value(7));
    let empty = Arc::new(Value(0));
    let missing = owner.begin_retirement(empty.clone(), |value| value.0 == 0).err().unwrap();
    assert_eq!(missing.0, "presence close requires its installed local-root retirement factory");
    assert!(Arc::ptr_eq(&missing.1, &empty));
    assert!(!owner.retirement_started());
    drop((missing, empty));
    owner.install_local_retirement_factory(factory.clone()).unwrap();
    owner.install_peer_retirement_factory(factory).unwrap();
    let terminal = Arc::new(Value(9));
    let rejected = owner.begin_retirement(terminal.clone(), |value| value.0 == 0).err().unwrap();
    assert!(Arc::ptr_eq(&terminal, &rejected.1));
    assert!(!owner.retirement_started());
    drop((terminal, rejected));
    let mut publication = owner.begin_peer_publication().unwrap();
    assert!(!publication.prune_one(|_| true).unwrap());
    assert!(publication.adopt("late".into(), Value(11), 0).is_ok());
    assert!(publication.release_created_one());
    let commit = publication.take_commit().unwrap();
    let mut close = owner.begin_retirement(Arc::new(Value(0)), |value| value.0 == 0).ok().unwrap();
    let rejected = owner.publish_peer_commit(commit).err().expect("closed store preserves the exact rejected commit");
    assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 0);
    let mut rejected = rejected.into_retirement();
    for turn in 0..128 {
        for cursor in [&mut close, &mut rejected] {
            observed_close(cursor, CLOSE_GRANT);
        }
        if close.terminal_is_empty() && rejected.terminal_is_empty() {
            break;
        }
        assert!(turn < 127);
    }
    assert!(close.terminal_is_empty() && rejected.terminal_is_empty());
    assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 2);
}


#[test]
fn original_presence_examples_supply_independent_physical_close_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧹️retirement.json")).unwrap();
    assert_eq!(serde_json::json!({ "maximumItems": CLOSE_GRANT.maximum_items, "maximumCopyBytes": CLOSE_GRANT.maximum_copy_bytes, "maximumCapacityBytes": CLOSE_GRANT.maximum_capacity_bytes, "maximumReleaseBytes": CLOSE_GRANT.maximum_release_bytes, "maximumDepth": CLOSE_GRANT.maximum_depth }), fixture["physicalCloseGrant"]);
    assert_eq!(CLOSE_GRANT.maximum_copy_bytes, fixture["maximumBytes"].as_u64().unwrap() as usize);
    assert!(CLOSE_GRANT.maximum_release_bytes > fixture["localCapture"]["workerPageBytes"].as_u64().unwrap() as usize);
}
