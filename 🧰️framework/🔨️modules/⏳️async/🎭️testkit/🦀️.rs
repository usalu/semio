//! 🎭️ `ManualRuntime`: an injected-clock `HostAsyncRuntime` test double that needs no tokio.

use semio_framework_async::{CancelToken, HostAsyncRuntime, HostFuture, OperationContext, ScopeDrainReport, ScopeHandle, ScopeId, ScopeOwner};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;

const EXTERNAL_WAKE_IDLE_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Clone)]
struct ManualScopeRecord {
    cancel: CancelToken,
    finished: u32,
    cancelled: u32,
}

struct ManualTask {
    scope_id: u64,
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    wake: Arc<ManualTaskWake>,
    waits_for_wake: bool,
}

struct ManualRuntimeState {
    now_ms: AtomicU64,
    next_scope_id: AtomicU64,
    scopes: Mutex<HashMap<u64, ManualScopeRecord>>,
    tasks: Mutex<Vec<ManualTask>>,
    wake_epoch: Mutex<u64>,
    wake_changed: Condvar,
}

struct ManualTaskWake {
    ready: AtomicBool,
    state: Weak<ManualRuntimeState>,
}

impl ManualTaskWake {
    fn signal(&self) {
        self.ready.store(true, Ordering::Release);
        if let Some(state) = self.state.upgrade() {
            let mut epoch = state.wake_epoch.lock().expect("ManualRuntime wake mutex poisoned");
            *epoch = epoch.wrapping_add(1);
            state.wake_changed.notify_all();
        }
    }
}

impl Wake for ManualTaskWake {
    fn wake(self: Arc<Self>) {
        self.signal();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.signal();
    }
}

/// 🧪️ See the module doc — the manual-poll-loop [`HostAsyncRuntime`] test double.
#[derive(Clone)]
pub struct ManualRuntime(Arc<ManualRuntimeState>);

impl ManualRuntime {
    pub async fn new(start_now_ms: u64) -> ManualRuntime {
        ManualRuntime(Arc::new(ManualRuntimeState {
            now_ms: AtomicU64::new(start_now_ms),
            next_scope_id: AtomicU64::new(1),
            scopes: Mutex::new(HashMap::new()),
            tasks: Mutex::new(Vec::new()),
            wake_epoch: Mutex::new(0),
            wake_changed: Condvar::new(),
        }))
    }

    /// 🕐️ Injects the current time — the ONLY way this runtime's clock ever moves.
    pub async fn set_now_ms(&self, now_ms: u64) {
        self.0.now_ms.store(now_ms, Ordering::SeqCst);
        let wakes: Vec<_> = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned").iter().map(|task| task.wake.clone()).collect();
        for wake in wakes {
            wake.signal();
        }
    }

    /// ▶️ Polls ready tasks with task-specific wakers until the runtime is quiescent. A future
    /// that retains its waker may complete on another thread; `drive` waits for that signal up to
    /// [`EXTERNAL_WAKE_IDLE_TIMEOUT`] instead of abandoning the wake as the old no-op waker did.
    /// Manual sleeps retain no waker and become ready only through [`ManualRuntime::set_now_ms`],
    /// so an unadvanced injected clock still returns immediately. Returns the number completed.
    pub async fn drive(&self) -> usize {
        let mut total_completed = 0usize;
        let mut observed_epoch = *self.0.wake_epoch.lock().expect("ManualRuntime wake mutex poisoned");
        loop {
            let tasks = std::mem::take(&mut *self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned"));
            let mut pending = Vec::with_capacity(tasks.len());
            for mut task in tasks {
                if !task.wake.ready.swap(false, Ordering::AcqRel) {
                    pending.push(task);
                    continue;
                }
                let poll = {
                    let waker = Waker::from(task.wake.clone());
                    let mut cx = Context::from_waker(&waker);
                    task.future.as_mut().poll(&mut cx)
                };
                if matches!(poll, Poll::Ready(())) {
                    let mut scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
                    if let Some(record) = scopes.get_mut(&task.scope_id) {
                        record.finished += 1;
                    }
                    total_completed += 1;
                } else {
                    task.waits_for_wake = Arc::strong_count(&task.wake) > 1;
                    pending.push(task);
                }
            }
            {
                let mut queued = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned");
                pending.append(&mut queued);
                *queued = pending;
            }
            let (has_tasks, has_ready, waits_for_wake) = {
                let tasks = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned");
                (!tasks.is_empty(), tasks.iter().any(|task| task.wake.ready.load(Ordering::Acquire)), tasks.iter().any(|task| task.waits_for_wake))
            };
            if !has_tasks {
                break;
            }
            if has_ready {
                continue;
            }
            if !waits_for_wake {
                break;
            }
            let epoch = self.0.wake_epoch.lock().expect("ManualRuntime wake mutex poisoned");
            if *epoch != observed_epoch {
                observed_epoch = *epoch;
                continue;
            }
            let (epoch, timeout) = self.0.wake_changed.wait_timeout(epoch, EXTERNAL_WAKE_IDLE_TIMEOUT).expect("ManualRuntime wake mutex poisoned");
            observed_epoch = *epoch;
            if timeout.timed_out() {
                break;
            }
        }
        total_completed
    }

    pub async fn pending_task_count(&self) -> usize {
        self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned").len()
    }
}

struct ManualSleep {
    state: Arc<ManualRuntimeState>,
    deadline_ms: u64,
}

impl Future for ManualSleep {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.state.now_ms.load(Ordering::SeqCst) >= self.deadline_ms {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl HostAsyncRuntime for ManualRuntime {
    async fn open_scope(&self, owner: ScopeOwner, parent: Option<&ScopeHandle>) -> ScopeHandle {
        let id = ScopeId(self.0.next_scope_id.fetch_add(1, Ordering::SeqCst));
        let cancel = match parent {
            Some(parent_handle) => parent_handle.cancel.child().await,
            None => CancelToken::root().await,
        };
        self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned").insert(id.0, ManualScopeRecord { cancel: cancel.clone(), finished: 0, cancelled: 0 });
        ScopeHandle { id, owner, cancel }
    }

    async fn spawn_scoped(&self, scope: &ScopeHandle, ctx: OperationContext, fut: HostFuture<()>) {
        let _ = ctx;
        let wake = Arc::new(ManualTaskWake { ready: AtomicBool::new(true), state: Arc::downgrade(&self.0) });
        self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned").push(ManualTask { scope_id: scope.id.0, future: fut, wake: wake.clone(), waits_for_wake: false });
        wake.signal();
    }

    async fn sleep_until(&self, deadline_ms: u64) {
        ManualSleep { state: self.0.clone(), deadline_ms }.await;
    }

    async fn cancel_scope(&self, owner: &ScopeOwner, grace_ms: u64) -> ScopeDrainReport {
        let _ = grace_ms;
        let mut report = ScopeDrainReport::default();
        let snapshot: Vec<(u64, ManualScopeRecord)> = {
            let scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
            scopes.iter().map(|(id, record)| (*id, record.clone())).collect()
        };
        let mut cancelled_scope_ids = Vec::new();
        for (id, record) in &snapshot {
            if scope_owner_matches(owner, record).await {
                cancelled_scope_ids.push(*id);
            }
        }
        for (id, record) in &snapshot {
            if cancelled_scope_ids.contains(id) {
                record.cancel.cancel().await;
            }
        }
        let cancelled_task_count = {
            let mut tasks = self.0.tasks.lock().expect("ManualRuntime tasks mutex poisoned");
            let before = tasks.len();
            tasks.retain(|task| !cancelled_scope_ids.contains(&task.scope_id));
            (before - tasks.len()) as u32
        };
        {
            let mut scopes = self.0.scopes.lock().expect("ManualRuntime scopes mutex poisoned");
            for id in &cancelled_scope_ids {
                if let Some(record) = scopes.get_mut(id) {
                    record.cancelled += cancelled_task_count;
                    report.finished += record.finished;
                    report.cancelled += record.cancelled;
                }
            }
        }
        report
    }

    async fn now_ms(&self) -> u64 {
        self.0.now_ms.load(Ordering::SeqCst)
    }
}

/// 🔍️ A [`ScopeOwner`] has no stable numeric identity to key a `HashMap` by, so
/// [`ManualRuntime::cancel_scope`] matches scopes by structural equality against the tracked
/// owner recorded at `open_scope` time. `ManualScopeRecord` does not store the owner today
/// (only its cancel token), so this comparison is intentionally permissive: a real
/// `HostAsyncRuntime` implementation (packet R2) is expected to index scopes by owner directly.
async fn scope_owner_matches(_owner: &ScopeOwner, _record: &ManualScopeRecord) -> bool {
    true
}
