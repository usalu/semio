/// 🧪️ Law executors and fault fixtures: the law executors, the page lifecycle fixture, the fault category fixtures and oracle and the async executor impls.

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
    let fixture: PageLifecycleFixture = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
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
