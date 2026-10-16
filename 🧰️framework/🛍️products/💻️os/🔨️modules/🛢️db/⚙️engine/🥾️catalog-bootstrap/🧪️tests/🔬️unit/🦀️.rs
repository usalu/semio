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
enum ControlledCatalogBootstrapPoll {
    Pending,
    NoService,
    Ready,
    Fenced,
    Panic,
}

struct ControlledCatalogBootstrapFuture {
    pub mode: ControlledCatalogBootstrapPoll,
    pub pages: Option<db_io_core::DbIoPages>,
    pub polls: Arc<std::sync::atomic::AtomicUsize>,
    pub waker: Arc<Mutex<Option<std::task::Waker>>>,
}

impl Future for ControlledCatalogBootstrapFuture {
    type Output = Result<EpochFence, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let poll = self.polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
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

async fn catalog_bootstrap_pages(page_count: usize) -> db_io_core::DbIoPages {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(page_count).unwrap();
    let full = [b'x'; db_io_core::DB_IO_PAGE_BYTES];
    for index in 0..page_count {
        let source = if index + 1 == page_count { &b"[x]"[..] } else { &full[..] };
        assert_eq!(writer.write_fragment(source).unwrap(), source.len());
    }
    writer.seal_retained().await.unwrap()
}

async fn empty_catalog_bootstrap_pages() -> db_io_core::DbIoPages {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(1).unwrap();
    assert_eq!(writer.write_fragment(b"[]").unwrap(), 2);
    writer.seal_retained().await.unwrap()
}

async fn controlled_catalog_bootstrap_probe(mode: ControlledCatalogBootstrapPoll) -> (DatabaseCatalogBootstrapFuture, Arc<std::sync::atomic::AtomicUsize>, Arc<Mutex<Option<std::task::Waker>>>, usize, u64) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    if !db_io_driver::process_isolated_law("tests::database_catalog_bootstrap_real_max_plus_one_refusal_returns_pages_storage_key_and_fence") {
        return;
    }
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
        loop {
            let successor = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop();
            let Some(successor) = successor else { break };
            successor();
        }
        *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        state.schedule();
        let result = probe.await.unwrap();
        let (storage, _, _, actual) = result.into_parts().unwrap();
        assert_eq!(Arc::as_ptr(&storage) as usize, pointer);
        if mode == ControlledCatalogBootstrapPoll::Panic {
            assert_eq!(actual, Err(DbError::LimitExceeded("database catalog-bootstrap backend poll panic")));
        } else {
            assert!(matches!(actual, Err(DbError::Closed) | Err(DbError::Fenced { .. }) | Err(DbError::Unavailable(_))));
        }
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let pointer = Arc::as_ptr(&storage) as usize;
    let probe = DatabaseCatalogBootstrapFuture::try_prepare_with_key(pool, storage, catalog_bootstrap_pages(3).await, DatabaseCatalogBootstrapKey::root(), EpochFence::INITIAL, false).unwrap();
    let state = probe.state.clone();
    let submitted = Arc::new(Mutex::new(ControlledCapabilitySubmitQueue::new()));
    let queue = submitted.clone();
    *state.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |job| queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(job)));
    state.schedule();
    let job = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
    job();
    assert_eq!(state.phase(), DatabaseCatalogBootstrapPhase::Poll);
    {
        let work = state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let work = work.as_ref().unwrap();
        assert!(work.future.is_none());
        assert_eq!(work.pages.as_ref().map(db_io_core::DbIoPages::page_count), Some(3));
        assert_eq!(work.storage.as_ref().map(|storage| Arc::as_ptr(storage) as usize), Some(pointer));
    }
    probe.cancel();
    while state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
        let job = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
        job();
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

    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    let initial_grants = state.driver_grants.load(std::sync::atomic::Ordering::Acquire);
    let grant_in_flight = u64::from(state.driver_authority.load(std::sync::atomic::Ordering::Acquire) == DatabaseCatalogBootstrapDriverAuthority::Driving as u8);
    let initial = terminal.witness().retained_owners;
    while !terminal.terminal_is_empty() {
        let step = terminal.close_step();
        assert!(matches!(step, DatabaseCatalogBootstrapCloseStep::Progress | DatabaseCatalogBootstrapCloseStep::Blocked | DatabaseCatalogBootstrapCloseStep::Complete));
        let current = terminal.witness().retained_owners;
        let granted = state.driver_grants.load(std::sync::atomic::Ordering::Acquire);
        assert!(initial.saturating_sub(current) as u64 <= granted - initial_grants + grant_in_flight, "one mounted close grant retires at most one owner");
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
    admit_fixture_blocker(&pool,
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            let (lock, ready) = &*worker_gate;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    );
    started_rx.recv().unwrap();
    loop {
        if let Err(error) = pool.try_submit(Lane::Io, Box::new(|| {})) {
            assert_eq!(error.kind(), semio_framework_async::WorkerSubmitErrorKind::Saturated);
            drop(error.into_job());
            break;
        }
    }
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
        let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    let job = submitted.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pop().unwrap();
    job();
    assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
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
    for (name, source) in [("db_engine_catalog_bootstrap", include_str!("../../🦀️.rs"))] {
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
