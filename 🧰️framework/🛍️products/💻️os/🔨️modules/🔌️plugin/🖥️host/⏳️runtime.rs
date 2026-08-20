//! ⏳️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-async-runtime). `WasmtimeAsyncRuntime` — the
//! execution backend that actually RUNS a `world actor-async` component: one root task per actor,
//! `store.run_concurrent(async move |accessor| { loop { select!(&mut call_run_fut, control) } })`,
//! consuming `⏳️imports.rs`'s `AsyncActorHostState`/`host_async` import layer (finished, accepted —
//! this file drives it, never reimplements it) and synthesizing `semio_framework::kernel::TurnResult`s
//! the DRR scheduler can feed into `Kernel::complete` exactly the way `🧵️shard/🦀️component.rs`'s
//! `to_actor_turn_result` bridge already does for the sync poll world (`ShardOutcome::Turn` →
//! `🎯️targets/🧊️wgpu/🎠️runtime.rs`'s `ParallelRuntime::complete` — this file's own outcome channel is
//! the async-world analogue of that same seam, not a new one).
//!
//! 🧪️ Every non-obvious claim below was type-checked and RUN against real wasmtime 47.0.3 in a
//! throwaway harness under the session scratchpad (never in-tree — this file cannot be mounted by
//! its own owning packet, see `## in-tree compilation` in the packet report) before being written
//! here. See `📓️terra-async-runtime-report.md`'s `## harness` for the six proven results (A-F) this
//! design rests on, and its `## honest gaps` for what is designed-but-NOT-separately-harness-proven
//! (the exact `select!` composition below) vs genuinely BLOCKED (the real `checkpoint-async`/
//! `jobs-async` WIT interfaces do not exist in the schema yet — the coordinator is landing that
//! change separately; this file's calls to them are written against the interface NAMES and SHAPE
//! the coordinator stated, unverified against a real compile).
//!
//! 🎯️ S1 vs S1b/S1c (`📓️terra-probe-spikes-report.md`): `Accessor::spawn` does NOT give CPU-bound
//! fairness between two tasks in ONE `Store` — this file never uses it for that. It DOES correctly
//! dispatch a short, non-CPU-bound concurrent call against the SAME instance while the root call is
//! in flight (S1's own `AccessorTask`+oneshot idiom, reproduced here as harness test F for
//! `checkpoint-async`) — that is the ONLY thing `Accessor::spawn` is used for below. Fairness BETWEEN
//! actors comes from one `Store` per actor, each driven by its own `tokio::spawn`ed task —
//! [`AsyncActorTask::spawn`] does exactly that internally (S1b/S1c's proven shape) — never from
//! `Accessor::spawn` across actors.

use crate::imports::{host_async_bindings, AsyncActorHostState};
use crate::{PluginHostError, SharedEngineConfig};
use semio_framework::kernel::{Effect, TurnResult as KernelTurnResult, TurnStatus as KernelTurnStatus};
use semio_framework_actor::Budget as ActorBudget;
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use wasmtime::component::{Accessor, AccessorTask, Destination, HasSelf, Linker, StreamProducer, StreamReader, StreamResult, VecBuffer};
use wasmtime::{AsContextMut, Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig, Store, StoreContextMut, UpdateDeadline};

//#region 🐎️AsyncEngineHandle
/// 🧩️ Mirrors `🖥️host/🦀️component.rs`'s `CORE_INSTANCES_PER_COMPONENT`/`MEMORIES_PER_COMPONENT`/
/// `TABLES_PER_COMPONENT` ratios verbatim — those `const`s are private to that file's module and this
/// file may not edit it (one-file `path_scope`), so the ratios are duplicated rather than shared, the
/// same small-duplication precedent `⏳️imports.rs`'s own module doc already establishes for
/// `fault_bytes`/`TraceIdAllocator`/`LANE_DEADLINE_CEILING_MS`.
const CORE_INSTANCES_PER_COMPONENT: u32 = 8;
const MEMORIES_PER_COMPONENT: u32 = 1;
const TABLES_PER_COMPONENT: u32 = 4;

/// 🐎️ ONE async-capable `Engine` per process, distinct from `build_shared_engine`'s (that one never
/// sets `wasm_component_model_async`/`concurrency_support` — it backs `world actor`'s sync poll
/// loop). `wasm_component_model_async(true)` is engine-wide and, per the coordinator's post-S7
/// finding, makes EVERY plain sync `func` export on stores built from this engine uncallable at all
/// (not merely unsafe-if-concurrent) — this is exactly why `checkpoint`/`jobs` are becoming
/// `checkpoint-async`/`jobs-async` rather than staying sync, and why this engine must never be
/// shared with `world actor`'s stores.
pub async fn build_async_engine(cfg: SharedEngineConfig) -> Result<(Engine, bool), PluginHostError> {
    let build = |pooling: bool| -> wasmtime::Result<Engine> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.wasm_component_model_async(true);
        config.concurrency_support(true);
        config.consume_fuel(true);
        config.epoch_interruption(true);
        if pooling {
            let mut pooling_cfg = PoolingAllocationConfig::default();
            pooling_cfg.total_component_instances(cfg.total_component_instances);
            pooling_cfg.total_core_instances(cfg.total_component_instances * CORE_INSTANCES_PER_COMPONENT);
            pooling_cfg.total_memories(cfg.total_component_instances * MEMORIES_PER_COMPONENT);
            pooling_cfg.total_tables(cfg.total_component_instances * TABLES_PER_COMPONENT);
            pooling_cfg.total_gc_heaps(cfg.total_component_instances * MEMORIES_PER_COMPONENT);
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

/// 🐎️ One async engine + its epoch ticker + a `Linker` with `pure`/`host_async` already wired —
/// built ONCE per process, `Arc`-shared into every [`AsyncActorTask::spawn`] call. Reuses
/// `crate::EpochTicker` (already `pub` in `🦀️component.rs`) rather than duplicating a THIRD 1ms
/// ticker thread — one ticker per shared `Engine` is that type's own stated contract.
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
        let (engine, _pooling_active) = build_async_engine(cfg)?;
        let epoch_ticker = crate::EpochTicker::start(&engine);
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        // 🔒️ LEASE-REQUIRED (see packet report `## lease-requests`): `host_async_bindings` is a
        // private `mod` inside `⏳️imports.rs` today. This call — and every other reference to
        // `host_async_bindings::*`/`AsyncActorHostState` below — cannot resolve until that mount
        // changes to `pub(crate) mod host_async_bindings`. Written against the shape that fix
        // produces; UNRUN until it lands (see report `## in-tree compilation`).
        host_async_bindings::ActorAsync::add_to_linker::<AsyncActorHostState, HasSelf<AsyncActorHostState>>(&mut linker, |state| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(Self { engine, _epoch_ticker: epoch_ticker, linker: Arc::new(linker) })
    }
}
//#endregion 🐎️AsyncEngineHandle

//#region ⏱️Budgets — per-grant fuel + epoch Yield/Interrupt
/// ⏱️ terra-async-runtime harness test C, byte-for-byte: `Yield(1)` while `Instant::now()` is still
/// before the current grant's deadline (cooperative — S1c proved this genuinely preempts pure
/// CPU-bound guest code across separate `Store`s, no host-import confound), `Interrupt` once it has
/// passed (hard trap — the actor exceeded ITS OWN grant's wall-clock ceiling without the host ever
/// refilling it in time; becomes `TurnStatus::Faulted` at synthesis). `Mutex<Instant>` rather than an
/// `AtomicU64` epoch count: `set_epoch_deadline` takes a DELTA, not an absolute epoch (coordinator's
/// post-S7 finding — `u64::MAX` wraps `current_epoch + delta` and traps the store on its very first
/// call), so this file never encodes "no deadline" as a sentinel at all — [`AsyncActorTask::spawn`]
/// REQUIRES an initial [`TurnGrant`] up front (mirrors `ShardExecutor::spawn`'s own `initial` list),
/// and every grant sets a real `Instant` deadline plus re-arms the epoch counter by exactly 1 tick
/// (`set_epoch_deadline(1)`) — never a large delta meant to represent infinity.
struct DeadlineCell(Mutex<Instant>);

impl DeadlineCell {
    async fn new(initial: Duration) -> Arc<Self> {
        Arc::new(Self(Mutex::new(Instant::now() + initial)))
    }

    async fn extend(&self, from_now: Duration) {
        *self.0.lock().expect("DeadlineCell poisoned") = Instant::now() + from_now;
    }

    async fn passed(&self) -> bool {
        Instant::now() >= *self.0.lock().expect("DeadlineCell poisoned")
    }
}

/// ⏱️ Installed ONCE per `Store` at construction (wasmtime allows exactly one
/// `epoch_deadline_callback`), reading `deadline` fresh every tick — this is what lets a single
/// long-lived callback serve every grant an actor is ever given across its whole lifetime, not just
/// the first one.
async fn install_epoch_budget(store: &mut Store<AsyncActorHostState>, deadline: Arc<DeadlineCell>) {
    store.epoch_deadline_callback(move |_ctx: StoreContextMut<'_, AsyncActorHostState>| if deadline.passed() { Ok(UpdateDeadline::Interrupt) } else { Ok(UpdateDeadline::Yield(1)) });
    store.set_epoch_deadline(1);
}

/// ⏱️ terra-async-runtime honest gap: fuel is armed via `access.as_context_mut().set_fuel(fuel)`
/// (`Access` implements `AsContextMut` — confirmed by reading `wasmtime-47.0.3/src/runtime/
/// component/concurrent.rs`'s own `impl AsContextMut for Access`) directly at each call site inside
/// [`AsyncActorTask::spawn`]'s control loop, rather than through a wrapper here — there is no `&mut
/// Store` reachable from outside the `run_concurrent` closure to hand a helper function, only an
/// `Access` obtained moment-to-moment via `accessor.with(...)`. Fuel is the HARD per-grant CPU cap
/// (exhaustion traps immediately; `consume_fuel` is enabled with no `fuel_async_yield_interval` —
/// deliberately: a grant's fuel is a ceiling, not a cooperative-yield lever, that job belongs to
/// epoch alone here); the epoch deadline (above) is the independent WALL-CLOCK safety net. Both are
/// named in the mission brief as the two per-grant levers.
//#endregion ⏱️Budgets

//#region 🎫️GrantWindow / GrantedEventProducer — "a grant is delivery permission plus a refill, not a thread"
/// 🎫️ terra-async-runtime harness test B, unchanged: parks whenever `remaining == 0`, even if
/// `queue` still holds items from a grant that has already been spent — unlike the proven
/// `ChunkStreamProducer`/`WakeyProducer` (`⏳️imports.rs`/S5), which only ever park on a genuinely
/// EMPTY queue. `exhausted` is notified the INSTANT `remaining` hits zero while parking, which is
/// exactly the boundary [`AsyncActorTask::spawn`]'s control loop waits on to synthesize a
/// [`KernelTurnResult`] — turn synthesis is driven by THIS notification, not by polling.
struct GrantWindow {
    queue: VecDeque<host_async_bindings::semio::framework::events::Event>,
    remaining: u32,
    closed: bool,
    waker: Option<Waker>,
    exhausted: Arc<tokio::sync::Notify>,
}

struct GrantedEventProducer {
    window: Arc<Mutex<GrantWindow>>,
}

impl StreamProducer<AsyncActorHostState> for GrantedEventProducer {
    type Item = host_async_bindings::semio::framework::events::Event;
    type Buffer = VecBuffer<host_async_bindings::semio::framework::events::Event>;

    fn poll_produce<'a>(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _store: StoreContextMut<'a, AsyncActorHostState>,
        mut destination: Destination<'a, host_async_bindings::semio::framework::events::Event, Self::Buffer>,
        _finish: bool,
    ) -> Poll<wasmtime::Result<StreamResult>> {
        let mut window = self.window.lock().expect("GrantWindow poisoned");
        if window.remaining > 0 {
            if let Some(item) = window.queue.pop_front() {
                window.remaining -= 1;
                let just_exhausted = window.remaining == 0;
                destination.set_buffer(vec![item].into());
                if just_exhausted {
                    window.exhausted.notify_one();
                }
                return Poll::Ready(Ok(StreamResult::Completed));
            }
        }
        if window.closed && window.queue.is_empty() {
            return Poll::Ready(Ok(StreamResult::Dropped));
        }
        window.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// 🎫️ One `TurnGrant` = one `ShardFrame::Grant`'s worth of work, reusing
/// [`semio_framework_actor::Budget`] field-for-field (the SAME type `ShardFrame::Grant`/
/// `turn_budget_from_grant` already carry) so a future DRR-scheduler caller needs no new conversion
/// to feed this file — only `to_actor_turn_result`'s existing bridge on the way OUT.
pub struct TurnGrant {
    pub events: Vec<host_async_bindings::semio::framework::events::Event>,
    pub budget: ActorBudget,
}

/// 🎫️ The caller-facing refill handle — `Arc`-cloned, `Send + Sync`, safe to call from whatever
/// thread the DRR scheduler/`ShardLoop`-equivalent for async actors runs on. Refilling does NOT
/// itself touch the `Store` (it cannot — only code running inside the accessor closure may); it only
/// updates shared state and wakes whichever waker is parked, mirroring `refill`/`close` from the
/// harness exactly.
pub struct GrantHandle {
    window: Arc<Mutex<GrantWindow>>,
    pending_budget: Arc<Mutex<Option<ActorBudget>>>,
    refilled: Arc<tokio::sync::Notify>,
}

impl GrantHandle {
    /// 🎫️ `remaining` increases by exactly `grant.events.len()` — a `TurnGrant` IS its own event
    /// budget (unlike the harness's `refill(events, budget: u32)`, which took the count separately
    /// only because its test wanted to probe queuing more events than the current grant allows;
    /// here the grant and the events it releases are the same value by construction, so there is no
    /// separate number to get wrong). `pending_budget` carries the fuel/deadline half of the grant
    /// to the control loop, which is the only code with `Access` to actually apply it to the `Store`.
    pub async fn refill(&self, grant: TurnGrant) {
        let event_count = grant.events.len() as u32;
        *self.pending_budget.lock().expect("pending_budget poisoned") = Some(grant.budget);
        let waker = {
            let mut w = self.window.lock().expect("GrantWindow poisoned");
            w.queue.extend(grant.events);
            w.remaining += event_count;
            w.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
        self.refilled.notify_one();
    }

    pub async fn close(&self) {
        let waker = {
            let mut w = self.window.lock().expect("GrantWindow poisoned");
            w.closed = true;
            w.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}
//#endregion 🎫️GrantWindow / GrantedEventProducer

//#region 🏁️Turn synthesis
/// 🏁️ `semio_framework::kernel::TurnResult` synthesized ENTIRELY host-side from three sources — the
/// fuel delta since the current grant was armed, `state.take_effects()` (already
/// `semio_framework::kernel::Effect`-shaped by `⏳️imports.rs`'s own `emit` handler, so no
/// conversion needed here), and the boundary `status` the caller determines (see call sites in
/// [`AsyncActorTask::spawn`]). Never calls into the guest — this is what makes it possible to
/// synthesize a turn boundary mid-grant, without the guest itself returning from anything.
///
/// 🚧️ `ui_patches: Vec::new()` — the SAME open gap `🖥️host/🦀️component.rs`'s own `execute_turn` and
/// `⏳️imports.rs`'s own `patch_sink` doc already document (WIT `patch-op`'s `path: list<u32>` +
/// `node: pack` vs kernel `PatchOp`'s `path: String` + `node: UiNode` — no agreed conversion yet).
/// `next_wake: None` — this world has no per-grant timer-request signal reaching this file yet
/// (`effects::SetTimer` still lands in `effects`, not as a distinguished return value); a real
/// value needs a source, not an invented one.
async fn synthesize_turn_result(state: &mut AsyncActorHostState, fuel_before: u64, fuel_after: u64, status: KernelTurnStatus) -> KernelTurnResult {
    let effects: Vec<Effect> = state.take_effects();
    let _patches_gap_see_module_doc = state.take_patches();
    KernelTurnResult { ui_patches: Vec::new(), effects, next_wake: None, status, fuel_used: fuel_before.saturating_sub(fuel_after) }
}
//#endregion 🏁️Turn synthesis

//#region 📡️AsyncActorCommand — checkpoint/restore/jobs, dispatched via Accessor::spawn (S1's idiom, harness test F)
/// 📡️ terra-async-runtime BLOCKED-SEAM, resolved mid-packet: the coordinator's post-S7 finding
/// (`checkpoint`/`jobs` become `checkpoint-async`/`jobs-async`, `async func` throughout) means these
/// ARE now callable from here — via the exact mechanism harness test F proved (a short-lived
/// `AccessorTask` + oneshot reply, `accessor.spawn`'d from the control loop so it runs CONCURRENTLY
/// with the long-lived `call_run` future on the SAME instance, never blocking it). What is still
/// unverified: the WIT interfaces `checkpoint-async`/`jobs-async` do not exist in
/// `🧬️schema/📜️component.wit` as this file is written — the coordinator is landing that change
/// separately. The generated accessor method names below (`instance.semio_framework_checkpoint_async()`
/// / `instance.semio_framework_jobs_async()`) are predicted from the SAME kebab-to-snake,
/// package-then-interface naming this file already confirmed empirically for `runner`/
/// `checkpoint-async` in the scratch harness (`instance.semio_runtimeprobe_runner()`/
/// `instance.semio_runtimeprobe_checkpoint_async()`), not independently verified against the real
/// schema. `JobBudgetArg`/`JobStepResult` below are plain host-side mirrors of the CURRENT (soon to
/// be `async`) `job-budget`/`job-step` WIT shapes (`🧬️schema/📜️component.wit`
/// `interface jobs` — `fuel: u64, deadline-ms: u32` / `running(option<list<u8>>) | done(list<u8>) |
/// failed(list<u8>)`), not the real generated bindgen types — a real compile will need to convert
/// between these and whatever `jobs_async`'s bindgen output actually names them.
pub struct JobBudgetArg {
    pub fuel: u64,
    pub deadline_ms: u32,
}

pub enum JobStepResult {
    Running { progress: Option<Vec<u8>> },
    Done { output: Vec<u8> },
    Failed { error: Vec<u8> },
}

pub enum AsyncActorCommand {
    Checkpoint(tokio::sync::oneshot::Sender<Result<Vec<u8>, String>>),
    Restore(Vec<u8>, tokio::sync::oneshot::Sender<Result<(), String>>),
    StartJob { job: u64, kind: String, input: Vec<u8>, reply: tokio::sync::oneshot::Sender<Result<(), String>> },
    StepJob { job: u64, budget: JobBudgetArg, reply: tokio::sync::oneshot::Sender<Result<JobStepResult, String>> },
    CancelJob { job: u64 },
    /// 🛑️ Ends the root task's control loop without waiting for `call_run` to return on its own —
    /// the caller is expected to follow this with [`AsyncActorTask::cancel`] if `call_run` does not
    /// resolve promptly on its own (e.g. it is genuinely parked forever on a host import).
    Shutdown,
}
//#endregion 📡️AsyncActorCommand

//#region 🧬️AsyncActorTask — one root task per actor
/// 🧬️ Emitted per grant boundary AND once, finally, when the actor's `run()` call itself resolves
/// (cleanly or trapped) — the SAME channel carries both, mirroring `ShardOutcome::Turn` (one variant,
/// many sends) rather than inventing a separate "final" wire type. The receiver (a future packet's
/// async-actor scheduler, this file's analogue of `ParallelRuntime`) tells `Turn` from `Finished`
/// apart to know whether to expect more.
pub enum AsyncTurnOutcome {
    /// 🎫️ A grant window closed while the actor is still alive — `to_actor_turn_result` + host clock
    /// measurements + `Kernel::complete` is the caller's next hop, exactly as it already is for
    /// `ShardOutcome::Turn` (see this file's module doc).
    Turn(KernelTurnResult),
    /// 🏁️ `call_run` itself returned — the actor's `runner::run` call is OVER (the host closed the
    /// event stream, or the guest trapped). No further `Turn`s will follow on this channel for this
    /// actor generation; a real restart needs a fresh [`AsyncActorTask::spawn`].
    Finished(KernelTurnResult),
}

/// 🧬️ One [`AsyncActorTask`] = one `Store<AsyncActorHostState>` = one actor generation (S1b/S1c's
/// confirmed shape). `spawn` REQUIRES an initial [`TurnGrant`] (mirrors `ShardExecutor::spawn`'s own
/// `initial` list) so this file never has to model "no budget yet" as a sentinel — `consume_fuel`
/// with `set_fuel(0)` or an epoch deadline computed from a missing budget would both trap
/// immediately, which is exactly the class of bug the coordinator's post-S7 `set_epoch_deadline`
/// finding warns about.
pub struct AsyncActorTask {
    join: tokio::task::JoinHandle<KernelTurnResult>,
    pub commands: tokio::sync::mpsc::UnboundedSender<AsyncActorCommand>,
    pub grant: GrantHandle,
}

impl AsyncActorTask {
    #[allow(clippy::too_many_arguments)]
    pub async fn spawn(engine: &AsyncEngineHandle, component: Arc<wasmtime::component::Component>, mut state: AsyncActorHostState, initial: TurnGrant, outcomes: tokio::sync::mpsc::UnboundedSender<AsyncTurnOutcome>) -> Self {
        let (commands_tx, mut commands_rx) = tokio::sync::mpsc::unbounded_channel::<AsyncActorCommand>();
        let window = Arc::new(Mutex::new(GrantWindow { queue: VecDeque::new(), remaining: 0, closed: false, waker: None, exhausted: Arc::new(tokio::sync::Notify::new()) }));
        let pending_budget = Arc::new(Mutex::new(None::<ActorBudget>));
        let refilled = Arc::new(tokio::sync::Notify::new());
        let grant = GrantHandle { window: window.clone(), pending_budget: pending_budget.clone(), refilled: refilled.clone() };
        // 🎯️ The initial grant is applied directly (not through `GrantHandle::refill`, which exists
        // for a caller on a DIFFERENT task/thread) — no waker to wake yet, nothing parked.
        {
            let mut w = window.lock().expect("GrantWindow poisoned");
            w.remaining = initial.events.len() as u32;
            w.queue.extend(initial.events);
        }
        let initial_budget = initial.budget;
        let engine_handle = engine.engine.clone();
        let linker = engine.linker.clone();
        let exhausted = window.lock().expect("GrantWindow poisoned").exhausted.clone();

        let join = tokio::spawn(async move {
            // 🎯️ Harness tests D/E: the `Store` is constructed and OWNED right here, inside the
            // spawned task body — never handed out to, or borrowed by, anything outside this async
            // block. `JoinHandle::abort()` on THIS task therefore drops the future AND the Store
            // together, which is what actually cancels an in-flight host import (test D proved
            // dropping a borrowed future alone is NOT enough).
            let deadline = DeadlineCell::new(Duration::from_millis(initial_budget.wall_ms as u64));
            let mut store = Store::new(&engine_handle, state);
            let fuel_at_grant_start = Arc::new(Mutex::new(initial_budget.fuel));
            let _ = store.set_fuel(initial_budget.fuel);
            install_epoch_budget(&mut store, deadline.clone());

            // 🧪️ harness test F (re-run with this exact fix applied, see packet report `## harness`):
            // the generated world/instance struct (`ActorAsync` here, `RuntimeProbe` in the harness)
            // implements neither `Clone` nor `Copy` (S1's own report first found this and worked
            // around it with a SECOND `instantiate_async` call) — but it IS `Send + Sync`, so wrapping
            // the ONE instance in `Arc` and cloning the `Arc` into every `accessor.spawn`'d
            // checkpoint/job task works, confirmed by re-running the harness with exactly this change
            // substituted for the double-instantiate. One instance, not two.
            let instance = match host_async_bindings::ActorAsync::instantiate_async(&mut store, &component, &linker).await {
                Ok(instance) => Arc::new(instance),
                Err(error) => return synthesize_turn_result(store.data_mut(), initial_budget.fuel, 0, KernelTurnStatus::Faulted(error.to_string().into_bytes())),
            };
            let instance_ops = instance.clone();

            let final_result = store
                .run_concurrent(async move |accessor: &Accessor<AsyncActorHostState>| -> KernelTurnResult {
                    let events: StreamReader<host_async_bindings::semio::framework::events::Event> = match accessor.with(|access| StreamReader::new(access, GrantedEventProducer { window: window.clone() })) {
                        Ok(reader) => reader,
                        Err(error) => {
                            return accessor.with(|mut access| synthesize_turn_result(access.get(), initial_budget.fuel, 0, KernelTurnStatus::Faulted(error.to_string().into_bytes())));
                        }
                    };
                    let call_run_fut = instance.semio_framework_runner().call_run(accessor, events);
                    tokio::pin!(call_run_fut);
                    loop {
                        tokio::select! {
                            biased;
                            result = &mut call_run_fut => {
                                let fuel_after = accessor.with(|mut access| access.as_context_mut().get_fuel().unwrap_or(0));
                                let fuel_before = *fuel_at_grant_start.lock().expect("fuel_at_grant_start poisoned");
                                let status = match result {
                                    Ok(Ok(())) => KernelTurnStatus::Idle,
                                    Ok(Err(plugin_error)) => KernelTurnStatus::Faulted(format!("{plugin_error:?}").into_bytes()),
                                    Err(trap) => KernelTurnStatus::Faulted(trap.to_string().into_bytes()),
                                };
                                let turn = accessor.with(|mut access| synthesize_turn_result(access.get(), fuel_before, fuel_after, status));
                                break turn;
                            }
                            _ = exhausted.notified() => {
                                let fuel_after = accessor.with(|mut access| access.as_context_mut().get_fuel().unwrap_or(0));
                                let fuel_before = *fuel_at_grant_start.lock().expect("fuel_at_grant_start poisoned");
                                let turn = accessor.with(|mut access| synthesize_turn_result(access.get(), fuel_before, fuel_after, KernelTurnStatus::MoreWork));
                                if outcomes.send(AsyncTurnOutcome::Turn(turn)).is_err() {
                                    // 🛑️ Caller gone — treat exactly like an explicit Shutdown command
                                    // rather than spinning with nowhere to report further turns.
                                    break KernelTurnResult { ui_patches: Vec::new(), effects: Vec::new(), next_wake: None, status: KernelTurnStatus::Idle, fuel_used: 0 };
                                }
                            }
                            _ = refilled.notified() => {
                                if let Some(budget) = pending_budget.lock().expect("pending_budget poisoned").take() {
                                    accessor.with(|mut access| {
                                        let mut ctx = access.as_context_mut();
                                        let _ = ctx.set_fuel(budget.fuel);
                                    });
                                    deadline.extend(Duration::from_millis(budget.wall_ms as u64));
                                    *fuel_at_grant_start.lock().expect("fuel_at_grant_start poisoned") = budget.fuel;
                                }
                            }
                            command = commands_rx.recv() => {
                                match command {
                                    None | Some(AsyncActorCommand::Shutdown) => {
                                        let turn = accessor.with(|mut access| synthesize_turn_result(access.get(), *fuel_at_grant_start.lock().expect("fuel_at_grant_start poisoned"), 0, KernelTurnStatus::Idle));
                                        break turn;
                                    }
                                    Some(AsyncActorCommand::Checkpoint(reply)) => {
                                        struct CheckpointTask { instance: Arc<host_async_bindings::ActorAsync>, reply: tokio::sync::oneshot::Sender<Result<Vec<u8>, String>> }
                                        impl AccessorTask<AsyncActorHostState> for CheckpointTask {
                                            async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                                let outcome = self.instance.semio_framework_checkpoint_async().call_checkpoint(accessor).await;
                                                let mapped = match outcome { Ok(Ok(bytes)) => Ok(bytes), Ok(Err(fault)) => Err(format!("{fault:?}")), Err(trap) => Err(trap.to_string()) };
                                                let _ = self.reply.send(mapped);
                                                Ok(())
                                            }
                                        }
                                        let _ = accessor.spawn(CheckpointTask { instance: instance_ops.clone(), reply });
                                    }
                                    Some(AsyncActorCommand::Restore(bytes, reply)) => {
                                        struct RestoreTask { instance: Arc<host_async_bindings::ActorAsync>, state: Vec<u8>, reply: tokio::sync::oneshot::Sender<Result<(), String>> }
                                        impl AccessorTask<AsyncActorHostState> for RestoreTask {
                                            async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                                let outcome = self.instance.semio_framework_checkpoint_async().call_restore(accessor, &self.state).await;
                                                let mapped = match outcome { Ok(Ok(())) => Ok(()), Ok(Err(fault)) => Err(format!("{fault:?}")), Err(trap) => Err(trap.to_string()) };
                                                let _ = self.reply.send(mapped);
                                                Ok(())
                                            }
                                        }
                                        let _ = accessor.spawn(RestoreTask { instance: instance_ops.clone(), state: bytes, reply });
                                    }
                                    Some(AsyncActorCommand::StartJob { job, kind, input, reply }) => {
                                        struct StartJobTask { instance: Arc<host_async_bindings::ActorAsync>, job: u64, kind: String, input: Vec<u8>, reply: tokio::sync::oneshot::Sender<Result<(), String>> }
                                        impl AccessorTask<AsyncActorHostState> for StartJobTask {
                                            async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                                let outcome = self.instance.semio_framework_jobs_async().call_start_job(accessor, self.job, &self.kind, &self.input).await;
                                                let mapped = match outcome { Ok(Ok(())) => Ok(()), Ok(Err(fault)) => Err(format!("{fault:?}")), Err(trap) => Err(trap.to_string()) };
                                                let _ = self.reply.send(mapped);
                                                Ok(())
                                            }
                                        }
                                        let _ = accessor.spawn(StartJobTask { instance: instance_ops.clone(), job, kind, input, reply });
                                    }
                                    Some(AsyncActorCommand::StepJob { job, budget, reply }) => {
                                        struct StepJobTask { instance: Arc<host_async_bindings::ActorAsync>, job: u64, budget: JobBudgetArg, reply: tokio::sync::oneshot::Sender<Result<JobStepResult, String>> }
                                        impl AccessorTask<AsyncActorHostState> for StepJobTask {
                                            async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                                let outcome = self.instance.semio_framework_jobs_async().call_step_job(accessor, self.job, self.budget).await;
                                                let mapped = match outcome { Ok(Ok(step)) => Ok(step), Ok(Err(fault)) => Err(format!("{fault:?}")), Err(trap) => Err(trap.to_string()) };
                                                let _ = self.reply.send(mapped);
                                                Ok(())
                                            }
                                        }
                                        let _ = accessor.spawn(StepJobTask { instance: instance_ops.clone(), job, budget, reply });
                                    }
                                    Some(AsyncActorCommand::CancelJob { job }) => {
                                        struct CancelJobTask { instance: Arc<host_async_bindings::ActorAsync>, job: u64 }
                                        impl AccessorTask<AsyncActorHostState> for CancelJobTask {
                                            async fn run(self, accessor: &Accessor<AsyncActorHostState>) -> wasmtime::Result<()> {
                                                let _ = self.instance.semio_framework_jobs_async().call_cancel_job(accessor, self.job).await;
                                                Ok(())
                                            }
                                        }
                                        let _ = accessor.spawn(CancelJobTask { instance: instance_ops.clone(), job });
                                    }
                                }
                            }
                        }
                    }
                })
                .await;

            let final_turn = final_result.unwrap_or_else(|trap| KernelTurnResult { ui_patches: Vec::new(), effects: Vec::new(), next_wake: None, status: KernelTurnStatus::Faulted(trap.to_string().into_bytes()), fuel_used: 0 });
            let _ = outcomes.send(AsyncTurnOutcome::Finished(final_turn.clone()));
            final_turn
        });

        Self { join, commands: commands_tx, grant }
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
