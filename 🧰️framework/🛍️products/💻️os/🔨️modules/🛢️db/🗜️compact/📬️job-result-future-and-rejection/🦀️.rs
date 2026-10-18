//! 📬️ Job result, future and rejection: the job result, the future, the close of a rejected job and the rejection. See [`super`].

use super::*;

//#region 🧵️RetainedCompactionJob
/// 🧹 Retained completion carrying exact storage, document, holder and report owners.
pub struct DatabaseCompactionResult {
    pub(super) state: Option<Arc<DatabaseCompactionState>>,
    pub(super) execution: Option<DatabaseCompactionExecution>,
}

impl std::fmt::Debug for DatabaseCompactionResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCompactionResult").field("generation", &self.state.as_ref().map(|state| state.generation)).field("execution", &self.execution.is_some()).finish()
    }
}

impl DatabaseCompactionResult {
    pub fn into_parts(mut self) -> Result<(Arc<db_storage::DbBackend>, ArtifactId, db_io_core::DbIoText, Result<CompactionReport, DbError>), Self> {
        let execution = self.execution.take();
        match execution {
            Some(execution) => {
                if let Some(state) = self.state.take() {
                    state.release_terminal();
                }
                Ok((execution.storage, execution.document, execution.holder, execution.result))
            }
            None => Err(self),
        }
    }

    pub fn close_and_take_report(mut self) -> Result<CompactionReport, DbError> {
        let Some(execution) = self.execution.take() else { return Err(DbError::Internal("database compaction result owner missing".to_string())) };
        let DatabaseCompactionExecution { storage, document, holder, result } = execution;
        if let Some(state) = self.state.take() {
            *state.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners { storage: Some(storage), document: Some(document), holder: Some(holder), result: None });
            state.callback_close.store(true, std::sync::atomic::Ordering::Release);
            state.arm_callback_close();
        }
        result
    }
}

impl Drop for DatabaseCompactionResult {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else { return };
        if let Some(execution) = self.execution.take() {
            *state.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners::from_execution(execution));
        }
        state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        state.callback_close.store(true, std::sync::atomic::Ordering::Release);
        state.arm_callback_close();
    }
}

/// 🚦 Future facade for the generation-qualified compaction registry authority.
pub struct DatabaseCompactionFuture {
    pub(super) state: Option<Arc<DatabaseCompactionState>>,
    completed: bool,
}

impl std::fmt::Debug for DatabaseCompactionFuture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCompactionFuture").field("generation", &self.generation()).field("completed", &self.completed).finish()
    }
}

impl DatabaseCompactionFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, document: ArtifactId, holder: db_io_core::DbIoText, consolidate_snapshots: bool, budget: CompactionBudget, now_ms: u64) -> Result<Self, DatabaseCompactionRejected> {
        let pool_use = match pool.acquire_use() {
            Ok(pool_use) => pool_use,
            Err(error) => return Err(DatabaseCompactionRejected::new(pool, DbError::Unavailable(format!("database compaction WorkerPool use rejected: {error:?}")), storage, document, holder)),
        };
        let admission = match DatabaseCompactionAdmission::try_claim(&document) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCompactionRejected::new(pool, error, storage, document, holder)),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        let resource = match compaction_resource(&document) {
            Ok(resource) => resource,
            Err(error) => return Err(DatabaseCompactionRejected::new(pool, error, storage, document, holder)),
        };
        let lease_recovery = DatabaseCompactionLeaseRecovery::new(storage.clone(), resource, holder.clone());
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let expired = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let progress = Arc::new(std::sync::atomic::AtomicU8::new(DatabaseCompactionProgress::Admitted as u8));
        let future = Box::pin(retained_compaction_execute(storage, document, holder, consolidate_snapshots, budget, now_ms, cancelled.clone(), expired.clone(), progress.clone(), lease_recovery.clone()));
        let deadline_ms = pool.now_ms().saturating_add(DATABASE_COMPACTION_DEADLINE_MS);
        let state = Arc::new(DatabaseCompactionState {
            pool: pool.clone(),
            _pool_use: pool_use,
            slot,
            generation,
            admission: std::sync::Mutex::new(Some(admission)),
            core: std::sync::Mutex::new(DatabaseCompactionCore { future: Some(future), output: None, release_waiting: None, release_fault: None, release_retry_fault: None, quarantined: None, release_quarantined: None, panic_release: None }),
            terminal: std::sync::Mutex::new(None),
            driver: std::sync::atomic::AtomicU8::new(DatabaseCompactionDriverAuthority::Idle as u8),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            cancelled,
            expired,
            deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            progress,
            abandoned: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            callback_close: std::sync::atomic::AtomicBool::new(false),
            callback_armed: std::sync::atomic::AtomicBool::new(false),
            release_retry_armed: std::sync::atomic::AtomicBool::new(false),
            panic_fault: std::sync::atomic::AtomicBool::new(false),
            panic_retired: std::sync::atomic::AtomicBool::new(false),
            lease_recovery,
            waker: std::sync::Mutex::new(None),
        });
        database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        let deadline = state.clone();
        pool.callback_at(deadline_ms, move || deadline.deadline_callback());
        state.schedule();
        Ok(Self { state: Some(state), completed: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn progress(&self) -> DatabaseCompactionProgress {
        self.state.as_ref().map_or(DatabaseCompactionProgress::Fault, |state| state.progress())
    }

    pub fn cancel(&self) {
        if let Some(state) = self.state.as_ref() {
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }
}

impl DatabaseCompactionState {
    pub(super) fn progress(&self) -> DatabaseCompactionProgress {
        DatabaseCompactionProgress::from_u8(self.progress.load(std::sync::atomic::Ordering::Acquire))
    }

    pub(super) fn deadline_callback(self: &Arc<Self>) {
        if self.current() && !matches!(self.progress(), DatabaseCompactionProgress::Completed | DatabaseCompactionProgress::Cancelled | DatabaseCompactionProgress::Fault) {
            self.expired.store(true, std::sync::atomic::Ordering::Release);
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.schedule();
        }
    }
}

impl Future for DatabaseCompactionFuture {
    type Output = Result<DatabaseCompactionResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(Err(DbError::Closed));
        }
        let Some(state) = self.state.as_ref().cloned() else { return std::task::Poll::Ready(Err(DbError::Closed)) };
        if state.panic_retired.load(std::sync::atomic::Ordering::Acquire) {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Err(DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string())));
        }
        if !state.current() {
            self.completed = true;
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.callback_close.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
            return std::task::Poll::Ready(Err(DbError::StaleGeneration { expected: GenerationId(state.generation), actual: GenerationId(state.observed_generation()) }));
        }
        let execution = { state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take() };
        if let Some(execution) = execution {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseCompactionResult { state: Some(state), execution: Some(execution) }));
        }
        *state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let execution = { state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take() };
        if let Some(execution) = execution {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseCompactionResult { state: Some(state), execution: Some(execution) }));
        }
        state.schedule();
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseCompactionFuture {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.callback_close.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }
}

struct DatabaseCompactionRejectedClose {
    pub(super) pool: Arc<WorkerPool>,
    pub(super) owners: std::sync::Mutex<Option<DatabaseCompactionTerminalOwners>>,
    queued: std::sync::atomic::AtomicBool,
    retries: std::sync::atomic::AtomicU8,
}

impl DatabaseCompactionRejectedClose {
    pub(super) fn schedule(self: &Arc<Self>) {
        if self.queued.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let close = self.clone();
        match self.pool.try_submit(Lane::Io, Box::new(move || close.drive_one())) {
            Ok(()) => {}
            Err(error) => {
                drop(error.into_job());
                self.queued.store(false, std::sync::atomic::Ordering::Release);
                let attempt = self.retries.fetch_add(1, std::sync::atomic::Ordering::AcqRel).saturating_add(1);
                let close = self.clone();
                self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
                    if attempt >= DATABASE_COMPACTION_RETRY_LIMIT {
                        close.drive_one();
                    } else {
                        close.schedule();
                    }
                });
            }
        }
    }

    pub(super) fn drive_one(self: Arc<Self>) {
        self.queued.store(false, std::sync::atomic::Ordering::Release);
        let mut owners = self.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let pending = owners.as_mut().is_some_and(DatabaseCompactionTerminalOwners::close_one);
        if owners.as_ref().is_some_and(DatabaseCompactionTerminalOwners::terminal_is_empty) {
            owners.take();
        }
        drop(owners);
        if pending {
            self.schedule();
        }
    }
}

/// ⛔ Lossless pre-admission refusal carrying exact compaction input identities.
pub struct DatabaseCompactionRejected {
    pub(super) error: Option<DbError>,
    pub(super) close: Arc<DatabaseCompactionRejectedClose>,
}

impl std::fmt::Debug for DatabaseCompactionRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCompactionRejected").field("error", &self.error).finish()
    }
}

impl DatabaseCompactionRejected {
    pub(super) fn new(pool: Arc<WorkerPool>, error: DbError, storage: Arc<db_storage::DbBackend>, document: ArtifactId, holder: db_io_core::DbIoText) -> Self {
        Self {
            error: Some(error),
            close: Arc::new(DatabaseCompactionRejectedClose {
                pool,
                owners: std::sync::Mutex::new(Some(DatabaseCompactionTerminalOwners { storage: Some(storage), document: Some(document), holder: Some(holder), result: None })),
                queued: std::sync::atomic::AtomicBool::new(false),
                retries: std::sync::atomic::AtomicU8::new(0),
            }),
        }
    }

    pub fn into_parts(mut self) -> Result<(DbError, Arc<db_storage::DbBackend>, ArtifactId, db_io_core::DbIoText), Self> {
        let error = self.error.take();
        let mut owners = self.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let storage = owners.as_mut().and_then(|owners| owners.storage.take());
        let document = owners.as_mut().and_then(|owners| owners.document.take());
        let holder = owners.as_mut().and_then(|owners| owners.holder.take());
        drop(owners);
        match (error, storage, document, holder) {
            (Some(error), Some(storage), Some(document), Some(holder)) => Ok((error, storage, document, holder)),
            (error, storage, document, holder) => {
                self.error = error;
                *self.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners { storage, document, holder, result: None });
                Err(self)
            }
        }
    }

    pub fn close_and_take_error(mut self) -> DbError {
        let error = self.error.take().unwrap_or(DbError::LimitExceeded("database compaction refusal error"));
        self.close.schedule();
        error
    }
}

impl Drop for DatabaseCompactionRejected {
    fn drop(&mut self) {
        if self.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            self.close.schedule();
        }
    }
}
//#endregion 🧵️RetainedCompactionJob
