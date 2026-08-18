//! 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop). The real, multi-shard native
//! kernel loop this ticket's DRR scheduler / failure ladder / metrics were built for, finally wired
//! all the way through natively: [`semio_framework_actor::Kernel::submit`] (honouring
//! [`semio_framework_actor::Backpressure`]) → [`semio_framework_actor::Kernel::tick`] → dispatch each
//! granted [`semio_framework_actor::TurnGrant`] to its PINNED shard's own
//! [`semio_framework_plugin_host::shard::executor::ShardExecutor`] thread (a REAL OS thread —
//! `design-runtime.md` §2's "K shards run in parallel" made true natively for the first time, not one
//! physical [`semio_framework_plugin_host::shard::ShardLoop`] behind all K shard labels) → collect
//! [`semio_framework_plugin_host::shard::ShardOutcome`]s → bridge via
//! [`semio_framework_plugin_host::shard::to_actor_turn_result`] → [`semio_framework_actor::Kernel::complete`].
//!
//! Before this packet, `📦️glue.rs`'s `kernel_runtime` module was a request-servant: ONE `ShardLoop`,
//! no `Kernel::tick`, `Kernel::complete` never called — the DRR scheduler/failure-ladder/metrics this
//! whole ticket built were inert natively. Bench budget 5 ("interactive p95 under 40 cpu actors")
//! was consequently unmeasurable: a single physical loop serialized every actor's turn regardless of
//! how many shard LABELS `Kernel::activate` handed out, so "K shards run in parallel" was a label, not
//! a mechanism — 30 samples inside a 0.1ms band is a constant, not a measurement. `ParallelRuntime`
//! is the fix: BOTH native call sites this packet's own brief names —
//! `📦️glue.rs::kernel_runtime`'s winit-driven interactive host, and `📦️glue.rs::scale_bench::Env`
//! (the harness budget 5 is measured through) — drive their actors through this ONE engine, so
//! "K real shard threads" is the identical mechanism in both, not a bench-only shortcut that would
//! make budget 5's own instrument dishonest again in a different way.
//!
//! Mounted from `📦️glue.rs` via a one-line `#[path]` (that file's own `//#region 🎠️ParallelRuntime`)
//! rather than growing `📦️glue.rs` (2700+ lines, shared with several concurrent packets) further —
//! `📌️important.md` rule 17's own lesson about half-landed collisions on that specific file.
//!
//! Native-only end to end: [`semio_framework_actor::ThreadTransport`] is `std::sync::mpsc`-backed,
//! and every OS thread this file spawns (`ShardExecutor`'s own thread plus one lightweight outcome
//! forwarder per shard, `//#region 🔀️OutcomeForwarding`'s own doc explains why the forwarder exists)
//! is real `std::thread`, so this whole module is mounted `#[cfg(not(target_arch = "wasm32"))]` —
//! see `📦️glue.rs`'s own `pub mod parallel_runtime` declaration.

use semio_framework::kernel::{BrokerCapabilityGrant, Budget as TurnBudget, TurnResult as KernelTurnResult};
use semio_framework_actor::{
    ActivationEvent, ActorId, ActorKind, Backpressure, Decision, Envelope, FailureEscalation, Kernel, KernelError, Lane, PackageId, ShardId, ShardKind, ShardTransport, ThreadTransport, WindowId,
};
use semio_framework_plugin_host::shard::executor::ShardExecutor;
use semio_framework_plugin_host::shard::{to_actor_turn_result, ShardFrame, ShardOutcome};
use semio_framework_plugin_host::{CompiledHandle, GuestRuntime};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

//#region 🔀️OutcomeForwarding
/// 🔀️ How long one shard's outcome-forwarder thread blocks on [`ThreadTransport::recv_deadline`]
/// before re-checking [`ParallelRuntime`]'s stop flag with nothing to forward — bounds SHUTDOWN
/// latency only, never DISPATCH latency: `recv_deadline` returns the instant a `ShardExecutor` sends
/// a `ShardOutcome`, so a live shard never actually waits this long. Kept generous (well past any
/// interactive budget this ticket measures) precisely so it never shows up as a floor under those
/// measurements.
const FORWARD_POLL: Duration = Duration::from_millis(250);

/// 🔀️ Why one small forwarder thread per shard exists instead of the kernel-loop thread blocking
/// directly on each shard's own `ThreadTransport::recv_deadline`: `ParallelRuntime` needs to park on
/// "the NEXT outcome from ANY of K shards", i.e. a select across K duplex channels — `std::sync::mpsc`
/// gives no such primitive, and `ThreadTransport` (a `🎭️actor` foundation type touched by several
/// concurrent packets) is out of this packet's `path_scope` to extend with one. Fan-in instead: each
/// shard's own kernel-side `ThreadTransport` end gets ONE dedicated thread that does nothing but block
/// on `recv_deadline` and forward whatever arrives into a SINGLE shared `mpsc::Receiver` the kernel
/// loop genuinely blocks on via `recv_timeout` — real multiplexed low-latency wake, not a polling
/// sweep (the packet brief's own "not a spin" requirement).
struct ShardHandle {
    executor: ShardExecutor,
    kernel_side: Arc<ThreadTransport>,
}
//#endregion 🔀️OutcomeForwarding

/// 🎠️ Owns one [`Kernel`] and K real [`ShardExecutor`] threads (each pinned 1:1 to a
/// [`semio_framework_actor::ShardId`] — `Kernel::activate`'s own `ShardTable::pin` call is the single
/// source of truth for which shard an actor lives on; this type never re-derives that assignment).
/// `activate`/`submit`/`tick_and_dispatch`/`complete` are the same façade shape `Kernel` itself
/// exposes, widened to also own the real thread/transport plumbing `Kernel`'s own purity rule
/// (`🎭️actor/🦀️component.rs`'s module doc: "no I/O, no clock... transports are injected") keeps out
/// of that crate.
pub struct ParallelRuntime {
    kernel: Kernel,
    guest_runtime: Arc<dyn GuestRuntime>,
    shards: Vec<ShardHandle>,
    outcomes_rx: mpsc::Receiver<(ShardId, Vec<u8>)>,
    forwarders: Vec<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}

impl ParallelRuntime {
    /// ▶️ Spawns `shard_count.max(1)` real `ShardExecutor` threads (plus their outcome forwarders)
    /// and one `Kernel`. `exclusive_reserve`/`grants_per_tick` pass straight through to
    /// `Kernel::new` — see that constructor's own doc for what each controls
    /// (`request_exclusive`'s reserve pool; the DRR tick's per-call grant ceiling).
    pub fn new(guest_runtime: Arc<dyn GuestRuntime>, shard_count: u16, exclusive_reserve: u16, grants_per_tick: u32) -> Self {
        let shard_count = shard_count.max(1);
        let kernel = Kernel::new(ShardKind::Thread, shard_count, exclusive_reserve, grants_per_tick);
        let (outcomes_tx, outcomes_rx) = mpsc::channel::<(ShardId, Vec<u8>)>();
        let stop = Arc::new(AtomicBool::new(false));
        let mut shards = Vec::with_capacity(shard_count as usize);
        let mut forwarders = Vec::with_capacity(shard_count as usize);
        for index in 0..shard_count {
            let (kernel_side, shard_side) = ThreadTransport::new_pair();
            let kernel_side = Arc::new(kernel_side);
            let executor = ShardExecutor::spawn(guest_runtime.clone(), shard_side, Vec::new());
            let forward_side = kernel_side.clone();
            let forward_stop = stop.clone();
            let forward_tx = outcomes_tx.clone();
            let shard_id = ShardId(index);
            let handle = std::thread::Builder::new()
                .name(format!("semio-kernel-shard-forward-{index}"))
                .spawn(move || {
                    while !forward_stop.load(Ordering::SeqCst) {
                        if let Some(bytes) = forward_side.recv_deadline(FORWARD_POLL) {
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

    pub fn kernel_mut(&mut self) -> &mut Kernel {
        &mut self.kernel
    }

    pub fn shard_count(&self) -> u16 {
        self.shards.len() as u16
    }

    /// ▶️ `Kernel::activate` (mints the `ActorId`, pins it to a shard via `ShardTable::pin`) +
    /// `GuestRuntime::instantiate` (host-side, may run on ANY thread — `GuestInstance` is `Send`,
    /// proven already by `ShardExecutor::spawn`'s own `initial` parameter moving one across a
    /// `std::thread::spawn` boundary) + `ShardExecutor::register` (hands the freshly-built
    /// `GuestInstance` to the SPECIFIC executor thread `ShardTable::pin` assigned, the one thread
    /// that will ever touch its `wasmtime::Store` from here on). `instantiate_budget` is the ceiling
    /// `GuestRuntime::instantiate` itself wants (independent of whatever `Kernel::tick` later grants
    /// per turn) — callers already compute one for their own purpose (`kernel_runtime::TURN_BUDGET`,
    /// `scale_bench::turn_budget_of`), so this takes it rather than re-deriving a third value.
    #[allow(clippy::too_many_arguments)]
    pub fn activate(
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
        let actor = self.kernel.activate(package, plugin_ordinal, kind, lane, window, event);
        let shard_index = self.kernel.actor_record(actor).map(|record| record.shard.0 as usize).unwrap_or(0);
        let instance = match self.guest_runtime.instantiate(compiled, actor, caps, instantiate_budget) {
            Ok(instance) => instance,
            Err(error) => return Err(error.to_string()),
        };
        match self.shards.get(shard_index) {
            Some(shard) => {
                shard.executor.register(actor, instance);
                Ok(actor)
            }
            None => Err(format!("ParallelRuntime::activate: ShardTable::pin assigned shard {shard_index} but only {} shards were spawned", self.shards.len())),
        }
    }

    /// ✉️ `Kernel::submit` — enqueues onto the actor's DRR mailbox; drained by the NEXT
    /// `tick_and_dispatch`. Callers must honour a non-[`Backpressure::Accept`] result the same way
    /// `Scheduler::submit`'s own doc already documents (a `Rejected`/`Dropped` must surface as a busy
    /// badge, never a silent drop) — this method does not retry or coalesce on the caller's behalf.
    pub fn submit(&mut self, envelope: &Envelope) -> Backpressure {
        self.kernel.submit(envelope)
    }

    /// ⏱️ `Kernel::tick(now_ms)`, then dispatches every granted [`semio_framework_actor::TurnGrant`]
    /// to its OWN pinned shard as a real [`ShardFrame::Grant`].
    ///
    /// 🐛️ terra-kernel-loop finding, load-bearing: the grant's OWN `budget` field is NOT used for
    /// the dispatched frame. `Kernel::activate` has no per-actor budget parameter at all — it always
    /// computes the SCHEDULED (and later throttle-scaled) budget from `lane_defaults::budget_for
    /// (lane)`, a 4-tier table with no room for a caller's own ceiling. Both native callers this
    /// packet touches carry their OWN, deliberately-higher fuel ceilings for a documented reason —
    /// `kernel_runtime::TURN_BUDGET` (50M fuel) and `scale_bench::BENCH_FUEL` (200M fuel, whose own
    /// doc comment measured ~92M fuel burned by ONE real `describe()` call in an unoptimized wasip2
    /// build) — both far above `lane_defaults`' Interactive tier (2M fuel). Dispatching `grant.budget`
    /// verbatim would silently fuel-starve nearly every real turn behind a K-shards-in-parallel fix
    /// that was never supposed to touch budget fidelity — the exact "flatten to a default" anti-
    /// pattern the packet brief's own step-0 instructions forbade one layer down (`ShardFrame::Grant`
    /// budgets "do NOT flatten to a Maintenance default"); this is that same principle one level up.
    /// `budget_for` lets the caller supply the ACTUAL per-actor ceiling to dispatch, keeping
    /// `Kernel::tick`'s real value: WHO is due and WHICH envelopes/shard, i.e. genuine DRR fairness
    /// and shard routing — while leaving `Scheduler`'s own throttle-scaling of `grant.budget` unused
    /// for now (an honest, explicitly-flagged gap, not a silent regression: throttle-driven budget
    /// scaling was ALREADY inert before this packet, since `Kernel::complete` — the only thing that
    /// ever sets a nonzero throttle factor — was never called at all).
    ///
    /// Returns the raw `Decision` so a caller can honour `wake_at` for its own park deadline.
    pub fn tick_and_dispatch(&mut self, now_ms: u64, budget_for: impl Fn(ActorId) -> semio_framework_actor::Budget) -> Decision {
        let decision = self.kernel.tick(now_ms);
        for grant in &decision.run {
            let shard_index = grant.shard.0 as usize;
            let Some(shard) = self.shards.get(shard_index) else { continue };
            let mut bytes = Vec::new();
            ShardFrame::Grant { actor: grant.actor, budget: budget_for(grant.actor), envelopes: grant.envelopes.clone() }.pack_encode(&mut bytes);
            shard.kernel_side.send(&bytes);
        }
        decision
    }

    /// ✂️ Mirrors `activate`: sends a `ShardFrame::Unregister` to the actor's own pinned shard.
    /// `Kernel` itself has no actor-retirement method (`activate`/`submit`/`tick`/`complete`/
    /// `suspend`/`resume`/`request_exclusive`/`commit_frame` is its whole façade) — this only
    /// retires the SHARD-side `GuestInstance`, the same scope the pre-existing single-`ShardLoop`
    /// `kernel_runtime::destroy_app`/`scale_bench`'s direct `env.shard.unregister` calls already had;
    /// a stale `Kernel`-level registry entry for a destroyed actor is a pre-existing gap this packet
    /// did not introduce and does not close.
    pub fn unregister(&mut self, actor: ActorId) {
        let Some(record) = self.kernel.actor_record(actor) else { return };
        let Some(shard) = self.shards.get(record.shard.0 as usize) else { return };
        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes);
        shard.kernel_side.send(&bytes);
    }

    /// 🌉️ `to_actor_turn_result` + `Kernel::complete` — the exact bridge
    /// `kernel_runtime::KernelThreadState::apply_turn_result`'s own doc comment used to flag as
    /// unreached ("bridging the two needs a real pack-encode step this packet didn't reach"). Callers
    /// pass the RAW `semio_framework::kernel::TurnResult` a `ShardOutcome::Turn` carried, plus
    /// host-measured `wall_us`/`memory_bytes` (this crate has no clock of its own by design — see
    /// `to_actor_turn_result`'s own doc).
    pub fn complete(&mut self, actor: ActorId, result: &KernelTurnResult, wall_us: u64, memory_bytes: u64, now_ms: u64) -> Result<FailureEscalation, KernelError> {
        let actor_result = to_actor_turn_result(result, wall_us, memory_bytes);
        self.kernel.complete(actor, &actor_result, now_ms)
    }

    /// 🌀️ Drains every `ShardOutcome` CURRENTLY buffered on the aggregated forwarder channel —
    /// never blocks. Malformed bytes (should not happen; every `ShardOutcome` is JSON-encoded by
    /// `ShardLoop::send_outcome`) are silently skipped rather than surfaced, matching
    /// `scale_bench::Env::drain`'s own pre-existing policy for the single-shard harness this replaces.
    pub fn try_recv_outcomes(&self) -> Vec<ShardOutcome> {
        let mut out = Vec::new();
        while let Ok((_, bytes)) = self.outcomes_rx.try_recv() {
            if let Ok(outcome) = serde_json::from_slice::<ShardOutcome>(&bytes) {
                out.push(outcome);
            }
        }
        out
    }

    /// ⏳️ Blocks on the aggregated outcome channel until EITHER `expected` outcomes have been
    /// collected OR `timeout` elapses, whichever comes first — a genuine multiplexed wait (via
    /// `mpsc::Receiver::recv_timeout` racing against a computed deadline across every call), not a
    /// polling sweep. This is the primitive both the interactive host's per-exchange wait and the
    /// scale-bench harness's own round-trip latency measurement (bench budget 5) are built on: the
    /// ELAPSED time this method actually spends IS the K-real-shard-threads dispatch latency the
    /// packet's acceptance gate measures.
    pub fn wait_for_outcomes(&self, expected: usize, timeout: Duration) -> Vec<ShardOutcome> {
        let deadline = Instant::now() + timeout;
        let mut out = Vec::with_capacity(expected);
        while out.len() < expected {
            let now = Instant::now();
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

impl Drop for ParallelRuntime {
    fn drop(&mut self) {
        // 🛑️ Unblocks every forwarder thread's `recv_deadline` immediately (rather than waiting up
        // to `FORWARD_POLL`) — `ThreadTransport::kill` makes `recv_deadline` return `None` right
        // away (see that method's own doc), so the forwarder loop's `!forward_stop.load(..)` check
        // is reached promptly instead of on the next poll timeout.
        self.stop.store(true, Ordering::SeqCst);
        for shard in &self.shards {
            shard.kernel_side.kill();
        }
        for handle in self.forwarders.drain(..) {
            let _ = handle.join();
        }
        // `self.shards`' own drop runs next, dropping each `ShardExecutor` — its `Drop` impl already
        // signals+joins its thread (`🏃️executor.rs`'s own `impl Drop for ShardExecutor`).
    }
}
