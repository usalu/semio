//! 🎟️ Task operation lease and admission: the task operation, the async task lease and the admitted submission. See [`super`].

use super::*;

//#region 🔖️Limits
pub struct DbIoTaskOperation {
    pub handle: DbIoTaskHandle,
    pub resolved: bool,
    pub custody: Option<DbIoFaultCustody>,
}

/// 🌐️ Exact task/backend lease driven by an async-native platform executor after Lane::Io admission.
pub struct DbIoAsyncTaskLease {
    pub handle: DbIoTaskHandle,
    pub backend: DbIoBackendControl,
    pub task: Option<DbIoTask>,
    pub executor: Option<Box<dyn DbIoTaskExecutor>>,
    pub completed: bool,
    pub(super) credit_returned: bool,
}

pub(super) fn db_io_async_lease_credit() -> DbIoCredit {
    DbIoCredit { pages: 0, bytes: size_of::<DbIoAsyncTaskLease>() as u64, items: 1, controls: 1 }
}

impl DbIoAsyncTaskLease {
    pub fn operation(&self) -> u64 {
        self.handle.operation
    }

    pub fn task_mut(&mut self) -> Result<&mut DbIoTask, DbError> {
        self.task.as_mut().ok_or_else(|| DbError::Internal("DB I/O async task lease lost its typed task".to_string()))
    }

    pub fn executor_mut<T: DbIoTaskExecutor + 'static>(&mut self) -> Result<&mut T, DbError> {
        self.executor.as_mut().and_then(|executor| executor.as_any_mut().downcast_mut::<T>()).ok_or_else(|| DbError::Internal("DB I/O async executor taxonomy mismatch".to_string()))
    }

    pub fn parts_mut<T: DbIoTaskExecutor + 'static>(&mut self) -> Result<(&mut T, &mut DbIoTask), DbError> {
        let executor = self.executor.as_mut().and_then(|executor| executor.as_any_mut().downcast_mut::<T>()).ok_or_else(|| DbError::Internal("DB I/O async executor taxonomy mismatch".to_string()))?;
        let task = self.task.as_mut().ok_or_else(|| DbError::Internal("DB I/O async task lease lost task".to_string()))?;
        Ok((executor, task))
    }

    pub fn enter_lane_io_driver_turn(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || owner.phase != DbIoTaskPhase::Executing || owner.async_lane_turn {
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
        }
        owner.async_lane_turn = true;
        Ok(())
    }

    pub fn leave_lane_io_driver_turn(&self) -> Result<(), DbError> {
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || !owner.async_lane_turn {
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
        }
        owner.async_lane_turn = false;
        Ok(())
    }

    pub fn complete(mut self, terminal: Result<DbIoResult, DbError>) -> Result<(), DbError> {
        let executor = self.executor.take().ok_or_else(|| DbError::Internal("DB I/O async executor returned twice".to_string()))?;
        db_io_return_async_executor(self.backend, self.handle.operation, executor)?;
        let task = self.task.take().ok_or_else(|| DbError::Internal("DB I/O async task returned twice".to_string()))?;
        let transition = task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
        let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !db_io_slot_matches(&owner, self.handle) {
            return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
        }
        if owner.async_lane_turn {
            return Err(DbError::Internal("DB I/O async driver completed before leaving Lane::Io authority".to_string()));
        }
        owner.async_detached = false;
        owner.task = Some(task);
        owner.terminal = Some(match transition {
            Err(error) => DbIoTerminal::Fault(db_io_task_fault(owner.fault_custody.take().expect("preborn task fault custody"), DbIoFaultKind::Backend, error)),
            Ok(()) if owner.cancelled => DbIoTerminal::Cancelled(terminal.ok()),
            Ok(()) => match terminal {
                Ok(result) => DbIoTerminal::Result(result),
                Err(error) => DbIoTerminal::Fault(db_io_task_fault(owner.fault_custody.take().expect("preborn task fault custody"), DbIoFaultKind::Backend, error)),
            },
        });
        owner.phase = match owner.terminal.as_ref() {
            Some(DbIoTerminal::Result(_)) => DbIoTaskPhase::Completed,
            Some(DbIoTerminal::Cancelled(_)) => DbIoTaskPhase::Cancelled,
            _ => DbIoTaskPhase::Faulted,
        };
        let waker = db_io_wake(&mut owner);
        drop(owner);
        db_io_operation_return(self.handle.operation, db_io_async_lease_credit())?;
        self.credit_returned = true;
        self.completed = true;
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }

    pub fn start_on_lane_io(mut self) -> Result<(), DbError> {
        let executor = self.executor.take().ok_or_else(|| DbError::Internal("DB I/O async executor transferred twice".to_string()))?;
        let task = self.task.take().ok_or_else(|| DbError::Internal("DB I/O async task transferred twice".to_string()))?;
        let runtime = executor.driver_runtime();
        let future = executor.drive_async(self.handle.operation, task);
        let future = match runtime {
            Some(runtime) => runtime.detach(future),
            None => future,
        };
        {
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if !db_io_slot_matches(&owner, self.handle) || !owner.async_detached || owner.async_driver.is_some() {
                return Err(DbError::StaleGeneration { expected: GenerationId(self.handle.generation), actual: GenerationId(owner.generation) });
            }
            owner.async_driver = Some(future);
        }
        self.completed = true;
        self.credit_returned = true;
        db_io_request_async_driver_poll(self.handle);
        Ok(())
    }
}

impl Drop for DbIoAsyncTaskLease {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(executor) = self.executor.take() {
            let _ = db_io_return_async_executor(self.backend, self.handle.operation, executor);
        }
        if let Some(task) = self.task.take() {
            let _ = task.transition_pages(DbIoPagePhase::Executing, DbIoPagePhase::TerminalResult);
            let mut owner = DB_IO_TASK_SLOTS[self.handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if db_io_slot_matches(&owner, self.handle) {
                owner.task = Some(task);
                owner.async_lane_turn = false;
                owner.async_detached = false;
                owner.cancelled = true;
                owner.phase = DbIoTaskPhase::Cancelled;
                owner.terminal = Some(DbIoTerminal::Cancelled(None));
                if let Some(waker) = db_io_wake(&mut owner) {
                    drop(owner);
                    waker.wake();
                }
            }
        }
        if !self.credit_returned {
            if db_io_operation_return(self.handle.operation, db_io_async_lease_credit()).is_ok() {
                self.credit_returned = true;
            } else {
                DB_IO_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
            }
        }
    }
}

pub(super) const DB_IO_TASK_CAPACITY_EXHAUSTED: &str = "db I/O task capacity exhausted";

pub(super) const DB_IO_ASYNC_NATIVE_OPERATION_BUSY: &str = "DB I/O async-native backend operation capacity exhausted";

pub(super) const DB_IO_BACKEND_PENDING_OPERATIONS: &str = "DB I/O backend pending operations";

/// ⏳️ How long [`submit_db_io_task_admitted`] waits for DB I/O admission before it answers the refusal: long
/// enough to ride out a burst (an operation holds its capacity for milliseconds to seconds), and well inside the hub's
/// 30 s document-socket frame deadline, so a capacity that never frees (resident values) still reaches its writer as
/// a refusal instead of a closed socket.
pub const DB_IO_ADMISSION_WAIT_MS: u64 = 10_000;

/// 🚦️ Whether `error` only says a fixed DB I/O capacity is momentarily taken — the task arena, the process
/// credit ledger, the page arena, a backend's pending operations, or an async-native backend's one in-flight
/// operation — so the same task is admitted once something is released.
fn db_io_admission_saturated(error: &DbError) -> bool {
    match error {
        DbError::Unavailable(message) => [DB_IO_TASK_CAPACITY_EXHAUSTED, DB_IO_PROCESS_CREDIT_EXHAUSTED, DB_IO_TASK_ADMISSION_EXHAUSTED, DB_IO_ASYNC_NATIVE_OPERATION_BUSY, DB_IO_PROCESS_PAGES_EXHAUSTED].contains(&message.as_str()),
        DbError::LimitExceeded(label) => *label == DB_IO_BACKEND_PENDING_OPERATIONS,
        _ => false,
    }
}

/// 🚦️ Whether registering a backend was refused only because a fixed backend capacity is momentarily taken
/// (the control registry, the rollback registry, the backend process credit) — so a fresh registration succeeds
/// once a retiring backend releases its share.
pub fn db_io_backend_admission_saturated(error: &DbError) -> bool {
    matches!(error, DbError::Unavailable(message) if [DB_IO_BACKEND_CONTROLS_EXHAUSTED, DB_IO_BACKEND_ROLLBACK_EXHAUSTED, DB_IO_BACKEND_CREDIT_EXHAUSTED].contains(&message.as_str()))
}

/// ⏳️ Resolves once DB I/O capacity has been released since `epoch` (registered before it re-checks, so no
/// release between a refused submission and this wait is lost), or at `deadline_ms` on `pool`'s clock — the wait is
/// bounded by the pool's timer, not by the next release, so a capacity that never frees still ends in its refusal.
struct DbIoAdmissionReleased {
    pub epoch: u64,
    pub waiter: Option<u64>,
    pub pool: Arc<WorkerPool>,
    pub(super) deadline_ms: u64,
    deadline_armed: bool,
}

impl DbIoAdmissionReleased {
    pub fn new(epoch: u64, pool: Arc<WorkerPool>, deadline_ms: u64) -> Self {
        Self { epoch, waiter: None, pool, deadline_ms, deadline_armed: false }
    }
}

impl Future for DbIoAdmissionReleased {
    type Output = Result<(), DbError>;

    fn poll(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        if DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire) != this.epoch || this.pool.now_ms() >= this.deadline_ms {
            return std::task::Poll::Ready(Ok(()));
        }
        let mut table = DB_IO_ADMISSION_WAITER_TABLE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let fresh = this.waiter.is_none();
        if !table.register(&mut this.waiter, context.waker()) {
            return std::task::Poll::Ready(Err(DbError::Unavailable("DB I/O admission waiters saturated".to_string())));
        }
        if fresh {
            DB_IO_ADMISSION_WAITING.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        }
        drop(table);
        if DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire) != this.epoch {
            return std::task::Poll::Ready(Ok(()));
        }
        if !this.deadline_armed {
            this.deadline_armed = true;
            let waker = context.waker().clone();
            this.pool.callback_at(this.deadline_ms, move || waker.wake());
        }
        std::task::Poll::Pending
    }
}

impl Drop for DbIoAdmissionReleased {
    fn drop(&mut self) {
        if let Some(waiter) = self.waiter.take() {
            DB_IO_ADMISSION_WAITER_TABLE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(waiter);
            DB_IO_ADMISSION_WAITING.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

/// 🚦️ Submits `task`, waiting (at most [`DB_IO_ADMISSION_WAIT_MS`]) while a fixed DB I/O capacity is taken
/// instead of answering the refusal: a postgres or neo4j backend runs one async-native operation at a time, so the
/// second concurrent write of a busy hub used to be refused (`async-native backend operation capacity exhausted`, the
/// growth e2e's 24 documents on postgres, g18), and the process ledger refused writes while other documents held
/// credit. Any other refusal is answered at once; dropping the future cancels the wait.
pub async fn submit_db_io_task_admitted(mut task: DbIoTask) -> Result<DbIoTaskOperation, DbError> {
    let kind = task.kind();
    let mut clock: Option<(Arc<WorkerPool>, u64)> = None;
    let mut waiting: Option<std::time::Instant> = None;
    let outcome = loop {
        let epoch = DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire);
        match submit_db_io_task(task) {
            Ok(operation) => break Ok(operation),
            Err((error, returned)) if db_io_admission_saturated(&error) => {
                if clock.is_none() {
                    clock = db_io_backend_pool(returned.backend()).map(|pool| {
                        let deadline_ms = pool.now_ms().saturating_add(DB_IO_ADMISSION_WAIT_MS);
                        (pool, deadline_ms)
                    });
                }
                let Some((pool, deadline_ms)) = clock.clone().filter(|(pool, deadline_ms)| pool.now_ms() < *deadline_ms) else { break Err(error) };
                task = returned;
                waiting.get_or_insert_with(std::time::Instant::now);
                if let Err(error) = DbIoAdmissionReleased::new(epoch, pool, deadline_ms).await {
                    break Err(error);
                }
            }
            Err((error, _)) => break Err(error),
        }
    };
    if let Some(started) = waiting {
        db_io_census_admission_wait(kind, started.elapsed(), outcome.is_err());
    }
    outcome
}

/// 🕰️ The worker pool (and so the clock) serving `control`'s backend while it is registered.
fn db_io_backend_pool(control: DbIoBackendControl) -> Option<Arc<WorkerPool>> {
    let (slot, generation) = db_io_backend_parts(control);
    let registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    registry.slots.get(usize::from(slot)).filter(|owner| owner.generation == generation).and_then(|owner| owner.pool.clone())
}

/// 🚪️ Opens one storage backend with `open`, waiting (at most [`DB_IO_ADMISSION_WAIT_MS`] on `pool`'s clock) while
/// the process's fixed backend capacity — the control registry, the rollback registry, the backend process credit — is
/// taken by backends still retiring, instead of refusing. Every storage facade opens through it: in a shared test process
/// dozens of laws open and retire backends at once, and a refused open failed an unrelated law ("db I/O backend control
/// capacity exhausted", the in-process `--lib` flake). Any other refusal is answered at once; dropping the future
/// cancels the wait.
pub async fn open_db_io_backend_admitted<T, O>(pool: &Arc<WorkerPool>, mut open: impl FnMut() -> O) -> Result<T, DbStorageOpenRejected>
where
    O: Future<Output = Result<T, DbStorageOpenRejected>>,
{
    let deadline_ms = pool.now_ms().saturating_add(DB_IO_ADMISSION_WAIT_MS);
    let mut waiting: Option<std::time::Instant> = None;
    let outcome = loop {
        let epoch = DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire);
        match open().await {
            Err(rejected) if db_io_backend_admission_saturated(rejected.error()) && pool.now_ms() < deadline_ms => {
                drop(rejected);
                waiting.get_or_insert_with(std::time::Instant::now);
                if let Err(error) = DbIoAdmissionReleased::new(epoch, pool.clone(), deadline_ms).await {
                    break Err(error.into());
                }
            }
            outcome => break outcome,
        }
    };
    if let Some(started) = waiting {
        db_io_census_admission_wait(DbIoTaskKind::BackendOpen, started.elapsed(), outcome.is_err());
    }
    outcome
}

pub fn submit_db_io_task(task: DbIoTask) -> Result<DbIoTaskOperation, (DbError, DbIoTask)> {
    let _ = db_io_maintenance_step();
    let kind = task.kind();
    let handle = db_io_allocate_task(task)?;
        let custody = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).poll_custody.take();
DB_IO_CENSUS_TASKS[kind as usize].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    db_io_submit_job(handle, Box::new(move || db_io_drive_one(handle)), 0);
    Ok(DbIoTaskOperation { handle, resolved: false, custody })
}
//#endregion 🔖️Limits
