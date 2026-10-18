//! 🏃️ Actor runner types and handles: the runner limits, the turn owner, the terminal reason, state and job, the runner handle, ticket and struct and the turn wake. See [`super`].

use super::*;

pub(super) const ACTOR_RUNNER_RETRY_MS: u64 = 1;

pub(super) const ACTOR_RUNNER_RETRY_LIMIT: u8 = 8;

pub(super) type ActorTurnFuture = std::pin::Pin<Box<dyn std::future::Future<Output = (Box<ArtifactActor>, ArtifactDrive)> + Send>>;

pub(super) enum ActorTurnOwner {
    Actor(Box<ArtifactActor>),
    Future(ActorTurnFuture),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactActorTerminalReason {
    Detached,
    Cancelled,
    TurnFault,
    Pool(semio_framework_async::WorkerSubmitErrorKind),
}

pub(super) struct ActorTerminalJob {
    pub reason: ArtifactActorTerminalReason,
    pub(super) job: semio_framework_async::Job,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactActorTerminalState {
    Live,
    Closing(ArtifactActorTerminalReason),
    Complete(ArtifactActorTerminalReason),
}

#[derive(Clone)]
pub struct ArtifactActorRunnerHandle {
    pub(super) generation: u64,
    pub(super) runner: Arc<ActorRunner>,
}

pub struct ArtifactActorRunnerTicket {
    pub generation: u64,
    pub runner: std::sync::Weak<ActorRunner>,
    pub host: Option<Arc<std::sync::Mutex<ArtifactHostState<ArtifactActorRunnerHandle>>>>,
    pub returned: bool,
}

pub struct ArtifactActorTerminalJob {
    pub(super) handle: ArtifactActorRunnerHandle,
    pub owner: Option<ActorTerminalJob>,
}

pub(super) struct ActorRunner {
    pub pool: Arc<semio_framework_async::WorkerPool>,
    pub(super) io_reactor: Option<tokio::runtime::Handle>,
    pub generation: u64,
    pub(super) turn: std::sync::Mutex<Option<ActorTurnOwner>>,
    pub(super) terminal_turn: std::sync::Mutex<Option<ActorTurnOwner>>,
    pub(super) mailbox: ArtifactMailboxClose,
    pub(super) scheduled: std::sync::atomic::AtomicBool,
    pub(super) wake_requested: std::sync::atomic::AtomicBool,
    pub(super) turn_generation: std::sync::atomic::AtomicU64,
    pub(super) deadline_armed: std::sync::atomic::AtomicBool,
    pub(super) deadline_generation: std::sync::atomic::AtomicU64,
    pub(super) deadline_at_ms: std::sync::atomic::AtomicU64,
    pub(super) retry_armed: std::sync::atomic::AtomicBool,
    pub(super) retry_generation: std::sync::atomic::AtomicU64,
    pub(super) retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub(super) terminal_job: std::sync::Mutex<Option<ActorTerminalJob>>,
    pub(super) terminal_reason: std::sync::Mutex<Option<ArtifactActorTerminalReason>>,
    pub(super) terminal_empty_callback: std::sync::Mutex<Option<Arc<dyn Fn(u64) + Send + Sync>>>,
    pub(super) self_retained: std::sync::Mutex<Option<Arc<ActorRunner>>>,
    pub(super) external_tickets: std::sync::atomic::AtomicUsize,
    pub(super) close_requested: std::sync::atomic::AtomicBool,
    pub terminal: std::sync::atomic::AtomicBool,
    pub complete: std::sync::atomic::AtomicBool,
}

impl ArtifactActorRunnerHandle {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn cancel(&self) {
        self.runner.cancel();
    }

    pub fn terminal_state(&self) -> ArtifactActorTerminalState {
        let reason = *self.runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match reason {
            None => ArtifactActorTerminalState::Live,
            Some(reason) if self.runner.complete.load(std::sync::atomic::Ordering::Acquire) => ArtifactActorTerminalState::Complete(reason),
            Some(reason) => ArtifactActorTerminalState::Closing(reason),
        }
    }

    pub fn close_step(&self) -> bool {
        if !self.runner.terminal.load(std::sync::atomic::Ordering::Acquire) {
            self.runner.request_close();
            return false;
        }
        let advanced = self.runner.close_one_terminal_owner();
        if !advanced && self.runner.terminal_is_empty() {
            self.runner.finish_terminal();
        }
        advanced
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.runner.terminal_is_empty()
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactActorTerminalJob> {
        self.runner.take_terminal_job().map(|owner| ArtifactActorTerminalJob { handle: self.clone(), owner: Some(owner) })
    }

    pub(in super::super) fn start(&self) {
        self.runner.schedule();
    }

    pub(in super::super) fn request_close(&self) {
        self.runner.request_close();
    }

    pub(in super::super) fn issue_ticket(&self, host: Arc<std::sync::Mutex<ArtifactHostState<ArtifactActorRunnerHandle>>>) -> ArtifactActorRunnerTicket {
        self.runner.external_tickets.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| count.checked_add(1)).expect("artifact actor ticket capacity exhausted");
        ArtifactActorRunnerTicket { generation: self.generation, runner: Arc::downgrade(&self.runner), host: Some(host), returned: false }
    }

    pub(in super::super) fn set_terminal_empty_callback(&self, callback: Arc<dyn Fn(u64) + Send + Sync>) {
        *self.runner.terminal_empty_callback.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(callback);
        if self.runner.terminal_is_empty() {
            self.runner.finish_terminal();
        }
    }
}

impl ArtifactActorRunnerTicket {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn terminal_state(&self) -> Option<ArtifactActorTerminalState> {
        self.runner.upgrade().map(|runner| ArtifactActorRunnerHandle { generation: self.generation, runner }.terminal_state())
    }

    pub fn return_to_host(mut self) {
        self.return_once();
    }

    fn return_once(&mut self) {
        if self.returned {
            return;
        }
        self.returned = true;
        if let Some(runner) = self.runner.upgrade() {
            if runner.generation == self.generation {
                runner.return_ticket();
            }
        }
        self.host.take();
    }
}

impl Drop for ArtifactActorRunnerTicket {
    fn drop(&mut self) {
        self.return_once();
    }
}

impl ArtifactActorTerminalJob {
    pub fn reason(&self) -> ArtifactActorTerminalReason {
        self.owner.as_ref().expect("terminal job owner already resolved").reason
    }

    pub fn resume(mut self) {
        let owner = self.owner.take().expect("terminal job owner already resolved");
        self.handle.runner.resume_terminal_job(owner);
    }

    pub fn close(mut self) {
        let owner = self.owner.take().expect("terminal job owner already resolved");
        self.handle.runner.close_terminal_job(owner);
    }
}

impl Drop for ArtifactActorTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            self.handle.runner.return_terminal_job(owner);
        }
    }
}

pub(super) struct ActorTurnWake {
    pub runner: std::sync::Weak<ActorRunner>,
    pub generation: u64,
}

impl std::task::Wake for ActorTurnWake {
    fn wake(self: Arc<Self>) {
        if let Some(runner) = self.runner.upgrade() {
            runner.request_wake(self.generation);
        }
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(runner) = self.runner.upgrade() {
            runner.request_wake(self.generation);
        }
    }
}
