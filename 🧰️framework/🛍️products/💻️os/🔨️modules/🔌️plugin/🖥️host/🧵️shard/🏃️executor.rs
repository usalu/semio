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
//! forwarder thread necessary in the first place (`💻️os/🖥️host/🎠️activation.rs`'s deleted
//! `semio-os-host-kernel-shard-forward-*` threads) no longer exists.

use super::{AdmissionLimit, DeferredAuthority, FixedOwnerRing, ShardDrive, ShardLoop, ShardOutcome, ShardTransports, SHARD_DEFERRED_BYTES, SHARD_DEFERRED_ITEMS, SHARD_FRAME_MAX_BYTES};
use crate::{GuestInstance, GuestRuntimes};
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
/// to this crate; `🧵️shard/🦀️component.rs`'s own `LoopbackProbe` doc comment already names this same
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

type ShardDriveFuture = Pin<Box<dyn Future<Output = (ShardLoop, ShardDrive)> + Send>>;

struct ShardExecutorState {
    shard: Option<ShardLoop>,
    drive: Option<ShardDriveFuture>,
    registrations: FixedOwnerRing<(ActorId, GuestInstance), SHARD_DEFERRED_ITEMS>,
}

pub enum RegistrationAdmission {
    Admitted,
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
            shard.register(actor, instance);
        }
        Arc::new(ShardExecutor {
            state: Mutex::new(ShardExecutorState { shard: Some(shard), drive: None, registrations: FixedOwnerRing::new(SHARD_DEFERRED_BYTES) }),
            kernel_side,
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

    /// 🆕️ Registers `instance` on this shard's `ShardLoop` directly, under `state`'s mutex — no
    /// thread to hand a `RegisterRequest` to anymore, so no ack rendezvous, no
    /// `REGISTER_ACK_TIMEOUT`: a caller blocks (briefly — bounded by however long the CURRENT pump
    /// job, if any, takes to finish its `wasmtime` turn budget) on the same mutex a pump job would,
    /// then applies immediately. The interleaving `terra-shard-routing`'s ack fixed (a `Grant` sent
    /// right after `register()` returning reaching a still-parked executor thread BEFORE that
    /// thread's own registration drain) cannot recur: there is no second thread with its own drain
    /// cadence to race against — `register` and every pump job serialize on the SAME lock.
    pub async fn register(self: &Arc<Self>, actor: ActorId, instance: GuestInstance) -> RegistrationAdmission {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        match state.shard.as_mut() {
            Some(shard) => {
                shard.register(actor, instance);
                RegistrationAdmission::Admitted
            }
            None => match state.registrations.try_push((actor, instance), std::mem::size_of::<(ActorId, GuestInstance)>()) {
                Ok(_) => {
                    drop(state);
                    self.schedule();
                    RegistrationAdmission::Admitted
                }
                Err(rejected) => RegistrationAdmission::Rejected { actor: rejected.owner.0, instance: rejected.owner.1, limit: rejected.limit },
            },
        }
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

    /// ✉️ Injects one already-encoded [`super::ShardFrame`]'s bytes (a `Grant` or `Unregister`) and
    /// schedules a pump job on `lane` — the caller-visible replacement for the old
    /// `kernel_side.send(&bytes)` + relying on the executor thread's own 5ms park/pump cadence to
    /// notice it. `lane` should be the triggering envelope's own `ActorLane` (a `TurnGrant`'s
    /// envelopes all share one lane, fixed per actor at scheduler registration) — see
    /// [`Self::schedule`] for what this actually buys versus costs on a race.
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
        if let Err(frame) = self.kernel_side.send_now(bytes) {
            self.ingress_state.store(1, Ordering::Release);
            return FrameIngress::Rejected(TerminalFrameOwner { reason: IngressCloseReason::Closing, frame });
        }
        self.epoch.fetch_add(1, Ordering::SeqCst);
        self.bump_lane_hint(lane);
        drop(ingress);
        self.schedule();
        FrameIngress::Admitted
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

    fn terminalize_handoff(&self, kind: WorkerSubmitErrorKind, lane: PoolLane, job: PoolJob) {
        self.close_ingress(match kind {
            WorkerSubmitErrorKind::Shutdown => IngressCloseReason::Shutdown,
            WorkerSubmitErrorKind::Poisoned => IngressCloseReason::Poisoned,
            WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => IngressCloseReason::Closing,
        });
        self.closed.store(true, Ordering::Release);
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
        let result = self.failure.lock().unwrap_or_else(PoisonError::into_inner).try_push(failure, std::mem::size_of::<crate::PluginHostError>());
        if let Err(rejected) = result {
            self.close_ingress(IngressCloseReason::Closing);
            self.closed.store(true, Ordering::Release);
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
        let (polled, registrations_remain) = {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            if state.drive.is_none() {
                let mut shard = state.shard.take().expect("ShardExecutor: retained drive lost shard ownership");
                if let Some((_, (actor, instance))) = state.registrations.pop_front() {
                    shard.register(actor, instance);
                }
                self.drive_generation.fetch_add(1, Ordering::AcqRel);
                state.drive = Some(Box::pin(async move {
                    let drive = shard.drive_one().await;
                    (shard, drive)
                }));
            }
            let generation = self.drive_generation.load(Ordering::Acquire);
            let polled = match poll_retained_drive_once(state.drive.as_mut().expect("ShardExecutor: drive cursor missing"), &self, generation) {
                Some((shard, drive)) => {
                    if matches!(&drive, ShardDrive::Fault { terminal_overflow: true, .. }) {
                        self.terminal_overflow_occupied.store(true, Ordering::Release);
                        self.close_ingress(IngressCloseReason::TerminalCapacity);
                    } else if matches!(&drive, ShardDrive::Fault { terminal_frame: true, .. }) {
                        self.close_ingress(IngressCloseReason::Closing);
                    }
                    state.drive = None;
                    state.shard = Some(shard);
                    Some(drive)
                }
                None => None,
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
mod tests {
    use super::*;
    use crate::{GuestRuntime, MockGuestRuntime, PackageHash, PackageId, PackageRef};
    use semio_framework::kernel::Budget;
    use semio_framework_actor::{ActorId, Envelope, JobOperation, Payload, ShardKind, ShardTable};
    use semio_framework_async::{ProcessKind, WorkerPoolConfig};
    use std::time::Duration;

    async fn encode_frame(frame: super::super::ShardFrame) -> Vec<u8> {
        let mut bytes = Vec::new();
        frame.pack_encode(&mut bytes).await;
        bytes
    }

    fn test_pool() -> Arc<WorkerPool> {
        Arc::new(WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 4)))
    }

    fn wait_for_one(outcomes: &Arc<OutcomeSink>) -> ShardOutcome {
        let mut got = outcomes.wait_for(1, Duration::from_secs(2));
        got.pop().unwrap_or_else(|| panic!("no outcome received within the wait window"))
    }

    /// 🎯️ End-to-end proof: a `ShardExecutor`'s turn is genuinely driven by a `WorkerPool` job (no
    /// dedicated thread of its own) in response to `send_frame` — not merely that construction
    /// succeeds.
    #[semio_framework_async_macros::async_test]
    async fn shard_executor_drives_a_turn_for_a_registered_actor_via_the_worker_pool() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(101);
        let package = PackageRef { package: PackageId("executor-smoke".to_string()), hash: PackageHash([30u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        let mut scripted = MockGuestRuntime::idle_turn().await;
        scripted.fuel_used = 77;
        mock.script_turn(actor, scripted).await;

        let pool = test_pool();
        let outcomes = OutcomeSink::new();
        let executor = ShardExecutor::new(pool, Arc::new(GuestRuntimes::Mock(mock.clone())), vec![(actor, instance)], outcomes.clone()).await;

        let envelope = Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: serde_json::to_vec(&semio_framework::kernel::Event::InstanceClose).unwrap() },
        };
        let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        let bytes = encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![envelope] }).await;
        executor.send_frame(bytes, semio_framework_actor::Lane::Interactive).await;

        match wait_for_one(&outcomes) {
            ShardOutcome::Turn { actor: reported, result } => {
                assert_eq!(reported, actor.0);
                assert_eq!(result.usage.fuel, 77, "the scripted turn's own fuel_used must round-trip through the pool job, proving it genuinely ran pump() there");
            }
            other => panic!("expected ShardOutcome::Turn from the pool job, got {other:?}"),
        }
    }

    /// 🎯️ PROPERTY (terra-shard-routing, re-verified under the pool-scheduled model): for K real
    /// `ShardExecutor`s and N actors pinned by the SAME [`ShardTable::pin`] a real `Kernel::activate`
    /// uses, every actor's own `Grant` must arrive at a shard where THAT actor is already registered
    /// — never `ShardOutcome::Fault`. `register` is now synchronous under `state`'s own mutex (no
    /// ack rendezvous needed — see `ShardExecutor::register`'s own doc for why the old race cannot
    /// recur), so this reproduces the identical zero-slack register-then-dispatch pipeline
    /// `NativeKernelRuntime::activate` + `tick_and_dispatch` produce in production.
    #[semio_framework_async_macros::async_test]
    async fn every_actors_grant_lands_on_the_shard_it_was_registered_on_across_k_shards() {
        const SHARDS: u16 = 4;
        const ACTORS: usize = 200;

        let mock = Arc::new(MockGuestRuntime::new().await);
        let pool = test_pool();
        let outcomes = OutcomeSink::new();
        let mut executors = Vec::new();
        for _ in 0..SHARDS {
            executors.push(ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(mock.clone())), Vec::new(), outcomes.clone()).await);
        }
        let mut shards = ShardTable::new(ShardKind::Native, SHARDS, 0).await;
        let package = PackageRef { package: PackageId("grant-routing-property".to_string()), hash: PackageHash([55u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let grant_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);

        for i in 0..ACTORS {
            let actor = ActorId::new(0, 0, i as u32, 0).await;
            let shard_id = shards.pin(actor).await;
            let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
            mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
            executors[shard_id.0 as usize].register(actor, instance).await;
            let envelope = Envelope {
                to: actor,
                from: semio_framework_actor::Origin::Kernel,
                lane: semio_framework_actor::Lane::Interactive,
                seq: i as u64 + 1,
                deadline_ms: None,
                coalesce: None,
                cancel_of: None,
                payload: Payload::Event { bytes: serde_json::to_vec(&semio_framework::kernel::Event::InstanceClose).unwrap() },
            };
            let bytes = encode_frame(super::super::ShardFrame::Grant { actor, budget: grant_budget, envelopes: vec![envelope] }).await;
            executors[shard_id.0 as usize].send_frame(bytes, semio_framework_actor::Lane::Interactive).await;
        }

        let collected = outcomes.wait_for(ACTORS, Duration::from_secs(10));
        assert_eq!(collected.len(), ACTORS, "every one of {ACTORS} actors must produce exactly one outcome");
        for outcome in collected {
            match outcome {
                ShardOutcome::Turn { .. } => {}
                other => panic!("expected Turn for every actor, got {other:?} — every actor's own Grant must arrive at a shard where it is already registered"),
            }
        }
    }

    /// 🎯️ PROPERTY: a suspend→resume round trip must land back on a shard where the actor is
    /// registered, under the pool-scheduled model.
    #[semio_framework_async_macros::async_test]
    async fn suspend_then_resume_round_trip_lands_on_a_shard_where_the_actor_is_registered() {
        const SHARDS: u16 = 4;
        const ACTORS: usize = 60;

        let mock = Arc::new(MockGuestRuntime::new().await);
        let pool = test_pool();
        let outcomes = OutcomeSink::new();
        let mut executors = Vec::new();
        for _ in 0..SHARDS {
            executors.push(ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(mock.clone())), Vec::new(), outcomes.clone()).await);
        }
        let mut shards = ShardTable::new(ShardKind::Native, SHARDS, 0).await;
        let package = PackageRef { package: PackageId("suspend-resume-property".to_string()), hash: PackageHash([77u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };

        for i in 0..ACTORS {
            let actor = ActorId::new(0, 0, i as u32, 0).await;
            let shard_id = shards.pin(actor).await;
            let executor = &executors[shard_id.0 as usize];
            let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
            executor.register(actor, instance).await;

            let operation = JobOperation { operation: actor.0, base_revision: 0, generation: actor.generation() as u64, preview_sequence: 0, seed: actor.0 };
            let suspend = Envelope {
                to: actor,
                from: semio_framework_actor::Origin::Kernel,
                lane: semio_framework_actor::Lane::Background,
                seq: 1,
                deadline_ms: None,
                coalesce: None,
                cancel_of: None,
                payload: Payload::Suspend { operation, applied_progress: i as u64 },
            };
            executor.send_frame(encode_frame(super::super::ShardFrame::Envelope(suspend)).await, semio_framework_actor::Lane::Background).await;
            let state = match wait_for_one(&outcomes) {
                ShardOutcome::Checkpoint { actor: reported, operation: reported_operation, checkpoint } => {
                    assert_eq!(reported, actor.0);
                    assert_eq!(reported_operation, operation);
                    assert_eq!(checkpoint.applied_progress, i as u64);
                    checkpoint
                }
                other => panic!("actor {i}: expected Checkpoint outcome for Suspend, got {other:?} — a just-registered actor's own Suspend must never fault"),
            };

            let fresh_instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate (fresh)");
            executor.register(actor, fresh_instance).await;
            let resume =
                Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Resume { operation, checkpoint: state } };
            executor.send_frame(encode_frame(super::super::ShardFrame::Envelope(resume)).await, semio_framework_actor::Lane::Background).await;
            match wait_for_one(&outcomes) {
                ShardOutcome::Resumed { actor: reported, operation: reported_operation } => {
                    assert_eq!(reported, actor.0);
                    assert_eq!(reported_operation, operation);
                }
                other => panic!("actor {i}: expected Resumed outcome, got {other:?} — a Resume dispatched right after register() must find the actor already registered, never fault"),
            }
        }
    }

    /// 🎯️ A burst of `send_frame` calls that all race BEFORE the first pump job even starts must
    /// still each produce exactly one outcome — the single-flight `scheduled`/`epoch` protocol must
    /// neither drop a frame nor deadlock when every caller sees `scheduled` already true.
    #[semio_framework_async_macros::async_test]
    async fn concurrent_send_frame_bursts_never_drop_an_outcome() {
        const ACTORS: usize = 32;
        let mock = Arc::new(MockGuestRuntime::new().await);
        let pool = test_pool();
        let outcomes = OutcomeSink::new();
        let package = PackageRef { package: PackageId("burst-property".to_string()), hash: PackageHash([9u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instantiate_budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let mut initial = Vec::new();
        let mut actors = Vec::new();
        for i in 0..ACTORS {
            let actor = ActorId::new(0, 0, i as u32, 0).await;
            let instance = mock.instantiate(&compiled, actor, &[], &instantiate_budget).await.expect("mock instantiate");
            mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
            initial.push((actor, instance));
            actors.push(actor);
        }
        let executor = ShardExecutor::new(pool, Arc::new(GuestRuntimes::Mock(mock.clone())), initial, outcomes.clone()).await;
        let budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);

        let mut handles = Vec::new();
        for (i, actor) in actors.into_iter().enumerate() {
            let executor = Arc::clone(&executor);
            handles.push(std::thread::spawn(move || {
                let envelope = Envelope {
                    to: actor,
                    from: semio_framework_actor::Origin::Kernel,
                    lane: semio_framework_actor::Lane::Interactive,
                    seq: i as u64 + 1,
                    deadline_ms: None,
                    coalesce: None,
                    cancel_of: None,
                    payload: Payload::Event { bytes: serde_json::to_vec(&semio_framework::kernel::Event::InstanceClose).unwrap() },
                };
                let bytes = semio_framework_async::block_on(encode_frame(super::super::ShardFrame::Grant { actor, budget, envelopes: vec![envelope] }));
                semio_framework_async::block_on(executor.send_frame(bytes, semio_framework_actor::Lane::Interactive));
            }));
        }
        for handle in handles {
            handle.join().expect("sender thread panicked");
        }

        let collected = outcomes.wait_for(ACTORS, Duration::from_secs(10));
        assert_eq!(collected.len(), ACTORS, "every concurrently-sent frame must still produce exactly one outcome — none dropped by the single-flight race");
    }

    #[test]
    fn pending_drive_wake_and_wake_storm_claim_exactly_one_schedule() {
        let queued = AtomicBool::new(false);
        assert!(claim_drive_wake(7, 7, &queued), "the first matching readiness wake owns scheduling");
        for _ in 0..64 {
            assert!(!claim_drive_wake(7, 7, &queued), "a wake storm cannot claim a second schedule");
        }
        queued.store(false, Ordering::Release);
        assert!(!claim_drive_wake(8, 7, &queued), "a stale drive generation cannot schedule or mutate the current cursor");
        assert!(!queued.load(Ordering::Acquire));
    }

    #[test]
    fn retry_trigger_is_one_shot_until_the_generation_owner_releases_it() {
        let armed = AtomicBool::new(false);
        assert!(claim_one_shot(&armed));
        assert!(!claim_one_shot(&armed), "concurrent saturation notices coalesce behind one timer authority");
        armed.store(false, Ordering::Release);
        assert!(claim_one_shot(&armed), "the callback may arm one later bounded generation");
    }

    #[semio_framework_async_macros::async_test]
    async fn terminal_ingress_returns_the_exact_late_frame_before_transport_or_epoch_mutation() {
        let pool = test_pool();
        let executor = ShardExecutor::new(pool.clone(), Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await))), Vec::new(), OutcomeSink::new()).await;
        pool.shutdown();
        let raw = vec![4, 3, 2, 1];
        let owner = match executor.send_frame(raw.clone(), semio_framework_actor::Lane::Maintenance).await {
            FrameIngress::Rejected(owner) => owner,
            FrameIngress::Admitted => panic!("shutdown ingress cannot admit a late frame"),
        };
        assert_eq!(owner.reason(), IngressCloseReason::Shutdown);
        assert_eq!(owner.into_frame(), raw, "late ingress returns the exact original owner");
        assert_eq!(executor.epoch.load(Ordering::Acquire), 0, "rejected late ingress cannot mutate the admitted epoch");
        assert!(executor.kernel_side.try_recv_now().is_none(), "rejected late ingress never reaches transport ownership");
    }

    #[semio_framework_async_macros::async_test]
    async fn over_capacity_ingress_hands_back_bytes_plus_one_exactly() {
        let executor = ShardExecutor::new(test_pool(), Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await))), Vec::new(), OutcomeSink::new()).await;
        let raw = vec![7; SHARD_FRAME_MAX_BYTES + 1];
        let owner = match executor.send_frame(raw, semio_framework_actor::Lane::Maintenance).await {
            FrameIngress::Rejected(owner) => owner,
            FrameIngress::Admitted => panic!("raw Grant credit plus one cannot be admitted"),
        };
        assert_eq!(owner.reason(), IngressCloseReason::OverCapacity);
        assert_eq!(owner.into_frame().len(), SHARD_FRAME_MAX_BYTES + 1);
        assert_eq!(executor.epoch.load(Ordering::Acquire), 0);
        assert!(executor.kernel_side.try_recv_now().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn occupied_terminal_overflow_rejects_plus_one_and_plus_two_before_mutation() {
        let executor = ShardExecutor::new(test_pool(), Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await))), Vec::new(), OutcomeSink::new()).await;
        executor.terminal_overflow_occupied.store(true, Ordering::Release);
        let first = vec![0xf1, 1];
        let second = vec![0xf2, 2];
        let first_owner = match executor.send_frame(first.clone(), semio_framework_actor::Lane::Maintenance).await {
            FrameIngress::Rejected(owner) => owner,
            FrameIngress::Admitted => panic!("occupied terminal overflow cannot accept plus one"),
        };
        let second_owner = match executor.send_frame(second.clone(), semio_framework_actor::Lane::Maintenance).await {
            FrameIngress::Rejected(owner) => owner,
            FrameIngress::Admitted => panic!("occupied terminal overflow cannot accept plus two"),
        };
        assert_eq!(first_owner.reason(), IngressCloseReason::TerminalCapacity);
        assert_eq!(first_owner.into_frame(), first, "plus one returns its exact owner");
        assert_eq!(second_owner.reason(), IngressCloseReason::TerminalCapacity);
        second_owner.close();
        assert_eq!(executor.epoch.load(Ordering::Acquire), 0, "overflow rejection accepts no epoch");
        assert!(executor.kernel_side.try_recv_now().is_none(), "overflow rejection mutates no transport owner");
    }
}
