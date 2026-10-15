//! 🛡️ Sandboxed wasmtime component plugin host with capability-gated imports.

#[path="🚪️io/🎛️operation/🦀️.rs"]
mod operation;
#[cfg(test)]
#[path="🧪️tests/🛂️receiving/🦀️.rs"]
pub(crate) mod test_native_authority;
#[path="🚪️io/🪶️snapshot/🦀️.rs"]
mod snapshot_call;
pub use snapshot_call::{SnapshotCall,SnapshotCallOutput,SnapshotCallStatus,SnapshotComponentReceipt};
#[path="⏱️deadline/🦀️.rs"]
mod original_deadline;
use original_deadline::{OriginalDeadlineError,remaining_original_deadline_us};

#[path = "🧵️shard/🦀️.rs"]
pub mod shard;

#[cfg(not(target_arch = "wasm32"))]
#[path = "🎠️activation/🦀️.rs"]
pub mod activation;
// 🚚️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1-process-shards): `ProcessTransport`/
// `StdioTransport` — see that file's own module doc for why they live here and not in `🎭️actor`.
#[path = "🧵️shard/🚚️process-transport/🦀️.rs"]
pub mod process_transport;
// ⚡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-effects-async): async effect execution — the
// `AsyncEffectExecutor` that classifies each post-turn `Effect`, derives its `OperationContext` and
// spawns it into the emitting actor's scope on `semio-framework-os-services`' pools — see that
// module's own doc.
#[path = "⚡️effects/🦀️.rs"]
pub mod effects;
// ⏳️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-async-imports → B1 world-collapse): `world
// actor`'s `interface host-async` import layer — 24 `async func` imports the guest can actually await, plus
// the `emit`/`emit-patch` one-way doors. Reuses `effects::AsyncServices`/`RouterEffectHandler`
// (above) as the real backends it awaits directly — see that module's own doc for the routing rule.
#[path = "📥️imports/🦀️.rs"]
pub mod imports;
#[path = "📥️ui-patch/🦀️.rs"]
mod ui_patch;
#[cfg(test)]
#[path = "📥️ui-patch/🧪️tests/🧪️component/🦀️.rs"]
mod ui_patch_component_tests;
#[path = "🧬️component-codec/🦀️.rs"]
mod component_codec;
pub use component_codec::{OwnedComponentDocumentCodec, GUEST_CODEC_BUDGET};

use semio_framework::{DslValue, PluginManifest, kernel::{ArtifactHandle, BrokerCapabilityGrant, Budget, CapabilityId, CapabilityRequest, Effect, Event, JobPlacement, MessageEndpoint, RequestId, RequestOutcome, TurnResult, TurnStatus, WindowHandle, WindowKindId}, };
use semio_framework_actor::ActorId as RuntimeActorId;
// 🌉️ `pub use`, not a plain `use` — `PackageRef`'s own fields are `PackageId`/`PackageHash`
// (`semio_framework_actor`, packet A1), so a downstream crate that does not itself depend on
// `semio_framework_actor` (e.g. `🏃️run`, which only depends on THIS crate) still needs a path to
// name them in order to construct one, exactly the way `GuestRuntime`/`GuestInstance`/`Budget`
// already reach it through this crate's own public API.
use semio_framework_plugin_guest_contract::{decode_guest_fault_bytes, identity_turn_fault, sqlite_wire, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, IoEntryRouteConflict, IoRouteConflict, JobBudget, JobStep, PluginHostError, TurnFault, GUEST_DIAGNOSTICS_CAPACITY_BYTES};
use semio_framework_plugin_interpreter_runtime::{OwnedCompiledGuest, OwnedGuestInstance, OwnedRuntime};
pub use semio_framework_actor::{PackageHash, PackageId};
use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_actor::{OriginalJobBudget,OriginalJobSource,OriginalJobStartReply,OriginalJobStepReply,OriginalJobOutcome,OriginalJobOutputReply,OriginalJobOutputAckReply,OriginalJobCancelReply,OriginalGuestExecutionBudget,OriginalGuestExecutionReceipt};
use semio_framework_plugin_interpreter_runtime::OriginalGuestJobCall;
use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig, ResourceLimiter, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

// 🗑️ `PLUGIN_FUEL_BUDGET` is gone — `design-runtime.md` §1's `Budget.fuel`/`⚖️LaneDefaults` (per-lane,
// per-turn, threaded through every `GuestRuntime::execute_turn`/`step_job` call) replaces the single
// process-wide constant this used to be.


//#region 🔧️SharedWasmtimeEngine
/// 🚧️ Staged for packet `B1-host-native` (`📓️design-runtime.md` §2, `//#region 🦀️WasmtimeRuntime`).
/// The blocked pieces — the `GuestRuntime` trait, the `WasmtimeRuntime` impl that closes over it,
/// `ShardLoop`, `MockGuestRuntime`, and the deletion of `WasmPluginRuntime`/`ExtensionRuntime`/both
/// `ProgramSupervisorState`s/`PLUGIN_FUEL_BUDGET` — all need `RuntimeActorId` (packet `A1-actor`,
/// crate `semio-framework-actor`, not yet a workspace member) and `Effect`/`Event`/`Budget`/
/// `TurnResult`/`JobBudget`/`JobStep`/`TurnFault` (packet `A3-kernel-types`, not yet landed in
/// `🎠️kernel/🦀️.rs`). This region holds only the four pieces of §2 that do NOT depend on
/// either: the one-per-process `Engine` (pooling allocator + on-demand fallback + fuel/epoch config),
/// demand-driven epoch deadlines, a generic per-store `ResourceLimiter`, and the compiled-artifact cache.
/// Wiring plan once A1/A3 land is in `📓️terra-B1-host-native-report.md`.
/// ⚙️ Knobs for [`build_shared_engine`] — mirrors §2's pooling-allocator list verbatim. Plain fields
/// rather than a `Budget`-derived config (A3's type) so this compiles today; `WasmtimeRuntime` will
/// build one from `Budget` fields with a one-line call once A3 lands.
#[derive(Clone, Copy, Debug)]
pub struct SharedEngineConfig {
    pub total_component_instances: u32,
    pub max_memory_bytes: usize,
    pub linear_memory_keep_resident_bytes: usize,
    pub force_on_demand: bool,
    /// ⛽️ Whether guest code is compiled with fuel metering, so a finite `Budget.fuel` can cut a
    /// turn. Metering instruments every block of every guest: measured 2026-09-24 (ticket 26/09/23
    /// slice G6) the `🀄️wfc` genesis solve ran 35.9–53.9 s unmetered against 79.6–84.1 s metered,
    /// back to back in one process kind. A process whose every budget disarms fuel (`u64::MAX`) and
    /// bounds guest calls by epoch deadline alone — the semio MCP gateway — builds its engine
    /// unmetered; its compiled code lives under its own engine-config hash.
    pub fuel_metering: bool,
}

/// 🧩️ Core instances (and memories/tables) the pooling allocator must reserve per component
/// instance. A `wasm32-wasip2` component is a graph of core modules — guest module, the WASI
/// adapter, whatever `wit-bindgen` composes — so the core pools must be a multiple of the component
/// pool, never equal to it. Measured need is small; the headroom is cheap because these pools are
/// virtual-address reservations, and `build_shared_engine` already falls back to on-demand
/// allocation if the host refuses the reservation.
const CORE_INSTANCES_PER_COMPONENT: u32 = 8;

/// 💾️ Linear memories per component instance. Kept far below [`CORE_INSTANCES_PER_COMPONENT`] on
/// purpose: the memory pool is a VIRTUAL ADDRESS RESERVATION of `total_memories × max_memory_size`,
/// so multiplying it by the core-instance factor asks the OS for tens of terabytes and the pooling
/// allocator is refused outright — `build_shared_engine` then silently falls back to on-demand and
/// the whole pooling design is lost. Core-instance slots are cheap bookkeeping; memory slots are not.
const MEMORIES_PER_COMPONENT: u32 = 1;

/// 🪑️ Tables per component instance — bookkeeping like core instances, not address space.
const TABLES_PER_COMPONENT: u32 = 4;

impl Default for SharedEngineConfig {
    fn default() -> Self {
        Self { total_component_instances: 4096, max_memory_bytes: 512 * 1024 * 1024, linear_memory_keep_resident_bytes: 2 * 1024 * 1024, force_on_demand: false, fuel_metering: true }
    }
}

/// 🐎️ ONE shared `Engine` for the process (§2). `epoch_interruption` is always enabled and
/// `consume_fuel` follows [`SharedEngineConfig::fuel_metering`], so every `Store` built on it MUST
/// install [`EpochDeadlines::install`]'s callback and arm [`EpochDeadlines::arm`] (and, when metered, `set_fuel`) before each wasm call — the bug this replaces (`WasmPluginRuntime::build_engine`/`prepare_call` below)
/// sets fuel once and an epoch deadline of `u64::MAX`, so nothing is ever enforced. Falls back to
/// `OnDemand` allocation — the fallback knob §2 asks for — if the pooling allocator rejects `cfg` on
/// this host (e.g. insufficient virtual address space, or a hardened container); the returned `bool`
/// reports which strategy actually got used, for logging/metrics.
///
/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): NOT optional. `world
/// actor` imports `host-async`, whose 24 `async func`s only lift/lower under the
/// component-model async ABI; and S7's categorical finding runs the other way too — with
/// this on, a plain sync `func` export becomes uncallable, which is exactly why all seven of
/// the world's exports gained `async` in the same edit. `Config::concurrency_support` (what
/// `Store::run_concurrent` and `StreamReader` need) defaults to `true` and is left alone —
/// wasmtime rejects a Config that enables component-model-async while disabling it.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b): `total_component_instances` alone is
/// not enough — the pooling allocator meters CORE instances, memories and tables from
/// separate pools that each default to 1000, and one component instance consumes several
/// of each (guest module + wasip2 adapter + composed modules). At full scale the bench
/// died with "maximum concurrent limit of 1000 for core instances reached" while the
/// component pool still had thousands free. Sized off the component budget rather than
/// hardcoded, so raising one knob no longer silently leaves the others behind.
///
/// ♻️ The FOURTH sub-pool with its own 1000 default, found only because fixing the third
/// let the bench run far enough to hit it ("maximum concurrent GC heap limit of 1000
/// reached"). Every one of these caps is invisible until the scale exceeds it, so they
/// surface one run at a time; sized off the same component budget as the rest.
async fn build_shared_engine(cfg: SharedEngineConfig) -> Result<(Engine, bool), PluginHostError> {
    let build = |pooling: bool| -> wasmtime::Result<Engine> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.wasm_component_model_async(true);
        config.consume_fuel(cfg.fuel_metering);
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

//#region 🧵️PluginHostWorkerPool
/// 🧵️ Returns the process-wide `WorkerPool` owned by `semio-framework-async`.
/// `ProcessKind::InteractiveNative` — this crate is loaded by both interactive hosts (the
/// renderer) and headless ones (`🏃️run`, the MCP gateway, the `semio-shard` child binary), and
/// `InteractiveNative`'s only cost for a headless caller is reserving one core it was never going to
/// saturate anyway, the same tradeoff `global_worker_pool` already accepted.
///
/// 🎚️ Sizing normally reads `std::thread::available_parallelism()`, but `SEMIO_PLUGIN_HOST_WORKER_COUNT`
/// (read ONCE, at first construction) overrides it — `👶️child/🦀️.rs` sets it to `1` before this
/// pool is ever touched: an out-of-process shard's own `ShardLoop::pump` runs directly on that
/// process's main thread (never submitted to this pool), so the ONLY work this pool ever carries
/// there is the heartbeat sender (below) — sizing it to `available_parallelism()-1`
/// (potentially many cores) would spin up that many OS threads per shard CHILD PROCESS for one
/// sub-millisecond periodic job, multiplying total host thread count by however many shard processes
/// are running. No other caller (the renderer, `🏃️run`, the MCP gateway) sets this variable, so they
/// keep the full-parallelism default.
pub(crate) fn plugin_host_worker_pool() -> WorkerPool {
    let cores = std::env::var("SEMIO_PLUGIN_HOST_WORKER_COUNT").ok().and_then(|value| value.parse::<usize>().ok()).filter(|count| *count > 0).unwrap_or_else(|| std::thread::available_parallelism().map_or(1, std::num::NonZero::get));
    semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores))
}
//#endregion 🧵️PluginHostWorkerPool

//#region ⏲️PeriodicPoolTimer
/// ⏲️ P1f: the shared shape behind a "tick forever" mechanism (`process_transport::StdioTransport`'s
/// heartbeat sender). Each tick is one callback on the
/// pool's timer wheel, run by the worker that fires the wheel, and registers the next deadline from
/// there — waiting owns no worker, and a tick wakes exactly one.
///
/// ⏱️ It used to `submit_at` a lane job per tick, so every tick woke the timer keeper
/// AND a second worker to run the resubmitted job: measured 2026-09-24 (ticket 26/09/23 slice G6)
/// in a solving semio MCP process, all nine pool workers each spent ~5.7 % of a core on it.
///
/// Heartbeat senders are not process singletons, so retaining their waits in the pool's timer wheel
/// prevents any number of concurrent tickers from consuming worker permits.
struct PeriodicPoolTimer {
    stop: Arc<AtomicBool>,
}

impl PeriodicPoolTimer {
    /// ▶️ Registers the first tick. `tick` must be quick and non-blocking — it runs inside the pool's
    /// timer firing — as the epoch/heartbeat bodies below are: a single atomic increment or a short,
    /// already-buffered write.
    /// 🚫️async: E1-adjacent — no suspension point of its own (only REGISTERS the tick). See R9.
    fn start(pool: &WorkerPool, interval_ms: u64, tick: impl FnMut() -> bool + Send + 'static) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        Self::schedule(pool, interval_ms, Arc::new(Mutex::new(tick)), Arc::clone(&stop));
        Self { stop }
    }

    /// 🔁️ One timer registration = one `tick()`, one next registration.
    fn schedule(pool: &WorkerPool, interval_ms: u64, tick: Arc<Mutex<dyn FnMut() -> bool + Send>>, stop: Arc<AtomicBool>) {
        let driver_pool = pool.clone();
        pool.callback_at(pool.now_ms().saturating_add(interval_ms), move || {
            if stop.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }
            let should_continue = (tick.lock().unwrap_or_else(std::sync::PoisonError::into_inner))();
            if should_continue {
                Self::schedule(&driver_pool, interval_ms, tick, stop);
            }
        });
    }
}

impl Drop for PeriodicPoolTimer {
    /// 🛑️ Requests the next scheduled callback to stop rather than register another deadline.
    fn drop(&mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
//#endregion ⏲️PeriodicPoolTimer

//#region ⏱️EpochDeadlines
/// ⏱️ Demand-driven epoch interruption for ONE shared `Engine`: the engine's epoch advances only when
/// an armed guest call's wall deadline arrives, and while a timesliced call runs. Every `Store`
/// installs [`EpochDeadlines::install`]'s callback, which answers `Interrupt` once the store's own
/// deadline ([`EpochDeadlineCell`]) has passed and `Continue(1)` otherwise, so an advance meant for
/// another store costs this one a single callback. With no call armed no timer is registered, and an
/// idle engine costs nothing.
///
/// ⏱️ It replaces a 1 ms periodic ticker that ran whenever an engine existed — measured 2026-09-24
/// (ticket 26/09/23 slice G6) at ~8 % of a core in an idle semio MCP gateway, and 1 000 wake-ups a
/// second in every guest call even though a deadline is reached once or never.
#[derive(Clone)]
pub(crate) struct EpochDeadlines {
    inner: Arc<EpochDeadlinesInner>,
}

struct EpochDeadlinesInner {
    engine: Engine,
    pool: WorkerPool,
    demands: Mutex<EpochDemands>,
}

/// 📋️ Armed deadlines as (pool ms, demand id), running timeslices, the instants a timer is already
/// registered for, and the latest instant every deadline up to which has advanced the epoch.
#[derive(Default)]
struct EpochDemands {
    deadlines: BTreeSet<(u64, u64)>,
    timeslices: usize,
    armed: BTreeSet<u64>,
    advanced_through: u64,
    advances: u64,
    next_id: u64,
}

/// 🎚️ A store's current wall deadline on the pool clock, read by its epoch callback.
#[derive(Debug)]
pub(crate) struct EpochDeadlineCell(std::sync::atomic::AtomicU64,std::sync::atomic::AtomicU64,Option<fn()->Option<u64>>);

impl Default for EpochDeadlineCell {
    fn default() -> Self {
        Self(std::sync::atomic::AtomicU64::new(u64::MAX),std::sync::atomic::AtomicU64::new(u64::MAX),None)
    }
}

/// 🔒️ One armed guest call's demand on the epoch; dropping it withdraws the demand.
pub(crate) struct EpochDemand {
    inner: Arc<EpochDeadlinesInner>,
    kind: EpochDemandKind,
}

enum EpochDemandKind {
    Deadline((u64, u64)),
    Timeslice,
}

impl EpochDeadlines {
    /// ▶️ `pool` is normally [`plugin_host_worker_pool`]; a test passes its own deterministic pool.
    pub fn new(engine: &Engine, pool: &WorkerPool) -> Self {
        Self { inner: Arc::new(EpochDeadlinesInner { engine: engine.clone(), pool: pool.clone(), demands: Mutex::new(EpochDemands::default()) }) }
    }

    /// 🪝️ Installs the deadline callback on a store built on this engine. Once per store: wasmtime
    /// holds exactly one callback, and it reads `cell` fresh on every advance.
    pub fn install<T: 'static>(&self, store: &mut Store<T>, cell: Arc<EpochDeadlineCell>) {
        let pool = self.inner.pool.clone();
        store.epoch_deadline_callback(move |_| Ok(if pool.now_ms() >= cell.0.load(std::sync::atomic::Ordering::Acquire)||cell.2.is_some_and(|clock|clock().is_none_or(|now|now>=cell.1.load(std::sync::atomic::Ordering::Acquire))) { wasmtime::UpdateDeadline::Interrupt } else { wasmtime::UpdateDeadline::Continue(1) }));
        store.set_epoch_deadline(1);
    }

    /// ⏰️ Arms one guest call: `store` is interrupted once `after_ms` of wall time have passed, for
    /// as long as the returned demand lives.
    pub fn arm<T>(&self, store: &mut Store<T>, cell: &EpochDeadlineCell, after_ms: u64) -> EpochDemand {
        cell.1.store(u64::MAX,std::sync::atomic::Ordering::Release);
        let deadline = self.inner.pool.now_ms().saturating_add(after_ms);
        cell.0.store(deadline, std::sync::atomic::Ordering::Release);
        store.set_epoch_deadline(1);
        let mut demands = self.inner.demands.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = (deadline, demands.next_id);
        demands.next_id += 1;
        demands.deadlines.insert(key);
        EpochDeadlinesInner::rearm(&self.inner, &mut demands);
        EpochDemand { inner: Arc::clone(&self.inner), kind: EpochDemandKind::Deadline(key) }
    }

    /// 🕰️ Arms the unchanged absolute caller deadline while translating its wake to the pool clock.
    pub fn arm_original<T>(&self,store:&mut Store<T>,cell:&EpochDeadlineCell,deadline_us:u64)->Result<EpochDemand,TurnFault>{
        let clock=cell.2.ok_or_else(||identity_turn_fault(crate::operation_authority::refusal(4)))?;
        let remaining=remaining_original_deadline_us(clock(),deadline_us).map_err(|error|match error{OriginalDeadlineError::MissingClock=>identity_turn_fault(crate::operation_authority::refusal(4)),OriginalDeadlineError::Expired=>TurnFault::DeadlineExceeded})?;
        let demand=self.arm(store,cell,remaining.div_ceil(1_000));
        cell.1.store(deadline_us,std::sync::atomic::Ordering::Release);
        Ok(demand)
    }

    /// 🔁️ Arms a timesliced call: the epoch advances every millisecond while the demand lives, for a
    /// store whose callback yields between slices.
    pub fn arm_timeslice(&self) -> EpochDemand {
        let mut demands = self.inner.demands.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        demands.timeslices += 1;
        EpochDeadlinesInner::rearm(&self.inner, &mut demands);
        EpochDemand { inner: Arc::clone(&self.inner), kind: EpochDemandKind::Timeslice }
    }

    /// 🔎️ Timers registered on the pool for this engine. A withdrawn deadline's timer stays
    /// registered until its instant and then fires once without advancing anything.
    pub fn armed_timers(&self) -> usize {
        self.inner.demands.lock().unwrap_or_else(std::sync::PoisonError::into_inner).armed.len()
    }

    /// 🔢️ How many times this engine's epoch has advanced.
    pub fn advances(&self) -> u64 {
        self.inner.demands.lock().unwrap_or_else(std::sync::PoisonError::into_inner).advances
    }
}

impl EpochDeadlinesInner {
    /// 📅️ Registers a timer for the next instant the epoch must advance, unless one no later is
    /// already registered: the first deadline not yet advanced through, or the next slice.
    fn rearm(inner: &Arc<Self>, demands: &mut EpochDemands) {
        let now = inner.pool.now_ms();
        let deadline = demands.deadlines.iter().map(|(at, _)| *at).find(|at| *at > demands.advanced_through);
        let slice = (demands.timeslices > 0).then(|| now.saturating_add(1));
        let Some(next) = deadline.into_iter().chain(slice).min() else { return };
        if demands.armed.first().is_some_and(|first| *first <= next) {
            return;
        }
        demands.armed.insert(next);
        let weak = Arc::downgrade(inner);
        inner.pool.callback_at(next, move || {
            if let Some(inner) = weak.upgrade() {
                Self::fire(&inner, next);
            }
        });
    }

    fn fire(inner: &Arc<Self>, at: u64) {
        let mut demands = inner.demands.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        demands.armed.remove(&at);
        let now = inner.pool.now_ms();
        let due = demands.timeslices > 0 || demands.deadlines.iter().any(|(deadline, _)| *deadline > demands.advanced_through && *deadline <= now);
        if due {
            inner.engine.increment_epoch();
            demands.advances += 1;
            demands.advanced_through = demands.advanced_through.max(now);
        }
        Self::rearm(inner, &mut demands);
    }
}

impl Drop for EpochDemand {
    fn drop(&mut self) {
        let mut demands = self.inner.demands.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match self.kind {
            EpochDemandKind::Deadline(key) => {
                demands.deadlines.remove(&key);
            }
            EpochDemandKind::Timeslice => demands.timeslices -= 1,
        }
    }
}
//#endregion ⏱️EpochDeadlines

#[path="🧬️component-codec/🎟️retained-turn/🦀️.rs"]
mod retained_turn_wire;

/// 📏️ Generic per-store `ResourceLimiter` (§2: "a `ResourceLimiter` per store bounding
/// memory/tables/instances against the budget"). Plain numeric bounds rather than a `Budget`-typed
/// constructor — `Budget` is A3's type, not yet landed — so `WasmtimeRuntime` builds one from
/// `budget.memory_bytes` etc. with a one-line call once it lands.
pub(crate) struct BudgetLimiter {
    pub max_memory_bytes: usize,
    /// 📈️ High-water mark of the guest's linear memory, in bytes — the host-side twin of the
    /// guest's own `semio_framework_trace::guest_linear_memory_bytes` (`memory.size`). Wasm memory
    /// never shrinks, so this equals the CURRENT size and every `memory.grow` updates it. It is
    /// what turns "the actor trapped on a failed allocation" into a per-turn growth curve — see
    /// `📓️poll-task-leak-2026-09-10.md`.
    pub peak_memory_bytes: usize,
    pub max_table_elements: u32,
    pub max_instances: usize,
    pub max_tables: usize,
    pub max_memories: usize,
}

/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b): `max_instances` was `1`, `max_tables`/
/// `max_memories` `8`. Those are *core-module* numbers, and this limiter guards a **component**
/// store: the component model instantiates one core instance per module in the component graph
/// (guest module + the `wasm32-wasip2` adapter + whatever else `wit-bindgen` composes), so the very
/// first real plugin died at its SECOND core instance with `resource limit exceeded: instance count
/// too high at 2`. Every budget in the scale bench except the pure-JS registry parse failed on that
/// one line — the runtime could not instantiate ANY real component, and nothing had noticed because
/// the only path that had ever instantiated one (the `🖨️describe` emitter) installs no limiter at
/// all. Sized to bound a hostile component while leaving ordinary composition room; re-measure
/// rather than re-guess if a legitimate component ever trips it.
impl Default for BudgetLimiter {
    fn default() -> Self {
        Self { max_memory_bytes: 512 * 1024 * 1024, peak_memory_bytes: 0, max_table_elements: 100_000, max_instances: 256, max_tables: 128, max_memories: 128 }
    }
}

impl ResourceLimiter for BudgetLimiter {
    fn memory_growing(&mut self, _current: usize, desired: usize, _maximum: Option<usize>) -> wasmtime::Result<bool> {
        let admitted = desired <= self.max_memory_bytes;
        if admitted {
            self.peak_memory_bytes = self.peak_memory_bytes.max(desired);
        }
        Ok(admitted)
    }

    fn table_growing(&mut self, _current: usize, desired: usize, _maximum: Option<usize>) -> wasmtime::Result<bool> {
        Ok(desired <= self.max_table_elements as usize)
    }

    fn instances(&self) -> usize {
        self.max_instances
    }

    fn tables(&self) -> usize {
        self.max_tables
    }

    fn memories(&self) -> usize {
        self.max_memories
    }
}

/// 💾️ Compiled-artifact cache (§2): `Component::serialize`/`deserialize_file` keyed by
/// `<engine-config-hash>/<package-hash>.cwasm` under `~/.semio/cache/wasmtime/`. Both hashes are
/// plain `[u8; 32]` (blake3) rather than A1's `PackageHash` newtype — `compiled_cache_path` becomes a
/// one-line call with `package_hash.0` once that crate is a workspace member.
pub async fn default_compiled_cache_root() -> PathBuf {
    let home = std::env::var("SEMIO_HOME").or_else(|_| std::env::var("HOME")).or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".semio").join("cache").join("wasmtime")
}

pub async fn shared_engine_config_hash(cfg: &SharedEngineConfig, pooling_active: bool) -> [u8; 32] {
    let descriptor = format!("wasmtime=47.0.3;component_model=1;fuel={};epoch=1;pooling={};instances={};max_memory={};keep_resident={}", u8::from(cfg.fuel_metering), pooling_active, cfg.total_component_instances, cfg.max_memory_bytes, cfg.linear_memory_keep_resident_bytes);
    *semio_framework_hash::hash(descriptor.as_bytes()).as_bytes()
}

// 🚫️async: E1 pure formatter consumed by `impl Debug for CompiledHandle` (external trait,
// sync-only) — R9.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub async fn compiled_cache_path(cache_root: &Path, engine_config_hash: &[u8; 32], package_hash: &[u8; 32]) -> PathBuf {
    cache_root.join(hex_encode(engine_config_hash)).join(format!("{}.cwasm", hex_encode(package_hash)))
}

/// ⚠️ SAFETY: `deserialize_file` trusts the file completely (wasmtime docs). Callers MUST only point
/// this at paths written via [`store_compiled_component`] by an engine of the SAME config (so the
/// same compiled ABI) — this process's own, or an isolated compile process that rebuilt it from
/// [`SharedEngineConfig::to_isolated_arg`] — a hostile or stale `.cwasm` is a sandbox escape, not a
/// cache-miss. Any I/O or deserialize error is treated as a cache miss (`None`), never surfaced as a
/// fault: recompiling from the original component bytes is always the safe fallback.
async fn load_compiled_component(engine: &Engine, path: &Path) -> Option<Component> {
    if !path.exists() {
        return None;
    }
    unsafe { Component::deserialize_file(engine, path).ok() }
}

/// 💾️ Writes the compiled code under a scratch name owned by this process and renames it into
/// place, so a reader never sees a partial `.cwasm` and a writer killed midway leaves only its own
/// [`compiled_cache_scratch_path`] behind.
async fn store_compiled_component(component: &Component, path: &Path) -> std::io::Result<()> {
    write_compiled_cache_entry(&component.serialize().map_err(|error| std::io::Error::other(error.to_string()))?, path)
}

/// 💾️ The one atomic write every `.cwasm` takes: scratch name of this process, then rename into place.
fn write_compiled_cache_entry(bytes: &[u8], path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let scratch = compiled_cache_scratch_path(path, std::process::id());
    std::fs::write(&scratch, bytes)?;
    std::fs::rename(&scratch, path)
}

/// 🧻️ Where process `pid` stages a `.cwasm` before renaming it to `path`.
pub fn compiled_cache_scratch_path(path: &Path, pid: u32) -> PathBuf {
    let mut name = path.file_name().map(std::ffi::OsStr::to_os_string).unwrap_or_default();
    name.push(format!(".{pid}.scratch"));
    path.with_file_name(name)
}

//#region 🧊️IsolatedCompile
/// 🧊️ Compiling a component is the one guest-preparation step with no interruption point inside it:
/// cranelift runs to its end once started, on every core it can get. A host that must be able to
/// abandon a compile runs it in a process of its own — [`compile_component_isolated`] is that
/// process's whole body — and kills the process, which returns every thread and byte the compile
/// held. The compiled code reaches the host through the same on-disk cache every other load reads,
/// written atomically, so a killed compile leaves no entry behind.
pub struct IsolatedCompileTarget {
    /// 🗂️ The `.cwasm` the isolated process writes and the host then loads.
    pub cache_path: PathBuf,
    /// ⚙️ The host engine's configuration, as [`SharedEngineConfig::to_isolated_arg`] encodes it.
    pub engine: String,
}

impl SharedEngineConfig {
    /// ⚙️ One argument an isolated compile process rebuilds exactly this engine from.
    pub fn to_isolated_arg(&self) -> String {
        format!("instances={};max-memory={};keep-resident={};on-demand={};fuel={}", self.total_component_instances, self.max_memory_bytes, self.linear_memory_keep_resident_bytes, u8::from(self.force_on_demand), u8::from(self.fuel_metering))
    }

    pub fn from_isolated_arg(arg: &str) -> Result<Self, PluginHostError> {
        let mut cfg = Self::default();
        for field in arg.split(';') {
            let (key, value) = field.split_once('=').ok_or_else(|| PluginHostError::Plugin(format!("isolated compile engine field `{field}` is not key=value")))?;
            let number = value.parse::<u64>().map_err(|error| PluginHostError::Plugin(format!("isolated compile engine field `{key}`: {error}")))?;
            match key {
                "instances" => cfg.total_component_instances = u32::try_from(number).map_err(|error| PluginHostError::Plugin(error.to_string()))?,
                "max-memory" => cfg.max_memory_bytes = usize::try_from(number).map_err(|error| PluginHostError::Plugin(error.to_string()))?,
                "keep-resident" => cfg.linear_memory_keep_resident_bytes = usize::try_from(number).map_err(|error| PluginHostError::Plugin(error.to_string()))?,
                "on-demand" => cfg.force_on_demand = number != 0,
                "fuel" => cfg.fuel_metering = number != 0,
                other => return Err(PluginHostError::Plugin(format!("isolated compile engine field `{other}` is unknown"))),
            }
        }
        Ok(cfg)
    }
}

/// 🧊️ The whole body of an isolated compile process: rebuild the host's engine from `engine`,
/// compile `bytes` straight to its serialized code ([`Engine::precompile_component`]) and store it at
/// `cache_path`. The code is never published as executable memory here, so the process registers — and on
/// its way out tears down — no unwind frame: macOS deregisters each frame by a linear scan of every frame,
/// which kept a finished 330 MB compile (stdio) stuck in `__deregister_frame` for more than 15 minutes
/// after its `.cwasm` was written, while the host waited for its exit (ticket 26/09/23, G12 session 14c).
pub fn compile_component_isolated(engine: &str, bytes: &[u8], cache_path: &Path) -> Result<(), PluginHostError> {
    let cfg = SharedEngineConfig::from_isolated_arg(engine)?;
    let (engine, _pooling_active) = semio_framework_async::block_on(build_shared_engine(cfg))?;
    let compiled = engine.precompile_component(bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
    write_compiled_cache_entry(&compiled, cache_path).map_err(|error| PluginHostError::Plugin(format!("storing {}: {error}", cache_path.display())))
}
//#endregion 🧊️IsolatedCompile

#[cfg(test)]
include!("🧪️tests/🔬️standalone/🦀️.rs");

#[cfg(test)]
#[path = "🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs"]
mod shared_wasmtime_engine_tests;
//#endregion 🔧️SharedWasmtimeEngine

#[path = "🎭️guest-runtime/🦀️.rs"]
mod guest_runtime;
pub use guest_runtime::*;
#[path = "🔀️post-turn-relay/🦀️.rs"]
mod post_turn_relay;
pub use post_turn_relay::*;
#[path = "🧭️routers/🦀️.rs"]
mod routers;
pub use routers::*;

/// 🧪️ Only `host_transaction_coordinator_tests`' `FakeCluster` fixture calls this now — the old
/// `HostState`'s WIT `host.*` import handlers (which encoded EVERY fault this way) are gone with
/// `WasmPluginRuntime`. Kept (not deleted) because that fixture still needs it; `cfg_attr` silences
/// the resulting "never used" warning on a plain (non-test) `--lib` build without hiding a REAL
/// dead-code case under a blanket `#[allow]`.
//#region 📈️RuntimeMetricsPublisher
/// 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): native-side sampling + 2Hz cadence gate for bus
/// topic `os.runtime.metrics` — `semio_framework_actor::KernelMetrics`'s own doc comment: "the host
/// publishes this as bus topic `os.runtime.metrics` at 2Hz." Wraps `Kernel::runtime_metrics_snapshot`
/// (pure, clock-injected) and overlays the one field the pure crate cannot compute itself:
/// `ShardMetricsSample.metrics.heartbeat_age_ms`, from each shard's own
/// `ShardTransport::heartbeat()` reading (`🎭️actor/🦀️.rs`'s `ShardTransport` trait — this
/// file owns the transports, the pure crate does not).
///
/// 🚧️ GAP (see `📓️terra-T1-report.md` `## honest gaps`): nothing in this codebase yet drives a live
/// `semio_framework_actor::Kernel` on a native thread — `Kernel::new` has zero call sites outside
/// that crate's own tests and its wasm-only `KernelHost` glue (`grep -rn "Kernel::new(" --include
/// "*.rs" .`, checked at packet start). This publisher is therefore wired as the exact call the
/// future kernel-thread owner (H1-H4) makes each pump; it is unit-tested end-to-end against a real
/// `Kernel`, but nothing currently invokes it at runtime, and no subscriber-fanout for
/// `Effect::PublishEvent`/`Effect::Subscribe` exists anywhere in this file to hand the payload to
/// (same `grep` found zero delivery call sites, native or web).
pub struct RuntimeMetricsPublisher {
    last_published_ms: Option<u64>,
}

impl RuntimeMetricsPublisher {
    /// 🚫️async: E1 pure constructor consumed by `impl Default` (external trait, sync-only) — R9.
    pub fn new() -> Self {
        Self { last_published_ms: None }
    }

    /// 📡️ Samples `kernel` and returns the pack-encoded `os.runtime.metrics` payload
    /// (`semio_framework_actor::RuntimeMetricsSnapshot::pack_encode`) when the 2Hz interval elapsed,
    /// else `None`. `shard_heartbeats` maps each live `ShardId` to its transport's last
    /// `ShardTransport::heartbeat()` reading — the overlay `Kernel::shard_metrics_samples` cannot do
    /// itself (see this struct's doc comment).
    ///
    /// 🚫️async: R10 residue shape 2 — a future is consumed by a single `.await`; awaited once
    /// here instead of once inside the loop and again (moved) at `pack_encode` below.
    pub async fn maybe_sample(&mut self, kernel: &semio_framework_actor::Kernel, now_ms: u64, shard_heartbeats: &HashMap<semio_framework_actor::ShardId, u64>) -> Option<Vec<u8>> {
        if !semio_framework_actor::runtime_metrics_due(self.last_published_ms, now_ms).await {
            return None;
        }
        self.last_published_ms = Some(now_ms);
        let mut snapshot = kernel.runtime_metrics_snapshot(now_ms).await;
        for shard in &mut snapshot.shards {
            if let Some(&last_beat) = shard_heartbeats.get(&shard.shard) {
                shard.metrics.heartbeat_age_ms = now_ms.saturating_sub(last_beat) as u32;
            }
        }
        let mut out = Vec::new();
        snapshot.pack_encode(&mut out).await;
        Some(out)
    }
}

impl Default for RuntimeMetricsPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️runtime-metrics-publisher/🦀️.rs"]
mod runtime_metrics_publisher_tests;
//#endregion 📈️RuntimeMetricsPublisher

//#region 🔖️ArtifactSession
/// 📦️ Opaque pack triple for one artifact lane (document / config / draft). Typed `ArtifactStore`
/// still lives in guest `VcsArtifactApp` until `AppCommand::PureCommand` + host `dispatch` land;
/// the host already mirrors these bytes as the authority seam.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionLanePack {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
    pub ops: String,
    /// 🧾 Binary op packs from guest `AppFrame::Emit` awaiting host typed apply.
    pub pending_binary_ops: Vec<u8>,
}

impl SessionLanePack {
    /// 🏗️ Empty lane (no snapshot yet).
    pub async fn empty() -> Self {
        Self::default()
    }

    /// 🏗️ From a full pack+spr+ops snapshot.
    pub async fn from_files(pack: Vec<u8>, spr: Vec<u8>, ops: String) -> Self {
        Self { pack, spr, ops, pending_binary_ops: Vec::new() }
    }

    /// 📥 Replaces this lane's opaque snapshot.
    pub async fn adopt(&mut self, pack: Vec<u8>, spr: Vec<u8>, ops: String) {
        self.pack = pack;
        self.spr = spr;
        self.ops = ops;
        self.pending_binary_ops.clear();
    }

    /// 🧾 Applies guest `AppFrame::Emit` op bytes onto this lane via `ArtifactCodec` when `schema` is set.
    pub async fn apply_emit_ops(&mut self, schema: Option<&str>, ops: Vec<u8>, identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        if ops.is_empty() {
            return;
        }
        let Some(schema) = schema.filter(|s| !s.is_empty()) else {
            self.pending_binary_ops = ops;
            return;
        };
        let Ok(Some(codec)) = store::os_store::codec_registry::document_codec(schema).await else {
            self.pending_binary_ops = ops;
            return;
        };
        if self.pack.is_empty() && self.spr.is_empty() {
            self.pending_binary_ops = ops;
            return;
        }
        match (codec.apply_ops_binary)(&self.pack, &self.spr, &ops,identity).await {
            Ok((pack, spr, ops_text)) => {
                self.pack = pack;
                self.spr = spr;
                self.ops = ops_text;
                self.pending_binary_ops.clear();
            }
            Err(_error) => {
                self.pending_binary_ops = ops;
            }
        }
    }

    /// 📭 True when no pack bytes have been adopted.
    pub async fn is_empty(&self) -> bool {
        self.pack.is_empty() && self.spr.is_empty()
    }
}

/// 🧾 Host-owned per-instance document authority: opaque document/config/draft packs plus generation
/// counters. The plugin-wide {@link EngineCache} lives on `HostState` (WIT `engine-derive`/`engine-read`
/// have no instance id). Typed stores and the command log remain guest-side until PureCommand apply.
#[derive(Clone, Debug, Default)]
pub struct ArtifactSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub artifact_schema: Option<String>,
    pub config_schema: Option<String>,
    pub draft_schema: Option<String>,
    pub document: SessionLanePack,
    pub config: SessionLanePack,
    pub draft: SessionLanePack,
    /// ⚖️ This session's own view of its local/authority `MergePolicy`, mirrored from the last
    /// `AppCommand::SetMergePolicy` this session's `exchange` calls have sent (`Normal` — the wire
    /// default — until the first one). See `WasmPluginRuntime::hello`/`set_merge_policy`.
    pub merge_policy: protocol::MergePolicy,
    /// 📨 The most recent dispatch's `protocol::MutationMessage`s — mirrored from whichever of
    /// `AppFrame::Invocation.messages` (success) or `AppFrame::Error.report` (rejected) last carried
    /// a non-empty packed `protocol::DispatchReport`. Ticket 26/08/16/MUTATION-OUTCOMES-MERGE-
    /// POLICIES-AND-FIRST-CLASS-CONFLICTS (C9): mirrors `diagnostics`'s own DSL-diagnostics
    /// surfacing convention exactly (see `AppFrame::Emit`'s `diagnostics` field, `push_app_fault`'s
    /// `fault` field) rather than a second event/callback mechanism.
    pub last_dispatch_messages: Vec<protocol::MutationMessage>,
    /// 🔀 The most recent unsolicited `AppFrame::MergeReport`, if this session has seen one.
    pub last_merge_report: Option<protocol::MergeReport>,
    /// ⚔️ This artifact's currently open conflicts, mirrored from the most recent
    /// `AppFrame::Conflicts` (pushed unsolicited after every ingest, and the reply to
    /// `AppCommand::ReadConflicts`).
    pub open_conflicts: Vec<protocol::Conflict>,
}

impl ArtifactSession {
    /// 🏗️ Empty per-instance session (no packs yet).
    pub async fn new() -> Self {
        Self::default()
    }
}

//#endregion 🔖️ArtifactSession

//#region 🔖️MutationReports
/// 🧾 Decodes a packed `protocol::DispatchReport` off the wire — the shape `AppFrame::Invocation`'s
/// trailing `messages` and `AppFrame::Error`'s trailing `report` both carry (contract-freeze.md
/// §C8/C9). Empty bytes (a pre-CHANNEL_VERSION-11 peer, or a frame that legitimately carries none)
/// decode to a message-free report under this session's own tracked policy rather than erroring —
/// callers that need to distinguish "no report" from "empty report" should check `bytes.is_empty()`
/// themselves first.
pub async fn decode_dispatch_report(bytes: &[u8]) -> Result<protocol::DispatchReport, PluginHostError> {
    if bytes.is_empty() {
        return Ok(protocol::DispatchReport { policy: protocol::MergePolicy::default(), worst: None, messages: Vec::new() });
    }
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

/// 🔀 Decodes a packed `protocol::MergeReport` — `AppFrame::MergeReport.report`.
pub async fn decode_merge_report(bytes: &[u8]) -> Result<protocol::MergeReport, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

/// ⚔️ Decodes a packed `Vec<protocol::Conflict>` — `AppFrame::Conflicts.conflicts`.
pub async fn decode_conflicts(bytes: &[u8]) -> Result<Vec<protocol::Conflict>, PluginHostError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

//#endregion 🔖️MutationReports


#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/⚠️refusal/🦀️.rs"]
mod sqlite_refusal_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

/// 🛂️ Generated concurrent imports forward the real inline admission slot to the original caller pump.
impl actor_bindings::semio::framework::pure::HostWithStore<ActorHostState> for wasmtime::component::HasSelf<ActorHostState> {
 async fn operation_begin(accessor:&Accessor<ActorHostState,Self>)->Result<u64,u32>{match operation::wasmtime::request(accessor,operation::slot::OperationRequest::Begin).await{operation::slot::OperationReply::Begin(value)=>value,_=>Err(4)}}
 async fn operation_progress(accessor:&Accessor<ActorHostState,Self>,completed:u64,total:u64,owned:u64)->u32{match operation::wasmtime::request(accessor,operation::slot::OperationRequest::Progress{completed,total,owned}).await{operation::slot::OperationReply::Code(value)=>value,_=>4}}
 async fn operation_allocation(accessor:&Accessor<ActorHostState,Self>,bytes:u64,owned:u64,next:u64,maximum:u64)->u32{match operation::wasmtime::request(accessor,operation::slot::OperationRequest::Allocation{bytes,owned,next,maximum}).await{operation::slot::OperationReply::Code(value)=>value,_=>4}}
 async fn operation_reserve_return(accessor:&Accessor<ActorHostState,Self>,kind:actor_bindings::semio::framework::pure::OperationReturnAllocation,count:u64)->u32{match operation::wasmtime::request(accessor,operation::slot::OperationRequest::ReserveReturn{kind:operation::wasmtime::allocation_kind(kind),count}).await{operation::slot::OperationReply::Code(value)=>value,_=>4}}
 async fn operation_finish(accessor:&Accessor<ActorHostState,Self>,owned:u64)->u32{match operation::wasmtime::request(accessor,operation::slot::OperationRequest::Finish{owned}).await{operation::slot::OperationReply::Code(value)=>value,_=>4}}
}
