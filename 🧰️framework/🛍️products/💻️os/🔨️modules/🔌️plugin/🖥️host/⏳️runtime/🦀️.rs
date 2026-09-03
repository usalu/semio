//! ⏳️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-runtime-rewrite). `WasmtimeAsyncRuntime` —
//! the execution backend for a REAL `world actor` component under `component-model-async`: one
//! root `tokio::spawn`ed task per actor, owning a `Store<AsyncActorHostState>` for the actor's
//! whole lifetime and driven by a command channel, never re-instantiating between calls (the
//! "pooled" half of this ticket's name) — `imports.rs`'s `AsyncActorHostState`/host-async import
//! layer (finished, accepted; this file drives it, never reimplements it).
//!
//! 🔁️ **This is the SECOND draft of this file, not a patch on the first.** The original
//! (`terra-async-runtime`, `📓️terra-async-runtime-report.md`) was written against WIT names that
//! `B1 world-collapse` then deleted: `interface runner`'s `run: async func(events: stream<event>)`
//! and `world actor-async` are both GONE. `world actor` now exports `reactor::poll` (`list<event>`,
//! not a stream — request/response, exactly like the SYNC world's `execute_turn`) plus
//! `jobs::{start-job,step-job,cancel-job}` and `checkpoint::{checkpoint,restore}`, all plain
//! `async func`, called through `Store::run_concurrent` + `Accessor` — `../⏳️⚙️runtime/🦀️.rs`'s
//! `WasmtimeRuntime` demonstrates the exact working call shape for every one of them, and this file
//! reuses that shape rather than re-deriving it. See `📓️terra-world-collapse-report.md` (what the
//! host side now does) and `📓️terra-async-runtime-report.md`'s `## harness` (six results proven
//! against real wasmtime 47.0.3 — the cancellation-by-dropping-the-Store-inside-the-task rule (D/E)
//! and the concurrent-`accessor.spawn`-on-one-instance rule (F) both still hold and are reused
//! below verbatim; the harness's A/B results, about a STREAM export, do not apply to this world at
//! all any more and are not reused).
//!
//! 🧬️ **Consequence of `poll` taking `list<event>` instead of `stream<event>`: there is no
//! continuous "grant" to model.** The old draft's `GrantWindow`/`GrantedEventProducer` (a
//! `StreamProducer` parking on an exhausted delivery budget) and `synthesize_turn_result` (host-side
//! turn synthesis because `runner::run` never itself returned a `turn-result`) are BOTH deleted
//! outright, not adapted: `reactor::poll` returns a real `wit_reactor::TurnResult` — with
//! `status`/`next_wake`/`fuel_used` already computed by the guest — on every call, exactly the value
//! `WasmtimeRuntime::execute_turn` already unwraps. This file's `Poll` command is a straight
//! async analogue of `execute_turn`, not an invention.
//!
//! 🧩️ **What IS kept, byte-for-byte where the shape survived**: [`build_async_engine`]/
//! [`AsyncEngineHandle`] (unaffected by the collapse — engine/linker construction never named the
//! deleted world), [`DeadlineCell`]/[`install_epoch_budget`] (S1c/harness-test-C's Yield/Interrupt
//! epoch callback, still the only wall-clock lever this file installs), the
//! [`AsyncActorTask`] command-channel skeleton with the proven "construct the `Store` INSIDE the
//! `tokio::spawn`ed task body" rule (harness D/E — `AsyncActorTask::cancel` still `abort()`s the
//! `JoinHandle` and awaits it so the `Store` drops with the future, not merely the future alone),
//! and the per-export `AccessorTask` + oneshot pattern (harness F) — now used for EVERY command,
//! not only checkpoint/jobs, because there is no more long-lived `call_run` future for the other
//! commands to run concurrently alongside. The whole actor lifetime lives inside ONE
//! `store.run_concurrent(...)` call: the outer closure is a plain command-receive loop that never
//! itself awaits a WIT call — every command is handed to `accessor.spawn` as its own `AccessorTask`,
//! so two commands against the SAME `Store` (e.g. a `Checkpoint` answered while a slow `StepJob` is
//! still in flight) genuinely run concurrently, reproducing harness test F's proof against the real
//! collapsed world rather than merely asserting it still holds.
//!
//! 🧪️ **Honest status**: this file now compiles in-tree (see the packet report's acceptance
//! section for the exact command run and its exit code) — the two blockers the previous draft
//! carried are both gone: `host_async_bindings` (this crate's `actor_bindings` module) has been
//! `pub(crate)` since `B1 world-collapse` landed, and `checkpoint`/`jobs` never grew `-async`
//! suffixes (they went async IN PLACE — `bindings.semio_framework_checkpoint()`/
//! `.semio_framework_jobs()`, unsuffixed, exactly as `../⏳️⚙️runtime/🦀️.rs` already calls them). It is
//! still UNCALLED — nothing in `GuestRuntimes` constructs an `AsyncActorTask` yet (see
//! `GuestRuntimes`'s own doc comment: "a later packet adds `AsyncActor(...)` here ... do not mount
//! `⏳️runtime.rs` from this packet" — a PRIOR packet's note about ITSELF, not a standing
//! prohibition; wiring `AsyncActorTask` into that enum, and into a real DRR-style caller, is the
//! next packet's job, exactly as `terra-async-runtime-report.md`'s own `## honest gaps` already
//! said before this rewrite).

use crate::actor_bindings::{
    self,
    exports::semio::framework::{jobs as wit_jobs, reactor as wit_reactor},
    semio::framework::events as wit_events,
};
use crate::imports::AsyncActorHostState;
use crate::{JobBudget, JobStep, PluginHostError, SharedEngineConfig};
use semio_framework::kernel::{Budget, Effect, Event, TurnResult as KernelTurnResult};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use wasmtime::component::{Accessor, AccessorTask, HasSelf, Linker};
use wasmtime::{AsContextMut, Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig, Store, StoreContextMut, UpdateDeadline};

//#region 🐎️AsyncEngineHandle
/// 🧩️ Mirrors `🖥️host/🦀️.rs`'s `CORE_INSTANCES_PER_COMPONENT`/`MEMORIES_PER_COMPONENT`/
/// `TABLES_PER_COMPONENT` ratios verbatim. These are module-private `const`s in `component.rs`, but
/// `runtime` is a Rust DESCENDANT of `component` (`#[path] pub mod runtime;` is declared inside
/// `component.rs`'s own body, the same nesting `imports`/`effects`/`shard` already use), so private
/// items there — these consts included — are visible here via `super::`; verified by compiling this
/// file (see the packet report), not assumed. Referenced via `super::` rather than duplicated.
/// 🐎️ ONE async-capable `Engine` per process, distinct from `build_shared_engine`'s (that one never
/// sets `wasm_component_model_async`/`concurrency_support` — it backs `world actor`'s SYNC-style
/// call sites, i.e. `WasmtimeRuntime`, which still drives every export through `run_concurrent` too
/// post-collapse but never parks a guest future across host turns the way this pooled runtime does).
/// `wasm_component_model_async(true)` is engine-wide and makes every Store built from this engine
/// share the same async ABI requirement `build_shared_engine` already turned on crate-wide (B1
/// world-collapse made this NOT optional — see that fn's own doc) — this second engine exists so a
/// pooled actor's `Store` can be tuned/pooled independently of `WasmtimeRuntime`'s per-call one,
/// not because the Config knobs differ.
pub async fn build_async_engine(cfg: SharedEngineConfig) -> Result<(Engine, bool), PluginHostError> {
    let build = |pooling: bool| -> wasmtime::Result<Engine> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.wasm_component_model_async(true);
        config.consume_fuel(true);
        config.epoch_interruption(true);
        if pooling {
            let mut pooling_cfg = PoolingAllocationConfig::default();
            pooling_cfg.total_component_instances(cfg.total_component_instances);
            pooling_cfg.total_core_instances(cfg.total_component_instances * super::CORE_INSTANCES_PER_COMPONENT);
            pooling_cfg.total_memories(cfg.total_component_instances * super::MEMORIES_PER_COMPONENT);
            pooling_cfg.total_tables(cfg.total_component_instances * super::TABLES_PER_COMPONENT);
            pooling_cfg.total_gc_heaps(cfg.total_component_instances * super::MEMORIES_PER_COMPONENT);
            pooling_cfg.max_memory_size(cfg.max_memory_bytes);
            pooling_cfg.linear_memory_keep_resident(cfg.linear_memory_keep_resident_bytes);
            config.allocation_strategy(InstanceAllocationStrategy::Pooling(pooling_cfg));
        } else {
            config.allocation_strategy(InstanceAllocationStrategy::OnDemand);
        }
        Engine::new(&config)
    };
    if !cfg.force_on_demand {
        if let Ok(engine) = build(true) {
            return Ok((engine, true));
        }
    }
    let engine = build(false).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
    Ok((engine, false))
}

/// 🐎️ One async engine + its epoch ticker + a `Linker` with WASI-async and `world actor`'s whole
/// import surface (`pure` + `host-async`, ONE call — `B1 world-collapse` made `Actor::add_to_linker`
/// define both) already wired — built ONCE per process, `Arc`-shared into every
/// [`AsyncActorTask::spawn`] call. Reuses `crate::EpochTicker` (already `pub` in `../⏳️⚙️runtime/🦀️.rs`)
/// rather than duplicating a THIRD 1ms ticker thread.
pub struct AsyncEngineHandle {
    pub engine: Engine,
    _epoch_ticker: crate::EpochTicker,
    /// 🔗️ `Arc`, not a bare `Linker` — [`AsyncActorTask::spawn`] moves a clone of this handle into a
    /// `tokio::spawn`ed `'static` task body (harness tests D/E: the Store, and everything it needs
    /// to instantiate against, must be OWNED by that task, never borrowed from an outer scope, or
    /// cancellation-on-drop silently fails to actually cancel in-flight guest work).
    linker: Arc<Linker<AsyncActorHostState>>,
}

impl AsyncEngineHandle {
    pub async fn new(cfg: SharedEngineConfig) -> Result<Self, PluginHostError> {
        let (engine, _pooling_active) = build_async_engine(cfg).await?;
        let epoch_ticker = crate::EpochTicker::start(&engine, &crate::plugin_host_worker_pool());
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        // 🧬️ B1 world-collapse already landed `pub(crate) mod actor_bindings` — no lease needed
        // (the previous draft's blocking lease request is resolved; see this file's module doc).
        actor_bindings::Actor::add_to_linker::<AsyncActorHostState, HasSelf<AsyncActorHostState>>(&mut linker, |state: &mut AsyncActorHostState| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(Self { engine, _epoch_ticker: epoch_ticker, linker: Arc::new(linker) })
    }
}
//#endregion 🐎️AsyncEngineHandle

//#region 🪧️Type-only Host marker impls
/// 🪧️ `Actor::add_to_linker::<AsyncActorHostState, _>` (above) demands a `Host` impl for EVERY
/// interface `wit-parser` surfaces as an import of `world actor` — including the seven present only
/// because an exported function's signature references their types (`types`/`capabilities`/
/// `effects`/`events`/`ui`/`byte-page`/`instance-lifetime`, the last two via `reactor::turn-result`'s
/// `command-ingress`/`lifecycle-receipt`/`ui-patch-receipt` fields), which declare no functions at
/// all. `component.rs`'s `ActorHostState` already carries this exact set of empty impls
/// (`WasmtimeRuntime::new`'s own linker call needed them first); `AsyncActorHostState`
/// (`imports.rs`) never needed them until THIS file became the first caller to link the whole world
/// against it. Implemented HERE, not in `imports.rs`: Rust's orphan rule only cares about crate
/// boundaries, and both the trait (bindgen-generated in `component.rs`, `pub(crate)`) and the type
/// (`imports.rs`, `pub`) are local to this crate, so an empty marker impl may live in any module of
/// it — adding it here needs no edit to `imports.rs` and no lease. `ui`'s empty marker resource
/// additionally forces a `HostSurface::drop` that can never actually be called (no host function
/// ever hands the guest a `Surface` handle) — same as `component.rs`'s own
/// `impl wit_ui::HostSurface for ActorHostState`.
impl actor_bindings::semio::framework::types::Host for AsyncActorHostState {}
impl actor_bindings::semio::framework::capabilities::Host for AsyncActorHostState {}
impl actor_bindings::semio::framework::effects::Host for AsyncActorHostState {}
impl actor_bindings::semio::framework::events::Host for AsyncActorHostState {}
impl actor_bindings::semio::framework::ui::Host for AsyncActorHostState {}
impl actor_bindings::semio::framework::byte_page::Host for AsyncActorHostState {}
impl actor_bindings::semio::framework::instance_lifetime::Host for AsyncActorHostState {}
impl actor_bindings::semio::framework::ui::HostSurface for AsyncActorHostState {
    // 🚫️async: E1 — `bindgen!` fixes this signature (the resource-destructor hook wasmtime calls
    // when a guest handle goes out of scope); not chosen by this repo. See R9/R2 E1, and
    // `component.rs`'s identical tag on `impl wit_ui::HostSurface for ActorHostState`.
    fn drop(&mut self, _rep: wasmtime::component::Resource<actor_bindings::semio::framework::ui::Surface>) -> wasmtime::Result<()> {
        Ok(())
    }
}
//#endregion 🪧️Type-only Host marker impls

//#region ⏱️Budgets — per-call fuel + epoch Yield/Interrupt
/// ⏱️ terra-async-runtime harness test C, unchanged: `Yield(1)` while `Instant::now()` is still
/// before the current call's deadline (cooperative — S1c proved this genuinely preempts pure
/// CPU-bound guest code across separate `Store`s, no host-import confound), `Interrupt` once it has
/// passed (hard trap — becomes an `Err` string at the calling `AccessorTask`'s reply). `Mutex<Instant>`
/// rather than an `AtomicU64` epoch count: `set_epoch_deadline` takes a DELTA, not an absolute epoch
/// (`u64::MAX` wraps `current_epoch + delta` and traps the store on its very first call), so this
/// file never encodes "no deadline" as a sentinel — [`AsyncActorTask::spawn`] REQUIRES an initial
/// [`Budget`] up front, and every command that carries its own `Budget` (`Poll`, `StepJob`) re-arms
/// the deadline before dispatching, mirroring `WasmtimeRuntime::execute_turn`/`step_job`'s own
/// per-call `store.set_fuel`/`store.set_epoch_deadline` pattern.
struct DeadlineCell(Mutex<Instant>);

impl DeadlineCell {
    // 🚫️async: R9 — `new`/`extend` are pure `Mutex` writes with zero suspension points, and their
    // one real consumer, `passed`, is called from INSIDE the sync `FnMut` closure
    // `Store::epoch_deadline_callback` requires (wasmtime's own API — E1-equivalent, fixed outside
    // this repo). `.await` is illegal inside that closure, so `passed` cannot be async, and R9's
    // "E1 propagates one hop backwards" rule takes `new`/`extend` sync with it for symmetry (they
    // have no suspension point of their own either — verified: a single `Mutex::lock` write/read).
    fn new(initial: Duration) -> Arc<Self> {
        Arc::new(Self(Mutex::new(Instant::now() + initial)))
    }

    fn extend(&self, from_now: Duration) {
        *self.0.lock().expect("DeadlineCell poisoned") = Instant::now() + from_now;
    }

    fn passed(&self) -> bool {
        Instant::now() >= *self.0.lock().expect("DeadlineCell poisoned")
    }
}

/// ⏱️ Installed ONCE per `Store` at construction (wasmtime allows exactly one
/// `epoch_deadline_callback`), reading `deadline` fresh every tick — this is what lets a single
/// long-lived callback serve every command an actor is ever given across its whole lifetime, not
/// just the first one.
async fn install_epoch_budget(store: &mut Store<AsyncActorHostState>, deadline: Arc<DeadlineCell>) {
    store.epoch_deadline_callback(move |_ctx: StoreContextMut<'_, AsyncActorHostState>| if deadline.passed() { Ok(UpdateDeadline::Interrupt) } else { Ok(UpdateDeadline::Yield(1)) });
    store.set_epoch_deadline(1);
}
//#endregion ⏱️Budgets

//#region 📡️AsyncActorCommand
/// 📡️ One variant per `world actor` export this pooled runtime drives. No `Describe` variant:
/// `describe.wit`'s `describe()` is called once, out-of-band, by `semio-framework-plugin-describe`'s
/// own binary (`📓️terra-world-collapse-report.md` §8) — never per-actor-instance, so it has no
/// place in a per-actor command channel. `GuestRuntime::compile`/`instantiate`/`drop_instance` have
/// no export-call analogue at all (they never touch a live `Store`), so they have no command either.
pub enum AsyncActorCommand {
    /// 🧬️ `reactor::poll` — the async analogue of `GuestRuntime::execute_turn`. `events`/`budget`
    /// are the same `semio_framework::kernel::{Event,Budget}` that trait method already takes; this
    /// file does the kernel→WIT event encoding and WIT→kernel turn-result decoding, reusing
    /// `component.rs`'s own conversion fns (see [`AsyncActorTask::spawn`]) rather than
    /// re-implementing them a second time.
    Poll {
        events: Vec<Event>,
        budget: Budget,
        reply: tokio::sync::oneshot::Sender<Result<KernelTurnResult, String>>,
    },
    StartJob {
        job: u64,
        kind: String,
        input: Vec<u8>,
        reply: tokio::sync::oneshot::Sender<Result<(), String>>,
    },
    StepJob {
        job: u64,
        budget: JobBudget,
        reply: tokio::sync::oneshot::Sender<Result<JobStep, String>>,
    },
    CancelJob {
        job: u64,
        reply: tokio::sync::oneshot::Sender<Result<(), String>>,
    },
    Checkpoint(tokio::sync::oneshot::Sender<Result<Vec<u8>, String>>),
    Restore(Vec<u8>, tokio::sync::oneshot::Sender<Result<(), String>>),
    /// 🛑️ Ends the root task's control loop. The caller is expected to follow this with
    /// [`AsyncActorTask::cancel`] if the loop does not exit promptly on its own for some reason
    /// (it always should — unlike the deleted `call_run`, nothing here ever parks on a host import
    /// at the OUTER loop level; every WIT call runs inside its own `accessor.spawn`'d task).
    Shutdown,
}
//#endregion 📡️AsyncActorCommand

//#region 🐛️Poll result conversion
/// 🐛️ Reuses `component.rs`'s own `wit_effect_to_kernel`/`wit_turn_status_to_kernel` (private
/// module-level fns in `component`, visible here because `runtime` is a Rust descendant of that
/// module — same access `imports`/`effects`/`shard` already rely on for their own crate-private
/// helpers) rather than re-deriving the ~35-variant `Effect` match a second time in this file.
/// `emitted` is prepended (it happened earlier in the turn, via `host-async.emit`, than anything
/// `poll` itself returns) — same ordering `WasmtimeRuntime::execute_turn` uses for its own
/// `emit_sink.chain(wit_turn_result.effects)`.
async fn convert_poll_success(turn: wit_reactor::TurnResult, mut effects: Vec<Effect>) -> Result<KernelTurnResult, String> {
    for effect in turn.effects {
        match super::wit_effect_to_kernel(effect).await {
            Ok(kernel_effect) => effects.push(kernel_effect),
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(KernelTurnResult {
        // 🚧️ Same open gap `component.rs`'s own `execute_turn` and `imports.rs`'s own `patch_sink`
        // doc already carry: WIT `patch-op`'s `path: list<u32>` + `node: pack` vs kernel `PatchOp`'s
        // `path: String` + `node: UiNode` has no agreed conversion yet.
        ui_patches: semio_framework::kernel::UiTurnPatches::default(),
        effects,
        // 👥️ terra-shard-lane: same wire-shape mismatch as `component.rs`'s `execute_turn` —
        // `turn.presence` is real guest-emitted data (WIT `presence-update{peer: pack}`, a
        // pack-encoded `📡️replication/📡️wire::PresencePeer`, the collaboration-ROSTER shape), but
        // `KernelTurnResult.presence: Vec<ui_contract::PresenceUpdate>` wants the render-plane,
        // `(surface, node_key)`-addressed channel — a DIFFERENT shape by that field's own doc
        // comment (`🎠️kernel/🦀️.rs:918-923`), and no `PresencePeer → PresenceUpdate`
        // conversion exists anywhere in this repo yet. See `📓️terra-shard-lane-report.md`'s
        // presence-wire-mismatch finding.
        presence: Vec::new(),
        next_wake: turn.next_wake,
        status: super::wit_turn_status_to_kernel(turn.status).await,
        fuel_used: turn.fuel_used,
        command_ingress: super::wit_command_ingress_to_kernel(turn.command_ingress),
        lifecycle_receipt: turn.lifecycle_receipt.map(super::wit_lifecycle_receipt_to_kernel),
        ui_patch_receipt: turn.ui_patch_receipt.map(super::wit_patch_receipt_to_kernel),
    })
}
//#endregion 🐛️Poll result conversion

//#region 🧬️AsyncActorTask — one root task per actor
/// 🧬️ One [`AsyncActorTask`] = one `Store<AsyncActorHostState>` = one actor generation (S1b/S1c's
/// confirmed shape, unaffected by the collapse). `spawn` REQUIRES an `initial_budget` up front
/// (mirrors `ShardExecutor::spawn`'s own `initial` list, and `WasmtimeRuntime::instantiate`'s own
/// `store.set_fuel(budget.fuel)` at construction) so this file never has to model "no budget yet" as
/// a sentinel.
pub struct AsyncActorTask {
    join: tokio::task::JoinHandle<()>,
    pub commands: tokio::sync::mpsc::UnboundedSender<AsyncActorCommand>,
}

impl AsyncActorTask {
    pub async fn spawn(engine: &AsyncEngineHandle, component: Arc<wasmtime::component::Component>, state: AsyncActorHostState, initial_budget: Budget, instance_id: u32) -> Result<Self, PluginHostError> {
        let (commands_tx, mut commands_rx) = tokio::sync::mpsc::unbounded_channel::<AsyncActorCommand>();
        let (ready_tx, ready_rx) = tokio::sync::oneshot::channel::<Result<(), PluginHostError>>();
        let engine_handle = engine.engine.clone();
        let linker = engine.linker.clone();

        let join = tokio::spawn(async move {
            // 🎯️ Harness tests D/E: the `Store` is constructed and OWNED right here, inside the
            // spawned task body — never handed out to, or borrowed by, anything outside this async
            // block. `JoinHandle::abort()` on THIS task therefore drops the future AND the Store
            // together, which is what actually cancels an in-flight host import (test D proved
            // dropping a borrowed future alone is NOT enough).
            let deadline = DeadlineCell::new(Duration::from_millis(initial_budget.deadline_ms as u64));
            let mut store = Store::new(&engine_handle, state);
            let _ = store.set_fuel(initial_budget.fuel);
            install_epoch_budget(&mut store, deadline.clone()).await;

            // 🧪️ harness test F, re-confirmed against the real collapsed world by this rewrite's own
            // acceptance run (see packet report): the generated `Actor` implements neither `Clone`
            // nor `Copy` but IS `Send + Sync`, so wrapping the ONE instance in `Arc` and cloning the
            // `Arc` into every `accessor.spawn`'d command task works. One instance, not two.
            let instance = match actor_bindings::Actor::instantiate_async(&mut store, &component, &linker).await {
                Ok(instance) => Arc::new(instance),
                Err(error) => {
                    let _ = ready_tx.send(Err(PluginHostError::Wasmtime(error.to_string())));
                    return;
                }
            };
            let _ = ready_tx.send(Ok(()));

            // 🧬️ The whole actor lifetime lives inside ONE `run_concurrent` call. The closure below
            // never itself awaits a WIT export — it only receives commands and hands each one to
            // `accessor.spawn` as its own `AccessorTask`, so two commands against this SAME `Store`
            // (e.g. `Checkpoint` answered while a slow `StepJob` is still in flight) genuinely run
            // concurrently, reproducing harness test F's proof against the real world rather than
            // merely asserting it still holds.
            let _ = store
                .run_concurrent(async move |accessor: &Accessor<AsyncActorHostState>| {
                    loop {
                        match commands_rx.recv().await {
                            None | Some(AsyncActorCommand::Shutdown) => break,
                            Some(AsyncActorCommand::Poll { events, budget, reply }) => {
                                deadline.extend(Duration::from_millis(budget.deadline_ms as u64));
                                accessor.with(|mut access| {
                                    let _ = access.as_context_mut().set_fuel(budget.fuel);
                                });
                                let mut wit_events_vec: Vec<wit_events::Event> = Vec::with_capacity(events.len());
                                for event in &events {
                                    wit_events_vec.push(super::kernel_event_to_wit(event, instance_id).await);
                                }
                                let wit_budget = wit_reactor::Budget { fuel: budget.fuel, deadline_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };

                                struct PollTask {
                                    instance: Arc<actor_bindings::Actor>,
                                    events: Vec<wit_events::Event>,
                                    budget: wit_reactor::Budget,
                                    reply: tokio::sync::oneshot::Sender<Result<KernelTurnResult, String>>,
                                }
                                impl AccessorTask<AsyncActorHostState> for PollTask {
                                    async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                        let outcome = self.instance.semio_framework_reactor().call_poll(accessor, self.events, None, self.budget).await;
                                        let mapped = match outcome {
                                            Ok(Ok(turn)) => {
                                                // 🚫️async: E5 executor bridge. `AsyncActorHostState::take_effects`/`take_patches`
                                                // (`imports.rs`) are async-signatured but their bodies are a single `mem::take` with
                                                // zero suspension points — same idiom `imports.rs`'s own `emit()` uses for
                                                // `wit_effect_to_kernel`. `Accessor::with` closures are sync-only (see `imports.rs`'s
                                                // `snapshot_call` precedent: every async follow-up happens OUTSIDE `.with()`, on plain
                                                // owned data extracted synchronously), so `block_on` is the sound bridge here, not a
                                                // shortcut around it.
                                                let (emitted, _patches) = accessor.with(|mut access| {
                                                    let state = access.get();
                                                    (semio_framework_async::block_on(state.take_effects()), semio_framework_async::block_on(state.take_patches()))
                                                });
                                                convert_poll_success(turn, emitted).await
                                            }
                                            Ok(Err(fault)) => Err(format!("{fault:?}")),
                                            Err(trap) => Err(trap.to_string()),
                                        };
                                        let _ = self.reply.send(mapped);
                                        Ok(())
                                    }
                                }
                                let _ = accessor.spawn(PollTask { instance: instance.clone(), events: wit_events_vec, budget: wit_budget, reply });
                            }
                            Some(AsyncActorCommand::StartJob { job, kind, input, reply }) => {
                                struct StartJobTask {
                                    instance: Arc<actor_bindings::Actor>,
                                    job: u64,
                                    kind: String,
                                    input: Vec<u8>,
                                    reply: tokio::sync::oneshot::Sender<Result<(), String>>,
                                }
                                impl AccessorTask<AsyncActorHostState> for StartJobTask {
                                    async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                        let outcome = self.instance.semio_framework_jobs().call_start_job(accessor, self.job, self.kind, self.input).await;
                                        let mapped = match outcome {
                                            Ok(Ok(())) => Ok(()),
                                            Ok(Err(fault)) => Err(format!("{fault:?}")),
                                            Err(trap) => Err(trap.to_string()),
                                        };
                                        let _ = self.reply.send(mapped);
                                        Ok(())
                                    }
                                }
                                let _ = accessor.spawn(StartJobTask { instance: instance.clone(), job, kind, input, reply });
                            }
                            Some(AsyncActorCommand::StepJob { job, budget, reply }) => {
                                deadline.extend(Duration::from_millis(budget.deadline_ms as u64));
                                accessor.with(|mut access| {
                                    let _ = access.as_context_mut().set_fuel(budget.fuel);
                                });
                                let wit_budget = wit_jobs::JobBudget { fuel: budget.fuel, deadline_ms: budget.deadline_ms };

                                struct StepJobTask {
                                    instance: Arc<actor_bindings::Actor>,
                                    job: u64,
                                    budget: wit_jobs::JobBudget,
                                    reply: tokio::sync::oneshot::Sender<Result<JobStep, String>>,
                                }
                                impl AccessorTask<AsyncActorHostState> for StepJobTask {
                                    async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                        let outcome = self.instance.semio_framework_jobs().call_step_job(accessor, self.job, self.budget).await;
                                        let mapped = match outcome {
                                            Ok(Ok(step)) => Ok(match step {
                                                wit_jobs::JobStep::Running(progress) => JobStep::Running { progress },
                                                wit_jobs::JobStep::Done(output) => JobStep::Done { output },
                                                wit_jobs::JobStep::Failed(error) => JobStep::Failed { error },
                                            }),
                                            Ok(Err(fault)) => Err(format!("{fault:?}")),
                                            Err(trap) => Err(trap.to_string()),
                                        };
                                        let _ = self.reply.send(mapped);
                                        Ok(())
                                    }
                                }
                                let _ = accessor.spawn(StepJobTask { instance: instance.clone(), job, budget: wit_budget, reply });
                            }
                            Some(AsyncActorCommand::CancelJob { job, reply }) => {
                                struct CancelJobTask {
                                    instance: Arc<actor_bindings::Actor>,
                                    job: u64,
                                    reply: tokio::sync::oneshot::Sender<Result<(), String>>,
                                }
                                impl AccessorTask<AsyncActorHostState> for CancelJobTask {
                                    async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                        // 🧬️ `jobs.wit`'s `cancel-job: async func(job: u64);` has no `result<_, plugin-error>`
                                        // wrapper (unlike `start-job`/`step-job`), same asymmetry `component.rs`'s own
                                        // `cancel_job` comment documents — only the trap-level Result exists here.
                                        let mapped = match self.instance.semio_framework_jobs().call_cancel_job(accessor, self.job).await {
                                            Ok(()) => Ok(()),
                                            Err(trap) => Err(trap.to_string()),
                                        };
                                        let _ = self.reply.send(mapped);
                                        Ok(())
                                    }
                                }
                                let _ = accessor.spawn(CancelJobTask { instance: instance.clone(), job, reply });
                            }
                            Some(AsyncActorCommand::Checkpoint(reply)) => {
                                struct CheckpointTask {
                                    instance: Arc<actor_bindings::Actor>,
                                    reply: tokio::sync::oneshot::Sender<Result<Vec<u8>, String>>,
                                }
                                impl AccessorTask<AsyncActorHostState> for CheckpointTask {
                                    async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                        let outcome = self.instance.semio_framework_checkpoint().call_checkpoint(accessor).await;
                                        let mapped = match outcome {
                                            Ok(Ok(bytes)) => Ok(bytes),
                                            Ok(Err(fault)) => Err(format!("{fault:?}")),
                                            Err(trap) => Err(trap.to_string()),
                                        };
                                        let _ = self.reply.send(mapped);
                                        Ok(())
                                    }
                                }
                                let _ = accessor.spawn(CheckpointTask { instance: instance.clone(), reply });
                            }
                            Some(AsyncActorCommand::Restore(bytes, reply)) => {
                                struct RestoreTask {
                                    instance: Arc<actor_bindings::Actor>,
                                    state: Vec<u8>,
                                    reply: tokio::sync::oneshot::Sender<Result<(), String>>,
                                }
                                impl AccessorTask<AsyncActorHostState> for RestoreTask {
                                    async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                        let outcome = self.instance.semio_framework_checkpoint().call_restore(accessor, self.state).await;
                                        let mapped = match outcome {
                                            Ok(Ok(())) => Ok(()),
                                            Ok(Err(fault)) => Err(format!("{fault:?}")),
                                            Err(trap) => Err(trap.to_string()),
                                        };
                                        let _ = self.reply.send(mapped);
                                        Ok(())
                                    }
                                }
                                let _ = accessor.spawn(RestoreTask { instance: instance.clone(), state: bytes, reply });
                            }
                        }
                    }
                })
                .await;
        });

        match ready_rx.await {
            Ok(Ok(())) => Ok(Self { join, commands: commands_tx }),
            Ok(Err(error)) => {
                let _ = join.await;
                Err(error)
            }
            Err(_recv_error) => {
                let _ = join.await;
                Err(PluginHostError::Plugin("async actor task ended before instantiation completed".to_string()))
            }
        }
    }

    /// 🎯️ Root-task cancellation (harness tests D/E): `abort()` THEN `.await` the handle, so the
    /// `Store` this task owns is actually dropped (not merely the future) before this returns —
    /// see [`AsyncActorTask`]'s own doc for why dropping the future alone is not enough.
    pub async fn cancel(self) {
        self.join.abort();
        let _ = self.join.await;
    }
}
//#endregion 🧬️AsyncActorTask
