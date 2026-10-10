//! 🎠️ Headless native Kernel and pooled shard coordination.
//! Guest activation ownership is shared with the WGPU host through plugin-host activation.

#![cfg(not(target_arch = "wasm32"))]

use semio_framework::kernel::{BrokerCapabilityGrant, Budget as TurnBudget};
use semio_framework_actor::{ActivationEvent, ActorId, ActorKind, Backpressure, Decision, Envelope, FailureEscalation, Kernel, KernelError, Lane, PackageId, ShardKind, WindowId};
use semio_framework_async::{ProcessKind, WorkerPoolConfig};
use semio_framework_plugin_host::shard::executor::{OutcomeSink, ShardExecutor};
use semio_framework_plugin_host::shard::{to_actor_turn_result, ShardFrame, ShardOutcome};
use semio_framework_plugin_host::{CompiledHandle, GuestRuntime, GuestRuntimes};
use std::sync::Arc;
use std::time::Duration;

/// 🎠️ Owns one [`Kernel`] and K real [`ShardExecutor`]s, all scheduled onto one shared
/// [`WorkerPool`] — see the module doc for the full mechanism. `run`'s own use is sequential and
/// single-actor-at-a-time (`SpaceRunner::compute_node`'s own doc: "never issues a second `exchange`
/// for the same `node` handle before the first one's future resolves"), so its caller constructs this
/// with `shard_count: 1` — the type itself stays general, exactly like `ParallelRuntime`, since a
/// future caller (or a relocated shared copy) may want more.
pub struct NativeKernelRuntime {
    kernel: Kernel,
    guest_runtime: Arc<GuestRuntimes>,
    shards: Vec<Arc<ShardExecutor>>,
    outcomes: Arc<OutcomeSink>,
    dispatch: semio_framework_plugin_host::shard::grant::OriginalShardDispatch,
    dispatch_bytes: Vec<u8>,
}

impl NativeKernelRuntime {
    /// ▶️ Acquires the interactive host process's one [`WorkerPool`] (`ProcessKind::InteractiveNative`),
    /// `shard_count.max(1)` real [`ShardExecutor`]s sharing it, and one [`Kernel`]. No threads are
    /// spawned by this constructor — every [`ShardExecutor`] is pool-scheduled, only actually running
    /// a job once its first `ShardFrame` arrives via [`Self::activate`]/[`Self::tick_and_dispatch`].
    pub async fn new(guest_runtime: Arc<GuestRuntimes>, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32, mut identity_issuers:impl FnMut(u16)->semio_framework_plugin_host::shard::OriginalShardIdentityIssuer) -> Self {
        let shard_count = shard_count.max(1);
        let kernel = Kernel::new(ShardKind::Native, shard_count, exclusive_reserve, grants_per_tick).await;
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = Arc::new(semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores)));
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

    /// ▶️ Raw `Kernel` access for callers that only need `Kernel::activate`'s ID-minting/shard-pinning
    /// bookkeeping WITHOUT handing a `GuestInstance` to a `ShardExecutor` — e.g.
    /// `WasmtimeNodeHost::load_runtime_recursive`'s per-plugin router-registration instance, whose
    /// `PluginInstanceHandle` calls `GuestRuntime::execute_turn` directly today (a pre-existing,
    /// out-of-boundary design this file does not change). Using this instead of [`Self::activate`]
    /// for that one call site is deliberate, not an oversight: `activate` below hands the instance's
    /// `wasmtime::Store` to one shard's own affinity permanently, which would break
    /// `PluginInstanceHandle`'s direct-call model.
    pub fn kernel_mut(&mut self) -> &mut Kernel {
        &mut self.kernel
    }

    pub fn shard_count(&self) -> u16 {
        self.shards.len() as u16
    }

    /// ▶️ `Kernel::activate` (mints the `ActorId`, pins it to a shard) + `GuestRuntime::instantiate`
    /// (host-side) + `ShardExecutor::register` (hands the freshly-built `GuestInstance` to the
    /// SPECIFIC executor `Kernel::activate` pinned it to — the one shard that will ever touch its
    /// `wasmtime::Store` from here on). Identical contract to `ParallelRuntime::activate`.
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

    /// ✉️ `Kernel::submit` — enqueues onto the actor's DRR mailbox; drained by the next
    /// `tick_and_dispatch`. Same non-retry contract as `ParallelRuntime::submit`.
    pub async fn submit(&mut self, envelope: &Envelope) -> Backpressure {
        self.kernel.submit(envelope).await
    }

    /// ⏱️ `Kernel::tick(now_ms)`, then dispatches every granted `TurnGrant` to its own pinned shard's
    /// `ShardExecutor::send_frame` — one `WorkerPool` job submission per shard that received at least
    /// one grant this tick, on whichever `Lane` the grant's own envelopes carry (every envelope for
    /// one actor shares a lane, fixed at scheduler registration — see `ShardExecutor::send_frame`'s
    /// own doc). Caller resource overrides preserve the exact original scheduler receipt.
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
    pub async fn unregister(&mut self, actor: ActorId) -> Result<(), semio_framework_actor::pack::PackError> {
        let Some(record) = self.kernel.actor_record(actor).await else { return Ok(()) };
        let Some(shard) = self.shards.get(record.shard.0 as usize) else { return Ok(()) };
        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes).await?;
        shard.send_frame(bytes, Lane::Maintenance).await;
        Ok(())
    }

    /// 🌉️ `to_actor_turn_result` + `Kernel::complete` — identical bridge to `ParallelRuntime::
    /// complete`. This crate has no clock of its own by design; callers pass host-measured
    /// `wall_us`/`memory_bytes`.
    pub async fn complete(&mut self, actor: ActorId, result: semio_framework::kernel::TurnResult, wall_us: u64, memory_bytes: u64, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        let actor_result = to_actor_turn_result(result, actor.0, wall_us, memory_bytes).await.map_err(|_| KernelError::InvalidTransition)?;
        self.kernel.complete(actor, &actor_result, now_ms).await
    }

    /// 🌀️ Drains every `ShardOutcome` currently buffered across every shard — never blocks. Same
    /// malformed-bytes-tolerant policy as before (an `OutcomeSink` only ever holds successfully
    /// decoded outcomes — decoding happens once, inside `ShardExecutor::run`, before this is ever
    /// reachable).
    pub fn try_recv_outcomes(&self) -> Vec<ShardOutcome> {
        self.outcomes.try_recv_all()
    }

    /// ⏳️ Blocks the calling thread until EITHER `expected` outcomes have been collected OR
    /// `timeout` elapses — identical primitive to `ParallelRuntime::wait_for_outcomes`. `run`'s own
    /// callers are already off any UI thread (a one-shot CLI), so a genuine blocking wait here is
    /// correct, not a smell — it is this crate's own thread root for the turn loop, same shape as
    /// `📦️bin.rs`'s `fn main` being the thread root for the whole run.
    pub fn wait_for_outcomes(&self, expected: usize, timeout: Duration) -> Vec<ShardOutcome> {
        self.outcomes.wait_for(expected, timeout)
    }
}

//#region 🔖️BudgetBridge
/// ⚖️ Applies host CPU and output limits while preserving the exact issued original authority.
pub fn actor_budget_from_turn_budget(budget: TurnBudget, original: semio_framework_plugin_host::shard::grant::ShardResourceBudget) -> semio_framework_plugin_host::shard::grant::ShardResourceBudget {
    semio_framework_plugin_host::shard::grant::ShardResourceBudget { fuel: budget.fuel, wall_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, ..original }
}
//#endregion 🔖️BudgetBridge
