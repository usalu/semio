//! 🛡️ Sandboxed wasmtime component plugin host with capability-gated imports.

use semio_framework::{
    kernel::{
        ArtifactHandle, ArtifactKind, BrokerCapabilityGrant, Budget, CapabilityId, CapabilityRequest, CapabilityRequirement, Effect, Event, JobPlacement, MessageEndpoint, RequestId,
        RequestOutcome, Rights, Scope, TurnResult, TurnStatus, WindowHandle, WindowKindId,
    },
    DslValue, PluginManifest, TopicContribution, ViewModel,
};
use semio_framework_actor::{ActorId as RuntimeActorId, PackageHash, PackageId};
use std::collections::{BTreeMap, BTreeSet, HashMap};
#[cfg(test)]
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use ui_wgpu::wgpu::{UtilityNode, WindowEngagement, WindowMeasure};
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig, ResourceLimiter, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

const PLUGIN_FUEL_BUDGET: u64 = 50_000_000;

/// 🧯️ Errors from `WasmPluginRuntime`'s own engine/component/call-boundary plumbing. The
/// `impl semio::framework::host::Host for HostState` block encodes {@link Fault} bytes on the
/// wasm component ABI (`result<_, list<u8>>` in `framework/wit/📜️world.wit`).
#[derive(Debug, thiserror::Error)]
pub enum PluginHostError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("wasmtime: {0}")]
    Wasmtime(String),
    #[error("plugin: {0}")]
    Plugin(String),
    #[error("io route conflict for {key:?}: {existing_plugin} already owns it; {incoming_plugin} cannot replace it")]
    IoRouteConflict { key: semio_framework::IoKey, existing_plugin: String, incoming_plugin: String },
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW `(ArtifactDialect,
    /// ArtifactDialect)`-keyed graph's own conflict — separate from `IoRouteConflict` above (OLD
    /// `IoKey`-keyed graph), since the two mechanisms are additive and independently registered.
    #[error("io entry route conflict for {from:?} -> {into:?}: {existing_plugin} already owns it; {incoming_plugin} cannot replace it")]
    IoEntryRouteConflict { from: semio_framework::io_schema::ArtifactDialect, into: semio_framework::io_schema::ArtifactDialect, existing_plugin: String, incoming_plugin: String },
    #[error("plugin runtime conflict for {plugin_id}")]
    PluginRuntimeConflict { plugin_id: String },
    #[error("{0} lock poisoned")]
    LockPoisoned(&'static str),
}

//#endregion ⚠️ Errors

//#region 🔧️SharedWasmtimeEngine
/// 🚧️ Staged for packet `B1-host-native` (`📓️design-runtime.md` §2, `//#region 🦀️WasmtimeRuntime`).
/// The blocked pieces — the `GuestRuntime` trait, the `WasmtimeRuntime` impl that closes over it,
/// `ShardLoop`, `MockGuestRuntime`, and the deletion of `WasmPluginRuntime`/`ExtensionRuntime`/both
/// `ProgramSupervisorState`s/`PLUGIN_FUEL_BUDGET` — all need `RuntimeActorId` (packet `A1-actor`,
/// crate `semio-framework-actor`, not yet a workspace member) and `Effect`/`Event`/`Budget`/
/// `TurnResult`/`JobBudget`/`JobStep`/`TurnFault` (packet `A3-kernel-types`, not yet landed in
/// `🎠️kernel/🦀️component.rs`). This region holds only the four pieces of §2 that do NOT depend on
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
pub fn build_shared_engine(cfg: SharedEngineConfig) -> Result<(Engine, bool), PluginHostError> {
    let build = |pooling: bool| -> wasmtime::Result<Engine> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.consume_fuel(true);
        config.epoch_interruption(true);
        if pooling {
            let mut pooling_cfg = PoolingAllocationConfig::default();
            pooling_cfg.total_component_instances(cfg.total_component_instances);
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

/// ⏱️ Ticks `engine.increment_epoch()` every 1 ms on a dedicated thread (§2), so
/// `Store::set_epoch_deadline(budget.wall_ms)` is actually enforced — wasmtime's epoch counter never
/// advances on its own. One ticker per shared `Engine`; `Drop` stops and joins the thread.
pub struct EpochTicker {
    stop: Arc<std::sync::atomic::AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl EpochTicker {
    pub fn start(engine: &Engine) -> Self {
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop_thread = Arc::clone(&stop);
        let engine = engine.clone();
        let handle = std::thread::Builder::new()
            .name("semio-epoch-ticker".to_string())
            .spawn(move || {
                while !stop_thread.load(std::sync::atomic::Ordering::Relaxed) {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    engine.increment_epoch();
                }
            })
            .expect("spawn epoch ticker thread");
        Self { stop, handle: Some(handle) }
    }
}

impl Drop for EpochTicker {
    fn drop(&mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
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

impl Default for BudgetLimiter {
    fn default() -> Self {
        Self { max_memory_bytes: 512 * 1024 * 1024, max_table_elements: 100_000, max_instances: 1, max_tables: 8, max_memories: 8 }
    }
}

impl ResourceLimiter for BudgetLimiter {
    fn memory_growing(&mut self, _current: usize, desired: usize, _maximum: Option<usize>) -> wasmtime::Result<bool> {
        Ok(desired <= self.max_memory_bytes)
    }

    fn table_growing(&mut self, _current: u32, desired: u32, _maximum: Option<u32>) -> wasmtime::Result<bool> {
        Ok(desired <= self.max_table_elements)
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
pub fn default_compiled_cache_root() -> PathBuf {
    let home = std::env::var("SEMIO_HOME").or_else(|_| std::env::var("HOME")).or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".semio").join("cache").join("wasmtime")
}

pub fn shared_engine_config_hash(cfg: &SharedEngineConfig, pooling_active: bool) -> [u8; 32] {
    let descriptor = format!(
        "wasmtime=22.0.1;component_model=1;fuel=1;epoch=1;pooling={};instances={};max_memory={};keep_resident={}",
        pooling_active, cfg.total_component_instances, cfg.max_memory_bytes, cfg.linear_memory_keep_resident_bytes
    );
    *blake3::hash(descriptor.as_bytes()).as_bytes()
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn compiled_cache_path(cache_root: &Path, engine_config_hash: &[u8; 32], package_hash: &[u8; 32]) -> PathBuf {
    cache_root.join(hex_encode(engine_config_hash)).join(format!("{}.cwasm", hex_encode(package_hash)))
}

/// ⚠️ SAFETY: `deserialize_file` trusts the file completely (wasmtime docs). Callers MUST only point
/// this at paths this process itself wrote via [`store_compiled_component`] with the SAME `engine`
/// (same config, so same compiled ABI) — a hostile or stale `.cwasm` is a sandbox escape, not a
/// cache-miss. Any I/O or deserialize error is treated as a cache miss (`None`), never surfaced as a
/// fault: recompiling from the original component bytes is always the safe fallback.
pub fn load_compiled_component(engine: &Engine, path: &Path) -> Option<Component> {
    if !path.exists() {
        return None;
    }
    unsafe { Component::deserialize_file(engine, path).ok() }
}

pub fn store_compiled_component(component: &Component, path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let bytes = component.serialize().map_err(|error| std::io::Error::other(error.to_string()))?;
    std::fs::write(path, bytes)
}

#[cfg(test)]
mod shared_wasmtime_engine_tests {
    use super::*;

    #[test]
    fn build_shared_engine_defaults_to_pooling() {
        let (_engine, pooling_active) = build_shared_engine(SharedEngineConfig::default()).expect("pooling engine builds on this host");
        assert!(pooling_active, "pooling allocator should be available in test/CI containers");
    }

    #[test]
    fn build_shared_engine_forced_on_demand_reports_on_demand() {
        let cfg = SharedEngineConfig { force_on_demand: true, ..SharedEngineConfig::default() };
        let (_engine, pooling_active) = build_shared_engine(cfg).expect("on-demand engine always builds");
        assert!(!pooling_active);
    }

    #[test]
    fn epoch_ticker_starts_and_stops_cleanly_around_a_deadline_bearing_store() {
        let (engine, _pooling_active) = build_shared_engine(SharedEngineConfig::default()).expect("engine builds");
        let mut store = Store::new(&engine, ());
        store.set_epoch_deadline(1);
        store.set_fuel(1_000).expect("consume_fuel is enabled on the shared engine");
        let ticker = EpochTicker::start(&engine);
        std::thread::sleep(std::time::Duration::from_millis(10));
        drop(ticker);
    }

    #[test]
    fn shared_engine_config_hash_is_deterministic_and_config_sensitive() {
        let cfg = SharedEngineConfig::default();
        let a = shared_engine_config_hash(&cfg, true);
        let b = shared_engine_config_hash(&cfg, true);
        assert_eq!(a, b);
        let c = shared_engine_config_hash(&cfg, false);
        assert_ne!(a, c, "pooling vs on-demand must be different cache namespaces");
    }

    #[test]
    fn compiled_cache_path_is_namespaced_by_both_hashes() {
        let root = Path::new("/tmp/semio-cache-test");
        let engine_hash = [1u8; 32];
        let package_hash = [2u8; 32];
        let path = compiled_cache_path(root, &engine_hash, &package_hash);
        assert!(path.starts_with(root));
        assert!(path.to_string_lossy().ends_with(&format!("{}.cwasm", hex_encode(&package_hash))));
    }

    #[test]
    fn compiled_component_round_trips_through_cache_for_a_real_wasm_file() {
        let wasm_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        if !wasm_path.exists() {
            return;
        }
        let (engine, _pooling_active) = build_shared_engine(SharedEngineConfig::default()).expect("engine builds");
        let wasm_bytes = std::fs::read(wasm_path).expect("read real stdio.wasm");
        let component = Component::from_binary(&engine, &wasm_bytes).expect("compile real stdio.wasm as a component");
        let cache_dir = std::env::temp_dir().join(format!("semio-compiled-cache-test-{}", std::process::id()));
        let cache_path = compiled_cache_path(&cache_dir, &shared_engine_config_hash(&SharedEngineConfig::default(), true), &[3u8; 32]);
        assert!(load_compiled_component(&engine, &cache_path).is_none(), "cache must start empty");
        store_compiled_component(&component, &cache_path).expect("write compiled cache entry");
        let restored = load_compiled_component(&engine, &cache_path).expect("cache hit after writing");
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
}

impl std::fmt::Debug for CompiledHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledHandle").field("package_hash", &hex_encode(&self.package_hash)).field("has_component", &self.component.is_some()).finish()
    }
}

/// 🏃️ One running actor instance, host-owned. `Mock(..)` backs [`MockGuestRuntime`]; `Wasmtime(..)`
/// backs [`WasmtimeRuntime`] (`//#region 🐎️WasmtimeRuntime` below).
pub struct GuestInstance {
    pub actor: RuntimeActorId,
    state: GuestInstanceState,
}

enum GuestInstanceState {
    Mock(MockInstanceState),
    Wasmtime(WasmtimeInstanceState),
}

/// ⛽️ `jobs.wit`'s `job-budget` record, mirrored field-for-field (design-abi.md §1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobBudget {
    pub fuel: u64,
    pub deadline_ms: u32,
}

/// 🪜️ `jobs.wit`'s `job-step` variant, mirrored field-for-field.
#[derive(Clone, Debug, PartialEq)]
pub enum JobStep {
    Running(Option<Vec<u8>>),
    Done(Vec<u8>),
    Failed(Vec<u8>),
}

/// 🧯️ Why a turn or job step didn't produce a result — distinct from [`PluginHostError`] (host-side
/// plumbing faults): every `GuestRuntime::execute_turn`/`step_job` failure is one of these.
#[derive(Debug, thiserror::Error)]
pub enum TurnFault {
    #[error(transparent)]
    Host(#[from] PluginHostError),
    #[error("guest instance has no more scripted/actual turns")]
    Exhausted,
    #[error("guest trapped: {0}")]
    Trapped(String),
    #[error("epoch deadline exceeded")]
    DeadlineExceeded,
    #[error("fuel exhausted")]
    FuelExhausted,
}

/// 🐎️ Host-side driver for one actor's execution — `design-runtime.md` §2. `WasmtimeRuntime` (the
/// native implementation, backed by [`build_shared_engine`]/[`EpochTicker`]/[`BudgetLimiter`]/the
/// compiled-artifact cache above) and `MockGuestRuntime` (test double, below) both implement this;
/// nothing else in the host — `ShardLoop`, the task manager, `WasmtimeNodeHost` — talks to a guest
/// through any other surface.
pub trait GuestRuntime: Send + Sync {
    fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError>;
    fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError>;
    fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault>;
    fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault>;
    fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError>;
    fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError>;
    fn drop_instance(&self, inst: GuestInstance);
}

//#region 🔖️MockGuestRuntime
/// 🎬️ One actor's scripted future: either the next `execute_turn`/`step_job` call returns this
/// `TurnResult`/`JobStep`, or the runtime raises this fault instead.
enum ScriptedOutcome {
    Turn(TurnResult),
    Job(JobStep),
    Fault(String),
}

#[derive(Default)]
struct MockInstanceState {
    checkpoint: Option<Vec<u8>>,
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
}

#[cfg(test)]
impl Default for MockGuestRuntime {
    fn default() -> Self {
        Self { now_ms: std::sync::atomic::AtomicI64::new(0), scripts: Mutex::new(HashMap::new()) }
    }
}

#[cfg(test)]
impl MockGuestRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn now_ms(&self) -> i64 {
        self.now_ms.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn set_now_ms(&self, ms: i64) {
        self.now_ms.store(ms, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn advance_ms(&self, delta: i64) {
        self.now_ms.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
    }

    fn queue_for(&self, actor: RuntimeActorId) -> std::sync::MutexGuard<'_, HashMap<u64, VecDeque<ScriptedOutcome>>> {
        let mut scripts = self.scripts.lock().expect("mock runtime lock poisoned");
        scripts.entry(actor.0).or_default();
        scripts
    }

    pub fn script_turn(&self, actor: RuntimeActorId, result: TurnResult) {
        self.queue_for(actor).get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Turn(result));
    }

    pub fn script_job_step(&self, actor: RuntimeActorId, step: JobStep) {
        self.queue_for(actor).get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Job(step));
    }

    pub fn script_fault(&self, actor: RuntimeActorId, message: impl Into<String>) {
        self.queue_for(actor).get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Fault(message.into()));
    }

    /// 🏁️ A plain `Idle`, no-effects, no-patches turn result — convenience for tests that only
    /// care about scheduling/backpressure, not turn content.
    pub fn idle_turn() -> TurnResult {
        TurnResult { ui_patches: Vec::new(), effects: Vec::new(), next_wake: None, status: TurnStatus::Idle, fuel_used: 0 }
    }
}

#[cfg(test)]
impl GuestRuntime for MockGuestRuntime {
    fn compile(&self, package: &PackageRef, _bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        Ok(CompiledHandle { package_hash: package.hash.0, component: None })
    }

    fn instantiate(&self, _compiled: &CompiledHandle, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        self.queue_for(actor);
        Ok(GuestInstance { actor, state: GuestInstanceState::Mock(MockInstanceState::default()) })
    }

    fn execute_turn(&self, inst: &mut GuestInstance, _events: &[Event], _budget: Budget) -> Result<TurnResult, TurnFault> {
        let mut scripts = self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?;
        let queue = scripts.entry(inst.actor.0).or_default();
        match queue.pop_front() {
            Some(ScriptedOutcome::Turn(result)) => Ok(result),
            Some(ScriptedOutcome::Job(_)) => Err(TurnFault::Trapped("scripted outcome was a job step, not a turn".to_string())),
            Some(ScriptedOutcome::Fault(message)) => Err(TurnFault::Trapped(message)),
            None => Err(TurnFault::Exhausted),
        }
    }

    fn step_job(&self, inst: &mut GuestInstance, _job: u64, _budget: JobBudget) -> Result<JobStep, TurnFault> {
        let mut scripts = self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?;
        let queue = scripts.entry(inst.actor.0).or_default();
        match queue.pop_front() {
            Some(ScriptedOutcome::Job(step)) => Ok(step),
            Some(ScriptedOutcome::Turn(_)) => Err(TurnFault::Trapped("scripted outcome was a turn, not a job step".to_string())),
            Some(ScriptedOutcome::Fault(message)) => Err(TurnFault::Trapped(message)),
            None => Err(TurnFault::Exhausted),
        }
    }

    fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Mock(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("MockGuestRuntime::checkpoint called on a non-mock GuestInstance".to_string()));
        };
        let bytes = format!("mock-checkpoint:{}", inst.actor.0).into_bytes();
        state.checkpoint = Some(bytes.clone());
        Ok(bytes)
    }

    fn restore(&self, inst: &mut GuestInstance, state: &[u8]) -> Result<(), PluginHostError> {
        let GuestInstanceState::Mock(mock_state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("MockGuestRuntime::restore called on a non-mock GuestInstance".to_string()));
        };
        mock_state.checkpoint = Some(state.to_vec());
        Ok(())
    }

    fn drop_instance(&self, inst: GuestInstance) {
        self.scripts.lock().map(|mut scripts| scripts.remove(&inst.actor.0)).ok();
    }
}

#[cfg(test)]
mod mock_guest_runtime_tests {
    use super::*;

    fn hash(byte: u8) -> PackageHash {
        PackageHash([byte; 32])
    }

    #[test]
    fn scripted_turn_is_returned_exactly_once_fifo() {
        let runtime = MockGuestRuntime::new();
        let compiled = runtime.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: hash(1) }, &[]).expect("compile");
        let actor = RuntimeActorId(42);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("instantiate");

        let mut first = MockGuestRuntime::idle_turn();
        first.fuel_used = 7;
        let mut second = MockGuestRuntime::idle_turn();
        second.fuel_used = 9;
        runtime.script_turn(actor, first);
        runtime.script_turn(actor, second);

        let got_first = runtime.execute_turn(&mut inst, &[], Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("first scripted turn");
        assert_eq!(got_first.fuel_used, 7);
        let got_second = runtime.execute_turn(&mut inst, &[], Budget { fuel: 1000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("second scripted turn");
        assert_eq!(got_second.fuel_used, 9);
    }

    #[test]
    fn exhausted_script_queue_is_a_loud_error_not_a_fabricated_idle_turn() {
        let runtime = MockGuestRuntime::new();
        let compiled = runtime.compile(&PackageRef { package: PackageId("cad".to_string()), hash: hash(2) }, &[]).expect("compile");
        let actor = RuntimeActorId(7);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("instantiate");
        let error = runtime.execute_turn(&mut inst, &[], Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect_err("no script queued");
        assert!(matches!(error, TurnFault::Exhausted));
    }

    #[test]
    fn scripted_fault_surfaces_as_trapped() {
        let runtime = MockGuestRuntime::new();
        let compiled = runtime.compile(&PackageRef { package: PackageId("block".to_string()), hash: hash(3) }, &[]).expect("compile");
        let actor = RuntimeActorId(9);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("instantiate");
        runtime.script_fault(actor, "epoch deadline exceeded");
        let error = runtime.execute_turn(&mut inst, &[], Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect_err("scripted fault");
        assert!(matches!(error, TurnFault::Trapped(message) if message == "epoch deadline exceeded"));
    }

    #[test]
    fn checkpoint_then_restore_round_trips_through_a_fresh_instance() {
        let runtime = MockGuestRuntime::new();
        let compiled = runtime.compile(&PackageRef { package: PackageId("puzzle".to_string()), hash: hash(4) }, &[]).expect("compile");
        let actor = RuntimeActorId(11);
        let mut inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("instantiate");
        let snapshot = runtime.checkpoint(&mut inst).expect("checkpoint");

        let mut restored = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("re-instantiate");
        runtime.restore(&mut restored, &snapshot).expect("restore");
        let GuestInstanceState::Mock(state) = &restored.state else { panic!("expected a Mock instance") };
        assert_eq!(state.checkpoint.as_deref(), Some(snapshot.as_slice()));
    }

    #[test]
    fn controllable_clock_advances_deterministically() {
        let runtime = MockGuestRuntime::new();
        runtime.set_now_ms(1_000);
        assert_eq!(runtime.now_ms(), 1_000);
        runtime.advance_ms(250);
        assert_eq!(runtime.now_ms(), 1_250);
    }

    #[test]
    fn drop_instance_forgets_the_actors_script_queue() {
        let runtime = MockGuestRuntime::new();
        let compiled = runtime.compile(&PackageRef { package: PackageId("layout".to_string()), hash: hash(5) }, &[]).expect("compile");
        let actor = RuntimeActorId(13);
        let inst = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("instantiate");
        runtime.script_turn(actor, MockGuestRuntime::idle_turn());
        runtime.drop_instance(inst);
        assert!(!runtime.scripts.lock().expect("lock").contains_key(&actor.0));
    }
}
//#endregion 🔖️MockGuestRuntime

//#region 🐎️WasmtimeRuntime
/// 🧬️ The real `impl GuestRuntime for WasmtimeRuntime` — `design-runtime.md` §2. Nested `mod
/// actor_bindings` (mirrors the file's own `mod extension_bindings` idiom below, "wasmtime's
/// `bindgen!` cannot be invoked twice at the same module scope") so this coexists with the OLD
/// `plugin-world`/`extension-world` `bindgen!` calls until the deletion pass — both of which are
/// now unconditionally broken independent of this packet (their `world` names no longer exist in
/// `📜️world.wit`, which declares only `world actor`), a finding recorded in
/// `📓️terra-B1-host-native-report.md`.
mod actor_bindings {
    wasmtime::component::bindgen!({
        world: "actor",
        path: "../../../🧬️schema",
        async: false,
        additional_derives: [Clone, Debug],
    });
}

use actor_bindings::semio::framework::{capabilities as wit_capabilities, effects as wit_effects, events as wit_events, jobs as wit_jobs, reactor as wit_reactor, types as wit_types, ui as wit_ui};

/// 🧬️ `design-runtime.md` §2's slimmed `HostState { plugin_id, actor, caps, effect_sink,
/// asset_map }` — `limiter` is an implementation necessity (`Store::limiter` needs somewhere to
/// read bounds from), not part of the design's literal 5-field list.
struct ActorHostState {
    plugin_id: String,
    actor: RuntimeActorId,
    #[allow(dead_code)]
    caps: Vec<BrokerCapabilityGrant>,
    #[allow(dead_code)]
    effect_sink: Vec<Effect>,
    #[allow(dead_code)]
    asset_map: HashMap<String, Vec<u8>>,
    limiter: BudgetLimiter,
}

/// 🧬️ `pure` (`📜️wit/📜️pure.wit`) is `world actor`'s ONLY import — `log`/`now-ms`/`trace-span`,
/// none fallible, none async.
impl actor_bindings::semio::framework::pure::Host for ActorHostState {
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
    pub fn new(cfg: SharedEngineConfig) -> Result<Self, PluginHostError> {
        let (engine, pooling_active) = build_shared_engine(cfg)?;
        let epoch_ticker = EpochTicker::start(&engine);
        let mut linker = Linker::new(&engine);
        actor_bindings::semio::framework::pure::add_to_linker(&mut linker, |state: &mut ActorHostState| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let engine_config_hash = shared_engine_config_hash(&cfg, pooling_active);
        Ok(Self { engine, _epoch_ticker: epoch_ticker, linker, cache_root: default_compiled_cache_root(), engine_config_hash, next_instance_id: std::sync::atomic::AtomicU32::new(1) })
    }
}

impl GuestRuntime for WasmtimeRuntime {
    fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        let cache_path = compiled_cache_path(&self.cache_root, &self.engine_config_hash, &package.hash.0);
        if let Some(component) = load_compiled_component(&self.engine, &cache_path) {
            return Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)) });
        }
        let component = Component::from_binary(&self.engine, bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let _ = store_compiled_component(&component, &cache_path);
        Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)) })
    }

    fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        let component = compiled.component.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no wasmtime Component — built by MockGuestRuntime::compile, not WasmtimeRuntime::compile".to_string()))?;
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let host_state = ActorHostState { plugin_id: format!("actor-{}", actor.0), actor, caps: caps.to_vec(), effect_sink: Vec::new(), asset_map: HashMap::new(), limiter: BudgetLimiter::default() };
        let mut store = Store::new(&self.engine, host_state);
        store.limiter(|state| &mut state.limiter as &mut dyn ResourceLimiter);
        store.set_fuel(budget.fuel).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        store.set_epoch_deadline(budget.deadline_ms as u64);
        let bindings = actor_bindings::Actor::instantiate(&mut store, component, &self.linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(GuestInstance { actor, state: GuestInstanceState::Wasmtime(WasmtimeInstanceState { store, bindings, instance_id }) })
    }

    fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("execute_turn called on a non-wasmtime GuestInstance".to_string()));
        };
        state.store.set_fuel(budget.fuel).map_err(|error| TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;
        state.store.set_epoch_deadline(budget.deadline_ms as u64);
        let wit_budget = wit_reactor::Budget { fuel: budget.fuel, deadline_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
        let wit_events: Vec<wit_events::Event> = events.iter().map(|event| kernel_event_to_wit(event, state.instance_id)).collect();
        let call_result = state.bindings.semio_framework_reactor().call_poll(&mut state.store, &wit_events, wit_budget);
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
        let mut effects = Vec::with_capacity(wit_turn_result.effects.len());
        for effect in wit_turn_result.effects {
            effects.push(wit_effect_to_kernel(effect).map_err(TurnFault::Host)?);
        }
        Ok(TurnResult {
            // 🚧️ UI patch marshaling (WIT `patch-op`'s `path: list<u32>` + `node: pack` vs kernel
            // `PatchOp`'s `path: String` + `node: UiNode`) is NOT implemented — a real path/node
            // encoding convention needs to be agreed with A2/A3 first (`📓️terra-B1-host-native-
            // report.md`'s `## blocked-on` — tracked there, not silently dropped).
            ui_patches: Vec::new(),
            effects,
            next_wake: wit_turn_result.next_wake,
            status: wit_turn_status_to_kernel(wit_turn_result.status),
            fuel_used: wit_turn_result.fuel_used,
        })
    }

    fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("step_job called on a non-wasmtime GuestInstance".to_string()));
        };
        state.store.set_fuel(budget.fuel).map_err(|error| TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;
        state.store.set_epoch_deadline(budget.deadline_ms as u64);
        let wit_budget = wit_jobs::JobBudget { fuel: budget.fuel, deadline_ms: budget.deadline_ms };
        let step = state
            .bindings
            .semio_framework_jobs()
            .call_step_job(&mut state.store, job, wit_budget)
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(format!("{error:?}")))?;
        Ok(match step {
            wit_jobs::JobStep::Running(bytes) => JobStep::Running(bytes),
            wit_jobs::JobStep::Done(bytes) => JobStep::Done(bytes),
            wit_jobs::JobStep::Failed(bytes) => JobStep::Failed(bytes),
        })
    }

    fn checkpoint(&self, inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("checkpoint called on a non-wasmtime GuestInstance".to_string()));
        };
        state
            .bindings
            .semio_framework_checkpoint()
            .call_checkpoint(&mut state.store)
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    fn restore(&self, inst: &mut GuestInstance, state_bytes: &[u8]) -> Result<(), PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("restore called on a non-wasmtime GuestInstance".to_string()));
        };
        state
            .bindings
            .semio_framework_checkpoint()
            .call_restore(&mut state.store, state_bytes)
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    fn drop_instance(&self, _inst: GuestInstance) {
        // 🗑️ `Store<ActorHostState>` and its `Component` `Arc` drop with `_inst` — nothing else to
        // release; the pooling allocator reclaims the instance's slab on `Store` drop.
    }
}

//#region 🔀️EffectEventMarshal
/// 🌉️ `wit_effect_to_kernel`/`kernel_event_to_wit` are the host-side half of `design-abi.md` §2's
/// "WIT variants mirror [`Effect`/`Event`] field-for-field; the guest SDK glue converts between
/// them" — this is that conversion, mirrored for the host. It is NOT fully field-for-field in
/// practice: several real shape gaps between `📜️wit/*.wit` (packet A2) and `🎠️kernel/🦀️component.rs`
/// (packet A3) surfaced while writing this and are called out inline + in
/// `📓️terra-B1-host-native-report.md`'s `## blocked-on` (most notably: `📜️wit/📜️effects.wit`'s
/// `io-run` effect has no `Effect::IoRun` counterpart yet).
fn decode_dsl(bytes: &[u8]) -> Option<DslValue> {
    if bytes.is_empty() {
        return None;
    }
    store::pack_rt::decode_wire_value(bytes).ok()
}

fn encode_dsl(value: &DslValue) -> Vec<u8> {
    store::pack_rt::encode_wire_value(value)
}

fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Option<T> {
    if bytes.is_empty() {
        return None;
    }
    serde_json::from_slice(bytes).ok()
}

fn encode_json<T: serde::Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

fn wit_message_endpoint_to_kernel(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    match endpoint {
        wit_types::MessageEndpoint::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Backbone(uri) => MessageEndpoint::Backbone { uri },
        wit_types::MessageEndpoint::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Extension(id) => MessageEndpoint::Extension { id },
        wit_types::MessageEndpoint::Topic(name) => MessageEndpoint::Topic { name },
    }
}

fn kernel_message_endpoint_to_wit(endpoint: &MessageEndpoint) -> wit_types::MessageEndpoint {
    match endpoint {
        MessageEndpoint::Shell { instance } => wit_types::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => wit_types::MessageEndpoint::Backbone(uri.clone()),
        MessageEndpoint::PluginInstance { id } => wit_types::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => wit_types::MessageEndpoint::Extension(id.clone()),
        MessageEndpoint::Topic { name } => wit_types::MessageEndpoint::Topic(name.clone()),
    }
}

fn wit_request_outcome_to_kernel(result: wit_events::CompletionResult) -> RequestOutcome {
    match result {
        wit_events::CompletionResult::Ok(bytes) => RequestOutcome::Ok(bytes),
        wit_events::CompletionResult::Fault(bytes) => RequestOutcome::Err(bytes),
    }
}

fn kernel_request_outcome_to_wit(result: &RequestOutcome) -> wit_events::CompletionResult {
    match result {
        RequestOutcome::Ok(bytes) => wit_events::CompletionResult::Ok(bytes.clone()),
        RequestOutcome::Err(bytes) => wit_events::CompletionResult::Fault(bytes.clone()),
    }
}

fn wit_turn_status_to_kernel(status: wit_reactor::TurnStatus) -> TurnStatus {
    match status {
        wit_reactor::TurnStatus::Idle => TurnStatus::Idle,
        wit_reactor::TurnStatus::MoreWork => TurnStatus::MoreWork,
        wit_reactor::TurnStatus::CheckpointReady => TurnStatus::CheckpointReady,
        wit_reactor::TurnStatus::Faulted(bytes) => TurnStatus::Faulted(bytes),
    }
}

/// 🎁️ Every effect that carries a `req: request-id` becomes `RequestId(req)` — one line, so it's
/// inlined at each call site below rather than its own helper.

/// 🐛️ Guest → host: WIT `effect` (`📜️wit/📜️effects.wit`) to `semio_framework::kernel::Effect`.
/// `Err` is returned (never a silently-wrong `Effect`) for `io-run` — the one variant with no
/// kernel counterpart yet (`## blocked-on` in the report).
fn wit_effect_to_kernel(effect: wit_effects::Effect) -> Result<Effect, PluginHostError> {
    use wit_effects::Effect as E;
    Ok(match effect {
        E::SendMessage(inner) => Effect::SendMessage { target: kernel_message_endpoint_to_wit_reverse(inner.target), payload: inner.payload },
        E::PublishEvent(inner) => Effect::PublishEvent { topic: inner.topic, payload: inner.payload },
        E::BlobLoad(inner) => Effect::BlobLoad { req: RequestId(inner.req), hash: inner.hash },
        E::BlobWrite(inner) => Effect::BlobWrite {
            req: RequestId(inner.req),
            media_type: decode_json(&inner.media_type).unwrap_or(semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value }),
            bytes: inner.bytes,
        },
        E::HttpRequest(inner) => Effect::HttpRequest { req: RequestId(inner.req), method: inner.method, url: inner.url, headers: inner.headers, body: inner.body, stream: inner.streaming },
        E::DocumentRead(inner) => Effect::DocumentRead { req: RequestId(inner.req), doc: ArtifactHandle(inner.doc as u128), lane: inner.lane },
        E::DocumentWrite(inner) => Effect::DocumentWrite { req: RequestId(inner.req), doc: ArtifactHandle(inner.doc as u128), lane: inner.lane, ops: inner.ops },
        E::LinkResolve(inner) => Effect::LinkResolve { req: RequestId(inner.req), link: String::from_utf8_lossy(&inner.link).into_owned() },
        E::RegistryQuery(inner) => Effect::RegistryQuery { req: RequestId(inner.req), kind: inner.kind, filter: decode_dsl(&inner.filter) },
        E::IoCompose(inner) => Effect::IoCompose {
            req: RequestId(inner.req),
            key: String::from_utf8_lossy(&inner.key).into_owned(),
            sources: decode_json(&inner.sources).unwrap_or_default(),
        },
        // 🚧️ blocked-on-A3: no `Effect::IoRun` variant exists yet (`## blocked-on` in the report).
        E::IoRun(_inner) => return Err(PluginHostError::Plugin("effect io-run has no semio_framework::kernel::Effect variant yet (needs A3 to add Effect::IoRun) — see 📓️terra-B1-host-native-report.md".to_string())),
        E::CacheDerive(inner) => Effect::CacheDerive { req: RequestId(inner.req), engine_id: inner.engine_id, input: inner.input },
        E::CacheRead(inner) => Effect::CacheRead { req: RequestId(inner.req), engine_id: inner.engine_id, key: inner.key },
        E::OpenWindow(inner) => Effect::OpenWindow { req: RequestId(inner.req), kind: WindowKindId(inner.kind), params: decode_dsl(&inner.params).unwrap_or(DslValue::Null) },
        E::CloseWindow(inner) => Effect::CloseWindow { window: WindowHandle(inner.window as u128) },
        E::DispatchAction(inner) => Effect::DispatchAction { req: RequestId(inner.req), action: inner.target, args: decode_dsl(&inner.invocation), delay_ms: 0 },
        E::InvokeExtension(inner) => Effect::InvokeExtension { req: RequestId(inner.req), extension_id: inner.extension_id, capability: inner.capability, request_json: String::from_utf8_lossy(&inner.payload).into_owned() },
        E::Notify(inner) => Effect::Notify { message: inner.message },
        E::ClipboardWrite(inner) => Effect::ClipboardWrite { fragment: decode_json(&inner.fragment).unwrap_or_default() },
        E::Navigate(inner) => Effect::Navigate { uri: inner.uri },
        E::OpenExternalUrl(inner) => Effect::OpenExternalUrl { url: inner.url },
        E::SetPanel(inner) => Effect::SetPanel { panel_json: inner.panel_json },
        E::SetActiveUtility(inner) => Effect::SetActiveUtility { window_id: inner.window_id, utility_id: inner.utility_id },
        E::SetActiveTool(inner) => Effect::SetActiveTool { tool_id: inner.tool_id },
        E::PatchWorld3dChrome(inner) => Effect::PatchWorld3dChrome {
            selection_json: inner.selection_json,
            vortices_json: inner.vortices_json,
            document_selected_ids: inner.document_selected_ids,
            document_highlighted_ids: inner.document_highlighted_ids,
        },
        E::ReplayShellCommand(inner) => Effect::ReplayShellCommand { action_id: inner.action_id, args: inner.args.and_then(|bytes| decode_dsl(&bytes)) },
        E::SpawnPluginInstance(inner) => Effect::SpawnPluginInstance { req: RequestId(inner.req), plugin_id: inner.plugin_id, app_id: inner.app_id, os_instance_id: inner.os_instance_id, label: inner.label, document_json: inner.document_json },
        E::OpenPluginInstance(inner) => Effect::OpenPluginInstance { plugin_id: inner.plugin_id, app_id: inner.app_id, os_instance_id: inner.os_instance_id },
        E::OpenDialog(inner) => Effect::OpenDialog { req: RequestId(inner.req), dialog_id: inner.dialog_id, args: inner.args.and_then(|bytes| decode_dsl(&bytes)) },
        E::IconRenderExport(inner) => Effect::IconRenderExport { items: decode_json(&inner.items).unwrap_or_default() },
        E::DownloadMediaExport(inner) => Effect::DownloadMediaExport { filename: inner.filename, mime_type: inner.mime_type, data: inner.data, encoding: inner.encoding },
        E::RequestFileOpen(inner) => Effect::RequestFileOpen { req: RequestId(inner.req), accept: inner.accept, read_as: inner.read_as, import_action: String::new(), multiple: inner.multiple },
        E::RequestMediaFrames(inner) => Effect::RequestMediaFrames {
            req: RequestId(inner.req),
            accept: inner.accept,
            frame_action: String::new(),
            done_action: String::new(),
            fallback_action: String::new(),
            sample_stride: inner.sample_stride,
            max_frames: inner.max_frames,
            max_long_edge_px: inner.max_long_edge_px,
            fps_hint: inner.fps_hint,
            payload: inner.payload.map(|bytes| String::from_utf8_lossy(&bytes).into_owned()),
            args: inner.args.and_then(|bytes| decode_dsl(&bytes)),
        },
        E::LoadDocument(inner) => Effect::LoadDocument { pack: inner.doc_pack, spr: inner.spr },
        E::RequestSync => Effect::RequestSync,
        E::SetTimer(inner) => Effect::SetTimer { id: inner.id, after_ms: inner.after_ms as u64, repeat: inner.repeat },
        E::SpawnJob(inner) => Effect::SpawnJob { job: inner.job, kind: inner.kind, input: inner.input, placement: match inner.placement { wit_effects::JobPlacement::Inline => JobPlacement::Inline, wit_effects::JobPlacement::Isolated => JobPlacement::Isolated, wit_effects::JobPlacement::Exclusive => JobPlacement::Exclusive } },
        E::CancelJob(inner) => Effect::CancelJob { job: inner.job },
        E::Respond(inner) => Effect::Respond {
            req: RequestId(inner.req),
            result: match inner.outcome { wit_effects::RespondResult::Ok(bytes) => RequestOutcome::Ok(bytes), wit_effects::RespondResult::Fault(bytes) => RequestOutcome::Err(bytes) },
        },
        E::StorageRead(inner) => Effect::StorageRead { req: RequestId(inner.req), key: inner.key },
        E::StorageWrite(inner) => Effect::StorageWrite { req: RequestId(inner.req), key: inner.key, bytes: inner.value },
        E::StorageDelete(inner) => Effect::StorageDelete { req: RequestId(inner.req), key: inner.key },
        E::RequestCapability(inner) => Effect::RequestCapability { req: RequestId(inner.req), capability: CapabilityRequest { id: CapabilityId(inner.id), scope: inner.scope, reason: inner.reason, optional: inner.optional } },
        E::ReleaseCapability(inner) => Effect::ReleaseCapability { id: CapabilityId(inner.id) },
        E::Subscribe(inner) => Effect::Subscribe { topic: inner.topic },
        E::Unsubscribe(inner) => Effect::Unsubscribe { topic: inner.topic },
    })
}

/// 🐛️ `SendMessageEffect.target` is `wit_types::MessageEndpoint` (from the `types` interface via
/// `use types.{message-endpoint}` in `effects.wit`) — same generated Rust type as
/// `wit_message_endpoint_to_kernel` above takes, just named to keep the giant match arm list above
/// readable.
fn kernel_message_endpoint_to_wit_reverse(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    wit_message_endpoint_to_kernel(endpoint)
}

fn wit_surface_ref(instance_id: u32, surface: &str) -> wit_ui::SurfaceRef {
    // 🌉️ Convention (not yet confirmed with A2/A3 — `## blocked-on`): kernel's `Event`/`Effect`
    // surface fields are a plain `String`; WIT's `surface-ref` is a structured `{instance, surface:
    // u32}`. Treated here as the decimal string of the WIT `surface: u32`, `instance` supplied from
    // context (this actor's own instance id) since kernel's `String` never carried it.
    wit_ui::SurfaceRef { instance: instance_id, surface: surface.parse().unwrap_or(0) }
}

/// 🏁️ Host → guest: `semio_framework::kernel::Event` to WIT `event` (`📜️wit/📜️events.wit`).
/// `instance_id` fills the WIT `instance` field several kernel lifecycle variants dropped (see
/// `WasmtimeInstanceState::instance_id`'s docstring).
fn kernel_event_to_wit(event: &Event, instance_id: u32) -> wit_events::Event {
    match event {
        Event::InstanceOpen { instance, app_id, actor, config, assets, capabilities, quotas } => wit_events::Event::InstanceOpen(wit_events::InstanceOpenEvent {
            instance: instance.0.parse().unwrap_or(instance_id),
            app_id: app_id.clone(),
            actor: actor.clone(),
            config: config.clone(),
            assets: assets.clone(),
            capabilities: capabilities.iter().map(kernel_broker_grant_to_wit).collect(),
            quotas: encode_json(quotas),
        }),
        Event::InstanceClose => wit_events::Event::InstanceClose(wit_events::InstanceCloseEvent { instance: instance_id }),
        Event::Activate { reason } => wit_events::Event::Activate(wit_events::ActivateEvent { instance: instance_id, reason: kernel_activation_event_to_wit(reason) }),
        Event::SuspendRequest => wit_events::Event::SuspendRequest(wit_events::SuspendRequestEvent { instance: instance_id }),
        Event::CapabilityChanged { change } => wit_events::Event::CapabilityChanged(wit_events::CapabilityChangedEvent { instance: instance_id, change: kernel_capability_change_to_wit(change) }),
        Event::QuotaChanged { quotas } => wit_events::Event::QuotaChanged(wit_events::QuotaChangedEvent { instance: instance_id, quotas: encode_json(quotas) }),
        Event::AppCommandEvent { instance, seq, command } => wit_events::Event::AppCommand(wit_events::AppCommandEvent { instance: instance.0.parse().unwrap_or(instance_id), seq: *seq, command: command.clone() }),
        Event::SurfaceVisible { surface } => wit_events::Event::SurfaceVisible(wit_events::SurfaceVisibleEvent { surface: wit_surface_ref(instance_id, surface) }),
        Event::SurfaceHidden { surface } => wit_events::Event::SurfaceHidden(wit_events::SurfaceHiddenEvent { surface: wit_surface_ref(instance_id, surface) }),
        Event::SurfaceResized { surface, width, height } => wit_events::Event::SurfaceResized(wit_events::SurfaceResizedEvent { surface: wit_surface_ref(instance_id, surface), width: *width, height: *height }),
        Event::PatchAck { surface, revision } => wit_events::Event::PatchAck(wit_events::PatchAckEvent { surface: wit_surface_ref(instance_id, surface), revision: *revision }),
        Event::PatchRejected { surface, revision, reason } => wit_events::Event::PatchRejected(wit_events::PatchRejectedEvent { surface: wit_surface_ref(instance_id, surface), revision: *revision, reason: reason.clone() }),
        Event::Completed { req, result } => wit_events::Event::Completed(wit_events::CompletedEvent { req: req.0, outcome: kernel_request_outcome_to_wit(result) }),
        Event::HttpChunk { req, bytes, done } => wit_events::Event::HttpChunk(wit_events::HttpChunkEvent { req: req.0, bytes: bytes.clone(), done: *done }),
        Event::JobProgress { job, progress } => wit_events::Event::JobProgress(wit_events::JobProgressEvent { job: *job, progress: progress.clone().unwrap_or_default() }),
        Event::JobCompleted { job, result } => wit_events::Event::JobCompleted(wit_events::JobCompletedEvent { job: *job, outcome: kernel_request_outcome_to_wit(result) }),
        Event::Message { source, payload } => wit_events::Event::Message(wit_events::MessageEvent { source: kernel_message_endpoint_to_wit(source), payload: payload.clone() }),
        Event::Timer { id } => wit_events::Event::Timer(wit_events::TimerEvent { id: *id }),
        Event::Wake => wit_events::Event::Wake,
        Event::Request { req, from, capability, payload } => wit_events::Event::Request(wit_events::RequestEvent { req: req.0, from: kernel_message_endpoint_to_wit(from), capability: capability.clone(), payload: payload.clone() }),
    }
}

fn kernel_activation_event_to_wit(reason: &semio_framework::kernel::ActivationEvent) -> wit_events::ActivationEvent {
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

fn kernel_capability_change_to_wit(change: &semio_framework::kernel::CapabilityChange) -> wit_capabilities::CapabilityChange {
    use semio_framework::kernel::CapabilityChange as C;
    match change {
        C::Granted { id, grant } => wit_capabilities::CapabilityChange::Granted(kernel_broker_grant_to_wit(grant)),
        C::Revoked { id } => wit_capabilities::CapabilityChange::Revoked(id.0.clone()),
        C::Narrowed { id: _, grant } => wit_capabilities::CapabilityChange::Narrowed(kernel_broker_grant_to_wit(grant)),
    }
}

fn kernel_broker_grant_to_wit(grant: &BrokerCapabilityGrant) -> wit_capabilities::CapabilityGrant {
    wit_capabilities::CapabilityGrant {
        token: wit_capabilities::CapabilityToken { id: grant.id.0.clone(), token: grant.token.0 as u64 },
        scope: grant.scope.clone(),
        expires_ms: grant.expires_ms.map(|value| value as i64),
    }
}
//#endregion 🔀️EffectEventMarshal

#[cfg(test)]
mod wasmtime_runtime_tests {
    use super::*;

    #[test]
    fn compile_accepts_a_real_component_and_caches_it() {
        let wasm_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        if !wasm_path.exists() {
            return;
        }
        let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).expect("engine builds");
        let bytes = std::fs::read(wasm_path).expect("read real stdio.wasm");
        let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([9u8; 32]) };
        let compiled = runtime.compile(&package, &bytes).expect("a real wasip2 component compiles even though it does not export the new `actor` world yet");
        assert!(compiled.component.is_some());
    }

    #[test]
    fn instantiate_rejects_a_component_that_does_not_export_the_actor_world() {
        // 🧬️ No `.wasm` in this repo exports `world actor` yet (A2's guest SDK rewrite / the W3
        // plugin migrations haven't landed) — this asserts the HONEST negative: `instantiate`
        // rejects a real, valid, but wrong-ABI component rather than silently mis-binding it.
        let wasm_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        if !wasm_path.exists() {
            return;
        }
        let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).expect("engine builds");
        let bytes = std::fs::read(wasm_path).expect("read real stdio.wasm");
        let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([10u8; 32]) };
        let compiled = runtime.compile(&package, &bytes).expect("compiles as a component");
        let budget = Budget { fuel: 1_000_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
        let error = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).expect_err("stdio.wasm does not export `reactor`/`jobs`/`checkpoint`/`describe`");
        let _ = error;
    }

    #[test]
    fn wit_turn_status_conversion_is_a_plain_rename() {
        assert_eq!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::Idle), TurnStatus::Idle);
        assert_eq!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::MoreWork), TurnStatus::MoreWork);
        assert!(matches!(wit_turn_status_to_kernel(wit_reactor::TurnStatus::Faulted(vec![1, 2, 3])), TurnStatus::Faulted(bytes) if bytes == vec![1, 2, 3]));
    }

    #[test]
    fn message_endpoint_round_trips_through_wit_and_back() {
        let original = MessageEndpoint::Topic { name: "os.runtime.metrics".to_string() };
        let wit = kernel_message_endpoint_to_wit(&original);
        let back = wit_message_endpoint_to_kernel(wit);
        assert_eq!(original, back);
    }

    #[test]
    fn io_run_effect_is_a_reported_error_not_a_silent_mismap() {
        let effect = wit_effects::Effect::IoRun(wit_effects::IoRunEffect { req: 1, source: "a".to_string(), target: "b".to_string(), payload: vec![] });
        let result = wit_effect_to_kernel(effect);
        assert!(result.is_err(), "io-run must surface as an error until Effect::IoRun exists");
    }
}
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime

fn host_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    let code = code.into();
    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message))
}

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
    pub fn empty() -> Self {
        Self::default()
    }

    /// 🏗️ From a full pack+spr+ops snapshot.
    pub fn from_files(pack: Vec<u8>, spr: Vec<u8>, ops: String) -> Self {
        Self { pack, spr, ops, pending_binary_ops: Vec::new() }
    }

    /// 📥 Replaces this lane's opaque snapshot.
    pub fn adopt(&mut self, pack: Vec<u8>, spr: Vec<u8>, ops: String) {
        self.pack = pack;
        self.spr = spr;
        self.ops = ops;
        self.pending_binary_ops.clear();
    }

    /// 🧾 Applies guest `AppFrame::Emit` op bytes onto this lane via `ArtifactCodec` when `schema` is set.
    pub fn apply_emit_ops(&mut self, schema: Option<&str>, ops: Vec<u8>) {
        if ops.is_empty() {
            return;
        }
        let Some(schema) = schema.filter(|s| !s.is_empty()) else {
            self.pending_binary_ops = ops;
            return;
        };
        let Ok(Some(codec)) = store::document_codec(schema) else {
            self.pending_binary_ops = ops;
            return;
        };
        if self.pack.is_empty() && self.spr.is_empty() {
            self.pending_binary_ops = ops;
            return;
        }
        match (codec.apply_ops_binary)(&self.pack, &self.spr, &ops) {
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
    pub fn is_empty(&self) -> bool {
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
    pub fn new() -> Self {
        Self::default()
    }
}

const DEFAULT_ENGINE_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;
//#endregion 🔖️ArtifactSession

//#region 🔖️MutationReports
/// 🧾 Decodes a packed `protocol::DispatchReport` off the wire — the shape `AppFrame::Invocation`'s
/// trailing `messages` and `AppFrame::Error`'s trailing `report` both carry (contract-freeze.md
/// §C8/C9). Empty bytes (a pre-CHANNEL_VERSION-11 peer, or a frame that legitimately carries none)
/// decode to a message-free report under this session's own tracked policy rather than erroring —
/// callers that need to distinguish "no report" from "empty report" should check `bytes.is_empty()`
/// themselves first.
pub fn decode_dispatch_report(bytes: &[u8]) -> Result<protocol::DispatchReport, PluginHostError> {
    if bytes.is_empty() {
        return Ok(protocol::DispatchReport { policy: protocol::MergePolicy::default(), worst: None, messages: Vec::new() });
    }
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

/// 🔀 Decodes a packed `protocol::MergeReport` — `AppFrame::MergeReport.report`.
pub fn decode_merge_report(bytes: &[u8]) -> Result<protocol::MergeReport, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

/// ⚔️ Decodes a packed `Vec<protocol::Conflict>` — `AppFrame::Conflicts.conflicts`.
pub fn decode_conflicts(bytes: &[u8]) -> Result<Vec<protocol::Conflict>, PluginHostError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

/// 🔢️ One shared, monotonic `seq` source for every host-INITIATED `AppCommand` this crate sends on
/// a caller's behalf (`context_menu`, `hello`, `set_merge_policy`, `resolve_conflict`,
/// `read_conflicts`) — a single counter, not one static per call site, so two host-initiated
/// commands issued back to back (e.g. `hello`'s batched `Hello` + `SetMergePolicy`) never collide.
fn next_host_seq() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(1);
    SEQ.fetch_add(1, Ordering::Relaxed)
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
    runtimes: HashMap<String, Arc<WasmPluginRuntime>>,
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
fn rank_to_io_confidence(rank: u8) -> semio_framework::io_schema::Confidence {
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
fn rank_to_io_fidelity(rank: u8) -> semio_framework::io_schema::IoFidelity {
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
fn io_route_rank(hops: &[semio_framework::io_schema::IoEntryDescriptor]) -> (std::cmp::Reverse<u8>, usize, String) {
    let min_fidelity = hops.iter().map(|hop| hop.fidelity.rank()).min().unwrap_or(0);
    let joined = hops.iter().map(|hop| hop.into.to_coordinate()).collect::<Vec<_>>().join(",");
    (std::cmp::Reverse(min_fidelity), hops.len(), joined)
}

/// 🌉️ Breadth-bounded, cycle-free DFS enumeration of every simple path `from -> into` up to
/// `remaining_hops`, mirroring `io::io_mechanism::walk_routes` exactly. `graph` is a `BTreeMap`, so
/// iteration order is a pure function of the KEY SET, never of insertion/registration order — this
/// plus sorting the FULL candidate set at the end in `resolve_io_route` (never short-circuiting on
/// the first hit) is what makes the result independent of plugin load order — proven by
/// `io_router_route_is_deterministic_across_load_order` below.
fn walk_io_routes(
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
            walk_io_routes(graph, hop_into, into, remaining_hops - 1, path, visited, candidates);
            visited.remove(hop_into);
        }
        path.pop();
    }
}

/// 🌉️ `resolve_route`'s host-side twin (`io::io_mechanism::resolve_route`) over the merged
/// multi-plugin graph instead of one plugin's own local registry. Pure — no lock, no wasm call —
/// so it is directly unit-testable with a synthetic graph (`io_router_route_is_deterministic_
/// across_load_order`, `io_router_route_prefers_higher_minimum_fidelity`, below).
fn resolve_io_route(graph: &BTreeMap<IoEntryKey, IoEntryRoute>, from: &semio_framework::io_schema::ArtifactDialect, into: &semio_framework::io_schema::ArtifactDialect, max_hops: u8) -> Result<semio_framework::io_schema::IoRoute, PluginHostError> {
    let max_hops = max_hops.min(3);
    if max_hops == 0 {
        return Err(PluginHostError::Plugin(format!("io_routes {} -> {}: max_hops clamped to 0", from.to_coordinate(), into.to_coordinate())));
    }
    let mut candidates: Vec<Vec<semio_framework::io_schema::IoEntryDescriptor>> = Vec::new();
    let mut path: Vec<semio_framework::io_schema::IoEntryDescriptor> = Vec::new();
    let mut visited: BTreeSet<semio_framework::io_schema::ArtifactDialect> = BTreeSet::new();
    visited.insert(from.clone());
    walk_io_routes(graph, from, into, max_hops, &mut path, &mut visited, &mut candidates);
    if candidates.is_empty() {
        return Err(PluginHostError::Plugin(format!("no io route from {} to {} within {max_hops} hops", from.to_coordinate(), into.to_coordinate())));
    }
    candidates.sort_by(|a, b| io_route_rank(a).cmp(&io_route_rank(b)));
    let best = candidates.into_iter().next().expect("candidates checked non-empty above");
    let fidelity = rank_to_io_fidelity(best.iter().map(|hop| hop.fidelity.rank()).min().expect("a route has at least one hop"));
    Ok(semio_framework::io_schema::IoRoute { hops: best, fidelity })
}

/// 🌉️ Pure preflight check behind `IoRouter::register_plugin`'s io-entries half: does merging
/// `plugin_id`'s `incoming` roster (its `list-io-entries` wire bytes, decoded) into `existing`
/// claim a `(from, into)` key a DIFFERENT plugin already owns? `None` means the merge is safe
/// (either a brand-new key, or `plugin_id` re-claiming its OWN key — idempotent). Extracted as its
/// own function so the conflict rule is unit-testable without a real `Arc<WasmPluginRuntime>`.
fn io_entries_conflict(existing: &BTreeMap<IoEntryKey, IoEntryRoute>, plugin_id: &str, incoming: &[semio_framework::io_schema::IoEntryDescriptor]) -> Option<PluginHostError> {
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
fn route_reenters_calling_plugin<'route>(
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

    /// 📌️ Registers one already-loaded plugin's runtime + merges its composer roster into the
    /// shared route table, AND (CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-D) its NEW `list-io-
    /// entries` roster into `state.io_entries`. Call once per plugin, after `WasmPluginRuntime::
    /// load` succeeds. Both graphs preflight BEFORE either commits — a conflict in either leaves
    /// BOTH untouched, matching this file's existing all-or-nothing registration shape.
    pub fn register_plugin(&self, plugin_id: &str, runtime: Arc<WasmPluginRuntime>) -> Result<(), PluginHostError> {
        let wire_bytes = runtime.list_artifact_dialects()?;
        let entries: Vec<(semio_framework::ArtifactDialect, Vec<semio_framework::ArtifactDialect>)> = serde_json::from_slice(&wire_bytes).map_err(PluginHostError::Json)?;
        let mut candidate_routes = Vec::new();
        for (writes, reads) in entries {
            for read in &reads {
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
        let io_entries_bytes = runtime.list_io_entries()?;
        let io_entries: Vec<semio_framework::io_schema::IoEntryDescriptor> = serde_json::from_slice(&io_entries_bytes).map_err(PluginHostError::Json)?;
        let mut state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        if let Some(existing) = state.runtimes.get(plugin_id) {
            if !Arc::ptr_eq(existing, &runtime) {
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
        if let Some(conflict) = io_entries_conflict(&state.io_entries, plugin_id, &io_entries) {
            return Err(conflict);
        }
        state.runtimes.entry(plugin_id.to_owned()).or_insert(runtime);
        for key in candidate_routes {
            state.routes.entry(key).or_insert_with(|| plugin_id.to_owned());
        }
        for descriptor in io_entries {
            let key: IoEntryKey = (descriptor.from, descriptor.into);
            state.io_entries.entry(key).or_insert(IoEntryRoute { owner: plugin_id.to_owned(), fidelity: descriptor.fidelity, sniffs: descriptor.sniffs });
        }
        Ok(())
    }

    /// 📊️ `N plugins / M keys` — logged at boot so a dev-boot smoke test can confirm the router
    /// actually picked up more than zero cross-plugin routes.
    pub fn stats(&self) -> Result<(usize, usize), PluginHostError> {
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        Ok((state.runtimes.len(), state.routes.len()))
    }

    /// 🌉️ Routes `key`/`sources` (JSON wire bytes) to whichever OTHER plugin owns `key`. Refuses to
    /// route back into `calling_plugin_id` itself: the target plugin's `artifact-compose` guest
    /// handler is local-only by construction (see `io::wire_artifact_compose`'s own doc comment) and
    /// never calls `io-compose` again, so every route is exactly one hop — the self-route guard is
    /// what keeps a plugin from ever needing to reason about calling back into its own in-flight
    /// `Store` mutex (which would deadlock, since that mutex is already held by the caller of this
    /// very host call).
    pub fn compose(&self, calling_plugin_id: &str, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let key: semio_framework::IoKey = serde_json::from_slice(key_bytes)?;
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        let owner = state
            .routes
            .get(&key)
            .cloned()
            .ok_or_else(|| PluginHostError::Plugin(format!("no plugin registered for {}/{}/{} {:?} {}/{}/{}", key.artifact_kind, key.standard, key.subset, key.direction, key.format_kind, key.format_standard, key.format_subset)))?;
        if owner == calling_plugin_id {
            return Err(PluginHostError::Plugin(format!("io-compose refused: plugin `{calling_plugin_id}` would be routing to itself (should have resolved locally)")));
        }
        let runtime = state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns this key but its runtime is not registered with the router")))?;
        drop(state);
        runtime.artifact_compose(key_bytes, sources_bytes)
    }

    /// 📚️ Every dialect ANY loaded plugin can move `artifact_kind` through in `direction`
    /// ("import"|"export"), JSON `Vec<ArtifactDialect>` bytes.
    pub fn dialects(&self, artifact_kind: &str, direction: &str) -> Result<Vec<u8>, PluginHostError> {
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
        serde_json::to_vec(&dialects).map_err(PluginHostError::Json)
    }

    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): resolves the deterministic, ≤3-hop,
    /// cycle-free route `from -> into` over the merged `io_entries` graph — the WIT `io-routes`
    /// host import. JSON `io_schema::IoRoute` bytes.
    pub fn io_routes(&self, from: &str, into: &str) -> Result<Vec<u8>, PluginHostError> {
        let from = semio_framework::io_schema::ArtifactDialect::parse_coordinate(from).map_err(PluginHostError::Plugin)?;
        let into = semio_framework::io_schema::ArtifactDialect::parse_coordinate(into).map_err(PluginHostError::Plugin)?;
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        let route = resolve_io_route(&state.io_entries, &from, &into, 3)?;
        drop(state);
        serde_json::to_vec(&route).map_err(PluginHostError::Json)
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
    pub fn run_io(&self, calling_plugin_id: &str, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let from_dialect = semio_framework::io_schema::ArtifactDialect::parse_coordinate(from).map_err(PluginHostError::Plugin)?;
        let into_dialect = semio_framework::io_schema::ArtifactDialect::parse_coordinate(into).map_err(PluginHostError::Plugin)?;
        let hops = {
            let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
            let route = resolve_io_route(&state.io_entries, &from_dialect, &into_dialect, 3)?;
            if let Some(reentrant_hop) = route_reenters_calling_plugin(&state.io_entries, &route, calling_plugin_id) {
                return Err(PluginHostError::Plugin(format!(
                    "io-run refused: hop {} -> {} is owned by the calling plugin `{calling_plugin_id}` itself — executing it would re-enter that plugin's own in-flight, non-reentrant store lock",
                    reentrant_hop.0.to_coordinate(),
                    reentrant_hop.1.to_coordinate()
                )));
            }
            let mut hops = Vec::with_capacity(route.hops.len());
            for hop in &route.hops {
                let key: IoEntryKey = (hop.from.clone(), hop.into.clone());
                let owner = state
                    .io_entries
                    .get(&key)
                    .map(|entry| entry.owner.clone())
                    .ok_or_else(|| PluginHostError::Plugin(format!("io-run: hop {} -> {} vanished from the router between resolve and execute", hop.from.to_coordinate(), hop.into.to_coordinate())))?;
                let runtime = state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns hop {} -> {} but its runtime is not registered with the router", hop.from.to_coordinate(), hop.into.to_coordinate())))?;
                hops.push((hop.from.to_coordinate(), hop.into.to_coordinate(), runtime));
            }
            hops
        };
        let mut current = payload;
        for (from_coordinate, into_coordinate, runtime) in hops {
            current = runtime.io_run(&from_coordinate, &into_coordinate, current)?;
        }
        Ok(current)
    }

    /// 🔍️ Fans `io-sniff` out across every OTHER loaded plugin's carrier-`from` entries — the WIT
    /// `io-identify` host import. Skips the calling plugin's own carrier entries for the SAME
    /// reentrancy reason `run_io` refuses a self-owned hop (a fan-out is best-effort across
    /// multiple plugins, so this SKIPS rather than refuses the whole call). JSON `Vec<(ArtifactDialect,
    /// Confidence)>` bytes, sorted confidence descending then coordinate ascending — same shape and
    /// order `io::io_mechanism::io_identify` produces for the guest-local case.
    pub fn identify(&self, calling_plugin_id: &str, payload_bytes: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let payload: semio_framework::io_schema::IoPayload = serde_json::from_slice(&payload_bytes).map_err(PluginHostError::Json)?;
        let carrier = semio_framework::io_schema::ArtifactDialect::from(match &payload {
            semio_framework::io_schema::IoPayload::Binary(_) => semio_framework::io_schema::CARRIER_BINARY,
            semio_framework::io_schema::IoPayload::Text(_) => semio_framework::io_schema::CARRIER_TEXT,
        });
        let candidates: Vec<(semio_framework::io_schema::ArtifactDialect, String)> = {
            let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
            state
                .io_entries
                .iter()
                .filter(|((from, _into), route)| *from == carrier && route.sniffs && route.owner != calling_plugin_id)
                .map(|((_from, into), route)| (into.clone(), route.owner.clone()))
                .collect()
        };
        let mut found: Vec<(semio_framework::io_schema::ArtifactDialect, semio_framework::io_schema::Confidence)> = Vec::new();
        for (into, owner) in candidates {
            let runtime = {
                let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
                state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns a carrier io entry but its runtime is not registered with the router")))?
            };
            let rank = runtime.io_sniff(&carrier.to_coordinate(), &into.to_coordinate(), &payload_bytes)?;
            let confidence = rank_to_io_confidence(rank);
            if confidence != semio_framework::io_schema::Confidence::None {
                found.push((into, confidence));
            }
        }
        found.sort_by(|a, b| b.1.rank().cmp(&a.1.rank()).then_with(|| a.0.to_coordinate().cmp(&b.0.to_coordinate())));
        serde_json::to_vec(&found).map_err(PluginHostError::Json)
    }

    /// ✂️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): drops
    /// `plugin_id`'s runtime handle and every route it owns — required before a hot-reloaded
    /// plugin can re-register a FRESH `Arc<WasmPluginRuntime>` under the same id (`register_plugin`'s
    /// `PluginRuntimeConflict` check would otherwise reject the new `Arc` as a different pointer for
    /// an already-registered plugin id) and before an unload actually drops the runtime. Also drops
    /// every NEW-mechanism `io_entries` row `plugin_id` owns (W1-D).
    pub fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
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
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
    pub contributor: Option<String>,
    /// 🕸️ Other `inference_schema` ids (same `artifact_kind`) this inference's own computation
    /// consumes the RESULT of — the per-artifact `depends_on` DAG `ArtifactInferenceRouter` toposorts
    /// at registration and injects into `artifact-inference-request.dependencies` at call time.
    #[serde(default)]
    pub depends_on: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct InferenceRouteBudget {
    allocation_bytes: u64,
    work_units: u64,
    recursion_depth: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum InferenceRouteCacheMode {
    Cold,
    Incremental,
    Bypass,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
    runtimes: Mutex<HashMap<String, Arc<WasmPluginRuntime>>>,
}

impl ArtifactInferenceRouter {
    pub fn new() -> Self {
        Self { routes: Mutex::new(BTreeMap::new()), runtimes: Mutex::new(HashMap::new()) }
    }

    /// 📌️ `dependencies` is the reporting plugin's OWN declared `PluginManifest.dependencies` —
    /// required to gate a CONTRIBUTED row (contract §4 rule 1, same rule `ArtifactMutationRouter`
    /// applies): its `contributor` must equal `plugin_id`, and `owner` must be a direct entry of
    /// `dependencies`. Also toposorts every registered row's `depends_on` DAG (existing rows plus
    /// this registration's own) and rejects a cycle before committing anything.
    pub fn register_plugin(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], runtime: Arc<WasmPluginRuntime>) -> Result<(), PluginHostError> {
        let metadata: Vec<GuestArtifactInferenceMetadata> = serde_json::from_slice(&runtime.list_artifact_inferences()?)?;
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
        validate_inference_dependency_graph(&candidate)?;
        *routes = candidate;
        drop(routes);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.insert(plugin_id.to_string(), runtime);
        Ok(())
    }

    pub fn metadata(&self) -> Result<Vec<GuestArtifactInferenceMetadata>, PluginHostError> {
        Ok(self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?.values().map(|(_, item)| item.clone()).collect())
    }

    pub fn infer(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        self.infer_with_visited(request, &mut Vec::new())
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
    fn infer_with_visited(&self, request: &[u8], visited: &mut Vec<String>) -> Result<Vec<u8>, PluginHostError> {
        let mut route: InferenceRouteRequest = serde_json::from_slice(request)?;
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
                    routes.get(&(route.artifact_kind.clone(), dependency_schema.clone())).map(|(_, metadata)| metadata.clone()).ok_or_else(|| PluginHostError::Plugin(format!("inference `{}` declares depends_on `{dependency_schema}` which is not registered for artifact kind `{}`", route.inference_schema, route.artifact_kind)))?
                };
                let dependency_request = build_dependency_inference_request(&route, &dependency_metadata);
                let dependency_request_bytes = serde_json::to_vec(&dependency_request).map_err(PluginHostError::Json)?;
                let dependency_result_bytes = self.infer_with_visited(&dependency_request_bytes, visited)?;
                dependencies.push((dependency_schema.clone(), dependency_result_bytes));
            }
            visited.pop();
            route.dependencies = dependencies;
        }

        let request = serde_json::to_vec(&route).map_err(PluginHostError::Json)?;
        let runtime = self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("inference owner `{owner}` is not loaded")))?;
        let result = runtime.artifact_infer(&request)?;
        let echoed: InferenceRouteResult = serde_json::from_slice(&result)?;
        validate_inference_echo(&route, &echoed)?;
        Ok(result)
    }

    /// ✂️ Drops every route reported by `plugin_id` — called on unload/hot-reload.
    pub fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
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
fn validate_inference_dependency_graph(routes: &BTreeMap<(String, String), (String, GuestArtifactInferenceMetadata)>) -> Result<(), PluginHostError> {
    let mut adjacency: BTreeMap<(&str, &str), Vec<&str>> = BTreeMap::new();
    for ((artifact_kind, inference_schema), (_, item)) in routes {
        adjacency.entry((artifact_kind.as_str(), inference_schema.as_str())).or_default().extend(item.depends_on.iter().map(|dep| dep.as_str()));
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Mark {
        Visiting,
        Done,
    }

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
fn build_dependency_inference_request(base: &InferenceRouteRequest, dependency: &GuestArtifactInferenceMetadata) -> InferenceRouteRequest {
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

fn validate_inference_echo(request: &InferenceRouteRequest, result: &InferenceRouteResult) -> Result<(), PluginHostError> {
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

    #[test]
    fn only_exactly_echoed_guest_results_are_publishable() {
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
        assert!(validate_inference_echo(&request, &valid).is_ok());
        let stale = InferenceRouteResult { generation: 8, ..valid };
        assert!(matches!(validate_inference_echo(&request, &stale), Err(PluginHostError::Plugin(_))));
    }
}
//#endregion 💡️InferenceRouter

//#region 🔖️PluginGraph
/// 🕸️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A host wave): the
/// registered set of every loaded plugin's/extension's `PluginManifest`, validated against
/// contract freeze §4 rule 5 by calling straight through to W0-C's PURE `semio_framework::
/// {validate_dependency_graph, resolve_load_order, dependents}` — this type owns none of that
/// graph logic itself, only the registration/lifecycle bookkeeping `runtime_for`
/// (`🏃️run/🦀️component.rs`) needs to load a dependency before its dependent and to gate
/// hot-reload/unload against a plugin's live dependents (scout-2 §3/§5: neither existed before
/// this ticket).
pub struct PluginGraph {
    state: Mutex<BTreeMap<String, PluginManifest>>,
}

#[derive(Debug, thiserror::Error)]
pub enum PluginGraphError {
    #[error(transparent)]
    Graph(#[from] semio_framework::DependencyGraphError),
    #[error("plugin `{plugin_id}` cannot be unloaded: still depended on by {dependents:?}")]
    UnloadBlocked { plugin_id: String, dependents: Vec<String> },
    #[error("plugin `{plugin_id}` is not registered")]
    Unknown { plugin_id: String },
    #[error("plugin graph lock poisoned")]
    LockPoisoned,
}

impl PluginGraph {
    pub fn new() -> Self {
        Self { state: Mutex::new(BTreeMap::new()) }
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, BTreeMap<String, PluginManifest>>, PluginGraphError> {
        self.state.lock().map_err(|_| PluginGraphError::LockPoisoned)
    }

    /// 🔗️ Why a contributor may not act on `owner`'s artifact right now, if anything: `None` means
    /// the dependency is declared, the owner is loaded, and its version satisfies the requirement.
    /// Load-time registration already rejects all three, but a transaction can still meet them at
    /// dispatch time — an owner unloaded after registration, or a contributor loaded against a
    /// different build — so the transaction path asks again rather than assuming.
    pub fn contribution_block(&self, contributor: &str, owner: &str) -> Result<Option<(&'static str, String)>, PluginGraphError> {
        let state = self.lock()?;
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
    pub fn register(&self, manifest: PluginManifest) -> Result<(), PluginGraphError> {
        let mut state = self.lock()?;
        let mut candidate = state.clone();
        candidate.insert(manifest.plugin_id.clone(), manifest);
        let list: Vec<PluginManifest> = candidate.values().cloned().collect();
        // 🪪️ `resolve_load_order` (not `validate_dependency_graph` alone) is what actually detects a
        // CYCLE — per W0-C's own report: "a real cycle among present plugins passes validation and
        // is caught by the toposort leftover-set walk". `validate_dependency_graph` alone only
        // catches missing-dependency/version-mismatch.
        semio_framework::resolve_load_order(&list)?;
        *state = candidate;
        Ok(())
    }

    /// 🩹️ Contract §4.5 hot-reload half: re-validates the graph as it would look with `plugin_id`
    /// replaced by `new_manifest` — in particular every OTHER plugin's dependency ON `plugin_id`
    /// is re-checked against `new_manifest.version`, so a reload that would break a live
    /// dependent's contribution is rejected before the swap. Does not mutate on failure.
    pub fn prepare_hot_reload(&self, new_manifest: &PluginManifest) -> Result<(), PluginGraphError> {
        let state = self.lock()?;
        let mut candidate = state.clone();
        candidate.insert(new_manifest.plugin_id.clone(), new_manifest.clone());
        let list: Vec<PluginManifest> = candidate.values().cloned().collect();
        semio_framework::resolve_load_order(&list)?;
        Ok(())
    }

    /// ✅️ Commits `new_manifest` after `prepare_hot_reload` has already validated it.
    pub fn commit_hot_reload(&self, new_manifest: PluginManifest) -> Result<(), PluginGraphError> {
        self.register(new_manifest)
    }

    /// 🔒️ Contract §4.5 unload half: refuses while any OTHER registered plugin still depends on
    /// `plugin_id` (typed `UnloadBlocked`, names every blocking dependent).
    pub fn guard_unload(&self, plugin_id: &str) -> Result<(), PluginGraphError> {
        let blockers = self.dependents(plugin_id)?;
        if !blockers.is_empty() {
            return Err(PluginGraphError::UnloadBlocked { plugin_id: plugin_id.to_string(), dependents: blockers });
        }
        Ok(())
    }

    /// ✂️ Removes `plugin_id`'s registration — callers MUST call `guard_unload` first.
    pub fn unregister(&self, plugin_id: &str) -> Result<(), PluginGraphError> {
        let mut state = self.lock()?;
        state.remove(plugin_id).ok_or_else(|| PluginGraphError::Unknown { plugin_id: plugin_id.to_string() })?;
        Ok(())
    }

    /// 🔢️ Deterministic dependency-respecting load order over every currently registered plugin.
    pub fn load_order(&self) -> Result<Vec<String>, PluginGraphError> {
        let state = self.lock()?;
        let list: Vec<PluginManifest> = state.values().cloned().collect();
        Ok(semio_framework::resolve_load_order(&list)?)
    }

    /// 👥️ Direct dependents of `plugin_id`, sorted.
    pub fn dependents(&self, plugin_id: &str) -> Result<Vec<String>, PluginGraphError> {
        let state = self.lock()?;
        let list: Vec<PluginManifest> = state.values().cloned().collect();
        Ok(semio_framework::dependents(&list, plugin_id))
    }

    pub fn manifest(&self, plugin_id: &str) -> Result<Option<PluginManifest>, PluginGraphError> {
        Ok(self.lock()?.get(plugin_id).cloned())
    }

    pub fn is_registered(&self, plugin_id: &str) -> Result<bool, PluginGraphError> {
        Ok(self.lock()?.contains_key(plugin_id))
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

    fn manifest(plugin_id: &str, version: &str, deps: &[(&str, &str)]) -> PluginManifest {
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
            dependencies: deps.iter().map(|(id, req)| semio_framework::PluginDependency::new(*id, semio_framework::VersionReq::parse(req).unwrap())).collect(),
            contributions: vec![],
        }
    }

    /// 🔗️ The three ways a contribution can be blocked at DISPATCH time are distinguishable — the
    /// frozen taxonomy has separate codes for them, and collapsing "owner gone" into
    /// "not permitted" would tell an operator to fix a declaration that is already correct.
    #[test]
    fn contribution_block_separates_missing_owner_from_version_mismatch_from_undeclared() {
        let graph = PluginGraph::new();
        graph.register(manifest("cad", "1.0.0", &[])).unwrap();
        graph.register(manifest("aec", "1.0.0", &[("cad", "^1.0.0")])).unwrap();
        assert_eq!(graph.contribution_block("aec", "cad").unwrap(), None, "a declared, satisfied dependency blocks nothing");

        let (code, _) = graph.contribution_block("ghost", "cad").unwrap().expect("an unloaded contributor is blocked");
        assert_eq!(code, "transaction.dependency-missing");

        let (code, _) = graph.contribution_block("cad", "aec").unwrap().expect("an undeclared dependency is blocked");
        assert_eq!(code, "transaction.contribution-not-permitted");

        // 🛡️ The version branch is defence-in-depth, and this asserts WHY it cannot fire today rather
        // than pretending to exercise it: `register` re-validates the whole graph, so swapping `cad`
        // for a build `aec`'s requirement excludes is refused outright and the registered set keeps
        // its invariant. The branch stays because `contribution_block` is called per transaction, and
        // a future load path that mutates the set without that re-validation would otherwise hand a
        // contributor an owner it was never compiled against.
        let drift = graph.register(manifest("cad", "2.0.0", &[])).unwrap_err();
        assert!(matches!(drift, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
        assert_eq!(graph.contribution_block("aec", "cad").unwrap(), None, "the refused swap must leave the satisfied dependency intact");
    }

    #[test]
    fn load_order_respects_a_real_dependency_edge() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[])).unwrap();
        graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")])).unwrap();
        assert_eq!(graph.load_order().unwrap(), vec!["base".to_string(), "dependent".to_string()]);
        assert_eq!(graph.dependents("base").unwrap(), vec!["dependent".to_string()]);
    }

    #[test]
    fn register_rejects_a_missing_dependency() {
        let graph = PluginGraph::new();
        let error = graph.register(manifest("dependent", "1.0.0", &[("missing", "*")])).unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::MissingDependency { .. })));
        assert!(!graph.is_registered("dependent").unwrap(), "a rejected registration must not partially commit");
    }

    #[test]
    fn register_rejects_a_version_mismatch() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[])).unwrap();
        let error = graph.register(manifest("dependent", "1.0.0", &[("base", "^2.0.0")])).unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
    }

    #[test]
    fn a_later_registration_that_would_close_a_cycle_is_rejected() {
        let graph = PluginGraph::new();
        graph.register(manifest("a", "1.0.0", &[])).unwrap();
        graph.register(manifest("b", "1.0.0", &[("a", "*")])).unwrap();
        // Re-registering "a" (as if hot-reloading it) to depend on "b" would close a -> b -> a.
        let error = graph.register(manifest("a", "1.0.0", &[("b", "*")])).unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::Cycle { .. })));
    }

    #[test]
    fn unload_is_refused_while_a_dependent_is_registered() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[])).unwrap();
        graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")])).unwrap();
        let error = graph.guard_unload("base").unwrap_err();
        assert!(matches!(error, PluginGraphError::UnloadBlocked { .. }));
        graph.unregister("dependent").unwrap();
        graph.guard_unload("base").expect("no dependents left, unload must now be permitted");
    }

    #[test]
    fn hot_reload_is_rejected_when_it_would_break_a_live_dependents_version_requirement() {
        let graph = PluginGraph::new();
        graph.register(manifest("base", "1.0.0", &[])).unwrap();
        graph.register(manifest("dependent", "1.0.0", &[("base", "^1.0.0")])).unwrap();
        let error = graph.prepare_hot_reload(&manifest("base", "2.0.0", &[])).unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })));
        graph.prepare_hot_reload(&manifest("base", "1.1.0", &[])).expect("a caret-compatible bump must still validate");
    }
}
//#endregion 🔖️PluginGraph

//#region 🎯️MutationRouter
/// 🗂️ Host mirror of the guest `WireMutationRosterEntry` (contract §6's `contributor.
/// list-artifact-mutations` roster row) — `semio-framework-plugin-host` does not depend on the
/// guest SDK crate (same reasoning `ExtensionManifest` below already documents), so this is a
/// field-for-field JSON-shape-identical local copy, decoded off the same `store::pack_rt::
/// encode_wire_value`/`dsl::to_dsl_value` wire `list_artifact_mutations()` returns.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostMutationRosterEntry {
    pub mutation_id: String,
    pub verb: String,
    pub entity: String,
    pub kind: String,
    pub record: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_kind: Option<String>,
}

/// 🎞️ Host mirror of `WireArtifactMutationPlanRequest`/`Result` (contract §5.3's `contributor.
/// artifact-mutation-plan` call) — same "cannot depend on the guest SDK crate" reasoning.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostArtifactMutationPlanRequest {
    pub artifact_kind: String,
    pub mutation_id: String,
    pub revision: u64,
    pub generation: u64,
    pub snapshot_pack: Vec<u8>,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
/// 🦀️component.rs`) is `store::pack_rt::encode_wire_value(&to_dsl_value(value))`, NOT plain JSON.
fn decode_wire_dsl<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    let value = store::pack_rt::renormalize_whole_number_floats(value);
    dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
}

fn encode_wire_dsl<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, PluginHostError> {
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
}

impl ArtifactMutationRouter {
    pub fn new() -> Self {
        Self { routes: Mutex::new(BTreeMap::new()) }
    }

    /// 📌️ Decodes `roster_wire_bytes` (the exact `contributor.list-artifact-mutations` wire
    /// payload) and registers every row — see `register_roster` for the gating rules.
    pub fn register_plugin(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], roster_wire_bytes: &[u8]) -> Result<(), PluginHostError> {
        let roster: Vec<HostMutationRosterEntry> = decode_wire_dsl(roster_wire_bytes)?;
        self.register_roster(plugin_id, dependencies, roster)
    }

    /// 🧪️ Pure half of `register_plugin`, split out for deterministic testing without any wasm —
    /// mirrors `crate::app::register_contributions`'s own split in the guest SDK (W1-A). A
    /// CONTRIBUTED row (`contributor`/`artifact_kind` both `Some`) is gated against contract §4
    /// rule 1: `entry.contributor` must equal the reporting `plugin_id`, and `artifact_kind`'s
    /// owning plugin must be a DIRECT entry of `dependencies`. An OWNER row (both `None`) is keyed
    /// by the reporting plugin's own id and gated only by the ordinary conflict rule.
    pub fn register_roster(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], roster: Vec<HostMutationRosterEntry>) -> Result<(), PluginHostError> {
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
    pub fn resolve(&self, artifact_kind: &str, mutation_id: &str) -> Result<MutationOwnership, PluginHostError> {
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
    pub fn roster(&self) -> Result<Vec<HostMutationRosterEntry>, PluginHostError> {
        Ok(self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?.values().map(|(_, entry)| entry.clone()).collect())
    }

    /// ✂️ Drops every route reported by `plugin_id` (owner rows keyed under its own id, contributed
    /// rows it reported under an owner artifact kind) — called on unload/hot-reload.
    pub fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?;
        routes.retain(|_, (owner_plugin, entry)| {
            let reporter = entry.contributor.as_deref().unwrap_or(owner_plugin.as_str());
            reporter != plugin_id
        });
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

    fn owner_entry(mutation_id: &str) -> HostMutationRosterEntry {
        HostMutationRosterEntry { mutation_id: mutation_id.to_string(), verb: "set".into(), entity: "widget".into(), kind: "set-color".into(), record: "widget.doc".into(), contributor: None, artifact_kind: None }
    }

    fn contributed_entry(mutation_id: &str, contributor: &str, artifact_kind: &str) -> HostMutationRosterEntry {
        HostMutationRosterEntry { mutation_id: mutation_id.to_string(), verb: "annotate".into(), entity: "widget".into(), kind: "annotate".into(), record: "widget.doc".into(), contributor: Some(contributor.to_string()), artifact_kind: Some(artifact_kind.to_string()) }
    }

    // 🪪️ `io::ArtifactKindId::parse("s.owner.widget").plugin()` returns the BARE middle segment
    // ("owner", not "s.owner") — plugin ids throughout these fixtures are deliberately bare to
    // match the real grammar (a real loaded plugin's `manifest.plugin_id` is its Cargo component
    // metadata id, e.g. `"cad"`, never `"s.cad"`; only a canonical artifact kind string carries the
    // `s.` prefix).
    #[test]
    fn owner_and_contributed_rows_both_resolve_correctly() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color")]).unwrap();
        let dependency = semio_framework::PluginDependency::new("owner", semio_framework::VersionReq::Any);
        router.register_roster("contributor", &[dependency], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget")]).unwrap();

        assert_eq!(router.resolve("s.owner.widget", "widget.doc#set-color").unwrap(), MutationOwnership::Owner { plugin_id: "owner".into() });
        assert_eq!(router.resolve("s.owner.widget", "widget.doc#contributor:annotate").unwrap(), MutationOwnership::Contributed { plugin_id: "contributor".into() });
        assert_eq!(router.roster().unwrap().len(), 2, "both the owner and the contributed row must be visible");
    }

    #[test]
    fn a_contribution_onto_a_non_dependency_is_rejected() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color")]).unwrap();
        let error = router.register_roster("contributor", &[], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget")]).unwrap_err();
        assert!(matches!(error, PluginHostError::Plugin(_)));
    }

    #[test]
    fn conflicting_owner_rows_are_rejected_unless_byte_identical() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("a", &[], vec![owner_entry("widget.doc#set-color")]).unwrap();
        let error = router.register_roster("a", &[], vec![HostMutationRosterEntry { verb: "different".into(), ..owner_entry("widget.doc#set-color") }]).unwrap_err();
        assert!(matches!(error, PluginHostError::Plugin(_)));
        router.register_roster("a", &[], vec![owner_entry("widget.doc#set-color")]).expect("byte-identical re-registration is idempotent");
    }

    #[test]
    fn unregister_drops_only_that_plugins_rows() {
        let router = ArtifactMutationRouter::new();
        router.register_roster("owner", &[], vec![owner_entry("widget.doc#set-color")]).unwrap();
        let dependency = semio_framework::PluginDependency::new("owner", semio_framework::VersionReq::Any);
        router.register_roster("contributor", &[dependency], vec![contributed_entry("widget.doc#contributor:annotate", "contributor", "s.owner.widget")]).unwrap();
        router.unregister_plugin("contributor").unwrap();
        assert_eq!(router.roster().unwrap().len(), 1);
        assert!(router.resolve("s.owner.widget", "widget.doc#set-color").is_ok());
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
    pub fn bind(&self, artifact_id: &str, plugin_id: &str, instance_id: u32, artifact_kind: &str) -> Result<(), PluginHostError> {
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

    pub fn resolve(&self, artifact_id: &str) -> Option<InstanceLocation> {
        self.state.lock().ok()?.by_artifact_id.get(artifact_id).cloned()
    }

    /// ✂️ Drops every binding for `(plugin_id, instance_id)` — called on `destroy_app`.
    pub fn unbind_instance(&self, plugin_id: &str, instance_id: u32) {
        let Ok(mut state) = self.state.lock() else { return };
        if let Some(refs) = state.by_instance.remove(&(plugin_id.to_string(), instance_id)) {
            for artifact_id in refs {
                state.by_artifact_id.remove(&artifact_id);
            }
        }
    }

    pub fn artifact_ids_for_instance(&self, plugin_id: &str, instance_id: u32) -> Vec<String> {
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

    #[test]
    fn bind_resolve_and_unbind_round_trip() {
        let directory = InstanceDirectory::new();
        directory.bind("artifacts/node-a", "s.cad", 7, "s.cad.document").unwrap();
        let location = directory.resolve("artifacts/node-a").expect("bound artifact must resolve");
        assert_eq!(location, InstanceLocation { plugin_id: "s.cad".into(), instance_id: 7, artifact_kind: "s.cad.document".into() });
        directory.unbind_instance("s.cad", 7);
        assert!(directory.resolve("artifacts/node-a").is_none());
    }

    #[test]
    fn rebinding_the_same_artifact_id_replaces_the_prior_location() {
        let directory = InstanceDirectory::new();
        directory.bind("artifacts/node-a", "s.cad", 1, "s.cad.document").unwrap();
        directory.bind("artifacts/node-a", "s.cad", 2, "s.cad.document").unwrap();
        assert_eq!(directory.resolve("artifacts/node-a").unwrap().instance_id, 2);
        assert!(directory.artifact_ids_for_instance("s.cad", 1).is_empty(), "the stale instance no longer owns this artifact id");
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
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("{code}: {message}")]
    Rejected { code: String, message: String },
    #[error("plugin host error: {0}")]
    Host(#[from] PluginHostError),
}

impl TransactionError {
    pub fn rejected(code: &str, message: impl Into<String>) -> Self {
        Self::Rejected { code: code.to_string(), message: message.into() }
    }

    pub fn code(&self) -> &str {
        match self {
            TransactionError::Rejected { code, .. } => code,
            TransactionError::Host(_) => "transaction.commit-failed",
        }
    }
}

fn payload_hash_of(bytes: &[u8]) -> protocol::PayloadHash {
    protocol::PayloadHash(*blake3::hash(bytes).as_bytes())
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

    fn next_seq(&self) -> u64 {
        self.seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    fn mint_txn_id(&self, initiator: &TransactionMember) -> String {
        format!("txn-{}-{}-{}", initiator.plugin_id, initiator.instance_id, self.next_seq())
    }

    /// 🎯️ Contract §5 steps 1-6: resolves every `foreign` step to a live instance via
    /// `instances`/`mutation_router`, recurses with depth/cycle guards (§5.4), issues ONE
    /// `TransactionPrepare` per member (always pre-planned form — see `MemberDraft`'s doc),
    /// requires every member's `TransactionPrepared.rejection` to be empty (§5.5, else rolls back
    /// every already-prepared member), then commits in REVERSE discovery order (§5.6,
    /// compensating already-committed members via `TransactionUndo` on a commit failure).
    ///
    /// `exchange`/`plan_contributed` are closures so this can be driven over real
    /// `WasmPluginRuntime`s (production, see `🏃️run/🦀️component.rs`) or an in-process fake
    /// (tests) without either depending on the other.
    #[allow(clippy::too_many_arguments)]
    pub fn run_transaction(
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
        let txn_id = self.mint_txn_id(&initiator);

        let initiator_target = instances
            .artifact_ids_for_instance(&initiator.plugin_id, initiator.instance_id)
            .into_iter()
            .next()
            .and_then(|artifact_id| instances.resolve(&artifact_id).map(|location| protocol::ForeignTarget { artifact_id, artifact_kind: location.artifact_kind, dialect: None }))
            .unwrap_or_else(|| protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None });

        let mut discovery_order: Vec<TransactionMember> = vec![initiator.clone()];
        let mut drafts: BTreeMap<(String, u32), MemberDraft> = BTreeMap::new();
        drafts.insert((initiator.plugin_id.clone(), initiator.instance_id), MemberDraft { prepared_ops: local_ops, label: description, origin: protocol::MutationOrigin::Owner });

        let mut visited: std::collections::HashSet<(String, String, [u8; 32])> = std::collections::HashSet::new();
        let mut frontier: Vec<protocol::ForeignStep> = foreign;
        let mut depth: u8 = 0;

        while !frontier.is_empty() {
            depth += 1;
            if depth > protocol::MAX_PLAN_DEPTH {
                return Err(TransactionError::rejected("transaction.depth-exceeded", format!("transaction `{txn_id}` exceeded MAX_PLAN_DEPTH ({})", protocol::MAX_PLAN_DEPTH)));
            }
            let mut next_frontier: Vec<protocol::ForeignStep> = Vec::new();
            for step in frontier {
                let cycle_key = (step.target.artifact_id.clone(), step.mutation_id.0.clone(), *blake3::hash(&step.payload).as_bytes());
                if !visited.insert(cycle_key) {
                    return Err(TransactionError::rejected("transaction.cycle", format!("transaction `{txn_id}` revisited {}/{}", step.target.artifact_id, step.mutation_id.0)));
                }

                let location = instances.resolve(&step.target.artifact_id).ok_or_else(|| TransactionError::rejected("transaction.unknown-target", format!("no live instance bound to artifact id `{}`", step.target.artifact_id)))?;
                let ownership = mutation_router.resolve(&location.artifact_kind, &step.mutation_id.0).map_err(|error| TransactionError::rejected("transaction.unknown-mutation", error.to_string()))?;

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
                        let origin = protocol::MutationOrigin::Contributed { plugin_id: contributor.clone(), mutation_id: step.mutation_id.clone(), payload_hash: payload_hash_of(&step.payload) };
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
            let origin_bytes = match encode_wire_dsl(&draft.origin) {
                Ok(bytes) => bytes,
                Err(error) => {
                    rejection = Some(TransactionError::Host(error));
                    break;
                }
            };
            let command = protocol::AppCommand::TransactionPrepare { seq: self.next_seq(), txn_id: txn_id.clone(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: draft.prepared_ops.clone(), label: draft.label.clone(), origin: origin_bytes };
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
                    rejection = Some(TransactionError::rejected("transaction.member-rejected", format!("{}/{} rejected prepare (`{}`): {}", member.plugin_id, member.instance_id, fault.code.0, fault.message)));
                    break;
                }
                None => {
                    rejection = Some(TransactionError::rejected("transaction.member-rejected", format!("{}/{} sent no TransactionPrepared reply", member.plugin_id, member.instance_id)));
                    break;
                }
            }
        }

        if let Some(error) = rejection {
            for member in prepared.iter().rev() {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRollback { seq: self.next_seq(), txn_id: txn_id.clone() });
            }
            return Err(error);
        }

        // Phase 2 (§5.6): commit in REVERSE discovery order.
        let mut committed: Vec<TransactionMember> = Vec::new();
        let mut edit_id_by_member: BTreeMap<(String, u32), String> = BTreeMap::new();
        let mut commit_error: Option<TransactionError> = None;
        for member in discovery_order.iter().rev() {
            let command = protocol::AppCommand::TransactionCommit { seq: self.next_seq(), txn_id: txn_id.clone() };
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
                    commit_error = Some(TransactionError::rejected("transaction.commit-failed", format!("{}/{} failed to commit txn `{txn_id}`", member.plugin_id, member.instance_id)));
                    break;
                }
            }
        }

        if let Some(error) = commit_error {
            for member in &committed {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionUndo { seq: self.next_seq(), group_id: txn_id.clone() });
            }
            for member in discovery_order.iter().filter(|member| !committed.contains(member)) {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRollback { seq: self.next_seq(), txn_id: txn_id.clone() });
            }
            return Err(error);
        }

        let edit_ids: Vec<String> = discovery_order.iter().map(|member| edit_id_by_member.get(&(member.plugin_id.clone(), member.instance_id)).cloned().unwrap_or_default()).collect();
        Ok(TransactionOutcome { txn_id, members: discovery_order, edit_ids })
    }

    /// 🔁️ Contract §5.7: fans `TransactionUndo{group_id}` out to every member, best-effort (a
    /// member whose tail has since moved on independently errors on ITS side, not here — a host
    /// must not assume success for every member).
    pub fn undo_group(&self, mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>, members: &[TransactionMember], group_id: &str) {
        for member in members {
            let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionUndo { seq: self.next_seq(), group_id: group_id.to_string() });
        }
    }

    pub fn redo_group(&self, mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>, members: &[TransactionMember], group_id: &str) {
        for member in members {
            let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRedo { seq: self.next_seq(), group_id: group_id.to_string() });
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
    //! `🏃️run/🦀️component.rs` proves the wire-level plumbing against a REAL guest.
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

    fn dependency(id: &str) -> semio_framework::PluginDependency {
        semio_framework::PluginDependency::new(id, semio_framework::VersionReq::Any)
    }

    #[test]
    fn a_two_member_transaction_commits_and_group_undo_restores_both() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").unwrap();

        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router.register_roster("a", &[dependency("b")], vec![HostMutationRosterEntry { mutation_id: "s.b.widget#a:annotate".into(), verb: "annotate".into(), entity: "widget".into(), kind: "annotate".into(), record: "widget.doc".into(), contributor: Some("a".into()), artifact_kind: Some("s.b.widget".into()) }]).unwrap();

        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep { target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None }, mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()), payload: vec![9, 9], label: "annotate".into() }];

        let outcome = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                |_contributor, _artifact_kind, _mutation_id, _member, payload| Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "annotate".into(), foreign: Vec::new() }),
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1, 2, 3]],
                "propose annotate".into(),
                foreign,
            )
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

        coordinator.undo_group(|plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command), &outcome.members, &outcome.txn_id);
        let instances_map = cluster.instances.borrow();
        assert!(instances_map.get(&("s.a".to_string(), 1)).unwrap().undone.contains(&outcome.txn_id), "initiator must be undone");
        assert!(instances_map.get(&("s.b".to_string(), 2)).unwrap().undone.contains(&outcome.txn_id), "the contributed target must ALSO be undone (group undo restores both members)");
    }

    #[test]
    fn an_unknown_target_is_rejected_before_any_prepare_is_sent() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").unwrap();
        let router = ArtifactMutationRouter::new();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep { target: protocol::ForeignTarget { artifact_id: "artifacts/nowhere".into(), artifact_kind: "s.b.widget".into(), dialect: None }, mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()), payload: vec![1], label: "x".into() }];
        let error = coordinator
            .run_transaction(&instances, &router, |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command), |_, _, _, _, _| unreachable!("no contributed step to plan"), TransactionMember { plugin_id: "s.a".into(), instance_id: 1 }, vec![vec![1]], "x".into(), foreign)
            .unwrap_err();
        assert_eq!(error.code(), "transaction.unknown-target");
    }

    #[test]
    fn an_unknown_mutation_is_rejected() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").unwrap();
        let router = ArtifactMutationRouter::new();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep { target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None }, mutation_id: protocol::SchemaId("s.b.widget#unregistered".into()), payload: vec![1], label: "x".into() }];
        let error = coordinator
            .run_transaction(&instances, &router, |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command), |_, _, _, _, _| unreachable!("owner route, never contributed"), TransactionMember { plugin_id: "s.a".into(), instance_id: 1 }, vec![vec![1]], "x".into(), foreign)
            .unwrap_err();
        assert_eq!(error.code(), "transaction.unknown-mutation");
    }

    #[test]
    fn a_cycle_is_rejected() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").unwrap();
        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router.register_roster("a", &[dependency("b")], vec![HostMutationRosterEntry { mutation_id: "s.b.widget#a:annotate".into(), verb: "annotate".into(), entity: "widget".into(), kind: "annotate".into(), record: "widget.doc".into(), contributor: Some("a".into()), artifact_kind: Some("s.b.widget".into()) }]).unwrap();
        let coordinator = HostTransactionCoordinator::new();
        let step = protocol::ForeignStep { target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None }, mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()), payload: vec![7], label: "x".into() };
        // The contributed plan returns the SAME step again -> a real cycle by (artifact_id, mutation_id, payload_hash).
        let step_for_plan = step.clone();
        let error = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                move |_, _, _, _, payload| Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "x".into(), foreign: vec![step_for_plan.clone()] }),
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                vec![step],
            )
            .unwrap_err();
        assert_eq!(error.code(), "transaction.cycle");
    }

    #[test]
    fn a_member_rejection_rolls_back_every_already_prepared_member() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").unwrap();
        instances.bind("artifacts/target", "s.b", 2, "s.b.widget").unwrap();
        // Pre-occupy s.b/2's pending slot so its OWN prepare hits `transaction.instance-busy`
        // for real, through the fake's genuine busy-check — not a stubbed rejection.
        cluster.instances.borrow_mut().entry(("s.b".to_string(), 2)).or_default().pending = Some(("someone-elses-txn".into(), vec![]));

        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router.register_roster("a", &[dependency("b")], vec![HostMutationRosterEntry { mutation_id: "s.b.widget#a:annotate".into(), verb: "annotate".into(), entity: "widget".into(), kind: "annotate".into(), record: "widget.doc".into(), contributor: Some("a".into()), artifact_kind: Some("s.b.widget".into()) }]).unwrap();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep { target: protocol::ForeignTarget { artifact_id: "artifacts/target".into(), artifact_kind: "s.b.widget".into(), dialect: None }, mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()), payload: vec![1], label: "x".into() }];

        let error = coordinator
            .run_transaction(
                &instances,
                &router,
                |plugin_id, instance_id, command| cluster.exchange(plugin_id, instance_id, command),
                |_, _, _, _, payload| Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "x".into(), foreign: Vec::new() }),
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                foreign,
            )
            .unwrap_err();
        assert_eq!(error.code(), "transaction.member-rejected");
        let instances_map = cluster.instances.borrow();
        assert!(instances_map.get(&("s.a".to_string(), 1)).unwrap().pending.is_none(), "the initiator, prepared before the rejection, must have been rolled back");
    }

    #[test]
    fn a_chain_deeper_than_max_plan_depth_is_rejected() {
        let cluster = FakeCluster::default();
        let instances = InstanceDirectory::new();
        instances.bind("artifacts/initiator", "s.a", 1, "s.a.widget").unwrap();
        for i in 0..10u8 {
            instances.bind(&format!("artifacts/target-{i}"), "s.b", 100 + i as u32, "s.b.widget").unwrap();
        }
        let router = ArtifactMutationRouter::new();
        // 🪪️ `io::ArtifactKindId::parse("s.b.widget").plugin()` == "b" (bare middle segment) — the
        // CONTRIBUTED row must be registered under the CONTRIBUTOR's own bare plugin id ("a"), with
        // "b" (matching the artifact kind's real owner) as its declared dependency.
        router.register_roster("a", &[dependency("b")], vec![HostMutationRosterEntry { mutation_id: "s.b.widget#a:annotate".into(), verb: "annotate".into(), entity: "widget".into(), kind: "annotate".into(), record: "widget.doc".into(), contributor: Some("a".into()), artifact_kind: Some("s.b.widget".into()) }]).unwrap();
        let coordinator = HostTransactionCoordinator::new();
        let foreign = vec![protocol::ForeignStep { target: protocol::ForeignTarget { artifact_id: "artifacts/target-0".into(), artifact_kind: "s.b.widget".into(), dialect: None }, mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()), payload: vec![0], label: "x".into() }];
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
                    let foreign = if (next as usize) < 10 { vec![protocol::ForeignStep { target: protocol::ForeignTarget { artifact_id: format!("artifacts/target-{next}"), artifact_kind: "s.b.widget".into(), dialect: None }, mutation_id: protocol::SchemaId("s.b.widget#a:annotate".into()), payload: vec![next], label: "x".into() }] } else { Vec::new() };
                    Ok(HostArtifactMutationPlanResult { artifact_kind: "s.b.widget".into(), mutation_id: "s.b.widget#a:annotate".into(), revision: 0, generation: 0, owner_ops: vec![payload.to_vec()], label: "x".into(), foreign })
                },
                TransactionMember { plugin_id: "s.a".into(), instance_id: 1 },
                vec![vec![1]],
                "x".into(),
                foreign,
            )
            .unwrap_err();
        assert_eq!(error.code(), "transaction.depth-exceeded");
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

    /// 📌️ Registers one already-loaded plugin's surfaces — call once per plugin, after
    /// `WasmPluginRuntime::load` succeeds (mirrors `IoRouter::register_plugin`/
    /// `ArtifactInferenceRouter::register_plugin`'s call site in `semio-framework-os-run`).
    pub fn register_plugin(&self, plugin_id: &str, runtime: &WasmPluginRuntime) -> Result<(), semio_framework::Fault> {
        self.register_manifest(plugin_id, &runtime.manifest)
    }

    /// 🧪️ `register_plugin` split out for direct manifest-driven testing (no wasmtime component
    /// needed to exercise the two frozen conflict/gate faults) — pure aside from the `Mutex` lock.
    pub fn register_manifest(&self, plugin_id: &str, manifest: &PluginManifest) -> Result<(), semio_framework::Fault> {
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
                return Err(semio_framework::Fault::new(
                    semio_framework::FaultOrigin::Framework,
                    semio_framework::FaultCode::new("surface.conflict"),
                    format!("surface `{}` is already registered for plugin `{}`", app_ref.app_id, app_ref.plugin_id),
                ));
            }
            state.surfaces.entry((app.dialect.clone(), app.role)).or_default().push(app_ref);
        }
        Ok(())
    }

    /// 📚️ Every `AppRef` serving `(dialect, role)`, deterministically ordered: the dialect's owner
    /// plugin's surface first (if it has one), then the rest sorted `plugin_id` asc / `app_id` asc
    /// (contract §3).
    pub fn surfaces_for(&self, dialect: &semio_framework::ArtifactDialect, role: semio_framework::AppRole) -> Vec<semio_framework::AppRef> {
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
    pub fn owner_of(&self, artifact_kind: &str) -> Option<String> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).owners.get(artifact_kind).cloned()
    }

    /// ✂️ Drops every surface AND dependency record `plugin_id` registered — mirrors `IoRouter`/
    /// `ArtifactMutationRouter`/`ArtifactInferenceRouter`'s own `unregister_plugin` (called from
    /// `WasmtimeNodeHost::unload_plugin`/`hot_reload_plugin`). Deliberately does NOT clear `owners`:
    /// a hot-reload of the owner plugin re-registers the same `artifact_kinds` and re-claims
    /// ownership on its own; clearing the entry here would let whichever OTHER plugin happens to be
    /// registered next silently inherit ownership mid-reload — worse than a momentarily-stale owner
    /// pointing at a plugin with zero live surfaces (which `owned_surface_gaps` would then flag).
    pub fn unregister_plugin(&self, plugin_id: &str) {
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
    pub fn owned_surface_gaps(&self) -> Vec<semio_framework::Fault> {
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

    fn fixture_artifact_kind(id: &str) -> semio_framework::ArtifactKindSpec {
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

    pub(super) fn fixture_app(id: &str, dialect: semio_framework::ArtifactDialect, role: semio_framework::AppRole) -> semio_framework::AppDefinition {
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
            config: semio_framework::ConfigSpec::empty(),
            command_grammar: semio_framework::CommandGrammar::empty(),
            io: semio_framework::AppIo::from_document(id, semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value }, semio_framework::ArtifactPresentation { id: id.into(), name: id.into(), dimension: String::new(), component_kind: id.into() }),
        }
    }

    fn fixture_manifest(plugin_id: &str, dependencies: Vec<&str>, artifact_kinds: Vec<semio_framework::ArtifactKindSpec>, apps: Vec<semio_framework::AppDefinition>) -> PluginManifest {
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
            dependencies: dependencies.into_iter().map(|id| semio_framework::PluginDependency::new(id, semio_framework::VersionReq::Any)).collect(),
            contributions: Vec::new(),
        }
    }

    pub(super) fn dialect(subset: &str) -> semio_framework::ArtifactDialect {
        semio_framework::ArtifactDialect { artifact_kind: "s.cad.cad".into(), standard: "1".into(), subset: subset.into() }
    }

    fn register(router: &AppRouter, plugin_id: &str, dependencies: Vec<&str>, artifact_kinds: Vec<semio_framework::ArtifactKindSpec>, apps: Vec<semio_framework::AppDefinition>) -> Result<(), semio_framework::Fault> {
        router.register_manifest(plugin_id, &fixture_manifest(plugin_id, dependencies, artifact_kinds, apps))
    }

    #[test]
    fn owner_surface_sorts_first_then_plugin_id_then_app_id() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect("owner registers");
        register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/1#editor", dialect("1"), semio_framework::AppRole::Editor)]).expect("a distinct subset's editor, contributed by a dependent, does not conflict");
        let refs = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor);
        assert_eq!(refs, vec![semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() }]);
        assert_eq!(router.owner_of("s.cad.cad"), Some("cad".to_string()));
    }

    #[test]
    fn duplicate_app_ref_is_a_conflict() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        let app = fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor);
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![app.clone()]).expect("first registration succeeds");
        let error = register(&router, "cad", vec![], vec![], vec![app]).expect_err("re-registering the same AppRef must conflict");
        assert_eq!(error.code.0, "surface.conflict");
    }

    #[test]
    fn contribution_without_a_declared_dependency_is_rejected() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![]).expect("owner claims the kind with zero apps");
        let error = register(&router, "norm", vec![], vec![], vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect, semio_framework::AppRole::Viewer)]).expect_err("a non-owner plugin without a dependency on the owner must be rejected");
        assert_eq!(error.code.0, "surface.contribution-not-permitted");
    }

    #[test]
    fn contribution_with_a_declared_dependency_is_admitted_and_sorted_after_the_owner() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect("owner registers its editor");
        register(&router, "norm", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect.clone(), semio_framework::AppRole::Viewer)]).expect("norm depends on cad, so contributing a viewer for cad's dialect is permitted");
        let viewers = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Viewer);
        assert_eq!(viewers, vec![semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#viewer".into() }]);
    }

    #[test]
    fn owned_surface_gaps_reports_the_missing_role_only() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect, semio_framework::AppRole::Editor)]).expect("owner registers only an editor");
        let gaps = router.owned_surface_gaps();
        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0].code.0, "surface.missing-owner-surface");
        assert!(gaps[0].message.contains("viewer"));
    }

    #[test]
    fn unregister_plugin_drops_its_surfaces_but_keeps_its_ownership_claim() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect("owner registers");
        router.unregister_plugin("cad");
        assert!(router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor).is_empty(), "the surface itself is gone");
        assert_eq!(router.owner_of("s.cad.cad"), Some("cad".to_string()), "ownership claim survives so a re-registering hot-reload reclaims it, not a stray contributor");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect("re-registering after unregister succeeds (no stale conflict)");
        assert_eq!(router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor), vec![semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() }]);
    }

    /// 🔗️ Lane 1-D parity reconciliation (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET,
    /// `📓️w1-d-report.md`): the SAME ordered fixture — owner surface, two contributed surfaces
    /// from different plugins, a duplicate, an unknown dialect — is asserted here AND in the TS
    /// twin (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/🧪️w1-d-parity.ts`),
    /// which builds the identical manifests through `AppRouter.build`/`resolveOpeningApp`. Both
    /// sides must produce identical `surfaces_for` ordering and identical fault codes — run both,
    /// paste both outputs into the report, per the ticket's verification rule.
    #[test]
    fn w1_d_parity_fixture_owner_two_contributors_duplicate_and_unknown_dialect() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        register(&router, "cad", vec![], vec![fixture_artifact_kind("s.cad.cad")], vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect("owner registers");
        register(&router, "norm", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-norm", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect("norm depends on cad, contributes a second editor");
        register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-aec", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect("aec-building depends on cad, contributes a third editor");

        let refs = router.surfaces_for(&editor_dialect, semio_framework::AppRole::Editor);
        assert_eq!(
            refs,
            vec![
                semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() },
                semio_framework::AppRef { plugin_id: "aec-building".into(), app_id: "s.cad.cad@1/*#editor-aec".into() },
                semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#editor-norm".into() },
            ],
            "owner first, then contributors pluginId-ascending (aec-building < norm)"
        );

        let duplicate = register(&router, "aec-building", vec!["cad"], vec![], vec![fixture_app("s.cad.cad@1/*#editor-aec", editor_dialect.clone(), semio_framework::AppRole::Editor)]).expect_err("re-registering the same AppRef must conflict");
        assert_eq!(duplicate.code.0, "surface.conflict");

        let unknown_dialect = dialect("does-not-exist");
        let unknown = OpeningResolver::resolve(&router, &unknown_dialect, semio_framework::AppRole::Editor, None).expect_err("no surface registered for this subset");
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
    pub fn resolve(router: &AppRouter, dialect: &semio_framework::ArtifactDialect, role: semio_framework::AppRole, user_default: Option<&semio_framework::AppRef>) -> Result<semio_framework::AppRef, semio_framework::Fault> {
        let candidates = router.surfaces_for(dialect, role);
        if let Some(default_ref) = user_default {
            if candidates.contains(default_ref) {
                return Ok(default_ref.clone());
            }
        }
        if let Some(first) = candidates.into_iter().next() {
            return Ok(first);
        }
        Err(semio_framework::Fault::new(
            semio_framework::FaultOrigin::Framework,
            semio_framework::FaultCode::new("surface.unknown-dialect"),
            format!("no {} surface registered for `{}`", role.as_str(), dialect.to_coordinate()),
        ))
    }
}

#[cfg(test)]
mod opening_resolver_tests {
    use super::app_router_tests::{dialect, fixture_app};
    use super::*;

    #[test]
    fn step1_explicit_default_still_in_router_wins() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        router.register_manifest("cad", &PluginManifest { plugin_id: "cad".into(), label: "cad".into(), version: "0.1.0".into(), apps: vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor)], examples: Vec::new(), capabilities: Vec::new(), topic_contributions: Vec::new(), commands: Vec::new(), artifact_kinds: Vec::new(), dependencies: Vec::new(), contributions: Vec::new() }).expect("owner registers");
        router
            .register_manifest(
                "norm",
                &PluginManifest {
                    plugin_id: "norm".into(),
                    label: "norm".into(),
                    version: "0.1.0".into(),
                    apps: vec![fixture_app("s.cad.cad@1/*#editor-alt", editor_dialect.clone(), semio_framework::AppRole::Editor)],
                    examples: Vec::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                    commands: Vec::new(),
                    artifact_kinds: Vec::new(),
                    dependencies: vec![semio_framework::PluginDependency::new("cad", semio_framework::VersionReq::Any)],
                    contributions: Vec::new(),
                },
            )
            .expect("norm contributes a second editor for the same dialect");
        let pinned = semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#editor-alt".into() };
        let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, Some(&pinned)).expect("pinned default resolves");
        assert_eq!(resolved, pinned);
    }

    #[test]
    fn step2_and_step3_collapse_to_the_owner_surface_when_default_is_stale() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        router.register_manifest("cad", &PluginManifest { plugin_id: "cad".into(), label: "cad".into(), version: "0.1.0".into(), apps: vec![fixture_app("s.cad.cad@1/*#editor", editor_dialect.clone(), semio_framework::AppRole::Editor)], examples: Vec::new(), capabilities: Vec::new(), topic_contributions: Vec::new(), commands: Vec::new(), artifact_kinds: Vec::new(), dependencies: Vec::new(), contributions: Vec::new() }).expect("owner registers");
        let stale_default = semio_framework::AppRef { plugin_id: "gone".into(), app_id: "s.cad.cad@1/*#editor".into() };
        let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, Some(&stale_default)).expect("falls through to owner surface");
        assert_eq!(resolved, semio_framework::AppRef { plugin_id: "cad".into(), app_id: "s.cad.cad@1/*#editor".into() });
    }

    #[test]
    fn step3_first_entry_when_the_owner_has_no_surface_for_this_role() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        router.register_manifest("cad", &PluginManifest { plugin_id: "cad".into(), label: "cad".into(), version: "0.1.0".into(), apps: Vec::new(), examples: Vec::new(), capabilities: Vec::new(), topic_contributions: Vec::new(), commands: Vec::new(), artifact_kinds: Vec::new(), dependencies: Vec::new(), contributions: Vec::new() }).expect("owner claims nothing yet, zero apps");
        router
            .register_manifest(
                "norm",
                &PluginManifest {
                    plugin_id: "norm".into(),
                    label: "norm".into(),
                    version: "0.1.0".into(),
                    apps: vec![fixture_app("s.cad.cad@1/*#viewer", editor_dialect.clone(), semio_framework::AppRole::Viewer)],
                    examples: Vec::new(),
                    capabilities: Vec::new(),
                    topic_contributions: Vec::new(),
                    commands: Vec::new(),
                    artifact_kinds: Vec::new(),
                    dependencies: Vec::new(),
                    contributions: Vec::new(),
                },
            )
            .expect("s.cad.cad has no owner yet, so norm becomes it by being first to declare a surface for it");
        let resolved = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Viewer, None).expect("first (only) entry resolves");
        assert_eq!(resolved, semio_framework::AppRef { plugin_id: "norm".into(), app_id: "s.cad.cad@1/*#viewer".into() });
    }

    #[test]
    fn step4_unknown_dialect_when_the_router_has_nothing() {
        let router = AppRouter::new();
        let editor_dialect = dialect("*");
        let error = OpeningResolver::resolve(&router, &editor_dialect, semio_framework::AppRole::Editor, None).expect_err("empty router must fault");
        assert_eq!(error.code.0, "surface.unknown-dialect");
    }
}
//#endregion 🔖️OpeningResolver

//#region 🔖️HostState
struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    granted_capabilities: Vec<CapabilityRequirement>,
    plugin_id: String,
    backbones: HashMap<String, Box<dyn store::Backbone>>,
    /// @emoji 📦️ Backing store for `write-blob`/`read-blob`, injected via
    /// {@link WasmPluginRuntime::register_host_blob_store} — `None` until a caller registers one
    /// (mirrors `backbones`' explicit-registration convention, not a stub-forever like `read-asset`).
    blob_store: Option<Arc<dyn store::BlobStore>>,
    /// 🌉️ Backing cross-plugin `IoRouter`, injected via {@link WasmPluginRuntime::register_host_io_router}
    /// — `None` (WIT `io-dialects`/`io-compose` calls fault) until a caller registers one, same
    /// explicit-registration convention as `blob_store`/`backbones`.
    io_router: Option<Arc<IoRouter>>,
    /// @emoji ⚙️ Plugin-wide host engine cache (content-addressed; not per document instance).
    engines: store::EngineCache,
    /// @emoji 🧾 Per-instance opaque pack authority (`create_app` inserts; `destroy_app` removes).
    sessions: HashMap<u32, ArtifactSession>,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl HostState {
    fn has_backbone_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Backbone && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }

    fn has_engine_access(&self, rights: Rights) -> bool {
        self.granted_capabilities.iter().any(|cap| cap.artifact == ArtifactKind::Engine && cap.rights == rights && matches!(cap.scope, Scope::Plugin | Scope::Global))
    }

    /// @emoji 🔌️ Looks up the real, native-side backbone for a plugin-attached uri — the plugin only
    /// ever sees an opaque channel; this host process owns the actual sync endpoint. Native URI→IO
    /// resolution left this crate with WS-A (`store::resolve_backbone` is wasm-only now); the endpoint
    /// must be registered up front via {@link WasmPluginRuntime::register_host_backbone}. WS-E wires a
    /// `sync::ArtifactHost`-backed backbone in here; until then this is an explicit-registration map.
    fn backbone_for(&mut self, uri: &str) -> Result<&mut Box<dyn store::Backbone>, String> {
        self.backbones.get_mut(uri).ok_or_else(|| format!("no host backbone registered for {uri}; call register_host_backbone (WS-E wires ArtifactHost here)"))
    }

    fn ensure_session(&mut self, instance_id: u32) -> &mut ArtifactSession {
        self.sessions.entry(instance_id).or_insert_with(ArtifactSession::new)
    }

    fn adopt_document(&mut self, instance_id: u32, pack: Vec<u8>, spr: Vec<u8>, ops: String) {
        let session = self.ensure_session(instance_id);
        session.document.adopt(pack, spr, ops);
        session.generation = session.generation.saturating_add(1);
    }

    fn adopt_config(&mut self, instance_id: u32, pack: Vec<u8>, spr: Vec<u8>, ops: String) {
        let session = self.ensure_session(instance_id);
        session.config.adopt(pack, spr, ops);
        session.generation = session.generation.saturating_add(1);
    }

    fn adopt_draft(&mut self, instance_id: u32, pack: Vec<u8>, spr: Vec<u8>, ops: String) {
        let session = self.ensure_session(instance_id);
        session.draft.adopt(pack, spr, ops);
        session.generation = session.generation.saturating_add(1);
    }
}

impl semio::framework::host::Host for HostState {
    fn log(&mut self, level: String, message: String) {
        eprintln!("[plugin:{}:{level}] {message}", self.plugin_id);
    }

    fn now_ms(&mut self) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
    }

    fn read_artifact(&mut self, _handle: u64) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.read-document", "read-document not implemented"))
    }

    fn write_artifact(&mut self, _handle: u64, _payload: Vec<u8>) -> Result<(), Vec<u8>> {
        Err(host_fault_bytes("os.host.write-document", "write-document not implemented"))
    }

    fn open_window(&mut self, _kind: String, _params: Vec<u8>) -> Result<u64, Vec<u8>> {
        Err(host_fault_bytes("os.host.open-window", "open-window not implemented"))
    }

    fn invoke_action(&mut self, _target: String, _invocation: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.invoke-action", "invoke-action not implemented"))
    }

    fn read_asset(&mut self, handle: u64) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.read-asset", format!("read-asset: unknown handle {handle}")))
    }

    fn network_fetch(&mut self, _origin: String, _path: String) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.network-fetch", "network-fetch not implemented"))
    }

    fn write_blob(&mut self, data: Vec<u8>, media_type: String) -> Result<String, Vec<u8>> {
        let store = self.blob_store.as_ref().ok_or_else(|| host_fault_bytes("os.host.write-blob", "no host blob store registered; call register_host_blob_store"))?;
        store.put(&data, &media_type).map(|blob_ref| blob_ref.hash).map_err(|error| host_fault_bytes("os.host.write-blob", error.to_string()))
    }

    fn read_blob(&mut self, hash: String) -> Result<Vec<u8>, Vec<u8>> {
        let store = self.blob_store.as_ref().ok_or_else(|| host_fault_bytes("os.host.read-blob", "no host blob store registered; call register_host_blob_store"))?;
        store.get(&hash).map_err(|error| host_fault_bytes("os.host.read-blob", error.to_string()))?.ok_or_else(|| host_fault_bytes("os.host.read-blob", format!("blob not found: {hash}")))
    }

    fn backbone_send(&mut self, uri: String, message: Vec<u8>) -> Result<(), Vec<u8>> {
        if !self.has_backbone_access(Rights::Write) {
            return Err(host_fault_bytes("os.host.backbone-send", "backbone write capability missing"));
        }
        let message = <store::BackboneMessage as protocol::OpBinary>::decode_op(&message).map_err(|error| host_fault_bytes("os.host.backbone-send", error.to_string()))?;
        self.backbone_for(&uri).map_err(|error| host_fault_bytes("os.host.backbone-send", error))?.send(message).map_err(|error| host_fault_bytes("os.host.backbone-send", error.to_string()))
    }

    fn backbone_poll(&mut self, uri: String) -> Result<Vec<Vec<u8>>, Vec<u8>> {
        if !self.has_backbone_access(Rights::Read) {
            return Err(host_fault_bytes("os.host.backbone-poll", "backbone read capability missing"));
        }
        let messages = self.backbone_for(&uri).map_err(|error| host_fault_bytes("os.host.backbone-poll", error))?.receive().map_err(|error| host_fault_bytes("os.host.backbone-poll", error.to_string()))?;
        messages.into_iter().map(|message| protocol::OpBinary::encode_op(&message).map_err(|error| host_fault_bytes("os.host.backbone-poll", error.to_string()))).collect()
    }

    fn backbone_status(&mut self, uri: String) -> Result<String, Vec<u8>> {
        Ok(if self.backbones.contains_key(&uri) { "attached".into() } else { "detached".into() })
    }

    fn engine_derive(&mut self, engine_id: String, input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        if !self.has_engine_access(Rights::Invoke) {
            return Err(host_fault_bytes("os.host.engine-derive", "engine invoke capability missing"));
        }
        let handle = self.engines.derive(&engine_id, &input).map_err(|error| host_fault_bytes("os.host.engine-derive", error.to_string()))?;
        Ok(handle.key.0.to_vec())
    }

    fn io_dialects(&mut self, artifact_kind: String, direction: String) -> Result<Vec<u8>, Vec<u8>> {
        let router = self.io_router.as_ref().ok_or_else(|| host_fault_bytes("os.host.io-dialects", "no host io router registered; call register_host_io_router"))?;
        router.dialects(&artifact_kind, &direction).map_err(|error| host_fault_bytes("os.host.io-dialects", error.to_string()))
    }

    fn io_compose(&mut self, key: Vec<u8>, sources: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        let router = self.io_router.as_ref().ok_or_else(|| host_fault_bytes("os.host.io-compose", "no host io router registered; call register_host_io_router"))?;
        router.compose(&self.plugin_id, &key, &sources).map_err(|error| host_fault_bytes("os.host.io-compose", error.to_string()))
    }

    fn engine_read(&mut self, engine_id: String, key: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        if !self.has_engine_access(Rights::Read) {
            return Err(host_fault_bytes("os.host.engine-read", "engine read capability missing"));
        }
        let key_bytes: [u8; 32] = key.as_slice().try_into().map_err(|_| host_fault_bytes("os.host.engine-read", format!("engine key must be 32 bytes, got {}", key.len())))?;
        let handle = store::EngineHandle { key: store::EngineKey(key_bytes), engine_id };
        self.engines.read(&handle).map_err(|error| host_fault_bytes("os.host.engine-read", error.to_string()))
    }

    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): stub passthrough — the WIT signature's error is
    /// a plain `string` (not `list<u8>` Fault bytes like every other function here), so no
    /// `host_fault_bytes` wrapper. Full resolver wiring (`🏪️store::LinkResolver`/`LinkState`
    /// against a real document registry) is a later wave; this stub exists so the guest-side import
    /// and every plugin's `bindgen!`-generated `Host` trait already compile against the final shape.
    fn resolve_artifact_link(&mut self, _link: Vec<u8>) -> Result<Vec<u8>, String> {
        Err("resolve-artifact-link not implemented — full resolver wiring is a later wave".to_string())
    }

    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): see `IoRouter::io_routes`.
    fn io_routes(&mut self, from: String, into: String) -> Result<Vec<u8>, Vec<u8>> {
        let router = self.io_router.as_ref().ok_or_else(|| host_fault_bytes("os.host.io-routes", "no host io router registered; call register_host_io_router"))?;
        router.io_routes(&from, &into).map_err(|error| host_fault_bytes("os.host.io-routes", error.to_string()))
    }

    /// 🌉️ See `IoRouter::run_io` for the resolved-route execution + reentrancy guard.
    fn io_run(&mut self, from: String, into: String, payload: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        let router = self.io_router.as_ref().ok_or_else(|| host_fault_bytes("os.host.io-run", "no host io router registered; call register_host_io_router"))?;
        router.run_io(&self.plugin_id, &from, &into, payload).map_err(|error| host_fault_bytes("os.host.io-run", error.to_string()))
    }

    /// 🔍️ See `IoRouter::identify` for the carrier-scoped fan-out + skip-self guard.
    fn io_identify(&mut self, payload: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        let router = self.io_router.as_ref().ok_or_else(|| host_fault_bytes("os.host.io-identify", "no host io router registered; call register_host_io_router"))?;
        router.identify(&self.plugin_id, payload).map_err(|error| host_fault_bytes("os.host.io-identify", error.to_string()))
    }
}
//#endregion 🔖️HostState

//#region 🔖️WasmPluginRuntime
pub struct WasmPluginRuntime {
    engine: Engine,
    component: Component,
    linker: Linker<HostState>,
    store: Mutex<Store<HostState>>,
    bindings: Mutex<PluginWorld>,
    pub manifest: PluginManifest,
    pub path: PathBuf,
    pub supervisor_state: Mutex<ProgramSupervisorState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgramSupervisorState {
    Loaded,
    Running,
    Crashed,
    TimedOut,
    Restarting,
    Quarantined,
    Unloaded,
}

impl WasmPluginRuntime {
    fn store_guard(&self) -> Result<std::sync::MutexGuard<'_, Store<HostState>>, PluginHostError> {
        self.store.lock().map_err(|_| PluginHostError::LockPoisoned("plugin store"))
    }

    fn bindings_guard(&self) -> Result<std::sync::MutexGuard<'_, PluginWorld>, PluginHostError> {
        self.bindings.lock().map_err(|_| PluginHostError::LockPoisoned("plugin bindings"))
    }

    fn plugin_result<T>(result: Result<T, semio::framework::types::PluginError>) -> Result<T, PluginHostError> {
        result.map_err(|error| match error {
            semio::framework::types::PluginError::Fault(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                PluginHostError::Plugin(fault.message)
            }
        })
    }

    fn build_engine() -> Result<Engine, PluginHostError> {
        let mut config = Config::new();
        config.consume_fuel(true);
        config.epoch_interruption(true);
        config.wasm_component_model(true);
        Engine::new(&config).map_err(|error| PluginHostError::Wasmtime(error.to_string()))
    }

    fn build_linker(engine: &Engine) -> Result<Linker<HostState>, PluginHostError> {
        let mut linker = Linker::new(engine);
        semio::framework::host::add_to_linker(&mut linker, |state| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(linker)
    }

    fn host_state(plugin_id: &str, manifest: &PluginManifest) -> HostState {
        HostState {
            wasi: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
            granted_capabilities: manifest.capabilities.clone(),
            plugin_id: plugin_id.to_string(),
            backbones: HashMap::new(),
            blob_store: None,
            io_router: None,
            engines: store::EngineCache::new(DEFAULT_ENGINE_CACHE_BUDGET_BYTES),
            sessions: HashMap::new(),
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, PluginHostError> {
        let path = path.as_ref().to_path_buf();
        let wasm_bytes = std::fs::read(&path)?;
        Self::load_from_wasm_bytes(&wasm_bytes, path)
    }

    /// @emoji 📦 Installs a plugin runtime directly from in-memory wasip2 component bytes (extension store / sideload).
    pub fn load_bytes(wasm_bytes: &[u8]) -> Result<Self, PluginHostError> {
        Self::load_from_wasm_bytes(wasm_bytes, PathBuf::new())
    }

    fn load_from_wasm_bytes(wasm_bytes: &[u8], path: PathBuf) -> Result<Self, PluginHostError> {
        let engine = Self::build_engine()?;
        let component = Component::from_binary(&engine, wasm_bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let linker = Self::build_linker(&engine)?;
        let manifest = Self::read_manifest(&engine, &component, &linker)?;
        let store = Store::new(&engine, Self::host_state(&manifest.plugin_id, &manifest));
        let (store, bindings) = Self::instantiate(store, &component, &linker)?;
        Ok(Self { engine, component, linker, store: Mutex::new(store), bindings: Mutex::new(bindings), manifest, path, supervisor_state: Mutex::new(ProgramSupervisorState::Running) })
    }

    pub fn hot_reload(&mut self) -> Result<(), PluginHostError> {
        *self.supervisor_state.lock().map_err(|_| PluginHostError::LockPoisoned("supervisor"))? = ProgramSupervisorState::Restarting;
        let wasm_bytes = std::fs::read(&self.path)?;
        let component = Component::from_binary(&self.engine, &wasm_bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        self.manifest = Self::read_manifest(&self.engine, &component, &self.linker)?;
        let store = Store::new(&self.engine, Self::host_state(&self.manifest.plugin_id, &self.manifest));
        let (store, bindings) = Self::instantiate(store, &component, &self.linker)?;
        self.component = component;
        *self.store.lock().map_err(|_| PluginHostError::LockPoisoned("plugin store"))? = store;
        *self.bindings.lock().map_err(|_| PluginHostError::LockPoisoned("plugin bindings"))? = bindings;
        *self.supervisor_state.lock().map_err(|_| PluginHostError::LockPoisoned("supervisor"))? = ProgramSupervisorState::Running;
        Ok(())
    }

    pub fn supervisor_state(&self) -> ProgramSupervisorState {
        self.supervisor_state.lock().map(|state| *state).unwrap_or(ProgramSupervisorState::Crashed)
    }

    /// ⛽️ Both `consume_fuel` and `epoch_interruption` are enabled on this runtime's `Engine`
    /// (`build_engine`), so every store MUST have fuel + an epoch deadline set before its first wasm
    /// call or wasmtime traps immediately (`all fuel consumed` / `interrupt`) — a fresh `Store`'s
    /// fuel and epoch deadline both default to zero. Nothing in this runtime increments the engine's
    /// epoch, so `u64::MAX` is effectively "never interrupt" rather than a real cooperative budget.
    fn prepare_call(store: &mut Store<HostState>) {
        store.set_fuel(PLUGIN_FUEL_BUDGET).ok();
        store.set_epoch_deadline(u64::MAX);
    }

    pub fn manifest_json(&self) -> Result<String, PluginHostError> {
        Ok(serde_json::to_string(&self.manifest)?)
    }

    pub fn create_app(&self, app_id: &str) -> Result<u32, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_plugin().call_instantiate_app(&mut *store, app_id, app_id).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let instance_id = Self::plugin_result(result)?;
        let manifest = self.manifest.clone();
        let host = store.data_mut();
        let session = host.ensure_session(instance_id);
        if let Some(app) = manifest.apps.iter().find(|app| app.id == app_id) {
            let doc = app.io.document_schema.clone();
            if !doc.is_empty() {
                session.document_schema = Some(doc);
            }
        }
        Ok(instance_id)
    }

    pub fn destroy_app(&self, instance_id: u32) {
        if let Ok(mut store) = self.store_guard() {
            store.data_mut().sessions.remove(&instance_id);
        }
    }

    /// Bind document/config/draft schema ids so Emit can fold through ArtifactCodec.
    pub fn bind_session_schemas(&self, instance_id: u32, document_schema: impl Into<Option<String>>, config_schema: impl Into<Option<String>>, draft_schema: impl Into<Option<String>>) {
        if let Ok(mut store) = self.store_guard() {
            let session = store.data_mut().ensure_session(instance_id);
            session.document_schema = document_schema.into();
            session.config_schema = config_schema.into();
            session.draft_schema = draft_schema.into();
        }
    }

    /// 👁 Host-authoritative opaque packs for `instance_id`, if a session was allocated.
    pub fn document_session(&self, instance_id: u32) -> Result<Option<ArtifactSession>, PluginHostError> {
        let store = self.store_guard()?;
        Ok(store.data().sessions.get(&instance_id).cloned())
    }

    /// @emoji 🔗️ Registers the native-side backbone endpoint the sandboxed plugin's `backbone-send`/
    /// `backbone-poll`/`backbone-status` host calls operate against, keyed by uri. WS-E calls this
    /// with a `sync::ArtifactHost`-backed backbone once the actor layer is wired; until then it is an
    /// explicit in-process registration (there is no native URI→IO resolution in this crate anymore).
    pub fn register_host_backbone(&self, uri: &str, backbone: Box<dyn store::Backbone>) -> Result<(), PluginHostError> {
        let mut store = self.store_guard()?;
        store.data_mut().backbones.insert(uri.to_string(), backbone);
        Ok(())
    }

    /// @emoji ✂️ Removes a previously registered native backbone endpoint.
    pub fn deregister_host_backbone(&self, uri: &str) -> Result<(), PluginHostError> {
        let mut store = self.store_guard()?;
        store.data_mut().backbones.remove(uri);
        Ok(())
    }

    /// @emoji 📦️ Registers the native-side `BlobStore` the sandboxed plugin's `write-blob`/`read-blob`
    /// host calls operate against. Not granted by default (unlike backbones there is no capability
    /// gate on these two calls today — every program that links `write-blob`/`read-blob` gets them once
    /// a store is registered); callers that embed this runtime decide when/whether to call this.
    pub fn register_host_blob_store(&self, store: Arc<dyn store::BlobStore>) -> Result<(), PluginHostError> {
        let mut plugin_store = self.store_guard()?;
        plugin_store.data_mut().blob_store = Some(store);
        Ok(())
    }

    /// @emoji ✂️ Removes the previously registered native blob store.
    pub fn deregister_host_blob_store(&self) -> Result<(), PluginHostError> {
        let mut plugin_store = self.store_guard()?;
        plugin_store.data_mut().blob_store = None;
        Ok(())
    }

    /// 🌉️ Registers the shared `IoRouter` this plugin's `host.io-dialects`/`host.io-compose` calls
    /// route through. Callers that load multiple plugins into one process (e.g. `WasmtimeNodeHost`)
    /// should build ONE `IoRouter`, call `IoRouter::register_plugin` for each loaded runtime, and
    /// register that same shared router on every one of them.
    pub fn register_host_io_router(&self, router: Arc<IoRouter>) -> Result<(), PluginHostError> {
        let mut plugin_store = self.store_guard()?;
        plugin_store.data_mut().io_router = Some(router);
        Ok(())
    }

    /// @emoji ⚙️ Registers a compute kernel on the host `EngineCache` under its `ENGINE_ID`.
    pub fn register_engine<E: store::Engine>(&self, engine: E) -> Result<(), PluginHostError> {
        let mut plugin_store = self.store_guard()?;
        plugin_store.data_mut().engines.register(engine);
        Ok(())
    }

    /// @emoji 🔀️ The single bidirectional entry point onto `semio:framework/plugin.exchange` — every
    /// former per-verb call (`handle-action`, `handle-command`, `update-window`, `refresh-ui`,
    /// `context-menu`, `apply-operations[-text]`, `read/load-app-document-{text,pack}`,
    /// `attach/detach-backbone`, `consume/produce-media`) is now just a caller-encoded
    /// `protocol_channel::AppCommand` batch forwarded here; the result is every `AppFrame` the batch
    /// produced plus anything queued since the previous call. `exchange(id, [])` is a pure drain, the
    /// heartbeat tick. Host mirrors LoadDocument/LoadConfig inputs and Document/Config/Draft/Emit
    /// outputs into the per-instance {@link ArtifactSession} pack authority.
    pub fn exchange(&self, instance_id: u32, commands: Vec<Vec<u8>>) -> Result<Vec<Vec<u8>>, PluginHostError> {
        let mut store = self.store_guard()?;
        store.data_mut().ensure_session(instance_id);
        Self::pre_adopt_command_packs(store.data_mut(), instance_id, &commands);
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_plugin().call_exchange(&mut *store, instance_id, &commands).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let frames = Self::plugin_result(result)?;
        Self::post_adopt_frame_packs(store.data_mut(), instance_id, &frames);
        Ok(frames)
    }

    fn pre_adopt_command_packs(host: &mut HostState, instance_id: u32, commands: &[Vec<u8>]) {
        use protocol::{decode_app_command, AppCommand};
        for bytes in commands {
            let Ok(command) = decode_app_command(bytes) else { continue };
            match command {
                AppCommand::LoadDocument { pack, spr, .. } => {
                    host.adopt_document(instance_id, pack, spr, String::new());
                }
                AppCommand::LoadConfig { pack, spr, .. } => {
                    host.adopt_config(instance_id, pack, spr, String::new());
                }
                AppCommand::Hello { config, .. } if !config.is_empty() => {
                    if let Ok((pack, spr)) = store::decode_document_pack_bytes(&config) {
                        host.adopt_config(instance_id, pack, spr, String::new());
                    }
                }
                AppCommand::PureCommand { document, document_spr, config, config_spr, draft, draft_spr, .. } => {
                    if !document.is_empty() || !document_spr.is_empty() {
                        host.adopt_document(instance_id, document, document_spr, String::new());
                    }
                    if !config.is_empty() || !config_spr.is_empty() {
                        host.adopt_config(instance_id, config, config_spr, String::new());
                    }
                    if !draft.is_empty() || !draft_spr.is_empty() {
                        host.adopt_draft(instance_id, draft, draft_spr, String::new());
                    }
                }
                // ⚖️ Mirrors this session's own view of its local/authority `MergePolicy` the exact
                // moment the command that sets it is SENT, not once the reply comes back — same
                // "mirror on the way past" convention `LoadDocument`/`LoadConfig` above already use.
                AppCommand::SetMergePolicy { policy, .. } => {
                    if let Some(policy) = protocol::MergePolicy::from_u8(policy) {
                        host.ensure_session(instance_id).merge_policy = policy;
                    }
                }
                _ => {}
            }
        }
    }

    fn post_adopt_frame_packs(host: &mut HostState, instance_id: u32, frames: &[Vec<u8>]) {
        use protocol::{decode_app_frame, AppFrame};
        for bytes in frames {
            let Ok(frame) = decode_app_frame(bytes) else { continue };
            match frame {
                AppFrame::Document { pack, spr, ops, .. } => {
                    host.adopt_document(instance_id, pack, spr, ops);
                }
                AppFrame::Config { pack, spr, ops, .. } => {
                    host.adopt_config(instance_id, pack, spr, ops);
                }
                AppFrame::Draft { pack, spr, ops, .. } => {
                    host.adopt_draft(instance_id, pack, spr, ops);
                }
                AppFrame::Emit { document_ops, config_ops, draft_ops, .. } => {
                    let session = host.ensure_session(instance_id);
                    let document_schema = session.document_schema.clone().or_else(|| store::lane_schema_from_spr(&session.document.spr));
                    let config_schema = session.config_schema.clone().or_else(|| store::lane_schema_from_spr(&session.config.spr));
                    let draft_schema = session.draft_schema.clone().or_else(|| store::lane_schema_from_spr(&session.draft.spr));
                    session.document.apply_emit_ops(document_schema.as_deref(), document_ops);
                    session.config.apply_emit_ops(config_schema.as_deref(), config_ops);
                    session.draft.apply_emit_ops(draft_schema.as_deref(), draft_ops);
                    session.command_log_len = session.command_log_len.saturating_add(1);
                    session.generation = session.generation.saturating_add(1);
                }
                // 🧾 Mirrors a successful dispatch's `DispatchReport` messages — the trailing
                // CHANNEL_VERSION 11 addition on `AppFrame::Invocation` (contract-freeze.md §C8/C9).
                AppFrame::Invocation { messages, .. } if !messages.is_empty() => {
                    if let Ok(report) = decode_dispatch_report(&messages) {
                        host.ensure_session(instance_id).last_dispatch_messages = report.messages;
                    }
                }
                // 🧾 Mirrors a REJECTED dispatch's `DispatchReport` messages — `AppFrame::Error`'s
                // trailing `report`, present whenever `fault.code == "mutation.rejected"`.
                AppFrame::Error { report, .. } if !report.is_empty() => {
                    if let Ok(dispatch_report) = decode_dispatch_report(&report) {
                        host.ensure_session(instance_id).last_dispatch_messages = dispatch_report.messages;
                    }
                }
                // 🔀 Mirrors the unsolicited merge report pushed after every ingest.
                AppFrame::MergeReport { report, .. } => {
                    if let Ok(merge_report) = decode_merge_report(&report) {
                        host.ensure_session(instance_id).last_merge_report = Some(merge_report);
                    }
                }
                // ⚔️ Mirrors this artifact's open-conflict projection — pushed unsolicited after every
                // ingest, and also the reply to `AppCommand::ReadConflicts`.
                AppFrame::Conflicts { conflicts, .. } => {
                    if let Ok(list) = decode_conflicts(&conflicts) {
                        host.ensure_session(instance_id).open_conflicts = list;
                    }
                }
                _ => {}
            }
        }
    }

    /// 🖱️ On-demand context menu via `AppCommand::ContextMenu` on the plugin exchange channel.
    pub fn context_menu(&self, instance_id: u32, request: serde_json::Value) -> Result<Vec<ui_wgpu::wgpu::ContextMenuItemSpec>, PluginHostError> {
        use protocol::{decode_app_frame, encode_app_command, AppCommand, AppFrame};
        let seq = next_host_seq();
        let request_dsl = dsl::to_dsl_value(&request).map_err(|error| PluginHostError::Plugin(error))?;
        let request_bytes = store::pack_rt::encode_wire_value(&request_dsl);
        let command = AppCommand::ContextMenu { seq, request: request_bytes };
        let frames = self.exchange(instance_id, vec![encode_app_command(&command)])?;
        for bytes in frames {
            let frame = decode_app_frame(&bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
            match frame {
                AppFrame::ContextMenu { in_reply_to, items } if in_reply_to == seq => {
                    let value = store::pack_rt::decode_wire_value(&items).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
                    return dsl::from_dsl_value(value).map_err(|error| PluginHostError::Plugin(error));
                }
                AppFrame::Error { in_reply_to, fault, .. } if in_reply_to == Some(seq) => {
                    let decoded = dsl::decode_fault_bytes(&fault);
                    return Err(PluginHostError::Plugin(decoded.message));
                }
                _ => {}
            }
        }
        Ok(Vec::new())
    }

    /// 🤝️ The instance handshake: `AppCommand::Hello` (channel-version negotiation + optional
    /// initial config) batched with an immediate `AppCommand::SetMergePolicy`, in the SAME `exchange`
    /// call — this session's local/authority `MergePolicy` is established before any other command
    /// can reach the instance, satisfying contract-freeze.md §C9's "`Hello.config` seeds policy"
    /// without a second wire mechanism (the frozen §C8 tag table adds no policy field to `Hello`
    /// itself; `SetMergePolicy` already IS the wire command for this — see `📋️contract-freeze.md`).
    /// Returns every decoded reply frame (`Welcome` plus the policy command's `Done`/`Error`).
    pub fn hello(&self, instance_id: u32, app_id: &str, actor: &str, config: Vec<u8>, merge_policy: protocol::MergePolicy) -> Result<Vec<protocol::AppFrame>, PluginHostError> {
        use protocol::{decode_app_frame, encode_app_command, AppCommand};
        let policy_seq = next_host_seq();
        let commands = vec![
            encode_app_command(&AppCommand::Hello { channel_version: protocol::CHANNEL_VERSION, app_id: app_id.to_string(), actor: actor.to_string(), config }),
            encode_app_command(&AppCommand::SetMergePolicy { seq: policy_seq, policy: merge_policy.as_u8() }),
        ];
        let frames = self.exchange(instance_id, commands)?;
        frames.iter().map(|bytes| decode_app_frame(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))).collect()
    }

    /// ⚖️ Sends `AppCommand::SetMergePolicy` on its own (outside the initial handshake — e.g. a
    /// mid-session policy change from a settings toggle). See `hello` for the handshake-time form.
    pub fn set_merge_policy(&self, instance_id: u32, policy: protocol::MergePolicy) -> Result<(), PluginHostError> {
        use protocol::{decode_app_frame, encode_app_command, AppCommand, AppFrame};
        let seq = next_host_seq();
        let command = AppCommand::SetMergePolicy { seq, policy: policy.as_u8() };
        let frames = self.exchange(instance_id, vec![encode_app_command(&command)])?;
        for bytes in frames {
            if let Ok(AppFrame::Error { in_reply_to: Some(reply), fault, .. }) = decode_app_frame(&bytes) {
                if reply == seq {
                    return Err(PluginHostError::Plugin(dsl::decode_fault_bytes(&fault).message));
                }
            }
        }
        Ok(())
    }

    /// ⚔️ `AppCommand::ResolveConflict` pass-through — returns the authoritative `MergeReport` this
    /// resolution produced (contract-freeze.md §C5/C6/C9).
    pub fn resolve_conflict(&self, instance_id: u32, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, PluginHostError> {
        use protocol::{decode_app_frame, encode_app_command, AppCommand, AppFrame};
        let seq = next_host_seq();
        let resolution_wire = match resolution {
            protocol::ConflictResolution::Accept => 0,
            protocol::ConflictResolution::Discard => 1,
        };
        let command = AppCommand::ResolveConflict { seq, conflict_id: conflict_id.to_string(), resolution: resolution_wire };
        let frames = self.exchange(instance_id, vec![encode_app_command(&command)])?;
        for bytes in &frames {
            match decode_app_frame(bytes) {
                Ok(AppFrame::MergeReport { in_reply_to: Some(reply), report }) if reply == seq => return decode_merge_report(&report),
                Ok(AppFrame::Error { in_reply_to: Some(reply), fault, .. }) if reply == seq => return Err(PluginHostError::Plugin(dsl::decode_fault_bytes(&fault).message)),
                _ => {}
            }
        }
        Err(PluginHostError::Plugin(format!("no MergeReport reply for seq {seq}")))
    }

    /// ⚔️ `AppCommand::ReadConflicts` pass-through — this artifact's currently open conflicts.
    pub fn read_conflicts(&self, instance_id: u32) -> Result<Vec<protocol::Conflict>, PluginHostError> {
        use protocol::{decode_app_frame, encode_app_command, AppCommand, AppFrame};
        let seq = next_host_seq();
        let frames = self.exchange(instance_id, vec![encode_app_command(&AppCommand::ReadConflicts { seq })])?;
        for bytes in &frames {
            if let Ok(AppFrame::Conflicts { in_reply_to: Some(reply), conflicts }) = decode_app_frame(bytes) {
                if reply == seq {
                    return decode_conflicts(&conflicts);
                }
            }
        }
        Ok(Vec::new())
    }

    /// 📦️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION (D3): this
    /// plugin's own composer roster — JSON `Vec<(ArtifactDialect, Vec<ArtifactDialect>)>` bytes,
    /// straight off the WIT `list-artifact-dialects` export. Called once per plugin at load time
    /// by whichever `IoRouter` owns this runtime.
    pub fn list_artifact_dialects(&self) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_plugin().call_list_artifact_dialects(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// 📦️ Composes `sources` against THIS plugin's registry entry for `key` (JSON wire bytes, same
    /// shapes `io::wire_artifact_compose` uses) — the WIT `artifact-compose` export. Callers
    /// (an `IoRouter`) are expected to have already confirmed this plugin owns `key`; a genuine
    /// miss surfaces as the same "no composer registered" message `io::resolve` would produce.
    pub fn artifact_compose(&self, key: &[u8], sources: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_plugin().call_artifact_compose(&mut *store, key, sources).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// 📇️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): this plugin's own registered NEW io
    /// mechanism roster — JSON `Vec<io_schema::IoEntryDescriptor>` bytes, the WIT `list-io-entries`
    /// export. Called once per plugin at load time by `IoRouter::register_plugin`, additive
    /// alongside `list_artifact_dialects` (D3).
    pub fn list_io_entries(&self) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_plugin().call_list_io_entries(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// 🌉️ Executes ONE hop of THIS plugin's own local io mechanism registry — the WIT `io-run`
    /// guest export. `from`/`into` are `ArtifactDialect::to_coordinate()` strings; `payload`/the ok
    /// result are JSON `io_schema::IoPayload` bytes. Callers (the host `IoRouter::run_io`) are
    /// expected to have already confirmed this plugin owns `(from, into)`.
    pub fn io_run(&self, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_plugin().call_io_run(&mut *store, from, into, &payload).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// 🔍️ Sniffs THIS plugin's own `(from, into)` hop — the WIT `io-sniff` guest export. Returns the
    /// raw `io_schema::Confidence::rank()` byte (`0`=None..`3`=High).
    pub fn io_sniff(&self, from: &str, into: &str, payload: &[u8]) -> Result<u8, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_plugin().call_io_sniff(&mut *store, from, into, payload).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// 💡️ Reads this guest's deterministic executable inference roster once at load time. Moved
    /// (PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS §6) from the `plugin`
    /// interface bindings to the `contributor` interface bindings — same call idiom, new accessor.
    pub fn list_artifact_inferences(&self) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_contributor().call_list_artifact_inferences(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// 💡️ Executes one serialized guest inference call. Holding the runtime's store mutex for
    /// the complete call applies the same single-flight/reentrancy boundary as every other export.
    /// Moved (§6) to the `contributor` interface bindings — same call idiom, new accessor.
    pub fn artifact_infer(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let req_struct: semio::framework::types::ArtifactInferenceRequest = serde_json::from_slice(request)?;
        let result_struct = bindings.semio_framework_contributor().call_artifact_infer(&mut *store, &req_struct).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let result_struct = Self::plugin_result(result_struct)?;
        serde_json::to_vec(&result_struct).map_err(|error| PluginHostError::Wasmtime(error.to_string()))
    }

    /// 🎯️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (§6): this guest's
    /// deterministic wire roster of contributed artifact mutations against OTHER plugins' artifact
    /// kinds — the WIT `contributor.list-artifact-mutations` export, mirroring `list_artifact_inferences`.
    /// Wave 0 wiring only; the guest side is a W1-A placeholder returning an empty roster until W1-A
    /// lands the real `plugin_runtime` implementation.
    pub fn list_artifact_mutations(&self) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_contributor().call_list_artifact_mutations(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// 🎯️ Plans one contributed mutation against a target artifact this plugin does not own — the
    /// WIT `contributor.artifact-mutation-plan` export, mirroring `artifact_infer`'s call idiom.
    /// `request`/result are opaque wire bytes owned by W1-A's guest SDK layer; this is Wave 0 ABI
    /// wiring only, later consumed by the transaction protocol (contract freeze §5), not built here.
    pub fn artifact_mutation_plan(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let result = bindings.semio_framework_contributor().call_artifact_mutation_plan(&mut *store, request).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result)
    }

    /// @emoji 🩹️ Mirrors the WIT `migrate-document` call unchanged — `input`/output `data` is
    /// pack-container bytes (see `document-pack-files`).
    pub fn migrate_artifact(&self, from_version: &str, to_version: &str, data: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        let input = semio::framework::types::MigrateArtifactInput { from_version: from_version.to_string(), to_version: to_version.to_string(), data };
        let result = bindings.semio_framework_plugin().call_migrate_artifact(&mut *store, &input).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::plugin_result(result).map(|output| output.data)
    }

    /// @emoji 🩹️ Clears the plugin's single-flight instance guard after a wasm trap skipped `Drop`.
    /// Callers must only invoke this between serialized top-level calls — never while another call is
    /// in flight (mirrors the WIT doc's own caveat).
    pub fn clear_instance_guard(&self) -> Result<(), PluginHostError> {
        let mut store = self.store_guard()?;
        let bindings = self.bindings_guard()?;
        Self::prepare_call(&mut store);
        bindings.semio_framework_plugin().call_clear_instance_guard(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))
    }

    pub fn utilities(&self, _instance_id: u32, _view_state: &ViewModel) -> Result<Vec<UtilityNode>, PluginHostError> {
        Ok(Vec::new())
    }

    pub fn window_engagements(&self, _instance_id: u32, _view_state: &ViewModel) -> Result<HashMap<String, WindowEngagement>, PluginHostError> {
        Ok(HashMap::new())
    }

    pub fn window_measures(&self, _instance_id: u32, _view_state: &ViewModel) -> Result<HashMap<String, Vec<WindowMeasure>>, PluginHostError> {
        Ok(HashMap::new())
    }

    fn read_manifest(engine: &Engine, component: &Component, linker: &Linker<HostState>) -> Result<PluginManifest, PluginHostError> {
        let manifest = PluginManifest { plugin_id: "unknown".into(), label: "Unknown".into(), version: "0.0.0".into(), apps: vec![], examples: vec![], capabilities: vec![], topic_contributions: vec![], commands: vec![], artifact_kinds: vec![], dependencies: vec![], contributions: vec![] };
        let mut store = Store::new(engine, Self::host_state("bootstrap", &manifest));
        Self::prepare_call(&mut store);
        let (bindings, _instance) = PluginWorld::instantiate(&mut store, component, linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::prepare_call(&mut store);
        let wire_bytes = bindings.semio_framework_plugin().call_manifest(&mut store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let value = store::pack_rt::decode_wire_value(&wire_bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        let value = store::pack_rt::renormalize_whole_number_floats(value);
        Ok(dsl::from_dsl_value(value).map_err(|error| PluginHostError::Plugin(error))?)
    }

    fn instantiate(mut store: Store<HostState>, component: &Component, linker: &Linker<HostState>) -> Result<(Store<HostState>, PluginWorld), PluginHostError> {
        Self::prepare_call(&mut store);
        let (bindings, _instance) = PluginWorld::instantiate(&mut store, component, linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok((store, bindings))
    }
}
//#endregion 🔖️WasmPluginRuntime

//#region 🔖️ExtensionRuntime
/// 🧩️ `extension-world` bindings, isolated in their own submodule so the generated `semio::framework::*`
/// tree doesn't collide with the `plugin-world` bindings' identically-named tree above — wasmtime's
/// `bindgen!` cannot be invoked twice at the same module scope.
mod extension_bindings {
    wasmtime::component::bindgen!({
        world: "actor",
        path: "../../../🧬️schema",
        async: false,
        with: {
            "semio:framework/types": crate::semio::framework::types,
        },
    });
}

/// 📦️ Host-side mirror of the guest `ExtensionManifest` (defined in the plugin guest SDK crate,
/// which this host crate does not depend on) — decoded from the same `extension.manifest` wire bytes.
#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionManifest {
    pub extension_id: String,
    pub label: String,
    pub version: String,
    pub extends: String,
    #[serde(default)]
    pub capabilities: Vec<CapabilityRequirement>,
    #[serde(default)]
    pub topic_contributions: Vec<TopicContribution>,
}

struct ExtensionHostState {
    wasi: WasiCtx,
    table: ResourceTable,
    extension_id: String,
}

impl WasiView for ExtensionHostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

/// 🕳️ Every capability this host doesn't yet back for extensions faults as not-implemented — this
/// runtime is purely additive infra (not wired into any boot sequence yet), so there is no
/// `EngineCache`/`IoRouter`/backbone registry to route these through until a later wave wires one in.
impl extension_bindings::semio::framework::host::Host for ExtensionHostState {
    fn log(&mut self, level: String, message: String) {
        eprintln!("[extension:{}:{level}] {message}", self.extension_id);
    }

    fn now_ms(&mut self) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
    }

    fn read_artifact(&mut self, _handle: u64) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.read-artifact", "read-artifact not implemented for extension host"))
    }

    fn write_artifact(&mut self, _handle: u64, _payload: Vec<u8>) -> Result<(), Vec<u8>> {
        Err(host_fault_bytes("os.host.write-artifact", "write-artifact not implemented for extension host"))
    }

    fn open_window(&mut self, _kind: String, _params: Vec<u8>) -> Result<u64, Vec<u8>> {
        Err(host_fault_bytes("os.host.open-window", "open-window not implemented for extension host"))
    }

    fn invoke_action(&mut self, _target: String, _invocation: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.invoke-action", "invoke-action not implemented for extension host"))
    }

    fn read_asset(&mut self, handle: u64) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.read-asset", format!("read-asset: unknown handle {handle}")))
    }

    fn network_fetch(&mut self, _origin: String, _path: String) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.network-fetch", "network-fetch not implemented for extension host"))
    }

    fn write_blob(&mut self, _data: Vec<u8>, _media_type: String) -> Result<String, Vec<u8>> {
        Err(host_fault_bytes("os.host.write-blob", "write-blob not implemented for extension host"))
    }

    fn read_blob(&mut self, hash: String) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.read-blob", format!("blob not found: {hash}")))
    }

    fn backbone_send(&mut self, uri: String, _message: Vec<u8>) -> Result<(), Vec<u8>> {
        Err(host_fault_bytes("os.host.backbone-send", format!("backbone unavailable: {uri}")))
    }

    fn backbone_poll(&mut self, uri: String) -> Result<Vec<Vec<u8>>, Vec<u8>> {
        Err(host_fault_bytes("os.host.backbone-poll", format!("backbone unavailable: {uri}")))
    }

    fn backbone_status(&mut self, _uri: String) -> Result<String, Vec<u8>> {
        Ok("detached".into())
    }

    fn engine_derive(&mut self, _engine_id: String, _input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.engine-derive", "engine-derive not implemented for extension host"))
    }

    fn io_dialects(&mut self, _artifact_kind: String, _direction: String) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.io-dialects", "io-dialects not implemented for extension host"))
    }

    fn io_compose(&mut self, _key: Vec<u8>, _sources: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.io-compose", "io-compose not implemented for extension host"))
    }

    fn engine_read(&mut self, _engine_id: String, _key: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.engine-read", "engine-read not implemented for extension host"))
    }

    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): same stub shape as `HostState`'s own
    /// `resolve_artifact_link` — `string` error, not `list<u8>` Fault bytes.
    fn resolve_artifact_link(&mut self, _link: Vec<u8>) -> Result<Vec<u8>, String> {
        Err("resolve-artifact-link not implemented for extension host".to_string())
    }

    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): same "purely additive, not wired into any
    /// boot sequence" stub shape as every other `Host` fn on `ExtensionHostState` above.
    fn io_routes(&mut self, _from: String, _into: String) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.io-routes", "io-routes not implemented for extension host"))
    }

    fn io_run(&mut self, _from: String, _into: String, _payload: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.io-run", "io-run not implemented for extension host"))
    }

    fn io_identify(&mut self, _payload: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(host_fault_bytes("os.host.io-identify", "io-identify not implemented for extension host"))
    }
}

/// 🧩️ One instantiated `extension-world` component: its wasmtime store/bindings plus decoded manifest.
struct LoadedExtension {
    store: Mutex<Store<ExtensionHostState>>,
    bindings: extension_bindings::ExtensionWorld,
    manifest: ExtensionManifest,
}

/// 🧩️ Native wasmtime host for `extension-world` components — mirrors `WasmPluginRuntime`'s
/// load/instantiate pattern but keyed by extension id in an instance table, since a process loads
/// many small extensions rather than one big plugin. Purely additive: nothing in the boot sequence
/// instantiates this yet (a later wave wires it in once producers migrate off the
/// `plugin-world`-as-extension workaround).
pub struct ExtensionRuntime {
    engine: Engine,
    linker: Linker<ExtensionHostState>,
    instances: Mutex<HashMap<String, Arc<LoadedExtension>>>,
}

impl ExtensionRuntime {
    fn build_engine() -> Result<Engine, PluginHostError> {
        let mut config = Config::new();
        config.consume_fuel(true);
        config.epoch_interruption(true);
        config.wasm_component_model(true);
        Engine::new(&config).map_err(|error| PluginHostError::Wasmtime(error.to_string()))
    }

    fn build_linker(engine: &Engine) -> Result<Linker<ExtensionHostState>, PluginHostError> {
        let mut linker = Linker::new(engine);
        extension_bindings::semio::framework::host::add_to_linker(&mut linker, |state| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(linker)
    }

    /// 🏗️ Fresh runtime with its own wasmtime `Engine` + capability `Linker`; no extensions loaded yet.
    pub fn new() -> Result<Self, PluginHostError> {
        let engine = Self::build_engine()?;
        let linker = Self::build_linker(&engine)?;
        Ok(Self { engine, linker, instances: Mutex::new(HashMap::new()) })
    }

    fn host_state(extension_id: &str) -> ExtensionHostState {
        ExtensionHostState { wasi: WasiCtxBuilder::new().build(), table: ResourceTable::new(), extension_id: extension_id.to_string() }
    }

    /// ⛽️ Both `consume_fuel` and `epoch_interruption` are enabled on this runtime's `Engine`
    /// (`build_engine`), so every store MUST have fuel + an epoch deadline set before its first wasm
    /// call or wasmtime traps immediately (`all fuel consumed` / `interrupt`) — a fresh `Store`'s
    /// fuel and epoch deadline both default to zero. Nothing in this runtime increments the engine's
    /// epoch, so `u64::MAX` is effectively "never interrupt" rather than a real cooperative budget.
    fn prepare_call(store: &mut Store<ExtensionHostState>) {
        store.set_fuel(PLUGIN_FUEL_BUDGET).ok();
        store.set_epoch_deadline(u64::MAX);
    }

    fn extension_result<T>(result: Result<T, extension_bindings::semio::framework::types::PluginError>) -> Result<T, PluginHostError> {
        result.map_err(|error| match error {
            extension_bindings::semio::framework::types::PluginError::Fault(bytes) => PluginHostError::Plugin(dsl::decode_fault_bytes(&bytes).message),
        })
    }

    fn decode_manifest(store: &mut Store<ExtensionHostState>, bindings: &extension_bindings::ExtensionWorld) -> Result<ExtensionManifest, PluginHostError> {
        let wire_bytes = bindings.semio_framework_extension().call_manifest(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let value = store::pack_rt::decode_wire_value(&wire_bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        let value = store::pack_rt::renormalize_whole_number_floats(value);
        dsl::from_dsl_value(value).map_err(PluginHostError::Plugin)
    }

    /// 📦️ Instantiates `wasm_bytes` as an `extension-world` component, calls its `manifest()` +
    /// `activate()`, and keys it in this runtime's instance table by the manifest's own
    /// `extension_id` (the caller doesn't pick the id — it's authoritative from the guest). Returns
    /// that id.
    pub fn load_bytes(&self, wasm_bytes: &[u8]) -> Result<String, PluginHostError> {
        let component = Component::from_binary(&self.engine, wasm_bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let mut store = Store::new(&self.engine, Self::host_state("bootstrap"));
        Self::prepare_call(&mut store);
        let (bindings, _instance) = extension_bindings::ExtensionWorld::instantiate(&mut store, &component, &self.linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let manifest = Self::decode_manifest(&mut store, &bindings)?;
        store.data_mut().extension_id = manifest.extension_id.clone();
        Self::prepare_call(&mut store);
        let activation = bindings.semio_framework_extension().call_activate(&mut store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Self::extension_result(activation)?;
        let extension_id = manifest.extension_id.clone();
        let loaded = LoadedExtension { store: Mutex::new(store), bindings, manifest };
        self.instances.lock().map_err(|_| PluginHostError::LockPoisoned("extension instances"))?.insert(extension_id.clone(), Arc::new(loaded));
        Ok(extension_id)
    }

    /// 📁 Reads `path` off disk and loads it the same way `load_bytes` does.
    pub fn load(&self, path: impl AsRef<Path>) -> Result<String, PluginHostError> {
        let wasm_bytes = std::fs::read(path)?;
        self.load_bytes(&wasm_bytes)
    }

    /// ✂️ Calls `deactivate()` and drops the loaded instance from the table.
    pub fn unload(&self, extension_id: &str) -> Result<(), PluginHostError> {
        let loaded = self.instances.lock().map_err(|_| PluginHostError::LockPoisoned("extension instances"))?.remove(extension_id);
        if let Some(loaded) = loaded {
            let mut store = loaded.store.lock().map_err(|_| PluginHostError::LockPoisoned("extension store"))?;
            Self::prepare_call(&mut store);
            loaded.bindings.semio_framework_extension().call_deactivate(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        }
        Ok(())
    }

    /// 👁️ The decoded manifest of a loaded extension, if one is registered under `extension_id`.
    pub fn manifest(&self, extension_id: &str) -> Option<ExtensionManifest> {
        self.instances.lock().ok()?.get(extension_id).map(|loaded| loaded.manifest.clone())
    }

    /// 🔀️ Routes `capability`/`request` to the loaded extension's `invoke` export. Unlike
    /// `WasmPluginRuntime`'s methods (which surface `PluginHostError`), this matches the WIT ABI's
    /// own fault channel one level higher and returns `Fault` directly.
    pub fn extension_invoke(&self, extension_id: &str, capability: &str, request: &[u8]) -> Result<Vec<u8>, dsl::Fault> {
        let loaded = {
            let instances = self.instances.lock().map_err(|_| dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("extension.lock-poisoned"), "extension instances lock poisoned"))?;
            instances.get(extension_id).cloned().ok_or_else(|| dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("extension.unknown"), format!("no extension loaded with id `{extension_id}`")))?
        };
        let mut store = loaded.store.lock().map_err(|_| dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("extension.lock-poisoned"), "extension store lock poisoned"))?;
        Self::prepare_call(&mut store);
        let result = loaded.bindings.semio_framework_extension().call_invoke(&mut *store, capability, request).map_err(|error| dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("extension.wasmtime"), error.to_string()))?;
        result.map_err(|error| match error {
            extension_bindings::semio::framework::types::PluginError::Fault(bytes) => dsl::decode_fault_bytes(&bytes),
        })
    }
}
//#endregion 🔖️ExtensionRuntime

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_plugin_runtime_api_exists() {
        let _ = size_of::<WasmPluginRuntime>();
    }

    #[test]
    fn extension_runtime_constructs_engine_and_linker() {
        let runtime = ExtensionRuntime::new().expect("extension runtime engine/linker build");
        assert!(runtime.manifest("nonexistent").is_none());
        let error = runtime.extension_invoke("nonexistent", "noop", &[]).expect_err("unknown extension id must fault");
        assert_eq!(error.code.0, "extension.unknown");
    }

    #[test]
    fn wasm_plugin_runtime_loads_real_plugin_component_if_present() {
        let path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/puzzle/semio_s_plugin_puzzle_component.core.wasm");
        if path.exists() {
            let runtime = WasmPluginRuntime::load(path).expect("WasmPluginRuntime::load should succeed without epoch/fuel trap");
            assert!(!runtime.manifest.plugin_id.is_empty());
        }
    }

    //#region 🔖️IoRouterW1d
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW `IoRouter` mechanism's own
    /// route-resolution/determinism/reentrancy tests. Pure — `resolve_io_route`/
    /// `route_reenters_calling_plugin` take a synthetic `BTreeMap<IoEntryKey, IoEntryRoute>`
    /// directly, no `Arc<WasmPluginRuntime>`/real wasm component needed — so these run on every
    /// CI/dev machine, unlike `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_
    /// plugins` below.
    fn io_dialect(kind: &str, standard: &str, subset: &str) -> semio_framework::io_schema::ArtifactDialect {
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
    fn io_router_w1d_fixture_entries() -> Vec<(&'static str, semio_framework::io_schema::IoEntryDescriptor)> {
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*");
        let gif_87a = io_dialect("s.stdio.gif", "87a", "*");
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*");
        vec![
            ("stdio", semio_framework::io_schema::IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true }),
            ("gif", semio_framework::io_schema::IoEntryDescriptor { from: gif_87a, into: gif_89a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Canonical, sniffs: false }),
            ("gif", semio_framework::io_schema::IoEntryDescriptor { from: binary_raw, into: gif_89a, fidelity: semio_framework::io_schema::IoFidelity::Lossy, sniffs: true }),
        ]
    }

    /// 🏗️ `IoRouter::register_plugin`'s io-entries merge, without needing a real
    /// `Arc<WasmPluginRuntime>` — builds the SAME `BTreeMap<IoEntryKey, IoEntryRoute>` shape
    /// directly from `(owner, descriptor)` rows, inserted in WHATEVER order `rows` lists them.
    fn build_io_entry_graph(rows: &[(&'static str, semio_framework::io_schema::IoEntryDescriptor)]) -> BTreeMap<IoEntryKey, IoEntryRoute> {
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
    #[test]
    fn io_router_route_is_deterministic_across_load_order() {
        let forward = io_router_w1d_fixture_entries();
        let mut reversed = forward.clone();
        reversed.reverse();
        let graph_forward = build_io_entry_graph(&forward);
        let graph_reversed = build_io_entry_graph(&reversed);
        assert_eq!(graph_forward, graph_reversed, "the merged graph itself must not depend on registration order");

        let binary_raw = io_dialect("s.stdio.binary", "raw", "*");
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*");
        let route_forward = resolve_io_route(&graph_forward, &binary_raw, &gif_89a, 3).expect("forward-order route resolves");
        let route_reversed = resolve_io_route(&graph_reversed, &binary_raw, &gif_89a, 3).expect("reversed-order route resolves");
        assert_eq!(route_forward, route_reversed, "the resolved route must not depend on registration order");
        assert_eq!(route_forward.hops.len(), 2, "the winning route is the 2-hop stdio->gif87a->gif89a path, not the 1-hop lossy shortcut");
    }

    /// ⚖️ Proves the ranking rule's FIRST tie-break: highest minimum fidelity beats fewest hops.
    /// The 1-hop `binary->gif89a` shortcut (Lossy) loses to the 2-hop `binary->gif87a->gif89a`
    /// path (min fidelity Canonical) even though it has fewer hops.
    #[test]
    fn io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops() {
        let graph = build_io_entry_graph(&io_router_w1d_fixture_entries());
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*");
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*");
        let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).expect("route resolves");
        assert_eq!(route.fidelity, semio_framework::io_schema::IoFidelity::Canonical);
        assert_eq!(route.hops.len(), 2);
        assert_eq!(route.hops[0].from, binary_raw);
        assert_eq!(route.hops[1].into, gif_89a);
    }

    /// 🌉️ `max_hops` bound is honored: clamped to 1, only the direct (Lossy) shortcut is reachable.
    #[test]
    fn io_router_route_respects_max_hops() {
        let graph = build_io_entry_graph(&io_router_w1d_fixture_entries());
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*");
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*");
        let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 1).expect("1-hop route resolves");
        assert_eq!(route.hops.len(), 1);
        assert_eq!(route.fidelity, semio_framework::io_schema::IoFidelity::Lossy);
    }

    /// 🔒️ `route_reenters_calling_plugin` — the pure predicate behind `run_io`'s guard. A route
    /// with NO hop owned by the caller is safe (`None`); a route where the caller owns even ONE
    /// hop is refused, naming that hop.
    #[test]
    fn io_router_run_io_reentrancy_guard_predicate() {
        let graph = build_io_entry_graph(&io_router_w1d_fixture_entries());
        let binary_raw = io_dialect("s.stdio.binary", "raw", "*");
        let gif_89a = io_dialect("s.stdio.gif", "89a", "*");
        let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).expect("route resolves");
        assert_eq!(route_reenters_calling_plugin(&graph, &route, "norm"), None, "a plugin owning neither hop is safe");
        let hop = route_reenters_calling_plugin(&graph, &route, "stdio").expect("stdio owns the first hop of this route");
        assert_eq!(hop.0, &binary_raw);
        assert_eq!(hop.1, &io_dialect("s.stdio.gif", "87a", "*"));
        let hop = route_reenters_calling_plugin(&graph, &route, "gif").expect("gif owns the second hop of this route");
        assert_eq!(hop.1, &gif_89a);
    }

    /// 🧯️ A duplicate `(from, into)` claimed by a DIFFERENT plugin than the first registration is a
    /// typed conflict — mirrors `io::io_mechanism`'s own `duplicate_entry_is_a_typed_error` law for
    /// the OLD graph's `IoRouteConflict`, generalized to the NEW `IoEntryRouteConflict`. Exercises
    /// `io_entries_conflict` directly — the SAME function `register_plugin` calls — so this proves
    /// the real preflight rule, not a re-derivation of it, without needing a live wasm component.
    #[test]
    fn io_router_register_plugin_rejects_conflicting_io_entry_ownership() {
        let graph = build_io_entry_graph(&[("stdio", semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*"), into: io_dialect("s.stdio.gif", "87a", "*"), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true })]);

        let same_plugin_reclaim = vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*"), into: io_dialect("s.stdio.gif", "87a", "*"), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true }];
        assert!(io_entries_conflict(&graph, "stdio", &same_plugin_reclaim).is_none(), "the SAME plugin reclaiming its own key must not conflict");

        let different_plugin_claim = vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*"), into: io_dialect("s.stdio.gif", "87a", "*"), fidelity: semio_framework::io_schema::IoFidelity::Lossy, sniffs: false }];
        let conflict = io_entries_conflict(&graph, "gif", &different_plugin_claim).expect("a second plugin claiming the same key must conflict");
        assert!(matches!(conflict, PluginHostError::IoEntryRouteConflict { ref existing_plugin, ref incoming_plugin, .. } if existing_plugin == "stdio" && incoming_plugin == "gif"));
    }
    //#endregion 🔖️IoRouterW1d

    //#region 🔖️IoRouterE2e
    /// 🌉️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W7: the
    /// first real compile-and-run verification of the cross-plugin `IoRouter` wiring built across
    /// W1/W3 — two REAL wasm components (`bun nx run @semio-tech/framework-os-dev:build -- stdio` /
    /// `-- cad`), each loaded into its OWN native `WasmPluginRuntime`, both registered into one
    /// shared `IoRouter` (mirrors exactly how `WasmtimeNodeHost::runtime_for` wires a real dev-boot).
    ///
    /// The routed key: cad's own composer (`🎹️composer/🦀️component.rs`, `EXPORT_STEP_DIALECT`
    /// entry) registers `writes: s.stdio.step@ap214/*, reads: [s.cad@1/*]` — `register_composer_entries`
    /// mirrors that into BOTH an Import (`step` reads from `cad`) and an Export (`cad` exports to
    /// `step`) `IoKey`, and BOTH resolve, locally, only inside CAD's own registry (`s.stdio` never
    /// registers anything naming `s.cad` — confirmed by inspection, not assumed). So asking the
    /// shared `IoRouter` to compose this key with `calling_plugin_id = "stdio"` can only ever
    /// succeed by crossing into CAD's separately-loaded wasm instance for real — the exact
    /// `HostState::io_compose` -> `IoRouter::compose` -> owner-lookup -> `WasmPluginRuntime::
    /// artifact_compose` path a genuine guest-triggered `host.io-compose` call would take.
    #[test]
    fn io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins() {
        let stdio_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        let cad_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/cad/semio_s_plugin_cad_component.core.wasm");
        if !stdio_path.exists() || !cad_path.exists() {
            // 🧊️ Same convention as `wasm_plugin_runtime_loads_real_plugin_component_if_present`
            // above: this crate's own suite must stay green on a fresh clone / no-wasm-toolchain CI
            // run — but wherever both real build artifacts DO exist, every assertion below is real.
            return;
        }
        let stdio_runtime = Arc::new(WasmPluginRuntime::load(stdio_path).expect("load real stdio.wasm"));
        let cad_runtime = Arc::new(WasmPluginRuntime::load(cad_path).expect("load real cad.wasm"));

        let router = Arc::new(IoRouter::new());
        stdio_runtime.register_host_io_router(Arc::clone(&router)).expect("register stdio io router");
        cad_runtime.register_host_io_router(Arc::clone(&router)).expect("register cad io router");
        router.register_plugin(&stdio_runtime.manifest.plugin_id, Arc::clone(&stdio_runtime)).expect("register stdio plugin");
        router.register_plugin(&cad_runtime.manifest.plugin_id, Arc::clone(&cad_runtime)).expect("register cad plugin");

        let (plugins, keys) = router.stats().expect("router stats");
        assert_eq!(plugins, 2, "both real plugins must be registered with the shared router");
        assert!(keys > 0, "both plugins' real composer rosters must have produced at least one route");

        let fixture_text = std::fs::read_to_string("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio").expect("real cad demo fixture must be on disk");

        // 🗝️ Owned ONLY by cad's registry (`s.cad` is never a dialect `stdio` registers) -- routing
        // this with `calling_plugin_id = stdio` is the real cross-instance test: stdio's own guest
        // has nothing locally for this key, so a genuine `host.io-compose` call from it could only
        // ever be answered by hopping the host router into CAD's separately loaded instance.
        let key = semio_framework::IoKey {
            artifact_kind: "s.cad".to_string(),
            standard: "1".to_string(),
            subset: "*".to_string(),
            direction: semio_framework::IoDirection::Export,
            format_kind: "s.stdio.step".to_string(),
            format_standard: "ap214".to_string(),
            format_subset: "*".to_string(),
        };
        let key_bytes = serde_json::to_vec(&key).expect("encode io key");
        let make_sources = |text: String| {
            serde_json::to_vec(&vec![semio_framework::WireComposeSource {
                dialect: semio_framework::ArtifactDialect { artifact_kind: "s.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() },
                payload: semio_framework::IoPayload::Text(text),
            }])
            .expect("encode compose sources")
        };

        // 🌉️ The real routed call: `IoRouter::compose` is the exact function `HostState::io_compose`
        // (this file, `impl semio::framework::host::Host for HostState`) invokes for a genuine guest
        // `host.io-compose` import call -- calling it here with a real "stdio" caller id against a
        // cad-owned key exercises the identical host-side dispatch, landing on cad's real wasm.
        let result_bytes = router.compose(&stdio_runtime.manifest.plugin_id, &key_bytes, &make_sources(fixture_text)).expect("routed cross-plugin compose must succeed, not error");
        let composed: semio_framework::WireComposedArtifact = serde_json::from_slice(&result_bytes).expect("decode composed artifact wire bytes");
        assert_eq!(composed.dialect.artifact_kind, "s.stdio.step");
        let first_text = match composed.payload {
            semio_framework::IoPayload::Text(text) => text,
            semio_framework::IoPayload::Binary(bytes) => panic!("expected text payload, got {} binary bytes", bytes.len()),
        };
        assert!(!first_text.is_empty(), "composed result must carry real bytes, not an empty payload");
        assert!(first_text.contains("cad.document"), "composed text must be a real re-print of the source document, not garbage: {first_text:?}");

        // 🔁️ Idempotency: feed the composed output BACK in as a fresh `s.cad` source through the SAME
        // routed key. print(parse(x)) is a fixpoint by construction for every other DSL type in this
        // codebase (`assert_dsl_round_trip`'s law) -- getting byte-identical text back here is real,
        // non-trivial evidence the routed call ran cad's genuine parse+print logic on both hops,
        // not e.g. a stub that silently swallows an error into an empty "ok".
        let second_result_bytes = router.compose(&stdio_runtime.manifest.plugin_id, &key_bytes, &make_sources(first_text.clone())).expect("second routed cross-plugin compose must succeed");
        let second_composed: semio_framework::WireComposedArtifact = serde_json::from_slice(&second_result_bytes).expect("decode second composed artifact wire bytes");
        let second_text = match second_composed.payload {
            semio_framework::IoPayload::Text(text) => text,
            semio_framework::IoPayload::Binary(bytes) => panic!("expected text payload, got {} binary bytes", bytes.len()),
        };
        assert_eq!(second_text, first_text, "print(parse(print(parse(x)))) must be a fixpoint -- proves real parse/print logic ran on both routed hops, not a stub");
    }
    //#endregion 🔖️IoRouterE2e

    //#region 🔖️W2aPluginDependencyE2e
    /// 🎯️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): the real
    /// wasmtime proof for `PluginGraph`/`ArtifactMutationRouter`/`ArtifactInferenceRouter`/
    /// `InstanceDirectory` — TWO real plugin components (cad, stdio — the same pair
    /// `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins` above already
    /// proves cross-instance IO with) plus ONE real extension component (flow's `math` extension),
    /// all loaded via this crate's own `WasmPluginRuntime`/`ExtensionRuntime`, registered into REAL
    /// router/graph/directory instances exactly as `🏃️run/🦀️component.rs`'s `WasmtimeNodeHost::
    /// load_runtime_recursive`/`open` do it.
    ///
    /// ⚠️ Neither cad nor stdio (nor any shipped plugin) declares a real `.depends_on()`/
    /// `.contributes()` relationship yet — that is explicitly W3 pilot work per this ticket's master
    /// plan (`📓️master-plan.md`: "W3 pilots ... prove the mechanisms end to end"), not W2's. This
    /// test therefore proves the REAL wiring (real manifests flow through real registration/lookup
    /// code against real loaded components) and layers SYNTHETIC dependency manifests on top (via
    /// the same `PluginGraph::register` a real dependent's real manifest would go through) to
    /// exercise the three graph-level typed rejections. `HostTransactionCoordinator`'s full
    /// two-member commit/group-undo path (which inherently needs a guest that actually proposes a
    /// composite mutation) is proven separately in `host_transaction_coordinator_tests` above via
    /// an in-process fake that speaks the identical wire protocol — see that module's own doc.
    #[test]
    fn plugin_dependency_infrastructure_wires_real_loaded_plugins_and_one_real_extension() {
        let cad_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/cad/semio_s_plugin_cad_component.core.wasm");
        let stdio_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/stdio/semio_s_plugin_stdio_component.core.wasm");
        let extension_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/flow-extension-math/semio_s_plugin_flow_extension_math_component.core.wasm");
        if !cad_path.exists() || !stdio_path.exists() || !extension_path.exists() {
            // 🧊️ Same convention as every other real-component test in this file: stays green on a
            // fresh clone / no-wasm-toolchain CI run.
            return;
        }

        let cad = WasmPluginRuntime::load(cad_path).expect("load real cad.wasm");
        let stdio = WasmPluginRuntime::load(stdio_path).expect("load real stdio.wasm");
        let cad_id = cad.manifest.plugin_id.clone();
        let stdio_id = stdio.manifest.plugin_id.clone();
        assert_ne!(cad_id, stdio_id, "two distinct real plugins");

        //#region 🔖️PluginGraphOverRealManifests
        let graph = PluginGraph::new();
        graph.register(cad.manifest.clone()).expect("cad's real manifest registers cleanly (no declared dependencies yet)");
        graph.register(stdio.manifest.clone()).expect("stdio's real manifest registers cleanly");
        let order = graph.load_order().expect("no cycle among two dependency-free real plugins");
        assert!(order.contains(&cad_id) && order.contains(&stdio_id), "load order must name both real plugins: {order:?}");
        assert!(graph.dependents(&cad_id).unwrap().is_empty(), "neither real plugin declares a dependency yet");

        // Typed rejections (contract §4 rule 5), layered as SYNTHETIC manifests on top of the real
        // graph — see this test's own doc for why a real dependency edge doesn't exist yet.
        let mut missing_dependency = cad.manifest.clone();
        missing_dependency.plugin_id = "w2a-e2e-missing-dep-probe".into();
        missing_dependency.dependencies = vec![semio_framework::PluginDependency::new("w2a-e2e-nonexistent", semio_framework::VersionReq::Any)];
        let error = graph.register(missing_dependency).unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::MissingDependency { .. })), "expected MissingDependency, got {error:?}");

        let mut version_mismatch = cad.manifest.clone();
        version_mismatch.plugin_id = "w2a-e2e-version-probe".into();
        version_mismatch.dependencies = vec![semio_framework::PluginDependency::new(&cad_id, semio_framework::VersionReq::parse("^999.0.0").unwrap())];
        let error = graph.register(version_mismatch).unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::VersionMismatch { .. })), "expected VersionMismatch against cad's real version `{}`, got {error:?}", cad.manifest.version);

        // A cycle: re-register the REAL cad manifest (as if hot-reloading it) with a synthetic
        // dependency on stdio, after separately registering a synthetic plugin that depends on cad —
        // closing synthetic-dependent -> cad -> stdio -> ??? is simpler as a direct two-node cycle:
        // stdio (synthetic dependency on cad) already exists as a REAL, dependency-free manifest, so
        // instead prove the cycle path the same way `plugin_graph_tests` does: two synthetic nodes.
        let cycle_graph = PluginGraph::new();
        cycle_graph.register(cad.manifest.clone()).unwrap();
        let mut synthetic_dependent = stdio.manifest.clone();
        synthetic_dependent.dependencies = vec![semio_framework::PluginDependency::new(&cad_id, semio_framework::VersionReq::Any)];
        cycle_graph.register(synthetic_dependent).unwrap();
        let mut cad_now_depends_on_stdio = cad.manifest.clone();
        cad_now_depends_on_stdio.dependencies = vec![semio_framework::PluginDependency::new(&stdio_id, semio_framework::VersionReq::Any)];
        let error = cycle_graph.register(cad_now_depends_on_stdio).unwrap_err();
        assert!(matches!(error, PluginGraphError::Graph(semio_framework::DependencyGraphError::Cycle { .. })), "expected Cycle, got {error:?}");

        // Unload guard: refuse while a (synthetic) dependent is registered; permitted once it's gone.
        assert!(matches!(graph.guard_unload(&cad_id), Ok(())), "no dependent registered against the MAIN graph yet");
        let mut real_dependent = stdio.manifest.clone();
        real_dependent.plugin_id = "w2a-e2e-real-dependent-probe".into();
        real_dependent.dependencies = vec![semio_framework::PluginDependency::new(&cad_id, semio_framework::VersionReq::Any)];
        graph.register(real_dependent).unwrap();
        assert!(matches!(graph.guard_unload(&cad_id), Err(PluginGraphError::UnloadBlocked { .. })), "unload must now be refused");
        graph.unregister("w2a-e2e-real-dependent-probe").unwrap();
        graph.guard_unload(&cad_id).expect("no dependents left, unload now permitted");
        //#endregion 🔖️PluginGraphOverRealManifests

        //#region 🔖️MutationAndInferenceRoutersOverRealPlugins
        let mutation_router = ArtifactMutationRouter::new();
        let cad_roster_bytes = cad.list_artifact_mutations().expect("real cad.wasm answers contributor.list-artifact-mutations");
        mutation_router.register_plugin(&cad_id, &cad.manifest.dependencies, &cad_roster_bytes).expect("cad's real (today likely empty) roster registers without conflict");
        let stdio_roster_bytes = stdio.list_artifact_mutations().expect("real stdio.wasm answers contributor.list-artifact-mutations");
        mutation_router.register_plugin(&stdio_id, &stdio.manifest.dependencies, &stdio_roster_bytes).expect("stdio's real roster registers without conflict");
        mutation_router.roster().expect("merged roster is readable (contributed rows, if any real plugin ever adds one, are visible here)");

        let inference_router = ArtifactInferenceRouter::new();
        let cad_dependencies = cad.manifest.dependencies.clone();
        let cad_arc = Arc::new(cad);
        inference_router.register_plugin(&cad_id, &cad_dependencies, Arc::clone(&cad_arc)).expect("cad's real inference metadata registers");
        let stdio_arc = Arc::new(stdio);
        inference_router.register_plugin(&stdio_id, &stdio_arc.manifest.dependencies, Arc::clone(&stdio_arc)).expect("stdio's real inference metadata registers");
        inference_router.metadata().expect("merged inference metadata is readable");
        //#endregion 🔖️MutationAndInferenceRoutersOverRealPlugins

        //#region 🔖️InstanceDirectoryOverRealInstances
        let directory = InstanceDirectory::new();
        directory.bind("artifacts/w2a-e2e-cad-instance", &cad_id, 1, "s.cad.document").expect("bind a synthetic artifact ref to a real plugin's identity");
        directory.bind("artifacts/w2a-e2e-stdio-instance", &stdio_id, 1, "s.stdio.document").expect("bind a synthetic artifact ref to the other real plugin's identity");
        assert_eq!(directory.resolve("artifacts/w2a-e2e-cad-instance").unwrap().plugin_id, cad_id);
        assert_eq!(directory.resolve("artifacts/w2a-e2e-stdio-instance").unwrap().plugin_id, stdio_id);
        directory.unbind_instance(&cad_id, 1);
        assert!(directory.resolve("artifacts/w2a-e2e-cad-instance").is_none());
        //#endregion 🔖️InstanceDirectoryOverRealInstances

        //#region 🔖️ExtensionComponentE2e
        let extension_runtime = ExtensionRuntime::new().expect("extension runtime engine/linker build");
        let extension_id = extension_runtime.load(extension_path).expect("load real flow-extension-math.wasm as an extension-world component");
        let extension_manifest = extension_runtime.manifest(&extension_id).expect("loaded extension's manifest is readable");
        assert_eq!(extension_manifest.extension_id, extension_id);
        assert!(!extension_manifest.extends.is_empty(), "a real extension names the plugin it extends");
        //#endregion 🔖️ExtensionComponentE2e
    }
    //#endregion 🔖️W2aPluginDependencyE2e

    //#region 🔖️MergePolicyE2e
    /// ⚖️ Ticket 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS (W2-B): the real
    /// wasmtime proof that a merge-policy-rejected dispatch surfaces through `AppFrame::Error`'s
    /// trailing `report` (contract-freeze.md §C8/C9), and that flipping the session's policy to
    /// `LaissezFaire` (via `WasmPluginRuntime::set_merge_policy`) changes the outcome from rejected to
    /// applied while the SAME `mutation.target-missing` message still surfaces — this time on
    /// `AppFrame::Invocation.messages`. Uses the real `block` plugin's block2d editor
    /// (`removeHandle{id}` -> the `delete-handle` mutation kind, whose `🔺️diff` returns
    /// `MutationOutcome::error("mutation.target-missing", ..)` for an absent handle id — see
    /// `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
    /// ❌️delete-handle/🔺️diff/🦀️component.rs`) — no new plugin crate, no in-process fake: a real
    /// dispatch through a real sandboxed wasm component, the exact `create_app`/`exchange` calling
    /// convention `WasmtimeNodeHost`/`ProgramBridge` already use.
    #[test]
    fn merge_policy_gates_a_real_dispatch_and_laissez_faire_still_surfaces_its_message() {
        let block_path = Path::new("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/block/semio_s_plugin_block_component.core.wasm");
        if !block_path.exists() {
            // 🧊️ Same convention as every other real-component test in this file: stays green on a
            // fresh clone / no-wasm-toolchain CI run.
            return;
        }
        let runtime = WasmPluginRuntime::load(block_path).expect("load real block.wasm");

        // 🪪️ The block2d editor's real manifest app id — never hardcoded (`AppDefinition.id` is
        // derived via `semio_framework::manifest::surface_app_id`, not a stable hand-picked constant).
        let editor_app_id = runtime
            .manifest
            .apps
            .iter()
            .find(|app| app.dialect.artifact_kind == "s.block.block2d" && app.role == semio_framework::manifest::AppRole::Editor)
            .expect("block plugin declares a block2d editor surface")
            .id
            .clone();

        let instance_id = runtime.create_app(&editor_app_id).expect("create a real block2d editor instance");

        let mut seq: u64 = 0;
        let mut next_seq = || {
            seq += 1;
            seq
        };

        let hello_frames = runtime
            .exchange(instance_id, vec![protocol::encode_app_command(&protocol::AppCommand::Hello { channel_version: protocol::CHANNEL_VERSION, app_id: editor_app_id.clone(), actor: "w2b-e2e".into(), config: Vec::new() })])
            .expect("Hello exchange succeeds");
        assert!(hello_frames.iter().any(|bytes| matches!(protocol::decode_app_frame(bytes), Ok(protocol::AppFrame::Welcome { .. }))), "real block2d instance must welcome a valid Hello");

        let view_state = ViewModel { active_mode_id: Some("edit".into()), ..Default::default() };
        let view_state_bytes = store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&view_state).expect("view state encodes"));

        let action_frame = |seq: u64, action_id: &str, arguments: Vec<(&str, serde_json::Value)>| -> Vec<u8> {
            let invocation = semio_framework::manifest::ActionInvocation {
                address: semio_framework::manifest::ActionAddress {
                    plugin_id: "block".into(),
                    app_id: editor_app_id.clone(),
                    mode_id: "edit".into(),
                    window_kind_id: "block2d-board".into(),
                    window_instance_id: "w2b-e2e-window".into(),
                    action_id: action_id.into(),
                },
                arguments: arguments.into_iter().map(|(key, value)| (key.to_string(), value)).collect(),
            };
            let command_bytes = store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&invocation).expect("action invocation encodes"));
            protocol::encode_app_command(&protocol::AppCommand::Command { seq, command: command_bytes, view_state: view_state_bytes.clone() })
        };

        // 🌱️ Mint one real handle so this artifact's document is genuinely non-trivial (not needed for
        // the missing-id assertion itself, but proves the wasm document store is real, not a stub that
        // accepts everything).
        let add_kind_seq = next_seq();
        let add_kind_frames = runtime.exchange(instance_id, vec![action_frame(add_kind_seq, "addHandleKind", vec![])]).expect("addHandleKind exchange succeeds");
        assert!(add_kind_frames.iter().any(|bytes| matches!(protocol::decode_app_frame(bytes), Ok(protocol::AppFrame::Invocation { in_reply_to, .. }) if in_reply_to == add_kind_seq)), "addHandleKind must produce a real Invocation frame, not an Error");

        let add_handle_seq = next_seq();
        let add_handle_frames = runtime.exchange(instance_id, vec![action_frame(add_handle_seq, "addHandle", vec![])]).expect("addHandle exchange succeeds");
        assert!(add_handle_frames.iter().any(|bytes| matches!(protocol::decode_app_frame(bytes), Ok(protocol::AppFrame::Invocation { in_reply_to, .. }) if in_reply_to == add_handle_seq)), "addHandle must produce a real Invocation frame, not an Error");

        let read_seq_before = next_seq();
        let before_frames = runtime.exchange(instance_id, vec![protocol::encode_app_command(&protocol::AppCommand::ReadDocument { seq: read_seq_before })]).expect("ReadDocument exchange succeeds");
        let document_before = before_frames
            .iter()
            .find_map(|bytes| match protocol::decode_app_frame(bytes) {
                Ok(protocol::AppFrame::Document { in_reply_to, pack, spr, .. }) if in_reply_to == read_seq_before => Some((pack, spr)),
                _ => None,
            })
            .expect("ReadDocument must reply with a real Document frame");
        assert!(!document_before.0.is_empty(), "a document with one real handle must not be an empty pack");

        //#region 🔖️Normal
        // ⚖️ Default policy is `Normal` (never set explicitly here) — `Normal.rejects(Error)` is true
        // (contract-freeze.md's policy table), so `removeHandle` on a missing id must be REJECTED.
        let reject_seq = next_seq();
        let reject_frames = runtime.exchange(instance_id, vec![action_frame(reject_seq, "removeHandle", vec![("id", serde_json::json!("nonexistent-handle-id"))])]).expect("removeHandle exchange succeeds (rejection is a frame, not a wasmtime error)");
        let (fault, report) = reject_frames
            .iter()
            .find_map(|bytes| match protocol::decode_app_frame(bytes) {
                Ok(protocol::AppFrame::Error { in_reply_to: Some(reply), fault, report }) if reply == reject_seq => Some((fault, report)),
                _ => None,
            })
            .expect("Normal policy must reject a delete-missing-id dispatch with a real AppFrame::Error");
        assert_eq!(dsl::decode_fault_bytes(&fault).code.0, "mutation.rejected", "the rejection fault must be the real mutation.rejected code, not a generic one");
        let dispatch_report = decode_dispatch_report(&report).expect("Error.report must decode to a real packed DispatchReport");
        assert_eq!(dispatch_report.policy, protocol::MergePolicy::Normal);
        assert!(dispatch_report.messages.iter().any(|message| message.code.0 == "mutation.target-missing"), "the rejected dispatch's real messages must name mutation.target-missing, got {:?}", dispatch_report.messages);

        let read_seq_after_reject = next_seq();
        let after_reject_frames = runtime.exchange(instance_id, vec![protocol::encode_app_command(&protocol::AppCommand::ReadDocument { seq: read_seq_after_reject })]).expect("ReadDocument exchange succeeds");
        let document_after_reject = after_reject_frames
            .iter()
            .find_map(|bytes| match protocol::decode_app_frame(bytes) {
                Ok(protocol::AppFrame::Document { in_reply_to, pack, spr, .. }) if in_reply_to == read_seq_after_reject => Some((pack, spr)),
                _ => None,
            })
            .expect("ReadDocument must reply with a real Document frame");
        assert_eq!(document_after_reject, document_before, "a rejected dispatch must leave the document byte-for-byte unchanged");
        //#endregion 🔖️Normal

        //#region 🔖️LaissezFaire
        // ⚖️ `LaissezFaire.rejects` is true only for `Fatal` — `mutation.target-missing` is `Error`, so
        // switching this session's policy must let the SAME dispatch apply (an `Invocation` frame, not
        // `Error`), while its real message still surfaces — now on `Invocation.messages`.
        runtime.set_merge_policy(instance_id, protocol::MergePolicy::LaissezFaire).expect("set_merge_policy(LaissezFaire) succeeds against a real instance");

        let apply_seq = next_seq();
        let apply_frames = runtime.exchange(instance_id, vec![action_frame(apply_seq, "removeHandle", vec![("id", serde_json::json!("nonexistent-handle-id"))])]).expect("removeHandle exchange succeeds under LaissezFaire");
        assert!(
            !apply_frames.iter().any(|bytes| matches!(protocol::decode_app_frame(bytes), Ok(protocol::AppFrame::Error { in_reply_to: Some(reply), .. }) if reply == apply_seq)),
            "LaissezFaire must not reject an Error-level (non-Fatal) dispatch"
        );
        let messages = apply_frames
            .iter()
            .find_map(|bytes| match protocol::decode_app_frame(bytes) {
                Ok(protocol::AppFrame::Invocation { in_reply_to, messages, .. }) if in_reply_to == apply_seq => Some(messages),
                _ => None,
            })
            .expect("LaissezFaire must apply the dispatch as a real Invocation frame");
        let applied_report = decode_dispatch_report(&messages).expect("Invocation.messages must decode to a real packed DispatchReport");
        assert_eq!(applied_report.policy, protocol::MergePolicy::LaissezFaire);
        assert!(applied_report.messages.iter().any(|message| message.code.0 == "mutation.target-missing"), "the applied dispatch's real messages must STILL name mutation.target-missing under LaissezFaire, got {:?}", applied_report.messages);
        //#endregion 🔖️LaissezFaire
    }
    //#endregion 🔖️MergePolicyE2e
}
