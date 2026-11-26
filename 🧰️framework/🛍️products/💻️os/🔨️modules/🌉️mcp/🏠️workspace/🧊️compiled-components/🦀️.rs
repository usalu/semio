//! 🧊️ Process-wide compiled runtime, component caches, compile worker and compile flights.

#[cfg(not(target_arch = "wasm32"))]
use semio_plugin_host::GuestRuntime as _;
#[cfg(not(target_arch = "wasm32"))]
use semio_plugin_host::GuestRuntimes;
#[cfg(not(target_arch = "wasm32"))]
use semio_plugin_host::OwnedRuntime;
#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::OnceLock;
#[cfg(not(target_arch = "wasm32"))]
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
#[cfg(not(target_arch = "wasm32"))]
use semio_os_mcp_port::channel_port::{ActivationPhase, ActivationScope, Fault, map_fault};
#[cfg(not(target_arch = "wasm32"))]
#[path = "🌱️restore/🦀️.rs"]
mod actor_restore;

/// 🧠️ The process shares one firstparty interpreter for dynamically supplied component bytes.
#[cfg(not(target_arch = "wasm32"))]
pub fn shared_plugin_runtime() -> Result<Arc<GuestRuntimes>, GatewayError> {
    Ok(Arc::clone(&plugin_runtime_origin().get_or_init(||{static NEXT:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);let identity=NEXT.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|value|value.checked_add(1)).expect("process cache identity remains finite");PluginRuntimeOrigin{identity,runtime:Arc::new(GuestRuntimes::Owned(OwnedRuntime::new(semio_framework_job::default_now_us)))}}).runtime))
}

#[cfg(not(target_arch="wasm32"))]
fn plugin_runtime_origin()->&'static OnceLock<PluginRuntimeOrigin>{static RUNTIME:OnceLock<PluginRuntimeOrigin>=OnceLock::new();&RUNTIME}

#[cfg(not(target_arch="wasm32"))]
struct PluginRuntimeOrigin{identity:u64,runtime:Arc<GuestRuntimes>}

/// 🏠️ The actual immutable process cache lends its selected compiled parent without issuing aliases.
#[cfg(not(target_arch="wasm32"))]
pub fn native_instance_origin(package_hash:[u8;32])->Result<semio_plugin_host::GuestRelayNativeOrigin,semio_framework_job::OriginalTurnFailure>{
 let runtime=plugin_runtime_origin().get().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native origin requires its initialized immutable process cache"))?;
 Ok(semio_plugin_host::GuestRelayNativeOrigin{cache_identity:runtime.identity,package_hash,acquire:acquire_native_instance_origin,resolve:resolve_native_instance_origin,release:release_native_instance_origin})
}
#[cfg(not(target_arch="wasm32"))]
fn resolve_native_instance_origin(cache_identity:u64,hash:[u8;32],run:&mut dyn FnMut(&GuestRuntimes,&semio_plugin_host::CompiledHandle)->Result<(),semio_framework_job::OriginalTurnFailure>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let Some(runtime)=plugin_runtime_origin().get()else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native instance has no admitted process runtime origin").into())};
    if cache_identity==0||cache_identity!=runtime.identity{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native origin belongs to another immutable cache generation").into())}
    let Ok(cache)=compiled_components().try_lock()else{return Ok(false)};
    let compiled=cache.get(&hash).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native instance has no admitted immutable compiled origin"))?;
    if compiled.compiled.package_hash!=hash||compiled.homes==0{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native compiled cache identity differs from its selected origin").into())}
    run(&runtime.runtime,&compiled.compiled)?;Ok(true)
}

#[cfg(not(target_arch="wasm32"))]
struct CompiledParent{compiled:semio_plugin_host::CompiledHandle,homes:u64}
#[cfg(not(target_arch="wasm32"))]
fn acquire_native_instance_origin(identity:u64,hash:[u8;32],held:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{change_native_instance_custody(identity,hash,held,true,cx)}
#[cfg(not(target_arch="wasm32"))]
fn release_native_instance_origin(identity:u64,hash:[u8;32],held:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{change_native_instance_custody(identity,hash,held,false,cx)}
#[cfg(not(target_arch="wasm32"))]
fn change_native_instance_custody(identity:u64,hash:[u8;32],held:&mut bool,acquire:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
 let refuse=||ValueError::literal(ValueRefusalKind::InvariantViolated,"native parent custody identity or balance differs");
 let runtime=plugin_runtime_origin().get().ok_or_else(refuse)?;if identity==0||identity!=runtime.identity||*held==acquire{return Err(refuse().into())}
 let Ok(mut cache)=compiled_components().try_lock()else{return Ok(())};let parent=cache.get_mut(&hash).ok_or_else(refuse)?;
 if parent.compiled.package_hash!=hash{return Err(refuse().into())}let next=if acquire{parent.homes.checked_add(1)}else{parent.homes.checked_sub(1)}.ok_or_else(refuse)?;
 let grant=cx.retained_grant();let bytes=std::mem::size_of::<u64>()+std::mem::size_of::<bool>();
 if(if acquire{cx.should_yield()}else{cx.retirement_should_yield()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}
 cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);parent.homes=next;*held=acquire;Ok(())
}

/// 🧩️ ONE compiled component per staged build, for the life of the process, keyed by the [Design notes](README.md#source-notes-72).
#[cfg(not(target_arch = "wasm32"))]
fn compiled_components() -> &'static Mutex<HashMap<[u8; 32], CompiledParent>> {
    static COMPILED: OnceLock<Mutex<HashMap<[u8; 32], CompiledParent>>> = OnceLock::new();
    COMPILED.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct ComponentPreparationGate {
    active: Mutex<bool>,
    changed: std::sync::Condvar,
}

#[cfg(not(target_arch = "wasm32"))]
struct ComponentPreparationClaim<'a>(&'a ComponentPreparationGate);

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ComponentPreparationClaim<'_> {
    fn drop(&mut self) {
        *self.0.active.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = false;
        self.0.changed.notify_all();
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ComponentPreparationGate {
    fn for_component(hash: [u8; 32]) -> Arc<Self> {
        static GATES: OnceLock<Mutex<HashMap<[u8; 32], std::sync::Weak<ComponentPreparationGate>>>> = OnceLock::new();
        let mut gates = GATES.get_or_init(Default::default).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        gates.retain(|_, gate| gate.strong_count() != 0);
        if let Some(gate) = gates.get(&hash).and_then(std::sync::Weak::upgrade) {
            return gate;
        }
        let gate = Arc::new(Self::default());
        gates.insert(hash, Arc::downgrade(&gate));
        gate
    }

    fn acquire(&self, scope: &ActivationScope) -> Result<ComponentPreparationClaim<'_>, GatewayError> {
        let mut active = self.active.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            scope.checkpoint().map_err(activation_fault)?;
            if !*active {
                *active = true;
                return Ok(ComponentPreparationClaim(self));
            }
            active = self.changed.wait_timeout(active, std::time::Duration::from_millis(50)).unwrap_or_else(std::sync::PoisonError::into_inner).0;
        }
    }
}

/// 🌱️ Shares the owned descriptor stage's finite fuel and elapsed-time envelope for cold authored bundle construction.
#[cfg(not(target_arch = "wasm32"))]
fn prepare_compiled_actor(compiled: &mut semio_plugin_host::CompiledHandle, plugin_id: &str, scope: &ActivationScope) -> Result<(), GatewayError> {
    const FUEL: u64 = 32_000_000_000;
    const PROGRESS_INTERVAL: u64 = 1 << 24;
    const WALL: std::time::Duration = std::time::Duration::from_secs(1_800);
    let began = std::time::Instant::now();
    let mut next_report = 0;
    let mut spent = 0;
    let prepared = compiled.prepare_owned_actor(FUEL, |fuel| {
        spent = fuel;
        if began.elapsed() >= WALL || scope.cancel().is_cancelled_now() {
            return false;
        }
        if fuel >= next_report {
            next_report = fuel + PROGRESS_INTERVAL;
            return scope.advance(ActivationPhase::CompilingComponent, fuel as f64 / FUEL as f64).is_ok();
        }
        true
    });
    scope.checkpoint().map_err(activation_fault)?;
    if began.elapsed() >= WALL {
        return Err(GatewayError::new(GatewayErrorCode::BudgetExceeded, format!("preparing `{plugin_id}` reached its wall budget")).retryable());
    }
    if prepared.is_err() && spent == FUEL {
        return Err(GatewayError::new(GatewayErrorCode::BudgetExceeded, format!("preparing `{plugin_id}` reached its instruction budget")).retryable());
    }
    if prepared.map_err(|error| GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("preparing `{plugin_id}`: {error}")))? {
        scope.advance(ActivationPhase::CompilingComponent, 1.0).map_err(activation_fault)?;
    }
    Ok(())
}

/// 🪪️ What a staged component file IS without reading it: its path, length and modification time.
/// A rebuild rewrites the file, so it changes the identity and is read and hashed again.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, PartialEq, Eq, Hash)]
struct ComponentFileIdentity {
    pub path: PathBuf,
    pub length: u64,
    modified_ns: u128,
}

/// 🔏️ The content hash of every staged component file already read, by file identity — held in this [Design notes](README.md#source-notes-99).
#[cfg(not(target_arch = "wasm32"))]
fn component_file_hashes() -> &'static Mutex<HashMap<ComponentFileIdentity, [u8; 32]>> {
    static HASHES: OnceLock<Mutex<HashMap<ComponentFileIdentity, [u8; 32]>>> = OnceLock::new();
    HASHES.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(not(target_arch = "wasm32"))]
impl ComponentFileIdentity {
    fn read(path: &Path) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            length: metadata.len(),
            modified_ns: metadata.modified()?.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos(),
        })
    }

    fn known_hash(&self) -> Option<[u8; 32]> {
        component_file_hashes().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(self).copied()
    }

    fn remember(self, hash: [u8; 32]) {
        if Self::read(&self.path).ok().as_ref() == Some(&self) {
            let mut hashes = component_file_hashes().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            hashes.retain(|identity, _| identity.path != self.path);
            hashes.insert(self, hash);
        }
    }
}

/// 🧩️ Where a component's bytes come from: a staged file this process may already know, or bytes a
/// caller already holds (a hub-authorized execution target).
#[cfg(not(target_arch = "wasm32"))]
pub enum ComponentBytes<'a> {
    File(&'a Path),
    Held(&'a [u8]),
}

/// 📏️ How much of a component one hashing step covers before it reports progress and checks cancel.
#[cfg(not(target_arch = "wasm32"))]
const COMPONENT_HASH_CHUNK_BYTES: usize = 4 << 20;

#[cfg(not(target_arch = "wasm32"))]
pub fn activation_fault(fault: Fault) -> GatewayError {
    map_fault(&fault)
}

/// 🔏️ `bytes`' own 32-byte content digest, one chunk per progress report and cancel check. [Design notes](README.md#source-notes-161).
#[cfg(not(target_arch = "wasm32"))]
fn scoped_component_hash(bytes: &[u8], scope: &ActivationScope) -> Result<[u8; 32], GatewayError> {
    let mut hasher = framework_hash::Hasher::new();
    let mut hashed = 0usize;
    for chunk in bytes.chunks(COMPONENT_HASH_CHUNK_BYTES) {
        hasher.update(chunk);
        hashed += chunk.len();
        scope.advance(ActivationPhase::HashingComponent, hashed as f64 / bytes.len().max(1) as f64).map_err(activation_fault)?;
    }
    Ok(*hasher.finalize().as_bytes())
}

#[cfg(not(target_arch = "wasm32"))]
fn read_component(path: &Path, scope: &ActivationScope) -> Result<Vec<u8>, GatewayError> {
    scope.enter(ActivationPhase::ReadingComponent).map_err(activation_fault)?;
    std::fs::read(path).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("reading {}: {error}", path.display())))
}

/// 🧩️ The process's compiled handle for one component, reporting each phase it actually runs to [Design notes](README.md#source-notes-186).
#[cfg(not(target_arch = "wasm32"))]
pub fn scoped_compiled_component(runtime: &GuestRuntimes, plugin_id: &str, component: ComponentBytes<'_>, scope: &ActivationScope) -> Result<semio_plugin_host::CompiledHandle, GatewayError> {
    scope.checkpoint().map_err(activation_fault)?;
    let shared = plugin_runtime_origin().get().is_some_and(|origin| std::ptr::eq(runtime, origin.runtime.as_ref()));
    let (identity, bytes) = match component {
        ComponentBytes::File(path) => {
            let identity = shared.then(|| ComponentFileIdentity::read(path).ok()).flatten();
            if let Some(hash) = identity.as_ref().and_then(ComponentFileIdentity::known_hash) {
                if let Some(compiled) = cached_component(hash, scope)? {
                    return Ok(compiled);
                }
            }
            (identity, std::borrow::Cow::Owned(read_component(path, scope)?))
        }
        ComponentBytes::Held(bytes) => (None, std::borrow::Cow::Borrowed(bytes)),
    };
    let package_hash = scoped_component_hash(&bytes, scope)?;
    let gate = shared.then(|| ComponentPreparationGate::for_component(package_hash));
    let _claim = gate.as_ref().map(|gate| gate.acquire(scope)).transpose()?;
    if shared {
        if let Some(compiled) = cached_component(package_hash, scope)? {
            if let Some(identity) = identity {
                identity.remember(package_hash);
            }
            return Ok(compiled);
        }
    }
    scope.enter(ActivationPhase::LoadingCompiledCode).map_err(activation_fault)?;
    let package = semio_plugin_host::PackageRef { package: semio_plugin_host::PackageId(plugin_id.to_string()), hash: semio_plugin_host::PackageHash(package_hash) };
    let mut compiled = match semio_framework_async::block_on(runtime.load_compiled(&package)) {
        Some(compiled) => compiled,
        None => {
            scope.enter(ActivationPhase::CompilingComponent).map_err(activation_fault)?;
            match semio_framework_async::block_on(runtime.isolated_compile_target(&package)) {
                Some(target) => {
                    CompileFlight::join(target, &bytes, scope)?;
                    semio_framework_async::block_on(runtime.load_compiled(&package)).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("compiling `{plugin_id}`: the isolated compile finished but its compiled code does not load")))?
                }
                None => semio_framework_async::block_on(runtime.compile(&package, &bytes)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("compiling `{plugin_id}`: {error}")))?,
            }
        }
    };
    let preparation = match component {
        ComponentBytes::File(path) if shared => semio_os_mcp_catalog::plugin_paths_and_descriptor::retained_actor_preparation(path, plugin_id)?,
        _ => None,
    };
    match preparation {
        Some(preparation) => actor_restore::restore_compiled_actor(&mut compiled, &bytes, &preparation, scope)?,
        None => prepare_compiled_actor(&mut compiled, plugin_id, scope)?,
    }
    if !shared {
        return Ok(compiled);
    }
    let compiled = {
        let mut cache = compiled_components().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        scope.checkpoint().map_err(activation_fault)?;
        cache.entry(package_hash).or_insert(CompiledParent { compiled, homes: 0 }).compiled.clone()
    };
    if let Some(identity) = identity {
        identity.remember(package_hash);
    }
    Ok(compiled)
}

#[cfg(not(target_arch = "wasm32"))]
fn cached_component(hash: [u8; 32], scope: &ActivationScope) -> Result<Option<semio_plugin_host::CompiledHandle>, GatewayError> {
    let cache = compiled_components().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    scope.checkpoint().map_err(activation_fault)?;
    Ok(cache.get(&hash).map(|parent| parent.compiled.clone()))
}

//#region 🧊️CompileFlight

/// 🧊️ The program an isolated compile runs: this process's own executable (`semio-os-mcp
/// compile-component`) unless a host names another one that answers the same subcommand.
#[cfg(not(target_arch = "wasm32"))]
static COMPONENT_COMPILE_WORKER: OnceLock<PathBuf> = OnceLock::new();

/// 🧊️ Names the program isolated compiles run, for a host whose own executable is not
/// `semio-os-mcp` (a test binary). First call wins.
#[cfg(not(target_arch = "wasm32"))]
pub fn set_component_compile_worker(program: PathBuf) {
    let _ = COMPONENT_COMPILE_WORKER.set(program);
}

/// 🧪️ A unit-test binary is not `semio-os-mcp` and has no `compile-component` subcommand, so under
/// `cfg(test)` the worker is this test binary running exactly [`ISOLATED_COMPILE_TEST_ENTRY`], which
/// reads the worker inputs from its environment.
#[cfg(test)]
pub(crate) const ISOLATED_COMPILE_TEST_ENTRY: &str = "workspace::quick::isolated_compile_worker_entry";

#[cfg(any(test, feature = "testkit"))]
pub const ISOLATED_COMPILE_TEST_ENGINE: &str = "SEMIO_ISOLATED_COMPILE_ENGINE";

#[cfg(any(test, feature = "testkit"))]
pub const ISOLATED_COMPILE_TEST_OUT: &str = "SEMIO_ISOLATED_COMPILE_OUT";

#[cfg(any(test, feature = "testkit"))]
pub const ISOLATED_COMPILE_TEST_BYTES: &str = "SEMIO_ISOLATED_COMPILE_BYTES";

/// 🔢️ Isolated compile workers this process has started — a shared compile starts exactly one.
#[cfg(not(target_arch = "wasm32"))]
static COMPONENT_COMPILE_WORKERS_STARTED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg(not(target_arch = "wasm32"))]
pub fn component_compile_workers_started() -> u64 {
    COMPONENT_COMPILE_WORKERS_STARTED.load(std::sync::atomic::Ordering::Acquire)
}

/// 🧩️ Brings `bytes` (a component of `plugin_id`) to compiled code in this process under `scope`,
/// exactly as a plugin session's activation does — for a caller that prepares a component ahead of
/// opening it.
#[cfg(not(target_arch = "wasm32"))]
pub fn prepare_plugin_component(plugin_id: &str, bytes: &[u8], scope: &ActivationScope) -> Result<(), GatewayError> {
    let runtime = shared_plugin_runtime()?;
    scoped_compiled_component(runtime.as_ref(), plugin_id, ComponentBytes::Held(bytes), scope).map(drop)
}

/// 🔎️ Isolated compile workers alive right now — zero once every compile finished or was killed.
#[cfg(not(target_arch = "wasm32"))]
pub fn live_component_compile_workers() -> usize {
    compile_slot().active.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|flight| flight.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).worker.is_some()).count()
}

/// 🧊️ One cold compile of one component, in an isolated worker process, shared by every requester
/// for the same cache entry while it runs. A requester that cancels detaches and is answered at
/// once; when the last one detaches the worker is killed, which returns every core and byte the
/// compile held, and its unfinished output is removed. Nothing completes in the background after
/// its last requester is gone. Law: `🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json`.
#[cfg(not(target_arch = "wasm32"))]
struct CompileFlight {
    cache_path: PathBuf,
    pub state: Mutex<CompileFlightState>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct CompileFlightState {
    pub requesters: usize,
    waiters: Vec<semio_framework_async_sync::oneshot::Sender<Result<(), GatewayError>>>,
    worker: Option<std::process::Child>,
    pub outcome: Option<Result<(), GatewayError>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct CompileSlot {
    active: Mutex<Option<Arc<CompileFlight>>>,
    changed: std::sync::Condvar,
}

#[cfg(not(target_arch = "wasm32"))]
fn compile_slot() -> &'static CompileSlot {
    static SLOT: OnceLock<CompileSlot> = OnceLock::new();
    SLOT.get_or_init(CompileSlot::default)
}

#[cfg(not(target_arch = "wasm32"))]
impl CompileFlight {
    /// 🛫️ Joins the compile for this cache entry or waits for the process's cold worker slot.
    pub fn join(target: semio_plugin_host::IsolatedCompileTarget, bytes: &[u8], scope: &ActivationScope) -> Result<(), GatewayError> {
        let (flight, answer) = {
            let slot = compile_slot();
            let mut active = slot.active.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let flight = loop {
                scope.checkpoint().map_err(activation_fault)?;
                match active.as_ref() {
                    Some(flight) if flight.cache_path == target.cache_path => break Arc::clone(flight),
                    Some(_) => active = slot.changed.wait_timeout(active, std::time::Duration::from_millis(50)).unwrap_or_else(std::sync::PoisonError::into_inner).0,
                    None => {
                        let flight = Self::launch(target, bytes)?;
                        *active = Some(Arc::clone(&flight));
                        break flight;
                    }
                }
            };
            let (sender, answer) = semio_framework_async_sync::oneshot::channel();
            let mut state = flight.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            state.requesters += 1;
            match &state.outcome {
                Some(outcome) => drop(sender.send(outcome.clone())),
                None => state.waiters.push(sender),
            }
            drop(state);
            (flight, answer)
        };
        match semio_framework_async::block_on(semio_framework_async::select2(answer, scope.cancel().cancelled())) {
            semio_framework_async::Either::Left(answer) => {
                flight.leave();
                match answer {
                    Ok(result) => result,
                    Err(_) => Err(GatewayError::new(GatewayErrorCode::Internal, "the isolated component compile ended without answering")),
                }
            }
            semio_framework_async::Either::Right(()) => {
                flight.detach();
                Err(GatewayError::new(GatewayErrorCode::Cancelled, "the caller cancelled while its component was compiling"))
            }
        }
    }

    /// 🚀️ Starts the worker and the thread that feeds it the component and observes its exit.
    fn launch(target: semio_plugin_host::IsolatedCompileTarget, bytes: &[u8]) -> Result<Arc<Self>, GatewayError> {
        let program = match COMPONENT_COMPILE_WORKER.get() {
            Some(program) => program.clone(),
            None => std::env::current_exe().map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("locating the compile worker: {error}")))?,
        };
        let mut command = std::process::Command::new(&program);
        command.env_clear();
        if let Some(system_root) = std::env::var_os("SYSTEMROOT") {
            command.env("SYSTEMROOT", system_root);
        }
        #[cfg(not(test))]
        command.arg("compile-component").arg("--engine").arg(&target.engine).arg("--out").arg(&target.cache_path).arg("--component-bytes").arg(bytes.len().to_string());
        #[cfg(test)]
        command.args([ISOLATED_COMPILE_TEST_ENTRY, "--exact", "--test-threads=1"]).env(ISOLATED_COMPILE_TEST_ENGINE, &target.engine).env(ISOLATED_COMPILE_TEST_OUT, &target.cache_path).env(ISOLATED_COMPILE_TEST_BYTES, bytes.len().to_string());
        let mut worker = command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("starting the compile worker {}: {error}", program.display())))?;
        COMPONENT_COMPILE_WORKERS_STARTED.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let input = worker.stdin.take().expect("piped compile worker stdin");
        let report = worker.stderr.take().expect("piped compile worker stderr");
        let flight = Arc::new(Self { cache_path: target.cache_path, state: Mutex::new(CompileFlightState { requesters: 0, waiters: Vec::new(), worker: Some(worker), outcome: None }) });
        let observed = Arc::clone(&flight);
        let bytes = bytes.to_vec();
        if let Err(error) = std::thread::Builder::new()
            .name("semio-compile-flight".to_string())
            .spawn(move || observed.observe(input, report, bytes))
        {
            flight.kill_worker();
            return Err(GatewayError::new(GatewayErrorCode::Internal, format!("observing the compile worker: {error}")));
        }
        Ok(flight)
    }

    /// 👁️ Feeds the worker its component, waits for it to exit — its report closes when it does —
    /// and answers every requester. A worker already killed by the last detach answers no one.
    pub(crate) fn observe(self: Arc<Self>, mut input: std::process::ChildStdin, mut report: std::process::ChildStderr, bytes: Vec<u8>) {
        use std::io::{Read as _, Write as _};
        let _ = input.write_all(&bytes);
        drop(input);
        drop(bytes);
        let mut message = String::new();
        let _ = report.read_to_string(&mut message);
        let slot = compile_slot();
        let mut active = slot.active.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(mut worker) = state.worker.take() else { return };
        let outcome = match worker.wait() {
            Ok(status) if status.success() => Ok(()),
            Ok(status) if status.code() == Some(semio_plugin_host::ISOLATED_COMPILE_DEFERRED_EXIT) => Err(GatewayError::new(GatewayErrorCode::BudgetExceeded, message.trim()).retryable()),
            Ok(status) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("isolated component compile failed: {status}: {}", message.trim()))),
            Err(error) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("waiting for the isolated component compile: {error}"))),
        };
        if outcome.is_err() {
            let _ = std::fs::remove_file(semio_plugin_host::compiled_cache_scratch_path(&self.cache_path, worker.id()));
        }
        for waiter in state.waiters.drain(..) {
            let _ = waiter.send(outcome.clone());
        }
        state.outcome = Some(outcome);
        *active = None;
        slot.changed.notify_all();
    }

    fn leave(&self) {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.requesters -= 1;
    }

    /// ✂️ Withdraws one requester; the last one kills the worker and removes its unfinished output.
    fn detach(&self) {
        let slot = compile_slot();
        let mut active = slot.active.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.requesters -= 1;
        if state.requesters > 0 || state.outcome.is_some() {
            return;
        }
        self.stop_worker(&mut state);
        state.waiters.clear();
        state.outcome = Some(Err(GatewayError::new(GatewayErrorCode::Cancelled, "every requester cancelled; the compile was killed")));
        *active = None;
        slot.changed.notify_all();
    }

    fn kill_worker(&self) {
        self.stop_worker(&mut self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
    }

    fn stop_worker(&self, state: &mut CompileFlightState) {
        if let Some(mut worker) = state.worker.take() {
            let _ = worker.kill();
            let _ = worker.wait();
            let _ = std::fs::remove_file(semio_plugin_host::compiled_cache_scratch_path(&self.cache_path, worker.id()));
        }
    }
}

//#endregion 🧊️CompileFlight

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/publication.rs"]
mod publication_tests;
