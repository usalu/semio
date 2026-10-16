use super::*;

use db_io_core::{DB_IO_BACKEND_CONTROLS, DB_IO_LEDGER_OWNER, DB_IO_LOST_OWNERS, DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_OVERFLOW_SLOTS, DB_IO_LOST_OWNER_QUARANTINE, DB_IO_LOST_OWNER_SLOTS, DB_IO_OPERATION_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DB_IO_RETIREMENT_PRESSURE_FAULT, DB_IO_TASK_SLOTS, DB_IO_TOTAL_PAGES, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoCredit, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoFault, DbIoFaultCause, DbIoFaultKind, DbIoLeaseResult, DbIoLedgerOwner, DbIoLostOwner, DbIoPageArenaState, DbIoPagePhase, DbIoPageWriter, DbIoPages, DbIoResult, DbIoResultHandback, DbIoResultLease, DbIoTask, DbIoTaskExecutor, DbIoTaskPhase, DbIoText, LeaseStorage as _, PayloadStorage as _, db_io_backend_owner_reserve, db_io_backend_parts, db_io_ledger_census, db_io_operation_reserve, db_io_operation_return, db_io_page_arena, db_io_park_lost_owner, db_io_preflight_page_checkout, db_io_slot_matches, db_io_try_park_lost_owner};
use db_io_registry::{DB_IO_PLATFORM_BUFFERS, DB_IO_RETRY_DELAY_MS, DB_IO_RETRY_LIMIT, DB_IO_RETRY_MAINTENANCE_CURSOR, DbIoBackendKind, DbIoBackendRollbackReservation, DbIoPlatformBuffer, DbIoRejectedBackendSlot, db_io_backend_close_step, db_io_backend_registry, db_io_close_platform, db_io_literal_fault, db_io_lost_owner_close_opportunity, db_io_lost_owner_close_step, db_io_page_maintenance_step, db_io_platform_maintenance_step, db_io_prepare_platform, db_io_rejected_backend_maintenance_step, db_io_rejected_backends, db_io_task_fault, db_io_write_observed_bytes, register_db_io_backend};

fn fixture_owner() -> DbIoLedgerOwner {
    DbIoLedgerOwner::enter()
}

struct AsyncNativeLawExecutor {
    pub terminal: bool,
}

struct AsyncFaultTaxonomyLawExecutor {
    pub terminal: bool,
}

struct BlockingFaultLawExecutor {
    pub panic: bool,
    pub terminal: bool,
}

struct BlockingCompleteLawExecutor {
    pub terminal: bool,
}

struct DeferredBackendCloseLawExecutor {
    allow_close: Arc<std::sync::atomic::AtomicBool>,
    pub terminal: bool,
}

struct CleanupFaultLawExecutor {
    pub fail: Arc<std::sync::atomic::AtomicBool>,
    pub terminal: bool,
}

impl DbIoTaskExecutor for CleanupFaultLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("cleanup fault fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        if self.fail.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Io("injected operation cleanup fault".to_string()));
        }
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

struct DropRegisteredBackend(DbIoBackendControl);

impl Drop for DropRegisteredBackend {
    fn drop(&mut self) {
        let _ = retire_db_io_backend(self.0);
    }
}

struct BlockingOutputLifecycleLawExecutor {
    pub terminal: bool,
    success_steps: Arc<std::sync::atomic::AtomicUsize>,
    cancel_steps: Arc<std::sync::atomic::AtomicUsize>,
    abandon_steps: Arc<std::sync::atomic::AtomicUsize>,
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

struct AsyncLaneProbeExecutor {
    pub terminal: bool,
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
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
    fixture.fault_categories.get(usize::from(hash.0[0])).copied().map(fault_fixture_error).unwrap_or_else(|| DbError::Internal("fault taxonomy fixture discriminator is out of range".to_string()))
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
        *self.close_thread.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(std::thread::current().id());
        self.close_worker_role.store(semio_framework_trace::is_worker_thread(), std::sync::atomic::Ordering::Release);
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
    db_io_ledger_census(DB_IO_LEDGER_OWNER.with(std::cell::Cell::get))
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

fn admit_fixture_job(pool: &WorkerPool, lane: Lane, mut job: semio_framework_async::Job) {
    loop {
        match pool.try_submit(lane, job) {
            Ok(()) => return,
            Err(error) if error.kind() == WorkerSubmitErrorKind::Contended => {
                job = error.into_job();
                std::thread::yield_now();
            }
            Err(error) => panic!("fixture job admission refused: {:?}", error.kind()),
        }
    }
}

async fn drain_control_tasks(control: DbIoBackendControl) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while DB_IO_TASK_SLOTS.iter().any(|task| task.lock().unwrap_or_else(std::sync::PoisonError::into_inner).backend == Some(control)) {
        assert!(std::time::Instant::now() < deadline, "closed tasks did not retire through the backend hook or an owner maintenance opportunity");
        db_io_maintenance_step().unwrap();
        semio_framework_async::yield_once().await;
    }
}

#[test]
fn db_io_fixed_page_max_plus_one_and_zero_are_exact() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_fixed_page_max_plus_one_and_zero_are_exact") {
        return;
    }
    let _owner = fixture_owner();
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
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fault = db_io_task_fault(DbIoFaultKind::Backend, &DbError::Rejected { policy: protocol::MergePolicy::Normal, worst: semio_framework_diagnostic::Severity::Error, messages: Vec::new() });
    assert_eq!(fault.cause, DbIoFaultCause::Internal);
    assert_eq!(fault.into_db_error(), DbError::Internal("DB I/O backend returned an artifact-layer rejection".to_string()));
    while db_io_maintenance_step().unwrap() {}
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_async_native_fault_preserves_exact_category_scalars_and_retires() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
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
            if db_io_slot_matches(&owner, handle) {
                if let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() {
                    let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult), "async fault taxonomy output left TerminalResult before its task retired");
                }
            }
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

#[semio_framework_async_macros::async_test]
async fn db_io_output_task_yield_cancel_abandon_and_close_retire_exactly_once() {
    let _owner = fixture_owner();
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
    let steps_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while cancel_steps.load(std::sync::atomic::Ordering::Acquire) < 3 && std::time::Instant::now() < steps_deadline {
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
        if db_io_slot_matches(&owner, handle) {
            if let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() {
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
            }
        }
    }
    assert_eq!(fault.kind, DbIoFaultKind::Cancelled);
    while fault.close_step() {}
    drain_control_tasks(control).await;

    let output = DbIoPageWriter::try_reserve(2).unwrap();
    let operation = submit_db_io_task(DbIoTask::PayloadGet { backend: control, hash: ContentHash([0x73; 32]), output }).unwrap_or_else(|(error, _)| panic!("abandon lifecycle task admission failed: {error}"));
    let steps_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while abandon_steps.load(std::sync::atomic::Ordering::Acquire) < 3 && std::time::Instant::now() < steps_deadline {
        std::thread::yield_now();
    }
    assert!(abandon_steps.load(std::sync::atomic::Ordering::Acquire) >= 3);
    let handle = operation.handle;
    drop(operation);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, handle) || matches!(owner.phase, DbIoTaskPhase::Cancelled | DbIoTaskPhase::Closing) {
            break;
        }
        let witness = format!("phase={:?} cancelled={} abandoned={} task={} retry={:?} submit_kind={:?} steps={}", owner.phase, owner.cancelled, owner.abandoned, owner.task.is_some(), owner.retry_attempt, owner.terminal_submit_kind, abandon_steps.load(std::sync::atomic::Ordering::Acquire));
        drop(owner);
        assert!(std::time::Instant::now() < deadline, "abandoned lifecycle task never observed its cancellation: {witness}");
        std::thread::yield_now();
    }
    {
        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if db_io_slot_matches(&owner, handle) {
            assert!(matches!(owner.phase, DbIoTaskPhase::Cancelled | DbIoTaskPhase::Closing));
            if let Some(DbIoTask::PayloadGet { output, .. }) = owner.task.as_ref() {
                let arena = db_io_page_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                assert!(output.pages.iter().take(output.reserved as usize).flatten().all(|page| arena.slots[page.slot as usize].phase == DbIoPagePhase::TerminalResult));
            }
        }
    }
    drain_control_tasks(control).await;

    retire_db_io_backend(control).unwrap();
    close_db_io_backend(control).await.unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_artifact_and_lease_result_owners_retain_exact_incremental_handback") {
        return;
    }
    let _owner = fixture_owner();
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
fn db_io_process_page_max_plus_one_preflight_is_atomic() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_process_page_max_plus_one_preflight_is_atomic") {
        return;
    }
    let mut state = DbIoPageArenaState::new();
    for _ in 0..DB_IO_TOTAL_PAGES / DB_IO_OPERATION_PAGES {
        db_io_preflight_page_checkout(&state, DB_IO_OPERATION_PAGES).unwrap();
        state.free_len -= DB_IO_OPERATION_PAGES;
    }
    let before = (state.free_read, state.free_len, state.next_generation);
    assert!(db_io_preflight_page_checkout(&state, 1).is_err());
    assert_eq!((state.free_read, state.free_len, state.next_generation), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_platform_fixed_ring_max_plus_one_returns_exact_capacity() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_platform_fixed_ring_max_plus_one_returns_exact_capacity") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    let sources: [DbIoPages; DB_IO_PLATFORM_BUFFERS + 1] = std::array::from_fn(|_| pages(&[0x39]));
    let mut owners: [Option<DbIoPlatformBuffer>; DB_IO_PLATFORM_BUFFERS] = std::array::from_fn(|_| None);
    for (owner, source) in owners.iter_mut().zip(&sources) {
        let copy = match db_io_prepare_platform(source) {
            Ok(copy) => copy,
            Err(error) => panic!("platform max fixture reservation failed: {error}"),
        };
        *owner = Some(match copy.await {
            Ok(owner) => owner,
            Err(error) => panic!("platform max fixture copy failed: {error}"),
        });
    }
    let credit_before_refusal = ledger_witness();
    assert!(matches!(db_io_prepare_platform(&sources[DB_IO_PLATFORM_BUFFERS]), Err(DbError::Unavailable(detail)) if detail == "DB I/O prepared platform capacity exhausted"));
    assert_eq!(ledger_witness(), credit_before_refusal, "the ring max+1 refusal returns its exact capacity");
    for owner in &mut owners {
        let Some(owner) = owner.take() else { panic!("platform max fixture lost an admitted owner") };
        if let Err(error) = db_io_close_platform(owner).await {
            panic!("platform max fixture close failed: {error}");
        }
    }
    for source in sources {
        drain_pages(source);
    }
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate") {
        return;
    }
    let _owner = fixture_owner();
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
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger") {
        return;
    }
    let _owner = fixture_owner();
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
    assert!(matches!(Pin::new(&mut fault).poll(context), std::task::Poll::Ready(Err(DbError::LimitExceeded("DB I/O external driver allocation capacity")))));
    drop(fault);
    while db_io_lost_owner_close_step().unwrap() {}
    while writer.close_step().unwrap().is_some() {}
    db_io_operation_return(fault_operation, DbIoCredit { pages: 0, bytes: 0, items: 0, controls: 1 }).unwrap();
    assert_eq!(ledger_witness(), before);
}

#[test]
fn db_io_interrupted_close_retires_one_page_or_owner_per_grant() {
    let _owner = fixture_owner();
    let input = pages(&[0x66; DB_IO_PAGE_BYTES + 1]);
    let backend = DbIoBackendControl::Memory { slot: 0, generation: 1 };
    let document = DbIoText::try_from_str("close-fixture").unwrap();
    let mut writers = crate::WalWriterTable::new(backend);
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
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    admit_fixture_job(
        &pool,
        Lane::Io,
        Box::new(move || {
            let _ = started_tx.send(());
            let _ = release_rx.recv();
        }),
    );
    started_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();

    let task = DbIoTask::BackendOpen { backend: control, path: DbIoText::try_from_str("fixture://aba-old").unwrap() };
    let old = db_io_allocate_task(task).unwrap_or_else(|(error, _)| panic!("{error}"));
    let (callback_tx, callback_rx) = std::sync::mpsc::channel();
    admit_fixture_job(
        &pool,
        Lane::Io,
        Box::new(move || {
            db_io_drive_one(old);
            let _ = callback_tx.send(());
        }),
    );
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
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧮️memory-backing/🔣️.json")).unwrap();
    assert_eq!(u64::from(DB_IO_RETRY_LIMIT), fixture["retry"]["maximumAttempts"].as_u64().unwrap());
    assert_eq!(DB_IO_RETRY_DELAY_MS, fixture["retry"]["timerDelayMs"].as_u64().unwrap());
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            release_rx.recv_timeout(std::time::Duration::from_secs(5)).unwrap();
        }),
    );
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
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let control = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    admit_fixture_job(
        &pool,
        Lane::Io,
        Box::new(move || {
            let _ = started_tx.send(());
            let _ = release_rx.recv();
        }),
    );
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
    let _owner = fixture_owner();
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
        assert!(DbIoDriverReservation::try_reserve(task_operation, DB_IO_OPERATION_BYTES as usize).is_err());
        let mut reservation = DbIoDriverReservation::try_reserve(task_operation, DB_IO_PAGE_BYTES).unwrap();
        let mut driver_result = Vec::with_capacity(DB_IO_PAGE_BYTES);
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
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker") {
        return;
    }
    let _owner = fixture_owner();
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
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_all_five_backend_controls_require_explicit_terminal_close_witness") {
        return;
    }
    let _owner = fixture_owner();
    let before = ledger_witness();
    for kind in [DbIoBackendKind::Memory, DbIoBackendKind::Filesystem, DbIoBackendKind::Sqlite, DbIoBackendKind::Postgres, DbIoBackendKind::Neo4j] {
        let pool = db_io_test_pool();
        let control = register_db_io_backend(kind, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool).unwrap();
        retire_db_io_backend(control).unwrap();
        close_db_io_backend(control).await.unwrap();
        let waker = std::task::Waker::noop();
        let context = &mut std::task::Context::from_waker(waker);
        assert!(matches!(db_io_backend_close_step(control, context), Err(DbError::StaleGeneration { .. })));
    }
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_lost_result_lease_retains_every_page_and_final_handback() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
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
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure") {
        return;
    }
    let _owner = fixture_owner();
    while db_io_lost_owner_close_step().unwrap() {}
    let before = ledger_witness();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
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
}

#[test]
fn db_io_lost_page_handle_resumes_the_same_retirement_cursor() {
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_lost_page_handle_resumes_the_same_retirement_cursor") {
        return;
    }
    let _owner = fixture_owner();
    while db_io_lost_owner_close_step().unwrap() {}
    let before = ledger_witness();
    let owner = pages(&[0x77; DB_IO_PAGE_BYTES + 1]);
    let operation = owner.operation();
    assert_ne!(operation, 0);
    drop(owner);
    assert_eq!(db_io_page_maintenance_step().unwrap(), None, "a dropped page set parks as one lost owner, never as loose page handles");
    let mut opportunities = 0;
    while db_io_lost_owner_close_step().unwrap() {
        opportunities += 1;
        let parked = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().flatten().count();
        assert!(parked <= 1, "the lost page set resumes one retirement cursor");
    }
    assert_eq!(opportunities, 4, "two pages, the shell credit, then the terminal slot release: one owner per opportunity");
    assert_eq!(ledger_witness(), before);
}

#[semio_framework_async_macros::async_test]
async fn db_io_async_native_lost_backend_uses_typed_lane_lease_and_mounted_terminal_witness() {
    let _owner = fixture_owner();
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
    let _owner = fixture_owner();
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
    db_io_maintenance_step().unwrap();
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
            None => {
                db_io_maintenance_step().unwrap();
                semio_framework_async::yield_once().await;
            }
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
    let _owner = fixture_owner();
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

#[semio_framework_async_macros::async_test]
async fn db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close() {
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
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
    if !crate::process_isolated_law("db_io_retained_fixtures::db_io_backend_registration_saturation_returns_exact_executor_before_pool_use") {
        return;
    }
    let _owner = fixture_owner();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔐️backend-pool-use/🔣️.json")).unwrap();
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
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    pool.submit(
        Lane::Io,
        Box::new(move || {
            started_tx.send(()).unwrap();
            release_rx.recv().unwrap();
        }),
    );
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
    let _owner = fixture_owner();
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
    let _owner = fixture_owner();
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
    let _owner = fixture_owner();
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

/// 🧯️ An operation-cleanup fault stays with its backend: the close ring records it on the failing backend and moves on, so a
/// healthy backend's operation finishes, drains and closes `Ok` beside it, while the failing backend's own waiter and close
/// report the fault (its kind preserved) until its cleanup succeeds.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let fail = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let faulty = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(CleanupFaultLawExecutor { fail: fail.clone(), terminal: false }), pool.clone()).unwrap();
    let healthy = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let injected = DbError::Io("injected operation cleanup fault".to_string());
    let faulted = submit_db_io_task(DbIoTask::BackendOpen { backend: faulty, path: DbIoText::try_from_str("fixture://cleanup-fault").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert_eq!(faulted.finish().await.err(), Some(injected.clone()), "the failing backend's own waiter reports its cleanup fault");
    let neighbour = submit_db_io_task(DbIoTask::BackendOpen { backend: healthy, path: DbIoText::try_from_str("fixture://cleanup-fault-neighbour").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert!(matches!(neighbour.finish().await, Ok(DbIoResult::Unit)), "a neighbour's operation never inherits another backend's cleanup fault");
    drain_control_tasks(healthy).await;
    close_db_io_backend(healthy).await.unwrap();
    assert_eq!(close_db_io_backend(faulty).await, Err(injected), "the failing backend's close reports its own cleanup fault");
    fail.store(false, std::sync::atomic::Ordering::Release);
    drain_control_tasks(faulty).await;
    close_db_io_backend(faulty).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
    assert_eq!(ledger_witness(), before);
}
