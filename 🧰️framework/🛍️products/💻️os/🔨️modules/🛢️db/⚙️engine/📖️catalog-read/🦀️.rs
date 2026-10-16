//! 📖️ Db engine retained catalog root read operation.

use db_durability::EpochFence;
use db_ids::DbError;
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::test_worker_pool;

//#region 🔖️CatalogRootRead
const DATABASE_CATALOG_READ_SLOTS: usize = 64;

const DATABASE_CATALOG_READ_ITEMS: u64 = 8;

const DATABASE_CATALOG_READ_BYTES: u64 = 64 * 1024;

const DATABASE_CATALOG_READ_TOTAL_ITEMS: u64 = DATABASE_CATALOG_READ_ITEMS * DATABASE_CATALOG_READ_SLOTS as u64;

const DATABASE_CATALOG_READ_TOTAL_BYTES: u64 = DATABASE_CATALOG_READ_BYTES * DATABASE_CATALOG_READ_SLOTS as u64;

const DATABASE_CATALOG_READ_RETRY_LIMIT: u8 = 8;

const DATABASE_CATALOG_READ_ACTIVE: u8 = 1;

const DATABASE_CATALOG_READ_QUEUED: u8 = 2;

const DATABASE_CATALOG_READ_CLOSED: u8 = 4;

#[derive(Clone, Copy)]
struct DatabaseCatalogReadAdmissionSlot {
    pub generation: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CATALOG_READ_SLOT: DatabaseCatalogReadAdmissionSlot = DatabaseCatalogReadAdmissionSlot { generation: 0, occupied: false };

struct DatabaseCatalogReadAdmissionState {
    pub slots: [DatabaseCatalogReadAdmissionSlot; DATABASE_CATALOG_READ_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCatalogReadAdmissionState {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CATALOG_READ_SLOT; DATABASE_CATALOG_READ_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    pub fn try_claim(&mut self, items: u64, bytes: u64) -> Result<(usize, u64), DbError> {
        if items == 0 || items > DATABASE_CATALOG_READ_ITEMS {
            return Err(DbError::LimitExceeded("database catalog-read item credit"));
        }
        if bytes == 0 || bytes > DATABASE_CATALOG_READ_BYTES {
            return Err(DbError::LimitExceeded("database catalog-read byte credit"));
        }
        let slot = self.slots.iter().position(|entry| !entry.occupied).ok_or_else(|| DbError::Unavailable("database catalog-read slot capacity exhausted".to_string()))?;
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded("database catalog-read aggregate items"))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database catalog-read aggregate bytes"))?;
        if next_items > DATABASE_CATALOG_READ_TOTAL_ITEMS || next_bytes > DATABASE_CATALOG_READ_TOTAL_BYTES {
            return Err(DbError::Unavailable("database catalog-read aggregate capacity exhausted".to_string()));
        }
        let generation = self.next_generation;
        self.next_generation = generation.checked_add(1).ok_or(DbError::LimitExceeded("database catalog-read generation"))?;
        self.slots[slot] = DatabaseCatalogReadAdmissionSlot { generation, occupied: true };
        self.items = next_items;
        self.bytes = next_bytes;
        Ok((slot, generation))
    }

    pub fn is_current(&self, slot: usize, generation: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
    }

    pub fn release(&mut self, slot: usize, generation: u64) -> bool {
        if !self.is_current(slot, generation) {
            return false;
        }
        self.slots[slot] = EMPTY_DATABASE_CATALOG_READ_SLOT;
        self.items = self.items.checked_sub(DATABASE_CATALOG_READ_ITEMS).expect("database catalog-read item credit underflow");
        self.bytes = self.bytes.checked_sub(DATABASE_CATALOG_READ_BYTES).expect("database catalog-read byte credit underflow");
        true
    }
}

static DATABASE_CATALOG_READ_ADMISSION: Mutex<DatabaseCatalogReadAdmissionState> = Mutex::new(DatabaseCatalogReadAdmissionState { slots: [EMPTY_DATABASE_CATALOG_READ_SLOT; DATABASE_CATALOG_READ_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseCatalogReadAdmission {
    pub slot: usize,
    pub generation: u64,
}

impl DatabaseCatalogReadAdmission {
    pub fn try_claim() -> Result<Self, DbError> {
        let (slot, generation) = DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(DATABASE_CATALOG_READ_ITEMS, DATABASE_CATALOG_READ_BYTES)?;
        Ok(Self { slot, generation })
    }

    pub fn is_current(&self) -> bool {
        DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation)
    }
}

impl Drop for DatabaseCatalogReadAdmission {
    fn drop(&mut self) {
        DATABASE_CATALOG_READ_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DatabaseCatalogRootKey(pub &'static str);

impl DatabaseCatalogRootKey {
    pub fn root() -> Self {
        Self("catalog-root")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCatalogReadProgress {
    Admitted,
    Scheduled,
    Polling,
    Pending,
    Completed,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCatalogReadCloseStep {
    Progress,
    Blocked,
    Complete,
}

pub struct DatabaseCatalogReadResult {
    pub storage: Arc<db_storage::DbBackend>,
    pub key: DatabaseCatalogRootKey,
    pub root: Result<Option<(db_io_core::DbIoPages, EpochFence)>, DbError>,
}

impl DatabaseCatalogReadResult {
    pub fn into_parts(self) -> (Arc<db_storage::DbBackend>, DatabaseCatalogRootKey, Result<Option<(db_io_core::DbIoPages, EpochFence)>, DbError>) {
        (self.storage, self.key, self.root)
    }
}

pub struct DatabaseCatalogReadRejected {
    pub error: Option<DbError>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub key: Option<DatabaseCatalogRootKey>,
}

impl std::fmt::Debug for DatabaseCatalogReadRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCatalogReadRejected").field("error", &self.error).field("storage", &self.storage.as_ref().map(Arc::as_ptr)).field("key", &self.key).finish()
    }
}

impl DatabaseCatalogReadRejected {
    pub fn retry(mut self, pool: Arc<WorkerPool>) -> Result<DatabaseCatalogReadFuture, Self> {
        let Some(storage) = self.storage.take() else { return Err(self) };
        let Some(key) = self.key.take() else {
            self.storage = Some(storage);
            return Err(self);
        };
        match DatabaseCatalogReadFuture::try_submit(pool, storage, key) {
            Ok(future) => Ok(future),
            Err(mut rejected) => {
                self.error = rejected.error.take();
                self.storage = rejected.storage.take();
                self.key = rejected.key.take();
                Err(self)
            }
        }
    }

    pub fn close_step(&mut self) -> DatabaseCatalogReadCloseStep {
        if let Some(storage) = self.storage.take() {
            drop(storage);
            DatabaseCatalogReadCloseStep::Progress
        } else if let Some(key) = self.key.take() {
            drop(key);
            DatabaseCatalogReadCloseStep::Progress
        } else {
            DatabaseCatalogReadCloseStep::Complete
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.key.is_none()
    }

    pub fn into_error_after_close(mut self) -> Result<DbError, Self> {
        if !self.terminal_is_empty() {
            return Err(self);
        }
        Ok(self.error.take().expect("database catalog-read rejection error missing"))
    }

    pub fn close_and_take_error(self, pool: Arc<WorkerPool>) -> DbError {
        self.mount_close_and_take_error(pool, true).0
    }

    pub fn mount_close_and_take_error(mut self, pool: Arc<WorkerPool>, schedule: bool) -> (DbError, Arc<DatabaseCatalogReadRejectedClose>) {
        let error = self.error.take().expect("database catalog-read rejection error missing");
        let close = DatabaseCatalogReadRejectedClose::mount(pool, self, schedule);
        (error, close)
    }
}

struct DatabaseCatalogReadRejectedClose {
    pub pool: Arc<WorkerPool>,
    pub owner: Mutex<Option<DatabaseCatalogReadRejected>>,
    pub retry_job: Mutex<Option<semio_framework_async::Job>>,
    pub scheduled: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_submit_hook: Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
}

impl DatabaseCatalogReadRejectedClose {
    pub fn mount(pool: Arc<WorkerPool>, owner: DatabaseCatalogReadRejected, schedule: bool) -> Arc<Self> {
        let state = Arc::new(Self {
            pool,
            owner: Mutex::new(Some(owner)),
            retry_job: Mutex::new(None),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            #[cfg(any(test, feature = "testkit"))]
            controlled_submit_hook: Mutex::new(None),
        });
        if schedule {
            state.schedule();
        }
        state
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()));
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job) {
        #[cfg(test)]
        let job = if let Some(submit) = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
            match submit(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error.into_job());
                let state = self.clone();
                self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
            }
        }
    }

    pub fn retry(self: Arc<Self>) {
        let job = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(job) = job {
            self.submit_exact(job);
        }
    }

    pub fn drive_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.scheduled.store(false, Ordering::Release);
        let owner = self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let Some(mut owner) = owner else {
            self.finished.store(true, Ordering::Release);
            return;
        };
        let step = owner.close_step();
        if step == DatabaseCatalogReadCloseStep::Progress && !owner.terminal_is_empty() {
            *self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
            self.schedule();
        } else {
            debug_assert!(owner.terminal_is_empty());
            self.finished.store(true, Ordering::Release);
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire)
            && self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
    }
}

type DatabaseCatalogReadBackendFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCatalogReadResult> + Send + 'static>>;

struct DatabaseCatalogReadWork {
    pub future: Option<DatabaseCatalogReadBackendFuture>,
    pollable: bool,
    #[cfg(any(test, feature = "testkit"))]
    pub storage_identity: usize,
}

impl DatabaseCatalogReadWork {
    pub fn new(storage: Arc<db_storage::DbBackend>, key: DatabaseCatalogRootKey) -> Self {
        #[cfg(any(test, feature = "testkit"))]
        let storage_identity = Arc::as_ptr(&storage) as usize;
        let future = Box::pin(async move {
            let root = storage.catalog().await.read_root().await;
            DatabaseCatalogReadResult { storage, key, root }
        });
        Self {
            future: Some(future),
            pollable: true,
            #[cfg(any(test, feature = "testkit"))]
            storage_identity,
        }
    }

    pub fn poll(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<DatabaseCatalogReadResult> {
        let output = self.future.as_mut().map_or(std::task::Poll::Pending, |future| future.as_mut().poll(context));
        if output.is_ready() {
            self.pollable = false;
        }
        output
    }

    fn can_poll(&self) -> bool {
        self.pollable && self.future.is_some()
    }

    pub fn close_step(&mut self) -> bool {
        self.pollable = false;
        self.future.take().is_some()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.future.is_none()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn controlled(future: DatabaseCatalogReadBackendFuture, storage_identity: usize) -> Self {
        Self { future: Some(future), pollable: true, storage_identity }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCatalogReadPhase {
    Handoff,
    Poll,
    RetainWork,
    DrainWork,
    ReleaseWork,
    RetainResult,
    Publish,
    Terminal,
}

impl DatabaseCatalogReadPhase {
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

struct DatabaseCatalogReadState {
    pub pool: Arc<WorkerPool>,
    pub _pool_use: Arc<WorkerPoolUse>,
    pub slot: usize,
    pub generation: u64,
    pub admission: Mutex<Option<DatabaseCatalogReadAdmission>>,
    pub work: Mutex<Option<DatabaseCatalogReadWork>>,
    pub poll_work: Mutex<Option<DatabaseCatalogReadWork>>,
    pub staged_result: Mutex<Option<DatabaseCatalogReadResult>>,
    pub terminal_error: Mutex<Option<(DbError, DatabaseCatalogReadProgress)>>,
    pub completion: Mutex<Option<Result<DatabaseCatalogReadResult, DbError>>>,
    pub terminal_work: Mutex<Option<DatabaseCatalogReadWork>>,
    pub terminal_result: Mutex<Option<Result<DatabaseCatalogReadResult, DbError>>>,
    pub terminal_completion: Mutex<Option<Result<DatabaseCatalogReadResult, DbError>>>,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub retry_armed: std::sync::atomic::AtomicBool,
    pub lease: std::sync::atomic::AtomicU8,
    pub completion_consumed: std::sync::atomic::AtomicBool,
    pub retry_generation: std::sync::atomic::AtomicU64,
    pub polling: std::sync::atomic::AtomicBool,
    pub wake_requested: std::sync::atomic::AtomicBool,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub terminal_checked_out: std::sync::atomic::AtomicBool,
    pub terminal_result_checked_out: std::sync::atomic::AtomicBool,
    pub phase: std::sync::atomic::AtomicU8,
    pub progress: std::sync::atomic::AtomicU8,
    #[cfg(test)]
    controlled_retry_callback_hook: Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_submit_refusal: Mutex<Option<semio_framework_async::WorkerSubmitErrorKind>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_owner_transfer_hook: Mutex<Option<Box<dyn FnOnce(usize) + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_completion_before_wake_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_lease_release_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_submit_hook: Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_publication_before_waker_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub poll_worker_thread: std::sync::atomic::AtomicBool,
}

struct DatabaseCatalogReadLease {
    pub state: Arc<DatabaseCatalogReadState>,
    pub held: bool,
    pub finalize: bool,
}

impl DatabaseCatalogReadLease {
    pub fn retire(&mut self) {
        self.state.lease.store(DATABASE_CATALOG_READ_CLOSED, std::sync::atomic::Ordering::Release);
        self.held = false;
        self.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.state.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.state.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.state.generation) {
            registry[self.state.slot] = None;
        }
    }

    pub fn submit_exact(mut self, job: semio_framework_async::Job, attempt: u8) {
        self.state.lease.store(DATABASE_CATALOG_READ_QUEUED, std::sync::atomic::Ordering::Release);
        self.held = false;
        self.state.submit_exact(job, attempt);
    }
}

impl Drop for DatabaseCatalogReadLease {
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
        let retained = self.state.has_retained_submission();
        let mask = if retained { !(DATABASE_CATALOG_READ_ACTIVE | DATABASE_CATALOG_READ_QUEUED) } else { !DATABASE_CATALOG_READ_ACTIVE };
        let previous = self.state.lease.fetch_and(mask, Ordering::AcqRel);
        self.held = false;
        if !retained && previous & DATABASE_CATALOG_READ_QUEUED != 0 {
            self.state.submit_drive_job();
        }
    }
}

struct DatabaseCatalogReadWake {
    pub state: std::sync::Weak<DatabaseCatalogReadState>,
    pub generation: u64,
}

pub fn database_catalog_read_registry() -> &'static Mutex<[Option<Arc<DatabaseCatalogReadState>>; DATABASE_CATALOG_READ_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<Mutex<[Option<Arc<DatabaseCatalogReadState>>; DATABASE_CATALOG_READ_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCatalogReadWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let Some(state) = self.state.upgrade() else { return };
        if state.generation != self.generation || !state.is_current() || state.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if !state.polling.load(std::sync::atomic::Ordering::Acquire) {
            state.schedule();
        }
    }
}

impl DatabaseCatalogReadState {
    pub fn try_lease(self: &Arc<Self>, expected: u8, finalize: bool) -> Option<DatabaseCatalogReadLease> {
        self.lease.compare_exchange(expected, DATABASE_CATALOG_READ_ACTIVE, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok()?;
        Some(DatabaseCatalogReadLease { state: self.clone(), held: true, finalize })
    }

    fn has_retained_submission(&self) -> bool {
        let retry = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        retry || self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    pub fn request_drive(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let mut observed = self.lease.load(Ordering::Acquire);
        loop {
            if observed & (DATABASE_CATALOG_READ_CLOSED | DATABASE_CATALOG_READ_QUEUED) != 0 || self.has_retained_submission() {
                return;
            }
            match self.lease.compare_exchange_weak(observed, observed | DATABASE_CATALOG_READ_QUEUED, Ordering::AcqRel, Ordering::Acquire) {
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
        self.set_progress(DatabaseCatalogReadProgress::Cancelled);
        if let Some(_lease) = self.try_lease(0, false) {
            if self.has_retained_submission() {
                return;
            }
            if self.phase() != DatabaseCatalogReadPhase::Terminal && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
            } else {
                self.request_drive();
            }
        } else {
            self.request_drive();
        }
    }

    pub fn phase(&self) -> DatabaseCatalogReadPhase {
        DatabaseCatalogReadPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCatalogReadPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn set_progress(&self, progress: DatabaseCatalogReadProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogReadAdmission::is_current)
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
        self.submit_exact(job, 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
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
        #[cfg(test)]
        let refusal = self.controlled_submit_refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        #[cfg(test)]
        if let Some(kind) = refusal {
            self.handle_submit_refusal(kind, job, attempt);
            return;
        }
        if let Err(error) = self.pool.try_submit(Lane::Io, job) {
            self.handle_submit_refusal(error.kind(), error.into_job(), attempt);
        }
    }

    pub fn handle_submit_refusal(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind, job: semio_framework_async::Job, attempt: u8) {
        let mut lease = self.try_lease(DATABASE_CATALOG_READ_QUEUED, true).expect("exact catalog refusal owns the queued turn");
        if self.completion_consumed.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() {
            drop(job);
            lease.retire();
            return;
        }
        if let Some(next) = worker_submit_retry_attempt(kind, attempt, DATABASE_CATALOG_READ_RETRY_LIMIT) {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, next));
            self.arm_retry();
        } else {
            *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
            self.set_phase(DatabaseCatalogReadPhase::RetainWork);
            self.set_progress(DatabaseCatalogReadProgress::Fault);
            self.publish_public_completion(Err(DbError::Unavailable(format!("database catalog-read WorkerPool submission failed: {kind:?}"))));
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.swap(true, Ordering::AcqRel) {
            return;
        }
        let Some(generation) = self.retry_generation.load(Ordering::Acquire).checked_add(1) else {
            self.retry_armed.store(false, Ordering::Release);
            self.set_progress(DatabaseCatalogReadProgress::Fault);
            self.publish_public_completion(Err(DbError::LimitExceeded("database catalog-read retry generation")));
            return;
        };
        self.retry_generation.store(generation, Ordering::Release);
        self.retry_submission_at(generation);
    }

    pub fn retry_submission_at(self: &Arc<Self>, generation: u64) {
        let state = self.clone();
        self.schedule_retry_callback(Box::new(move || state.retry_submission_once(generation)));
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
            let stale = !self.is_current();
            if self.cancelled.load(Ordering::Acquire) || stale {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                self.set_phase(DatabaseCatalogReadPhase::RetainWork);
                self.set_progress(if stale { DatabaseCatalogReadProgress::Fault } else { DatabaseCatalogReadProgress::Cancelled });
                self.publish_public_completion(Err(if stale { DbError::Unavailable("database catalog-read retry generation stale".into()) } else { DbError::Closed }));
            } else {
                lease.submit_exact(job, attempt);
            }
        }
    }

    fn schedule_retry_callback(self: &Arc<Self>, job: semio_framework_async::Job) {
        #[cfg(test)]
        let callback = self.controlled_retry_callback_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        #[cfg(test)]
        let job = if let Some(callback) = callback {
            match callback(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), job);
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        let Some(_lease) = self.try_lease(DATABASE_CATALOG_READ_QUEUED, true) else { return };
        if !self.is_current() && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCatalogReadPhase::Terminal {
            self.stage_terminal(DbError::Unavailable("database catalog-read generation stale".to_string()), DatabaseCatalogReadProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCatalogReadPhase::Terminal {
            self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
            return;
        }
        match self.phase() {
            DatabaseCatalogReadPhase::Handoff => {
                let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
                    self.stage_terminal(DbError::Unavailable("database catalog-read handoff owner missing".to_string()), DatabaseCatalogReadProgress::Fault);
                    return;
                };
                #[cfg(test)]
                if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap().take() {
                    hook(work.storage_identity);
                }
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.set_phase(DatabaseCatalogReadPhase::Poll);
                self.schedule();
            }
            DatabaseCatalogReadPhase::Poll => self.poll_backend_once(generation),
            DatabaseCatalogReadPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    #[cfg(test)]
                    if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap().take() {
                        hook(work.storage_identity);
                    }
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                }
                self.set_phase(DatabaseCatalogReadPhase::DrainWork);
                self.schedule_cleanup();
            }
            DatabaseCatalogReadPhase::DrainWork => {
                let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let progressed = terminal.as_mut().is_some_and(DatabaseCatalogReadWork::close_step);
                let empty = terminal.as_ref().is_some_and(DatabaseCatalogReadWork::terminal_is_empty);
                drop(terminal);
                if progressed || empty {
                    self.set_phase(DatabaseCatalogReadPhase::ReleaseWork);
                    self.schedule_cleanup();
                } else {
                    self.stage_terminal(DbError::Unavailable("database catalog-read work failed to release".to_string()), DatabaseCatalogReadProgress::Fault);
                }
            }
            DatabaseCatalogReadPhase::ReleaseWork => {
                let terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if terminal.as_ref().is_some_and(DatabaseCatalogReadWork::terminal_is_empty) {
                    let has_result = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
                    self.set_phase(if has_result { DatabaseCatalogReadPhase::RetainResult } else { DatabaseCatalogReadPhase::Publish });
                    self.schedule_cleanup();
                } else {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = terminal;
                    self.stage_terminal(DbError::Unavailable("database catalog-read work witness failed".to_string()), DatabaseCatalogReadProgress::Fault);
                }
            }
            DatabaseCatalogReadPhase::RetainResult => {
                if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                    if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
                    }
                }
                self.set_phase(DatabaseCatalogReadPhase::Publish);
                self.schedule_cleanup();
            }
            DatabaseCatalogReadPhase::Publish => self.publish_staged(),
            DatabaseCatalogReadPhase::Terminal => {}
        }
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.poll_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        let Some(mut work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            self.stage_terminal(DbError::Unavailable("database catalog-read poll owner missing".to_string()), DatabaseCatalogReadProgress::Fault);
            return;
        };
        #[cfg(test)]
        if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap().take() {
            hook(work.storage_identity);
        }
        self.polling.store(true, Ordering::Release);
        self.set_progress(DatabaseCatalogReadProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCatalogReadWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if !self.is_current() {
                    self.publish_terminal(DbError::Unavailable("database catalog-read generation stale during poll".to_string()), DatabaseCatalogReadProgress::Fault);
                    self.release_terminal_poll();
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                    self.release_terminal_poll();
                } else {
                    self.set_progress(DatabaseCatalogReadProgress::Pending);
                    self.polling.store(false, Ordering::Release);
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        self.schedule();
                    }
                }
            }
            Ok(std::task::Poll::Ready(result)) => {
                let too_large = result.root.as_ref().ok().and_then(Option::as_ref).is_some_and(|(bytes, _)| bytes.len() > DATABASE_CATALOG_READ_BYTES as usize);
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                self.set_phase(DatabaseCatalogReadPhase::RetainWork);
                if too_large {
                    self.publish_terminal(DbError::LimitExceeded("database catalog-read result bytes"), DatabaseCatalogReadProgress::Fault);
                } else if !self.is_current() {
                    self.publish_terminal(DbError::Unavailable("database catalog-read generation stale after result".to_string()), DatabaseCatalogReadProgress::Fault);
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                }
                self.release_terminal_poll();
            }
            Err(_) => {
                work.pollable = false;
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_terminal(DbError::Unavailable("database catalog-read backend poll panicked".to_string()), DatabaseCatalogReadProgress::Fault);
                self.release_terminal_poll();
            }
        }
    }

    fn publish_terminal(&self, error: DbError, progress: DatabaseCatalogReadProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        let mut terminal = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            *terminal = Some((error, progress));
        }
        drop(terminal);
        self.set_phase(DatabaseCatalogReadPhase::RetainWork);
    }

    pub fn release_terminal_poll(self: &Arc<Self>) {
        self.polling.store(false, std::sync::atomic::Ordering::Release);
        self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
        self.schedule_cleanup();
    }

    pub fn stage_terminal(self: &Arc<Self>, error: DbError, progress: DatabaseCatalogReadProgress) {
        self.publish_terminal(error, progress);
        self.schedule_cleanup();
    }

    pub fn publish_staged(self: &Arc<Self>) {
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            if !self.is_current() {
                self.stage_terminal(DbError::Unavailable("database catalog-read generation stale before publication".to_string()), DatabaseCatalogReadProgress::Fault);
                return;
            }
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                return;
            }
        }
        let result = if let Some((error, progress)) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.set_progress(progress);
            Err(error)
        } else if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.set_progress(DatabaseCatalogReadProgress::Completed);
            Ok(result)
        } else {
            self.set_progress(DatabaseCatalogReadProgress::Fault);
            Err(DbError::Unavailable("database catalog-read publication owner missing".to_string()))
        };
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            self.publish_public_completion(result);
        }
        self.set_phase(DatabaseCatalogReadPhase::Terminal);
    }

    fn publish_public_completion(&self, result: Result<DatabaseCatalogReadResult, DbError>) {
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return;
        }
        *completion = Some(result);
        drop(completion);
        #[cfg(test)]
        if let Some(hook) = self.controlled_completion_before_wake_hook.lock().unwrap().take() {
            hook();
        }
        let waker = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(waker) = waker {
            waker.wake();
        }
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

    pub fn close_step(self: &Arc<Self>) -> DatabaseCatalogReadCloseStep {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        let Some(mut lease) = self.try_lease(0, false) else {
            return if self.terminal_is_empty() { DatabaseCatalogReadCloseStep::Complete } else { DatabaseCatalogReadCloseStep::Blocked };
        };
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.retry_armed.load(Ordering::Acquire) {
            let current = self.retry_generation.load(Ordering::Acquire);
            if let Some(next) = current.checked_add(1) {
                self.retry_generation.store(next, Ordering::Release);
            }
            self.retry_armed.store(false, Ordering::Release);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.polling.load(Ordering::Acquire) {
            return DatabaseCatalogReadCloseStep::Blocked;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        {
            let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(work) = terminal.as_mut() {
                if work.close_step() {
                    return DatabaseCatalogReadCloseStep::Progress;
                }
                if work.terminal_is_empty() {
                    terminal.take();
                    return DatabaseCatalogReadCloseStep::Progress;
                }
            }
        }
        if self.terminal_result_checked_out.load(Ordering::Acquire) {
            return DatabaseCatalogReadCloseStep::Blocked;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            lease.retire();
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.roots_are_empty() {
            lease.retire();
            DatabaseCatalogReadCloseStep::Complete
        } else {
            DatabaseCatalogReadCloseStep::Blocked
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
}

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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
