use super::*;

use db_engine_base::test_worker_pool;
use db_io_core::{PayloadStorage as _, WalStorage as _};

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlledCatalogReadPoll {
    Pending,
    Ready,
    Panic,
}

struct ControlledCatalogReadFuture {
    pub mode: ControlledCatalogReadPoll,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub key: Option<DatabaseCatalogRootKey>,
    pub root: Option<db_io_core::DbIoPages>,
    pub polls: Arc<std::sync::atomic::AtomicUsize>,
    pub waker: Arc<Mutex<Option<std::task::Waker>>>,
}

impl Future for ControlledCatalogReadFuture {
    type Output = DatabaseCatalogReadResult;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let poll = self.polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        if poll == 0 {
            context.waker().wake_by_ref();
        }
        match self.mode {
            ControlledCatalogReadPoll::Pending => std::task::Poll::Pending,
            ControlledCatalogReadPoll::Ready => std::task::Poll::Ready(DatabaseCatalogReadResult {
                storage: self.storage.take().expect("controlled catalog storage"),
                key: self.key.take().expect("controlled catalog key"),
                root: Ok(Some((self.root.take().expect("controlled catalog root"), EpochFence::INITIAL))),
            }),
            ControlledCatalogReadPoll::Panic => panic!("controlled catalog-read panic"),
        }
    }
}

async fn controlled_catalog_read_probe(mode: ControlledCatalogReadPoll) -> (DatabaseCatalogReadFuture, Arc<std::sync::atomic::AtomicUsize>, Arc<Mutex<Option<std::task::Waker>>>, usize) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage.clone(), DatabaseCatalogRootKey::root(), false).expect("controlled catalog-read preparation");
    let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let waker = Arc::new(Mutex::new(None));
    let root = db_io_registry::db_io_copy_pages(&[1, 2, 3]).unwrap().await.unwrap();
    let future = ControlledCatalogReadFuture { mode, storage: Some(storage), key: Some(DatabaseCatalogRootKey::root()), root: Some(root), polls: polls.clone(), waker: waker.clone() };
    *probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCatalogReadWork::controlled(Box::pin(future), pointer));
    let work = probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("controlled catalog-read work");
    *probe.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
    probe.state.set_phase(DatabaseCatalogReadPhase::Poll);
    (probe, polls, waker, pointer)
}

fn control_catalog_read_submissions(state: &Arc<DatabaseCatalogReadState>) -> Arc<Mutex<ControlledCapabilitySubmitQueue>> {
    let queue = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let captured = queue.clone();
    *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| captured.lock().unwrap().push(job)));
    queue
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_paused_transfers_exclude_successors_and_public_cleanup() {
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📖️catalog-read-ownership/🔣️.json")).unwrap();
    for row in fixture["transfers"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
        let pointer = Arc::as_ptr(&storage) as usize;
        let storage_weak = Arc::downgrade(&storage);
        let probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage, DatabaseCatalogRootKey::root(), false).unwrap();
        let state = probe.state.clone();
        let phase = match row["phase"].as_str().unwrap() {
            "Handoff" => DatabaseCatalogReadPhase::Handoff,
            "RetainWork" => DatabaseCatalogReadPhase::RetainWork,
            "Poll" => DatabaseCatalogReadPhase::Poll,
            _ => unreachable!(),
        };
        if phase == DatabaseCatalogReadPhase::Poll {
            *state.poll_work.lock().unwrap() = state.work.lock().unwrap().take();
        }
        state.set_phase(phase);
        let queue = control_catalog_read_submissions(&state);
        state.schedule();
        let initial = queue.lock().unwrap().pop().unwrap();
        let submissions = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let count = submissions.clone();
        let captured = queue.clone();
        *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| {
            count.fetch_add(1, Ordering::AcqRel);
            captured.lock().unwrap().push(job)
        }));
        let (claimed, observed) = std::sync::mpsc::sync_channel(1);
        let (release, released) = std::sync::mpsc::sync_channel(1);
        *state.controlled_owner_transfer_hook.lock().unwrap() = Some(Box::new(move |identity| {
            claimed.send(identity).unwrap();
            released.recv_timeout(std::time::Duration::from_secs(5)).expect("catalog transfer release deadline");
        }));
        let driver = std::thread::spawn(initial);
        let claimed_pointer = observed.recv_timeout(std::time::Duration::from_secs(5)).expect("catalog transfer claim deadline");
        state.schedule();
        state.schedule();
        probe.cancel();
        let submissions_while_active = submissions.load(Ordering::Acquire);
        drop(probe);
        let terminal = take_database_catalog_read_terminal(state.generation).unwrap();
        let (terminal, resume_blocked) = match terminal.resume() {
            Err(terminal) => (terminal, true),
            Ok(resumed) => {
                drop(resumed);
                (take_database_catalog_read_terminal(state.generation).unwrap(), false)
            }
        };
        let first = terminal.close_step();
        for _ in 0..64 {
            if terminal.close_step() == DatabaseCatalogReadCloseStep::Complete {
                break;
            }
        }
        let retained = state.admission.lock().unwrap().is_some() && database_catalog_read_registry().lock().unwrap()[state.slot].as_ref().is_some_and(|entry| Arc::ptr_eq(entry, &state));
        let exact_storage = claimed_pointer == pointer && storage_weak.upgrade().is_some_and(|storage| Arc::as_ptr(&storage) as usize == pointer);
        release.send(()).unwrap();
        driver.join().unwrap();
        for _ in 0..64 {
            let job = queue.lock().unwrap().pop();
            if let Some(job) = job {
                job();
            }
            if terminal.close_step() == DatabaseCatalogReadCloseStep::Complete {
                break;
            }
        }
        let empty = terminal.terminal_is_empty() && database_catalog_read_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state));
        assert_eq!(submissions_while_active, row["submissionsWhileActive"].as_u64().unwrap() as usize, "active transfer must defer successor submission");
        assert!(resume_blocked);
        assert_eq!(first == DatabaseCatalogReadCloseStep::Blocked, row["closeBlocked"].as_bool().unwrap());
        assert_eq!(retained, row["admissionRetained"].as_bool().unwrap());
        assert_eq!(exact_storage, row["storageRetained"].as_bool().unwrap());
        assert_eq!(empty, row["finalEmpty"].as_bool().unwrap());
        assert!(queue.lock().unwrap().pop().is_none());
        assert!(storage_weak.upgrade().is_none());
    }
}

async fn catalog_read_fixture_probe(fixture: &serde_json::Value, outcome: &str) -> (DatabaseCatalogReadFuture, usize, Option<u64>) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage.clone(), DatabaseCatalogRootKey::root(), false).unwrap();
    let bytes: Vec<u8> = fixture["root"]["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let (root, operation) = if outcome == "pages" {
        let pages = db_io_registry::db_io_copy_pages(&bytes).unwrap().await.unwrap();
        let operation = pages.operation();
        (Ok(Some((pages, EpochFence { epoch: fixture["root"]["epoch"].as_u64().unwrap() }))), Some(operation))
    } else {
        (Err(DbError::Unavailable("fixture-root-fault".into())), None)
    };
    let result = DatabaseCatalogReadResult { storage, key: DatabaseCatalogRootKey::root(), root };
    *probe.state.work.lock().unwrap() = Some(DatabaseCatalogReadWork::controlled(Box::pin(async move { result }), pointer));
    (probe, pointer, operation)
}

fn assert_catalog_read_fixture_result(fixture: &serde_json::Value, result: DatabaseCatalogReadResult, pointer: usize, operation: Option<u64>) {
    let (storage, key, root) = result.into_parts();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(key, DatabaseCatalogRootKey::root());
    match operation {
        Some(operation) => {
            let bytes: Vec<u8> = fixture["root"]["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
            let (mut pages, fence) = root.unwrap().unwrap();
            assert_eq!(pages.operation(), operation);
            assert_eq!(pages.page(0).unwrap(), bytes.as_slice());
            assert_eq!(fence.epoch, fixture["root"]["epoch"].as_u64().unwrap());
            for _ in 0..128 {
                if pages.terminal_is_empty() {
                    break;
                }
                pages.close_step().unwrap();
            }
            assert!(pages.terminal_is_empty());
        }
        None => assert!(matches!(root, Err(DbError::Unavailable(ref message)) if message == "fixture-root-fault")),
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_retry_and_terminal_resume_preserve_exact_root() {
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📖️catalog-read-ownership/🔣️.json")).unwrap();
    for row in fixture["recovery"].as_array().unwrap() {
        let (probe, pointer, operation) = catalog_read_fixture_probe(&fixture, row["outcome"].as_str().unwrap()).await;
        let state = probe.state.clone();
        let queue = control_catalog_read_submissions(&state);
        let mut terminal = None;
        let resumed;
        if row["path"] == "retry" {
            let callbacks = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
            let captured = callbacks.clone();
            *state.controlled_retry_callback_hook.lock().unwrap() = Some(Arc::new(move |job| captured.lock().unwrap().push(job)));
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(Err));
            *state.controlled_submit_refusal.lock().unwrap() = Some(semio_framework_async::WorkerSubmitErrorKind::Saturated);
            state.schedule();
            let old_callback = callbacks.lock().unwrap().pop().expect("first exact retry callback");
            assert!(state.retry_job.lock().unwrap().is_some());
            let captured = queue.clone();
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| captured.lock().unwrap().push(job)));
            drop(probe);
            let cursor = take_database_catalog_read_terminal(state.generation).unwrap();
            assert!(queue.lock().unwrap().pop().is_none(), "retained retry is the sole successor owner during cancellation");
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(Err));
            *state.controlled_submit_refusal.lock().unwrap() = Some(semio_framework_async::WorkerSubmitErrorKind::Saturated);
            resumed = match cursor.resume() {
                Ok(resumed) => resumed,
                Err(_) => panic!("exact retained retry must resume"),
            };
            let next_callback = callbacks.lock().unwrap().pop().expect("replacement exact retry callback");
            let captured = queue.clone();
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| captured.lock().unwrap().push(job)));
            old_callback();
            let stale_submissions = queue.lock().unwrap().len;
            assert_eq!(stale_submissions, row["staleRetrySubmissions"].as_u64().unwrap() as usize);
            assert!(state.retry_job.lock().unwrap().is_some(), "old callback cannot steal the replacement retry");
            assert!(state.work.lock().unwrap().is_some());
            next_callback();
            for _ in 0..64 {
                let job = queue.lock().unwrap().pop();
                let Some(job) = job else { break };
                job();
            }
        } else {
            state.schedule();
            let target = if row["path"] == "terminal-completion" { DatabaseCatalogReadPhase::Publish } else { DatabaseCatalogReadPhase::RetainWork };
            for _ in 0..32 {
                if state.phase() == target {
                    break;
                }
                let job = queue.lock().unwrap().pop().unwrap();
                job();
            }
            assert_eq!(state.phase(), target);
            if row["path"] == "terminal-completion" {
                let publish = queue.lock().unwrap().pop().unwrap();
                let (published, observed) = std::sync::mpsc::sync_channel(1);
                let (release, released) = std::sync::mpsc::sync_channel(1);
                *state.controlled_completion_before_wake_hook.lock().unwrap() = Some(Box::new(move || {
                    published.send(()).unwrap();
                    released.recv_timeout(std::time::Duration::from_secs(5)).expect("catalog recovery publisher release deadline");
                }));
                let publisher = std::thread::spawn(publish);
                observed.recv_timeout(std::time::Duration::from_secs(5)).expect("catalog recovery publication deadline");
                drop(probe);
                release.send(()).unwrap();
                publisher.join().unwrap();
            } else if row["path"] == "spent-work" {
                let retain = queue.lock().unwrap().pop().unwrap();
                let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
                let captured = queue.clone();
                *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| if count.fetch_add(1, Ordering::AcqRel) == 0 { Err(job) } else { captured.lock().unwrap().push(job) }));
                *state.controlled_submit_refusal.lock().unwrap() = Some(semio_framework_async::WorkerSubmitErrorKind::Shutdown);
                retain();
                assert!(state.terminal_work.lock().unwrap().is_some(), "Ready work is retained before its close grant");
                assert!(state.terminal_job.lock().unwrap().is_some());
                drop(probe);
                assert!(queue.lock().unwrap().pop().is_none(), "refused cleanup retains the sole exact successor");
            } else {
                probe.cancel();
                drop(probe);
            }
            for _ in 0..64 {
                let job = queue.lock().unwrap().pop();
                let Some(job) = job else { break };
                job();
            }
            assert!(queue.lock().unwrap().pop().is_none(), "catalog recovery cleanup must converge");
            let cursor = take_database_catalog_read_terminal(state.generation).unwrap();
            let cursor = if row["path"] == "spent-work" {
                match cursor.resume() {
                    Err(cursor) => cursor,
                    Ok(resumed) => {
                        drop(resumed);
                        panic!("terminal work that already returned Ready must never be repolled");
                    }
                }
            } else {
                cursor
            };
            if row["path"] == "spent-work" {
                for _ in 0..64 {
                    if state.terminal_result.lock().unwrap().is_some() {
                        break;
                    }
                    cursor.close_step();
                }
            }
            if row["path"] == "terminal-completion" {
                resumed = match cursor.resume() {
                    Ok(resumed) => resumed,
                    Err(_) => panic!("exact terminal completion must resume"),
                };
            } else {
                let checkout = cursor.take_result().expect("exact root result checkout");
                for _ in 0..64 {
                    if cursor.close_step() == DatabaseCatalogReadCloseStep::Blocked {
                        break;
                    }
                }
                assert_eq!(state.admission.lock().unwrap().is_some(), row["checkoutBlocksRetirement"].as_bool().unwrap());
                assert_eq!(cursor.close_step(), DatabaseCatalogReadCloseStep::Blocked);
                drop(checkout);
                resumed = match cursor.take_result().expect("root result checkout handback").resume() {
                    Ok(resumed) => resumed,
                    Err(_) => panic!("exact shallow root result must resume"),
                };
                terminal = Some(cursor);
            }
        }
        let mut resumed = resumed;
        let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
        let waker = std::task::Waker::from(wake);
        let mut context = std::task::Context::from_waker(&waker);
        let std::task::Poll::Ready(Ok(result)) = std::pin::Pin::new(&mut resumed).poll(&mut context) else { panic!("resumed catalog root must be delivered exactly once") };
        assert_catalog_read_fixture_result(&fixture, result, pointer, operation);
        if let Some(cursor) = terminal {
            for _ in 0..64 {
                if cursor.close_step() == DatabaseCatalogReadCloseStep::Complete {
                    break;
                }
            }
        }
        let empty = state.terminal_is_empty() && database_catalog_read_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state));
        assert_eq!(empty, row["terminalEmpty"].as_bool().unwrap());
        assert!(!state.retry_armed.load(Ordering::Acquire));
        assert!(!state.terminal_result_checked_out.load(Ordering::Acquire));
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_consumed_publication_preserves_exact_root_and_retires() {
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📖️catalog-read-ownership/🔣️.json")).unwrap();
    for row in fixture["completion"].as_array().unwrap() {
        let (mut probe, pointer, operation) = catalog_read_fixture_probe(&fixture, row["outcome"].as_str().unwrap()).await;
        let state = probe.state.clone();
        let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
        let waker = std::task::Waker::from(wake);
        let mut context = std::task::Context::from_waker(&waker);
        assert!(std::pin::Pin::new(&mut probe).poll(&mut context).is_pending());
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let active = row["publication"] != "synchronous";
        let after_check = row["publication"] == "after-finalizer-check" || row["publication"] == "refused-finalizer";
        let refused = row["publication"] == "refused-finalizer";
        let mut publisher = None;
        let mut release_publisher = None;
        if active {
            let queue = control_catalog_read_submissions(&state);
            state.schedule();
            for _ in 0..32 {
                if state.phase() == DatabaseCatalogReadPhase::Publish {
                    break;
                }
                let job = queue.lock().unwrap().pop().expect("catalog pipeline successor");
                job();
            }
            assert_eq!(state.phase(), DatabaseCatalogReadPhase::Publish);
            let publish = queue.lock().unwrap().pop().unwrap();
            assert!(queue.lock().unwrap().pop().is_none());
            let (published, observed) = std::sync::mpsc::sync_channel(1);
            let (release, released) = std::sync::mpsc::sync_channel(1);
            let hook: Box<dyn FnOnce() + Send> = Box::new(move || {
                published.send(()).unwrap();
                released.recv_timeout(std::time::Duration::from_secs(5)).expect("catalog publisher release deadline");
            });
            if after_check {
                *state.controlled_lease_release_hook.lock().unwrap() = Some(hook);
            } else {
                *state.controlled_completion_before_wake_hook.lock().unwrap() = Some(hook);
            }
            publisher = Some(std::thread::spawn(publish));
            release_publisher = Some(release);
            observed.recv_timeout(std::time::Duration::from_secs(5)).expect("catalog publication deadline");
            if refused {
                *state.controlled_submit_refusal.lock().unwrap() = Some(semio_framework_async::WorkerSubmitErrorKind::Shutdown);
            }
            let late_count = count.clone();
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| {
                assert!(late_count.fetch_add(1, Ordering::AcqRel) < 32, "catalog retirement phase budget");
                if refused {
                    return Err(job);
                }
                job();
                Ok(())
            }));
        } else {
            let submitted = count.clone();
            *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| {
                assert!(submitted.fetch_add(1, Ordering::AcqRel) < 32, "catalog synchronous phase budget");
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
        let std::task::Poll::Ready(Ok(result)) = completed else { panic!("catalog result must remain consumable") };
        assert_catalog_read_fixture_result(&fixture, result, pointer, operation);
        let retirement_submissions = count.load(Ordering::Acquire) - before_retirement;
        let empty = state.terminal_is_empty();
        assert_eq!(retained, row["admissionDuringPublication"].as_bool().unwrap());
        assert_eq!(retirement_submissions, row["retirementSubmissions"].as_u64().unwrap() as usize);
        assert_eq!(empty, row["terminalEmpty"].as_bool().unwrap());
        assert!(state.controlled_submit_refusal.lock().unwrap().is_none());
        assert!(state.terminal_job.lock().unwrap().is_none());
        assert!(state.completion.lock().unwrap().is_none());
        assert!(database_catalog_read_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state)));
        let before_late = count.load(Ordering::Acquire);
        state.schedule();
        state.schedule_cleanup();
        assert_eq!(count.load(Ordering::Acquire) - before_late, row["lateSubmissions"].as_u64().unwrap() as usize);
    }
}

#[test]
fn database_catalog_read_fixed_cap_plus_one_and_generation_aba() {
    let mut admission = DatabaseCatalogReadAdmissionState::empty();
    assert!(admission.try_claim(DATABASE_CATALOG_READ_ITEMS + 1, DATABASE_CATALOG_READ_BYTES).is_err());
    assert!(admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES + 1).is_err());
    let mut claims = Vec::with_capacity(DATABASE_CATALOG_READ_SLOTS);
    for _ in 0..DATABASE_CATALOG_READ_SLOTS {
        claims.push(admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES).expect("catalog-read fixed admission"));
    }
    assert!(admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES).is_err());
    let (slot, generation) = claims.remove(0);
    assert!(admission.release(slot, generation));
    let replacement = admission.try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES).expect("catalog-read slot reuse");
    assert_eq!(replacement.0, slot);
    assert_ne!(replacement.1, generation);
    assert!(!admission.release(slot, generation));
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_success_returns_exact_storage_key_and_root() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage);
    let result = DatabaseCatalogReadFuture::try_submit(test_worker_pool(), storage, DatabaseCatalogRootKey::root()).expect("catalog-read success admission").await.expect("catalog-read success completion");
    let (returned_storage, returned_key, root) = result.into_parts();
    assert_eq!(Arc::as_ptr(&returned_storage), pointer);
    assert_eq!(returned_key, DatabaseCatalogRootKey::root());
    assert!(root.expect("catalog-read backend result").is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_controlled_wakes_coalesce_and_terminal_never_repolls() {
    for mode in [ControlledCatalogReadPoll::Pending, ControlledCatalogReadPoll::Ready, ControlledCatalogReadPoll::Panic] {
        let (mut probe, polls, waker, _) = controlled_catalog_read_probe(mode).await;
        probe.resolved = true;
        let state = probe.state.clone();
        state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        let queue = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
        let queue_hook = queue.clone();
        *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
        state.schedule();
        let initial = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("catalog-read initial callback");
        initial();
        assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
        assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "one coalesced catalog-read successor");
        waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().expect("catalog-read retained waker").wake_by_ref();
        assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "late wake cannot overtake the retained successor");
        let successor = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("catalog-read successor callback");
        successor();
        if mode == ControlledCatalogReadPoll::Pending {
            assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 2);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
        } else {
            assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
            assert!(state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            if mode == ControlledCatalogReadPoll::Ready {
                assert!(state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
            }
        }
        loop {
            let Some(job) = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop() else { break };
            job();
        }
        while !state.terminal_is_empty() {
            let _ = state.close_step();
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_publication_between_check_and_waker_registration_is_observed() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage, DatabaseCatalogRootKey::root(), false).expect("public lost-wake preparation");
    let state = probe.state.clone();
    let mut work = state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("public lost-wake retained work");
    assert!(work.close_step());
    assert!(work.terminal_is_empty());
    drop(work);

    let result_storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let result_pointer = Arc::as_ptr(&result_storage) as usize;
    let publish_state = state.clone();
    *state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
        publish_state.publish_public_completion(Ok(DatabaseCatalogReadResult { storage: result_storage, key: DatabaseCatalogRootKey::root(), root: Ok(None) }));
    }));
    let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
    let waker = std::task::Waker::from(wake.clone());
    let mut context = std::task::Context::from_waker(&waker);
    let ready = std::pin::Pin::new(&mut probe).poll(&mut context);
    let std::task::Poll::Ready(Ok(result)) = ready else { panic!("completion published in the registration window must be observed in the same public poll") };
    assert_eq!(Arc::as_ptr(&result.into_parts().0) as usize, result_pointer, "public recheck returns the exact published owner");
    assert_eq!(wake.0.load(std::sync::atomic::Ordering::Acquire), 0, "the recheck consumes completion without needing a later wake");
    assert!(state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none(), "Ready clears the transient public waker");
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_rejected_mount_retires_storage_and_key_on_distinct_grants() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let storage_weak = Arc::downgrade(&storage);
    let rejected = DatabaseCatalogReadRejected { error: Some(DbError::Closed), storage: Some(storage), key: Some(DatabaseCatalogRootKey::root()) };
    let (error, close) = rejected.mount_close_and_take_error(test_worker_pool(), false);
    assert_eq!(error, DbError::Closed);
    let queue = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let queue_hook = queue.clone();
    *close.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
    close.schedule();
    assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1);

    let storage_grant = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("rejected storage close grant");
    storage_grant();
    assert!(storage_weak.upgrade().is_none(), "first mounted close grant releases only the exact storage owner");
    {
        let owner = close.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = owner.as_ref().expect("key remains mounted after storage grant");
        assert!(owner.storage.is_none());
        assert_eq!(owner.key.as_ref(), Some(&DatabaseCatalogRootKey::root()));
    }
    assert!(!close.terminal_is_empty());
    assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "unfinished key close is retained as the next governed grant");

    let key_grant = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().expect("rejected key close grant");
    key_grant();
    assert!(close.terminal_is_empty(), "second mounted close grant releases the exact key and reaches terminal empty");
    assert_eq!(queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 0);
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_cancel_stale_and_rejection_preserve_exact_storage_key() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage);
    let mut rejected = DatabaseCatalogReadRejected { error: Some(DbError::Closed), storage: Some(storage), key: Some(DatabaseCatalogRootKey::root()) };
    assert_eq!(Arc::as_ptr(rejected.storage.as_ref().expect("rejected storage")), pointer);
    assert_eq!(rejected.close_step(), DatabaseCatalogReadCloseStep::Progress);
    assert!(!rejected.terminal_is_empty());
    assert_eq!(rejected.close_step(), DatabaseCatalogReadCloseStep::Progress);
    assert!(rejected.terminal_is_empty());
    assert_eq!(rejected.into_error_after_close().expect("rejected error"), DbError::Closed);

    for stale in [false, true] {
        let (probe, _, _, pointer) = controlled_catalog_read_probe(ControlledCatalogReadPoll::Pending).await;
        let state = probe.state.clone();
        let generation = state.generation;
        let queue = control_catalog_read_submissions(&state);
        state.schedule();
        let initial = queue.lock().unwrap().pop().unwrap();
        if stale {
            DATABASE_CATALOG_READ_ADMISSION.lock().unwrap().slots[state.slot].generation = generation.checked_add(1).unwrap();
        } else {
            probe.cancel();
        }
        drop(probe);
        initial();
        if stale {
            DATABASE_CATALOG_READ_ADMISSION.lock().unwrap().slots[state.slot].generation = generation;
        }
        let owners = [state.work.lock().unwrap().as_ref().map(|work| work.storage_identity), state.poll_work.lock().unwrap().as_ref().map(|work| work.storage_identity), state.terminal_work.lock().unwrap().as_ref().map(|work| work.storage_identity)];
        assert_eq!(owners.into_iter().flatten().collect::<Vec<_>>(), vec![pointer], "one exact storage owner remains after interruption");
        let terminal = take_database_catalog_read_terminal(generation).unwrap();
        for _ in 0..64 {
            let job = queue.lock().unwrap().pop();
            if let Some(job) = job {
                job();
            }
            if terminal.close_step() == DatabaseCatalogReadCloseStep::Complete {
                break;
            }
        }
        assert!(queue.lock().unwrap().pop().is_none());
        assert!(terminal.terminal_is_empty());
        assert!(database_catalog_read_registry().lock().unwrap()[state.slot].as_ref().is_none_or(|owner| !Arc::ptr_eq(owner, &state)));
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_terminal_result_drop_hands_back_exact_result() {
    let (mut probe, _, _, _) = controlled_catalog_read_probe(ControlledCatalogReadPoll::Ready).await;
    probe.resolved = true;
    probe.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let result_pointer = Arc::as_ptr(&storage) as usize;
    *probe.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(DatabaseCatalogReadResult { storage, key: DatabaseCatalogRootKey::root(), root: Ok(None) }));
    let terminal = DatabaseCatalogReadTerminalHandle { state: probe.state.clone() };
    let checked_out = terminal.take_result().expect("catalog result checkout");
    drop(checked_out);
    let result = terminal.take_result().expect("catalog result handback").take().expect("catalog result take").expect("catalog result owner");
    assert_eq!(Arc::as_ptr(&result.into_parts().0) as usize, result_pointer, "catalog result checkout returns the exact storage owner");
    while !terminal.terminal_is_empty() {
        let _ = terminal.close_step();
    }
}

//#region 🔖️Source laws
#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_engine_catalog_read", include_str!("../../🦀️.rs"))] {
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
