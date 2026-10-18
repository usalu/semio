//! 📬️ Capability open future and terminal handles: the open future, the terminal handle and result, the terminal takers and the live open counter. See [`super`].

use super::*;

//#region 🔖️CapabilityOpen
/// 🔭️ Retained capability-probe future; backend polling occurs only on the process I/O lane.
pub struct DatabaseCapabilityOpenFuture {
    pub state: Arc<DatabaseCapabilityOpenState>,
    pub resolved: bool,
}

impl DatabaseCapabilityOpenFuture {
    pub fn take_completion(&mut self) -> Option<Result<DatabaseCapabilityOpenResult, DbError>> {
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

    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<Self, DatabaseCapabilityOpenRejected> {
        Self::try_prepare(pool, storage, true)
    }

    pub fn try_prepare(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, schedule: bool) -> Result<Self, DatabaseCapabilityOpenRejected> {
        let pool_use = match pool.acquire_use() {
            Ok(pool_use) => pool_use,
            Err(error) => return Err(DatabaseCapabilityOpenRejected { error: Some(DbError::Unavailable(format!("database capability-open WorkerPool use rejected: {error:?}"))), storage: Some(storage) }),
        };
        Self::try_prepare_with_use(pool, pool_use, storage, schedule)
    }

    pub fn try_prepare_with_use(pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>, storage: Arc<db_storage::DbBackend>, schedule: bool) -> Result<Self, DatabaseCapabilityOpenRejected> {
        let admission = match DatabaseCapabilityOpenAdmission::try_claim(DATABASE_CAPABILITY_OPEN_ITEMS, DATABASE_CAPABILITY_OPEN_BYTES) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCapabilityOpenRejected { error: Some(error), storage: Some(storage) }),
        };
        {
            let registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry[admission.slot].is_some() {
                return Err(DatabaseCapabilityOpenRejected { error: Some(DbError::Unavailable("database capability-open terminal slot remained occupied".to_string())), storage: Some(storage) });
            }
        }
        let slot = admission.slot;
        let generation = admission.generation;
        DATABASE_CAPABILITY_OPEN_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let state = Arc::new(DatabaseCapabilityOpenState {
            pool,
            _pool_use: pool_use,
            slot,
            generation,
            admission: Mutex::new(Some(admission)),
            work: Mutex::new(Some(DatabaseCapabilityOpenWork::new(storage))),
            poll_work: Mutex::new(None),
            staged_result: Mutex::new(None),
            terminal_error: Mutex::new(None),
            completion: Mutex::new(None),
            terminal_work: Mutex::new(None),
            terminal_result: Mutex::new(None),
            terminal_result_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_completion: Mutex::new(None),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            lease: std::sync::atomic::AtomicU8::new(0),
            completion_consumed: std::sync::atomic::AtomicBool::new(false),
            polling: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            terminal_checked_out: std::sync::atomic::AtomicBool::new(false),
            phase: std::sync::atomic::AtomicU8::new(DatabaseCapabilityOpenPhase::Handoff as u8),
            progress: std::sync::atomic::AtomicU8::new(DatabaseCapabilityOpenProgress::Admitted as u8),
            #[cfg(any(test, feature = "testkit"))]
            controlled_submit_refusal: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_lease_release_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_owner_transfer_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_publication_before_waker_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_completion_before_wake_hook: Mutex::new(None),
            #[cfg(test)]
            poll_publication_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_submit_hook: Mutex::new(None),
        });
        database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        if schedule {
            state.schedule();
        }
        Ok(Self { state, resolved: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> DatabaseCapabilityOpenProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => DatabaseCapabilityOpenProgress::Admitted,
            1 => DatabaseCapabilityOpenProgress::Scheduled,
            2 => DatabaseCapabilityOpenProgress::Polling,
            3 => DatabaseCapabilityOpenProgress::Pending,
            4 => DatabaseCapabilityOpenProgress::Completed,
            5 => DatabaseCapabilityOpenProgress::Cancelled,
            _ => DatabaseCapabilityOpenProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        self.state.cancel();
    }

    #[cfg(test)]
    pub(super) fn retained_storage_identity(&self) -> Option<usize> {
        self.state
            .work
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
            .map(|work| work.storage_identity)
            .or_else(|| self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|work| work.storage_identity))
            .or_else(|| self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(|work| work.storage_identity))
    }
}

impl Future for DatabaseCapabilityOpenFuture {
    type Output = Result<DatabaseCapabilityOpenResult, DbError>;

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

impl Drop for DatabaseCapabilityOpenFuture {
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

/// 🧯️ Public retained authority for cancellation, saturation, fault, and abandoned-open cleanup.
pub struct DatabaseCapabilityOpenTerminalHandle {
    pub state: Arc<DatabaseCapabilityOpenState>,
}

impl DatabaseCapabilityOpenTerminalHandle {
    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn close_step(&self) -> DatabaseCapabilityOpenCloseStep {
        self.state.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty()
    }

    pub fn take_result(&self) -> Option<DatabaseCapabilityOpenTerminalResult> {
        use std::sync::atomic::Ordering;
        if self.state.terminal_result_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        if self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.terminal_result_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(DatabaseCapabilityOpenTerminalResult { state: self.state.clone(), checked_out: true })
    }

    pub fn resume(self) -> Result<DatabaseCapabilityOpenFuture, Self> {
        use std::sync::atomic::Ordering;
        let Some(lease) = self.state.try_lease(0, false) else { return Err(self) };
        if self.state.polling.load(Ordering::Acquire) || self.state.finished.load(Ordering::Acquire) {
            return Err(self);
        }
        let mut job = self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|(_, job)| (job, 0));
        if job.is_none() {
            let retry = self.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some(retry) = retry {
                let generation = self.state.retry_generation.load(Ordering::Acquire);
                let Some(next) = generation.checked_add(1) else {
                    *self.state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(retry);
                    return Err(self);
                };
                self.state.retry_generation.store(next, Ordering::Release);
                self.state.retry_armed.store(false, Ordering::Release);
                job = Some(retry);
            }
        }
        let work = if job.is_none() { self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() } else { None };
        let result = if job.is_none() && work.is_none() { self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() } else { None };
        if let Some((job, attempt)) = job {
            self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.set_phase(if self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() { DatabaseCapabilityOpenPhase::Poll } else { DatabaseCapabilityOpenPhase::Handoff });
            self.state.completion_consumed.store(false, Ordering::Release);
            self.state.abandoned.store(false, Ordering::Release);
            self.state.cancelled.store(false, Ordering::Release);
            self.state.terminal_checked_out.store(false, Ordering::Release);
            lease.submit_exact(job, attempt);
            return Ok(DatabaseCapabilityOpenFuture { state: self.state.clone(), resolved: false });
        } else if let Some(work) = work {
            *self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.state.set_phase(DatabaseCapabilityOpenPhase::Poll);
        } else if let Some(result) = result {
            *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            self.state.set_phase(DatabaseCapabilityOpenPhase::Terminal);
        } else if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.set_phase(DatabaseCapabilityOpenPhase::Handoff);
        } else if self.state.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            self.state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.state.set_phase(DatabaseCapabilityOpenPhase::Poll);
        } else {
            return Err(self);
        }
        self.state.completion_consumed.store(false, Ordering::Release);
        self.state.abandoned.store(false, Ordering::Release);
        self.state.cancelled.store(false, Ordering::Release);
        self.state.terminal_checked_out.store(false, Ordering::Release);
        if self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(DatabaseCapabilityOpenFuture { state: self.state.clone(), resolved: false })
    }
}

/// 📦️ Shallow checkout of one retained capability-open result owner.
pub struct DatabaseCapabilityOpenTerminalResult {
    pub state: Arc<DatabaseCapabilityOpenState>,
    pub checked_out: bool,
}

impl DatabaseCapabilityOpenTerminalResult {
    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn take(mut self) -> Option<Result<DatabaseCapabilityOpenResult, DbError>> {
        let _lease = self.state.try_lease(0, false)?;
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
        result
    }

    pub fn resume(mut self) -> Result<DatabaseCapabilityOpenFuture, Self> {
        use std::sync::atomic::Ordering;
        let Some(_lease) = self.state.try_lease(0, false) else { return Err(self) };
        if self.state.polling.load(Ordering::Acquire) || self.state.finished.load(Ordering::Acquire) || self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            return Err(self);
        }
        let Some(result) = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            return Err(self);
        };
        *self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        self.state.set_phase(DatabaseCapabilityOpenPhase::Terminal);
        self.state.abandoned.store(false, Ordering::Release);
        self.state.cancelled.store(false, Ordering::Release);
        self.state.completion_consumed.store(false, Ordering::Release);
        self.checked_out = false;
        self.state.terminal_result_checked_out.store(false, Ordering::Release);
        Ok(DatabaseCapabilityOpenFuture { state: self.state.clone(), resolved: false })
    }

    pub fn close_step(&mut self) -> DatabaseCapabilityOpenCloseStep {
        let Some(_lease) = self.state.try_lease(0, false) else {
            return if self.terminal_is_empty() { DatabaseCapabilityOpenCloseStep::Complete } else { DatabaseCapabilityOpenCloseStep::Blocked };
        };
        if let Some(result) = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            self.checked_out = false;
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
            DatabaseCapabilityOpenCloseStep::Progress
        } else {
            DatabaseCapabilityOpenCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}

impl Drop for DatabaseCapabilityOpenTerminalResult {
    fn drop(&mut self) {
        if self.checked_out {
            self.state.terminal_result_checked_out.store(false, std::sync::atomic::Ordering::Release);
        }
    }
}

impl Drop for DatabaseCapabilityOpenTerminalHandle {
    fn drop(&mut self) {
        self.state.terminal_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

/// 🧲️ Takes the exact abandoned capability-open generation without moving its nested owners.
pub fn take_database_capability_open_terminal(generation: u64) -> Option<DatabaseCapabilityOpenTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).find(|state| state.generation == generation && state.abandoned.load(Ordering::Acquire))?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCapabilityOpenTerminalHandle { state })
}

/// 🧲️ Takes the oldest abandoned capability-open authority when its generation was not observed.
pub fn take_next_database_capability_open_terminal() -> Option<DatabaseCapabilityOpenTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).filter(|state| state.abandoned.load(Ordering::Acquire)).min_by_key(|state| state.generation)?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCapabilityOpenTerminalHandle { state })
}
//#endregion 🔖️CapabilityOpen

pub static DATABASE_CAPABILITY_OPEN_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Drop for DatabaseCapabilityOpenState {
    fn drop(&mut self) {
        DATABASE_CAPABILITY_OPEN_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}
