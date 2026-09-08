
use super::*;

static FIXTURE_LOCK: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

struct FixtureSerial;

impl Drop for FixtureSerial {
    fn drop(&mut self) {
        FIXTURE_LOCK.store(false, std::sync::atomic::Ordering::Release);
    }
}

fn fixture_serial() -> FixtureSerial {
    while FIXTURE_LOCK.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
        std::thread::yield_now();
    }
    FixtureSerial
}

struct AsyncNativeLawExecutor {
    terminal: bool,
}

struct WriterControllerLawGuard {
    failures: Arc<std::sync::atomic::AtomicUsize>,
    closed: bool,
}

impl writer::WalWriterGuard for WriterControllerLawGuard {
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
    table: std::sync::Mutex<Option<Box<writer::WalWriterTable<WriterControllerLawGuard>>>>,
    mode: DbIoExecutorMode,
    turns: Arc<std::sync::atomic::AtomicUsize>,
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
        (size_of::<Self>() + size_of::<writer::WalWriterTable<WriterControllerLawGuard>>()) as u64
    }
    fn bind_writer_control(&mut self, control: DbIoBackendControl) -> Result<(), DbError> {
        let mut table = lock(&self.table);
        assert!(table.is_none());
        *table = Some(Box::new(writer::WalWriterTable::for_backend(control)));
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
        self.registry_available.store(db_io_backend_registry().try_lock().is_ok(), std::sync::atomic::Ordering::Release);
        self.wakes.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

struct WriterDeferredRefusalWake {
    wakes: std::sync::atomic::AtomicUsize,
    external: Arc<std::sync::Mutex<()>>,
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

fn writer_controller_law_table<T: 'static>(control: DbIoBackendControl, action: impl FnOnce(&mut writer::WalWriterTable<WriterControllerLawGuard>) -> T) -> T {
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
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
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
        assert!(
            table
                .acquire_with(&document, || {
                    factory_calls += 1;
                    Ok(WriterControllerLawGuard { failures: Arc::new(std::sync::atomic::AtomicUsize::new(0)), closed: false })
                })
                .is_err()
        );
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
    eprintln!("[DEBUG] mounted writer controller fenced before its first callback, retained the pin without new tasks, woke outside the registry and returned exact backend credit");
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer() {
    let _serial = fixture_serial();
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
    eprintln!("[DEBUG] exact failed writer kept its guard and retry owner while another writer remained valid; explicit retry retired both without cross-writer faults");
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch() {
    let _serial = fixture_serial();
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
    writer::release::suspend_controller_for_refusal(control).unwrap();
    let external_guard = external.lock().unwrap();
    let mut release_a = permit_a.release();
    let failure_a = match Pin::new(&mut release_a).poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(Err(failure)) => failure,
        _ => panic!("A must immediately retain the missing-controller refusal that defers B's waiter"),
    };
    assert!(writer::release::deferred_wake_pending_for_test(key_b));
    assert!(!writer::release::deferred_wake_pending_for_test(key_a));
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
    assert!(!writer::release::deferred_wake_pending_for_test(key_b));
    let failure_b = match Pin::new(&mut release_b).poll(&mut std::task::Context::from_waker(&waker)) {
        std::task::Poll::Ready(Err(failure)) => failure,
        _ => panic!("B must expose its retained refusal only after the exact deferred slot drains"),
    };
    writer_controller_law_table(control, |table| {
        assert!(matches!(table.validate(key_a, control, &document_a), Err(DbError::Closed)));
        assert!(matches!(table.validate(key_b, control, &document_b), Err(DbError::Closed)));
    });
    writer::release::restore_controller_after_refusal(control).unwrap();
    failure_b.into_parts().1.retry().await.unwrap();
    failure_a.into_parts().1.retry().await.unwrap();
    writer_controller_law_table(control, |table| assert!(table.terminal_is_empty()));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    assert_eq!(ledger_witness(), before);
    eprintln!("[DEBUG] stale controller refusal moved B's exact waiter to the fixed pool slot, blocked Ready/retry until drain, and retained both guards through explicit retry");
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_rerequests_after_async_executor_handback() {
    let _serial = fixture_serial();
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
    eprintln!("[DEBUG] leased async writer remained retained through both lease and admission, then its existing hook retired it after exact handback without a new DB task");
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_coalesced_fault_does_not_strand_healthy_release() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
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
    eprintln!("[DEBUG] coalesced healthy writer completed while its peer retained an unlock fault; exact retry later returned all backend credit");
}

#[semio_framework_async_macros::async_test]
async fn wal_writer_mounted_controller_outer_panic_faults_waiters_once_and_stops() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
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
    eprintln!("[DEBUG] unexpected writer executor panic woke every retained owner outside registry locks, stopped after one executor turn, and backend close retired both guards");
}

struct BlockingFaultTaxonomyLawExecutor {
    terminal: bool,
}

struct AsyncFaultTaxonomyLawExecutor {
    terminal: bool,
}

struct BlockingFaultLawExecutor {
    panic: bool,
    terminal: bool,
}

struct BlockingCompleteLawExecutor {
    terminal: bool,
}

struct DeferredBackendCloseLawExecutor {
    allow_close: Arc<std::sync::atomic::AtomicBool>,
    terminal: bool,
}

struct DropRegisteredBackend(DbIoBackendControl);

impl Drop for DropRegisteredBackend {
    fn drop(&mut self) {
        let _ = retire_db_io_backend(self.0);
    }
}

struct BlockingOutputLifecycleLawExecutor {
    terminal: bool,
    success_steps: Arc<std::sync::atomic::AtomicUsize>,
    cancel_steps: Arc<std::sync::atomic::AtomicUsize>,
    abandon_steps: Arc<std::sync::atomic::AtomicUsize>,
}

#[derive(serde::Deserialize)]
struct PageLifecycleFixture {
    pattern_modulo: usize,
    pattern_addend: usize,
    lengths: Vec<usize>,
    fault_conversion_retires_result_lease: bool,
    fault_categories: Vec<FaultCategoryFixture>,
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

struct AsyncLaneProbeExecutor {
    terminal: bool,
    caller: std::thread::ThreadId,
    polled_on_worker: Arc<std::sync::atomic::AtomicBool>,
    poll_thread: Arc<std::sync::Mutex<Option<std::thread::ThreadId>>>,
    worker_role: Arc<std::sync::atomic::AtomicBool>,
    close_thread: Arc<std::sync::Mutex<Option<std::thread::ThreadId>>>,
    close_worker_role: Arc<std::sync::atomic::AtomicBool>,
}

impl DbIoTaskExecutor for AsyncNativeLawExecutor {
    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::AsyncNative
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Err(DbError::Internal("async-native law executor entered blocking step".to_string()))
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
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
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
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
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

impl DbIoTaskExecutor for AsyncFaultTaxonomyLawExecutor {
    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::AsyncNative
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Err(DbError::Internal("async fault taxonomy fixture entered the blocking driver".to_string()))
    }
    fn drive_async(self: Box<Self>, _operation: u64, mut task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let terminal = match &mut task {
                DbIoTask::PayloadGet { output, .. } => match output.write_fragment(&[0xa2]) {
                    Ok(1) => Err(fault_fixture_error_for_task(&task)),
                    Ok(_) => Err(DbError::Internal("async fault taxonomy fixture lost its retained page".to_string())),
                    Err(error) => Err(error),
                },
                _ => Err(DbError::Internal("async fault taxonomy fixture received the wrong task".to_string())),
            };
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, terminal)
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

impl DbIoTaskExecutor for AsyncLaneProbeExecutor {
    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::AsyncNative
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Err(DbError::Internal("async lane probe entered blocking step".to_string()))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let thread = std::thread::current().id();
            self.polled_on_worker.store(thread != self.caller, std::sync::atomic::Ordering::Release);
            self.worker_role.store(semio_framework_trace::is_worker_thread(), std::sync::atomic::Ordering::Release);
            *self.poll_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(thread);
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Ok(DbIoResult::Unit))
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

impl DbIoTaskExecutor for BlockingFaultLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        if self.panic {
            panic!("hostile DB I/O fixture panic");
        }
        Err(DbError::Internal("hostile DB I/O fixture backend fault".to_string()))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("blocking fault fixture has no async driver".to_string())))
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

impl DbIoTaskExecutor for BlockingCompleteLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("blocking completion fixture has no async driver".to_string())))
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

impl DbIoTaskExecutor for DeferredBackendCloseLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("deferred backend close fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        if !self.allow_close.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(false);
        }
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl DbIoTaskExecutor for BlockingOutputLifecycleLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        let DbIoTask::PayloadGet { hash, output, .. } = task else {
            return Err(DbError::Internal("output lifecycle fixture received the wrong task".to_string()));
        };
        let counter = match hash.0[0] {
            0x71 => &self.success_steps,
            0x72 => &self.cancel_steps,
            0x73 => &self.abandon_steps,
            _ => return Err(DbError::Internal("output lifecycle fixture received an unknown scenario".to_string())),
        };
        counter.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        {
            let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::Executing) {
                return Err(DbError::Internal("output lifecycle fixture entered a turn outside Executing".to_string()));
            }
        }
        let total = DB_IO_PAGE_BYTES + 1;
        if output.len() < total {
            let start = output.len();
            let end = start.saturating_add(DB_IO_PAGE_BYTES).min(total);
            let fragment: Vec<u8> = (start..end).map(|index| (index % 251) as u8).collect();
            if output.write_fragment(&fragment)? != fragment.len() {
                return Err(DbError::Internal("output lifecycle fixture lost an admitted fragment".to_string()));
            }
            return Ok((DbIoExecutionStep::Yield, None));
        }
        if hash.0[0] != 0x71 {
            return Ok((DbIoExecutionStep::Yield, None));
        }
        match output.seal_retained_step()? {
            Some(pages) => Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Pages(pages)))),
            None => Ok((DbIoExecutionStep::Yield, None)),
        }
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("blocking output lifecycle fixture has no async driver".to_string())))
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
    let ledger = db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    (ledger.totals, ledger.free_len)
}

fn exact_fixture_result(terminal: Result<DbIoResultLease, DbIoFault>, context: &str) -> DbIoResult {
    match terminal {
        Ok(lease) => match lease.into_result() {
            Ok(result) => result,
            Err(error) => panic!("{context} result handback failed: {error}"),
        },
        Err(mut fault) => {
            while fault.close_step() {}
            panic!("{context} task faulted")
        }
    }
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
    while DB_IO_TASK_SLOTS.iter().any(|task| task.lock().unwrap_or_else(std::sync::PoisonError::into_inner).backend == Some(control)) {
        assert!(db_io_maintenance_step().unwrap());
        std::future::poll_fn(|context| {
            context.waker().wake_by_ref();
            std::task::Poll::Ready(())
        })
        .await;
    }
}

#[test]
fn db_io_fixed_page_max_plus_one_and_zero_are_exact() {
    let _serial = fixture_serial();
    let empty = pages(&[]);
    assert!(empty.is_empty());
    drain_pages(empty);
    let max = vec![0x5a; DB_IO_PAGE_BYTES * DB_IO_OPERATION_PAGES];
    let retained = pages(&max);
    assert_eq!(retained.len(), max.len());
    assert_eq!(usize::from(retained.page_count()), DB_IO_OPERATION_PAGES);
    assert!(DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES + 1).is_err());
    drain_pages(retained);
}

#[test]
fn db_io_artifact_rejection_is_an_internal_executor_boundary_violation() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fault = db_io_task_fault(DbIoFaultKind::Backend, &DbError::Rejected { policy: protocol::MergePolicy::Normal, worst: protocol::Severity::Error, messages: Vec::new() });
    assert_eq!(fault.cause, DbIoFaultCause::Internal);
    assert_eq!(fault.into_db_error(), DbError::Internal("DB I/O backend returned an artifact-layer rejection".to_string()));
    while db_io_maintenance_step().unwrap() {}
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_blocking_fault_preserves_exact_category_scalars_and_retires() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
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
            let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("blocking fault taxonomy task lost its writer") };
            let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
        }
        let actual_error = fault.into_db_error();
        assert_eq!(actual_error, expected_error);
        assert_eq!(fault_category_oracle(&actual_error), expected_category);
        let ledger = lock(db_io_operation_ledger());
        assert_eq!(ledger.slots[db_io_operation_slot(&ledger, handle.operation).unwrap()].result_leases == 0, fixture.fault_conversion_retires_result_lease);
        drop(ledger);
        drain_control_tasks(control).await;
    }

    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_async_native_fault_preserves_exact_category_scalars_and_retires() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(AsyncFaultTaxonomyLawExecutor { terminal: false }), pool.clone()).unwrap();

    for (index, category) in fixture.fault_categories.into_iter().enumerate() {
        let expected_error = fault_fixture_error(category);
        let expected_category = fault_category_oracle(&expected_error);
        let output = DbIoPageWriter::try_reserve(1).unwrap();
        let mut operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([u8::try_from(index).unwrap(); 32]), output }).unwrap_or_else(|(error, _)| panic!("async fault taxonomy admission failed: {error}"));
        let handle = operation.handle;
        operation.start_async_native_on_lane_io().await.unwrap();
        let fault = match operation.await {
            Err(fault) => fault,
            Ok(_) => panic!("async fault taxonomy fixture returned a result"),
        };
        assert_eq!(fault.kind, DbIoFaultKind::Backend);
        {
            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("async fault taxonomy task lost its writer") };
            let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
        }
        let actual_error = fault.into_db_error();
        assert_eq!(actual_error, expected_error);
        assert_eq!(fault_category_oracle(&actual_error), expected_category);
        drain_control_tasks(control).await;
    }

    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_executing_output_seal_keeps_every_page_executing_until_atomic_publication() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let bytes: Vec<u8> = (0..DB_IO_PAGE_BYTES + 1).map(|index| (index % 251) as u8).collect();
    let mut writer = DbIoPageWriter::try_reserve(2).unwrap();
    assert_eq!(writer.write_fragment(&bytes).unwrap(), DB_IO_PAGE_BYTES);
    assert_eq!(writer.write_fragment(&bytes[DB_IO_PAGE_BYTES..]).unwrap(), 1);
    writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap();
    writer.transition(DbIoPagePhase::Queued, DbIoPagePhase::Executing).unwrap();

    let mut yields = 0;
    let mut published = loop {
        match writer.seal_retained_step().unwrap() {
            Some(pages) => break pages,
            None => {
                yields += 1;
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(writer.pages.iter().take(writer.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::Executing));
                drop(arena);
                writer.transition(DbIoPagePhase::Executing, DbIoPagePhase::Queued).unwrap();
                writer.transition(DbIoPagePhase::Queued, DbIoPagePhase::Executing).unwrap();
            }
        }
    };

    assert!(yields > 1);
    assert_eq!(published, bytes);
    {
        let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(published.pages.iter().take(published.retained as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
    }
    while published.close_step().unwrap().is_some() {}
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_page_identity_rejects_generation_operation_and_phase_mismatches_exactly() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    let page = writer.pages[0].as_ref().unwrap();
    let (slot_index, generation, operation) = (page.slot as usize, page.generation, page.operation);

    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].generation = generation + 1;
    }
    let generation_error = writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap_err();
    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].generation = generation;
    }
    assert_eq!(generation_error, DbError::StaleGeneration { expected: GenerationId(generation), actual: GenerationId(generation + 1) });

    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].operation = operation + 1;
    }
    let operation_error = writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap_err();
    {
        let mut arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        arena.slots[slot_index].operation = operation;
    }
    assert!(matches!(operation_error, DbError::Internal(message) if message.contains("page operation mismatch") && message.contains(&operation.to_string())));

    let phase_error = writer.transition(DbIoPagePhase::Executing, DbIoPagePhase::Queued).unwrap_err();
    assert!(matches!(phase_error, DbError::Internal(message) if message.contains("page phase mismatch") && message.contains("CheckedOutWriter")));
    while writer.close_step().unwrap().is_some() {}
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_output_task_yield_cancel_abandon_and_close_retire_exactly_once() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    let success_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let cancel_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let abandon_steps = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let control =
        register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingOutputLifecycleLawExecutor { terminal: false, success_steps: success_steps.clone(), cancel_steps: cancel_steps.clone(), abandon_steps: abandon_steps.clone() }), pool.clone())
            .unwrap();
    let expected: Vec<u8> = (0..DB_IO_PAGE_BYTES + 1).map(|index| (index % 251) as u8).collect();

    let output = DbIoPageWriter::try_reserve(2).unwrap();
    let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x71; 32]), output }).unwrap_or_else(|(error, _)| panic!("output lifecycle task admission failed: {error}"));
    let mut result = match exact_fixture_result(operation.await, "output lifecycle") {
        DbIoResult::Pages(pages) => pages,
        _ => panic!("output lifecycle returned the wrong typed result"),
    };
    assert!(success_steps.load(std::sync::atomic::Ordering::Acquire) > 4);
    assert_eq!(result, expected);
    while result.close_step().unwrap().is_some() {}
    drain_control_tasks(control).await;

    let output = DbIoPageWriter::try_reserve(2).unwrap();
    let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x72; 32]), output }).unwrap_or_else(|(error, _)| panic!("cancel lifecycle task admission failed: {error}"));
    for _ in 0..1_000_000 {
        if cancel_steps.load(std::sync::atomic::Ordering::Acquire) >= 3 {
            break;
        }
        std::thread::yield_now();
    }
    assert!(cancel_steps.load(std::sync::atomic::Ordering::Acquire) >= 3);
    let handle = operation.handle;
    operation.cancel().unwrap();
    let mut fault = match operation.await {
        Err(fault) => fault,
        Ok(_) => panic!("cancel lifecycle task published a result"),
    };
    {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("cancel lifecycle task lost its writer") };
        let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
    }
    assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
    while fault.close_step() {}
    drain_control_tasks(control).await;

    let output = DbIoPageWriter::try_reserve(2).unwrap();
    let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x73; 32]), output }).unwrap_or_else(|(error, _)| panic!("abandon lifecycle task admission failed: {error}"));
    for _ in 0..1_000_000 {
        if abandon_steps.load(std::sync::atomic::Ordering::Acquire) >= 3 {
            break;
        }
        std::thread::yield_now();
    }
    assert!(abandon_steps.load(std::sync::atomic::Ordering::Acquire) >= 3);
    let handle = operation.handle;
    drop(operation);
    for _ in 0..1_000_000 {
        let phase = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).phase;
        if phase == DbIoTaskPhase::Cancelled {
            break;
        }
        std::thread::yield_now();
    }
    {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(owner.phase, DbIoTaskPhase::Cancelled);
        let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() else { panic!("abandon lifecycle task lost its writer") };
        let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
    }
    drain_control_tasks(control).await;

    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[cfg(all(feature = "sqlite", not(target_arch = "wasm32")))]
#[semio_framework_async_macros::async_test]
async fn sqlite_payload_roundtrip_obeys_the_neutral_page_lifecycle_fixture() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
    let storage = db_storage_sqlite::SqliteStorage::open_in_memory(db_io_test_pool()).await.unwrap();

    for length in fixture.lengths {
        let bytes: Vec<u8> = (0..length).map(|index| (index % fixture.pattern_modulo + fixture.pattern_addend) as u8).collect();
        let expected_hash = ContentHash(*semio_framework_hash::hash(&bytes).as_bytes());
        let hash = storage.put(pages(&bytes)).await.unwrap();
        assert_eq!(hash, expected_hash);
        assert!(storage.contains(&hash).await.unwrap());
        assert_eq!(storage.len(&hash).await.unwrap(), length as u64);
        let fetched = storage.get(&hash).await.unwrap();
        assert_eq!(fetched, bytes);
        drain_pages(fetched);
        storage.delete(&hash).await.unwrap();
        assert!(!storage.contains(&hash).await.unwrap());
    }

    let missing = ContentHash([0xff; 32]);
    assert!(!storage.contains(&missing).await.unwrap());
    assert!(matches!(storage.get(&missing).await, Err(DbError::NotFound(_))));
    storage.close().await.unwrap();
    while db_io_maintenance_step().unwrap() {}
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_page_writer_seal_memory_sqlite_neo_state_wal_index_max_cancel_fault_drop_is_one_opportunity() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let waker = std::task::Waker::noop();
    let context = &mut std::task::Context::from_waker(waker);

    let mut writer = DbIoPageWriter::try_reserve(2).unwrap();
    assert_eq!(writer.write_fragment(&vec![0x41; DB_IO_PAGE_BYTES + 1]).unwrap(), DB_IO_PAGE_BYTES);
    assert_eq!(writer.write_fragment(&[0x42]).unwrap(), 1);
    let mut seal = Box::pin(writer.seal_retained());
    assert!(matches!(Future::poll(seal.as_mut(), context), std::task::Poll::Pending));
    drop(seal);
    while db_io_lost_owner_close_step().unwrap() {}
    assert_eq!(ledger_witness(), before);

    let mut writer = DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES).unwrap();
    assert_eq!(writer.write_fragment(&[0x43]).unwrap(), 1);
    let mut seal = Box::pin(writer.seal_retained());
    let mut pending = 0usize;
    let pages = loop {
        match Future::poll(seal.as_mut(), context) {
            std::task::Poll::Pending => pending += 1,
            std::task::Poll::Ready(Ok(pages)) => break pages,
            std::task::Poll::Ready(Err(rejected)) => panic!("retained seal faulted: {}", rejected.error()),
        }
    };
    assert!(pending > DB_IO_OPERATION_PAGES);
    drain_pages(pages);
    assert!(DbIoPageWriter::try_reserve(DB_IO_OPERATION_PAGES + 1).is_err());

    let writer = DbIoPageWriter::try_reserve(1).unwrap();
    writer.transition(DbIoPagePhase::CheckedOutWriter, DbIoPagePhase::Queued).unwrap();
    let mut seal = Box::pin(writer.seal_retained());
    let rejected = match Future::poll(seal.as_mut(), context) {
        std::task::Poll::Ready(Err(rejected)) => rejected,
        _ => panic!("invalid-phase seal did not retain its typed fault owner"),
    };
    let mut writer = rejected.into_writer().unwrap();
    while writer.close_step().unwrap().is_some() {}
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_one_byte_high_capacity_candidate_is_rejected_with_exact_owner() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let mut reservation = DbIoDriverReservation::try_reserve(operation, DB_IO_OPERATION_BYTES as usize).unwrap();
    let mut candidate = Vec::with_capacity(DB_IO_OPERATION_BYTES as usize + 1);
    candidate.push(0x5a);
    let identity = candidate.as_ptr();
    let capacity = candidate.capacity();
    let error = reservation.observe_capacity(candidate.capacity()).unwrap_err();
    assert!(matches!(error, DbError::LimitExceeded("DB I/O external driver allocation capacity")));
    assert_eq!(candidate.as_ptr(), identity);
    assert_eq!(candidate.capacity(), capacity);
    assert_eq!(candidate, [0x5a]);
    drop(candidate);
    reservation.close_step().unwrap();
    db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let text = DbIoText::try_from_str("post-admission-artifact").unwrap();
    let mut artifact = DbIoArtifactId::try_from_text(operation, &text).unwrap();
    assert_eq!(artifact.as_str(), "post-admission-artifact");
    assert_eq!(artifact.as_artifact().unwrap().0, "post-admission-artifact");
    assert!(artifact.close_step().unwrap());
    drop(artifact);
    while db_io_lost_owner_close_step().unwrap() {}
    db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();

    let mut lease = DbIoLeaseResult::new(DbIoText::try_from_str("resource").unwrap(), DbIoText::try_from_str("holder").unwrap(), EpochFence::INITIAL, 10);
    assert!(lease.close_step());
    assert!(!lease.terminal_is_empty());
    drop(lease);
    assert!(db_io_lost_owner_close_step().unwrap());
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_process_and_operation_ledger_return_to_exact_prior_witness() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let owner = pages(&[0x41; DB_IO_PAGE_BYTES + 1]);
    let during = ledger_witness();
    assert_eq!(during.0.pages, before.0.pages + 2);
    assert_eq!(during.0.bytes, before.0.bytes + (2 * DB_IO_PAGE_BYTES) as u64);
    assert_eq!(during.0.items, before.0.items + 3);
    assert_eq!(during.0.controls, before.0.controls + 1);
    drain_pages(owner);
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_range_moves_the_same_page_leases_without_suffix_copy() {
    let _serial = fixture_serial();
    let bytes = vec![0x33; DB_IO_PAGE_BYTES + 3];
    let owner = pages(&bytes);
    let operation = owner.operation();
    let suffix = owner.try_range(DB_IO_PAGE_BYTES).unwrap();
    assert_eq!(suffix.operation(), operation);
    assert_eq!(suffix, [0x33; 3]);
    drain_pages(suffix);
}

#[test]
fn db_io_list_capacity_plus_one_does_not_mutate_the_fixed_owner() {
    let _serial = fixture_serial();
    let mut list = DbIoU64List::new();
    for value in 0..DB_IO_LIST_ITEMS as u64 {
        list.push(value).unwrap();
    }
    assert!(list.push(DB_IO_LIST_ITEMS as u64).is_err());
    assert_eq!(list.len(), DB_IO_LIST_ITEMS);
    assert_eq!(list.as_slice().last(), Some(&((DB_IO_LIST_ITEMS - 1) as u64)));
    while list.close_step() {}
    assert!(list.terminal_is_empty());
}

#[test]
fn db_io_list_keeps_exact_capacity_off_worker_stacks_and_in_the_ledger() {
    let _serial = fixture_serial();
    let mut task = DbIoTask::WalList { backend: DbIoBackendControl::Memory { slot: 0, generation: 1 }, document: DbIoText::try_from_str("budget-witness").unwrap(), output: DbIoU64List::new() };
    assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.is_none()));
    let credit = task.aggregate_credit();
    assert_eq!(credit.bytes, DB_IO_TASK_SLOT_BYTES + DB_IO_LIST_TRANSIENT_BYTES);
    assert_eq!(credit.items, DB_IO_LIST_ITEMS * 2 + 1);
    assert!(db_io_credit_within_limits(credit, false));
    assert!(db_io_credit_within_limits(DbIoCredit { pages: DB_IO_OPERATION_PAGES, bytes: DB_IO_OPERATION_BYTES, items: DB_IO_OPERATION_ITEM_CREDIT, controls: DB_IO_OPERATION_CONTROL_CREDIT }, false,));
    assert!(!db_io_credit_within_limits(DbIoCredit { pages: DB_IO_OPERATION_PAGES, bytes: DB_IO_OPERATION_BYTES + 1, items: DB_IO_OPERATION_ITEM_CREDIT, controls: DB_IO_OPERATION_CONTROL_CREDIT }, false,));
    let before = ledger_witness();
    let operation = db_io_operation_reserve(credit).unwrap();
    let during = ledger_witness();
    assert_eq!(during.0, before.0.checked_add(credit).unwrap());
    assert_eq!(during.1 + 1, before.1);
    task.admit_list_backing().unwrap();
    assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.as_deref().map(<[u64]>::len) == Some(DB_IO_LIST_ITEMS)));
    task.release_unstarted_list_backing();
    assert!(matches!(&task, DbIoTask::WalList { output, .. } if output.values.is_none()));
    task.admit_list_backing().unwrap();
    let mut source = DbIoU64List::new();
    source.push(1).unwrap();
    assert_eq!(source.values.as_deref().map(<[u64]>::len), Some(DB_IO_LIST_ITEMS));
    assert_eq!(DB_IO_LIST_TRANSIENT_BYTES, 2 * DB_IO_LIST_ITEMS as u64 * size_of::<u64>() as u64);
    while source.close_step() {}
    db_io_operation_return(operation, credit).unwrap();
    assert_eq!(ledger_witness(), before);
    assert!(size_of::<DbIoU64List>() <= 64);
    assert!(size_of::<DbIoTask>() <= 4 * 1024);
    assert!(size_of::<DbIoResult>() <= 4 * 1024);
    assert!(size_of::<DbIoTaskSlot>() <= 8 * 1024);
    assert!(size_of::<DbIoLostOwner>() <= 4 * 1024);
}

#[test]
fn db_io_process_page_max_plus_one_preflight_is_atomic() {
    let mut state = DbIoPageArenaState::new();
    for _ in 0..DB_IO_TOTAL_PAGES / DB_IO_OPERATION_PAGES {
        db_io_preflight_page_checkout(&state, DB_IO_OPERATION_PAGES).unwrap();
        state.free_len -= DB_IO_OPERATION_PAGES;
    }
    let before = (state.free_read, state.free_len, state.next_generation);
    assert!(db_io_preflight_page_checkout(&state, 1).is_err());
    assert_eq!((state.free_read, state.free_len, state.next_generation), before);
}

#[test]
fn db_io_result_page_reservation_plus_one_returns_the_writer() {
    let _serial = fixture_serial();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    assert_eq!(writer.write_fragment(&[0x44; DB_IO_PAGE_BYTES]).unwrap(), DB_IO_PAGE_BYTES);
    assert!(matches!(writer.write_fragment(&[0x55]), Err(DbError::LimitExceeded(_))));
    assert_eq!(writer.len(), DB_IO_PAGE_BYTES);
    assert!(writer.close_step().unwrap().is_some());
    assert!(writer.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn db_io_platform_fixed_ring_max_plus_one_returns_exact_capacity() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let source = pages(&[0x39]);
    let mut owners: [Option<DbIoPlatformBuffer>; DB_IO_PLATFORM_BUFFERS] = std::array::from_fn(|_| None);
    for owner in &mut owners {
        let copy = match db_io_prepare_platform(&source) {
            Ok(copy) => copy,
            Err(error) => panic!("platform max fixture reservation failed: {error}"),
        };
        *owner = Some(match copy.await {
            Ok(owner) => owner,
            Err(error) => panic!("platform max fixture copy failed: {error}"),
        });
    }
    assert!(matches!(db_io_prepare_platform(&source), Err(DbError::Unavailable(_))));
    for owner in &mut owners {
        let Some(owner) = owner.take() else { panic!("platform max fixture lost an admitted owner") };
        if let Err(error) = db_io_close_platform(owner).await {
            panic!("platform max fixture close failed: {error}");
        }
    }
    drain_pages(source);
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate() {
    let _serial = fixture_serial();
    while db_io_lost_owner_close_step().unwrap() {}
    DB_IO_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        let owner = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "retained-ring-owner"));
        assert!(db_io_try_park_lost_owner(owner).is_ok());
    }
    {
        let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "retained-overflow-owner")));
        }
    }
    let owner = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-plus-one-candidate"));
    assert!(db_io_park_lost_owner(owner).is_ok());
    let second = DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-plus-two-candidate"));
    assert!(db_io_park_lost_owner(second).is_ok());
    assert!(DB_IO_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let exact = quarantine.iter().flatten().find_map(|owner| match owner {
            DbIoLostOwner::Fault(candidate) => Some(candidate.detail.as_str()),
            _ => None,
        });
        assert_eq!(exact, Some("exact-plus-one-candidate"));
        assert!(quarantine.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-two-candidate")));
    }
    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        assert!(db_io_lost_owner_close_step().unwrap());
    }
    for _ in 0..DB_IO_LOST_OWNER_OVERFLOW_SLOTS * 2 {
        assert!(db_io_lost_owner_close_step().unwrap());
    }
    assert!(db_io_lost_owner_close_step().unwrap());
    {
        let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-one-candidate")));
    }
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(db_io_lost_owner_close_step().unwrap());
    {
        let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().flatten().any(|owner| matches!(owner, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-plus-two-candidate")));
    }
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(!db_io_lost_owner_close_step().unwrap());

    for _ in 0..DB_IO_LOST_OWNER_SLOTS {
        assert!(db_io_try_park_lost_owner(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-primary-owner"))).is_ok());
    }
    {
        let mut overflow = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-overflow-owner")));
        }
    }
    {
        let mut quarantine = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in quarantine.iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "full-quarantine-owner")));
        }
    }
    let refused = match db_io_park_lost_owner(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "exact-all-tier-refusal"))) {
        Err(owner) => owner,
        Ok(()) => panic!("all-tier retirement saturation accepted an unreserved owner"),
    };
    assert!(matches!(&refused, DbIoLostOwner::Fault(candidate) if candidate.detail.as_str() == "exact-all-tier-refusal"));
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(db_io_park_lost_owner(refused).is_ok());
    while db_io_lost_owner_close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let source = pages(&[0x61; DB_IO_PAGE_BYTES + 1]);
    let platform = db_io_prepare_platform(&source).unwrap().await.unwrap();
    let mut close = db_io_close_platform(platform);
    let waker = std::task::Waker::noop();
    let context = &mut std::task::Context::from_waker(waker);
    assert!(matches!(Pin::new(&mut close).poll(context), std::task::Poll::Pending));
    drop(close);
    assert!(db_io_platform_maintenance_step().unwrap());
    while db_io_lost_owner_close_step().unwrap() {}
    drain_pages(source);

    let empty_source = pages(&[]);
    let empty_platform = db_io_prepare_platform(&empty_source).unwrap().await.unwrap();
    let mut ready_close = db_io_close_platform(empty_platform);
    assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Pending));
    assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Pending));
    assert!(matches!(Pin::new(&mut ready_close).poll(context), std::task::Poll::Ready(Ok(()))));
    drop(ready_close);
    drain_pages(empty_source);

    let operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let reservation = DbIoDriverReservation::try_reserve(operation, DB_IO_PAGE_BYTES).unwrap();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    let mut copy = db_io_write_observed_bytes(reservation, vec![0x62; DB_IO_PAGE_BYTES], &mut writer);
    assert!(matches!(Pin::new(&mut copy).poll(context), std::task::Poll::Pending));
    assert!(matches!(Pin::new(&mut copy).poll(context), std::task::Poll::Pending));
    drop(copy);
    while db_io_lost_owner_close_step().unwrap() {}
    while writer.close_step().unwrap().is_some() {}
    db_io_operation_return(operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();

    let fault_operation = db_io_operation_reserve(DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    let reservation = DbIoDriverReservation::try_reserve(fault_operation, 1).unwrap();
    let mut writer = DbIoPageWriter::try_reserve(1).unwrap();
    let mut fault = db_io_write_observed_bytes(reservation, Vec::with_capacity(DB_IO_PAGE_BYTES), &mut writer);
    assert!(matches!(Pin::new(&mut fault).poll(context), std::task::Poll::Ready(Err(DbError::Unavailable(_)))));
    drop(fault);
    while db_io_lost_owner_close_step().unwrap() {}
    while writer.close_step().unwrap().is_some() {}
    db_io_operation_return(fault_operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_interrupted_close_retires_one_page_or_owner_per_grant() {
    let _serial = fixture_serial();
    let input = pages(&[0x66; DB_IO_PAGE_BYTES + 1]);
    let backend = DbIoBackendControl::Memory { slot: 0, generation: 1 };
    let document = DbIoText::try_from_str("close-fixture").unwrap();
    let mut writers = writer::WalWriterTable::new(backend);
    let permit = writers.acquire(&document, ()).unwrap();
    let mut task = DbIoTask::WalAppend { backend, document: document.clone(), writer: permit.key(), index: 0, input };
    assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
    assert!(!task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), Some(DB_IO_PAGE_BYTES));
    assert!(!task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), Some(0));
    assert!(!task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), Some(0));
    assert!(task.terminal_is_empty());
    assert_eq!(task.close_step().unwrap(), None);
    assert!(!writers.release_step(permit.key(), backend, &document).unwrap());
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_real_queued_callback_rejects_a_reused_task_slot_aba() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            let _ = started_tx.send(());
            let _ = release_rx.recv();
        }),
    )
    .ok()
    .expect("DB I/O ABA blocker admission");
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();

    let task = DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://aba-old").unwrap() };
    let old = db_io_allocate_task(task).unwrap_or_else(|(error, _)| panic!("{error}"));
    let (callback_tx, callback_rx) = std::sync::mpsc::channel();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            db_io_drive_one(old);
            let _ = callback_tx.send(());
        }),
    )
    .ok()
    .expect("DB I/O ABA callback admission");
    db_io_drive_one(old);
    let terminal = match (DbIoTaskOperation { handle: old, resolved: false }).await {
        Ok(lease) => match lease.into_result() {
            Ok(result) => result,
            Err(error) => panic!("old ABA result handback failed: {error}"),
        },
        Err(_) => panic!("old ABA task faulted"),
    };
    assert!(matches!(terminal, DbIoResult::Unit));
    drain_control_tasks(control).await;

    let reused = loop {
        let task = DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://aba-reused").unwrap() };
        let handle = db_io_allocate_task(task).unwrap_or_else(|(error, _)| panic!("{error}"));
        if handle.slot == old.slot {
            break handle;
        }
        db_io_drive_one(handle);
        let terminal = match (DbIoTaskOperation { handle, resolved: false }).await {
            Ok(lease) => match lease.into_result() {
                Ok(result) => result,
                Err(error) => panic!("ABA cycle result handback failed: {error}"),
            },
            Err(_) => panic!("ABA cycle task faulted"),
        };
        assert!(matches!(terminal, DbIoResult::Unit));
        drain_control_tasks(control).await;
    };
    let before_callback = {
        let owner = DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        (owner.generation, owner.operation, owner.phase)
    };
    release_tx.send(()).unwrap();
    callback_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    let after_callback = {
        let owner = DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        (owner.generation, owner.operation, owner.phase)
    };
    assert_eq!(before_callback, after_callback);
    assert!(db_io_slot_matches(&DB_IO_TASK_SLOTS[reused.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner), reused));
    db_io_drive_one(reused);
    let terminal = match (DbIoTaskOperation { handle: reused, resolved: false }).await {
        Ok(lease) => match lease.into_result() {
            Ok(result) => result,
            Err(error) => panic!("reused ABA result handback failed: {error}"),
        },
        Err(_) => panic!("reused ABA task faulted"),
    };
    assert!(matches!(terminal, DbIoResult::Unit));
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    loop {
        match db_io_maintenance_step() {
            Ok(true) => {}
            Ok(false) => break,
            Err(error) => panic!("ABA maintenance failed: {error}"),
        }
    }
    assert_eq!(ledger_witness(), before);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_saturated_task_retry_wakes_parked_caller_without_unrelated_ingress() {
    let _serial = fixture_serial();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
    assert_eq!(u64::from(DB_IO_RETRY_LIMIT), fixture["retry"]["maximumAttempts"].as_u64().unwrap());
    assert_eq!(DB_IO_RETRY_DELAY_MS, fixture["retry"]["timerDelayMs"].as_u64().unwrap());
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            release_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        }),
    )
    .ok()
    .expect("retry blocker admitted");
    started_rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap();
    for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
        pool.try_submit(Lane::Io, Box::new(|| {})).ok().expect("exact queue capacity");
    }
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://parked-retry").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert!(DB_IO_TASK_SLOTS[operation.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).retry_attempt.is_some());
    struct WakeSender(std::sync::mpsc::Sender<()>);
    impl std::task::Wake for WakeSender {
        fn wake(self: Arc<Self>) {
            let _ = self.0.send(());
        }
    }
    let (wake_tx, wake_rx) = std::sync::mpsc::channel();
    let waker = std::task::Waker::from(Arc::new(WakeSender(wake_tx)));
    let mut context = std::task::Context::from_waker(&waker);
    DB_IO_RETRY_MAINTENANCE_CURSOR.store((operation.handle.slot as usize + 1) % DB_IO_OPERATION_ITEMS, std::sync::atomic::Ordering::Release);
    assert!(Pin::new(&mut operation).poll(&mut context).is_pending());
    release_tx.send(()).unwrap();
    assert_eq!(wake_rx.recv_timeout(std::time::Duration::from_secs(2)).is_ok(), fixture["retry"]["terminalAfterQueueRelease"].as_bool().unwrap(), "retry must wake the parked caller without another DB request or maintenance poll");
    assert!(matches!(exact_fixture_result(operation.await, "parked retry"), DbIoResult::Unit));
    drain_control_tasks(control).await;
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    assert_eq!(ledger_witness(), before);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_retry_generation_max_publishes_a_lossless_terminal_fault() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            let _ = started_tx.send(());
            let _ = release_rx.recv();
        }),
    )
    .ok()
    .expect("DB I/O retry blocker admission");
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
        if let Err(error) = pool.try_submit(Lane::Io, Box::new(|| {})) {
            panic!("retry max fixture exhausted early: {:?}", error.kind());
        }
    }
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://retry-generation-max").unwrap() }).unwrap_or_else(|(error, _)| panic!("retry max fixture allocation failed: {error}"));
    {
        let mut owner = DB_IO_TASK_SLOTS[operation.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owner.retry_attempt.is_some());
        owner.retry_generation = u64::MAX;
    }
    db_io_retry(operation.handle, u64::MAX);
    let mut fault = match operation.take() {
        Ok(Some(Err(fault))) => fault,
        Ok(Some(Ok(_))) => panic!("retry generation exhaustion published a result"),
        Ok(None) => panic!("retry generation exhaustion did not publish its terminal fault"),
        Err(error) => panic!("retry generation exhaustion take failed: {error}"),
    };
    assert_eq!(fault.kind, DbIoFaultKind::Saturated);
    assert_eq!(fault.detail.as_str(), "DB I/O retry generation exhausted");
    while fault.close_step() {}
    release_tx.send(()).unwrap();
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    pool.shutdown();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_postgres_and_neo4j_mock_drivers_use_supplied_writer_and_observed_capacity() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    for kind in [DbIoBackendKind::Postgres, DbIoBackendKind::Neo4j] {
        let control = register_db_io_backend(kind, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();
        let output = DbIoPageWriter::try_reserve(1).unwrap();
        let task = DbIoTask::WalRead { backend: control, document: DbIoText::try_from_str("fixture-driver-result").unwrap(), index: 0, range: ByteRange { offset: 0, len: 1 }, output };
        let mut operation = submit_db_io_task(task).unwrap_or_else(|(error, _)| panic!("mock driver task allocation failed: {error}"));
        let mut lease = match operation.take_async_native().await {
            Ok(lease) => lease,
            Err(error) => panic!("mock driver lease failed: {error}"),
        };
        lease.enter_lane_io_driver_turn().unwrap();
        let task_operation = lease.operation();
        let mut reservation = DbIoDriverReservation::try_reserve(task_operation, DB_IO_OPERATION_BYTES as usize).unwrap();
        let mut driver_result = Vec::with_capacity(DB_IO_OPERATION_BYTES as usize);
        driver_result.push(0x7c);
        reservation.observe_capacity(driver_result.capacity()).unwrap();
        let result = match lease.task_mut() {
            Ok(DbIoTask::WalRead { output, .. }) => {
                assert_eq!(output.write_fragment(&driver_result).unwrap(), driver_result.len());
                drop(driver_result);
                reservation.close_step().unwrap();
                DbIoResult::Pages(output.finish().unwrap())
            }
            Ok(_) => panic!("mock driver task taxonomy changed"),
            Err(error) => panic!("mock driver lost supplied writer: {error}"),
        };
        lease.leave_lane_io_driver_turn().unwrap();
        lease.complete(Ok(result)).unwrap();
        let mut pages = match exact_fixture_result(operation.await, "mock driver") {
            DbIoResult::Pages(pages) => pages,
            _ => panic!("mock driver returned the wrong typed result"),
        };
        while pages.close_step().unwrap().is_some() {}
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
    }
    assert_eq!(ledger_witness(), before);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let polled_on_worker = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let poll_thread = Arc::new(std::sync::Mutex::new(None));
    let worker_role = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let close_thread = Arc::new(std::sync::Mutex::new(None));
    let close_worker_role = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let caller = std::thread::current().id();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(
        DbIoBackendKind::Postgres,
        Box::new(AsyncLaneProbeExecutor {
            terminal: false,
            caller,
            polled_on_worker: polled_on_worker.clone(),
            poll_thread: poll_thread.clone(),
            worker_role: worker_role.clone(),
            close_thread: close_thread.clone(),
            close_worker_role: close_worker_role.clone(),
        }),
        pool.clone(),
    )
    .unwrap();
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://actual-lane-io").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    operation.start_async_native_on_lane_io().await.unwrap();
    let terminal = operation.await.unwrap().into_result().unwrap();
    assert!(matches!(terminal, DbIoResult::Unit));
    assert!(polled_on_worker.load(std::sync::atomic::Ordering::Acquire));
    assert!(worker_role.load(std::sync::atomic::Ordering::Acquire));
    assert_ne!(*poll_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(caller));
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert!(close_worker_role.load(std::sync::atomic::Ordering::Acquire));
    assert_ne!(*close_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(caller));
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_all_five_backend_controls_require_explicit_terminal_close_witness() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    for kind in [DbIoBackendKind::Memory, DbIoBackendKind::Filesystem, DbIoBackendKind::Sqlite, DbIoBackendKind::Postgres, DbIoBackendKind::Neo4j] {
        let pool = db_io_test_pool();
        let control = register_db_io_backend(kind, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool).unwrap();
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        let waker = std::task::Waker::noop();
        let context = &mut std::task::Context::from_waker(waker);
        assert!(matches!(db_io_backend_close_lane_step(control, context), Err(DbError::StaleGeneration { .. })));
    }
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_lost_result_lease_retains_every_page_and_final_handback() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
    let pool = db_io_test_pool();
    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingOutputLifecycleLawExecutor { terminal: false, success_steps: counter.clone(), cancel_steps: counter.clone(), abandon_steps: counter }), pool.clone()).unwrap();
    let output = DbIoPageWriter::try_reserve(fixture["resultRetirement"]["pages"].as_u64().unwrap() as usize).unwrap();
    let task = DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x71; 32]), output };
    let operation = submit_db_io_task(task).unwrap_or_else(|(error, _)| panic!("{error}"));
    let mut lease = operation.await.unwrap();
    let handle = lease.handle;
    db_io_wait_task_retirement(handle).await.unwrap();
    let handback = DbIoResultHandback { handle, retained_credit: lease.retained_credit };
    let mut owner = DbIoLostOwner::ResultLease { handback, result: lease.result.take() };
    lease.transferred = true;
    drop(lease);
    let mut trace = Vec::new();
    for _ in fixture["resultRetirement"]["terminal"].as_array().unwrap() {
        trace.push(db_io_lost_owner_close_opportunity(&mut owner).unwrap());
    }
    assert!(matches!(owner, DbIoLostOwner::ResultLease { result: None, .. }));
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
    assert_eq!(serde_json::to_value(trace).unwrap(), fixture["resultRetirement"]["terminal"]);
    eprintln!("[DEBUG] lost DB result retained both pages, shell, terminal result, and final lease handback across separate close opportunities");
}

#[test]
fn db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["maintenanceFairness"];
    assert_eq!(DB_IO_MAINTENANCE_CLASSES, row["classes"].as_array().unwrap().len());
    for (name, faults) in [("continuouslyReady", false), ("firstClassFaults", true)] {
        let cursor = std::sync::atomic::AtomicUsize::new(0);
        let mut trace = Vec::new();
        for _ in row[name].as_array().unwrap() {
            let mut attempted = 0;
            let result = db_io_maintenance_turn(&cursor, |class| {
                attempted += 1;
                trace.push(class);
                if faults && class == 0 {
                    return Err(DbError::Unavailable("retained maintenance fixture fault".to_string()));
                }
                Ok(true)
            });
            assert_eq!(attempted, 1);
            assert_eq!(result.is_err(), faults && trace.last() == Some(&0));
        }
        assert_eq!(serde_json::to_value(trace).unwrap(), row[name]);
    }
    let cursor = std::sync::atomic::AtomicUsize::new(0);
    let mut attempted = 0;
    assert!(
        !db_io_maintenance_turn(&cursor, |_| {
            attempted += 1;
            Ok(false)
        })
        .unwrap()
    );
    assert_eq!(attempted, DB_IO_MAINTENANCE_CLASSES);
    eprintln!("[DEBUG] mounted DB maintenance rotates every continuously-ready or faulted class and bounds a fully idle scan to one round");
}

struct RejectedBackendPressureLawSlots;

impl RejectedBackendPressureLawSlots {
    fn reserve() -> Self {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(registry.slots.iter().all(|slot| slot.generation == 0));
        assert_ne!(registry.next_generation, u64::MAX);
        for slot in &mut registry.slots {
            slot.generation = u64::MAX;
            slot.reserved = true;
        }
        Self
    }
}

impl Drop for RejectedBackendPressureLawSlots {
    fn drop(&mut self) {
        let mut registry = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in &mut registry.slots {
            if slot.generation == u64::MAX && slot.reserved && slot.executor.is_none() && slot.pool.is_none() {
                *slot = DbIoRejectedBackendSlot::empty();
            }
        }
    }
}

struct LostOwnerPressureLawSlots;

fn retire_lost_owner_pressure_slots<const N: usize>(slots: &std::sync::Mutex<[Option<DbIoLostOwner>; N]>) {
    let mut slots = slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    for slot in slots.iter_mut() {
        let Some(DbIoLostOwner::Fault(owner)) = slot.as_mut() else { panic!("lost-owner pressure row changed its exact fault sentinel") };
        while owner.close_step() {}
        *slot = None;
    }
}

impl LostOwnerPressureLawSlots {
    fn reserve() -> Self {
        while db_io_lost_owner_close_step().unwrap() {}
        for slot in DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "registration-primary-sentinel")));
        }
        for slot in DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "registration-overflow-sentinel")));
        }
        for slot in DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() {
            *slot = Some(DbIoLostOwner::Fault(db_io_literal_fault(DbIoFaultKind::Backend, DbIoFaultCause::Internal, "registration-quarantine-sentinel")));
        }
        Self
    }
}

impl Drop for LostOwnerPressureLawSlots {
    fn drop(&mut self) {
        retire_lost_owner_pressure_slots(&DB_IO_LOST_OWNERS);
        retire_lost_owner_pressure_slots(&DB_IO_LOST_OWNER_OVERFLOW);
        retire_lost_owner_pressure_slots(&DB_IO_LOST_OWNER_QUARANTINE);
    }
}

#[semio_framework_async_macros::async_test]
async fn db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure() {
    let _serial = fixture_serial();
    while db_io_lost_owner_close_step().unwrap() {}
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔐️writer/🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["backendPressure"];
    assert_eq!(DB_IO_BACKEND_CONTROLS, row["capacity"].as_u64().unwrap() as usize);
    let pressure = DB_IO_RETIREMENT_PRESSURE_FAULT.swap(false, std::sync::atomic::Ordering::AcqRel);
    let sentinels = RejectedBackendPressureLawSlots::reserve();
    let credit = DbIoCredit { pages: 0, bytes: size_of::<BlockingCompleteLawExecutor>() as u64, items: 1, controls: 1 };
    let operation = db_io_backend_owner_reserve(credit).unwrap();
    let pool = db_io_test_pool();
    let pool_use = pool.acquire_use().unwrap();
    assert!(db_io_try_park_lost_owner(DbIoLostOwner::Backend { owner: Some(Box::new(BlockingCompleteLawExecutor { terminal: false })), operation, credit, pool: Some(pool.clone()), pool_use: Some(pool_use.clone()) }).is_ok());
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(DB_IO_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    let retained = {
        let owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        matches!(&owners[0], Some(DbIoLostOwner::Backend { owner: Some(owner), operation: actual_operation, credit: actual_credit, pool: Some(actual_pool), pool_use: Some(actual_pool_use) }) if !owner.backend_terminal_is_empty() && *actual_operation == operation && *actual_credit == credit && Arc::ptr_eq(actual_pool, &pool) && Arc::ptr_eq(actual_pool_use, &pool_use))
    };
    assert_eq!(retained, row["retainedWhenFull"].as_bool().unwrap());
    assert_eq!(ledger_witness().0, before.0.checked_add(credit).unwrap());
    drop(sentinels);
    assert!(db_io_lost_owner_close_step().unwrap());
    assert!(!db_io_lost_owner_close_step().unwrap());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let terminal = db_io_rejected_backends().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.iter().all(|slot| slot.generation == 0);
        if terminal {
            assert_eq!(terminal, row["terminalAfterCapacityReturns"].as_bool().unwrap());
            break;
        }
        assert!(std::time::Instant::now() < deadline, "retained rejected backend did not finish after capacity returned");
        db_io_rejected_backend_maintenance_step().unwrap();
        semio_framework_async::yield_once().await;
    }
    assert_eq!(ledger_witness(), before);
    DB_IO_RETIREMENT_PRESSURE_FAULT.store(pressure, std::sync::atomic::Ordering::Release);
    eprintln!("[DEBUG] full rejected-backend registry preserved the exact lost executor, pool, operation, and credit until normal lane retirement returned every owner");
}

#[test]
fn db_io_lost_page_handle_resumes_the_same_retirement_cursor() {
    let _serial = fixture_serial();
    let owner = pages(&[0x77; DB_IO_PAGE_BYTES + 1]);
    let operation = owner.operation();
    drop(owner);
    assert_eq!(db_io_page_maintenance_step().unwrap(), Some(DB_IO_PAGE_BYTES));
    assert_eq!(db_io_page_maintenance_step().unwrap(), Some(DB_IO_PAGE_BYTES));
    assert_eq!(db_io_page_maintenance_step().unwrap(), None);
    assert_ne!(operation, 0);
}

#[semio_framework_async_macros::async_test]
async fn db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return() {
    let _serial = fixture_serial();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
    let inline = size_of::<MemoryDbIoExecutor>() as u64;
    assert!(inline <= fixture["maximumInlineBytes"].as_u64().unwrap(), "fixed backend tables must not occupy the caller stack");
    let before = ledger_witness();
    let static_expected = DbIoCredit { pages: 0, bytes: MemoryDbIoExecutor::backing_bytes(), items: 1, controls: 1 };
    let controller_credit = writer::release::controller_credit();
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
        let filler = DbIoCredit { bytes: DB_IO_PROCESS_BYTES - before.0.bytes - remaining, ..DbIoCredit::default() };
        let filler_operation = db_io_backend_owner_reserve(filler).unwrap();
        let admitted = db_io_backend_owner_reserve(expected);
        assert_eq!(admitted.is_ok(), case["accepted"].as_bool().unwrap());
        if let Ok(operation) = admitted {
            let ledger = lock(db_io_operation_ledger());
            assert_eq!(ledger.slots[db_io_operation_slot(&ledger, operation).unwrap()].live, expected);
            drop(ledger);
            db_io_operation_return(operation, expected).unwrap();
        }
        db_io_operation_return(filler_operation, filler).unwrap();
        assert_eq!(ledger_witness(), before);
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
        assert_eq!(fixture["writerTable"]["slots"].as_u64().unwrap() as usize, writer::WAL_WRITER_CAPACITY);
        assert_eq!(fixture["writerTable"]["separateBox"], true);
        assert_eq!(fixture["tables"].as_array().unwrap().len(), tables.len());
        for ((name, slots, _), row) in tables.iter().zip(fixture["tables"].as_array().unwrap()) {
            assert_eq!(row["name"].as_str().unwrap(), *name);
            assert_eq!(row["slots"].as_u64().unwrap(), *slots as u64);
        }
        assert_eq!(inline + size_of::<writer::WalWriterTable<()>>() as u64 + tables.iter().map(|(_, _, bytes)| *bytes as u64).sum::<u64>(), static_expected.bytes);
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
    assert_eq!(db_io_backend_close_lane_step(storage.control, &mut context).unwrap(), fixture["closeWhileAdmitted"].as_bool().unwrap());
    assert!(lock(db_io_backend_registry()).slots[slot as usize].executor.is_some());
    db_io_backend_return_operation(storage.control, probe).unwrap();
    db_io_operation_return(probe, probe_credit).unwrap();
    let document = ArtifactId("memory-retirement-frontier".into());
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.create_segment(&writer, 0).await.unwrap();
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
    let _serial = fixture_serial();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
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
        (task.generation, task.operation, task.phase)
    };
    drain_pages(retained[0].take().unwrap());
    let reused_after = {
        let task = DB_IO_TASK_SLOTS[reused.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(db_io_slot_matches(&task, reused.handle));
        (task.generation, task.operation, task.phase)
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
    let _serial = fixture_serial();
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

#[semio_framework_async_macros::async_test]
async fn db_io_async_native_lost_backend_uses_typed_lane_lease_and_mounted_terminal_witness() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(DbIoBackendKind::Postgres, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();
    let mut operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://async-native").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    let lease = operation.take_async_native().await.unwrap();
    lease.complete(Ok(DbIoResult::Unit)).unwrap();
    let terminal = match operation.await {
        Ok(terminal) => terminal,
        Err(_) => panic!("async-native law task faulted"),
    };
    assert!(matches!(terminal.into_result().unwrap(), DbIoResult::Unit));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_cancellation_before_during_and_receiver_drop_retain_exact_terminal_owners() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    let control = register_db_io_backend(DbIoBackendKind::Neo4j, Box::new(AsyncNativeLawExecutor { terminal: false }), pool.clone()).unwrap();

    let mut before_execution = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://cancel-before").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    before_execution.cancel().unwrap();
    let mut fault = match before_execution.await {
        Err(fault) => fault,
        Ok(_) => panic!("cancel-before fixture returned a result"),
    };
    assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
    assert_eq!(fault.cause, DbIoFaultCause::Closed);
    while fault.close_step() {}
    drain_control_tasks(control).await;

    let mut during_execution = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://cancel-during").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    let lease = during_execution.take_async_native().await.unwrap();
    during_execution.cancel().unwrap();
    assert!(db_io_maintenance_step().unwrap());
    {
        let owner = DB_IO_TASK_SLOTS[during_execution.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owner.async_detached);
        assert!(owner.task.is_none());
        assert!(owner.close_enqueued);
        assert!(owner.terminal.is_none());
    }
    let mut output = DbIoPageWriter::try_reserve_for_operation(lease.operation(), 1).unwrap();
    assert_eq!(output.write_fragment(&[0xA5]).unwrap(), 1);
    lease.complete(Ok(DbIoResult::Pages(output.finish().unwrap()))).unwrap();
    let mut fault = loop {
        match during_execution.take().unwrap() {
            Some(Err(fault)) => break fault,
            Some(Ok(_)) => panic!("cancel-during fixture published its retained result"),
            None => assert!(db_io_maintenance_step().unwrap()),
        }
    };
    assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
    assert_eq!(fault.cause, DbIoFaultCause::Closed);
    while fault.close_step() {}
    drain_control_tasks(control).await;

    let mut receiver_drop = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://receiver-drop").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    let lease = receiver_drop.take_async_native().await.unwrap();
    drop(receiver_drop);
    drop(lease);
    drain_control_tasks(control).await;
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_panic_backend_fault_and_shutdown_close_reach_exact_prior_witness() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = db_io_test_pool();
    for (panics, expected) in [(false, DbIoFaultKind::Backend), (true, DbIoFaultKind::Panic)] {
        let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingFaultLawExecutor { panic: panics, terminal: false }), pool.clone()).unwrap();
        let operation = submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://hostile-blocking").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
        let mut fault = match operation.await {
            Err(fault) => fault,
            Ok(_) => panic!("hostile blocking fixture returned a result"),
        };
        assert_eq!(fault.kind, expected);
        assert_eq!(fault.cause, DbIoFaultCause::Internal);
        let expected_detail = if panics { "DB I/O backend panicked" } else { "hostile DB I/O fixture backend fault" };
        assert_eq!(fault.detail.as_str(), expected_detail);
        while fault.close_step() {}
        drain_control_tasks(control).await;
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
    }
    assert_eq!(ledger_witness(), before);
}

#[cfg(all(feature = "fs", feature = "sqlite", not(target_arch = "wasm32")))]
async fn open_real_storage_fixture(backend: &str, pool: Arc<WorkerPool>, path: &std::path::Path) -> Result<(), DbStorageOpenRejected> {
    match backend {
        "fs" => drop(FsStorage::open(pool, path).await?),
        "sqlite" => drop(crate::db_storage_sqlite::SqliteStorage::open(pool, path).await?),
        _ => panic!("unknown physical storage fixture"),
    }
    Ok(())
}

#[cfg(all(feature = "fs", feature = "sqlite", not(target_arch = "wasm32")))]
fn opening_fixture_control(pool: &Arc<WorkerPool>) -> DbIoBackendControl {
    let registry = lock(db_io_backend_registry());
    let (index, slot) = registry.slots.iter().enumerate().find(|(_, slot)| slot.pool.as_ref().is_some_and(|registered| Arc::ptr_eq(registered, pool))).expect("opening registered its exact pool");
    db_io_backend_control(slot.kind, index as u16, slot.generation)
}

#[cfg(all(feature = "fs", feature = "sqlite", not(target_arch = "wasm32")))]
fn opening_fixture_close_requested(control: DbIoBackendControl) -> bool {
    let registry = lock(db_io_backend_registry());
    let (index, generation) = db_io_backend_parts(control);
    let slot = &registry.slots[index as usize];
    slot.generation != generation || slot.close_requested
}

#[cfg(all(feature = "fs", feature = "sqlite", not(target_arch = "wasm32")))]
async fn drain_opening_fixture_pool(pool: &Arc<WorkerPool>) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let retained = lock(db_io_backend_registry()).slots.iter().any(|slot| slot.pool.as_ref().is_some_and(|registered| Arc::ptr_eq(registered, pool)));
        let tasks_retained = DB_IO_TASK_SLOTS.iter().any(|task| lock(task).pool.as_ref().is_some_and(|registered| Arc::ptr_eq(registered, pool)));
        if !retained && !tasks_retained {
            break;
        }
        assert!(std::time::Instant::now() < deadline, "opening Drop failed to retire the exact registered pool use");
        db_io_maintenance_step().expect("opening terminal maintenance");
        semio_framework_async::yield_once().await;
    }
}

#[cfg(all(feature = "fs", feature = "sqlite", not(target_arch = "wasm32")))]
#[semio_framework_async_macros::async_test]
async fn db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen() {
    let _serial = fixture_serial();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    for row in fixture["opening"].as_array().unwrap().iter().filter(|row| row["cause"] == "queued-drop") {
        let before = ledger_witness();
        let slots_before = lock(db_io_backend_registry()).free_len;
        let backend = row["backend"].as_str().unwrap();
        let root = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("physical opening law requires its ticket artifact directory")).join(format!("storage-open-queued-{backend}-{}", std::process::id()));
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        pool.try_submit(
            Lane::Io,
            Box::new(move || {
                let _ = started_tx.send(());
                let _ = release_rx.recv();
            }),
        )
        .ok()
        .expect("opening lane blocker admission");
        started_rx.recv_timeout(std::time::Duration::from_secs(5)).expect("opening lane blocker starts");
        let mut opening = Box::pin(open_real_storage_fixture(backend, pool.clone(), &root));
        let pending = std::future::Future::poll(opening.as_mut(), &mut std::task::Context::from_waker(std::task::Waker::noop())).is_pending();
        let control = opening_fixture_control(&pool);
        assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
        drop(opening);
        let close_requested = opening_fixture_close_requested(control);
        release_tx.send(()).expect("opening lane blocker release");
        assert!(pending, "actual BackendOpen remained queued before cancellation");
        assert_eq!(close_requested, row["closeRequestedBeforeRetry"].as_bool().unwrap(), "cancelled {backend} opening must request close before any caller retry");
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(ledger_witness() == before && lock(db_io_backend_registry()).free_len == slots_before, row["ledgerBaseline"].as_bool().unwrap(), "cancelled {backend} registration returns exact credit and slot");
        assert_eq!(open_real_storage_fixture(backend, pool.clone(), &root).await.is_ok(), row["reopens"].as_bool().unwrap(), "fresh {backend} storage opens after cancellation");
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(pool.shutdown().is_ok(), row["poolShutdownAfterDrain"].as_bool().unwrap());
        assert_eq!(ledger_witness(), before);
        assert_eq!(lock(db_io_backend_registry()).free_len, slots_before);
        eprintln!("[DEBUG] real-storage-open: backend={backend} cause=queued-drop close-requested={close_requested} ledger=baseline reopened=true pool=terminal");
    }
}

#[cfg(all(feature = "fs", feature = "sqlite", not(target_arch = "wasm32")))]
#[semio_framework_async_macros::async_test]
async fn db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry() {
    let _serial = fixture_serial();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    for row in fixture["opening"].as_array().unwrap().iter().filter(|row| row["cause"] == "path-type-conflict") {
        let before = ledger_witness();
        let slots_before = lock(db_io_backend_registry()).free_len;
        let backend = row["backend"].as_str().unwrap();
        let root = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("physical opening law requires its ticket artifact directory")).join(format!("storage-open-fault-{backend}-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let bad = root.join("wrong-physical-type");
        if backend == "fs" {
            std::fs::write(&bad, b"not a directory").unwrap();
        } else {
            std::fs::create_dir_all(&bad).unwrap();
            assert!(rusqlite::Connection::open(&bad).is_err(), "independent SQLite rejects the same physical directory path");
        }
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let rejected = open_real_storage_fixture(backend, pool.clone(), &bad).await.expect_err("physical BackendOpen must reject the wrong filesystem type");
        let control = match &rejected {
            DbStorageOpenRejected::Registered { control, .. } => *control,
            _ => panic!("physical opening fault lost its registered backend"),
        };
        let close_requested = opening_fixture_close_requested(control);
        drop(rejected);
        assert_eq!(close_requested, row["closeRequestedBeforeRetry"].as_bool().unwrap(), "failed {backend} opening must already own deferred close");
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(ledger_witness() == before && lock(db_io_backend_registry()).free_len == slots_before, row["ledgerBaseline"].as_bool().unwrap());
        assert_eq!(open_real_storage_fixture(backend, pool.clone(), &root.join("recovered")).await.is_ok(), row["reopens"].as_bool().unwrap());
        drain_opening_fixture_pool(&pool).await;
        assert_eq!(pool.shutdown().is_ok(), row["poolShutdownAfterDrain"].as_bool().unwrap());
        assert_eq!(ledger_witness(), before);
        assert_eq!(lock(db_io_backend_registry()).free_len, slots_before);
        eprintln!("[DEBUG] real-storage-open: backend={backend} cause=path-type-conflict close-requested={close_requested} ledger=baseline reopened=true pool=terminal");
    }
}

#[semio_framework_async_macros::async_test]
async fn db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close() {
    let _serial = fixture_serial();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    assert_eq!(fixture["version"], 1);
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 9);
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn db_io_backend_registration_saturation_returns_exact_executor_before_pool_use() {
    let _serial = fixture_serial();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "all-retirement-tiers-full-return-exact-executor").unwrap();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let lost_owner_sentinels = LostOwnerPressureLawSlots::reserve();
    let sentinels = RejectedBackendPressureLawSlots::reserve();
    let rejected = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap_err();
    assert_eq!(rejected.has_retained_executor(), row["expected"] == "exact-owner");
    drop(sentinels);
    drop(lost_owner_sentinels);
    let control = rejected.retry().unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal() {
    let _serial = fixture_serial();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.try_submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            release_rx.recv().unwrap();
        }),
    )
    .ok()
    .expect("prepared rollback blocker admission");
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
    for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
        pool.try_submit(Lane::Io, Box::new(|| {})).ok().expect("exact prepared rollback refusal capacity");
    }
    let rollback = DbIoBackendRollbackReservation::try_reserve().unwrap();
    let pool_use = pool.acquire_use().unwrap();
    let previous_generation = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let previous = registry.next_generation;
        registry.next_generation = u64::MAX;
        previous
    };
    let rejected = register_db_io_backend_prepared_with_use(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone(), pool_use, rollback).unwrap_err();
    db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation = previous_generation;
    let mut rejected = DbStorageOpenRejected::Registration(rejected);
    assert!(rejected.has_retained_backend());
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    release_tx.send(()).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let cause = loop {
        match rejected.retry_close().await {
            Ok(cause) => break cause,
            Err(retained) => rejected = retained,
        }
        assert!(std::time::Instant::now() < deadline, "prepared registration rejection lost its terminal close owner");
    };
    assert!(matches!(cause, DbError::Unavailable(_)));
    assert_eq!(pool.shutdown(), Ok(()));
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_task_uses_registered_backend_pool_not_caller_pool() {
    let _serial = fixture_serial();
    let registered = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let unrelated = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), registered.clone()).unwrap();
    assert_eq!(unrelated.shutdown(), Ok(()));
    let result =
        submit_db_io_task(DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://registered-pool").unwrap() }).unwrap_or_else(|(error, _)| panic!("registered-pool task admission failed: {error}")).finish().await.unwrap();
    assert!(matches!(result, DbIoResult::Unit));
    assert_eq!(registered.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(registered.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn db_io_backend_drop_retains_pool_until_deferred_close_terminal() {
    let _serial = fixture_serial();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let allow_close = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(DeferredBackendCloseLawExecutor { allow_close: allow_close.clone(), terminal: false }), pool.clone()).unwrap();
    drop(DropRegisteredBackend(control));
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    allow_close.store(true, std::sync::atomic::Ordering::Release);
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn db_io_forged_backend_kind_is_rejected_before_task_page_admission() {
    let _serial = fixture_serial();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Memory, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (slot, generation) = db_io_backend_parts(control);
    let forged = DbIoBackendControl::Filesystem { slot, generation };
    let input = pages(b"must-remain-owned");
    let (error, task) = match submit_db_io_task(DbIoTask::PayloadPut { backend: forged, input }) {
        Err(rejected) => rejected,
        Ok(_) => panic!("forged kind published a task"),
    };
    assert!(matches!(error, DbError::StaleGeneration { .. }));
    let DbIoTask::PayloadPut { input, .. } = task else { panic!("forged task returned the wrong exact owner") };
    assert!(input.pages.iter().flatten().all(|page| page.phase().unwrap() == DbIoPagePhase::CheckedOutInput));
    drain_pages(input);
    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
}
