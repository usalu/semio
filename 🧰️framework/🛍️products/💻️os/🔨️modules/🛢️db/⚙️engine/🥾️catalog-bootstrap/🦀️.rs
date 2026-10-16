//! 🥾️ Db engine retained catalog bootstrap operation.

use db_durability::EpochFence;
use db_ids::{DbError, GenerationId};
use db_io_core::CatalogStorage as _;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::test_worker_pool;

//#region 🔖️CatalogBootstrapCas
const DATABASE_CATALOG_BOOTSTRAP_SLOTS: usize = 64;

const DATABASE_CATALOG_BOOTSTRAP_ITEMS: u64 = 8;

const DATABASE_CATALOG_BOOTSTRAP_PAGES: u8 = 8;

const DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES: u64 = 16 * 1024;

const DATABASE_CATALOG_BOOTSTRAP_BYTES: u64 = DATABASE_CATALOG_BOOTSTRAP_PAGES as u64 * db_io_core::DB_IO_PAGE_BYTES as u64 + DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES;

const DATABASE_CATALOG_BOOTSTRAP_TOTAL_ITEMS: u64 = DATABASE_CATALOG_BOOTSTRAP_ITEMS * DATABASE_CATALOG_BOOTSTRAP_SLOTS as u64;

const DATABASE_CATALOG_BOOTSTRAP_TOTAL_BYTES: u64 = DATABASE_CATALOG_BOOTSTRAP_BYTES * DATABASE_CATALOG_BOOTSTRAP_SLOTS as u64;

const DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT: u8 = 8;

#[derive(Clone, Copy)]
struct DatabaseCatalogBootstrapAdmissionSlot {
    pub generation: u64,
    pub items: u64,
    pub bytes: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT: DatabaseCatalogBootstrapAdmissionSlot = DatabaseCatalogBootstrapAdmissionSlot { generation: 0, items: 0, bytes: 0, occupied: false };

struct DatabaseCatalogBootstrapAdmissionState {
    pub slots: [DatabaseCatalogBootstrapAdmissionSlot; DATABASE_CATALOG_BOOTSTRAP_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCatalogBootstrapAdmissionState {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self { slots: [EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT; DATABASE_CATALOG_BOOTSTRAP_SLOTS], items: 0, bytes: 0, next_generation: 1 }
    }

    pub fn try_claim(&mut self, pages: u8) -> Result<(usize, u64, u64), DbError> {
        if pages == 0 || pages > DATABASE_CATALOG_BOOTSTRAP_PAGES {
            return Err(DbError::LimitExceeded("database catalog-bootstrap page credit"));
        }
        let bytes = u64::from(pages).checked_mul(db_io_core::DB_IO_PAGE_BYTES as u64).and_then(|bytes| bytes.checked_add(DATABASE_CATALOG_BOOTSTRAP_RESULT_BYTES)).ok_or(DbError::LimitExceeded("database catalog-bootstrap byte credit"))?;
        let Some(slot) = self.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::LimitExceeded("database catalog-bootstrap slot capacity"));
        };
        let items = self.items.checked_add(DATABASE_CATALOG_BOOTSTRAP_ITEMS).ok_or(DbError::LimitExceeded("database catalog-bootstrap aggregate items"))?;
        let total_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database catalog-bootstrap aggregate bytes"))?;
        if items > DATABASE_CATALOG_BOOTSTRAP_TOTAL_ITEMS || total_bytes > DATABASE_CATALOG_BOOTSTRAP_TOTAL_BYTES {
            return Err(DbError::LimitExceeded("database catalog-bootstrap aggregate capacity"));
        }
        let generation = self.next_generation;
        let Some(next_generation) = generation.checked_add(1) else {
            return Err(DbError::LimitExceeded("database catalog-bootstrap generation"));
        };
        self.next_generation = next_generation;
        self.slots[slot] = DatabaseCatalogBootstrapAdmissionSlot { generation, items: DATABASE_CATALOG_BOOTSTRAP_ITEMS, bytes, occupied: true };
        self.items = items;
        self.bytes = total_bytes;
        Ok((slot, generation, bytes))
    }

    pub fn is_current(&self, slot: usize, generation: u64, bytes: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation && entry.items == DATABASE_CATALOG_BOOTSTRAP_ITEMS && entry.bytes == bytes)
    }

    pub fn release(&mut self, slot: usize, generation: u64, bytes: u64) -> bool {
        if !self.is_current(slot, generation, bytes) {
            return false;
        }
        let Some(items) = self.items.checked_sub(DATABASE_CATALOG_BOOTSTRAP_ITEMS) else { return false };
        let Some(total_bytes) = self.bytes.checked_sub(bytes) else { return false };
        self.slots[slot] = EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT;
        self.items = items;
        self.bytes = total_bytes;
        true
    }
}

static DATABASE_CATALOG_BOOTSTRAP_ADMISSION: Mutex<DatabaseCatalogBootstrapAdmissionState> =
    Mutex::new(DatabaseCatalogBootstrapAdmissionState { slots: [EMPTY_DATABASE_CATALOG_BOOTSTRAP_SLOT; DATABASE_CATALOG_BOOTSTRAP_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseCatalogBootstrapAdmission {
    pub slot: usize,
    pub generation: u64,
    pub bytes: u64,
}

impl DatabaseCatalogBootstrapAdmission {
    pub fn try_claim(pages: u8) -> Result<Self, DbError> {
        let (slot, generation, bytes) = DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(pages)?;
        Ok(Self { slot, generation, bytes })
    }

    pub fn is_current(&self) -> bool {
        DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation, self.bytes)
    }
}

impl Drop for DatabaseCatalogBootstrapAdmission {
    fn drop(&mut self) {
        DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation, self.bytes);
    }
}

/// 🗝️ Fixed retained authority naming the one initial catalog root.
#[derive(Debug, PartialEq, Eq)]
pub struct DatabaseCatalogBootstrapKey(pub [u8; 16]);

impl DatabaseCatalogBootstrapKey {
    pub fn root() -> Self {
        Self(*b"catalog-root-v01")
    }
}

/// 🧭️ Observable phase of one mounted initial-catalog compare-and-swap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCatalogBootstrapProgress {
    Admitted,
    Scheduled,
    Polling,
    Pending,
    Completed,
    Cancelled,
    Fault,
}

/// 🧹️ Result of requesting one mounted terminal-close opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCatalogBootstrapCloseStep {
    Progress,
    Blocked,
    Complete,
}

/// 🧾️ Exact generation and retained-root witness for a lost bootstrap authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatabaseCatalogBootstrapTerminalWitness {
    pub generation: u64,
    pub retained_owners: usize,
    pub terminal_empty: bool,
}

/// 📦️ Exact storage/key/fence result returned for success, mismatch, refusal, cancellation, or fault.
pub struct DatabaseCatalogBootstrapResult {
    pub state: Option<Arc<DatabaseCatalogBootstrapState>>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub key: Option<DatabaseCatalogBootstrapKey>,
    pub expected: EpochFence,
    pub actual: Option<Result<EpochFence, DbError>>,
}

impl std::fmt::Debug for DatabaseCatalogBootstrapResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCatalogBootstrapResult").field("storage", &self.storage.as_ref().map(Arc::as_ptr)).field("key", &self.key).field("expected", &self.expected).field("actual", &self.actual).finish()
    }
}

impl DatabaseCatalogBootstrapResult {
    pub fn into_parts(mut self) -> Result<(Arc<db_storage::DbBackend>, DatabaseCatalogBootstrapKey, EpochFence, Result<EpochFence, DbError>), Self> {
        let storage = self.storage.take();
        let key = self.key.take();
        let actual = self.actual.take();
        match (storage, key, actual) {
            (Some(storage), Some(key), Some(actual)) => {
                if let Some(state) = self.state.take() {
                    state.release_success();
                }
                Ok((storage, key, self.expected, actual))
            }
            (storage, key, actual) => {
                self.storage = storage;
                self.key = key;
                self.actual = actual;
                Err(self)
            }
        }
    }

    pub fn close_one(&mut self) -> bool {
        if self.actual.take().is_some() {
            true
        } else if self.key.take().is_some() {
            true
        } else {
            self.storage.take().is_some()
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.key.is_none() && self.actual.is_none()
    }
}

impl Drop for DatabaseCatalogBootstrapResult {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let Some(state) = self.state.take() else { return };
        let owner = Self { state: None, storage: self.storage.take(), key: self.key.take(), expected: self.expected, actual: self.actual.take() };
        *state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(owner));
        state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        state.closing.store(true, std::sync::atomic::Ordering::Release);
        state.schedule();
    }
}

struct DatabaseCatalogBootstrapRejectedCloseOwner {
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub pages: Option<db_io_core::DbIoPages>,
    pub key: Option<DatabaseCatalogBootstrapKey>,
}

impl DatabaseCatalogBootstrapRejectedCloseOwner {
    pub fn empty() -> Self {
        Self { storage: None, pages: None, key: None }
    }

    pub fn close_one(&mut self) -> Result<bool, DbError> {
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
            if pages.terminal_is_empty() {
                self.pages.take();
                return Ok(true);
            }
        }
        if self.key.take().is_some() {
            Ok(true)
        } else {
            Ok(self.storage.take().is_some())
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.pages.is_none() && self.key.is_none()
    }
}

/// ↩️ Exact pre-submission handback for a refused bootstrap operation.
pub struct DatabaseCatalogBootstrapRejected {
    pub error: Option<DbError>,
    pub close: Arc<DatabaseCatalogBootstrapRejectedClose>,
    pub expected: EpochFence,
}

impl std::fmt::Debug for DatabaseCatalogBootstrapRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let owner = self.close.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        formatter
            .debug_struct("DatabaseCatalogBootstrapRejected")
            .field("error", &self.error)
            .field("storage", &owner.as_ref().and_then(|owner| owner.storage.as_ref()).map(Arc::as_ptr))
            .field("pages", &owner.as_ref().and_then(|owner| owner.pages.as_ref()).map(db_io_core::DbIoPages::page_count))
            .field("key", &owner.as_ref().and_then(|owner| owner.key.as_ref()))
            .field("expected", &self.expected)
            .finish()
    }
}

impl DatabaseCatalogBootstrapRejected {
    pub fn new(pool: Arc<WorkerPool>, error: DbError, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, key: DatabaseCatalogBootstrapKey, expected: EpochFence) -> Self {
        let owner = DatabaseCatalogBootstrapRejectedCloseOwner { storage: Some(storage), pages: Some(pages), key: Some(key) };
        Self { error: Some(error), close: DatabaseCatalogBootstrapRejectedClose::prepare(pool, owner), expected }
    }

    pub fn retry(self, pool: Arc<WorkerPool>) -> Result<DatabaseCatalogBootstrapFuture, Self> {
        let mut owner = self.close.take_owner().unwrap_or_else(DatabaseCatalogBootstrapRejectedCloseOwner::empty);
        let storage = owner.storage.take();
        let pages = owner.pages.take();
        let key = owner.key.take();
        match (storage, pages, key) {
            (Some(storage), Some(pages), Some(key)) => match DatabaseCatalogBootstrapFuture::try_submit_with_key(pool, storage, pages, key, self.expected) {
                Ok(future) => Ok(future),
                Err(rejected) => Err(rejected),
            },
            (storage, pages, key) => {
                owner.storage = storage;
                owner.pages = pages;
                owner.key = key;
                self.close.restore_owner(owner);
                Err(self)
            }
        }
    }

    pub fn into_parts(mut self) -> Result<(DbError, Arc<db_storage::DbBackend>, db_io_core::DbIoPages, DatabaseCatalogBootstrapKey, EpochFence), Self> {
        let error = self.error.take();
        let mut owner = self.close.take_owner().unwrap_or_else(DatabaseCatalogBootstrapRejectedCloseOwner::empty);
        let storage = owner.storage.take();
        let pages = owner.pages.take();
        let key = owner.key.take();
        match (error, storage, pages, key) {
            (Some(error), Some(storage), Some(pages), Some(key)) => Ok((error, storage, pages, key, self.expected)),
            (error, storage, pages, key) => {
                self.error = error;
                owner.storage = storage;
                owner.pages = pages;
                owner.key = key;
                self.close.restore_owner(owner);
                Err(self)
            }
        }
    }

    pub fn close_and_take_error(mut self) -> DbError {
        let error = match self.error.take() {
            Some(error) => error,
            None => DbError::LimitExceeded("database catalog-bootstrap rejection error owner"),
        };
        self.close.schedule();
        error
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn mount_close_and_take_error(mut self) -> (DbError, Option<Arc<DatabaseCatalogBootstrapRejectedClose>>) {
        let error = match self.error.take() {
            Some(error) => error,
            None => DbError::LimitExceeded("database catalog-bootstrap rejection error owner"),
        };
        self.close.schedule();
        (error, Some(self.close.clone()))
    }
}

impl Drop for DatabaseCatalogBootstrapRejected {
    fn drop(&mut self) {
        if self.close.has_owner() {
            self.close.schedule();
        }
    }
}

struct DatabaseCatalogBootstrapRejectedClose {
    pub pool: Arc<WorkerPool>,
    pub owner: Mutex<Option<DatabaseCatalogBootstrapRejectedCloseOwner>>,
    pub retry_job: Mutex<Option<semio_framework_async::Job>>,
    pub scheduled: std::sync::atomic::AtomicBool,
    pub fault: Mutex<Option<DbError>>,
}

impl DatabaseCatalogBootstrapRejectedClose {
    pub fn prepare(pool: Arc<WorkerPool>, owner: DatabaseCatalogBootstrapRejectedCloseOwner) -> Arc<Self> {
        Arc::new(Self { pool, owner: Mutex::new(Some(owner)), retry_job: Mutex::new(None), scheduled: std::sync::atomic::AtomicBool::new(false), fault: Mutex::new(None) })
    }

    pub fn take_owner(&self) -> Option<DatabaseCatalogBootstrapRejectedCloseOwner> {
        self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }

    pub fn restore_owner(&self, owner: DatabaseCatalogBootstrapRejectedCloseOwner) {
        *self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
    }

    fn has_owner(&self) -> bool {
        self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| !owner.terminal_is_empty())
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()));
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error.into_job());
                let state = self.clone();
                self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
            }
        }
    }

    pub fn retry(self: Arc<Self>) {
        let retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(job) = retained {
            self.scheduled.store(true, std::sync::atomic::Ordering::Release);
            self.submit_exact(job);
        }
    }

    pub fn drive_one(self: Arc<Self>) {
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        let mut owner = self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let step = owner.as_mut().map(DatabaseCatalogBootstrapRejectedCloseOwner::close_one);
        match step {
            Some(Ok(_)) if owner.as_ref().is_some_and(DatabaseCatalogBootstrapRejectedCloseOwner::terminal_is_empty) => {
                owner.take();
            }
            Some(Ok(true)) => {
                drop(owner);
                self.schedule();
            }
            Some(Err(error)) => {
                *self.fault.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error);
            }
            _ => {}
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn terminal_is_empty(&self) -> bool {
        self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
    }
}

type DatabaseCatalogBootstrapBackendFuture = std::pin::Pin<Box<dyn Future<Output = Result<EpochFence, DbError>> + Send + 'static>>;

struct DatabaseCatalogBootstrapWork {
    pub future: Option<DatabaseCatalogBootstrapBackendFuture>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub pages: Option<db_io_core::DbIoPages>,
    pub expected: EpochFence,
    #[cfg(any(test, feature = "testkit"))]
    pub storage_identity: usize,
    #[cfg(any(test, feature = "testkit"))]
    pub page_identity: Option<u64>,
}

impl DatabaseCatalogBootstrapWork {
    pub fn new(storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, expected: EpochFence) -> Self {
        #[cfg(any(test, feature = "testkit"))]
        let storage_identity = Arc::as_ptr(&storage) as usize;
        #[cfg(any(test, feature = "testkit"))]
        let page_identity = Some(pages.operation());
        Self {
            future: None,
            storage: Some(storage),
            pages: Some(pages),
            expected,
            #[cfg(any(test, feature = "testkit"))]
            storage_identity,
            #[cfg(any(test, feature = "testkit"))]
            page_identity,
        }
    }

    pub fn poll(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<EpochFence, DbError>> {
        if self.future.is_none() {
            let storage = self.storage.take();
            let pages = self.pages.take();
            match (storage, pages) {
                (Some(storage), Some(pages)) => {
                    let expected = self.expected;
                    self.future = Some(Box::pin(async move { storage.catalog().await.cas_root(expected, pages).await }));
                }
                (storage, pages) => {
                    self.storage = storage;
                    self.pages = pages;
                    return std::task::Poll::Ready(Err(DbError::LimitExceeded("database catalog-bootstrap work input owner")));
                }
            }
        }
        self.future.as_mut().map_or(std::task::Poll::Ready(Err(DbError::LimitExceeded("database catalog-bootstrap future owner"))), |future| future.as_mut().poll(context))
    }

    pub fn close_one(&mut self) -> Result<bool, DbError> {
        if let Some(pages) = self.pages.as_mut() {
            if pages.close_step()?.is_some() {
                return Ok(true);
            }
            if pages.terminal_is_empty() {
                self.pages.take();
                return Ok(true);
            }
            return Ok(false);
        }
        Ok(self.storage.take().is_some() || self.future.take().is_some())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.future.is_none() && self.storage.is_none() && self.pages.is_none()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn controlled(future: DatabaseCatalogBootstrapBackendFuture, storage_identity: usize, page_identity: u64) -> Self {
        Self { future: Some(future), storage: None, pages: None, expected: EpochFence::INITIAL, storage_identity, page_identity: Some(page_identity) }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCatalogBootstrapPhase {
    Handoff,
    Poll,
    RetainWork,
    CloseWork,
    RetireInput,
    Validate,
    Publish,
    Terminal,
}

impl DatabaseCatalogBootstrapPhase {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Handoff,
            1 => Self::Poll,
            2 => Self::RetainWork,
            3 => Self::CloseWork,
            4 => Self::RetireInput,
            5 => Self::Validate,
            6 => Self::Publish,
            _ => Self::Terminal,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCatalogBootstrapDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

struct DatabaseCatalogBootstrapState {
    pub pool: Arc<WorkerPool>,
    pub _pool_use: Arc<WorkerPoolUse>,
    pub slot: usize,
    pub generation: u64,
    pub admission: Mutex<Option<DatabaseCatalogBootstrapAdmission>>,
    pub storage: Mutex<Option<Arc<db_storage::DbBackend>>>,
    pub pages: Mutex<Option<db_io_core::DbIoPages>>,
    pub key: Mutex<Option<DatabaseCatalogBootstrapKey>>,
    pub expected: EpochFence,
    pub work: Mutex<Option<DatabaseCatalogBootstrapWork>>,
    pub poll_work: Mutex<Option<DatabaseCatalogBootstrapWork>>,
    pub terminal_work: Mutex<Option<DatabaseCatalogBootstrapWork>>,
    staged_actual: Mutex<Option<Result<EpochFence, DbError>>>,
    pub terminal_error: Mutex<Option<DbError>>,
    pub completion: Mutex<Option<Result<DatabaseCatalogBootstrapResult, DbError>>>,
    pub terminal_completion: Mutex<Option<Result<DatabaseCatalogBootstrapResult, DbError>>>,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<semio_framework_async::Job>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub driver_authority: std::sync::atomic::AtomicU8,
    pub scheduled: std::sync::atomic::AtomicBool,
    pub polling: std::sync::atomic::AtomicBool,
    pub wake_requested: std::sync::atomic::AtomicBool,
    pub retry_armed: std::sync::atomic::AtomicBool,
    retry_pressure: std::sync::atomic::AtomicBool,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub closing: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pub terminal_checked_out: std::sync::atomic::AtomicBool,
    pub terminal_result_checked_out: std::sync::atomic::AtomicBool,
    pub phase: std::sync::atomic::AtomicU8,
    pub progress: std::sync::atomic::AtomicU8,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_submit_hook: Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_publication_before_waker_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(test)]
    controlled_driver_claim_hook: Mutex<Option<Arc<dyn Fn(DatabaseCatalogBootstrapPhase) + Send + Sync>>>,
    #[cfg(test)]
    controlled_driver_release_hook: Mutex<Option<Arc<dyn Fn(DatabaseCatalogBootstrapPhase) + Send + Sync>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub max_active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    driver_grants: std::sync::atomic::AtomicU64,
    #[cfg(any(test, feature = "testkit"))]
    pub poll_worker_thread: std::sync::atomic::AtomicBool,
}

struct DatabaseCatalogBootstrapWake {
    pub state: std::sync::Weak<DatabaseCatalogBootstrapState>,
    pub generation: u64,
}

pub fn database_catalog_bootstrap_registry() -> &'static Mutex<[Option<Arc<DatabaseCatalogBootstrapState>>; DATABASE_CATALOG_BOOTSTRAP_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<Mutex<[Option<Arc<DatabaseCatalogBootstrapState>>; DATABASE_CATALOG_BOOTSTRAP_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCatalogBootstrapWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let Some(state) = self.state.upgrade() else { return };
        if state.generation != self.generation || !state.is_current() || state.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        state.schedule();
    }
}

impl DatabaseCatalogBootstrapState {
    pub fn phase(&self) -> DatabaseCatalogBootstrapPhase {
        DatabaseCatalogBootstrapPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCatalogBootstrapPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn set_progress(&self, progress: DatabaseCatalogBootstrapProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogBootstrapAdmission::is_current)
    }

    pub fn observed_generation(&self) -> u64 {
        DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(self.slot).map_or(0, |entry| entry.generation)
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) {
            return;
        }
        while self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Idle as u8, DatabaseCatalogBootstrapDriverAuthority::Queued as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            self.wake_requested.store(true, Ordering::SeqCst);
            if self.driver_authority.load(Ordering::SeqCst) != DatabaseCatalogBootstrapDriverAuthority::Idle as u8 || !self.wake_requested.swap(false, Ordering::SeqCst) {
                return;
            }
        }
        self.wake_requested.swap(false, Ordering::SeqCst);
        self.scheduled.store(true, Ordering::Release);
        self.set_progress(DatabaseCatalogBootstrapProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        let job: semio_framework_async::Job = Box::new(move || state.drive_one(generation));
        #[cfg(test)]
        let job = if let Some(submit) = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
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
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next_attempt = attempt.checked_add(1).map_or(DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT, |next| next.min(DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT));
                if next_attempt == DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT {
                    self.retry_pressure.store(true, std::sync::atomic::Ordering::Release);
                }
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next_attempt));
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                if self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Queued as u8, DatabaseCatalogBootstrapDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
                {
                    self.arm_retry();
                } else {
                    *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DbError::LimitExceeded("database catalog-bootstrap driver queue authority"));
                    self.retry_pressure.store(true, std::sync::atomic::Ordering::Release);
                    self.closing.store(true, std::sync::atomic::Ordering::Release);
                    self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                    self.schedule();
                }
            }
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            state.retry_armed.store(false, Ordering::Release);
            let retained = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retained {
                if state.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Retry as u8, DatabaseCatalogBootstrapDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    state.scheduled.store(true, Ordering::Release);
                    state.submit_exact(job, attempt);
                } else {
                    *state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
                    *state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DbError::LimitExceeded("database catalog-bootstrap retry authority"));
                    state.retry_pressure.store(true, Ordering::Release);
                    state.closing.store(true, Ordering::Release);
                    state.wake_requested.store(true, Ordering::Release);
                    state.schedule();
                }
            } else if state.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Retry as u8, DatabaseCatalogBootstrapDriverAuthority::Idle as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                state.schedule();
            }
        });
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Queued as u8, DatabaseCatalogBootstrapDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        #[cfg(test)]
        {
            self.driver_grants.fetch_add(1, Ordering::AcqRel);
            let active = self.active_drivers.fetch_add(1, Ordering::AcqRel) + 1;
            self.max_active_drivers.fetch_max(active, Ordering::AcqRel);
            if let Some(hook) = self.controlled_driver_claim_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
                hook(self.phase());
            }
        }
        self.drive_claimed(generation);
        #[cfg(test)]
        if let Some(hook) = self.controlled_driver_release_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
            hook(self.phase());
        }
        #[cfg(test)]
        self.active_drivers.fetch_sub(1, Ordering::AcqRel);
        if self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Driving as u8, DatabaseCatalogBootstrapDriverAuthority::Idle as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DbError::LimitExceeded("database catalog-bootstrap driver release authority"));
            self.retry_pressure.store(true, Ordering::Release);
            self.closing.store(true, Ordering::Release);
        }
        if self.wake_requested.swap(false, Ordering::SeqCst) {
            self.schedule();
        }
    }

    pub fn drive_claimed(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        if self.closing.load(Ordering::Acquire) {
            self.retire_one();
            if !self.finished.load(Ordering::Acquire) || !self.roots_are_empty() {
                self.schedule();
            }
            return;
        }
        if !self.is_current() && matches!(self.phase(), DatabaseCatalogBootstrapPhase::Handoff | DatabaseCatalogBootstrapPhase::Poll) {
            self.stage_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCatalogBootstrapProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && !matches!(self.phase(), DatabaseCatalogBootstrapPhase::Validate | DatabaseCatalogBootstrapPhase::Publish | DatabaseCatalogBootstrapPhase::Terminal) {
            let mut terminal = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some(DbError::Closed);
            }
            drop(terminal);
            self.set_progress(DatabaseCatalogBootstrapProgress::Cancelled);
            if matches!(self.phase(), DatabaseCatalogBootstrapPhase::Handoff | DatabaseCatalogBootstrapPhase::Poll) {
                self.set_phase(DatabaseCatalogBootstrapPhase::RetainWork);
                self.schedule();
                return;
            }
        }
        match self.phase() {
            DatabaseCatalogBootstrapPhase::Handoff => self.handoff_one(),
            DatabaseCatalogBootstrapPhase::Poll => self.poll_backend_once(generation),
            DatabaseCatalogBootstrapPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                    self.set_phase(DatabaseCatalogBootstrapPhase::CloseWork);
                } else {
                    self.set_phase(DatabaseCatalogBootstrapPhase::RetireInput);
                }
                self.schedule();
            }
            DatabaseCatalogBootstrapPhase::CloseWork => {
                let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let step = work.as_mut().map(DatabaseCatalogBootstrapWork::close_one);
                let empty = work.as_ref().is_some_and(DatabaseCatalogBootstrapWork::terminal_is_empty);
                if empty {
                    work.take();
                }
                drop(work);
                if empty {
                    self.set_phase(DatabaseCatalogBootstrapPhase::RetireInput);
                    self.schedule();
                } else if matches!(step, Some(Ok(true))) {
                    self.schedule();
                } else if let Some(Err(error)) = step {
                    *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error);
                    self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
                    self.closing.store(true, Ordering::Release);
                    self.schedule();
                } else {
                    self.stage_error(DbError::LimitExceeded("database catalog-bootstrap work witness"), DatabaseCatalogBootstrapProgress::Fault);
                }
            }
            DatabaseCatalogBootstrapPhase::RetireInput => self.retire_input_one(),
            DatabaseCatalogBootstrapPhase::Validate => self.validate_one(),
            DatabaseCatalogBootstrapPhase::Publish => self.publish_one(),
            DatabaseCatalogBootstrapPhase::Terminal => {}
        }
    }

    pub fn handoff_one(self: &Arc<Self>) {
        if self.expected != EpochFence::INITIAL {
            self.stage_error(DbError::Fenced { expected: EpochFence::INITIAL.epoch, actual: self.expected.epoch }, DatabaseCatalogBootstrapProgress::Fault);
            return;
        }
        let pages = self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(Arc::clone);
        match (storage, pages) {
            (Some(storage), Some(pages)) => {
                *self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCatalogBootstrapWork::new(storage, pages, self.expected));
                self.set_phase(DatabaseCatalogBootstrapPhase::Poll);
                self.schedule();
            }
            (storage, pages) => {
                if let Some(pages) = pages {
                    *self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(pages);
                }
                drop(storage);
                self.stage_error(DbError::LimitExceeded("database catalog-bootstrap handoff owner"), DatabaseCatalogBootstrapProgress::Fault);
            }
        }
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) else {
            self.stage_error(DbError::LimitExceeded("database catalog-bootstrap poll owner"), DatabaseCatalogBootstrapProgress::Fault);
            return;
        };
        if self.polling.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.wake_requested.store(true, Ordering::Release);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.publish_poll_error(DbError::Closed, DatabaseCatalogBootstrapProgress::Cancelled);
            self.release_poll();
            return;
        }
        self.set_progress(DatabaseCatalogBootstrapProgress::Polling);
        #[cfg(test)]
        self.poll_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        let wake = std::task::Waker::from(Arc::new(DatabaseCatalogBootstrapWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if !self.is_current() {
                    self.publish_poll_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCatalogBootstrapProgress::Fault);
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_poll_error(DbError::Closed, DatabaseCatalogBootstrapProgress::Cancelled);
                } else {
                    self.set_progress(DatabaseCatalogBootstrapProgress::Pending);
                }
                self.release_poll();
            }
            Ok(std::task::Poll::Ready(actual)) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(actual);
                self.set_phase(DatabaseCatalogBootstrapPhase::RetainWork);
                if !self.is_current() {
                    self.publish_poll_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCatalogBootstrapProgress::Fault);
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_poll_error(DbError::Closed, DatabaseCatalogBootstrapProgress::Cancelled);
                }
                self.release_poll();
            }
            Err(_) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_poll_error(DbError::LimitExceeded("database catalog-bootstrap backend poll panic"), DatabaseCatalogBootstrapProgress::Fault);
                self.release_poll();
            }
        }
    }

    fn publish_poll_error(&self, error: DbError, progress: DatabaseCatalogBootstrapProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        let mut terminal = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            *terminal = Some(error);
        }
        self.set_phase(DatabaseCatalogBootstrapPhase::RetainWork);
    }

    fn release_poll(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.polling.store(false, Ordering::Release);
        if self.phase() != DatabaseCatalogBootstrapPhase::Poll {
            self.wake_requested.store(true, Ordering::Release);
        }
    }

    pub fn stage_error(self: &Arc<Self>, error: DbError, progress: DatabaseCatalogBootstrapProgress) {
        self.publish_poll_error(error, progress);
        self.schedule();
    }

    fn retire_input_one(self: &Arc<Self>) {
        let mut pages = self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(owner) = pages.as_mut() {
            match owner.close_step() {
                Ok(Some(_)) => {
                    drop(pages);
                    self.schedule();
                    return;
                }
                Ok(None) if owner.terminal_is_empty() => {
                    pages.take();
                    drop(pages);
                    self.schedule();
                    return;
                }
                Ok(None) => {
                    drop(pages);
                    self.stage_error(DbError::LimitExceeded("database catalog-bootstrap input retirement"), DatabaseCatalogBootstrapProgress::Fault);
                    return;
                }
                Err(error) => {
                    drop(pages);
                    self.stage_error(error, DatabaseCatalogBootstrapProgress::Fault);
                    return;
                }
            }
        }
        drop(pages);
        self.set_phase(DatabaseCatalogBootstrapPhase::Validate);
        self.schedule();
    }

    fn validate_one(self: &Arc<Self>) {
        let actual = if let Some(error) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            Err(error)
        } else if let Some(actual) = self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            match actual {
                Ok(actual) => match self.expected.epoch.checked_add(1) {
                    Some(expected) if actual.epoch == expected => Ok(actual),
                    Some(expected) => Err(DbError::Fenced { expected, actual: actual.epoch }),
                    None => Err(DbError::LimitExceeded("database catalog-bootstrap expected revision")),
                },
                Err(error) => Err(error),
            }
        } else {
            Err(DbError::LimitExceeded("database catalog-bootstrap result owner"))
        };
        *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(actual);
        self.set_phase(DatabaseCatalogBootstrapPhase::Publish);
        self.schedule();
    }

    pub fn publish_one(self: &Arc<Self>) {
        if !self.is_current() && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }));
            self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
        }
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let key = self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let actual = self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let completion = match (storage, key, actual) {
            (Some(storage), Some(key), Some(actual)) => {
                self.set_progress(match actual {
                    Ok(_) => DatabaseCatalogBootstrapProgress::Completed,
                    Err(DbError::Closed) => DatabaseCatalogBootstrapProgress::Cancelled,
                    Err(_) => DatabaseCatalogBootstrapProgress::Fault,
                });
                Ok(DatabaseCatalogBootstrapResult { state: Some(self.clone()), storage: Some(storage), key: Some(key), expected: self.expected, actual: Some(actual) })
            }
            (storage, key, actual) => {
                *self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = storage;
                *self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = key;
                *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = actual;
                self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
                Err(DbError::LimitExceeded("database catalog-bootstrap publication owner"))
            }
        };
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(completion);
        } else {
            *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(completion);
            if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                waker.wake();
            }
        }
        self.set_phase(DatabaseCatalogBootstrapPhase::Terminal);
    }

    pub fn retained_owner_count(&self) -> usize {
        usize::from(self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
    }

    pub fn roots_are_empty(&self) -> bool {
        self.retained_owner_count() == 0
    }

    fn retire_one(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() || self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return;
        }
        {
            let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(step) = work.as_mut().map(DatabaseCatalogBootstrapWork::close_one) {
                match step {
                    Ok(true) => return,
                    Ok(false) => {}
                    Err(error) => {
                        *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error);
                        self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
                        return;
                    }
                }
            }
            if work.as_ref().is_some_and(DatabaseCatalogBootstrapWork::terminal_is_empty) {
                work.take();
                return;
            }
        }
        {
            let mut pages = self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(owner) = pages.as_mut() {
                if owner.close_step().ok().flatten().is_some() {
                    return;
                }
                if owner.terminal_is_empty() {
                    pages.take();
                    return;
                }
            }
        }
        let retained = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
        if let Some(mut result) = retained {
            if let Ok(owner) = result.as_mut() {
                owner.close_one();
                if !owner.terminal_is_empty() {
                    *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                }
            }
            return;
        }
        if self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
        {
            return;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            let mut registry = database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
                registry[self.slot] = None;
            }
            return;
        }
        self.finished.store(true, Ordering::Release);
    }

    pub fn release_success(&self) {
        if !self.roots_are_empty() && self.retained_owner_count() != 1 {
            return;
        }
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() && self.driver_authority.load(std::sync::atomic::Ordering::Acquire) == DatabaseCatalogBootstrapDriverAuthority::Idle as u8
    }
}

/// ⏳️ Public check-register-recheck future for one mounted initial-catalog CAS.
pub struct DatabaseCatalogBootstrapFuture {
    pub state: Arc<DatabaseCatalogBootstrapState>,
    pub resolved: bool,
}

impl DatabaseCatalogBootstrapFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, expected: EpochFence) -> Result<Self, DatabaseCatalogBootstrapRejected> {
        Self::try_prepare_with_key(pool, storage, pages, DatabaseCatalogBootstrapKey::root(), expected, true)
    }

    fn try_submit_with_key(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, key: DatabaseCatalogBootstrapKey, expected: EpochFence) -> Result<Self, DatabaseCatalogBootstrapRejected> {
        Self::try_prepare_with_key(pool, storage, pages, key, expected, true)
    }

    fn try_prepare_with_key(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages, key: DatabaseCatalogBootstrapKey, expected: EpochFence, schedule: bool) -> Result<Self, DatabaseCatalogBootstrapRejected> {
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
