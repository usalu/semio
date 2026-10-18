use super::*;

use db_engine_base::{CatalogEntry, CatalogState, DatabaseCreateCatalogToken, encode_catalog_pages, test_worker_pool};
use db_io_core::{PayloadStorage as _, WalStorage as _};

/// 🧾️ The authored source of the create-catalog crate: every child module and then the root, in module order, as the one text the source laws scan.
const CATALOG_CREATE_SOURCE: &str = concat!(
    include_str!("../../🧮️limits-ledger-and-admission/🦀️.rs"),
    include_str!("../../📋️progress-result-and-rejection/🦀️.rs"),
    include_str!("../../🧵️work-cursor-phase-and-state/🦀️.rs"),
    include_str!("../../🚦️state-driver/🦀️.rs"),
    include_str!("../../🔎️state-scan-clone-and-encode/🦀️.rs"),
    include_str!("../../📌️state-claim-revalidate-and-publish/🦀️.rs"),
    include_str!("../../📤️future-terminal-and-state-drop/🦀️.rs"),
    include_str!("../../🦀️.rs"),
);

async fn catalog_bootstrap_pages(page_count: usize) -> db_io_core::DbIoPages {
    let mut writer = db_io_core::DbIoPageWriter::try_reserve(page_count).unwrap();
    let full = [b'x'; db_io_core::DB_IO_PAGE_BYTES];
    for index in 0..page_count {
        let source = if index + 1 == page_count { &b"[x]"[..] } else { &full[..] };
        assert_eq!(writer.write_fragment(source).unwrap(), source.len());
    }
    writer.seal_retained().await.unwrap()
}
//#region 🧸️Fixtures
async fn create_catalog_fixture(entries: Vec<CatalogEntry>) -> (Arc<db_storage::DbBackend>, Arc<Mutex<CatalogState>>, EpochFence) {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    pub mode: ControlledCreateCatalogPoll,
    pub pages: Option<db_io_core::DbIoPages>,
    ready_epoch: EpochFence,
    cancel_on_ready: Option<std::sync::Weak<DatabaseCreateCatalogState>>,
    pub polls: Arc<std::sync::atomic::AtomicUsize>,
    pub waker: Arc<Mutex<Option<std::task::Waker>>>,
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

fn held_create_catalog_io_pool() -> (Arc<WorkerPool>, Arc<(Mutex<bool>, std::sync::Condvar)>) {
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
    (pool, gate)
}

fn replenishing_create_catalog_io_job(pool: Arc<WorkerPool>, active: Arc<std::sync::atomic::AtomicBool>) -> semio_framework_async::Job {
    Box::new(move || {
        let mut next = replenishing_create_catalog_io_job(pool.clone(), active.clone());
        while active.load(std::sync::atomic::Ordering::Acquire) {
            match pool.try_submit(Lane::Io, next) {
                Ok(()) => return,
                Err(error) if error.kind() == semio_framework_async::WorkerSubmitErrorKind::Shutdown => return,
                Err(error) => next = error.into_job(),
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
    admit_fixture_blocker(&pool,
        Lane::Maintenance,
        Box::new(move || {
            maintenance_tx.send(()).unwrap();
            let (lock, ready) = &*held_maintenance;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    );
    maintenance_rx.recv().unwrap();
    let (service_tx, service_rx) = std::sync::mpsc::sync_channel(1);
    let held_service = service_gate.clone();
    admit_fixture_blocker(&pool,
        Lane::UserVisible,
        Box::new(move || {
            service_tx.send(()).unwrap();
            let (lock, ready) = &*held_service;
            let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            while !*released {
                released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }),
    );
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
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while vector_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
        assert!(std::time::Instant::now() < deadline, "create-catalog callback close did not release its admission");
        semio_framework_async::yield_once().await;
    }

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
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while string_state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
        assert!(std::time::Instant::now() < deadline, "create-catalog callback close did not release its admission");
        semio_framework_async::yield_once().await;
    }
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
    assert_eq!(state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pages.as_ref().map(db_io_core::DbIoPages::operation), Some(operation));
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
        let mut writer = db_io_core::DbIoPageWriter::try_reserve(1).unwrap();
        writer.write_fragment(b"[]").unwrap();
        let pages = writer.seal_retained().await.unwrap();
        let operation = pages.operation();
        let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let waker = Arc::new(Mutex::new(None));
        let future = ControlledCreateCatalogFuture { mode, pages: Some(pages), ready_epoch: epoch.next(), cancel_on_ready: (mode == ControlledCreateCatalogPoll::Ready).then(|| Arc::downgrade(&state)), polls: polls.clone(), waker: waker.clone() };
        *state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCreateCatalogWork::controlled(Box::pin(future), pointer, operation));
        {
            let mut cursor = state.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            cursor.snapshot = Some(Arc::new(Vec::new()));
            state.catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner).pending = Some(DatabaseCreateCatalogToken { slot: state.slot, generation: state.generation, revision: cursor.base_revision });
            state.pending_owned.store(true, std::sync::atomic::Ordering::Release);
        }
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
    assert_eq!(cancel_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), 1);
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
    assert_eq!(deadline_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), 1);
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
    assert_eq!(exhaust_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
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
    assert_eq!(exhaust_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    active.store(false, std::sync::atomic::Ordering::Release);
    pool.shutdown();

    let (pool, gate, active) = replenishing_held_create_catalog_io_pool();
    let rejection_storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    assert_eq!(close.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
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
    assert_eq!(cancel_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), 1);
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
    assert_eq!(deadline_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), 1);
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
    assert_eq!(exhaust_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
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
    assert_eq!(exhaust_state.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
    active.store(false, std::sync::atomic::Ordering::Release);
    release_held_create_catalog_worker(&maintenance_gate);
    pool.shutdown();

    let (pool, maintenance_gate, service_gate, active) = reserved_replenishing_create_catalog_io_pool();
    let rejection_storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
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
    assert_eq!(close.spent_submission_attempts.load(std::sync::atomic::Ordering::Acquire), usize::from(DATABASE_CREATE_CATALOG_RETRY_LIMIT));
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
    let initial = terminal.witness().retained_owners;
    let initial_grants = state.opportunities.load(std::sync::atomic::Ordering::Acquire);
    while !terminal.terminal_is_empty() {
        let step = terminal.close_step();
        assert!(matches!(step, DatabaseCreateCatalogCloseStep::Progress | DatabaseCreateCatalogCloseStep::Blocked | DatabaseCreateCatalogCloseStep::Complete));
        let current = terminal.witness().retained_owners;
        let granted = state.opportunities.load(std::sync::atomic::Ordering::Acquire);
        assert!(initial.saturating_sub(current) as u64 <= granted - initial_grants, "every retired owner consumes its own lane grant");
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
    let source = CATALOG_CREATE_SOURCE;
    let region = &source[source.find("//#region 🔖️CreateDocumentCatalogCas").unwrap()..source.rfind("//#endregion 🔖️CreateDocumentCatalogCas").unwrap()];
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
    let source = include_str!("../../../🛠️database-catalog-shutdown-and-hello/🦀️.rs");
    let catalog = &source[source.find("pub async fn catalog(&self)").unwrap()..source.find("pub async fn health(&self)").unwrap()];
    assert!(catalog.find("Arc::clone(&catalog.entries)").unwrap() < catalog.find("entries.as_ref().clone()").unwrap());
    assert!(!catalog.contains("catalog.entries.as_ref().clone()"));
    pool.shutdown();
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
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !terminal.terminal_is_empty() {
        assert!(std::time::Instant::now() < deadline, "resolved create-catalog drop did not drain its retained terminal owner");
        let _ = terminal.close_step();
        semio_framework_async::yield_once().await;
    }
    drop(terminal);
    assert_eq!(pool.shutdown(), Ok(()));
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
//#endregion 🔖️Database open/catalog

//#region 🔖️Source laws
#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_engine_catalog_create", CATALOG_CREATE_SOURCE)] {
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
