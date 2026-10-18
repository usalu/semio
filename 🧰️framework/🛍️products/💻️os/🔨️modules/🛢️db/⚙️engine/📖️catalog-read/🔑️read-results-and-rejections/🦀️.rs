//! 🔑️ Catalog read results and rejections: the root key, the progress and close steps, the read result and the rejected read with its close. See [`super`].

use super::*;

//#region 🔖️CatalogRootRead
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
//#endregion 🔖️CatalogRootRead
