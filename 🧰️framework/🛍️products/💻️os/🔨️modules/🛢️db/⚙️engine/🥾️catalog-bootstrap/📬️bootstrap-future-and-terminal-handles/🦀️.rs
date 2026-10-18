//! 📬️ Catalog bootstrap future and terminal handles: the bootstrap future, the terminal handle and result, the terminal taker and the live bootstrap counter. See [`super`].

use super::*;

//#region 🔖️CatalogBootstrapCas
/// ⏳️ Public check-register-recheck future for one mounted initial-catalog CAS.
pub struct DatabaseCatalogBootstrapFuture {
    pub state: Arc<DatabaseCatalogBootstrapState>,
    pub resolved: bool,
}

impl DatabaseCatalogBootstrapFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, expected: EpochFence) -> Result<Self, DatabaseCatalogBootstrapRejected> {
        Self::try_prepare_with_key(pool, storage, pages, DatabaseCatalogBootstrapKey::root(), expected, true)
    }

    pub(super) fn try_submit_with_key(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, key: DatabaseCatalogBootstrapKey, expected: EpochFence) -> Result<Self, DatabaseCatalogBootstrapRejected> {
        Self::try_prepare_with_key(pool, storage, pages, key, expected, true)
    }

    pub(super) fn try_prepare_with_key(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, key: DatabaseCatalogBootstrapKey, expected: EpochFence, schedule: bool) -> Result<Self, DatabaseCatalogBootstrapRejected> {
        let pool_use = match pool.acquire_use() {
            Ok(pool_use) => pool_use,
            Err(error) => return Err(DatabaseCatalogBootstrapRejected::new(pool, DbError::Unavailable(format!("database catalog-bootstrap WorkerPool use rejected: {error:?}")), storage, pages, key, expected)),
        };
        Self::try_prepare_with_use(pool, pool_use, storage, pages, key, expected, schedule)
    }

    pub fn try_prepare_with_use(
        pool: Arc<WorkerPool>,
        pool_use: Arc<WorkerPoolUse>,
        storage: Arc<db_storage::DbBackend>,
        pages: db_io_core::DbIoPages,
        key: DatabaseCatalogBootstrapKey,
        expected: EpochFence,
        schedule: bool,
    ) -> Result<Self, DatabaseCatalogBootstrapRejected> {
        let admission = match DatabaseCatalogBootstrapAdmission::try_claim(pages.page_count()) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCatalogBootstrapRejected::new(pool, error, storage, pages, key, expected)),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        if database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot].is_some() {
            return Err(DatabaseCatalogBootstrapRejected::new(pool, DbError::LimitExceeded("database catalog-bootstrap terminal slot"), storage, pages, key, expected));
        }
        DATABASE_CATALOG_BOOTSTRAP_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let state = Arc::new(DatabaseCatalogBootstrapState {
            pool,
            _pool_use: pool_use,
            slot,
            generation,
            admission: Mutex::new(Some(admission)),
            storage: Mutex::new(Some(storage)),
            pages: Mutex::new(Some(pages)),
            key: Mutex::new(Some(key)),
            expected,
            work: Mutex::new(None),
            poll_work: Mutex::new(None),
            terminal_work: Mutex::new(None),
            staged_actual: Mutex::new(None),
            terminal_error: Mutex::new(None),
            completion: Mutex::new(None),
            terminal_completion: Mutex::new(None),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            driver_authority: std::sync::atomic::AtomicU8::new(DatabaseCatalogBootstrapDriverAuthority::Idle as u8),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            polling: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_pressure: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            closing: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            terminal_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_result_checked_out: std::sync::atomic::AtomicBool::new(false),
            phase: std::sync::atomic::AtomicU8::new(DatabaseCatalogBootstrapPhase::Handoff as u8),
            progress: std::sync::atomic::AtomicU8::new(DatabaseCatalogBootstrapProgress::Admitted as u8),
            #[cfg(any(test, feature = "testkit"))]
            controlled_submit_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_publication_before_waker_hook: Mutex::new(None),
            #[cfg(test)]
            controlled_driver_claim_hook: Mutex::new(None),
            #[cfg(test)]
            controlled_driver_release_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            active_drivers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(any(test, feature = "testkit"))]
            max_active_drivers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            driver_grants: std::sync::atomic::AtomicU64::new(0),
            #[cfg(any(test, feature = "testkit"))]
            poll_worker_thread: std::sync::atomic::AtomicBool::new(false),
        });
        database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        if schedule {
            state.schedule();
        }
        Ok(Self { state, resolved: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> DatabaseCatalogBootstrapProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => DatabaseCatalogBootstrapProgress::Admitted,
            1 => DatabaseCatalogBootstrapProgress::Scheduled,
            2 => DatabaseCatalogBootstrapProgress::Polling,
            3 => DatabaseCatalogBootstrapProgress::Pending,
            4 => DatabaseCatalogBootstrapProgress::Completed,
            5 => DatabaseCatalogBootstrapProgress::Cancelled,
            _ => DatabaseCatalogBootstrapProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }
}

impl Future for DatabaseCatalogBootstrapFuture {
    type Output = Result<DatabaseCatalogBootstrapResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let completion = { self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() };
        if let Some(result) = completion {
            self.resolved = true;
            return std::task::Poll::Ready(result);
        }
        #[cfg(test)]
        if let Some(hook) = self.state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let completion = { self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() };
        if let Some(result) = completion {
            self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.resolved = true;
            return std::task::Poll::Ready(result);
        }
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseCatalogBootstrapFuture {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        let completion = { self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() };
        if let Some(result) = completion {
            *self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }
}

/// 🧲️ Generation-qualified public authority for one abandoned bootstrap CAS.
pub struct DatabaseCatalogBootstrapTerminalHandle {
    pub state: Arc<DatabaseCatalogBootstrapState>,
}

impl std::fmt::Debug for DatabaseCatalogBootstrapTerminalHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCatalogBootstrapTerminalHandle").field("witness", &self.witness()).finish()
    }
}

impl DatabaseCatalogBootstrapTerminalHandle {
    pub fn witness(&self) -> DatabaseCatalogBootstrapTerminalWitness {
        DatabaseCatalogBootstrapTerminalWitness { generation: self.state.generation, retained_owners: self.state.retained_owner_count(), terminal_empty: self.state.terminal_is_empty() }
    }

    pub fn close_step(&self) -> DatabaseCatalogBootstrapCloseStep {
        if self.state.terminal_is_empty() {
            return DatabaseCatalogBootstrapCloseStep::Complete;
        }
        self.state.closing.store(true, std::sync::atomic::Ordering::Release);
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        let authority = self.state.driver_authority.load(std::sync::atomic::Ordering::Acquire);
        if authority == DatabaseCatalogBootstrapDriverAuthority::Driving as u8 || authority == DatabaseCatalogBootstrapDriverAuthority::Retry as u8 {
            DatabaseCatalogBootstrapCloseStep::Blocked
        } else {
            self.state.schedule();
            DatabaseCatalogBootstrapCloseStep::Progress
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty()
    }

    pub fn take_result(&self) -> Option<DatabaseCatalogBootstrapTerminalResult> {
        use std::sync::atomic::Ordering;
        if self.state.terminal_result_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        if self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.terminal_result_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(DatabaseCatalogBootstrapTerminalResult { state: self.state.clone(), checked_out: true })
    }

    pub fn resume(self) -> Result<DatabaseCatalogBootstrapFuture, Self> {
        use std::sync::atomic::Ordering;
        let state = self.state.clone();
        if state.finished.load(Ordering::Acquire) || state.driver_authority.load(Ordering::Acquire) != DatabaseCatalogBootstrapDriverAuthority::Idle as u8 || state.closing.load(Ordering::Acquire) {
            return Err(self);
        }
        let terminal_completion = { state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() };
        if let Some(mut result) = terminal_completion {
            if let Ok(owner) = result.as_mut() {
                owner.state = Some(state.clone());
            }
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else if let Some((job, attempt)) = { state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() } {
            if state.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Idle as u8, DatabaseCatalogBootstrapDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
                *state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
                return Err(self);
            }
            state.scheduled.store(true, Ordering::Release);
            state.submit_exact(job, attempt);
        } else if let Some(work) = { state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() } {
            *state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            state.set_phase(DatabaseCatalogBootstrapPhase::Poll);
            state.schedule();
        } else if state.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            state.set_phase(DatabaseCatalogBootstrapPhase::Handoff);
            state.schedule();
        } else {
            return Err(self);
        }
        state.cancelled.store(false, Ordering::Release);
        state.abandoned.store(false, Ordering::Release);
        state.terminal_checked_out.store(false, Ordering::Release);
        Ok(DatabaseCatalogBootstrapFuture { state, resolved: false })
    }
}

impl Drop for DatabaseCatalogBootstrapTerminalHandle {
    fn drop(&mut self) {
        self.state.terminal_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

/// 📤️ Checked-out terminal result whose Drop hands authority back to the registry.
pub struct DatabaseCatalogBootstrapTerminalResult {
    pub state: Arc<DatabaseCatalogBootstrapState>,
    pub checked_out: bool,
}

impl DatabaseCatalogBootstrapTerminalResult {
    pub fn take(mut self) -> Option<Result<DatabaseCatalogBootstrapResult, DbError>> {
        let mut result = self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(Ok(owner)) = result.as_mut() {
            owner.state = Some(self.state.clone());
        }
        if result.is_some() {
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub fn resume(mut self) -> Result<DatabaseCatalogBootstrapFuture, Self> {
        let Some(mut result) = self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return Err(self) };
        if let Ok(owner) = result.as_mut() {
            owner.state = Some(self.state.clone());
        }
        *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.abandoned.store(false, std::sync::atomic::Ordering::Release);
        self.checked_out = false;
        self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        Ok(DatabaseCatalogBootstrapFuture { state: self.state.clone(), resolved: false })
    }
}

impl Drop for DatabaseCatalogBootstrapTerminalResult {
    fn drop(&mut self) {
        if self.checked_out {
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
    }
}

/// 🧲️ Takes one abandoned bootstrap authority by exact non-reusable generation.
pub fn take_database_catalog_bootstrap_terminal(generation: u64) -> Option<DatabaseCatalogBootstrapTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).find(|state| state.generation == generation && state.abandoned.load(Ordering::Acquire))?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCatalogBootstrapTerminalHandle { state })
}
//#endregion 🔖️CatalogBootstrapCas

pub static DATABASE_CATALOG_BOOTSTRAP_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Drop for DatabaseCatalogBootstrapState {
    fn drop(&mut self) {
        DATABASE_CATALOG_BOOTSTRAP_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}
