//! 🔭️ Db engine retained capability open operation.

use db_ids::DbError;
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_durability::DurabilityClass;
#[cfg(test)]
use db_engine_base::test_worker_pool;

//#region 🔖️CapabilityOpen
const DATABASE_CAPABILITY_OPEN_SLOTS: usize = 64;

const DATABASE_CAPABILITY_OPEN_ITEMS: u64 = 8;

const DATABASE_CAPABILITY_OPEN_BYTES: u64 = 16 * 1024;

const DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS: u64 = DATABASE_CAPABILITY_OPEN_ITEMS * DATABASE_CAPABILITY_OPEN_SLOTS as u64;

const DATABASE_CAPABILITY_OPEN_TOTAL_BYTES: u64 = DATABASE_CAPABILITY_OPEN_BYTES * DATABASE_CAPABILITY_OPEN_SLOTS as u64;

const DATABASE_CAPABILITY_OPEN_RETRY_MS: u64 = 1;

const DATABASE_CAPABILITY_OPEN_RETRY_LIMIT: u8 = 8;

const DATABASE_CAPABILITY_OPEN_ACTIVE: u8 = 1;

const DATABASE_CAPABILITY_OPEN_QUEUED: u8 = 2;

const DATABASE_CAPABILITY_OPEN_CLOSED: u8 = 4;

#[derive(Clone, Copy)]
struct DatabaseCapabilityOpenAdmissionSlot {
    pub generation: u64,
    pub items: u64,
    pub bytes: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CAPABILITY_OPEN_SLOT: DatabaseCapabilityOpenAdmissionSlot = DatabaseCapabilityOpenAdmissionSlot { generation: 0, items: 0, bytes: 0, occupied: false };

struct DatabaseCapabilityOpenAdmissionState {
    pub slots: [DatabaseCapabilityOpenAdmissionSlot; DATABASE_CAPABILITY_OPEN_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCapabilityOpenAdmissionState {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CAPABILITY_OPEN_SLOT; DATABASE_CAPABILITY_OPEN_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    pub fn try_claim(&mut self, items: u64, bytes: u64) -> Result<(usize, u64), DbError> {
        if items == 0 || items > DATABASE_CAPABILITY_OPEN_ITEMS {
            return Err(DbError::LimitExceeded("database capability-open item credit"));
        }
        if bytes == 0 || bytes > DATABASE_CAPABILITY_OPEN_BYTES {
            return Err(DbError::LimitExceeded("database capability-open byte credit"));
        }
        let Some(slot) = self.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("database capability-open item capacity exhausted".to_string()));
        };
        if self.items.checked_add(items).is_none_or(|next| next > DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS) {
            return Err(DbError::Unavailable("database capability-open aggregate item capacity exhausted".to_string()));
        }
        if self.bytes.checked_add(bytes).is_none_or(|next| next > DATABASE_CAPABILITY_OPEN_TOTAL_BYTES) {
            return Err(DbError::Unavailable("database capability-open aggregate byte capacity exhausted".to_string()));
        }
        let generation = self.next_generation;
        let Some(next_generation) = generation.checked_add(1) else {
            return Err(DbError::LimitExceeded("database capability-open generation"));
        };
        self.next_generation = next_generation;
        self.slots[slot] = DatabaseCapabilityOpenAdmissionSlot { generation, items, bytes, occupied: true };
        self.items += items;
        self.bytes += bytes;
        Ok((slot, generation))
    }

    pub fn release(&mut self, slot: usize, generation: u64, items: u64, bytes: u64) -> bool {
        let Some(entry) = self.slots.get_mut(slot) else {
            return false;
        };
        if !entry.occupied || entry.generation != generation || entry.items != items || entry.bytes != bytes {
            return false;
        }
        *entry = EMPTY_DATABASE_CAPABILITY_OPEN_SLOT;
        self.items = self.items.checked_sub(items).expect("database capability-open item credit underflow");
        self.bytes = self.bytes.checked_sub(bytes).expect("database capability-open byte credit underflow");
        true
    }
}

static DATABASE_CAPABILITY_OPEN_ADMISSION: Mutex<DatabaseCapabilityOpenAdmissionState> =
    Mutex::new(DatabaseCapabilityOpenAdmissionState { slots: [EMPTY_DATABASE_CAPABILITY_OPEN_SLOT; DATABASE_CAPABILITY_OPEN_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseCapabilityOpenAdmission {
    pub slot: usize,
    pub generation: u64,
    pub items: u64,
    pub bytes: u64,
}

impl DatabaseCapabilityOpenAdmission {
    pub fn try_claim(items: u64, bytes: u64) -> Result<Self, DbError> {
        let mut state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let (slot, generation) = state.try_claim(items, bytes)?;
        Ok(Self { slot, generation, items, bytes })
    }

    pub fn is_current(&self) -> bool {
        let state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.slots.get(self.slot).is_some_and(|entry| entry.occupied && entry.generation == self.generation && entry.items == self.items && entry.bytes == self.bytes)
    }
}

impl Drop for DatabaseCapabilityOpenAdmission {
    fn drop(&mut self) {
        let mut state = DATABASE_CAPABILITY_OPEN_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.release(self.slot, self.generation, self.items, self.bytes);
    }
}

/// 🧭️ Progress of one retained database capability probe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCapabilityOpenProgress {
    Admitted,
    Scheduled,
    Polling,
    Pending,
    Completed,
    Cancelled,
    Fault,
}

/// 🧹️ One bounded public terminal-cleanup opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCapabilityOpenCloseStep {
    Progress,
    Blocked,
    Complete,
}

/// 📦️ Exact storage owner and capability scalar returned by the retained probe.
pub struct DatabaseCapabilityOpenResult {
    pub storage: Arc<db_storage::DbBackend>,
    pub capabilities: db_io_core::StorageCapabilities,
}

impl DatabaseCapabilityOpenResult {
    pub fn into_parts(self) -> (Arc<db_storage::DbBackend>, db_io_core::StorageCapabilities) {
        (self.storage, self.capabilities)
    }
}

/// ↩️ Admission rejection that returns the exact storage owner unchanged.
pub struct DatabaseCapabilityOpenRejected {
    pub error: Option<DbError>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
}

impl DatabaseCapabilityOpenRejected {
    pub fn error(&self) -> &DbError {
        self.error.as_ref().expect("database capability-open rejection error owner missing")
    }

    pub fn take_storage(&mut self) -> Option<Arc<db_storage::DbBackend>> {
        self.storage.take()
    }

    pub fn retry(mut self, pool: Arc<WorkerPool>) -> Result<DatabaseCapabilityOpenFuture, Self> {
        let Some(storage) = self.storage.take() else {
            return Err(self);
        };
        match DatabaseCapabilityOpenFuture::try_submit(pool, storage) {
            Ok(future) => Ok(future),
            Err(rejected) => Err(rejected),
        }
    }

    pub fn close_step(&mut self) -> DatabaseCapabilityOpenCloseStep {
        if let Some(storage) = self.storage.take() {
            drop(storage);
            DatabaseCapabilityOpenCloseStep::Progress
        } else {
            DatabaseCapabilityOpenCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none()
    }

    pub fn into_error_after_close(mut self) -> Result<DbError, Self> {
        if self.storage.is_some() {
            return Err(self);
        }
        Ok(self.error.take().expect("database capability-open rejection error owner missing"))
    }

    pub fn close_and_take_error(mut self) -> DbError {
        if let Some(storage) = self.storage.take() {
            drop(storage);
        }
        self.error.take().expect("database capability-open rejection error owner missing")
    }

    pub fn into_parts(mut self) -> (DbError, Arc<db_storage::DbBackend>) {
        (self.error.take().expect("database capability-open rejection error owner missing"), self.storage.take().expect("database capability-open rejected storage owner missing"))
    }
}

impl std::fmt::Debug for DatabaseCapabilityOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCapabilityOpenRejected").field("error", &self.error).field("storage", &self.storage.as_ref().map(Arc::as_ptr)).finish()
    }
}

type DatabaseCapabilityOpenBackendFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCapabilityOpenResult> + Send + 'static>>;

struct DatabaseCapabilityOpenWork {
    pub future: Option<DatabaseCapabilityOpenBackendFuture>,
    #[cfg(any(test, feature = "testkit"))]
    pub storage_identity: usize,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCapabilityOpenPhase {
    Handoff,
    Poll,
    RetainWork,
    DrainWork,
    ReleaseWork,
    RetainResult,
    Publish,
    Terminal,
}

impl DatabaseCapabilityOpenPhase {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Handoff,
            1 => Self::Poll,
            2 => Self::RetainWork,
            3 => Self::DrainWork,
            4 => Self::ReleaseWork,
            5 => Self::RetainResult,
            6 => Self::Publish,
            _ => Self::Terminal,
        }
    }
}

impl DatabaseCapabilityOpenWork {
    pub fn new(storage: Arc<db_storage::DbBackend>) -> Self {
        #[cfg(any(test, feature = "testkit"))]
        let storage_identity = Arc::as_ptr(&storage) as usize;
        let future = Box::pin(async move {
            let capabilities = storage.capabilities().await;
            DatabaseCapabilityOpenResult { storage, capabilities }
        });
        Self {
            future: Some(future),
            #[cfg(any(test, feature = "testkit"))]
            storage_identity,
        }
    }

    pub fn poll(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<DatabaseCapabilityOpenResult> {
        self.future.as_mut().map_or(std::task::Poll::Pending, |future| future.as_mut().poll(context))
    }

    pub fn close_step(&mut self) -> bool {
        self.future.take().is_some()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.future.is_none()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn controlled(future: DatabaseCapabilityOpenBackendFuture, storage_identity: usize) -> Self {
        Self { future: Some(future), storage_identity }
    }
}

struct DatabaseCapabilityOpenState {
    pub pool: Arc<WorkerPool>,
    pub _pool_use: Arc<WorkerPoolUse>,
    pub slot: usize,
    pub generation: u64,
    pub admission: Mutex<Option<DatabaseCapabilityOpenAdmission>>,
    pub work: Mutex<Option<DatabaseCapabilityOpenWork>>,
    pub poll_work: Mutex<Option<DatabaseCapabilityOpenWork>>,
    pub staged_result: Mutex<Option<DatabaseCapabilityOpenResult>>,
    pub terminal_error: Mutex<Option<(DbError, DatabaseCapabilityOpenProgress)>>,
    pub completion: Mutex<Option<Result<DatabaseCapabilityOpenResult, DbError>>>,
    pub terminal_work: Mutex<Option<DatabaseCapabilityOpenWork>>,
    pub terminal_result: Mutex<Option<Result<DatabaseCapabilityOpenResult, DbError>>>,
    pub terminal_result_checked_out: std::sync::atomic::AtomicBool,
    pub terminal_completion: Mutex<Option<Result<DatabaseCapabilityOpenResult, DbError>>>,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub retry_armed: std::sync::atomic::AtomicBool,
    pub retry_generation: std::sync::atomic::AtomicU64,
    pub lease: std::sync::atomic::AtomicU8,
    pub completion_consumed: std::sync::atomic::AtomicBool,
    pub polling: std::sync::atomic::AtomicBool,
    pub wake_requested: std::sync::atomic::AtomicBool,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub terminal_checked_out: std::sync::atomic::AtomicBool,
    pub phase: std::sync::atomic::AtomicU8,
    pub progress: std::sync::atomic::AtomicU8,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_submit_refusal: Mutex<Option<semio_framework_async::WorkerSubmitErrorKind>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_lease_release_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_owner_transfer_hook: Mutex<Option<Box<dyn FnOnce(usize) + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_publication_before_waker_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_completion_before_wake_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(test)]
    poll_publication_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_submit_hook: Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
}

struct DatabaseCapabilityOpenLease {
    pub state: Arc<DatabaseCapabilityOpenState>,
    pub held: bool,
    pub finalize: bool,
}

impl DatabaseCapabilityOpenLease {
    pub fn retire(&mut self) {
        self.state.lease.store(DATABASE_CAPABILITY_OPEN_CLOSED, std::sync::atomic::Ordering::Release);
        self.held = false;
        self.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.state.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.state.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.state.generation) {
            registry[self.state.slot] = None;
        }
    }

    pub fn submit_exact(mut self, job: semio_framework_async::Job, attempt: u8) {
        self.state.lease.store(DATABASE_CAPABILITY_OPEN_QUEUED, std::sync::atomic::Ordering::Release);
        self.held = false;
        self.state.submit_exact(job, attempt);
    }
}

impl Drop for DatabaseCapabilityOpenLease {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        if !self.held {
            return;
        }
        if self.state.abandoned.load(Ordering::Acquire) {
            let mut completion = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(result) = completion.take() {
                *self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            }
        }
        if self.finalize && self.state.completion_consumed.load(Ordering::Acquire) && self.state.roots_are_empty() {
            self.retire();
            return;
        }
        #[cfg(test)]
        if let Some(hook) = self.state.controlled_lease_release_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        let previous = self.state.lease.fetch_and(!DATABASE_CAPABILITY_OPEN_ACTIVE, Ordering::AcqRel);
        self.held = false;
        if previous & DATABASE_CAPABILITY_OPEN_QUEUED != 0 {
            self.state.submit_drive_job();
        }
    }
}

struct DatabaseCapabilityOpenWake {
    pub state: std::sync::Weak<DatabaseCapabilityOpenState>,
    pub generation: u64,
}

pub fn database_capability_open_registry() -> &'static Mutex<[Option<Arc<DatabaseCapabilityOpenState>>; DATABASE_CAPABILITY_OPEN_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<Mutex<[Option<Arc<DatabaseCapabilityOpenState>>; DATABASE_CAPABILITY_OPEN_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCapabilityOpenWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let Some(state) = self.state.upgrade() else {
            return;
        };
        if state.generation != self.generation || !state.is_current() || state.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if !state.polling.load(std::sync::atomic::Ordering::Acquire) {
            state.schedule();
        }
    }
}

impl DatabaseCapabilityOpenState {
    pub fn try_lease(self: &Arc<Self>, expected: u8, finalize: bool) -> Option<DatabaseCapabilityOpenLease> {
        self.lease.compare_exchange(expected, DATABASE_CAPABILITY_OPEN_ACTIVE, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok()?;
        Some(DatabaseCapabilityOpenLease { state: self.clone(), held: true, finalize })
    }

    pub fn request_drive(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let mut observed = self.lease.load(Ordering::Acquire);
        loop {
            if observed & (DATABASE_CAPABILITY_OPEN_CLOSED | DATABASE_CAPABILITY_OPEN_QUEUED) != 0 {
                return;
            }
            match self.lease.compare_exchange_weak(observed, observed | DATABASE_CAPABILITY_OPEN_QUEUED, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => {
                    if observed == 0 {
                        self.submit_drive_job();
                    }
                    return;
                }
                Err(current) => observed = current,
            }
        }
    }

    pub fn cancel(self: &Arc<Self>) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(DatabaseCapabilityOpenProgress::Cancelled);
        if let Some(_lease) = self.try_lease(0, false) {
            self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
        } else {
            self.request_drive();
        }
    }

    pub fn phase(&self) -> DatabaseCapabilityOpenPhase {
        DatabaseCapabilityOpenPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCapabilityOpenPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn set_progress(&self, progress: DatabaseCapabilityOpenProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCapabilityOpenAdmission::is_current)
    }

    pub fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    pub fn complete(&self, result: Result<DatabaseCapabilityOpenResult, DbError>, progress: DatabaseCapabilityOpenProgress) {
        self.set_progress(progress);
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *completion = Some(result);
            drop(completion);
            #[cfg(test)]
            if let Some(hook) = self.controlled_completion_before_wake_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                hook();
            }
            self.wake_waiter();
        }
    }

    pub fn schedule(self: &Arc<Self>) {
        self.request_drive();
    }

    pub fn schedule_cleanup(self: &Arc<Self>) {
        self.request_drive();
    }

    pub fn submit_drive_job(self: &Arc<Self>) {
        let state = self.clone();
        let generation = self.generation;
        let job: semio_framework_async::Job = Box::new(move || state.drive_one(generation));
        #[cfg(test)]
        let submit = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        #[cfg(test)]
        let job = if let Some(submit) = submit {
            match submit(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        self.submit_exact(job, 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        #[cfg(test)]
        if let Some(kind) = self.controlled_submit_refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.handle_submit_refusal(kind, job, attempt);
            return;
        }
        if let Err(error) = self.pool.try_submit(Lane::Io, job) {
            self.handle_submit_refusal(error.kind(), error.into_job(), attempt);
        }
    }

    pub fn handle_submit_refusal(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind, job: semio_framework_async::Job, attempt: u8) {
        let mut lease = self.try_lease(DATABASE_CAPABILITY_OPEN_QUEUED, true).expect("exact capability refusal owns the queued turn");
        if self.completion_consumed.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() {
            drop(job);
            lease.retire();
            return;
        }
        match worker_submit_retry_attempt(kind, attempt, DATABASE_CAPABILITY_OPEN_RETRY_LIMIT) {
            Some(next) => {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, next));
                self.arm_retry();
            }
            None => {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
                self.complete(Err(DbError::Unavailable(format!("database capability-open WorkerPool submission failed: {kind:?}"))), DatabaseCapabilityOpenProgress::Fault);
            }
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.advance_retry_generation_once();
    }

    fn advance_retry_generation_once(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let current = self.retry_generation.load(Ordering::Acquire);
        self.advance_retry_generation_observed_once(current);
    }

    fn advance_retry_generation_observed_once(self: &Arc<Self>, current: u64) {
        use std::sync::atomic::Ordering;
        let Some(generation) = current.checked_add(1) else {
            self.retry_armed.store(false, Ordering::Release);
            self.set_progress(DatabaseCapabilityOpenProgress::Fault);
            self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
            self.complete(Err(DbError::LimitExceeded("database capability-open retry generation")), DatabaseCapabilityOpenProgress::Fault);
            return;
        };
        if let Err(observed) = self.retry_generation.compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire) {
            let state = self.clone();
            self.pool.callback_at(self.pool.now_ms().saturating_add(DATABASE_CAPABILITY_OPEN_RETRY_MS), move || {
                if state.retry_armed.load(Ordering::Acquire) && observed == state.retry_generation.load(Ordering::Acquire) {
                    state.advance_retry_generation_observed_once(observed);
                }
            });
            return;
        }
        self.retry_submission_at(generation);
    }

    pub fn retry_submission_at(self: &Arc<Self>, generation: u64) {
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(DATABASE_CAPABILITY_OPEN_RETRY_MS), move || state.retry_submission_once(generation));
    }

    pub fn retry_submission_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.retry_generation.load(Ordering::Acquire) || !self.retry_armed.load(Ordering::Acquire) {
            return;
        }
        let Some(lease) = self.try_lease(0, true) else {
            if !self.finished.load(Ordering::Acquire) {
                self.retry_submission_at(generation);
            }
            return;
        };
        if generation != self.retry_generation.load(Ordering::Acquire) || !self.retry_armed.load(Ordering::Acquire) {
            return;
        }
        self.retry_armed.store(false, Ordering::Release);
        let retry = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retry {
            if self.cancelled.load(Ordering::Acquire) || !self.is_current() {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            } else {
                lease.submit_exact(job, attempt);
            }
        }
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        let Some(_lease) = self.try_lease(DATABASE_CAPABILITY_OPEN_QUEUED, true) else { return };
        if !self.is_current() && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCapabilityOpenPhase::Terminal {
            self.stage_terminal(DbError::Unavailable("database capability-open generation became stale".to_string()), DatabaseCapabilityOpenProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCapabilityOpenPhase::Terminal {
            self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            return;
        }
        match self.phase() {
            DatabaseCapabilityOpenPhase::Handoff => {
                let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
                    self.stage_terminal(DbError::Unavailable("database capability-open handoff owner missing".to_string()), DatabaseCapabilityOpenProgress::Fault);
                    return;
                };
                #[cfg(test)]
                if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    hook(work.storage_identity);
                }
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.set_phase(DatabaseCapabilityOpenPhase::Poll);
                self.schedule();
            }
            DatabaseCapabilityOpenPhase::Poll => self.poll_backend_once(generation),
            DatabaseCapabilityOpenPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    #[cfg(test)]
                    if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        hook(work.storage_identity);
                    }
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                }
                self.set_phase(DatabaseCapabilityOpenPhase::DrainWork);
                self.schedule_cleanup();
            }
            DatabaseCapabilityOpenPhase::DrainWork => {
                let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let progressed = terminal.as_mut().is_some_and(DatabaseCapabilityOpenWork::close_step);
                let empty = terminal.as_ref().is_some_and(DatabaseCapabilityOpenWork::terminal_is_empty);
                drop(terminal);
                if progressed || empty {
                    self.set_phase(DatabaseCapabilityOpenPhase::ReleaseWork);
                    self.schedule_cleanup();
                } else {
                    self.stage_terminal(DbError::Unavailable("database capability-open retained work failed to release".to_string()), DatabaseCapabilityOpenProgress::Fault);
                }
            }
            DatabaseCapabilityOpenPhase::ReleaseWork => {
                let terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if terminal.as_ref().is_some_and(DatabaseCapabilityOpenWork::terminal_is_empty) {
                    self.set_phase(if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                        DatabaseCapabilityOpenPhase::RetainResult
                    } else {
                        DatabaseCapabilityOpenPhase::Publish
                    });
                    self.schedule_cleanup();
                } else {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = terminal;
                    self.stage_terminal(DbError::Unavailable("database capability-open terminal work witness failed".to_string()), DatabaseCapabilityOpenProgress::Fault);
                }
            }
            DatabaseCapabilityOpenPhase::RetainResult => {
                if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
                }
                self.set_phase(DatabaseCapabilityOpenPhase::Publish);
                self.schedule_cleanup();
            }
            DatabaseCapabilityOpenPhase::Publish => self.publish_staged(),
            DatabaseCapabilityOpenPhase::Terminal => {}
        }
    }

    fn publish_poll_terminal(&self, error: DbError, progress: DatabaseCapabilityOpenProgress) {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        self.set_progress(progress);
        let mut terminal_error = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal_error.is_none() {
            *terminal_error = Some((error, progress));
        }
        drop(terminal_error);
        self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
    }

    fn poll_terminal_if_cancelled_or_stale(&self) -> bool {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            self.publish_poll_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            true
        } else if !self.is_current() {
            self.publish_poll_terminal(DbError::Unavailable("database capability-open generation became stale during polling".to_string()), DatabaseCapabilityOpenProgress::Fault);
            true
        } else {
            false
        }
    }

    pub fn release_terminal_poll(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.polling.store(false, Ordering::Release);
        self.wake_requested.store(false, Ordering::Release);
        self.schedule_cleanup();
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            self.stage_terminal(DbError::Unavailable("database capability-open poll owner missing".to_string()), DatabaseCapabilityOpenProgress::Fault);
            return;
        };
        #[cfg(test)]
        if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook(work.storage_identity);
        }
        self.polling.store(true, Ordering::Release);
        self.set_progress(DatabaseCapabilityOpenProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCapabilityOpenWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        #[cfg(test)]
        if let Some(hook) = self.poll_publication_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if self.poll_terminal_if_cancelled_or_stale() {
                    self.release_terminal_poll();
                } else {
                    self.set_progress(DatabaseCapabilityOpenProgress::Pending);
                    self.polling.store(false, Ordering::Release);
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        self.schedule();
                    }
                }
            }
            Ok(std::task::Poll::Ready(output)) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(output);
                self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
                if self.poll_terminal_if_cancelled_or_stale() {
                    self.release_terminal_poll();
                } else {
                    self.polling.store(false, Ordering::Release);
                    self.wake_requested.store(false, Ordering::Release);
                    self.schedule_cleanup();
                }
            }
            Err(_) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_poll_terminal(DbError::Unavailable("database capability-open backend poll panicked".to_string()), DatabaseCapabilityOpenProgress::Fault);
                self.release_terminal_poll();
            }
        }
    }

    pub fn stage_terminal(self: &Arc<Self>, error: DbError, progress: DatabaseCapabilityOpenProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        {
            let mut terminal_error = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal_error.is_none() {
                *terminal_error = Some((error, progress));
            }
        }
        let has_work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() || self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let has_terminal_work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let phase = if has_work {
            DatabaseCapabilityOpenPhase::RetainWork
        } else if has_terminal_work {
            match self.phase() {
                DatabaseCapabilityOpenPhase::ReleaseWork => DatabaseCapabilityOpenPhase::ReleaseWork,
                _ => DatabaseCapabilityOpenPhase::DrainWork,
            }
        } else if self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            DatabaseCapabilityOpenPhase::RetainResult
        } else {
            DatabaseCapabilityOpenPhase::Publish
        };
        self.set_phase(phase);
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.schedule_cleanup();
        } else {
            self.complete(Err(DbError::Closed), progress);
        }
    }

    pub fn publish_staged(self: &Arc<Self>) {
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
                return;
            }
            if !self.is_current() {
                self.stage_terminal(DbError::Unavailable("database capability-open generation became stale before publication".to_string()), DatabaseCapabilityOpenProgress::Fault);
                return;
            }
        }
        if let Some((error, progress)) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.complete(Err(error), progress);
        } else if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.complete(Ok(result), DatabaseCapabilityOpenProgress::Completed);
        } else {
            self.complete(Err(DbError::Unavailable("database capability-open publication owner missing".to_string())), DatabaseCapabilityOpenProgress::Fault);
        }
        self.set_phase(DatabaseCapabilityOpenPhase::Terminal);
    }

    pub fn roots_are_empty(&self) -> bool {
        self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.terminal_result_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    pub fn close_step(self: &Arc<Self>) -> DatabaseCapabilityOpenCloseStep {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        let Some(mut lease) = self.try_lease(0, false) else {
            return if self.terminal_is_empty() { DatabaseCapabilityOpenCloseStep::Complete } else { DatabaseCapabilityOpenCloseStep::Blocked };
        };
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.retry_armed.load(Ordering::Acquire) {
            let current = self.retry_generation.load(Ordering::Acquire);
            if let Some(next) = current.checked_add(1) {
                self.retry_generation.store(next, Ordering::Release);
                self.retry_armed.store(false, Ordering::Release);
                return DatabaseCapabilityOpenCloseStep::Progress;
            }
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if self.polling.load(Ordering::Acquire) {
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        {
            let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(work) = terminal.as_mut() {
                if work.close_step() {
                    return DatabaseCapabilityOpenCloseStep::Progress;
                }
                if work.terminal_is_empty() {
                    terminal.take();
                    return DatabaseCapabilityOpenCloseStep::Progress;
                }
            }
        }
        if self.terminal_result_checked_out.load(Ordering::Acquire) {
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            lease.retire();
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.roots_are_empty() {
            lease.retire();
            DatabaseCapabilityOpenCloseStep::Complete
        } else {
            DatabaseCapabilityOpenCloseStep::Blocked
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire) && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.roots_are_empty()
    }

    pub fn release_success(self: &Arc<Self>) {
        let Some(mut lease) = self.try_lease(0, false) else {
            self.request_drive();
            return;
        };
        if self.completion_consumed.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() {
            lease.retire();
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn retained_owner_count(&self) -> usize {
        usize::from(self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
    }
}

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
    fn retained_storage_identity(&self) -> Option<usize> {
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
