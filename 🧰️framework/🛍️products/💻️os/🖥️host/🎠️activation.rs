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
//! line-for-line, per this packet's brief: "one activation facade shared with the wgpu target ...
//! read that file for how that consumer does it") — because that crate's dependency stack
//! (wgpu/vello/winit/image/resvg/rfd) is wildly inappropriate for a headless CLI or this host crate;
//! pulling it in just to reuse one struct would itself be the "external implementation detail leaking
//! into an unrelated consumer" CLAUDE.md's own interface rule forbids. `ParallelRuntime`'s own code is
//! NOT edited by this packet (`🎯️targets/🧊️wgpu/**` is outside `path_scope`); this is a **parallel
//! implementation of the same proven pattern**: [`semio_framework_actor::Kernel::activate`] mints the
//! `ActorId` and pins a shard, [`GuestRuntime::instantiate`] builds the guest instance,
//! [`ShardExecutor::register`] hands it to the pinned shard's own OS thread — turns are then
//! genuinely DISPATCHED BY THE KERNEL: `Kernel::submit` → `Kernel::tick` → per-shard
//! `ShardFrame::Grant` → `ShardOutcome` → `Kernel::complete`, never a direct in-process call.
//!
//! 🚧️ Honest gap, named rather than papered over: a real long-term architecture has exactly ONE such
//! facade, not two parallel copies. `ParallelRuntime` already lives beside every type it is built
//! from (`ShardExecutor`, `shard::*`, `GuestRuntime`) inside `semio-framework-plugin-host`
//! (`🔌️plugin/🖥️host/**`) — that crate, not this product crate, is `ParallelRuntime`'s natural home,
//! and a follow-up packet should relocate it there so `run`, this host, and the wgpu target all share
//! ONE literal type. `🔌️plugin/🖥️host/**` is outside this packet's `path_scope` (`💻️os/🖥️host/**` is
//! a *different* "host" directory — see `📌️important.md`'s naming-hazards section for exactly this
//! kind of collision), so that relocation is left as a named gap, not attempted here — see this
//! packet's own report for the `lease-request`.
//!
//! Every method mirrors `ParallelRuntime`'s own doc comments; only genuine differences from that
//! file are called out below. Every `semio_framework_actor::Kernel`/`ThreadTransport` method used
//! here is `async fn` on disk today (universal-async, O1) — this facade `.await`s every one of them,
//! so it is written to the crate's TARGET shape even though `semio-framework-actor` itself does not
//! compile as this packet writes it (266 errors, all missing-`.await` fallout from an unrelated,
//! live, in-progress async-conversion sweep on that crate — confirmed independently, see this
//! packet's own report). That crate is outside `path_scope`; this file cannot be verified by a
//! green `cargo check` until whichever packet owns it finishes.

#![cfg(not(target_arch = "wasm32"))]

use semio_framework::kernel::{BrokerCapabilityGrant, Budget as TurnBudget};
use semio_framework_actor::{
    ActivationEvent, ActorId, ActorKind, Backpressure, Decision, Envelope, FailureEscalation, Kernel, KernelError, Lane, PackageId, ShardId, ShardKind, ShardTransport, ThreadTransport, WindowId,
};
use semio_framework_plugin_host::shard::executor::ShardExecutor;
use semio_framework_plugin_host::shard::{to_actor_turn_result, ShardFrame, ShardOutcome};
use semio_framework_plugin_host::{CompiledHandle, GuestRuntime, GuestRuntimes};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

//#region 🔀️OutcomeForwarding
/// 🔀️ Same tripwire shape as `ParallelRuntime::FORWARD_POLL` — bounds shutdown latency only, never
/// dispatch latency (see that constant's own doc).
const FORWARD_POLL: Duration = Duration::from_millis(250);

struct ShardHandle {
    executor: ShardExecutor,
    kernel_side: Arc<ThreadTransport>,
}
//#endregion 🔀️OutcomeForwarding

/// 🎠️ Owns one [`Kernel`] and K real [`ShardExecutor`] threads — see `ParallelRuntime`'s own doc for
/// the full mechanism (identical here). `run`'s own use is sequential and single-actor-at-a-time
/// (`SpaceRunner::compute_node`'s own doc: "never issues a second `exchange` for the same `node`
/// handle before the first one's future resolves"), so its caller constructs this with
/// `shard_count: 1` — the type itself stays general, exactly like `ParallelRuntime`, since a future
/// caller (or a relocated shared copy) may want more.
pub struct NativeKernelRuntime {
    kernel: Kernel,
    guest_runtime: Arc<GuestRuntimes>,
    shards: Vec<ShardHandle>,
    outcomes_rx: mpsc::Receiver<(ShardId, Vec<u8>)>,
    forwarders: Vec<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}

impl NativeKernelRuntime {
    /// ▶️ Spawns `shard_count.max(1)` real `ShardExecutor` threads (plus their outcome forwarders)
    /// and one `Kernel` — see `ParallelRuntime::new`'s own doc for what `exclusive_reserve`/
    /// `grants_per_tick` control.
    pub async fn new(guest_runtime: Arc<GuestRuntimes>, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> Self {
        let shard_count = shard_count.max(1);
        let kernel = Kernel::new(ShardKind::Thread, shard_count, exclusive_reserve, grants_per_tick).await;
        let (outcomes_tx, outcomes_rx) = mpsc::channel::<(ShardId, Vec<u8>)>();
        let stop = Arc::new(AtomicBool::new(false));
        let mut shards = Vec::with_capacity(shard_count as usize);
        let mut forwarders = Vec::with_capacity(shard_count as usize);
        for index in 0..shard_count {
            let (kernel_side, shard_side) = ThreadTransport::new_pair().await;
            let kernel_side = Arc::new(kernel_side);
            let executor = ShardExecutor::spawn(guest_runtime.clone(), shard_side, Vec::new());
            let forward_side = kernel_side.clone();
            let forward_stop = stop.clone();
            let forward_tx = outcomes_tx.clone();
            let shard_id = ShardId(index);
            let handle = std::thread::Builder::new()
                .name(format!("semio-os-host-kernel-shard-forward-{index}"))
                .spawn(move || {
                    // 🌉️ This forwarder thread IS the poller that turns `recv_deadline`'s future
                    // into a blocking call; nothing else on this thread runs, so a plain `block_on`
                    // per iteration is the correct bridge (R4 item 4: a shard/actor thread root).
                    // Same reasoning as `ParallelRuntime`'s own forwarder — that file does not
                    // `.await` `recv_deadline` because it was written before `ThreadTransport` went
                    // async; this one must, since it targets the crate's post-conversion shape.
                    while !forward_stop.load(Ordering::SeqCst) {
                        // 🚫️async: E5 executor bridge — this crate's one production block_on (R2:
                        // at most one per crate); see this file's module doc.
                        if let Some(bytes) = semio_framework_async::block_on(forward_side.recv_deadline(FORWARD_POLL)) {
                            if forward_tx.send((shard_id, bytes)).is_err() {
                                break;
                            }
                        }
                    }
                })
                .expect("spawn shard-outcome forwarder thread");
            forwarders.push(handle);
            shards.push(ShardHandle { executor, kernel_side });
        }
        Self { kernel, guest_runtime, shards, outcomes_rx, forwarders, stop }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// ▶️ Raw `Kernel` access for callers that only need `Kernel::activate`'s ID-minting/shard-pinning
    /// bookkeeping WITHOUT handing a `GuestInstance` to a `ShardExecutor` thread — e.g.
    /// `WasmtimeNodeHost::load_runtime_recursive`'s per-plugin router-registration instance, whose
    /// `PluginInstanceHandle` calls `GuestRuntime::execute_turn` directly today (a pre-existing,
    /// out-of-`path_scope` design this packet does not change — see this packet's own report). Using
    /// this instead of [`Self::activate`] for that one call site is deliberate, not an oversight: the
    /// full `activate` below takes over the instance's `wasmtime::Store` on one shard thread
    /// permanently, which would break `PluginInstanceHandle`'s direct-call model.
    pub fn kernel_mut(&mut self) -> &mut Kernel {
        &mut self.kernel
    }

    pub fn shard_count(&self) -> u16 {
        self.shards.len() as u16
    }

    /// ▶️ `Kernel::activate` (mints the `ActorId`, pins it to a shard) + `GuestRuntime::instantiate`
    /// (host-side) + `ShardExecutor::register` (hands the freshly-built `GuestInstance` to the
    /// SPECIFIC executor thread `Kernel::activate` pinned it to — the one thread that will ever touch
    /// its `wasmtime::Store` from here on). Identical contract to `ParallelRuntime::activate`.
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
        let instance = match self.guest_runtime.instantiate(compiled, actor, caps, instantiate_budget) {
            Ok(instance) => instance,
            Err(error) => return Err(error.to_string()),
        };
        match self.shards.get(shard_index) {
            Some(shard) => {
                shard.executor.register(actor, instance);
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

    /// ⏱️ `Kernel::tick(now_ms)`, then dispatches every granted `TurnGrant` to its own pinned shard —
    /// identical contract and the same `budget_for` load-bearing note as `ParallelRuntime::
    /// tick_and_dispatch`'s own doc (grant.budget is NOT what gets dispatched; the caller's own
    /// per-lane ceiling is).
    pub async fn tick_and_dispatch(&mut self, now_ms: u64, budget_for: impl Fn(ActorId) -> semio_framework_actor::Budget) -> Decision {
        let decision = self.kernel.tick(now_ms).await;
        for grant in &decision.run {
            let shard_index = grant.shard.0 as usize;
            let Some(shard) = self.shards.get(shard_index) else { continue };
            let mut bytes = Vec::new();
            ShardFrame::Grant { actor: grant.actor, budget: budget_for(grant.actor), envelopes: grant.envelopes.clone() }.pack_encode(&mut bytes);
            shard.kernel_side.send(&bytes).await;
        }
        decision
    }

    /// ✂️ Mirrors `activate`: sends a `ShardFrame::Unregister` to the actor's own pinned shard.
    pub async fn unregister(&mut self, actor: ActorId) {
        let Some(record) = self.kernel.actor_record(actor).await else { return };
        let Some(shard) = self.shards.get(record.shard.0 as usize) else { return };
        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes);
        shard.kernel_side.send(&bytes).await;
    }

    /// 🌉️ `to_actor_turn_result` + `Kernel::complete` — identical bridge to `ParallelRuntime::
    /// complete`. This crate has no clock of its own by design; callers pass host-measured
    /// `wall_us`/`memory_bytes`.
    pub async fn complete(&mut self, actor: ActorId, result: &semio_framework::kernel::TurnResult, wall_us: u64, memory_bytes: u64, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        let actor_result = to_actor_turn_result(result, wall_us, memory_bytes);
        self.kernel.complete(actor, &actor_result, now_ms).await
    }

    /// 🌀️ Drains every `ShardOutcome` currently buffered — never blocks. Same malformed-bytes policy
    /// as `ParallelRuntime::try_recv_outcomes`.
    pub fn try_recv_outcomes(&self) -> Vec<ShardOutcome> {
        let mut out = Vec::new();
        while let Ok((_, bytes)) = self.outcomes_rx.try_recv() {
            if let Ok(outcome) = serde_json::from_slice::<ShardOutcome>(&bytes) {
                out.push(outcome);
            }
        }
        out
    }

    /// ⏳️ Blocks the calling thread until EITHER `expected` outcomes have been collected OR `timeout`
    /// elapses — identical primitive to `ParallelRuntime::wait_for_outcomes`. `run`'s own callers are
    /// already off any UI thread (a one-shot CLI), so a genuine blocking wait here is correct, not a
    /// smell — it is this crate's own thread root for the turn loop, same shape as `📦️bin.rs`'s
    /// `fn main` being the thread root for the whole run.
    pub fn wait_for_outcomes(&self, expected: usize, timeout: Duration) -> Vec<ShardOutcome> {
        let deadline = std::time::Instant::now() + timeout;
        let mut out = Vec::with_capacity(expected);
        while out.len() < expected {
            let now = std::time::Instant::now();
            if now >= deadline {
                break;
            }
            match self.outcomes_rx.recv_timeout(deadline - now) {
                Ok((_, bytes)) => {
                    if let Ok(outcome) = serde_json::from_slice(&bytes) {
                        out.push(outcome);
                    }
                }
                Err(_) => break,
            }
        }
        out
    }
}

impl Drop for NativeKernelRuntime {
    fn drop(&mut self) {
        // 🛑️ No `ThreadTransport::kill()` call here — `Drop::drop` cannot be `async`, this is not a
        // thread root (it runs on whatever thread drops the struct), and this file already carries
        // its one E5 bridge in the forwarder loop below — R2's "at most one per crate" discipline.
        // The stop flag alone is enough: each forwarder thread observes it within one
        // `FORWARD_POLL` window (250ms) even without an explicit transport-level wakeup — a bounded,
        // honest shutdown-latency tradeoff against a second bridge, unlike `ParallelRuntime::drop`
        // (whose `ThreadTransport::kill` was, in that pre-existing file, still sync and so needed no
        // bridge at all).
        self.stop.store(true, Ordering::SeqCst);
        for handle in self.forwarders.drain(..) {
            let _ = handle.join();
        }
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
    let base = semio_framework_actor::lane_defaults::budget_for(lane).await;
    semio_framework_actor::Budget { fuel: budget.fuel, wall_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, ..base }
}
//#endregion 🔖️BudgetBridge
