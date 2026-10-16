//! 🏗️ Db engine retained create-document catalog operation.

use db_durability::EpochFence;
use db_engine_base::{CatalogEntry, CatalogState, DatabaseCreateCatalogToken, decimal_u64};
use db_ids::{ArtifactId, DbError, GenerationId};
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::{encode_catalog_pages, test_worker_pool};

//#region 🔖️CreateDocumentCatalogCas
const DATABASE_CREATE_CATALOG_SLOTS: usize = 32;

const DATABASE_CREATE_CATALOG_MAX_ENTRIES: usize = 4_096;

const DATABASE_CREATE_CATALOG_MAX_ID_BYTES: usize = db_io_core::DbIoText::maximum_capacity();

const DATABASE_CREATE_CATALOG_MAX_PAGES: usize = db_io_core::DB_IO_OPERATION_PAGES;

const DATABASE_CREATE_CATALOG_COPY_BYTES: usize = 256;

const DATABASE_CREATE_CATALOG_ITEMS: u64 = (DATABASE_CREATE_CATALOG_MAX_ENTRIES * 4 + DATABASE_CREATE_CATALOG_MAX_PAGES + 64) as u64;

const DATABASE_CREATE_CATALOG_BYTES: u64 = (DATABASE_CREATE_CATALOG_MAX_ENTRIES * 2 * (DATABASE_CREATE_CATALOG_MAX_ID_BYTES + size_of::<CatalogEntry>()) + DATABASE_CREATE_CATALOG_MAX_PAGES * db_io_core::DB_IO_PAGE_BYTES + 128 * 1024) as u64;

const DATABASE_CREATE_CATALOG_TOTAL_ITEMS: u64 = DATABASE_CREATE_CATALOG_ITEMS * DATABASE_CREATE_CATALOG_SLOTS as u64;

const DATABASE_CREATE_CATALOG_TOTAL_BYTES: u64 = DATABASE_CREATE_CATALOG_BYTES * DATABASE_CREATE_CATALOG_SLOTS as u64;

const DATABASE_CREATE_CATALOG_RETRY_LIMIT: u8 = 8;

const DATABASE_CREATE_CATALOG_DEADLINE_MS: u64 = 30_000;

const DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES: usize = size_of::<Vec<CatalogEntry>>() + size_of::<usize>() * 2;

#[derive(Clone, Copy)]
struct DatabaseCreateCatalogBackingLedger {
    pub items: u64,
    pub bytes: u64,
}

impl DatabaseCreateCatalogBackingLedger {
    pub fn new(document_capacity: usize, base_capacity: usize) -> Result<Self, DbError> {
        let base_bytes = base_capacity.checked_mul(size_of::<CatalogEntry>()).ok_or(DbError::LimitExceeded("database create-catalog base backing bytes"))?;
        let bytes = document_capacity.checked_add(base_bytes).and_then(|bytes| bytes.checked_add(DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES)).ok_or(DbError::LimitExceeded("database create-catalog initial backing bytes"))?;
        let items = u64::try_from(base_capacity).ok().and_then(|items| items.checked_add(3)).ok_or(DbError::LimitExceeded("database create-catalog initial backing items"))?;
        let bytes = u64::try_from(bytes).map_err(|_| DbError::LimitExceeded("database create-catalog initial backing bytes"))?;
        let ledger = Self { items, bytes };
        ledger.validate()?;
        Ok(ledger)
    }

    pub fn observe(&mut self, items: u64, bytes: usize) -> Result<(), DbError> {
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded("database create-catalog observed backing items"))?;
        let bytes = u64::try_from(bytes).map_err(|_| DbError::LimitExceeded("database create-catalog observed backing bytes"))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database create-catalog observed backing bytes"))?;
        let next = Self { items: next_items, bytes: next_bytes };
        next.validate()?;
        *self = next;
        Ok(())
    }

    fn validate(&self) -> Result<(), DbError> {
        if self.items > DATABASE_CREATE_CATALOG_ITEMS || self.bytes > DATABASE_CREATE_CATALOG_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog observed backing capacity"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct DatabaseCreateCatalogAdmissionSlot {
    pub generation: u64,
    pub occupied: bool,
}

const EMPTY_DATABASE_CREATE_CATALOG_SLOT: DatabaseCreateCatalogAdmissionSlot = DatabaseCreateCatalogAdmissionSlot { generation: 0, occupied: false };

struct DatabaseCreateCatalogAdmissionState {
    pub slots: [DatabaseCreateCatalogAdmissionSlot; DATABASE_CREATE_CATALOG_SLOTS],
    pub items: u64,
    pub bytes: u64,
    pub next_generation: u64,
}

impl DatabaseCreateCatalogAdmissionState {
    pub fn try_claim(&mut self, document: &protocol::ArtifactId) -> Result<(usize, u64), DbError> {
        if document.0.capacity() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES || document.0.len() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog document bytes"));
        }
        let Some(slot) = self.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::LimitExceeded("database create-catalog slot capacity"));
        };
        let Some(items) = self.items.checked_add(DATABASE_CREATE_CATALOG_ITEMS) else {
            return Err(DbError::LimitExceeded("database create-catalog aggregate items"));
        };
        let Some(bytes) = self.bytes.checked_add(DATABASE_CREATE_CATALOG_BYTES) else {
            return Err(DbError::LimitExceeded("database create-catalog aggregate bytes"));
        };
        if items > DATABASE_CREATE_CATALOG_TOTAL_ITEMS || bytes > DATABASE_CREATE_CATALOG_TOTAL_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog aggregate capacity"));
        }
        let generation = self.next_generation;
        let Some(next_generation) = generation.checked_add(1) else {
            return Err(DbError::LimitExceeded("database create-catalog generation"));
        };
        self.next_generation = next_generation;
        self.slots[slot] = DatabaseCreateCatalogAdmissionSlot { generation, occupied: true };
        self.items = items;
        self.bytes = bytes;
        Ok((slot, generation))
    }

    pub fn is_current(&self, slot: usize, generation: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
    }

    pub fn release(&mut self, slot: usize, generation: u64) -> bool {
        if !self.is_current(slot, generation) {
            return false;
        }
        let Some(items) = self.items.checked_sub(DATABASE_CREATE_CATALOG_ITEMS) else { return false };
        let Some(bytes) = self.bytes.checked_sub(DATABASE_CREATE_CATALOG_BYTES) else { return false };
        self.slots[slot] = EMPTY_DATABASE_CREATE_CATALOG_SLOT;
        self.items = items;
        self.bytes = bytes;
        true
    }
}

static DATABASE_CREATE_CATALOG_ADMISSION: Mutex<DatabaseCreateCatalogAdmissionState> =
    Mutex::new(DatabaseCreateCatalogAdmissionState { slots: [EMPTY_DATABASE_CREATE_CATALOG_SLOT; DATABASE_CREATE_CATALOG_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseCreateCatalogAdmission {
    pub slot: usize,
    pub generation: u64,
}

impl DatabaseCreateCatalogAdmission {
    pub fn try_claim(document: &protocol::ArtifactId) -> Result<Self, DbError> {
        let (slot, generation) = DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(document)?;
        Ok(Self { slot, generation })
    }

    pub fn is_current(&self) -> bool {
        DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation)
    }
}

impl Drop for DatabaseCreateCatalogAdmission {
    fn drop(&mut self) {
        DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCreateCatalogProgress {
    Admitted,
    Scanning,
    Copying,
    Encoding,
    Polling,
    Revalidating,
    Completed,
    Cancelled,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCreateCatalogCloseStep {
    Progress,
    Blocked,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatabaseCreateCatalogTerminalWitness {
    pub generation: u64,
    pub retained_owners: usize,
    pub terminal_empty: bool,
}

pub struct DatabaseCreateCatalogResult {
    pub state: Option<Arc<DatabaseCreateCatalogState>>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub document: Option<protocol::ArtifactId>,
    pub expected: EpochFence,
    pub actual: Option<Result<EpochFence, DbError>>,
}

impl std::fmt::Debug for DatabaseCreateCatalogResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DatabaseCreateCatalogResult")
            .field("generation", &self.state.as_ref().map(|state| state.generation))
            .field("storage", &self.storage.is_some())
            .field("document", &self.document)
            .field("expected", &self.expected)
            .field("actual", &self.actual)
            .finish()
    }
}

impl DatabaseCreateCatalogResult {
    pub fn into_parts(mut self) -> Result<(Arc<db_storage::DbBackend>, protocol::ArtifactId, EpochFence, Result<EpochFence, DbError>), Self> {
        let storage = self.storage.take();
        let document = self.document.take();
        let actual = self.actual.take();
        match (storage, document, actual) {
            (Some(storage), Some(document), Some(actual)) => {
                if let Some(state) = self.state.take() {
                    state.release_success();
                }
                Ok((storage, document, self.expected, actual))
            }
            (storage, document, actual) => {
                self.storage = storage;
                self.document = document;
                self.actual = actual;
                Err(self)
            }
        }
    }

    pub fn close_one(&mut self) -> bool {
        if self.actual.take().is_some() {
            true
        } else if self.document.take().is_some() {
            true
        } else {
            self.storage.take().is_some()
        }
    }

    pub fn retained_owner_count(outcome: &Result<Self, DbError>) -> usize {
        outcome.as_ref().map_or(1, |result| usize::from(result.storage.is_some()) + usize::from(result.document.is_some()) + usize::from(result.actual.is_some()))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.document.is_none() && self.actual.is_none()
    }
}

impl Drop for DatabaseCreateCatalogResult {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let Some(state) = self.state.take() else { return };
        let owner = Self { state: None, storage: self.storage.take(), document: self.document.take(), expected: self.expected, actual: self.actual.take() };
        *state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(owner));
        state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        state.closing.store(true, std::sync::atomic::Ordering::Release);
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        state.begin_callback_close();
    }
}

struct DatabaseCreateCatalogRejectedOwner {
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub document: Option<protocol::ArtifactId>,
}

impl DatabaseCreateCatalogRejectedOwner {
    pub fn close_one(&mut self) -> bool {
        self.document.take().is_some() || self.storage.take().is_some()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.document.is_none()
    }
}

struct DatabaseCreateCatalogRejectedClose {
    pub pool: Arc<WorkerPool>,
    pub owner: Mutex<Option<DatabaseCreateCatalogRejectedOwner>>,
    pub driver: std::sync::atomic::AtomicU8,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<semio_framework_async::Job>>,
    pub deadline_ms: u64,
    callback_close: std::sync::atomic::AtomicBool,
    callback_armed: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    spent_submission_attempts: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    callback_worker_thread: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    terminal_job_retirements: std::sync::atomic::AtomicUsize,
}

impl DatabaseCreateCatalogRejectedClose {
    pub fn prepare(pool: Arc<WorkerPool>, owner: DatabaseCreateCatalogRejectedOwner) -> Arc<Self> {
        let deadline_ms = pool.now_ms().checked_add(DATABASE_CREATE_CATALOG_DEADLINE_MS).unwrap_or(u64::MAX);
        Arc::new(Self {
            pool,
            owner: Mutex::new(Some(owner)),
            driver: std::sync::atomic::AtomicU8::new(DatabaseCreateCatalogDriverAuthority::Idle as u8),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            deadline_ms,
            callback_close: std::sync::atomic::AtomicBool::new(false),
            callback_armed: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            spent_submission_attempts: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            callback_worker_thread: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            terminal_job_retirements: std::sync::atomic::AtomicUsize::new(0),
        })
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.callback_close.load(Ordering::Acquire) {
            self.arm_callback_close();
            return;
        }
        if self.driver.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()), 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next_attempt = worker_submit_retry_attempt(error.kind(), attempt, DATABASE_CREATE_CATALOG_RETRY_LIMIT).unwrap_or(DATABASE_CREATE_CATALOG_RETRY_LIMIT);
                #[cfg(test)]
                if next_attempt != attempt {
                    self.spent_submission_attempts.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                }
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next_attempt));
                if self.driver.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let state = self.clone();
                    self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
                }
            }
        }
    }

    pub fn retry(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.callback_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT || self.pool.now_ms() >= self.deadline_ms {
            if self.driver.compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                self.callback_close.store(true, Ordering::Release);
                self.drive_close_claimed();
            } else {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
            return;
        }
        if self.driver.compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
        } else {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        }
    }

    pub fn drive_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.driver.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let pending = self.close_one();
        self.driver.store(DatabaseCreateCatalogDriverAuthority::Idle as u8, Ordering::Release);
        if pending {
            self.schedule();
        }
    }

    pub fn close_one(&self) -> bool {
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            #[cfg(test)]
            self.terminal_job_retirements.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            return true;
        }
        let mut owner = self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(owner) = owner.as_mut() {
            owner.close_one();
        }
        if owner.as_ref().is_some_and(DatabaseCreateCatalogRejectedOwner::terminal_is_empty) {
            owner.take();
        }
        owner.is_some()
    }

    fn drive_close_claimed(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        let pending = self.close_one();
        self.driver.store(DatabaseCreateCatalogDriverAuthority::Idle as u8, Ordering::Release);
        if pending {
            self.arm_callback_close();
        } else {
            self.callback_close.store(false, Ordering::Release);
        }
    }

    fn arm_callback_close(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.callback_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.callback_close_one());
    }

    fn callback_close_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.callback_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        self.callback_armed.store(false, Ordering::Release);
        if !self.callback_close.load(Ordering::Acquire) {
            return;
        }
        if self.driver.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            self.arm_callback_close();
            return;
        }
        self.drive_close_claimed();
    }

    pub fn take_owner(&self) -> Option<DatabaseCreateCatalogRejectedOwner> {
        self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }

    pub fn restore_owner(&self, owner: DatabaseCreateCatalogRejectedOwner) {
        *self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.callback_close.load(std::sync::atomic::Ordering::Acquire)
            && !self.callback_armed.load(std::sync::atomic::Ordering::Acquire)
            && self.driver.load(std::sync::atomic::Ordering::Acquire) == DatabaseCreateCatalogDriverAuthority::Idle as u8
    }
}

pub struct DatabaseCreateCatalogRejected {
    pub error: Option<DbError>,
    pub close: Arc<DatabaseCreateCatalogRejectedClose>,
}

impl std::fmt::Debug for DatabaseCreateCatalogRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCreateCatalogRejected").field("error", &self.error).field("terminal_empty", &self.close.terminal_is_empty()).finish()
    }
}

impl DatabaseCreateCatalogRejected {
    pub fn new(pool: Arc<WorkerPool>, error: DbError, storage: Arc<db_storage::DbBackend>, document: protocol::ArtifactId) -> Self {
        Self { error: Some(error), close: DatabaseCreateCatalogRejectedClose::prepare(pool, DatabaseCreateCatalogRejectedOwner { storage: Some(storage), document: Some(document) }) }
    }

    pub fn into_parts(mut self) -> Result<(DbError, Arc<db_storage::DbBackend>, protocol::ArtifactId), Self> {
        let error = self.error.take();
        let mut owner = self.close.take_owner().unwrap_or(DatabaseCreateCatalogRejectedOwner { storage: None, document: None });
        let storage = owner.storage.take();
        let document = owner.document.take();
        match (error, storage, document) {
            (Some(error), Some(storage), Some(document)) => Ok((error, storage, document)),
            (error, storage, document) => {
                self.error = error;
                owner.storage = storage;
                owner.document = document;
                self.close.restore_owner(owner);
                Err(self)
            }
        }
    }

    pub fn close_and_take_error(mut self) -> DbError {
        let error = self.error.take().unwrap_or(DbError::LimitExceeded("database create-catalog rejection error"));
        self.close.schedule();
        error
    }
}

impl Drop for DatabaseCreateCatalogRejected {
    fn drop(&mut self) {
        if !self.close.terminal_is_empty() {
            self.close.schedule();
        }
    }
}

type DatabaseCreateCatalogBackendFuture = std::pin::Pin<Box<dyn Future<Output = Result<EpochFence, DbError>> + Send + 'static>>;

struct DatabaseCreateCatalogWork {
    pub future: Option<DatabaseCreateCatalogBackendFuture>,
    pub storage: Option<Arc<db_storage::DbBackend>>,
    pub pages: Option<db_io_core::DbIoPages>,
    pub expected: EpochFence,
    #[cfg(any(test, feature = "testkit"))]
    pub storage_identity: usize,
    #[cfg(any(test, feature = "testkit"))]
    pub page_identity: Option<u64>,
}

impl DatabaseCreateCatalogWork {
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
                    return std::task::Poll::Ready(Err(DbError::LimitExceeded("database create-catalog work input")));
                }
            }
        }
        self.future.as_mut().map_or(std::task::Poll::Ready(Err(DbError::LimitExceeded("database create-catalog backend future"))), |future| future.as_mut().poll(context))
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
    pub fn controlled(future: DatabaseCreateCatalogBackendFuture, storage_identity: usize, page_identity: u64) -> Self {
        Self { future: Some(future), storage: None, pages: None, expected: EpochFence::INITIAL, storage_identity, page_identity: Some(page_identity) }
    }
}

struct DatabaseCreateCatalogEncodeCursor {
    pub entry: usize,
    pub byte: usize,
    pub phase: u8,
    pub pending: [u8; 32],
    pending_len: u8,
    pending_cursor: u8,
}

impl DatabaseCreateCatalogEncodeCursor {
    pub fn new() -> Self {
        Self { entry: 0, byte: 0, phase: 0, pending: [0; 32], pending_len: 0, pending_cursor: 0 }
    }

    fn set_pending(&mut self, source: &[u8]) -> Result<(), DbError> {
        let target = self.pending.get_mut(..source.len()).ok_or(DbError::LimitExceeded("database create-catalog encode fragment"))?;
        target.copy_from_slice(source);
        self.pending_len = source.len() as u8;
        self.pending_cursor = 0;
        Ok(())
    }

    fn flush_one(&mut self, writer: &mut db_io_core::DbIoPageWriter) -> Result<(), DbError> {
        let start = usize::from(self.pending_cursor);
        let end = usize::from(self.pending_len);
        let written = writer.write_fragment(&self.pending[start..end])?;
        if written == 0 && start != end {
            return Err(DbError::LimitExceeded("database create-catalog writer progress"));
        }
        self.pending_cursor = self.pending_cursor.checked_add(written as u8).ok_or(DbError::LimitExceeded("database create-catalog encode cursor"))?;
        if self.pending_cursor == self.pending_len {
            self.pending_cursor = 0;
            self.pending_len = 0;
        }
        Ok(())
    }

    pub fn step(&mut self, entries: &[CatalogEntry], writer: &mut db_io_core::DbIoPageWriter) -> Result<bool, DbError> {
        if self.pending_cursor < self.pending_len {
            self.flush_one(writer)?;
            return Ok(false);
        }
        match self.phase {
            0 => {
                self.set_pending(b"[")?;
                self.phase = 1;
            }
            1 if self.entry == entries.len() => {
                self.set_pending(b"]")?;
                self.phase = 7;
            }
            1 => {
                self.set_pending(if self.entry == 0 { b"{\"document\":\"" } else { b",{\"document\":\"" })?;
                self.byte = 0;
                self.phase = 2;
            }
            2 => {
                let source = entries[self.entry].document.0.as_bytes();
                if self.byte == source.len() {
                    self.set_pending(b"\",\"created_at_ms\":")?;
                    self.phase = 4;
                } else {
                    let byte = source[self.byte];
                    self.byte += 1;
                    let mut escape = [0u8; 6];
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    let encoded: &[u8] = match byte {
                        b'"' => b"\\\"",
                        b'\\' => b"\\\\",
                        0x08 => b"\\b",
                        0x0c => b"\\f",
                        b'\n' => b"\\n",
                        b'\r' => b"\\r",
                        b'\t' => b"\\t",
                        value @ 0x00..=0x1f => {
                            escape.copy_from_slice(&[b'\\', b'u', b'0', b'0', HEX[(value >> 4) as usize], HEX[(value & 0x0f) as usize]]);
                            &escape
                        }
                        _ => std::slice::from_ref(&byte),
                    };
                    self.set_pending(encoded)?;
                }
            }
            4 => {
                let mut decimal = [0u8; 20];
                self.set_pending(decimal_u64(entries[self.entry].created_at_ms, &mut decimal))?;
                self.phase = 5;
            }
            5 => {
                self.set_pending(b"}")?;
                self.entry += 1;
                self.phase = 1;
            }
            7 => return Ok(true),
            _ => return Err(DbError::LimitExceeded("database create-catalog encode phase")),
        }
        self.flush_one(writer)?;
        Ok(false)
    }
}

struct DatabaseCreateCatalogCursor {
    pub base: Option<Arc<Vec<CatalogEntry>>>,
    backing: DatabaseCreateCatalogBackingLedger,
    base_epoch: EpochFence,
    base_revision: u64,
    base_identity: usize,
    pub created_at_ms: u64,
    pub encoded_len: usize,
    scan_entry: usize,
    scan_byte: usize,
    scan_started: bool,
    candidate: Option<Vec<CatalogEntry>>,
    clone_entry: usize,
    clone_byte: usize,
    clone_text: Option<String>,
    pub snapshot: Option<Arc<Vec<CatalogEntry>>>,
    encode: DatabaseCreateCatalogEncodeCursor,
    pub writer: Option<db_io_core::DbIoPageWriter>,
    pub pages: Option<db_io_core::DbIoPages>,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCreateCatalogPhase {
    Scan,
    Reserve,
    Clone,
    Snapshot,
    Encode,
    Seal,
    Claim,
    Handoff,
    Poll,
    CloseWork,
    Revalidate,
    Retire,
    Publish,
    Terminal,
}

impl DatabaseCreateCatalogPhase {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Scan,
            1 => Self::Reserve,
            2 => Self::Clone,
            3 => Self::Snapshot,
            4 => Self::Encode,
            5 => Self::Seal,
            6 => Self::Claim,
            7 => Self::Handoff,
            8 => Self::Poll,
            9 => Self::CloseWork,
            10 => Self::Revalidate,
            11 => Self::Retire,
            12 => Self::Publish,
            _ => Self::Terminal,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCreateCatalogDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

struct DatabaseCreateCatalogState {
    pub pool: Arc<WorkerPool>,
    pub pool_use: Mutex<Option<Arc<WorkerPoolUse>>>,
    pub catalog: Arc<Mutex<CatalogState>>,
    pub slot: usize,
    pub generation: u64,
    pub admission: Mutex<Option<DatabaseCreateCatalogAdmission>>,
    pub storage: Mutex<Option<Arc<db_storage::DbBackend>>>,
    pub document: Mutex<Option<protocol::ArtifactId>>,
    pub cursor: Mutex<DatabaseCreateCatalogCursor>,
    pub work: Mutex<Option<DatabaseCreateCatalogWork>>,
    pub poll_work: Mutex<Option<DatabaseCreateCatalogWork>>,
    pub terminal_work: Mutex<Option<DatabaseCreateCatalogWork>>,
    pub outcome: Mutex<Option<Result<EpochFence, DbError>>>,
    pub completion: Mutex<Option<Result<DatabaseCreateCatalogResult, DbError>>>,
    pub terminal_completion: Mutex<Option<Result<DatabaseCreateCatalogResult, DbError>>>,
    pub retry_job: Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub terminal_job: Mutex<Option<semio_framework_async::Job>>,
    pub waker: Mutex<Option<std::task::Waker>>,
    pub driver_authority: std::sync::atomic::AtomicU8,
    pub polling: std::sync::atomic::AtomicBool,
    pub wake_requested: std::sync::atomic::AtomicBool,
    pub cancelled: std::sync::atomic::AtomicBool,
    pub abandoned: std::sync::atomic::AtomicBool,
    pub closing: std::sync::atomic::AtomicBool,
    pub finished: std::sync::atomic::AtomicBool,
    pending_owned: std::sync::atomic::AtomicBool,
    pub terminal_checked_out: std::sync::atomic::AtomicBool,
    retry_closing: std::sync::atomic::AtomicBool,
    callback_close_armed: std::sync::atomic::AtomicBool,
    catalog_contention_armed: std::sync::atomic::AtomicBool,
    pub phase: std::sync::atomic::AtomicU8,
    pub progress: std::sync::atomic::AtomicU8,
    pub deadline_ms: std::sync::atomic::AtomicU64,
    opportunities: std::sync::atomic::AtomicU64,
    #[cfg(any(test, feature = "testkit"))]
    pub active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub max_active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub poll_worker_thread: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    spent_submission_attempts: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    backend_polls: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    callback_worker_thread: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    terminal_job_retirements: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    controlled_capacity_overage: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    controlled_driver_hook: Mutex<Option<Arc<dyn Fn(DatabaseCreateCatalogPhase) + Send + Sync>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_publication_before_waker_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

struct DatabaseCreateCatalogWake {
    pub state: std::sync::Weak<DatabaseCreateCatalogState>,
    pub generation: u64,
}

pub fn database_create_catalog_registry() -> &'static Mutex<[Option<Arc<DatabaseCreateCatalogState>>; DATABASE_CREATE_CATALOG_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<Mutex<[Option<Arc<DatabaseCreateCatalogState>>; DATABASE_CREATE_CATALOG_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCreateCatalogWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let Some(state) = self.state.upgrade() else { return };
        if state.generation != self.generation || !state.is_current() {
            return;
        }
        state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        state.schedule();
    }
}

impl DatabaseCreateCatalogState {
    pub fn phase(&self) -> DatabaseCreateCatalogPhase {
        DatabaseCreateCatalogPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCreateCatalogPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
    }

    fn accepts_interruption(&self) -> bool {
        match self.phase() {
            DatabaseCreateCatalogPhase::Publish | DatabaseCreateCatalogPhase::Terminal => false,
            DatabaseCreateCatalogPhase::CloseWork | DatabaseCreateCatalogPhase::Revalidate | DatabaseCreateCatalogPhase::Retire => {
                !self.closing.load(std::sync::atomic::Ordering::Acquire) && self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            }
            _ => true,
        }
    }

    pub fn set_progress(&self, progress: DatabaseCreateCatalogProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCreateCatalogAdmission::is_current)
    }

    pub fn observed_generation(&self) -> u64 {
        DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(self.slot).map_or(0, |entry| entry.generation)
    }

    fn observed_capacity(&self, capacity: usize) -> usize {
        #[cfg(test)]
        {
            return capacity.saturating_add(self.controlled_capacity_overage.swap(0, std::sync::atomic::Ordering::AcqRel));
        }
        #[cfg(not(test))]
        capacity
    }

    fn defer_catalog_contention(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.catalog_contention_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            state.catalog_contention_armed.store(false, Ordering::Release);
            if !state.finished.load(Ordering::Acquire) {
                state.wake_requested.store(true, Ordering::Release);
                state.schedule();
            }
        });
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) {
            return;
        }
        if self.retry_closing.load(Ordering::Acquire) {
            self.arm_callback_close();
            return;
        }
        while self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            self.wake_requested.store(true, Ordering::SeqCst);
            if self.driver_authority.load(Ordering::SeqCst) != DatabaseCreateCatalogDriverAuthority::Idle as u8 || !self.wake_requested.swap(false, Ordering::SeqCst) {
                return;
            }
        }
        self.wake_requested.swap(false, Ordering::SeqCst);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next_attempt = worker_submit_retry_attempt(error.kind(), attempt, DATABASE_CREATE_CATALOG_RETRY_LIMIT).unwrap_or(DATABASE_CREATE_CATALOG_RETRY_LIMIT);
                #[cfg(test)]
                if next_attempt != attempt {
                    self.spent_submission_attempts.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                }
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next_attempt));
                if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let state = self.clone();
                    self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
                }
            }
        }
    }

    pub fn retry(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.callback_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        let interruptible = self.accepts_interruption();
        let cancelled = self.cancelled.load(Ordering::Acquire);
        let exhausted = attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT;
        let terminal = if !self.is_current() {
            Some((DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCreateCatalogProgress::Fault))
        } else if cancelled && interruptible {
            Some((DbError::Closed, DatabaseCreateCatalogProgress::Cancelled))
        } else if self.pool.now_ms() >= self.deadline_ms.load(Ordering::Acquire) && interruptible {
            Some((DbError::Timeout(String::from("database create-catalog retry deadline")), DatabaseCreateCatalogProgress::Fault))
        } else if exhausted && interruptible {
            Some((DbError::LimitExceeded("database create-catalog retry exhausted"), DatabaseCreateCatalogProgress::Fault))
        } else {
            None
        };
        if terminal.is_some() || cancelled || exhausted {
            if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                self.retry_closing.store(true, Ordering::Release);
                if let Some((error, progress)) = terminal {
                    self.stage_error(error, progress);
                }
                self.drive_callback_close_claimed(true);
            } else {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
            return;
        }
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
        } else {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        }
    }

    fn arm_callback_close(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.callback_close_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.callback_close_one());
    }

    fn callback_close_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.callback_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        self.callback_close_armed.store(false, Ordering::Release);
        if !self.retry_closing.load(Ordering::Acquire) || self.finished.load(Ordering::Acquire) {
            return;
        }
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            self.arm_callback_close();
            return;
        }
        self.drive_callback_close_claimed(true);
    }

    fn drive_callback_close_claimed(self: Arc<Self>, autonomous: bool) {
        use std::sync::atomic::Ordering;
        self.opportunities.fetch_add(1, Ordering::AcqRel);
        let driven = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.drive_claimed(self.generation)));
        if driven.is_err() {
            self.stage_error(DbError::LimitExceeded("database create-catalog callback close panic"), DatabaseCreateCatalogProgress::Fault);
        }
        self.wake_requested.store(false, Ordering::Release);
        self.driver_authority.store(DatabaseCreateCatalogDriverAuthority::Idle as u8, Ordering::Release);
        let keep_closing = !self.finished.load(Ordering::Acquire) && (self.phase() != DatabaseCreateCatalogPhase::Terminal || self.closing.load(Ordering::Acquire));
        if keep_closing && autonomous {
            self.arm_callback_close();
        } else {
            self.retry_closing.store(false, Ordering::Release);
            if !keep_closing {
                self.callback_close_armed.store(false, Ordering::Release);
            }
        }
    }

    fn drive_explicit_close_one(self: &Arc<Self>) -> bool {
        use std::sync::atomic::Ordering;
        self.retry_closing.store(true, Ordering::Release);
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        self.clone().drive_callback_close_claimed(false);
        true
    }

    fn begin_callback_close(self: &Arc<Self>) {
        self.retry_closing.store(true, std::sync::atomic::Ordering::Release);
        self.arm_callback_close();
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        #[cfg(test)]
        {
            let active = self.active_drivers.fetch_add(1, Ordering::AcqRel) + 1;
            self.max_active_drivers.fetch_max(active, Ordering::AcqRel);
        }
        self.opportunities.fetch_add(1, Ordering::AcqRel);
        let driven = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            #[cfg(test)]
            if let Some(hook) = self.controlled_driver_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
                hook(self.phase());
            }
            self.drive_claimed(generation);
        }));
        if driven.is_err() {
            self.stage_error(DbError::LimitExceeded("database create-catalog worker panic"), DatabaseCreateCatalogProgress::Fault);
        }
        #[cfg(test)]
        self.active_drivers.fetch_sub(1, Ordering::AcqRel);
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Driving as u8, DatabaseCreateCatalogDriverAuthority::Idle as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            self.stage_error(DbError::LimitExceeded("database create-catalog driver release"), DatabaseCreateCatalogProgress::Fault);
        }
        if self.wake_requested.swap(false, Ordering::SeqCst) {
            self.schedule();
        }
    }

    pub fn drive_claimed(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) {
            return;
        }
        if self.phase() == DatabaseCreateCatalogPhase::Terminal {
            if self.closing.load(Ordering::Acquire) {
                self.retire_terminal_one();
            }
            return;
        }
        if !self.retry_closing.load(Ordering::Acquire) && (generation != self.generation || !self.is_current()) {
            self.stage_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.accepts_interruption() {
            self.stage_error(DbError::Closed, DatabaseCreateCatalogProgress::Cancelled);
            return;
        }
        if self.pool.now_ms() >= self.deadline_ms.load(Ordering::Acquire) && self.accepts_interruption() {
            self.stage_error(DbError::Timeout(String::from("database create-catalog deadline")), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        match self.phase() {
            DatabaseCreateCatalogPhase::Scan => self.scan_one(),
            DatabaseCreateCatalogPhase::Reserve => self.reserve_candidate_one(),
            DatabaseCreateCatalogPhase::Clone => self.clone_one(),
            DatabaseCreateCatalogPhase::Snapshot => self.snapshot_one(),
            DatabaseCreateCatalogPhase::Encode => self.encode_one(),
            DatabaseCreateCatalogPhase::Seal => self.seal_one(),
            DatabaseCreateCatalogPhase::Claim => self.claim_one(),
            DatabaseCreateCatalogPhase::Handoff => self.handoff_one(),
            DatabaseCreateCatalogPhase::Poll => self.poll_backend_once(generation),
            DatabaseCreateCatalogPhase::CloseWork => self.close_work_one(),
            DatabaseCreateCatalogPhase::Revalidate => self.revalidate_one(),
            DatabaseCreateCatalogPhase::Retire => self.retire_intermediate_one(),
            DatabaseCreateCatalogPhase::Publish => self.publish_one(),
            DatabaseCreateCatalogPhase::Terminal => {}
        }
    }

    pub fn stage_error(&self, error: DbError, progress: DatabaseCreateCatalogProgress) {
        let mut outcome = self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if outcome.is_none() || outcome.as_ref().is_some_and(Result::is_ok) {
            *outcome = Some(Err(error));
        }
        drop(outcome);
        self.set_progress(progress);
        self.set_phase(DatabaseCreateCatalogPhase::Retire);
    }

    fn add_encoded(cursor: &mut DatabaseCreateCatalogCursor, bytes: usize) -> Result<(), DbError> {
        let next = cursor.encoded_len.checked_add(bytes).ok_or(DbError::LimitExceeded("database create-catalog encoded bytes"))?;
        if next > DATABASE_CREATE_CATALOG_MAX_PAGES * db_io_core::DB_IO_PAGE_BYTES {
            return Err(DbError::LimitExceeded("database create-catalog page capacity"));
        }
        cursor.encoded_len = next;
        Ok(())
    }

    fn scan_one(&self) {
        self.set_progress(DatabaseCreateCatalogProgress::Scanning);
        let document_guard = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(document) = document_guard.as_ref() else {
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog document owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(base) = cursor.base.as_ref().map(Arc::clone) else {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog base owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let entry_count = base.len();
        let (source, source_capacity, created_at_ms, duplicate) = if cursor.scan_entry < entry_count {
            let entry = &base[cursor.scan_entry];
            (entry.document.0.as_str(), entry.document.0.capacity(), entry.created_at_ms, !cursor.scan_started && entry.document == *document)
        } else {
            (document.0.as_str(), document.0.capacity(), cursor.created_at_ms, false)
        };
        if source_capacity > DATABASE_CREATE_CATALOG_MAX_ID_BYTES || source.len() > DATABASE_CREATE_CATALOG_MAX_ID_BYTES {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog base document bytes"), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        if !cursor.scan_started {
            if cursor.scan_entry < entry_count {
                if let Err(error) = cursor.backing.observe(1, source_capacity) {
                    drop(cursor);
                    drop(document_guard);
                    self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            if duplicate {
                drop(cursor);
                drop(document_guard);
                self.stage_error(DbError::AlreadyExists(String::from("document already exists")), DatabaseCreateCatalogProgress::Fault);
                return;
            }
            let mut decimal = [0u8; 20];
            let fixed = usize::from(cursor.scan_entry != 0) + b"{\"document\":\"".len() + b"\",\"created_at_ms\":".len() + decimal_u64(created_at_ms, &mut decimal).len() + 1;
            if let Err(error) = Self::add_encoded(&mut cursor, fixed) {
                drop(cursor);
                drop(document_guard);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
            cursor.scan_started = true;
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.scan_byte < source.len() {
            let byte = source.as_bytes()[cursor.scan_byte];
            cursor.scan_byte += 1;
            let bytes = match byte {
                b'"' | b'\\' | 0x08 | 0x0c | b'\n' | b'\r' | b'\t' => 2,
                0x00..=0x1f => 6,
                _ => 1,
            };
            if let Err(error) = Self::add_encoded(&mut cursor, bytes) {
                drop(cursor);
                drop(document_guard);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        cursor.scan_entry += 1;
        cursor.scan_byte = 0;
        cursor.scan_started = false;
        if cursor.scan_entry > entry_count {
            self.set_phase(DatabaseCreateCatalogPhase::Reserve);
        } else {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        }
    }

    fn reserve_candidate_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(base_len) = cursor.base.as_ref().map(|base| base.len()) else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog reserve base"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let Some(capacity) = base_len.checked_add(1) else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog entry count"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut candidate = Vec::new();
        if candidate.try_reserve_exact(capacity).is_err() {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog entry backing"), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        let observed_capacity = self.observed_capacity(candidate.capacity());
        cursor.candidate = Some(candidate);
        let backing_bytes = match observed_capacity.checked_mul(size_of::<CatalogEntry>()) {
            Some(bytes) => bytes,
            None => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog candidate observed bytes"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        let observed_items = match u64::try_from(observed_capacity) {
            Ok(items) => items,
            Err(_) => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog candidate observed items"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        if let Err(error) = cursor.backing.observe(observed_items, backing_bytes) {
            drop(cursor);
            self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.clone_entry = 0;
        cursor.clone_byte = 0;
        self.set_phase(DatabaseCreateCatalogPhase::Clone);
    }

    fn clone_boundary(source: &str, start: usize) -> usize {
        let mut end = start.saturating_add(DATABASE_CREATE_CATALOG_COPY_BYTES).min(source.len());
        if !source.is_char_boundary(end) {
            end -= 1;
        }
        if !source.is_char_boundary(end) {
            end -= 1;
        }
        if !source.is_char_boundary(end) {
            end -= 1;
        }
        end
    }

    fn clone_one(&self) {
        self.set_progress(DatabaseCreateCatalogProgress::Copying);
        let document_guard = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(document) = document_guard.as_ref() else {
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog clone document"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(base) = cursor.base.as_ref().map(Arc::clone) else {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog clone base"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let (source, created_at_ms) = if cursor.clone_entry < base.len() {
            let entry = &base[cursor.clone_entry];
            (entry.document.0.as_str(), entry.created_at_ms)
        } else {
            (document.0.as_str(), cursor.created_at_ms)
        };
        if cursor.clone_text.is_none() {
            let mut text = String::new();
            if text.try_reserve_exact(source.len()).is_err() {
                drop(cursor);
                drop(document_guard);
                self.stage_error(DbError::LimitExceeded("database create-catalog string backing"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
            let observed_capacity = self.observed_capacity(text.capacity());
            cursor.clone_text = Some(text);
            if observed_capacity > DATABASE_CREATE_CATALOG_MAX_ID_BYTES {
                drop(cursor);
                drop(document_guard);
                self.stage_error(DbError::LimitExceeded("database create-catalog cloned string capacity"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
            if let Err(error) = cursor.backing.observe(1, observed_capacity) {
                drop(cursor);
                drop(document_guard);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.clone_byte < source.len() {
            let end = Self::clone_boundary(source, cursor.clone_byte);
            let start = cursor.clone_byte;
            cursor.clone_text.as_mut().unwrap_or_else(|| unreachable!()).push_str(&source[start..end]);
            cursor.clone_byte = end;
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        let text = cursor.clone_text.take().unwrap_or_default();
        let Some(candidate) = cursor.candidate.as_mut() else {
            drop(cursor);
            drop(document_guard);
            self.stage_error(DbError::LimitExceeded("database create-catalog candidate owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        candidate.push(CatalogEntry { document: protocol::ArtifactId(text), created_at_ms });
        cursor.clone_entry += 1;
        cursor.clone_byte = 0;
        if cursor.clone_entry > base.len() {
            self.set_phase(DatabaseCreateCatalogPhase::Snapshot);
        } else {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        }
    }

    fn snapshot_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(candidate) = cursor.candidate.take() else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog candidate snapshot"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let pages = cursor.encoded_len.div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        if pages == 0 || pages > DATABASE_CREATE_CATALOG_MAX_PAGES {
            cursor.candidate = Some(candidate);
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog page count"), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.snapshot = Some(Arc::new(candidate));
        if let Err(error) = cursor.backing.observe(1, DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES) {
            drop(cursor);
            self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.writer = match db_io_core::DbIoPageWriter::try_reserve(pages) {
            Ok(writer) => Some(writer),
            Err(rejected) => {
                let error = rejected.into_error();
                drop(cursor);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        let page_bytes = match pages.checked_mul(db_io_core::DB_IO_PAGE_BYTES) {
            Some(bytes) => bytes,
            None => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog observed page bytes"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        let page_items = match u64::try_from(pages) {
            Ok(items) => items,
            Err(_) => {
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog observed page items"), DatabaseCreateCatalogProgress::Fault);
                return;
            }
        };
        if let Err(error) = cursor.backing.observe(page_items, page_bytes) {
            drop(cursor);
            self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        cursor.encode = DatabaseCreateCatalogEncodeCursor::new();
        self.set_phase(DatabaseCreateCatalogPhase::Encode);
    }

    fn encode_one(&self) {
        self.set_progress(DatabaseCreateCatalogProgress::Encoding);
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let snapshot = cursor.snapshot.as_ref().map(Arc::clone);
        let Some(snapshot) = snapshot else {
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog encode snapshot"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let mut encode = std::mem::replace(&mut cursor.encode, DatabaseCreateCatalogEncodeCursor::new());
        let step = cursor.writer.as_mut().ok_or(DbError::LimitExceeded("database create-catalog writer owner")).and_then(|writer| encode.step(snapshot.as_slice(), writer));
        cursor.encode = encode;
        match step {
            Ok(true) => self.set_phase(DatabaseCreateCatalogPhase::Seal),
            Ok(false) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => {
                drop(cursor);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    fn seal_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let step = cursor.writer.as_mut().ok_or(DbError::LimitExceeded("database create-catalog seal writer")).and_then(db_io_core::DbIoPageWriter::seal_retained_step);
        match step {
            Ok(Some(pages)) => {
                cursor.writer.take();
                cursor.pages = Some(pages);
                self.set_phase(DatabaseCreateCatalogPhase::Claim);
            }
            Ok(None) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => {
                drop(cursor);
                self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    fn claim_one(self: &Arc<Self>) {
        let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let token = DatabaseCreateCatalogToken { slot: self.slot, generation: self.generation, revision: cursor.base_revision };
        let mut catalog = match self.catalog.try_lock() {
            Ok(catalog) => catalog,
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                drop(cursor);
                self.defer_catalog_contention();
                return;
            }
        };
        if catalog.pending.is_some() && catalog.pending != Some(token) {
            drop(catalog);
            drop(cursor);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if catalog.revision != cursor.base_revision || catalog.epoch != cursor.base_epoch || Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity {
            let actual = catalog.epoch.epoch;
            let expected = cursor.base_epoch.epoch;
            drop(catalog);
            drop(cursor);
            self.stage_error(DbError::Fenced { expected, actual }, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        catalog.pending = Some(token);
        self.pending_owned.store(true, std::sync::atomic::Ordering::Release);
        drop(catalog);
        drop(cursor);
        self.set_phase(DatabaseCreateCatalogPhase::Handoff);
    }

    pub fn handoff_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let pages = cursor.pages.take();
        let expected = cursor.base_epoch;
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(Arc::clone);
        match (storage, pages) {
            (Some(storage), Some(pages)) => {
                *self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCreateCatalogWork::new(storage, pages, expected));
                self.set_phase(DatabaseCreateCatalogPhase::Poll);
            }
            (storage, pages) => {
                drop(storage);
                cursor.pages = pages;
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog handoff owner"), DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) else {
            self.stage_error(DbError::LimitExceeded("database create-catalog poll owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        if self.polling.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.wake_requested.store(true, Ordering::Release);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.polling.store(false, Ordering::Release);
            self.stage_error(DbError::Closed, DatabaseCreateCatalogProgress::Cancelled);
            return;
        }
        #[cfg(test)]
        self.poll_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        self.set_progress(DatabaseCreateCatalogProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCreateCatalogWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        #[cfg(test)]
        self.backend_polls.fetch_add(1, Ordering::AcqRel);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.polling.store(false, Ordering::Release);
                if self.cancelled.load(Ordering::Acquire) {
                    self.stage_error(DbError::Closed, DatabaseCreateCatalogProgress::Cancelled);
                }
            }
            Ok(std::task::Poll::Ready(actual)) => {
                *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(actual);
                self.polling.store(false, Ordering::Release);
                self.set_phase(DatabaseCreateCatalogPhase::CloseWork);
            }
            Err(_) => {
                *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.polling.store(false, Ordering::Release);
                self.stage_error(DbError::LimitExceeded("database create-catalog backend poll panic"), DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    fn close_work_one(&self) {
        let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let step = work.as_mut().map(DatabaseCreateCatalogWork::close_one);
        let empty = work.as_ref().is_some_and(DatabaseCreateCatalogWork::terminal_is_empty);
        if empty {
            work.take();
        }
        drop(work);
        match step {
            Some(Ok(_)) if empty => {
                if self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(Result::is_ok) {
                    self.set_phase(DatabaseCreateCatalogPhase::Revalidate);
                } else {
                    self.set_phase(DatabaseCreateCatalogPhase::Retire);
                }
            }
            Some(Ok(true)) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
            Some(Err(error)) => self.stage_error(error, DatabaseCreateCatalogProgress::Fault),
            _ => self.stage_error(DbError::LimitExceeded("database create-catalog close work"), DatabaseCreateCatalogProgress::Fault),
        }
    }

    fn revalidate_one(self: &Arc<Self>) {
        self.set_progress(DatabaseCreateCatalogProgress::Revalidating);
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let token = DatabaseCreateCatalogToken { slot: self.slot, generation: self.generation, revision: cursor.base_revision };
        let actual = self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().and_then(|actual| actual.as_ref().ok()).copied();
        let expected_next = cursor.base_epoch.epoch.checked_add(1);
        let mut catalog = match self.catalog.try_lock() {
            Ok(catalog) => catalog,
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                drop(cursor);
                self.defer_catalog_contention();
                return;
            }
        };
        if catalog.pending != Some(token) || catalog.revision != cursor.base_revision || catalog.epoch != cursor.base_epoch || Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity || actual.map(|actual| actual.epoch) != expected_next {
            let expected = cursor.base_epoch.epoch;
            let actual = actual.map_or(catalog.epoch.epoch, |actual| actual.epoch);
            if catalog.pending == Some(token) {
                catalog.pending = None;
            }
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            drop(catalog);
            drop(cursor);
            self.stage_error(DbError::Fenced { expected, actual }, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        let Some(snapshot) = cursor.snapshot.take() else {
            catalog.pending = None;
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            drop(catalog);
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog publication snapshot"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let Some(revision) = catalog.revision.checked_add(1) else {
            catalog.pending = None;
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            drop(catalog);
            cursor.snapshot = Some(snapshot);
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog publication revision"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        catalog.entries = snapshot;
        catalog.epoch = actual.unwrap_or(cursor.base_epoch);
        catalog.revision = revision;
        catalog.pending = None;
        self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
        drop(catalog);
        cursor.base.take();
        self.set_progress(DatabaseCreateCatalogProgress::Completed);
        self.set_phase(DatabaseCreateCatalogPhase::Publish);
    }

    fn retire_intermediate_one(self: &Arc<Self>) {
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            #[cfg(test)]
            self.terminal_job_retirements.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if self.pending_owned.load(std::sync::atomic::Ordering::Acquire) {
            let token = {
                let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                DatabaseCreateCatalogToken { slot: self.slot, generation: self.generation, revision: cursor.base_revision }
            };
            let mut catalog = match self.catalog.try_lock() {
                Ok(catalog) => catalog,
                Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
                Err(std::sync::TryLockError::WouldBlock) => {
                    self.defer_catalog_contention();
                    return;
                }
            };
            if catalog.pending == Some(token) {
                catalog.pending = None;
            }
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(owner) = work.as_mut() {
            let step = owner.close_one();
            let empty = owner.terminal_is_empty();
            if empty {
                work.take();
            }
            drop(work);
            match step {
                Ok(_) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
                Err(error) => {
                    *self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(error));
                    self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                }
            }
            return;
        }
        drop(work);
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(pages) = cursor.pages.as_mut() {
            match pages.close_step() {
                Ok(Some(_)) => {}
                Ok(None) if pages.terminal_is_empty() => {
                    cursor.pages.take();
                }
                Ok(None) => {
                    drop(cursor);
                    self.stage_error(DbError::LimitExceeded("database create-catalog page retirement"), DatabaseCreateCatalogProgress::Fault);
                    return;
                }
                Err(error) => {
                    drop(cursor);
                    self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(writer) = cursor.writer.as_mut() {
            match writer.close_step() {
                Ok(Some(_)) => {}
                Ok(None) if writer.terminal_is_empty() => {
                    cursor.writer.take();
                }
                Ok(None) => {
                    drop(cursor);
                    self.stage_error(DbError::LimitExceeded("database create-catalog writer retirement"), DatabaseCreateCatalogProgress::Fault);
                    return;
                }
                Err(error) => {
                    drop(cursor);
                    self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(snapshot) = cursor.snapshot.take() {
            match Arc::try_unwrap(snapshot) {
                Ok(candidate) => cursor.candidate = Some(candidate),
                Err(snapshot) => {
                    cursor.snapshot = Some(snapshot);
                    drop(cursor);
                    self.stage_error(DbError::LimitExceeded("database create-catalog snapshot retirement"), DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(candidate) = cursor.candidate.as_mut() {
            if candidate.pop().is_some() {
                self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                return;
            }
            cursor.candidate.take();
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.clone_text.take().is_some() {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.base.take().is_some() {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        drop(cursor);
        let unpublished = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        if self.closing.load(std::sync::atomic::Ordering::Acquire) && !unpublished {
            self.set_phase(DatabaseCreateCatalogPhase::Terminal);
            return;
        }
        self.set_phase(DatabaseCreateCatalogPhase::Publish);
    }

    pub fn publish_one(self: &Arc<Self>) {
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let document = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let actual = self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let expected = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).base_epoch;
        match (storage, document, actual) {
            (Some(storage), Some(document), Some(actual)) => {
                let result = Ok(DatabaseCreateCatalogResult { state: Some(self.clone()), storage: Some(storage), document: Some(document), expected, actual: Some(actual) });
                if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
                    *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                } else {
                    *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                }
                self.set_phase(DatabaseCreateCatalogPhase::Terminal);
                if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
            }
            (storage, document, actual) => {
                *self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = storage;
                *self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = document;
                *self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = actual;
                self.stage_error(DbError::LimitExceeded("database create-catalog result publication"), DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    pub fn roots_are_empty(&self) -> bool {
        let storage_empty = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
        let document_empty = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
        let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        storage_empty
            && document_empty
            && cursor.base.is_none()
            && cursor.candidate.is_none()
            && cursor.clone_text.is_none()
            && cursor.snapshot.is_none()
            && cursor.writer.is_none()
            && cursor.pages.is_none()
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.pending_owned.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn retained_owner_count(&self) -> usize {
        let storage = usize::from(self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        let document = usize::from(self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        storage
            + document
            + usize::from(cursor.base.is_some())
            + cursor.candidate.as_ref().map_or(0, |entries| entries.len() + 1)
            + usize::from(cursor.clone_text.is_some())
            + cursor.snapshot.as_ref().map_or(0, |entries| entries.len() + 1)
            + usize::from(cursor.writer.is_some())
            + usize::from(cursor.pages.is_some())
            + usize::from(self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map_or(0, DatabaseCreateCatalogResult::retained_owner_count)
            + self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map_or(0, DatabaseCreateCatalogResult::retained_owner_count)
            + usize::from(self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.pending_owned.load(std::sync::atomic::Ordering::Acquire))
    }

    fn retire_terminal_one(self: &Arc<Self>) {
        if !self.roots_are_empty() {
            self.set_phase(DatabaseCreateCatalogPhase::Retire);
            self.retire_intermediate_one();
            return;
        }
        let mut result = self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(Ok(owner)) = result.as_mut() {
            if owner.close_one() {
                if owner.terminal_is_empty() {
                    result.take();
                }
                drop(result);
                self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                return;
            }
        }
        if result.take().is_some() {
            drop(result);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        drop(result);
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
    }

    pub fn release_success(self: &Arc<Self>) {
        if self.roots_are_empty() && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.retire_terminal_one();
            return;
        }
        self.closing.store(true, std::sync::atomic::Ordering::Release);
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.begin_callback_close();
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire)
            && self.roots_are_empty()
            && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.retry_closing.load(std::sync::atomic::Ordering::Acquire)
            && !self.callback_close_armed.load(std::sync::atomic::Ordering::Acquire)
            && self.driver_authority.load(std::sync::atomic::Ordering::Acquire) == DatabaseCreateCatalogDriverAuthority::Idle as u8
    }
}

pub struct DatabaseCreateCatalogFuture {
    pub state: Arc<DatabaseCreateCatalogState>,
    pub resolved: bool,
}

impl DatabaseCreateCatalogFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, catalog: Arc<Mutex<CatalogState>>, storage: Arc<db_storage::DbBackend>, document: protocol::ArtifactId) -> Result<Self, DatabaseCreateCatalogRejected> {
        Self::try_prepare(pool, catalog, storage, document, true)
    }

    pub fn try_prepare(pool: Arc<WorkerPool>, catalog: Arc<Mutex<CatalogState>>, storage: Arc<db_storage::DbBackend>, document: protocol::ArtifactId, schedule: bool) -> Result<Self, DatabaseCreateCatalogRejected> {
        let pool_use = match pool.acquire_use() {
            Ok(pool_use) => pool_use,
            Err(error) => return Err(DatabaseCreateCatalogRejected::new(pool, DbError::Unavailable(format!("database create-catalog WorkerPool use rejected: {error:?}")), storage, document)),
        };
        Self::try_prepare_with_use(pool, pool_use, catalog, storage, document, schedule)
    }

    pub fn try_prepare_with_use(pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>, catalog: Arc<Mutex<CatalogState>>, storage: Arc<db_storage::DbBackend>, document: protocol::ArtifactId, schedule: bool) -> Result<Self, DatabaseCreateCatalogRejected> {
        let admission = match DatabaseCreateCatalogAdmission::try_claim(&document) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCreateCatalogRejected::new(pool, error, storage, document)),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        let (base, base_epoch, base_revision, backing) = {
            let catalog = catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if catalog.entries.len() >= DATABASE_CREATE_CATALOG_MAX_ENTRIES {
                drop(catalog);
                drop(admission);
                return Err(DatabaseCreateCatalogRejected::new(pool, DbError::LimitExceeded("database create-catalog entry capacity"), storage, document));
            }
            let backing = match DatabaseCreateCatalogBackingLedger::new(document.0.capacity(), catalog.entries.capacity()) {
                Ok(backing) => backing,
                Err(error) => {
                    drop(catalog);
                    drop(admission);
                    return Err(DatabaseCreateCatalogRejected::new(pool, error, storage, document));
                }
            };
            (Arc::clone(&catalog.entries), catalog.epoch, catalog.revision, backing)
        };
        if database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot].is_some() {
            drop(admission);
            return Err(DatabaseCreateCatalogRejected::new(pool, DbError::LimitExceeded("database create-catalog terminal slot"), storage, document));
        }
        let base_identity = Arc::as_ptr(&base) as usize;
        let created_at_ms = pool.now_ms();
        let deadline_ms = created_at_ms.checked_add(DATABASE_CREATE_CATALOG_DEADLINE_MS).unwrap_or(u64::MAX);
        DATABASE_CREATE_CATALOG_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let state = Arc::new(DatabaseCreateCatalogState {
            pool,
            pool_use: Mutex::new(Some(pool_use)),
            catalog,
            slot,
            generation,
            admission: Mutex::new(Some(admission)),
            storage: Mutex::new(Some(storage)),
            document: Mutex::new(Some(document)),
            cursor: Mutex::new(DatabaseCreateCatalogCursor {
                base: Some(base),
                backing,
                base_epoch,
                base_revision,
                base_identity,
                created_at_ms,
                encoded_len: 2,
                scan_entry: 0,
                scan_byte: 0,
                scan_started: false,
                candidate: None,
                clone_entry: 0,
                clone_byte: 0,
                clone_text: None,
                snapshot: None,
                encode: DatabaseCreateCatalogEncodeCursor::new(),
                writer: None,
                pages: None,
            }),
            work: Mutex::new(None),
            poll_work: Mutex::new(None),
            terminal_work: Mutex::new(None),
            outcome: Mutex::new(None),
            completion: Mutex::new(None),
            terminal_completion: Mutex::new(None),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            driver_authority: std::sync::atomic::AtomicU8::new(DatabaseCreateCatalogDriverAuthority::Idle as u8),
            polling: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            closing: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            pending_owned: std::sync::atomic::AtomicBool::new(false),
            terminal_checked_out: std::sync::atomic::AtomicBool::new(false),
            retry_closing: std::sync::atomic::AtomicBool::new(false),
            callback_close_armed: std::sync::atomic::AtomicBool::new(false),
            catalog_contention_armed: std::sync::atomic::AtomicBool::new(false),
            phase: std::sync::atomic::AtomicU8::new(DatabaseCreateCatalogPhase::Scan as u8),
            progress: std::sync::atomic::AtomicU8::new(DatabaseCreateCatalogProgress::Admitted as u8),
            deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            opportunities: std::sync::atomic::AtomicU64::new(0),
            #[cfg(any(test, feature = "testkit"))]
            active_drivers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(any(test, feature = "testkit"))]
            max_active_drivers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(any(test, feature = "testkit"))]
            poll_worker_thread: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            spent_submission_attempts: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            backend_polls: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            callback_worker_thread: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            terminal_job_retirements: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            controlled_capacity_overage: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            controlled_driver_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_publication_before_waker_hook: Mutex::new(None),
        });
        database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        if schedule {
            state.schedule();
        }
        Ok(Self { state, resolved: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> DatabaseCreateCatalogProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => DatabaseCreateCatalogProgress::Admitted,
            1 => DatabaseCreateCatalogProgress::Scanning,
            2 => DatabaseCreateCatalogProgress::Copying,
            3 => DatabaseCreateCatalogProgress::Encoding,
            4 => DatabaseCreateCatalogProgress::Polling,
            5 => DatabaseCreateCatalogProgress::Revalidating,
            6 => DatabaseCreateCatalogProgress::Completed,
            7 => DatabaseCreateCatalogProgress::Cancelled,
            _ => DatabaseCreateCatalogProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if self.state.phase() == DatabaseCreateCatalogPhase::Terminal {
            self.state.begin_callback_close();
        } else {
            self.state.schedule();
        }
    }
}

impl Future for DatabaseCreateCatalogFuture {
    type Output = Result<DatabaseCreateCatalogResult, DbError>;

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

impl Drop for DatabaseCreateCatalogFuture {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }
}

pub struct DatabaseCreateCatalogTerminalHandle {
    pub state: Arc<DatabaseCreateCatalogState>,
}

impl DatabaseCreateCatalogTerminalHandle {
    pub fn witness(&self) -> DatabaseCreateCatalogTerminalWitness {
        DatabaseCreateCatalogTerminalWitness { generation: self.state.generation, retained_owners: self.state.retained_owner_count(), terminal_empty: self.state.terminal_is_empty() }
    }

    pub fn close_step(&self) -> DatabaseCreateCatalogCloseStep {
        if self.state.terminal_is_empty() {
            return DatabaseCreateCatalogCloseStep::Complete;
        }
        self.state.closing.store(true, std::sync::atomic::Ordering::Release);
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        let authority = self.state.driver_authority.load(std::sync::atomic::Ordering::Acquire);
        if authority == DatabaseCreateCatalogDriverAuthority::Driving as u8 || authority == DatabaseCreateCatalogDriverAuthority::Retry as u8 {
            DatabaseCreateCatalogCloseStep::Blocked
        } else if self.state.drive_explicit_close_one() {
            if self.state.terminal_is_empty() {
                DatabaseCreateCatalogCloseStep::Complete
            } else {
                DatabaseCreateCatalogCloseStep::Progress
            }
        } else {
            self.state.begin_callback_close();
            DatabaseCreateCatalogCloseStep::Blocked
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty()
    }
}

impl Drop for DatabaseCreateCatalogTerminalHandle {
    fn drop(&mut self) {
        self.state.terminal_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub fn take_database_create_catalog_terminal(generation: u64) -> Option<DatabaseCreateCatalogTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).find(|state| state.generation == generation && state.abandoned.load(Ordering::Acquire))?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCreateCatalogTerminalHandle { state })
}
//#endregion 🔖️CreateDocumentCatalogCas

pub static DATABASE_CREATE_CATALOG_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Drop for DatabaseCreateCatalogState {
    fn drop(&mut self) {
        DATABASE_CREATE_CATALOG_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
