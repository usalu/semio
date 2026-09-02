//! 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop; rewritten by
//! INTERACTIVE-JOB-RUNTIME-REFACTOR Phase 1, packet P1e). `ParallelRuntime` owns one
//! [`semio_framework_actor::Kernel`] and K [`semio_framework_plugin_host::shard::executor::
//! ShardExecutor`]s — real `Kernel::submit`/`tick`/`complete` (DRR fairness, failure-ladder/metrics
//! bookkeeping) dispatched to each granted [`semio_framework_actor::TurnGrant`]'s PINNED shard.
//!
//! P1e retires the K dedicated `"semio-kernel-shard-forward-*"` OS threads (one `ShardExecutor`
//! thread + one outcome-forwarder thread per shard) this file used to spawn: P1c
//! (`.🧬semio/…/PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1c-actor-shards.md`) turned [`ShardExecutor`] into
//! a logical affinity unit — a single-flight `std::sync::Mutex`-guarded scheduling protocol — that
//! runs its turns as ordinary jobs on ONE shared, process-wide [`semio_framework_async::WorkerPool`]
//! instead of owning a thread. Turn outcomes flow back via
//! [`semio_framework_plugin_host::shard::executor::OutcomeSink`], pushed directly by whichever pool
//! worker executed the turn — "completion notification through the pool," never a polled channel —
//! so the `//#region 🔀️OutcomeForwarding` this file used to carry (its own forwarder-thread fan-in,
//! `std::sync::mpsc`-backed, `FORWARD_POLL`-bounded) no longer has anything to bridge.
//!
//! This type is a near-verbatim mirror of `semio-framework-os`'s own `NativeKernelRuntime`
//! (`🖥️host/🎠️activation/🦀️.rs`, P1c's own "parallel implementation of the same proven pattern" —
//! see that file's module doc for why the two are not yet unified into one shared type) with exactly
//! ONE deliberate deviation: **`ParallelRuntime::new` does not construct its own `WorkerPool`.** P1a's
//! and P1b's own reports (`📓️p1a-worker-pool.md`, `📓️p1b-services.md`) both name this file explicitly
//! as "the natural place a future packet should inject a single real, externally-owned `WorkerPool`
//! instead of ever falling through to a crate-private lazy default" — the renderer is not allowed to
//! size or own its own thread pool, that is the entire point of Phase 1. The caller
//! (`🦀️.rs`'s `crate::renderer_worker_pool()`, `kernel_runtime::KernelThreadState::new`) injects
//! the ONE process-wide pool this whole renderer crate shares — with the directory-client
//! `TokioHostRuntime` in `Shell/🎯️targets/🧊️wgpu/🦀️.rs` — rather than this type minting a second one.
//!
//! Native-only end to end (unchanged): [`semio_framework_actor::ShardKind::Native`] — the execution
//! HOST is "native process, shared pool," never wasm — so this module stays mounted
//! `#[cfg(not(target_arch = "wasm32"))]` from `🦀️.rs`.

use semio_framework::kernel::{BrokerCapabilityGrant, Budget as TurnBudget, TurnResult as KernelTurnResult};
use semio_framework_actor::{ActivationEvent, ActorId, ActorKind, Backpressure, Decision, Envelope, FailureEscalation, Kernel, KernelError, Lane, PackageId, ShardKind, WindowId};
use semio_framework_async::WorkerPool;
use semio_framework_plugin_host::shard::executor::{OutcomeSink, ShardExecutor};
use semio_framework_plugin_host::shard::{to_actor_turn_result, ShardFrame, ShardOutcome};
use semio_framework_plugin_host::{CompiledHandle, GuestRuntime, GuestRuntimes};
use std::sync::Arc;
use std::time::Duration;

/// 🎠️ Owns one [`Kernel`] and K [`ShardExecutor`]s, all scheduled onto ONE shared, caller-injected
/// [`WorkerPool`] — see the module doc for the full mechanism and why this type does not build its
/// own pool. `activate`/`submit`/`tick_and_dispatch`/`complete` are the same façade shape `Kernel`
/// itself exposes, widened to also own the shard/transport plumbing `Kernel`'s own purity rule keeps
/// out of that crate.
pub struct ParallelRuntime {
    kernel: Kernel,
    guest_runtime: Arc<GuestRuntimes>,
    shards: Vec<Arc<ShardExecutor>>,
    outcomes: Arc<OutcomeSink>,
}

impl ParallelRuntime {
    /// ▶️ Builds `shard_count.max(1)` [`ShardExecutor`]s sharing the CALLER'S OWN `pool`, and one
    /// [`Kernel`]. No threads are spawned by this constructor — every `ShardExecutor` is
    /// pool-scheduled, only actually running a job once its first `ShardFrame` arrives via
    /// [`Self::activate`]/[`Self::tick_and_dispatch`]. `exclusive_reserve`/`grants_per_tick` pass
    /// straight through to `Kernel::new` — see that constructor's own doc for what each controls.
    pub async fn new(pool: Arc<WorkerPool>, guest_runtime: Arc<GuestRuntimes>, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> Self {
        let shard_count = shard_count.max(1);
        let kernel = Kernel::new(ShardKind::Native, shard_count, exclusive_reserve, grants_per_tick).await;
        let outcomes = OutcomeSink::new();
        let mut shards = Vec::with_capacity(shard_count as usize);
        for _ in 0..shard_count {
            shards.push(ShardExecutor::new(pool.clone(), guest_runtime.clone(), Vec::new(), outcomes.clone()).await);
        }
        Self { kernel, guest_runtime, shards, outcomes }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    pub fn kernel_mut(&mut self) -> &mut Kernel {
        &mut self.kernel
    }

    pub fn shard_count(&self) -> u16 {
        self.shards.len() as u16
    }

    /// ▶️ `Kernel::activate` (mints the `ActorId`, pins it to a shard via `ShardTable::pin`) +
    /// `GuestRuntime::instantiate` (host-side, may run on ANY thread — `GuestInstance` is `Send`) +
    /// `ShardExecutor::register` (hands the freshly-built `GuestInstance` to the SPECIFIC executor
    /// `ShardTable::pin` assigned — the one logical shard that will ever touch its `wasmtime::Store`
    /// from here on, regardless of which physical pool worker happens to run any given turn).
    /// `instantiate_budget` is the ceiling `GuestRuntime::instantiate` itself wants (independent of
    /// whatever `Kernel::tick` later grants per turn) — callers already compute one for their own
    /// purpose (`kernel_runtime::TURN_BUDGET`, `scale_bench::turn_budget_of`), so this takes it rather
    /// than re-deriving a third value.
    #[allow(clippy::too_many_arguments)]
    pub async fn activate(
        &mut self,
        package: PackageId,
        plugin_ordinal: u16,
        kind: ActorKind,
        lane: Lane,
        window: Option<WindowId>,
        event: ActivationEvent,
        compiled: &CompiledHandle,
        caps: &[BrokerCapabilityGrant],
        instantiate_budget: &TurnBudget,
    ) -> Result<ActorId, String> {
        let actor = self.kernel.activate(package, plugin_ordinal, kind, lane, window, event).await;
        let shard_index = self.kernel.actor_record(actor).await.map(|record| record.shard.0 as usize).unwrap_or(0);
        let instance = match self.guest_runtime.instantiate(compiled, actor, caps, instantiate_budget).await {
            Ok(instance) => instance,
            Err(error) => return Err(error.to_string()),
        };
        match self.shards.get(shard_index) {
            Some(shard) => {
                shard.register(actor, instance).await;
                Ok(actor)
            }
            None => Err(format!("ParallelRuntime::activate: ShardTable::pin assigned shard {shard_index} but only {} shards were spawned", self.shards.len())),
        }
    }

    /// ✉️ `Kernel::submit` — enqueues onto the actor's DRR mailbox; drained by the NEXT
    /// `tick_and_dispatch`. Callers must honour a non-[`Backpressure::Accept`] result the same way
    /// `Scheduler::submit`'s own doc already documents (a `Rejected`/`Dropped` must surface as a busy
    /// badge, never a silent drop) — this method does not retry or coalesce on the caller's behalf.
    pub async fn submit(&mut self, envelope: &Envelope) -> Backpressure {
        self.kernel.submit(envelope).await
    }

    /// ⏱️ `Kernel::tick(now_ms)`, then dispatches every granted [`semio_framework_actor::TurnGrant`]
    /// to its own pinned shard's `ShardExecutor::send_frame` — one `WorkerPool` job submission per
    /// shard that received at least one grant this tick, on whichever `Lane` the grant's own
    /// envelopes carry (every envelope for one actor shares a lane, fixed at scheduler registration).
    ///
    /// 🐛️ terra-kernel-loop finding, load-bearing (unchanged by P1e): the grant's OWN `budget` field
    /// is NOT used for the dispatched frame. `Kernel::activate` has no per-actor budget parameter at
    /// all — it always computes the SCHEDULED (and later throttle-scaled) budget from
    /// `lane_defaults::budget_for(lane)`, a 4-tier table with no room for a caller's own ceiling. Both
    /// native callers this file serves carry their OWN, deliberately-higher fuel ceilings for a
    /// documented reason (`kernel_runtime::TURN_BUDGET`, `scale_bench::BENCH_FUEL`) — dispatching
    /// `grant.budget` verbatim would silently fuel-starve nearly every real turn. `budget_for` lets
    /// the caller supply the ACTUAL per-actor ceiling to dispatch, keeping `Kernel::tick`'s real
    /// value: WHO is due and WHICH envelopes/shard, i.e. genuine DRR fairness and shard routing.
    ///
    /// Returns the raw `Decision` so a caller can honour `wake_at` for its own park deadline.
    pub async fn tick_and_dispatch(&mut self, now_ms: u64, budget_for: impl Fn(ActorId) -> semio_framework_actor::Budget) -> Decision {
        let decision = self.kernel.tick(now_ms).await;
        for grant in &decision.run {
            let shard_index = grant.shard.0 as usize;
            let Some(shard) = self.shards.get(shard_index) else { continue };
            let lane = grant.envelopes.first().map(|envelope| envelope.lane).unwrap_or(Lane::Maintenance);
            let mut bytes = Vec::new();
            ShardFrame::Grant { actor: grant.actor, budget: budget_for(grant.actor), envelopes: grant.envelopes.clone() }.pack_encode(&mut bytes).await;
            shard.send_frame(bytes, lane).await;
        }
        decision
    }

    /// ✂️ Mirrors `activate`: sends a `ShardFrame::Unregister` to the actor's own pinned shard.
    /// `Kernel` itself has no actor-retirement method (`activate`/`submit`/`tick`/`complete`/
    /// `suspend`/`resume`/`request_exclusive`/`commit_frame` is its whole façade) — this only retires
    /// the SHARD-side `GuestInstance`; a stale `Kernel`-level registry entry for a destroyed actor is
    /// a pre-existing gap this file did not introduce and does not close.
    pub async fn unregister(&mut self, actor: ActorId) {
        let Some(record) = self.kernel.actor_record(actor).await else { return };
        let Some(shard) = self.shards.get(record.shard.0 as usize) else { return };
        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes).await;
        shard.send_frame(bytes, Lane::Maintenance).await;
    }

    /// 🌉️ `to_actor_turn_result` + `Kernel::complete`. Callers pass the RAW
    /// `semio_framework::kernel::TurnResult` a `ShardOutcome::Turn` carried, plus host-measured
    /// `wall_us`/`memory_bytes` (this crate has no clock of its own by design).
    pub async fn complete(&mut self, actor: ActorId, result: KernelTurnResult, wall_us: u64, memory_bytes: u64, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        let actor_result = to_actor_turn_result(result, actor.0, wall_us, memory_bytes).await.map_err(|_| KernelError::InvalidTransition)?;
        self.kernel.complete(actor, &actor_result, now_ms).await
    }

    /// 🎭️ Completes a turn already returned across the shard wire in the actor scheduler's native
    /// result shape, avoiding a lossy actor → kernel → actor round trip for scheduler bookkeeping.
    pub async fn complete_actor(&mut self, actor: ActorId, result: &semio_framework_actor::TurnResult, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        self.kernel.complete(actor, result, now_ms).await
    }

    /// 🌀️ Drains every `ShardOutcome` CURRENTLY buffered across every shard's `OutcomeSink` — never
    /// blocks. Malformed bytes cannot reach here (an `OutcomeSink` only ever holds successfully
    /// decoded outcomes — decoding happens once, inside `ShardExecutor::run`).
    pub fn try_recv_outcomes(&self) -> Vec<ShardOutcome> {
        self.outcomes.try_recv_all()
    }

    /// ⏳️ Blocks the calling thread until EITHER `expected` outcomes have been collected OR
    /// `timeout` elapses. This is the primitive both the interactive host's per-exchange wait and the
    /// scale-bench harness's own round-trip latency measurement (bench budget 5) are built on — the
    /// ELAPSED time this method actually spends IS the shard-dispatch latency the packet's
    /// acceptance gate measures. Callers of this file are already off the winit/UI thread (this
    /// facade runs on the dedicated `"semio-kernel"` thread — see `kernel_runtime::KernelClient::get`
    /// — or `scale_bench`'s own standalone process), so a genuine blocking wait here is correct, not
    /// a UI-thread stall.
    pub fn wait_for_outcomes(&self, expected: usize, timeout: Duration) -> Vec<ShardOutcome> {
        self.outcomes.wait_for(expected, timeout)
    }
}
