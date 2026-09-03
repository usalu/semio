//! 🛡️ Sandboxed wasmtime component plugin host with capability-gated imports.

#[path = "🧵️shard/🦀️.rs"]
pub mod shard;
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
#[path = "⏳️imports/🦀️.rs"]
pub mod imports;
// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-runtime-rewrite): `WasmtimeAsyncRuntime` — one
// pooled `tokio::spawn`ed task per actor, driving `⏳️imports/🦀️.rs`'s host-async import layer against a
// real `Store<AsyncActorHostState>` under `component-model-async`. See that module's own doc.
#[path = "⏳️runtime/🦀️.rs"]
pub mod runtime;

use semio_framework::{
    kernel::{ArtifactHandle, BrokerCapabilityGrant, Budget, CapabilityId, CapabilityRequest, Effect, Event, JobPlacement, MessageEndpoint, RequestId, RequestOutcome, TurnResult, TurnStatus, WindowHandle, WindowKindId},
    DslValue, PluginManifest,
};
use semio_framework_actor::ActorId as RuntimeActorId;
// 🌉️ `pub use`, not a plain `use` — `PackageRef`'s own fields are `PackageId`/`PackageHash`
// (`semio_framework_actor`, packet A1), so a downstream crate that does not itself depend on
// `semio_framework_actor` (e.g. `🏃️run`, which only depends on THIS crate) still needs a path to
// name them in order to construct one, exactly the way `GuestRuntime`/`GuestInstance`/`Budget`
// already reach it through this crate's own public API.
use crate::interpreter::{CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};
pub use semio_framework_actor::{PackageHash, PackageId};
use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
#[cfg(test)]
use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig, ResourceLimiter, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
use semio_framework_value_derive::{FromValue, ToValue};

// 🗑️ `PLUGIN_FUEL_BUDGET` is gone — `design-runtime.md` §1's `Budget.fuel`/`⚖️LaneDefaults` (per-lane,
// per-turn, threaded through every `GuestRuntime::execute_turn`/`step_job` call) replaces the single
// process-wide constant this used to be.

/// 🧯️ Errors from the host's engine/component/call-boundary plumbing (`SharedWasmtimeEngine`,
/// `WasmtimeRuntime`, `PluginInstanceHandle`'s post-turn job dispatch).
#[derive(Debug)]
pub enum PluginHostError {
    Io(std::io::Error),
    Json(String),
    Wasmtime(String),
    Plugin(String),
    IoRouteConflict {
        key: semio_framework::IoKey,
        existing_plugin: String,
        incoming_plugin: String,
    },
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW `(ArtifactDialect,
    /// ArtifactDialect)`-keyed graph's own conflict — separate from `IoRouteConflict` above (OLD
    /// `IoKey`-keyed graph), since the two mechanisms are additive and independently registered.
    IoEntryRouteConflict {
        from: semio_framework::io_schema::ArtifactDialect,
        into: semio_framework::io_schema::ArtifactDialect,
        existing_plugin: String,
        incoming_plugin: String,
    },
    PluginRuntimeConflict {
        plugin_id: String,
    },
    LockPoisoned(&'static str),
}

impl std::fmt::Display for PluginHostError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "io: {error}"),
            Self::Json(error) => write!(formatter, "json: {error}"),
            Self::Wasmtime(message) => write!(formatter, "wasmtime: {message}"),
            Self::Plugin(message) => write!(formatter, "plugin: {message}"),
            Self::IoRouteConflict { key, existing_plugin, incoming_plugin } => write!(formatter, "io route conflict for {key:?}: {existing_plugin} already owns it; {incoming_plugin} cannot replace it"),
            Self::IoEntryRouteConflict { from, into, existing_plugin, incoming_plugin } => write!(formatter, "io entry route conflict for {from:?} -> {into:?}: {existing_plugin} already owns it; {incoming_plugin} cannot replace it"),
            Self::PluginRuntimeConflict { plugin_id } => write!(formatter, "plugin runtime conflict for {plugin_id}"),
            Self::LockPoisoned(name) => write!(formatter, "{name} lock poisoned"),
        }
    }
}

impl std::error::Error for PluginHostError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PluginHostError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

//#endregion ⚠️ Errors

//#region 🔧️SharedWasmtimeEngine
/// 🚧️ Staged for packet `B1-host-native` (`📓️design-runtime.md` §2, `//#region 🦀️WasmtimeRuntime`).
/// The blocked pieces — the `GuestRuntime` trait, the `WasmtimeRuntime` impl that closes over it,
/// `ShardLoop`, `MockGuestRuntime`, and the deletion of `WasmPluginRuntime`/`ExtensionRuntime`/both
/// `ProgramSupervisorState`s/`PLUGIN_FUEL_BUDGET` — all need `RuntimeActorId` (packet `A1-actor`,
/// crate `semio-framework-actor`, not yet a workspace member) and `Effect`/`Event`/`Budget`/
/// `TurnResult`/`JobBudget`/`JobStep`/`TurnFault` (packet `A3-kernel-types`, not yet landed in
/// `🎠️kernel/🦀️.rs`). This region holds only the four pieces of §2 that do NOT depend on
/// either: the one-per-process `Engine` (pooling allocator + on-demand fallback + fuel/epoch config),
/// the 1 ms epoch ticker, a generic per-store `ResourceLimiter`, and the compiled-artifact cache.
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
        Self { total_component_instances: 4096, max_memory_bytes: 512 * 1024 * 1024, linear_memory_keep_resident_bytes: 2 * 1024 * 1024, force_on_demand: false }
    }
}

/// 🐎️ ONE shared `Engine` for the process (§2). `consume_fuel` + `epoch_interruption` are both
/// enabled, so every `Store` built on it MUST call `set_fuel` + `set_epoch_deadline` before its
/// first wasm call — the bug this replaces (`WasmPluginRuntime::build_engine`/`prepare_call` below)
/// sets fuel once and an epoch deadline of `u64::MAX`, so nothing is ever enforced. Falls back to
/// `OnDemand` allocation — the fallback knob §2 asks for — if the pooling allocator rejects `cfg` on
/// this host (e.g. insufficient virtual address space, or a hardened container); the returned `bool`
/// reports which strategy actually got used, for logging/metrics.
pub async fn build_shared_engine(cfg: SharedEngineConfig) -> Result<(Engine, bool), PluginHostError> {
    let build = |pooling: bool| -> wasmtime::Result<Engine> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): NOT optional. `world
        // actor` imports `host-async`, whose 24 `async func`s only lift/lower under the
        // component-model async ABI; and S7's categorical finding runs the other way too — with
        // this on, a plain sync `func` export becomes uncallable, which is exactly why all seven of
        // the world's exports gained `async` in the same edit. `Config::concurrency_support` (what
        // `Store::run_concurrent` and `StreamReader` need) defaults to `true` and is left alone —
        // wasmtime rejects a Config that enables component-model-async while disabling it.
        config.wasm_component_model_async(true);
        config.consume_fuel(true);
        config.epoch_interruption(true);
        if pooling {
            let mut pooling_cfg = PoolingAllocationConfig::default();
            pooling_cfg.total_component_instances(cfg.total_component_instances);
            // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b): `total_component_instances` alone is
            // not enough — the pooling allocator meters CORE instances, memories and tables from
            // separate pools that each default to 1000, and one component instance consumes several
            // of each (guest module + wasip2 adapter + composed modules). At full scale the bench
            // died with "maximum concurrent limit of 1000 for core instances reached" while the
            // component pool still had thousands free. Sized off the component budget rather than
            // hardcoded, so raising one knob no longer silently leaves the others behind.
            pooling_cfg.total_core_instances(cfg.total_component_instances * CORE_INSTANCES_PER_COMPONENT);
            pooling_cfg.total_memories(cfg.total_component_instances * MEMORIES_PER_COMPONENT);
            pooling_cfg.total_tables(cfg.total_component_instances * TABLES_PER_COMPONENT);
            // ♻️ The FOURTH sub-pool with its own 1000 default, found only because fixing the third
            // let the bench run far enough to hit it ("maximum concurrent GC heap limit of 1000
            // reached"). Every one of these caps is invisible until the scale exceeds it, so they
            // surface one run at a time; sized off the same component budget as the rest.
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
/// there is the epoch ticker + heartbeat sender (below) — sizing it to `available_parallelism()-1`
/// (potentially many cores) would spin up that many OS threads per shard CHILD PROCESS for two
/// sub-millisecond periodic jobs, multiplying total host thread count by however many shard processes
/// are running. No other caller (the renderer, `🏃️run`, the MCP gateway) sets this variable, so they
/// keep the full-parallelism default.
pub(crate) fn plugin_host_worker_pool() -> WorkerPool {
    let cores = std::env::var("SEMIO_PLUGIN_HOST_WORKER_COUNT").ok().and_then(|value| value.parse::<usize>().ok()).filter(|count| *count > 0).unwrap_or_else(|| std::thread::available_parallelism().map_or(1, std::num::NonZero::get));
    semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores))
}
//#endregion 🧵️PluginHostWorkerPool

//#region ⏲️PeriodicPoolTimer
/// ⏲️ P1f: the shared shape behind every "tick forever on `Lane::Timer`" mechanism this crate needs
/// (the epoch ticker below; `process_transport::StdioTransport`'s heartbeat sender) — `WorkerPool::
/// submit_at` retains the deadline without occupying a worker, then submits one finite callback and
/// registers the next deadline instead of looping inside a job closure.
///
/// [`EpochTicker`] and the heartbeat sender are not process singletons, so retaining their waits in
/// the pool's timer wheel prevents any number of concurrent tickers from consuming worker permits.
struct PeriodicPoolTimer {
    stop: Arc<AtomicBool>,
}

impl PeriodicPoolTimer {
    /// ▶️ Submits the first tick job on `lane`. `tick` must be quick and non-blocking (it runs ON
    /// the pool worker, between the wait that preceded it and the resubmission that follows) — the
    /// epoch/heartbeat bodies below are a single atomic increment or a short, already-buffered write.
    // 🚫️async: E1-adjacent — no suspension point of its own (only SUBMITS the job; never drives it
    // here). See R9.
    fn start(pool: &WorkerPool, lane: Lane, interval_ms: u64, tick: impl FnMut() -> bool + Send + 'static) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        Self::schedule(pool, lane, interval_ms, Arc::new(Mutex::new(tick)), Arc::clone(&stop));
        Self { stop }
    }

    /// 🔁️ One timer registration = one `tick()`, one next registration. Waiting owns no worker.
    fn schedule(pool: &WorkerPool, lane: Lane, interval_ms: u64, tick: Arc<Mutex<dyn FnMut() -> bool + Send>>, stop: Arc<AtomicBool>) {
        let driver_pool = pool.clone();
        pool.submit_at(
            pool.now_ms().saturating_add(interval_ms),
            lane,
            Box::new(move || {
                if stop.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }
                let should_continue = (tick.lock().unwrap_or_else(std::sync::PoisonError::into_inner))();
                if should_continue {
                    Self::schedule(&driver_pool, lane, interval_ms, tick, stop);
                }
            }),
        );
    }
}

impl Drop for PeriodicPoolTimer {
    /// 🛑️ Requests the next scheduled callback to stop rather than register another deadline.
    fn drop(&mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
//#endregion ⏲️PeriodicPoolTimer

/// ⏱️ Ticks `engine.increment_epoch()` every 1 ms on the shared `WorkerPool`'s `Lane::Timer` (P1f;
/// was a dedicated `"semio-epoch-ticker"` OS thread — see `📓️p1c-actor-shards.md` §3 and
/// `📓️p1f-epoch-transport.md` for the replacement mechanism and what a later, repo-owned WASM
/// interpreter's fuel metering must take over from it). One ticker per shared `Engine`; `Drop`
/// requests the pool job to stop (see [`PeriodicPoolTimer::drop`] — not a synchronous join).
pub struct EpochTicker {
    _driver: PeriodicPoolTimer,
}

/// ⏱️ Matches wasmtime's own epoch granularity assumption (`Store::set_epoch_deadline` counts whole
/// epochs, and every `Budget.deadline_ms` in this codebase is set 1:1 against milliseconds) — see
/// `build_shared_engine`'s `config.epoch_interruption(true)`.
const EPOCH_TICK_INTERVAL_MS: u64 = 1;

impl EpochTicker {
    /// ▶️ `pool` is normally [`plugin_host_worker_pool`] — a caller-supplied pool is accepted (not
    /// just the singleton) so a test can use its own small, deterministic `WorkerPool` instead of
    /// this crate's shared one, matching `semio-framework-os-services`' own `test_pool` convention.
    // 🚫️async: R9 — submitting the job is a synchronous, non-suspending call (`WorkerPool::submit`
    // never awaits); both call sites already use this synchronously.
    pub fn start(engine: &Engine, pool: &WorkerPool) -> Self {
        let engine = engine.clone();
        EpochTicker {
            _driver: PeriodicPoolTimer::start(pool, Lane::Timer, EPOCH_TICK_INTERVAL_MS, move || {
                engine.increment_epoch();
                true
            }),
        }
    }
}

/// 📏️ Generic per-store `ResourceLimiter` (§2: "a `ResourceLimiter` per store bounding
/// memory/tables/instances against the budget"). Plain numeric bounds rather than a `Budget`-typed
/// constructor — `Budget` is A3's type, not yet landed — so `WasmtimeRuntime` builds one from
/// `budget.memory_bytes` etc. with a one-line call once it lands.
pub struct BudgetLimiter {
    pub max_memory_bytes: usize,
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
/// the only path that had ever instantiated one (the `📇️describe` emitter) installs no limiter at
/// all. Sized to bound a hostile component while leaving ordinary composition room; re-measure
/// rather than re-guess if a legitimate component ever trips it.
impl Default for BudgetLimiter {
    fn default() -> Self {
        Self { max_memory_bytes: 512 * 1024 * 1024, max_table_elements: 100_000, max_instances: 256, max_tables: 128, max_memories: 128 }
    }
}

impl ResourceLimiter for BudgetLimiter {
    fn memory_growing(&mut self, _current: usize, desired: usize, _maximum: Option<usize>) -> wasmtime::Result<bool> {
        Ok(desired <= self.max_memory_bytes)
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
    let descriptor = format!("wasmtime=47.0.3;component_model=1;fuel=1;epoch=1;pooling={};instances={};max_memory={};keep_resident={}", pooling_active, cfg.total_component_instances, cfg.max_memory_bytes, cfg.linear_memory_keep_resident_bytes);
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
/// this at paths this process itself wrote via [`store_compiled_component`] with the SAME `engine`
/// (same config, so same compiled ABI) — a hostile or stale `.cwasm` is a sandbox escape, not a
/// cache-miss. Any I/O or deserialize error is treated as a cache miss (`None`), never surfaced as a
/// fault: recompiling from the original component bytes is always the safe fallback.
pub async fn load_compiled_component(engine: &Engine, path: &Path) -> Option<Component> {
    if !path.exists() {
        return None;
    }
    unsafe { Component::deserialize_file(engine, path).ok() }
}

pub async fn store_compiled_component(component: &Component, path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let bytes = component.serialize().map_err(|error| std::io::Error::other(error.to_string()))?;
    std::fs::write(path, bytes)
}

#[cfg(test)]
mod shared_wasmtime_engine_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn build_shared_engine_defaults_to_pooling() {
        let (_engine, pooling_active) = build_shared_engine(SharedEngineConfig::default()).await.expect("pooling engine builds on this host");
        assert!(pooling_active, "pooling allocator should be available in test/CI containers");
    }

    #[semio_framework_async_macros::async_test]
    async fn build_shared_engine_forced_on_demand_reports_on_demand() {
        let cfg = SharedEngineConfig { force_on_demand: true, ..SharedEngineConfig::default() };
        let (_engine, pooling_active) = build_shared_engine(cfg).await.expect("on-demand engine always builds");
        assert!(!pooling_active);
    }

    #[semio_framework_async_macros::async_test]
    async fn epoch_ticker_starts_and_stops_cleanly_around_a_deadline_bearing_store() {
        let (engine, _pooling_active) = build_shared_engine(SharedEngineConfig::default()).await.expect("engine builds");
        let mut store = Store::new(&engine, ());
        store.set_epoch_deadline(1);
        store.set_fuel(1_000).expect("consume_fuel is enabled on the shared engine");
        // 🧵️ P1f: `EpochTicker` now drives off a `WorkerPool` `Lane::Timer` job, not a dedicated OS
        // thread — a small, deterministic own-pool (never the crate-wide singleton) so this test
        // stays isolated, matching `semio-framework-os-services`' own `test_pool` convention.
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
        let ticker = EpochTicker::start(&engine, &pool);
        std::thread::sleep(std::time::Duration::from_millis(10));
        drop(ticker);
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_engine_config_hash_is_deterministic_and_config_sensitive() {
        let cfg = SharedEngineConfig::default();
        let a = shared_engine_config_hash(&cfg, true).await;
        let b = shared_engine_config_hash(&cfg, true).await;
        assert_eq!(a, b);
        let c = shared_engine_config_hash(&cfg, false).await;
        assert_ne!(a, c, "pooling vs on-demand must be different cache namespaces");
    }

    #[semio_framework_async_macros::async_test]
    async fn compiled_cache_path_is_namespaced_by_both_hashes() {
        let root = Path::new("/tmp/semio-cache-test");
        let engine_hash = [1u8; 32];
        let package_hash = [2u8; 32];
        let path = compiled_cache_path(root, &engine_hash, &package_hash).await;
        assert!(path.starts_with(root));
        assert!(path.to_string_lossy().ends_with(&format!("{}.cwasm", hex_encode(&package_hash))));
    }

    #[semio_framework_async_macros::async_test]
    async fn compiled_component_round_trips_through_cache_for_a_real_wasm_file() {
        let wasm_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        if !wasm_path.exists() {
            return;
        }
        let (engine, _pooling_active) = build_shared_engine(SharedEngineConfig::default()).await.expect("engine builds");
        let wasm_bytes = std::fs::read(wasm_path).expect("read real stdio.wasm");
        let component = Component::from_binary(&engine, &wasm_bytes).expect("compile real stdio.wasm as a component");
        let cache_dir = std::env::temp_dir().join(format!("semio-compiled-cache-test-{}", std::process::id()));
        let cache_path = compiled_cache_path(&cache_dir, &shared_engine_config_hash(&SharedEngineConfig::default(), true).await, &[3u8; 32]).await;
        assert!(load_compiled_component(&engine, &cache_path).await.is_none(), "cache must start empty");
        store_compiled_component(&component, &cache_path).await.expect("write compiled cache entry");
        let restored = load_compiled_component(&engine, &cache_path).await.expect("cache hit after writing");
        drop(restored);
        let _ = std::fs::remove_dir_all(&cache_dir);
    }
}
//#endregion 🔧️SharedWasmtimeEngine

//#region 🎭️GuestRuntime
/// 🧬️ `design-runtime.md` §2's `GuestRuntime` trait, verbatim. Replaces regions `🔖️WasmPluginRuntime`
/// and `🔖️ExtensionRuntime` (below) — NOT deleted yet: `WasmtimeRuntime`'s `impl GuestRuntime` needs
/// `wasmtime::component::bindgen!` to compile against the new `actor` world, which is currently
/// broken (see `📓️terra-B1-host-native-report.md`, `blocked-on-A2`: `📜️wit/📜️effects.wit:44`'s
/// `stream: bool` field uses a WIT-reserved keyword, so BOTH this file's `bindgen!` calls —
/// `plugin-world` and `extension-world`, themselves stale names since `world.wit` now only
/// declares `actor` — fail to parse). Landing the trait + `MockGuestRuntime` now, without
/// `WasmtimeRuntime`, still unblocks every downstream packet (H1-H4, T1) that only needs to code
/// against the interface.
///
/// `RuntimeActorId` never shadows `kernel::ActorId` (`📌️important.md`'s naming-hazard note) — this
/// file does not import `kernel::ActorId` at all, only `semio_framework_actor::ActorId` (aliased
/// `RuntimeActorId` in the top-of-file `use`).
/// 📦️ What [`GuestRuntime::compile`] compiles — a package identity plus the content hash that also
/// keys the compiled-artifact cache (`shared_engine_config_hash`/`compiled_cache_path` above).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageRef {
    pub package: PackageId,
    pub hash: PackageHash,
}

/// 🧩️ A compiled, not-yet-instantiated component. `component` is `None` for [`MockGuestRuntime`]
/// (which never compiles real wasm) and `Some` for [`WasmtimeRuntime`] (`//#region 🐎️WasmtimeRuntime`
/// below).
#[derive(Clone)]
pub struct CompiledHandle {
    pub package_hash: [u8; 32],
    component: Option<Arc<Component>>,
    owned: Option<Arc<OwnedSemioArtifact>>,
}

impl std::fmt::Debug for CompiledHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledHandle").field("package_hash", &hex_encode(&self.package_hash)).field("has_component", &self.component.is_some()).field("has_owned_actor", &self.owned.is_some()).finish()
    }
}

/// 🏃️ One running actor instance, host-owned. `Mock(..)` backs [`MockGuestRuntime`]; `Wasmtime(..)`
/// backs [`WasmtimeRuntime`] (`//#region 🐎️WasmtimeRuntime` below).
pub struct GuestInstance {
    pub actor: RuntimeActorId,
    state: GuestInstanceState,
}

/// 🧬️ Shallow, actor-id-only — `WasmtimeInstanceState.store: Store<ActorHostState>` and
/// `bindings: actor_bindings::Actor` implement neither `Debug` (wasmtime's `Store` deliberately
/// doesn't; bindgen-generated binding structs don't either), so `#[derive(Debug)]` cannot reach
/// through them. Callers (`Result::expect_err` on a `GuestInstance`-returning call, mainly) only ever
/// need enough to identify WHICH instance failed.
impl std::fmt::Debug for GuestInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuestInstance").field("actor", &self.actor).finish()
    }
}

enum GuestInstanceState {
    // 🧪️ Only `MockGuestRuntime` (`#[cfg(test)]`) ever constructs this variant — `#[allow(dead_code)]`
    // rather than `#[cfg(test)]` on the variant itself, so `GuestInstanceState` stays a REFUTABLE
    // match target (`Wasmtime` is not the only variant) in every build config; a `#[cfg(test)]`
    // variant would make the `let GuestInstanceState::Wasmtime(state) = .. else { .. }` guards in
    // `WasmtimeRuntime`'s own methods irrefutable outside `cfg(test)`, turning their `else` arms into
    // a NEW dead-code warning instead of fixing one.
    #[allow(dead_code)]
    Mock(MockInstanceState),
    Owned(OwnedInstanceState),
    Wasmtime(WasmtimeInstanceState),
}

/// ⛽️ `jobs.wit`'s `job-budget` record, mirrored field-for-field (design-abi.md §1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
pub struct JobBudget {
    pub fuel: u64,
    pub deadline_ms: u32,
}

/// 🪜️ `jobs.wit`'s `job-step` variant, mirrored field-for-field. `Serialize`/`Deserialize` added
/// (past `design-runtime.md` §2's literal trait listing) so `🧵️shard/🦀️.rs`'s `ShardOutcome`
/// can carry one over a `ShardTransport` — every other outcome shape (`TurnResult`) already derives
/// both, so this was the one gap.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1): `Running` is a STRUCT variant, not the newtype
/// `Running(Option<Vec<u8>>)` this used to be — serde's internal tagging (`#[serde(tag = "kind")]`)
/// cannot serialize a newtype variant whose payload is itself an `Option<T>` ("cannot serialize
/// tagged newtype variant ... containing an optional" — a real serde limitation, not a typo).
/// `ShardLoop::pump`'s job-stepping path never actually SENT a `Running` outcome over a transport
/// before this packet (no test drove a job past its first step, so a `Running` step never reached
/// `send_outcome`'s `serde_json::to_vec` — the exact "a contract that compiles is not a contract
/// that runs" shape this whole packet exists to close), which is how this stayed latent. Every
/// OTHER `Option`-carrying variant in this crate's kernel types already uses a struct variant for
/// exactly this reason (e.g. `kernel::Event::JobProgress { job, progress: Option<Vec<u8>> }`).
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (K1/registrar): `Done`/`Failed` are struct variants
/// for the SAME reason, found one wave later. J1 fixed only `Running`, because serde's internal
/// tagging rejects a newtype variant carrying an `Option` — but it rejects one carrying ANY
/// sequence, and `Vec<u8>` is a sequence: `serde_json` errors with "cannot serialize tagged newtype
/// variant JobStep::Done containing a sequence". So every *successful* job completion failed to
/// serialize, on the one path a job must survive to be useful. J1's resumability test drove three
/// `step_job` calls but asserted on the in-process `JobStep` values rather than on bytes that had
/// crossed `send_outcome`, so the completion path was proven in memory and never on the wire.
/// The lesson generalises past serde: **fixing one variant of a defect is not fixing the defect** —
/// the sibling variants must be re-derived from the rule, not from the symptom that was reported.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum JobStep {
    Running { progress: Option<Vec<u8>> },
    Done { output: Vec<u8> },
    Failed { error: Vec<u8> },
}

/// 🧯️ Why a turn or job step didn't produce a result — distinct from [`PluginHostError`] (host-side
/// plumbing faults): every `GuestRuntime::execute_turn`/`step_job` failure is one of these.
#[derive(Debug)]
pub enum TurnFault {
    Host(PluginHostError),
    Exhausted,
    Trapped(String),
    DeadlineExceeded,
    FuelExhausted,
}

impl std::fmt::Display for TurnFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Host(error) => std::fmt::Display::fmt(error, formatter),
            Self::Exhausted => formatter.write_str("guest instance has no more scripted/actual turns"),
            Self::Trapped(message) => write!(formatter, "guest trapped: {message}"),
            Self::DeadlineExceeded => formatter.write_str("epoch deadline exceeded"),
            Self::FuelExhausted => formatter.write_str("fuel exhausted"),
        }
    }
}

impl std::error::Error for TurnFault {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Host(error) => Some(error),
            _ => None,
        }
    }
}

impl From<PluginHostError> for TurnFault {
    fn from(error: PluginHostError) -> Self {
        Self::Host(error)
    }
}

/// 🐎️ Host-side driver for one actor's execution — `design-runtime.md` §2. `WasmtimeRuntime` (the
/// native implementation, backed by [`build_shared_engine`]/[`EpochTicker`]/[`BudgetLimiter`]/the
/// compiled-artifact cache above), `MockGuestRuntime` (test double, below), and
/// `shard::RecordingRuntime` (`🧵️shard`'s own test double) all implement this; nothing else in the
/// host — `ShardLoop`, the task manager, `WasmtimeNodeHost` — talks to a guest through any other
/// surface. The closed set of impls is dispatched through the [`GuestRuntimes`] enum, never `dyn`
/// (O1/R1 — `async fn` in a trait is not dyn-compatible, and dyn-Future erasure in trait-method
/// return position is banned).
///
/// 👶️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn): the turn/job/checkpoint methods are
/// plain `async fn` returning their `Result` directly — the double-future `HostFuture<Result<..>>`
/// wrapper terra-trait-asyncify staged here is gone (R1: `dyn Future` is banned from trait-method
/// return position). `compile`/`instantiate`/`drop_instance` stay plain, non-`async fn`: `compile`
/// is CPU-bound with no await point, `instantiate` only BUILDS a task spec for an async backend
/// (never runs one), and `drop_instance` is a destructor. `PluginInstanceHandle` treats every
/// implementation as genuinely suspending: it moves the guest instance into a retained-waker
/// future and polls it through the process worker pool without parking a worker or holding the
/// instance mutex across suspension.
pub trait GuestRuntime: Send + Sync {
    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError>;
    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError>;
    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault>;
    /// 🧬️ `jobs.wit`'s `start-job` export — added past `design-runtime.md` §2's literal trait listing
    /// because that listing omits it even though `jobs.wit` declares three functions
    /// (`start-job`/`step-job`/`cancel-job`), not one: a job cannot be stepped before it exists.
    /// `PluginInstanceHandle::run_job_on_worker` (`//#region 🔀️PostTurnRelay`) is the only caller.
    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault>;
    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault>;
    /// 🛑️ `jobs.wit`'s `cancel-job` export — added past `design-runtime.md` §2's literal trait
    /// listing for the SAME reason `start_job` was (that listing names only `execute_turn`/
    /// `step_job`, but `jobs.wit` declares three functions, and a generic `Effect::CancelJob`
    /// admission path — `🧵️shard/🦀️.rs`'s `ShardLoop::pump` — needs somewhere to call).
    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault>;
    async fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError>;
    async fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError>;
    async fn drop_instance(&self, inst: GuestInstance);
}

//#region 🔖️MockGuestRuntime
/// 🎬️ One actor's scripted future: either the next `execute_turn`/`step_job` call returns this
/// `TurnResult`/`JobStep`, or the runtime raises this fault instead. `#[allow(dead_code)]`, not
/// `#[cfg(test)]`, for the same reason `GuestInstanceState::Mock` isn't `cfg(test)`-gated either —
/// see that variant's own doc comment.
#[allow(dead_code)]
enum ScriptedOutcome {
    Turn(TurnResult),
    Job(JobStep),
    #[cfg(test)]
    PendingJob {
        gate: Arc<MockJobStepGate>,
        step: JobStep,
    },
    Fault(String),
    /// 🛑️ terra-shard-lane piece 2: distinct from `Fault(String)` (which always becomes
    /// `TurnFault::Trapped`) — this scripts the SPECIFIC `TurnFault::DeadlineExceeded` variant a
    /// real `WasmtimeRuntime::execute_turn` raises when an epoch deadline armed from
    /// `budget.deadline_ms` is hit, so shard-level tests can prove `ShardLoop::pump_primed`
    /// converts it into a graceful `ShardOutcome::Turn{status: MoreWork}` instead of a
    /// `ShardOutcome::Fault` — see `📓️terra-shard-lane-report.md`.
    DeadlineExceeded,
}

#[derive(Default)]
struct MockInstanceState {
    #[allow(dead_code)]
    checkpoint: Option<Vec<u8>>,
}

#[cfg(test)]
#[derive(Default)]
struct MockJobStepGate {
    ready: AtomicBool,
    polls: std::sync::atomic::AtomicUsize,
    waker: Mutex<Option<std::task::Waker>>,
}

#[cfg(test)]
impl MockJobStepGate {
    async fn wait(&self) {
        std::future::poll_fn(|context| {
            self.polls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if self.ready.load(std::sync::atomic::Ordering::Acquire) {
                std::task::Poll::Ready(())
            } else {
                *self.waker.lock().expect("mock job gate lock poisoned") = Some(context.waker().clone());
                std::task::Poll::Pending
            }
        })
        .await
    }

    fn release(&self) {
        self.ready.store(true, std::sync::atomic::Ordering::Release);
        if let Some(waker) = self.waker.lock().expect("mock job gate lock poisoned").take() {
            waker.wake();
        }
    }

    fn is_waiting(&self) -> bool {
        self.polls.load(std::sync::atomic::Ordering::SeqCst) > 0
    }
}

/// 🎭️ `design-runtime.md` §2's `MockGuestRuntime` (`#[cfg(test)]`): scripted turns + a controllable
/// clock, backing scheduler/failure-ladder tests without a real wasm component or `bindgen!` — the
/// TS twin is `createMockShard()`. `script_turn`/`script_fault` queue outcomes FIFO per actor;
/// `execute_turn` on an actor with an empty queue returns `TurnFault::Exhausted` rather than
/// silently fabricating an idle turn, so a test that forgets to script a call fails loudly instead
/// of passing by accident.
#[cfg(test)]
pub struct MockGuestRuntime {
    now_ms: std::sync::atomic::AtomicI64,
    scripts: Mutex<HashMap<u64, VecDeque<ScriptedOutcome>>>,
    start_admissions: std::sync::atomic::AtomicUsize,
    step_admissions: std::sync::atomic::AtomicUsize,
    cancel_admissions: std::sync::atomic::AtomicUsize,
    fail_start: AtomicBool,
    fail_cancel: AtomicBool,
    panic_start: AtomicBool,
    panic_step: AtomicBool,
    panic_cancel: AtomicBool,
    relay_start_failure_release: Mutex<Option<Arc<MockJobStepGate>>>,
    relay_step_failure_release: Mutex<Option<Arc<MockJobStepGate>>>,
    relay_cancel_release: Mutex<Option<Arc<MockJobStepGate>>>,
    /// 📼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1): every `events` slice `execute_turn` was
    /// ever called with, per actor, in call order — `_events` used to be ignored entirely. A test
    /// proving a job's completion actually reaches the ORIGINATING actor (not just that
    /// `step_job` returned `Done`) needs to see the synthesized `Event::JobCompleted` really was
    /// handed to a later `execute_turn` call, which this makes assertable without a real guest.
    observed_events: Mutex<HashMap<u64, Vec<Event>>>,
}

#[cfg(test)]
impl Default for MockGuestRuntime {
    fn default() -> Self {
        Self {
            now_ms: std::sync::atomic::AtomicI64::new(0),
            scripts: Mutex::new(HashMap::new()),
            start_admissions: std::sync::atomic::AtomicUsize::new(0),
            step_admissions: std::sync::atomic::AtomicUsize::new(0),
            cancel_admissions: std::sync::atomic::AtomicUsize::new(0),
            fail_start: AtomicBool::new(false),
            fail_cancel: AtomicBool::new(false),
            panic_start: AtomicBool::new(false),
            panic_step: AtomicBool::new(false),
            panic_cancel: AtomicBool::new(false),
            relay_start_failure_release: Mutex::new(None),
            relay_step_failure_release: Mutex::new(None),
            relay_cancel_release: Mutex::new(None),
            observed_events: Mutex::new(HashMap::new()),
        }
    }
}

#[cfg(test)]
impl MockGuestRuntime {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn now_ms(&self) -> i64 {
        self.now_ms.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub async fn set_now_ms(&self, ms: i64) {
        self.now_ms.store(ms, std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn advance_ms(&self, delta: i64) {
        self.now_ms.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
    }

    async fn queue_for(&self, actor: RuntimeActorId) -> std::sync::MutexGuard<'_, HashMap<u64, VecDeque<ScriptedOutcome>>> {
        let mut scripts = self.scripts.lock().expect("mock runtime lock poisoned");
        scripts.entry(actor.0).or_default();
        scripts
    }

    pub async fn script_turn(&self, actor: RuntimeActorId, result: TurnResult) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Turn(result));
    }

    pub async fn script_job_step(&self, actor: RuntimeActorId, step: JobStep) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Job(step));
    }

    async fn script_pending_job_step(&self, actor: RuntimeActorId, step: JobStep) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::PendingJob { gate: Arc::clone(&gate), step });
        gate
    }

    fn script_pending_cancel_job(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_cancel_release.lock().expect("mock cancel release lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    fn start_admissions(&self) -> usize {
        self.start_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn step_admissions(&self) -> usize {
        self.step_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn cancel_admissions(&self) -> usize {
        self.cancel_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn panic_next_start(&self) {
        self.panic_start.store(true, std::sync::atomic::Ordering::Release);
    }

    fn fail_next_start(&self) {
        self.fail_start.store(true, std::sync::atomic::Ordering::Release);
    }

    fn panic_next_step(&self) {
        self.panic_step.store(true, std::sync::atomic::Ordering::Release);
    }

    fn panic_next_cancel(&self) {
        self.panic_cancel.store(true, std::sync::atomic::Ordering::Release);
    }

    fn fail_next_cancel(&self) {
        self.fail_cancel.store(true, std::sync::atomic::Ordering::Release);
    }

    fn pause_after_next_start_failure_release(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_start_failure_release.lock().expect("mock start release barrier lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    fn pause_after_next_step_failure_release(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_step_failure_release.lock().expect("mock step release barrier lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    fn take_relay_start_failure_release(&self) -> Option<Arc<MockJobStepGate>> {
        self.relay_start_failure_release.lock().expect("mock start release barrier lock poisoned").take()
    }

    fn take_relay_step_failure_release(&self) -> Option<Arc<MockJobStepGate>> {
        self.relay_step_failure_release.lock().expect("mock step release barrier lock poisoned").take()
    }

    pub async fn script_fault(&self, actor: RuntimeActorId, message: impl Into<String>) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Fault(message.into()));
    }

    /// 🛑️ terra-shard-lane piece 2: schedules `TurnFault::DeadlineExceeded` specifically — see
    /// `ScriptedOutcome::DeadlineExceeded`'s own doc for why this is not just `script_fault` with a
    /// fixed message.
    pub async fn script_deadline_exceeded(&self, actor: RuntimeActorId) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::DeadlineExceeded);
    }

    /// 🏁️ A plain `Idle`, no-effects, no-patches turn result — convenience for tests that only
    /// care about scheduling/backpressure, not turn content.
    pub async fn idle_turn() -> TurnResult {
        TurnResult { ui_patches: semio_framework::kernel::UiTurnPatches::default(), effects: Vec::new(), presence: Vec::new(), next_wake: None, status: TurnStatus::Idle, fuel_used: 0, lifecycle_receipt: None, ui_patch_receipt: None, command_ingress: semio_framework::kernel::CommandIngressStatus::Idle }
    }

    /// 📼️ Every `events` slice `execute_turn` has been called with for `actor`, flattened across
    /// every call, in order — see `observed_events`'s own doc comment for why this exists.
    pub async fn observed_events(&self, actor: RuntimeActorId) -> Vec<Event> {
        self.observed_events.lock().expect("mock runtime lock poisoned").get(&actor.0).cloned().unwrap_or_default()
    }
}

#[cfg(test)]
impl GuestRuntime for MockGuestRuntime {
    async fn compile(&self, package: &PackageRef, _bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        Ok(CompiledHandle { package_hash: package.hash.0, component: None, owned: None })
    }

    async fn instantiate(&self, _compiled: &CompiledHandle, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        drop(self.queue_for(actor));
        Ok(GuestInstance { actor, state: GuestInstanceState::Mock(MockInstanceState::default()) })
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], _budget: Budget) -> Result<TurnResult, TurnFault> {
        // 👶️ host-dedyn: identical body to before this packet — no suspension point, so this
        // resolves on its very first poll, same contract `GuestRuntime`'s own doc comment names.
        self.observed_events.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?.entry(inst.actor.0).or_default().extend_from_slice(events);
        let mut scripts = self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?;
        let queue = scripts.entry(inst.actor.0).or_default();
        match queue.pop_front() {
            Some(ScriptedOutcome::Turn(result)) => Ok(result),
            Some(ScriptedOutcome::Job(_)) => Err(TurnFault::Trapped("scripted outcome was a job step, not a turn".to_string())),
            Some(ScriptedOutcome::PendingJob { .. }) => Err(TurnFault::Trapped("scripted outcome was a pending job step, not a turn".to_string())),
            Some(ScriptedOutcome::Fault(message)) => Err(TurnFault::Trapped(message)),
            Some(ScriptedOutcome::DeadlineExceeded) => Err(TurnFault::DeadlineExceeded),
            None => Err(TurnFault::Exhausted),
        }
    }

    /// 🎬️ `start-job` has no interesting success payload, so normal outcomes do not consume
    /// `ScriptedOutcome`; dedicated one-shot failure/panic flags exercise relay recovery directly.
    async fn start_job(&self, inst: &mut GuestInstance, _job: u64, _kind: &str, _input: Vec<u8>) -> Result<(), TurnFault> {
        self.start_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_start.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted start-job panic after guest instance acquisition");
        if self.fail_start.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return Err(TurnFault::Trapped("scripted start-job failure".to_string()));
        }
        if !self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?.contains_key(&inst.actor.0) {
            return Err(TurnFault::Exhausted);
        }
        Ok(())
    }

    async fn step_job(&self, inst: &mut GuestInstance, _job: u64, _budget: JobBudget) -> Result<JobStep, TurnFault> {
        self.step_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_step.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted step-job panic after guest instance acquisition");
        let scripted = {
            let mut scripts = self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?;
            scripts.entry(inst.actor.0).or_default().pop_front()
        };
        match scripted {
            Some(ScriptedOutcome::Job(step)) => Ok(step),
            Some(ScriptedOutcome::PendingJob { gate, step }) => {
                gate.wait().await;
                Ok(step)
            }
            Some(ScriptedOutcome::Turn(_)) => Err(TurnFault::Trapped("scripted outcome was a turn, not a job step".to_string())),
            Some(ScriptedOutcome::Fault(message)) => Err(TurnFault::Trapped(message)),
            Some(ScriptedOutcome::DeadlineExceeded) => Err(TurnFault::DeadlineExceeded),
            None => Err(TurnFault::Exhausted),
        }
    }

    /// 🛑️ Cancellation succeeds without consuming the step queue by default; dedicated
    /// one-shot failure/panic flags prove callers retain or retire uncertain guest state.
    async fn cancel_job(&self, _inst: &mut GuestInstance, _job: u64) -> Result<(), TurnFault> {
        self.cancel_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_cancel.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted cancel-job panic after guest instance acquisition");
        if self.fail_cancel.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return Err(TurnFault::Trapped("scripted cancel-job failure".to_string()));
        }
        let release = self.relay_cancel_release.lock().expect("mock cancel release lock poisoned").take();
        if let Some(release) = release {
            release.wait().await;
        }
        Ok(())
    }

    async fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Mock(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("MockGuestRuntime::checkpoint called on a non-mock GuestInstance".to_string()));
        };
        let bytes = format!("mock-checkpoint:{}", inst.actor.0).into_bytes();
        state.checkpoint = Some(bytes.clone());
        Ok(bytes)
    }

    async fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError> {
        let GuestInstanceState::Mock(mock_state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("MockGuestRuntime::restore called on a non-mock GuestInstance".to_string()));
        };
        mock_state.checkpoint = Some(state.to_vec());
        Ok(())
    }

    async fn drop_instance(&self, inst: GuestInstance) {
        self.scripts.lock().map(|mut scripts| scripts.remove(&inst.actor.0)).ok();
    }
}

#[cfg(test)]
mod mock_guest_runtime_tests {
    use super::*;

    async fn hash(byte: u8) -> PackageHash {
        PackageHash([byte; 32])
    }

    #[semio_framework_async_macros::async_test]
    async fn scripted_turn_is_returned_exactly_once_fifo() {
        let runtime = MockGuestRuntime::new().await;
        let compiled = runtime.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: hash(1).await }, &[]).await.expect("compile");
        let actor = RuntimeActorId(42);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("instantiate");

        let mut first = MockGuestRuntime::idle_turn().await;
        first.fuel_used = 7;
        let mut second = MockGuestRuntime::idle_turn().await;
        second.fuel_used = 9;
        runtime.script_turn(actor, first).await;
        runtime.script_turn(actor, second).await;

        // 👶️ host-dedyn: `#[test] fn` is a sanctioned `block_on` entry point (R4 clause 5) — this
        // test drives `GuestRuntime`'s async methods directly against the concrete `MockGuestRuntime`
        // (not through the `GuestRuntimes` enum), same as it did through the deleted `poll_ready`.
        let got_first = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 })).expect("first scripted turn");
        assert_eq!(got_first.fuel_used, 7);
        let got_second = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 })).expect("second scripted turn");
        assert_eq!(got_second.fuel_used, 9);
    }

    #[semio_framework_async_macros::async_test]
    async fn exhausted_script_queue_is_a_loud_error_not_a_fabricated_idle_turn() {
        let runtime = MockGuestRuntime::new().await;
        let compiled = runtime.compile(&PackageRef { package: PackageId("cad".to_string()), hash: hash(2).await }, &[]).await.expect("compile");
        let actor = RuntimeActorId(7);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
        let error = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 })).expect_err("no script queued");
        assert!(matches!(error, TurnFault::Exhausted));
    }

    #[semio_framework_async_macros::async_test]
    async fn scripted_fault_surfaces_as_trapped() {
        let runtime = MockGuestRuntime::new().await;
        let compiled = runtime.compile(&PackageRef { package: PackageId("block".to_string()), hash: hash(3).await }, &[]).await.expect("compile");
        let actor = RuntimeActorId(9);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
        runtime.script_fault(actor, "epoch deadline exceeded").await;
        let error = semio_framework_async::block_on(runtime.execute_turn(&mut inst, &[], Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 })).expect_err("scripted fault");
        assert!(matches!(error, TurnFault::Trapped(message) if message == "epoch deadline exceeded"));
    }

    #[semio_framework_async_macros::async_test]
    async fn checkpoint_then_restore_round_trips_through_a_fresh_instance() {
        let runtime = MockGuestRuntime::new().await;
        let compiled = runtime.compile(&PackageRef { package: PackageId("puzzle".to_string()), hash: hash(4).await }, &[]).await.expect("compile");
        let actor = RuntimeActorId(11);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
        let snapshot = semio_framework_async::block_on(runtime.checkpoint(&mut inst)).expect("checkpoint");

        let mut restored = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("re-instantiate");
        semio_framework_async::block_on(runtime.restore(&mut restored, &snapshot)).expect("restore");
        let GuestInstanceState::Mock(state) = &restored.state else { panic!("expected a Mock instance") };
        assert_eq!(state.checkpoint.as_deref(), Some(snapshot.as_slice()));
    }

    #[semio_framework_async_macros::async_test]
    async fn controllable_clock_advances_deterministically() {
        let runtime = MockGuestRuntime::new().await;
        runtime.set_now_ms(1_000).await;
        assert_eq!(runtime.now_ms().await, 1_000);
        runtime.advance_ms(250).await;
        assert_eq!(runtime.now_ms().await, 1_250);
    }

    #[semio_framework_async_macros::async_test]
    async fn drop_instance_forgets_the_actors_script_queue() {
        let runtime = MockGuestRuntime::new().await;
        let compiled = runtime.compile(&PackageRef { package: PackageId("layout".to_string()), hash: hash(5).await }, &[]).await.expect("compile");
        let actor = RuntimeActorId(13);
        let inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");
        runtime.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
        runtime.drop_instance(inst).await;
        assert!(!runtime.scripts.lock().expect("lock").contains_key(&actor.0));
    }
}
//#endregion 🔖️MockGuestRuntime

//#region 🧠️OwnedRuntime

const OWNED_RESULT_LIMIT: usize = 64 * 1024 * 1024;
const OWNED_CHECKPOINT_MAGIC: &[u8; 8] = b"SMOWNH01";
const OWNED_STEP_FUEL: u64 = 4_096;
const OWNED_SLICE_DEADLINE_MS: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
enum OwnedOperation {
    Describe,
    Poll,
    StartJob,
    StepJob,
    CancelJob,
    Checkpoint,
    Restore,
}

impl OwnedOperation {
    fn export(self) -> OwnedSemioExport {
        match self {
            Self::Describe => OwnedSemioExport::Describe,
            Self::Poll => OwnedSemioExport::Poll,
            Self::StartJob => OwnedSemioExport::StartJob,
            Self::StepJob => OwnedSemioExport::StepJob,
            Self::CancelJob => OwnedSemioExport::CancelJob,
            Self::Checkpoint => OwnedSemioExport::Checkpoint,
            Self::Restore => OwnedSemioExport::Restore,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
enum OwnedStage {
    Allocate { input: Vec<u8> },
    Call,
    Deallocate { output: Vec<u8> },
}

#[derive(Clone, Debug, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
struct OwnedPending {
    operation: OwnedOperation,
    stage: OwnedStage,
    fuel_used: u64,
}

#[derive(Clone, Debug, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
struct OwnedCheckpointMetadata {
    pending: Option<OwnedPending>,
    guest_checkpoint: Option<Vec<u8>>,
    context: i32,
    next_resource: i32,
    instance_id: u32,
}

struct OwnedInstanceState {
    artifact: Arc<OwnedSemioArtifact>,
    actor: OwnedSemioInstance,
    pending: Option<OwnedPending>,
    context: i32,
    next_resource: i32,
    instance_id: u32,
}

/// 🔀️ `serde`-only, not `ToValue`: `events`/`command_page`/`budget` embed `semio_framework::kernel::
/// {Event, CommandPageCursor, FixedCommandPage, Budget}`, none of which have `ToValue`/`FromValue`
/// yet (same orphan-rule/`rename_all_fields`-unsupported blocker as `owned_abi::PollInput`'s own
/// hand-written bridge in `🔌️plugin/🦀️.rs` — see that impl's docstring). Converting this
/// needs the `🎠️kernel` module's own conversion pass first.
#[derive(serde::Serialize)]
struct OwnedPollInput<'a> {
    events: &'a [Event],
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    budget: Budget,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedStartJobInput<'a> {
    job: u64,
    kind: &'a str,
    input: Vec<u8>,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedStepJobInput {
    job: u64,
    budget: JobBudget,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedCancelJobInput {
    job: u64,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedRestoreInput {
    state: Vec<u8>,
}

struct OwnedInvocation {
    output: Vec<u8>,
    fuel_used: u64,
}

pub struct OwnedRuntime {
    next_instance_id: std::sync::atomic::AtomicU32,
}

impl Default for OwnedRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedRuntime {
    pub fn new() -> Self {
        Self { next_instance_id: std::sync::atomic::AtomicU32::new(1) }
    }

    pub fn compile_component(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        let artifact = OwnedSemioArtifact::parse(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        Ok(CompiledHandle { package_hash: package.hash.0, component: None, owned: Some(Arc::new(artifact)) })
    }

    pub fn instantiate_actor(&self, compiled: &CompiledHandle, actor: RuntimeActorId) -> Result<GuestInstance, PluginHostError> {
        let artifact = compiled.owned.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no repository-owned actor artifact".to_string()))?;
        let owned = artifact.instantiate().map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        if owned.startup_active() {
            return Err(PluginHostError::Plugin("owned actor has an undriven start function".to_string()));
        }
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(GuestInstance { actor, state: GuestInstanceState::Owned(OwnedInstanceState { artifact: Arc::clone(artifact), actor: owned, pending: None, context: 0, next_resource: 1, instance_id }) })
    }

    pub fn execute_actor_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault> {
        let state = owned_state_mut(inst)?;
        let mut ordinary_events = Vec::with_capacity(events.len());
        let mut command_page = None;
        for event in events {
            match event {
                Event::CommandIngressPage { cursor, bytes } if command_page.is_none() && bytes.len() <= semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES && (!bytes.is_empty() || (cursor.kind == 28 && cursor.item_count == 0)) => {
                    command_page = Some((cursor.clone(), bytes.clone()));
                }
                Event::CommandIngressPage { .. } => return Err(TurnFault::Trapped("turn carries more than one command page or an invalid page size".to_string())),
                event => ordinary_events.push(event.clone()),
            }
        }
        let input = serde_json::to_vec(&OwnedPollInput { events: &ordinary_events, command_page, budget }).map_err(|error| PluginHostError::Json(error.to_string()))?;
        begin_owned_operation(state, OwnedOperation::Poll, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::Poll, budget.fuel, budget.deadline_ms)?;
        let mut result: TurnResult = decode_owned_result(&invocation.output)?;
        result.fuel_used = invocation.fuel_used;
        Ok(result)
    }

    pub fn drop_actor(&self, _inst: GuestInstance) {}

    pub async fn describe(&self, compiled: &CompiledHandle, budget: Budget) -> Result<Vec<u8>, TurnFault> {
        let mut instance = self.instantiate_actor(compiled, RuntimeActorId(0)).map_err(TurnFault::Host)?;
        let state = owned_state_mut(&mut instance)?;
        begin_owned_operation(state, OwnedOperation::Describe, None)?;
        resume_owned_operation(state, OwnedOperation::Describe, budget.fuel, budget.deadline_ms).map(|invocation| invocation.output)
    }
}

impl GuestRuntime for OwnedRuntime {
    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        self.compile_component(package, bytes)
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        self.instantiate_actor(compiled, actor)
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault> {
        self.execute_actor_turn(inst, events, budget)
    }

    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        let state = owned_state_mut(inst)?;
        let input = dsl::os_pack::json::to_json_string(&OwnedStartJobInput { job, kind, input }).into_bytes();
        begin_owned_operation(state, OwnedOperation::StartJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::StartJob, u64::MAX, 1_000)?;
        decode_owned_result(&invocation.output)
    }

    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        let state = owned_state_mut(inst)?;
        let input = dsl::os_pack::json::to_json_string(&OwnedStepJobInput { job, budget }).into_bytes();
        begin_owned_operation(state, OwnedOperation::StepJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::StepJob, budget.fuel, budget.deadline_ms)?;
        decode_owned_result(&invocation.output)
    }

    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault> {
        let state = owned_state_mut(inst)?;
        if state.pending.as_ref().is_some_and(|pending| pending.operation != OwnedOperation::CancelJob) {
            cancel_owned_operation(state)?;
        }
        let input = dsl::os_pack::json::to_json_string(&OwnedCancelJobInput { job }).into_bytes();
        begin_owned_operation(state, OwnedOperation::CancelJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::CancelJob, u64::MAX, OWNED_SLICE_DEADLINE_MS)?;
        decode_owned_result(&invocation.output)
    }

    async fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError> {
        let state = match &mut inst.state {
            GuestInstanceState::Owned(state) => state,
            _ => return Err(PluginHostError::Plugin("checkpoint called on a non-owned GuestInstance".to_string())),
        };
        let guest_checkpoint = if state.pending.is_none() {
            begin_owned_operation(state, OwnedOperation::Checkpoint, None).map_err(turn_fault_host)?;
            let invocation = resume_owned_operation(state, OwnedOperation::Checkpoint, 100_000_000, 1_000).map_err(turn_fault_host)?;
            Some(decode_owned_result::<Vec<u8>>(&invocation.output).map_err(turn_fault_host)?)
        } else {
            None
        };
        let metadata = OwnedCheckpointMetadata { pending: state.pending.clone(), guest_checkpoint, context: state.context, next_resource: state.next_resource, instance_id: state.instance_id };
        let metadata = dsl::os_pack::json::to_json_string(&metadata).into_bytes();
        let actor = state.actor.checkpoint();
        let mut checkpoint = Vec::with_capacity(20 + metadata.len() + actor.len());
        checkpoint.extend_from_slice(OWNED_CHECKPOINT_MAGIC);
        checkpoint.extend_from_slice(&(metadata.len() as u32).to_le_bytes());
        checkpoint.extend_from_slice(&(actor.len() as u64).to_le_bytes());
        checkpoint.extend_from_slice(&metadata);
        checkpoint.extend_from_slice(&actor);
        Ok(checkpoint)
    }

    async fn restore(&self, inst: &mut GuestInstance, checkpoint: &[u8]) -> Result<(), PluginHostError> {
        let state = match &mut inst.state {
            GuestInstanceState::Owned(state) => state,
            _ => return Err(PluginHostError::Plugin("restore called on a non-owned GuestInstance".to_string())),
        };
        if checkpoint.get(..8) != Some(OWNED_CHECKPOINT_MAGIC) {
            return Err(PluginHostError::Plugin("invalid owned host checkpoint header".to_string()));
        }
        let metadata_length =
            checkpoint.get(8..12).and_then(|bytes| bytes.try_into().ok()).map(u32::from_le_bytes).map(|length| length as usize).ok_or_else(|| PluginHostError::Plugin("truncated owned host checkpoint metadata length".to_string()))?;
        let actor_length =
            checkpoint.get(12..20).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).and_then(|length| usize::try_from(length).ok()).ok_or_else(|| PluginHostError::Plugin("invalid owned host checkpoint actor length".to_string()))?;
        let metadata_end = 20usize.checked_add(metadata_length).ok_or_else(|| PluginHostError::Plugin("owned host checkpoint metadata length overflow".to_string()))?;
        let actor_end = metadata_end.checked_add(actor_length).ok_or_else(|| PluginHostError::Plugin("owned host checkpoint actor length overflow".to_string()))?;
        if actor_end != checkpoint.len() {
            return Err(PluginHostError::Plugin("owned host checkpoint length mismatch".to_string()));
        }
        let metadata_text = std::str::from_utf8(&checkpoint[20..metadata_end]).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let metadata: OwnedCheckpointMetadata = dsl::os_pack::json::from_json_str(metadata_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        state.actor = state.artifact.restore(&checkpoint[metadata_end..]).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        state.pending = metadata.pending;
        state.context = metadata.context;
        state.next_resource = metadata.next_resource;
        state.instance_id = metadata.instance_id;
        if let Some(guest_checkpoint) = metadata.guest_checkpoint {
            let input = dsl::os_pack::json::to_json_string(&OwnedRestoreInput { state: guest_checkpoint }).into_bytes();
            begin_owned_operation(state, OwnedOperation::Restore, Some(input)).map_err(turn_fault_host)?;
            let invocation = resume_owned_operation(state, OwnedOperation::Restore, 100_000_000, 1_000).map_err(turn_fault_host)?;
            decode_owned_result::<()>(&invocation.output).map_err(turn_fault_host)?;
        }
        Ok(())
    }

    async fn drop_instance(&self, inst: GuestInstance) {
        self.drop_actor(inst);
    }
}

fn owned_state_mut(inst: &mut GuestInstance) -> Result<&mut OwnedInstanceState, TurnFault> {
    match &mut inst.state {
        GuestInstanceState::Owned(state) => Ok(state),
        _ => Err(TurnFault::Trapped("owned runtime called on a non-owned GuestInstance".to_string())),
    }
}

fn begin_owned_operation(state: &mut OwnedInstanceState, operation: OwnedOperation, input: Option<Vec<u8>>) -> Result<(), TurnFault> {
    if let Some(pending) = &state.pending {
        return if pending.operation == operation { Ok(()) } else { Err(TurnFault::Trapped(format!("owned operation {:?} is still active", pending.operation))) };
    }
    let stage = match input {
        Some(input) => {
            state.actor.begin(OwnedSemioExport::Allocate, vec![Value::I32(i32::try_from(input.len()).map_err(|_| TurnFault::Trapped("owned input exceeds wasm32 address space".to_string()))?)])?;
            OwnedStage::Allocate { input }
        }
        None => {
            state.actor.begin(operation.export(), Vec::new())?;
            OwnedStage::Call
        }
    };
    state.pending = Some(OwnedPending { operation, stage, fuel_used: 0 });
    Ok(())
}

fn resume_owned_operation(state: &mut OwnedInstanceState, operation: OwnedOperation, fuel: u64, deadline_ms: u32) -> Result<OwnedInvocation, TurnFault> {
    let started = std::time::Instant::now();
    let mut remaining = fuel;
    loop {
        if started.elapsed() >= std::time::Duration::from_millis(u64::from(deadline_ms)) {
            return Err(TurnFault::DeadlineExceeded);
        }
        if remaining == 0 {
            return Err(TurnFault::FuelExhausted);
        }
        let mut pending = state.pending.take().ok_or_else(|| TurnFault::Trapped("owned operation has no resumable state".to_string()))?;
        if pending.operation != operation {
            state.pending = Some(pending);
            return Err(TurnFault::Trapped("owned operation resume type mismatch".to_string()));
        }
        let grant = remaining.min(OWNED_STEP_FUEL);
        match state.actor.step(grant, StepControl::default()) {
            CoreStepOutcome::Yield { fuel_used } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                state.pending = Some(pending);
                if fuel_used == 0 {
                    return Err(TurnFault::Trapped("owned interpreter yielded without consuming fuel".to_string()));
                }
            }
            CoreStepOutcome::HostCall { fuel_used, call } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                let results = reply_owned_host(state, &call).map_err(|error| TurnFault::Trapped(error.to_string()))?;
                state.actor.resume_host(call.id, Ok(results))?;
                state.pending = Some(pending);
            }
            CoreStepOutcome::Complete { fuel_used, values } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                match pending.stage {
                    OwnedStage::Allocate { input } => {
                        let [Value::I32(pointer)] = values.as_slice() else { return Err(TurnFault::Trapped("owned allocator returned an invalid pointer".to_string())) };
                        write_owned_memory(&mut state.actor, *pointer, &input)?;
                        state.actor.begin(operation.export(), vec![Value::I32(*pointer), Value::I32(input.len() as i32)])?;
                        pending.stage = OwnedStage::Call;
                        state.pending = Some(pending);
                    }
                    OwnedStage::Call => {
                        let output = state.actor.read_bytes_result(&values, OWNED_RESULT_LIMIT)?;
                        let [Value::I64(pair)] = values.as_slice() else { return Err(TurnFault::Trapped("owned call returned an invalid pointer/length pair".to_string())) };
                        let pair = *pair as u64;
                        state.actor.begin(OwnedSemioExport::Deallocate, vec![Value::I32(pair as u32 as i32), Value::I32((pair >> 32) as u32 as i32)])?;
                        pending.stage = OwnedStage::Deallocate { output };
                        state.pending = Some(pending);
                    }
                    OwnedStage::Deallocate { output } => return Ok(OwnedInvocation { output, fuel_used: pending.fuel_used }),
                }
            }
            CoreStepOutcome::Cancelled { fuel_used } => return Err(TurnFault::Trapped(format!("owned operation cancelled after {fuel_used} instructions"))),
            CoreStepOutcome::Fault { error, .. } => return Err(TurnFault::Trapped(error.to_string())),
        }
    }
}

fn cancel_owned_operation(state: &mut OwnedInstanceState) -> Result<(), TurnFault> {
    match state.actor.step(1, StepControl { cancelled: true }) {
        CoreStepOutcome::HostCall { call, .. } => {
            state.actor.resume_host(call.id, Err("owned operation cancelled".to_string()))?;
            if !matches!(state.actor.step(1, StepControl { cancelled: true }), CoreStepOutcome::Cancelled { .. }) {
                return Err(TurnFault::Trapped("owned interpreter did not acknowledge cancellation".to_string()));
            }
        }
        CoreStepOutcome::Cancelled { .. } => {}
        _ => return Err(TurnFault::Trapped("owned interpreter did not acknowledge cancellation".to_string())),
    }
    state.pending = None;
    Ok(())
}

fn owned_wasi_nanoseconds(duration: std::time::Duration) -> Result<u64, PluginHostError> {
    u64::try_from(duration.as_nanos()).map_err(|_| PluginHostError::Plugin("owned WASI monotonic clock exhausted its unsigned nanosecond range".to_string()))
}

fn owned_wasi_monotonic_clock() -> Result<Value, PluginHostError> {
    static ORIGIN: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let nanoseconds = owned_wasi_nanoseconds(ORIGIN.get_or_init(std::time::Instant::now).elapsed())?;
    Ok(Value::I64(i64::from_ne_bytes(nanoseconds.to_ne_bytes())))
}

fn reply_owned_host(state: &mut OwnedInstanceState, call: &HostCall) -> Result<Vec<Value>, PluginHostError> {
    let values = match (call.module.as_str(), call.name.as_str()) {
        ("semio:framework/pure@1.0.0", "now-ms") => vec![Value::I64(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as i64))],
        ("semio:framework/pure@1.0.0", "log") | ("semio:framework/pure@1.0.0", "trace-span") => Vec::new(),
        ("$root", "[context-get-0]") => vec![Value::I32(state.context)],
        ("$root", "[context-set-0]") => {
            let [Value::I32(context)] = call.arguments.as_slice() else { return Err(PluginHostError::Plugin("invalid owned context-set argument".to_string())) };
            state.context = *context;
            Vec::new()
        }
        ("$root", "[waitable-set-new]")
        | ("wasi:io/streams@0.2.0", "[method]output-stream.subscribe")
        | ("wasi:clocks/monotonic-clock@0.2.0", "subscribe-duration")
        | ("wasi:cli/stdin@0.2.0", "get-stdin")
        | ("wasi:cli/stdout@0.2.0", "get-stdout")
        | ("wasi:cli/stderr@0.2.0", "get-stderr") => {
            let resource = state.next_resource;
            state.next_resource = state.next_resource.wrapping_add(1).max(1);
            vec![Value::I32(resource)]
        }
        ("$root", "[waitable-set-poll]") => vec![Value::I32(0)],
        ("$root", "[waitable-join]")
        | ("$root", "[waitable-set-drop]")
        | ("[export]$root", "[task-cancel]")
        | ("wasi:io/poll@0.2.0", "[method]pollable.block")
        | ("wasi:io/error@0.2.0", "[resource-drop]error")
        | ("wasi:io/poll@0.2.0", "[resource-drop]pollable")
        | ("wasi:io/streams@0.2.0", "[resource-drop]input-stream")
        | ("wasi:io/streams@0.2.0", "[resource-drop]output-stream") => Vec::new(),
        ("wasi:clocks/monotonic-clock@0.2.0", "now") => vec![owned_wasi_monotonic_clock()?],
        ("wasi:random/insecure-seed@0.2.9", "insecure-seed") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 0)?, 16)?;
            Vec::new()
        }
        ("wasi:cli/environment@0.2.0", "get-environment") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 0)?, 8)?;
            Vec::new()
        }
        ("wasi:clocks/wall-clock@0.2.0", "now") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 0)?, 16)?;
            Vec::new()
        }
        ("wasi:cli/terminal-stdin@0.2.0", "get-terminal-stdin") | ("wasi:cli/terminal-stdout@0.2.0", "get-terminal-stdout") | ("wasi:cli/terminal-stderr@0.2.0", "get-terminal-stderr") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 0)?, 8)?;
            Vec::new()
        }
        ("wasi:io/poll@0.2.0", "poll") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 2)?, 8)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.check-write") => {
            let pointer = owned_argument_i32(call, 1)?;
            write_owned_zeroes(&mut state.actor, pointer, 16)?;
            write_owned_memory(&mut state.actor, pointer.wrapping_add(8), &65_536u64.to_le_bytes()).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.write") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 3)?, 16)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.blocking-flush") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 1)?, 16)?;
            Vec::new()
        }
        _ => return Err(PluginHostError::Plugin(format!("owned host import {}::{} is unavailable", call.module, call.name))),
    };
    if values.len() != call.results.len() || values.iter().zip(&call.results).any(|(value, expected)| value.value_type() != *expected) {
        return Err(PluginHostError::Plugin(format!("owned host import {}::{} returned the wrong shape", call.module, call.name)));
    }
    Ok(values)
}

fn owned_argument_i32(call: &HostCall, index: usize) -> Result<i32, PluginHostError> {
    match call.arguments.get(index) {
        Some(Value::I32(value)) => Ok(*value),
        _ => Err(PluginHostError::Plugin(format!("owned host import {}::{} argument {index} is not i32", call.module, call.name))),
    }
}

fn write_owned_zeroes(actor: &mut OwnedSemioInstance, pointer: i32, length: usize) -> Result<(), PluginHostError> {
    write_owned_memory(actor, pointer, &vec![0; length]).map_err(turn_fault_host)
}

fn write_owned_memory(actor: &mut OwnedSemioInstance, pointer: i32, bytes: &[u8]) -> Result<(), TurnFault> {
    let start = pointer as u32 as usize;
    let memory = actor.memory_mut().ok_or_else(|| TurnFault::Trapped("owned actor memory is unavailable".to_string()))?;
    let end = start.checked_add(bytes.len()).filter(|end| *end <= memory.len()).ok_or_else(|| TurnFault::Trapped("owned input is outside guest memory".to_string()))?;
    memory[start..end].copy_from_slice(bytes);
    Ok(())
}

fn decode_owned_result<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, TurnFault> {
    let result: Result<T, Vec<u8>> = serde_json::from_slice(bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
    result.map_err(|error| TurnFault::Trapped(String::from_utf8_lossy(&error).into_owned()))
}

fn turn_fault_host(fault: TurnFault) -> PluginHostError {
    match fault {
        TurnFault::Host(error) => error,
        other => PluginHostError::Plugin(other.to_string()),
    }
}

impl From<crate::interpreter::CoreError> for TurnFault {
    fn from(error: crate::interpreter::CoreError) -> Self {
        Self::Trapped(error.to_string())
    }
}

#[cfg(test)]
mod owned_runtime_tests {
    use super::*;

    fn budget() -> Budget {
        Budget { fuel: 500_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
    }

    async fn cancel_to_completion(runtime: &OwnedRuntime, instance: &mut GuestInstance, job: u64) {
        let started = std::time::Instant::now();
        loop {
            match runtime.cancel_job(instance, job).await {
                Ok(()) => return,
                Err(TurnFault::DeadlineExceeded | TurnFault::FuelExhausted) if started.elapsed() < std::time::Duration::from_secs(60) => {}
                Err(error) => panic!("cancel owned job {job}: {error}"),
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn configured_component_executes_owned_describe_reactor_jobs_cancel_and_checkpoint_restore() {
        let Some(path) = std::env::var_os("SEMIO_OWNED_COMPONENT_FIXTURE") else { return };
        let bytes = std::fs::read(path).expect("read owned component fixture");
        let runtime = OwnedRuntime::new();
        let package = PackageRef { package: PackageId("owned-fixture".to_string()), hash: PackageHash([9; 32]) };
        let compiled = runtime.compile(&package, &bytes).await.expect("compile owned fixture");
        let descriptor = runtime.describe(&compiled, budget()).await.expect("execute owned describe");
        assert!(!descriptor.is_empty(), "owned describe returned no bytes");

        let mut resumed = runtime.instantiate(&compiled, RuntimeActorId(40), &[], &budget()).await.expect("instantiate resumable owned fixture");
        let one_instruction = Budget { fuel: 1, ..budget() };
        assert!(matches!(runtime.execute_turn(&mut resumed, &[], one_instruction).await, Err(TurnFault::FuelExhausted)));
        let mid_call = runtime.checkpoint(&mut resumed).await.expect("checkpoint fuel-yielded owned turn");
        runtime.restore(&mut resumed, &mid_call).await.expect("restore fuel-yielded owned turn");
        assert!(runtime.execute_turn(&mut resumed, &[], budget()).await.expect("resume fuel-yielded owned turn").fuel_used > 1);

        let mut cancelled = runtime.instantiate(&compiled, RuntimeActorId(42), &[], &budget()).await.expect("instantiate cancellable owned fixture");
        assert!(matches!(runtime.execute_turn(&mut cancelled, &[], one_instruction).await, Err(TurnFault::FuelExhausted)));
        cancel_to_completion(&runtime, &mut cancelled, 69).await;

        let mut instance = runtime.instantiate(&compiled, RuntimeActorId(41), &[], &budget()).await.expect("instantiate owned fixture");
        let turn = runtime.execute_turn(&mut instance, &[], budget()).await.expect("execute owned empty turn");
        assert!(turn.fuel_used > 0, "owned turn did not report interpreter fuel");

        runtime.start_job(&mut instance, 70, "semio.test-owned-checkpoint", Vec::new()).await.expect("start checkpointed owned job");
        let checkpoint = runtime.checkpoint(&mut instance).await.expect("checkpoint owned instance");
        cancel_to_completion(&runtime, &mut instance, 70).await;
        runtime.restore(&mut instance, &checkpoint).await.expect("restore owned checkpoint");
        assert!(matches!(runtime.step_job(&mut instance, 70, JobBudget { fuel: 10_000_000, deadline_ms: 10_000 }).await.expect("step restored owned job"), JobStep::Failed { .. }));

        runtime.start_job(&mut instance, 71, "semio.test-owned-cancel", Vec::new()).await.expect("start cancellable owned job");
        cancel_to_completion(&runtime, &mut instance, 71).await;
        assert!(matches!(runtime.step_job(&mut instance, 71, JobBudget { fuel: 10_000_000, deadline_ms: 10_000 }).await.expect("step cancelled owned job"), JobStep::Failed { .. }));
    }
}

//#endregion 🧠️OwnedRuntime

//#region 🐎️WasmtimeRuntime
/// 🧬️ The real `impl GuestRuntime for WasmtimeRuntime` — `design-runtime.md` §2. Nested `mod
/// actor_bindings` (mirrors the file's own `mod extension_bindings` idiom below, "wasmtime's
/// `bindgen!` cannot be invoked twice at the same module scope") so this coexists with the OLD
/// `plugin-world`/`extension-world` `bindgen!` calls until the deletion pass — both of which are
/// now unconditionally broken independent of this packet (their `world` names no longer exist in
/// `📜️world.wit`, which declares only `world actor`), a finding recorded in
/// `📓️terra-B1-host-native-report.md`.
pub(crate) mod actor_bindings {
    // 🐛️ `additional_derives` carries NEITHER `Debug` NOR `Clone`, each for its own reason.
    // `Debug`: wasmtime-wit-bindgen's `type_record`/`print_rust_enum` always emit a hand-written
    // `impl core::fmt::Debug` for every WIT `record`/`variant`/`enum` regardless of
    // `additional_derives`, so requesting it produces `E0119: conflicting implementations` for
    // every generated type — it is already available on all of them.
    // `Clone`: MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse) — `world actor` now
    // imports `host-async`, which carries `stream<u8>` (`blob-read`, `http-fetch`'s
    // `http-response.body`). That lowers to `StreamReader<u8>`, a one-shot resource handle that is
    // deliberately not `Clone`, and `additional_derives` applies blanket to every generated type
    // rather than only the plain data records. Nothing in this crate cloned a generated `wit_*`
    // value (verified by grep before removing the derive — every `.clone()` near a `wit_*`
    // constructor is on the KERNEL-side `String`/`Vec<u8>` being moved INTO it).
    //
    // 🧬️ This is the crate's ONE `bindgen!` invocation for `semio:framework` — `⏳️imports/🦀️.rs`'s 24
    // `host-async` implementations re-export this module rather than generating a second, nominally
    // distinct copy of every type (which is what the two-world split forced before the collapse).
    wasmtime::component::bindgen!({
        world: "actor",
        path: "../../../🧬️schema",
    });
}

use actor_bindings::semio::framework::{byte_page as wit_byte_page, capabilities as wit_capabilities, effects as wit_effects, events as wit_events, host_async as wit_host_async, instance_lifetime as wit_lifetime, types as wit_types, ui as wit_ui};
use wasmtime::component::Accessor;
// 🧬️ `reactor`/`jobs` are `export`s of `world actor` (design-runtime.md §2's `execute_turn`/`step_job`
// exports), not `import`s, so their generated bindings live under `exports::` — unlike `pure`'s
// argument/return types (`capabilities`/`effects`/`events`/`types`/`ui`), which sit at the top level.
use actor_bindings::exports::semio::framework::{jobs as wit_jobs, reactor as wit_reactor};

/// 🧬️ `design-runtime.md` §2's slimmed `HostState { plugin_id, actor, caps, effect_sink,
/// asset_map }` — `limiter` is an implementation necessity (`Store::limiter` needs somewhere to
/// read bounds from), not part of the design's literal 5-field list.
struct ActorHostState {
    plugin_id: String,
    #[allow(dead_code)]
    actor: RuntimeActorId,
    #[allow(dead_code)]
    caps: Vec<BrokerCapabilityGrant>,
    /// 🚪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): where `host-async.emit`'s
    /// fire-and-forget effects land mid-turn. Held as the WIT variant, NOT the kernel `Effect`:
    /// `emit` is declared sync in the WIT (a deliberate one-way door), `wit_effect_to_kernel` is
    /// genuinely `async`, and pushing the raw variant lets `execute_turn` do the conversion where
    /// it can actually `.await` — no `block_on` bridge on a wasm host path (R4 forbids one there).
    emit_sink: Vec<wit_effects::Effect>,
    /// 🚪️ The `emit-patch` counterpart. Drained by `execute_turn` alongside `emit_sink`.
    emit_patch_sink: Vec<wit_ui::UiPatch>,
    #[allow(dead_code)]
    asset_map: HashMap<String, Vec<u8>>,
    limiter: BudgetLimiter,
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
}

/// 🌐️ WASI Preview 2 for every real `wasm32-wasip2` component. `world actor` itself imports only
/// `pure` (`log`/`now-ms`/`trace-span`), but the Rust `wasm32-wasip2` target's own runtime shim
/// pulls in `wasi:io/poll` and friends transitively, so any genuinely-built plugin fails
/// instantiation with "component imports instance `wasi:io/poll@0.2.9`, but a matching
/// implementation was not found in the linker" unless the linker carries full WASI. The ctx is the
/// sandboxed default — no inherited stdio, filesystem, network or environment — matching this
/// crate's capability-gated stance; widen per [`BrokerCapabilityGrant`] only when a capability
/// actually needs real WASI access.
impl WasiView for ActorHostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi_ctx, table: &mut self.resource_table }
    }
}

/// 🧬️ `pure` (`📜️wit/📜️pure.wit`) is `world actor`'s ONLY import — `log`/`now-ms`/`trace-span`,
/// none fallible, none async.
impl actor_bindings::semio::framework::pure::Host for ActorHostState {
    // 🚫️async: E1 — `wasmtime::component::bindgen!` generates this `Host` trait from the WIT
    // interface, which declares `log`/`now-ms`/`trace-span` sync (see doc comment above); the
    // trait's signature is external and fixed, not chosen by this repo. See R9/R2 E1.
    fn log(&mut self, level: String, message: String) {
        eprintln!("[actor:{}:{level}] {message}", self.plugin_id);
    }

    fn now_ms(&mut self) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
    }

    fn trace_span(&mut self, name: String) {
        eprintln!("[actor:{}:trace] {name}", self.plugin_id);
    }
}

/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): the seven type-only interfaces
/// (`byte-page`/`instance-lifetime` referenced by `reactor::turn-result`'s `command-ingress`/
/// `lifecycle-receipt`/`ui-patch-receipt` fields).
/// `Actor::add_to_linker` — the whole-world linker call the collapse requires (see
/// `WasmtimeRuntime::new`) — demands a `Host` impl for every interface `wit-parser` surfaces as an
/// import, including the ones present ONLY because an exported function's signature references
/// their types. Those traits declare no methods at all (the schema-parity test's
/// `functional_import_names`/`type_only_import_names` split asserts exactly this distinction), so
/// each impl is empty by construction, not by omission.
impl wit_types::Host for ActorHostState {}
impl wit_capabilities::Host for ActorHostState {}
impl wit_effects::Host for ActorHostState {}
impl wit_events::Host for ActorHostState {}
impl wit_ui::Host for ActorHostState {}
impl wit_byte_page::Host for ActorHostState {}
impl wit_lifetime::Host for ActorHostState {}

/// 🪧️ `ui.wit`'s `resource surface` is an empty MARKER resource — declared purely so the design
/// table's vocabulary has a nominal type, referenced by no function signature in the world (see
/// `interface ui`'s own doc comment). `bindgen!` still generates a `HostSurface` trait with the
/// mandatory `drop`, so this impl exists to satisfy the linker and can never actually be called:
/// no host function hands a `Surface` handle to the guest, so no handle exists to drop.
impl wit_ui::HostSurface for ActorHostState {
    // 🚫️async: E1 — `bindgen!` fixes this signature (the resource-destructor hook wasmtime calls
    // when a guest handle goes out of scope); it is not chosen by this repo. See R9/R2 E1.
    fn drop(&mut self, _rep: wasmtime::component::Resource<wit_ui::Surface>) -> wasmtime::Result<()> {
        Ok(())
    }
}

//#region 🚪️host-async on the poll-backed runtime
/// 🚪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `emit`/`emit-patch`, the two
/// deliberate one-way doors. On THIS runtime they are fully functional — the sinks are drained by
/// `execute_turn` at the end of the same turn and merged into the `TurnResult`, which is precisely
/// the poll world's own delivery mechanism, so an `emit`ed effect reaches the host through exactly
/// the path an effect returned from `poll` would.
impl wit_host_async::Host for ActorHostState {
    // 🚫️async: E1 — `bindgen!` declares `emit`/`emit-patch` sync because the WIT does (see
    // `host-async`'s own doc comment on why these two stay fire-and-forget). Both bodies are a
    // single `Vec::push` with no suspension point, so nothing is lost: the ASYNC half of the work
    // (`wit_effect_to_kernel`) is deferred to `execute_turn`, which can await it properly.
    fn emit(&mut self, value: wit_effects::Effect) {
        self.emit_sink.push(value);
    }

    fn emit_patch(&mut self, patch: wit_ui::UiPatch) {
        self.emit_patch_sink.push(patch);
    }
}

/// 🧯️ The fault every direct `host-async` await resolves to under [`WasmtimeRuntime`] — the
/// poll-backed runtime, whose contract is one call in, one `turn-result` out.
///
/// This is a real semantic boundary, not a stub: a `host-async` import must resolve DURING the
/// guest's own call, but this runtime answers host operations by returning the effect in the
/// `turn-result` and delivering an `event.completed` on a LATER `poll`. There is no point in the
/// turn at which such a future could complete, so failing loudly with a typed fault is the only
/// honest answer — silently parking would deadlock the turn, and trapping would kill the actor.
/// The runtime that DOES serve these awaits is the one built on `⏳️imports/🦀️.rs`'s
/// `AsyncActorHostState` (24 real implementations, dispatching straight onto `AsyncServices`),
/// mounted by the `async-plugin-runtime` packet.
async fn poll_backed_direct_await_fault(name: &str) -> Vec<u8> {
    dsl::encode_fault_bytes(&dsl::Fault::new(
        dsl::FaultOrigin::Os,
        dsl::FaultCode::new("host-async.poll-backed"),
        format!("host-async {name} cannot be awaited directly on the poll-backed WasmtimeRuntime — emit the matching `effect` from `poll` and await its `event.completed` on a later turn"),
    ))
}

/// ⏳️ The 24 awaitable imports. Every one delegates to [`poll_backed_direct_await_fault`] — see its
/// doc for why that is the correct behaviour here rather than a gap.
impl wit_host_async::HostWithStore<ActorHostState> for wasmtime::component::HasSelf<ActorHostState> {
    async fn storage_read(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::StorageReadParams) -> Result<Option<Vec<u8>>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("storage-read").await)
    }

    async fn storage_write(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::StorageWriteParams) -> Result<(), Vec<u8>> {
        Err(poll_backed_direct_await_fault("storage-write").await)
    }

    async fn storage_delete(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::StorageDeleteParams) -> Result<(), Vec<u8>> {
        Err(poll_backed_direct_await_fault("storage-delete").await)
    }

    async fn blob_load(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::BlobLoadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("blob-load").await)
    }

    async fn blob_write(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::BlobWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("blob-write").await)
    }

    async fn blob_read(_accessor: &Accessor<ActorHostState, Self>, _hash: String) -> Result<wasmtime::component::StreamReader<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("blob-read").await)
    }

    async fn http_fetch(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::HttpParams) -> Result<wit_host_async::HttpResponse, Vec<u8>> {
        Err(poll_backed_direct_await_fault("http-fetch").await)
    }

    async fn document_read(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::DocumentReadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("document-read").await)
    }

    async fn document_write(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::DocumentWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("document-write").await)
    }

    async fn link_resolve(_accessor: &Accessor<ActorHostState, Self>, _link: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("link-resolve").await)
    }

    async fn registry_query(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RegistryQueryParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("registry-query").await)
    }

    async fn io_compose(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::IoComposeParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("io-compose").await)
    }

    async fn io_run(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::IoRunParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("io-run").await)
    }

    async fn cache_derive(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::CacheDeriveParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("cache-derive").await)
    }

    async fn cache_read(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::CacheReadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("cache-read").await)
    }

    async fn invoke_extension(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::InvokeExtensionParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("invoke-extension").await)
    }

    async fn open_window(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::OpenWindowParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("open-window").await)
    }

    async fn open_dialog(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::OpenDialogParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("open-dialog").await)
    }

    async fn dispatch_action(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::DispatchActionParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("dispatch-action").await)
    }

    async fn spawn_plugin_instance(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::SpawnPluginInstanceParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("spawn-plugin-instance").await)
    }

    async fn request_file_open(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RequestFileOpenParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("request-file-open").await)
    }

    async fn request_media_frames(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RequestMediaFramesParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("request-media-frames").await)
    }

    async fn request_capability(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RequestCapabilityParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("request-capability").await)
    }

    async fn spawn_job(_accessor: &Accessor<ActorHostState, Self>, _job: u64, _kind: String, _input: Vec<u8>, _placement: wit_effects::JobPlacement) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("spawn-job").await)
    }
}
//#endregion 🚪️host-async on the poll-backed runtime

struct WasmtimeInstanceState {
    store: Store<ActorHostState>,
    bindings: actor_bindings::Actor,
    /// 🪪️ "One actor per app instance is the default" (design-abi.md §4) — minted once at
    /// `instantiate` and used to fill the `instance` field WIT's per-instance events carry but
    /// `semio_framework::kernel::Event`'s own lifecycle variants (`InstanceClose`/`Activate`/
    /// `SuspendRequest`/`CapabilityChanged`/`QuotaChanged`) dropped (kernel's `Event` is already
    /// actor-scoped by construction, so re-carrying the instance id per-event was redundant there).
    instance_id: u32,
}

/// 🐎️ ONE shared `Engine` per process (`build_shared_engine`), one `Store<ActorHostState>` per
/// actor (`instantiate`). Wraps the type-independent primitives in `//#region 🔧️SharedWasmtimeEngine`
/// above — this is where they get consumed.
pub struct WasmtimeRuntime {
    engine: Engine,
    _epoch_ticker: EpochTicker,
    linker: Linker<ActorHostState>,
    cache_root: PathBuf,
    engine_config_hash: [u8; 32],
    next_instance_id: std::sync::atomic::AtomicU32,
}

impl WasmtimeRuntime {
    pub async fn new(cfg: SharedEngineConfig) -> Result<Self, PluginHostError> {
        let (engine, pooling_active) = build_shared_engine(cfg).await?;
        let epoch_ticker = EpochTicker::start(&engine, &plugin_host_worker_pool());
        let mut linker = Linker::new(&engine);
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `Actor::add_to_linker`
        // defines BOTH of the collapsed world's imports in one call (`pure` + `host-async`) —
        // adding only `pure` now leaves 24 imports unresolved and every instantiation fails with
        // "a matching implementation was not found in the linker".
        actor_bindings::Actor::add_to_linker::<ActorHostState, wasmtime::component::HasSelf<ActorHostState>>(&mut linker, |state: &mut ActorHostState| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        // 🌐️ `add_to_linker_async`, not `_sync`: the Store is component-model-async now, and the
        // sync WASI shim installs host functions that cannot be called from an async-lifted guest.
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let engine_config_hash = shared_engine_config_hash(&cfg, pooling_active).await;
        Ok(Self { engine, _epoch_ticker: epoch_ticker, linker, cache_root: default_compiled_cache_root().await, engine_config_hash, next_instance_id: std::sync::atomic::AtomicU32::new(1) })
    }
}

impl GuestRuntime for WasmtimeRuntime {
    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        let cache_path = compiled_cache_path(&self.cache_root, &self.engine_config_hash, &package.hash.0).await;
        if let Some(component) = load_compiled_component(&self.engine, &cache_path).await {
            return Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None });
        }
        let component = Component::from_binary(&self.engine, bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        // 🚫️async: R13/R14 corollary — `let _ = <async call>;` used to suppress the lint while
        // silently dropping this call's future: the on-disk compilation cache write never ran, so
        // `compile` was recompiling from wasm bytes on every single call regardless of
        // `load_compiled_component`'s cache-hit path above. Best-effort by design (a failed cache
        // write must not fail `compile` itself), so the `Result` still discards, only the future
        // is now actually driven.
        let _ = store_compiled_component(&component, &cache_path).await;
        Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None })
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        let component = compiled.component.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no wasmtime Component — built by MockGuestRuntime::compile, not WasmtimeRuntime::compile".to_string()))?;
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let host_state = ActorHostState {
            plugin_id: format!("actor-{}", actor.0),
            actor,
            caps: caps.to_vec(),
            emit_sink: Vec::new(),
            emit_patch_sink: Vec::new(),
            asset_map: HashMap::new(),
            limiter: BudgetLimiter::default(),
            wasi_ctx: WasiCtxBuilder::new().build(),
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&self.engine, host_state);
        store.limiter(|state| &mut state.limiter as &mut dyn ResourceLimiter);
        store.set_fuel(budget.fuel).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        store.set_epoch_deadline(budget.deadline_ms as u64);
        // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-wasmtime-upgrade): wasmtime 22.0.1's
        // `bindgen!`-generated `Actor::instantiate` returned `(Actor, wasmtime::component::Instance)`;
        // wasmtime 47.0.3 dropped the raw `Instance` from the convenience wrapper (it was never used
        // here anyway — every subsequent call goes through `bindings`' own typed accessors) and
        // returns bare `Actor` instead. See `path2.rs`'s expanded bindgen output in
        // `wasmtime-internal-component-macro-47.0.3/tests/expanded/` for the confirmed new shape.
        //
        // 🧬️ B1 world-collapse: `instantiate_async`, not `instantiate` — the world's `host-async`
        // imports are async host functions, and the sync entry point refuses a Store whose linker
        // carries any.
        let bindings = actor_bindings::Actor::instantiate_async(&mut store, component, &self.linker).await.map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(GuestInstance { actor, state: GuestInstanceState::Wasmtime(WasmtimeInstanceState { store, bindings, instance_id }) })
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("execute_turn called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, instance_id } = state;
        store.set_fuel(budget.fuel).map_err(|error| TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;
        store.set_epoch_deadline(budget.deadline_ms as u64);
        let wit_budget = wit_reactor::Budget { fuel: budget.fuel, deadline_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
        // 🚫️async: R10 residue shape 1 — `kernel_event_to_wit` is async, hoisted out of the sync
        // `Iterator::map` closure via a plain loop.
        let mut wit_events: Vec<wit_events::Event> = Vec::with_capacity(events.len());
        let mut wit_command_page = None;
        for event in events {
            if let Event::CommandIngressPage { cursor, bytes } = event {
                if wit_command_page.is_some() || bytes.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES || (bytes.is_empty() && !(cursor.kind == 28 && cursor.item_count == 0)) {
                    return Err(TurnFault::Trapped("turn carries more than one command page or an invalid page size".to_string()));
                }
                wit_command_page = Some(kernel_command_page_to_wit(cursor, bytes));
            } else {
                wit_events.push(kernel_event_to_wit(event, *instance_id).await);
            }
        }
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `poll` is `async func`
        // now, so it is driven through `Store::run_concurrent`'s `Accessor` rather than called
        // directly against `&mut Store` — that is the ONLY shape wasmtime offers for an async-lifted
        // export, and it is what lets the guest suspend on a `host-async` import mid-turn without
        // unwinding the call. Args are owned (moved into the concurrent task), not borrowed.
        let call_result = store.run_concurrent(async |accessor| bindings.semio_framework_reactor().call_poll(accessor, wit_events, wit_command_page, wit_budget).await).await.and_then(|inner| inner);
        let poll_result = match call_result {
            Ok(inner) => inner,
            Err(trap) => {
                let message = trap.to_string();
                let lowered = message.to_ascii_lowercase();
                return Err(if lowered.contains("fuel") {
                    TurnFault::FuelExhausted
                } else if lowered.contains("epoch") || lowered.contains("interrupt") {
                    TurnFault::DeadlineExceeded
                } else {
                    TurnFault::Trapped(message)
                });
            }
        };
        let wit_turn_result = poll_result.map_err(|error| TurnFault::Trapped(format!("{error:?}")))?;
        // 🚪️ B1 world-collapse: everything the guest pushed through `host-async.emit` during THIS
        // turn is delivered on the same `turn-result` as the effects it returned — one merged list,
        // emitted-first (they happened earlier in the turn, by construction).
        let emitted: Vec<wit_effects::Effect> = std::mem::take(&mut store.data_mut().emit_sink);
        // 🚧️ `emit_patch_sink` is drained and DISCARDED, deliberately and visibly: this runtime's
        // `ui_patches` is `Vec::new()` unconditionally (see the note on that field below — the
        // WIT `patch-op` ↔ kernel `PatchOp` path/node encoding is still unagreed), so an emitted
        // patch has nowhere correct to go yet. Draining rather than accumulating keeps a long-lived
        // actor from growing an unbounded sink; the moment `ui_patches` is real, both halves get
        // marshaled together here.
        let _emitted_patches: Vec<wit_ui::UiPatch> = std::mem::take(&mut store.data_mut().emit_patch_sink);
        let mut effects = Vec::with_capacity(emitted.len() + wit_turn_result.effects.len());
        for effect in emitted.into_iter().chain(wit_turn_result.effects) {
            effects.push(wit_effect_to_kernel(effect).await.map_err(TurnFault::Host)?);
        }
        Ok(TurnResult {
            // 🚧️ UI patch marshaling (WIT `patch-op`'s `path: list<u32>` + `node: pack` vs kernel
            // `PatchOp`'s `path: String` + `node: UiNode`) is NOT implemented — a real path/node
            // encoding convention needs to be agreed with A2/A3 first (`📓️terra-B1-host-native-
            // report.md`'s `## blocked-on` — tracked there, not silently dropped).
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects,
            // 👥️ M2 render-plane presence (sol's ruling, 26/08/20): `presence-update.update` is a
            // pack-encoded `ui_contract::PresenceUpdate`, NOT the replication `PresencePeer` the
            // record originally declared — the consumer of a turn result is the renderer, which
            // needs `(surface, node_key)` addressing and a TTL, while the collaboration roster keeps
            // its own channel (`ephemeral_snapshot` out, `AppCommand::Presence`/`adopt_presence`
            // in). Symmetric with the guest's `kernel_presence_update_to_wit`.
            //
            // A malformed entry is SKIPPED rather than failing the turn, matching `AppCommand::
            // Presence`'s own roster-decode convention: presence is best-effort and TTL-scoped, so
            // one bad update must not cost the actor an otherwise valid turn's patches and effects.
            presence: {
                let mut updates = Vec::with_capacity(wit_turn_result.presence.len());
                for entry in wit_turn_result.presence {
                    let Ok(value) = store::pack_rt::decode_wire_value(&entry.update) else { continue };
                    if let Ok(update) = serde_json::from_value::<semio_framework::kernel::PresenceUpdate>(serde_json::Value::from(&value)) {
                        updates.push(update);
                    }
                }
                updates
            },
            next_wake: wit_turn_result.next_wake,
            status: wit_turn_status_to_kernel(wit_turn_result.status).await,
            fuel_used: wit_turn_result.fuel_used,
            command_ingress: wit_command_ingress_to_kernel(wit_turn_result.command_ingress),
            lifecycle_receipt: wit_turn_result.lifecycle_receipt.map(wit_lifecycle_receipt_to_kernel),
            ui_patch_receipt: wit_turn_result.ui_patch_receipt.map(wit_patch_receipt_to_kernel),
        })
    }

    // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): every export below is
    // `async func` in the WIT now, so each goes through `Store::run_concurrent` + `Accessor` — the
    // same shape `execute_turn` uses, and the only one wasmtime offers for an async-lifted export.
    // The doubled `Result` is unchanged in meaning: outer = trap, inner = `plugin-error`.
    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("start_job called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, .. } = state;
        let kind = kind.to_string();
        store
            .run_concurrent(async |accessor| bindings.semio_framework_jobs().call_start_job(accessor, job, kind, input).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(format!("{error:?}")))
    }

    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("step_job called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, .. } = state;
        store.set_fuel(budget.fuel).map_err(|error| TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;
        store.set_epoch_deadline(budget.deadline_ms as u64);
        let wit_budget = wit_jobs::JobBudget { fuel: budget.fuel, deadline_ms: budget.deadline_ms };
        let step = store
            .run_concurrent(async |accessor| bindings.semio_framework_jobs().call_step_job(accessor, job, wit_budget).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(format!("{error:?}")))?;
        Ok(match step {
            wit_jobs::JobStep::Running(bytes) => JobStep::Running { progress: bytes },
            wit_jobs::JobStep::Done(bytes) => JobStep::Done { output: bytes },
            wit_jobs::JobStep::Failed(bytes) => JobStep::Failed { error: bytes },
        })
    }

    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("cancel_job called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, .. } = state;
        // 🧬️ `jobs.wit`'s `cancel-job: async func(job: u64);` has no `result<_, plugin-error>`
        // wrapper (unlike `start-job`/`step-job`), so only the trap-level results can fail: one
        // from `run_concurrent` itself, one from the call.
        store.run_concurrent(async |accessor| bindings.semio_framework_jobs().call_cancel_job(accessor, job).await).await.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(|error| TurnFault::Trapped(error.to_string()))
    }

    async fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("checkpoint called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, .. } = state;
        store
            .run_concurrent(async |accessor| bindings.semio_framework_checkpoint().call_checkpoint(accessor).await)
            .await
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    async fn restore(&self, inst: &mut GuestInstance, state_bytes: &[u8]) -> Result<(), PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("restore called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, .. } = state;
        let state_bytes = state_bytes.to_vec();
        store
            .run_concurrent(async |accessor| bindings.semio_framework_checkpoint().call_restore(accessor, state_bytes).await)
            .await
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    async fn drop_instance(&self, _inst: GuestInstance) {
        // 🗑️ `Store<ActorHostState>` and its `Component` `Arc` drop with `_inst` — nothing else to
        // release; the pooling allocator reclaims the instance's slab on `Store` drop.
    }
}

//#region 🎛️GuestRuntimes
/// 🎛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn): the closed-set enum replacing every
/// `Arc<dyn GuestRuntime>` in this crate (O1/R1 — `async fn` in a trait cannot be `dyn`-dispatched).
/// Hand-written, not `#[dyn_enum]`/`dyn_enum_close!` (`semio-framework-dispatch-macros`): TWO of the
/// three variants are `#[cfg(test)]`-gated, and the macro's own acceptance suite
/// (`📓️terra-dyn-enum-macro-report.md`) never exercised cfg-gated variants at all — a hand-written
/// match-delegation is the lower-risk choice here, exactly the fallback the packet brief
/// anticipates ("if cfg-gated variants defeat the macro, hand-write the enum ... say so").
///
/// `Mock`/`Recording` wrap an `Arc<..>` of their concrete type, not the bare value — many existing
/// tests hold their OWN `Arc<MockGuestRuntime>`/`Arc<RecordingRuntime>` to call inherent,
/// non-`GuestRuntime` methods (`script_turn`, `observed_events`, `last_turn_budget`, ...) on the
/// SAME instance a `PluginInstanceHandle`/`ShardLoop` is driving; a bare-value variant would force
/// every one of those call sites to construct a SECOND, disconnected instance instead of sharing
/// state through the clone `Arc` already gives for free. `Wasmtime` needs no such indirection — no
/// test in this crate keeps a second handle to the SAME `WasmtimeRuntime` for out-of-band inherent
/// calls — so it stays a bare value, one indirection (the enum's own `Arc<GuestRuntimes>` wrapper at
/// every call site) instead of two.
pub enum GuestRuntimes {
    Owned(OwnedRuntime),
    Wasmtime(WasmtimeRuntime),
    // 🔮️ a later packet adds `AsyncActor(AsyncPluginRuntime)` here, backed by wasmtime's
    // `component-model-async` — do not mount `⏳️runtime/🦀️.rs` from this packet (out of scope, needs a
    // rewritten schema; see this ticket's brief).
    #[cfg(test)]
    Mock(Arc<MockGuestRuntime>),
    #[cfg(test)]
    Recording(Arc<shard::RecordingRuntime>),
}

impl GuestRuntime for GuestRuntimes {
    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        match self {
            Self::Owned(r) => r.compile(package, bytes).await,
            Self::Wasmtime(r) => r.compile(package, bytes).await,
            #[cfg(test)]
            Self::Mock(r) => r.compile(package, bytes).await,
            #[cfg(test)]
            Self::Recording(r) => r.compile(package, bytes).await,
        }
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        match self {
            Self::Owned(r) => r.instantiate(compiled, actor, caps, budget).await,
            Self::Wasmtime(r) => r.instantiate(compiled, actor, caps, budget).await,
            #[cfg(test)]
            Self::Mock(r) => r.instantiate(compiled, actor, caps, budget).await,
            #[cfg(test)]
            Self::Recording(r) => r.instantiate(compiled, actor, caps, budget).await,
        }
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault> {
        match self {
            Self::Owned(r) => r.execute_turn(inst, events, budget).await,
            Self::Wasmtime(r) => r.execute_turn(inst, events, budget).await,
            #[cfg(test)]
            Self::Mock(r) => r.execute_turn(inst, events, budget).await,
            #[cfg(test)]
            Self::Recording(r) => r.execute_turn(inst, events, budget).await,
        }
    }

    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        match self {
            Self::Owned(r) => r.start_job(inst, job, kind, input).await,
            Self::Wasmtime(r) => r.start_job(inst, job, kind, input).await,
            #[cfg(test)]
            Self::Mock(r) => r.start_job(inst, job, kind, input).await,
            #[cfg(test)]
            Self::Recording(r) => r.start_job(inst, job, kind, input).await,
        }
    }

    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        match self {
            Self::Owned(r) => r.step_job(inst, job, budget).await,
            Self::Wasmtime(r) => r.step_job(inst, job, budget).await,
            #[cfg(test)]
            Self::Mock(r) => r.step_job(inst, job, budget).await,
            #[cfg(test)]
            Self::Recording(r) => r.step_job(inst, job, budget).await,
        }
    }

    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault> {
        match self {
            Self::Owned(r) => r.cancel_job(inst, job).await,
            Self::Wasmtime(r) => r.cancel_job(inst, job).await,
            #[cfg(test)]
            Self::Mock(r) => r.cancel_job(inst, job).await,
            #[cfg(test)]
            Self::Recording(r) => r.cancel_job(inst, job).await,
        }
    }

    async fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError> {
        match self {
            Self::Owned(r) => r.checkpoint(inst).await,
            Self::Wasmtime(r) => r.checkpoint(inst).await,
            #[cfg(test)]
            Self::Mock(r) => r.checkpoint(inst).await,
            #[cfg(test)]
            Self::Recording(r) => r.checkpoint(inst).await,
        }
    }

    async fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError> {
        match self {
            Self::Owned(r) => r.restore(inst, state).await,
            Self::Wasmtime(r) => r.restore(inst, state).await,
            #[cfg(test)]
            Self::Mock(r) => r.restore(inst, state).await,
            #[cfg(test)]
            Self::Recording(r) => r.restore(inst, state).await,
        }
    }

    async fn drop_instance(&self, inst: GuestInstance) {
        match self {
            Self::Owned(r) => r.drop_instance(inst).await,
            Self::Wasmtime(r) => r.drop_instance(inst).await,
            #[cfg(test)]
            Self::Mock(r) => r.drop_instance(inst).await,
            #[cfg(test)]
            Self::Recording(r) => r.drop_instance(inst).await,
        }
    }
}

impl From<WasmtimeRuntime> for GuestRuntimes {
    fn from(r: WasmtimeRuntime) -> Self {
        Self::Wasmtime(r)
    }
}
impl From<OwnedRuntime> for GuestRuntimes {
    fn from(r: OwnedRuntime) -> Self {
        Self::Owned(r)
    }
}
#[cfg(test)]
impl From<Arc<MockGuestRuntime>> for GuestRuntimes {
    fn from(r: Arc<MockGuestRuntime>) -> Self {
        Self::Mock(r)
    }
}
#[cfg(test)]
impl From<Arc<shard::RecordingRuntime>> for GuestRuntimes {
    fn from(r: Arc<shard::RecordingRuntime>) -> Self {
        Self::Recording(r)
    }
}
//#endregion 🎛️GuestRuntimes

//#region 🔀️EffectEventMarshal
/// 🌉️ `wit_effect_to_kernel`/`kernel_event_to_wit` are the host-side half of `design-abi.md` §2's
/// "WIT variants mirror [`Effect`/`Event`] field-for-field; the guest SDK glue converts between
/// them" — this is that conversion, mirrored for the host. It is NOT fully field-for-field in
/// practice: several real shape gaps between `📜️wit/*.wit` (packet A2) and `🎠️kernel/🦀️.rs`
/// (packet A3) surfaced while writing this and are called out inline + in
/// `📓️terra-B1-host-native-report.md`'s `## blocked-on` (most notably: `📜️wit/📜️effects.wit`'s
/// `io-run` effect has no `Effect::IoRun` counterpart yet).
async fn decode_dsl(bytes: &[u8]) -> Option<DslValue> {
    if bytes.is_empty() {
        return None;
    }
    store::pack_rt::decode_wire_value(bytes).ok()
}

async fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Option<T> {
    if bytes.is_empty() {
        return None;
    }
    serde_json::from_slice(bytes).ok()
}

async fn encode_json<T: serde::Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

async fn wit_message_endpoint_to_kernel(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    match endpoint {
        wit_types::MessageEndpoint::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Backbone(uri) => MessageEndpoint::Backbone { uri },
        wit_types::MessageEndpoint::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Extension(id) => MessageEndpoint::Extension { id },
        wit_types::MessageEndpoint::Topic(name) => MessageEndpoint::Topic { name },
    }
}

async fn kernel_message_endpoint_to_wit(endpoint: &MessageEndpoint) -> wit_types::MessageEndpoint {
    match endpoint {
        MessageEndpoint::Shell { instance } => wit_types::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => wit_types::MessageEndpoint::Backbone(uri.clone()),
        MessageEndpoint::PluginInstance { id } => wit_types::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => wit_types::MessageEndpoint::Extension(id.clone()),
        MessageEndpoint::Topic { name } => wit_types::MessageEndpoint::Topic(name.clone()),
    }
}

async fn kernel_request_outcome_to_wit(result: &RequestOutcome) -> wit_events::CompletionResult {
    match result {
        RequestOutcome::Ok(bytes) => wit_events::CompletionResult::Ok(bytes.clone()),
        RequestOutcome::Err(bytes) => wit_events::CompletionResult::Fault(bytes.clone()),
    }
}

async fn wit_turn_status_to_kernel(status: wit_reactor::TurnStatus) -> TurnStatus {
    match status {
        wit_reactor::TurnStatus::Idle => TurnStatus::Idle,
        wit_reactor::TurnStatus::MoreWork => TurnStatus::MoreWork,
        wit_reactor::TurnStatus::CheckpointReady(checkpoint) => TurnStatus::CheckpointReady { checkpoint: semio_framework_actor::JobCheckpoint { state: checkpoint.state, applied_progress: checkpoint.applied_progress } },
        wit_reactor::TurnStatus::Faulted(bytes) => TurnStatus::Faulted(bytes),
    }
}

fn wit_actor_byte_page_block(bytes: &[u8], block: usize) -> wit_reactor::Block {
    let mut words = [0u64; 8];
    for (index, word) in words.iter_mut().enumerate() {
        let start = block * 64 + index * 8;
        let end = (start + 8).min(bytes.len());
        if start < end {
            let mut fixed = [0u8; 8];
            fixed[..end - start].copy_from_slice(&bytes[start..end]);
            *word = u64::from_le_bytes(fixed);
        }
    }
    wit_reactor::Block { word_0: words[0], word_1: words[1], word_2: words[2], word_3: words[3], word_4: words[4], word_5: words[5], word_6: words[6], word_7: words[7] }
}

fn kernel_command_page_to_wit(cursor: &semio_framework::kernel::CommandPageCursor, page: &semio_framework::kernel::FixedCommandPage) -> wit_reactor::CommandIngressPage {
    let bytes = page.as_slice();
    wit_reactor::CommandIngressPage {
        cursor: kernel_command_cursor_to_wit(cursor),
        page: wit_reactor::Page {
            length: bytes.len() as u32,
            block_00: wit_actor_byte_page_block(bytes, 0),
            block_01: wit_actor_byte_page_block(bytes, 1),
            block_02: wit_actor_byte_page_block(bytes, 2),
            block_03: wit_actor_byte_page_block(bytes, 3),
            block_04: wit_actor_byte_page_block(bytes, 4),
            block_05: wit_actor_byte_page_block(bytes, 5),
            block_06: wit_actor_byte_page_block(bytes, 6),
            block_07: wit_actor_byte_page_block(bytes, 7),
            block_08: wit_actor_byte_page_block(bytes, 8),
            block_09: wit_actor_byte_page_block(bytes, 9),
            block_10: wit_actor_byte_page_block(bytes, 10),
            block_11: wit_actor_byte_page_block(bytes, 11),
            block_12: wit_actor_byte_page_block(bytes, 12),
            block_13: wit_actor_byte_page_block(bytes, 13),
            block_14: wit_actor_byte_page_block(bytes, 14),
            block_15: wit_actor_byte_page_block(bytes, 15),
            block_16: wit_actor_byte_page_block(bytes, 16),
            block_17: wit_actor_byte_page_block(bytes, 17),
            block_18: wit_actor_byte_page_block(bytes, 18),
            block_19: wit_actor_byte_page_block(bytes, 19),
            block_20: wit_actor_byte_page_block(bytes, 20),
            block_21: wit_actor_byte_page_block(bytes, 21),
            block_22: wit_actor_byte_page_block(bytes, 22),
            block_23: wit_actor_byte_page_block(bytes, 23),
            block_24: wit_actor_byte_page_block(bytes, 24),
            block_25: wit_actor_byte_page_block(bytes, 25),
            block_26: wit_actor_byte_page_block(bytes, 26),
            block_27: wit_actor_byte_page_block(bytes, 27),
            block_28: wit_actor_byte_page_block(bytes, 28),
            block_29: wit_actor_byte_page_block(bytes, 29),
            block_30: wit_actor_byte_page_block(bytes, 30),
            block_31: wit_actor_byte_page_block(bytes, 31),
            block_32: wit_actor_byte_page_block(bytes, 32),
            block_33: wit_actor_byte_page_block(bytes, 33),
            block_34: wit_actor_byte_page_block(bytes, 34),
            block_35: wit_actor_byte_page_block(bytes, 35),
            block_36: wit_actor_byte_page_block(bytes, 36),
            block_37: wit_actor_byte_page_block(bytes, 37),
            block_38: wit_actor_byte_page_block(bytes, 38),
            block_39: wit_actor_byte_page_block(bytes, 39),
            block_40: wit_actor_byte_page_block(bytes, 40),
            block_41: wit_actor_byte_page_block(bytes, 41),
            block_42: wit_actor_byte_page_block(bytes, 42),
            block_43: wit_actor_byte_page_block(bytes, 43),
            block_44: wit_actor_byte_page_block(bytes, 44),
            block_45: wit_actor_byte_page_block(bytes, 45),
            block_46: wit_actor_byte_page_block(bytes, 46),
            block_47: wit_actor_byte_page_block(bytes, 47),
            block_48: wit_actor_byte_page_block(bytes, 48),
            block_49: wit_actor_byte_page_block(bytes, 49),
            block_50: wit_actor_byte_page_block(bytes, 50),
            block_51: wit_actor_byte_page_block(bytes, 51),
            block_52: wit_actor_byte_page_block(bytes, 52),
            block_53: wit_actor_byte_page_block(bytes, 53),
            block_54: wit_actor_byte_page_block(bytes, 54),
            block_55: wit_actor_byte_page_block(bytes, 55),
            block_56: wit_actor_byte_page_block(bytes, 56),
            block_57: wit_actor_byte_page_block(bytes, 57),
            block_58: wit_actor_byte_page_block(bytes, 58),
            block_59: wit_actor_byte_page_block(bytes, 59),
            block_60: wit_actor_byte_page_block(bytes, 60),
            block_61: wit_actor_byte_page_block(bytes, 61),
            block_62: wit_actor_byte_page_block(bytes, 62),
            block_63: wit_actor_byte_page_block(bytes, 63),
        },
    }
}

fn wit_command_cursor_to_kernel(cursor: wit_reactor::CommandPageCursor) -> semio_framework::kernel::CommandPageCursor {
    semio_framework::kernel::CommandPageCursor {
        owner: cursor.owner,
        generation: cursor.generation,
        command_index: cursor.command_index,
        command_count: cursor.command_count,
        instance: cursor.instance,
        seq: cursor.seq,
        kind: cursor.kind,
        page_index: cursor.page_index,
        page_count: cursor.page_count,
        item_count: cursor.item_count,
        metadata: cursor.metadata,
    }
}

fn kernel_command_cursor_to_wit(cursor: &semio_framework::kernel::CommandPageCursor) -> wit_reactor::CommandPageCursor {
    wit_reactor::CommandPageCursor {
        owner: cursor.owner,
        generation: cursor.generation,
        command_index: cursor.command_index,
        command_count: cursor.command_count,
        instance: cursor.instance,
        seq: cursor.seq,
        kind: cursor.kind,
        page_index: cursor.page_index,
        page_count: cursor.page_count,
        item_count: cursor.item_count,
        metadata: cursor.metadata,
    }
}

fn wit_command_ingress_to_kernel(status: wit_reactor::CommandIngressStatus) -> semio_framework::kernel::CommandIngressStatus {
    let cursor = wit_command_cursor_to_kernel(status.cursor);
    match status.kind {
        0 => semio_framework::kernel::CommandIngressStatus::Idle,
        1 => semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor),
        2 => semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
        3 => semio_framework::kernel::CommandIngressStatus::CommandPending(cursor),
        4 => semio_framework::kernel::CommandIngressStatus::CommandComplete(cursor),
        5 => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: status.fault },
        _ => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-ingress-kind".to_vec() },
    }
}

/// 🎁️ Every effect that carries a `req: request-id` becomes `RequestId(req)` — one line, so it's
/// inlined at each call site below rather than its own helper.
/// 🐛️ Guest → host: WIT `effect` (`📜️wit/📜️effects.wit`) to `semio_framework::kernel::Effect`.
/// `Err` is returned (never a silently-wrong `Effect`) for `io-run` — the one variant with no
/// kernel counterpart yet (`## blocked-on` in the report).
async fn wit_effect_to_kernel(effect: wit_effects::Effect) -> Result<Effect, PluginHostError> {
    use wit_effects::Effect as E;
    Ok(match effect {
        E::SendMessage(inner) => Effect::SendMessage { target: kernel_message_endpoint_to_wit_reverse(inner.target).await, payload: inner.payload },
        E::PublishEvent(inner) => Effect::PublishEvent { topic: inner.topic, payload: inner.payload },
        E::BlobLoad(inner) => Effect::BlobLoad { req: RequestId(inner.req), hash: inner.params.hash },
        E::BlobWrite(inner) => Effect::BlobWrite {
            req: RequestId(inner.req),
            media_type: decode_json(&inner.params.media_type).await.unwrap_or(semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value }),
            bytes: inner.params.bytes,
        },
        E::HttpRequest(inner) => Effect::HttpRequest { req: RequestId(inner.req), method: inner.params.method, url: inner.params.url, headers: inner.params.headers, body: inner.params.body, stream: inner.params.streaming },
        E::DocumentRead(inner) => Effect::DocumentRead { req: RequestId(inner.req), doc: ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane },
        E::DocumentWrite(inner) => Effect::DocumentWrite { req: RequestId(inner.req), doc: ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane, ops: inner.params.ops },
        E::LinkResolve(inner) => Effect::LinkResolve { req: RequestId(inner.req), link: String::from_utf8_lossy(&inner.link).into_owned() },
        E::RegistryQuery(inner) => Effect::RegistryQuery { req: RequestId(inner.req), kind: inner.params.kind, filter: decode_dsl(&inner.params.filter).await },
        E::IoCompose(inner) => Effect::IoCompose { req: RequestId(inner.req), key: String::from_utf8_lossy(&inner.params.key).into_owned(), sources: decode_json(&inner.params.sources).await.unwrap_or_default() },
        // 🚧️ blocked-on-A3: no `Effect::IoRun` variant exists yet (`## blocked-on` in the report).
        E::IoRun(_inner) => return Err(PluginHostError::Plugin("effect io-run has no semio_framework::kernel::Effect variant yet (needs A3 to add Effect::IoRun) — see 📓️terra-B1-host-native-report.md".to_string())),
        E::CacheDerive(inner) => Effect::CacheDerive { req: RequestId(inner.req), engine_id: inner.params.engine_id, input: inner.params.input },
        E::CacheRead(inner) => Effect::CacheRead { req: RequestId(inner.req), engine_id: inner.params.engine_id, key: String::from_utf8_lossy(&inner.params.key).into_owned() },
        E::OpenWindow(inner) => Effect::OpenWindow { req: RequestId(inner.req), kind: WindowKindId(inner.params.kind), params: decode_dsl(&inner.params.params).await.unwrap_or(DslValue::Null) },
        E::CloseWindow(inner) => Effect::CloseWindow { window: WindowHandle(inner.window as u128) },
        // 🚫️async: R10 residue shape 1 — `decode_dsl` is async, hoisted out of `Option::and_then`'s
        // sync closure below (and at every other `.args.and_then(|bytes| decode_dsl(&bytes))` site
        // in this match).
        E::DispatchAction(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::DispatchAction { req: RequestId(inner.req), action: inner.params.action, args, delay_ms: inner.params.delay_ms }
        }
        E::InvokeExtension(inner) => Effect::InvokeExtension { req: RequestId(inner.req), extension_id: inner.params.extension_id, capability: inner.params.capability, request_json: String::from_utf8_lossy(&inner.params.payload).into_owned() },
        E::Notify(inner) => Effect::Notify { message: inner.message },
        E::ClipboardWrite(inner) => Effect::ClipboardWrite { fragment: decode_json(&inner.fragment).await.ok_or_else(|| PluginHostError::Plugin("clipboard-write-effect.fragment failed to decode as JSON ClipboardFragment".to_string()))? },
        E::Navigate(inner) => Effect::Navigate { uri: inner.uri },
        E::OpenExternalUrl(inner) => Effect::OpenExternalUrl { url: inner.url },
        E::SetPanel(inner) => Effect::SetPanel { panel_json: inner.panel_json },
        E::SetActiveUtility(inner) => Effect::SetActiveUtility { window_id: inner.window_id, utility_id: inner.utility_id },
        E::SetActiveTool(inner) => Effect::SetActiveTool { tool_id: inner.tool_id },
        E::ReplayShellCommand(inner) => {
            let args = match inner.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::ReplayShellCommand { action_id: inner.action_id, args }
        }
        E::SpawnPluginInstance(inner) => {
            Effect::SpawnPluginInstance { req: RequestId(inner.req), plugin_id: inner.params.plugin_id, app_id: inner.params.app_id, os_instance_id: inner.params.os_instance_id, label: inner.params.label, document_json: inner.params.document_json }
        }
        E::OpenPluginInstance(inner) => Effect::OpenPluginInstance { plugin_id: inner.plugin_id, app_id: inner.app_id, os_instance_id: inner.os_instance_id },
        E::OpenDialog(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::OpenDialog { req: RequestId(inner.req), dialog_id: inner.params.dialog_id, args }
        }
        E::IconRenderExport(inner) => Effect::IconRenderExport { items: decode_json(&inner.items).await.unwrap_or_default() },
        E::DownloadMediaExport(inner) => Effect::DownloadMediaExport { filename: inner.filename, mime_type: inner.mime_type, data: inner.data, encoding: inner.encoding },
        E::RequestFileOpen(inner) => Effect::RequestFileOpen { req: RequestId(inner.req), accept: inner.params.accept, read_as: inner.params.read_as, import_action: String::new(), multiple: inner.params.multiple },
        E::RequestMediaFrames(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::RequestMediaFrames {
                req: RequestId(inner.req),
                accept: inner.params.accept,
                frame_action: String::new(),
                done_action: String::new(),
                fallback_action: String::new(),
                sample_stride: inner.params.sample_stride,
                max_frames: inner.params.max_frames,
                max_long_edge_px: inner.params.max_long_edge_px,
                fps_hint: inner.params.fps_hint,
                // 🧬️ A2b narrowed `request-media-frames-effect.payload` from `option<pack>` to
                // `option<string>` (correctly honoring the kernel as SSOT) — already a `String`, no
                // decode needed.
                payload: inner.params.payload,
                args,
            }
        }
        E::LoadDocument(inner) => Effect::LoadDocument { pack: inner.doc_pack, spr: inner.spr },
        E::RequestSync => Effect::RequestSync,
        E::SetTimer(inner) => Effect::SetTimer { id: inner.id, after_ms: inner.after_ms as u64, repeat: inner.repeat },
        E::SpawnJob(inner) => Effect::SpawnJob {
            job: inner.job,
            kind: inner.kind,
            input: inner.input,
            placement: match inner.placement {
                wit_effects::JobPlacement::Inline => JobPlacement::Inline,
                wit_effects::JobPlacement::Isolated => JobPlacement::Isolated,
                wit_effects::JobPlacement::Exclusive => JobPlacement::Exclusive,
            },
        },
        E::CancelJob(inner) => Effect::CancelJob { job: inner.job },
        E::Respond(inner) => Effect::Respond {
            req: RequestId(inner.req),
            result: match inner.outcome {
                wit_effects::RespondResult::Ok(bytes) => RequestOutcome::Ok(bytes),
                wit_effects::RespondResult::Fault(bytes) => RequestOutcome::Err(bytes),
            },
        },
        E::StorageRead(inner) => Effect::StorageRead { req: RequestId(inner.req), key: inner.params.key },
        E::StorageWrite(inner) => Effect::StorageWrite { req: RequestId(inner.req), key: inner.params.key, bytes: inner.params.value },
        E::StorageDelete(inner) => Effect::StorageDelete { req: RequestId(inner.req), key: inner.params.key },
        E::RequestCapability(inner) => {
            Effect::RequestCapability { req: RequestId(inner.req), capability: CapabilityRequest { id: CapabilityId(inner.params.id), scope: inner.params.scope, reason: inner.params.reason, optional: inner.params.optional } }
        }
        E::ReleaseCapability(inner) => Effect::ReleaseCapability { id: CapabilityId(inner.id) },
        E::Subscribe(inner) => Effect::Subscribe { topic: inner.topic },
        E::Unsubscribe(inner) => Effect::Unsubscribe { topic: inner.topic },
    })
}

/// 🐛️ `SendMessageEffect.target` is `wit_types::MessageEndpoint` (from the `types` interface via
/// `use types.{message-endpoint}` in `effects.wit`) — same generated Rust type as
/// `wit_message_endpoint_to_kernel` above takes, just named to keep the giant match arm list above
/// readable.
async fn kernel_message_endpoint_to_wit_reverse(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    wit_message_endpoint_to_kernel(endpoint).await
}

async fn wit_surface_ref(instance_id: u32, surface: &str) -> wit_ui::SurfaceRef {
    let body_key = surface.split_once(':').map(|(_, body_key)| body_key).unwrap_or(surface);
    wit_ui::SurfaceRef { instance: instance_id, surface: body_key.to_owned() }
}

/// 🏁️ Host → guest: `semio_framework::kernel::Event` to WIT `event` (`📜️wit/📜️events.wit`).
/// `instance_id` fills the WIT `instance` field several kernel lifecycle variants dropped (see
/// `WasmtimeInstanceState::instance_id`'s docstring).
fn kernel_lifetime_to_wit(value: semio_framework::kernel::ActorInstanceLifetime) -> wit_lifetime::Lifetime {
    wit_lifetime::Lifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

fn wit_lifetime_to_kernel(value: wit_lifetime::Lifetime) -> semio_framework::kernel::ActorInstanceLifetime {
    semio_framework::kernel::ActorInstanceLifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

fn kernel_patch_receipt_to_wit(value: semio_framework::kernel::ActorUiPatchReceipt) -> wit_lifetime::UiPatchReceipt {
    wit_lifetime::UiPatchReceipt { lifetime: kernel_lifetime_to_wit(value.lifetime), patch_sequence: value.patch_sequence }
}

fn wit_patch_receipt_to_kernel(value: wit_lifetime::UiPatchReceipt) -> semio_framework::kernel::ActorUiPatchReceipt {
    semio_framework::kernel::ActorUiPatchReceipt { lifetime: wit_lifetime_to_kernel(value.lifetime), patch_sequence: value.patch_sequence }
}

fn kernel_lifecycle_receipt_to_wit(value: semio_framework::kernel::ActorInstanceLifecycleReceipt) -> wit_lifetime::Receipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        R::Captured { lifetime, request_sequence } => wit_lifetime::Receipt::Captured(wit_lifetime::CapturedReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence }),
        R::Accepted { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Accepted(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
        R::Retired { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Retired(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
    }
}

fn wit_lifecycle_receipt_to_kernel(value: wit_lifetime::Receipt) -> semio_framework::kernel::ActorInstanceLifecycleReceipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        wit_lifetime::Receipt::Captured(value) => R::Captured { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence },
        wit_lifetime::Receipt::Accepted(value) => R::Accepted { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
        wit_lifetime::Receipt::Retired(value) => R::Retired { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
    }
}

async fn kernel_event_to_wit(event: &Event, instance_id: u32) -> wit_events::Event {
    match event {
        Event::InstanceOpen { request, app_id, actor, config, assets, capabilities, quotas } => {
            // 🚫️async: R10 residue shape 1 — `kernel_broker_grant_to_wit` is async, hoisted out of
            // the sync `Iterator::map` via a plain loop.
            let mut wit_capabilities = Vec::with_capacity(capabilities.len());
            for capability in capabilities {
                wit_capabilities.push(kernel_broker_grant_to_wit(capability).await);
            }
            wit_events::Event::InstanceOpen(wit_events::InstanceOpenEvent {
                instance: request.instance_id,
                activation_generation: request.activation_generation,
                request_sequence: request.request_sequence,
                app_id: app_id.0.clone(),
                actor: actor.clone(),
                config: config.clone(),
                assets: assets.clone(),
                capabilities: wit_capabilities,
                quotas: encode_json(quotas).await,
            })
        }
        Event::InstanceClose(request) => wit_events::Event::InstanceClose(wit_lifetime::CloseRequest { lifetime: kernel_lifetime_to_wit(request.lifetime), request_sequence: request.request_sequence }),
        Event::InstanceLifecycleAck(ack) => wit_events::Event::InstanceLifecycleAck(kernel_lifecycle_receipt_to_wit(ack.receipt)),
        Event::Activate { reason } => wit_events::Event::Activate(wit_events::ActivateEvent { instance: instance_id, reason: kernel_activation_event_to_wit(reason).await }),
        Event::SuspendRequest => wit_events::Event::SuspendRequest(wit_events::SuspendRequestEvent { instance: instance_id }),
        Event::CapabilityChanged { change } => wit_events::Event::CapabilityChanged(wit_events::CapabilityChangedEvent { instance: instance_id, change: kernel_capability_change_to_wit(change).await }),
        Event::QuotaChanged { quotas } => wit_events::Event::QuotaChanged(wit_events::QuotaChangedEvent { instance: instance_id, quotas: encode_json(quotas).await }),
        Event::CommandIngressPage { .. } => unreachable!("command pages are lifted through reactor.poll's dedicated page argument"),
        Event::UiIntent { instance, intent } => wit_events::Event::UiIntent(wit_events::UiIntentEvent { instance: instance.0.parse().unwrap_or(instance_id), intent: intent.clone() }),
        Event::SurfaceVisible { surface } => wit_events::Event::SurfaceVisible(wit_events::SurfaceVisibleEvent { surface: wit_surface_ref(instance_id, surface).await }),
        Event::SurfaceHidden { surface } => wit_events::Event::SurfaceHidden(wit_events::SurfaceHiddenEvent { surface: wit_surface_ref(instance_id, surface).await }),
        Event::SurfaceResized { surface, width, height } => wit_events::Event::SurfaceResized(wit_events::SurfaceResizedEvent { surface: wit_surface_ref(instance_id, surface).await, width: *width, height: *height }),
        Event::PatchAck { receipt, surface, revision } => wit_events::Event::PatchAck(wit_events::PatchAckEvent { receipt: kernel_patch_receipt_to_wit(*receipt), surface: wit_surface_ref(instance_id, surface).await, revision: *revision }),
        Event::PatchRejected { receipt, surface, revision, reason } => wit_events::Event::PatchRejected(wit_events::PatchRejectedEvent { receipt: kernel_patch_receipt_to_wit(*receipt), surface: wit_surface_ref(instance_id, surface).await, revision: *revision, reason: reason.clone() }),
        Event::Completed { req, result } => wit_events::Event::Completed(wit_events::CompletedEvent { req: req.0, outcome: kernel_request_outcome_to_wit(result).await }),
        Event::HttpChunk { req, bytes, done } => wit_events::Event::HttpChunk(wit_events::HttpChunkEvent { req: req.0, params: wit_events::HttpChunkParams { bytes: bytes.clone(), done: *done } }),
        Event::JobProgress { job, progress } => wit_events::Event::JobProgress(wit_events::JobProgressEvent { job: *job, progress: progress.clone().unwrap_or_default() }),
        Event::JobCompleted { job, result } => wit_events::Event::JobCompleted(wit_events::JobCompletedEvent { job: *job, outcome: kernel_request_outcome_to_wit(result).await }),
        Event::Message { source, payload } => wit_events::Event::Message(wit_events::MessageEvent { source: kernel_message_endpoint_to_wit(source).await, payload: payload.clone() }),
        Event::Timer { id } => wit_events::Event::Timer(wit_events::TimerEvent { id: *id }),
        Event::Wake => wit_events::Event::Wake,
        // 🐛️ WIT `request-event.from` was renamed `origin` — `from` is WIT-reserved (the SAME
        // reserved-keyword class B1's report already fixed for `stream`/`result`; this one was
        // fixed by A2 between B1's last pass and now, per the packet brief's "guest side is green").
        Event::Request { req, from, capability, payload } => {
            wit_events::Event::Request(wit_events::RequestEvent { req: req.0, params: wit_events::RequestParams { origin: kernel_message_endpoint_to_wit(from).await, capability: capability.clone(), payload: payload.clone() } })
        }
    }
}

async fn kernel_activation_event_to_wit(reason: &semio_framework::kernel::ActivationEvent) -> wit_events::ActivationEvent {
    use semio_framework::kernel::ActivationEvent as A;
    match reason {
        A::OnCommand { id } => wit_events::ActivationEvent::OnCommand(id.clone()),
        A::OnViewVisible { id } => wit_events::ActivationEvent::OnViewVisible(id.clone()),
        A::OnFileType { ext } => wit_events::ActivationEvent::OnFileType(ext.clone()),
        A::OnArtifactKind { kind } => wit_events::ActivationEvent::OnArtifactKind(kind.clone()),
        A::OnExtensionRequest { point } => wit_events::ActivationEvent::OnExtensionRequest(point.clone()),
        A::OnStartupFinished => wit_events::ActivationEvent::OnStartupFinished,
    }
}

async fn kernel_capability_change_to_wit(change: &semio_framework::kernel::CapabilityChange) -> wit_capabilities::CapabilityChange {
    use semio_framework::kernel::CapabilityChange as C;
    match change {
        C::Granted { id: _, grant } => wit_capabilities::CapabilityChange::Granted(kernel_broker_grant_to_wit(grant).await),
        C::Revoked { id } => wit_capabilities::CapabilityChange::Revoked(id.0.clone()),
        C::Narrowed { id: _, grant } => wit_capabilities::CapabilityChange::Narrowed(kernel_broker_grant_to_wit(grant).await),
    }
}

async fn kernel_broker_grant_to_wit(grant: &BrokerCapabilityGrant) -> wit_capabilities::CapabilityGrant {
    wit_capabilities::CapabilityGrant { token: wit_capabilities::CapabilityToken { id: grant.id.0.clone(), token: grant.token.0 as u64 }, scope: grant.scope.clone(), expires_ms: grant.expires_ms.map(|value| value as i64) }
}
//#endregion 🔀️EffectEventMarshal

#[cfg(test)]
mod wasmtime_runtime_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn compile_accepts_a_real_component_and_caches_it() {
        let wasm_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        if !wasm_path.exists() {
            return;
        }
        let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
        let bytes = std::fs::read(wasm_path).expect("read real stdio.wasm");
        let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([9u8; 32]) };
        let compiled = runtime.compile(&package, &bytes).await.expect("a real wasip2 component compiles even though it does not export the new `actor` world yet");
        assert!(compiled.component.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn instantiate_rejects_a_component_that_does_not_export_the_actor_world() {
        // 🧬️ No `.wasm` in this repo exports `world actor` yet (A2's guest SDK rewrite / the W3
        // plugin migrations haven't landed) — this asserts the HONEST negative: `instantiate`
        // rejects a real, valid, but wrong-ABI component rather than silently mis-binding it.
        let wasm_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        if !wasm_path.exists() {
            return;
        }
        let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
        let bytes = std::fs::read(wasm_path).expect("read real stdio.wasm");
        let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([10u8; 32]) };
        let compiled = runtime.compile(&package, &bytes).await.expect("compiles as a component");
        let budget = Budget { fuel: 1_000_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let error = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).await.expect_err("stdio.wasm does not export `reactor`/`jobs`/`checkpoint`/`describe`");
        let _ = error;
    }

    #[semio_framework_async_macros::async_test]
    async fn wit_turn_status_conversion_is_a_plain_rename() {
        assert_eq!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::Idle).await, TurnStatus::Idle);
        assert_eq!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::MoreWork).await, TurnStatus::MoreWork);
        assert!(matches!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::Faulted(vec![1, 2, 3])).await, TurnStatus::Faulted(bytes) if bytes == vec![1, 2, 3]));
    }

    #[semio_framework_async_macros::async_test]
    async fn message_endpoint_round_trips_through_wit_and_back() {
        let original = MessageEndpoint::Topic { name: "os.runtime.metrics".to_string() };
        let wit = kernel_message_endpoint_to_wit(&original);
        let back = wit_message_endpoint_to_kernel(wit.await);
        assert_eq!(original, back.await);
    }

    #[semio_framework_async_macros::async_test]
    async fn io_run_effect_is_a_reported_error_not_a_silent_mismap() {
        let effect = wit_effects::Effect::IoRun(wit_effects::IoRunEffect { req: 1, params: wit_effects::IoRunParams { source: "a".to_string(), target: "b".to_string(), payload: vec![] } });
        let result = wit_effect_to_kernel(effect).await;
        assert!(result.is_err(), "io-run must surface as an error until Effect::IoRun exists");
    }
}
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime

//#region 🔀️PostTurnRelay
/// ⛽️ Every cold relay turn stays inside the user-visible interactive slice. The guest job owns its
/// persistent state; the host grants one `step-job` call per shared-pool closure.
const RELAY_JOB_BUDGET: JobBudget = JobBudget { fuel: semio_framework_job::USER_VISIBLE_LANE_FUEL, deadline_ms: (semio_framework_job::USER_VISIBLE_LANE_WALL_US / 1_000) as u32 };

/// 🔁️ One retained-waker future polled once per finite shared-pool turn.
struct GuestRelayPoolFuture {
    pool: WorkerPool,
    lane: Lane,
    future: Mutex<Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>>>,
    failure_handler: Mutex<Option<Box<dyn FnOnce(GuestRelayPoolFailure) + Send + 'static>>>,
    scheduled: AtomicBool,
    wake_requested: AtomicBool,
    complete: AtomicBool,
}

#[derive(Clone, Copy)]
enum GuestRelayPoolFailure {
    FuturePanicked,
    Admission(semio_framework_async::WorkerSubmitErrorKind),
}

impl GuestRelayPoolFuture {
    fn spawn_recoverable(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static, failure_handler: impl FnOnce(GuestRelayPoolFailure) + Send + 'static) {
        Self::spawn_inner(pool, lane, future, Box::new(failure_handler));
    }

    fn spawn_inner(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static, failure_handler: Box<dyn FnOnce(GuestRelayPoolFailure) + Send + 'static>) {
        let task = Arc::new(Self { pool, lane, future: Mutex::new(Some(Box::pin(future))), failure_handler: Mutex::new(Some(failure_handler)), scheduled: AtomicBool::new(false), wake_requested: AtomicBool::new(false), complete: AtomicBool::new(false) });
        task.schedule();
    }

    fn schedule(self: &Arc<Self>) {
        if self.complete.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        if self.pool.is_shutdown() {
            self.fail(GuestRelayPoolFailure::Admission(semio_framework_async::WorkerSubmitErrorKind::Shutdown));
            return;
        }
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if self.scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
            let task = Arc::clone(self);
            if let Err(error) = self.pool.try_submit(self.lane, Box::new(move || task.poll_once())) {
                let kind = error.kind();
                let retry = Arc::clone(self);
                drop(error.into_job());
                match kind {
                    semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated => {
                        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
                            retry.scheduled.store(false, std::sync::atomic::Ordering::Release);
                            retry.schedule();
                        });
                    }
                    semio_framework_async::WorkerSubmitErrorKind::Shutdown | semio_framework_async::WorkerSubmitErrorKind::Poisoned => self.fail(GuestRelayPoolFailure::Admission(kind)),
                }
            }
        }
    }

    fn fail(&self, failure: GuestRelayPoolFailure) {
        if self.complete.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        self.future.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if let Some(handler) = self.failure_handler.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler(failure)));
        }
    }

    fn poll_once(self: Arc<Self>) {
        self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
        let waker = std::task::Waker::from(Arc::clone(&self));
        let mut context = std::task::Context::from_waker(&waker);
        let poll = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut future = self.future.lock().expect("guest relay future lock poisoned");
            future.as_mut().map_or(std::task::Poll::Ready(()), |future| std::future::Future::poll(future.as_mut(), &mut context))
        }));
        match poll {
            Ok(std::task::Poll::Pending) => {
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                if self.wake_requested.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    self.schedule();
                }
            }
            Ok(std::task::Poll::Ready(())) => {
                self.complete.store(true, std::sync::atomic::Ordering::Release);
                self.future.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            }
            Err(_) => {
                self.fail(GuestRelayPoolFailure::FuturePanicked);
            }
        }
    }
}

impl std::task::Wake for GuestRelayPoolFuture {
    fn wake(self: Arc<Self>) {
        self.schedule();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.schedule();
    }
}

enum GuestRelayRequest {
    Start { kind: String, input: Vec<u8> },
    Step,
    Cancel,
}

#[derive(Clone, Copy)]
enum GuestRelayRequestKind {
    Start,
    Step,
    Cancel,
}

impl GuestRelayRequest {
    fn kind(&self) -> GuestRelayRequestKind {
        match self {
            Self::Start { .. } => GuestRelayRequestKind::Start,
            Self::Step => GuestRelayRequestKind::Step,
            Self::Cancel => GuestRelayRequestKind::Cancel,
        }
    }
}

enum GuestRelayCompletion {
    Started(Result<(), TurnFault>),
    Stepped(Result<GuestRelayStepCompletion, TurnFault>),
    Cancelled,
    Rejected(GuestRelayOwnedBytes),
    Fault(GuestRelayOwnedBytes),
    TerminalFault(GuestRelayOwnedBytes),
}

enum GuestRelayStepCompletion {
    Running { progress: Option<GuestRelayOwnedBytes> },
    Done { output: GuestRelayOwnedBytes },
    Failed { error: GuestRelayOwnedBytes },
}

impl GuestRelayStepCompletion {
    fn from_guest(step: JobStep) -> Self {
        match step {
            JobStep::Running { progress } => Self::Running { progress: progress.map(GuestRelayOwnedBytes::new) },
            JobStep::Done { output } => Self::Done { output: GuestRelayOwnedBytes::new(output) },
            JobStep::Failed { error } => Self::Failed { error: GuestRelayOwnedBytes::new(error) },
        }
    }
}

struct GuestRelayOwnedBytes {
    source: Vec<u8>,
}

impl GuestRelayOwnedBytes {
    fn new(source: Vec<u8>) -> Self {
        Self { source }
    }

    fn into_source(self) -> Vec<u8> {
        self.source
    }
}

#[cfg(test)]
async fn wait_for_scripted_guest_relay_release(runtime: &GuestRuntimes, completion: &GuestRelayCompletion) {
    let GuestRuntimes::Mock(mock) = runtime else {
        return;
    };
    let barrier = match completion {
        GuestRelayCompletion::Started(Err(_)) => mock.take_relay_start_failure_release(),
        GuestRelayCompletion::Stepped(Err(_)) => mock.take_relay_step_failure_release(),
        _ => None,
    };
    if let Some(barrier) = barrier {
        barrier.wait().await;
    }
}

#[derive(Clone)]
struct GuestRelayCompletionSender {
    slot: Arc<GuestRelayCompletionSlot>,
}

impl GuestRelayCompletionSender {
    fn new(slot: Arc<GuestRelayCompletionSlot>) -> Self {
        Self { slot }
    }

    fn send(&self, completion: GuestRelayCompletion) {
        let mut value = self.slot.value.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if value.is_none() {
            *value = Some(completion);
            self.slot.wake.store(true, std::sync::atomic::Ordering::Release);
            if let Some(waker) = self.slot.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                waker.wake();
            }
        }
    }
}

struct GuestRelayCompletionSlot {
    value: Mutex<Option<GuestRelayCompletion>>,
    wake: AtomicBool,
    waker: Mutex<Option<std::task::Waker>>,
}

impl GuestRelayCompletionSlot {
    fn new() -> Self {
        Self { value: Mutex::new(None), wake: AtomicBool::new(false), waker: Mutex::new(None) }
    }

    fn try_take(&self) -> Option<GuestRelayCompletion> {
        let completion = self.value.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if completion.is_some() {
            self.wake.store(false, std::sync::atomic::Ordering::Release);
        }
        completion
    }

    fn register_wake(&self, waker: &std::task::Waker) {
        if self.wake.load(std::sync::atomic::Ordering::Acquire) {
            waker.wake_by_ref();
            return;
        }
        *self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(waker.clone());
        if self.wake.load(std::sync::atomic::Ordering::Acquire) {
            if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                waker.wake();
            }
        }
    }
}

enum GuestInstanceSlot {
    Available(GuestInstance),
    Leased,
    CleanupPending { instance: Option<GuestInstance>, detail: Vec<u8> },
    Quarantined { instance: GuestInstance, detail: Vec<u8> },
}

impl GuestInstanceSlot {
    #[cfg(test)]
    fn is_available(&self) -> bool {
        matches!(self, Self::Available(_))
    }

    #[cfg(test)]
    fn is_quarantined(&self) -> bool {
        matches!(self, Self::Quarantined { .. })
    }

    #[cfg(test)]
    fn is_cleanup_pending(&self) -> bool {
        matches!(self, Self::CleanupPending { .. })
    }

    fn admission_fault(&self) -> Option<Vec<u8>> {
        match self {
            Self::CleanupPending { detail, .. } | Self::Quarantined { detail, .. } => Some(detail.clone()),
            Self::Leased => Some(b"plugin instance is busy retiring a prior relay".to_vec()),
            Self::Available(_) => None,
        }
    }
}

enum GuestInstanceLeaseOrigin {
    Mounted,
    Cleanup,
}

enum GuestInstanceLeaseDisposition {
    Available,
    CleanupResolved,
    CleanupPending(Vec<u8>),
    Quarantined(Vec<u8>),
}

struct GuestInstanceLease {
    slot: Arc<Mutex<GuestInstanceSlot>>,
    guest: Option<GuestInstance>,
    origin: GuestInstanceLeaseOrigin,
    disposition: GuestInstanceLeaseDisposition,
}

impl GuestInstanceLease {
    fn acquire(slot: Arc<Mutex<GuestInstanceSlot>>, unwind_detail: Vec<u8>) -> Result<Self, Vec<u8>> {
        let state = {
            let mut state = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::replace(&mut *state, GuestInstanceSlot::Leased)
        };
        match state {
            GuestInstanceSlot::Available(guest) => Ok(Self { slot, guest: Some(guest), origin: GuestInstanceLeaseOrigin::Mounted, disposition: GuestInstanceLeaseDisposition::CleanupPending(unwind_detail) }),
            GuestInstanceSlot::Leased => {
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::Leased;
                Err(b"plugin instance missing behind an acquired relay permit".to_vec())
            }
            pending @ GuestInstanceSlot::CleanupPending { .. } => {
                let error = pending.admission_fault().expect("cleanup-pending slot owns its detail");
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = pending;
                Err(error)
            }
            GuestInstanceSlot::Quarantined { instance, detail } => {
                let error = detail.clone();
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::Quarantined { instance, detail };
                Err(error)
            }
        }
    }

    fn acquire_cleanup(slot: Arc<Mutex<GuestInstanceSlot>>) -> Result<Self, Vec<u8>> {
        let state = {
            let mut state = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::replace(&mut *state, GuestInstanceSlot::Leased)
        };
        match state {
            GuestInstanceSlot::CleanupPending { instance: Some(guest), detail } => {
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestInstanceSlot::CleanupPending { instance: None, detail };
                Ok(Self { slot, guest: Some(guest), origin: GuestInstanceLeaseOrigin::Cleanup, disposition: GuestInstanceLeaseDisposition::Quarantined(b"plugin instance quarantined after cleanup cancel-job panic".to_vec()) })
            }
            state => {
                let error = state.admission_fault().unwrap_or_else(|| b"plugin instance was not cleanup-pending behind an acquired cleanup permit".to_vec());
                *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = state;
                Err(error)
            }
        }
    }

    fn guest(&mut self) -> &mut GuestInstance {
        self.guest.as_mut().expect("guest instance lease owns its instance")
    }

    fn quarantine(&mut self, detail: Vec<u8>) {
        self.disposition = GuestInstanceLeaseDisposition::Quarantined(detail);
    }

    fn cleanup_pending(&mut self, detail: Vec<u8>) {
        self.disposition = GuestInstanceLeaseDisposition::CleanupPending(detail);
    }

    fn available(&mut self) {
        self.disposition = GuestInstanceLeaseDisposition::Available;
    }

    fn cleanup_resolved(&mut self) {
        self.disposition = GuestInstanceLeaseDisposition::CleanupResolved;
    }
}

impl Drop for GuestInstanceLease {
    fn drop(&mut self) {
        let Some(guest) = self.guest.take() else {
            return;
        };
        let mut slot = self.slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let external_cleanup_detail = match (&self.origin, &*slot) {
            (GuestInstanceLeaseOrigin::Mounted, GuestInstanceSlot::CleanupPending { instance: None, detail }) => Some(detail.clone()),
            _ => None,
        };
        let expected = match self.origin {
            GuestInstanceLeaseOrigin::Mounted => matches!(&*slot, GuestInstanceSlot::Leased | GuestInstanceSlot::CleanupPending { instance: None, .. }),
            GuestInstanceLeaseOrigin::Cleanup => matches!(&*slot, GuestInstanceSlot::CleanupPending { instance: None, .. }),
        };
        if expected {
            *slot = match std::mem::replace(&mut self.disposition, GuestInstanceLeaseDisposition::Quarantined(b"plugin instance lease lost its disposition".to_vec())) {
                GuestInstanceLeaseDisposition::Available => match external_cleanup_detail {
                    Some(detail) => GuestInstanceSlot::CleanupPending { instance: Some(guest), detail },
                    None => GuestInstanceSlot::Available(guest),
                },
                GuestInstanceLeaseDisposition::CleanupResolved => GuestInstanceSlot::Available(guest),
                GuestInstanceLeaseDisposition::CleanupPending(detail) => GuestInstanceSlot::CleanupPending { instance: Some(guest), detail },
                GuestInstanceLeaseDisposition::Quarantined(detail) => GuestInstanceSlot::Quarantined { instance: guest, detail },
            };
            return;
        }
        *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
            GuestInstanceSlot::Available(instance) => GuestInstanceSlot::Quarantined { instance, detail: b"plugin instance ownership conflict during relay recovery".to_vec() },
            GuestInstanceSlot::CleanupPending { instance: Some(instance), .. } => GuestInstanceSlot::Quarantined { instance, detail: b"plugin instance ownership conflict during cleanup recovery".to_vec() },
            GuestInstanceSlot::CleanupPending { instance: None, .. } | GuestInstanceSlot::Leased => GuestInstanceSlot::Quarantined { instance: guest, detail: b"plugin instance ownership conflict during relay recovery".to_vec() },
            quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
        };
    }
}

fn quarantine_guest_instance(slot: &Arc<Mutex<GuestInstanceSlot>>, detail: Vec<u8>) {
    let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
        GuestInstanceSlot::Available(instance) => GuestInstanceSlot::Quarantined { instance, detail },
        GuestInstanceSlot::CleanupPending { instance: Some(instance), .. } => GuestInstanceSlot::Quarantined { instance, detail },
        pending @ GuestInstanceSlot::CleanupPending { instance: None, .. } => pending,
        GuestInstanceSlot::Leased => GuestInstanceSlot::Leased,
        quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
    };
}

fn mark_guest_cleanup_pending(slot: &Arc<Mutex<GuestInstanceSlot>>, detail: Vec<u8>) {
    let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *slot = match std::mem::replace(&mut *slot, GuestInstanceSlot::Leased) {
        GuestInstanceSlot::Available(instance) => GuestInstanceSlot::CleanupPending { instance: Some(instance), detail },
        GuestInstanceSlot::Leased => GuestInstanceSlot::CleanupPending { instance: None, detail },
        pending @ GuestInstanceSlot::CleanupPending { .. } => pending,
        quarantine @ GuestInstanceSlot::Quarantined { .. } => quarantine,
    };
}

async fn wait_for_guest_relay_cancellation(pool: WorkerPool, cancel: semio_framework_async::CancelToken) {
    while !cancel.is_cancelled_now() {
        pool.timer().sleep_until(pool.now_ms().saturating_add(1)).await;
    }
}

async fn cancel_guest_job_once(runtime: &GuestRuntimes, instance: &mut GuestInstance, job: u64, cancel_admitted: &AtomicBool) -> Result<bool, TurnFault> {
    if cancel_admitted.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
        runtime.cancel_job(instance, job).await?;
        return Ok(true);
    }
    Ok(false)
}

fn cancel_guest_job_failure_detail(error: &TurnFault) -> Vec<u8> {
    format!("plugin instance quarantined after cancel-job failure: {error}").into_bytes()
}

fn guest_relay_cleanup_pending_detail(stage: &str, error: &TurnFault) -> Vec<u8> {
    format!("plugin instance cleanup pending after {stage} failure: {error}").into_bytes()
}

fn foreground_cancel_completion(result: Result<bool, TurnFault>, guest: &mut GuestInstanceLease) -> GuestRelayCompletion {
    match result {
        Ok(true) => {
            guest.cleanup_resolved();
            GuestRelayCompletion::Cancelled
        }
        Ok(false) => {
            let detail = b"plugin instance quarantined because cancel-job admission was already consumed".to_vec();
            guest.quarantine(detail.clone());
            GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail))
        }
        Err(error) => {
            let detail = cancel_guest_job_failure_detail(&error);
            guest.quarantine(detail.clone());
            GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail))
        }
    }
}

async fn run_guest_relay_request(
    runtime: Arc<GuestRuntimes>,
    instance: Arc<Mutex<GuestInstanceSlot>>,
    instance_gate: Arc<semio_framework_async::Semaphore>,
    pool: WorkerPool,
    cancel: semio_framework_async::CancelToken,
    cancel_scheduled: Arc<AtomicBool>,
    cancel_admitted: Arc<AtomicBool>,
    job: u64,
    request: GuestRelayRequest,
    sender: GuestRelayCompletionSender,
) {
    let permit = if matches!(&request, GuestRelayRequest::Cancel) {
        instance_gate.acquire_owned().await
    } else {
        match semio_framework_async::select2(instance_gate.acquire_owned(), wait_for_guest_relay_cancellation(pool.clone(), cancel.clone())).await {
            semio_framework_async::Either::Left(permit) => permit,
            semio_framework_async::Either::Right(()) => {
                sender.send(GuestRelayCompletion::Cancelled);
                return;
            }
        }
    };
    let unwind_detail = match &request {
        GuestRelayRequest::Cancel => b"plugin instance quarantined after cancel-job panic".to_vec(),
        GuestRelayRequest::Start { .. } | GuestRelayRequest::Step => b"plugin instance cleanup pending after guest relay panic".to_vec(),
    };
    let mut guest = match if matches!(&request, GuestRelayRequest::Cancel) {
        GuestInstanceLease::acquire_cleanup(Arc::clone(&instance))
    } else {
        GuestInstanceLease::acquire(Arc::clone(&instance), unwind_detail)
    } {
        Ok(guest) => guest,
        Err(error) => {
            sender.send(GuestRelayCompletion::Rejected(GuestRelayOwnedBytes::new(error)));
            return;
        }
    };
    let mut cleanup_pending = false;
    let completion = match request {
        GuestRelayRequest::Start { kind, input } => match semio_framework_async::select2(runtime.start_job(guest.guest(), job, &kind, input), wait_for_guest_relay_cancellation(pool.clone(), cancel.clone())).await {
            semio_framework_async::Either::Left(Ok(())) => {
                guest.available();
                GuestRelayCompletion::Started(Ok(()))
            }
            semio_framework_async::Either::Left(Err(error)) => {
                guest.cleanup_pending(guest_relay_cleanup_pending_detail("start-job", &error));
                cleanup_pending = true;
                GuestRelayCompletion::Started(Err(error))
            }
            semio_framework_async::Either::Right(()) => {
                let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
                foreground_cancel_completion(result, &mut guest)
            }
        },
        GuestRelayRequest::Step => match semio_framework_async::select2(runtime.step_job(guest.guest(), job, RELAY_JOB_BUDGET), wait_for_guest_relay_cancellation(pool.clone(), cancel)).await {
            semio_framework_async::Either::Left(Ok(step)) => {
                guest.available();
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::from_guest(step)))
            }
            semio_framework_async::Either::Left(Err(error)) => {
                guest.cleanup_pending(guest_relay_cleanup_pending_detail("step-job", &error));
                cleanup_pending = true;
                GuestRelayCompletion::Stepped(Err(error))
            }
            semio_framework_async::Either::Right(()) => {
                let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
                foreground_cancel_completion(result, &mut guest)
            }
        },
        GuestRelayRequest::Cancel => {
            let result = cancel_guest_job_once(&runtime, guest.guest(), job, &cancel_admitted).await;
            foreground_cancel_completion(result, &mut guest)
        }
    };
    drop(guest);
    drop(permit);
    #[cfg(test)]
    wait_for_scripted_guest_relay_release(&runtime, &completion).await;
    let _ = cleanup_pending;
    sender.send(completion);
}

fn recover_guest_relay_failure(
    runtime: Arc<GuestRuntimes>,
    instance: Arc<Mutex<GuestInstanceSlot>>,
    instance_gate: Arc<semio_framework_async::Semaphore>,
    pool: WorkerPool,
    job: u64,
    cancel_scheduled: Arc<AtomicBool>,
    cancel_admitted: Arc<AtomicBool>,
    sender: GuestRelayCompletionSender,
    request: GuestRelayRequestKind,
    failure: GuestRelayPoolFailure,
) {
    if let GuestRelayPoolFailure::Admission(kind) = failure {
        let detail = match kind {
            semio_framework_async::WorkerSubmitErrorKind::Shutdown => b"plugin instance quarantined after relay scheduler shutdown".to_vec(),
            semio_framework_async::WorkerSubmitErrorKind::Poisoned => b"plugin instance quarantined after relay scheduler poison".to_vec(),
            semio_framework_async::WorkerSubmitErrorKind::Contended | semio_framework_async::WorkerSubmitErrorKind::Saturated => b"plugin instance quarantined after non-retryable relay admission failure".to_vec(),
        };
        quarantine_guest_instance(&instance, detail.clone());
        sender.send(GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail)));
        return;
    }
    #[cfg(test)]
    if let GuestRuntimes::Mock(mock) = runtime.as_ref() {
        let barrier = match request {
            GuestRelayRequestKind::Start => mock.take_relay_start_failure_release(),
            GuestRelayRequestKind::Step => mock.take_relay_step_failure_release(),
            GuestRelayRequestKind::Cancel => None,
        };
        if let Some(barrier) = barrier {
            let panic_instance = Arc::clone(&instance);
            GuestRelayPoolFuture::spawn_recoverable(
                pool.clone(),
                Lane::UserVisible,
                async move {
                    barrier.wait().await;
                    recover_guest_relay_failure(runtime, instance, instance_gate, pool, job, cancel_scheduled, cancel_admitted, sender, request, GuestRelayPoolFailure::FuturePanicked);
                },
                move |_| quarantine_guest_instance(&panic_instance, b"plugin instance quarantined after relay panic recovery panic".to_vec()),
            );
            return;
        }
    }
    let detail = if matches!(request, GuestRelayRequestKind::Cancel) || cancel_admitted.load(std::sync::atomic::Ordering::Acquire) {
        let detail = b"plugin instance quarantined after cancel-job panic";
        quarantine_guest_instance(&instance, detail.to_vec());
        detail.to_vec()
    } else {
        let _ = (runtime, instance_gate, pool, job, cancel_scheduled, cancel_admitted);
        mark_guest_cleanup_pending(&instance, b"plugin instance cleanup pending after guest relay panic".to_vec());
        b"plugin guest relay panicked after acquiring its instance".to_vec()
    };
    sender.send(if matches!(request, GuestRelayRequestKind::Cancel) {
        GuestRelayCompletion::TerminalFault(GuestRelayOwnedBytes::new(detail))
    } else {
        GuestRelayCompletion::Fault(GuestRelayOwnedBytes::new(detail))
    });
}

#[derive(Clone, Copy)]
enum GuestRelayPublicationKind {
    Preview,
    Commit,
    Fault,
}

struct GuestRelayPublication {
    kind: GuestRelayPublicationKind,
    source: Vec<u8>,
    cursor: usize,
    copied: bool,
    retired: bool,
    oversized: bool,
    writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
}

impl GuestRelayPublication {
    fn new(kind: GuestRelayPublicationKind, source: Vec<u8>) -> Self {
        let oversized = source.len() > semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES;
        let kind = if oversized { GuestRelayPublicationKind::Fault } else { kind };
        let stream = match kind {
            GuestRelayPublicationKind::Preview => semio_framework_job::JobPayloadStream::Preview,
            GuestRelayPublicationKind::Commit => semio_framework_job::JobPayloadStream::CommitOutput,
            GuestRelayPublicationKind::Fault => semio_framework_job::JobPayloadStream::Fault,
        };
        Self { kind, source, cursor: 0, copied: false, retired: false, oversized, writer: Some(semio_framework_job::RetainedJobPayloadWriter::new(stream)) }
    }

    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if !self.copied {
            let bytes = if self.oversized { &b"plugin-guest-relay-output-limit"[..] } else { self.source.as_slice() };
            let writer = self.writer.as_mut().expect("guest relay publication owns writer");
            match writer.write_slice_page(context, bytes, &mut self.cursor) {
                Ok(true) => self.copied = true,
                Ok(false) | Err(_) => return semio_framework_job::StepOutcome::Yield,
            }
            context.consume_fuel(1);
            return semio_framework_job::StepOutcome::Yield;
        }
        if !self.retired {
            if !self.source.is_empty() {
                let keep = self.source.len().saturating_sub(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                self.source.truncate(keep);
                context.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            drop(std::mem::take(&mut self.source));
            self.retired = true;
            context.consume_fuel(1);
            return semio_framework_job::StepOutcome::Yield;
        }
        let writer = self.writer.take().expect("guest relay publication owns finished writer");
        let payload = match writer.finish() {
            Ok(payload) => payload,
            Err(writer) => {
                self.writer = Some(writer);
                return semio_framework_job::StepOutcome::Yield;
            }
        };
        match self.kind {
            GuestRelayPublicationKind::Preview => semio_framework_job::StepOutcome::PreviewReady(payload),
            GuestRelayPublicationKind::Commit => {
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState), output: payload })
            }
            GuestRelayPublicationKind::Fault => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: payload }),
        }
    }

    fn begin_close(&mut self) {
        if let Some(writer) = self.writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if let Some(writer) = self.writer.as_mut() {
            match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => {
                    if writer.terminal_is_empty() {
                        self.writer = None;
                    }
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
                }
                semio_framework_job::JobPayloadCloseStep::Complete => self.writer = None,
            }
        }
        if self.source.is_empty() {
            return semio_framework_job::InteractiveJobCloseStep::Complete;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let released_bytes = maximum_bytes.min(self.source.len());
        self.source.truncate(self.source.len() - released_bytes);
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes }
    }

    fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.source.is_empty()
    }
}

struct GuestColdRelayJob {
    runtime: Arc<GuestRuntimes>,
    instance: Arc<Mutex<GuestInstanceSlot>>,
    instance_gate: Arc<semio_framework_async::Semaphore>,
    pool: WorkerPool,
    cancel: semio_framework_async::CancelToken,
    cancel_scheduled: Arc<AtomicBool>,
    cancel_admitted: Arc<AtomicBool>,
    job: u64,
    start: Option<(String, Vec<u8>)>,
    pending: Option<Arc<GuestRelayCompletionSlot>>,
    publication: Option<GuestRelayPublication>,
    closing: bool,
    start_submitted: bool,
    started: bool,
    cleanup_required: bool,
    terminal_delivered: bool,
}

impl GuestColdRelayJob {
    fn register_wake(&self, waker: &std::task::Waker) -> bool {
        if let Some(slot) = self.pending.as_ref() {
            slot.register_wake(waker);
            false
        } else {
            true
        }
    }

    fn submit(&mut self, request: GuestRelayRequest) {
        let slot = Arc::new(GuestRelayCompletionSlot::new());
        let sender = GuestRelayCompletionSender::new(Arc::clone(&slot));
        let request_kind = request.kind();
        GuestRelayPoolFuture::spawn_recoverable(
            self.pool.clone(),
            Lane::UserVisible,
            run_guest_relay_request(
                Arc::clone(&self.runtime),
                Arc::clone(&self.instance),
                Arc::clone(&self.instance_gate),
                self.pool.clone(),
                self.cancel.clone(),
                Arc::clone(&self.cancel_scheduled),
                Arc::clone(&self.cancel_admitted),
                self.job,
                request,
                sender.clone(),
            ),
            {
                let runtime = Arc::clone(&self.runtime);
                let instance = Arc::clone(&self.instance);
                let instance_gate = Arc::clone(&self.instance_gate);
                let pool = self.pool.clone();
                let job = self.job;
                let cancel_scheduled = Arc::clone(&self.cancel_scheduled);
                let cancel_admitted = Arc::clone(&self.cancel_admitted);
                move |failure| recover_guest_relay_failure(runtime, instance, instance_gate, pool, job, cancel_scheduled, cancel_admitted, sender, request_kind, failure)
            },
        );
        self.pending = Some(slot);
    }

    fn submit_cleanup(&mut self) {
        self.cancel.cancel_now();
        if self.start_submitted && self.pending.is_none() && self.cancel_scheduled.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
            self.submit(GuestRelayRequest::Cancel);
        }
    }

    fn terminal(&mut self, outcome: semio_framework_job::StepOutcome) -> semio_framework_job::StepOutcome {
        self.terminal_delivered = true;
        outcome
    }
}

impl semio_framework_job::InteractiveJob for GuestColdRelayJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if let Some(publication) = self.publication.as_mut() {
            let outcome = publication.step(context);
            if !matches!(outcome, semio_framework_job::StepOutcome::Yield) {
                let terminal = outcome.is_terminal();
                self.publication = None;
                if terminal {
                    self.terminal_delivered = true;
                }
            }
            return outcome;
        }
        if self.terminal_delivered {
            return semio_framework_job::StepOutcome::Yield;
        }
        if context.is_cancelled() || self.cancel.is_cancelled_now() {
            self.cancel.cancel_now();
            if self.pending.is_none() && self.cleanup_required {
                self.submit_cleanup();
                return semio_framework_job::StepOutcome::Yield;
            }
            if self.pending.is_none() {
                return self.terminal(semio_framework_job::StepOutcome::Cancelled);
            }
        }
        if let Some(slot) = &self.pending {
            let Some(completion) = slot.try_take() else { return semio_framework_job::StepOutcome::Yield };
            self.pending = None;
            return match completion {
                GuestRelayCompletion::Started(Ok(())) => {
                    self.started = true;
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::Started(Err(error)) | GuestRelayCompletion::Stepped(Err(error)) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.to_string().into_bytes()));
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: Some(progress) })) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Preview, progress.into_source()));
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: None })) => semio_framework_job::StepOutcome::Yield,
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Done { output })) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Commit, output.into_source()));
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Failed { error })) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::Rejected(error) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::Fault(error) => {
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::TerminalFault(error) => {
                    self.cleanup_required = false;
                    self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()));
                    semio_framework_job::StepOutcome::Yield
                }
                GuestRelayCompletion::Cancelled => {
                    self.cleanup_required = false;
                    self.terminal(semio_framework_job::StepOutcome::Cancelled)
                }
            };
        }
        if let Some((kind, input)) = self.start.take() {
            self.start_submitted = true;
            self.cleanup_required = true;
            self.submit(GuestRelayRequest::Start { kind, input });
        } else if self.started {
            self.submit(GuestRelayRequest::Step);
        } else {
            self.publication = Some(GuestRelayPublication::new(GuestRelayPublicationKind::Fault, b"plugin guest relay lost its start state".to_vec()));
            return semio_framework_job::StepOutcome::Yield;
        }
        semio_framework_job::StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.cancel.cancel_now();
        if let Some(publication) = self.publication.as_mut() {
            publication.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            self.begin_close();
        }
        if let Some(publication) = self.publication.as_mut() {
            let step = publication.close_step(maximum_items, maximum_bytes);
            if publication.terminal_is_empty() {
                self.publication = None;
            }
            return step;
        }
        if let Some(slot) = &self.pending {
            let Some(completion) = slot.try_take() else { return semio_framework_job::InteractiveJobCloseStep::Blocked };
            self.pending = None;
            self.publication = Some(match completion {
                GuestRelayCompletion::Rejected(bytes) | GuestRelayCompletion::Fault(bytes) => GuestRelayPublication::new(GuestRelayPublicationKind::Fault, bytes.into_source()),
                GuestRelayCompletion::TerminalFault(bytes) => {
                    self.cleanup_required = false;
                    GuestRelayPublication::new(GuestRelayPublicationKind::Fault, bytes.into_source())
                }
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: Some(bytes) })) => GuestRelayPublication::new(GuestRelayPublicationKind::Preview, bytes.into_source()),
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Done { output })) => GuestRelayPublication::new(GuestRelayPublicationKind::Commit, output.into_source()),
                GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Failed { error })) => GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.into_source()),
                GuestRelayCompletion::Started(Err(error)) | GuestRelayCompletion::Stepped(Err(error)) => GuestRelayPublication::new(GuestRelayPublicationKind::Fault, error.to_string().into_bytes()),
                GuestRelayCompletion::Cancelled => {
                    self.cleanup_required = false;
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                GuestRelayCompletion::Started(Ok(())) | GuestRelayCompletion::Stepped(Ok(GuestRelayStepCompletion::Running { progress: None })) => {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            });
            self.publication.as_mut().expect("close retained relay publication").begin_close();
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some((kind, input)) = self.start.as_mut() {
            if !input.is_empty() {
                if maximum_items == 0 || maximum_bytes == 0 {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let released_bytes = maximum_bytes.min(input.len());
                input.truncate(input.len() - released_bytes);
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            if !kind.is_empty() {
                if maximum_items == 0 || maximum_bytes < kind.len() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                let released_bytes = kind.len();
                kind.clear();
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
            }
            self.start = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.cleanup_required {
            self.submit_cleanup();
            return if self.pending.is_some() { semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 } } else { semio_framework_job::InteractiveJobCloseStep::Blocked };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn register_close_wake(&self, waker: &std::task::Waker) -> bool {
        self.pending.as_ref().is_some_and(|slot| {
            slot.register_wake(waker);
            true
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && !self.cleanup_required && self.start.is_none() && self.pending.is_none() && self.publication.as_ref().is_none_or(GuestRelayPublication::terminal_is_empty)
    }
}

impl Drop for GuestColdRelayJob {
    fn drop(&mut self) {
        if self.cleanup_required || self.start.is_some() || self.pending.is_some() || self.publication.as_ref().is_some_and(|publication| !publication.terminal_is_empty()) {
            self.cancel.cancel_now();
            self.closing = true;
        }
    }
}

impl GuestColdRelayJob {
    fn new(runtime: Arc<GuestRuntimes>, instance: Arc<Mutex<GuestInstanceSlot>>, instance_gate: Arc<semio_framework_async::Semaphore>, pool: WorkerPool, cancel: semio_framework_async::CancelToken, job: u64, kind: String, input: Vec<u8>) -> Self {
        Self {
            runtime,
            instance,
            instance_gate,
            pool,
            cancel,
            cancel_scheduled: Arc::new(AtomicBool::new(false)),
            cancel_admitted: Arc::new(AtomicBool::new(false)),
            job,
            start: Some((kind, input)),
            pending: None,
            publication: None,
            closing: false,
            start_submitted: false,
            started: false,
            cleanup_required: false,
            terminal_delivered: false,
        }
    }
}

const GUEST_RELAY_MOUNTED_SLOTS: usize = 16;
const GUEST_RELAY_REAPER_FALLBACK_MS: u64 = 8;

struct GuestRelayLifecycleProbeControl {
    awake: AtomicBool,
    release_permits: std::sync::atomic::AtomicUsize,
    waker: Mutex<Option<std::task::Waker>>,
    close_admissions: std::sync::atomic::AtomicUsize,
    releases: std::sync::atomic::AtomicUsize,
}

impl GuestRelayLifecycleProbeControl {
    fn new() -> Self {
        Self { awake: AtomicBool::new(false), release_permits: std::sync::atomic::AtomicUsize::new(0), waker: Mutex::new(None), close_admissions: std::sync::atomic::AtomicUsize::new(0), releases: std::sync::atomic::AtomicUsize::new(0) }
    }

    fn wake(&self) {
        self.awake.store(true, std::sync::atomic::Ordering::Release);
        self.wake_registered();
    }

    fn release_one(&self) {
        self.release_permits.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.wake_registered();
    }

    fn wake_registered(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }
}

struct GuestRelayLifecycleProbeJob {
    control: Arc<GuestRelayLifecycleProbeControl>,
    remaining: usize,
    terminal_output: Option<Vec<u8>>,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for GuestRelayLifecycleProbeJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        let Some(output) = self.terminal_output.take() else { return semio_framework_job::StepOutcome::Yield };
        let output = context
            .payload_from_bytes(semio_framework_job::JobPayloadStream::CommitOutput, &output)
            .unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput));
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output,
        })
    }

    fn begin_close(&mut self) {
        if !self.closing {
            self.closing = true;
            self.control.close_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.control.awake.load(std::sync::atomic::Ordering::Acquire) {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.remaining == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Complete;
        }
        if maximum_items == 0
            || self
                .control
                .release_permits
                .fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |permits| permits.checked_sub(1))
                .is_err()
        {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        self.remaining -= 1;
        self.control.releases.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    fn register_close_wake(&self, waker: &std::task::Waker) -> bool {
        let ready = || {
            self.control.awake.load(std::sync::atomic::Ordering::Acquire)
                && (self.remaining == 0 || self.control.release_permits.load(std::sync::atomic::Ordering::Acquire) > 0)
        };
        if ready() {
            waker.wake_by_ref();
            return true;
        }
        *self.control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(waker.clone());
        if ready() {
            if let Some(waker) = self.control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                waker.wake();
            }
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.remaining == 0
    }
}

enum GuestRelayMountedOwner {
    Session(semio_framework_job::WorkerJobSession<GuestColdRelayJob>),
    Rejected(semio_framework_job::WorkerJobSessionAdmissionRejected<GuestColdRelayJob>),
    LifecycleProbe(semio_framework_job::WorkerJobSession<GuestRelayLifecycleProbeJob>),
    Empty,
}

struct GuestRelayMountedSession {
    generation: u64,
    owner: GuestRelayMountedOwner,
    checked_out: Option<semio_framework_job::WorkerJobOutcome<GuestColdRelayJob>>,
    lifecycle_probe_checked_out: Option<semio_framework_job::WorkerJobOutcome<GuestRelayLifecycleProbeJob>>,
    outcome: Option<semio_framework_job::StepOutcome>,
    outcome_page: usize,
    output: GuestRelayMountedOutput,
    terminal: Option<GuestRelayMountedTerminal>,
    lifecycle: GuestRelayMountedLifecycle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuestRelayMountedLifecycle {
    Running,
    DrainingForCaller,
    DetachedForReap,
}

struct GuestRelayMountedOutput {
    storage: Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>,
    length: usize,
}

impl GuestRelayMountedOutput {
    fn new() -> Self {
        let storage = Box::<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>::new_uninit();
        Self { storage: unsafe { storage.assume_init() }, length: 0 }
    }

    fn write_page(&mut self, page: &[u8]) -> Result<(), PluginHostError> {
        let end = self.length.checked_add(page.len()).filter(|end| *end <= semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES).ok_or_else(|| PluginHostError::Plugin("plugin cold relay mounted output exceeds its admitted authority".into()))?;
        for (target, byte) in self.storage[self.length..end].iter_mut().zip(page.iter().copied()) {
            target.write(byte);
        }
        self.length = end;
        Ok(())
    }

    fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.storage.as_ptr().cast::<u8>(), self.length) }
    }

    fn into_vec(self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

enum GuestRelayMountedTerminal {
    Complete,
    Cancelled,
    Fault,
    HostFault(PluginHostError),
}

enum GuestRelayMountedClose {
    Complete,
    Pending,
    Blocked { wake_registered: bool },
}

enum GuestRelayMountedSlot {
    Empty,
    Reserved { generation: u64, output: GuestRelayMountedOutput },
    Mounted(GuestRelayMountedSession),
}

struct GuestRelayMountedRegistry {
    slots: Box<[Mutex<GuestRelayMountedSlot>]>,
    next_generation: std::sync::atomic::AtomicU64,
    pool: WorkerPool,
    reaper_cursor: std::sync::atomic::AtomicUsize,
    reaper_epoch: std::sync::atomic::AtomicU64,
    reaper_active: AtomicBool,
    reaper_failed: AtomicBool,
    reaper_timer_pending: AtomicBool,
    #[cfg(test)]
    reaper_polls: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    reaper_timers: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    reaper_wake_waits: std::sync::atomic::AtomicUsize,
}

impl GuestRelayMountedRegistry {
    fn new() -> Self {
        Self::with_pool(plugin_host_worker_pool())
    }

    fn with_pool(pool: WorkerPool) -> Self {
        Self {
            slots: std::iter::repeat_with(|| Mutex::new(GuestRelayMountedSlot::Empty)).take(GUEST_RELAY_MOUNTED_SLOTS).collect(),
            next_generation: std::sync::atomic::AtomicU64::new(1),
            pool,
            reaper_cursor: std::sync::atomic::AtomicUsize::new(0),
            reaper_epoch: std::sync::atomic::AtomicU64::new(0),
            reaper_active: AtomicBool::new(false),
            reaper_failed: AtomicBool::new(false),
            reaper_timer_pending: AtomicBool::new(false),
            #[cfg(test)]
            reaper_polls: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            reaper_timers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            reaper_wake_waits: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn reserve(&self) -> Option<(usize, u64)> {
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        for (index, slot) in self.slots.iter().enumerate() {
            let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
                return None;
            }
            if matches!(*slot, GuestRelayMountedSlot::Empty) {
                let generation = self
                    .next_generation
                    .fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| match generation {
                        0 => None,
                        u64::MAX => Some(0),
                        generation => Some(generation + 1),
                    })
                    .ok()?;
                *slot = GuestRelayMountedSlot::Reserved { generation, output: GuestRelayMountedOutput::new() };
                return Some((index, generation));
            }
        }
        None
    }

    fn mount(&self, index: usize, generation: u64, mut owner: GuestRelayMountedOwner) {
        let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let reserved = std::mem::replace(&mut *slot, GuestRelayMountedSlot::Empty);
        let GuestRelayMountedSlot::Reserved { generation: reserved_generation, output } = reserved else {
            debug_assert!(false, "mounted relay requires its exact reserved output owner");
            return;
        };
        if reserved_generation != generation {
            debug_assert!(false, "mounted relay generation must match its reserved output owner");
            *slot = GuestRelayMountedSlot::Reserved { generation: reserved_generation, output };
            return;
        }
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
            if let GuestRelayMountedOwner::Rejected(rejected) = &mut owner {
                Self::close_rejected(rejected);
            }
            drop(owner);
            return;
        }
        *slot = GuestRelayMountedSlot::Mounted(GuestRelayMountedSession { generation, owner, checked_out: None, lifecycle_probe_checked_out: None, outcome: None, outcome_page: 0, output, terminal: None, lifecycle: GuestRelayMountedLifecycle::Running });
    }

    fn detach(self: &Arc<Self>, index: usize, generation: u64) {
        let detached = {
            let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { return };
            if session.generation != generation {
                return;
            }
            if let Some(owner) = session.checked_out.take() {
                owner.begin_close();
            } else if let Some(owner) = session.lifecycle_probe_checked_out.take() {
                owner.begin_close();
            } else {
                match &session.owner {
                    GuestRelayMountedOwner::Session(owner) => {
                        let _ = owner.begin_close();
                    }
                    GuestRelayMountedOwner::LifecycleProbe(owner) => {
                        let _ = owner.begin_close();
                    }
                    GuestRelayMountedOwner::Rejected(_) | GuestRelayMountedOwner::Empty => {}
                }
            }
            session.lifecycle = GuestRelayMountedLifecycle::DetachedForReap;
            true
        };
        if detached {
            self.reaper_epoch.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.ensure_reaper();
        }
    }

    fn ensure_reaper(self: &Arc<Self>) {
        if self.reaper_failed.load(std::sync::atomic::Ordering::Acquire) {
            self.fail_detached();
            return;
        }
        if self.reaper_active.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let registry = Arc::clone(self);
        let recovery = Arc::clone(self);
        GuestRelayPoolFuture::spawn_recoverable(
            self.pool.clone(),
            Lane::Maintenance,
            GuestRelayMountedReaper { registry },
            move |failure| {
                recovery.reaper_active.store(false, std::sync::atomic::Ordering::Release);
                match failure {
                    GuestRelayPoolFailure::FuturePanicked => recovery.ensure_reaper(),
                    GuestRelayPoolFailure::Admission(_) => {
                        recovery.reaper_failed.store(true, std::sync::atomic::Ordering::Release);
                        recovery.fail_detached();
                    }
                }
            },
        );
    }

    fn fail_detached(&self) {
        for slot in &self.slots {
            let retired = {
                let mut slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { continue };
                if !matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap) {
                    continue;
                }
                if matches!(session.owner, GuestRelayMountedOwner::Rejected(_)) {
                    let GuestRelayMountedOwner::Rejected(rejected) = &mut session.owner else { unreachable!() };
                    Self::close_rejected(rejected);
                    if rejected.terminal_is_empty() {
                        session.owner = GuestRelayMountedOwner::Empty;
                    }
                    if matches!(session.owner, GuestRelayMountedOwner::Rejected(_)) {
                        continue;
                    }
                }
                Some(std::mem::replace(&mut *slot, GuestRelayMountedSlot::Empty))
            };
            drop(retired);
        }
    }

    fn close_rejected(owner: &mut semio_framework_job::WorkerJobSessionAdmissionRejected<GuestColdRelayJob>) {
        owner.begin_close();
        for _ in 0..semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES.saturating_add(8) {
            if owner.terminal_is_empty() {
                return;
            }
            let _ = owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
    }

    fn reap_one(&self, waker: &std::task::Waker) -> GuestRelayMountedReap {
        let cursor = self.reaper_cursor.load(std::sync::atomic::Ordering::Acquire) % GUEST_RELAY_MOUNTED_SLOTS;
        for offset in 0..GUEST_RELAY_MOUNTED_SLOTS {
            let index = (cursor + offset) % GUEST_RELAY_MOUNTED_SLOTS;
            let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { continue };
            if !matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap) {
                continue;
            }
            self.reaper_cursor.store((index + 1) % GUEST_RELAY_MOUNTED_SLOTS, std::sync::atomic::Ordering::Release);
            return match Self::pump_close(session, Some(waker)) {
                GuestRelayMountedClose::Complete => {
                    *slot = GuestRelayMountedSlot::Empty;
                    GuestRelayMountedReap::Progress
                }
                GuestRelayMountedClose::Pending => GuestRelayMountedReap::Progress,
                GuestRelayMountedClose::Blocked { wake_registered } => GuestRelayMountedReap::Blocked { wake_registered },
            };
        }
        GuestRelayMountedReap::Idle
    }

    fn schedule_reaper_timer(self: &Arc<Self>, waker: &std::task::Waker) {
        if self.reaper_timer_pending.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        #[cfg(test)]
        self.reaper_timers.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let registry = Arc::clone(self);
        let wake = waker.clone();
        self.pool.submit_at(
            self.pool.now_ms().saturating_add(GUEST_RELAY_REAPER_FALLBACK_MS),
            Lane::Maintenance,
            Box::new(move || {
                registry.reaper_timer_pending.store(false, std::sync::atomic::Ordering::Release);
                wake.wake();
            }),
        );
    }

    fn park_blocked_reaper(self: &Arc<Self>, waker: &std::task::Waker, wake_registered: bool) {
        if wake_registered {
            #[cfg(test)]
            self.reaper_wake_waits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        } else {
            self.schedule_reaper_timer(waker);
        }
    }

    fn has_detached(&self) -> bool {
        self.slots.iter().any(|slot| {
            let slot = slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap))
        })
    }

    fn pump_close(session: &mut GuestRelayMountedSession, waker: Option<&std::task::Waker>) -> GuestRelayMountedClose {
        if let Some(outcome) = session.outcome.as_mut() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if outcome.terminal_is_empty() {
                session.outcome = None;
            }
            return GuestRelayMountedClose::Pending;
        }
        if let Some(owner) = session.checked_out.take() {
            owner.begin_close();
            return GuestRelayMountedClose::Pending;
        }
        if let Some(owner) = session.lifecycle_probe_checked_out.take() {
            owner.begin_close();
            return GuestRelayMountedClose::Pending;
        }
        match &mut session.owner {
            GuestRelayMountedOwner::Session(owner) => match owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                semio_framework_job::WorkerJobCloseStep::Complete => {
                    session.owner = GuestRelayMountedOwner::Empty;
                    GuestRelayMountedClose::Complete
                }
                semio_framework_job::WorkerJobCloseStep::Pending { .. } => GuestRelayMountedClose::Pending,
                semio_framework_job::WorkerJobCloseStep::Blocked => {
                    let wake_registered = waker.is_some_and(|waker| owner.register_close_wake(waker).unwrap_or(false));
                    GuestRelayMountedClose::Blocked { wake_registered }
                }
            },
            GuestRelayMountedOwner::Rejected(owner) => {
                owner.begin_close();
                let step = owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if owner.terminal_is_empty() {
                    session.owner = GuestRelayMountedOwner::Empty;
                    GuestRelayMountedClose::Complete
                } else {
                    match step {
                        semio_framework_job::InteractiveJobCloseStep::Pending { .. } => GuestRelayMountedClose::Pending,
                        semio_framework_job::InteractiveJobCloseStep::Blocked => {
                            let wake_registered = waker.is_some_and(|waker| semio_framework_job::InteractiveJob::register_close_wake(owner.job(), waker));
                            GuestRelayMountedClose::Blocked { wake_registered }
                        }
                        semio_framework_job::InteractiveJobCloseStep::Complete => GuestRelayMountedClose::Pending,
                    }
                }
            }
            GuestRelayMountedOwner::LifecycleProbe(owner) => match owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                semio_framework_job::WorkerJobCloseStep::Complete => {
                    session.owner = GuestRelayMountedOwner::Empty;
                    GuestRelayMountedClose::Complete
                }
                semio_framework_job::WorkerJobCloseStep::Pending { .. } => GuestRelayMountedClose::Pending,
                semio_framework_job::WorkerJobCloseStep::Blocked => {
                    let wake_registered = waker.is_some_and(|waker| owner.register_close_wake(waker).unwrap_or(false));
                    GuestRelayMountedClose::Blocked { wake_registered }
                }
            },
            GuestRelayMountedOwner::Empty => GuestRelayMountedClose::Complete,
        }
    }

    fn outcome_payload(outcome: &semio_framework_job::StepOutcome) -> Option<&semio_framework_job::RetainedJobPayload> {
        match outcome {
            semio_framework_job::StepOutcome::PreviewReady(payload) => Some(payload),
            semio_framework_job::StepOutcome::CheckpointReady(checkpoint) => Some(&checkpoint.state),
            semio_framework_job::StepOutcome::Complete(candidate) if !candidate.state.terminal_is_empty() => Some(&candidate.state),
            semio_framework_job::StepOutcome::Complete(candidate) => Some(&candidate.output),
            semio_framework_job::StepOutcome::Fault(fault) => Some(&fault.detail),
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::Cancelled => None,
        }
    }

    fn finish_outcome(session: &mut GuestRelayMountedSession, waker: &std::task::Waker) {
        let terminal = session.outcome.as_ref().is_some_and(semio_framework_job::StepOutcome::is_terminal);
        if terminal {
            let outcome = session.outcome.take().expect("terminal relay outcome remains owned");
            session.terminal = Some(match outcome {
                semio_framework_job::StepOutcome::Complete(_) => GuestRelayMountedTerminal::Complete,
                semio_framework_job::StepOutcome::Cancelled => GuestRelayMountedTerminal::Cancelled,
                semio_framework_job::StepOutcome::Fault(_) => GuestRelayMountedTerminal::Fault,
                _ => unreachable!("terminal relay classification remains stable"),
            });
            if let Some(owner) = session.checked_out.take() {
                owner.begin_close();
            } else {
                session.lifecycle_probe_checked_out.take().expect("terminal relay owner remains checked out").begin_close();
            }
            session.lifecycle = GuestRelayMountedLifecycle::DrainingForCaller;
            waker.wake_by_ref();
        } else {
            session.outcome = None;
            if let Some(owner) = session.checked_out.take() {
                let wake_after_resume = owner.job().register_wake(waker);
                if let Err(owner) = owner.resume() {
                    owner.begin_close();
                    session.terminal = Some(GuestRelayMountedTerminal::HostFault(PluginHostError::Plugin("plugin cold relay resume rejected".into())));
                    session.lifecycle = GuestRelayMountedLifecycle::DrainingForCaller;
                } else if wake_after_resume {
                    waker.wake_by_ref();
                }
            } else {
                let owner = session.lifecycle_probe_checked_out.take().expect("resumable relay owner remains checked out");
                if let Err(owner) = owner.resume() {
                    owner.begin_close();
                    session.terminal = Some(GuestRelayMountedTerminal::HostFault(PluginHostError::Plugin("plugin relay lifecycle probe resume rejected".into())));
                    session.lifecycle = GuestRelayMountedLifecycle::DrainingForCaller;
                }
            }
        }
    }

    fn pump(self: &Arc<Self>, index: usize, generation: u64, waker: &std::task::Waker) -> std::task::Poll<Result<Vec<u8>, PluginHostError>> {
        let mut slot = self.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let GuestRelayMountedSlot::Mounted(session) = &mut *slot else {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay slot is not mounted".into())));
        };
        if session.generation != generation {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay generation is stale".into())));
        }
        if matches!(session.lifecycle, GuestRelayMountedLifecycle::DetachedForReap) {
            return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay ownership is detached".into())));
        }
        if matches!(session.lifecycle, GuestRelayMountedLifecycle::DrainingForCaller) {
            match Self::pump_close(session, Some(waker)) {
                GuestRelayMountedClose::Complete => {
                    let mounted = std::mem::replace(&mut *slot, GuestRelayMountedSlot::Empty);
                    let GuestRelayMountedSlot::Mounted(mut session) = mounted else { unreachable!("mounted relay slot remains mounted through close") };
                    let result = match session.terminal.take() {
                        Some(GuestRelayMountedTerminal::Complete) => Ok(session.output.into_vec()),
                        Some(GuestRelayMountedTerminal::Cancelled) => Err(PluginHostError::Plugin("plugin cold relay cancelled".into())),
                        Some(GuestRelayMountedTerminal::Fault) => Err(PluginHostError::Plugin(String::from_utf8_lossy(session.output.bytes()).into_owned())),
                        Some(GuestRelayMountedTerminal::HostFault(error)) => Err(error),
                        None => Err(PluginHostError::Plugin("plugin cold relay closed".into())),
                    };
                    return std::task::Poll::Ready(result);
                }
                GuestRelayMountedClose::Pending => waker.wake_by_ref(),
                GuestRelayMountedClose::Blocked { wake_registered: false } => self.schedule_reaper_timer(waker),
                GuestRelayMountedClose::Blocked { wake_registered: true } => {}
            }
            return std::task::Poll::Pending;
        }
        if let Some(outcome) = session.outcome.as_mut() {
            if let Some(payload) = Self::outcome_payload(outcome) {
                if let Some(page) = payload.page(session.outcome_page) {
                    if matches!(outcome, semio_framework_job::StepOutcome::Complete(_) | semio_framework_job::StepOutcome::Fault(_)) {
                        if let Err(error) = session.output.write_page(page) {
                            session.terminal = Some(GuestRelayMountedTerminal::HostFault(error));
                            if let Some(owner) = session.checked_out.take() {
                                owner.begin_close();
                            } else {
                                session.lifecycle_probe_checked_out.take().expect("faulted relay output retains its checked-out owner").begin_close();
                            }
                            session.lifecycle = GuestRelayMountedLifecycle::DrainingForCaller;
                        }
                    }
                    session.outcome_page += 1;
                }
            }
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if matches!(session.lifecycle, GuestRelayMountedLifecycle::DrainingForCaller) {
                waker.wake_by_ref();
                return std::task::Poll::Pending;
            }
            if outcome.terminal_is_empty() {
                Self::finish_outcome(session, waker);
            } else {
                waker.wake_by_ref();
            }
            return std::task::Poll::Pending;
        }
        if let GuestRelayMountedOwner::LifecycleProbe(owner) = &session.owner {
            match owner.try_step_on_caller() {
                Ok((ticket, semio_framework_job::WorkerJobPoll::Outcome)) => {
                    let mut checked = owner.take_outcome(ticket).map_err(|_| PluginHostError::Plugin("plugin relay lifecycle probe outcome unavailable".into()))?;
                    session.outcome = Some(checked.take_outcome());
                    session.lifecycle_probe_checked_out = Some(checked);
                    session.outcome_page = 0;
                    waker.wake_by_ref();
                }
                Ok((_, semio_framework_job::WorkerJobPoll::Terminal)) => {
                    let mut checked = owner.take_terminal().map_err(|_| PluginHostError::Plugin("plugin relay lifecycle probe terminal unavailable".into()))?;
                    session.outcome = Some(checked.take_outcome());
                    session.lifecycle_probe_checked_out = Some(checked);
                    session.outcome_page = 0;
                    waker.wake_by_ref();
                }
                Ok(_) => {}
                Err(_) => {
                    let _ = owner.register_wake(waker);
                }
            }
            return std::task::Poll::Pending;
        }
        let GuestRelayMountedOwner::Session(owner) = &session.owner else {
            if let GuestRelayMountedOwner::Rejected(rejected) = &mut session.owner {
                rejected.begin_close();
                let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if rejected.terminal_is_empty() {
                    session.owner = GuestRelayMountedOwner::Empty;
                    *slot = GuestRelayMountedSlot::Empty;
                    return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay admission rejected".into())));
                }
            }
            waker.wake_by_ref();
            return std::task::Poll::Pending;
        };
        match owner.try_step_on_caller() {
            Ok((ticket, semio_framework_job::WorkerJobPoll::Outcome)) => {
                let checked = owner.take_outcome(ticket).map_err(|_| PluginHostError::Plugin("plugin cold relay outcome unavailable".into()));
                match checked {
                    Ok(mut checked) => {
                        let _ = checked.job().register_wake(waker);
                        session.outcome = Some(checked.take_outcome());
                        session.checked_out = Some(checked);
                        session.outcome_page = 0;
                        waker.wake_by_ref();
                    }
                    Err(error) => return std::task::Poll::Ready(Err(error)),
                }
            }
            Ok((_, semio_framework_job::WorkerJobPoll::Terminal)) => match owner.take_terminal() {
                Ok(mut checked) => {
                    session.outcome = Some(checked.take_outcome());
                    session.checked_out = Some(checked);
                    session.outcome_page = 0;
                    waker.wake_by_ref();
                }
                Err(_) => return std::task::Poll::Ready(Err(PluginHostError::Plugin("plugin cold relay terminal unavailable".into()))),
            },
            Ok(_) => {}
            Err(_) => {
                let _ = owner.register_wake(waker);
            }
        };
        std::task::Poll::Pending
    }
}

enum GuestRelayMountedReap {
    Idle,
    Progress,
    Blocked { wake_registered: bool },
}

struct GuestRelayMountedReaper {
    registry: Arc<GuestRelayMountedRegistry>,
}

impl std::future::Future for GuestRelayMountedReaper {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        #[cfg(test)]
        {
            self.registry.reaper_polls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        let observed_epoch = self.registry.reaper_epoch.load(std::sync::atomic::Ordering::Acquire);
        match self.registry.reap_one(context.waker()) {
            GuestRelayMountedReap::Progress => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            GuestRelayMountedReap::Blocked { wake_registered } => {
                self.registry.park_blocked_reaper(context.waker(), wake_registered);
                std::task::Poll::Pending
            }
            GuestRelayMountedReap::Idle => {
                self.registry.reaper_active.store(false, std::sync::atomic::Ordering::Release);
                if self.registry.reaper_epoch.load(std::sync::atomic::Ordering::Acquire) != observed_epoch || self.registry.has_detached() {
                    self.registry.ensure_reaper();
                }
                std::task::Poll::Ready(())
            }
        }
    }
}

struct GuestRelayMountedFuture {
    registry: Arc<GuestRelayMountedRegistry>,
    index: usize,
    generation: u64,
    complete: bool,
}

impl std::future::Future for GuestRelayMountedFuture {
    type Output = Result<Vec<u8>, PluginHostError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let poll = self.registry.pump(self.index, self.generation, context.waker());
        if poll.is_ready() {
            self.complete = true;
        }
        poll
    }
}

impl Drop for GuestRelayMountedFuture {
    fn drop(&mut self) {
        if !self.complete {
            self.registry.detach(self.index, self.generation);
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuestRelayLifecycleTrace {
    #[serde(rename = "🪪️id")]
    id: String,
    machine: String,
    generation: u64,
    initial: String,
    events: Vec<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GuestRelayLifecycleProjection {
    state: String,
    first_reason: Option<String>,
    cancel_admissions: usize,
    release_opportunities: usize,
    caller_output: Option<String>,
}

fn guest_relay_lifecycle_barrier(pool: &WorkerPool) -> Result<(), String> {
    let (sender, receiver) = semio_framework_async::oneshot::channel();
    pool.submit_at(
        pool.now_ms().saturating_add(1),
        Lane::Maintenance,
        Box::new(move || {
            let _ = sender.send(());
        }),
    );
    semio_framework_async::block_on(async move { receiver.await }).map_err(|_| "relay lifecycle worker barrier closed".to_string())
}

fn guest_relay_lifecycle_wait(pool: &WorkerPool, mut ready: impl FnMut() -> bool, detail: &'static str) -> Result<(), String> {
    for _ in 0..256 {
        if ready() {
            return Ok(());
        }
        guest_relay_lifecycle_barrier(pool)?;
    }
    Err(detail.to_string())
}

fn guest_relay_lifecycle_probe_session(
    control: Arc<GuestRelayLifecycleProbeControl>,
    generation: u64,
    terminal_output: Option<Vec<u8>>,
    remaining: usize,
) -> Result<semio_framework_job::WorkerJobSession<GuestRelayLifecycleProbeJob>, String> {
    let job = GuestRelayLifecycleProbeJob { control: Arc::clone(&control), remaining, terminal_output, closing: false };
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::OperationId(generation),
        generation: semio_framework_job::Generation(generation),
        cancel: semio_framework_job::root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig {
            site: "plugin-host.relay-lifecycle",
            stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
            fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
            step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
        },
        now_us: semio_framework_job::default_now_us,
    };
    semio_framework_job::WorkerJobSession::try_new(job, params).map_err(|mut rejected| {
        rejected.begin_close();
        control.wake();
        control.release_one();
        control.release_one();
        for _ in 0..64 {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                break;
            }
        }
        "relay lifecycle probe admission refused".to_string()
    })
}

fn exercise_abandoned_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        let control = Arc::new(GuestRelayLifecycleProbeControl::new());
        let session = guest_relay_lifecycle_probe_session(Arc::clone(&control), generation, None, 2)?;
        registry.mount(index, generation, GuestRelayMountedOwner::LifecycleProbe(session));
        let mut first_reason = None;
        for event in &trace.events {
            if let Some(dropped) = event.strip_prefix("drop:").and_then(|value| value.parse::<u64>().ok()) {
                registry.detach(index, dropped);
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DetachedForReap) {
                    first_reason.get_or_insert_with(|| "detached".to_string());
                }
            } else if event == "blocked" {
                guest_relay_lifecycle_wait(
                    &pool,
                    || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
                    "relay lifecycle reaper did not park on the exact close wake",
                )?;
            } else if event == "wake" {
                control.wake();
                guest_relay_lifecycle_wait(
                    &pool,
                    || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
                    "relay lifecycle reaper did not park for its first release",
                )?;
            } else if event == "release" {
                let released = control.releases.load(std::sync::atomic::Ordering::Acquire);
                control.release_one();
                guest_relay_lifecycle_wait(
                    &pool,
                    || control.releases.load(std::sync::atomic::Ordering::Acquire) > released,
                    "relay lifecycle reaper did not consume its admitted release",
                )?;
            }
        }
        guest_relay_lifecycle_wait(
            &pool,
            || matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty),
            "relay lifecycle reaper did not reach Empty",
        )?;
        Ok(GuestRelayLifecycleProjection {
            state: "Empty".to_string(),
            first_reason,
            cancel_admissions: control.close_admissions.load(std::sync::atomic::Ordering::Acquire),
            release_opportunities: control.releases.load(std::sync::atomic::Ordering::Acquire),
            caller_output: None,
        })
    })();
    pool.shutdown();
    result
}

fn exercise_live_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        let mut first_reason = None;
        let mut releases = 0;
        let mut caller_output = None;
        for event in &trace.events {
            if let Some(output) = event.strip_prefix("terminal:") {
                let control = Arc::new(GuestRelayLifecycleProbeControl::new());
                control.wake();
                let session = guest_relay_lifecycle_probe_session(control, generation, Some(output.as_bytes().to_vec()), 0)?;
                registry.mount(index, generation, GuestRelayMountedOwner::LifecycleProbe(session));
                for _ in 0..256 {
                    let _ = registry.pump(index, generation, std::task::Waker::noop());
                    let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                        break;
                    }
                }
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                    return Err("relay lifecycle production terminal did not reach caller drain".to_string());
                }
                first_reason.get_or_insert_with(|| "caller".to_string());
            } else if event == "reap-other" {
                let control = Arc::new(GuestRelayLifecycleProbeControl::new());
                let (other_index, other_generation) = registry.reserve().ok_or_else(|| "relay lifecycle competing slot unavailable".to_string())?;
                let session = guest_relay_lifecycle_probe_session(Arc::clone(&control), other_generation, None, 2)?;
                registry.mount(other_index, other_generation, GuestRelayMountedOwner::LifecycleProbe(session));
                registry.detach(other_index, other_generation);
                guest_relay_lifecycle_wait(
                    &pool,
                    || control.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
                    "relay lifecycle competing reaper did not park",
                )?;
                control.wake();
                control.release_one();
                control.release_one();
                guest_relay_lifecycle_wait(
                    &pool,
                    || matches!(&*registry.slots[other_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty),
                    "relay lifecycle competing detached owner was not reclaimed",
                )?;
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if !matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller) {
                    return Err("relay lifecycle reaper stole the live caller's exact owner".to_string());
                }
            } else if event == "caller-release" {
                for _ in 0..256 {
                    match registry.pump(index, generation, std::task::Waker::noop()) {
                        std::task::Poll::Ready(Ok(bytes)) => {
                            caller_output = Some(String::from_utf8(bytes).map_err(|_| "relay lifecycle caller output is not UTF-8".to_string())?);
                            releases += 1;
                            break;
                        }
                        std::task::Poll::Ready(Err(error)) => return Err(error.to_string()),
                        std::task::Poll::Pending => {}
                    }
                }
                if caller_output.is_none() {
                    return Err("relay lifecycle caller did not receive its exact terminal output".to_string());
                }
            }
        }
        let state = if matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty) { "Empty" } else { "DrainingForCaller" };
        Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason, cancel_admissions: 0, release_opportunities: releases, caller_output })
    })();
    pool.shutdown();
    result
}

fn exercise_stale_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let result = (|| {
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
        registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
        let (index, generation) = registry.reserve().ok_or_else(|| "relay lifecycle slot unavailable".to_string())?;
        registry.mount(index, generation, GuestRelayMountedOwner::Empty);
        for event in &trace.events {
            if let Some(dropped) = event.strip_prefix("drop:").and_then(|value| value.parse::<u64>().ok()) {
                registry.detach(index, dropped);
            }
        }
        let state = {
            let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.generation == generation && session.lifecycle == GuestRelayMountedLifecycle::Running) {
                "Running"
            } else {
                return Err("relay lifecycle stale generation changed the live owner".to_string());
            }
        };
        *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
        Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason: None, cancel_admissions: 0, release_opportunities: 0, caller_output: None })
    })();
    pool.shutdown();
    result
}

fn exercise_capacity_relay_lifecycle_trace(trace: &GuestRelayLifecycleTrace) -> Result<GuestRelayLifecycleProjection, String> {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let registry = GuestRelayMountedRegistry::with_pool(pool.clone());
    registry.next_generation.store(trace.generation, std::sync::atomic::Ordering::Release);
    let mut reserved = Vec::new();
    let mut state = "Empty";
    for event in &trace.events {
        let Some(requested) = event.strip_prefix("reserve:").and_then(|value| value.parse::<usize>().ok()) else { continue };
        for _ in 0..requested {
            match registry.reserve() {
                Some(owner) => {
                    reserved.push(owner);
                    state = "ReservedFull";
                }
                None => state = "CapacityRefused",
            }
        }
    }
    for (index, _) in reserved {
        *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
    }
    pool.shutdown();
    Ok(GuestRelayLifecycleProjection { state: state.to_string(), first_reason: None, cancel_admissions: 0, release_opportunities: 0, caller_output: None })
}

/// 🧪️ Drives one schema-owned lifecycle trace through the production replay or mounted-relay transition machinery.
#[doc(hidden)]
pub fn exercise_relay_lifecycle_trace(trace_json: &str) -> Result<String, String> {
    let trace: GuestRelayLifecycleTrace = serde_json::from_str(trace_json).map_err(|error| format!("invalid relay lifecycle trace: {error}"))?;
    let projection = match (trace.id.as_str(), trace.machine.as_str(), trace.initial.as_str()) {
        ("replay-first-fault-wins", "replay", "CaptureKind") => {
            let actual = shard::exercise_replay_lifecycle_trace(&trace.events)?;
            GuestRelayLifecycleProjection {
                state: actual.state.to_string(),
                first_reason: actual.first_reason,
                cancel_admissions: 0,
                release_opportunities: actual.release_opportunities,
                caller_output: None,
            }
        }
        ("relay-abandoned-blocked-wake", "relay", "Running") => exercise_abandoned_relay_lifecycle_trace(&trace)?,
        ("relay-live-terminal-caller-output", "relay", "Running") => exercise_live_relay_lifecycle_trace(&trace)?,
        ("relay-stale-generation-refused", "relay", "Running") => exercise_stale_relay_lifecycle_trace(&trace)?,
        ("relay-max-plus-one-refused", "relay", "Empty") => exercise_capacity_relay_lifecycle_trace(&trace)?,
        _ => return Err(format!("unsupported relay lifecycle trace {:?}", trace.id)),
    };
    serde_json::to_string(&projection).map_err(|error| format!("relay lifecycle projection failed: {error}"))
}

/// 🧬️ design-runtime.md §2's post-turn dispatch replacement for the old `Arc<WasmPluginRuntime>`
/// synchronous-call handle. `IoRouter`/`ArtifactInferenceRouter` used to hold a full `WasmPluginRuntime`
/// (component + linker + `Store`) per plugin so a router method could call straight into the guest's
/// own exported functions (`artifact-compose`, `io-run`, `io-sniff`, `artifact-infer`) mid-turn. Those
/// exports do not exist on `world actor` at all — the ONLY guest entry points left are
/// `reactor::poll` and `jobs::{start-job,step-job,cancel-job}` (`📜️.wit`) — so a router now
/// drives a well-known cold job kind (`semio.io-run` / `semio.io-sniff` / `semio.infer`, all three
/// named in `jobs.wit`'s own doc comments) to completion via `GuestRuntime`, strictly AFTER the turn
/// that produced the triggering effect returned (never re-entrantly against that turn's own `Store`).
pub struct PluginInstanceHandle {
    pub actor: RuntimeActorId,
    runtime: Arc<GuestRuntimes>,
    instance: Arc<Mutex<GuestInstanceSlot>>,
    instance_gate: Arc<semio_framework_async::Semaphore>,
    relay_registry: Arc<GuestRelayMountedRegistry>,
    next_job_id: std::sync::atomic::AtomicU64,
}

impl PluginInstanceHandle {
    pub async fn new(actor: RuntimeActorId, runtime: Arc<GuestRuntimes>, instance: GuestInstance) -> Self {
        Self {
            actor,
            runtime,
            instance: Arc::new(Mutex::new(GuestInstanceSlot::Available(instance))),
            instance_gate: Arc::new(semio_framework_async::Semaphore::new(1)),
            relay_registry: Arc::new(GuestRelayMountedRegistry::new()),
            next_job_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// 🧵️ Starts one cold job in the fixed mounted registry. Each host poll admits at most one
    /// relay or close opportunity; the pool never owns an internal run-to-completion chain.
    async fn run_job_on_worker(&self, kind: &str, input: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        if input.len() > semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES {
            return Err(PluginHostError::Plugin(format!("{kind} input exceeds the retained operation byte limit")));
        }
        if let Some(detail) = self.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).admission_fault() {
            return Err(PluginHostError::Plugin(format!("{kind} job rejected: {}", String::from_utf8_lossy(&detail))));
        }
        let job = self.next_job_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let pool = plugin_host_worker_pool();
        let cancel = semio_framework_job::root_cancel_token();
        let operation = semio_framework_job::OperationId(self.actor.0);
        let generation = semio_framework_job::Generation(job);
        let params = semio_framework_job::BatchJobParams {
            operation,
            generation,
            cancel: semio_framework_job::root_cancel_token(),
            config: semio_framework_job::BatchDriveConfig {
                site: "plugin-host.cold-relay",
                stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
                fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
                step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
            },
            now_us: semio_framework_job::default_now_us,
        };
        let relay = GuestColdRelayJob::new(Arc::clone(&self.runtime), Arc::clone(&self.instance), Arc::clone(&self.instance_gate), pool.clone(), cancel, job, kind.to_string(), input);
        let (index, mounted_generation) = self.relay_registry.reserve().ok_or_else(|| PluginHostError::Plugin(format!("{kind} mounted relay registry is full")))?;
        let owner = match semio_framework_job::WorkerJobSession::try_new(relay, params) {
            Ok(session) => GuestRelayMountedOwner::Session(session),
            Err(rejected) => GuestRelayMountedOwner::Rejected(rejected),
        };
        self.relay_registry.mount(index, mounted_generation, owner);
        GuestRelayMountedFuture { registry: Arc::clone(&self.relay_registry), index, generation: mounted_generation, complete: false }.await
    }

    /// 🌉️ Executes ONE hop of this plugin's own local io mechanism registry — the absorbed `io-run`
    /// guest export, now `jobs.wit`'s cold job kind `semio.io-run`. `payload` is already JSON
    /// `io_schema::IoPayload` bytes (this method's own caller contract, mirroring the deleted
    /// `WasmPluginRuntime::io_run`'s); `input` bundles what used to be three separate export
    /// parameters into the one JSON `(source, target, IoPayload)` tuple `jobs.wit`'s doc comment
    /// specifies, since a job only carries one opaque `list<u8>`.
    pub async fn io_run(&self, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a> {
            source: &'a str,
            target: &'a str,
            payload: semio_framework::io_schema::IoPayload,
        }
        let payload_text = std::str::from_utf8(&payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = dsl::os_pack::json::from_json_str(payload_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = dsl::os_pack::json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        self.run_job_on_worker("semio.io-run", input).await
    }

    /// 🔍️ Sniffs this plugin's own `(from, into)` hop — the absorbed `io-sniff` guest export, now
    /// `semio.io-sniff`. Returns the raw `io_schema::Confidence::rank()` byte (`0`=None..`3`=High),
    /// mirroring the deleted `WasmPluginRuntime::io_sniff`'s return shape exactly.
    pub async fn io_sniff(&self, from: &str, into: &str, payload: &[u8]) -> Result<u8, PluginHostError> {
        #[derive(ToValue)]
        struct IoRunInputWire<'a> {
            source: &'a str,
            target: &'a str,
            payload: semio_framework::io_schema::IoPayload,
        }
        let payload_text = std::str::from_utf8(payload).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let io_payload: semio_framework::io_schema::IoPayload = dsl::os_pack::json::from_json_str(payload_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let input = dsl::os_pack::json::to_json_string(&IoRunInputWire { source: from, target: into, payload: io_payload }).into_bytes();
        let result = self.run_job_on_worker("semio.io-sniff", input).await?;
        result.first().copied().ok_or_else(|| PluginHostError::Plugin("semio.io-sniff job returned an empty result".to_string()))
    }

    /// 💡️ Executes one guest inference call — the absorbed `contributor.artifact-infer` export, now
    /// `semio.infer` (`types.wit`'s own doc comment names this job kind explicitly). `request`/the
    /// result are the SAME JSON `io_schema::ArtifactInferenceRequest`/`ArtifactInferenceResult` bytes
    /// the deleted `WasmPluginRuntime::artifact_infer` used — no tuple wrapping needed, since that
    /// call already took exactly one opaque payload.
    pub async fn infer(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.infer", request.to_vec()).await
    }

    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): executes one guest mutation-plan
    /// call — the absorbed `contributor.artifact-mutation-plan` export, now `semio.mutation-plan`.
    /// `request`/the result are the SAME DSL wire-pack bytes `ArtifactMutationRouter::plan` already
    /// builds via `encode_wire_dsl`/decodes via `decode_wire_dsl` (this region's own helpers,
    /// `HostArtifactMutationPlanRequest`/`Result`'s field-for-field guest mirror) — no tuple
    /// wrapping needed, mirroring `infer`'s own doc note above.
    pub async fn mutation_plan(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        self.run_job_on_worker("semio.mutation-plan", request.to_vec()).await
    }

    /// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): executes one guest versioned
    /// re-encode — the absorbed `migrate-artifact` export, now `semio.migrate`. `from`/`to` are
    /// `io_schema::ArtifactDialect::to_coordinate()` strings; `pack` is the bytes to re-encode.
    /// Bundles all three into one JSON tuple the SAME way `io_run` above bundles its own former
    /// three separate export parameters (`⚛️reactor/💼️jobs/🔀️migrate/🦀️.rs`'s own
    /// `MigrateInput` decodes a struct positionally from this tuple, matching `IoRunInput`'s
    /// established idiom). The caller that resolves WHICH `PluginInstanceHandle` to migrate through
    /// belongs to the pending runtime/db refactor (this method only provides the dispatch, not its
    /// call site — see this packet's report `## lease-requests`).
    pub async fn migrate(&self, from: &str, to: &str, pack: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct MigrateInputWire<'a> {
            from: &'a str,
            to: &'a str,
            pack: Vec<u8>,
        }
        let input = dsl::os_pack::json::to_json_string(&MigrateInputWire { from, to, pack }).into_bytes();
        self.run_job_on_worker("semio.migrate", input).await
    }

    /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): routes to this plugin's own
    /// `semio.compose` cold job — design-abi.md §2: "`artifact-compose` ... become well-known cold
    /// job kinds ... driven by `start-job` + `step-job`". The guest body is owned by the live
    /// `compose-await` packet (`ComposeStepper`/`ComposeState`, deliberately NOT defined here or
    /// anywhere in this file) — until it registers `"semio.compose"` via `register_job_kind`, this
    /// fails with the ordinary `job.unknown-kind` fault `step_job` already produces for any
    /// unregistered kind, not a hand-written host refusal. `key_bytes` is the JSON `IoKey`
    /// `IoRouter::compose` already resolved ownership from; `sources_bytes` passes through
    /// byte-for-byte. This wire shape is provisional until `compose-await`'s own guest decode is
    /// defined — coordinate any format change with that packet before altering it here.
    pub async fn compose(&self, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        #[derive(serde::Serialize, ToValue)]
        struct ComposeInput {
            key: Vec<u8>,
            sources: Vec<u8>,
        }
        let input = dsl::os_pack::json::to_json_string(&ComposeInput { key: key_bytes.to_vec(), sources: sources_bytes.to_vec() }).into_bytes();
        self.run_job_on_worker("semio.compose", input).await
    }
}

impl std::fmt::Debug for PluginInstanceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginInstanceHandle").field("actor", &self.actor).finish()
    }
}

#[cfg(test)]
mod guest_cold_relay_tests {
    use super::*;
    use semio_framework_job::InteractiveJob;

    fn relay_lifecycle_fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧫️fixtures/🔣️relay-lifecycle.json")).expect("relay lifecycle fixture")
    }

    #[test]
    fn neutral_relay_lifecycle_traces_drive_production_machines() {
        let _replay_authority = shard::replay_test_authority();
        for trace in relay_lifecycle_fixture()["traces"].as_array().expect("relay lifecycle traces") {
            let actual: serde_json::Value = serde_json::from_str(&exercise_relay_lifecycle_trace(&trace.to_string()).expect("production lifecycle trace")).expect("production lifecycle projection");
            assert_eq!(actual, trace["expected"], "{} production projection", trace["🪪️id"]);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_pool_future_retries_saturation_once_and_terminalizes_shutdown() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let (entered_sender, entered_receiver) = std::sync::mpsc::sync_channel(0);
        let (release_sender, release_receiver) = std::sync::mpsc::sync_channel(0);
        pool.submit(
            Lane::UserVisible,
            Box::new(move || {
                entered_sender.send(()).expect("blocked worker announces entry");
                release_receiver.recv().expect("blocked worker release");
            }),
        );
        entered_receiver.recv().expect("worker enters the blocking closure");
        for _ in 0..semio_framework_async::WORKER_JOBS_PER_LANE {
            pool.submit(Lane::UserVisible, Box::new(|| {}));
        }
        let (completion_sender, completion_receiver) = semio_framework_async::oneshot::channel();
        let failures = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let failure_count = Arc::clone(&failures);
        GuestRelayPoolFuture::spawn_recoverable(
            pool.clone(),
            Lane::UserVisible,
            async move {
                let _ = completion_sender.send(b"retried".to_vec());
            },
            move |_| {
                failure_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            },
        );
        release_sender.send(()).expect("release saturated worker");
        assert_eq!(completion_receiver.await.expect("retained future completion after saturation"), b"retried");
        assert_eq!(failures.load(std::sync::atomic::Ordering::SeqCst), 0);
        pool.shutdown();

        let ran = Arc::new(AtomicBool::new(false));
        let future_ran = Arc::clone(&ran);
        let (failure_sender, failure_receiver) = semio_framework_async::oneshot::channel();
        GuestRelayPoolFuture::spawn_recoverable(
            pool.clone(),
            Lane::UserVisible,
            async move {
                future_ran.store(true, std::sync::atomic::Ordering::SeqCst);
            },
            move |failure| {
                let _ = failure_sender.send(failure);
            },
        );
        assert!(matches!(
            failure_receiver.await.expect("shutdown terminal failure"),
            GuestRelayPoolFailure::Admission(semio_framework_async::WorkerSubmitErrorKind::Shutdown)
        ));
        assert!(!ran.load(std::sync::atomic::Ordering::SeqCst));
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool));
        let (index, generation) = registry.reserve().expect("pre-failure mounted slot");
        registry.mount(index, generation, GuestRelayMountedOwner::Empty);
        registry.detach(index, generation);
        assert!(registry.reaper_failed.load(std::sync::atomic::Ordering::Acquire));
        assert!(matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty));
        assert!(registry.reserve().is_none(), "a scheduler-failed registry must reject future mounts");

        let mock = Arc::new(MockGuestRuntime::new().await);
        let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_017)).await;
        let oversized = vec![0; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1];
        let error = handle.infer(&oversized).await.expect_err("oversized relay input");
        assert!(error.to_string().contains("input exceeds the retained operation byte limit"));
        assert_eq!(mock.start_admissions(), 0);
        let mut maximum_rejected_job = GuestColdRelayJob::new(
            Arc::clone(&handle.runtime),
            Arc::clone(&handle.instance),
            Arc::clone(&handle.instance_gate),
            registry.pool.clone(),
            semio_framework_async::CancelToken::root_now(),
            1,
            "semio.infer".to_string(),
            vec![0; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES],
        );
        maximum_rejected_job.begin_close();
        for _ in 0..semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES.saturating_add(3) {
            if matches!(
                maximum_rejected_job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES),
                semio_framework_job::InteractiveJobCloseStep::Complete
            ) {
                break;
            }
        }
        assert!(maximum_rejected_job.terminal_is_empty(), "maximum admitted pre-start relay closes within the fatal rejection bound");
    }

    /// 🧵️ Keeps fixed mounted capacity on the heap while every one-opportunity owner remains small
    /// enough for the native test stack on every supported architecture.
    #[test]
    fn mounted_relay_stack_authority_matches_the_neutral_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️stack-authority.json")).expect("stack authority fixture");
        let maximum = fixture["maximumInlineBytes"].as_u64().expect("maximum inline bytes") as usize;
        let registry = fixture["registries"].as_array().expect("registry rows").iter().find(|row| row["id"] == "guest-relay-mounted").expect("mounted relay row");
        assert_eq!(fixture["schemaVersion"], 1);
        assert_eq!(registry["capacity"].as_u64(), Some(GUEST_RELAY_MOUNTED_SLOTS as u64));
        assert_eq!(registry["storage"], "heap");
        assert_eq!(GuestRelayMountedRegistry::new().slots.len(), GUEST_RELAY_MOUNTED_SLOTS);
        assert!(std::mem::size_of::<GuestRelayMountedSlot>() <= maximum);
        assert!(std::mem::size_of::<GuestRelayMountedRegistry>() <= maximum);
    }

    #[derive(Debug, PartialEq, Eq)]
    enum TestRelayOutcome {
        Yield,
        Complete(Vec<u8>),
        Cancelled,
        Fault(Vec<u8>),
    }

    fn admit_test_relay(relay: GuestColdRelayJob, params: semio_framework_job::BatchJobParams) -> semio_framework_job::WorkerJobSession<GuestColdRelayJob> {
        match semio_framework_job::WorkerJobSession::try_new(relay, params) {
            Ok(session) => session,
            Err(mut rejected) => {
                rejected.begin_close();
                while !rejected.terminal_is_empty() {
                    let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                }
                panic!("relay test session admission");
            }
        }
    }

    fn test_payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(payload.len());
        for page in 0..payload.page_count() {
            if let Some(page) = payload.page(page) {
                bytes.extend_from_slice(page);
            }
        }
        bytes
    }

    async fn test_relay_step(session: &semio_framework_job::WorkerJobSession<GuestColdRelayJob>, _pool: &WorkerPool) -> TestRelayOutcome {
        if matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            return TestRelayOutcome::Yield;
        }
        let (ticket, poll) = session.try_step_on_caller().expect("relay test caller opportunity");
        let mut owner = match poll {
            semio_framework_job::WorkerJobPoll::Outcome => session.take_outcome(ticket).expect("relay test outcome"),
            semio_framework_job::WorkerJobPoll::Terminal => session.take_terminal().expect("relay test terminal"),
            _ => return TestRelayOutcome::Yield,
        };
        let mut outcome = owner.take_outcome();
        let result = match &outcome {
            semio_framework_job::StepOutcome::Complete(candidate) => TestRelayOutcome::Complete(test_payload_bytes(&candidate.output)),
            semio_framework_job::StepOutcome::Cancelled => TestRelayOutcome::Cancelled,
            semio_framework_job::StepOutcome::Fault(fault) => TestRelayOutcome::Fault(test_payload_bytes(&fault.detail)),
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => TestRelayOutcome::Yield,
        };
        while !outcome.terminal_is_empty() {
            let _ = outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if outcome.is_terminal() {
            owner.begin_close();
        } else if let Err(owner) = owner.resume() {
            owner.begin_close();
        }
        result
    }

    #[test]
    fn guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact() {
        let registry = GuestRelayMountedRegistry::new();
        let mut generations = Vec::new();
        for _ in 0..GUEST_RELAY_MOUNTED_SLOTS {
            let (index, generation) = registry.reserve().expect("exact mounted relay capacity");
            let output_identity = {
                let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let GuestRelayMountedSlot::Reserved { output, .. } = &*slot else { panic!("reserved output owner") };
                output.storage.as_ptr()
            };
            generations.push((index, generation, output_identity));
        }
        assert!(registry.reserve().is_none());
        assert!(generations.windows(2).all(|pair| pair[0].1 < pair[1].1));
        assert!(!registry.has_detached());
        for (index, generation, output_identity) in generations {
            let mut slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Reserved { generation: actual, output } = &*slot else { panic!("reserved output remains mounted") };
            assert_eq!(*actual, generation);
            assert_eq!(output.storage.as_ptr(), output_identity);
            *slot = GuestRelayMountedSlot::Empty;
        }
        let exhausted = GuestRelayMountedRegistry::new();
        exhausted.next_generation.store(u64::MAX, std::sync::atomic::Ordering::Release);
        let (index, generation) = exhausted.reserve().expect("last generation remains admissible");
        assert_eq!(generation, u64::MAX);
        *exhausted.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
        assert!(exhausted.reserve().is_none());
    }

    fn mounted_test_session(generation: u64, lifecycle: GuestRelayMountedLifecycle, output: &[u8]) -> GuestRelayMountedSession {
        let mut mounted_output = GuestRelayMountedOutput::new();
        mounted_output.write_page(output).expect("mounted test output");
        GuestRelayMountedSession {
            generation,
            owner: GuestRelayMountedOwner::Empty,
            checked_out: None,
            lifecycle_probe_checked_out: None,
            outcome: None,
            outcome_page: 0,
            output: mounted_output,
            terminal: Some(GuestRelayMountedTerminal::Complete),
            lifecycle,
        }
    }

    #[test]
    fn detached_reaper_reclaims_one_slot_per_opportunity_round_robin_and_refuses_stale_generation() {
        let fixture = relay_lifecycle_fixture();
        assert_eq!(fixture["schemaVersion"], 1);
        assert_eq!(fixture["capacities"]["mountedRelaySlots"].as_u64(), Some(GUEST_RELAY_MOUNTED_SLOTS as u64));
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
        let mut generations = Vec::new();
        for _ in 0..GUEST_RELAY_MOUNTED_SLOTS {
            let (index, generation) = registry.reserve().expect("full detached fixture capacity");
            registry.mount(index, generation, GuestRelayMountedOwner::Empty);
            let mut slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let GuestRelayMountedSlot::Mounted(session) = &mut *slot else { panic!("mounted fixture") };
            session.lifecycle = GuestRelayMountedLifecycle::DetachedForReap;
            generations.push((index, generation));
        }
        for released in 1..=GUEST_RELAY_MOUNTED_SLOTS {
            assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Progress));
            let empty = registry.slots.iter().filter(|slot| matches!(&*slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty)).count();
            assert_eq!(empty, released, "one reaper opportunity releases one rotating slot");
        }
        assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Idle));
        let (index, generation) = registry.reserve().expect("reclaimed capacity is reusable");
        registry.mount(index, generation, GuestRelayMountedOwner::Empty);
        registry.detach(index, generations[0].1);
        let slot = registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(matches!(&*slot, GuestRelayMountedSlot::Mounted(session) if session.generation == generation && session.lifecycle == GuestRelayMountedLifecycle::Running));
        drop(slot);
        *registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Empty;
        pool.shutdown();
    }

    #[test]
    fn detached_reaper_never_steals_a_live_draining_callers_exact_output() {
        let fixture = relay_lifecycle_fixture();
        let expected = fixture["traces"].as_array().expect("fixture traces").iter().find(|trace| trace["🪪️id"] == "relay-live-terminal-caller-output").and_then(|trace| trace["expected"]["callerOutput"].as_str()).expect("caller output trace");
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
        let (detached_index, detached_generation) = registry.reserve().expect("detached slot");
        *registry.slots[detached_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Mounted(mounted_test_session(detached_generation, GuestRelayMountedLifecycle::DetachedForReap, &[]));
        let (live_index, live_generation) = registry.reserve().expect("live slot");
        *registry.slots[live_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner) = GuestRelayMountedSlot::Mounted(mounted_test_session(live_generation, GuestRelayMountedLifecycle::DrainingForCaller, expected.as_bytes()));

        assert!(matches!(registry.reap_one(std::task::Waker::noop()), GuestRelayMountedReap::Progress));
        assert!(matches!(&*registry.slots[detached_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty));
        assert!(matches!(&*registry.slots[live_index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DrainingForCaller));
        assert!(matches!(registry.pump(live_index, live_generation, std::task::Waker::noop()), std::task::Poll::Ready(Ok(bytes)) if bytes == expected.as_bytes()));
        pool.shutdown();
    }

    #[test]
    fn guest_cold_relay_publication_max_plus_one_selects_retained_fault_without_losing_source() {
        let mut maximum = GuestRelayPublication::new(GuestRelayPublicationKind::Commit, vec![7; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]);
        assert!(!maximum.oversized);
        assert_eq!(maximum.source.len(), semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES);
        let mut plus_one = GuestRelayPublication::new(GuestRelayPublicationKind::Commit, vec![9; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1]);
        assert!(plus_one.oversized);
        assert_eq!(plus_one.source.len(), semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1);
        assert!(matches!(plus_one.kind, GuestRelayPublicationKind::Fault));
        maximum.begin_close();
        plus_one.begin_close();
        while !maximum.terminal_is_empty() {
            let _ = maximum.close_step(1, semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1);
        }
        while !plus_one.terminal_is_empty() {
            let _ = plus_one.close_step(1, semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES + 1);
        }
    }

    async fn relay_session_with_tokens(
        mock: Arc<MockGuestRuntime>,
        actor: RuntimeActorId,
        pool: WorkerPool,
        relay_cancel: semio_framework_async::CancelToken,
        session_cancel: semio_framework_async::CancelToken,
        step: JobStep,
    ) -> (semio_framework_job::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>, Arc<Mutex<GuestInstanceSlot>>) {
        let compiled = mock.compile(&PackageRef { package: PackageId("relay-test".to_string()), hash: PackageHash([41; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4_096, max_frames: 1 }).await.expect("mock instantiate");
        let instance = Arc::new(Mutex::new(GuestInstanceSlot::Available(instance)));
        let gate = mock.script_pending_job_step(actor, step).await;
        let relay = GuestColdRelayJob::new(Arc::new(GuestRuntimes::Mock(mock)), Arc::clone(&instance), Arc::new(semio_framework_async::Semaphore::new(1)), pool, relay_cancel, 1, "semio.infer".to_string(), b"request".to_vec());
        let params = semio_framework_job::BatchJobParams {
            operation: semio_framework_job::OperationId(actor.0),
            generation: semio_framework_job::Generation(1),
            cancel: session_cancel,
            config: semio_framework_job::BatchDriveConfig {
                site: "test.plugin-host.guest-cold-relay",
                stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
                fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
                step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
            },
            now_us: semio_framework_job::default_now_us,
        };
        (admit_test_relay(relay, params), gate, instance)
    }

    async fn relay_session(
        mock: Arc<MockGuestRuntime>,
        actor: RuntimeActorId,
        pool: WorkerPool,
        cancel: semio_framework_async::CancelToken,
        step: JobStep,
    ) -> (semio_framework_job::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>, Arc<Mutex<GuestInstanceSlot>>) {
        relay_session_with_tokens(mock, actor, pool, cancel.clone(), cancel, step).await
    }

    async fn mounted_handle(mock: Arc<MockGuestRuntime>, actor: RuntimeActorId) -> PluginInstanceHandle {
        let compiled = mock.compile(&PackageRef { package: PackageId("mounted-relay-test".to_string()), hash: PackageHash([43; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4_096, max_frames: 1 }).await.expect("mock instantiate");
        PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock)), instance).await
    }

    async fn relay_session_for_handle(
        handle: &PluginInstanceHandle,
        mock: &MockGuestRuntime,
        pool: WorkerPool,
        relay_cancel: semio_framework_async::CancelToken,
        session_cancel: semio_framework_async::CancelToken,
        step: JobStep,
    ) -> (semio_framework_job::WorkerJobSession<GuestColdRelayJob>, Arc<MockJobStepGate>) {
        let gate = mock.script_pending_job_step(handle.actor, step).await;
        let relay = GuestColdRelayJob::new(Arc::clone(&handle.runtime), Arc::clone(&handle.instance), Arc::clone(&handle.instance_gate), pool, relay_cancel, 77, "semio.infer".to_string(), b"request".to_vec());
        let params = semio_framework_job::BatchJobParams {
            operation: semio_framework_job::OperationId(handle.actor.0),
            generation: semio_framework_job::Generation(77),
            cancel: session_cancel,
            config: semio_framework_job::BatchDriveConfig {
                site: "test.plugin-host.guest-cold-relay.mounted",
                stage: semio_framework_job::InteractiveStage::UserVisibleSimStep,
                fuel_per_step: semio_framework_job::USER_VISIBLE_LANE_FUEL,
                step_budget_us: semio_framework_job::USER_VISIBLE_LANE_WALL_US,
            },
            now_us: semio_framework_job::default_now_us,
        };
        (admit_test_relay(relay, params), gate)
    }

    async fn drive_until_step_admitted(session: &semio_framework_job::WorkerJobSession<GuestColdRelayJob>, pool: &WorkerPool, mock: &MockGuestRuntime) {
        for _ in 0..64 {
            assert_eq!(test_relay_step(session, pool).await, TestRelayOutcome::Yield);
            if mock.step_admissions() == 1 {
                return;
            }
            pool_timer_barrier(pool).await;
        }
        panic!("pending guest step was not admitted");
    }

    async fn wait_for_cancel_admission(pool: &WorkerPool, mock: &MockGuestRuntime) {
        for _ in 0..64 {
            let _ = semio_framework_job::pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if mock.cancel_admissions() == 1 {
                return;
            }
            let (sender, receiver) = semio_framework_async::oneshot::channel();
            pool.submit_at(
                pool.now_ms().saturating_add(1),
                Lane::UserVisible,
                Box::new(move || {
                    let _ = sender.send(());
                }),
            );
            receiver.await.expect("timer barrier");
        }
        panic!("guest cancellation was not admitted");
    }

    async fn wait_for_quarantine(pool: &WorkerPool, instance: &Arc<Mutex<GuestInstanceSlot>>) {
        for _ in 0..64 {
            if instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_quarantined() {
                return;
            }
            let (sender, receiver) = semio_framework_async::oneshot::channel();
            pool.submit_at(
                pool.now_ms().saturating_add(1),
                Lane::UserVisible,
                Box::new(move || {
                    let _ = sender.send(());
                }),
            );
            receiver.await.expect("quarantine timer barrier");
        }
        panic!("guest instance was not quarantined");
    }

    async fn wait_for_available_instance(pool: &WorkerPool, instance: &Arc<Mutex<GuestInstanceSlot>>) {
        for _ in 0..64 {
            if instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available() {
                return;
            }
            let (sender, receiver) = semio_framework_async::oneshot::channel();
            pool.submit_at(
                pool.now_ms().saturating_add(1),
                Lane::UserVisible,
                Box::new(move || {
                    let _ = sender.send(());
                }),
            );
            receiver.await.expect("instance restoration timer barrier");
        }
        panic!("guest instance was not restored");
    }

    async fn wait_for_release_barrier(pool: &WorkerPool, barrier: &MockJobStepGate) {
        for _ in 0..64 {
            if barrier.is_waiting() {
                return;
            }
            let (sender, receiver) = semio_framework_async::oneshot::channel();
            pool.submit_at(
                pool.now_ms().saturating_add(1),
                Lane::UserVisible,
                Box::new(move || {
                    let _ = sender.send(());
                }),
            );
            receiver.await.expect("release barrier timer");
        }
        panic!("guest relay did not reach its post-release barrier");
    }

    fn spawn_mounted_infer(pool: WorkerPool, handle: Arc<PluginInstanceHandle>, request: &'static [u8]) -> semio_framework_async::oneshot::Receiver<Result<Vec<u8>, PluginHostError>> {
        let (sender, receiver) = semio_framework_async::oneshot::channel();
        GuestRelayPoolFuture::spawn_recoverable(
            pool,
            Lane::UserVisible,
            async move {
                let _ = sender.send(handle.infer(request).await);
            },
            |_| {},
        );
        receiver
    }

    async fn pool_timer_barrier(pool: &WorkerPool) {
        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit_at(pool.now_ms().saturating_add(1), Lane::Maintenance, Box::new(move || {
            let _ = sender.send(());
        }));
        receiver.await.expect("maintenance timer barrier");
    }

    #[semio_framework_async_macros::async_test]
    async fn dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll() {
        let fixture = relay_lifecycle_fixture();
        let expected_cancels = fixture["traces"].as_array().expect("fixture traces").iter().find(|trace| trace["🪪️id"] == "relay-abandoned-blocked-wake").and_then(|trace| trace["expected"]["cancelAdmissions"].as_u64()).expect("cancel trace");
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_016);
        let cancel = semio_framework_async::CancelToken::root_now();
        let (session, gate, instance) = relay_session(Arc::clone(&mock), actor, pool.clone(), cancel.clone(), JobStep::Done { output: b"abandoned".to_vec() }).await;
        let cancel_gate = mock.script_pending_cancel_job();
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
        let (index, generation) = registry.reserve().expect("mounted relay slot");
        registry.mount(index, generation, GuestRelayMountedOwner::Session(session));
        let mut future = Box::pin(GuestRelayMountedFuture { registry: Arc::clone(&registry), index, generation, complete: false });
        let mut admitted = false;
        for _ in 0..64 {
            let poll = std::future::poll_fn(|context| std::task::Poll::Ready(std::future::Future::poll(future.as_mut(), context))).await;
            assert!(poll.is_pending(), "gated mounted relay remains pending");
            if mock.step_admissions() == 1 && gate.is_waiting() {
                admitted = true;
                break;
            }
            pool_timer_barrier(&pool).await;
        }
        assert!(admitted, "the pending guest step must be owned before detachment");
        drop(future);
        assert!(matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Mounted(session) if session.lifecycle == GuestRelayMountedLifecycle::DetachedForReap));
        for _ in 0..64 {
            if registry.reaper_wake_waits.load(std::sync::atomic::Ordering::SeqCst) == 1 {
                break;
            }
            pool_timer_barrier(&pool).await;
        }
        assert_eq!(registry.reaper_wake_waits.load(std::sync::atomic::Ordering::SeqCst), 1, "the detached session parks once on its relay-completion waker");
        assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "a wake-capable detached session needs no timer fallback");
        let blocked_polls = registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst);
        for _ in 0..8 {
            pool_timer_barrier(&pool).await;
        }
        assert_eq!(registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst), blocked_polls, "an unwoken detached session performs zero close rechecks");
        assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "an unwoken detached session creates zero timers");
        cancel_gate.release();
        let mut reaped = false;
        for _ in 0..256 {
            if matches!(&*registry.slots[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner), GuestRelayMountedSlot::Empty) {
                reaped = true;
                break;
            }
            pool_timer_barrier(&pool).await;
        }
        assert!(reaped, "the registry-owned reaper must release the detached slot without foreground pump");
        let post_wake_polls = registry.reaper_polls.load(std::sync::atomic::Ordering::SeqCst) - blocked_polls;
        assert!((1..=64).contains(&post_wake_polls), "one actual wake permits only a finite close sequence, got {post_wake_polls}");
        assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "wake-driven close remains timer-free through terminal release");
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_available_instance(&pool, &instance).await;
        assert!(cancel.is_cancelled_now());
        assert_eq!(mock.cancel_admissions(), expected_cancels as usize);
        assert_eq!(mock.step_admissions(), 1);
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn wake_incapable_close_uses_one_coalesced_bounded_fallback() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let registry = Arc::new(GuestRelayMountedRegistry::with_pool(pool.clone()));
        registry.park_blocked_reaper(std::task::Waker::noop(), true);
        assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 0, "wake-capable owners never enter fallback timing");
        registry.park_blocked_reaper(std::task::Waker::noop(), false);
        registry.park_blocked_reaper(std::task::Waker::noop(), false);
        assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 1, "a wake-incapable owner receives one coalesced fallback");
        assert!(registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire));
        for _ in 0..64 {
            if !registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire) {
                break;
            }
            pool_timer_barrier(&pool).await;
        }
        assert!(!registry.reaper_timer_pending.load(std::sync::atomic::Ordering::Acquire), "the bounded fallback relinquishes its pending ownership");
        assert_eq!(registry.reaper_timers.load(std::sync::atomic::Ordering::SeqCst), 1, "the fallback never duplicates itself while coalesced");
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn pending_guest_releases_the_only_worker_and_admits_no_duplicate_step() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let mock = Arc::new(MockGuestRuntime::new().await);
        let cancel = semio_framework_async::CancelToken::root_now();
        let (session, gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_001), pool.clone(), cancel, JobStep::Done { output: b"done".to_vec() }).await;
        drive_until_step_admitted(&session, &pool, &mock).await;

        for _ in 0..8 {
            assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
        }
        assert_eq!(mock.start_admissions(), 1);
        assert_eq!(mock.step_admissions(), 1, "pending caller turns must only try-receive the admitted guest future");

        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit(
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(b"competitor-progress".to_vec());
            }),
        );
        assert_eq!(receiver.await.expect("competing user-visible job"), b"competitor-progress", "a genuinely pending guest must not park the single worker");

        gate.release();
        let terminal = loop {
            let outcome = test_relay_step(&session, &pool).await;
            if !matches!(outcome, TestRelayOutcome::Yield) {
                break outcome;
            }
        };
        assert_eq!(terminal, TestRelayOutcome::Complete(b"done".to_vec()));
        assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
        assert_eq!(mock.step_admissions(), 1, "neither pending polls nor terminal replay may duplicate guest admission");
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn cancellation_race_admits_one_guest_cancel_and_one_terminal_outcome() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let mock = Arc::new(MockGuestRuntime::new().await);
        let cancel = semio_framework_async::CancelToken::root_now();
        let (session, _gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_002), pool.clone(), cancel.clone(), JobStep::Done { output: b"raced".to_vec() }).await;
        drive_until_step_admitted(&session, &pool, &mock).await;

        cancel.cancel_now();
        let terminal = loop {
            let outcome = test_relay_step(&session, &pool).await;
            if !matches!(outcome, TestRelayOutcome::Yield) {
                break outcome;
            }
        };
        assert_eq!(terminal, TestRelayOutcome::Cancelled);
        assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield, "the terminal cancellation must not be delivered twice");
        drop(session);
        wait_for_cancel_admission(&pool, &mock).await;
        assert_eq!(mock.step_admissions(), 1);
        assert_eq!(mock.cancel_admissions(), 1, "the completion/cancellation race must admit guest cancellation exactly once");
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn dropping_a_live_nonterminal_relay_cancels_the_guest_exactly_once() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let mock = Arc::new(MockGuestRuntime::new().await);
        let cancel = semio_framework_async::CancelToken::root_now();
        let (session, _gate, _) = relay_session(Arc::clone(&mock), RuntimeActorId(8_003), pool.clone(), cancel.clone(), JobStep::Done { output: b"unreachable".to_vec() }).await;
        drive_until_step_admitted(&session, &pool, &mock).await;

        drop(session);
        wait_for_cancel_admission(&pool, &mock).await;
        assert!(cancel.is_cancelled_now(), "dropping an admitted nonterminal relay must cancel its owned scope before scheduling guest cleanup");
        assert_eq!(mock.step_admissions(), 1);
        assert_eq!(mock.cancel_admissions(), 1, "the pending-step and Drop cleanup paths share one cancellation admission bit");
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn mounted_start_panic_restores_the_instance_and_the_next_route_progresses() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_004);
        let handle = mounted_handle(Arc::clone(&mock), actor).await;
        mock.script_job_step(actor, JobStep::Done { output: b"mounted-after-start-panic".to_vec() }).await;
        mock.panic_next_start();

        let error = handle.infer(b"first").await.expect_err("start panic must surface as a typed host fault");
        assert!(error.to_string().contains("plugin guest relay panicked"));
        let pool = plugin_host_worker_pool();
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_available_instance(&pool, &handle.instance).await;
        assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available(), "the unwind lease must restore the taken instance");

        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit(
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(());
            }),
        );
        receiver.await.expect("the process worker pool must survive the retained-future panic");
        assert_eq!(handle.infer(b"second").await.expect("the next mounted route must acquire the restored instance"), b"mounted-after-start-panic");
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn mounted_step_panic_restores_the_instance_and_terminalizes_once() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_005);
        let handle = mounted_handle(Arc::clone(&mock), actor).await;
        mock.script_job_step(actor, JobStep::Done { output: b"mounted-after-step-panic".to_vec() }).await;
        mock.panic_next_step();

        let error = handle.infer(b"first").await.expect_err("step panic must surface as a typed host fault");
        assert!(error.to_string().contains("plugin guest relay panicked"));
        let pool = plugin_host_worker_pool();
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_available_instance(&pool, &handle.instance).await;
        assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available());
        assert_eq!(handle.infer(b"second").await.expect("a mounted route after step panic must not deadlock"), b"mounted-after-step-panic");
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_panic_quarantines_instance_releases_permit_and_faults_once_on_one_worker() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let mock = Arc::new(MockGuestRuntime::new().await);
        let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_006)).await;
        let relay_cancel = semio_framework_async::CancelToken::root_now();
        let session_cancel = semio_framework_async::CancelToken::root_now();
        let (session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), relay_cancel.clone(), session_cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
        drive_until_step_admitted(&session, &pool, &mock).await;
        mock.panic_next_cancel();
        relay_cancel.cancel_now();

        let terminal = loop {
            let outcome = test_relay_step(&session, &pool).await;
            if !matches!(outcome, TestRelayOutcome::Yield) {
                break outcome;
            }
        };
        assert_eq!(terminal, TestRelayOutcome::Fault(b"plugin instance quarantined after cancel-job panic".to_vec()));
        assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
        wait_for_cancel_admission(&pool, &mock).await;
        assert_eq!(mock.cancel_admissions(), 1, "a panicking cancel-job admission must never be retried");
        assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_quarantined(), "cancel unwind must retain but quarantine the uncertain instance before fault delivery");

        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit(
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(());
            }),
        );
        receiver.await.expect("the sole worker and semaphore permit must survive cancel panic");
        pool.shutdown();
        let next = handle.infer(b"next-route").await.expect_err("a mounted route must reject rather than reuse a quarantined guest");
        assert!(next.to_string().contains("quarantined after cancel-job panic"));
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_008)).await;
        mock.panic_next_start();
        mock.panic_next_cancel();

        let first = handle.infer(b"first").await.expect_err("start panic must fault");
        assert!(first.to_string().contains("plugin guest relay panicked"));
        let pool = plugin_host_worker_pool();
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_quarantine(&pool, &handle.instance).await;
        let next = handle.infer(b"next-route").await.expect_err("the mounted route must reject background-cancel quarantine without deadlock");
        assert!(next.to_string().contains("quarantined after cleanup cancel-job panic"));
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn context_cancellation_failure_faults_once_quarantines_and_releases_one_worker() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let mock = Arc::new(MockGuestRuntime::new().await);
        let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_009)).await;
        let relay_cancel = semio_framework_async::CancelToken::root_now();
        let session_cancel = semio_framework_async::CancelToken::root_now();
        let (session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), relay_cancel.clone(), session_cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
        drive_until_step_admitted(&session, &pool, &mock).await;
        mock.fail_next_cancel();
        relay_cancel.cancel_now();

        let terminal = loop {
            let outcome = test_relay_step(&session, &pool).await;
            if !matches!(outcome, TestRelayOutcome::Yield) {
                break outcome;
            }
        };
        assert!(matches!(
            terminal,
            TestRelayOutcome::Fault(fault)
                if fault == b"plugin instance quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"
        ));
        assert_eq!(test_relay_step(&session, &pool).await, TestRelayOutcome::Yield);
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_quarantine(&pool, &handle.instance).await;
        assert_eq!(mock.cancel_admissions(), 1, "ordinary cancel failure must consume the sole admission without retry");

        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit(
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(b"worker-survived".to_vec());
            }),
        );
        assert_eq!(receiver.await.expect("one-worker competitor after cancel failure"), b"worker-survived");
        pool.shutdown();

        let start_admissions = mock.start_admissions();
        let next = handle.infer(b"next-route").await.expect_err("quarantined mounted route must reject without entering the guest");
        assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
        assert_eq!(mock.start_admissions(), start_admissions, "quarantine rejection must not enter start-job");
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn drop_cleanup_cancel_failure_quarantines_before_the_next_mounted_route() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
        let mock = Arc::new(MockGuestRuntime::new().await);
        let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_010)).await;
        let cancel = semio_framework_async::CancelToken::root_now();
        let (session, _gate) = relay_session_for_handle(&handle, &mock, pool.clone(), cancel.clone(), cancel, JobStep::Done { output: b"unreachable".to_vec() }).await;
        drive_until_step_admitted(&session, &pool, &mock).await;
        mock.fail_next_cancel();

        drop(session);
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_quarantine(&pool, &handle.instance).await;
        assert_eq!(mock.cancel_admissions(), 1, "Drop cleanup must not retry an ordinary cancel failure");
        let (sender, receiver) = semio_framework_async::oneshot::channel();
        pool.submit(
            Lane::UserVisible,
            Box::new(move || {
                let _ = sender.send(());
            }),
        );
        receiver.await.expect("one-worker permit and worker survive Drop cleanup failure");
        pool.shutdown();

        let start_admissions = mock.start_admissions();
        let next = handle.infer(b"next-route").await.expect_err("stored Drop cleanup quarantine must reject promptly");
        assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
        assert_eq!(mock.start_admissions(), start_admissions);
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn start_failure_cleanup_cancel_failure_is_stored_for_the_next_route() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let handle = mounted_handle(Arc::clone(&mock), RuntimeActorId(8_011)).await;
        mock.fail_next_start();
        mock.fail_next_cancel();

        let first = handle.infer(b"first").await.expect_err("ordinary start failure must fault");
        assert!(first.to_string().contains("guest trapped: scripted start-job failure"));
        let pool = plugin_host_worker_pool();
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_quarantine(&pool, &handle.instance).await;
        let start_admissions = mock.start_admissions();
        let next = handle.infer(b"next-route").await.expect_err("start-failure cleanup quarantine must reject promptly");
        assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
        assert_eq!(mock.start_admissions(), start_admissions);
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn step_failure_cleanup_cancel_failure_is_stored_for_the_next_route() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_012);
        let handle = mounted_handle(Arc::clone(&mock), actor).await;
        mock.script_fault(actor, "scripted step-job failure").await;
        mock.fail_next_cancel();

        let first = handle.infer(b"first").await.expect_err("ordinary step failure must fault");
        assert!(first.to_string().contains("guest trapped: scripted step-job failure"));
        let pool = plugin_host_worker_pool();
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_quarantine(&pool, &handle.instance).await;
        let start_admissions = mock.start_admissions();
        let next = handle.infer(b"next-route").await.expect_err("step-failure cleanup quarantine must reject promptly");
        assert!(next.to_string().contains("quarantined after cancel-job failure: guest trapped: scripted cancel-job failure"));
        assert_eq!(mock.start_admissions(), start_admissions);
        assert_eq!(mock.step_admissions(), 1);
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn concurrent_route_rejects_start_failure_cleanup_pending_then_reuses_only_after_cleanup_success() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_013);
        let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
        let release = mock.pause_after_next_start_failure_release();
        mock.fail_next_start();
        let pool = plugin_host_worker_pool();
        let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

        wait_for_release_barrier(&pool, &release).await;
        assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
        assert_eq!(mock.cancel_admissions(), 0, "the barrier must precede cleanup scheduling");
        let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
        let second_error = second.await.expect("concurrent mounted result").expect_err("cleanup-pending route must reject promptly");
        assert!(second_error.to_string().contains("cleanup pending after start-job failure"));
        assert_eq!(mock.start_admissions(), 1, "cleanup-pending rejection must not re-enter start-job");

        release.release();
        let first_error = first.await.expect("failed producer result").expect_err("ordinary start fault");
        assert!(first_error.to_string().contains("scripted start-job failure"));
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_available_instance(&pool, &handle.instance).await;
        mock.script_job_step(actor, JobStep::Done { output: b"clean-reuse".to_vec() }).await;
        assert_eq!(handle.infer(b"after-cleanup").await.expect("cleanup success restores mounted availability"), b"clean-reuse");
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn concurrent_route_rejects_step_failure_cleanup_pending_then_quarantine_is_stable() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_014);
        let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
        let release = mock.pause_after_next_step_failure_release();
        mock.script_fault(actor, "scripted concurrent step-job failure").await;
        mock.fail_next_cancel();
        let pool = plugin_host_worker_pool();
        let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

        wait_for_release_barrier(&pool, &release).await;
        assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
        let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
        let second_error = second.await.expect("concurrent mounted result").expect_err("cleanup-pending route must not enter the guest");
        assert!(second_error.to_string().contains("cleanup pending after step-job failure"));
        assert_eq!(mock.start_admissions(), 1);
        assert_eq!(mock.step_admissions(), 1);

        release.release();
        let first_error = first.await.expect("failed producer result").expect_err("ordinary step fault");
        assert!(first_error.to_string().contains("scripted concurrent step-job failure"));
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_quarantine(&pool, &handle.instance).await;
        let third_error = handle.infer(b"after-failed-cleanup").await.expect_err("cancel failure must leave a stable quarantine");
        assert!(third_error.to_string().contains("quarantined after cancel-job failure"));
        assert_eq!(mock.start_admissions(), 1);
        assert_eq!(mock.step_admissions(), 1);
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn concurrent_route_rejects_retained_start_panic_cleanup_pending_before_recovery() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_015);
        let handle = Arc::new(mounted_handle(Arc::clone(&mock), actor).await);
        let release = mock.pause_after_next_start_failure_release();
        mock.panic_next_start();
        let pool = plugin_host_worker_pool();
        let first = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"first");

        wait_for_release_barrier(&pool, &release).await;
        assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_cleanup_pending());
        let second = spawn_mounted_infer(pool.clone(), Arc::clone(&handle), b"concurrent");
        let second_error = second.await.expect("concurrent mounted result").expect_err("panic cleanup-pending route must reject promptly");
        assert!(second_error.to_string().contains("cleanup pending after guest relay panic"));
        assert_eq!(mock.start_admissions(), 1, "panic cleanup-pending rejection must not re-enter start-job");

        release.release();
        let first_error = first.await.expect("panicking producer result").expect_err("retained start panic must fault");
        assert!(first_error.to_string().contains("plugin guest relay panicked"));
        wait_for_cancel_admission(&pool, &mock).await;
        wait_for_available_instance(&pool, &handle.instance).await;
        mock.script_job_step(actor, JobStep::Done { output: b"panic-clean-reuse".to_vec() }).await;
        assert_eq!(handle.infer(b"after-panic-cleanup").await.expect("panic cleanup success restores availability"), b"panic-clean-reuse");
        assert_eq!(mock.cancel_admissions(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn poisoned_instance_slot_recovers_without_losing_the_mounted_route() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(8_007);
        let handle = mounted_handle(Arc::clone(&mock), actor).await;
        mock.script_job_step(actor, JobStep::Done { output: b"poison-recovered".to_vec() }).await;
        let instance = Arc::clone(&handle.instance);
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = instance.lock().expect("unpoisoned fixture slot");
            panic!("scripted instance-slot poison");
        }));
        assert!(handle.instance.is_poisoned());
        assert_eq!(handle.infer(b"request").await.expect("poison recovery must preserve the resident instance"), b"poison-recovered");
        assert!(handle.instance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_available());
    }
}
//#endregion 🔀️PostTurnRelay

/// 🧪️ Only `host_transaction_coordinator_tests`' `FakeCluster` fixture calls this now — the old
/// `HostState`'s WIT `host.*` import handlers (which encoded EVERY fault this way) are gone with
/// `WasmPluginRuntime`. Kept (not deleted) because that fixture still needs it; `cfg_attr` silences
/// the resulting "never used" warning on a plain (non-test) `--lib` build without hiding a REAL
/// dead-code case under a blanket `#[allow]`.
#[cfg_attr(not(test), allow(dead_code))]
// 🚫️async: E1 — pure in-memory encoder, no suspension point; reverted per R9 (its only
// consumer FakeCluster::exchange must itself be sync to satisfy run_transaction/undo_group
// production FnMut(...) -> Result<...> closure signature).
fn host_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    let code = code.into();
    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message))
}

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
    // 🚫️async: E1 pure constructor consumed by `impl Default` (external trait, sync-only) — R9.
    pub fn new() -> Self {
        Self { last_published_ms: None }
    }

    /// 📡️ Samples `kernel` and returns the pack-encoded `os.runtime.metrics` payload
    /// (`semio_framework_actor::RuntimeMetricsSnapshot::pack_encode`) when the 2Hz interval elapsed,
    /// else `None`. `shard_heartbeats` maps each live `ShardId` to its transport's last
    /// `ShardTransport::heartbeat()` reading — the overlay `Kernel::shard_metrics_samples` cannot do
    /// itself (see this struct's doc comment).
    pub async fn maybe_sample(&mut self, kernel: &semio_framework_actor::Kernel, now_ms: u64, shard_heartbeats: &HashMap<semio_framework_actor::ShardId, u64>) -> Option<Vec<u8>> {
        if !semio_framework_actor::runtime_metrics_due(self.last_published_ms, now_ms).await {
            return None;
        }
        self.last_published_ms = Some(now_ms);
        // 🚫️async: R10 residue shape 2 — a future is consumed by a single `.await`; awaited once
        // here instead of once inside the loop and again (moved) at `pack_encode` below.
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
mod runtime_metrics_publisher_tests {
    use super::RuntimeMetricsPublisher;
    use semio_framework_actor::{ActivationEvent, ActorKind, Envelope, Kernel, Lane, Origin, PackageId, Payload, ShardId, ShardKind, TurnResult, TurnStatus, Usage};
    use std::collections::HashMap;

    async fn env(to: semio_framework_actor::ActorId, lane: Lane, seq: u64) -> Envelope {
        Envelope { to, from: Origin::Kernel, lane, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: vec![1] } }
    }

    async fn ok_turn() -> TurnResult {
        TurnResult { ui_patches: vec![], effects: vec![], lifecycle_receipt: None, ui_patch_receipt: None, command_ingress: vec![], next_wake: None, status: TurnStatus::Idle, usage: Usage { fuel: 10, wall_us: 5, memory_bytes: 512 } }
    }

    /// 📈️ Drives a real `Kernel` (not a fake) through one turn, confirms the 2Hz gate (500ms), and
    /// decodes the published bytes back into a `RuntimeMetricsSnapshot` to prove the payload round-
    /// trips and carries the heartbeat overlay this file is responsible for.
    #[semio_framework_async_macros::async_test]
    async fn maybe_sample_gates_at_2hz_and_overlays_heartbeat_age_from_the_host() {
        let mut kernel = Kernel::new(ShardKind::Native, 1, 0, 4).await;
        let actor = kernel.activate(PackageId("s.cad".into()), 1, ActorKind::PluginApp { plugin: PackageId("s.cad".into()), app_id: "editor".into(), instance_id: 0 }, Lane::Interactive, None, ActivationEvent::Manual).await;
        kernel.submit(&env(actor, Lane::Interactive, 1).await).await;
        kernel.tick(0).await;
        kernel.complete(actor, &ok_turn().await, 0).await.unwrap();

        let mut publisher = RuntimeMetricsPublisher::new();
        let heartbeats: HashMap<ShardId, u64> = [(ShardId(0), 400u64)].into_iter().collect();

        let first = publisher.maybe_sample(&kernel, 1_000, &heartbeats).await.expect("never published yet must be due");
        let mut pos = 0usize;
        let decoded = semio_framework_actor::RuntimeMetricsSnapshot::pack_decode(&first, &mut pos).await.unwrap();
        assert_eq!(pos, first.len());
        assert_eq!(decoded.actors.len(), 1);
        assert_eq!(decoded.actors[0].metrics.turns, 1);
        let shard_row = decoded.shards.iter().find(|row| row.shard == ShardId(0)).expect("shard 0 row present");
        assert_eq!(shard_row.metrics.heartbeat_age_ms, 600, "1_000 - 400 heartbeat overlay");

        assert!(publisher.maybe_sample(&kernel, 1_200, &heartbeats).await.is_none(), "200ms since last publish is inside the 500ms window");
        assert!(publisher.maybe_sample(&kernel, 1_500, &heartbeats).await.is_some(), "exactly the 500ms interval must fire again");
    }

    //#region 🔖️ScaleFixture
    /// 🧫️ The 50×50 scale fixture registry (M2's own data source: 50 plugins × 50 extensions each,
    /// 2550 records, 7 `scaleFixture.profile` behaviour profiles) — see this module's own doc comment
    /// for why `include_str!` (not `std::fs`) is how a `#[cfg(test)]` block reads it without tripping
    /// the crate-purity grep this repo's acceptance criteria runs unconditionally over `component.rs`
    /// files (this one is the HOST, not the pure `🎭️actor` crate, but the same discipline is followed
    /// here since the file is right next to it and easy to mistake for one).
    const SCALE_FIXTURE_REGISTRY_JSON: &str = include_str!("../../../🧫️fixtures/🔌️scale/🤖️generated/🧪️registry/🔣️.json");

    #[derive(serde::Deserialize)]
    struct ScaleFixtureRegistry {
        #[serde(rename = "recordCount")]
        record_count: u32,
        records: Vec<ScaleFixtureRecord>,
    }

    #[derive(serde::Deserialize)]
    struct ScaleFixtureRecord {
        id: String,
        kind: String,
        #[serde(rename = "parentId")]
        parent_id: Option<String>,
        #[serde(rename = "scaleFixture")]
        scale_fixture: ScaleFixtureProfile,
    }

    #[derive(serde::Deserialize)]
    struct ScaleFixtureProfile {
        profile: String,
    }

    /// 🧮️ `"scale-fixture-plugin-0007"` → `7` — every record's OWN `plugin_ordinal` for `Kernel::
    /// activate` is its ancestor plugin's numeric suffix (extensions share their parent plugin's
    /// ordinal, matching `ActorId`'s bit-packed "which plugin family" semantics).
    async fn plugin_ordinal_from_id(plugin_id: &str) -> u16 {
        plugin_id.rsplit('-').next().and_then(|suffix| suffix.parse::<u16>().ok()).unwrap_or(0)
    }

    /// 🛣️ Deterministic, arbitrary profile→lane mapping — realism, not a claimed design contract (the
    /// scale fixture itself is silent on lane assignment).
    async fn lane_for_profile(profile: &str) -> Lane {
        match profile {
            "ui" | "stateful" => Lane::Interactive,
            "cpu" | "crash" => Lane::UserVisible,
            "io" | "hang" => Lane::Background,
            _ => Lane::Maintenance,
        }
    }

    /// 📈️ T1 runtime evidence at scale (`📓️terra-T1-report.md`'s acceptance criteria): activates
    /// every one of the fixture's 2550 real records through a real `Kernel` (not a fake), drives a
    /// deterministic sample through `submit`/`tick`/`complete` (including a `Faulted` turn for the
    /// `"crash"` profile), then asserts `RuntimeMetricsPublisher::maybe_sample`'s decoded snapshot
    /// reflects it — row count, package count, and the specific driven actors' turns/traps.
    #[semio_framework_async_macros::async_test]
    async fn runtime_metrics_publisher_reflects_the_2550_record_scale_fixture_registry() {
        let registry: ScaleFixtureRegistry = serde_json::from_str(SCALE_FIXTURE_REGISTRY_JSON).expect("scale fixture registry must be valid JSON matching the documented shape");
        assert_eq!(registry.record_count as usize, registry.records.len(), "recordCount header must match the actual records array length");
        assert_eq!(registry.record_count, 2550, "the documented 50 plugins x (1 + 50 extensions) fixture shape");

        let mut kernel = Kernel::new(ShardKind::Native, 8, 0, 256).await;
        let mut actor_ids = HashMap::new();
        let mut crash_profile_actor = None;

        for record in &registry.records {
            let plugin_id = record.parent_id.as_deref().unwrap_or(&record.id);
            let ordinal = plugin_ordinal_from_id(plugin_id);
            let lane = lane_for_profile(&record.scale_fixture.profile);
            let (package, kind) = if record.kind == "plugin" {
                (PackageId(record.id.clone()), ActorKind::PluginApp { plugin: PackageId(record.id.clone()), app_id: "main".into(), instance_id: 0 })
            } else {
                (PackageId(plugin_id.to_string()), ActorKind::Extension { plugin: PackageId(plugin_id.to_string()), extension_id: record.id.clone() })
            };
            let actor = kernel.activate(package, ordinal.await, kind, lane.await, None, ActivationEvent::Manual).await;
            if record.scale_fixture.profile == "crash" && crash_profile_actor.is_none() {
                crash_profile_actor = Some(actor);
            }
            actor_ids.insert(record.id.clone(), actor);
        }
        assert_eq!(actor_ids.len(), 2550, "every record activated exactly one distinct actor");

        // 🎬️ Drive a deterministic sample so the snapshot has non-zero activity to assert on: the
        // first plugin gets a clean turn, the first "crash" profile actor gets a `Faulted` turn.
        let first_record = &registry.records[0];
        let first_actor = actor_ids[&first_record.id];
        kernel.submit(&env(first_actor, lane_for_profile(&first_record.scale_fixture.profile).await, 1).await).await;
        kernel.tick(0).await;
        kernel.complete(first_actor, &ok_turn().await, 1).await.unwrap();

        let crash_actor = crash_profile_actor.expect("fixture has at least one \"crash\" profile record");
        kernel.submit(&env(crash_actor, Lane::UserVisible, 1).await).await;
        kernel.tick(1).await;
        let faulted =
            TurnResult { ui_patches: vec![], effects: vec![], lifecycle_receipt: None, ui_patch_receipt: None, command_ingress: vec![], next_wake: None, status: TurnStatus::Faulted { detail: b"scale-fixture crash profile".to_vec() }, usage: Usage { fuel: 5, wall_us: 3, memory_bytes: 256 } };
        kernel.complete(crash_actor, &faulted, 2).await.unwrap();

        let mut publisher = RuntimeMetricsPublisher::new();
        let payload = publisher.maybe_sample(&kernel, 10_000, &HashMap::new()).await.expect("first sample is always due");
        let mut pos = 0usize;
        let snapshot = semio_framework_actor::RuntimeMetricsSnapshot::pack_decode(&payload, &mut pos).await.unwrap();
        assert_eq!(pos, payload.len());

        assert_eq!(snapshot.kernel.actors, 2550);
        assert_eq!(snapshot.kernel.packages, 50, "50 plugins, each extension's package folds into its parent plugin's PackageId");
        assert_eq!(snapshot.actors.len(), 2550);

        let first_row = snapshot.actors.iter().find(|row| row.id == first_actor).expect("first record's row present");
        assert_eq!(first_row.metrics.turns, 1);
        assert_eq!(first_row.status, semio_framework_actor::ActorStatus::Active);

        let crash_row = snapshot.actors.iter().find(|row| row.id == crash_actor).expect("crash-profile record's row present");
        assert_eq!(crash_row.metrics.turns, 1);
        assert_eq!(crash_row.metrics.traps, 1, "the Faulted turn must register as a trap");

        let total_shard_actors: u32 = snapshot.shards.iter().map(|row| row.metrics.actors).sum();
        assert_eq!(total_shard_actors, 2550, "every one of the 2550 actors is counted on exactly one of the 8 shards");
    }
    //#endregion 🔖️ScaleFixture
}
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
    pub async fn apply_emit_ops(&mut self, schema: Option<&str>, ops: Vec<u8>) {
        if ops.is_empty() {
            return;
        }
        let Some(schema) = schema.filter(|s| !s.is_empty()) else {
            self.pending_binary_ops = ops;
            return;
        };
        let Ok(Some(codec)) = store::document_codec(schema).await else {
            self.pending_binary_ops = ops;
            return;
        };
        if self.pack.is_empty() && self.spr.is_empty() {
            self.pending_binary_ops = ops;
            return;
        }
        match (codec.apply_ops_binary)(&self.pack, &self.spr, &ops).await {
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
    pub document_schema: Option<String>,
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
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

/// 🔀 Decodes a packed `protocol::MergeReport` — `AppFrame::MergeReport.report`.
pub async fn decode_merge_report(bytes: &[u8]) -> Result<protocol::MergeReport, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

/// ⚔️ Decodes a packed `Vec<protocol::Conflict>` — `AppFrame::Conflicts.conflicts`.
pub async fn decode_conflicts(bytes: &[u8]) -> Result<Vec<protocol::Conflict>, PluginHostError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

//#endregion 🔖️MutationReports

//#region 🔖️IoRouter
/// 🌉️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION (D3): the host
/// half of cross-plugin artifact reuse. Each `WasmPluginRuntime`'s own in-guest `IO_REGISTRY` only
/// ever sees composers registered inside ITS OWN wasm linear memory; this router is what makes a
/// key owned by plugin B actually reachable from plugin A's `host.io-compose` import — a single
/// shared table (keyed exactly like `semio_framework::IoKey`) built by calling `list-artifact-
/// dialects` on every plugin as it loads, mapping each key to the plugin id that owns it, plus a
/// handle to that plugin's own `WasmPluginRuntime` to actually forward the call.
pub struct IoRouter {
    state: Mutex<IoRouterState>,
}

struct IoRouterState {
    routes: HashMap<semio_framework::IoKey, String>,
    runtimes: HashMap<String, Arc<PluginInstanceHandle>>,
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW mechanism's merged cross-plugin
    /// graph — `(from, into) -> IoEntryRoute` — built from every plugin's `list-io-entries` roster,
    /// separate from `routes` above (OLD `IoKey`-keyed graph). `BTreeMap` (not `HashMap`) is load-
    /// bearing: `resolve_io_route`'s determinism proof depends on canonical iteration order, mirroring
    /// `io::io_mechanism`'s own `EntryMap`.
    io_entries: BTreeMap<IoEntryKey, IoEntryRoute>,
}

/// 🌉️ One edge of the NEW mechanism's merged graph: `(from, into)`.
type IoEntryKey = (semio_framework::io_schema::ArtifactDialect, semio_framework::io_schema::ArtifactDialect);

/// 🌉️ One edge's owner + declared strength — erased from the owning plugin's `IoEntryDescriptor`,
/// plus which plugin registered it (needed by `run_io`'s reentrancy guard and `identify`'s fan-out).
#[derive(Clone, Debug, PartialEq)]
struct IoEntryRoute {
    owner: String,
    fidelity: semio_framework::io_schema::IoFidelity,
    sniffs: bool,
}

/// 🌉️ Inverse of `io_schema::Confidence::rank()` — the WIT `io-sniff` guest export returns a raw
/// `u8` rank byte, so the host reconstructs the typed `Confidence` from it before merging.
async fn rank_to_io_confidence(rank: u8) -> semio_framework::io_schema::Confidence {
    match rank {
        3 => semio_framework::io_schema::Confidence::High,
        2 => semio_framework::io_schema::Confidence::Medium,
        1 => semio_framework::io_schema::Confidence::Low,
        _ => semio_framework::io_schema::Confidence::None,
    }
}

/// 🌉️ Inverse of `io_schema::IoFidelity::rank()` — mirrors `io::io_mechanism::rank_to_fidelity`
/// (this file cannot import that private fn from `🚪️io/**`, out of this wave's boundary, so the
/// tiny 4-arm match is duplicated here rather than requesting a visibility patch for one line).
async fn rank_to_io_fidelity(rank: u8) -> semio_framework::io_schema::IoFidelity {
    match rank {
        3 => semio_framework::io_schema::IoFidelity::Exact,
        2 => semio_framework::io_schema::IoFidelity::Canonical,
        1 => semio_framework::io_schema::IoFidelity::Semantic,
        _ => semio_framework::io_schema::IoFidelity::Lossy,
    }
}

/// 🌉️ `(Reverse(min fidelity rank), hop count, joined into-coordinate string)` — the SAME ranking
/// tuple `io::io_mechanism::route_rank` uses, reimplemented here because the host's merged graph is
/// a DIFFERENT data structure (owner-aware, built from N plugins' wire rosters) than the guest-local
/// `io_mechanism`'s own `&'static IoEntry` registry; the algorithm is identical, only the storage
/// differs.
async fn io_route_rank(hops: &[semio_framework::io_schema::IoEntryDescriptor]) -> (std::cmp::Reverse<u8>, usize, String) {
    // 🚫️async: R10 residue shape 1 — `IoFidelity::rank`/`ArtifactDialect::to_coordinate` are
    // external `🚪️io` async accessors that cannot be awaited inside `Iterator::map`'s sync
    // closure, so both are hoisted into plain loops instead.
    let mut min_fidelity: Option<u8> = None;
    for hop in hops {
        let rank = hop.fidelity.rank().await;
        min_fidelity = Some(min_fidelity.map_or(rank, |current| current.min(rank)));
    }
    let mut coordinates = Vec::with_capacity(hops.len());
    for hop in hops {
        coordinates.push(hop.into.to_coordinate());
    }
    let joined = coordinates.join(",");
    (std::cmp::Reverse(min_fidelity.unwrap_or(0)), hops.len(), joined)
}

/// 🌉️ Breadth-bounded, cycle-free DFS enumeration of every simple path `from -> into` up to
/// `remaining_hops`, mirroring `io::io_mechanism::walk_routes` exactly. `graph` is a `BTreeMap`, so
/// iteration order is a pure function of the KEY SET, never of insertion/registration order — this
/// plus sorting the FULL candidate set at the end in `resolve_io_route` (never short-circuiting on
/// the first hit) is what makes the result independent of plugin load order — proven by
/// `io_router_route_is_deterministic_across_load_order` below.
async fn walk_io_routes(
    graph: &BTreeMap<IoEntryKey, IoEntryRoute>,
    current: &semio_framework::io_schema::ArtifactDialect,
    into: &semio_framework::io_schema::ArtifactDialect,
    remaining_hops: u8,
    path: &mut Vec<semio_framework::io_schema::IoEntryDescriptor>,
    visited: &mut BTreeSet<semio_framework::io_schema::ArtifactDialect>,
    candidates: &mut Vec<Vec<semio_framework::io_schema::IoEntryDescriptor>>,
) {
    if remaining_hops == 0 {
        return;
    }
    for ((from, hop_into), route) in graph.iter() {
        if from != current || visited.contains(hop_into) {
            continue;
        }
        let descriptor = semio_framework::io_schema::IoEntryDescriptor { from: from.clone(), into: hop_into.clone(), fidelity: route.fidelity, sniffs: route.sniffs };
        path.push(descriptor);
        if hop_into == into {
            candidates.push(path.clone());
        } else {
            visited.insert(hop_into.clone());
            Box::pin(walk_io_routes(graph, hop_into, into, remaining_hops - 1, path, visited, candidates)).await;
            visited.remove(hop_into);
        }
        path.pop();
    }
}

/// 🌉️ `resolve_route`'s host-side twin (`io::io_mechanism::resolve_route`) over the merged
/// multi-plugin graph instead of one plugin's own local registry. Pure — no lock, no wasm call —
/// so it is directly unit-testable with a synthetic graph (`io_router_route_is_deterministic_
/// across_load_order`, `io_router_route_prefers_higher_minimum_fidelity`, below).
async fn resolve_io_route(
    graph: &BTreeMap<IoEntryKey, IoEntryRoute>,
    from: &semio_framework::io_schema::ArtifactDialect,
    into: &semio_framework::io_schema::ArtifactDialect,
    max_hops: u8,
) -> Result<semio_framework::io_schema::IoRoute, PluginHostError> {
    let max_hops = max_hops.min(3);
    if max_hops == 0 {
        return Err(PluginHostError::Plugin(format!("io_routes {} -> {}: max_hops clamped to 0", from.to_coordinate(), into.to_coordinate())));
    }
    let mut candidates: Vec<Vec<semio_framework::io_schema::IoEntryDescriptor>> = Vec::new();
    let mut path: Vec<semio_framework::io_schema::IoEntryDescriptor> = Vec::new();
    let mut visited: BTreeSet<semio_framework::io_schema::ArtifactDialect> = BTreeSet::new();
    visited.insert(from.clone());
    walk_io_routes(graph, from, into, max_hops, &mut path, &mut visited, &mut candidates).await;
    if candidates.is_empty() {
        return Err(PluginHostError::Plugin(format!("no io route from {} to {} within {max_hops} hops", from.to_coordinate(), into.to_coordinate())));
    }
    // 🚫️async: R10 residue shape 1 — `io_route_rank` is async, so ranks are precomputed before
    // the sync `sort_by` comparator rather than called from inside it.
    let mut ranked: Vec<(_, Vec<semio_framework::io_schema::IoEntryDescriptor>)> = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let rank = io_route_rank(&candidate).await;
        ranked.push((rank, candidate));
    }
    ranked.sort_by(|a, b| a.0.cmp(&b.0));
    let best = ranked.into_iter().next().expect("candidates checked non-empty above").1;
    let mut min_rank: Option<u8> = None;
    for hop in &best {
        let rank = hop.fidelity.rank().await;
        min_rank = Some(min_rank.map_or(rank, |current| current.min(rank)));
    }
    let fidelity = rank_to_io_fidelity(min_rank.expect("a route has at least one hop")).await;
    Ok(semio_framework::io_schema::IoRoute { hops: best, fidelity })
}

/// 🌉️ Pure preflight check behind `IoRouter::register_plugin`'s io-entries half: does merging
/// `plugin_id`'s `incoming` roster (its `list-io-entries` wire bytes, decoded) into `existing`
/// claim a `(from, into)` key a DIFFERENT plugin already owns? `None` means the merge is safe
/// (either a brand-new key, or `plugin_id` re-claiming its OWN key — idempotent). Extracted as its
/// own function so the conflict rule is unit-testable without a real `Arc<WasmPluginRuntime>`.
async fn io_entries_conflict(existing: &BTreeMap<IoEntryKey, IoEntryRoute>, plugin_id: &str, incoming: &[semio_framework::io_schema::IoEntryDescriptor]) -> Option<PluginHostError> {
    for descriptor in incoming {
        let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
        if let Some(current) = existing.get(&key) {
            if current.owner != plugin_id {
                return Some(PluginHostError::IoEntryRouteConflict { from: key.0, into: key.1, existing_plugin: current.owner.clone(), incoming_plugin: plugin_id.to_string() });
            }
        }
    }
    None
}

/// 🌉️ Pure predicate behind `IoRouter::run_io`'s reentrancy guard: does ANY hop of `route`, per
/// `graph`, belong to `calling_plugin_id`? Returns the FIRST such hop's `(from, into)` for the
/// error message, or `None` if the whole route is safe to execute. Extracted as its own function so
/// the guard is unit-testable without a real `Arc<WasmPluginRuntime>` (`run_io` needs one for every
/// OTHER hop it actually executes; this predicate needs none).
async fn route_reenters_calling_plugin<'route>(
    graph: &BTreeMap<IoEntryKey, IoEntryRoute>,
    route: &'route semio_framework::io_schema::IoRoute,
    calling_plugin_id: &str,
) -> Option<(&'route semio_framework::io_schema::ArtifactDialect, &'route semio_framework::io_schema::ArtifactDialect)> {
    route.hops.iter().find_map(|hop| {
        let owner = &graph.get(&(hop.from.clone(), hop.into.clone()))?.owner;
        (owner == calling_plugin_id).then_some((&hop.from, &hop.into))
    })
}

impl IoRouter {
    pub fn new() -> Self {
        Self { state: Mutex::new(IoRouterState { routes: HashMap::new(), runtimes: HashMap::new(), io_entries: BTreeMap::new() }) }
    }

    /// 📌️ Registers one already-instantiated plugin's `PluginInstanceHandle` + merges its composer
    /// roster into the shared route table, AND (CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-D) its
    /// NEW `list-io-entries` roster into `state.io_entries`. Call once per plugin, after its
    /// `describe()` `PackageDescriptor` (packet E1) has been decoded — `world actor` has no runtime
    /// `list-artifact-dialects`/`list-io-entries` guest export anymore (`📜️.wit`'s own
    /// `describe` interface doc: "everything a plugin ... used to expose as separate `list-*` WIT
    /// exports ... is now static data inside this one descriptor instead"), so both rosters are the
    /// CALLER's job to decode and pass in — this function makes no wasm call at all. Both graphs
    /// preflight BEFORE either commits — a conflict in either leaves BOTH untouched, matching this
    /// file's existing all-or-nothing registration shape.
    pub async fn register_plugin(
        &self,
        plugin_id: &str,
        handle: Arc<PluginInstanceHandle>,
        artifact_dialect_entries: &[(semio_framework::ArtifactDialect, Vec<semio_framework::ArtifactDialect>)],
        io_entries: &[semio_framework::io_schema::IoEntryDescriptor],
    ) -> Result<(), PluginHostError> {
        let mut candidate_routes = Vec::new();
        for (writes, reads) in artifact_dialect_entries {
            for read in reads {
                candidate_routes.push(semio_framework::IoKey {
                    artifact_kind: writes.artifact_kind.clone(),
                    standard: writes.standard.clone(),
                    subset: writes.subset.clone(),
                    direction: semio_framework::IoDirection::Import,
                    format_kind: read.artifact_kind.clone(),
                    format_standard: read.standard.clone(),
                    format_subset: read.subset.clone(),
                });
                candidate_routes.push(semio_framework::IoKey {
                    artifact_kind: read.artifact_kind.clone(),
                    standard: read.standard.clone(),
                    subset: read.subset.clone(),
                    direction: semio_framework::IoDirection::Export,
                    format_kind: writes.artifact_kind.clone(),
                    format_standard: writes.standard.clone(),
                    format_subset: writes.subset.clone(),
                });
            }
        }
        let mut state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        if let Some(existing) = state.runtimes.get(plugin_id) {
            if !Arc::ptr_eq(existing, &handle) {
                return Err(PluginHostError::PluginRuntimeConflict { plugin_id: plugin_id.to_owned() });
            }
        }
        for key in &candidate_routes {
            if let Some(existing_plugin) = state.routes.get(key) {
                if existing_plugin != plugin_id {
                    return Err(PluginHostError::IoRouteConflict { key: key.clone(), existing_plugin: existing_plugin.clone(), incoming_plugin: plugin_id.to_owned() });
                }
            }
        }
        if let Some(conflict) = io_entries_conflict(&state.io_entries, plugin_id, io_entries).await {
            return Err(conflict);
        }
        state.runtimes.entry(plugin_id.to_owned()).or_insert(handle);
        for key in candidate_routes {
            state.routes.entry(key).or_insert_with(|| plugin_id.to_owned());
        }
        for descriptor in io_entries {
            let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
            state.io_entries.entry(key).or_insert(IoEntryRoute { owner: plugin_id.to_owned(), fidelity: descriptor.fidelity, sniffs: descriptor.sniffs });
        }
        Ok(())
    }

    /// 📊️ `N plugins / M keys` — logged at boot so a dev-boot smoke test can confirm the router
    /// actually picked up more than zero cross-plugin routes.
    pub async fn stats(&self) -> Result<(usize, usize), PluginHostError> {
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        Ok((state.runtimes.len(), state.routes.len()))
    }

    /// 🌉️ Resolves `key` (JSON wire bytes) to whichever OTHER plugin owns it. Refuses to route back
    /// into `calling_plugin_id` itself: the target plugin's `artifact-compose` guest handler is
    /// local-only by construction (see `io::wire_artifact_compose`'s own doc comment) and never calls
    /// `io-compose` again, so every route is exactly one hop — the self-route guard is what keeps a
    /// plugin from ever needing to reason about calling back into its own in-flight `Store` mutex
    /// (which would deadlock, since that mutex is already held by the caller of this very host call).
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): resolution (unchanged) now feeds a
    /// REAL dispatch — `PluginInstanceHandle::compose` driving the owner's own `semio.compose` cold
    /// job through `run_job_on_worker`, the same post-turn relay `run_io`/`identify`
    /// below already use. The guest body is the live `compose-await` packet's, not this file's —
    /// until it registers `"semio.compose"`, this surfaces the OWNER's own `job.unknown-kind` fault
    /// (a real, dynamic failure reflecting actual guest state) rather than the previous hand-written
    /// permanent host refusal. Once `compose-await` lands, this exact code starts succeeding with no
    /// further host change.
    pub async fn compose(&self, calling_plugin_id: &str, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let key_text = std::str::from_utf8(key_bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let key: semio_framework::IoKey = dsl::os_pack::json::from_json_str(key_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        let owner = state
            .routes
            .get(&key)
            .cloned()
            .ok_or_else(|| PluginHostError::Plugin(format!("no plugin registered for {}/{}/{} {:?} {}/{}/{}", key.artifact_kind, key.standard, key.subset, key.direction, key.format_kind, key.format_standard, key.format_subset)))?;
        if owner == calling_plugin_id {
            return Err(PluginHostError::Plugin(format!("io-compose refused: plugin `{calling_plugin_id}` would be routing to itself (should have resolved locally)")));
        }
        let handle = state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns this key but its instance handle is not registered with the router")))?;
        drop(state);
        handle.compose(key_bytes, sources_bytes).await
    }

    /// 📚️ Every dialect ANY loaded plugin can move `artifact_kind` through in `direction`
    /// ("import"|"export"), JSON `Vec<ArtifactDialect>` bytes.
    pub async fn dialects(&self, artifact_kind: &str, direction: &str) -> Result<Vec<u8>, PluginHostError> {
        let direction = match direction {
            "import" => semio_framework::IoDirection::Import,
            "export" => semio_framework::IoDirection::Export,
            other => return Err(PluginHostError::Plugin(format!("unknown io direction `{other}` (expected \"import\" or \"export\")"))),
        };
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        let dialects: Vec<semio_framework::ArtifactDialect> = state
            .routes
            .keys()
            .filter(|key| key.artifact_kind == artifact_kind && key.direction == direction)
            .map(|key| semio_framework::ArtifactDialect { artifact_kind: key.format_kind.clone(), standard: key.format_standard.clone(), subset: key.format_subset.clone() })
            .collect();
        Ok(dsl::os_pack::json::to_json_string(&dialects).into_bytes())
    }

    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): resolves the deterministic, ≤3-hop,
    /// cycle-free route `from -> into` over the merged `io_entries` graph — the WIT `io-routes`
    /// host import. JSON `io_schema::IoRoute` bytes.
    pub async fn io_routes(&self, from: &str, into: &str) -> Result<Vec<u8>, PluginHostError> {
        let from = semio_framework::io_schema::ArtifactDialect::parse_coordinate(from).map_err(PluginHostError::Plugin)?;
        let into = semio_framework::io_schema::ArtifactDialect::parse_coordinate(into).map_err(PluginHostError::Plugin)?;
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        let route = resolve_io_route(&state.io_entries, &from, &into, 3).await?;
        drop(state);
        Ok(dsl::os_pack::json::to_json_string(&route).into_bytes())
    }

    /// 🌉️ Executes the WHOLE resolved `from -> into` route — the WIT `io-run` host import. Resolves
    /// the route and every hop's owning runtime FIRST (holding the router's own lock only for that
    /// lookup, never across a guest call), THEN, before running anything, refuses the ENTIRE route
    /// if ANY hop is owned by `calling_plugin_id` itself: executing that hop would call back into
    /// the calling plugin's own `Store` mutex while it is still held by the in-flight outer call
    /// that reached this host import in the first place — a guaranteed deadlock on a non-reentrant
    /// `std::sync::Mutex`. This generalizes `compose`'s one-hop self-route refusal (above) to a
    /// resolved route of up to 3 hops; the guard is an up-front scan, not a per-hop check, so a
    /// route is either run in full or not run at all — no partial execution on a refusal.
    pub async fn run_io(&self, calling_plugin_id: &str, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let from_dialect = semio_framework::io_schema::ArtifactDialect::parse_coordinate(from).map_err(PluginHostError::Plugin)?;
        let into_dialect = semio_framework::io_schema::ArtifactDialect::parse_coordinate(into).map_err(PluginHostError::Plugin)?;
        let hops = {
            let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
            let route = resolve_io_route(&state.io_entries, &from_dialect, &into_dialect, 3).await?;
            if let Some(reentrant_hop) = route_reenters_calling_plugin(&state.io_entries, &route, calling_plugin_id).await {
                return Err(PluginHostError::Plugin(format!(
                    "io-run refused: hop {} -> {} is owned by the calling plugin `{calling_plugin_id}` itself — executing it would re-enter that plugin's own in-flight, non-reentrant store lock",
                    reentrant_hop.0.to_coordinate(),
                    reentrant_hop.1.to_coordinate()
                )));
            }
            let mut hops = Vec::with_capacity(route.hops.len());
            for hop in &route.hops {
                let key: IoEntryKey = (hop.from.clone(), hop.into.clone());
                // 🚫️async: R10 residue shape 1 — `to_coordinate` is external/async, hoisted out of
                // the `ok_or_else` sync closures below.
                let from_coord = hop.from.to_coordinate();
                let into_coord = hop.into.to_coordinate();
                let owner = state.io_entries.get(&key).map(|entry| entry.owner.clone()).ok_or_else(|| PluginHostError::Plugin(format!("io-run: hop {from_coord} -> {into_coord} vanished from the router between resolve and execute")))?;
                let runtime = state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns hop {from_coord} -> {into_coord} but its runtime is not registered with the router")))?;
                hops.push((from_coord, into_coord, runtime));
            }
            hops
        };
        let mut current = payload;
        for (from_coordinate, into_coordinate, runtime) in hops {
            current = runtime.io_run(&from_coordinate, &into_coordinate, current).await?;
        }
        Ok(current)
    }

    /// 🔍️ Fans `io-sniff` out across every OTHER loaded plugin's carrier-`from` entries — the WIT
    /// `io-identify` host import. Skips the calling plugin's own carrier entries for the SAME
    /// reentrancy reason `run_io` refuses a self-owned hop (a fan-out is best-effort across
    /// multiple plugins, so this SKIPS rather than refuses the whole call). JSON `Vec<(ArtifactDialect,
    /// Confidence)>` bytes, sorted confidence descending then coordinate ascending — same shape and
    /// order `io::io_mechanism::io_identify` produces for the guest-local case.
    pub async fn identify(&self, calling_plugin_id: &str, payload_bytes: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let payload_text = std::str::from_utf8(&payload_bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let payload: semio_framework::io_schema::IoPayload = dsl::os_pack::json::from_json_str(payload_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let carrier = semio_framework::io_schema::ArtifactDialect::from(match &payload {
            semio_framework::io_schema::IoPayload::Binary(_) => semio_framework::io_schema::CARRIER_BINARY,
            semio_framework::io_schema::IoPayload::Text(_) => semio_framework::io_schema::CARRIER_TEXT,
        });
        let candidates: Vec<(semio_framework::io_schema::ArtifactDialect, String)> = {
            let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
            state.io_entries.iter().filter(|((from, _into), route)| *from == carrier && route.sniffs && route.owner != calling_plugin_id).map(|((_from, into), route)| (into.clone(), route.owner.clone())).collect()
        };
        let mut found: Vec<(semio_framework::io_schema::ArtifactDialect, semio_framework::io_schema::Confidence)> = Vec::new();
        for (into, owner) in candidates {
            let runtime = {
                let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
                state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns a carrier io entry but its runtime is not registered with the router")))?
            };
            let carrier_coord = carrier.to_coordinate();
            let into_coord = into.to_coordinate();
            let rank = runtime.io_sniff(&carrier_coord, &into_coord, &payload_bytes).await?;
            let confidence = rank_to_io_confidence(rank).await;
            if confidence != semio_framework::io_schema::Confidence::None {
                found.push((into, confidence));
            }
        }
        // 🚫️async: R10 residue shape 1 — `Confidence::rank`/`ArtifactDialect::to_coordinate` are
        // external async accessors, so the sort key is precomputed before the sync `sort_by`.
        let mut decorated = Vec::with_capacity(found.len());
        for (dialect, confidence) in found {
            let rank = confidence.rank().await;
            let coord = dialect.to_coordinate();
            decorated.push((rank, coord, dialect, confidence));
        }
        decorated.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        let found: Vec<(semio_framework::io_schema::ArtifactDialect, semio_framework::io_schema::Confidence)> = decorated.into_iter().map(|(_, _, dialect, confidence)| (dialect, confidence)).collect();
        Ok(dsl::os_pack::json::to_json_string(&found).into_bytes())
    }

    /// ✂️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): drops
    /// `plugin_id`'s runtime handle and every route it owns — required before a hot-reloaded
    /// plugin can re-register a FRESH `Arc<WasmPluginRuntime>` under the same id (`register_plugin`'s
    /// `PluginRuntimeConflict` check would otherwise reject the new `Arc` as a different pointer for
    /// an already-registered plugin id) and before an unload actually drops the runtime. Also drops
    /// every NEW-mechanism `io_entries` row `plugin_id` owns (W1-D).
    pub async fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        let mut state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        state.runtimes.remove(plugin_id);
        state.routes.retain(|_, owner| owner != plugin_id);
        state.io_entries.retain(|_, route| route.owner != plugin_id);
        Ok(())
    }
}

impl Default for IoRouter {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️IoRouter

//#region 💡️InferenceRouter
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GuestArtifactInferenceMetadata {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub document_schema: String,
    pub document_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    /// 🎯️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS: `Some(plugin_id)` when
    /// this row is a CONTRIBUTED inference (contract §3/§4); absent/`None`, or equal to `owner`,
    /// for an ordinary owner-authored inference. Additive+defaulted so today's guest wire (which
    /// never sends this field) still decodes unchanged.
    #[serde(default)]
    #[value(default)]
    pub contributor: Option<String>,
    /// 🕸️ Other `inference_schema` ids (same `artifact_kind`) this inference's own computation
    /// consumes the RESULT of — the per-artifact `depends_on` DAG `ArtifactInferenceRouter` toposorts
    /// at registration and injects into `artifact-inference-request.dependencies` at call time.
    #[serde(default)]
    #[value(default)]
    pub depends_on: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceRouteRequest {
    wire_version: u32,
    owner: String,
    artifact_kind: String,
    artifact_schema: String,
    artifact_schema_version: u32,
    document_schema: String,
    document_schema_version: u32,
    inference_schema: String,
    inference_schema_version: u32,
    algorithm_version: u32,
    policy_version: u32,
    revision: u64,
    generation: u64,
    source_dialect: String,
    policy: Vec<u8>,
    budgets: InferenceRouteBudget,
    cancellation_id: String,
    previous_state: Option<Vec<u8>>,
    requested_cache_mode: InferenceRouteCacheMode,
    canonical_payload: Vec<u8>,
    dependencies: Vec<(String, Vec<u8>)>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceRouteBudget {
    allocation_bytes: u64,
    work_units: u64,
    recursion_depth: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
enum InferenceRouteCacheMode {
    Cold,
    Incremental,
    Bypass,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceRouteResult {
    wire_version: u32,
    owner: String,
    artifact_kind: String,
    artifact_schema: String,
    artifact_schema_version: u32,
    document_schema: String,
    document_schema_version: u32,
    inference_schema: String,
    inference_schema_version: u32,
    algorithm_version: u32,
    policy_version: u32,
    revision: u64,
    generation: u64,
    source_dialect: String,
    policy: Vec<u8>,
    budgets: InferenceRouteBudget,
    cancellation_id: String,
    previous_state: Option<Vec<u8>>,
    requested_cache_mode: InferenceRouteCacheMode,
    canonical_payload: Vec<u8>,
    dependencies: Vec<(String, Vec<u8>)>,
    complete: bool,
    actual_cache_mode: InferenceRouteCacheMode,
}

pub struct ArtifactInferenceRouter {
    routes: Mutex<BTreeMap<(String, String), (String, GuestArtifactInferenceMetadata)>>,
    runtimes: Mutex<HashMap<String, Arc<PluginInstanceHandle>>>,
    live_commits: Mutex<BTreeMap<String, (u64, u64)>>,
}

impl ArtifactInferenceRouter {
    pub fn new() -> Self {
        Self { routes: Mutex::new(BTreeMap::new()), runtimes: Mutex::new(HashMap::new()), live_commits: Mutex::new(BTreeMap::new()) }
    }

    /// 📌️ `dependencies` is the reporting plugin's OWN declared `PluginManifest.dependencies` —
    /// required to gate a CONTRIBUTED row (contract §4 rule 1, same rule `ArtifactMutationRouter`
    /// applies): its `contributor` must equal `plugin_id`, and `owner` must be a direct entry of
    /// `dependencies`. Also toposorts every registered row's `depends_on` DAG (existing rows plus
    /// this registration's own) and rejects a cycle before committing anything. `roster_wire_bytes`
    /// is JSON `Vec<GuestArtifactInferenceMetadata>` — mirrors `ArtifactMutationRouter::register_
    /// plugin`'s already-decoupled idiom (`roster_wire_bytes: &[u8]`, no runtime call inside this
    /// function) rather than `list_artifact_inferences()`, `contributor.list-artifact-inferences`
    /// not being a `world actor` export any more than the old `plugin` interface's own list-* exports
    /// were (`describe.wit`'s doc comment) — the caller decodes `describe()`'s `PackageDescriptor`
    /// and passes the roster in; `handle` is kept only for later `infer()` dispatch.
    pub async fn register_plugin(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], handle: Arc<PluginInstanceHandle>, roster_wire_bytes: &[u8]) -> Result<(), PluginHostError> {
        let roster_text = std::str::from_utf8(roster_wire_bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let metadata: Vec<GuestArtifactInferenceMetadata> = dsl::os_pack::json::from_json_str(roster_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        for item in &metadata {
            if let Some(contributor) = &item.contributor {
                if contributor != &item.owner {
                    if contributor != plugin_id {
                        return Err(PluginHostError::Plugin(format!("inference metadata for {}/{} claims contributor `{contributor}` but was reported by plugin `{plugin_id}`", item.artifact_kind, item.inference_schema)));
                    }
                    if !dependencies.iter().any(|dependency| dependency.plugin_id == item.owner) {
                        return Err(PluginHostError::Plugin(format!("plugin `{plugin_id}` contributes an inference on `{}` (owner `{}`) without declaring `{}` as a dependency (contract §4 rule 1)", item.artifact_kind, item.owner, item.owner)));
                    }
                }
            }
        }

        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?;
        let mut candidate = routes.clone();
        for item in &metadata {
            let key = (item.artifact_kind.clone(), item.inference_schema.clone());
            if let Some((existing_plugin, existing)) = candidate.get(&key) {
                if existing_plugin == plugin_id && existing == item {
                    continue;
                }
                return Err(PluginHostError::Plugin(format!("conflicting artifact inference owner for {}/{}: {} {:?}, incoming {} {:?}", key.0, key.1, existing_plugin, existing, plugin_id, item)));
            }
            candidate.insert(key, (plugin_id.to_string(), item.clone()));
        }
        validate_inference_dependency_graph(&candidate).await?;
        *routes = candidate;
        drop(routes);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.insert(plugin_id.to_string(), handle);
        Ok(())
    }

    pub async fn metadata(&self) -> Result<Vec<GuestArtifactInferenceMetadata>, PluginHostError> {
        Ok(self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?.values().map(|(_, item)| item.clone()).collect())
    }

    pub async fn infer(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let request_text = std::str::from_utf8(request).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let identity: InferenceRouteRequest = dsl::os_pack::json::from_json_str(request_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        {
            let mut live = self.live_commits.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference live commits"))?;
            if live.insert(identity.cancellation_id.clone(), (identity.revision, identity.generation)).is_some() {
                return Err(PluginHostError::Plugin(format!("inference cancellation identity {:?} is already active", identity.cancellation_id)));
            }
        }
        let result = self.infer_with_visited(request, &mut Vec::new()).await;
        let live = self
            .live_commits
            .lock()
            .map_err(|_| PluginHostError::LockPoisoned("artifact inference live commits"))?
            .remove(&identity.cancellation_id)
            .ok_or_else(|| PluginHostError::Plugin("inference live commit authority disappeared before admission".into()))?;
        let result = result?;
        let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(identity.revision), semio_framework_job::Generation(identity.generation), 0);
        match semio_framework_job::validate_commit(&operation, semio_framework_job::RevisionId(live.0), semio_framework_job::Generation(live.1)) {
            semio_framework_job::CommitValidation::Accepted => Ok(result),
            semio_framework_job::CommitValidation::Stale { .. } => {
                Err(PluginHostError::Plugin(format!("inference commit for {:?} is stale: based on revision/generation {}/{}, live authority is {}/{}", identity.cancellation_id, identity.revision, identity.generation, live.0, live.1)))
            }
        }
    }

    /// 🪪️ Model-actor freshness handoff for an active inference. The final route admission reads this
    /// value only after the guest's terminal candidate returns, then calls
    /// `semio_framework_job::validate_commit` immediately before exposing the result.
    pub async fn set_live_revision_generation(&self, cancellation_id: &str, revision: u64, generation: u64) -> Result<(), PluginHostError> {
        let mut live = self.live_commits.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference live commits"))?;
        let authority = live.get_mut(cancellation_id).ok_or_else(|| PluginHostError::Plugin(format!("no active inference has cancellation identity {cancellation_id:?}")))?;
        *authority = (revision, generation);
        Ok(())
    }

    /// 🕸️ Contract §6: before dispatching to the owner/contributor's own `artifact-infer`, resolves
    /// this route's `depends_on` (same `artifact_kind`, per `GuestArtifactInferenceMetadata`'s own
    /// doc) by recursively computing each dependency's OWN result first, injecting the pairs into
    /// `dependencies: Vec<(inference_schema, result_wire_bytes)>` on the outgoing request — exactly
    /// the WIT `artifact-inference-request.dependencies: list<tuple<string, list<u8>>>` shape.
    /// `visited` is the runtime counterpart of `register_plugin`'s registration-time toposort — a
    /// registered graph can still recurse infinitely if two rows' `depends_on` disagree with what
    /// was toposorted (e.g. a hot-reloaded plugin), so this is real defense-in-depth, not
    /// redundant.
    async fn infer_with_visited(&self, request: &[u8], visited: &mut Vec<String>) -> Result<Vec<u8>, PluginHostError> {
        let request_text = std::str::from_utf8(request).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let mut route: InferenceRouteRequest = dsl::os_pack::json::from_json_str(request_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        if visited.contains(&route.inference_schema) {
            return Err(PluginHostError::Plugin(format!("inference dependency cycle at call time: {} -> {}", visited.join(" -> "), route.inference_schema)));
        }
        let (owner, own_metadata) = {
            let routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?;
            routes
                .get(&(route.artifact_kind.clone(), route.inference_schema.clone()))
                .map(|(plugin_id, metadata)| (plugin_id.clone(), metadata.clone()))
                .ok_or_else(|| PluginHostError::Plugin(format!("no guest inference route for {}/{}", route.artifact_kind, route.inference_schema)))?
        };

        if !own_metadata.depends_on.is_empty() {
            visited.push(route.inference_schema.clone());
            let mut dependencies = Vec::with_capacity(own_metadata.depends_on.len());
            for dependency_schema in &own_metadata.depends_on {
                let dependency_metadata = {
                    let routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?;
                    routes
                        .get(&(route.artifact_kind.clone(), dependency_schema.clone()))
                        .map(|(_, metadata)| metadata.clone())
                        .ok_or_else(|| PluginHostError::Plugin(format!("inference `{}` declares depends_on `{dependency_schema}` which is not registered for artifact kind `{}`", route.inference_schema, route.artifact_kind)))?
                };
                let dependency_request = build_dependency_inference_request(&route, &dependency_metadata).await;
                let dependency_request_bytes = dsl::os_pack::json::to_json_string(&dependency_request).into_bytes();
                // 🚫️async: R10 residue shape 3 — genuinely self-recursive (this fn really does
                // await real plugin-runtime I/O via `handle.infer` below, so R9 does not apply);
                // `Box::pin` breaks the otherwise-infinite future size, per rustc's own E0733 hint.
                let dependency_result_bytes = Box::pin(self.infer_with_visited(&dependency_request_bytes, visited)).await?;
                dependencies.push((dependency_schema.clone(), dependency_result_bytes));
            }
            visited.pop();
            route.dependencies = dependencies;
        }

        let request = dsl::os_pack::json::to_json_string(&route).into_bytes();
        let handle = self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("inference owner `{owner}` is not loaded")))?;
        let result = handle.infer(&request).await?;
        let result_text = std::str::from_utf8(&result).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let echoed: InferenceRouteResult = dsl::os_pack::json::from_json_str(result_text).map_err(|error| PluginHostError::Json(error.to_string()))?;
        validate_inference_echo(&route, &echoed).await?;
        Ok(result)
    }

    /// ✂️ Drops every route reported by `plugin_id` — called on unload/hot-reload.
    pub async fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?.retain(|_, (owner, _)| owner != plugin_id);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.remove(plugin_id);
        Ok(())
    }
}

/// 🕸️ DFS cycle detector over the `depends_on` DAG among every row of `routes`, scoped by
/// `inference_schema` id (`depends_on` entries name a sibling `inference_schema` within the SAME
/// `artifact_kind`, per `GuestArtifactInferenceMetadata`'s own field doc). Genuinely new logic —
/// distinct from W0-C's plugin-manifest toposort, a different domain (inference schemas within one
/// artifact kind, not plugins).
async fn validate_inference_dependency_graph(routes: &BTreeMap<(String, String), (String, GuestArtifactInferenceMetadata)>) -> Result<(), PluginHostError> {
    let mut adjacency: BTreeMap<(&str, &str), Vec<&str>> = BTreeMap::new();
    for ((artifact_kind, inference_schema), (_, item)) in routes {
        adjacency.entry((artifact_kind.as_str(), inference_schema.as_str())).or_default().extend(item.depends_on.iter().map(|dep| dep.as_str()));
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Mark {
        Visiting,
        Done,
    }

    // 🚫️async: R10 residue shape 3 — self-recursive, and per E1's own principle: pure in-memory
    // BTreeMap/Vec recursion with no suspension point anywhere in the body, so async here would
    // ALSO need `Box::pin` at the recursive call just to compile, for zero behavioural benefit.
    // Reverted to sync rather than boxed — R9.
    fn visit<'a>(artifact_kind: &'a str, node: &'a str, adjacency: &BTreeMap<(&'a str, &'a str), Vec<&'a str>>, marks: &mut BTreeMap<(&'a str, &'a str), Mark>, stack: &mut Vec<&'a str>) -> Result<(), PluginHostError> {
        match marks.get(&(artifact_kind, node)) {
            Some(Mark::Done) => return Ok(()),
            Some(Mark::Visiting) => return Err(PluginHostError::Plugin(format!("inference dependency cycle on `{artifact_kind}`: {} -> {node}", stack.join(" -> ")))),
            None => {}
        }
        marks.insert((artifact_kind, node), Mark::Visiting);
        stack.push(node);
        if let Some(deps) = adjacency.get(&(artifact_kind, node)) {
            for dep in deps {
                visit(artifact_kind, dep, adjacency, marks, stack)?;
            }
        }
        stack.pop();
        marks.insert((artifact_kind, node), Mark::Done);
        Ok(())
    }

    let mut marks = BTreeMap::new();
    let mut stack = Vec::new();
    for (artifact_kind, node) in adjacency.keys() {
        visit(artifact_kind, node, &adjacency, &mut marks, &mut stack)?;
    }
    Ok(())
}

/// 🎞️ Builds the outgoing request for one `depends_on` entry: identity fields (`owner`/`artifact_schema*`/
/// `document_schema*`/`inference_schema*`/`algorithm_version`/`policy_version`) come from the
/// DEPENDENCY's own registered metadata; caller-context fields (`revision`/`generation`/
/// `source_dialect`/`policy`/`budgets`/`cancellation_id`/`requested_cache_mode`/`canonical_payload`)
/// are inherited from the parent request — the same underlying artifact source, viewed through a
/// different inference facet. `previous_state` always starts fresh (`None`): each dependency
/// resolves its own current result, not a cached delta.
async fn build_dependency_inference_request(base: &InferenceRouteRequest, dependency: &GuestArtifactInferenceMetadata) -> InferenceRouteRequest {
    InferenceRouteRequest {
        wire_version: base.wire_version,
        owner: dependency.owner.clone(),
        artifact_kind: base.artifact_kind.clone(),
        artifact_schema: dependency.artifact_schema.clone(),
        artifact_schema_version: dependency.artifact_schema_version,
        document_schema: dependency.document_schema.clone(),
        document_schema_version: dependency.document_schema_version,
        inference_schema: dependency.inference_schema.clone(),
        inference_schema_version: dependency.inference_schema_version,
        algorithm_version: dependency.algorithm_version,
        policy_version: dependency.policy_version,
        revision: base.revision,
        generation: base.generation,
        source_dialect: base.source_dialect.clone(),
        policy: base.policy.clone(),
        budgets: base.budgets.clone(),
        cancellation_id: base.cancellation_id.clone(),
        previous_state: None,
        requested_cache_mode: base.requested_cache_mode.clone(),
        canonical_payload: base.canonical_payload.clone(),
        dependencies: Vec::new(),
    }
}

async fn validate_inference_echo(request: &InferenceRouteRequest, result: &InferenceRouteResult) -> Result<(), PluginHostError> {
    if result.wire_version != request.wire_version
        || result.owner != request.owner
        || result.artifact_kind != request.artifact_kind
        || result.artifact_schema != request.artifact_schema
        || result.artifact_schema_version != request.artifact_schema_version
        || result.document_schema != request.document_schema
        || result.document_schema_version != request.document_schema_version
        || result.inference_schema != request.inference_schema
        || result.inference_schema_version != request.inference_schema_version
        || result.algorithm_version != request.algorithm_version
        || result.policy_version != request.policy_version
        || result.revision != request.revision
        || result.generation != request.generation
        || result.source_dialect != request.source_dialect
        || result.policy != request.policy
        || result.budgets != request.budgets
        || result.cancellation_id != request.cancellation_id
        || result.previous_state != request.previous_state
        || result.requested_cache_mode != request.requested_cache_mode
        || result.dependencies != request.dependencies
        || result.actual_cache_mode != request.requested_cache_mode
    {
        return Err(PluginHostError::Plugin("guest inference result did not exactly echo its request metadata".into()));
    }
    Ok(())
}

impl Default for ArtifactInferenceRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod artifact_inference_router_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn only_exactly_echoed_guest_results_are_publishable() {
        let request = InferenceRouteRequest {
            wire_version: 2,
            owner: "s.test".into(),
            artifact_kind: "s.test".into(),
            artifact_schema: "s.test".into(),
            artifact_schema_version: 1,
            document_schema: "s.test.document".into(),
            document_schema_version: 1,
            inference_schema: "s.test.inference".into(),
            inference_schema_version: 1,
            algorithm_version: 1,
            policy_version: 1,
            revision: 7,
            generation: 9,
            source_dialect: "s.test.standard.v1.dialect.canonical".into(),
            policy: vec![1],
            budgets: InferenceRouteBudget { allocation_bytes: 128, work_units: 1, recursion_depth: 1 },
            cancellation_id: "cancel-1".into(),
            previous_state: None,
            requested_cache_mode: InferenceRouteCacheMode::Cold,
            canonical_payload: vec![1],
            dependencies: vec![("s.dependency".into(), vec![2])],
        };
        let valid = InferenceRouteResult {
            wire_version: request.wire_version,
            owner: request.owner.clone(),
            artifact_kind: request.artifact_kind.clone(),
            artifact_schema: request.artifact_schema.clone(),
            artifact_schema_version: request.artifact_schema_version,
            document_schema: request.document_schema.clone(),
            document_schema_version: request.document_schema_version,
            inference_schema: request.inference_schema.clone(),
            inference_schema_version: request.inference_schema_version,
            algorithm_version: request.algorithm_version,
            policy_version: request.policy_version,
            revision: request.revision,
            generation: request.generation,
            source_dialect: request.source_dialect.clone(),
            policy: request.policy.clone(),
            budgets: request.budgets.clone(),
            cancellation_id: request.cancellation_id.clone(),
            previous_state: request.previous_state.clone(),
            requested_cache_mode: request.requested_cache_mode.clone(),
            canonical_payload: request.canonical_payload.clone(),
            dependencies: request.dependencies.clone(),
            complete: true,
            actual_cache_mode: request.requested_cache_mode.clone(),
        };
        assert!(validate_inference_echo(&request, &valid).await.is_ok());
        let stale = InferenceRouteResult { generation: 8, ..valid };
        assert!(matches!(validate_inference_echo(&request, &stale).await, Err(PluginHostError::Plugin(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn live_revision_and_generation_change_rejects_the_terminal_candidate() {
        let router = ArtifactInferenceRouter::new();
        router.live_commits.lock().expect("live authority").insert("assembly-stale".into(), (7, 9));
        router.set_live_revision_generation("assembly-stale", 8, 10).await.expect("model actor freshness update");
        let live = router.live_commits.lock().expect("live authority").remove("assembly-stale").expect("active authority");
        let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(7), semio_framework_job::Generation(9), 0);
        assert!(matches!(
            semio_framework_job::validate_commit(&operation, semio_framework_job::RevisionId(live.0), semio_framework_job::Generation(live.1)),
            semio_framework_job::CommitValidation::Stale { live_revision: semio_framework_job::RevisionId(8), live_generation: semio_framework_job::Generation(10) }
        ));
    }
}
//#endregion 💡️InferenceRouter

//#region 🔖️PluginGraph
/// 🕸️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A host wave): the
/// registered set of every loaded plugin's/extension's `PluginManifest`, validated against
/// contract freeze §4 rule 5 by calling straight through to W0-C's PURE `semio_framework::
/// {validate_dependency_graph, resolve_load_order, dependents}` — this type owns none of that
/// graph logic itself, only the registration/lifecycle bookkeeping `runtime_for`
/// (`🏃️run/🦀️.rs`) needs to load a dependency before its dependent and to gate
/// hot-reload/unload against a plugin's live dependents (scout-2 §3/§5: neither existed before
/// this ticket).
pub struct PluginGraph {
    state: Mutex<BTreeMap<String, PluginManifest>>,
}

#[derive(Debug)]
pub enum PluginGraphError {
    Graph(semio_framework::DependencyGraphError),
    UnloadBlocked { plugin_id: String, dependents: Vec<String> },
    Unknown { plugin_id: String },
    LockPoisoned,
}

impl std::fmt::Display for PluginGraphError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Graph(error) => std::fmt::Display::fmt(error, formatter),
            Self::UnloadBlocked { plugin_id, dependents } => write!(formatter, "plugin `{plugin_id}` cannot be unloaded: still depended on by {dependents:?}"),
            Self::Unknown { plugin_id } => write!(formatter, "plugin `{plugin_id}` is not registered"),
            Self::LockPoisoned => formatter.write_str("plugin graph lock poisoned"),
        }
    }
}

impl std::error::Error for PluginGraphError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Graph(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework::DependencyGraphError> for PluginGraphError {
    fn from(error: semio_framework::DependencyGraphError) -> Self {
        Self::Graph(error)
    }
}

impl PluginGraph {
    pub fn new() -> Self {
        Self { state: Mutex::new(BTreeMap::new()) }
    }

    async fn lock(&self) -> Result<std::sync::MutexGuard<'_, BTreeMap<String, PluginManifest>>, PluginGraphError> {
        self.state.lock().map_err(|_| PluginGraphError::LockPoisoned)
    }

    /// 🔗️ Why a contributor may not act on `owner`'s artifact right now, if anything: `None` means
    /// the dependency is declared, the owner is loaded, and its version satisfies the requirement.
    /// Load-time registration already rejects all three, but a transaction can still meet them at
    /// dispatch time — an owner unloaded after registration, or a contributor loaded against a
    /// different build — so the transaction path asks again rather than assuming.
    pub async fn contribution_block(&self, contributor: &str, owner: &str) -> Result<Option<(&'static str, String)>, PluginGraphError> {
        let state = self.lock().await?;
        let Some(contributor_manifest) = state.get(contributor) else {
            return Ok(Some(("transaction.dependency-missing", format!("contributor `{contributor}` is not loaded"))));
        };
        let Some(dependency) = contributor_manifest.dependencies.iter().find(|dependency| dependency.plugin_id == owner) else {
            return Ok(Some(("transaction.contribution-not-permitted", format!("`{contributor}` does not declare `{owner}` as a dependency"))));
        };
        let Some(owner_manifest) = state.get(owner) else {
            return Ok(Some(("transaction.dependency-missing", format!("`{contributor}` depends on `{owner}`, which is not loaded"))));
        };
        match semio_framework::Version::parse(&owner_manifest.version) {
            Ok(version) if dependency.version.matches(&version) => Ok(None),
            Ok(version) => Ok(Some(("transaction.version-mismatch", format!("`{contributor}` requires `{owner}` {}, but the loaded build is {version}", dependency.version)))),
            Err(error) => Ok(Some(("transaction.version-mismatch", format!("`{owner}`'s version {:?} is unparseable: {error}", owner_manifest.version)))),
        }
    }

    /// ✅️ Registers (or replaces) `manifest`'s entry, re-validating the WHOLE resulting graph
    /// before committing — an invalid addition (missing dependency, version mismatch, cycle)
    /// leaves the previously-registered set untouched.
    pub async fn register(&self, manifest: PluginManifest) -> Result<(), PluginGraphError> {
        let mut state = self.lock().await?;
        let mut candidate = state.clone();
        candidate.insert(manifest.plugin_id.clone(), manifest);
        let list: Vec<PluginManifest> = candidate.values().cloned().collect();
        // 🪪️ `resolve_load_order` (not `validate_dependency_graph` alone) is what actually detects a
        // CYCLE — per W0-C's own report: "a real cycle among present plugins passes validation and
        // is caught by the toposort leftover-set walk". `validate_dependency_graph` alone only
        // catches missing-dependency/version-mismatch.
        semio_framework::resolve_load_order(&list).await?;
        *state = candidate;
        Ok(())
    }

    /// 🩹️ Contract §4.5 hot-reload half: re-validates the graph as it would look with `plugin_id`
    /// replaced by `new_manifest` — in particular every OTHER plugin's dependency ON `plugin_id`
    /// is re-checked against `new_manifest.version`, so a reload that would break a live
    /// dependent's contribution is rejected before the swap. Does not mutate on failure.
    pub async fn prepare_hot_reload(&self, new_manifest: &PluginManifest) -> Result<(), PluginGraphError> {
        let state = self.lock().await?;
        let mut candidate = state.clone();
        candidate.insert(new_manifest.plugin_id.clone(), new_manifest.clone());
        let list: Vec<PluginManifest> = candidate.values().cloned().collect();
        semio_framework::resolve_load_order(&list).await?;
        Ok(())
    }

    /// ✅️ Commits `new_manifest` after `prepare_hot_reload` has already validated it.
    pub async fn commit_hot_reload(&self, new_manifest: PluginManifest) -> Result<(), PluginGraphError> {
        self.register(new_manifest).await
    }

    /// 🔒️ Contract §4.5 unload half: refuses while any OTHER registered plugin still depends on
    /// `plugin_id` (typed `UnloadBlocked`, names every blocking dependent).
    pub async fn guard_unload(&self, plugin_id: &str) -> Result<(), PluginGraphError> {
        let blockers = self.dependents(plugin_id).await?;
        if !blockers.is_empty() {
            return Err(PluginGraphError::UnloadBlocked { plugin_id: plugin_id.to_string(), dependents: blockers });
        }
        Ok(())
    }

    /// ✂️ Removes `plugin_id`'s registration — callers MUST call `guard_unload` first.
    pub async fn unregister(&self, plugin_id: &str) -> Result<(), PluginGraphError> {
        let mut state = self.lock().await?;
        state.remove(plugin_id).ok_or_else(|| PluginGraphError::Unknown { plugin_id: plugin_id.to_string() })?;
        Ok(())
    }

    /// 🔢️ Deterministic dependency-respecting load order over every currently registered plugin.
    pub async fn load_order(&self) -> Result<Vec<String>, PluginGraphError> {
        let state = self.lock().await?;
        let list: Vec<PluginManifest> = state.values().cloned().collect();
        Ok(semio_framework::resolve_load_order(&list).await?)
    }

    /// 👥️ Direct dependents of `plugin_id`, sorted.
    pub async fn dependents(&self, plugin_id: &str) -> Result<Vec<String>, PluginGraphError> {
        let state = self.lock().await?;
        let list: Vec<PluginManifest> = state.values().cloned().collect();
        Ok(semio_framework::dependents(&list, plugin_id).await)
    }

    pub async fn manifest(&self, plugin_id: &str) -> Result<Option<PluginManifest>, PluginGraphError> {
        Ok(self.lock().await?.get(plugin_id).cloned())
    }

    pub async fn is_registered(&self, plugin_id: &str) -> Result<bool, PluginGraphError> {
        Ok(self.lock().await?.contains_key(plugin_id))
    }
}

impl Default for PluginGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod plugin_graph_tests {
    use super::*;

    async fn manifest(plugin_id: &str, version: &str, deps: &[(&str, &str)]) -> PluginManifest {
        let mut dependencies = Vec::with_capacity(deps.len());
        for (id, req) in deps {
            dependencies.push(semio_framework::PluginDependency::new(*id, semio_framework::VersionReq::parse(req).unwrap()));
        }
        PluginManifest {
            plugin_id: plugin_id.to_string(),
            label: plugin_id.to_string(),
            version: version.to_string(),
            apps: vec![],
            examples: vec![],
            capabilities: vec![],
            topic_contributions: vec![],
            commands: vec![],
            artifact_kinds: vec![],
            dependencies,
            contributions: vec![],
        }
    }

    /// 🔗️ The three ways a contribution can be blocked at DISPATCH time are distinguishable — the
    /// frozen taxonomy has separate codes for them, and collapsing "owner gone" into
    /// "not permitted" would tell an operator to fix a declaration that is already correct.
    #[semio_framework_async_macros::async_test]
    async fn contribution_block_separates_missing_owner_from_version_mismatch_from_undeclared() {
        let graph = PluginGraph::new();
        graph.register(manifest("cad", "1.0.0", &[]).await).await.unwrap();
        graph.register(manifest("aec", "1.0.0", &[("cad", "^1.0.0")]).await).await.unwrap();
        assert_eq!(graph.contribution_block("aec", "cad").await.unwrap(), None, "a declared, satisfied dependency blocks nothing");

        let (code, _) = graph.contribution_block("ghost", "cad").await.unwrap().expect("an unloaded contributor is blocked");
        assert_eq!(code, "transaction.dependency-missing");

        let (code, _) = graph.contribution_block("cad", "aec").await.unwrap().expect("an undeclared dependency is blocked");
        assert_eq!(code, "transaction.contribution-not-permitted");

        // 🛡️ The version branch is defence-in-depth, and this asserts WHY it cannot fire today rather
        // than pretending to exercise it: `register` re-validates the whole graph, so swapping `cad`
        // for a build `aec`'s requirement excludes is refused outright and the registered set keeps
        // its invariant. The branch stays because `contribution_block` is called per transaction, and
        // a future load path that mutates the set without that re-validation would otherwise hand a
        // contributor an owner it was never compiled against.
        let drift = graph.register(manifest("cad", "2.0.0", &[]).await).await.unwrap_err();
        assert!(matches!(drift, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
        assert_eq!(graph.contribution_block("aec", "cad").await.unwrap(), None, "the refused swap must leave the satisfied dependency intact");
    }

    #[semio_framework_async_macros::async_test]
    async fn load_order_respects_a_real_dependency_edge() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
        graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")]).await).await.unwrap();
        assert_eq!(graph.load_order().await.unwrap(), vec!["base".to_string(), "dependent".to_string()]);
        assert_eq!(graph.dependents("base").await.unwrap(), vec!["dependent".to_string()]);
    }

    #[semio_framework_async_macros::async_test]
    async fn register_rejects_a_missing_dependency() {
        let graph = PluginGraph::new();
        let error = graph.register(manifest("dependent", "1.0.0", &[("missing", "*")]).await).await.unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::MissingDependency { .. })));
        assert!(!graph.is_registered("dependent").await.unwrap(), "a rejected registration must not partially commit");
    }

    #[semio_framework_async_macros::async_test]
    async fn register_rejects_a_version_mismatch() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
        let error = graph.register(manifest("dependent", "1.0.0", &[("base", "^2.0.0")]).await).await.unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn a_later_registration_that_would_close_a_cycle_is_rejected() {
        let graph = PluginGraph::new();
        graph.register(manifest("a", "1.0.0", &[]).await).await.unwrap();
        graph.register(manifest("b", "1.0.0", &[("a", "*")]).await).await.unwrap();
        // Re-registering "a" (as if hot-reloading it) to depend on "b" would close a -> b -> a.
        let error = graph.register(manifest("a", "1.0.0", &[("b", "*")]).await).await.unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::Cycle { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn unload_is_refused_while_a_dependent_is_registered() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
        graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")]).await).await.unwrap();
        let error = graph.guard_unload("base").await.unwrap_err();
        assert!(matches!(error, PluginGraphError::UnloadBlocked { .. }));
        graph.unregister("dependent").await.unwrap();
        graph.guard_unload("base").await.expect("no dependents left, unload must now be permitted");
    }

    #[semio_framework_async_macros::async_test]
    async fn hot_reload_is_rejected_when_it_would_break_a_live_dependents_version_requirement() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[]).await).await.unwrap();
        graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")]).await).await.unwrap();
        let error = graph.prepare_hot_reload(&manifest("base", "2.0.0", &[]).await).await.unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
        graph.prepare_hot_reload(&manifest("base", "1.1.0", &[]).await).await.expect("a caret-compatible bump must still validate");
    }
}
//#endregion 🔖️PluginGraph

//#region 🎯️MutationRouter
/// 🗂️ Host mirror of the guest `WireMutationRosterEntry` (contract §6's `contributor.
/// list-artifact-mutations` roster row) — `semio-framework-plugin-host` does not depend on the
/// guest SDK crate (same reasoning `ExtensionManifest` below already documents), so this is a
/// field-for-field JSON-shape-identical local copy, decoded off the same `store::pack_rt::
/// encode_wire_value`/`dsl::to_dsl_value` wire `list_artifact_mutations()` returns.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HostMutationRosterEntry {
    pub mutation_id: String,
    pub verb: String,
    pub entity: String,
    pub kind: String,
    pub record: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub contributor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_kind: Option<String>,
}

/// 🎞️ Host mirror of `WireArtifactMutationPlanRequest`/`Result` (contract §5.3's `contributor.
/// artifact-mutation-plan` call) — same "cannot depend on the guest SDK crate" reasoning.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostArtifactMutationPlanRequest {
    pub artifact_kind: String,
    pub mutation_id: String,
    pub revision: u64,
    pub generation: u64,
    pub snapshot_pack: Vec<u8>,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostArtifactMutationPlanResult {
    pub artifact_kind: String,
    pub mutation_id: String,
    pub revision: u64,
    pub generation: u64,
    pub owner_ops: Vec<Vec<u8>>,
    pub label: String,
    pub foreign: Vec<protocol::ForeignStep>,
}

/// 🪶️ `store::pack_rt::decode_wire_value` + `dsl::from_dsl_value` in one step — the exact decode
/// idiom `WasmPluginRuntime::read_manifest` already uses, mirrored here for the two new
/// `contributor` wire calls (contract §6): the guest's own `encode_wire_serialized` (`🔌️plugin/
/// 🦀️.rs`) is `store::pack_rt::encode_wire_value(&to_dsl_value(value))`, NOT plain JSON.
async fn decode_wire_dsl<T: dsl::FromValue>(bytes: &[u8]) -> Result<T, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    let value = store::pack_rt::renormalize_whole_number_floats(value);
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

async fn encode_wire_dsl<T: dsl::ToValue>(value: &T) -> Result<Vec<u8>, PluginHostError> {
    let dsl_value = dsl::to_dsl_value(value).map_err(PluginHostError::Plugin)?;
    Ok(store::pack_rt::encode_wire_value(&dsl_value))
}

/// 🎯️ Contract §5.3: who answers a `(artifact_kind, mutation_id)` mutation — the artifact kind's
/// own owning plugin, or a contributor plugin that declared it as a direct dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationOwnership {
    Owner { plugin_id: String },
    Contributed { plugin_id: String },
}

/// 🎯️ Merges every loaded plugin's `list-artifact-mutations` roster into one
/// `(artifact_kind, mutation_id) -> Owner | Contributed{plugin}` table (contract §4 rules 1/3).
/// Conflict rule mirrors `ArtifactInferenceRouter::register_plugin` EXACTLY: two sources claiming
/// the same key is an error unless byte-identical.
pub struct ArtifactMutationRouter {
    routes: Mutex<BTreeMap<(String, String), (String, HostMutationRosterEntry)>>,
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): mirrors `ArtifactInferenceRouter`'s
    /// own `runtimes` field EXACTLY (same shape, same purpose) — kept by `register_plugin`/
    /// `unregister_plugin` for `plan`'s dispatch. Production today calls `register_roster` directly
    /// (see `🏃️run/🦀️.rs`, outside this packet's owned paths), which never touches this
    /// field — see this packet's report `## lease-requests` for the one-line change that wires it.
    runtimes: Mutex<HashMap<String, Arc<PluginInstanceHandle>>>,
}

impl ArtifactMutationRouter {
    pub fn new() -> Self {
        Self { routes: Mutex::new(BTreeMap::new()), runtimes: Mutex::new(HashMap::new()) }
    }

    /// 📌️ Decodes `roster_wire_bytes` (the exact `contributor.list-artifact-mutations` wire
    /// payload), registers every row (see `register_roster` for the gating rules), and keeps
    /// `handle` for `plan`'s later dispatch — mirrors `ArtifactInferenceRouter::register_plugin`'s
    /// exact 4-argument shape (`plugin_id, dependencies, handle, roster_wire_bytes`).
    pub async fn register_plugin(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], handle: Arc<PluginInstanceHandle>, roster_wire_bytes: &[u8]) -> Result<(), PluginHostError> {
        let roster: Vec<HostMutationRosterEntry> = decode_wire_dsl(roster_wire_bytes).await?;
        self.register_roster_with_runtime(plugin_id, dependencies, handle, roster).await
    }

    /// 🔌️ The PRODUCTION registration entry point: typed roster rows plus the instance handle that
    /// `plan()` will dispatch through. `🏃️run` builds `HostMutationRosterEntry` values directly off
    /// the descriptor and never holds the wire bytes `register_plugin` decodes, so routing it
    /// through that method would mean encoding typed rows to JSON purely to decode them again.
    ///
    /// 🧪️ Kept SEPARATE from the pure `register_roster` below rather than folded into it: that
    /// function is deliberately wasm-free so 15 routing tests can exercise the contract §4 gating
    /// rules without constructing a `PluginInstanceHandle`, and requiring a handle would destroy
    /// that. Registering the runtime is therefore a wrapper around the pure core, never a second
    /// call a caller could forget — which is exactly how `runtimes` sat empty in production while
    /// every routing test passed.
    pub async fn register_roster_with_runtime(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], handle: Arc<PluginInstanceHandle>, roster: Vec<HostMutationRosterEntry>) -> Result<(), PluginHostError> {
        self.register_roster(plugin_id, dependencies, roster).await?;
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router runtimes"))?.insert(plugin_id.to_string(), handle);
        Ok(())
    }

    /// 🧪️ Pure half of `register_plugin`, split out for deterministic testing without any wasm —
    /// mirrors `crate::app::register_contributions`'s own split in the guest SDK (W1-A). A
    /// CONTRIBUTED row (`contributor`/`artifact_kind` both `Some`) is gated against contract §4
    /// rule 1: `entry.contributor` must equal the reporting `plugin_id`, and `artifact_kind`'s
    /// owning plugin must be a DIRECT entry of `dependencies`. An OWNER row (both `None`) is keyed
    /// by the reporting plugin's own id and gated only by the ordinary conflict rule.
    pub async fn register_roster(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], roster: Vec<HostMutationRosterEntry>) -> Result<(), PluginHostError> {
        let mut candidate_inserts = Vec::new();
        for entry in roster {
            let owner_of_kind = match (&entry.contributor, &entry.artifact_kind) {
                (Some(contributor), Some(artifact_kind)) => {
                    if contributor != plugin_id {
                        return Err(PluginHostError::Plugin(format!("mutation roster row {:?} claims contributor `{contributor}` but was reported by plugin `{plugin_id}`", entry.mutation_id)));
                    }
                    let owner_plugin = semio_framework::io::ArtifactKindId::parse(artifact_kind).map_err(PluginHostError::Plugin)?.plugin().to_string();
                    if !dependencies.iter().any(|dependency| dependency.plugin_id == owner_plugin) {
                        return Err(PluginHostError::Plugin(format!("plugin `{plugin_id}` contributes a mutation on `{artifact_kind}` (owner `{owner_plugin}`) without declaring `{owner_plugin}` as a dependency (contract §4 rule 1)")));
                    }
                    artifact_kind.clone()
                }
                (None, None) => plugin_id.to_string(),
                _ => return Err(PluginHostError::Plugin(format!("mutation roster row {:?} has exactly one of contributor/artifact_kind set — both or neither are required", entry.mutation_id))),
            };
            candidate_inserts.push(((owner_of_kind, entry.mutation_id.clone()), entry));
        }

        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?;
        for (key, entry) in &candidate_inserts {
            if let Some((existing_plugin, existing_entry)) = routes.get(key) {
                let reporting_plugin = entry.contributor.as_deref().unwrap_or(plugin_id);
                if existing_plugin != reporting_plugin || existing_entry != entry {
                    return Err(PluginHostError::Plugin(format!("conflicting artifact mutation owner for {}/{}: {} {:?}, incoming {} {:?}", key.0, key.1, existing_plugin, existing_entry, reporting_plugin, entry)));
                }
            }
        }
        for (key, entry) in candidate_inserts {
            let reporting_plugin = entry.contributor.clone().unwrap_or_else(|| plugin_id.to_string());
            routes.insert(key, (reporting_plugin, entry));
        }
        Ok(())
    }

    /// 🔎️ Contract §5.3 resolution: who answers `(artifact_kind, mutation_id)`.
    /// 🔎️ Two-tier lookup: a CONTRIBUTED row is keyed by the exact `(target_artifact_kind,
    /// mutation_id)` (the wire row carries `artifact_kind`), but an OWNER row's wire shape carries
    /// no `artifact_kind` at all — it is keyed by `(reporting_plugin_id, mutation_id)` at
    /// registration (see `register_roster`). So an owner lookup first tries the exact
    /// `(artifact_kind, mutation_id)` key (a contributed row on this exact kind), then falls back
    /// to `(ArtifactKindId::parse(artifact_kind).plugin(), mutation_id)` — the artifact kind's own
    /// owning plugin's roster.
    pub async fn resolve(&self, artifact_kind: &str, mutation_id: &str) -> Result<MutationOwnership, PluginHostError> {
        let routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?;
        if let Some((plugin_id, entry)) = routes.get(&(artifact_kind.to_string(), mutation_id.to_string())) {
            return Ok(match &entry.contributor {
                Some(_) => MutationOwnership::Contributed { plugin_id: plugin_id.clone() },
                None => MutationOwnership::Owner { plugin_id: plugin_id.clone() },
            });
        }
        if let Ok(parsed) = semio_framework::io::ArtifactKindId::parse(artifact_kind) {
            if let Some((plugin_id, _entry)) = routes.get(&(parsed.plugin().to_string(), mutation_id.to_string())) {
                return Ok(MutationOwnership::Owner { plugin_id: plugin_id.clone() });
            }
        }
        Err(PluginHostError::Plugin(format!("no mutation route for {artifact_kind}/{mutation_id}")))
    }

    /// 📚️ Every registered roster row — surfaced for the dev-boot smoke line and for tests
    /// ("assert the contributed roster is visible").
    pub async fn roster(&self) -> Result<Vec<HostMutationRosterEntry>, PluginHostError> {
        Ok(self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?.values().map(|(_, entry)| entry.clone()).collect())
    }

    /// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): contract §5.3 dispatch — resolves
    /// `(artifact_kind, mutation_id)` (reusing `resolve` above, never duplicating that lookup) then
    /// drives the owning/contributing plugin's own `semio.mutation-plan` cold job to completion.
    /// Mirrors `ArtifactInferenceRouter::infer`'s exact shape: raw wire bytes in, raw wire bytes out
    /// (`request_bytes`/the return are both the DSL wire form `HostArtifactMutationPlanRequest`/
    /// `Result` already use elsewhere in this region — `PluginInstanceHandle::mutation_plan` passes
    /// them straight through, no re-encoding).
    pub async fn plan(&self, request_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let request: HostArtifactMutationPlanRequest = decode_wire_dsl(request_bytes).await?;
        let plugin_id = match self.resolve(&request.artifact_kind, &request.mutation_id).await? {
            MutationOwnership::Contributed { plugin_id } | MutationOwnership::Owner { plugin_id } => plugin_id,
        };
        let handle = self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router runtimes"))?.get(&plugin_id).cloned().ok_or_else(|| PluginHostError::Plugin(format!("mutation plan owner `{plugin_id}` is not loaded")))?;
        handle.mutation_plan(request_bytes).await
    }

    /// ✂️ Drops every route reported by `plugin_id` (owner rows keyed under its own id, contributed
    /// rows it reported under an owner artifact kind) and its runtime handle — called on
    /// unload/hot-reload.
    pub async fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?;
        routes.retain(|_, (owner_plugin, entry)| {
            let reporter = entry.contributor.as_deref().unwrap_or(owner_plugin.as_str());
            reporter != plugin_id
        });
        drop(routes);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router runtimes"))?.remove(plugin_id);
        Ok(())
    }
}

impl Default for ArtifactMutationRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod artifact_mutation_router_tests {
    use super::*;

    async fn owner_entry(mutation_id: &str) -> HostMutationRosterEntry {
        HostMutationRosterEntry { mutation_id: mutation_id.to_string(), verb: "set".into(), entity: "widget".into(), kind: "set-color".into(), record: "widget.doc".into(), contributor: None, artifact_kind: None }
    }

    async fn contributed_entry(mutation_id: &str, contributor: &str, artifact_kind: &str) -> HostMutationRosterEntry {
        HostMutationRosterEntry {
            mutation_id: mutation_id.to_string(),
            verb: "annotate".into(),
            entity: "widget".into(),
            kind: "annotate".into(),
            record: "widget.doc".into(),
            contributor: Some(contributor.to_string()),
            artifact_kind: Some(artifact_kind.to_string()),
        }
    }

    // 🪪️ `io::ArtifactKindId::parse("s.owner.widget").plugin()` returns the BARE middle segment
    // ("owner", not "s.owner") — plugin ids throughout these fixtures are deliberately bare to
    // match the real grammar (a real loaded plugin's `manifest.plugin_id` is its Cargo component
    // metadata id, e.g. `"cad"`, never `"s.cad"`; only a canonical artifact kind string carries the
    // `s.` prefix).
    #[semio_framework_async_macros::async_test]
    async fn owner_and_contributed_rows_both_resolve_correctly() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
        let dependency = semio_framework::PluginDependency::new("owner", semio_framework::VersionReq::Any);
        router.register_roster("contributor", &[dependency], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget").await]).await.unwrap();

        assert_eq!(router.resolve("s.owner.widget", "widget.doc#set-color").await.unwrap(), MutationOwnership::Owner { plugin_id: "owner".into() });
        assert_eq!(router.resolve("s.owner.widget", "widget.doc#contributor:annotate").await.unwrap(), MutationOwnership::Contributed { plugin_id: "contributor".into() });
        assert_eq!(router.roster().await.unwrap().len(), 2, "both the owner and the contributed row must be visible");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_contribution_onto_a_non_dependency_is_rejected() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
        let error = router.register_roster("contributor", &[], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget").await]).await.unwrap_err();
        assert!(matches!(error, PluginHostError::Plugin(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn conflicting_owner_rows_are_rejected_unless_byte_identical() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("a", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
        let error = router.register_roster("a", &[], vec![HostMutationRosterEntry { verb: "different".into(), ..owner_entry("widget.doc#set-color").await }]).await.unwrap_err();
        assert!(matches!(error, PluginHostError::Plugin(_)));
        router.register_roster("a", &[], vec![owner_entry("widget.doc#set-color").await]).await.expect("byte-identical re-registration is idempotent");
    }

    #[semio_framework_async_macros::async_test]
    async fn unregister_drops_only_that_plugins_rows() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.unwrap();
        let dependency = semio_framework::PluginDependency::new("owner", semio_framework::VersionReq::Any);
        router.register_roster("contributor", &[dependency], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget").await]).await.unwrap();
        router.unregister_plugin("contributor").await.unwrap();
        assert_eq!(router.roster().await.unwrap().len(), 1);
        assert!(router.resolve("s.owner.widget", "widget.doc#set-color").await.is_ok());
    }

    async fn mock_handle(actor: RuntimeActorId) -> (Arc<MockGuestRuntime>, Arc<PluginInstanceHandle>) {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let compiled = mock.compile(&PackageRef { package: PackageId("mutplan".to_string()), hash: PackageHash([7u8; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &budget).await.expect("mock instantiate");
        let handle = Arc::new(PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await);
        (mock, handle)
    }

    /// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `register_plugin` keeps the handle
    /// `plan` later needs, and `plan` resolves ownership THEN drives the owner's real
    /// `semio.mutation-plan` job to completion (`MockGuestRuntime`-backed, not mocked-away resolution
    /// alone) — proves this router's new `runtimes` field/`plan` method actually reach a handle, not
    /// just that `resolve` still works.
    #[semio_framework_async_macros::async_test]
    async fn plan_drives_the_registered_owners_mutation_plan_job_to_completion() {
        let router = ArtifactMutationRouter::new();
        let (mock, handle) = mock_handle(RuntimeActorId(300)).await;
        let roster_bytes = encode_wire_dsl(&vec![owner_entry("widget.doc#set-color").await]).await.expect("encode roster");
        router.register_plugin("owner", &[], handle, &roster_bytes).await.expect("register_plugin must register the roster AND keep the handle");

        let request = HostArtifactMutationPlanRequest { artifact_kind: "s.owner.widget".to_string(), mutation_id: "widget.doc#set-color".to_string(), revision: 1, generation: 1, snapshot_pack: vec![1, 2, 3], payload: vec![4, 5] };
        let request_bytes = encode_wire_dsl(&request).await.expect("encode request");
        let expected_result =
            HostArtifactMutationPlanResult { artifact_kind: request.artifact_kind.clone(), mutation_id: request.mutation_id.clone(), revision: 1, generation: 1, owner_ops: vec![vec![9]], label: "mocked".to_string(), foreign: Vec::new() };
        mock.script_job_step(RuntimeActorId(300), JobStep::Done { output: encode_wire_dsl(&expected_result).await.expect("encode expected result") }).await;

        let result_bytes = router.plan(&request_bytes).await.expect("plan must resolve ownership AND drive the job to completion");
        let result: HostArtifactMutationPlanResult = decode_wire_dsl(&result_bytes).await.expect("decode plan result");
        assert_eq!(result, expected_result);
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_fails_with_a_named_error_when_the_owner_is_not_loaded() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color").await]).await.expect("register the owner row");
        let request = HostArtifactMutationPlanRequest { artifact_kind: "s.owner.widget".to_string(), mutation_id: "widget.doc#set-color".to_string(), revision: 1, generation: 1, snapshot_pack: Vec::new(), payload: Vec::new() };
        let request_bytes = encode_wire_dsl(&request).await.expect("encode request");
        let error = router.plan(&request_bytes).await.expect_err("resolve succeeds but no handle was ever registered, so plan must still fail");
        assert!(matches!(error, PluginHostError::Plugin(message) if message.contains("not loaded")), "unexpected error");
    }
}
//#endregion 🎯️MutationRouter

//#region 🔖️InstanceDirectory
/// 🗺️ `ArtifactRef ↔ (plugin_id, instance_id, artifact_kind)` — genuinely new (scout-2 §3: "there
/// is no instance directory" before this ticket). Populated at `instantiate-app`/`Hello`/
/// `LoadDocument` (see `WasmPluginRuntime::create_app` and `HostState::pre_adopt_command_packs`'s
/// hooks below) and consulted by `HostTransactionCoordinator` to resolve a `ForeignStep.target`
/// into a live instance. Keyed by plain artifact-id strings (not the full `io::ArtifactRef`, which
/// requires an `ArtifactDialect` not always resolvable at bind time) — callers that have a real
/// `io::ArtifactRef` pass `.artifact_id` through.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceLocation {
    pub plugin_id: String,
    pub instance_id: u32,
    pub artifact_kind: String,
}

pub struct InstanceDirectory {
    state: Mutex<InstanceDirectoryState>,
}

#[derive(Default)]
struct InstanceDirectoryState {
    by_artifact_id: HashMap<String, InstanceLocation>,
    by_instance: HashMap<(String, u32), Vec<String>>,
}

impl InstanceDirectory {
    pub fn new() -> Self {
        Self { state: Mutex::new(InstanceDirectoryState::default()) }
    }

    /// 📌️ Binds `artifact_id` to `(plugin_id, instance_id, artifact_kind)`, replacing any prior
    /// binding for the SAME `artifact_id` — a `Hello`/`LoadDocument` re-bind on the same instance
    /// is expected to be idempotent, not an error (unlike the mutation/inference routers' conflict
    /// rule, this is a live pointer table, not a registration ledger).
    pub async fn bind(&self, artifact_id: &str, plugin_id: &str, instance_id: u32, artifact_kind: &str) -> Result<(), PluginHostError> {
        let mut state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("instance directory"))?;
        if let Some(previous) = state.by_artifact_id.get(artifact_id) {
            if previous.plugin_id != plugin_id || previous.instance_id != instance_id {
                let previous_key = (previous.plugin_id.clone(), previous.instance_id);
                if let Some(previous_refs) = state.by_instance.get_mut(&previous_key) {
                    previous_refs.retain(|existing| existing != artifact_id);
                }
            }
        }
        state.by_artifact_id.insert(artifact_id.to_string(), InstanceLocation { plugin_id: plugin_id.to_string(), instance_id, artifact_kind: artifact_kind.to_string() });
        let refs = state.by_instance.entry((plugin_id.to_string(), instance_id)).or_default();
        if !refs.iter().any(|existing| existing == artifact_id) {
            refs.push(artifact_id.to_string());
        }
        Ok(())
    }

    pub async fn resolve(&self, artifact_id: &str) -> Option<InstanceLocation> {
        self.state.lock().ok()?.by_artifact_id.get(artifact_id).cloned()
    }

    /// ✂️ Drops every binding for `(plugin_id, instance_id)` — called on `destroy_app`.
    pub async fn unbind_instance(&self, plugin_id: &str, instance_id: u32) {
        let Ok(mut state) = self.state.lock() else { return };
        if let Some(refs) = state.by_instance.remove(&(plugin_id.to_string(), instance_id)) {
            for artifact_id in refs {
                state.by_artifact_id.remove(&artifact_id);
            }
        }
    }

    pub async fn artifact_ids_for_instance(&self, plugin_id: &str, instance_id: u32) -> Vec<String> {
        self.state.lock().ok().and_then(|state| state.by_instance.get(&(plugin_id.to_string(), instance_id)).cloned()).unwrap_or_default()
    }
}

impl Default for InstanceDirectory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod instance_directory_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn bind_resolve_and_unbind_round_trip() {
        let directory = InstanceDirectory::new();
        directory.bind("artifacts/node-a", "s.cad", 7, "s.cad.document").await.unwrap();
        let location = directory.resolve("artifacts/node-a").await.expect("bound artifact must resolve");
        assert_eq!(location, InstanceLocation { plugin_id: "s.cad".into(), instance_id: 7, artifact_kind: "s.cad.document".into() });
        directory.unbind_instance("s.cad", 7).await;
        assert!(directory.resolve("artifacts/node-a").await.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn rebinding_the_same_artifact_id_replaces_the_prior_location() {
        let directory = InstanceDirectory::new();
        directory.bind("artifacts/node-a", "s.cad", 1, "s.cad.document").await.unwrap();
        directory.bind("artifacts/node-a", "s.cad", 2, "s.cad.document").await.unwrap();
        assert_eq!(directory.resolve("artifacts/node-a").await.unwrap().instance_id, 2);
        assert!(directory.artifact_ids_for_instance("s.cad", 1).await.is_empty(), "the stale instance no longer owns this artifact id");
    }
}
//#endregion 🔖️InstanceDirectory

//#region 🎯️TransactionCoordinator
/// 👥️ One transaction participant: which plugin, which live instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionMember {
    pub plugin_id: String,
    pub instance_id: u32,
}

/// ✅️ A completed (committed) transaction — `members` is discovery order (member 0 = initiator);
/// `edit_ids[i]` is `members[i]`'s own edit id, from the actual (reverse-order) commit phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionOutcome {
    pub txn_id: String,
    pub members: Vec<TransactionMember>,
    pub edit_ids: Vec<String>,
}

/// 🧯 Contract freeze §5's frozen rejection taxonomy, typed.
#[derive(Debug)]
pub enum TransactionError {
    Rejected { code: String, message: String },
    Host(PluginHostError),
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rejected { code, message } => write!(formatter, "{code}: {message}"),
            Self::Host(error) => write!(formatter, "plugin host error: {error}"),
        }
    }
}

impl std::error::Error for TransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Host(error) => Some(error),
            Self::Rejected { .. } => None,
        }
    }
}

impl From<PluginHostError> for TransactionError {
    fn from(error: PluginHostError) -> Self {
        Self::Host(error)
    }
}

#[cfg(test)]
mod microsecond_clock_tests {
    use super::*;

    #[test]
    fn microsecond_owned_wasi_clock_is_real_nanoseconds_with_checked_unsigned_range() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔨️modules/🧵️job/⏱️budget/🧪️clock.json")).unwrap();
        for law in fixture["wasi"].as_array().unwrap() {
            let nanoseconds = law["nanoseconds"].as_str().unwrap().parse::<u128>().unwrap();
            let duration = std::time::Duration::new((nanoseconds / 1_000_000_000) as u64, (nanoseconds % 1_000_000_000) as u32);
            let result = owned_wasi_nanoseconds(duration);
            assert_eq!(result.is_ok(), law["accepted"].as_bool().unwrap());
            if let Ok(actual) = result { assert_eq!(u128::from(actual), nanoseconds); }
        }
        let read = || match owned_wasi_monotonic_clock().unwrap() { Value::I64(value) => u64::from_ne_bytes(value.to_ne_bytes()), _ => panic!("WASI clock primitive") };
        let start = read();
        let mut last = start;
        for _ in 0..10_000 {
            let next = read();
            assert!(next >= last);
            last = next;
            if last > start { break; }
        }
        assert!(last > start, "owned WASI host must not return a frozen zero clock");
        eprintln!("[DEBUG] owned WASI real monotonic nanoseconds start={start} last={last}");
    }
}

#[cfg(test)]
mod owned_error_tests {
    use super::*;
    use std::error::Error;

    #[semio_framework_async_macros::async_test]
    async fn owned_errors_preserve_display_source_and_from() {
        let host = PluginHostError::from(std::io::Error::new(std::io::ErrorKind::Other, "disk"));
        assert_eq!(host.to_string(), "io: disk");
        assert!(host.source().is_some());
        let turn = TurnFault::from(host);
        assert_eq!(turn.to_string(), "io: disk");
        assert!(turn.source().is_some());
        let transaction = TransactionError::from(PluginHostError::Plugin("broken".into()));
        assert_eq!(transaction.to_string(), "plugin host error: plugin: broken");
        assert!(transaction.source().is_some());
        assert_eq!(PluginGraphError::Unknown { plugin_id: "missing".into() }.to_string(), "plugin `missing` is not registered");
    }
}

impl TransactionError {
    pub async fn rejected(code: &str, message: impl Into<String>) -> Self {
        Self::Rejected { code: code.to_string(), message: message.into() }
    }

    pub async fn code(&self) -> &str {
        match self {
            TransactionError::Rejected { code, .. } => code,
            TransactionError::Host(_) => "transaction.commit-failed",
        }
    }
}

async fn payload_hash_of(bytes: &[u8]) -> protocol::PayloadHash {
    protocol::PayloadHash(*semio_framework_hash::hash(bytes).as_bytes())
}

/// 🧮️ One member's accumulated prepare state, built up while the resolution loop walks the
/// foreign-step graph — never sent to the guest until every step targeting this member AT THE
/// CURRENT DEPTH has been folded in, so a member is `TransactionPrepare`d AT MOST ONCE per
/// transaction (contract §5.9: a second concurrent prepare on one instance is a hard
/// `instance-busy` reject at the guest, so the host must never issue two). Always sent in the
/// PRE-PLANNED wire form (never the owner-mutation form), per W1-B's own recommendation
/// (`📓️w1-b-report.md` §5): the owner-mutation form carries no `origin` on the wire, so preferring
/// pre-planned form for every foreign target closes that provenance gap and lets N accumulated ops
/// ride in one `prepared_ops` list regardless of how many `ForeignStep`s targeted this member.
struct MemberDraft {
    prepared_ops: Vec<Vec<u8>>,
    label: String,
    origin: protocol::MutationOrigin,
}

pub struct HostTransactionCoordinator {
    seq: std::sync::atomic::AtomicU64,
}

impl HostTransactionCoordinator {
    pub fn new() -> Self {
        Self { seq: std::sync::atomic::AtomicU64::new(1) }
    }

    async fn next_seq(&self) -> u64 {
        self.seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    async fn mint_txn_id(&self, initiator: &TransactionMember) -> String {
        format!("txn-{}-{}-{}", initiator.plugin_id, initiator.instance_id, self.next_seq().await)
    }

    /// 🎯️ Contract §5 steps 1-6: resolves every `foreign` step to a live instance via
    /// `instances`/`mutation_router`, recurses with depth/cycle guards (§5.4), issues ONE
    /// `TransactionPrepare` per member (always pre-planned form — see `MemberDraft`'s doc),
    /// requires every member's `TransactionPrepared.rejection` to be empty (§5.5, else rolls back
    /// every already-prepared member), then commits in REVERSE discovery order (§5.6,
    /// compensating already-committed members via `TransactionUndo` on a commit failure).
    ///
    /// `exchange`/`plan_contributed` are closures so this can be driven over real
    /// `WasmPluginRuntime`s (production, see `🏃️run/🦀️.rs`) or an in-process fake
    /// (tests) without either depending on the other.
    #[allow(clippy::too_many_arguments)]
    pub async fn run_transaction(
        &self,
        instances: &InstanceDirectory,
        mutation_router: &ArtifactMutationRouter,
        mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>,
        mut plan_contributed: impl FnMut(&str, &str, &str, &TransactionMember, &[u8]) -> Result<HostArtifactMutationPlanResult, TransactionError>,
        initiator: TransactionMember,
        local_ops: Vec<Vec<u8>>,
        description: String,
        foreign: Vec<protocol::ForeignStep>,
    ) -> Result<TransactionOutcome, TransactionError> {
        let txn_id = self.mint_txn_id(&initiator).await;

        // 🚫️async: R10 residue shape 1 — `Option::and_then` takes a sync closure, so the
        // `resolve(...).await` that used to live inside it is hoisted out here instead.
        let initiator_artifact_id = instances.artifact_ids_for_instance(&initiator.plugin_id, initiator.instance_id).await.into_iter().next();
        let initiator_target = match initiator_artifact_id {
            Some(artifact_id) => match instances.resolve(&artifact_id).await {
                Some(location) => protocol::ForeignTarget { artifact_id, artifact_kind: location.artifact_kind, dialect: None },
                None => protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None },
            },
            None => protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None },
        };

        let mut discovery_order: Vec<TransactionMember> = vec![initiator.clone()];
        let mut drafts: BTreeMap<(String, u32), MemberDraft> = BTreeMap::new();
        drafts.insert((initiator.plugin_id.clone(), initiator.instance_id), MemberDraft { prepared_ops: local_ops, label: description, origin: protocol::MutationOrigin::Owner });

        let mut visited: std::collections::HashSet<(String, String, [u8; 32])> = std::collections::HashSet::new();
        let mut frontier: Vec<protocol::ForeignStep> = foreign;
        let mut depth: u8 = 0;

        while !frontier.is_empty() {
            depth += 1;
            if depth > protocol::MAX_PLAN_DEPTH {
                return Err(TransactionError::rejected("transaction.depth-exceeded", format!("transaction `{txn_id}` exceeded MAX_PLAN_DEPTH ({})", protocol::MAX_PLAN_DEPTH)).await);
            }
            let mut next_frontier: Vec<protocol::ForeignStep> = Vec::new();
            for step in frontier {
                let cycle_key = (step.target.artifact_id.clone(), step.mutation_id.0.clone(), *semio_framework_hash::hash(&step.payload).as_bytes());
                if !visited.insert(cycle_key) {
                    return Err(TransactionError::rejected("transaction.cycle", format!("transaction `{txn_id}` revisited {}/{}", step.target.artifact_id, step.mutation_id.0)).await);
                }

                // 🚫️async: R10 residue shape 1 — `TransactionError::rejected` is async, hoisted
                // out of `ok_or_else`/`map_err`'s sync closures via explicit matches.
                let location = match instances.resolve(&step.target.artifact_id).await {
                    Some(location) => location,
                    None => return Err(TransactionError::rejected("transaction.unknown-target", format!("no live instance bound to artifact id `{}`", step.target.artifact_id)).await),
                };
                let ownership = match mutation_router.resolve(&location.artifact_kind, &step.mutation_id.0).await {
                    Ok(ownership) => ownership,
                    Err(error) => return Err(TransactionError::rejected("transaction.unknown-mutation", error.to_string()).await),
                };

                let member = TransactionMember { plugin_id: location.plugin_id.clone(), instance_id: location.instance_id };
                let key = (member.plugin_id.clone(), member.instance_id);
                if !drafts.contains_key(&key) {
                    discovery_order.push(member.clone());
                }

                match ownership {
                    MutationOwnership::Owner { .. } => {
                        let draft = drafts.entry(key).or_insert_with(|| MemberDraft { prepared_ops: Vec::new(), label: step.label.clone(), origin: protocol::MutationOrigin::Transaction { initiator: initiator_target.clone() } });
                        draft.prepared_ops.push(step.payload.clone());
                    }
                    MutationOwnership::Contributed { plugin_id: contributor } => {
                        let plan = plan_contributed(&contributor, &location.artifact_kind, &step.mutation_id.0, &member, &step.payload)?;
                        let origin = protocol::MutationOrigin::Contributed { plugin_id: contributor.clone(), mutation_id: step.mutation_id.clone(), payload_hash: payload_hash_of(&step.payload).await };
                        let draft = drafts.entry(key).or_insert_with(|| MemberDraft { prepared_ops: Vec::new(), label: plan.label.clone(), origin: origin.clone() });
                        draft.prepared_ops.extend(plan.owner_ops);
                        draft.origin = origin;
                        next_frontier.extend(plan.foreign);
                    }
                }
            }
            frontier = next_frontier;
        }

        // Phase 1 (§5.5): prepare every member exactly once, in discovery order; any rejection
        // rolls back every member already prepared before it.
        let mut rejection: Option<TransactionError> = None;
        let mut prepared: Vec<TransactionMember> = Vec::new();
        for member in &discovery_order {
            let key = (member.plugin_id.clone(), member.instance_id);
            let draft = drafts.get(&key).expect("every discovered member has a draft");
            let origin_bytes = match encode_wire_dsl(&draft.origin).await {
                Ok(bytes) => bytes,
                Err(error) => {
                    rejection = Some(TransactionError::Host(error));
                    break;
                }
            };
            let command = protocol::AppCommand::TransactionPrepare {
                seq: self.next_seq().await,
                txn_id: txn_id.clone(),
                mutation_id: String::new(),
                payload: Vec::new(),
                prepared_ops: draft.prepared_ops.clone(),
                label: draft.label.clone(),
                origin: origin_bytes,
            };
            let frames = match exchange(&member.plugin_id, member.instance_id, command) {
                Ok(frames) => frames,
                Err(error) => {
                    rejection = Some(error);
                    break;
                }
            };
            let outcome = frames.iter().find_map(|frame| match frame {
                protocol::AppFrame::TransactionPrepared { txn_id: reply_txn, rejection, .. } if reply_txn == &txn_id => Some(rejection.clone()),
                protocol::AppFrame::Error { fault, .. } => Some(fault.clone()),
                _ => None,
            });
            match outcome {
                Some(rejection_bytes) if rejection_bytes.is_empty() => {
                    prepared.push(member.clone());
                }
                Some(rejection_bytes) => {
                    let fault = dsl::decode_fault_bytes(&rejection_bytes);
                    rejection = Some(TransactionError::rejected("transaction.member-rejected", format!("{}/{} rejected prepare (`{}`): {}", member.plugin_id, member.instance_id, fault.code.0, fault.message)).await);
                    break;
                }
                None => {
                    rejection = Some(TransactionError::rejected("transaction.member-rejected", format!("{}/{} sent no TransactionPrepared reply", member.plugin_id, member.instance_id)).await);
                    break;
                }
            }
        }

        if let Some(error) = rejection {
            for member in prepared.iter().rev() {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRollback { seq: self.next_seq().await, txn_id: txn_id.clone() });
            }
            return Err(error);
        }

        // Phase 2 (§5.6): commit in REVERSE discovery order.
        let mut committed: Vec<TransactionMember> = Vec::new();
        let mut edit_id_by_member: BTreeMap<(String, u32), String> = BTreeMap::new();
        let mut commit_error: Option<TransactionError> = None;
        for member in discovery_order.iter().rev() {
            let command = protocol::AppCommand::TransactionCommit { seq: self.next_seq().await, txn_id: txn_id.clone() };
            let outcome = exchange(&member.plugin_id, member.instance_id, command).ok().and_then(|frames| {
                frames.into_iter().find_map(|frame| match frame {
                    protocol::AppFrame::TransactionCommitted { txn_id: reply_txn, edit_id } if reply_txn == txn_id => Some(edit_id),
                    _ => None,
                })
            });
            match outcome {
                Some(edit_id) => {
                    edit_id_by_member.insert((member.plugin_id.clone(), member.instance_id), edit_id);
                    committed.push(member.clone());
                }
                None => {
                    commit_error = Some(TransactionError::rejected("transaction.commit-failed", format!("{}/{} failed to commit txn `{txn_id}`", member.plugin_id, member.instance_id)).await);
                    break;
                }
            }
        }

        if let Some(error) = commit_error {
            for member in &committed {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionUndo { seq: self.next_seq().await, group_id: txn_id.clone() });
            }
            for member in discovery_order.iter().filter(|member| !committed.contains(member)) {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRollback { seq: self.next_seq().await, txn_id: txn_id.clone() });
            }
            return Err(error);
        }

        let edit_ids: Vec<String> = discovery_order.iter().map(|member| edit_id_by_member.get(&(member.plugin_id.clone(), member.instance_id)).cloned().unwrap_or_default()).collect();
        Ok(TransactionOutcome { txn_id, members: discovery_order, edit_ids })
    }

    /// 🔁️ Contract §5.7: fans `TransactionUndo{group_id}` out to every member, best-effort (a
    /// member whose tail has since moved on independently errors on ITS side, not here — a host
    /// must not assume success for every member).
    pub async fn undo_group(&self, mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>, members: &[TransactionMember], group_id: &str) {
        for member in members {
            let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionUndo { seq: self.next_seq().await, group_id: group_id.to_string() });
        }
    }

    pub async fn redo_group(&self, mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>, members: &[TransactionMember], group_id: &str) {
        for member in members {
            let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRedo { seq: self.next_seq().await, group_id: group_id.to_string() });
        }
    }
}

impl Default for HostTransactionCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod host_transaction_coordinator_tests {
    //! 🧪️ A minimal in-process fake — no wasm — that faithfully implements the SAME two-phase wire
    //! contract §5 semantics `VcsArtifactApp::transaction_prepare/commit/rollback/undo/redo` (W1-B)
    //! implements: one `Option<pending>` per instance, `TransactionCommit` applies as one edit
    //! stamped with `group_id = txn_id`, `TransactionUndo`/`Redo` toggle a per-instance tail flag.
    //! This proves `HostTransactionCoordinator`'s OWN orchestration (resolution via
    //! `InstanceDirectory`/`ArtifactMutationRouter`, phase-1 all-or-nothing, reverse-order commit,
    //! compensation, group fan-out) deterministically; the real wasmtime e2e in
    //! `🏃️run/🦀️.rs` proves the wire-level plumbing against a REAL guest.
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct FakeInstance {
        pending: Option<(String, Vec<Vec<u8>>)>, // (txn_id, prepared_ops)
        edits: Vec<(String, Vec<Vec<u8>>)>,      // (group_id, ops) applied, in commit order
        undone: Vec<String>,
    }

    #[derive(Default)]
    struct FakeCluster {
        instances: RefCell<HashMap<(String, u32), FakeInstance>>,
    }

    impl FakeCluster {
        // 🚫️async: E1 — pure in-memory RefCell fake, no suspension point; reverted per R9
        // (run_transaction/undo_group require a SYNC FnMut(...) -> Result<...> closure).
        fn exchange(&self, plugin_id: &str, instance_id: u32, command: protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError> {
            let mut instances = self.instances.borrow_mut();
            let instance = instances.entry((plugin_id.to_string(), instance_id)).or_default();
            let frame = match command {
                protocol::AppCommand::TransactionPrepare { txn_id, prepared_ops, .. } => {
                    if instance.pending.is_some() {
                        protocol::AppFrame::TransactionPrepared { txn_id, foreign: Vec::new(), rejection: host_fault_bytes("transaction.instance-busy", "already has a pending transaction") }
                    } else {
                        instance.pending = Some((txn_id.clone(), prepared_ops));
                        protocol::AppFrame::TransactionPrepared { txn_id, foreign: Vec::new(), rejection: Vec::new() }
                    }
                }
                protocol::AppCommand::TransactionCommit { txn_id, .. } => match instance.pending.take() {
                    Some((pending_id, ops)) if pending_id == txn_id => {
                        instance.edits.push((txn_id.clone(), ops));
                        protocol::AppFrame::TransactionCommitted { txn_id: txn_id.clone(), edit_id: format!("edit-{}", instance.edits.len()) }
                    }
                    other => {
                        instance.pending = other;
                        protocol::AppFrame::Error { in_reply_to: None, fault: host_fault_bytes("transaction.commit-failed", "no matching pending transaction"), report: Vec::new() }
                    }
                },
                protocol::AppCommand::TransactionRollback { txn_id, .. } => {
                    if matches!(&instance.pending, Some((pending_id, _)) if pending_id == &txn_id) {
                        instance.pending = None;
                    }
                    protocol::AppFrame::Done { in_reply_to: 0 }
                }
                protocol::AppCommand::TransactionUndo { group_id, .. } => {
                    if instance.edits.iter().any(|(id, _)| id == &group_id) {
                        instance.undone.push(group_id.clone());
                    }
                    protocol::AppFrame::Done { in_reply_to: 0 }
                }
                other => panic!("unexpected command in fake transaction cluster: {other:?}"),
            };
            Ok(vec![frame])
        }
    }

    async fn dependency(id: &str) -> semio_framework::PluginDependency {
        semio_framework::PluginDependency::new(id, semio_framework::VersionReq::Any)
    }

    #[semio_framework_async_macros::async_test]
    async fn a_two_member_transaction_commits_and_group_undo_restores_both() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();

        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router
            .register_roster(
                "a",
                &[dependency("b").await],
                vec![HostMutationRosterEntry {
                    mutation_id: "s.b.widget#a:annotate".into(),
                    verb: "annotate".into(),
                    entity: "widget".into(),
                    kind: "annotate".into(),
                    record: "widget.doc".into(),
                    contributor: Some("a".into()),
                    artifact_kind: Some("s.b.widget".into()),
                }],
            )
            .await
            .unwrap();

        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep {
            target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
            mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
            payload: vec![9, 9],
            label: "annotate".into(),
        }];

        let outcome = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                |_contributor, _artifact_kind, _mutation_id, _member, payload| {
                    Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "annotate".into(), foreign: Vec::new() })
                },
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1, 2, 3]],
                "propose annotate".into(),
                foreign,
            )
            .await
            .expect("a well-formed two-member transaction must commit");

        assert_eq!(outcome.members.len(), 2);
        assert_eq!(outcome.members[0], TransactionMember { plugin_id: "s.a".into(), instance_id: 1 }, "member 0 is the initiator");
        assert_eq!(outcome.edit_ids.len(), 2);
        assert!(outcome.edit_ids.iter().all(|id| !id.is_empty()));

        {
            let instances_map = cluster.instances.borrow();
            assert_eq!(instances_map.get(&("s.a".to_string(), 1)).unwrap().edits.len(), 1);
            assert_eq!(instances_map.get(&("s.b".to_string(), 2)).unwrap().edits.len(), 1);
        }

        coordinator.undo_group(|plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command), &outcome.members, &outcome.txn_id).await;
        let instances_map = cluster.instances.borrow();
        assert!(instances_map.get(&("s.a".to_string(), 1)).unwrap().undone.contains(&outcome.txn_id), "initiator must be undone");
        assert!(instances_map.get(&("s.b".to_string(), 2)).unwrap().undone.contains(&outcome.txn_id), "the contributed target must ALSO be undone (group undo restores both members)");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_target_is_rejected_before_any_prepare_is_sent() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
        let router = ArtifactMutationRouter::new();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep {
            target: protocol::ForeignTarget { artifact_id: "artifacts/nowhere".into(), artifact_kind: "s.b.widget".into(), dialect: None },
            mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
            payload: vec![1],
            label: "x".into(),
        }];
        let error = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                |_, _, _, _, _| unreachable!("no contributed step to plan"),
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                foreign,
            )
            .await
            .unwrap_err();
        assert_eq!(error.code().await, "transaction.unknown-target");
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_mutation_is_rejected() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();
        let router = ArtifactMutationRouter::new();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep {
            target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
            mutation_id: protocol::SchemaId("s.b.widget#unregistered".into()),
            payload: vec![1],
            label: "x".into(),
        }];
        let error = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                |_, _, _, _, _| unreachable!("owner route, never contributed"),
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                foreign,
            )
            .await
            .unwrap_err();
        assert_eq!(error.code().await, "transaction.unknown-mutation");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_cycle_is_rejected() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();
        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router
            .register_roster(
                "a",
                &[dependency("b").await],
                vec![HostMutationRosterEntry {
                    mutation_id: "s.b.widget#a:annotate".into(),
                    verb: "annotate".into(),
                    entity: "widget".into(),
                    kind: "annotate".into(),
                    record: "widget.doc".into(),
                    contributor: Some("a".into()),
                    artifact_kind: Some("s.b.widget".into()),
                }],
            )
            .await
            .unwrap();
        let coordinator = HostTransactionCoordinator::new();
        let step = protocol::ForeignStep {
            target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
            mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
            payload: vec![7],
            label: "x".into(),
        };
        // The contributed plan returns the SAME step again -> a real cycle by (artifact_id, mutation_id, payload_hash).
        let step_for_plan = step.clone();
        let error = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                move |_, _, _, _, payload| {
                    Ok(HostArtifactMutationPlanResult {
                        artifact_kind: "s.b.widget".into(),
                        mutation_id: "s.b.widget#a:annotate".into(),
                        revision: 0,
                        generation: 0,
                        owner_ops: vec![payload.to_vec()],
                        label: "x".into(),
                        foreign: vec![step_for_plan.clone()],
                    })
                },
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                vec![step],
            )
            .await
            .unwrap_err();
        assert_eq!(error.code().await, "transaction.cycle");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_member_rejection_rolls_back_every_already_prepared_member() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").await.unwrap();
        // Pre-occupy s.b/2's pending slot so its OWN prepare hits `transaction.instance-busy`
        // for real, through the fake's genuine busy-check — not a stubbed rejection.
        cluster.instances.borrow_mut().entry(("s.b".to_string(), 2)).or_default().pending = Some(("someone-elses-txn".into(), vec![]));

        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router
            .register_roster(
                "a",
                &[dependency("b").await],
                vec![HostMutationRosterEntry {
                    mutation_id: "s.b.widget#a:annotate".into(),
                    verb: "annotate".into(),
                    entity: "widget".into(),
                    kind: "annotate".into(),
                    record: "widget.doc".into(),
                    contributor: Some("a".into()),
                    artifact_kind: Some("s.b.widget".into()),
                }],
            )
            .await
            .unwrap();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep {
            target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None },
            mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
            payload: vec![1],
            label: "x".into(),
        }];

        let error = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                |_, _, _, _, payload| {
                    Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "x".into(), foreign: Vec::new() })
                },
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                foreign,
            )
            .await
            .unwrap_err();
        assert_eq!(error.code().await, "transaction.member-rejected");
        let instances_map = cluster.instances.borrow();
        assert!(instances_map.get(&("s.a".to_string(), 1)).unwrap().pending.is_none(), "the initiator, prepared before the rejection, must have been rolled back");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_chain_deeper_than_max_plan_depth_is_rejected() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").await.unwrap();
        for i in 0..10u8 {
            instances.bind(&format!("artifacts/target-{i}"), "s.b", 100 + i as u32, "s.b.widget").await.unwrap();
        }
        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router
            .register_roster(
                "a",
                &[dependency("b").await],
                vec![HostMutationRosterEntry {
                    mutation_id: "s.b.widget#a:annotate".into(),
                    verb: "annotate".into(),
                    entity: "widget".into(),
                    kind: "annotate".into(),
                    record: "widget.doc".into(),
                    contributor: Some("a".into()),
                    artifact_kind: Some("s.b.widget".into()),
                }],
            )
            .await
            .unwrap();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep {
            target: protocol::ForeignTarget { artifact_id: "artifacts/target-0".into(), artifact_kind: "s.b.widget".into(), dialect: None },
            mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
            payload: vec![0],
            label: "x".into(),
        }];
        // Each level's contributed plan hands back ONE new foreign step targeting the NEXT (distinct)
        // instance in the chain, so the cycle guard (which keys on artifact_id) never fires — this is
        // purely a depth chain, 10 hops deep against `MAX_PLAN_DEPTH` = 8.
        let error = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                |_, _, _, _, payload| {
                    let next = payload[0] + 1;
                    let foreign = if (next as usize) < 10 {
                        vec![protocol::ForeignStep {
                            target: protocol::ForeignTarget { artifact_id: format!("artifacts/target-{next}"), artifact_kind: "s.b.widget".into(), dialect: None },
                            mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()),
                            payload: vec![next],
                            label: "x".into(),
                        }]
                    } else {
                        Vec::new()
                    };
                    Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "x".into(), foreign })
                },
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                foreign,
            )
            .await
            .unwrap_err();
        assert_eq!(error.code().await, "transaction.depth-exceeded");
    }
}
//#endregion 🎯️TransactionCoordinator

//#region 🔖️AppRouter
/// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §3: `(ArtifactDialect,
/// AppRole) -> Vec<AppRef>`, built by walking every loaded plugin's `PluginManifest.apps`.
/// Registration idiom mirrors `ArtifactInferenceRouter` above: one `Mutex`-guarded table,
/// `register_plugin` called once per loaded `WasmPluginRuntime` (reads its own already-resident
/// `.manifest` directly — no extra guest call needed, unlike `IoRouter`/`ArtifactInferenceRouter`,
/// which must query a composer/inference roster over the wasm ABI because the host has no other way
/// to see inside a guest's linear memory; a `PluginManifest` is already host-resident). Ownership of
/// a dialect's `artifact_kind` is derived from `PluginManifest.artifact_kinds` (plugin-level
/// declarations only — matches that field's own doc: "library plugins with zero apps declare kinds
/// here"); the first plugin to claim a kind wins. Nothing here consults per-app `artifact_kinds` —
/// that is a different vocabulary (OS resource/media catalog kinds, see `register_artifact_descriptors`
/// in `semio-framework-os`), not dialect ownership. Deliberately independent of the peer `PluginGraph`
/// region above (dependency-graph load-order/version validation) — this router only needs each
/// manifest's OWN `dependencies` list for the contribution gate, not global load order.
pub struct AppRouter {
    state: Mutex<AppRouterState>,
}

#[derive(Default)]
struct AppRouterState {
    /// 🗂️ `artifact_kind -> owning plugin_id`.
    owners: HashMap<String, String>,
    /// 🗂️ `plugin_id -> its declared dependency plugin ids` (contract §3's contribution gate).
    dependencies: HashMap<String, BTreeSet<String>>,
    /// 🗂️ `(dialect, role) -> registered AppRefs`, unsorted insertion order — `surfaces_for` sorts
    /// lazily against the CURRENT `owners` snapshot (never stale, since ownership never changes once
    /// claimed).
    surfaces: HashMap<(semio_framework::ArtifactDialect, semio_framework::AppRole), Vec<semio_framework::AppRef>>,
    /// 🚧️ Every `(plugin_id, app_id)` seen so far, for O(1) `surface.conflict` detection.
    registered_refs: std::collections::HashSet<(String, String)>,
}

impl AppRouter {
    pub fn new() -> Self {
        Self { state: Mutex::new(AppRouterState::default()) }
    }

    // 🗑️ The old `register_plugin(&self, plugin_id, runtime: &WasmPluginRuntime)` thin wrapper is
    // gone with `WasmPluginRuntime` itself — it only ever read `runtime.manifest` (already
    // host-resident, no wasm call), so every caller now calls `register_manifest` directly with
    // whatever `PluginManifest` it already has on hand (mirrors `IoRouter`/`ArtifactInferenceRouter`'s
    // own post-`WasmPluginRuntime` registration idiom: pre-decoded data in, no runtime dependency).

    /// 🧪️ `register_plugin` split out for direct manifest-driven testing (no wasmtime component
    /// needed to exercise the two frozen conflict/gate faults) — pure aside from the `Mutex` lock.
    pub async fn register_manifest(&self, plugin_id: &str, manifest: &PluginManifest) -> Result<(), semio_framework::Fault> {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let dependencies: BTreeSet<String> = manifest.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).collect();
        state.dependencies.insert(plugin_id.to_string(), dependencies);
        for spec in &manifest.artifact_kinds {
            state.owners.entry(spec.id.clone()).or_insert_with(|| plugin_id.to_string());
        }
        for app in &manifest.apps {
            let owner = state.owners.entry(app.dialect.artifact_kind.clone()).or_insert_with(|| plugin_id.to_string()).clone();
            if owner != plugin_id {
                let permitted = state.dependencies.get(plugin_id).map(|deps| deps.contains(&owner)).unwrap_or(false);
                if !permitted {
                    return Err(semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Framework,
                        semio_framework::FaultCode::new("surface.contribution-not-permitted"),
                        format!("plugin `{plugin_id}` declares a surface for `{}` (owned by `{owner}`) without listing `{owner}` in its dependencies", app.dialect.to_coordinate()),
                    ));
                }
            }
            let app_ref = semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: app.id.clone() };
            if !state.registered_refs.insert((app_ref.plugin_id.clone(), app_ref.app_id.clone())) {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("surface.conflict"), format!("surface `{}` is already registered for plugin `{}`", app_ref.app_id, app_ref.plugin_id)));
            }
            state.surfaces.entry((app.dialect.clone(), app.role)).or_default().push(app_ref);
        }
        Ok(())
    }

    /// 📚️ Every `AppRef` serving `(dialect, role)`, deterministically ordered: the dialect's owner
    /// plugin's surface first (if it has one), then the rest sorted `plugin_id` asc / `app_id` asc
    /// (contract §3).
    pub async fn surfaces_for(&self, dialect: &semio_framework::ArtifactDialect, role: semio_framework::AppRole) -> Vec<semio_framework::AppRef> {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = state.owners.get(&dialect.artifact_kind).cloned();
        let mut refs = state.surfaces.get(&(dialect.clone(), role)).cloned().unwrap_or_default();
        refs.sort_by(|a, b| {
            let a_owner = owner.as_deref() == Some(a.plugin_id.as_str());
            let b_owner = owner.as_deref() == Some(b.plugin_id.as_str());
            match (a_owner, b_owner) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => (a.plugin_id.as_str(), a.app_id.as_str()).cmp(&(b.plugin_id.as_str(), b.app_id.as_str())),
            }
        });
        refs
    }

    /// 🎯️ The dialect's owner plugin id, if any plugin has claimed `artifact_kind` (via its own
    /// `PluginManifest.artifact_kinds`, or by being first to declare ANY surface for it).
    pub async fn owner_of(&self, artifact_kind: &str) -> Option<String> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).owners.get(artifact_kind).cloned()
    }

    /// ✂️ Drops every surface AND dependency record `plugin_id` registered — mirrors `IoRouter`/
    /// `ArtifactMutationRouter`/`ArtifactInferenceRouter`'s own `unregister_plugin` (called from
    /// `WasmtimeNodeHost::unload_plugin`/`hot_reload_plugin`). Deliberately does NOT clear `owners`:
    /// a hot-reload of the owner plugin re-registers the same `artifact_kinds` and re-claims
    /// ownership on its own; clearing the entry here would let whichever OTHER plugin happens to be
    /// registered next silently inherit ownership mid-reload — worse than a momentarily-stale owner
    /// pointing at a plugin with zero live surfaces (which `owned_surface_gaps` would then flag).
    pub async fn unregister_plugin(&self, plugin_id: &str) {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.dependencies.remove(plugin_id);
        for refs in state.surfaces.values_mut() {
            refs.retain(|app_ref| app_ref.plugin_id != plugin_id);
        }
        state.registered_refs.retain(|(owner, _)| owner != plugin_id);
    }

    /// 🩺️ Contract §3: every dialect with AT LEAST ONE registered surface, whose `artifact_kind` has a
    /// known owner, must resolve for BOTH roles. Pure and total, never panics — the caller decides
    /// whether to log or hard-fail. W1 logged this as a soft diagnostic while every plugin still had
    /// zero surfaces; W3 (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, now that
    /// `policySubsetSurfaceCompletenessBreaches` + the W2 scaffolder have populated all 286 real
    /// surfaces) flips `load_runtime_recursive`'s caller to hard-fail the load instead. Deliberately
    /// scoped to what the router can actually see: full taxonomy-disk subset completeness (all 143
    /// subsets, including ones with zero surfaces at all) is `policySubsetSurfaceCompletenessBreaches`'s
    /// job in `📜️script.ts`, not a host runtime concern — a wasm plugin host cannot walk the repo
    /// filesystem.
    pub async fn owned_surface_gaps(&self) -> Vec<semio_framework::Fault> {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut dialects: Vec<semio_framework::ArtifactDialect> = state.surfaces.keys().map(|(dialect, _)| dialect.clone()).collect();
        dialects.sort();
        dialects.dedup();
        let mut gaps = Vec::new();
        for dialect in dialects {
            let Some(owner) = state.owners.get(&dialect.artifact_kind) else { continue };
            for role in [semio_framework::AppRole::Viewer, semio_framework::AppRole::Editor] {
                let has_surface = state.surfaces.get(&(dialect.clone(), role)).map(|refs| !refs.is_empty()).unwrap_or(false);
                if !has_surface {
                    gaps.push(semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Framework,
                        semio_framework::FaultCode::new("surface.missing-owner-surface"),
                        format!("`{}` (owned by `{owner}`) has no registered {} surface", dialect.to_coordinate(), role.as_str()),
                    ));
                }
            }
        }
        gaps
    }
}

impl Default for AppRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod app_router_tests {
    use super::*;

    async fn fixture_artifact_kind(id: &str) -> semio_framework::ArtifactKindSpec {
        semio_framework::ArtifactKindSpec {
            id: id.into(),
            name: id.into(),
            source_format: id.into(),
            component_kind: "document".into(),
            dimension: "data".into(),
            media_capability: semio_framework::OsMediaCapability::MeshOnly,
            media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
            schema: id.into(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            export_stdio_kinds: Vec::new(),
            import_stdio_kinds: Vec::new(),
        }
    }

    pub(super) async fn fixture_app(id: &str, dialect: semio_framework::ArtifactDialect, role: semio_framework::AppRole) -> semio_framework::AppDefinition {
        semio_framework::AppDefinition {
            id: id.into(),
            role,
            dialect,
            label: ui_wgpu::wgpu::LocalizedLabel::data(id),
            breadcrumb: vec![id.into()],
            icon_id: None,
            controller_id: format!("{id}-play"),
            modes: semio_framework::Modes::one(semio_framework::ModeDefinition { id: "edit".into(), label: ui_wgpu::wgpu::LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework::WindowKinds::one(semio_framework::WindowKindDefinition {
                id: id.into(),
                label: ui_wgpu::wgpu::LocalizedLabel::data(id),
                body_key: id.into(),
                surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
                icon_id: "app-window".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: Vec::new(),
            keybindings: Vec::new(),
            utilities: Vec::new(),
            tools: Vec::new(),
            commands: Vec::new(),
            interactions: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_breadcrumbs: HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: semio_framework::ConfigSpec::empty().await,
            command_grammar: semio_framework::CommandGrammar::empty().await,
            io: semio_framework::AppIo::from_document(
                id,
                semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
                semio_framework::ArtifactPresentation { id: id.into(), name: id.into(), dimension: String::new(), component_kind: id.into() },
            )
            .await,
        }
    }

    async fn fixture_manifest(plugin_id: &str, dependency_ids: Vec<&str>, artifact_kinds: Vec<semio_framework::ArtifactKindSpec>, apps: Vec<semio_framework::AppDefinition>) -> PluginManifest {
        let mut dependencies = Vec::with_capacity(dependency_ids.len());
        for id in dependency_ids {
            dependencies.push(semio_framework::PluginDependency::new(id, semio_framework::VersionReq::Any));
        }
        PluginManifest {
            plugin_id: plugin_id.into(),
            label: plugin_id.into(),
            version: "0.1.0".into(),
            apps,
            examples: Vec::new(),
            capabilities: Vec::new(),
            topic_contributions: Vec::new(),
            commands: Vec::new(),
            artifact_kinds,
            dependencies,
            contributions: Vec::new(),
        }
    }

    pub(super) async fn dialect(subset: &str) -> semio_framework::ArtifactDialect {
        semio_framework::ArtifactDialect { artifact_kind: "s.cad.cad".into(), standard: "1".into(), subset: subset.into() }
    }

    async fn register(router: &AppRouter, plugin_id: &str, dependencies: Vec<&str>, artifact_kinds: Vec<semio_framework::ArtifactKindSpec>, apps: Vec<semio_framework::AppDefinition>) -> Result<(), semio_framework::Fault> {
        router.register_manifest(plugin_id, &fixture_manifest(plugin_id, dependencies, artifact_kinds, apps).await).await
    }

    #[semio_framework_async_macros::async_test]
    async fn owner_surface_sorts_first_then_plugin_id_then_app_id() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers");
        register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/1#editor", dialect("1").await, semio_framework::AppRole::Editor).await])
            .await
            .expect("a distinct subset's editor, contributed by a dependent, does not conflict");
        let refs = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await;
        assert_eq!(refs, vec![semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() }]);
        assert_eq!(router.owner_of("s.cad.cad").await, Some("cad".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_app_ref_is_a_conflict() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        let app = fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await;
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![app.clone()]).await.expect("first registration succeeds");
        let error = register(&router, "cad", vec![], vec![], vec![app]).await.expect_err("re-registering the same AppRef must conflict");
        assert_eq!(error.code.0, "surface.conflict");
    }

    #[semio_framework_async_macros::async_test]
    async fn contribution_without_a_declared_dependency_is_rejected() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![]).await.expect("owner claims the kind with zero apps");
        let error =
            register(&router, "norm", vec![], vec![], vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect, semio_framework::AppRole::Viewer).await]).await.expect_err("a non-owner plugin without a dependency on the owner must be rejected");
        assert_eq!(error.code.0, "surface.contribution-not-permitted");
    }

    #[semio_framework_async_macros::async_test]
    async fn contribution_with_a_declared_dependency_is_admitted_and_sorted_after_the_owner() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers its editor");
        register(&router, "norm", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect.clone(), semio_framework::AppRole::Viewer).await])
            .await
            .expect("norm depends on cad, so contributing a viewer for cad's dialect is permitted");
        let viewers = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Viewer).await;
        assert_eq!(viewers, vec![semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#viewer".into() }]);
    }

    #[semio_framework_async_macros::async_test]
    async fn owned_surface_gaps_reports_the_missing_role_only() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect, semio_framework::AppRole::Editor).await]).await.expect("owner registers only an editor");
        let gaps = router.owned_surface_gaps().await;
        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0].code.0, "surface.missing-owner-surface");
        assert!(gaps[0].message.contains("viewer"));
    }

    #[semio_framework_async_macros::async_test]
    async fn unregister_plugin_drops_its_surfaces_but_keeps_its_ownership_claim() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers");
        router.unregister_plugin("cad").await;
        assert!(router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await.is_empty(), "the surface itself is gone");
        assert_eq!(router.owner_of("s.cad.cad").await, Some("cad".to_string()), "ownership claim survives so a re-registering hot-reload reclaims it, not a stray contributor");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await])
            .await
            .expect("re-registering after unregister succeeds (no stale conflict)");
        assert_eq!(router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await, vec![semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() }]);
    }

    /// 🔗️ Lane 1-D parity reconciliation (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET,
    /// `📓️w1-d-report.md`): the SAME ordered fixture — owner surface, two contributed surfaces
    /// from different plugins, a duplicate, an unknown dialect — is asserted here AND in the TS
    /// twin (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-d-parity.ts`),
    /// which builds the identical manifests through `AppRouter.build`/`resolveOpeningApp`. Both
    /// sides must produce identical `surfaces_for` ordering and identical fault codes — run both,
    /// paste both outputs into the report, per the ticket's verification rule.
    #[semio_framework_async_macros::async_test]
    async fn w1_d_parity_fixture_owner_two_contributors_duplicate_and_unknown_dialect() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad").await], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("owner registers");
        register(&router, "norm", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-norm", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("norm depends on cad, contributes a second editor");
        register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-aec", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect("aec-building depends on cad, contributes a third editor");

        let refs = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).await;
        assert_eq!(
            refs,
            vec![
                semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() },
                semio_framework::AppRef { plugin_id: "aec-building".into(), app_id: "s.cad.cad@1/*#editor-aec".into() },
                semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#editor-norm".into() },
            ],
            "owner first, then contributors pluginId-ascending (aec-building < norm)"
        );

        let duplicate =
            register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-aec", editor_dialect.clone(), semio_framework::AppRole::Editor).await]).await.expect_err("re-registering the same AppRef must conflict");
        assert_eq!(duplicate.code.0, "surface.conflict");

        let unknown_dialect = dialect("does-not-exist").await;
        let unknown = OpeningResolver::resolve(&router, &unknown_dialect, semio_framework::AppRole::Editor, None).await.expect_err("no surface registered for this subset");
        assert_eq!(unknown.code.0, "surface.unknown-dialect");
    }
}
//#endregion 🔖️AppRouter

//#region 🔖️OpeningResolver
/// 🧭️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §3: the frozen four-step
/// precedence — (1) an explicit user default, if it is still present in the router; (2) the
/// dialect's owner plugin's surface; (3) the router's first entry; (4) `surface.unknown-dialect`.
/// Steps 2 and 3 collapse into ONE branch below by construction: `AppRouter::surfaces_for` already
/// sorts the owner's surface first (when the owner has one), so "the first entry" and "the owner's
/// surface" are the same value whenever an owner surface exists — there is no code path where they
/// would disagree. `user_default` is a single already-resolved lookup, not the whole
/// `OpeningPreferences` value: folding the config op log into that lookup
/// (`OpeningPreferences`'s `MutationDiff` impl / `apply_opening_config_mutation`, `🎚️config/🧬️schema`)
/// is the CALLER's job — this function only ever reads, never mutates, matching the ticket's "never a
/// mutable map" instruction for `prefs`.
pub struct OpeningResolver;

impl OpeningResolver {
    pub async fn resolve(router: &AppRouter, dialect: &semio_framework::ArtifactDialect, role: semio_framework::AppRole, user_default: Option<&semio_framework::AppRef>) -> Result<semio_framework::AppRef, semio_framework::Fault> {
        // 🚫️async: R10 residue shape 2 — a future is consumed by one `.await`; awaited once here
        // instead of once inside the `if let` and again (moved) at `.into_iter()` below.
        let candidates = router.surfaces_for(dialect, role).await;
        if let Some(default_ref) = user_default {
            if candidates.contains(default_ref) {
                return Ok(default_ref.clone());
            }
        }
        if let Some(first) = candidates.into_iter().next() {
            return Ok(first);
        }
        Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("surface.unknown-dialect"), format!("no {} surface registered for `{}`", role.as_str(), dialect.to_coordinate())))
    }
}

#[cfg(test)]
mod opening_resolver_tests {
    use super::app_router_tests::{dialect, fixture_app};
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn step1_explicit_default_still_in_router_wins() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        router
            .register_manifest(
                "cad",
                &PluginManifest {
                    plugin_id: "cad".into(),
                    label: "cad".into(),
                    version: "0.1.0".into(),
                    apps: vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await],
                    examples: Vec::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                    commands: Vec::new(),
                    artifact_kinds: Vec::new(),
                    dependencies: Vec::new(),
                    contributions: Vec::new(),
                },
            )
            .await
            .expect("owner registers");
        router
            .register_manifest(
                "norm",
                &PluginManifest {
                    plugin_id: "norm".into(),
                    label: "norm".into(),
                    version: "0.1.0".into(),
                    apps: vec![fixture_app("s.cad.cad@1/*#editor-alt", editor_dialect.clone(), semio_framework::AppRole::Editor).await],
                    examples: Vec::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                    commands: Vec::new(),
                    artifact_kinds: Vec::new(),
                    dependencies: vec![semio_framework::PluginDependency::new("cad", semio_framework::VersionReq::Any)],
                    contributions: Vec::new(),
                },
            )
            .await
            .expect("norm contributes a second editor for the same dialect");
        let pinned = semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#editor-alt".into() };
        let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, Some(&pinned)).await.expect("pinned default resolves");
        assert_eq!(resolved, pinned);
    }

    #[semio_framework_async_macros::async_test]
    async fn step2_and_step3_collapse_to_the_owner_surface_when_default_is_stale() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        router
            .register_manifest(
                "cad",
                &PluginManifest {
                    plugin_id: "cad".into(),
                    label: "cad".into(),
                    version: "0.1.0".into(),
                    apps: vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor).await],
                    examples: Vec::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                    commands: Vec::new(),
                    artifact_kinds: Vec::new(),
                    dependencies: Vec::new(),
                    contributions: Vec::new(),
                },
            )
            .await
            .expect("owner registers");
        let stale_default = semio_framework::AppRef { plugin_id: "gone".into(), app_id: "s.cad.cad@1/*#editor".into() };
        let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, Some(&stale_default)).await.expect("falls through to owner surface");
        assert_eq!(resolved, semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn step3_first_entry_when_the_owner_has_no_surface_for_this_role() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        router
            .register_manifest(
                "cad",
                &PluginManifest {
                    plugin_id: "cad".into(),
                    label: "cad".into(),
                    version: "0.1.0".into(),
                    apps: Vec::new(),
                    examples: Vec::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                    commands: Vec::new(),
                    artifact_kinds: Vec::new(),
                    dependencies: Vec::new(),
                    contributions: Vec::new(),
                },
            )
            .await
            .expect("owner claims nothing yet, zero apps");
        router
            .register_manifest(
                "norm",
                &PluginManifest {
                    plugin_id: "norm".into(),
                    label: "norm".into(),
                    version: "0.1.0".into(),
                    apps: vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect.clone(), semio_framework::AppRole::Viewer).await],
                    examples: Vec::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                    commands: Vec::new(),
                    artifact_kinds: Vec::new(),
                    dependencies: Vec::new(),
                    contributions: Vec::new(),
                },
            )
            .await
            .expect("s.cad.cad has no owner yet, so norm becomes it by being first to declare a surface for it");
        let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Viewer, None).await.expect("first (only) entry resolves");
        assert_eq!(resolved, semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#viewer".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn step4_unknown_dialect_when_the_router_has_nothing() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*").await;
        let error = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, None).await.expect_err("empty router must fault");
        assert_eq!(error.code.0, "surface.unknown-dialect");
    }
}
//#endregion 🔖️OpeningResolver

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️IoRouterW1d
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW `IoRouter` mechanism's own
    /// route-resolution/determinism/reentrancy tests. Pure — `resolve_io_route`/
    /// `route_reenters_calling_plugin` take a synthetic `BTreeMap<IoEntryKey, IoEntryRoute>`
    /// directly, no `PluginInstanceHandle`/real wasm component needed — so these run on every
    /// CI/dev machine, unlike `//#region 🔖️IoRouterPostTurnRelay` below. Unchanged from before the
    /// `WasmPluginRuntime` deletion — never depended on it.
    async fn io_dialect(kind: &str, standard: &str, subset: &str) -> semio_framework::io_schema::ArtifactDialect {
        semio_framework::io_schema::ArtifactDialect { artifact_kind: kind.to_string(), standard: standard.to_string(), subset: subset.to_string() }
    }

    /// 🎯️ The fixture EVERY test in this region shares — TWO mock plugins:
    /// - `"stdio"` owns `s.stdio.binary@raw/*` (the binary carrier) `->` `s.stdio.gif@87a/*` at
    ///   `Exact` fidelity, and declares a sniff.
    /// - `"gif"` owns TWO hops: `s.stdio.gif@87a/*` `->` `s.stdio.gif@89a/*` (the 87a-to-89a
    ///   migration, `Canonical` fidelity, no sniff) AND a DIRECT `s.stdio.binary@raw/*` `->`
    ///   `s.stdio.gif@89a/*` shortcut at `Lossy` fidelity (with a sniff) — a real alternate route
    ///   from the carrier straight to 89a, deliberately weaker so `route_prefers_higher_minimum_
    ///   fidelity` below has something genuine to prefer AGAINST.
    ///
    /// This is also the literal fixture `🧪️w1d-io-router-parity.ts` (this ticket's folder) builds
    /// through the TS `IoEntryGraph` — both sides must resolve `binary@raw/* -> gif@89a/*` to the
    /// SAME 2-hop route via `stdio`'s `Exact` hop then `gif`'s `Canonical` migration hop, per
    /// `📓️w1-d-report.md`.
    async fn io_router_w1d_fixture_entries() -> Vec<(&'static str, semio_framework::io_schema::IoEntryDescriptor)> {
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
        let gif_87a = io_dialect("s.stdio.gif", "87a", "*").await;
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
        vec![
            ("stdio", semio_framework::io_schema::IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true }),
            ("gif", semio_framework::io_schema::IoEntryDescriptor { from: gif_87a, into: gif_89a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Canonical, sniffs: false }),
            ("gif", semio_framework::io_schema::IoEntryDescriptor { from: binary_raw, into: gif_89a, fidelity: semio_framework::io_schema::IoFidelity::Lossy, sniffs: true }),
        ]
    }

    /// 🏗️ `IoRouter::register_plugin`'s io-entries merge, without needing a real
    /// `PluginInstanceHandle` — builds the SAME `BTreeMap<IoEntryKey, IoEntryRoute>` shape
    /// directly from `(owner, descriptor)` rows, inserted in WHATEVER order `rows` lists them.
    async fn build_io_entry_graph(rows: &[(&'static str, semio_framework::io_schema::IoEntryDescriptor)]) -> BTreeMap<IoEntryKey, IoEntryRoute> {
        let mut graph = BTreeMap::new();
        for (owner, descriptor) in rows {
            let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
            graph.entry(key).or_insert(IoEntryRoute { owner: (*owner).to_string(), fidelity: descriptor.fidelity, sniffs: descriptor.sniffs });
        }
        graph
    }

    /// 🎯️ "Register two mock plugins in both orders" — the ticket's own determinism proof
    /// requirement. `fixture()` order is `stdio, gif, gif`; `reversed` is the exact reverse. Both
    /// graphs, and both resolved routes, must be byte-identical.
    #[semio_framework_async_macros::async_test]
    async fn io_router_route_is_deterministic_across_load_order() {
        let forward = io_router_w1d_fixture_entries().await;
        let mut reversed = forward.clone();
        reversed.reverse();
        let graph_forward = build_io_entry_graph(&forward).await;
        let graph_reversed = build_io_entry_graph(&reversed).await;
        assert_eq!(graph_forward, graph_reversed, "the merged graph itself must not depend on registration order");

        let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
        let route_forward = resolve_io_route(&graph_forward, &binary_raw, &gif_89a, 3).await.expect("forward-order route resolves");
        let route_reversed = resolve_io_route(&graph_reversed, &binary_raw, &gif_89a, 3).await.expect("reversed-order route resolves");
        assert_eq!(route_forward, route_reversed, "the resolved route must not depend on registration order");
        assert_eq!(route_forward.hops.len(), 2, "the winning route is the 2-hop stdio->gif87a->gif89a path, not the 1-hop lossy shortcut");
    }

    /// ⚖️ Proves the ranking rule's FIRST tie-break: highest minimum fidelity beats fewest hops.
    /// The 1-hop `binary->gif89a` shortcut (Lossy) loses to the 2-hop `binary->gif87a->gif89a`
    /// path (min fidelity Canonical) even though it has fewer hops.
    #[semio_framework_async_macros::async_test]
    async fn io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops() {
        let graph = build_io_entry_graph(&io_router_w1d_fixture_entries().await).await;
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
        let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).await.expect("route resolves");
        assert_eq!(route.fidelity, semio_framework::io_schema::IoFidelity::Canonical);
        assert_eq!(route.hops.len(), 2);
        assert_eq!(route.hops[0].from, binary_raw);
        assert_eq!(route.hops[1].into, gif_89a);
    }

    /// 🌉️ `max_hops` bound is honored: clamped to 1, only the direct (Lossy) shortcut is reachable.
    #[semio_framework_async_macros::async_test]
    async fn io_router_route_respects_max_hops() {
        let graph = build_io_entry_graph(&io_router_w1d_fixture_entries().await).await;
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
        let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 1).await.expect("1-hop route resolves");
        assert_eq!(route.hops.len(), 1);
        assert_eq!(route.fidelity, semio_framework::io_schema::IoFidelity::Lossy);
    }

    /// 🔒️ `route_reenters_calling_plugin` — the pure predicate behind `run_io`'s guard. A route
    /// with NO hop owned by the caller is safe (`None`); a route where the caller owns even ONE
    /// hop is refused, naming that hop.
    #[semio_framework_async_macros::async_test]
    async fn io_router_run_io_reentrancy_guard_predicate() {
        let graph = build_io_entry_graph(&io_router_w1d_fixture_entries().await).await;
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
        let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).await.expect("route resolves");
        assert_eq!(route_reenters_calling_plugin(&graph, &route, "norm").await, None, "a plugin owning neither hop is safe");
        let hop = route_reenters_calling_plugin(&graph, &route, "stdio").await.expect("stdio owns the first hop of this route");
        assert_eq!(hop.0, &binary_raw);
        assert_eq!(hop.1, &io_dialect("s.stdio.gif", "87a", "*").await);
        let hop = route_reenters_calling_plugin(&graph, &route, "gif").await.expect("gif owns the second hop of this route");
        assert_eq!(hop.1, &gif_89a);
    }

    /// 🧯️ A duplicate `(from, into)` claimed by a DIFFERENT plugin than the first registration is a
    /// typed conflict — mirrors `io::io_mechanism`'s own `duplicate_entry_is_a_typed_error` law for
    /// the OLD graph's `IoRouteConflict`, generalized to the NEW `IoEntryRouteConflict`. Exercises
    /// `io_entries_conflict` directly — the SAME function `register_plugin` calls — so this proves
    /// the real preflight rule, not a re-derivation of it, without needing a live wasm component.
    #[semio_framework_async_macros::async_test]
    async fn io_router_register_plugin_rejects_conflicting_io_entry_ownership() {
        let graph = build_io_entry_graph(&[(
            "stdio",
            semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true },
        )])
        .await;

        let same_plugin_reclaim =
            vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true }];
        assert!(io_entries_conflict(&graph, "stdio", &same_plugin_reclaim).await.is_none(), "the SAME plugin reclaiming its own key must not conflict");

        let different_plugin_claim =
            vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Lossy, sniffs: false }];
        let conflict = io_entries_conflict(&graph, "gif", &different_plugin_claim).await.expect("a second plugin claiming the same key must conflict");
        assert!(matches!(conflict, PluginHostError::IoEntryRouteConflict { ref existing_plugin, ref incoming_plugin, .. } if existing_plugin == "stdio" && incoming_plugin == "gif"));
    }
    //#endregion 🔖️IoRouterW1d

    //#region 🔖️IoRouterPostTurnRelay
    /// 🧬️ B1b: real-wasm-component coverage of `WasmPluginRuntime`'s deletion — same convention
    /// `wasmtime_runtime_tests` already established (`compile` succeeds against a real, valid,
    /// pre-migration component; `instantiate` against `world actor` correctly REJECTS it, since no
    /// `.wasm` in this repo exports `world actor` yet — A2's guest SDK migration landed the WIT/SDK
    /// side but no PLUGIN has rebuilt onto it, that is W3's job per `📌️important.md`'s sequencing
    /// section). This is the honest replacement for the old
    /// `wasm_plugin_runtime_loads_real_plugin_component_if_present` test, which asserted the
    /// opposite (a real load SUCCEEDING) against the OLD `plugin-world` ABI that no longer exists.
    #[semio_framework_async_macros::async_test]
    async fn wasmtime_runtime_compiles_real_stdio_and_cad_but_neither_exports_actor_yet() {
        let stdio_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        let cad_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/cad/semio_s_plugin_cad_component.core.wasm");
        if !stdio_path.exists() || !cad_path.exists() {
            // 🧊️ Same convention as every other real-component test in this file: stays green on a
            // fresh clone / no-wasm-toolchain CI run.
            return;
        }
        let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
        let budget = Budget { fuel: 1_000_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        for (name, path) in [("stdio", stdio_path), ("cad", cad_path)] {
            let bytes = std::fs::read(path).unwrap_or_else(|error| panic!("read real {name}.wasm: {error}"));
            let package = PackageRef { package: PackageId(name.to_string()), hash: PackageHash([name.len() as u8; 32]) };
            let compiled = runtime.compile(&package, &bytes).await.unwrap_or_else(|error| panic!("real {name}.wasm compiles as a component: {error}"));
            let error = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).await.expect_err(&format!("{name}.wasm does not export `reactor`/`jobs`/`checkpoint`/`describe` yet"));
            let _ = error;
        }
    }

    /// 🎬️ `PluginInstanceHandle`'s `run_job_on_worker` (`start-job`, then one `step-job` per pool turn)
    /// against `MockGuestRuntime` — the FIRST real coverage of the post-turn job-dispatch mechanism
    /// itself (design-runtime.md §2), independent of whether any real `.wasm` exports `world actor`
    /// yet. `step_job`'s FIRST scripted outcome is `Running`, forcing the persistent relay to
    /// resubmit before the scripted `Done` — proves this is not a single batch call.
    #[semio_framework_async_macros::async_test]
    async fn plugin_instance_handle_drives_io_run_job_on_worker_through_a_running_step() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(100);
        let compiled = mock.compile(&PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([1u8; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        mock.script_job_step(actor, JobStep::Running { progress: None }).await;
        let io_payload = semio_framework::io_schema::IoPayload::Text("87a-bytes".to_string());
        mock.script_job_step(actor, JobStep::Done { output: dsl::os_pack::json::to_json_string(&io_payload).into_bytes() }).await;
        let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

        let payload_bytes = dsl::os_pack::json::to_json_string(&semio_framework::io_schema::IoPayload::Text("raw-bytes".to_string())).into_bytes();
        let result = handle.io_run("s.stdio.gif@87a/*", "s.stdio.gif@89a/*", payload_bytes).await.expect("job-backed io_run must drive start-job + step-job to Done");
        let decoded: semio_framework::io_schema::IoPayload = dsl::os_pack::json::from_json_str(std::str::from_utf8(&result).expect("decode io_run result utf8")).expect("decode io_run result");
        assert_eq!(decoded, io_payload);
    }

    /// 🔍️ `PluginInstanceHandle::io_sniff` decodes the single confidence-rank byte `semio.io-sniff`
    /// returns — mirrors the deleted `WasmPluginRuntime::io_sniff`'s exact return contract.
    #[semio_framework_async_macros::async_test]
    async fn plugin_instance_handle_io_sniff_decodes_the_confidence_byte() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(101);
        let compiled = mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([2u8; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        mock.script_job_step(actor, JobStep::Done { output: vec![3u8] }).await;
        let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

        let payload_bytes = dsl::os_pack::json::to_json_string(&semio_framework::io_schema::IoPayload::Binary(vec![0xFF])).into_bytes();
        let rank = handle.io_sniff("s.stdio.binary@raw/*", "s.stdio.gif@87a/*", &payload_bytes).await.expect("job-backed io_sniff must decode a Done result");
        assert_eq!(rank, 3);
    }

    /// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `PluginInstanceHandle::migrate`
    /// drives `semio.migrate` through a `Running` slice before `Done`, exactly like the `io_run`
    /// test above — proves this is a real `start-job`/`step-job` loop, not a single call.
    #[semio_framework_async_macros::async_test]
    async fn plugin_instance_handle_migrate_drives_the_semio_migrate_job_to_completion() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(102);
        let compiled = mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([9u8; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        mock.script_job_step(actor, JobStep::Running { progress: None }).await;
        mock.script_job_step(actor, JobStep::Done { output: vec![1, 2, 3, 0xAB] }).await;
        let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

        let result = handle.migrate("s.stdio.gif@87a/*", "s.stdio.gif@89a/*", vec![1, 2, 3]).await.expect("job-backed migrate must drive start-job + step-job to Done");
        assert_eq!(result, vec![1, 2, 3, 0xAB]);
    }

    /// 🧬️ `PluginInstanceHandle::mutation_plan` passes DSL wire-pack bytes straight through, both
    /// directions — no re-encoding at this layer (that's `ArtifactMutationRouter::plan`'s job).
    #[semio_framework_async_macros::async_test]
    async fn plugin_instance_handle_mutation_plan_passes_wire_bytes_through_to_done() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(103);
        let compiled = mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([10u8; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        mock.script_job_step(actor, JobStep::Done { output: b"planned".to_vec() }).await;
        let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

        let result = handle.mutation_plan(b"request-wire-bytes").await.expect("job-backed mutation_plan must drive start-job + step-job to Done");
        assert_eq!(result, b"planned");
    }

    /// 🌉️ End-to-end through the REAL `IoRouter`, not just `PluginInstanceHandle` directly: registers
    /// two mock-backed plugins (`stdio` owns `binary->gif87a`, `gif` owns `gif87a->gif89a`) using the
    /// SAME `register_plugin`/`run_io` production code path a live `🏃️run` boot would use, then
    /// drives a real 2-hop `run_io` call where each hop is answered by a DIFFERENT `PluginInstanceHandle`
    /// backed by a DIFFERENT `MockGuestRuntime` instance (proving hop-to-hop chaining actually crosses
    /// plugin boundaries, not just calls the same handle twice). This is the direct replacement for
    /// the deleted `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins` —
    /// same shape (register two plugins into one shared router, route a call that can only be
    /// answered by crossing into the OTHER plugin's instance), narrowed to the NEW `io_entries`
    /// mechanism (`run_io`, which has a real `jobs.wit` job kind) rather than the OLD `IoKey`-keyed
    /// `compose` (which does not — see `IoRouter::compose`'s own doc comment on why that dispatch is
    /// not yet wired).
    #[semio_framework_async_macros::async_test]
    async fn io_router_run_io_crosses_two_real_plugin_instance_handles() {
        let router = IoRouter::new();
        let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };

        let stdio_mock = Arc::new(MockGuestRuntime::new().await);
        let stdio_actor = RuntimeActorId(200);
        let stdio_compiled = stdio_mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([3u8; 32]) }, &[]).await.expect("stdio mock compile");
        let stdio_instance = stdio_mock.instantiate(&stdio_compiled, stdio_actor, &[], &budget).await.expect("stdio mock instantiate");
        let midpoint = semio_framework::io_schema::IoPayload::Text("midpoint".to_string());
        stdio_mock.script_job_step(stdio_actor, JobStep::Done { output: dsl::os_pack::json::to_json_string(&midpoint).into_bytes() }).await;
        let stdio_handle = Arc::new(PluginInstanceHandle::new(stdio_actor, Arc::new(GuestRuntimes::Mock(stdio_mock)), stdio_instance).await);

        let gif_mock = Arc::new(MockGuestRuntime::new().await);
        let gif_actor = RuntimeActorId(201);
        let gif_compiled = gif_mock.compile(&PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([4u8; 32]) }, &[]).await.expect("gif mock compile");
        let gif_instance = gif_mock.instantiate(&gif_compiled, gif_actor, &[], &budget).await.expect("gif mock instantiate");
        let final_payload = semio_framework::io_schema::IoPayload::Text("final".to_string());
        gif_mock.script_job_step(gif_actor, JobStep::Done { output: dsl::os_pack::json::to_json_string(&final_payload).into_bytes() }).await;
        let gif_handle = Arc::new(PluginInstanceHandle::new(gif_actor, Arc::new(GuestRuntimes::Mock(gif_mock)), gif_instance).await);

        let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
        let gif_87a = io_dialect("s.stdio.gif", "87a", "*").await;
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
        router
            .register_plugin("stdio", stdio_handle, &[], &[semio_framework::io_schema::IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: false }])
            .await
            .expect("register stdio");
        router
            .register_plugin("gif", gif_handle, &[], &[semio_framework::io_schema::IoEntryDescriptor { from: gif_87a, into: gif_89a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Canonical, sniffs: false }])
            .await
            .expect("register gif");

        let (plugins, _keys) = router.stats().await.expect("router stats");
        assert_eq!(plugins, 2, "both plugin instance handles must be registered with the shared router");

        let start_payload = dsl::os_pack::json::to_json_string(&semio_framework::io_schema::IoPayload::Text("start".to_string())).into_bytes();
        let result_bytes = router.run_io("norm", &binary_raw.to_coordinate(), &gif_89a.to_coordinate(), start_payload).await.expect("2-hop run_io crossing stdio then gif must succeed");
        let decoded: semio_framework::io_schema::IoPayload = dsl::os_pack::json::from_json_str(std::str::from_utf8(&result_bytes).expect("decode final run_io result utf8")).expect("decode final run_io result");
        assert_eq!(decoded, final_payload, "the SECOND hop's (gif's) scripted result must be what comes out — proves the chain really crossed both instance handles in order");
    }

    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `IoRouter::compose` resolution
    /// (unchanged pure algorithm) now feeds a REAL dispatch through `PluginInstanceHandle::compose`
    /// — replaces the retired `io_router_compose_resolves_ownership_but_dispatch_is_not_yet_wired`,
    /// which pinned down the OLD hand-written host refusal this packet deleted. `MockGuestRuntime`'s
    /// `start_job`/`step_job` don't inspect `kind` at all (only whatever is scripted), so scripting a
    /// `Done` here proves the host-side plumbing — resolve ownership, find the handle,
    /// `start-job`/`step-job` to completion — is fully real end to end; the real guest kind
    /// `"semio.compose"` itself is `compose-await`'s to register.
    #[semio_framework_async_macros::async_test]
    async fn io_router_compose_resolves_ownership_and_drives_the_semio_compose_job_to_completion() {
        let router = IoRouter::new();
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(202);
        let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let compiled = mock.compile(&PackageRef { package: PackageId("cad".to_string()), hash: PackageHash([5u8; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &budget).await.expect("mock instantiate");
        mock.script_job_step(actor, JobStep::Running { progress: None }).await;
        mock.script_job_step(actor, JobStep::Done { output: b"composed".to_vec() }).await;
        let handle = Arc::new(PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock)), instance).await);

        let dialects = vec![(
            semio_framework::ArtifactDialect { artifact_kind: "s.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() },
            vec![semio_framework::ArtifactDialect { artifact_kind: "s.stdio.step".to_string(), standard: "ap214".to_string(), subset: "*".to_string() }],
        )];
        router.register_plugin("cad", handle, &dialects, &[]).await.expect("register cad");

        // 🧭️ Key orientation matches what `register_plugin` actually derives from a `(writes, reads)`
        // pair: the Export route is keyed on the READ dialect with the WRITE dialect as its format
        // (see the `candidate_routes` loop above). Asserting the inverse orientation here would fail
        // on route resolution before ever reaching dispatch.
        let key = semio_framework::IoKey {
            artifact_kind: "s.stdio.step".to_string(),
            standard: "ap214".to_string(),
            subset: "*".to_string(),
            direction: semio_framework::IoDirection::Export,
            format_kind: "s.cad".to_string(),
            format_standard: "1".to_string(),
            format_subset: "*".to_string(),
        };
        let key_bytes = dsl::os_pack::json::to_json_string(&key).into_bytes();
        let result = router.compose("stdio", &key_bytes, b"sources").await.expect("compose must resolve ownership AND drive the job to completion, not hard-error");
        assert_eq!(result, b"composed", "the SCRIPTED job outcome must be what comes out, proving real start-job/step-job dispatch reached the resolved owner's handle");
    }

    /// 🧬️ `IoRouter::compose` still refuses to route back into the calling plugin itself — that
    /// guard runs BEFORE dispatch, so it must fire even though dispatch is now real.
    #[semio_framework_async_macros::async_test]
    async fn io_router_compose_still_refuses_to_route_back_into_the_calling_plugin() {
        let router = IoRouter::new();
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = RuntimeActorId(203);
        let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let compiled = mock.compile(&PackageRef { package: PackageId("cad".to_string()), hash: PackageHash([6u8; 32]) }, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &budget).await.expect("mock instantiate");
        let handle = Arc::new(PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock)), instance).await);

        let dialects = vec![(
            semio_framework::ArtifactDialect { artifact_kind: "s.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() },
            vec![semio_framework::ArtifactDialect { artifact_kind: "s.stdio.step".to_string(), standard: "ap214".to_string(), subset: "*".to_string() }],
        )];
        router.register_plugin("cad", handle, &dialects, &[]).await.expect("register cad");

        let key = semio_framework::IoKey {
            artifact_kind: "s.stdio.step".to_string(),
            standard: "ap214".to_string(),
            subset: "*".to_string(),
            direction: semio_framework::IoDirection::Export,
            format_kind: "s.cad".to_string(),
            format_standard: "1".to_string(),
            format_subset: "*".to_string(),
        };
        let key_bytes = dsl::os_pack::json::to_json_string(&key).into_bytes();
        let error = router.compose("cad", &key_bytes, b"sources").await.expect_err("a plugin routing to its own key must be refused, not dispatched");
        assert!(error.to_string().contains("routing to itself"), "unexpected message: {error}");
    }
    //#endregion 🔖️IoRouterPostTurnRelay
}
