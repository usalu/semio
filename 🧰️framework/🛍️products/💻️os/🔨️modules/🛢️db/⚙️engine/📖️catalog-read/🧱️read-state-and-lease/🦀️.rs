//! 🧱️ Catalog read state and lease: the backend future, the work, the phase, the shared read state, its lease and the wake. See [`super`].

use super::*;

//#region 🔖️CatalogRootRead
type DatabaseCatalogReadBackendFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCatalogReadResult> + Send + 'static>>;

pub(super) struct DatabaseCatalogReadWork {
    pub future: Option<DatabaseCatalogReadBackendFuture>,
    pub(super) pollable: bool,
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

    pub(super) fn can_poll(&self) -> bool {
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
pub(super) enum DatabaseCatalogReadPhase {
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

pub(super) struct DatabaseCatalogReadState {
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
    pub(super) controlled_retry_callback_hook: Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
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

pub(super) struct DatabaseCatalogReadLease {
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

pub(super) struct DatabaseCatalogReadWake {
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
//#endregion 🔖️CatalogRootRead
