//! 🪪️ The transport authority: connection turns, retained job retry and generation custody.

use semio_framework_async::{Job, Lane, WorkerPool, WorkerSubmitErrorKind};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use super::http_surface::HttpTerminalReason;
use super::http_transport_state::{HTTP_READINESS_POLL_MS, HTTP_RETRY_MS, HttpConnectionPhase, HttpTransportState, HttpTurn};

//#region 🔖️HttpTransport
pub(super) enum ConnectionTurn {
    Keep(HttpConnectionPhase),
    Close,
    Terminal(HttpTerminalReason),
}

pub(super) struct HttpTransportAuthority {
    pub(super) pool: WorkerPool,
    pub(crate) state: Mutex<HttpTransportState>,
    pub(super) scheduled: AtomicBool,
    pub(super) wake_requested: AtomicBool,
    pub(super) retry_armed: AtomicBool,
    pub(super) readiness_armed: AtomicBool,
    pub(super) retry_generation: AtomicU64,
    pub(super) readiness_generation: AtomicU64,
    pub(super) retry_job: Mutex<Option<Job>>,
    pub(super) terminal_job: Mutex<Option<Job>>,
    pub completion: (Mutex<Option<Result<(), GatewayError>>>, Condvar),
}

impl HttpTransportAuthority {
    pub(super) fn request_schedule(self: &Arc<Self>) {
        self.wake_requested.store(true, Ordering::Release);
        if self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.wake_requested.store(false, Ordering::Release);
            let weak = Arc::downgrade(self);
            self.submit_exact(Box::new(move || {
                if let Some(authority) = weak.upgrade() {
                    authority.drive_pool_turn();
                }
            }));
        }
    }

    fn submit_exact(self: &Arc<Self>, job: Job) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => self.retain_rejected_job(error.kind(), error.into_job()),
        }
    }

    fn retain_rejected_job(self: &Arc<Self>, kind: WorkerSubmitErrorKind, job: Job) {
        match kind {
            WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => {
                let mut retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if retained.is_none() {
                    *retained = Some(job);
                } else {
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                    self.finish(Err(GatewayError::new(GatewayErrorCode::Internal, "http transport retry owner already occupied")));
                    return;
                }
                drop(retained);
                self.arm_retry();
            }
            WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned => {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                self.finish(Err(GatewayError::new(GatewayErrorCode::Internal, format!("http transport worker pool unavailable: {kind:?}"))));
            }
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let weak = Arc::downgrade(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(HTTP_RETRY_MS), move || retry_retained_job(weak, generation));
    }

    fn arm_readiness_poll(self: &Arc<Self>, run_generation: u64) {
        if self.readiness_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let readiness_generation = self.readiness_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let weak = Arc::downgrade(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(HTTP_READINESS_POLL_MS), move || {
            let Some(authority) = weak.upgrade() else { return };
            if !generation_is_current(authority.readiness_generation.load(Ordering::Acquire), readiness_generation) {
                return;
            }
            authority.readiness_armed.store(false, Ordering::Release);
            let current = authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).run_generation;
            if generation_is_current(current, run_generation) {
                authority.request_schedule();
            }
        });
    }

    pub(super) fn clear_readiness(&self) {
        self.readiness_generation.fetch_add(1, Ordering::AcqRel);
        self.readiness_armed.store(false, Ordering::Release);
    }

    fn drive_pool_turn(self: Arc<Self>) {
        let (turn, generation) = {
            let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let generation = state.run_generation;
            (state.drive_one(self.pool.now_ms()), generation)
        };
        self.scheduled.store(false, Ordering::Release);
        if self.wake_requested.swap(false, Ordering::AcqRel) {
            self.request_schedule();
            return;
        }
        match turn {
            HttpTurn::MoreWork => self.request_schedule(),
            HttpTurn::PollReadiness => self.arm_readiness_poll(generation),
            HttpTurn::Parked => {}
            HttpTurn::Complete(result) => self.finish(result),
        }
    }

    pub fn finish(&self, result: Result<(), GatewayError>) {
        let (lock, wake) = &self.completion;
        let mut completion = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if completion.is_none() {
            *completion = Some(result);
            wake.notify_all();
        }
    }
}

fn retry_retained_job(weak: Weak<HttpTransportAuthority>, generation: u64) {
    let Some(authority) = weak.upgrade() else { return };
    if !generation_is_current(authority.retry_generation.load(Ordering::Acquire), generation) {
        return;
    }
    authority.retry_armed.store(false, Ordering::Release);
    let job = authority.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    if let Some(job) = job {
        authority.submit_exact(job);
    }
}

pub(crate) fn generation_is_current(current: u64, retained: u64) -> bool {
    current == retained
}
//#endregion 🔖️HttpTransport
