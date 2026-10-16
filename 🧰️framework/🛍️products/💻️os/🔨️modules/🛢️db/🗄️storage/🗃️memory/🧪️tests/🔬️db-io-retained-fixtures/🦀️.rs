use super::*;

use db_io_core::{DB_IO_LEDGER_OWNER, DB_IO_LIST_BACKING_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_PAGE_BYTES, DB_IO_PROCESS_BYTES, DB_IO_PROCESS_WITH_WRITER_BACKING_BYTES, DB_IO_TASK_SLOTS, DB_IO_WRITER_STATIC_BACKING_BYTES, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoCredit, DbIoExecutionStep, DbIoExecutorMode, DbIoFaultKind, DbIoLedgerOwner, DbIoPagePhase, DbIoPageWriter, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoWriterReleaseStep, LeaseStorage as _, PayloadStorage as _, WalStorage as _, WalWriterPermit, db_io_backend_owner_admits, db_io_backend_parts, db_io_ledger_census, db_io_operation_ledger, db_io_operation_reserve, db_io_operation_return, db_io_operation_slot, db_io_page_arena, db_io_result_lease_credit, db_io_slot_matches};
use db_io_driver::{WalWriterGuard as _, close_db_io_backend, db_io_backend_admit_operation, db_io_maintenance_step, db_io_take_async_executor, db_io_test_pool, process_isolated_law, retire_db_io_backend, submit_db_io_task};
use db_io_registry::{DbIoBackendKind, db_io_backend_close_step, db_io_backend_registry, db_io_backend_return_operation, db_io_page_maintenance_step, db_io_return_async_executor, db_io_writer_release_lane_step, register_db_io_backend};

fn fixture_owner() -> DbIoLedgerOwner {
    DbIoLedgerOwner::enter()
}

struct WriterControllerLawGuard {
    failures: Arc<std::sync::atomic::AtomicUsize>,
    pub closed: bool,
}

impl db_io_driver::WalWriterGuard for WriterControllerLawGuard {
    fn close_step(&mut self) -> Result<bool, DbError> {
        if self.failures.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |value| value.checked_sub(1)).is_ok() {
            return Err(DbError::Io("exact writer unlock fault".to_string()));
        }
        if self.closed {
            return Ok(false);
        }
        self.closed = true;
        Ok(true)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closed
    }
}

struct WriterControllerLawExecutor {
    pub table: std::sync::Mutex<Option<Box<db_io_driver::WalWriterTable<WriterControllerLawGuard>>>>,
    pub mode: DbIoExecutorMode,
    pub turns: Arc<std::sync::atomic::AtomicUsize>,
    panic_release: bool,
}

impl DbIoTaskExecutor for WriterControllerLawExecutor {
    fn supports_writer_authority(&self) -> bool {
        true
    }
    fn mode(&self) -> DbIoExecutorMode {
        self.mode
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn owner_backing_bytes(&self) -> u64 {
        (size_of::<Self>() + size_of::<db_io_driver::WalWriterTable<WriterControllerLawGuard>>()) as u64
    }
    fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
        let mut table = lock(&self.table);
        assert!(table.is_none());
        *table = Some(Box::new(db_io_driver::WalWriterTable::for_backend(control)));
        Ok(())
    }
    fn writer_release_step(&self, _context: &mut std::task::Context<'_>) -> Result<DbIoWriterReleaseStep, DbError> {
        self.turns.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        assert!(!self.panic_release, "outer writer controller fault");
        match lock(&self.table).as_mut() {
            Some(table) => table.release_requested_step(),
            None => Ok(DbIoWriterReleaseStep::Idle),
        }
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Ok(DbIoResult::Unit))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        let mut table = lock(&self.table);
        if let Some(owner) = table.as_mut() {
            if owner.close_step()? {
                return Ok(false);
            }
            assert!(owner.terminal_is_empty());
            table.take();
            return Ok(false);
        }
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        lock(&self.table).is_none()
    }
}

struct WriterRegistryProbeWake {
    wakes: std::sync::atomic::AtomicUsize,
    registry_available: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for WriterRegistryProbeWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(250);
        let available = loop {
            if db_io_backend_registry().try_lock().is_ok() {
                break true;
            }
            if std::time::Instant::now() >= deadline {
                break false;
            }
            std::thread::yield_now();
        };
        self.registry_available.store(available, std::sync::atomic::Ordering::Release);
        self.wakes.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

struct WriterDeferredRefusalWake {
    wakes: std::sync::atomic::AtomicUsize,
    pub external: Arc<std::sync::Mutex<()>>,
    woke_under_external_lock: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for WriterDeferredRefusalWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.woke_under_external_lock.store(self.external.try_lock().is_err(), std::sync::atomic::Ordering::Release);
        self.wakes.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

fn writer_controller_law_table<T: 'static>(control: DbIoBackendControl, action: impl FnOnce(&mut db_io_driver::WalWriterTable<WriterControllerLawGuard>) -> T) -> T {
    let mut registry = lock(db_io_backend_registry());
    let owner = &mut registry.slots[usize::from(db_io_backend_parts(control).0)];
    assert_eq!(owner.generation, db_io_backend_parts(control).1);
    let executor = owner.executor.as_mut().unwrap().as_any_mut().downcast_mut::<WriterControllerLawExecutor>().unwrap();
    let mut table = lock(&executor.table);
    let result = action(table.as_mut().unwrap());
    drop(table);
    result
}

fn register_writer_controller_law(mode: DbIoExecutorMode) -> (DbIoBackendControl, Arc<std::sync::atomic::AtomicUsize>) {
    register_writer_controller_law_on(mode, db_io_test_pool())
}

fn register_writer_controller_law_on(mode: DbIoExecutorMode, pool: Arc<WorkerPool>) -> (DbIoBackendControl, Arc<std::sync::atomic::AtomicUsize>) {
    let turns = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(WriterControllerLawExecutor { table: std::sync::Mutex::new(None), mode, turns: turns.clone(), panic_release: false }), pool).unwrap();
    (control, turns)
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_fences_at_signal_and_wakes_outside_registry_without_tasks() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["controllerBinding"];
    assert!(DB_IO_WRITER_STATIC_BACKING_BYTES <= row["maximumStaticBytes"].as_u64().unwrap());
    assert_eq!(DB_IO_PROCESS_WITH_WRITER_BACKING_BYTES, DB_IO_PROCESS_BYTES + DB_IO_WRITER_STATIC_BACKING_BYTES);
    let (control, turns) = register_writer_controller_law(DbIoExecutorMode::BlockingLane);
    let mounted = ledger_witness();
    let document = DbIoText::try_from_str("mounted-writer-authority").unwrap();
    let probe = Arc::new(WriterRegistryProbeWake { wakes: std::sync::atomic::AtomicUsize::new(0), registry_available: std::sync::atomic::AtomicBool::new(false) });
    let waker = std::task::Waker::from(probe.clone());
    let (mut release, key) = writer_controller_law_table(control, |table| {
        let permit = table.acquire_with(&document, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).unwrap();
        let key = permit.key();
        let mut factory_calls = 0;
        assert!(table
            .acquire_with(&document, || {
                factory_calls += 1;
                Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })
            })
            .is_err());
        assert_eq!(factory_calls, row["guardFactoryCallsOnConflict"].as_u64().unwrap());
        table.pin_operation(key, control, &document, row["operation"].as_u64().unwrap()).unwrap();
        let mut release = permit.release();
        assert!(matches!(table.pin_operation(key, control, &document, row["contender"].as_u64().unwrap()), Err(DbError::Conflict(_))));
        assert!(Pin::new(&mut release).poll(&mut std::task::Context::from_waker(&waker)).is_pending());
        assert!(matches!(table.pin_operation(key, control, &document, row["contender"].as_u64().unwrap()), Err(DbError::Closed)));
        table.pin_operation(key, control, &document, row["operation"].as_u64().unwrap()).unwrap();
        (release, key)
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while turns.load(std::sync::atomic::Ordering::Acquire) == 0 {
        assert!(std::time::Instant::now() < deadline);
        semio_framework_async::yield_once().await;
    }
    assert_eq!(probe.wakes.load(std::sync::atomic::Ordering::Acquire), 0);
    assert_eq!(ledger_witness(), mounted);
    writer_controller_law_table(control, |table| table.finish_operation(key, control, &document, row["operation"].as_u64().unwrap()).unwrap());
    while probe.wakes.load(std::sync::atomic::Ordering::Acquire) == 0 {
        assert!(std::time::Instant::now() < deadline);
        semio_framework_async::yield_once().await;
    }
    assert_eq!(probe.wakes.load(std::sync::atomic::Ordering::Acquire) as u64, row["wakesAtTerminal"].as_u64().unwrap());
    assert!(probe.registry_available.load(std::sync::atomic::Ordering::Acquire));
    assert!(matches!(Pin::new(&mut release).poll(&mut std::task::Context::from_waker(&waker)), std::task::Poll::Ready(Ok(()))));
    assert_eq!(ledger_witness(), mounted);
    writer_controller_law_table(control, |table| assert!(table.terminal_is_empty()));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let (control, _) = register_writer_controller_law(DbIoExecutorMode::BlockingLane);
    let document = DbIoText::try_from_str("faulted-writer-authority").unwrap();
    let other = DbIoText::try_from_str("independent-writer-authority").unwrap();
    let probe = Arc::new(WriterRegistryProbeWake { wakes: std::sync::atomic::AtomicUsize::new(0), registry_available: std::sync::atomic::AtomicBool::new(false) });
    let waker = std::task::Waker::from(probe.clone());
    let (mut release, key, second) = writer_controller_law_table(control, |table| {
        let permit = table.acquire_with(&document, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(1)), closed: false })).unwrap();
        let key = permit.key();
        let second = table.acquire_with(&other, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).unwrap();
        let mut release = permit.release();
        assert!(Pin::new(&mut release).poll(&mut std::task::Context::from_waker(&waker)).is_pending());
        (release, key, second)
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while probe.wakes.load(std::sync::atomic::Ordering::Acquire) == 0 {
        assert!(std::time::Instant::now() < deadline);
        semio_framework_async::yield_once().await;
    }
    assert!(probe.registry_available.load(std::sync::atomic::Ordering::Acquire));
    let failure = match Pin::new(&mut release).poll(&mut std::task::Context::from_waker(&waker)) {
        std::task::Poll::Ready(Err(failure)) => failure,
        _ => panic!("exact unlock fault did not return retained close authority"),
    };
    assert!(failure.error().to_string().contains("exact writer unlock fault"));
    writer_controller_law_table(control, |table| {
        assert!(!table.terminal_is_empty());
        assert!(matches!(table.validate(key, control, &document), Err(DbError::Closed)));
        assert!(table.validate(second.key(), control, &other).is_ok());
    });
    let (_, retained) = failure.into_parts();
    retained.retry().await.unwrap();
    second.release().await.unwrap();
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let mut blocker: Job = Box::new(move || {
        started_tx.send(()).expect("refusal blocker start");
        release_rx.recv().expect("refusal blocker release");
    });
    let admission_deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match pool.try_submit(Lane::Interactive, blocker) {
            Ok(()) => break,
            Err(error) if error.kind() == WorkerSubmitErrorKind::Contended => {
                assert!(std::time::Instant::now() < admission_deadline, "refusal blocker queue remained contended");
                blocker = error.into_job();
                std::thread::yield_now();
            }
            Err(error) => panic!("refusal blocker admission failed: {:?}", error.kind()),
        }
    }
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("sole worker occupied");

    let (control, turns) = register_writer_controller_law_on(DbIoExecutorMode::BlockingLane, pool.clone());
    let mounted = ledger_witness();
    let document_a = DbIoText::try_from_str("refusal-writer-a").unwrap();
    let document_b = DbIoText::try_from_str("refusal-writer-b").unwrap();
    let (permit_a, mut release_b, key_a, key_b) = writer_controller_law_table(control, |table| {
        let permit_a = table.acquire_with(&document_a, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).unwrap();
        let permit_b = table.acquire_with(&document_b, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).unwrap();
        let key_a = permit_a.key();
        let key_b = permit_b.key();
        (permit_a, permit_b.release(), key_a, key_b)
    });
    let external = Arc::new(std::sync::Mutex::new(()));
    let probe = Arc::new(WriterDeferredRefusalWake { wakes: std::sync::atomic::AtomicUsize::new(0), external: external.clone(), woke_under_external_lock: std::sync::atomic::AtomicBool::new(false) });
    let waker = std::task::Waker::from(probe.clone());
    assert!(Pin::new(&mut release_b).poll(&mut std::task::Context::from_waker(&waker)).is_pending());
    db_io_driver::suspend_controller_for_refusal(control).unwrap();
    let external_guard = external.lock().unwrap();
    let mut release_a = permit_a.release();
    let failure_a = match Pin::new(&mut release_a).poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(Err(failure)) => failure,
        _ => panic!("A must immediately retain the missing-controller refusal that defers B's waiter"),
    };
    assert!(db_io_driver::deferred_wake_pending_for_test(key_b));
    assert!(!db_io_driver::deferred_wake_pending_for_test(key_a));
    assert!(Pin::new(&mut release_b).poll(&mut std::task::Context::from_waker(&waker)).is_pending());
    assert_eq!(probe.wakes.load(std::sync::atomic::Ordering::Acquire), 0);
    assert_eq!(turns.load(std::sync::atomic::Ordering::Acquire), 0);
    assert_eq!(ledger_witness(), mounted);
    drop(external_guard);
    release_tx.send(()).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while probe.wakes.load(std::sync::atomic::Ordering::Acquire) == 0 {
        assert!(std::time::Instant::now() < deadline);
        semio_framework_async::yield_once().await;
    }
    assert_eq!(probe.wakes.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(!probe.woke_under_external_lock.load(std::sync::atomic::Ordering::Acquire));
    assert!(!db_io_driver::deferred_wake_pending_for_test(key_b));
    let failure_b = match Pin::new(&mut release_b).poll(&mut std::task::Context::from_waker(&waker)) {
        std::task::Poll::Ready(Err(failure)) => failure,
        _ => panic!("B must expose its retained refusal only after the exact deferred slot drains"),
    };
    writer_controller_law_table(control, |table| {
        assert!(matches!(table.validate(key_a, control, &document_a), Err(DbError::Closed)));
        assert!(matches!(table.validate(key_b, control, &document_b), Err(DbError::Closed)));
    });
    db_io_driver::restore_controller_after_refusal(control).unwrap();
    failure_b.into_parts().1.retry().await.unwrap();
    failure_a.into_parts().1.retry().await.unwrap();
    writer_controller_law_table(control, |table| assert!(table.terminal_is_empty()));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_rerequests_after_async_executor_handback() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let (control, _) = register_writer_controller_law(DbIoExecutorMode::AsyncNative);
    let document = DbIoText::try_from_str("async-leased-writer").unwrap();
    let permit = writer_controller_law_table(control, |table| table.acquire_with(&document, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).unwrap());
    let credit = DbIoCredit { items: 1, ..DbIoCredit::default() };
    let operation = db_io_operation_reserve(credit).unwrap();
    let _registered_pool = db_io_backend_admit_operation(control, operation).unwrap();
    let executor = db_io_take_async_executor(control, operation).unwrap();
    let mut release = permit.release();
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    assert!(matches!(db_io_writer_release_lane_step(control, &mut context).unwrap(), DbIoWriterReleaseStep::Idle));
    assert!(Pin::new(&mut release).poll(&mut context).is_pending());
    db_io_return_async_executor(control, operation, executor).unwrap();
    assert!(matches!(db_io_writer_release_lane_step(control, &mut context).unwrap(), DbIoWriterReleaseStep::Idle));
    db_io_backend_return_operation(control, operation).unwrap();
    db_io_operation_return(operation, credit).unwrap();
    release.await.unwrap();
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_coalesced_fault_does_not_strand_healthy_release() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let (control, _) = register_writer_controller_law(DbIoExecutorMode::BlockingLane);
    let document = DbIoText::try_from_str("coalesced-fault").unwrap();
    let other = DbIoText::try_from_str("coalesced-healthy").unwrap();
    let (first, second) = writer_controller_law_table(control, |table| {
        let first = table.acquire_with(&document, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(1)), closed: false })).unwrap();
        let second = table.acquire_with(&other, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).unwrap();
        (first.release(), second.release())
    });
    let failure = first.await.unwrap_err();
    assert!(failure.error().to_string().contains("exact writer unlock fault"));
    let mut trace = vec!["fault-retained"];
    second.await.unwrap();
    trace.push("healthy-terminal");
    writer_controller_law_table(control, |table| {
        assert!(table.acquire_with(&document, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).is_err());
    });
    failure.into_parts().1.retry().await.unwrap();
    trace.push("fault-retry-terminal");
    assert_eq!(serde_json::json!(trace), fixture["controllerFaults"]["coalesced"]);
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_outer_panic_faults_waiters_once_and_stops() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let expected = &fixture["controllerFaults"]["outerPanic"];
    let turns = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let control =
        register_db_io_backend(DbIoBackendKind::Memory, Box::new(WriterControllerLawExecutor { table: std::sync::Mutex::new(None), mode: DbIoExecutorMode::BlockingLane, turns: turns.clone(), panic_release: true }), db_io_test_pool()).unwrap();
    let probes: Vec<_> = (0..expected["requestedOwners"].as_u64().unwrap()).map(|_| Arc::new(WriterRegistryProbeWake { wakes: std::sync::atomic::AtomicUsize::new(0), registry_available: std::sync::atomic::AtomicBool::new(false) })).collect();
    let mut releases = writer_controller_law_table(control, |table| {
        probes
            .iter()
            .enumerate()
            .map(|(index, probe)| {
                let document = DbIoText::try_from_str(&format!("outer-panic-{index}")).unwrap();
                let permit = table.acquire_with(&document, || Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })).unwrap();
                let mut release = permit.release();
                assert!(Pin::new(&mut release).poll(&mut std::task::Context::from_waker(&std::task::Waker::from(probe.clone()))).is_pending());
                release
            })
            .collect::<Vec<_>>()
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while probes.iter().any(|probe| probe.wakes.load(std::sync::atomic::Ordering::Acquire) == 0) {
        assert!(std::time::Instant::now() < deadline);
        semio_framework_async::yield_once().await;
    }
    let mut retained = Vec::new();
    for (release, probe) in releases.iter_mut().zip(&probes) {
        assert_eq!(probe.wakes.load(std::sync::atomic::Ordering::Acquire) as u64, expected["wakesPerOwner"].as_u64().unwrap());
        assert!(probe.registry_available.load(std::sync::atomic::Ordering::Acquire));
        let failure = match Pin::new(release).poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
            std::task::Poll::Ready(Err(failure)) => failure,
            _ => panic!("outer panic must return exact retained close owner"),
        };
        retained.push(failure.into_parts().1);
    }
    assert_eq!(turns.load(std::sync::atomic::Ordering::Acquire) as u64, expected["executorTurns"].as_u64().unwrap());
    writer_controller_law_table(control, |table| assert_eq!(!table.terminal_is_empty(), expected["guardsRetained"].as_bool().unwrap()));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    for release in retained {
        release.await.unwrap();
    }
    assert_eq!(ledger_witness(), before);
}

struct BlockingFaultTaxonomyLawExecutor {
    pub terminal: bool,
}

#[derive(serde::Deserialize)]
struct PageLifecycleFixture {
    pub pattern_modulo: usize,
    pub pattern_addend: usize,
    pub lengths: Vec<usize>,
    pub fault_conversion_retires_result_lease: bool,
    pub fault_categories: Vec<FaultCategoryFixture>,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(tag = "category", rename_all = "snake_case")]
enum FaultCategoryFixture {
    Io,
    NotFound,
    AlreadyExists,
    InvalidArgument,
    Conflict,
    Fenced { expected: u64, actual: u64 },
    StaleGeneration { expected: u64, actual: u64 },
    LimitExceeded,
    Unavailable,
    Timeout,
    Corrupt,
    Closed,
    Unauthorized,
    Unimplemented,
    Internal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaultCategoryOracle {
    Io,
    NotFound,
    AlreadyExists,
    InvalidArgument,
    Conflict,
    Fenced { expected: u64, actual: u64 },
    StaleGeneration { expected: u64, actual: u64 },
    LimitExceeded,
    Unavailable,
    Timeout,
    Corrupt,
    Closed,
    Unauthorized,
    Unimplemented,
    Internal,
    Other,
}

fn fault_fixture_error(category: FaultCategoryFixture) -> DbError {
    match category {
        FaultCategoryFixture::Io => DbError::Io("fault taxonomy fixture io".to_string()),
        FaultCategoryFixture::NotFound => DbError::NotFound("fault taxonomy fixture missing".to_string()),
        FaultCategoryFixture::AlreadyExists => DbError::AlreadyExists("fault taxonomy fixture exists".to_string()),
        FaultCategoryFixture::InvalidArgument => DbError::InvalidArgument("fault taxonomy fixture invalid".to_string()),
        FaultCategoryFixture::Conflict => DbError::Conflict("fault taxonomy fixture conflict".to_string()),
        FaultCategoryFixture::Fenced { expected, actual } => DbError::Fenced { expected, actual },
        FaultCategoryFixture::StaleGeneration { expected, actual } => DbError::StaleGeneration { expected: GenerationId(expected), actual: GenerationId(actual) },
        FaultCategoryFixture::LimitExceeded => DbError::LimitExceeded("fault taxonomy fixture limit"),
        FaultCategoryFixture::Unavailable => DbError::Unavailable("fault taxonomy fixture unavailable".to_string()),
        FaultCategoryFixture::Timeout => DbError::Timeout("fault taxonomy fixture timeout".to_string()),
        FaultCategoryFixture::Corrupt => DbError::Corrupt("fault taxonomy fixture corrupt".to_string()),
        FaultCategoryFixture::Closed => DbError::Closed,
        FaultCategoryFixture::Unauthorized => DbError::Unauthorized("fault taxonomy fixture unauthorized".to_string()),
        FaultCategoryFixture::Unimplemented => DbError::Unimplemented("fault taxonomy fixture unimplemented"),
        FaultCategoryFixture::Internal => DbError::Internal("fault taxonomy fixture internal".to_string()),
    }
}

fn fault_category_oracle(error: &DbError) -> FaultCategoryOracle {
    match error {
        DbError::Io(_) => FaultCategoryOracle::Io,
        DbError::NotFound(_) => FaultCategoryOracle::NotFound,
        DbError::AlreadyExists(_) => FaultCategoryOracle::AlreadyExists,
        DbError::InvalidArgument(_) => FaultCategoryOracle::InvalidArgument,
        DbError::Conflict(_) => FaultCategoryOracle::Conflict,
        DbError::Fenced { expected, actual } => FaultCategoryOracle::Fenced { expected: *expected, actual: *actual },
        DbError::StaleGeneration { expected, actual } => FaultCategoryOracle::StaleGeneration { expected: expected.0, actual: actual.0 },
        DbError::LimitExceeded(_) => FaultCategoryOracle::LimitExceeded,
        DbError::Unavailable(_) => FaultCategoryOracle::Unavailable,
        DbError::Timeout(_) => FaultCategoryOracle::Timeout,
        DbError::Corrupt(_) => FaultCategoryOracle::Corrupt,
        DbError::Closed => FaultCategoryOracle::Closed,
        DbError::Unauthorized(_) => FaultCategoryOracle::Unauthorized,
        DbError::Unimplemented(_) => FaultCategoryOracle::Unimplemented,
        DbError::Internal(_) => FaultCategoryOracle::Internal,
        _ => FaultCategoryOracle::Other,
    }
}

fn fault_fixture_error_for_task(task: &DbIoTask) -> DbError {
    let DbIoTask::PayloadGet { hash, .. } = task else { return DbError::Internal("fault taxonomy fixture received the wrong task".to_string()) };
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
    fixture.fault_categories.get(usize::from(hash.0[0])).copied().map(fault_fixture_error).unwrap_or_else(|| DbError::Internal("fault taxonomy fixture discriminator is out of range".to_string()))
}

impl DbIoTaskExecutor for BlockingFaultTaxonomyLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let DbIoTask::PayloadGet { output, .. } = task else { return Err(DbError::Internal("blocking fault taxonomy fixture received the wrong task".to_string())) };
        if output.is_empty() {
            if output.write_fragment(&[0xa1])? != 1 {
                return Err(DbError::Internal("blocking fault taxonomy fixture lost its retained page".to_string()));
            }
            return Ok((DbIoExecutionStep::Yield, None));
        }
        Err(fault_fixture_error_for_task(task))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("blocking fault taxonomy fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

fn ledger_witness() -> (DbIoCredit, usize) {
    db_io_ledger_census(DB_IO_LEDGER_OWNER.with(std::cell::Cell::get))
}

fn pages(bytes: &[u8]) -> DbIoPages {
    let mut writer = DbIoPageWriter::try_reserve(bytes.len().div_ceil(DB_IO_PAGE_BYTES)).expect("fixture pages admitted");
    for fragment in bytes.chunks(DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

fn drain_pages(mut pages: DbIoPages) {
    while pages.close_step().unwrap().is_some() {}
    while db_io_page_maintenance_step().unwrap().is_some() {}
}

async fn drain_control_tasks(control: DbIoBackendControl) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while DB_IO_TASK_SLOTS.iter().any(|task| task.lock().unwrap_or_else(std::sync::PoisonError::into_inner).backend == Some(control)) {
        assert!(std::time::Instant::now() < deadline, "closed tasks did not retire through the backend hook or an owner maintenance opportunity");
        db_io_maintenance_step().unwrap();
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn db_io_blocking_fault_preserves_exact_category_scalars_and_retires() {
    if !db_io_driver::process_isolated_law("db_io_retained_fixtures::db_io_blocking_fault_preserves_exact_category_scalars_and_retires") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingFaultTaxonomyLawExecutor { terminal: false }), pool.clone()).unwrap();

    for (index, category) in fixture.fault_categories.into_iter().enumerate() {
        let expected_error = fault_fixture_error(category);
        let expected_category = fault_category_oracle(&expected_error);
        let output = DbIoPageWriter::try_reserve(1).unwrap();
        let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([u8::try_from(index).unwrap(); 32]), output }).unwrap_or_else(|(error, _)| panic!("blocking fault taxonomy admission failed: {error}"));
        let handle = operation.handle;
        let fault = match operation.await {
            Err(fault) => fault,
            Ok(_) => panic!("blocking fault taxonomy fixture returned a result"),
        };
        assert_eq!(fault.kind, DbIoFaultKind::Backend);
        {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if db_io_slot_matches(&owner, handle) {
                if let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() {
                    let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult), "blocking fault taxonomy output left TerminalResult before its task retired");
                }
            }
        }
        let actual_error = fault.into_db_error();
        assert_eq!(actual_error, expected_error);
        assert_eq!(fault_category_oracle(&actual_error), expected_category);
        let ledger = lock(db_io_operation_ledger());
        assert_eq!(db_io_operation_slot(&ledger, handle.operation).map_or(0, |slot| ledger.slots[slot].result_leases) == 0, fixture.fault_conversion_retires_result_lease, "a fault holds no result lease; a task maintenance already retired holds no ledger slot at all");
        drop(ledger);
        drain_control_tasks(control).await;
    }

    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return() {
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
    let inline = size_of::<MemoryDbIoExecutor>() as u64;
    assert!(inline <= fixture["maximumInlineBytes"].as_u64().unwrap(), "fixed backend tables must not occupy the caller stack");
    let before = ledger_witness();
    let static_expected = DbIoCredit { pages: 0, bytes: MemoryDbIoExecutor::backing_bytes(), items: 1, controls: 1 };
    let controller_credit = db_io_registry::controller_credit();
    assert_eq!(controller_credit.items, fixture["controllerCredit"]["items"].as_u64().unwrap() as usize);
    assert_eq!(controller_credit.controls, fixture["controllerCredit"]["controls"].as_u64().unwrap() as usize);
    assert_eq!(fixture["controllerCredit"]["bytesFormula"], "wake-plus-two-usize");
    let expected = static_expected.checked_add(controller_credit).unwrap();
    for case in fixture["admission"].as_array().unwrap() {
        let remaining = match case["remaining"].as_str().unwrap() {
            "exact" => expected.bytes,
            "one-short" => expected.bytes - 1,
            "zero" => 0,
            other => panic!("unknown memory admission vector {other}"),
        };
        let occupied = DbIoCredit { bytes: DB_IO_PROCESS_BYTES - remaining, ..DbIoCredit::default() };
        assert_eq!(db_io_backend_owner_admits(occupied, expected), case["accepted"].as_bool().unwrap());
    }
    let storage = MemoryStorage::new(db_io_test_pool()).await.unwrap();
    let (slot, generation) = db_io_backend_parts(storage.control);
    let owner_operation = {
        let mut registry = lock(db_io_backend_registry());
        let owner = &mut registry.slots[slot as usize];
        assert_eq!(owner.generation, generation);
        assert_eq!(owner.owner_credit, expected);
        let executor = owner.executor.as_mut().unwrap().as_any_mut().downcast_mut::<MemoryDbIoExecutor>().unwrap();
        fn table<T>(name: &'static str, cells: &std::sync::Mutex<Box<[Option<T>]>>) -> (&'static str, usize, usize) {
            let cells = lock(cells);
            (name, cells.len(), size_of_val(cells.as_ref()))
        }
        let tables = [
            table("wal", &executor.wal),
            table("snapshots", &executor.snapshots),
            table("payloads", &executor.payloads),
            table("index-runs", &executor.index_runs),
            table("leases", &executor.leases),
            table("operations", &executor.operations),
            table("retired-pages", &executor.retired_pages),
            table("retired-wal", &executor.retired_wal),
        ];
        assert_eq!(fixture["writerTable"]["slots"].as_u64().unwrap() as usize, db_io_core::WAL_WRITER_CAPACITY);
        assert_eq!(fixture["writerTable"]["separateBox"], true);
        assert_eq!(fixture["tables"].as_array().unwrap().len(), tables.len());
        for ((name, slots, _), row) in tables.iter().zip(fixture["tables"].as_array().unwrap()) {
            assert_eq!(row["name"].as_str().unwrap(), *name);
            assert_eq!(row["slots"].as_u64().unwrap(), *slots as u64);
        }
        assert_eq!(inline + size_of::<db_io_driver::WalWriterTable<()>>() as u64 + tables.iter().map(|(_, _, bytes)| *bytes as u64).sum::<u64>(), static_expected.bytes);
        assert_eq!(executor.owner_backing_bytes(), static_expected.bytes);
        owner.owner_operation
    };
    {
        let ledger = lock(db_io_operation_ledger());
        assert_eq!(ledger.slots[db_io_operation_slot(&ledger, owner_operation).unwrap()].live, expected);
    }
    let probe_credit = DbIoCredit { items: 1, ..DbIoCredit::default() };
    let probe = db_io_operation_reserve(probe_credit).unwrap();
    let _registered_pool = db_io_backend_admit_operation(storage.control, probe).unwrap();
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    assert_eq!(db_io_backend_close_step(storage.control, &mut context).unwrap(), fixture["closeWhileAdmitted"].as_bool().unwrap());
    assert!(lock(db_io_backend_registry()).slots[slot as usize].executor.is_some());
    db_io_backend_return_operation(storage.control, probe).unwrap();
    db_io_operation_return(probe, probe_credit).unwrap();
    let document = ArtifactId("memory-retirement-frontier".into());
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.create_segment(&writer, 0).await.unwrap();
    let segment = &fixture["walSegment"];
    assert_eq!(segment["chunkSlots"].as_u64().unwrap() as usize, DB_IO_OPERATION_ITEMS);
    assert_eq!((segment["chargedOn"].as_str().unwrap(), segment["returnedOn"].as_str().unwrap()), ("create", "retirement"));
    let segment_credit = DbIoCredit { bytes: (DB_IO_OPERATION_ITEMS * size_of::<Option<DbIoPages>>()) as u64, items: segment["creditItems"].as_u64().unwrap() as usize, ..DbIoCredit::default() };
    assert_eq!(memory_wal_segment_credit(), segment_credit);
    {
        let ledger = lock(db_io_operation_ledger());
        assert_eq!(ledger.slots[db_io_operation_slot(&ledger, owner_operation).unwrap()].live, expected.checked_add(segment_credit).unwrap(), "a live WAL segment charges its chunk table to its backend");
    }
    let mut retained = storage.list_segments(&document).await.unwrap();
    assert_eq!(retained.as_slice(), &[0]);
    for index in 0..fixture["sequentialTasks"].as_u64().unwrap() {
        assert_eq!(storage.segment_len(&document, 0).await.unwrap_or_else(|error| panic!("sequential task {index} lost capacity with one retained result: {error}")), 0);
        assert_eq!(retained.as_slice(), &[0], "task retirement must preserve the caller-owned list");
        let handback = retained.result_handback.unwrap();
        let task_slot = DB_IO_TASK_SLOTS[handback.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(!db_io_slot_matches(&task_slot, handback.handle), fixture["taskSlotReleasedWhileResultRetained"].as_bool().unwrap());
        drop(task_slot);
        let ledger = lock(db_io_operation_ledger());
        let operation = &ledger.slots[db_io_operation_slot(&ledger, handback.handle.operation).unwrap()];
        assert_eq!(operation.result_leases, 1);
        assert!(operation.live.bytes >= DB_IO_LIST_BACKING_BYTES + db_io_result_lease_credit().bytes);
    }
    while retained.close_step() {}
    assert!(retained.values.is_none());
    writer.release().await.unwrap();
    storage.close().await.unwrap();
    close_db_io_backend(storage.control).await.unwrap();
    assert!(db_io_operation_slot(&lock(db_io_operation_ledger()), owner_operation).is_none());
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit() {
    if !db_io_driver::process_isolated_law("db_io_retained_fixtures::db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit") {
        return;
    }
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
    const RETAINED_PAGE_RESULTS: usize = 44;
    assert_eq!(fixture["retainedPageResults"].as_u64().unwrap() as usize, RETAINED_PAGE_RESULTS);
    assert_eq!(fixture["sameSlotReuseBeforeOldResultClose"], true);
    let before = ledger_witness();
    let storage = MemoryStorage::new(db_io_test_pool()).await.unwrap();
    let document = ArtifactId("memory-retained-pages-aba".into());
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.create_segment(&writer, 0).await.unwrap();
    assert_eq!(storage.append(&writer, 0, pages(&[0xa7])).await.unwrap(), 1);
    let steady = ledger_witness();
    let mut retained: [Option<DbIoPages>; RETAINED_PAGE_RESULTS] = std::array::from_fn(|_| None);
    for owner in &mut retained {
        let pages = storage.read(&document, 0, ByteRange { offset: 0, len: 1 }).await.unwrap();
        assert_eq!(pages, [0xa7]);
        let handback = pages.result_handback.expect("retained page result has exact task handback");
        let task = DB_IO_TASK_SLOTS[handback.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(!db_io_slot_matches(&task, handback.handle), "page result must outlive its retired task slot");
        drop(task);
        *owner = Some(pages);
    }
    let held = ledger_witness();
    assert_eq!(held.0.pages, steady.0.pages + RETAINED_PAGE_RESULTS);
    assert!(held.0.bytes > steady.0.bytes);
    assert!(held.0.items >= steady.0.items + RETAINED_PAGE_RESULTS);
    assert!(held.0.controls >= steady.0.controls + RETAINED_PAGE_RESULTS);
    let old_handle = retained[0].as_ref().unwrap().result_handback.unwrap().handle;
    let mut reused = None;
    for _ in 0..DB_IO_OPERATION_ITEMS * 2 {
        let operation = submit_db_io_task(DbIoTask::WalLength { backend: storage.control, document: memory_document(&document).unwrap(), index: 0 }).unwrap_or_else(|(error, _)| panic!("same-slot probe rejected: {error}"));
        if operation.handle.slot == old_handle.slot {
            reused = Some(operation);
            break;
        }
        assert!(matches!(operation.finish().await.unwrap(), DbIoResult::Length(1)));
    }
    let reused = reused.expect("task allocator did not reuse the retired page-result slot");
    assert_ne!(reused.handle.generation, old_handle.generation);
    assert_ne!(reused.handle.operation, old_handle.operation);
    let reused_before = {
        let task = DB_IO_TASK_SLOTS[reused.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(db_io_slot_matches(&task, reused.handle));
        (task.generation, task.operation)
    };
    drain_pages(retained[0].take().unwrap());
    let reused_after = {
        let task = DB_IO_TASK_SLOTS[reused.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(db_io_slot_matches(&task, reused.handle));
        (task.generation, task.operation)
    };
    assert_eq!(reused_after, reused_before, "stale page-result handback changed a reused task slot");
    assert!(matches!(reused.finish().await.unwrap(), DbIoResult::Length(1)));
    for owner in retained.into_iter().flatten() {
        drain_pages(owner);
    }
    assert_eq!(ledger_witness(), steady);
    writer.release().await.unwrap();
    storage.close().await.unwrap();
    close_db_io_backend(storage.control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_memory_backend_uses_actual_typed_submit_take_result_and_terminal_close() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    let storage = MemoryStorage::new(pool.clone()).await.unwrap();
    let second = MemoryStorage::new(pool.clone()).await.unwrap();
    assert!(Arc::ptr_eq(&storage.pool, &pool));
    assert!(Arc::ptr_eq(&second.pool, &pool));
    let document = ArtifactId("typed-memory-fixture".to_string());
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.create_segment(&writer, 1).await.unwrap();
    storage.append(&writer, 1, pages(&[0x91; DB_IO_PAGE_BYTES + 1])).await.unwrap();
    let mut result = storage.read(&document, 1, ByteRange { offset: 0, len: (DB_IO_PAGE_BYTES + 1) as u64 }).await.unwrap();
    assert_eq!(result.page_count(), 2);
    while result.close_step().unwrap().is_some() {}
    writer.release().await.unwrap();
    storage.close().await.unwrap();
    second.close().await.unwrap();
    assert_eq!(ledger_witness(), before);
}

fn leased_guard() -> Result<WriterControllerLawGuard, DbError> {
    Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })
}

fn leased_documents(prefix: &str) -> Vec<DbIoText> {
    (0..db_io_core::WAL_WRITER_CAPACITY).map(|ordinal| DbIoText::try_from_str(&format!("{prefix}-{ordinal}")).unwrap()).collect()
}

async fn release_leased(permits: impl IntoIterator<Item = WalWriterPermit>) {
    for permit in permits {
        permit.release().await.unwrap();
    }
}

/// 🅿️ A backend lends its writers to whichever document writes (declared `lease`): with every slot taken an acquisition
/// reclaims the least recently parked writer — never a held or resumed one, and a parked writer still owns its document
/// against a second acquisition of it. The reclaimed holder can no longer resume or validate its key, and its release is
/// already terminal.
#[semio_framework_async_macros::async_test]
async fn wal_writer_parked_writers_are_reclaimed_least_recently_parked_first_and_held_ones_never() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let lease = &fixture["lease"];
    let slots = |key: &str| lease[key].as_array().unwrap().iter().map(|slot| slot.as_u64().unwrap() as usize).collect::<Vec<_>>();
    let (control, _) = register_writer_controller_law(DbIoExecutorMode::BlockingLane);
    let documents = leased_documents("leased");
    let mut permits: Vec<Option<WalWriterPermit>> = writer_controller_law_table(control, |table| documents.iter().map(|document| Some(table.acquire_with(document, leased_guard).unwrap())).collect());
    let key = |permits: &[Option<WalWriterPermit>], slot: usize| permits[slot].as_ref().unwrap().key();
    for slot in slots("parkOrder") {
        permits[slot].as_ref().unwrap().park();
    }
    let resumed = lease["resumedBeforeReclaim"].as_u64().unwrap() as usize;
    assert!(permits[resumed].as_ref().unwrap().resume(), "a parked writer nobody reclaimed is still its holder's");
    let parked = slots("reclaimOrder")[0];
    let mut factory_calls = 0;
    let same_document = writer_controller_law_table(control, |table| {
        table
            .acquire_with(&documents[parked], || {
                factory_calls += 1;
                leased_guard()
            })
            .map(drop)
    });
    assert!(matches!(same_document, Err(DbError::Conflict(_))), "a parked writer still owns its document: {same_document:?}");
    assert_eq!((lease["sameDocumentWhileParked"].as_str().unwrap(), factory_calls), ("conflict", 0));
    let mut newcomers = Vec::new();
    for (ordinal, reclaimed) in slots("reclaimOrder").into_iter().enumerate() {
        let document = DbIoText::try_from_str(&format!("newcomer-{ordinal}")).unwrap();
        newcomers.push(writer_controller_law_table(control, |table| table.acquire_with(&document, leased_guard)).unwrap());
        assert!(db_io_core::parked_since(key(&permits, reclaimed)).is_none(), "newcomer {ordinal} reclaimed slot {reclaimed}, the least recently parked writer");
        for later in slots("reclaimOrder").into_iter().skip(ordinal + 1) {
            assert!(db_io_core::parked_since(key(&permits, later)).is_some(), "slot {later} stays parked until it is the least recent");
        }
    }
    let spare = DbIoText::try_from_str("newcomer-spare").unwrap();
    let full = writer_controller_law_table(control, |table| table.acquire_with(&spare, leased_guard).map(drop));
    assert!(matches!(full, Err(DbError::LimitExceeded("WAL writer capacity"))), "every writer held: {full:?}");
    assert_eq!(lease["everyWriterHeld"], "capacity");
    for reclaimed in slots("reclaimOrder") {
        let permit = permits[reclaimed].take().unwrap();
        assert_eq!(permit.resume(), lease["reclaimedResume"].as_bool().unwrap());
        let validated = writer_controller_law_table(control, |table| table.validate(permit.key(), control, &documents[reclaimed]).map(drop));
        assert!(matches!(validated, Err(DbError::Fenced { .. })) && lease["reclaimedKey"] == "fenced", "a reclaimed key never validates: {validated:?}");
        let mut release = permit.release();
        assert!(matches!(Pin::new(&mut release).poll(&mut std::task::Context::from_waker(std::task::Waker::noop())), std::task::Poll::Ready(Ok(()))) && lease["reclaimedRelease"] == "terminal");
    }
    release_leased(permits.into_iter().flatten().chain(newcomers)).await;
    writer_controller_law_table(control, |table| assert!(table.terminal_is_empty()));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

/// 🚦️ With every writer writing, acquisitions wait in the backend's admission line first come, first served: a parked
/// writer is lent to the head only, a newcomer joins behind the line instead of overtaking it, a waiter that gives up hands
/// nothing on unless it was the head, and a wait that outlives its bound is refused as transient on the backend's own
/// timer — never a stall.
#[semio_framework_async_macros::async_test]
async fn wal_writer_admission_serves_waiters_in_arrival_order_and_refuses_an_elapsed_wait() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let (control, _) = register_writer_controller_law(DbIoExecutorMode::BlockingLane);
    let documents = leased_documents("admitted");
    let permits: Vec<WalWriterPermit> = writer_controller_law_table(control, |table| documents.iter().map(|document| table.acquire_with(document, leased_guard).unwrap()).collect());
    let acquire = |name: &'static str, wait: std::time::Duration| {
        db_io_driver::admitted_acquire(control, wait, move || std::future::ready(writer_controller_law_table(control, |table| table.acquire_with(&DbIoText::try_from_str(name).unwrap(), leased_guard))))
    };
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    let long = std::time::Duration::from_secs(60);
    let mut first = Box::pin(acquire("first", long));
    let mut second = Box::pin(acquire("second", long));
    assert!(first.as_mut().poll(&mut context).is_pending(), "every writer writes: the first acquisition waits");
    assert!(second.as_mut().poll(&mut context).is_pending(), "and the second behind it");
    permits[0].park();
    let mut third = Box::pin(acquire("third", long));
    assert!(third.as_mut().poll(&mut context).is_pending(), "a newcomer joins behind the line although a writer is parked");
    assert!(second.as_mut().poll(&mut context).is_pending(), "only the head is lent the parked writer");
    let std::task::Poll::Ready(Ok(first_permit)) = first.as_mut().poll(&mut context) else { panic!("the head takes the parked writer") };
    assert_eq!(first_permit.document().as_str(), "first");
    drop(second);
    assert!(third.as_mut().poll(&mut context).is_pending(), "a cancelled waiter frees its place but no writer");
    permits[1].park();
    let std::task::Poll::Ready(Ok(third_permit)) = third.as_mut().poll(&mut context) else { panic!("the next in line takes the next parked writer") };
    assert_eq!(db_io_driver::admission_census(control), db_io_core::WalWriterAdmissionCensus { waited: 3, refused: 0, waiting: 0 });
    let started = std::time::Instant::now();
    let late = acquire("late", std::time::Duration::from_millis(50)).await;
    assert!(matches!(late, Err(DbError::Unavailable(ref detail)) if detail == db_io_core::WAL_WRITER_ADMISSION_REFUSAL), "an elapsed wait is the typed transient refusal: {:?}", late.as_ref().map(WalWriterPermit::document));
    assert!(started.elapsed() < std::time::Duration::from_secs(10), "the backend's timer ends the wait");
    assert_eq!(db_io_driver::admission_census(control), db_io_core::WalWriterAdmissionCensus { waited: 4, refused: 1, waiting: 0 });
    release_leased(permits.into_iter().chain([first_permit, third_permit])).await;
    writer_controller_law_table(control, |table| assert!(table.terminal_is_empty()));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}
