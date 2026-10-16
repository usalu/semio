use super::*;

use db_engine_base::test_worker_pool;
use db_io_core::{PayloadStorage as _, WalStorage as _};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlledCapabilityPoll {
    Pending,
    Ready,
    Panic,
}

struct ControlledCapabilityFuture {
    pub mode: ControlledCapabilityPoll,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub polls: Arc<std::sync::atomic::AtomicUsize>,
    pub waker: Arc<Mutex<Option<std::task::Waker>>>,
    boundary: Option<Arc<(Mutex<(bool, bool)>, std::sync::Condvar)>>,
    wake_during_poll: bool,
}

struct ControlledCapabilitySubmitQueue {
    pub slots: [Option<semio_framework_async::Job>; 8],
    pub head: usize,
    pub len: usize,
}

struct ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize);

impl std::task::Wake for ControlledCatalogPublicWake {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

impl ControlledCapabilitySubmitQueue {
    pub fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0 }
    }

    pub fn push(&mut self, job: semio_framework_async::Job) -> Result<(), semio_framework_async::Job> {
        if self.len == self.slots.len() {
            return Err(job);
        }
        let index = (self.head + self.len) % self.slots.len();
        self.slots[index] = Some(job);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<semio_framework_async::Job> {
        if self.len == 0 {
            return None;
        }
        let job = self.slots[self.head].take();
        self.head = (self.head + 1) % self.slots.len();
        self.len -= 1;
        job
    }
}

impl Future for ControlledCapabilityFuture {
    type Output = DatabaseCapabilityOpenResult;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let poll = self.polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        if self.wake_during_poll && poll == 0 {
            context.waker().wake_by_ref();
        }
        if let Some(boundary) = &self.boundary {
            let (state, ready) = &**boundary;
            let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.0 = true;
            ready.notify_one();
            while !state.1 {
                state = ready.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }
        match self.mode {
            ControlledCapabilityPoll::Pending => std::task::Poll::Pending,
            ControlledCapabilityPoll::Ready => std::task::Poll::Ready(DatabaseCapabilityOpenResult {
                storage: self.storage.take().expect("controlled Ready storage owner"),
                capabilities: db_io_core::StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true },
            }),
            ControlledCapabilityPoll::Panic => panic!("controlled capability-open poll panic"),
        }
    }
}

async fn controlled_capability_probe(
    mode: ControlledCapabilityPoll,
    boundary: Option<Arc<(Mutex<(bool, bool)>, std::sync::Condvar)>>,
) -> (DatabaseCapabilityOpenFuture, Arc<std::sync::atomic::AtomicUsize>, Arc<Mutex<Option<std::task::Waker>>>, usize) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage.clone(), false).expect("controlled capability-open preparation");
    let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let waker = Arc::new(Mutex::new(None));
    let future = ControlledCapabilityFuture { mode, storage: Some(storage), polls: polls.clone(), waker: waker.clone(), boundary, wake_during_poll: true };
    *probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCapabilityOpenWork::controlled(Box::pin(future), pointer));
    let work = probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("controlled poll owner");
    *probe.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
    probe.state.set_phase(DatabaseCapabilityOpenPhase::Poll);
    (probe, polls, waker, pointer)
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_paused_transfer_blocks_public_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["driveOwnership"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let storage_weak = Arc::downgrade(&storage);
        let probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).unwrap();
        let state = probe.state.clone();
        let phase = match row["phase"].as_str().unwrap() {
            "Handoff" => DatabaseCapabilityOpenPhase::Handoff,
            "RetainWork" => DatabaseCapabilityOpenPhase::RetainWork,
            "Poll" => DatabaseCapabilityOpenPhase::Poll,
            _ => unreachable!(),
        };
        if phase == DatabaseCapabilityOpenPhase::Poll {
            *state.poll_work.lock().unwrap() = state.work.lock().unwrap().take();
        }
        state.set_phase(phase);
        let submitted = control_capability_submissions(&state);
        state.schedule();
        let initial = submitted.lock().unwrap().pop().unwrap();
        let submissions = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let count = submissions.clone();
        *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| {
            count.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            Err(job)
        }));
        *state.controlled_submit_refusal.lock().unwrap() = Some(semio_framework_async::WorkerSubmitErrorKind::Shutdown);
        let (claimed, observed) = std::sync::mpsc::sync_channel(1);
        let (release, released) = std::sync::mpsc::sync_channel(1);
        *state.controlled_owner_transfer_hook.lock().unwrap() = Some(Box::new(move |identity| {
            claimed.send(identity).unwrap();
            released.recv_timeout(std::time::Duration::from_secs(5)).expect("transfer release deadline");
        }));
        let driver = std::thread::spawn(initial);
        let claimed_pointer = observed.recv_timeout(std::time::Duration::from_secs(5)).expect("transfer claim deadline");
        state.schedule();
        state.schedule();
        let submissions_while_active = submissions.load(std::sync::atomic::Ordering::Acquire);
        drop(probe);
        let terminal = take_database_capability_open_terminal(state.generation).unwrap();
        let terminal = match terminal.resume() {
            Err(terminal) => terminal,
            Ok(resumed) => {
                drop(resumed);
                panic!("active transfer must return the unchanged terminal cursor");
            }
        };
        let first = terminal.close_step();
        for _ in 0..64 {
            if terminal.close_step() == DatabaseCapabilityOpenCloseStep::Complete {
                break;
            }
        }
        let retained = state.admission.lock().unwrap().is_some() && database_capability_open_registry().lock().unwrap()[state.slot].as_ref().is_some_and(|entry| Arc::ptr_eq(entry, &state));
        let exact_storage = claimed_pointer == pointer && storage_weak.upgrade().is_some_and(|storage| Arc::as_ptr(&storage) as usize == pointer);
        release.send(()).unwrap();
        driver.join().unwrap();
        *state.controlled_submit_hook.lock().unwrap() = None;
        for _ in 0..64 {
            if terminal.close_step() == DatabaseCapabilityOpenCloseStep::Complete {
                break;
            }
        }
        let empty = terminal.terminal_is_empty() && database_capability_open_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state));
        assert_eq!(first == DatabaseCapabilityOpenCloseStep::Blocked, row["closeBlocked"].as_bool().unwrap(), "a live stack-local transfer must exclude public cleanup");
        assert_eq!(retained, row["admissionRetained"].as_bool().unwrap(), "live drive cannot release its admission or registry identity");
        assert_eq!(exact_storage, row["storageRetained"].as_bool().unwrap());
        assert!(empty, "ownership hand-back must converge after the drive returns");
        assert_eq!(submissions_while_active, 0, "active transfer must defer all successor submissions");
        assert_eq!(submissions.load(std::sync::atomic::Ordering::Acquire), 1, "two requests and cancellation share one exact successor");
        assert!(storage_weak.upgrade().is_none(), "exact storage owner must be released after cleanup");
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_lease_successors_and_active_publication_retire_once() {
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["leaseCompletion"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).unwrap();
        let state = probe.state.clone();
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
        let waker = std::task::Waker::from(wake);
        let mut context = std::task::Context::from_waker(&waker);
        assert!(std::pin::Pin::new(&mut probe).poll(&mut context).is_pending());
        let active = row["publication"] != "synchronous";
        let after_check = row["publication"].as_str().unwrap().starts_with("after-finalizer-check");
        let refuse_finalizer = row["publication"] == "after-finalizer-check-refused";
        let mut publisher = None;
        let mut release_publisher = None;
        if active {
            let submitted = control_capability_submissions(&state);
            state.schedule();
            for _ in 0..32 {
                if state.phase() == DatabaseCapabilityOpenPhase::Publish {
                    break;
                }
                let job = submitted.lock().unwrap().pop().unwrap();
                job();
            }
            assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::Publish);
            let publish = submitted.lock().unwrap().pop().unwrap();
            assert!(submitted.lock().unwrap().pop().is_none());
            let (published, observed) = std::sync::mpsc::sync_channel(1);
            let (release, released) = std::sync::mpsc::sync_channel(1);
            let hook: Box<dyn FnOnce() + Send> = Box::new(move || {
                published.send(()).unwrap();
                released.recv_timeout(std::time::Duration::from_secs(5)).expect("active publisher release deadline");
            });
            if after_check {
                *state.controlled_lease_release_hook.lock().unwrap() = Some(hook);
            } else {
                *state.controlled_completion_before_wake_hook.lock().unwrap() = Some(hook);
            }
            publisher = Some(std::thread::spawn(publish));
            release_publisher = Some(release);
            observed.recv_timeout(std::time::Duration::from_secs(5)).expect("active completion deadline");
            if refuse_finalizer {
                *state.controlled_submit_refusal.lock().unwrap() = Some(semio_framework_async::WorkerSubmitErrorKind::Shutdown);
            }
            let late_count = count.clone();
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| {
                late_count.fetch_add(1, Ordering::AcqRel);
                if refuse_finalizer {
                    return Err(job);
                }
                job();
                Ok(())
            }));
            if !after_check {
                state.schedule();
                state.schedule();
            }
        } else {
            let submitted_count = count.clone();
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| {
                assert!(submitted_count.fetch_add(1, Ordering::AcqRel) < 32, "synchronous phase budget");
                job();
                Ok(())
            }));
            state.schedule();
        }
        let completed = std::pin::Pin::new(&mut probe).poll(&mut context);
        let retained = state.admission.lock().unwrap().is_some();
        let before_retirement = count.load(Ordering::Acquire);
        if let Some(release) = release_publisher {
            release.send(()).unwrap();
        }
        if let Some(publisher) = publisher {
            publisher.join().unwrap();
        }
        let std::task::Poll::Ready(Ok(result)) = completed else { panic!("leased completion must remain consumable") };
        assert_eq!(Arc::as_ptr(&result.into_parts().0) as usize, pointer);
        assert_eq!(retained, row["admissionDuringPublication"].as_bool().unwrap());
        let retirement_submissions = count.load(Ordering::Acquire) - before_retirement;
        assert_eq!(retirement_submissions, row["retirementSubmissions"].as_u64().unwrap() as usize);
        assert!(state.terminal_is_empty());
        assert!(state.controlled_submit_refusal.lock().unwrap().is_none());
        assert!(state.terminal_job.lock().unwrap().is_none());
        assert!(state.completion.lock().unwrap().is_none());
        assert!(database_capability_open_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state)));
        assert_eq!(state.lease.load(Ordering::Acquire), DATABASE_CAPABILITY_OPEN_CLOSED);
        let before_late = count.load(Ordering::Acquire);
        state.schedule();
        state.schedule_cleanup();
        assert_eq!(count.load(Ordering::Acquire) - before_late, row["lateSubmissions"].as_u64().unwrap() as usize);
        if active {
            assert_eq!(before_late, row["retirementSubmissions"].as_u64().unwrap() as usize, "a consumed completion transfers only its required retirement successor");
        } else {
            assert!(before_late > 1, "real successors must have reentered the submitter");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_completion_interleavings_preserve_result_and_wake() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["publication"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).unwrap();
        let state = probe.state.clone();
        let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
        let waker = std::task::Waker::from(wake.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let mut work = state.work.lock().unwrap().take().unwrap();
        let std::task::Poll::Ready(result) = work.poll(&mut context) else { panic!("scalar capability must be ready") };
        assert!(work.close_step() && work.terminal_is_empty());
        drop(work);
        let fault = row["outcome"] == "fault";
        let result = if fault {
            drop(result);
            Err(DbError::Unavailable("fixture-publication-fault".into()))
        } else {
            Ok(result)
        };
        let publish_state = state.clone();
        let publish = move || publish_state.complete(result, if fault { DatabaseCapabilityOpenProgress::Fault } else { DatabaseCapabilityOpenProgress::Completed });
        let publication = row["publication"].as_str().unwrap();
        let mut deferred = None;
        match publication {
            "before-poll" => publish(),
            "before-registration" => *state.controlled_publication_before_waker_hook.lock().unwrap() = Some(Box::new(publish)),
            "after-pending" => deferred = Some(publish),
            _ => unreachable!(),
        }
        let first = std::pin::Pin::new(&mut probe).poll(&mut context);
        let first_ready = first.is_ready();
        if let Some(publish) = deferred {
            publish();
        }
        let completed = match first {
            std::task::Poll::Ready(result) => result,
            std::task::Poll::Pending => match std::pin::Pin::new(&mut probe).poll(&mut context) {
                std::task::Poll::Ready(result) => result,
                _ => panic!("published completion must be consumable"),
            },
        };
        match completed {
            Ok(result) => {
                assert!(!fault);
                let (storage, capabilities) = result.into_parts();
                assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
                assert!(!capabilities.durable);
            }
            Err(error) => {
                assert!(fault);
                assert_eq!(error, DbError::Unavailable("fixture-publication-fault".into()));
            }
        }
        let abandoned = state.abandoned.load(std::sync::atomic::Ordering::Acquire);
        let waiter_empty = state.waker.lock().unwrap().is_none();
        for _ in 0..32 {
            if state.terminal_is_empty() {
                break;
            }
            let _ = state.close_step();
        }
        assert!(state.terminal_is_empty(), "fixture cleanup must return exact admission");
        assert!(database_capability_open_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state)), "exact capability registry slot must be released");
        eprintln!("capability-completion: {} first-ready={first_ready} wakes={} waiter-empty={waiter_empty} abandoned={abandoned}", row["name"], wake.0.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(first_ready, row["firstReady"].as_bool().unwrap(), "completion in check-to-registration window must be observed in the same poll");
        assert_eq!(wake.0.load(std::sync::atomic::Ordering::Acquire) as u64, row["wakes"].as_u64().unwrap());
        assert!(waiter_empty, "Ready must retire its transient waiter");
        assert_eq!(abandoned, fault, "fault bookkeeping must match the fast Ready path");
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_consumed_completion_retires_before_publisher_wake() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["retirement"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).unwrap();
        let state = probe.state.clone();
        let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
        let waker = std::task::Waker::from(wake.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let mut work = state.work.lock().unwrap().take().unwrap();
        let std::task::Poll::Ready(result) = work.poll(&mut context) else { panic!("scalar capability must be ready") };
        assert!(work.close_step() && work.terminal_is_empty());
        drop(work);
        assert!(std::pin::Pin::new(&mut probe).poll(&mut context).is_pending());
        let fault = row["outcome"] == "fault";
        let result = if fault {
            drop(result);
            Err(DbError::Unavailable("fixture-publication-fault".into()))
        } else {
            Ok(result)
        };
        let (published, observed) = std::sync::mpsc::sync_channel(1);
        let (release, released) = std::sync::mpsc::sync_channel(1);
        *state.controlled_completion_before_wake_hook.lock().unwrap() = Some(Box::new(move || {
            published.send(()).unwrap();
            released.recv_timeout(std::time::Duration::from_secs(5)).expect("publisher release deadline");
        }));
        let publisher_state = state.clone();
        let publisher = std::thread::spawn(move || publisher_state.complete(result, if fault { DatabaseCapabilityOpenProgress::Fault } else { DatabaseCapabilityOpenProgress::Completed }));
        observed.recv_timeout(std::time::Duration::from_secs(5)).expect("physical publication deadline");
        let completed = std::pin::Pin::new(&mut probe).poll(&mut context);
        let terminal_before_wake = state.terminal_is_empty() && database_capability_open_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state));
        release.send(()).unwrap();
        publisher.join().unwrap();
        let std::task::Poll::Ready(result) = completed else { panic!("visible publication must return Ready") };
        match result {
            Ok(result) => {
                assert!(!fault);
                assert_eq!(Arc::as_ptr(&result.into_parts().0) as usize, pointer);
            }
            Err(error) => {
                assert!(fault);
                assert_eq!(error, DbError::Unavailable("fixture-publication-fault".into()));
            }
        }
        for _ in 0..32 {
            if state.terminal_is_empty() {
                break;
            }
            let _ = state.close_step();
        }
        assert!(state.terminal_is_empty(), "fixture cleanup must return exact admission");
        assert!(database_capability_open_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state)), "exact capability registry slot must be released");
        eprintln!("capability-completion: {} terminal-before-wake={terminal_before_wake} wakes={}", row["name"], wake.0.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(terminal_before_wake, row["terminalBeforePublisherWake"].as_bool().unwrap(), "Ready must not strand admission behind an already-consumed completion");
        assert_eq!(wake.0.load(std::sync::atomic::Ordering::Acquire) as u64, row["wakes"].as_u64().unwrap());
    }
}

#[test]
fn database_capability_open_fixed_admission_cap_plus_one_and_generation_aba() {
    let mut state = DatabaseCapabilityOpenAdmissionState::empty();
    assert!(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS + 1, DATABASE_CAPABILITY_OPEN_BYTES).is_err());
    assert!(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES + 1).is_err());
    let mut claims = Vec::with_capacity(DATABASE_CAPABILITY_OPEN_SLOTS);
    for _ in 0..DATABASE_CAPABILITY_OPEN_SLOTS {
        claims.push(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES).expect("fixed capability-open admission"));
    }
    assert!(state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES).is_err());
    assert_eq!(state.items, DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS);
    assert_eq!(state.bytes, DATABASE_CAPABILITY_OPEN_TOTAL_BYTES);
    let (slot, generation) = claims.remove(0);
    assert!(state.release(slot, generation, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES));
    let replacement = state.try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES).expect("released fixed slot is reusable");
    assert_eq!(replacement.0, slot);
    assert_ne!(replacement.1, generation);
    assert!(!state.release(slot, generation, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES), "stale generation cannot release the replacement");
    assert!(state.release(replacement.0, replacement.1, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES));
    for (slot, generation) in claims {
        assert!(state.release(slot, generation, DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES));
    }
    assert_eq!((state.items, state.bytes), (0, 0));
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_success_returns_exact_storage_owner_and_scalar() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage);
    let probe = match DatabaseCapabilityOpenFuture::try_submit(test_worker_pool(), storage) {
        Ok(probe) => probe,
        Err(_) => panic!("fixed capability-open admission unexpectedly rejected"),
    };
    let output = probe.await.expect("retained capability probe");
    let (storage, capabilities) = output.into_parts();
    assert_eq!(Arc::as_ptr(&storage), pointer);
    assert!(!capabilities.durable);
    assert_eq!(capabilities.max_durability, DurabilityClass::Memory);
}

fn control_capability_submissions(state: &Arc<DatabaseCapabilityOpenState>) -> Arc<Mutex<ControlledCapabilitySubmitQueue>> {
    let submitted = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let queue = submitted.clone();
    *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
    submitted
}

fn drain_controlled_capability_terminal(terminal: &DatabaseCapabilityOpenTerminalHandle, submitted: &Mutex<ControlledCapabilitySubmitQueue>) {
    for _ in 0..64 {
        let job = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop();
        if let Some(job) = job {
            let before = terminal.state.retained_owner_count();
            job();
            let after = terminal.state.retained_owner_count();
            assert!(before.saturating_sub(after) <= 1, "one actor grant releases at most one capability-open owner");
        }
        let before = terminal.state.retained_owner_count();
        let step = terminal.close_step();
        let after = terminal.state.retained_owner_count();
        assert!(before.saturating_sub(after) <= 1, "one public close grant releases at most one capability-open owner");
        assert!(after <= before, "public close cannot create a new retained owner");
        if step == DatabaseCapabilityOpenCloseStep::Complete {
            break;
        }
    }
    assert!(terminal.terminal_is_empty(), "bounded controlled cleanup must converge");
    assert!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().is_none(), "cleanup cannot strand a successor");
    assert!(terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[terminal.state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &terminal.state)));
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).expect("fixed capability-open preparation");
    let submitted = control_capability_submissions(&probe.state);
    let generation = probe.generation();
    assert_eq!(probe.retained_storage_identity(), Some(pointer));
    probe.cancel();
    assert_eq!(probe.progress(), DatabaseCapabilityOpenProgress::Cancelled);
    assert_eq!(probe.retained_storage_identity(), Some(pointer));
    drop(probe);
    let terminal = take_database_capability_open_terminal(generation).expect("cancelled capability-open terminal authority");
    drain_controlled_capability_terminal(&terminal, &submitted);

    let stale_storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let stale_pointer = Arc::as_ptr(&stale_storage) as usize;
    let stale = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), stale_storage, false).expect("fixed stale capability-open preparation");
    let submitted = control_capability_submissions(&stale.state);
    let stale_generation = stale.generation();
    stale.state.schedule();
    let initial = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
    {
        let mut admission = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        admission.slots[stale.state.slot].generation = stale_generation.checked_add(1).expect("fixture generation");
    }
    initial();
    assert_eq!(stale.progress(), DatabaseCapabilityOpenProgress::Fault);
    assert_eq!(stale.retained_storage_identity(), Some(stale_pointer));
    {
        let mut admission = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        admission.slots[stale.state.slot].generation = stale_generation;
    }
    drop(stale);
    let terminal = take_database_capability_open_terminal(stale_generation).expect("stale capability-open terminal authority");
    drain_controlled_capability_terminal(&terminal, &submitted);
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal() {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (started_tx, started_rx) = std::sync::mpsc::sync_channel(1);
    let worker_gate = gate.clone();
    admit_fixture_blocker(&pool,
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).expect("fixture start handoff");
            let (lock, ready) = &*worker_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    );
    started_rx.recv().expect("fixture worker entered blocker");
    loop {
        match pool.try_submit(Lane::Io, Box::new(|| {})) {
            Ok(()) => {}
            Err(error) => {
                assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
                drop(error.into_job());
                break;
            }
        }
    }
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCapabilityOpenFuture::try_submit(pool.clone(), storage).expect("capability operation admission remains independent of queue saturation");
    assert_eq!(probe.retained_storage_identity(), Some(pointer));
    assert!(probe.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "the exact saturated job remains retained for generation-keyed retry");
    let generation = probe.generation();
    drop(probe);
    let terminal = take_database_capability_open_terminal(generation).expect("abandoned saturated capability-open authority remains public");
    let resumed = match terminal.resume() {
        Ok(resumed) => resumed,
        Err(_) => panic!("public terminal authority failed to resume the exact retained retry job"),
    };
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let output = resumed.await.expect("resumed capability probe");
    assert_eq!(Arc::as_ptr(&output.into_parts().0) as usize, pointer);
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary() {
    for mode in [ControlledCapabilityPoll::Pending, ControlledCapabilityPoll::Ready, ControlledCapabilityPoll::Panic] {
        let (mut probe, polls, retained_waker, _) = controlled_capability_probe(mode, None).await;
        probe.resolved = true;
        let state = probe.state.clone();
        state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        let submitted = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
        let submitted_hook = submitted.clone();
        *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| submitted_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
        state.schedule();
        assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "one initial governed callback");
        let initial = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("initial controlled capability callback");
        initial();
        assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1, "one governed poll opportunity");
        assert!(!state.polling.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        match mode {
            ControlledCapabilityPoll::Pending => {
                assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::Poll);
                assert_eq!(state.progress.load(std::sync::atomic::Ordering::Acquire), DatabaseCapabilityOpenProgress::Pending as u8);
            }
            ControlledCapabilityPoll::Ready => {
                assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::RetainWork);
                assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            }
            ControlledCapabilityPoll::Panic => {
                assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::RetainWork);
                assert!(state.cancelled.load(std::sync::atomic::Ordering::Acquire));
                assert!(state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            }
        }
        assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "during-poll wake coalesces with the exact phase successor");
        retained_waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().expect("controlled backend retained its real waker").wake_by_ref();
        assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "wake-after-release cannot admit a duplicate successor");
        let successor = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("one controlled successor callback");
        successor();
        if mode == ControlledCapabilityPoll::Pending {
            assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 2, "Pending is repolled only by its next governed successor");
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
        } else {
            assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1, "terminal Ready/panic successor advances cleanup without repolling");
            assert!(state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "terminal successor retains the exact work owner");
            if mode == ControlledCapabilityPoll::Ready {
                assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(), "Ready successor retains the exact result owner");
            }
        }
        loop {
            let Some(job) = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop() else { break };
            job();
        }
        while !state.terminal_is_empty() {
            let _ = state.close_step();
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result() {
    for stale in [false, true] {
        let (mut probe, polls, _, pointer) = controlled_capability_probe(ControlledCapabilityPoll::Ready, None).await;
        let state = probe.state.clone();
        probe.resolved = true;
        state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        let submitted = control_capability_submissions(&state);
        state.schedule();
        let initial = submitted.lock().unwrap().pop().unwrap();
        if stale {
            let slot = state.slot;
            let generation = state.generation;
            *state.poll_publication_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
                let mut admission = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                admission.slots[slot].generation = generation.checked_add(1).expect("controlled stale generation");
            }));
        } else {
            let cancel_state = state.clone();
            *state.poll_publication_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
                DatabaseCapabilityOpenFuture { state: cancel_state, resolved: true }.cancel();
            }));
        }
        initial();
        assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
        assert_eq!(state.phase(), DatabaseCapabilityOpenPhase::RetainWork);
        assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        assert!(state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        if stale {
            DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = state.generation;
        }
        for _ in 0..32 {
            let job = submitted.lock().unwrap().pop();
            let Some(job) = job else { break };
            job();
        }
        assert!(submitted.lock().unwrap().pop().is_none());
        assert!(state.terminal_result.lock().unwrap().is_some());
        let terminal = take_database_capability_open_terminal(state.generation).expect("post-Ready terminal authority");
        let checked_out = terminal.take_result().expect("post-Ready result checkout");
        drop(checked_out);
        let resumed = match terminal.take_result().expect("post-Ready result handback").resume() {
            Ok(resumed) => resumed,
            Err(_) => panic!("post-Ready exact result resume rejected"),
        };
        let output = resumed.await.expect("post-Ready retained output");
        assert_eq!(Arc::as_ptr(&output.into_parts().0) as usize, pointer);
        while !terminal.terminal_is_empty() {
            let _ = terminal.close_step();
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_rejection_take_retry_and_close_preserve_exact_storage() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage);
    let rejected = DatabaseCapabilityOpenRejected { error: Some(DbError::Closed), storage: Some(storage) };
    let mut resumed = rejected.retry(test_worker_pool()).expect("rejected storage retry");
    assert_eq!(resumed.retained_storage_identity(), Some(pointer as usize));
    resumed.cancel();
    let generation = resumed.generation();
    drop(resumed);
    let terminal = take_database_capability_open_terminal(generation).expect("retried rejection terminal");
    while !terminal.terminal_is_empty() {
        let _ = terminal.close_step();
    }

    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage);
    let mut rejected = DatabaseCapabilityOpenRejected { error: Some(DbError::Closed), storage: Some(storage) };
    let returned = rejected.take_storage().expect("exact rejected storage take");
    assert_eq!(Arc::as_ptr(&returned), pointer);
    rejected.storage = Some(returned);
    assert_eq!(rejected.close_step(), DatabaseCapabilityOpenCloseStep::Progress);
    assert!(rejected.terminal_is_empty());
    assert_eq!(rejected.into_error_after_close().expect("closed rejection error"), DbError::Closed);
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage);
    let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage.clone(), false).expect("terminal-result preparation");
    probe.resolved = true;
    probe.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
    *probe.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
        Some(Ok(DatabaseCapabilityOpenResult { storage, capabilities: db_io_core::StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true } }));
    let terminal = DatabaseCapabilityOpenTerminalHandle { state: probe.state.clone() };
    let checked_out = terminal.take_result().expect("terminal result checkout");
    drop(checked_out);
    assert!(terminal.take_result().is_some(), "checked-out Drop returns the shallow result ticket");
    let resumed = match terminal.take_result().expect("terminal result retry checkout").resume() {
        Ok(resumed) => resumed,
        Err(_) => panic!("terminal result resume rejected"),
    };
    let output = resumed.await.expect("resumed exact terminal result");
    assert_eq!(Arc::as_ptr(&output.into_parts().0), pointer);
    while !terminal.terminal_is_empty() {
        let _ = terminal.close_step();
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_retry_contention_is_one_compare_exchange_per_callback() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage, false).expect("retry-contention preparation");
    probe.resolved = true;
    let state = probe.state.clone();
    state.retry_armed.store(true, std::sync::atomic::Ordering::Release);
    let observed = state.retry_generation.load(std::sync::atomic::Ordering::Acquire);
    state.retry_generation.store(observed.checked_add(1).expect("fixture generation"), std::sync::atomic::Ordering::Release);
    state.advance_retry_generation_observed_once(observed);
    assert!(state.retry_armed.load(std::sync::atomic::Ordering::Acquire));
    state.retry_armed.store(false, std::sync::atomic::Ordering::Release);
    while !state.terminal_is_empty() {
        let _ = state.close_step();
    }
}
//#region 🧸️Fixtures
fn admit_fixture_blocker(pool: &WorkerPool, lane: Lane, mut job: semio_framework_async::Job) {
    loop {
        match pool.try_submit(lane, job) {
            Ok(()) => return,
            Err(error) if error.kind() == semio_framework_async::WorkerSubmitErrorKind::Contended => {
                job = error.into_job();
                std::thread::yield_now();
            }
            Err(error) => panic!("fixture blocker admission refused: {:?}", error.kind()),
        }
    }
}
//#endregion 🧸️Fixtures

//#region 🔖️Source laws
#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_engine_capability_open", include_str!("../../🦀️.rs"))] {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start().trim_start_matches("} else ");
            let Some((_, scrutinee)) = trimmed.strip_prefix("if let ").or_else(|| trimmed.strip_prefix("while let ")).and_then(|rest| rest.split_once(" = ")) else { continue };
            if !(scrutinee.contains("retry_job.lock()") && !scrutinee.starts_with("{ ") && scrutinee.trim_end().ends_with('{')) {
                continue;
            }
            let mut depth = 1i64;
            for body in &lines[index + 1..] {
                assert!(!body.contains("retry_job.lock()") && !body.contains("submit_exact("), "{name}:{} holds the retry-job guard across a body that relocks or resubmits it: {line}", index + 1);
                depth += body.matches('{').count() as i64 - body.matches('}').count() as i64;
                if depth <= 0 {
                    break;
                }
            }
        }
    }
}
//#endregion 🔖️Source laws
