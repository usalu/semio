//! 📬️ Catalog read future and terminal handles: the read future, the terminal handle and result and the live read counter. See [`super`].

use super::*;

//#region 🔖️CatalogRootRead
pub struct DatabaseCatalogReadFuture {
    pub state: Arc<DatabaseCatalogReadState>,
    pub resolved: bool,
}

impl DatabaseCatalogReadFuture {
    pub fn take_completion(&mut self) -> Option<Result<DatabaseCatalogReadResult, DbError>> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()?;
        self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.resolved = true;
        self.state.completion_consumed.store(true, std::sync::atomic::Ordering::Release);
        if result.is_err() {
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        }
        self.state.release_success();
        Some(result)
    }

    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, key: DatabaseCatalogRootKey) -> Result<Self, DatabaseCatalogReadRejected> {
        Self::try_prepare(pool, storage, key, true)
    }

    pub fn try_prepare(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, key: DatabaseCatalogRootKey, schedule: bool) -> Result<Self, DatabaseCatalogReadRejected> {
        let pool_use = match pool.acquire_use() {
            Ok(pool_use) => pool_use,
            Err(error) => return Err(DatabaseCatalogReadRejected { error: Some(DbError::Unavailable(format!("database catalog-read WorkerPool use rejected: {error:?}"))), storage: Some(storage), key: Some(key) }),
        };
        Self::try_prepare_with_use(pool, pool_use, storage, key, schedule)
    }

    pub fn try_prepare_with_use(pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>, storage: Arc<db_storage::DbBackend>, key: DatabaseCatalogRootKey, schedule: bool) -> Result<Self, DatabaseCatalogReadRejected> {
        let admission = match DatabaseCatalogReadAdmission::try_claim() {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCatalogReadRejected { error: Some(error), storage: Some(storage), key: Some(key) }),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        if database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot].is_some() {
            return Err(DatabaseCatalogReadRejected { error: Some(DbError::Unavailable("database catalog-read terminal slot occupied".to_string())), storage: Some(storage), key: Some(key) });
        }
        DATABASE_CATALOG_READ_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let state = Arc::new(DatabaseCatalogReadState {
            pool,
            _pool_use: pool_use,
            slot,
            generation,
            admission: Mutex::new(Some(admission)),
            work: Mutex::new(Some(DatabaseCatalogReadWork::new(storage, key))),
            poll_work: Mutex::new(None),
            staged_result: Mutex::new(None),
            terminal_error: Mutex::new(None),
            completion: Mutex::new(None),
            terminal_work: Mutex::new(None),
            terminal_result: Mutex::new(None),
            terminal_completion: Mutex::new(None),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            lease: std::sync::atomic::AtomicU8::new(0),
            completion_consumed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(0),
            polling: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            terminal_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_result_checked_out: std::sync::atomic::AtomicBool::new(false),
            phase: std::sync::atomic::AtomicU8::new(DatabaseCatalogReadPhase::Handoff as u8),
            progress: std::sync::atomic::AtomicU8::new(DatabaseCatalogReadProgress::Admitted as u8),
            #[cfg(test)]
            controlled_retry_callback_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_submit_refusal: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_owner_transfer_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_completion_before_wake_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_lease_release_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_submit_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_publication_before_waker_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            poll_worker_thread: std::sync::atomic::AtomicBool::new(false),
        });
        database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        if schedule {
            state.schedule();
        }
        Ok(Self { state, resolved: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> DatabaseCatalogReadProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => DatabaseCatalogReadProgress::Admitted,
            1 => DatabaseCatalogReadProgress::Scheduled,
            2 => DatabaseCatalogReadProgress::Polling,
            3 => DatabaseCatalogReadProgress::Pending,
            4 => DatabaseCatalogReadProgress::Completed,
            5 => DatabaseCatalogReadProgress::Cancelled,
            _ => DatabaseCatalogReadProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        self.state.cancel();
    }
}

impl Future for DatabaseCatalogReadFuture {
    type Output = Result<DatabaseCatalogReadResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.take_completion() {
            return std::task::Poll::Ready(result);
        }
        #[cfg(test)]
        if let Some(hook) = self.state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        self.take_completion().map_or(std::task::Poll::Pending, std::task::Poll::Ready)
    }
}

impl Drop for DatabaseCatalogReadFuture {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        if self.resolved {
            return;
        }
        {
            let _completion = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.state.abandoned.store(true, Ordering::Release);
        }
        self.state.cancel();
    }
}

pub struct DatabaseCatalogReadTerminalHandle {
    pub state: Arc<DatabaseCatalogReadState>,
}

impl DatabaseCatalogReadTerminalHandle {
    pub fn close_step(&self) -> DatabaseCatalogReadCloseStep {
        self.state.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty()
    }

    pub fn take_result(&self) -> Option<DatabaseCatalogReadTerminalResult> {
        use std::sync::atomic::Ordering;
        let _lease = self.state.try_lease(0, false)?;
        if self.state.terminal_result_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        if self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.terminal_result_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(DatabaseCatalogReadTerminalResult { state: self.state.clone(), checked_out: true })
    }

    pub fn resume(self) -> Result<DatabaseCatalogReadFuture, Self> {
        use std::sync::atomic::Ordering;
        let Some(lease) = self.state.try_lease(0, false) else { return Err(self) };
        if self.state.finished.load(Ordering::Acquire) || self.state.terminal_result_checked_out.load(Ordering::Acquire) {
            return Err(self);
        }
        let work_live = self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogReadWork::can_poll);
        let poll_live = self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogReadWork::can_poll);
        let terminal_live = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogReadWork::can_poll);
        let result_staged = self.state.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() || self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        if result_staged {
            return Err(self);
        }
        if work_live || poll_live || terminal_live {
            let Some(next) = self.state.retry_generation.load(Ordering::Acquire).checked_add(1) else { return Err(self) };
            self.state.retry_generation.store(next, Ordering::Release);
            self.state.retry_armed.store(false, Ordering::Release);
            let mut job = self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|(_, job)| (job, 0));
            if job.is_none() {
                job = self.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            }
            if terminal_live {
                *self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            }
            self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.set_phase(if work_live { DatabaseCatalogReadPhase::Handoff } else { DatabaseCatalogReadPhase::Poll });
            self.state.completion_consumed.store(false, Ordering::Release);
            self.state.abandoned.store(false, Ordering::Release);
            self.state.cancelled.store(false, Ordering::Release);
            self.state.terminal_checked_out.store(false, Ordering::Release);
            if let Some((job, attempt)) = job {
                lease.submit_exact(job, attempt);
            } else {
                self.state.schedule();
            }
            return Ok(DatabaseCatalogReadFuture { state: self.state.clone(), resolved: false });
        }
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.state.has_retained_submission()
            || self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
        {
            return Err(self);
        }
        let Some(result) = self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return Err(self) };
        *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        self.state.set_phase(DatabaseCatalogReadPhase::Terminal);
        self.state.completion_consumed.store(false, Ordering::Release);
        self.state.abandoned.store(false, Ordering::Release);
        self.state.cancelled.store(false, Ordering::Release);
        self.state.terminal_checked_out.store(false, Ordering::Release);
        Ok(DatabaseCatalogReadFuture { state: self.state.clone(), resolved: false })
    }
}

impl Drop for DatabaseCatalogReadTerminalHandle {
    fn drop(&mut self) {
        self.state.terminal_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub struct DatabaseCatalogReadTerminalResult {
    pub state: Arc<DatabaseCatalogReadState>,
    pub checked_out: bool,
}

impl DatabaseCatalogReadTerminalResult {
    pub fn take(mut self) -> Option<Result<DatabaseCatalogReadResult, DbError>> {
        let _lease = self.state.try_lease(0, false)?;
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub fn resume(mut self) -> Result<DatabaseCatalogReadFuture, Self> {
        use std::sync::atomic::Ordering;
        let Some(_lease) = self.state.try_lease(0, false) else { return Err(self) };
        if self.state.polling.load(Ordering::Acquire) || self.state.finished.load(Ordering::Acquire) || self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            return Err(self);
        }
        let Some(result) = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            return Err(self);
        };
        *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        self.state.set_phase(DatabaseCatalogReadPhase::Terminal);
        self.state.abandoned.store(false, Ordering::Release);
        self.state.cancelled.store(false, Ordering::Release);
        self.state.completion_consumed.store(false, Ordering::Release);
        self.checked_out = false;
        self.state.terminal_result_checked_out.store(false, Ordering::Release);
        Ok(DatabaseCatalogReadFuture { state: self.state.clone(), resolved: false })
    }

    pub fn close_step(&mut self) -> DatabaseCatalogReadCloseStep {
        let Some(_lease) = self.state.try_lease(0, false) else {
            return if self.terminal_is_empty() { DatabaseCatalogReadCloseStep::Complete } else { DatabaseCatalogReadCloseStep::Blocked };
        };
        if let Some(result) = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
            DatabaseCatalogReadCloseStep::Progress
        } else {
            DatabaseCatalogReadCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}

impl Drop for DatabaseCatalogReadTerminalResult {
    fn drop(&mut self) {
        if self.checked_out {
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
    }
}

pub fn take_database_catalog_read_terminal(generation: u64) -> Option<DatabaseCatalogReadTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).find(|state| state.generation == generation && state.abandoned.load(Ordering::Acquire))?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCatalogReadTerminalHandle { state })
}
//#endregion 🔖️CatalogRootRead

pub static DATABASE_CATALOG_READ_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Drop for DatabaseCatalogReadState {
    fn drop(&mut self) {
        DATABASE_CATALOG_READ_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}
