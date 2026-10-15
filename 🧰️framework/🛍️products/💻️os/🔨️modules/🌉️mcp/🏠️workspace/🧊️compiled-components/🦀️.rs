//! 🧊️ Process-wide compiled runtime, component caches, compile worker and compile flights.

#[cfg(not(target_arch = "wasm32"))]
use super::{ACTIVATION_CANCELLED_FAULT_CODE, ActivationPhase, ActivationScope, Arc, Fault, GatewayError, GatewayErrorCode, GuestRuntimes, HashMap, Mutex, OnceLock, Path, PathBuf, SharedEngineConfig, WasmtimeRuntime};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::GuestRuntime as _;

/// 🐎️ ONE compiled runtime for this process, built on first use. `build_shared_engine`'s own
/// contract is one `Engine` per process (a pooling allocator reservation and a 1 ms epoch ticker
/// each), and a `wasmtime::Component` may only be instantiated by the `Engine` that compiled it — so
/// every `PluginArtifactChannel`, and every inference route they open, share this one.
///
/// 🧊️ Held as the `GuestRuntimes` enum because `PluginInstanceHandle::new` takes an
/// `Arc<GuestRuntimes>`; the enum delegates every `GuestRuntime` method to the wasmtime arm with no
/// indirection beyond a match.
///
/// ⛽️ Unmetered: every budget this gateway arms disarms fuel (`u64::MAX`) and bounds a guest call
/// by its epoch deadline, so metering bought nothing here but instrumented every guest block — see
/// [`SharedEngineConfig::fuel_metering`] for the measurement.
///
/// 🧯️ A failed engine build is cached as a `String` rather than retried per call: it fails for
/// reasons that do not change inside a process (no virtual address space, an unusable cache root),
/// and retrying it on every tool call would turn one honest refusal into a stall.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn shared_plugin_runtime() -> Result<Arc<GuestRuntimes>, GatewayError> {
    static RUNTIME: std::sync::OnceLock<Result<Arc<GuestRuntimes>, String>> = std::sync::OnceLock::new();
    RUNTIME
        .get_or_init(|| semio_framework_async::block_on(WasmtimeRuntime::new(SharedEngineConfig { fuel_metering: false, ..SharedEngineConfig::default() },semio_framework_job::default_now_us)).map(|runtime| Arc::new(GuestRuntimes::from(runtime))).map_err(|error| error.to_string()))
        .clone()
        .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("building the shared plugin engine: {error}")))
}

/// 🧩️ ONE compiled component per staged build, for the life of the process, keyed by the
/// component's own content hash so a rebuilt plugin is a different entry rather than a stale one.
///
/// 🪦️ This is not only about not compiling twice. A `wasmtime::Component` owns a mapped code image
/// — 136 MB for an unoptimized `🖍️draw` build — and releasing the LAST `Arc` to one runs
/// `CodeMemory`'s `UnwindRegistration::drop`, i.e. a `__deregister_frame` walk over every frame in
/// that image. Measured 2026-09-20 on a calm machine: the open itself is 34 ms and the drop of the
/// component behind it pins a thread at 100 % for minutes, which is what made the gateway's first
/// plugin tool call answer nothing at all inside the MCP client's 240 s budget — the work was in
/// tearing the component DOWN after a successful call, not in bringing it up. Holding the handle
/// here keeps the refcount above zero, so a channel's own drop releases its `Store` and stops.
#[cfg(not(target_arch = "wasm32"))]
fn compiled_components() -> &'static Mutex<HashMap<[u8; 32], semio_framework_plugin_host::CompiledHandle>> {
    static COMPILED: OnceLock<Mutex<HashMap<[u8; 32], semio_framework_plugin_host::CompiledHandle>>> = OnceLock::new();
    COMPILED.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 🪪️ What a staged component file IS without reading it: its path, length and modification time.
/// A rebuild rewrites the file, so it changes the identity and is read and hashed again.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, PartialEq, Eq, Hash)]
struct ComponentFileIdentity {
    pub(super) path: PathBuf,
    length: u64,
    modified_ns: u128,
}

/// 🔏️ The content hash of every staged component file already read, by file identity — held in this
/// process and persisted beside the compiled-code cache, so a staged build is read and hashed once
/// per build rather than once per process.
///
/// ⏱️ Every channel open used to read the whole component and hash it before it could ask the
/// compiled cache anything: measured 2026-09-24 (ticket 26/09/23 slice G6) the 125 MB `🀄️wfc`
/// build took 8.5–11.5 s to hash in a dev host, once for `artifact_create` and again for the first
/// `inference_run` binding of the same session, while the compiled-code lookup behind it took 0.3 s.
#[cfg(not(target_arch = "wasm32"))]
fn component_file_hashes() -> &'static Mutex<HashMap<ComponentFileIdentity, [u8; 32]>> {
    static HASHES: OnceLock<Mutex<HashMap<ComponentFileIdentity, [u8; 32]>>> = OnceLock::new();
    HASHES.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(not(target_arch = "wasm32"))]
impl ComponentFileIdentity {
    /// 🗂️ Where this identity's content hash is persisted: one 32-byte file under the compiled-code
    /// cache root, named by the digest of the identity itself.
    fn persisted_path(&self) -> PathBuf {
        let key = format!("semio.component-file-identity/v1\0{}\0{}\0{}", self.path.display(), self.length, self.modified_ns);
        semio_framework_async::block_on(semio_framework_plugin_host::default_compiled_cache_root()).join("component-identities").join(framework_hash::hash_bytes(key.as_bytes()))
    }

    fn known_hash(&self) -> Option<[u8; 32]> {
        if let Some(hash) = component_file_hashes().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(self).copied() {
            return Some(hash);
        }
        let hash: [u8; 32] = std::fs::read(self.persisted_path()).ok()?.try_into().ok()?;
        component_file_hashes().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(self.clone(), hash);
        Some(hash)
    }

    fn remember(&self, hash: [u8; 32]) {
        component_file_hashes().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(self.clone(), hash);
        let path = self.persisted_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(path, hash);
    }
}

/// 🧩️ Where a component's bytes come from: a staged file this process may already know, or bytes a
/// caller already holds (a hub-authorized execution target).
#[cfg(not(target_arch = "wasm32"))]
pub(super) enum ComponentBytes<'a> {
    File(&'a Path),
    Held(&'a [u8]),
}

/// 📏️ How much of a component one hashing step covers before it reports progress and checks cancel.
#[cfg(not(target_arch = "wasm32"))]
const COMPONENT_HASH_CHUNK_BYTES: usize = 4 << 20;

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn activation_fault(fault: Fault) -> GatewayError {
    if fault.code == ACTIVATION_CANCELLED_FAULT_CODE {
        return GatewayError::new(GatewayErrorCode::Cancelled, fault.message);
    }
    GatewayError::new(GatewayErrorCode::Internal, fault.message)
}

/// 🔏️ `bytes`' own 32-byte content digest, one chunk per progress report and cancel check.
///
/// 🔏️ The component's OWN digest, not `hash_bytes`' 64-char HEX STRING: that string's `into_bytes()`
/// is 64 long, so `try_into::<[u8; 32]>()` always failed and every plugin in this process took a
/// `[0u8; 32]` fallback. `WasmtimeRuntime::compile` keys its on-disk `.cwasm` cache on it, so the
/// placeholder made every plugin collide on ONE cache entry and replay whichever component compiled
/// first. Measured 2026-09-20: a single 130 MB `0000…0000.cwasm`.
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

/// 🧩️ The process's compiled handle for one component, reporting each phase it actually runs to
/// `scope`. A staged file already hashed in this process is neither read nor hashed again; compiled
/// code already in this process or on disk is loaded, never compiled again, and the component is
/// read only when it must be compiled. A cold compile runs in an isolated worker process shared by
/// every requester of the same content hash ([`CompileFlight`]), so a cancel detaches this requester
/// at once and the last requester's cancel kills the compile.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn scoped_compiled_component(runtime: &GuestRuntimes, plugin_id: &str, component: ComponentBytes<'_>, scope: &ActivationScope) -> Result<semio_framework_plugin_host::CompiledHandle, GatewayError> {
    let (package_hash, mut bytes) = match component {
        ComponentBytes::File(path) => {
            let metadata = std::fs::metadata(path).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("reading {}: {error}", path.display())))?;
            let identity = ComponentFileIdentity {
                path: path.to_path_buf(),
                length: metadata.len(),
                modified_ns: metadata.modified().ok().and_then(|modified| modified.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |since| since.as_nanos()),
            };
            match identity.known_hash() {
                Some(package_hash) => (package_hash, None),
                None => {
                    let bytes = read_component(path, scope)?;
                    let package_hash = scoped_component_hash(&bytes, scope)?;
                    identity.remember(package_hash);
                    (package_hash, Some(std::borrow::Cow::Owned(bytes)))
                }
            }
        }
        ComponentBytes::Held(bytes) => (scoped_component_hash(bytes, scope)?, Some(std::borrow::Cow::Borrowed(bytes))),
    };
    if let Some(compiled) = compiled_components().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&package_hash) {
        return Ok(compiled.clone());
    }
    scope.enter(ActivationPhase::LoadingCompiledCode).map_err(activation_fault)?;
    let package = semio_framework_plugin_host::PackageRef { package: semio_framework_plugin_host::PackageId(plugin_id.to_string()), hash: semio_framework_plugin_host::PackageHash(package_hash) };
    let compiled = match semio_framework_async::block_on(runtime.load_compiled(&package)) {
        Some(compiled) => compiled,
        None => {
            let bytes = match (bytes.take(), component) {
                (Some(bytes), _) => bytes,
                (None, ComponentBytes::File(path)) => std::borrow::Cow::Owned(read_component(path, scope)?),
                (None, ComponentBytes::Held(bytes)) => std::borrow::Cow::Borrowed(bytes),
            };
            scope.enter(ActivationPhase::CompilingComponent).map_err(activation_fault)?;
            match semio_framework_async::block_on(runtime.isolated_compile_target(&package)) {
                Some(target) => {
                    CompileFlight::join(package_hash, target, &bytes, scope)?;
                    semio_framework_async::block_on(runtime.load_compiled(&package)).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("compiling `{plugin_id}`: the isolated compile finished but its compiled code does not load")))?
                }
                None => semio_framework_async::block_on(runtime.compile(&package, &bytes)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("compiling `{plugin_id}`: {error}")))?,
            }
        }
    };
    compiled_components().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(package_hash, compiled.clone());
    Ok(compiled)
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
/// reads the same two inputs from its environment.
#[cfg(test)]
pub(crate) const ISOLATED_COMPILE_TEST_ENTRY: &str = "workspace::quick::isolated_compile_worker_entry";
#[cfg(test)]
pub(crate) const ISOLATED_COMPILE_TEST_ENGINE: &str = "SEMIO_ISOLATED_COMPILE_ENGINE";
#[cfg(test)]
pub(crate) const ISOLATED_COMPILE_TEST_OUT: &str = "SEMIO_ISOLATED_COMPILE_OUT";

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
    compile_flights().lock().unwrap_or_else(std::sync::PoisonError::into_inner).values().filter(|flight| flight.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).worker.is_some()).count()
}

/// 🧊️ One cold compile of one component, in an isolated worker process, shared by every requester
/// of the same content hash while it runs. A requester that cancels detaches and is answered at
/// once; when the last one detaches the worker is killed, which returns every core and byte the
/// compile held, and its unfinished output is removed. Nothing completes in the background after
/// its last requester is gone. Law: `🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json`.
#[cfg(not(target_arch = "wasm32"))]
struct CompileFlight {
    cache_path: PathBuf,
    pub(super) state: Mutex<CompileFlightState>,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct CompileFlightState {
    requesters: usize,
    waiters: Vec<semio_framework_async::oneshot::Sender<Result<(), String>>>,
    worker: Option<std::process::Child>,
    pub(super) outcome: Option<Result<(), String>>,
}

#[cfg(not(target_arch = "wasm32"))]
fn compile_flights() -> &'static Mutex<HashMap<[u8; 32], Arc<CompileFlight>>> {
    static FLIGHTS: OnceLock<Mutex<HashMap<[u8; 32], Arc<CompileFlight>>>> = OnceLock::new();
    FLIGHTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(not(target_arch = "wasm32"))]
impl CompileFlight {
    /// 🛫️ Joins the running compile of `package_hash`, or starts it, and waits for it or for the
    /// requester's cancel, whichever comes first.
    pub(super) fn join(package_hash: [u8; 32], target: semio_framework_plugin_host::IsolatedCompileTarget, bytes: &[u8], scope: &ActivationScope) -> Result<(), GatewayError> {
        let (flight, answer) = {
            let mut flights = compile_flights().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let flight = match flights.get(&package_hash) {
                Some(flight) => Arc::clone(flight),
                None => {
                    let flight = Self::launch(package_hash, target, bytes)?;
                    flights.insert(package_hash, Arc::clone(&flight));
                    flight
                }
            };
            let (sender, answer) = semio_framework_async::oneshot::channel();
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
                    Ok(Ok(())) => Ok(()),
                    Ok(Err(message)) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("isolated component compile failed: {message}"))),
                    Err(_) => Err(GatewayError::new(GatewayErrorCode::Internal, "the isolated component compile ended without answering")),
                }
            }
            semio_framework_async::Either::Right(()) => {
                flight.detach(package_hash);
                Err(GatewayError::new(GatewayErrorCode::Cancelled, "the caller cancelled while its component was compiling"))
            }
        }
    }

    /// 🚀️ Starts the worker and the thread that feeds it the component and observes its exit.
    fn launch(package_hash: [u8; 32], target: semio_framework_plugin_host::IsolatedCompileTarget, bytes: &[u8]) -> Result<Arc<Self>, GatewayError> {
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
        command.arg("compile-component").arg("--engine").arg(&target.engine).arg("--out").arg(&target.cache_path);
        #[cfg(test)]
        command.args([ISOLATED_COMPILE_TEST_ENTRY, "--exact", "--test-threads=1"]).env(ISOLATED_COMPILE_TEST_ENGINE, &target.engine).env(ISOLATED_COMPILE_TEST_OUT, &target.cache_path);
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
        std::thread::Builder::new()
            .name("semio-compile-flight".to_string())
            .spawn(move || observed.observe(package_hash, input, report, bytes))
            .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("observing the compile worker: {error}")))?;
        Ok(flight)
    }

    /// 👁️ Feeds the worker its component, waits for it to exit — its report closes when it does —
    /// and answers every requester. A worker already killed by the last detach answers no one.
    pub(super) fn observe(self: Arc<Self>, package_hash: [u8; 32], mut input: std::process::ChildStdin, mut report: std::process::ChildStderr, bytes: Vec<u8>) {
        use std::io::{Read as _, Write as _};
        let _ = input.write_all(&bytes);
        drop(input);
        drop(bytes);
        let mut message = String::new();
        let _ = report.read_to_string(&mut message);
        let mut flights = compile_flights().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(mut worker) = state.worker.take() else { return };
        let outcome = match worker.wait() {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => Err(format!("{status}: {}", message.trim())),
            Err(error) => Err(error.to_string()),
        };
        if outcome.is_err() {
            let _ = std::fs::remove_file(semio_framework_plugin_host::compiled_cache_scratch_path(&self.cache_path, worker.id()));
        }
        for waiter in state.waiters.drain(..) {
            let _ = waiter.send(outcome.clone());
        }
        state.outcome = Some(outcome);
        if flights.get(&package_hash).is_some_and(|flight| Arc::ptr_eq(flight, &self)) {
            flights.remove(&package_hash);
        }
    }

    fn leave(&self) {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.requesters -= 1;
    }

    /// ✂️ Withdraws one requester; the last one kills the worker and removes its unfinished output.
    fn detach(self: &Arc<Self>, package_hash: [u8; 32]) {
        let mut flights = compile_flights().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.requesters -= 1;
        if state.requesters > 0 || state.outcome.is_some() {
            return;
        }
        if let Some(mut worker) = state.worker.take() {
            let _ = worker.kill();
            let _ = worker.wait();
            let _ = std::fs::remove_file(semio_framework_plugin_host::compiled_cache_scratch_path(&self.cache_path, worker.id()));
        }
        state.waiters.clear();
        state.outcome = Some(Err("every requester cancelled; the compile was killed".to_string()));
        if flights.get(&package_hash).is_some_and(|flight| Arc::ptr_eq(flight, self)) {
            flights.remove(&package_hash);
        }
    }
}
//#endregion 🧊️CompileFlight

