//! 📋️ Progress, result and rejection: the progress and close steps, the result, the rejected owner and close and the rejection. See [`super`].

use super::*;

//#region 🔖️CreateDocumentCatalogCas
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
    pub(super) spent_submission_attempts: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) callback_worker_thread: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    pub(super) terminal_job_retirements: std::sync::atomic::AtomicUsize,
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

    pub(super) fn arm_callback_close(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.callback_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.callback_close_one());
    }

    pub(super) fn callback_close_one(self: Arc<Self>) {
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

pub(super) type DatabaseCreateCatalogBackendFuture = std::pin::Pin<Box<dyn Future<Output = Result<EpochFence, DbError>> + Send + 'static>>;
//#endregion 🔖️CreateDocumentCatalogCas
