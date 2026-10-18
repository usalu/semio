//! 🧵️ Job state: the driver authority, the terminal owners, the core, the state and its registry. See [`super`].

use super::*;

//#region 🧵️RetainedCompactionJob
pub(super) type DatabaseCompactionExecutionFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCompactionExecution> + Send + 'static>>;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DatabaseCompactionDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

pub(super) struct DatabaseCompactionTerminalOwners {
    pub(super) storage: Option<Arc<db_storage::DbBackend>>,
    pub(super) document: Option<ArtifactId>,
    pub(super) holder: Option<db_io_core::DbIoText>,
    pub(super) result: Option<Result<CompactionReport, DbError>>,
}

impl DatabaseCompactionTerminalOwners {
    pub(super) fn from_execution(execution: DatabaseCompactionExecution) -> Self {
        Self { storage: Some(execution.storage), document: Some(execution.document), holder: Some(execution.holder), result: Some(execution.result) }
    }

    pub(super) fn close_one(&mut self) -> bool {
        if let Some(Ok(report)) = self.result.as_mut() {
            if report.index_reports.close_step() {
                return true;
            }
        }
        if self.result.take().is_some() {
            return true;
        }
        if self.holder.as_mut().is_some_and(db_io_core::DbIoText::close_step) {
            return true;
        }
        if self.holder.take().is_some() {
            return true;
        }
        self.document.take().is_some() || self.storage.take().is_some()
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.document.is_none() && self.holder.is_none() && self.result.is_none()
    }
}

pub(super) struct DatabaseCompactionCore {
    pub(super) future: Option<DatabaseCompactionExecutionFuture>,
    pub(super) output: Option<DatabaseCompactionExecution>,
    pub(super) release_waiting: Option<DatabaseCompactionExecution>,
    pub(super) release_fault: Option<DbError>,
    pub(super) release_retry_fault: Option<DbError>,
    pub(super) quarantined: Option<DatabaseCompactionExecutionFuture>,
    pub(super) release_quarantined: Option<DatabaseCompactionLeaseReleaseFuture>,
    pub(super) panic_release: Option<DatabaseCompactionLeaseReleaseFuture>,
}

pub(super) struct DatabaseCompactionState {
    pub(super) pool: Arc<WorkerPool>,
    pub(super) _pool_use: Arc<WorkerPoolUse>,
    pub(super) slot: usize,
    pub(super) generation: u64,
    pub(super) admission: std::sync::Mutex<Option<DatabaseCompactionAdmission>>,
    pub(super) core: std::sync::Mutex<DatabaseCompactionCore>,
    pub(super) terminal: std::sync::Mutex<Option<DatabaseCompactionTerminalOwners>>,
    pub(super) driver: std::sync::atomic::AtomicU8,
    pub(super) retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub(super) terminal_job: std::sync::Mutex<Option<semio_framework_async::Job>>,
    pub(super) cancelled: Arc<std::sync::atomic::AtomicBool>,
    pub(super) expired: Arc<std::sync::atomic::AtomicBool>,
    pub(super) deadline_ms: std::sync::atomic::AtomicU64,
    pub(super) progress: Arc<std::sync::atomic::AtomicU8>,
    pub(super) abandoned: std::sync::atomic::AtomicBool,
    pub(super) wake_requested: std::sync::atomic::AtomicBool,
    pub(super) callback_close: std::sync::atomic::AtomicBool,
    pub(super) callback_armed: std::sync::atomic::AtomicBool,
    pub(super) release_retry_armed: std::sync::atomic::AtomicBool,
    pub(super) panic_fault: std::sync::atomic::AtomicBool,
    pub(super) panic_retired: std::sync::atomic::AtomicBool,
    pub(super) lease_recovery: Arc<DatabaseCompactionLeaseRecovery>,
    pub(super) waker: std::sync::Mutex<Option<std::task::Waker>>,
}

pub(super) fn database_compaction_registry() -> &'static std::sync::Mutex<[Option<Arc<DatabaseCompactionState>>; DATABASE_COMPACTION_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<[Option<Arc<DatabaseCompactionState>>; DATABASE_COMPACTION_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCompactionState {
    fn wake(self: Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }
}

impl DatabaseCompactionState {
    pub(super) fn current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCompactionAdmission::is_current)
    }

    pub(super) fn observed_generation(&self) -> u64 {
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(self.slot).map_or(0, |slot| slot.generation)
    }

    pub(super) fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let execution_terminal = {
            let core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            core.future.is_none() && core.panic_release.is_none() && core.release_waiting.is_none() && core.release_fault.is_none() && core.release_retry_fault.is_none() && !self.release_retry_armed.load(Ordering::Acquire)
        };
        if self.callback_close.load(Ordering::Acquire) && execution_terminal {
            self.arm_callback_close();
            return;
        }
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Idle as u8, DatabaseCompactionDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next = attempt.checked_add(1).map_or(DATABASE_COMPACTION_RETRY_LIMIT, |value| value.min(DATABASE_COMPACTION_RETRY_LIMIT));
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Queued as u8, DatabaseCompactionDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let state = self.clone();
                    self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
                }
            }
        }
    }

    pub(super) fn retry(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if self.pool.now_ms() >= self.deadline_ms.load(Ordering::Acquire) {
            self.expired.store(true, Ordering::Release);
            self.cancelled.store(true, Ordering::Release);
        }
        if attempt >= DATABASE_COMPACTION_RETRY_LIMIT {
            self.cancelled.store(true, Ordering::Release);
        }
        if self.cancelled.load(Ordering::Acquire) && self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Retry as u8, DatabaseCompactionDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            return;
        }
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Retry as u8, DatabaseCompactionDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
        } else {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        }
    }

    fn arm_release_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.release_retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.release_retry_callback());
    }

    fn release_retry_callback(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.release_retry_armed.store(false, Ordering::Release);
        if self.lease_recovery.released.load(Ordering::Acquire) {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(error) = core.release_retry_fault.take().or_else(|| core.release_fault.take()) {
                drop(core);
                drop(error);
                self.arm_release_retry();
                return;
            }
            if let Some(output) = core.release_waiting.take() {
                core.output = Some(output);
                drop(core);
                if self.abandoned.load(Ordering::Acquire) || self.callback_close.load(Ordering::Acquire) {
                    self.move_output_to_terminal();
                    self.arm_callback_close();
                } else if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
            } else {
                drop(core);
                self.callback_close.store(true, Ordering::Release);
                self.arm_callback_close();
            }
            return;
        }
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(error) = core.release_retry_fault.take() {
            drop(error);
        }
        if core.panic_release.is_none() {
            core.panic_release = self.lease_recovery.release_future();
        }
        let mounted = core.panic_release.is_some();
        drop(core);
        if mounted {
            self.schedule();
        } else {
            self.arm_release_retry();
        }
    }

    pub(super) fn drive_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Queued as u8, DatabaseCompactionDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let pending = self.poll_one();
        let wake = self.wake_requested.swap(false, Ordering::AcqRel);
        self.driver.store(DatabaseCompactionDriverAuthority::Idle as u8, Ordering::Release);
        if pending || wake {
            self.schedule();
        }
    }

    fn poll_one(self: &Arc<Self>) -> bool {
        if !self.current() {
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.progress.store(DatabaseCompactionProgress::Fault as u8, std::sync::atomic::Ordering::Release);
        }
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.future.is_none() {
            let Some(mut release) = core.panic_release.take() else { return false };
            drop(core);
            let waker = std::task::Waker::from(self.clone());
            let mut context = std::task::Context::from_waker(&waker);
            return match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| release.as_mut().poll(&mut context))) {
                Ok(std::task::Poll::Pending) => {
                    self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).panic_release = Some(release);
                    true
                }
                Ok(std::task::Poll::Ready(Ok(()))) => {
                    self.arm_release_retry();
                    false
                }
                Ok(std::task::Poll::Ready(Err(error))) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if core.release_fault.is_none() {
                        core.release_fault = Some(error);
                    } else {
                        core.release_retry_fault = Some(error);
                    }
                    drop(core);
                    self.arm_release_retry();
                    false
                }
                Err(_) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    core.release_quarantined = Some(release);
                    core.panic_release = self.lease_recovery.retry_release_after_panic();
                    if core.panic_release.is_none() {
                        self.callback_close.store(true, std::sync::atomic::Ordering::Release);
                    }
                    true
                }
            };
        }
        drop(core);
        let mut future = {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match core.future.take() {
                Some(future) => future,
                None => return false,
            }
        };
        let waker = std::task::Waker::from(self.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match polled {
            Ok(std::task::Poll::Pending) => {
                core.future = Some(future);
                true
            }
            Ok(std::task::Poll::Ready(output)) => {
                if !self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
                    core.release_waiting = Some(output);
                    drop(core);
                    self.arm_release_retry();
                    return false;
                }
                core.output = Some(output);
                drop(core);
                if self.abandoned.load(std::sync::atomic::Ordering::Acquire) || self.callback_close.load(std::sync::atomic::Ordering::Acquire) {
                    self.move_output_to_terminal();
                    self.arm_callback_close();
                } else if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
                false
            }
            Err(_) => {
                core.quarantined = Some(future);
                core.panic_release = if self.lease_recovery.releasing.load(std::sync::atomic::Ordering::Acquire) && !self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
                    self.lease_recovery.retry_release_after_panic()
                } else {
                    self.lease_recovery.release_future()
                };
                let release_pending = core.panic_release.is_some();
                self.panic_fault.store(true, std::sync::atomic::Ordering::Release);
                self.progress.store(DatabaseCompactionProgress::Fault as u8, std::sync::atomic::Ordering::Release);
                if !release_pending {
                    self.callback_close.store(true, std::sync::atomic::Ordering::Release);
                }
                true
            }
        }
    }

    fn move_output_to_terminal(&self) {
        let output = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take();
        if let Some(output) = output {
            *self.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners::from_execution(output));
        }
    }

    fn drive_close_claimed(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        let pending = if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            true
        } else {
            self.move_output_to_terminal();
            self.close_terminal_one()
        };
        self.driver.store(DatabaseCompactionDriverAuthority::Idle as u8, Ordering::Release);
        if pending || !self.terminal_is_empty() {
            self.arm_callback_close();
        } else {
            self.callback_close.store(false, Ordering::Release);
        }
    }

    fn close_terminal_one(&self) -> bool {
        if self.panic_fault.load(std::sync::atomic::Ordering::Acquire) && self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(quarantine) = core.release_quarantined.take() {
                drop(quarantine);
                return true;
            }
            if let Some(quarantine) = core.quarantined.take() {
                drop(quarantine);
                return true;
            }
        }
        let mut terminal = self.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.as_mut().is_some_and(DatabaseCompactionTerminalOwners::close_one) {
            return true;
        }
        let terminal_released = terminal.as_ref().is_some_and(DatabaseCompactionTerminalOwners::terminal_is_empty);
        if terminal_released {
            terminal.take();
        }
        drop(terminal);
        if terminal_released {
            self.release_terminal();
        }
        if self.panic_fault.load(std::sync::atomic::Ordering::Acquire) && self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
            let core_empty = {
                let core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                core.future.is_none()
                    && core.output.is_none()
                    && core.release_waiting.is_none()
                    && core.release_fault.is_none()
                    && core.release_retry_fault.is_none()
                    && core.quarantined.is_none()
                    && core.release_quarantined.is_none()
                    && core.panic_release.is_none()
            };
            if core_empty {
                self.release_terminal();
                self.panic_retired.store(true, std::sync::atomic::Ordering::Release);
                if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
            }
        }
        false
    }

    pub(super) fn arm_callback_close(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.callback_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.callback_close_one());
    }

    fn callback_close_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.callback_armed.store(false, Ordering::Release);
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Idle as u8, DatabaseCompactionDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            self.arm_callback_close();
            return;
        }
        self.drive_close_claimed();
    }

    pub(super) fn release_terminal(&self) {
        let mut registry = database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
        drop(registry);
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        let core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let terminal_empty = self.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_none_or(DatabaseCompactionTerminalOwners::terminal_is_empty);
        core.future.is_none()
            && core.output.is_none()
            && core.release_waiting.is_none()
            && core.release_fault.is_none()
            && core.release_retry_fault.is_none()
            && core.quarantined.is_none()
            && core.release_quarantined.is_none()
            && core.panic_release.is_none()
            && !self.release_retry_armed.load(std::sync::atomic::Ordering::Acquire)
            && terminal_empty
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}
//#endregion 🧵️RetainedCompactionJob
