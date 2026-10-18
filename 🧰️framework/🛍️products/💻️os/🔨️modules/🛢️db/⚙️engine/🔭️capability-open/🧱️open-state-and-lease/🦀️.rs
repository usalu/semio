//! 🧱️ Capability open state and lease: the backend future, the work, the phase, the shared open state, its lease and the wake. See [`super`].

use super::*;

//#region 🔖️CapabilityOpen
type DatabaseCapabilityOpenBackendFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCapabilityOpenResult> + Send + 'static>>;

pub(super) struct DatabaseCapabilityOpenWork {
    pub future: Option<DatabaseCapabilityOpenBackendFuture>,
    #[cfg(any(test, feature = "testkit"))]
    pub storage_identity: usize,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DatabaseCapabilityOpenPhase {
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

pub(super) struct DatabaseCapabilityOpenState {
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
    pub(super) poll_publication_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    #[cfg(any(test, feature = "testkit"))]
    pub controlled_submit_hook: Mutex<Option<Arc<dyn Fn(semio_framework_async::Job) -> Result<(), semio_framework_async::Job> + Send + Sync>>>,
}

pub(super) struct DatabaseCapabilityOpenLease {
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

pub(super) struct DatabaseCapabilityOpenWake {
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
//#endregion 🔖️CapabilityOpen
