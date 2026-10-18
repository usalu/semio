//! 🧱️ Catalog bootstrap state and wake: the backend future, the work, the phase, the driver authority, the shared bootstrap state and the wake. See [`super`].

use super::*;

//#region 🔖️CatalogBootstrapCas
type DatabaseCatalogBootstrapBackendFuture = std::pin::Pin<Box<dyn Future<Output = Result<EpochFence, DbError>> + Send + 'static>>;

pub(super) struct DatabaseCatalogBootstrapWork {
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
pub(super) enum DatabaseCatalogBootstrapPhase {
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
pub(super) enum DatabaseCatalogBootstrapDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

pub(super) struct DatabaseCatalogBootstrapState {
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
    pub(super) staged_actual: Mutex<Option<Result<EpochFence, DbError>>>,
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
    pub(super) retry_pressure: std::sync::atomic::AtomicBool,
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
    pub(super) controlled_driver_claim_hook: Mutex<Option<Arc<dyn Fn(DatabaseCatalogBootstrapPhase) + Send + Sync>>>,
    #[cfg(test)]
    pub(super) controlled_driver_release_hook: Mutex<Option<Arc<dyn Fn(DatabaseCatalogBootstrapPhase) + Send + Sync>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(any(test, feature = "testkit"))]
    pub max_active_drivers: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) driver_grants: std::sync::atomic::AtomicU64,
    #[cfg(any(test, feature = "testkit"))]
    pub poll_worker_thread: std::sync::atomic::AtomicBool,
}

pub(super) struct DatabaseCatalogBootstrapWake {
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
//#endregion 🔖️CatalogBootstrapCas
