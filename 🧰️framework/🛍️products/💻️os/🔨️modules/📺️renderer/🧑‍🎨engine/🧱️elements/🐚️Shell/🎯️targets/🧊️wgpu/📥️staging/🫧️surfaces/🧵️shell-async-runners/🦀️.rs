//! 🫧️ `shell-surfaces` staging of `shell-async-runners`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🧵️ShellDetached
/// 🧵️ One request the shell hands off and never waits on: the future owns everything it touches and
/// runs on the shared pool (native) or the page's executor (browser), and the frame pump only asks,
/// once per frame, whether it answered — so the guest turn behind it never holds the shell, and with it
/// the frame build, checked out.
pub(crate) struct ShellDetached<T> {
    pub(crate) answer: std::sync::Arc<std::sync::Mutex<Option<T>>>,
}

impl<T: 'static> ShellDetached<T> {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn spawn(request: impl std::future::Future<Output = T> + Send + 'static) -> Self
    where
        T: Send,
    {
        let answer = std::sync::Arc::new(std::sync::Mutex::new(None));
        let slot = answer.clone();
        crate::spawn_app_task(async move {
            let value = request.await;
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(value);
        });
        Self { answer }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn spawn(request: impl std::future::Future<Output = T> + 'static) -> Self {
        let answer = std::sync::Arc::new(std::sync::Mutex::new(None));
        let slot = answer.clone();
        crate::spawn_app_task(async move {
            let value = request.await;
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(value);
        });
        Self { answer }
    }

    /// 📬️ The answer, once, if it has arrived.
    pub(crate) fn take(&self) -> Option<T> {
        self.answer.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }
}

/// 🖼️ What one owed refresh read off the shell for one app instance: the guest bodies it names, keyed by
/// surface id, each with the host effects its render returned, and the reserved sections it read. A read
/// the refresh no longer wants is handed back by its document's own `Drop`.
pub(crate) struct ShellRenderedRefresh {
    pub(crate) instance_id: u32,
    pub(crate) surfaces: HashMap<String, (Result<UiDocumentLease, crate::program_bridge::ProgramFault>, Vec<semio_framework::kernel::Effect>)>,
    pub(crate) catalogue: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
    pub(crate) engagements: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
    pub(crate) measures: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
    pub(crate) tools: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
}

impl ShellRenderedRefresh {
    /// 🫙️ Nothing read ahead: every wanted body and section is read by the refresh itself.
    pub(crate) fn empty(instance_id: u32) -> Self {
        Self { instance_id, surfaces: HashMap::new(), catalogue: None, engagements: None, measures: None, tools: None }
    }
}

//#endregion 🧵️ShellDetached
#[cfg(not(target_arch = "wasm32"))]
/// 🔄️ A retained-waker future polled once per shared-pool turn; no worker waits for completion.
pub(crate) struct ShellPoolFuture {
    pub(crate) pool: WorkerPool,
    pub(crate) lane: Lane,
    pub(crate) future: std::sync::Mutex<Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>>>,
    pub(crate) scheduled: std::sync::atomic::AtomicBool,
    pub(crate) notified: std::sync::atomic::AtomicBool,
    pub(crate) cancelled: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellPoolFuture {
    pub(crate) fn spawn(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static) -> std::sync::Arc<Self> {
        let task = std::sync::Arc::new(Self {
            pool,
            lane,
            future: std::sync::Mutex::new(Some(Box::pin(future))),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            notified: std::sync::atomic::AtomicBool::new(true),
            cancelled: std::sync::atomic::AtomicBool::new(false),
        });
        task.schedule();
        task
    }

    pub(super) fn schedule(self: &std::sync::Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let task = self.clone();
        self.pool.submit(self.lane, Box::new(move || task.run_turn()));
    }

    pub(super) fn run_turn(self: std::sync::Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let future = self.future.lock().expect("ShellPoolFuture mutex poisoned").take();
        if let Some(mut future) = future {
            if !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                let waker = std::task::Waker::from(self.clone());
                let mut context = std::task::Context::from_waker(&waker);
                if std::future::Future::poll(future.as_mut(), &mut context).is_pending() && !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                    *self.future.lock().expect("ShellPoolFuture mutex poisoned") = Some(future);
                }
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) && self.future.lock().expect("ShellPoolFuture mutex poisoned").is_some() {
            self.schedule();
        }
    }

    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.future.lock().expect("ShellPoolFuture mutex poisoned").take();
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for ShellPoolFuture {
    fn wake(self: std::sync::Arc<Self>) {
        self.schedule();
    }

    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        self.schedule();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) enum ShellIoCompletion {
    Actions(Vec<ActionDescriptor>),
    Import(PickedImport),
    MediaFrames { controller_id: String, frames: Vec<ActionDescriptor> },
    Program(Result<(), crate::program_bridge::ProgramFault>),
    Finished,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct PendingShellIo {
    pub(crate) receiver: std::sync::mpsc::Receiver<ShellIoCompletion>,
    pub(crate) task: Option<std::sync::Arc<ShellPoolFuture>>,
}

//#endregion ShellTypes

