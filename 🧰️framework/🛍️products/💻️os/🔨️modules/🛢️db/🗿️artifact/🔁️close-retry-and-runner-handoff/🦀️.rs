//! 🔁️ Close retry and runner handoff: the close retry state and the runner driver and handoff. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Actor
#[cfg(not(target_arch = "wasm32"))]
pub(super) const ARTIFACT_RUNNER_RETRY_MS: u64 = 1;

/// 🔁️ Automatic re-polls a faulted artifact-engine close receives before it waits for an explicit
/// re-admission (`ArtifactAuthority::readmit_close_retry`, `Database::shutdown`).
pub const ARTIFACT_CLOSE_RETRY_LIMIT: u8 = 8;

#[cfg(not(target_arch = "wasm32"))]
const ARTIFACT_CLOSE_RETRY_BASE_MS: u64 = 2;

#[cfg(not(target_arch = "wasm32"))]
pub(super) const ARTIFACT_CLOSE_RETRY_CAP_MS: u64 = 512;

/// 🧭️ Where the bounded retry owner of a faulted artifact-engine close stands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactCloseRetryState {
    Clear,
    Scheduled,
    Admitted,
    Exhausted,
    Cancelled,
}

/// 📈️ Progress of one faulted close: `attempts` counts automatic re-polls armed so far out of `limit`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactCloseRetryProgress {
    pub state: ArtifactCloseRetryState,
    pub attempts: u8,
    pub limit: u8,
}

impl ArtifactCloseRetryProgress {
    /// 🧱️ True once no automatic retry remains: only an explicit re-admission can re-poll the close.
    pub fn is_blocked(&self) -> bool {
        matches!(self.state, ArtifactCloseRetryState::Exhausted | ArtifactCloseRetryState::Cancelled)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactCloseRetry {
    pub state: ArtifactCloseRetryState,
    pub(super) attempts: u8,
    pub generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactCloseRetry {
    pub const fn clear() -> Self {
        Self { state: ArtifactCloseRetryState::Clear, attempts: 0, generation: 0 }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) const ARTIFACT_RUNNER_RETRY_LIMIT: u8 = 8;

#[cfg(not(target_arch = "wasm32"))]
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum ArtifactRunnerDriver {
    RunnableIdle,
    Queued,
    Polling,
    PollingWake,
    Parked,
    ClosingReady,
    ClosingPolling,
    ClosingPollingWake,
    ClosingParked,
    Terminal,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct ArtifactRunnerHandoff {
    pub(super) pool: Arc<semio_framework_async::WorkerPool>,
    pub(super) pool_use: std::sync::Mutex<Option<Arc<semio_framework_async::WorkerPoolUse>>>,
    pub(super) terminal_job: std::sync::Mutex<Option<(semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job)>>,
    pub(super) close_runner: std::sync::Mutex<Option<Arc<dyn Fn() -> bool + Send + Sync>>>,
    pub(super) retirement_maintenance: std::sync::Mutex<Option<(Arc<semio_framework_async::WorkerPool>, semio_framework_async::WorkerMaintenanceTicket)>>,
    pub close_error: std::sync::Mutex<Option<DbError>>,
    pub(super) close_retry: std::sync::Mutex<ArtifactCloseRetry>,
    #[cfg(test)]
    pub(super) close_faults: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) close_polls: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) retirement_turns: std::sync::atomic::AtomicUsize,
    pub(super) active_history: std::sync::atomic::AtomicBool,
    pub driver: std::sync::atomic::AtomicU8,
    pub terminal: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerHandoff {
    fn drop(&mut self) {
        if self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunnerHandoff {
    /// ⏲️ Arms the next bounded-backoff re-poll of a faulted close on the pool timer wheel. The
    /// timer admits exactly one re-poll through the retirement hook; stray wakes and coalesced
    /// maintenance requests never re-poll a faulted close.
    pub(super) fn arm_close_retry(self: &Arc<Self>) {
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !matches!(retry.state, ArtifactCloseRetryState::Clear | ArtifactCloseRetryState::Admitted) {
            return;
        }
        if retry.attempts >= ARTIFACT_CLOSE_RETRY_LIMIT {
            retry.state = ArtifactCloseRetryState::Exhausted;
            return;
        }
        let delay = ARTIFACT_CLOSE_RETRY_BASE_MS.checked_shl(u32::from(retry.attempts)).unwrap_or(ARTIFACT_CLOSE_RETRY_CAP_MS).min(ARTIFACT_CLOSE_RETRY_CAP_MS);
        let Some(deadline) = self.pool.now_ms().checked_add(delay) else {
            retry.state = ArtifactCloseRetryState::Exhausted;
            return;
        };
        retry.attempts += 1;
        retry.generation = retry.generation.wrapping_add(1);
        retry.state = ArtifactCloseRetryState::Scheduled;
        let generation = retry.generation;
        drop(retry);
        let handoff = Arc::downgrade(self);
        self.pool.callback_at(deadline, move || {
            if let Some(handoff) = handoff.upgrade() {
                handoff.admit_close_retry(generation);
            }
        });
    }

    fn admit_close_retry(&self, generation: u64) {
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if retry.state != ArtifactCloseRetryState::Scheduled || retry.generation != generation {
            return;
        }
        retry.state = ArtifactCloseRetryState::Admitted;
        drop(retry);
        self.request_retirement_maintenance();
    }

    pub(super) fn clear_close_retry(&self) {
        *self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = ArtifactCloseRetry::clear();
    }

    /// 🚦️ A healthy close always polls; a faulted one polls only under its admitted retry.
    pub(super) fn close_poll_admitted(&self) -> bool {
        let faulted = self.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        !faulted || self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner).state == ArtifactCloseRetryState::Admitted
    }

    pub(super) fn close_retry_progress(&self) -> Option<ArtifactCloseRetryProgress> {
        if self.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            return None;
        }
        let retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        Some(ArtifactCloseRetryProgress { state: retry.state, attempts: retry.attempts, limit: ARTIFACT_CLOSE_RETRY_LIMIT })
    }

    /// 🔂️ Grants an exhausted or cancelled faulted close a fresh retry budget and admits one re-poll now.
    pub(super) fn readmit_close_retry(&self) -> bool {
        if self.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            return false;
        }
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !matches!(retry.state, ArtifactCloseRetryState::Exhausted | ArtifactCloseRetryState::Cancelled) {
            return false;
        }
        retry.attempts = 0;
        retry.generation = retry.generation.wrapping_add(1);
        retry.state = ArtifactCloseRetryState::Admitted;
        drop(retry);
        self.request_retirement_maintenance();
        true
    }

    /// 🛑️ Stops automatic re-polls of a faulted close; the exact runner stays retained for re-admission.
    pub(super) fn cancel_close_retry(&self) -> bool {
        let mut retry = self.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !matches!(retry.state, ArtifactCloseRetryState::Scheduled | ArtifactCloseRetryState::Admitted) {
            return false;
        }
        retry.generation = retry.generation.wrapping_add(1);
        retry.state = ArtifactCloseRetryState::Cancelled;
        true
    }

    /// 🏁️ The runner's terminal transition: surrenders the handoff's pool use, publishes `Terminal`, then wakes a committed
    /// retirement cursor. Its last turn may have found the runner still closing and gone idle, and a terminal runner makes
    /// no further progress that would wake it — a stranded cursor keeps its `WorkerPoolUse` and a database shutdown waits in
    /// its `PoolUse` phase until the deadline.
    pub(super) fn enter_terminal(&self) {
        if self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
        self.driver.store(ArtifactRunnerDriver::Terminal as u8, std::sync::atomic::Ordering::Release);
        self.terminal.store(true, std::sync::atomic::Ordering::Release);
        self.request_retirement_maintenance();
    }

    pub(super) fn request_retirement_maintenance(&self) {
        let owner = self.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        if let Some((pool, ticket)) = owner {
            let _ = pool.request_maintenance(ticket);
        }
    }
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids
