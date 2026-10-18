//! 🔑️ Catalog bootstrap results and rejections: the bootstrap key, the progress and close steps, the terminal witness, the result, the rejected close owner and the rejected bootstrap with its close. See [`super`].

use super::*;

//#region 🔖️CatalogBootstrapCas
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
//#endregion 🔖️CatalogBootstrapCas
