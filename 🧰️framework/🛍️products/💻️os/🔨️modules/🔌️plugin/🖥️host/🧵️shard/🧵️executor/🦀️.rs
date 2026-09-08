//! 🏃️ `ShardExecutor` — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1c, one-pool-worker-runtime).
//! Owns exactly ONE [`super::ShardLoop`] and its logical actor-affinity state, scheduled as jobs onto
//! the shared, process-wide `semio_framework_async::WorkerPool` — `design-runtime.md` §2's "K shards
//! run in parallel" made real WITHOUT a dedicated OS thread per shard.
//!
//! terra-shard-grants/P1-process-shards landed this as one `ShardLoop` per DEDICATED OS thread,
//! parked on a genuinely blocking [`ThreadTransport::recv_deadline`] poll. That thread is gone: a
//! `ShardExecutor` now holds its `ShardLoop` behind a plain [`Mutex`] and is driven by
//! [`ShardExecutor::send_frame`] — every inbound frame (`ShardFrame::Grant`/`Unregister`) schedules a
//! retained single-flight handoff. Each admitted `WorkerPool` closure validates its epoch, polls one
//! nonblocking [`super::ShardLoop::drive_one`] opportunity, transfers at most one outcome, and yields
//! before resubmitting. Shard AFFINITY — an actor's
//! `wasmtime::Store` staying pinned to one shard so its guest instance state stays coherent — is now
//! a MUTUAL-EXCLUSION property (the `state` mutex, plus the single-flight scheduling protocol below)
//! rather than a thread-identity property: at most one job ever executes a given shard's turns at
//! once, but which physical `WorkerPool` worker thread that job lands on varies call to call.
//!
//! Native-only: [`ThreadTransport`] is `std::sync::mpsc`-backed (host-supplied, per the actor crate's
//! own purity rule — transports live outside that crate's pure core); this file owns BOTH ends of one
//! duplex pair internally now (see [`ShardExecutor::new`]'s doc) instead of splitting them across a
//! shard-owning thread and an external forwarder thread — the design that made a 250ms-polling
//! forwarder thread necessary in the first place (`💻️os/🖥️host/🎠️activation/🦀️.rs`'s deleted
//! `semio-os-host-kernel-shard-forward-*` threads) no longer exists.

use super::{AdmissionLimit, DeferredAuthority, FixedOwnerRing, ShardDrive, ShardLoop, ShardOutcome, ShardTransports, SHARD_DEFERRED_BYTES, SHARD_DEFERRED_ITEMS, SHARD_FRAME_MAX_BYTES};
use crate::{GuestInstance, GuestRuntime, GuestRuntimes};
use semio_framework_actor::{ActorId, Lane as ActorLane, ShardTransport, ThreadTransport};
use semio_framework_async::{Job as PoolJob, Lane as PoolLane, WorkerPool, WorkerSubmitErrorKind};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::{pin, Pin};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError, Weak};
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

/// 🔌️ Local newtype so [`ShardExecutor::new`] can hand [`ShardLoop::new`] an `Arc<ThreadTransport>`
/// as a [`super::ShardTransports::SharedThread`] variant while the executor ALSO keeps its own
/// [`ThreadTransport`] end (the mirror of the one wrapped here) to inject frames and drain outcomes —
/// `impl ShardTransport for Arc<ThreadTransport>` directly would hit `E0117` (neither type is local
/// to this crate; `🧵️shard/🦀️.rs`'s own `LoopbackProbe` doc comment already names this same
/// orphan-rule constraint for `Arc<LoopbackTransport>`).
pub struct SharedThreadTransport(Arc<ThreadTransport>);

impl ShardTransport for SharedThreadTransport {
    async fn send(&self, bytes: &[u8]) {
        self.0.send(bytes).await;
    }
    async fn recv(&self) -> Option<Vec<u8>> {
        self.0.try_recv_now()
    }
    async fn heartbeat(&self) -> u64 {
        self.0.heartbeat().await
    }
    async fn kill(&self) {
        self.0.kill().await;
    }
}

//#region ⚖️LanePriority
/// ⚖️ `ActorLane` (0=Interactive .. 3=Maintenance) collapsed to a `WorkerPool::submit` priority rank
/// — lower is more urgent, matching `ActorLane`'s own declaration order. Sentinel [`NO_LANE`] means
/// "nothing pending" (distinct from a real, always-`<4`, rank).
fn lane_rank(lane: ActorLane) -> u8 {
    match lane {
        ActorLane::Interactive => 0,
        ActorLane::UserVisible => 1,
        ActorLane::Background => 2,
        ActorLane::Maintenance => 3,
    }
}

const NO_LANE: u8 = 4;

/// ⚖️ Best-effort inverse of [`lane_rank`] for picking WHICH `WorkerPool::Lane` a shard's next pump
/// job submits under — best-effort, not correctness-critical: see [`ShardExecutor::schedule`]'s doc
/// for why a rare race here can only mis-prioritize a submission, never drop one.
fn pool_lane_for_rank(rank: u8) -> PoolLane {
    match rank {
        0 => PoolLane::Interactive,
        1 => PoolLane::UserVisible,
        2 => PoolLane::Background,
        _ => PoolLane::Maintenance,
    }
}
//#endregion ⚖️LanePriority

//#region 📬️OutcomeSink
/// 📬️ Thread-safe collector for [`ShardOutcome`]s, shared by every [`ShardExecutor`] one
/// `NativeKernelRuntime`/`ParallelRuntime`-equivalent caller spawns. Replaces the old per-runtime
/// `mpsc::Receiver<(ShardId, Vec<u8>)>` fed by a dedicated 250ms-polling forwarder thread — a
/// [`ShardExecutor`]'s own [`ShardExecutor::run`] pool job [`OutcomeSink::push`]es directly into this
/// from whichever `WorkerPool` worker thread happened to execute that shard's turn: "completion
/// notification through the pool," no separate thread ever reads a channel to relay it.
pub struct OutcomeSink {
    queue: Mutex<VecDeque<ShardOutcome>>,
    ready: Condvar,
}

impl OutcomeSink {
    pub fn new() -> Arc<OutcomeSink> {
        Arc::new(OutcomeSink { queue: Mutex::new(VecDeque::new()), ready: Condvar::new() })
    }

    fn push(&self, outcome: ShardOutcome) {
        let mut queue = self.queue.lock().unwrap_or_else(PoisonError::into_inner);
        queue.push_back(outcome);
        drop(queue);
        self.ready.notify_all();
    }

    /// 🌀️ Drains every outcome currently buffered across every shard sharing this sink — never
    /// blocks. Same contract as `NativeKernelRuntime::try_recv_outcomes`'s old channel-drain loop.
    pub fn try_recv_all(&self) -> Vec<ShardOutcome> {
        let mut queue = self.queue.lock().unwrap_or_else(PoisonError::into_inner);
        queue.drain(..).collect()
    }

    /// ⏳️ Blocks the CALLING thread until either `expected` outcomes have been collected or
    /// `timeout` elapses — same contract as the old `mpsc::Receiver::recv_timeout`-based
    /// `NativeKernelRuntime::wait_for_outcomes`/`ParallelRuntime::wait_for_outcomes`. Sound to block
    /// here: this is called from a CLI/host thread root waiting on `WorkerPool` jobs to complete, not
    /// from inside a pool job itself (a pool job blocking on its own pool would deadlock a
    /// single-worker pool — nothing in this crate does that).
    pub fn wait_for(&self, expected: usize, timeout: Duration) -> Vec<ShardOutcome> {
        let deadline = Instant::now() + timeout;
        let mut queue = self.queue.lock().unwrap_or_else(PoisonError::into_inner);
        while queue.len() < expected {
            let now = Instant::now();
            if now >= deadline {
                break;
            }
            let (guard, timeout_result) = self.ready.wait_timeout(queue, deadline - now).unwrap_or_else(PoisonError::into_inner);
            queue = guard;
            if timeout_result.timed_out() && queue.len() < expected {
                break;
            }
        }
        queue.drain(..).collect()
    }
}
//#endregion 📬️OutcomeSink

/// 🏃️ Owns one [`ShardLoop`], its actor-affinity `state` mutex, and BOTH ends of one
/// [`ThreadTransport::new_pair`] duplex link internally — `shard_side` is handed to the `ShardLoop`
/// itself (its own `ShardTransport`), `kernel_side` stays here for [`ShardExecutor::send_frame`] to
/// inject inbound frames and for [`ShardExecutor::run`] to drain the outbound [`ShardOutcome`]s the
/// SAME pump call just produced — no external caller ever sees either transport end.
pub struct ShardExecutor {
    state: Mutex<ShardExecutorState>,
    kernel_side: ThreadTransport,
    ingress: Mutex<FixedOwnerRing<PendingIngressFrame, SHARD_DEFERRED_ITEMS>>,
    outcomes: Arc<OutcomeSink>,
    pool: Arc<WorkerPool>,
    /// 🚦 Single-flight gate: `true` while a pump job for this shard is either queued on the
    /// `WorkerPool` or actively running — see [`ShardExecutor::schedule`]/[`ShardExecutor::run`] for
    /// the full protocol this and [`Self::epoch`] together implement.
    scheduled: AtomicBool,
    /// 🕰️ Bumped once per [`ShardExecutor::send_frame`] call — the CORRECTNESS-CRITICAL "is there
    /// unseen work" signal (monotonic, so a concurrent bump is never lost the way a plain flag could
    /// be); [`Self::pending_lane_rank`] below is a separate, best-effort PRIORITY hint layered on top,
    /// not a substitute for this counter.
    epoch: AtomicU64,
    consumed_epoch: AtomicU64,
    /// ⚖️ The lowest (most urgent) [`lane_rank`] seen since the last time a pump job actually started
    /// — [`NO_LANE`] if nothing has arrived. Purely a submission-priority hint: a rare race can leave
    /// this looking less urgent than it should (see [`Self::schedule`]'s doc), which only ever costs
    /// this shard's next job a slightly worse `WorkerPool` lane, never a lost frame — [`Self::epoch`]
    /// is what actually guarantees every frame gets pumped.
    pending_lane_rank: AtomicU8,
    handoff: Mutex<Option<(PoolLane, PoolJob)>>,
    handoff_retry_armed: AtomicBool,
    handoff_retry_attempt: AtomicU8,
    handoff_retry_generation: AtomicU64,
    terminal_handoff: Mutex<Option<(WorkerSubmitErrorKind, PoolLane, PoolJob)>>,
    drive_generation: AtomicU64,
    drive_waiting: AtomicBool,
    drive_wake_queued: AtomicBool,
    closed: AtomicBool,
    ingress_gate: Mutex<()>,
    ingress_state: AtomicU8,
    terminal_overflow_occupied: AtomicBool,
    failure: Mutex<FixedOwnerRing<crate::PluginHostError, SHARD_DEFERRED_ITEMS>>,
    terminal_failure: Mutex<Option<crate::PluginHostError>>,
}

struct PendingIngressFrame {
    lane: ActorLane,
    bytes: Vec<u8>,
}

type ShardDriveFuture = Pin<Box<dyn Future<Output = (ShardLoop, ShardDrive)> + Send>>;

struct ShardExecutorState {
    shard: Option<ShardLoop>,
    drive: Option<ShardDriveFuture>,
    polling: bool,
    registrations: FixedOwnerRing<(ActorId, GuestInstance, Option<tokio::sync::oneshot::Sender<RegistrationAdmission>>), SHARD_DEFERRED_ITEMS>,
}

#[must_use]
pub enum RegistrationAdmission {
    Admitted(super::ShardActorAllocation),
    Refused(super::ShardRegistrationRejected),
    Stopped,
    Rejected { actor: ActorId, instance: GuestInstance, limit: AdmissionLimit },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IngressCloseReason {
    Closing,
    Shutdown,
    Poisoned,
    OverCapacity,
    TerminalCapacity,
}

pub enum FrameIngress {
    Admitted,
    Rejected(TerminalFrameOwner),
}

pub struct TerminalFrameOwner {
    reason: IngressCloseReason,
    frame: Vec<u8>,
}

impl TerminalFrameOwner {
    pub fn reason(&self) -> IngressCloseReason {
        self.reason
    }

    pub fn into_frame(self) -> Vec<u8> {
        self.frame
    }

    pub fn close(self) {
        drop(self.frame);
    }
}

struct ShardDriveWake {
    executor: Weak<ShardExecutor>,
    generation: u64,
}

impl Wake for ShardDriveWake {
    fn wake(self: Arc<Self>) {
        if let Some(executor) = self.executor.upgrade() {
            executor.request_drive_wake(self.generation);
        }
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(executor) = self.executor.upgrade() {
            executor.request_drive_wake(self.generation);
        }
    }
}

struct ImmediateWake(AtomicBool);

impl Wake for ImmediateWake {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::Release);
    }
}

fn poll_drive_once<F: Future>(future: F) -> Option<F::Output> {
    let waker = Waker::from(Arc::new(ImmediateWake(AtomicBool::new(false))));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => Some(output),
        Poll::Pending => None,
    }
}

fn poll_retained_drive_once(future: &mut ShardDriveFuture, executor: &Arc<ShardExecutor>, generation: u64) -> Option<(ShardLoop, ShardDrive)> {
    let waker = Waker::from(Arc::new(ShardDriveWake { executor: Arc::downgrade(executor), generation }));
    let mut context = Context::from_waker(&waker);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => Some(output),
        Poll::Pending => None,
    }
}

fn claim_one_shot(flag: &AtomicBool) -> bool {
    flag.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok()
}

fn claim_drive_wake(active_generation: u64, generation: u64, queued: &AtomicBool) -> bool {
    generation == active_generation && claim_one_shot(queued)
}

impl ShardExecutor {
    /// ▶️ Builds the duplex transport pair, constructs the `ShardLoop` on `shard_side`, registers
    /// `initial` synchronously (no thread exists yet to race with — unlike `spawn`'s old
    /// `RegisterRequest`/ack rendezvous, there is no interleaving to close here at all), and returns
    /// an `Arc` (needed so [`ShardExecutor::schedule`] can hand `WorkerPool::submit` a strong
    /// self-reference for its job closure).
    pub async fn new(pool: Arc<WorkerPool>, runtime: Arc<GuestRuntimes>, initial: Vec<(ActorId, GuestInstance)>, outcomes: Arc<OutcomeSink>) -> Arc<ShardExecutor> {
        let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
        let mut shard = ShardLoop::new(runtime, ShardTransports::SharedThread(SharedThreadTransport(Arc::new(shard_side)))).await;
        for (actor, instance) in initial {
            if let Err(rejected) = shard.register(actor, instance) {
                shard.runtime.drop_instance(rejected.instance).await;
                let _ = shard.send_outcome(&ShardOutcome::Fault { actor: actor.0, message: format!("initial shard registration refused: {:?}", rejected.reason) }).await;
            }
        }
        Arc::new(ShardExecutor {
            state: Mutex::new(ShardExecutorState { shard: Some(shard), drive: None, polling: false, registrations: FixedOwnerRing::new(SHARD_DEFERRED_BYTES) }),
            kernel_side,
            ingress: Mutex::new(FixedOwnerRing::new(SHARD_DEFERRED_BYTES)),
            outcomes,
            pool,
            scheduled: AtomicBool::new(false),
            epoch: AtomicU64::new(0),
            consumed_epoch: AtomicU64::new(0),
            pending_lane_rank: AtomicU8::new(NO_LANE),
            handoff: Mutex::new(None),
            handoff_retry_armed: AtomicBool::new(false),
            handoff_retry_attempt: AtomicU8::new(0),
            handoff_retry_generation: AtomicU64::new(0),
            terminal_handoff: Mutex::new(None),
            drive_generation: AtomicU64::new(0),
            drive_waiting: AtomicBool::new(false),
            drive_wake_queued: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            ingress_gate: Mutex::new(()),
            ingress_state: AtomicU8::new(0),
            terminal_overflow_occupied: AtomicBool::new(false),
            failure: Mutex::new(FixedOwnerRing::new(SHARD_DEFERRED_BYTES)),
            terminal_failure: Mutex::new(None),
        })
    }

    /// 🆕️ Acknowledges physical admission; a running drive retains the incoming owner in its fixed ring.
    /// Refusal returns the exact guest instance, and a cancelled receiver leaves a discoverable close owner.
    pub async fn register(self: &Arc<Self>, actor: ActorId, instance: GuestInstance) -> RegistrationAdmission {
        let receive = {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            if self.closed.load(Ordering::Acquire) || self.ingress_state.load(Ordering::Acquire) != 0 || self.pool.is_shutdown() {
                return RegistrationAdmission::Refused(super::ShardRegistrationRejected { actor, instance, reason: super::ShardRegistrationReason::Stopped });
            }
            if let Some(shard) = state.shard.as_mut() {
                return match shard.register(actor, instance) {
                    Ok(allocation) => RegistrationAdmission::Admitted(allocation),
                    Err(rejected) => RegistrationAdmission::Refused(rejected),
                };
            }
            let (reply, receive) = tokio::sync::oneshot::channel();
            match state.registrations.try_push((actor, instance, Some(reply)), size_of::<(ActorId, GuestInstance)>()) {
                Ok(_) => receive,
                Err(rejected) => return RegistrationAdmission::Rejected { actor: rejected.owner.0, instance: rejected.owner.1, limit: rejected.limit },
            }
        };
        self.schedule();
        receive.await.unwrap_or(RegistrationAdmission::Stopped)
    }

    /// 🧯️ Transfers the exact last malformed-frame or shard-drive failure to the host.
    pub fn take_failure(&self) -> Option<crate::PluginHostError> {
        let failure = self.failure.lock().unwrap_or_else(PoisonError::into_inner).pop_front().map(|(_, failure)| failure);
        failure.or_else(|| self.terminal_failure.lock().unwrap_or_else(PoisonError::into_inner).take())
    }

    /// 🧯️ Transfers the exact successor closure terminally rejected by a stopped or poisoned
    /// pool, or by exhausting the finite quiet-ingress retry budget.
    pub fn take_terminal_handoff(&self) -> Option<(WorkerSubmitErrorKind, PoolLane, PoolJob)> {
        self.terminal_handoff.lock().unwrap_or_else(PoisonError::into_inner).take()
    }

    /// 🔁️ Re-arms the single exact terminal successor after its owning host has restored pool
    /// admission. Each call transfers at most one closure back to the finite handoff slot.
    pub fn resume_terminal_handoff(self: &Arc<Self>) -> bool {
        let Some((_, lane, job)) = self.terminal_handoff.lock().unwrap_or_else(PoisonError::into_inner).take() else {
            return false;
        };
        *self.handoff.lock().unwrap_or_else(PoisonError::into_inner) = Some((lane, job));
        self.handoff_retry_attempt.store(0, Ordering::Release);
        {
            let _ingress = self.ingress_gate.lock().unwrap_or_else(PoisonError::into_inner);
            self.ingress_state.store(0, Ordering::Release);
        }
        self.closed.store(false, Ordering::Release);
        self.schedule();
        true
    }

    pub fn take_terminal_frame(self: &Arc<Self>) -> Option<Vec<u8>> {
        let (frame, rearmed_epoch) = self.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_mut().map(ShardLoop::take_terminal_frame_and_rearm).unwrap_or((None, None));
        if let Some(epoch) = rearmed_epoch {
            self.terminal_overflow_occupied.store(false, Ordering::Release);
            self.acknowledge_consumed_epoch(epoch);
            if !self.closed.load(Ordering::Acquire) && self.consumed_epoch.load(Ordering::Acquire) < self.epoch.load(Ordering::Acquire) {
                self.schedule();
            }
        }
        frame
    }

    pub fn close_terminal_frame(self: &Arc<Self>) -> bool {
        self.take_terminal_frame().is_some()
    }

    pub fn take_terminal_completion(&self) -> Option<(u64, semio_framework::kernel::Event)> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_mut().and_then(ShardLoop::take_terminal_completion)
    }

    pub fn take_terminal_authority(&self) -> Option<DeferredAuthority> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).shard.as_mut().and_then(ShardLoop::take_terminal_authority)
    }

    /// ✉️ Retains one already-encoded [`super::ShardFrame`] in the finite ingress ring and
    /// schedules a pump job on `lane`. The next drive selects the most urgent retained lane while
    /// preserving arrival order within equal lanes.
    pub async fn send_frame(self: &Arc<Self>, bytes: Vec<u8>, lane: ActorLane) -> FrameIngress {
        let ingress = self.ingress_gate.lock().unwrap_or_else(PoisonError::into_inner);
        if bytes.len() > SHARD_FRAME_MAX_BYTES {
            return FrameIngress::Rejected(TerminalFrameOwner { reason: IngressCloseReason::OverCapacity, frame: bytes });
        }
        if self.terminal_overflow_occupied.load(Ordering::Acquire) {
            return FrameIngress::Rejected(TerminalFrameOwner { reason: IngressCloseReason::TerminalCapacity, frame: bytes });
        }
        if self.pool.is_shutdown() && self.ingress_state.load(Ordering::Acquire) == 0 {
            self.ingress_state.store(2, Ordering::Release);
        }
        if self.ingress_state.load(Ordering::Acquire) != 0 || self.closed.load(Ordering::Acquire) {
            return FrameIngress::Rejected(TerminalFrameOwner { reason: self.ingress_close_reason(), frame: bytes });
        }
        let byte_len = bytes.len();
        if let Err(rejected) = self.ingress.lock().unwrap_or_else(PoisonError::into_inner).try_push(PendingIngressFrame { lane, bytes }, byte_len) {
            return FrameIngress::Rejected(TerminalFrameOwner { reason: IngressCloseReason::OverCapacity, frame: rejected.owner.bytes });
        }
        self.epoch.fetch_add(1, Ordering::SeqCst);
        self.bump_lane_hint(lane);
        drop(ingress);
        self.schedule();
        FrameIngress::Admitted
    }

    fn take_next_ingress_frame(&self) -> Option<Vec<u8>> {
        let mut ingress = self.ingress.lock().unwrap_or_else(PoisonError::into_inner);
        let offset = (0..ingress.len).min_by_key(|offset| lane_rank(ingress.get(*offset).expect("ingress offset remains occupied").lane))?;
        ingress.pop_at(offset).map(|(_, frame)| frame.bytes)
    }

    fn ingress_close_reason(&self) -> IngressCloseReason {
        match self.ingress_state.load(Ordering::Acquire) {
            2 => IngressCloseReason::Shutdown,
            3 => IngressCloseReason::Poisoned,
            _ => IngressCloseReason::Closing,
        }
    }

    fn close_ingress(&self, reason: IngressCloseReason) {
        let _ingress = self.ingress_gate.lock().unwrap_or_else(PoisonError::into_inner);
        let state = match reason {
            IngressCloseReason::Closing => 1,
            IngressCloseReason::Shutdown => 2,
            IngressCloseReason::Poisoned => 3,
            IngressCloseReason::OverCapacity | IngressCloseReason::TerminalCapacity => 1,
        };
        if self.ingress_state.load(Ordering::Acquire) == 0 {
            self.ingress_state.store(state, Ordering::Release);
        }
    }

    fn bump_lane_hint(&self, lane: ActorLane) {
        let rank = lane_rank(lane);
        let mut observed = self.pending_lane_rank.load(Ordering::SeqCst);
        while rank < observed {
            match self.pending_lane_rank.compare_exchange_weak(observed, rank, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(current) => observed = current,
            }
        }
    }

    /// 🚦 Single-flight submission: `WorkerPool::submit`s a fresh [`Self::run`] job only when
    /// `scheduled` was false (the classic Akka-mailbox / "mailbox already has a runner" idiom) — a
    /// `send_frame` that arrives while a drive job is already queued or running just returns, trusting
    /// [`Self::run`]'s post-turn epoch re-check to submit a successor. `pending_lane_rank` is
    /// read-and-reset here for the fresh job's OWN
    /// submission lane; a `send_frame` racing between this read and the reset can have its priority
    /// hint silently overwritten (worst case: the fresh job runs on a less urgent `WorkerPool::Lane`
    /// than it should have) — never a dropped frame, since [`Self::epoch`] (bumped in [`Self::
    /// send_frame`] before this is ever reached) is the sole signal [`Self::run`] trusts for "is
    /// there real work left."
    fn schedule(self: &Arc<Self>) {
        if self.closed.load(Ordering::Acquire) {
            return;
        }
        if self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let admitted_epoch = self.epoch.load(Ordering::Acquire);
        let retained = self.handoff.lock().unwrap_or_else(PoisonError::into_inner).take();
        let (lane, job) = retained.unwrap_or_else(|| {
            let rank = self.pending_lane_rank.swap(NO_LANE, Ordering::AcqRel);
            let worker = Arc::clone(self);
            (pool_lane_for_rank(rank), Box::new(move || worker.run(admitted_epoch)))
        });
        match self.pool.try_submit(lane, job) {
            Ok(()) => {
                self.handoff_retry_attempt.store(0, Ordering::Release);
                self.handoff_retry_armed.store(false, Ordering::Release);
                self.handoff_retry_generation.fetch_add(1, Ordering::AcqRel);
            }
            Err(rejected) => {
                let kind = rejected.kind();
                let job = rejected.into_job();
                self.scheduled.store(false, Ordering::Release);
                match kind {
                    WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned => self.terminalize_handoff(kind, lane, job),
                    WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => {
                        *self.handoff.lock().unwrap_or_else(PoisonError::into_inner) = Some((lane, job));
                        self.arm_handoff_retry();
                    }
                }
            }
        }
    }

    fn arm_handoff_retry(self: &Arc<Self>) {
        if !claim_one_shot(&self.handoff_retry_armed) {
            return;
        }
        let generation = self.handoff_retry_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let attempt = self.handoff_retry_attempt.fetch_add(1, Ordering::AcqRel).saturating_add(1);
        if attempt > 8 {
            self.handoff_retry_armed.store(false, Ordering::Release);
            if let Some((lane, job)) = self.handoff.lock().unwrap_or_else(PoisonError::into_inner).take() {
                self.terminalize_handoff(WorkerSubmitErrorKind::Saturated, lane, job);
            }
            return;
        }
        let deadline = self.pool.now_ms().saturating_add(1u64 << attempt.min(6));
        let executor = Arc::clone(self);
        self.pool.callback_at(deadline, move || {
            if generation != executor.handoff_retry_generation.load(Ordering::Acquire) {
                return;
            }
            executor.handoff_retry_armed.store(false, Ordering::Release);
            if executor.pool.is_shutdown() {
                if let Some((lane, job)) = executor.handoff.lock().unwrap_or_else(PoisonError::into_inner).take() {
                    executor.terminalize_handoff(WorkerSubmitErrorKind::Shutdown, lane, job);
                }
                return;
            }
            if executor.handoff.lock().unwrap_or_else(PoisonError::into_inner).is_some() {
                executor.schedule();
            }
        });
    }

    fn refuse_pending_registrations(&self) {
        let count = self.state.lock().unwrap_or_else(PoisonError::into_inner).registrations.len;
        for _ in 0..count {
            let (bytes, owner) = {
                let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
                let before = state.registrations.bytes;
                let owner = state.registrations.pop_front();
                (before - state.registrations.bytes, owner)
            };
            let Some((_, (actor, instance, reply))) = owner else { break };
            let unclaimed = match reply {
                Some(reply) => {
                    let refusal = RegistrationAdmission::Refused(super::ShardRegistrationRejected { actor, instance, reason: super::ShardRegistrationReason::Stopped });
                    match reply.send(refusal) {
                        Ok(()) => None,
                        Err(RegistrationAdmission::Refused(owner)) => Some(owner.instance),
                        Err(_) => unreachable!("only an owner-bearing refusal was sent"),
                    }
                }
                None => Some(instance),
            };
            if let Some(instance) = unclaimed {
                let returned = self.state.lock().unwrap_or_else(PoisonError::into_inner).registrations.try_push((actor, instance, None), bytes);
                assert!(returned.is_ok(), "terminal handback must retain its original fixed slot credit");
            }
        }
    }

    pub fn take_unclaimed_registration(&self) -> Option<(ActorId, GuestInstance)> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let offset = (0..state.registrations.len).find(|index| state.registrations.get(*index).is_some_and(|(_, _, reply)| reply.is_none()))?;
        state.registrations.pop_at(offset).map(|(_, (actor, instance, _))| (actor, instance))
    }

    fn terminalize_handoff(&self, kind: WorkerSubmitErrorKind, lane: PoolLane, job: PoolJob) {
        self.close_ingress(match kind {
            WorkerSubmitErrorKind::Shutdown => IngressCloseReason::Shutdown,
            WorkerSubmitErrorKind::Poisoned => IngressCloseReason::Poisoned,
            WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => IngressCloseReason::Closing,
        });
        self.closed.store(true, Ordering::Release);
        self.refuse_pending_registrations();
        let previous = self.terminal_handoff.lock().unwrap_or_else(PoisonError::into_inner).replace((kind, lane, job));
        debug_assert!(previous.is_none(), "ShardExecutor: exactly one terminal handoff owner");
    }

    fn request_drive_wake(self: &Arc<Self>, generation: u64) {
        if !claim_drive_wake(self.drive_generation.load(Ordering::Acquire), generation, &self.drive_wake_queued) {
            return;
        }
        if self.drive_waiting.swap(false, Ordering::AcqRel) {
            self.schedule();
        }
    }

    fn retain_failure(&self, failure: crate::PluginHostError) {
        let result = self.failure.lock().unwrap_or_else(PoisonError::into_inner).try_push(failure, size_of::<crate::PluginHostError>());
        if let Err(rejected) = result {
            self.close_ingress(IngressCloseReason::Closing);
            self.closed.store(true, Ordering::Release);
            self.refuse_pending_registrations();
            let previous = self.terminal_failure.lock().unwrap_or_else(PoisonError::into_inner).replace(rejected.owner);
            debug_assert!(previous.is_none(), "ShardExecutor: exactly one terminal failure owner");
        }
    }

    fn acknowledge_consumed_epoch(&self, epoch: u64) {
        let previous = epoch.checked_sub(1).expect("ShardExecutor: shard epochs start at one");
        if self.consumed_epoch.compare_exchange(previous, epoch, Ordering::AcqRel, Ordering::Acquire).is_err() {
            self.retain_failure(crate::PluginHostError::Plugin(format!("ShardExecutor: ingress epoch {epoch} was consumed out of FIFO order")));
        }
    }

    /// 🏃 The `WorkerPool` job body. A stale admitted epoch yields before locking shard state.
    /// A current admission polls exactly one bounded drive opportunity and takes at most one already
    /// buffered outcome with [`ThreadTransport::try_recv_now`]. A successor is attempted only when
    /// retained shard or ingress work remains; finite admission rejection stores the exact closure
    /// returned by [`WorkerPool::try_submit`] for the next retry.
    fn run(self: Arc<Self>, admitted_epoch: u64) {
        if admitted_epoch != self.epoch.load(Ordering::Acquire) {
            self.scheduled.store(false, Ordering::Release);
            self.schedule();
            return;
        }
        self.drive_waiting.store(false, Ordering::Release);
        self.drive_wake_queued.store(false, Ordering::Release);
        let mut drive = {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            assert!(!state.polling, "ShardExecutor: a drive already owns this poll opportunity");
            if state.drive.is_none() {
                let mut shard = state.shard.take().expect("ShardExecutor: retained drive lost shard ownership");
                let registration = state.registrations.pop_front().map(|(_, owner)| owner);
                let primed = shard.can_accept_primed_frame().then(|| self.take_next_ingress_frame()).flatten();
                self.drive_generation.fetch_add(1, Ordering::AcqRel);
                state.drive = Some(Box::pin(async move {
                    if let Some((actor, instance, reply)) = registration {
                        if let Some(reply) = reply.filter(|reply| !reply.is_closed()) {
                            let outcome = match shard.register(actor, instance) {
                                Ok(allocation) => RegistrationAdmission::Admitted(allocation),
                                Err(rejected) => RegistrationAdmission::Refused(rejected),
                            };
                            if let Err(unreceived) = reply.send(outcome) {
                                match unreceived {
                                    RegistrationAdmission::Admitted(_) => shard.unregister(actor).await,
                                    RegistrationAdmission::Refused(rejected) => shard.runtime.drop_instance(rejected.instance).await,
                                    RegistrationAdmission::Rejected { instance, .. } => shard.runtime.drop_instance(instance).await,
                                    RegistrationAdmission::Stopped => {}
                                }
                            }
                        } else {
                            shard.runtime.drop_instance(instance).await;
                        }
                    }
                    let drive = shard.drive_one_primed(primed).await;
                    (shard, drive)
                }));
            }
            state.polling = true;
            state.drive.take().expect("ShardExecutor: drive cursor missing")
        };
        let generation = self.drive_generation.load(Ordering::Acquire);
        let result = poll_retained_drive_once(&mut drive, &self, generation);
        let (polled, registrations_remain) = {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.polling = false;
            let polled = match result {
                Some((shard, drive)) => {
                    if matches!(&drive, ShardDrive::Fault { terminal_overflow: true, .. }) {
                        self.terminal_overflow_occupied.store(true, Ordering::Release);
                        self.close_ingress(IngressCloseReason::TerminalCapacity);
                    } else if matches!(&drive, ShardDrive::Fault { terminal_frame: true, .. }) {
                        self.close_ingress(IngressCloseReason::Closing);
                    }
                    state.shard = Some(shard);
                    Some(drive)
                }
                None => {
                    state.drive = Some(drive);
                    None
                }
            };
            (polled, !state.registrations.is_empty())
        };
        let Some(drive) = polled else {
            self.scheduled.store(false, Ordering::Release);
            self.drive_waiting.store(true, Ordering::Release);
            if self.drive_wake_queued.swap(false, Ordering::AcqRel) && self.drive_waiting.swap(false, Ordering::AcqRel) {
                self.schedule();
            }
            return;
        };
        self.drive_generation.fetch_add(1, Ordering::AcqRel);
        let (consumed_epoch, shard_more, terminal_overflow) = match &drive {
            ShardDrive::Idle { consumed_epoch } => (*consumed_epoch, false, false),
            ShardDrive::MoreWork { consumed_epoch } => (*consumed_epoch, true, false),
            ShardDrive::Blocked => (None, false, false),
            ShardDrive::Fault { consumed_epoch, work_remains, terminal_overflow, .. } => (*consumed_epoch, *work_remains, *terminal_overflow),
        };
        if let Some(epoch) = consumed_epoch {
            self.acknowledge_consumed_epoch(epoch);
        }
        if let Some(bytes) = self.kernel_side.try_recv_now() {
            let mut pos = 0usize;
            match poll_drive_once(ShardOutcome::pack_decode(&bytes, &mut pos)) {
                Some(Ok(outcome)) => self.outcomes.push(outcome),
                Some(Err(error)) => self.retain_failure(crate::PluginHostError::Plugin(format!("ShardExecutor: malformed outcome: {error:?}"))),
                None => self.retain_failure(crate::PluginHostError::Plugin("ShardExecutor: outcome decoder suspended without a retained cursor".to_string())),
            }
        }
        if let ShardDrive::Fault { error, .. } = drive {
            self.retain_failure(error);
        }
        let work_remains = !terminal_overflow && (registrations_remain || shard_more || self.consumed_epoch.load(Ordering::Acquire) < self.epoch.load(Ordering::Acquire));
        self.scheduled.store(false, Ordering::Release);
        if !terminal_overflow && (work_remains || self.consumed_epoch.load(Ordering::Acquire) < self.epoch.load(Ordering::Acquire)) {
            self.schedule();
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
