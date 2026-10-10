//! 🎠️ Rendered native Kernel coordination over the caller-owned process worker pool.
//! Guest activation ownership is shared with the headless host through plugin-host activation.

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
    dispatch: semio_framework_plugin_host::shard::grant::OriginalShardDispatch,
    dispatch_bytes: Vec<u8>,
}

impl ParallelRuntime {
    /// ▶️ Builds `shard_count.max(1)` [`ShardExecutor`]s sharing the CALLER'S OWN `pool`, and one
    /// [`Kernel`]. No threads are spawned by this constructor — every `ShardExecutor` is
    /// pool-scheduled, only actually running a job once its first `ShardFrame` arrives via
    /// [`Self::activate`]/[`Self::tick_and_dispatch`]. `exclusive_reserve`/`grants_per_tick` pass
    /// straight through to `Kernel::new` — see that constructor's own doc for what each controls.
    pub async fn new(pool: Arc<WorkerPool>, guest_runtime: Arc<GuestRuntimes>, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32, mut identity_issuers:impl FnMut(u16)->semio_framework_plugin_host::shard::OriginalShardIdentityIssuer) -> Self {
        let shard_count = shard_count.max(1);
        let kernel = Kernel::new(ShardKind::Native, shard_count, exclusive_reserve, grants_per_tick).await;
        let outcomes = OutcomeSink::new();
        let mut shards = Vec::with_capacity(shard_count as usize);
        for shard in 0..shard_count {
            shards.push(ShardExecutor::new(pool.clone(), guest_runtime.clone(), Vec::new(), outcomes.clone(),identity_issuers(shard)).await);
        }
        Self { kernel, guest_runtime, shards, outcomes, dispatch: Default::default(), dispatch_bytes: Vec::new() }
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
    ) -> Result<ActorId, semio_framework_plugin_host::activation::ActivationRefusal> {
        let request = semio_framework_actor::activation::KernelActivationRequest { package, plugin_ordinal, kind, lane, window, event, retained: instantiate_budget.retained };
        let reservation = self.kernel.reserve_activation(request).await.map_err(|refused| semio_framework_plugin_host::activation::ActivationRefusal::host(format!("Kernel activation refused: {:?}", refused.reason)))?;
        semio_framework_plugin_host::activation::install_actor(&mut self.kernel, &self.guest_runtime, &self.shards, reservation, compiled, caps, instantiate_budget).await
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
    /// 🎟️ Host resource overrides retain the scheduler's exact original receipt and five-axis grant.
    ///
    /// Returns the raw `Decision` so a caller can honour `wake_at` for its own park deadline.
    pub fn dispatch_refusal(&self) -> Option<&semio_framework_actor::pack::PackError> { self.dispatch.refusal() }
    pub fn pending_decision(&self) -> Option<&Decision> { self.dispatch.decision() }

    pub async fn tick_and_dispatch(&mut self, now_ms: u64, budget_for: impl Fn(ActorId, semio_framework_plugin_host::shard::grant::ShardResourceBudget) -> semio_framework_plugin_host::shard::grant::ShardResourceBudget) -> Result<Decision, semio_framework_actor::pack::PackError> {
        if self.dispatch.decision().is_none() {
            for shard in &self.shards {
                if let Some((_, instance)) = shard.take_unclaimed_registration() {
                    self.guest_runtime.drop_instance(instance).await;
                }
            }
            let mut decision = Some(self.kernel.tick(now_ms).await);
            self.dispatch.admit(&mut decision)?;
        }
        if let Some(error) = self.dispatch.refusal() { return Err(*error); }
        while let Some(grant) = self.dispatch.next() {
            let Some(shard) = self.shards.get(grant.shard.0 as usize) else {
                return Err(self.dispatch.refuse(semio_framework_actor::pack::PackError::InvalidLifecycle("original grant shard is absent")));
            };
            let lane = grant.envelopes.first().map_or(Lane::Maintenance, |envelope| envelope.lane);
            let budget = budget_for(grant.actor, semio_framework_plugin_host::shard::grant::ShardResourceBudget::from_original(&grant.budget));
            if let Err(error) = ShardFrame::pack_encode_grant(grant, budget, &mut self.dispatch_bytes).await {
                return Err(self.dispatch.refuse(error));
            }
            shard.send_frame(std::mem::take(&mut self.dispatch_bytes), lane).await;
            self.dispatch.published();
        }
        self.dispatch.finish().ok_or(semio_framework_actor::pack::PackError::InvalidLifecycle("original dispatch decision is absent"))
    }

    /// ✂️ Mirrors `activate`: sends a `ShardFrame::Unregister` to the actor's own pinned shard.
    /// `Kernel` itself has no actor-retirement method (`activate`/`submit`/`tick`/`complete`/
    /// `suspend`/`resume`/`request_exclusive`/`commit_frame` is its whole façade) — this only retires
    /// the SHARD-side `GuestInstance`; a stale `Kernel`-level registry entry for a destroyed actor is
    /// a pre-existing gap this file did not introduce and does not close.
    pub async fn unregister(&mut self, actor: ActorId) -> Result<(), semio_framework_actor::pack::PackError> {
        let Some(record) = self.kernel.actor_record(actor).await else { return Ok(()) };
        let Some(shard) = self.shards.get(record.shard.0 as usize) else { return Ok(()) };
        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes).await?;
        shard.send_frame(bytes, Lane::Maintenance).await;
        Ok(())
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

    /// 🧯️ Transfers the first failure a shard drive retained with no outcome to carry it (a replay
    /// seed that failed between turns, a malformed frame), so a host fault names its real cause.
    pub fn take_shard_failure(&self) -> Option<semio_framework_plugin_host::PluginHostError> {
        self.shards.iter().find_map(|shard| shard.take_failure())
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
