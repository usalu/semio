//! 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the native
//! kernel-activation facade `semio-framework-os-run`'s `WasmtimeNodeHost` drives instead of minting
//! [`semio_framework_actor::ActorId`]s ad hoc (`RuntimeActorId::new(0, 0, counter, 0)`) and calling
//! [`GuestRuntime::instantiate`] directly — the exact bypass this packet's brief names: "🏃️run
//! bypasses the microkernel entirely: it constructs a runtime directly ... and mints its own
//! `RuntimeActorId`s", which made "33/33 native smoke" nearly meaningless as evidence (it exercised a
//! code path the product does not use).
//!
//! Lives HERE (this product's own host crate, which `semio-framework-os-run` already depends on) —
//! not in `🎯️targets/🧊️wgpu`'s own `ParallelRuntime` (`🎠️runtime.rs`, the pattern this mirrors almost
//! line-for-line) — because that crate's dependency stack (wgpu/vello/winit/image/resvg/rfd) is
//! wildly inappropriate for a headless CLI or this host crate; pulling it in just to reuse one struct
//! would itself be the "external implementation detail leaking into an unrelated consumer" CLAUDE.md's
//! own interface rule forbids. `ParallelRuntime`'s own code is NOT edited by this file
//! (`🎯️targets/🧊️wgpu/**` is outside this packet's boundary too); this is a **parallel implementation
//! of the same proven pattern**: [`semio_framework_actor::Kernel::activate`] mints the `ActorId` and
//! pins a shard, [`GuestRuntime::instantiate`] builds the guest instance, [`ShardExecutor::register`]
//! hands it to the pinned shard — turns are then genuinely DISPATCHED BY THE KERNEL:
//! `Kernel::submit` → `Kernel::tick` → per-shard `ShardFrame::Grant` → `ShardOutcome` →
//! `Kernel::complete`, never a direct in-process call.
//!
//! 🚧️ Honest gap, named rather than papered over: a real long-term architecture has exactly ONE such
//! facade, not two parallel copies (`ParallelRuntime` and this one). `ParallelRuntime` already lives
//! beside every type it is built from (`ShardExecutor`, `shard::*`, `GuestRuntime`) inside
//! `semio-framework-plugin-host` — that crate, not this product crate, is `ParallelRuntime`'s natural
//! home, and a follow-up packet should relocate it there so `run`, this host, and the wgpu target all
//! share ONE literal type. Left as a named gap, not attempted here.
//!
//! MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1c, one-pool-worker-runtime): this facade used to own K
//! `ShardExecutor` THREADS plus K `semio-os-host-kernel-shard-forward-*` outcome-forwarder threads
//! polling `ThreadTransport::recv_deadline` every 250ms. Both kinds of thread are gone.
//! [`ShardExecutor`] is now a logical affinity unit scheduled onto one shared, process-wide
//! `semio_framework_async::WorkerPool` (`ProcessKind::InteractiveNative` — this host boundary shares
//! the process contract with plugin and renderer subsystems, so reserving the UI core cannot depend
//! on which subsystem reaches the singleton first); turn outcomes flow back via
//! [`semio_framework_plugin_host::shard::executor::OutcomeSink`], pushed directly by whichever pool
//! worker executed the turn — "completion notification through the pool," never a polled channel.

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
}

impl NativeKernelRuntime {
    /// ▶️ Acquires the interactive host process's one [`WorkerPool`] (`ProcessKind::InteractiveNative`),
    /// `shard_count.max(1)` real [`ShardExecutor`]s sharing it, and one [`Kernel`]. No threads are
    /// spawned by this constructor — every [`ShardExecutor`] is pool-scheduled, only actually running
    /// a job once its first `ShardFrame` arrives via [`Self::activate`]/[`Self::tick_and_dispatch`].
    pub async fn new(guest_runtime: Arc<GuestRuntimes>, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> Self {
        let shard_count = shard_count.max(1);
        let kernel = Kernel::new(ShardKind::Native, shard_count, exclusive_reserve, grants_per_tick).await;
        let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        let pool = Arc::new(semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores)));
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
            None => Err(format!("NativeKernelRuntime::activate: Kernel::activate assigned shard {shard_index} but only {} shards were spawned", self.shards.len())),
        }
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
    /// own doc). Same contract and the same `budget_for` load-bearing note as `ParallelRuntime::
    /// tick_and_dispatch`'s own doc (grant.budget is NOT what gets dispatched; the caller's own
    /// per-lane ceiling is).
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
    pub async fn unregister(&mut self, actor: ActorId) {
        let Some(record) = self.kernel.actor_record(actor).await else { return };
        let Some(shard) = self.shards.get(record.shard.0 as usize) else { return };
        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes).await;
        shard.send_frame(bytes, Lane::Maintenance).await;
    }

    /// 🌉️ `to_actor_turn_result` + `Kernel::complete` — identical bridge to `ParallelRuntime::
    /// complete`. This crate has no clock of its own by design; callers pass host-measured
    /// `wall_us`/`memory_bytes`.
    pub async fn complete(&mut self, actor: ActorId, result: &semio_framework::kernel::TurnResult, wall_us: u64, memory_bytes: u64, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        let actor_result = to_actor_turn_result(result, wall_us, memory_bytes).await;
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
/// ⚖️ `semio_framework::kernel::Budget` (what `WasmtimeNodeHost`'s own per-node turn budget speaks) →
/// `semio_framework_actor::Budget` (what a `ShardFrame::Grant` carries). Identical helper to the
/// wgpu target's own private `actor_budget_from_turn_budget` (`🎯️targets/🧊️wgpu/📦️glue.rs`) — kept
/// here, `pub`, so this facade is genuinely usable end to end without a caller reaching into another
/// crate for one five-line function. `memory_bytes`/`ui_nodes`/`mailbox_len` have no source field on
/// the kernel-`Budget` side; defaulted from `lane` via `lane_defaults::budget_for`, same documented
/// gap shape as the wgpu target's own copy.
pub async fn actor_budget_from_turn_budget(budget: TurnBudget, lane: Lane) -> semio_framework_actor::Budget {
    let base = semio_framework_actor::lane_defaults::budget_for(lane);
    semio_framework_actor::Budget { fuel: budget.fuel, wall_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, ..base }
}
//#endregion 🔖️BudgetBridge
