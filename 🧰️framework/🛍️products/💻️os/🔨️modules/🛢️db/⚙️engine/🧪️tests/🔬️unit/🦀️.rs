use super::*;
use crate::vcs_integration::{HashMutation, HashProjection};
use db_storage::{PayloadStorage as _, WalStorage as _};
use protocol::{OpBinary, OpText};
use store::ArtifactPack;

async fn rejected_document_open_error(mut rejected: DatabaseDocumentOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

#[derive(Default)]
struct ControlledMountEmitState {
    entered: bool,
    released: bool,
    events: Vec<EmitEvent>,
}

#[derive(Clone, Default)]
struct ControlledMountEmit {
    state: Arc<(Mutex<ControlledMountEmitState>, std::sync::Condvar)>,
}

impl ControlledMountEmit {
    fn wait_until_document_event(&self) {
        let (state, signal) = &*self.state;
        let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !state.entered {
            state = signal.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    fn release_document_event(&self) {
        let (state, signal) = &*self.state;
        state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).released = true;
        signal.notify_all();
    }

    fn document_event_count(&self) -> usize {
        self.state.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).events.iter().filter(|event| event.document.is_some()).count()
    }
}

impl Emit for ControlledMountEmit {
    async fn emit(&self, event: EmitEvent) {
        let (state, signal) = &*self.state;
        let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if event.document.is_some() {
            state.entered = true;
            signal.notify_all();
            while !state.released {
                state = signal.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }
        state.events.push(event);
        signal.notify_all();
    }
}

struct MountFanoutLockProbe {
    registry: Arc<Mutex<DatabaseDocumentMountRegistry>>,
    called: std::sync::atomic::AtomicBool,
    unlocked: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for MountFanoutLockProbe {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.unlocked.store(self.registry.try_lock().is_ok(), std::sync::atomic::Ordering::Release);
        self.called.store(true, std::sync::atomic::Ordering::Release);
    }
}

#[test]
fn interrupted_query_stream_drop_retains_one_resumable_close_owner() {
    while engine_query_maintenance_step().unwrap() {}
    let mut stream = QueryStream::new();
    stream.push(QueryResultEntry { path: db_storage::DbIoText::try_from_str("retained-path").unwrap(), value: None }).unwrap();
    drop(stream);
    assert!(engine_query_maintenance_step().unwrap());
    {
        let retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(retired.iter().flatten().any(|owner| !owner.terminal_is_empty()));
    }
    while engine_query_maintenance_step().unwrap() {}

    ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    {
        let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = Some(QueryStream::new());
        }
    }
    {
        let mut overflow = ENGINE_QUERY_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(QueryStream::new());
        }
    }
    let mut exact = QueryStream::new();
    exact.push(QueryResultEntry { path: db_storage::DbIoText::try_from_str("exact-overflow-stream").unwrap(), value: None }).unwrap();
    let mut second = QueryStream::new();
    second.push(QueryResultEntry { path: db_storage::DbIoText::try_from_str("second-overflow-stream").unwrap(), value: None }).unwrap();
    assert_eq!(exact.retirement.map(|reservation| reservation.tier), Some(2));
    assert_eq!(second.retirement.map(|reservation| reservation.tier), Some(2));
    assert!(retire_engine_query_stream(exact).is_ok());
    assert!(retire_engine_query_stream(second).is_ok());
    assert!(ENGINE_QUERY_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = ENGINE_QUERY_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(quarantine.iter().flatten().find_map(|stream| stream.get(0).map(QueryResultEntry::path)), Some("exact-overflow-stream"));
        assert!(quarantine.iter().flatten().any(|stream| stream.get(0).map(QueryResultEntry::path) == Some("second-overflow-stream")));
    }
    {
        let mut retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = None;
        }
    }
    for _ in 0..ENGINE_RETIRED_QUERY_STREAMS * 2 {
        assert!(engine_query_maintenance_step().unwrap());
    }
    assert!(engine_query_maintenance_step().unwrap());
    {
        let retired = ENGINE_QUERY_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(retired.iter().flatten().find_map(|stream| stream.get(0).map(QueryResultEntry::path)), Some("exact-overflow-stream"));
    }
    while engine_query_maintenance_step().unwrap() {}

    for tier in [&ENGINE_QUERY_RETIREMENT, &ENGINE_QUERY_RETIREMENT_OVERFLOW, &ENGINE_QUERY_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = Some(QueryStream::new());
        }
    }
    let exact_refusal = QueryResultEntry { path: db_storage::DbIoText::try_from_str("exact-all-tier-stream-refusal").unwrap(), value: None };
    let mut refused = QueryStream::new();
    let mut exact_refusal = refused.push(exact_refusal).unwrap_err();
    assert_eq!(exact_refusal.path(), "exact-all-tier-stream-refusal");
    assert!(exact_refusal.close_step().unwrap());
    for tier in [&ENGINE_QUERY_RETIREMENT, &ENGINE_QUERY_RETIREMENT_OVERFLOW, &ENGINE_QUERY_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = None;
        }
    }
    assert!(refused.terminal_is_empty());
}

async fn decode_query_json(mut stream: QueryStream) -> serde_json::Value {
    let mut bytes = Vec::new();
    for fragment in stream.get(0).and_then(QueryResultEntry::value).expect("retained query value").fragments() {
        bytes.extend_from_slice(fragment);
    }
    let value = db_artifact::decode_pathmap_json(&bytes).await.unwrap();
    while stream.close_step().unwrap() {}
    assert!(stream.terminal_is_empty());
    value
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlledCapabilityPoll {
    Pending,
    Ready,
    Panic,
}

struct ControlledCapabilityFuture {
    mode: ControlledCapabilityPoll,
    storage: Option<Arc<db_storage::DbBackend>>,
    polls: Arc<std::sync::atomic::AtomicUsize>,
    waker: Arc<Mutex<Option<std::task::Waker>>>,
    boundary: Option<Arc<(Mutex<(bool, bool)>, std::sync::Condvar)>>,
    wake_during_poll: bool,
}

struct ControlledCapabilitySubmitQueue {
    slots: [Option<semio_framework_async::Job>; 8],
    head: usize,
    len: usize,
}

struct ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize);

impl std::task::Wake for ControlledCatalogPublicWake {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

impl ControlledCapabilitySubmitQueue {
    fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0 }
    }

    fn push(&mut self, job: semio_framework_async::Job) -> Result<(), semio_framework_async::Job> {
        if self.len == self.slots.len() {
            return Err(job);
        }
        let index = (self.head + self.len) % self.slots.len();
        self.slots[index] = Some(job);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<semio_framework_async::Job> {
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
                capabilities: db_storage::StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true },
            }),
            ControlledCapabilityPoll::Panic => panic!("controlled capability-open poll panic"),
        }
    }
}

async fn controlled_capability_probe(
    mode: ControlledCapabilityPoll,
    boundary: Option<Arc<(Mutex<(bool, bool)>, std::sync::Condvar)>>,
) -> (DatabaseCapabilityOpenFuture, Arc<std::sync::atomic::AtomicUsize>, Arc<Mutex<Option<std::task::Waker>>>, usize) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlledCatalogReadPoll {
    Pending,
    Ready,
    Panic,
}

struct ControlledCatalogReadFuture {
    mode: ControlledCatalogReadPoll,
    storage: Option<Arc<db_storage::DbBackend>>,
    key: Option<DatabaseCatalogRootKey>,
    root: Option<db_storage::DbIoPages>,
    polls: Arc<std::sync::atomic::AtomicUsize>,
    waker: Arc<Mutex<Option<std::task::Waker>>>,
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage.clone(), DatabaseCatalogRootKey::root(), false).expect("controlled catalog-read preparation");
    let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let waker = Arc::new(Mutex::new(None));
    let root = db_storage::db_io_copy_pages(&[1, 2, 3]).unwrap().await.unwrap();
    let future = ControlledCatalogReadFuture { mode, storage: Some(storage), key: Some(DatabaseCatalogRootKey::root()), root: Some(root), polls: polls.clone(), waker: waker.clone() };
    *probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCatalogReadWork::controlled(Box::pin(future), pointer));
    let work = probe.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("controlled catalog-read work");
    *probe.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
    probe.state.set_phase(DatabaseCatalogReadPhase::Poll);
    (probe, polls, waker, pointer)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlledCatalogBootstrapPoll {
    Pending,
    NoService,
    Ready,
    Fenced,
    Panic,
}

struct ControlledCatalogBootstrapFuture {
    mode: ControlledCatalogBootstrapPoll,
    pages: Option<db_storage::DbIoPages>,
    polls: Arc<std::sync::atomic::AtomicUsize>,
    waker: Arc<Mutex<Option<std::task::Waker>>>,
}

impl Future for ControlledCatalogBootstrapFuture {
    type Output = Result<EpochFence, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let poll = self.polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        if poll == 0 && self.mode != ControlledCatalogBootstrapPoll::NoService {
            context.waker().wake_by_ref();
        }
        match self.mode {
            ControlledCatalogBootstrapPoll::Pending | ControlledCatalogBootstrapPoll::NoService => std::task::Poll::Pending,
            ControlledCatalogBootstrapPoll::Ready => {
                self.pages.take();
                std::task::Poll::Ready(Ok(EpochFence::INITIAL.next()))
            }
            ControlledCatalogBootstrapPoll::Fenced => {
                self.pages.take();
                std::task::Poll::Ready(Err(DbError::Fenced { expected: 1, actual: 0 }))
            }
            ControlledCatalogBootstrapPoll::Panic => panic!("controlled catalog-bootstrap poll panic"),
        }
    }
}

async fn catalog_bootstrap_pages(page_count: usize) -> db_storage::DbIoPages {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(page_count).unwrap();
    let full = [b'x'; db_storage::DB_IO_PAGE_BYTES];
    for index in 0..page_count {
        let source = if index + 1 == page_count { &b"[x]"[..] } else { &full[..] };
        assert_eq!(writer.write_fragment(source).unwrap(), source.len());
    }
    writer.seal_retained().await.unwrap()
}

async fn empty_catalog_bootstrap_pages() -> db_storage::DbIoPages {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(1).unwrap();
    assert_eq!(writer.write_fragment(b"[]").unwrap(), 2);
    writer.seal_retained().await.unwrap()
}

async fn controlled_catalog_bootstrap_probe(mode: ControlledCatalogBootstrapPoll) -> (DatabaseCatalogBootstrapFuture, Arc<std::sync::atomic::AtomicUsize>, Arc<Mutex<Option<std::task::Waker>>>, usize, u64) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let pages = empty_catalog_bootstrap_pages().await;
    let probe = DatabaseCatalogBootstrapFuture::try_prepare_with_key(test_worker_pool(), storage, pages, DatabaseCatalogBootstrapKey::root(), EpochFence::INITIAL, false).unwrap();
    let pages = probe.state.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().unwrap();
    let page_identity = pages.operation();
    let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let waker = Arc::new(Mutex::new(None));
    let future = ControlledCatalogBootstrapFuture { mode, pages: Some(pages), polls: polls.clone(), waker: waker.clone() };
    *probe.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCatalogBootstrapWork::controlled(Box::pin(future), pointer, page_identity));
    probe.state.set_phase(DatabaseCatalogBootstrapPhase::Poll);
    (probe, polls, waker, pointer, page_identity)
}

#[test]
fn database_catalog_bootstrap_max_plus_one_and_aba_preserve_exact_credit_identity() {
    let mut state = DatabaseCatalogBootstrapAdmissionState::empty();
    let mut claims = Vec::with_capacity(DATABASE_CATALOG_BOOTSTRAP_SLOTS);
    for _ in 0..DATABASE_CATALOG_BOOTSTRAP_SLOTS {
        claims.push(state.try_claim(DATABASE_CATALOG_BOOTSTRAP_PAGES).unwrap());
    }
    assert_eq!(state.items, DATABASE_CATALOG_BOOTSTRAP_TOTAL_ITEMS);
    assert_eq!(state.bytes, DATABASE_CATALOG_BOOTSTRAP_TOTAL_BYTES);
    assert!(state.try_claim(DATABASE_CATALOG_BOOTSTRAP_PAGES).is_err());
    assert!(state.try_claim(DATABASE_CATALOG_BOOTSTRAP_PAGES + 1).is_err());
    let (slot, generation, bytes) = claims.remove(0);
    assert!(state.release(slot, generation, bytes));
    let replacement = state.try_claim(DATABASE_CATALOG_BOOTSTRAP_PAGES).unwrap();
    assert_eq!(replacement.0, slot);
    assert_ne!(replacement.1, generation);
    assert!(!state.release(slot, generation, bytes), "ABA-stale generation cannot release the replacement owner");
    assert!(state.release(replacement.0, replacement.1, replacement.2));
    for (slot, generation, bytes) in claims {
        assert!(state.release(slot, generation, bytes));
    }
    assert_eq!((state.items, state.bytes), (0, 0));
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_real_max_plus_one_refusal_returns_pages_storage_key_and_fence() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let storage_pointer = Arc::as_ptr(&storage) as usize;
    let pages = catalog_bootstrap_pages(usize::from(DATABASE_CATALOG_BOOTSTRAP_PAGES) + 1).await;
    let operation = pages.operation();
    let rejected = match DatabaseCatalogBootstrapFuture::try_submit(pool, storage, pages, EpochFence::INITIAL) {
        Ok(_) => panic!("MAX+1 catalog-bootstrap pages were admitted"),
        Err(rejected) => rejected,
    };
    {
        let owner = rejected.close.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(Arc::as_ptr(owner.as_ref().unwrap().storage.as_ref().unwrap()) as usize, storage_pointer);
        assert_eq!(owner.as_ref().unwrap().pages.as_ref().unwrap().operation(), operation);
        assert_eq!(owner.as_ref().unwrap().pages.as_ref().unwrap().page_count(), DATABASE_CATALOG_BOOTSTRAP_PAGES + 1);
    }
    assert_eq!(rejected.expected, EpochFence::INITIAL);
    let (error, close) = rejected.mount_close_and_take_error();
    assert!(matches!(error, DbError::LimitExceeded("database catalog-bootstrap page credit")));
    let close = close.unwrap();
    while !close.terminal_is_empty() {
        std::thread::yield_now();
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_success_runs_on_pool_io_worker_and_returns_exact_storage_epoch() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogBootstrapFuture::try_submit(pool, storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap();
    let state = probe.state.clone();
    let result = probe.await.unwrap();
    assert!(state.poll_worker_thread.load(std::sync::atomic::Ordering::Acquire), "backend CAS future must be polled on the shared WorkerPool thread selected through Lane::Io");
    let (storage, key, expected, actual) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(key, DatabaseCatalogBootstrapKey::root());
    assert_eq!(expected, EpochFence::INITIAL);
    assert_eq!(actual.unwrap(), EpochFence::INITIAL.next());
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_cas_mismatch_returns_identical_storage_and_exact_fenced_error_without_retry() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let first = DatabaseCatalogBootstrapFuture::try_submit(pool.clone(), storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap().await.unwrap();
    let (storage, _, _, installed) = first.into_parts().unwrap();
    assert_eq!(installed.unwrap(), EpochFence::INITIAL.next());
    let pointer = Arc::as_ptr(&storage) as usize;
    let mismatch = DatabaseCatalogBootstrapFuture::try_submit(pool, storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap().await.unwrap();
    let (storage, _, expected, actual) = mismatch.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(expected, EpochFence::INITIAL);
    assert_eq!(actual, Err(DbError::Fenced { expected: 1, actual: 0 }));
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_ready_and_pending_interruption_publish_once_and_retain_owner() {
    for mode in [ControlledCatalogBootstrapPoll::Ready, ControlledCatalogBootstrapPoll::Fenced, ControlledCatalogBootstrapPoll::Panic] {
        let (probe, polls, _, pointer, _) = controlled_catalog_bootstrap_probe(mode).await;
        let state = probe.state.clone();
        let submitted = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
        let queue = submitted.clone();
        *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
        state.schedule();
        let initial = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
        initial();
        assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
        probe.cancel();
        while let Some(successor) = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop() {
            successor();
        }
        *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        state.schedule();
        let result = probe.await.unwrap();
        let (storage, _, _, actual) = result.into_parts().unwrap();
        assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
        assert!(matches!(actual, Err(DbError::Closed) | Err(DbError::Fenced { .. }) | Err(DbError::Unavailable(_))));
    }

    let (probe, polls, waker, pointer, _) = controlled_catalog_bootstrap_probe(ControlledCatalogBootstrapPoll::Pending).await;
    let state = probe.state.clone();
    state.schedule();
    while polls.load(std::sync::atomic::Ordering::Acquire) == 0 {
        std::thread::yield_now();
    }
    probe.cancel();
    waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().unwrap().wake_by_ref();
    let result = probe.await.unwrap();
    let (storage, _, _, actual) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(actual, Err(DbError::Closed));
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_handoff_interruption_retires_unpolled_pages_one_lane_opportunity_at_a_time() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogBootstrapFuture::try_prepare_with_key(pool, storage, catalog_bootstrap_pages(3).await, DatabaseCatalogBootstrapKey::root(), EpochFence::INITIAL, false).unwrap();
    let state = probe.state.clone();
    let submitted = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let queue = submitted.clone();
    *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
    state.schedule();
    submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap()();
    assert_eq!(state.phase(), DatabaseCatalogBootstrapPhase::Poll);
    {
        let work = state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let work = work.as_ref().unwrap();
        assert!(work.future.is_none());
        assert_eq!(work.pages.as_ref().map(db_storage::DbIoPages::page_count), Some(3));
        assert_eq!(work.storage.as_ref().map(|storage| Arc::as_ptr(storage) as usize), Some(pointer));
    }
    probe.cancel();
    while state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
        submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap()();
    }
    let result = probe.await.unwrap();
    let (storage, _, _, actual) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(actual, Err(DbError::Closed));
    assert!(state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_atomic_driver_claim_closes_first_poll_pending_ready_panic_and_retirement_races() {
    let (probe, polls, _, pointer, page_identity) = controlled_catalog_bootstrap_probe(ControlledCatalogBootstrapPoll::Ready).await;
    let state = probe.state.clone();
    let (slot, generation, bytes) = {
        let admission = state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let admission = admission.as_ref().unwrap();
        (admission.slot, admission.generation, admission.bytes)
    };
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let frozen = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let (claimed_tx, claimed_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    let hook_frozen = frozen.clone();
    *state.controlled_driver_claim_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |phase| {
        if phase != DatabaseCatalogBootstrapPhase::Poll || hook_frozen.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        claimed_tx.send(std::thread::current().id()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    state.schedule();
    let driver_thread = claimed_rx.recv().unwrap();
    assert_ne!(driver_thread, std::thread::current().id());
    assert_eq!(state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCatalogBootstrapDriverAuthority::Driving as u8);
    assert!(!state.scheduled.load(std::sync::atomic::Ordering::Acquire));
    {
        let work = state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(work.as_ref().and_then(|work| work.page_identity), Some(page_identity));
        assert_eq!(work.as_ref().map(|work| work.storage_identity), Some(pointer));
    }
    probe.cancel();
    assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 0, "accepted cancellation before the atomic poll claim forbids the backend poll");
    assert_eq!(state.active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let result = probe.await.unwrap();
    let (storage, _, _, actual) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(actual, Err(DbError::Closed));
    assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(!database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(slot).and_then(Option::as_ref).is_some_and(|entry| entry.generation == generation));
    assert!(!DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(slot, generation, bytes));

    for mode in [ControlledCatalogBootstrapPoll::Pending, ControlledCatalogBootstrapPoll::Ready, ControlledCatalogBootstrapPoll::Panic] {
        let (probe, polls, _, pointer, _) = controlled_catalog_bootstrap_probe(mode).await;
        let state = probe.state.clone();
        let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
        let frozen = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (published_tx, published_rx) = std::sync::mpsc::sync_channel(1);
        let hook_gate = gate.clone();
        let hook_frozen = frozen.clone();
        *state.controlled_driver_release_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |_| {
            if hook_frozen.swap(true, std::sync::atomic::Ordering::AcqRel) {
                return;
            }
            published_tx.send(()).unwrap();
            let (lock, ready) = &*hook_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }));
        state.schedule();
        published_rx.recv().unwrap();
        assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
        probe.cancel();
        assert_eq!(state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCatalogBootstrapDriverAuthority::Driving as u8);
        assert_eq!(state.active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
        assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
        {
            let (lock, ready) = &*gate;
            *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
            ready.notify_one();
        }
        let result = probe.await.unwrap();
        let (storage, _, _, actual) = result.into_parts().unwrap();
        assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
        match mode {
            ControlledCatalogBootstrapPoll::Panic => assert!(matches!(actual, Err(DbError::LimitExceeded("database catalog-bootstrap backend poll panic")))),
            _ => assert_eq!(actual, Err(DbError::Closed)),
        }
        assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    }

    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let result_probe = DatabaseCatalogBootstrapFuture::try_submit(test_worker_pool(), storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap();
    let generation = result_probe.generation();
    let state = result_probe.state.clone();
    let result = result_probe.await.unwrap();
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let frozen = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let (retire_tx, retire_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    let hook_frozen = frozen.clone();
    *state.controlled_driver_claim_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |phase| {
        if phase != DatabaseCatalogBootstrapPhase::Terminal || hook_frozen.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        retire_tx.send(()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    drop(result);
    retire_rx.recv().unwrap();
    let terminal = take_database_catalog_bootstrap_terminal(generation).unwrap();
    assert_eq!(terminal.close_step(), DatabaseCatalogBootstrapCloseStep::Blocked);
    assert_eq!(state.active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    while !terminal.terminal_is_empty() {
        terminal.close_step();
        std::thread::yield_now();
    }
    assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCatalogBootstrapDriverAuthority::Idle as u8);
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_lost_handle_take_resume_close_and_terminal_witness_are_exact() {
    let (probe, _, _, pointer, _) = controlled_catalog_bootstrap_probe(ControlledCatalogBootstrapPoll::NoService).await;
    let generation = probe.generation();
    let state = probe.state.clone();
    state.schedule();
    while state.scheduled.load(std::sync::atomic::Ordering::Acquire) || state.polling.load(std::sync::atomic::Ordering::Acquire) {
        std::thread::yield_now();
    }
    drop(probe);
    while state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
        std::thread::yield_now();
    }
    let terminal = take_database_catalog_bootstrap_terminal(generation).unwrap();
    assert_eq!(terminal.witness().generation, generation);
    assert!(terminal.witness().retained_owners > 0);
    let resumed = terminal.resume().unwrap();
    assert_eq!(resumed.generation(), generation);
    drop(resumed);
    let terminal = take_database_catalog_bootstrap_terminal(generation).unwrap();
    let mut previous = terminal.witness().retained_owners;
    while !terminal.terminal_is_empty() {
        let step = terminal.close_step();
        assert!(matches!(step, DatabaseCatalogBootstrapCloseStep::Progress | DatabaseCatalogBootstrapCloseStep::Blocked));
        let current = terminal.witness().retained_owners;
        assert!(previous.saturating_sub(current) <= 1, "one mounted close grant retires at most one owner");
        previous = current;
        std::thread::yield_now();
    }
    assert_eq!(state.retained_owner_count(), 0);
    assert_eq!(state.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|storage| Arc::as_ptr(storage) as usize), None);
    assert_ne!(pointer, 0);
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_backend_no_service_close_retires_only_on_io_lane() {
    let (probe, polls, _, _, _) = controlled_catalog_bootstrap_probe(ControlledCatalogBootstrapPoll::NoService).await;
    let generation = probe.generation();
    let state = probe.state.clone();
    state.schedule();
    while polls.load(std::sync::atomic::Ordering::Acquire) == 0 {
        std::thread::yield_now();
    }
    assert!(state.poll_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    drop(probe);
    while state.scheduled.load(std::sync::atomic::Ordering::Acquire) || state.polling.load(std::sync::atomic::Ordering::Acquire) {
        std::thread::yield_now();
    }
    let terminal = take_database_catalog_bootstrap_terminal(generation).unwrap();
    while !terminal.terminal_is_empty() {
        terminal.close_step();
        std::thread::yield_now();
    }
    assert!(terminal.witness().terminal_empty);
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_stale_generation_fault_preserves_storage_pages_and_current_slot() {
    let (probe, _, _, pointer, _) = controlled_catalog_bootstrap_probe(ControlledCatalogBootstrapPoll::Pending).await;
    let state = probe.state.clone();
    let stale_generation = probe.generation();
    let replacement_generation = stale_generation.checked_add(1).unwrap();
    {
        let mut admission = DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        admission.slots[state.slot].generation = replacement_generation;
    }
    state.schedule();
    let result = probe.await.unwrap();
    let (storage, _, _, actual) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(actual, Err(DbError::StaleGeneration { expected: GenerationId(stale_generation), actual: GenerationId(replacement_generation) }));
    {
        let mut admission = DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(admission.slots[state.slot].generation, replacement_generation, "stale authority cannot release or rewrite the replacement generation");
        let bytes = admission.slots[state.slot].bytes;
        assert!(admission.release(state.slot, replacement_generation, bytes));
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_real_queue_saturation_retains_exact_job_and_recovers_identity() {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (started_tx, started_rx) = std::sync::mpsc::sync_channel(1);
    let worker_gate = gate.clone();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            let (lock, ready) = &*worker_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    )
    .ok()
    .expect("catalog-bootstrap blocker admission");
    started_rx.recv().unwrap();
    loop {
        if let Err(error) = pool.try_submit(Lane::Io, Box::new(|| {})) {
            assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
            drop(error.into_job());
            break;
        }
    }
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogBootstrapFuture::try_submit(pool.clone(), storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap();
    assert!(probe.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert_eq!(probe.state.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|storage| Arc::as_ptr(storage) as usize), Some(pointer));
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let result = probe.await.unwrap();
    let (storage, _, _, actual) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(actual.unwrap(), EpochFence::INITIAL.next());
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_replay_is_deterministic_and_never_reuses_initial_after_a_winner() {
    for _ in 0..2 {
        let pool = test_worker_pool();
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
        let installed = DatabaseCatalogBootstrapFuture::try_submit(pool.clone(), storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap().await.unwrap();
        let (storage, _, _, actual) = installed.into_parts().unwrap();
        assert_eq!(actual.unwrap(), EpochFence::INITIAL.next());
        let replay = DatabaseCatalogBootstrapFuture::try_submit(pool, storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap().await.unwrap();
        assert_eq!(replay.into_parts().unwrap().3, Err(DbError::Fenced { expected: 1, actual: 0 }));
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_publication_race_and_queue_pressure_keep_exact_successor() {
    let (mut probe, polls, retained_waker, _, _) = controlled_catalog_bootstrap_probe(ControlledCatalogBootstrapPoll::Ready).await;
    let state = probe.state.clone();
    let submitted = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let queue = submitted.clone();
    *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
    state.schedule();
    let initial = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
    initial();
    assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
    retained_waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().unwrap().wake_by_ref();
    assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1, "wake storms retain one exact successor");
    let successor = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
    successor();
    assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1, "Ready is never repolled by its phase successor");
    for _ in 0..3 {
        let phase = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
        phase();
    }
    assert_eq!(state.phase(), DatabaseCatalogBootstrapPhase::Publish);
    assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1);
    let wake = Arc::new(ControlledCatalogPublicWake(std::sync::atomic::AtomicUsize::new(0)));
    let waker = std::task::Waker::from(wake.clone());
    let mut context = std::task::Context::from_waker(&waker);
    let queue = submitted.clone();
    *state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
        let publish = queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
        publish();
    }));
    let published = std::pin::Pin::new(&mut probe).poll(&mut context);
    assert!(matches!(published, std::task::Poll::Ready(Ok(_))), "check-register-recheck must observe publication between the first check and waker registration");
    assert!(wake.0.load(std::sync::atomic::Ordering::Acquire) <= 1);
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_bootstrap_public_result_drop_hands_back_exact_owner_without_post_admission_allocation() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogBootstrapFuture::try_submit(pool, storage, empty_catalog_bootstrap_pages().await, EpochFence::INITIAL).unwrap();
    let generation = probe.generation();
    let state = probe.state.clone();
    let result = probe.await.unwrap();
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    let submitted = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let queue = submitted.clone();
    *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
    drop(result);
    assert_eq!(submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, 1);
    {
        let completion = state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = completion.as_ref().and_then(|result| result.as_ref().ok()).unwrap();
        assert!(owner.state.is_none());
        assert_eq!(owner.storage.as_ref().map(|storage| Arc::as_ptr(storage) as usize), Some(pointer));
    }
    let terminal = take_database_catalog_bootstrap_terminal(generation).unwrap();
    assert_eq!(terminal.witness().generation, generation);
    let retained = terminal.take_result().unwrap().take().unwrap().unwrap();
    let (storage, key, expected, actual) = retained.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(key, DatabaseCatalogBootstrapKey::root());
    assert_eq!(expected, EpochFence::INITIAL);
    assert_eq!(actual.unwrap(), EpochFence::INITIAL.next());
    submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap()();
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_paused_transfer_blocks_public_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["driveOwnership"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
        let empty = terminal.terminal_is_empty() && database_capability_open_registry().lock().unwrap()[state.slot].is_none();
        eprintln!("[DEBUG] capability-drive-ownership: {} first={first:?} admission-retained={retained} exact-storage={exact_storage} final-empty={empty}", row["name"]);
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["leaseCompletion"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
        eprintln!("[DEBUG] capability-finalizer-handoff: {} admission-retained={retained} retirement-submissions={retirement_submissions} terminal-empty={}", row["name"], state.terminal_is_empty());
        assert_eq!(retirement_submissions, row["retirementSubmissions"].as_u64().unwrap() as usize);
        assert!(state.terminal_is_empty());
        assert!(state.controlled_submit_refusal.lock().unwrap().is_none());
        assert!(state.terminal_job.lock().unwrap().is_none());
        assert!(state.completion.lock().unwrap().is_none());
        assert!(database_capability_open_registry().lock().unwrap()[state.slot].is_none());
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
        eprintln!("[DEBUG] capability-lease-completion: {} admission-during-publication={retained} closed=true late-submissions=0", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_completion_interleavings_preserve_result_and_wake() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["publication"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
        assert!(database_capability_open_registry().lock().unwrap()[state.slot].is_none(), "exact capability registry slot must be released");
        eprintln!("[DEBUG] capability-completion: {} first-ready={first_ready} wakes={} waiter-empty={waiter_empty} abandoned={abandoned}", row["name"], wake.0.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(first_ready, row["firstReady"].as_bool().unwrap(), "completion in check-to-registration window must be observed in the same poll");
        assert_eq!(wake.0.load(std::sync::atomic::Ordering::Acquire) as u64, row["wakes"].as_u64().unwrap());
        assert!(waiter_empty, "Ready must retire its transient waiter");
        assert_eq!(abandoned, fault, "fault bookkeeping must match the fast Ready path");
    }
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_consumed_completion_retires_before_publisher_wake() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📬️capability-completion/🔣️.json")).unwrap();
    for row in fixture["retirement"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
        let terminal_before_wake = state.terminal_is_empty() && database_capability_open_registry().lock().unwrap()[state.slot].is_none();
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
        assert!(database_capability_open_registry().lock().unwrap()[state.slot].is_none(), "exact capability registry slot must be released");
        eprintln!("[DEBUG] capability-completion: {} terminal-before-wake={terminal_before_wake} wakes={}", row["name"], wake.0.load(std::sync::atomic::Ordering::Acquire));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    assert!(database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[terminal.state.slot].is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    eprintln!("[DEBUG] capability-cleanup: cancellation exact-storage=true actor/public-grants<=1 registry-empty=true");

    let stale_storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    eprintln!("[DEBUG] capability-cleanup: stale-generation exact-storage=true actor/public-grants<=1 registry-empty=true");
}

#[semio_framework_async_macros::async_test]
async fn database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal() {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (started_tx, started_rx) = std::sync::mpsc::sync_channel(1);
    let worker_gate = gate.clone();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).expect("fixture start handoff");
            let (lock, ready) = &*worker_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    )
    .ok()
    .expect("fixture blocker admission");
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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

    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage);
    let mut probe = DatabaseCapabilityOpenFuture::try_prepare(test_worker_pool(), storage.clone(), false).expect("terminal-result preparation");
    probe.resolved = true;
    probe.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
    *probe.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
        Some(Ok(DatabaseCapabilityOpenResult { storage, capabilities: db_storage::StorageCapabilities { durable: false, max_durability: DurabilityClass::Memory, supports_fsync: false, supports_cas: true } }));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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

fn control_catalog_read_submissions(state: &Arc<DatabaseCatalogReadState>) -> Arc<Mutex<ControlledCapabilitySubmitQueue>> {
    let queue = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let captured = queue.clone();
    *state.controlled_submit_hook.lock().unwrap() = Some(Arc::new(move |job| captured.lock().unwrap().push(job)));
    queue
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_paused_transfers_exclude_successors_and_public_cleanup() {
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️catalog-read-ownership/🔣️.json")).unwrap();
    for row in fixture["transfers"].as_array().unwrap() {
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
        let empty = terminal.terminal_is_empty() && database_catalog_read_registry().lock().unwrap()[state.slot].is_none();
        eprintln!("[DEBUG] catalog-read-transfer: {} first={first:?} resume-blocked={resume_blocked} submissions-active={submissions_while_active} admission-retained={retained} exact-storage={exact_storage} final-empty={empty}", row["name"]);
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage.clone(), DatabaseCatalogRootKey::root(), false).unwrap();
    let bytes: Vec<u8> = fixture["root"]["bytes"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let (root, operation) = if outcome == "pages" {
        let pages = db_storage::db_io_copy_pages(&bytes).unwrap().await.unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️catalog-read-ownership/🔣️.json")).unwrap();
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
        let empty = state.terminal_is_empty() && database_catalog_read_registry().lock().unwrap()[state.slot].is_none();
        eprintln!("[DEBUG] catalog-read-recovery: {} exact-storage-key-root=true terminal-empty={empty}", row["name"]);
        assert_eq!(empty, row["terminalEmpty"].as_bool().unwrap());
        assert!(!state.retry_armed.load(Ordering::Acquire));
        assert!(!state.terminal_result_checked_out.load(Ordering::Acquire));
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_consumed_publication_preserves_exact_root_and_retires() {
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️catalog-read-ownership/🔣️.json")).unwrap();
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
        eprintln!("[DEBUG] catalog-read-publication: {} exact-storage-key-root=true admission-retained={retained} retirement-submissions={retirement_submissions} terminal-empty={empty}", row["name"]);
        assert_eq!(retained, row["admissionDuringPublication"].as_bool().unwrap());
        assert_eq!(retirement_submissions, row["retirementSubmissions"].as_u64().unwrap() as usize);
        assert_eq!(empty, row["terminalEmpty"].as_bool().unwrap());
        assert!(state.controlled_submit_refusal.lock().unwrap().is_none());
        assert!(state.terminal_job.lock().unwrap().is_none());
        assert!(state.completion.lock().unwrap().is_none());
        assert!(database_catalog_read_registry().lock().unwrap()[state.slot].is_none());
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut probe = DatabaseCatalogReadFuture::try_prepare(test_worker_pool(), storage, DatabaseCatalogRootKey::root(), false).expect("public lost-wake preparation");
    let state = probe.state.clone();
    let mut work = state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().expect("public lost-wake retained work");
    assert!(work.close_step());
    assert!(work.terminal_is_empty());
    drop(work);

    let result_storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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
        assert!(database_catalog_read_registry().lock().unwrap()[state.slot].is_none());
        eprintln!("[DEBUG] catalog-read-interruption: stale={stale} exact-storage=true bounded-close=true registry-empty=true");
    }
}

#[semio_framework_async_macros::async_test]
async fn database_catalog_read_terminal_result_drop_hands_back_exact_result() {
    let (mut probe, _, _, _) = controlled_catalog_read_probe(ControlledCatalogReadPoll::Ready).await;
    probe.resolved = true;
    probe.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
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

#[semio_framework_async_macros::async_test]
async fn hash_operation_text_and_binary_round_trip_with_every_field_present_and_absent() {
    let bare = HashMutation { hash: [7u8; 32], author: None, timestamp: None };
    assert_eq!(HashMutation::parse_op(&bare.print_op()).unwrap().hash, bare.hash);
    assert!(HashMutation::parse_op(&bare.print_op()).unwrap().author.is_none());
    assert_eq!(HashMutation::decode_op(&bare.encode_op().unwrap()).unwrap(), bare);

    let full = HashMutation { hash: [9u8; 32], author: Some(protocol::ActorId("actor-1".into())), timestamp: Some(protocol::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 }) };
    let reparsed = HashMutation::parse_op(&full.print_op()).unwrap();
    assert_eq!(reparsed.hash, full.hash);
    assert_eq!(reparsed.author, full.author);
    assert_eq!(reparsed.timestamp, full.timestamp);
    let redecoded = HashMutation::decode_op(&full.encode_op().unwrap()).unwrap();
    assert_eq!(redecoded, full);
}

#[semio_framework_async_macros::async_test]
async fn hash_projection_pack_round_trips() {
    let projection = HashProjection { latest_hash: [3u8; 32] };
    let bytes = projection.encode_pack();
    assert_eq!(HashProjection::decode_pack(&bytes).unwrap(), projection);
}

//#region 🧸️Fixtures
async fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!("db_engine-test-{name}-{}-{}", std::process::id(), now_ms().await));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

async fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::ArtifactId, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let mut payload = serde_json::Map::new();
    for (path, value) in entries {
        payload.insert((*path).to_string(), value.clone());
    }
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact::encode_pathmap_json(&serde_json::Value::Object(serde_json::Map::new())).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

async fn create_catalog_fixture(entries: Vec<CatalogEntry>) -> (Arc<db_storage::DbBackend>, Arc<Mutex<CatalogState>>, EpochFence) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let pages = encode_catalog_pages(&entries).await.unwrap();
    let epoch = storage.catalog().await.cas_root(EpochFence::INITIAL, pages).await.unwrap();
    let catalog = Arc::new(Mutex::new(CatalogState { epoch, revision: 1, entries: Arc::new(entries), pending: None }));
    (storage, catalog, epoch)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlledCreateCatalogPoll {
    Pending,
    Ready,
    Panic,
}

struct ControlledCreateCatalogFuture {
    mode: ControlledCreateCatalogPoll,
    pages: Option<db_storage::DbIoPages>,
    ready_epoch: EpochFence,
    cancel_on_ready: Option<std::sync::Weak<DatabaseCreateCatalogState>>,
    polls: Arc<std::sync::atomic::AtomicUsize>,
    waker: Arc<Mutex<Option<std::task::Waker>>>,
}

impl Future for ControlledCreateCatalogFuture {
    type Output = Result<EpochFence, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        match self.mode {
            ControlledCreateCatalogPoll::Pending => std::task::Poll::Pending,
            ControlledCreateCatalogPoll::Ready => {
                self.pages.take();
                if let Some(state) = self.cancel_on_ready.as_ref().and_then(std::sync::Weak::upgrade) {
                    state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                }
                std::task::Poll::Ready(Ok(self.ready_epoch))
            }
            ControlledCreateCatalogPoll::Panic => panic!("controlled create-catalog backend panic"),
        }
    }
}

fn held_create_catalog_io_pool() -> (Arc<WorkerPool>, Arc<(Mutex<bool>, std::sync::Condvar)>) {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (started_tx, started_rx) = std::sync::mpsc::sync_channel(1);
    let worker_gate = gate.clone();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            let (lock, ready) = &*worker_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    )
    .ok()
    .expect("create-catalog blocker admission");
    started_rx.recv().unwrap();
    loop {
        if let Err(error) = pool.try_submit(Lane::Io, Box::new(|| {})) {
            assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
            drop(error.into_job());
            break;
        }
    }
    (pool, gate)
}

fn replenishing_create_catalog_io_job(pool: Arc<WorkerPool>, active: Arc<std::sync::atomic::AtomicBool>) -> semio_framework_async::Job {
    Box::new(move || {
        if active.load(std::sync::atomic::Ordering::Acquire) {
            let next = replenishing_create_catalog_io_job(pool.clone(), active.clone());
            if let Err(error) = pool.try_submit(Lane::Io, next) {
                drop(error.into_job());
            }
        }
    })
}

fn replenishing_held_create_catalog_io_pool() -> (Arc<WorkerPool>, Arc<(Mutex<bool>, std::sync::Condvar)>, Arc<std::sync::atomic::AtomicBool>) {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let active = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let (started_tx, started_rx) = std::sync::mpsc::sync_channel(1);
    let worker_gate = gate.clone();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            let (lock, ready) = &*worker_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    )
    .ok()
    .expect("create-catalog replenishing blocker admission");
    started_rx.recv().unwrap();
    loop {
        let job = replenishing_create_catalog_io_job(pool.clone(), active.clone());
        if let Err(error) = pool.try_submit(Lane::Io, job) {
            assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
            drop(error.into_job());
            break;
        }
    }
    (pool, gate, active)
}

fn reserved_replenishing_create_catalog_io_pool() -> (Arc<WorkerPool>, Arc<(Mutex<bool>, std::sync::Condvar)>, Arc<(Mutex<bool>, std::sync::Condvar)>, Arc<std::sync::atomic::AtomicBool>) {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    assert_eq!(pool.worker_count(), 2);
    let maintenance_gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let service_gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let active = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let (maintenance_tx, maintenance_rx) = std::sync::mpsc::sync_channel(1);
    let held_maintenance = maintenance_gate.clone();
    pool.try_submit(
        Lane::Maintenance,
        Box::new(move || {
            maintenance_tx.send(()).unwrap();
            let (lock, ready) = &*held_maintenance;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    )
    .ok()
    .expect("create-catalog maintenance blocker admission");
    maintenance_rx.recv().unwrap();
    let (service_tx, service_rx) = std::sync::mpsc::sync_channel(1);
    let held_service = service_gate.clone();
    pool.try_submit(
        Lane::UserVisible,
        Box::new(move || {
            service_tx.send(()).unwrap();
            let (lock, ready) = &*held_service;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    )
    .ok()
    .expect("create-catalog service blocker admission");
    service_rx.recv().unwrap();
    loop {
        let job = replenishing_create_catalog_io_job(pool.clone(), active.clone());
        if let Err(error) = pool.try_submit(Lane::Io, job) {
            assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
            drop(error.into_job());
            break;
        }
    }
    (pool, maintenance_gate, service_gate, active)
}

fn release_held_create_catalog_worker(gate: &Arc<(Mutex<bool>, std::sync::Condvar)>) {
    let (lock, ready) = &**gate;
    *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
    ready.notify_one();
}
//#endregion 🧸️Fixtures

//#region 🔖️Database open/catalog
#[semio_framework_async_macros::async_test]
async fn database_create_catalog_max_plus_one_document_and_entry_caps_return_exact_owners() {
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let pointer = Arc::as_ptr(&storage) as usize;
    let mut text = String::with_capacity(DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1);
    text.push('x');
    let capacity = text.capacity();
    let rejected = match DatabaseCreateCatalogFuture::try_submit(test_worker_pool(), catalog, storage, protocol::ArtifactId(text)) {
        Ok(_) => panic!("oversized create-catalog document was admitted"),
        Err(rejected) => rejected,
    };
    let (error, storage, document) = rejected.into_parts().unwrap();
    assert_eq!(error, DbError::LimitExceeded("database create-catalog document bytes"));
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(document.0.capacity(), capacity);

    let entries = (0..DATABASE_CREATE_CATALOG_MAX_ENTRIES).map(|index| CatalogEntry { document: protocol::ArtifactId(format!("max-{index}")), created_at_ms: index as u64 }).collect();
    let catalog = Arc::new(Mutex::new(CatalogState { epoch: EpochFence::INITIAL, revision: 1, entries: Arc::new(entries), pending: None }));
    let rejected = match DatabaseCreateCatalogFuture::try_submit(test_worker_pool(), catalog, storage, protocol::ArtifactId(String::from("max+1"))) {
        Ok(_) => panic!("create-catalog entry max+1 was admitted"),
        Err(rejected) => rejected,
    };
    let (error, storage, document) = rejected.into_parts().unwrap();
    assert_eq!(error, DbError::LimitExceeded("database create-catalog entry capacity"));
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(document.0, "max+1");
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_observed_vec_and_string_overallocation_faults_retire_exact_backings() {
    let (vector_storage, vector_catalog, _) = create_catalog_fixture(Vec::new()).await;
    let vector_pointer = Arc::as_ptr(&vector_storage) as usize;
    let vector = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), vector_catalog, vector_storage, protocol::ArtifactId(String::from("vector-overallocation")), false).unwrap();
    let vector_state = vector.state.clone();
    vector_state.controlled_capacity_overage.store(DATABASE_CREATE_CATALOG_ITEMS as usize + 1, std::sync::atomic::Ordering::Release);
    vector_state.set_phase(DatabaseCreateCatalogPhase::Reserve);
    vector_state.reserve_candidate_one();
    assert!(vector_state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).candidate.is_some());
    vector_state.schedule();
    let (storage, document, _, actual) = vector.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, vector_pointer);
    assert_eq!(document.0, "vector-overallocation");
    assert_eq!(actual, Err(DbError::LimitExceeded("database create-catalog observed backing capacity")));
    assert!(vector_state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).candidate.is_none());
    assert!(vector_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());

    let (string_storage, string_catalog, _) = create_catalog_fixture(Vec::new()).await;
    let string_pointer = Arc::as_ptr(&string_storage) as usize;
    let string = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), string_catalog, string_storage, protocol::ArtifactId(String::from("string-overallocation")), false).unwrap();
    let string_state = string.state.clone();
    string_state.set_phase(DatabaseCreateCatalogPhase::Reserve);
    string_state.reserve_candidate_one();
    string_state.controlled_capacity_overage.store(DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1, std::sync::atomic::Ordering::Release);
    string_state.clone_one();
    assert!(string_state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone_text.is_some());
    string_state.schedule();
    let (storage, document, _, actual) = string.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, string_pointer);
    assert_eq!(document.0, "string-overallocation");
    assert_eq!(actual, Err(DbError::LimitExceeded("database create-catalog cloned string capacity")));
    assert!(string_state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone_text.is_none());
    assert!(string_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_large_tree_yields_scan_copy_encode_seal_and_publishes_exact_epoch() {
    let entries = (0..128).map(|index| CatalogEntry { document: protocol::ArtifactId(format!("large-{index:04}")), created_at_ms: index as u64 }).collect();
    let (storage, catalog, epoch) = create_catalog_fixture(entries).await;
    let pointer = Arc::as_ptr(&storage) as usize;
    let document = protocol::ArtifactId(String::from("large-new"));
    let probe = DatabaseCreateCatalogFuture::try_submit(test_worker_pool(), catalog.clone(), storage, document).unwrap();
    let state = probe.state.clone();
    let result = probe.await.unwrap();
    let (storage, document, expected, actual) = result.into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(document.0, "large-new");
    assert_eq!(expected, epoch);
    assert_eq!(actual, Ok(epoch.next()));
    assert!(state.opportunities.load(std::sync::atomic::Ordering::Acquire) > 128 * 3);
    let catalog = catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_eq!(catalog.entries.len(), 129);
    assert_eq!(catalog.epoch, epoch.next());
    assert_eq!(catalog.revision, 2);
    assert!(catalog.pending.is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_duplicate_and_concurrent_same_base_are_deterministic() {
    let existing = CatalogEntry { document: protocol::ArtifactId(String::from("existing")), created_at_ms: 1 };
    let (storage, catalog, epoch) = create_catalog_fixture(vec![existing]).await;
    let duplicate = DatabaseCreateCatalogFuture::try_submit(test_worker_pool(), catalog.clone(), storage.clone(), protocol::ArtifactId(String::from("existing"))).unwrap();
    let (_, _, expected, actual) = duplicate.await.unwrap().into_parts().unwrap();
    assert_eq!(expected, epoch);
    assert!(matches!(actual, Err(DbError::AlreadyExists(_))));
    assert_eq!(catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner).revision, 1);

    let first = DatabaseCreateCatalogFuture::try_submit(test_worker_pool(), catalog.clone(), storage.clone(), protocol::ArtifactId(String::from("same"))).unwrap();
    let second = DatabaseCreateCatalogFuture::try_submit(test_worker_pool(), catalog.clone(), storage, protocol::ArtifactId(String::from("same"))).unwrap();
    let first = first.await.unwrap().into_parts().unwrap().3;
    let second = second.await.unwrap().into_parts().unwrap().3;
    assert_eq!(usize::from(first.is_ok()) + usize::from(second.is_ok()), 1);
    assert!(matches!(first, Ok(_) | Err(DbError::Fenced { .. })));
    assert!(matches!(second, Ok(_) | Err(DbError::Fenced { .. })));
    let catalog = catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_eq!(catalog.entries.iter().filter(|entry| entry.document.0 == "same").count(), 1);
    assert_eq!(catalog.revision, 2);
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_cancel_deadline_and_generation_aba_preserve_exact_identity() {
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog.clone(), storage.clone(), protocol::ArtifactId(String::from("cancel")), false).unwrap();
    probe.cancel();
    let (_, document, _, actual) = probe.await.unwrap().into_parts().unwrap();
    assert_eq!(document.0, "cancel");
    assert_eq!(actual, Err(DbError::Closed));

    let deadline = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog.clone(), storage.clone(), protocol::ArtifactId(String::from("deadline")), false).unwrap();
    deadline.state.deadline_ms.store(0, std::sync::atomic::Ordering::Release);
    deadline.state.schedule();
    let (_, document, _, actual) = deadline.await.unwrap().into_parts().unwrap();
    assert_eq!(document.0, "deadline");
    assert!(matches!(actual, Err(DbError::Timeout(_))));

    let stale = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog, storage, protocol::ArtifactId(String::from("stale")), false).unwrap();
    let state = stale.state.clone();
    let replacement = state.generation.checked_add(1).unwrap();
    DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = replacement;
    state.schedule();
    let (storage, document, _, actual) = stale.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(document.0, "stale");
    assert_eq!(actual, Err(DbError::StaleGeneration { expected: GenerationId(state.generation), actual: GenerationId(replacement) }));
    DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(state.slot, replacement);
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_handoff_cancel_claim_prevents_backend_poll_and_retires_exact_pages() {
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog, storage, protocol::ArtifactId(String::from("handoff-cancel")), false).unwrap();
    let state = probe.state.clone();
    let pages = catalog_bootstrap_pages(3).await;
    let operation = pages.operation();
    state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pages = Some(pages);
    state.set_phase(DatabaseCreateCatalogPhase::Handoff);
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let frozen = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let (claimed_tx, claimed_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    let hook_frozen = frozen.clone();
    *state.controlled_driver_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |phase| {
        if phase != DatabaseCreateCatalogPhase::Handoff || hook_frozen.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        claimed_tx.send(std::thread::current().id()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    state.schedule();
    let driver_thread = claimed_rx.recv().unwrap();
    assert_ne!(driver_thread, std::thread::current().id());
    assert_eq!(state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Driving as u8);
    assert_eq!(state.active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pages.as_ref().map(db_storage::DbIoPages::operation), Some(operation));
    probe.cancel();
    assert!(!state.polling.load(std::sync::atomic::Ordering::Acquire));
    assert!(state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let (storage, document, _, actual) = probe.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(document.0, "handoff-cancel");
    assert_eq!(actual, Err(DbError::Closed));
    assert_eq!(state.max_active_drivers.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pages.is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_pending_ready_and_panic_publish_work_before_driver_release() {
    for mode in [ControlledCreateCatalogPoll::Pending, ControlledCreateCatalogPoll::Ready, ControlledCreateCatalogPoll::Panic] {
        let (storage, catalog, epoch) = create_catalog_fixture(Vec::new()).await;
        let pointer = Arc::as_ptr(&storage) as usize;
        let probe = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog, storage, protocol::ArtifactId(format!("mode-{}", mode as u8)), false).unwrap();
        let state = probe.state.clone();
        let mut writer = db_storage::DbIoPageWriter::try_reserve(1).unwrap();
        writer.write_fragment(b"[]").unwrap();
        let pages = writer.seal_retained().await.unwrap();
        let operation = pages.operation();
        let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let waker = Arc::new(Mutex::new(None));
        let future = ControlledCreateCatalogFuture { mode, pages: Some(pages), ready_epoch: epoch.next(), cancel_on_ready: (mode == ControlledCreateCatalogPoll::Ready).then(|| Arc::downgrade(&state)), polls: polls.clone(), waker: waker.clone() };
        *state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCreateCatalogWork::controlled(Box::pin(future), pointer, operation));
        state.set_phase(DatabaseCreateCatalogPhase::Poll);
        state.schedule();
        while polls.load(std::sync::atomic::Ordering::Acquire) == 0 {
            std::thread::yield_now();
        }
        assert_eq!(polls.load(std::sync::atomic::Ordering::Acquire), 1);
        assert!(state.poll_worker_thread.load(std::sync::atomic::Ordering::Acquire));
        if mode == ControlledCreateCatalogPoll::Pending {
            probe.cancel();
        }
        let (storage, _, _, actual) = probe.await.unwrap().into_parts().unwrap();
        assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
        match mode {
            ControlledCreateCatalogPoll::Pending => assert_eq!(actual, Err(DbError::Closed)),
            ControlledCreateCatalogPoll::Ready => assert_eq!(actual, Ok(epoch.next())),
            ControlledCreateCatalogPoll::Panic => assert_eq!(actual, Err(DbError::LimitExceeded("database create-catalog backend poll panic"))),
        }
        assert!(!state.polling.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        assert!(state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        assert!(state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    }
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_saturation_retains_exact_job_and_recovers() {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (started_tx, started_rx) = std::sync::mpsc::sync_channel(1);
    let worker_gate = gate.clone();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            let (lock, ready) = &*worker_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    )
    .ok()
    .expect("create-catalog saturation blocker admission");
    started_rx.recv().unwrap();
    loop {
        if let Err(error) = pool.try_submit(Lane::Io, Box::new(|| {})) {
            assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
            drop(error.into_job());
            break;
        }
    }
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCreateCatalogFuture::try_submit(pool.clone(), catalog, storage, protocol::ArtifactId(String::from("saturated"))).unwrap();
    assert!(probe.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert_eq!(probe.state.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|storage| Arc::as_ptr(storage) as usize), Some(pointer));
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let (storage, _, _, actual) = probe.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert!(actual.is_ok());
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_real_worker_loop_services_finite_saturation_cancel_deadline_exhaustion_and_close() {
    let (cancel_storage, cancel_catalog, _) = create_catalog_fixture(Vec::new()).await;
    let (deadline_storage, deadline_catalog, _) = create_catalog_fixture(Vec::new()).await;
    let (exhaust_storage, exhaust_catalog, _) = create_catalog_fixture(Vec::new()).await;

    let (pool, gate, active) = replenishing_held_create_catalog_io_pool();
    let cancel_pointer = Arc::as_ptr(&cancel_storage) as usize;
    let cancel = DatabaseCreateCatalogFuture::try_submit(pool.clone(), cancel_catalog, cancel_storage, protocol::ArtifactId(String::from("saturation-cancel"))).unwrap();
    let cancel_state = cancel.state.clone();
    cancel.cancel();
    release_held_create_catalog_worker(&gate);
    let (storage, document, _, actual) = cancel.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, cancel_pointer);
    assert_eq!(document.0, "saturation-cancel");
    assert_eq!(actual, Err(DbError::Closed));
    assert_eq!(cancel_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(cancel_state.backend_polls.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(cancel_state.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(cancel_state.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(cancel_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    active.store(false, std::sync::atomic::Ordering::Release);
    pool.shutdown();

    let (pool, gate, active) = replenishing_held_create_catalog_io_pool();
    let deadline_pointer = Arc::as_ptr(&deadline_storage) as usize;
    let deadline = DatabaseCreateCatalogFuture::try_submit(pool.clone(), deadline_catalog, deadline_storage, protocol::ArtifactId(String::from("saturation-deadline"))).unwrap();
    let deadline_state = deadline.state.clone();
    deadline_state.deadline_ms.store(0, std::sync::atomic::Ordering::Release);
    release_held_create_catalog_worker(&gate);
    let (storage, document, _, actual) = deadline.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, deadline_pointer);
    assert_eq!(document.0, "saturation-deadline");
    assert!(matches!(actual, Err(DbError::Timeout(_))));
    assert_eq!(deadline_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(deadline_state.backend_polls.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(deadline_state.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(deadline_state.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(deadline_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    active.store(false, std::sync::atomic::Ordering::Release);
    pool.shutdown();

    let (pool, gate, active) = replenishing_held_create_catalog_io_pool();
    let exhaust_pointer = Arc::as_ptr(&exhaust_storage) as usize;
    let exhaust = DatabaseCreateCatalogFuture::try_submit(pool.clone(), exhaust_catalog, exhaust_storage, protocol::ArtifactId(String::from("saturation-exhaust"))).unwrap();
    let exhaust_state = exhaust.state.clone();
    let exhaust_generation = exhaust.generation();
    release_held_create_catalog_worker(&gate);
    let (storage, document, _, actual) = exhaust.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, exhaust_pointer);
    assert_eq!(document.0, "saturation-exhaust");
    assert_eq!(actual, Err(DbError::LimitExceeded("database create-catalog retry exhausted")));
    assert_eq!(exhaust_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    assert_eq!(exhaust_state.backend_polls.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(exhaust_state.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(exhaust_state.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(exhaust_state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(exhaust_state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(exhaust_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(!database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter_map(Option::as_ref).any(|state| state.generation == exhaust_generation));
    for _ in 0..32 {
        std::thread::yield_now();
    }
    assert_eq!(exhaust_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    active.store(false, std::sync::atomic::Ordering::Release);
    pool.shutdown();

    let (pool, gate, active) = replenishing_held_create_catalog_io_pool();
    let rejection_storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let rejection_pointer = Arc::as_ptr(&rejection_storage) as usize;
    let oversized = protocol::ArtifactId(String::with_capacity(DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1));
    let rejected = match DatabaseCreateCatalogFuture::try_submit(pool.clone(), Arc::new(Mutex::new(CatalogState { epoch: EpochFence::INITIAL, revision: 1, entries: Arc::new(Vec::new()), pending: None })), rejection_storage, oversized) {
        Ok(_) => panic!("oversized create-catalog rejection was admitted"),
        Err(rejected) => rejected,
    };
    let close = rejected.close.clone();
    assert_eq!(close.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().and_then(|owner| owner.storage.as_ref()).map(|storage| Arc::as_ptr(storage) as usize), Some(rejection_pointer));
    assert_eq!(rejected.close_and_take_error(), DbError::LimitExceeded("database create-catalog document bytes"));
    release_held_create_catalog_worker(&gate);
    while !close.terminal_is_empty() {
        std::thread::yield_now();
    }
    assert!(close.terminal_is_empty());
    assert_eq!(close.submission_refusals.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    assert!(close.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(close.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    active.store(false, std::sync::atomic::Ordering::Release);
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_two_worker_reserved_capacity_services_timers_while_one_violator_is_held() {
    let (cancel_storage, cancel_catalog, _) = create_catalog_fixture(Vec::new()).await;
    let (deadline_storage, deadline_catalog, _) = create_catalog_fixture(Vec::new()).await;
    let (exhaust_storage, exhaust_catalog, _) = create_catalog_fixture(Vec::new()).await;

    let (pool, maintenance_gate, service_gate, active) = reserved_replenishing_create_catalog_io_pool();
    let cancel_pointer = Arc::as_ptr(&cancel_storage) as usize;
    let cancel = DatabaseCreateCatalogFuture::try_submit(pool.clone(), cancel_catalog, cancel_storage, protocol::ArtifactId(String::from("reserved-cancel"))).unwrap();
    let cancel_state = cancel.state.clone();
    assert_eq!(cancel_state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Retry as u8);
    assert!(cancel_state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    cancel.cancel();
    release_held_create_catalog_worker(&service_gate);
    let (storage, document, _, actual) = cancel.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, cancel_pointer);
    assert_eq!(document.0, "reserved-cancel");
    assert_eq!(actual, Err(DbError::Closed));
    assert!(!*maintenance_gate.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
    assert_eq!(cancel_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(cancel_state.backend_polls.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(cancel_state.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(cancel_state.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(cancel_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    active.store(false, std::sync::atomic::Ordering::Release);
    release_held_create_catalog_worker(&maintenance_gate);
    pool.shutdown();

    let (pool, maintenance_gate, service_gate, active) = reserved_replenishing_create_catalog_io_pool();
    let deadline_pointer = Arc::as_ptr(&deadline_storage) as usize;
    let deadline = DatabaseCreateCatalogFuture::try_submit(pool.clone(), deadline_catalog, deadline_storage, protocol::ArtifactId(String::from("reserved-deadline"))).unwrap();
    let deadline_state = deadline.state.clone();
    assert_eq!(deadline_state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Retry as u8);
    deadline_state.deadline_ms.store(0, std::sync::atomic::Ordering::Release);
    release_held_create_catalog_worker(&service_gate);
    let (storage, document, _, actual) = deadline.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, deadline_pointer);
    assert_eq!(document.0, "reserved-deadline");
    assert!(matches!(actual, Err(DbError::Timeout(_))));
    assert!(!*maintenance_gate.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
    assert_eq!(deadline_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(deadline_state.backend_polls.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(deadline_state.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(deadline_state.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(deadline_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    active.store(false, std::sync::atomic::Ordering::Release);
    release_held_create_catalog_worker(&maintenance_gate);
    pool.shutdown();

    let (pool, maintenance_gate, service_gate, active) = reserved_replenishing_create_catalog_io_pool();
    let exhaust_pointer = Arc::as_ptr(&exhaust_storage) as usize;
    let exhaust = DatabaseCreateCatalogFuture::try_submit(pool.clone(), exhaust_catalog, exhaust_storage, protocol::ArtifactId(String::from("reserved-exhaust"))).unwrap();
    let exhaust_state = exhaust.state.clone();
    let generation = exhaust.generation();
    assert_eq!(exhaust_state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Retry as u8);
    release_held_create_catalog_worker(&service_gate);
    let (storage, document, _, actual) = exhaust.await.unwrap().into_parts().unwrap();
    assert_eq!(Arc::as_ptr(&storage) as usize, exhaust_pointer);
    assert_eq!(document.0, "reserved-exhaust");
    assert_eq!(actual, Err(DbError::LimitExceeded("database create-catalog retry exhausted")));
    assert!(!*maintenance_gate.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
    assert_eq!(exhaust_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    assert_eq!(exhaust_state.backend_polls.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(exhaust_state.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(exhaust_state.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(exhaust_state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(exhaust_state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(exhaust_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert!(!database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter_map(Option::as_ref).any(|state| state.generation == generation));
    for _ in 0..32 {
        std::thread::yield_now();
    }
    assert_eq!(exhaust_state.submission_refusals.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    active.store(false, std::sync::atomic::Ordering::Release);
    release_held_create_catalog_worker(&maintenance_gate);
    pool.shutdown();

    let (pool, maintenance_gate, service_gate, active) = reserved_replenishing_create_catalog_io_pool();
    let rejection_storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let rejection_pointer = Arc::as_ptr(&rejection_storage) as usize;
    let oversized = protocol::ArtifactId(String::with_capacity(DATABASE_CREATE_CATALOG_MAX_ID_BYTES + 1));
    let rejected = match DatabaseCreateCatalogFuture::try_submit(pool.clone(), Arc::new(Mutex::new(CatalogState { epoch: EpochFence::INITIAL, revision: 1, entries: Arc::new(Vec::new()), pending: None })), rejection_storage, oversized) {
        Ok(_) => panic!("oversized reserved create-catalog rejection was admitted"),
        Err(rejected) => rejected,
    };
    let close = rejected.close.clone();
    assert_eq!(close.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().and_then(|owner| owner.storage.as_ref()).map(|storage| Arc::as_ptr(storage) as usize), Some(rejection_pointer));
    assert_eq!(rejected.close_and_take_error(), DbError::LimitExceeded("database create-catalog document bytes"));
    assert_eq!(close.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Retry as u8);
    assert!(close.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    release_held_create_catalog_worker(&service_gate);
    while !close.terminal_is_empty() {
        std::thread::yield_now();
    }
    assert!(!*maintenance_gate.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
    assert_eq!(close.submission_refusals.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    assert!(close.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(close.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    active.store(false, std::sync::atomic::Ordering::Release);
    release_held_create_catalog_worker(&maintenance_gate);
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_sole_permanently_nonreturning_worker_retains_discoverable_owners_without_latency_claim() {
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let storage_pointer = Arc::as_ptr(&storage) as usize;
    let (pool, gate) = held_create_catalog_io_pool();
    let probe = DatabaseCreateCatalogFuture::try_submit(pool.clone(), catalog, storage, protocol::ArtifactId(String::from("nonreturning-retained"))).unwrap();
    let state = probe.state.clone();
    let generation = probe.generation();
    probe.cancel();
    drop(probe);
    assert_eq!(state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Retry as u8);
    assert!(state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert!(state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    assert_eq!(state.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|storage| Arc::as_ptr(storage) as usize), Some(storage_pointer));
    assert_eq!(state.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|document| document.0.as_str()), Some("nonreturning-retained"));
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert_eq!(state.backend_polls.load(std::sync::atomic::Ordering::Acquire), 0);
    assert!(!state.callback_worker_thread.load(std::sync::atomic::Ordering::Acquire));
    assert!(database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter_map(Option::as_ref).any(|retained| retained.generation == generation && Arc::ptr_eq(retained, &state)));
    let terminal = take_database_create_catalog_terminal(generation).unwrap();
    assert!(!terminal.witness().terminal_empty);
    assert!(terminal.witness().retained_owners > 0);
    assert_eq!(terminal.close_step(), DatabaseCreateCatalogCloseStep::Blocked);
    assert!(!terminal.terminal_is_empty());
    release_held_create_catalog_worker(&gate);
    while !terminal.terminal_is_empty() {
        std::thread::yield_now();
    }
    assert_eq!(state.terminal_job_retirements.load(std::sync::atomic::Ordering::Acquire), 1);
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_drop_terminal_close_retires_one_owner_per_lane_grant() {
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let probe = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog, storage, protocol::ArtifactId(String::from("lost")), false).unwrap();
    let generation = probe.generation();
    let state = probe.state.clone();
    drop(probe);
    let terminal = take_database_create_catalog_terminal(generation).unwrap();
    let mut previous = terminal.witness().retained_owners;
    while !terminal.terminal_is_empty() {
        let step = terminal.close_step();
        assert!(matches!(step, DatabaseCreateCatalogCloseStep::Progress | DatabaseCreateCatalogCloseStep::Blocked));
        let current = terminal.witness().retained_owners;
        assert!(previous.saturating_sub(current) <= 1);
        previous = current;
        std::thread::yield_now();
    }
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_one_production_opportunity_is_under_eight_ms_and_native_wasm_share_source() {
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let probe = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog, storage, protocol::ArtifactId(String::from("budget")), false).unwrap();
    let state = probe.state.clone();
    state.driver_authority.store(DatabaseCreateCatalogDriverAuthority::Queued as u8, std::sync::atomic::Ordering::Release);
    let started = std::time::Instant::now();
    state.clone().drive_one(state.generation);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
    assert!(state.opportunities.load(std::sync::atomic::Ordering::Acquire) >= 1);
    let source = include_str!("../../🦀️.rs");
    let region = &source[source.find("//#region 🔖️CreateDocumentCatalogCas").unwrap()..source.find("//#endregion 🔖️CreateDocumentCatalogCas").unwrap()];
    assert!(!region.contains("target_arch = \"wasm32\""));
    assert!(!region.contains("db_actor::block_on"));
    drop(probe);
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_maximum_catalog_claim_revalidate_and_snapshot_clone_never_hold_worker() {
    let entries: Vec<CatalogEntry> = (0..DATABASE_CREATE_CATALOG_MAX_ENTRIES - 1).map(|index| CatalogEntry { document: protocol::ArtifactId(format!("contention-{index:04}")), created_at_ms: index as u64 }).collect();
    let (claim_storage, claim_catalog, _) = create_catalog_fixture(entries.clone()).await;
    let (revalidate_storage, revalidate_catalog, _) = create_catalog_fixture(entries.clone()).await;
    let (retire_storage, retire_catalog, _) = create_catalog_fixture(entries).await;
    let (pool, gate) = held_create_catalog_io_pool();
    let claim = DatabaseCreateCatalogFuture::try_prepare(pool.clone(), claim_catalog.clone(), claim_storage, protocol::ArtifactId(String::from("claim-contention")), false).unwrap();
    let claim_state = claim.state.clone();
    claim_state.set_phase(DatabaseCreateCatalogPhase::Claim);
    claim_state.wake_requested.store(false, std::sync::atomic::Ordering::Release);
    claim_state.driver_authority.store(DatabaseCreateCatalogDriverAuthority::Queued as u8, std::sync::atomic::Ordering::Release);
    let claim_guard = claim_catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let started = std::time::Instant::now();
    claim_state.clone().drive_one(claim_state.generation);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
    assert_eq!(claim_state.phase(), DatabaseCreateCatalogPhase::Claim);
    assert!(claim_state.catalog_contention_armed.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(claim_state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Idle as u8);

    let revalidate = DatabaseCreateCatalogFuture::try_prepare(pool.clone(), revalidate_catalog.clone(), revalidate_storage, protocol::ArtifactId(String::from("revalidate-contention")), false).unwrap();
    let revalidate_state = revalidate.state.clone();
    revalidate_state.set_phase(DatabaseCreateCatalogPhase::Revalidate);
    revalidate_state.wake_requested.store(false, std::sync::atomic::Ordering::Release);
    revalidate_state.driver_authority.store(DatabaseCreateCatalogDriverAuthority::Queued as u8, std::sync::atomic::Ordering::Release);
    let revalidate_guard = revalidate_catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let started = std::time::Instant::now();
    revalidate_state.clone().drive_one(revalidate_state.generation);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
    assert_eq!(revalidate_state.phase(), DatabaseCreateCatalogPhase::Revalidate);
    assert!(revalidate_state.catalog_contention_armed.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(revalidate_state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Idle as u8);

    let retire = DatabaseCreateCatalogFuture::try_prepare(pool.clone(), retire_catalog.clone(), retire_storage, protocol::ArtifactId(String::from("retire-contention")), false).unwrap();
    let retire_state = retire.state.clone();
    retire_state.pending_owned.store(true, std::sync::atomic::Ordering::Release);
    retire_state.set_phase(DatabaseCreateCatalogPhase::Retire);
    retire_state.wake_requested.store(false, std::sync::atomic::Ordering::Release);
    retire_state.driver_authority.store(DatabaseCreateCatalogDriverAuthority::Queued as u8, std::sync::atomic::Ordering::Release);
    let retire_guard = retire_catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let started = std::time::Instant::now();
    retire_state.clone().drive_one(retire_state.generation);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
    assert_eq!(retire_state.phase(), DatabaseCreateCatalogPhase::Retire);
    assert!(retire_state.pending_owned.load(std::sync::atomic::Ordering::Acquire));
    assert!(retire_state.catalog_contention_armed.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(retire_state.driver_authority.load(std::sync::atomic::Ordering::Acquire), DatabaseCreateCatalogDriverAuthority::Idle as u8);
    drop(claim_guard);
    drop(revalidate_guard);
    drop(retire_guard);

    claim.cancel();
    revalidate.cancel();
    retire.cancel();
    release_held_create_catalog_worker(&gate);
    while claim_state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
        || revalidate_state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
        || retire_state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    {
        std::thread::yield_now();
    }
    assert_eq!(claim.await.unwrap().into_parts().unwrap().3, Err(DbError::Closed));
    assert_eq!(revalidate.await.unwrap().into_parts().unwrap().3, Err(DbError::Closed));
    assert_eq!(retire.await.unwrap().into_parts().unwrap().3, Err(DbError::Closed));
    let source = include_str!("../../🦀️.rs");
    let catalog = &source[source.find("pub async fn catalog(&self)").unwrap()..source.find("pub async fn health(&self)").unwrap()];
    assert!(catalog.find("Arc::clone(&catalog.entries)").unwrap() < catalog.find("entries.as_ref().clone()").unwrap());
    assert!(!catalog.contains("catalog.entries.as_ref().clone()"));
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_durable_publication_precedes_authority_spawn_emit_and_registration() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let database = Database::open(test_worker_pool(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let transaction = database.create_document_catalog_retained(protocol::ArtifactId(String::from("durable-first"))).unwrap();
    let (_, document, _, actual) = transaction.await.unwrap().into_parts().unwrap();
    assert!(actual.is_ok());
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert_eq!(database.catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entries.last().map(|entry| &entry.document), Some(&document));
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_resolved_drop_retains_use_until_terminal_drain() {
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let (storage, catalog, _) = create_catalog_fixture(Vec::new()).await;
    let result = DatabaseCreateCatalogFuture::try_submit(pool.clone(), catalog, storage, protocol::ArtifactId(String::from("resolved-owner-drop"))).unwrap().await.unwrap();
    let generation = result.state.as_ref().expect("resolved result retains its exact catalog state").generation;
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    drop(result);
    let terminal = take_database_create_catalog_terminal(generation).expect("dropping an unconsumed result exposes its retained terminal owner");
    for _ in 0..1024 {
        if terminal.terminal_is_empty() {
            break;
        }
        let _ = terminal.close_step();
        semio_framework_async::yield_once().await;
    }
    assert!(terminal.terminal_is_empty());
    drop(terminal);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn database_concurrent_ensure_mounts_one_actor_and_one_writer() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-empty"));
    let mut first_future = Box::pin(database.ensure_document(&document));
    let mut second_future = Box::pin(database.ensure_document(&document));
    let mut first_result = None;
    let mut second_result = None;
    let (first_result, second_result) = std::future::poll_fn(|context| {
        if first_result.is_none() {
            if let std::task::Poll::Ready(result) = first_future.as_mut().poll(context) {
                first_result = Some(result);
            }
        }
        if second_result.is_none() {
            if let std::task::Poll::Ready(result) = second_future.as_mut().poll(context) {
                second_result = Some(result);
            }
        }
        match (first_result.take(), second_result.take()) {
            (Some(first), Some(second)) => std::task::Poll::Ready((first, second)),
            (first, second) => {
                first_result = first;
                second_result = second;
                std::task::Poll::Pending
            }
        }
    })
    .await;
    drop(first_future);
    drop(second_future);
    let first = first_result.unwrap();
    let second = second_result.unwrap();
    assert!(Arc::ptr_eq(&first.authority, &second.authority));
    assert_eq!(database.catalog().await.artifacts.iter().filter(|entry| entry.document == document).count(), 1);
    let wal = storage.wal().await;
    let core = to_core_document_id(&document).await;
    assert!(matches!(wal.acquire_writer(&core).await, Err(DbError::Conflict(_))));
    drop(wal);
    drop(first);
    drop(second);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("no-closing-authority"));
        eprintln!(
            "[DEBUG] concurrent mount shutdown retained state: closing={closing} registry={} graph_complete={} emit_started={} pool_use={}; error={error}",
            database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
            database.shutdown_graph_complete,
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after concurrent mount failed: {error}");
    }
    pool.shutdown().unwrap();
}

#[semio_framework_async_macros::async_test]
async fn database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack() {
    let pool = test_worker_pool();
    let memory = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let storage = Arc::new(db_storage::DbBackend::Memory(memory));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    let document = protocol::ArtifactId(String::from("pool-use-mounted-authority"));
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    drop(handle);
    let unrelated = pool.acquire_use().unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    let rejected = match database.ensure_document(&document).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("terminal database admitted another mount")
        }
    };
    assert_eq!(rejected_document_open_error(rejected).await, DbError::Closed);
    assert!(matches!(database.create_document_catalog_retained(protocol::ArtifactId(String::from("post-terminal-catalog"))), Err(DatabaseRetainedActivityRejected::Closed(DbError::Closed))));
    assert!(matches!(
        database.hello_retained(to_core_document_id(&document).await, None, String::from("post-terminal-session"), protocol::ActorId(String::from("post-terminal-actor")), 4096,),
        Err(DatabaseRetainedActivityRejected::Closed(DbError::Closed))
    ));
    assert_eq!(database.checkpoint_document(&document, String::from("post-terminal-checkpoint"), &[]).await, Err(DbError::Closed));
    let (ran_tx, ran_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms(), Lane::UserVisible, Box::new(move || ran_tx.send(()).unwrap()));
    ran_rx.recv().unwrap();
    drop(unrelated);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn database_worker_pool_use_is_admitted_before_the_first_storage_probe() {
    let pool = test_worker_pool();
    assert_eq!(pool.shutdown(), Ok(()));
    let memory = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let storage = Arc::new(db_storage::DbBackend::Memory(memory));
    let error = match Database::open(pool, DbConfig::for_profile(Profile::Test), storage).await {
        Err(error) => error,
        Ok(mut database) => {
            let _ = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await;
            panic!("database opened on a stopped WorkerPool")
        }
    };
    assert!(matches!(error, DbError::Unavailable(detail) if detail.contains("WorkerPool use rejected")));
}

#[test]
fn database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer() {
    let pool = test_worker_pool();
    assert_eq!(pool.shutdown(), Ok(()));
    let registry = Arc::new(Mutex::new(DatabaseDocumentMountRegistry::new()));
    let owner = DatabaseDocumentMountOwner::new(
        pool,
        Arc::downgrade(&registry),
        String::from("non-runnable-mount"),
        1,
        Box::pin(async { std::future::pending::<Result<DatabaseDocumentMountReply, DatabaseDocumentMountFailure>>().await }),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
    );
    owner.request_drive(false);
    assert_eq!(owner.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseDocumentMountDriver::NonRunnable as u8);
    assert_eq!(owner.scheduler_fault(), Some(semio_framework_async::WorkerSubmitErrorKind::Shutdown));
    assert!(owner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
    assert!(!owner.retry_armed.load(std::sync::atomic::Ordering::Acquire));
    drop(owner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
    owner.terminal.store(true, std::sync::atomic::Ordering::Release);
    owner.driver.store(DatabaseDocumentMountDriver::Terminal as u8, std::sync::atomic::Ordering::Release);
}

#[semio_framework_async_macros::async_test]
async fn database_published_opening_joins_without_actor_overwrite() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-published"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (published_tx, published_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    *database.mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |document| {
        published_tx.send(document.clone()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut first = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    assert_eq!(published_rx.recv().unwrap(), document);
    let mut second = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(second.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { waiters, .. } = registry.slots.get(&document.0).unwrap() else { panic!("published mount was not retained as Opening") };
        assert_eq!(waiters.iter().flatten().count(), 2);
    }
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let first = first.await.unwrap();
    let second = second.await.unwrap();
    assert!(Arc::ptr_eq(&first.authority, &second.authority));
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 1);
    drop(first);
    drop(second);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_cancelled_ensure_waiter_does_not_cancel_mount_owner() {
    eprintln!("[DEBUG] cancelled-waiter stage=setup");
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-cancel"));
    let mut cancelled = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(cancelled.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    eprintln!("[DEBUG] cancelled-waiter stage=first-pending");
    drop(cancelled);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { owner, waiters, .. } = registry.slots.get(&document.0).unwrap() else { panic!("cancelled waiter removed retained owner") };
        assert_eq!(waiters.iter().flatten().count(), 0);
        assert!(!owner.terminal.load(std::sync::atomic::Ordering::Acquire));
        eprintln!(
            "[DEBUG] cancelled-waiter stage=owner-retained driver={} wake={} resume={}",
            owner.driver.load(std::sync::atomic::Ordering::Acquire),
            owner.wake_requested.load(std::sync::atomic::Ordering::Acquire),
            owner.resume_requested.load(std::sync::atomic::Ordering::Acquire)
        );
    }
    eprintln!("[DEBUG] cancelled-waiter stage=join");
    let handle = database.ensure_document(&document).await.unwrap();
    eprintln!("[DEBUG] cancelled-waiter stage=joined");
    assert_eq!(database.catalog().await.artifacts.iter().filter(|entry| entry.document == document).count(), 1);
    drop(handle);
    eprintln!("[DEBUG] cancelled-waiter stage=shutdown");
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        eprintln!(
            "[DEBUG] cancelled-waiter shutdown retained state: closing={closing} registry={} graph_complete={} emit_started={} pool_use={}; error={error}",
            registry.len(),
            database.shutdown_graph_complete,
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after cancelled waiter failed: {error}");
    }
    eprintln!("[DEBUG] cancelled-waiter stage=pool-shutdown");
    pool.shutdown();
    eprintln!("[DEBUG] cancelled-waiter stage=complete");
}

#[semio_framework_async_macros::async_test]
async fn database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-emission"));
    let mut elected = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(elected.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    emit.wait_until_document_event();
    let mut survivor = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(survivor.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    assert_eq!(emit.document_event_count(), 0);
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 0);
    drop(elected);
    emit.release_document_event();
    let survivor = survivor.await.unwrap();
    assert_eq!(emit.document_event_count(), 1);
    let later = database.ensure_document(&document).await.unwrap();
    assert!(Arc::ptr_eq(&survivor.authority, &later.authority));
    assert_eq!(emit.document_event_count(), 1);
    drop(survivor);
    drop(later);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        eprintln!(
            "[DEBUG] emission mount shutdown retained state: closing={closing} registry={} graph_complete={} emit_started={} pool_use={}; error={error}",
            registry.len(),
            database.shutdown_graph_complete,
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after controlled mount emission failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-capacity"));
    let mut waiters = Vec::new();
    for _ in 0..DATABASE_DOCUMENT_MOUNT_WAITERS {
        let mut waiter = Box::pin(database.ensure_document(&document));
        std::future::poll_fn(|context| {
            assert!(waiter.as_mut().poll(context).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        waiters.push(waiter);
    }
    emit.wait_until_document_event();
    let rejected = match database.ensure_document(&document).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("document mount admitted waiter 33")
        }
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Unavailable(_)));
    drop(waiters.remove(0));
    let mut replacement = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(replacement.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    waiters.push(replacement);
    emit.release_document_event();
    let mut handles = Vec::new();
    for waiter in waiters {
        handles.push(waiter.await.unwrap());
    }
    assert_eq!(handles.len(), DATABASE_DOCUMENT_MOUNT_WAITERS);
    assert!(handles.iter().all(|handle| Arc::ptr_eq(&handle.authority, &handles[0].authority)));
    assert_eq!(emit.document_event_count(), 1);
    drop(handles);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-reentrant-wake"));
    let mut mount = Box::pin(database.ensure_document(&document));
    let probe = Arc::new(MountFanoutLockProbe { registry: database.open_artifacts.clone(), called: std::sync::atomic::AtomicBool::new(false), unlocked: std::sync::atomic::AtomicBool::new(false) });
    let waker = std::task::Waker::from(probe.clone());
    let mut context = std::task::Context::from_waker(&waker);
    assert!(mount.as_mut().poll(&mut context).is_pending());
    emit.wait_until_document_event();
    emit.release_document_event();
    while !probe.called.load(std::sync::atomic::Ordering::Acquire) {
        semio_framework_async::yield_once().await;
    }
    assert!(probe.unlocked.load(std::sync::atomic::Ordering::Acquire));
    let handle = mount.await.unwrap();
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-shutdown"));
    let mut cancelled_waiter = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(cancelled_waiter.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    emit.wait_until_document_event();
    drop(cancelled_waiter);
    let (generation, owner) = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, waiters } = registry.slots.get(&document.0).unwrap() else { panic!("mount owner escaped before emission") };
        assert_eq!(waiters.iter().flatten().count(), 0);
        (*generation, Arc::as_ptr(owner))
    };
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(30), cancelled.clone());
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Interrupted);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation: retained_generation, owner: retained_owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("interrupted shutdown lost mount owner") };
        assert_eq!((*retained_generation, Arc::as_ptr(retained_owner)), (generation, owner));
    }
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    assert!(matches!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, .. }));
    emit.release_document_event();
    for _ in 0..100_000 {
        if database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count() == 1 {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 1);
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(emit.document_event_count(), 1);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_create_catalog_publication_check_register_recheck_has_no_lost_wake() {
    let (storage, catalog, epoch) = create_catalog_fixture(Vec::new()).await;
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCreateCatalogFuture::try_prepare(test_worker_pool(), catalog, storage, protocol::ArtifactId(String::from("wake-race")), false).unwrap();
    let state = probe.state.clone();
    let hook_state = state.clone();
    let published = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let hook_published = published.clone();
    *state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Box::new(move || {
        hook_state.schedule();
        while hook_state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            std::thread::yield_now();
        }
        hook_published.store(true, std::sync::atomic::Ordering::Release);
    }));
    let (storage, document, expected, actual) = probe.await.unwrap().into_parts().unwrap();
    assert!(published.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
    assert_eq!(document.0, "wake-race");
    assert_eq!(expected, epoch);
    assert_eq!(actual, Ok(epoch.next()));
    assert!(state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
}

#[semio_framework_async_macros::async_test]
async fn open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog() {
    let root = tempdir("open-at-fresh").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    assert!(database.catalog().await.artifacts.is_empty());
    assert_eq!(database.health().await.open_artifacts, 0);
    assert!(matches!(database.health().await.report.overall, db_observe::HealthState::Healthy));
}

#[semio_framework_async_macros::async_test]
async fn create_document_registers_it_in_the_catalog_and_document_finds_it() {
    let root = tempdir("create-and-find").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let catalog = database.catalog().await;
    assert_eq!(catalog.artifacts.len(), 1);
    assert_eq!(catalog.artifacts[0].document, document);

    let handle = database.document(&document).await.unwrap();
    assert_eq!(handle.document_id().await, &document);
}

#[semio_framework_async_macros::async_test]
async fn create_document_twice_errs_already_exists() {
    let root = tempdir("create-twice").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let result = database.create_document(ArtifactSpec::new(document).await);
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("duplicate document was admitted"),
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::AlreadyExists(_)));
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout() {
    let inner = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(test_worker_pool()).await.unwrap()));
    let fault = db_testkit::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(test_worker_pool(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_testkit::FaultScript { fail_nth_write: Some(1), ..db_testkit::FaultScript::default() }).await;
    let document = protocol::ArtifactId("database-retained-open-rejection".to_string());
    let rejected = match database.create_document(ArtifactSpec::new(document.clone()).await).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("faulted database document builder was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(!rejected.has_retained_writer());
    let core_document = to_core_document_id(&document).await;
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    storage.wal().await.acquire_writer(&core_document).await.unwrap().release().await.unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.unwrap();
    eprintln!("[DEBUG] database mount retained and terminalized the exact failed WAL release owner before caller fanout");
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation() {
    let pool = test_worker_pool();
    let inner = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let fault = db_testkit::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_testkit::FaultScript { fail_nth_sync: Some(1), ..db_testkit::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-retained-failure"));
    let mut first_future = Box::pin(database.ensure_document(&document));
    let mut second_future = Box::pin(database.ensure_document(&document));
    let mut first_result = None;
    let mut second_result = None;
    let (first_result, second_result) = std::future::poll_fn(|context| {
        if first_result.is_none() {
            if let std::task::Poll::Ready(result) = first_future.as_mut().poll(context) {
                first_result = Some(result);
            }
        }
        if second_result.is_none() {
            if let std::task::Poll::Ready(result) = second_future.as_mut().poll(context) {
                second_result = Some(result);
            }
        }
        match (first_result.take(), second_result.take()) {
            (Some(first), Some(second)) => std::task::Poll::Ready((first, second)),
            (first, second) => {
                first_result = first;
                second_result = second;
                std::task::Poll::Pending
            }
        }
    })
    .await;
    drop(first_future);
    drop(second_future);
    for result in [first_result, second_result] {
        let rejected = match result {
            Err(rejected) => rejected,
            Ok(handle) => {
                drop(handle);
                panic!("faulted mount was admitted")
            }
        };
        assert!(!rejected.has_retained_writer());
        assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    }
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    fault.set_script(db_testkit::FaultScript::default()).await;
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 1);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume() {
    let pool = test_worker_pool();
    let memory = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    memory.fail_next_writer_release();
    let inner = Arc::new(db_storage::DbBackend::Memory(memory));
    let fault = db_testkit::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_testkit::FaultScript { fail_nth_sync: Some(1), ..db_testkit::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-unlock-fault"));
    let (parked_tx, parked_rx) = std::sync::mpsc::sync_channel(1);
    *database.mount_parked_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || parked_tx.send(()).unwrap()));
    let mut first = Box::pin(database.ensure_document(&document));
    let mut second = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        assert!(second.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    parked_rx.recv().unwrap();
    let exact = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, .. } = registry.slots.get(&document.0).expect("temporary unlock fault retains the mounted owner") else { panic!("temporary unlock fault replaced its opening slot") };
        let work = owner.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountWork::Parked { rejected, .. } = &*work else { panic!("temporary unlock fault did not park its retained owner") };
        assert!(rejected.has_retained_writer());
        assert!(matches!(rejected.cleanup_error(), Some(DbError::Unavailable(detail)) if detail.contains("injected WAL writer unlock failure")));
        (*generation, Arc::as_ptr(owner))
    };
    drop(first);
    drop(second);
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(30), cancelled.clone());
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Interrupted);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("interrupted shutdown lost parked cleanup") };
        assert_eq!((*generation, Arc::as_ptr(owner)), exact);
        assert!(matches!(&*owner.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner), DatabaseDocumentMountWork::Parked { rejected, .. } if rejected.has_retained_writer()));
    }
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    assert!(matches!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, .. }));
    for _ in 0..100_000 {
        if database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty() {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    fault.set_script(db_testkit::FaultScript::default()).await;
    let handle = database.ensure_document(&document).await.unwrap();
    drop(handle);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        eprintln!(
            "[DEBUG] unlock-retry shutdown retained state: closing={closing} registry={} graph_complete={} emit_started={} pool_use={}; error={error}",
            database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
            database.shutdown_graph_complete,
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after controlled unlock retry failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_coalesces_join_drives_without_shared_pool_starvation() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-poller-coalescing"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (published_tx, published_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    *database.mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |_| {
        published_tx.send(()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut first = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    published_rx.recv().unwrap();
    let mut joins = Vec::new();
    for _ in 0..8 {
        let mut join = Box::pin(database.ensure_document(&document));
        std::future::poll_fn(|context| {
            assert!(join.as_mut().poll(context).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        joins.push(join);
    }
    let release_gate = gate.clone();
    pool.submit_at(
        pool.now_ms(),
        Lane::UserVisible,
        Box::new(move || {
            let (lock, ready) = &*release_gate;
            *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
            ready.notify_one();
        }),
    );
    let first = first.await.unwrap();
    for join in joins {
        let joined = join.await.unwrap();
        assert!(Arc::ptr_eq(&first.authority, &joined.authority));
        drop(joined);
    }
    drop(first);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once() {
    let pool = test_worker_pool();
    let memory = db_storage::MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    memory.fail_next_writer_release();
    let inner = Arc::new(db_storage::DbBackend::Memory(memory));
    let fault = db_testkit::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_testkit::FaultScript { fail_nth_sync: Some(1), ..db_testkit::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-racing-cleanup-resume"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (fault_tx, fault_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    let hook_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let hook_counted = hook_count.clone();
    *database.mount_cleanup_fault_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || {
        hook_counted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        fault_tx.send(()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut mount = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(mount.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    fault_rx.recv().unwrap();
    let owner = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("cleanup owner was not mounted") };
        owner.clone()
    };
    owner.drive();
    assert!(owner.resume_requested.load(std::sync::atomic::Ordering::Acquire));
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let rejected = match mount.await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("faulted mount was admitted")
        }
    };
    assert!(!rejected.has_retained_writer());
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    assert_eq!(hook_count.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(owner.terminal.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(owner.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseDocumentMountDriver::Terminal as u8);
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn document_of_an_unknown_id_errs_not_found() {
    let root = tempdir("unknown-doc").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let never_created = protocol::ArtifactId("never-created".to_string());
    let result = database.document(&never_created);
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("unknown document was admitted"),
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::NotFound(_)));
}
//#endregion 🔖️Database open/catalog

//#region 🔖️Round trip
#[semio_framework_async_macros::async_test]
async fn full_submit_durable_query_round_trip_over_a_real_document_authority() {
    let root = tempdir("round-trip").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
    let receipt = db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.frontier.document, document);
    assert_eq!(receipt.frontier.head_seq, 1);
    assert!(receipt.conflicts.is_empty());
    assert!(receipt.state_hash.is_some());

    let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).await.unwrap();
    let value = decode_query_json(queried).await;
    assert_eq!(value, serde_json::json!("hello"));

    let frontier = handle.frontier().await.unwrap();
    assert_eq!(frontier.head_seq, 1);
    let checkpoint_snapshot = handle.checkpoint_publication_snapshot().await.unwrap();
    assert_ne!(checkpoint_snapshot.authority_generation, 0);
    assert_eq!(checkpoint_snapshot.frontier, frontier);
    assert_eq!(checkpoint_snapshot.head_edit_id, Some(protocol::MutationId("op-1".to_string())));

    let mut at_least = handle.query(Query::Get { path: "name".to_string() }, Consistency::AtLeast(frontier)).await.unwrap();
    assert_eq!(at_least.len(), 1);
    while at_least.close_step().unwrap() {}
    assert!(at_least.terminal_is_empty());

    let mut history = handle.history().await.unwrap();
    assert_eq!(history.entries().len(), 1);
    assert!(history.operation_id_eq(0, 0, "op-1"));
    while history.close_step() {}
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_empty_and_two_batch_replay_are_deterministic() {
    let root = tempdir("history-order").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("history-doc".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let mut empty = handle.history().await.unwrap();
    assert!(empty.entries().is_empty());
    while empty.close_step() {}
    for id in ["history-1", "history-2"] {
        let batch = db_artifact::CommandBatch::new(vec![envelope(id, &[], "alice", &document, &[("value", serde_json::json!(id))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();
    }
    let mut first = handle.history().await.unwrap();
    let mut second = handle.history().await.unwrap();
    assert_eq!(first.entries(), second.entries());
    assert!(first.operation_id_eq(0, 0, "history-1"));
    assert!(first.operation_id_eq(1, 0, "history-2"));
    assert!(second.operation_id_eq(0, 0, "history-1"));
    assert!(second.operation_id_eq(1, 0, "history-2"));
    while first.close_step() {}
    while second.close_step() {}
}

#[semio_framework_async_macros::async_test]
async fn a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root() {
    let root = tempdir("reopen").await;
    let document = protocol::ArtifactId("doc-1".to_string());
    {
        let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("count", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
    }

    let reopened = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    assert_eq!(reopened.catalog().await.artifacts.len(), 1, "the catalog root must have survived the reopen");

    let handle = reopened.document(&document).await.unwrap();
    let queried = handle.query(Query::Get { path: "count".to_string() }, Consistency::Canonical).await.unwrap();
    let value = decode_query_json(queried).await;
    assert_eq!(value, serde_json::json!(1), "the document's committed state must have survived the reopen via WAL replay");
    assert_eq!(handle.frontier().await.unwrap().head_seq, 1);
    assert_eq!(handle.checkpoint_publication_snapshot().await.unwrap().head_edit_id, Some(protocol::MutationId("op-1".to_string())));
}

#[semio_framework_async_macros::async_test]
async fn exact_consistency_rejects_a_frontier_the_document_has_moved_past() {
    let root = tempdir("exact-consistency").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let stale = handle.frontier().await.unwrap();

    let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

    let result = handle.query(Query::Get { path: "x".to_string() }, Consistency::Exact(stale));
    assert!(matches!(result.await, Err(DbError::Unavailable(_))));
}

#[test]
fn query_stream_max_plus_one_hands_back_exact_owner_and_close_is_terminal() {
    let mut stream = QueryStream::new();
    for index in 0..64 {
        stream.push(QueryResultEntry { path: db_storage::DbIoText::try_from_str(&format!("path-{index:02}")).unwrap(), value: None }).unwrap();
    }
    let rejected = stream.push(QueryResultEntry { path: db_storage::DbIoText::try_from_str("overflow-owner").unwrap(), value: None }).unwrap_err();
    assert_eq!(rejected.path(), "overflow-owner");
    let mut rejected = rejected;
    while rejected.close_step().unwrap() {}
    while stream.close_step().unwrap() {}
    assert!(stream.terminal_is_empty());
}
//#endregion 🔖️Round trip

//#region 🔖️Deferred extension seams
#[semio_framework_async_macros::async_test]
async fn subscribe_preview_and_snapshot_now_are_documented_unimplemented_not_panics() {
    let root = tempdir("deferred").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();

    assert!(matches!(handle.subscribe(LiveQuerySpec { since: None }).await, Err(DbError::Unimplemented(_))));
    assert!(matches!(handle.preview(handle.frontier().await.unwrap()).await, Err(DbError::Unimplemented(_))));
    assert!(matches!(db_actor::block_on(handle.snapshot_now(SnapshotKind::Full).await), Ok(Err(DbError::Unimplemented(_)))));
}
//#endregion 🔖️Deferred extension seams

//#region 🔖️VersionGraph
#[cfg(feature = "vcs")]
#[semio_framework_async_macros::async_test]
async fn checkpoint_document_mints_distinct_real_vcs_content_addressed_checkpoint_ids() {
    let root = tempdir("vcs-checkpoint").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let batch1 = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch1, db_artifact::SubmitOptions::default())).unwrap().unwrap();
    let checkpoint_1 = database.checkpoint_document(&document, "first".to_string(), &[protocol::ActorId("alice".to_string())]).await.unwrap();
    assert!(checkpoint_1.starts_with("ck-"), "vcs checkpoint ids are content-addressed as ck-<hex16>, got {checkpoint_1:?}");

    let batch2 = db_artifact::CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &document, &[("x", serde_json::json!(2))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch2, db_artifact::SubmitOptions::default())).unwrap().unwrap();
    let checkpoint_2 = database.checkpoint_document(&document, "second".to_string(), &[protocol::ActorId("alice".to_string())]).await.unwrap();

    assert_ne!(checkpoint_1, checkpoint_2, "distinct commits must mint distinct content-addressed checkpoint ids");
}

#[cfg(not(feature = "vcs"))]
#[semio_framework_async_macros::async_test]
async fn checkpoint_document_errs_unimplemented_without_the_vcs_feature() {
    let root = tempdir("no-vcs-checkpoint").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    database.create_document(ArtifactSpec::new(document.clone())).await.unwrap();
    assert!(matches!(database.checkpoint_document(&document, "msg".to_string(), &[]).await, Err(DbError::Unimplemented(_))));
}
//#endregion 🔖️VersionGraph

//#region 🔖️Compact + Sync
#[semio_framework_async_macros::async_test]
async fn compact_document_runs_a_real_compaction_pass_without_error() {
    let root = tempdir("compact").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

    let report = database.compact_document(&document, "holder-1", false).await.unwrap();
    assert_eq!(report.wal_segments_deleted, 0, "nothing is below the (nonexistent) snapshot floor yet, but the pass itself must succeed");
}

#[semio_framework_async_macros::async_test]
async fn compact_document_uses_live_actor_writer_and_restores_submits() {
    let root = tempdir("compact-live-writer").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("compact-live-writer".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let first = db_artifact::CommandBatch::new(vec![envelope("compact-before", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(first, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact::SubmitOptions::default() })).unwrap().unwrap();
    let fill_ids = ["compact-fill-0", "compact-fill-1", "compact-fill-2", "compact-fill-3", "compact-fill-4", "compact-fill-5", "compact-fill-6", "compact-fill-7"];
    for (index, mutation_id) in fill_ids.iter().enumerate() {
        let dependency = if index == 0 { "compact-before" } else { fill_ids[index - 1] };
        let value = format!("{index}:{}", "x".repeat(40_000));
        let batch = db_artifact::CommandBatch::new(vec![envelope(mutation_id, &[dependency], "alice", &document, &[("x", serde_json::json!(value))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact::SubmitOptions::default() })).unwrap().unwrap();
    }
    handle.authority.snapshot_now(100).await.unwrap();

    let core_document = to_core_document_id(&document).await;
    let storage = database.storage().await;
    let mut before = storage.wal().await.list_segments(&core_document).await.unwrap();
    assert!(before.len() >= 2, "bounded public submissions must leave a sealed predecessor and active successor");
    let active = *before.last().unwrap();
    while before.close_step() {}
    assert!(before.terminal_is_empty());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "the live actor must retain its original writer before compaction");
    let compacted = database.compact_document(&document, "actor-holder", false).await;
    let report = compacted.unwrap();
    assert!(report.wal_segments_deleted >= 1);
    assert_eq!(report.payloads_deleted, 0, "document-scoped maintenance must not reclaim global CAS bytes");
    let mut after = storage.wal().await.list_segments(&core_document).await.unwrap();
    assert!(after.as_slice().contains(&active), "the actor WAL's authoritative active segment must survive compaction");
    while after.close_step() {}
    assert!(after.terminal_is_empty());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "compaction must return the same retained writer to the actor");

    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    assert!(matches!(handle.compact("cancelled-holder", false, cancelled).await, Err(DbError::Closed)));
    let second = db_artifact::CommandBatch::new(vec![envelope("compact-after", &["compact-fill-7"], "alice", &document, &[("x", serde_json::json!(2))]).await]).await.unwrap();
    let receipt = db_actor::block_on(handle.submit(second, db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact::SubmitOptions::default() })).unwrap().unwrap();
    assert_eq!(receipt.frontier.head_seq, 10);
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "cancelled maintenance must restore the engine without releasing its writer");
    drop(storage);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
}

#[cfg(feature = "vcs")]
#[semio_framework_async_macros::async_test]
async fn database_shutdown_cancellation_and_vcs_error_preserve_exact_retry_owners() {
    let root = tempdir("shutdown-retry-owner").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("shutdown-retry-owner".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact::CommandBatch::new(vec![envelope("shutdown-seed", &[], "owner", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();
    drop(handle);

    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(5), cancelled.clone());
    assert!(matches!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, .. }));
    let exact_authority = Arc::as_ptr(&database.closing_authority.as_ref().expect("database retains closing authority").1);
    cancelled.store(true, std::sync::atomic::Ordering::Release);
    assert_eq!(database.shutdown(&control).await, Err(DbError::Closed));
    assert_eq!(Arc::as_ptr(&database.closing_authority.as_ref().expect("cancelled shutdown retains authority").1), exact_authority);
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    for _ in 0..4_096 {
        database.shutdown_step(&control).await.unwrap();
        if database.closing_authority.is_none() && database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty() {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert!(database.closing_authority.is_none());
    match database.version_graph.as_ref() {
        VersionGraphs::Vcs(graph) => graph.fail_next_shutdown_step(),
        VersionGraphs::Null(_) => panic!("vcs feature must mount the real graph"),
    }
    assert!(matches!(database.shutdown(&control).await, Err(DbError::Internal(detail)) if detail == "injected retained VCS shutdown fault"));
    assert!(!database.shutdown_graph_complete);
    database.shutdown(&control).await.expect("retry consumes retained VCS owner");
    assert!(database.shutdown_complete);
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_shared_authority_blocks_without_closing_live_handle() {
    let root = tempdir("shutdown-shared-owner").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("shutdown-shared-owner".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();
    let control = DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5));
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities(1)));
    assert_eq!(handle.frontier().await.unwrap().head_seq, 0);
    drop(handle);
    database.shutdown(&control).await.expect("released shared authority permits terminal shutdown");
}

#[semio_framework_async_macros::async_test]
async fn hello_returns_a_welcome_with_a_fresh_bootstrap_for_a_brand_new_replica() {
    let root = tempdir("hello").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();

    let mut session = database.hello(document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string()), 4096).await.unwrap();
    let welcome = session.take_welcome().unwrap();
    assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { .. }));
    welcome.acknowledge().unwrap();
}

// 🔬️ `storage()` is a real escape hatch to the same backend `Database::open_at` wired — a
// caller below the document-actor boundary (os-semio_hub's blob routes) can round-trip a payload
// through it directly, independent of any document actor.
#[semio_framework_async_macros::async_test]
async fn storage_accessor_reaches_the_same_backend_payload_store() {
    let root = tempdir("storage-accessor").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let hash = db_actor::block_on(async {
        let pages = db_storage::db_io_copy_pages(b"hello storage accessor").unwrap().await.unwrap();
        database.storage().await.payload().await.put(pages).await
    })
    .unwrap();
    assert_eq!(db_actor::block_on(async { database.storage().await.payload().await.get(&hash).await }).unwrap(), b"hello storage accessor");
}
//#endregion 🔖️Compact + Sync

//#region 🔖️Retained submit authority
fn retained_submit_source() -> &'static str {
    include_str!("../../🦀️.rs")
}

fn retire_history_admission(mut admission: ArtifactHistoryAdmission) {
    let mut cursor = admission.begin_reservation_close().expect("history test admission retained its reservation");
    for _ in 0..50_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
}

#[test]
fn artifact_submit_late_readiness_parks_then_one_shot_wake_reschedules() {
    let source = retained_submit_source();
    assert!(source.contains("impl std::task::Wake for ArtifactSubmitWake"));
    assert!(source.contains("self.scheduled.compare_exchange(false, true"));
    assert!(source.contains("std::task::Poll::Pending =>"));
    assert!(source.contains("self.set_progress(SubmitProgress::Waiting)"));
}

#[test]
fn artifact_submit_pool_saturation_without_later_ingress_retains_exact_job() {
    let source = retained_submit_source();
    assert!(source.contains("self.pool.try_submit(Lane::Io, job)"));
    assert!(source.contains("error.into_job()"));
    assert!(source.contains("self.pool.callback_at"));
    assert!(source.contains("ARTIFACT_SUBMIT_RETRY_LIMIT"));
}

#[test]
fn artifact_submit_cancel_before_during_after_preserves_exact_owner() {
    let source = retained_submit_source();
    assert!(source.contains("self.state.cancelled.store(true"));
    assert!(source.contains("self.terminalize_work(Err(DbError::Closed), SubmitProgress::Cancelled)"));
    assert!(source.contains("*self.terminal_result.lock()"));
    assert!(source.contains("SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault"));
}

#[test]
fn artifact_submit_stale_generation_and_slot_aba_cannot_consume_current_work() {
    let first = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
    let first_slot = first.slot;
    let first_generation = first.generation;
    drop(first);
    let next = ArtifactSubmitAdmission::try_claim(1, ARTIFACT_SUBMIT_PAGE_BYTES).unwrap();
    assert_eq!(next.slot, first_slot);
    assert_ne!(next.generation, first_generation);
    let source = retained_submit_source();
    let stale = source.find("if generation != self.generation").unwrap();
    let mutation = source[stale..].find("self.scheduled.store").unwrap();
    assert!(mutation > 0);
}

#[test]
fn artifact_submit_missing_handle_terminalizes_without_mailbox_mutation() {
    let source = retained_submit_source();
    let stale = source.find("if self.authority.generation() != self.authority_generation").unwrap();
    let handoff = source.find("self.authority.submit_retained").unwrap();
    assert!(stale < handoff);
    assert!(source.contains("Err(DbError::StaleGeneration"));
}

#[test]
fn artifact_submit_terminal_job_work_result_take_resume_and_close_one_owner() {
    let source = retained_submit_source();
    for required in ["pub fn take_terminal_job", "pub fn take_terminal_work", "pub fn take_terminal_result", "pub fn take_actor_terminal_job", "pub fn close_step", "pub fn terminal_is_empty", "pub fn resume(mut self)"] {
        assert!(source.contains(required), "missing {required}");
    }
    assert!(source.contains("fn close_one(&self) -> bool"));
}

#[semio_framework_async_macros::async_test]
async fn artifact_submit_item_cap_plus_one_and_nested_bytes_plus_one_return_owner() {
    let document = protocol::ArtifactId("credit-doc".to_string());
    let one = envelope("credit-1", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
    let admitted = db_artifact::CommandBatch::new(vec![one]).await.unwrap();
    assert!(artifact_submit_credit(&admitted).is_ok());

    let mut envelopes = Vec::new();
    for index in 0..=ARTIFACT_SUBMIT_BATCH_ITEMS {
        envelopes.push(envelope(&format!("credit-{index}"), &[], "credit-actor", &document, &[("x", serde_json::json!(index))]).await);
    }
    let rejected = db_artifact::CommandBatch { envelopes };
    assert!(artifact_submit_credit(&rejected).is_err());

    let mut oversize = envelope("credit-oversize", &[], "credit-actor", &document, &[("x", serde_json::json!(1))]).await;
    oversize.diff.payload = vec![0; ARTIFACT_SUBMIT_OPERATION_BYTES as usize + 1];
    assert!(artifact_submit_credit(&db_artifact::CommandBatch { envelopes: vec![oversize] }).is_err());
}

#[test]
fn artifact_runner_one_grant_polls_one_turn_and_never_blocks_on() {
    let source = include_str!("../../../🗿️artifact/🦀️.rs");
    let runner = &source[source.find("type ArtifactBuildFuture").unwrap()..source.find("//#region 🧪️Tests").unwrap()];
    assert!(!runner.contains("block_on("));
    assert!(!runner.contains("ask_blocking"));
    assert!(runner.contains("future.as_mut().poll(&mut context)"));
    assert!(runner.contains("Self::start_turn(engine, envelope.payload)"));
    assert!(runner.contains("let closed ="));
    assert!(runner.contains("if !closed"));
}

struct ControlledHistoryPublicWake {
    state: std::sync::Weak<ArtifactHistoryState>,
    wakes: std::sync::atomic::AtomicUsize,
    lock_released: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for ControlledHistoryPublicWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            self.lock_released.store(state.waker.try_lock().is_ok(), std::sync::atomic::Ordering::Release);
        }
        self.wakes.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_completion_interleavings_preserve_result_and_wake() {
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📜️history-completion/🔣️.json")).unwrap();
    let artifact_root = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("history native law requires its ticket artifact directory"));
    let root = artifact_root.join(format!("history-publication-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId(fixture["document"].as_str().unwrap().into());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    for operation in fixture["operations"].as_array().unwrap() {
        let id = operation.as_str().unwrap();
        let batch = db_artifact::CommandBatch::new(vec![envelope(id, &[], "history-fixture", &document, &[("value", serde_json::json!(id))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact::SubmitOptions::default())).unwrap().unwrap();
    }
    let mut observations = Vec::new();
    for row in fixture["publication"].as_array().unwrap() {
        let mut probe = HistoryFuture::prepare(&handle);
        let state = probe.state.clone();
        let terminal = probe.terminal_handle();
        let generation = probe.generation();
        let admission_slot = state.admission.lock().unwrap().as_ref().unwrap().slot;
        let (captured, capture) = std::sync::mpsc::sync_channel(1);
        *state.controlled_completion_hook.lock().unwrap() = Some(Box::new(move |result, progress| {
            assert!(captured.send((result, progress)).is_ok(), "physical history result capture must remain live");
        }));
        let cancelled = row["outcome"] == "cancelled";
        if cancelled {
            probe.cancel();
        } else {
            state.schedule();
        }
        let (result, progress) = capture.recv_timeout(std::time::Duration::from_secs(5)).expect("actual history producer completion deadline");
        let entry_pointer = result.as_ref().ok().map(|view| view.entries().as_ptr() as usize);
        if let Ok(view) = result.as_ref() {
            assert_eq!(view.entries().len(), fixture["operations"].as_array().unwrap().len());
            assert_eq!(view.admission.as_ref().unwrap().generation, generation);
            assert!(Arc::ptr_eq(view.terminal_state.as_ref().unwrap(), &state));
        }
        assert!(ARTIFACT_HISTORY_ADMISSION.lock().unwrap().slots[admission_slot].occupied);
        let publish_state = state.clone();
        let publish = move || publish_state.complete(result, progress);
        let mut deferred = None;
        match row["publication"].as_str().unwrap() {
            "before-poll" => publish(),
            "before-registration" => *state.controlled_publication_before_waker_hook.lock().unwrap() = Some(Box::new(publish)),
            "after-pending" => deferred = Some(publish),
            _ => unreachable!(),
        }
        let wake = Arc::new(ControlledHistoryPublicWake { state: Arc::downgrade(&state), wakes: std::sync::atomic::AtomicUsize::new(0), lock_released: std::sync::atomic::AtomicBool::new(true) });
        let waker = std::task::Waker::from(wake.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let first = std::pin::Pin::new(&mut probe).poll(&mut context);
        let first_ready = first.is_ready();
        if let Some(publish) = deferred {
            publish();
        }
        let wake_count = wake.wakes.load(Ordering::Acquire);
        let wake_lock_released = wake.lock_released.load(Ordering::Acquire);
        let outcome = match first {
            std::task::Poll::Ready(result) => result,
            std::task::Poll::Pending => match std::pin::Pin::new(&mut probe).poll(&mut context) {
                std::task::Poll::Ready(result) => result,
                _ => panic!("physically published history result must remain consumable"),
            },
        };
        let waiter_empty = state.waker.lock().unwrap().is_none();
        let exact_result = match outcome {
            Ok(mut view) => {
                assert!(!cancelled);
                let exact = Some(view.entries().as_ptr() as usize) == entry_pointer
                    && view.admission.as_ref().is_some_and(|owner| owner.slot == admission_slot && owner.generation == generation)
                    && view.terminal_state.as_ref().is_some_and(|owner| Arc::ptr_eq(owner, &state))
                    && fixture["operations"].as_array().unwrap().iter().enumerate().all(|(index, operation)| view.operation_id_eq(index, 0, operation.as_str().unwrap()));
                for _ in 0..50_000 {
                    if view.terminal_is_empty() {
                        break;
                    }
                    assert!(view.close_step(), "actual replay result must make bounded retirement progress");
                }
                assert!(view.terminal_is_empty(), "history fixture must close every actual replay owner");
                exact
            }
            Err(error) => cancelled && error == DbError::Closed,
        };
        drop(probe);
        for _ in 0..50_000 {
            if terminal.terminal_is_empty() {
                break;
            }
            if !terminal.close_step() {
                std::thread::yield_now();
            }
        }
        let admission_released = !ARTIFACT_HISTORY_ADMISSION.lock().unwrap().slots[admission_slot].occupied;
        let registry_empty = handle.history_terminal(generation).is_none();
        assert!(terminal.terminal_is_empty(), "history fixture cleanup must finish its exact terminal state");
        eprintln!(
            "[DEBUG] history-completion: {} first-ready={first_ready} wakes={wake_count} waiter-empty={waiter_empty} wake-lock-released={wake_lock_released} exact-result={exact_result} admission-released={admission_released} registry-empty={registry_empty}",
            row["name"]
        );
        observations.push((row.clone(), first_ready, wake_count, waiter_empty, exact_result, admission_released, registry_empty, wake_lock_released));
    }
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    for (row, first_ready, wakes, waiter_empty, exact_result, admission_released, registry_empty, wake_lock_released) in observations {
        assert_eq!(first_ready, row["firstReady"].as_bool().unwrap(), "history completion in registration gap must be observed in the same poll: {}", row["name"]);
        assert_eq!(wakes as u64, row["wakes"].as_u64().unwrap(), "{}", row["name"]);
        assert_eq!(waiter_empty, row["waiterEmpty"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(wake_lock_released, row["wakeLockReleased"].as_bool().unwrap(), "wake callback must not execute under the waiter mutex: {}", row["name"]);
        assert_eq!(exact_result, row["exactResult"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(admission_released, row["admissionReleased"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(registry_empty, row["registryEmpty"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[test]
fn artifact_history_empty_one_cap_plus_one_admission_returns_exact_request() {
    let mut claims = Vec::new();
    for _ in 0..ARTIFACT_HISTORY_OPERATION_SLOTS {
        claims.push(ArtifactHistoryAdmission::try_claim().unwrap());
    }
    assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))));
    for claim in claims {
        retire_history_admission(claim);
    }
    let source = retained_submit_source();
    assert!(source.contains("HistoryFrameToken::End"));
    assert!(source.contains("ArtifactHistoryWorkOwner::Request"));
}

#[test]
fn artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release() {
    let mut cancelled = ArtifactHistoryAdmission::try_claim().unwrap();
    let cancelled_generation = cancelled.generation;
    let mut cursor = cancelled.begin_reservation_close().expect("cancelled pre-handoff request retained its full reservation");
    let mut peers = Vec::new();
    for _ in 1..ARTIFACT_HISTORY_OPERATION_SLOTS {
        peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
    }
    assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))), "retirement must retain admission credit until terminal-empty");
    assert!(cursor.close_step());
    assert!(!cursor.terminal_is_empty());
    for _ in 0..50_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
    drop(cursor);
    drop(cancelled);
    let replacement = ArtifactHistoryAdmission::try_claim().unwrap();
    assert_ne!(replacement.generation, cancelled_generation);
    retire_history_admission(replacement);
    for peer in peers {
        retire_history_admission(peer);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty() {
    let root = tempdir("history-public-terminal-release").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("history-public-terminal-release".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();
    let mut peers = Vec::new();
    for _ in 1..ARTIFACT_HISTORY_OPERATION_SLOTS {
        peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
    }
    let history = handle.history();
    let generation = history.generation();
    let terminal = history.terminal_handle();
    history.cancel();
    drop(history);
    assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))));
    assert!(!terminal.terminal_is_empty());
    let mut released = false;
    for _ in 0..100_000 {
        if terminal.terminal_is_empty() {
            break;
        }
        let roots_were_empty = terminal.state.terminal_roots_are_empty();
        let admission_was_retained = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let progressed = terminal.close_step();
        let admission_is_released = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
        if admission_was_retained && admission_is_released {
            assert!(roots_were_empty, "the admission release must occupy a grant after every terminal root was already empty");
            assert!(progressed);
            assert!(terminal.state.finished.load(std::sync::atomic::Ordering::Acquire));
            released = true;
        }
        if !progressed {
            std::thread::yield_now();
        }
    }
    assert!(released);
    assert!(terminal.terminal_is_empty());
    assert!(handle.history_terminal(generation).is_none(), "terminal completion must unregister the released generation");
    let replacement = ArtifactHistoryAdmission::try_claim().unwrap();
    retire_history_admission(replacement);
    for peer in peers {
        retire_history_admission(peer);
    }
}

#[test]
fn artifact_history_nested_derived_item_and_byte_caps_precede_materialization() {
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    for required in ["HISTORY_REPLAY_RESULT_BYTES", "HISTORY_REPLAY_OPERATION_BYTES", "HISTORY_REPLAY_MAX_ENTRIES", "HISTORY_REPLAY_MAX_OPERATION_IDS", "history dependency item credit", "history result byte credit"] {
        assert!(artifact.contains(required), "missing {required}");
    }
    let preflight = artifact.find("operation_count >= HISTORY_REPLAY_MAX_OPERATION_IDS").unwrap();
    let publish = artifact.find("reservation.operation_ids.push(HistoryTextRange").unwrap();
    assert!(preflight < publish);
}

#[test]
fn artifact_history_segment_cap_plus_one_reads_only_one_admitted_page() {
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    assert!(artifact.contains("HISTORY_REPLAY_SEGMENT_PAGES: u64 = 1_024"));
    assert!(artifact.contains(".min(HISTORY_REPLAY_PAGE_BYTES)"));
    assert!(artifact.contains("pack::ByteRange { offset, len: requested }"));
    assert!(artifact.contains("page.capacity() as u64 > HISTORY_REPLAY_PAGE_BYTES"));
    assert!(!artifact.contains("pack::ByteRange { offset: 0, len }"));
}

#[test]
fn artifact_history_crc_and_frame_tokenizer_advance_one_page_per_grant() {
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    assert!(artifact.contains("protocol::codec::Crc32cCursor"));
    assert!(artifact.contains("self.crc.update_page(page)"));
    assert!(artifact.contains("self.payload_remaining.min(HISTORY_REPLAY_PAGE_BYTES)"));
    assert!(!artifact.contains("protocol::codec::crc32c(whole_frame)"));
    assert!(!artifact.contains("decode_history_token"));
}

#[test]
fn artifact_history_cancel_retires_one_page_or_nested_owner_per_actor_grant() {
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    assert!(artifact.contains("HistoryReplayTransition::FaultRetire"));
    assert!(artifact.contains("self.page_count -= 1"));
    assert!(artifact.contains("self.operation_ids.pop().is_some()"));
    assert!(artifact.contains("self.result_pages.pop()"));
    assert!(!artifact.contains("pages.clear()"));
}

#[test]
fn artifact_history_quiet_late_wake_and_retry_are_generation_coalesced() {
    let source = retained_submit_source();
    assert!(source.contains("impl std::task::Wake for ArtifactHistoryWake"));
    assert!(source.contains("self.generation == state.generation"));
    assert!(source.contains("self.scheduled.compare_exchange(false, true"));
    assert!(source.contains("self.pool.callback_at"));
    assert!(source.contains("retry_generation"));
}

#[test]
fn artifact_history_cancel_before_during_after_retains_actor_and_result_owners() {
    let source = retained_submit_source();
    assert!(source.contains("history_retained(self.generation, self.cancelled.clone(), reservation)"));
    assert!(source.contains("terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled)"));
    assert!(source.contains("self.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled)"));
    assert!(source.contains("*self.terminal_result.lock()"));
    assert!(source.contains("HistoryProgress::Completed | HistoryProgress::Cancelled | HistoryProgress::Fault"));
}

#[test]
fn artifact_history_stale_generation_and_slot_aba_precede_mailbox_mutation() {
    let first = ArtifactHistoryAdmission::try_claim().unwrap();
    let slot = first.slot;
    let generation = first.generation;
    retire_history_admission(first);
    let next = ArtifactHistoryAdmission::try_claim().unwrap();
    assert_eq!(next.slot, slot);
    assert_ne!(next.generation, generation);
    let source = retained_submit_source();
    let stale = source.find("if self.authority.generation() != self.authority_generation").unwrap();
    let handoff = source.find("self.authority.history_retained").unwrap();
    assert!(stale < handoff);
    retire_history_admission(next);
}

#[test]
fn artifact_history_replay_ordering_is_segment_frame_then_result_fifo() {
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    assert!(artifact.contains("HistoryReplayPhase::Probe { index: 0 }"));
    assert!(artifact.contains("cursor: HistoryFrameCursor::new(next_offset)"));
    assert!(artifact.contains("reservation.entries.push(ArtifactHistoryEntry"));
    assert!(!retained_submit_source().contains("ArtifactHistoryWorkOwner::Map"));
}

#[test]
fn artifact_history_terminal_job_work_result_take_resume_and_close_one_owner() {
    let source = retained_submit_source();
    for required in [
        "pub fn take_terminal_job",
        "pub fn take_terminal_work",
        "pub fn take_terminal_result",
        "pub fn take_actor_terminal_job",
        "pub fn close_step",
        "pub fn terminal_is_empty",
        "impl ArtifactHistoryTerminalJob",
        "impl ArtifactHistoryTerminalWork",
        "pub struct ArtifactHistoryTerminalReservation",
        "pub fn take_terminal_reservation",
    ] {
        assert!(source.contains(required), "missing {required}");
    }
    assert!(source.contains("fn close_one(self: &Arc<Self>) -> bool"));
}

#[test]
fn artifact_history_construction_fault_is_public_and_admission_release_is_a_final_grant() {
    let engine = retained_submit_source();
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    for required in [
        "terminal_construction: std::sync::Mutex<Option<db_artifact::HistoryReplayReservationConstructionFault>>",
        "pub struct ArtifactHistoryTerminalConstructionFault",
        "pub fn take_terminal_construction_fault",
        "pub fn resume(mut self) -> Result<(), Self>",
        "fn terminal_roots_are_empty(&self) -> bool",
        "self.finished.load(std::sync::atomic::Ordering::Acquire)",
        "self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()",
        "self.state.finish_if_terminal_empty()",
    ] {
        assert!(engine.contains(required), "missing retained public construction/admission authority: {required}");
    }
    assert!(artifact.contains("const HISTORY_REPLAY_CONSTRUCTION_SLOTS: usize = 64"));
    assert!(artifact.contains("pub(crate) fn try_new()"));
    assert!(artifact.contains("pub(crate) struct HistoryReplayReservationConstructionFault"));
    assert!(artifact.contains("impl Drop for HistoryReplayReservationConstructionBuilder"));
    assert!(artifact.contains("impl Drop for HistoryReplayReservationConstructionFault"));
    assert!(artifact.contains("take_history_replay_reservation_construction_fault"));
    assert!(!artifact.contains("pub fn into_parts"));
    assert!(!artifact.contains("pub struct HistoryReplayReservationConstructionFaultCursor"));
    assert!(artifact.contains("try_new_with_result_page_failure"));
    let terminal = &engine[engine.find("impl ArtifactHistoryTerminalHandle").unwrap()..engine.find("impl Future for HistoryFuture").unwrap()];
    let close = &terminal[terminal.find("pub fn close_step(&self)").unwrap()..terminal.find("pub fn terminal_is_empty(&self)").unwrap()];
    assert!(close.find("self.state.close_one()").unwrap() < close.find("self.state.finish_if_terminal_empty()").unwrap());
    assert!(close.contains("return true"), "owner retirement must return before the final admission-release grant");
}

#[test]
fn artifact_history_one_grant_advances_one_retained_phase_without_blocking() {
    let engine = retained_submit_source();
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    let history = &engine[engine.find("const ARTIFACT_HISTORY_OPERATION_SLOTS").unwrap()..engine.find("pub struct ArtifactHandle").unwrap()];
    let replay = &artifact[artifact.find("//#region 🔖️HistoryReplay").unwrap()..artifact.find("//#endregion 🔖️HistoryReplay").unwrap()];
    for forbidden in ["block_on(", "ask_blocking", "submit_blocking", "loop {", "while "] {
        assert!(!history.contains(forbidden), "history outer authority retained {forbidden}");
        assert!(!replay.contains(forbidden), "history replay cursor retained {forbidden}");
    }
    assert!(history.contains("std::pin::Pin::new(future).poll(&mut context)"));
    assert!(artifact.contains("future.as_mut().poll(&mut context)"));
}

#[test]
fn artifact_history_drop_after_complete_moves_result_to_public_terminal_registry() {
    let source = retained_submit_source();
    let result_drop = &source[source.find("impl Drop for HistoryView").unwrap()..source.find("//#endregion 🔖️History").unwrap()];
    assert!(result_drop.contains("register_artifact_history(&state)"));
    assert!(result_drop.contains("state.terminal_result"));
    assert!(result_drop.contains("self.inner.take()"));
    assert!(source.contains("pub fn history_terminal(&self, generation: u64)"));
}

#[test]
fn artifact_history_runner_close_mid_turn_retains_replay_until_terminal_empty() {
    let artifact = include_str!("../../../🗿️artifact/🦀️.rs");
    let runner = &artifact[artifact.find("enum ArtifactTurn").unwrap()..artifact.find("//#region 🧪️Tests").unwrap()];
    assert!(runner.contains("ArtifactTurn::History"));
    assert!(runner.contains("replay.request_close(DbError::Closed)"));
    assert!(runner.contains("!replay.terminal_is_empty()"));
    assert!(runner.contains("Pin::new(&mut *replay).poll(&mut context)"));
    let panic_close = &runner[runner.find("history replay cursor panicked").unwrap()..];
    assert!(panic_close.contains("replay.request_close"));
    assert!(panic_close.contains("self.schedule()"));
    assert!(!runner.contains("turn.take();\n                    drop(turn);\n                    self.address.close();"));
}

#[test]
fn artifact_history_future_handle_drop_and_terminal_take_resume_are_exact() {
    let source = retained_submit_source();
    let future_drop = &source[source.find("impl Drop for HistoryFuture").unwrap()..source.find("impl ArtifactHistoryTerminalJob").unwrap()];
    assert!(future_drop.contains("completion.take()"));
    assert!(future_drop.contains("terminal_result"));
    assert!(!future_drop.contains("self.state.close_one()"));
    for required in ["pub struct ArtifactHistoryTerminalHandle", "pub fn terminal_handle(&self)", "pub fn take_terminal_result(&self)", "pub fn resume(mut self)", "pub fn close_step(&self)"] {
        assert!(source.contains(required), "missing {required}");
    }
}
//#endregion 🔖️Retained submit authority

//#region 🔖️Security
#[semio_framework_async_macros::async_test]
async fn security_authz_hook_rejects_a_principal_denied_by_its_policy() {
    let policy = db_security::RoleBasedPolicy::new();
    let gate = db_security::SecurityGate::new(policy, db_security::ReplayGuard::new(60_000, 16), db_security::BudgetRegistry::new(100, 10), Arc::new(NullEmit));
    let hook = SecurityAuthzHook::new(gate, |actor| db_security::Principal::new(actor.clone(), db_security::TenantId::from("tenant-1"), vec!["viewer".to_string()])).await;

    let document = protocol::ArtifactId("doc-1".to_string());
    let envelope = envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await;
    let result = db_artifact::AuthzHook::authorize(&hook, &envelope.actor, &envelope);
    assert!(matches!(result.await, Err(DbError::Unauthorized(_))), "a default-deny policy with no grants must reject every action");
}
//#endregion 🔖️Security
