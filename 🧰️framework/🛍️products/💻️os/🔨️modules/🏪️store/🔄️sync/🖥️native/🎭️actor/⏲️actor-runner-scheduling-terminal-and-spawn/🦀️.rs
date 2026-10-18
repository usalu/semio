//! ⏲️ Actor runner scheduling, terminal and spawn: the runner scheduling, wakes, retries and deadlines, the job run, the terminal close and the socket io reactor and actor spawn. See [`super`].

use super::*;

impl ActorRunner {
    /// 📬️ A mailbox, readiness or close wake targets whichever turn is current: it carries no
    /// generation, so a turn that completes between the sender's read and its request cannot
    /// turn it into a stale wake (the actor then idled with a queued command, ticket 26/09/23
    /// `📓️wp-h6.md`).
    pub(super) fn schedule(self: &Arc<Self>) {
        if self.complete.load(std::sync::atomic::Ordering::Acquire) || self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let _ = self.wake_requested.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
        self.enqueue(false);
    }

    /// ⏰️ A turn future's own waker, or its deadline: only the turn that armed it may wake it.
    pub(super) fn request_wake(self: &Arc<Self>, generation: u64) {
        if generation != self.turn_generation.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.schedule();
    }

    pub(super) fn enqueue(self: &Arc<Self>, terminal_close: bool) {
        if self.complete.load(std::sync::atomic::Ordering::Acquire) || (!terminal_close && self.terminal.load(std::sync::atomic::Ordering::Acquire)) {
            return;
        }
        if self.scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let runner = self.clone();
        self.submit_exact(Box::new(move || runner.run_job()), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(semio_framework_async::Lane::UserVisible, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated if attempt < ACTOR_RUNNER_RETRY_LIMIT => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt + 1));
                    self.arm_retry();
                }
                kind => {
                    let job = error.into_job();
                    self.begin_terminal(ArtifactActorTerminalReason::Pool(kind));
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason: ArtifactActorTerminalReason::Pool(kind), job });
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        if self.retry_armed.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel).wrapping_add(1);
        let runner = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(ACTOR_RUNNER_RETRY_MS), move || {
            if generation != runner.retry_generation.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            runner.retry_armed.store(false, std::sync::atomic::Ordering::Release);
            let retry = runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if runner.terminal.load(std::sync::atomic::Ordering::Acquire) {
                    let reason = runner.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner).unwrap_or(ArtifactActorTerminalReason::Cancelled);
                    *runner.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason, job });
                } else {
                    runner.submit_exact(job, attempt);
                }
            }
        });
    }

    /// 🏃️ TurnFault retains the future in `terminal_turn`. Do not `enqueue` a drain job:
    /// against a shut-down pool that would mint a second `Pool(Shutdown)` terminal_job
    /// grant and leak mailbox ownership (one-owner-per-grant). Host `close_one` drains.
    pub(super) fn run_job(self: Arc<Self>) {
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            self.close_one_terminal_owner();
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            if self.has_terminal_owner() {
                self.enqueue(true);
            } else {
                self.finish_terminal();
            }
            return;
        }
        self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
        let owner = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let Some(owner) = owner else {
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            return;
        };
        let mut future = match owner {
            ActorTurnOwner::Actor(mut actor) => {
                if self.close_requested.load(std::sync::atomic::Ordering::Acquire) {
                    actor.closing = true;
                    actor.cmd_rx.close();
                }
                Box::pin(async move {
                    let outcome = actor.drive_one().await;
                    (actor, outcome)
                }) as ActorTurnFuture
            }
            ActorTurnOwner::Future(future) => future,
        };
        let generation = self.turn_generation.load(std::sync::atomic::Ordering::Acquire);
        let waker = std::task::Waker::from(Arc::new(ActorTurnWake { runner: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        let _io_reactor = self.io_reactor.as_ref().map(tokio::runtime::Handle::enter);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                    *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    self.enqueue(true);
                    return;
                }
                *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                if self.release_scheduled_after_turn_with(|| {}) {
                    self.enqueue(false);
                }
            }
            Ok(std::task::Poll::Ready((mut actor, mut outcome))) => {
                self.turn_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
                self.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
                if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
                    *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    self.enqueue(true);
                    return;
                }
                if self.close_requested.load(std::sync::atomic::Ordering::Acquire) && !actor.closing && outcome != ArtifactDrive::Terminal {
                    actor.closing = true;
                    actor.cmd_rx.close();
                    outcome = ArtifactDrive::MoreWork;
                }
                match outcome {
                    ArtifactDrive::Terminal => {
                        *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                        self.begin_terminal(ArtifactActorTerminalReason::Detached);
                        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                        self.enqueue(true);
                    }
                    ArtifactDrive::MoreWork | ArtifactDrive::Idle { .. } => {
                        *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Actor(actor));
                        let woke = self.release_scheduled_after_turn_with(|| {});
                        if outcome == ArtifactDrive::MoreWork || woke {
                            self.schedule();
                        } else if let ArtifactDrive::Idle { deadline } = outcome {
                            self.arm_deadline(deadline);
                        }
                    }
                }
            }
            Err(_) => {
                *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTurnOwner::Future(future));
                self.begin_terminal(ArtifactActorTerminalReason::TurnFault);
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            }
        }
    }

    pub(super) fn release_scheduled_after_turn_with(&self, after_release: impl FnOnce()) -> bool {
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        after_release();
        self.wake_requested.swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    fn arm_deadline(self: &Arc<Self>, deadline: Option<Instant>) {
        let Some(deadline) = deadline else {
            self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
            self.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
            return;
        };
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let delay_ms = u64::try_from(deadline.saturating_duration_since(Instant::now()).as_millis()).unwrap_or(u64::MAX).max(1);
        let at_ms = self.pool.now_ms().saturating_add(delay_ms);
        if self.deadline_armed.load(std::sync::atomic::Ordering::Acquire) && self.deadline_at_ms.load(std::sync::atomic::Ordering::Acquire) <= at_ms {
            return;
        }
        self.deadline_at_ms.store(at_ms, std::sync::atomic::Ordering::Release);
        self.deadline_armed.store(true, std::sync::atomic::Ordering::Release);
        let generation = self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel).wrapping_add(1);
        let turn_generation = self.turn_generation.load(std::sync::atomic::Ordering::Acquire);
        let runner = self.clone();
        self.pool.callback_at(at_ms, move || {
            if generation != runner.deadline_generation.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            runner.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
            runner.deadline_at_ms.store(u64::MAX, std::sync::atomic::Ordering::Release);
            runner.request_wake(turn_generation);
        });
    }

    fn begin_terminal(&self, reason: ArtifactActorTerminalReason) {
        if self.terminal.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        self.mailbox.close();
        self.turn_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.deadline_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.deadline_armed.store(false, std::sync::atomic::Ordering::Release);
        self.retry_generation.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        *self.terminal_reason.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(reason);
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActorTerminalJob { reason, job });
        }
        if self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let owner = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = owner;
        }
    }

    pub fn cancel(self: &Arc<Self>) {
        self.begin_terminal(ArtifactActorTerminalReason::Cancelled);
        self.enqueue(true);
    }

    pub(super) fn request_close(self: &Arc<Self>) {
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.close_requested.store(true, std::sync::atomic::Ordering::Release);
        self.mailbox.close();
        self.schedule();
    }

    pub(super) fn close_one_terminal_owner(&self) -> bool {
        if let Some(owner) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            drop(owner);
            return true;
        }
        if self.mailbox.close_one() {
            return true;
        }
        let owner = self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            return false;
        }
        drop(owner);
        true
    }

    fn has_terminal_owner(&self) -> bool {
        self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.mailbox.has_pending()
            || self.terminal_turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal.load(std::sync::atomic::Ordering::Acquire)
            && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
            && self.external_tickets.load(std::sync::atomic::Ordering::Acquire) == 0
            && !self.has_terminal_owner()
            && self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    pub(super) fn finish_terminal(&self) {
        if !self.terminal_is_empty() {
            return;
        }
        if self.complete.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        if let Some(callback) = self.terminal_empty_callback.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            callback(self.generation);
        }
        self.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    }

    pub(super) fn return_ticket(&self) {
        let previous = self.external_tickets.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        debug_assert!(previous > 0, "artifact actor ticket returned twice");
        if previous == 1 && self.terminal_is_empty() {
            self.finish_terminal();
        }
    }

    pub(super) fn take_terminal_job(&self) -> Option<ActorTerminalJob> {
        self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }

    pub(super) fn resume_terminal_job(self: &Arc<Self>, terminal: ActorTerminalJob) {
        self.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.submit_exact(terminal.job, 0);
    }

    pub(super) fn return_terminal_job(&self, terminal: ActorTerminalJob) {
        let mut slot = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        debug_assert!(slot.is_none(), "terminal job handback slot already occupied");
        if slot.is_none() {
            *slot = Some(terminal);
        }
    }

    pub(super) fn close_terminal_job(&self, terminal: ActorTerminalJob) {
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        drop(terminal);
        if self.terminal_is_empty() {
            self.finish_terminal();
        }
    }
}

/// 🔌️ The one platform I/O reactor every native document actor polls its hub socket against: a
/// current-thread Tokio runtime whose only thread (`semio-document-io`) parks on the I/O and time
/// drivers, created on first use and never shut down, like `db_storage_driver_runtime`. Actors keep
/// running on the `WorkerPool` and only enter this handle while polled. It used to be whatever
/// runtime the spawning thread happened to be inside; a native wgpu shell spawns from a plain
/// thread, so the first hub dial panicked with `there is no reactor running` and the document
/// socket never left `Connecting` (ticket 26/09/23 slice WG8, `hub-live-collaboration-check` run 1).
/// `None` only when the operating system refuses the reactor, which the dial then reports as a fault.
pub(super) fn document_socket_io_reactor() -> Option<tokio::runtime::Handle> {
    static REACTOR: std::sync::OnceLock<Option<tokio::runtime::Handle>> = std::sync::OnceLock::new();
    REACTOR
        .get_or_init(|| {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_io().enable_time().build().ok()?;
            let handle = runtime.handle().clone();
            std::thread::Builder::new().name("semio-document-io".to_string()).stack_size(1 << 20).spawn(move || runtime.block_on(std::future::pending::<()>())).ok()?;
            Some(handle)
        })
        .clone()
}

/// 🚀️ Creates one finite-turn actor on the process WorkerPool. Tokio remains only the
/// platform I/O reactor ([`document_socket_io_reactor`]); it never owns an actor or timer thread.
pub(in super::super) async fn spawn_actor(
    pool: Arc<semio_framework_async::WorkerPool>,
    generation: u64,
    config: ArtifactActorConfig,
    remote: ChannelBackboneRemote,
    cmd_rx: ArtifactMailboxReceiver,
    events: broadcast::Sender<ArtifactEvent>,
    credential: Arc<std::sync::RwLock<Option<Arc<semio_framework_os_directory_client::LocalHubCredential>>>>,
    socket_grant_source: Arc<std::sync::RwLock<Option<Arc<dyn semio_framework_os_directory_client::HubSocketGrantSource>>>>,
    document_execution_target_lease: Option<semio_framework_os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    seed: Option<semio_framework_os_directory::CanonicalCheckpointPairV1>,
    operation_cancel: semio_framework_async::CancelToken,
) -> ArtifactActorRunnerHandle {
    let io_reactor = document_socket_io_reactor();
    let mailbox = cmd_rx.close_handle();
    let mut actor = ArtifactActor::new(pool.clone(), config, remote, cmd_rx, events, credential, socket_grant_source, document_execution_target_lease, operation_cancel).await;
    if let Some(pair) = seed {
        actor.seed_hub_document(pair).await;
    }
    let runner = Arc::new(ActorRunner {
        pool,
        io_reactor,
        generation,
        turn: std::sync::Mutex::new(Some(ActorTurnOwner::Actor(Box::new(actor)))),
        terminal_turn: std::sync::Mutex::new(None),
        mailbox,
        scheduled: std::sync::atomic::AtomicBool::new(false),
        wake_requested: std::sync::atomic::AtomicBool::new(false),
        turn_generation: std::sync::atomic::AtomicU64::new(1),
        deadline_armed: std::sync::atomic::AtomicBool::new(false),
        deadline_generation: std::sync::atomic::AtomicU64::new(1),
        deadline_at_ms: std::sync::atomic::AtomicU64::new(u64::MAX),
        retry_armed: std::sync::atomic::AtomicBool::new(false),
        retry_generation: std::sync::atomic::AtomicU64::new(1),
        retry_job: std::sync::Mutex::new(None),
        terminal_job: std::sync::Mutex::new(None),
        terminal_reason: std::sync::Mutex::new(None),
        terminal_empty_callback: std::sync::Mutex::new(None),
        self_retained: std::sync::Mutex::new(None),
        external_tickets: std::sync::atomic::AtomicUsize::new(0),
        close_requested: std::sync::atomic::AtomicBool::new(false),
        terminal: std::sync::atomic::AtomicBool::new(false),
        complete: std::sync::atomic::AtomicBool::new(false),
    });
    *runner.self_retained.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(runner.clone());
    let weak = Arc::downgrade(&runner);
    let schedule: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        if let Some(runner) = weak.upgrade() {
            runner.schedule();
        }
    });
    if let Some(ActorTurnOwner::Actor(actor)) = runner.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut() {
        let readiness_requested = actor.readiness_requested.clone();
        let readiness_schedule = schedule.clone();
        let readiness: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            readiness_requested.store(true, std::sync::atomic::Ordering::Release);
            readiness_schedule();
        });
        actor.remote.set_outbound_wake(schedule.clone());
        actor.cmd_rx.set_wake(schedule);
        actor.set_readiness(readiness);
    }
    ArtifactActorRunnerHandle { generation, runner }
}
