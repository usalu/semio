//! 🛂️ `semio-framework-plugin-describe` — the build-time-only descriptor emitter
//! (`📓️design-abi.md` §3, packet E1-describe). `describe <component.wasm> --core
//! <core.wasm> --out <dir>`
//! instantiates the built `world actor` component exactly once, with ONLY its `pure` import
//! satisfied and a fuel cap, calls the `describe()` export, decodes the packed
//! `semio_framework::PackageDescriptor` it returns, patches in the content hashes (which the guest
//! itself cannot compute — it doesn't know its own already-built wasm bytes), and writes
//! `🛂️.descriptor.semio` (pack bytes) + `🔣️.json` (readable mirror) to `--out <dir>`.
//! Never instantiated or invoked by the OS at runtime — see `🧬️schema/📜️.wit`'s
//! `describe` interface doc.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;

/// 🧠️ The build-time descriptor emitter and the native plugin host share one owned interpreter
/// implementation; Wasmtime remains only as the differential oracle until component parity closes.
#[path = "../../🧠️interpreter/🦀️.rs"]
pub mod interpreter;

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use semio_framework::{PackageDescriptor, ASSEMBLY_FAILED_PLUGIN_ID};
use semio_framework_plugin_host::{GuestRuntime, OwnedRuntime, PackageHash, PackageId, PackageRef};

//#region 🔖️ActorBindings
#[cfg(test)]
#[path = "../🧪️tests/🔬️actor-bindings/🦀️.rs"]
mod actor_bindings;

/// 🧬️ `pure` (`📜️pure.wit`) is `world actor`'s ONLY import — `log`/`now-ms`/`trace-span`, none
/// fallible, none async. This emitter's host state carries nothing: `describe()` never legitimately
/// needs to log, read the clock, or trace — a component that does so during `describe()` is doing
/// something the descriptor contract does not ask for, but the calls are still satisfied (never
/// trapped) so a component that calls them for its own bookkeeping still completes.
#[cfg(test)]
struct DescribeHostState {
    wasi_ctx: wasmtime_wasi::WasiCtx,
    resource_table: wasmtime::component::ResourceTable,
}

/// 🌐️ WASI Preview 2, required even though `world actor` declares no wasi import: a real
/// `wasm32-wasip2` build pulls `wasi:io/poll` and friends in transitively via the Rust target's own
/// runtime shim, so `pure` alone leaves the linker short and instantiation fails. Sandboxed default
/// ctx — `describe()` is a pure metadata read and is granted no stdio, filesystem or network.
#[cfg(test)]
impl wasmtime_wasi::WasiView for DescribeHostState {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        wasmtime_wasi::WasiCtxView { ctx: &mut self.wasi_ctx, table: &mut self.resource_table }
    }
}

// 🚫️async: E1 — `wasmtime::component::bindgen!` generates this `Host` trait from the WIT, which
// declares `log`/`now-ms`/`trace-span` sync; the signature is external and fixed, not chosen here.
// See R9/R2 E1.
#[cfg(test)]
impl actor_bindings::semio::framework::pure::Host for DescribeHostState {
    fn log(&mut self, level: String, message: String) {
        eprintln!("[describe:{level}] {message}");
    }

    fn now_ms(&mut self) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as i64).unwrap_or(0)
    }

    fn trace_span(&mut self, name: String) {
        eprintln!("[describe:trace] {name}");
    }
}

/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): the five type-only interfaces
/// plus `ui`'s empty marker `resource surface`. `Actor::add_to_linker` (the whole-world linker call
/// the collapsed world requires) demands a `Host` impl for every interface `wit-parser` surfaces as
/// an import, including those present ONLY because an exported signature references their types.
/// Those traits declare no methods, so each impl is empty by construction.
#[cfg(test)]
impl actor_bindings::semio::framework::types::Host for DescribeHostState {}
#[cfg(test)]
impl actor_bindings::semio::framework::capabilities::Host for DescribeHostState {}
#[cfg(test)]
impl actor_bindings::semio::framework::effects::Host for DescribeHostState {}
#[cfg(test)]
impl actor_bindings::semio::framework::events::Host for DescribeHostState {}
#[cfg(test)]
impl actor_bindings::semio::framework::ui::Host for DescribeHostState {}
#[cfg(test)]
impl actor_bindings::semio::framework::instance_lifetime::Host for DescribeHostState {}

#[cfg(test)]
impl actor_bindings::semio::framework::ui::HostSurface for DescribeHostState {
    // 🚫️async: E1 — `bindgen!` fixes this resource-destructor signature. No host function here ever
    // hands a `surface` handle to the guest, so no handle exists to drop.
    fn drop(&mut self, _rep: wasmtime::component::Resource<actor_bindings::semio::framework::ui::Surface>) -> wasmtime::Result<()> {
        Ok(())
    }
}

//#region 🚫️host-async is refused, on purpose
/// 🚫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `describe()` MUST BE PURE. It is
/// build-time-only metadata extraction over a sandboxed Store with no services behind it, so every
/// one of `host-async`'s 24 awaitable imports resolves to this fault rather than to a real host
/// operation — a component whose `describe()` tries to read storage, fetch a URL or open a window is
/// doing something the descriptor contract does not ask for, and must fail LOUDLY at describe time
/// rather than emit a descriptor built from a half-satisfied environment.
///
/// The world still has to be fully linked (`Actor::add_to_linker` defines `pure` AND `host-async`
/// together, and an unresolved import fails instantiation outright), which is exactly why these
/// exist as refusals rather than as omissions.
#[cfg(test)]
fn describe_must_be_pure(name: &str) -> Vec<u8> {
    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("describe.impure"), format!("host-async {name} is not available during describe() — the descriptor contract requires describe() to be pure")))
}

/// 🚪️ `emit`/`emit-patch`, the fire-and-forget doors. Dropped with a loud stderr line rather than
/// silently: nothing consumes effects at describe time, and a `describe()` that emits one is the
/// same contract violation the 24 refusals above cover.
#[cfg(test)]
impl actor_bindings::semio::framework::host_async::Host for DescribeHostState {
    // 🚫️async: E1 — the WIT declares both sync (deliberate one-way doors); `bindgen!` mirrors that.
    fn emit(&mut self, _value: actor_bindings::semio::framework::effects::Effect) {
        eprintln!("[describe] ignoring host-async emit(): describe() must be pure");
    }

    fn emit_patch(&mut self, _patch: actor_bindings::semio::framework::ui::UiPatch) {
        eprintln!("[describe] ignoring host-async emit-patch(): describe() must be pure");
    }
}

#[cfg(test)]
impl actor_bindings::semio::framework::host_async::HostWithStore<DescribeHostState> for wasmtime::component::HasSelf<DescribeHostState> {
    async fn storage_read(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::StorageReadParams) -> Result<Option<Vec<u8>>, Vec<u8>> {
        Err(describe_must_be_pure("storage-read"))
    }

    async fn storage_write(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::StorageWriteParams) -> Result<(), Vec<u8>> {
        Err(describe_must_be_pure("storage-write"))
    }

    async fn storage_delete(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::StorageDeleteParams) -> Result<(), Vec<u8>> {
        Err(describe_must_be_pure("storage-delete"))
    }

    async fn blob_load(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::BlobLoadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("blob-load"))
    }

    async fn blob_write(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::BlobWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("blob-write"))
    }

    async fn blob_read(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _hash: String) -> Result<wasmtime::component::StreamReader<u8>, Vec<u8>> {
        Err(describe_must_be_pure("blob-read"))
    }

    async fn http_fetch(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::HttpParams) -> Result<actor_bindings::semio::framework::host_async::HttpResponse, Vec<u8>> {
        Err(describe_must_be_pure("http-fetch"))
    }

    async fn artifact_read(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::ArtifactReadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("artifact-read"))
    }

    async fn artifact_write(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::ArtifactWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("artifact-write"))
    }

    async fn link_resolve(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _link: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("link-resolve"))
    }

    async fn registry_query(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::RegistryQueryParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("registry-query"))
    }

    async fn io_compose(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::IoComposeParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("io-compose"))
    }

    async fn io_run(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::IoRunParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("io-run"))
    }

    async fn cache_derive(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::CacheDeriveParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("cache-derive"))
    }

    async fn cache_read(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::CacheReadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("cache-read"))
    }

    async fn invoke_extension(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::InvokeExtensionParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("invoke-extension"))
    }

    async fn open_window(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::OpenWindowParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("open-window"))
    }

    async fn open_dialog(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::OpenDialogParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("open-dialog"))
    }

    async fn dispatch_action(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::DispatchActionParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("dispatch-action"))
    }

    async fn spawn_plugin_instance(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::SpawnPluginInstanceParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("spawn-plugin-instance"))
    }

    async fn request_file_open(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::RequestFileOpenParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("request-file-open"))
    }

    async fn request_media_frames(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::RequestMediaFramesParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("request-media-frames"))
    }

    async fn request_capability(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::RequestCapabilityParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("request-capability"))
    }

    async fn spawn_job(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _job: u64, _kind: String, _input: Vec<u8>, _placement: actor_bindings::semio::framework::effects::JobPlacement) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("spawn-job"))
    }
}
//#endregion 🚫️host-async is refused, on purpose
//#endregion 🔖️ActorBindings

//#region 🔖️Describe
/// ⛽️ Fuel cap for the single `describe()` call, bounded so a malformed or hostile `describe()`
/// cannot hang the build. This call happens once, at build time, and does no IO/UI/effect work.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (D0/registrar): was `5_000_000`, described as
/// "generous for a pure struct-building function" — an estimate made against the *shape* of the
/// function, never against a real component. `🗒️note`'s own `describe()` measured **92_327_773**
/// fuel on an unoptimized `wasm32-wasip2` build: 18× the old cap, so every real plugin trapped
/// mid-`AppBuilder::try_build_definition` with a bare "error while executing" and no mention of
/// fuel. Debug wasm is the build the describe step actually consumes, so it is the build the cap
/// must be sized against; `semio-s-plugin-gis` exhausted **4_000_000_000** fuel at ~1_024_604 ms
/// (2026-09-17), so `8_000_000_000` is the next measured step. Re-measure, do not re-estimate, if a
/// larger plugin trips it.
const DESCRIBE_FUEL_BUDGET: u64 = 8_000_000_000;

/// ⏳️ Aggregate plugin bundles build several complete app catalogs in one pure descriptor call.
/// The ten-surface demonstrator exceeded the former single-plugin 60-second wall cap, and the
/// current full-catalog Space component exceeded the later five-minute cap while remaining within
/// the measured fuel bound. Thirty minutes preserves a finite wall deadline on constrained
/// development machines; the independent fuel cap remains the deterministic runaway bound.
const DESCRIBE_DEADLINE_MS: u32 = 1_800_000;

/// 🌱️ The build-time document id `codecs` mints its probe genesis at. It is a valid
/// `🌱️artifact-document-id-v1` (`artifact-` + 32 lowercase hex, nonzero) because `codec.genesis`
/// stamps the id into the history it returns and refuses a hostile one; the pair itself is read for
/// its schema and discarded, never published.
const CODEC_PROBE_DOCUMENT_ID: &str = "artifact-c0dec0dec0dec0dec0dec0dec0dec0de";

/// 🛡️ Ceiling for the build artifacts the emitter reads — the raw `wasm32-wasip2` component and
/// jco's extracted core, both built with the UNOPTIMIZED `wasm-dev` profile. This is a build-time
/// input bound and deliberately NOT the strict catalog's runtime ceiling
/// (`DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES`), which applies to the optimized artifact that
/// actually ships. Mirrored by `FRESH_COMPONENT_MAX_BYTES` in this crate's `📜️script.ts`.
pub const DESCRIBE_ARTIFACT_MAX_BYTES: u64 = 256 * 1024 * 1024;

/// 🧱 Fixed-size IO keeps hashing and publication memory/work bounded and observable.
pub const DESCRIBE_IO_CHUNK_BYTES: usize = 64 * 1024;

/// 🚨️ Every way `describe_component` can fail, rendered as a plain message for the CLI's stderr.
#[derive(Debug)]
pub struct DescribeError(pub String);

impl std::fmt::Display for DescribeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// 📄 Reads exactly one bounded regular non-symlink artifact in fixed-size chunks.
fn read_artifact(path: &Path, label: &str) -> Result<(Vec<u8>, String), DescribeError> {
    let metadata = fs::symlink_metadata(path).map_err(|error| DescribeError(format!("reading {label} metadata {}: {error}", path.display())))?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(DescribeError(format!("{label} {} must be a regular non-symlink file", path.display())));
    }
    if metadata.len() > DESCRIBE_ARTIFACT_MAX_BYTES {
        return Err(DescribeError(format!("{label} {} exceeds {DESCRIBE_ARTIFACT_MAX_BYTES} bytes", path.display())));
    }
    let mut file = File::open(path).map_err(|error| DescribeError(format!("opening {label} {}: {error}", path.display())))?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    let mut hash = semio_framework_hash::Sha256::new();
    let mut chunk = [0_u8; DESCRIBE_IO_CHUNK_BYTES];
    loop {
        let read = file.read(&mut chunk).map_err(|error| DescribeError(format!("reading {label} {}: {error}", path.display())))?;
        if read == 0 {
            break;
        }
        if bytes.len().saturating_add(read) > DESCRIBE_ARTIFACT_MAX_BYTES as usize {
            return Err(DescribeError(format!("{label} {} changed while reading or exceeds {DESCRIBE_ARTIFACT_MAX_BYTES} bytes", path.display())));
        }
        hash.update(&chunk[..read]);
        bytes.extend_from_slice(&chunk[..read]);
    }
    if bytes.len() as u64 != metadata.len() {
        return Err(DescribeError(format!("{label} {} changed while reading", path.display())));
    }
    Ok((bytes, semio_framework_hash::hex_lower(&hash.finalize())))
}

/// #️⃣ Derives distinct raw/core identities and refuses the historical raw-only substitution.
fn artifact_hashes(wasm_sha256: String, core_wasm_sha256: String) -> Result<(String, String), DescribeError> {
    if wasm_sha256 == core_wasm_sha256 {
        return Err(DescribeError("raw component and independently extracted core module have the same SHA-256".to_string()));
    }
    Ok((wasm_sha256, core_wasm_sha256))
}

fn transaction_path(out_dir: &Path, name: &str, role: &str) -> PathBuf {
    out_dir.join(format!(".{name}.{role}.{}", std::process::id()))
}

fn write_synced_new(path: &Path, bytes: &[u8]) -> Result<(), DescribeError> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path).map_err(|error| DescribeError(format!("creating {}: {error}", path.display())))?;
    file.write_all(bytes).map_err(|error| DescribeError(format!("writing {}: {error}", path.display())))?;
    file.sync_all().map_err(|error| DescribeError(format!("syncing {}: {error}", path.display())))
}

/// 🧩 Publishes the canonical pack/JSON pair as one rollback-safe transaction.
fn write_descriptor_pair_atomic(out_dir: &Path, pack: &[u8], json: &[u8]) -> Result<(), DescribeError> {
    fs::create_dir_all(out_dir).map_err(|error| DescribeError(format!("creating {}: {error}", out_dir.display())))?;
    let metadata = fs::symlink_metadata(out_dir).map_err(|error| DescribeError(format!("reading output directory {}: {error}", out_dir.display())))?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
        return Err(DescribeError(format!("output {} must be a regular non-symlink directory", out_dir.display())));
    }
    let pack_path = out_dir.join("🛂️.descriptor.semio");
    let json_path = out_dir.join("🔣️.json");
    let pack_temp = transaction_path(out_dir, "descriptor", "pack-new");
    let json_temp = transaction_path(out_dir, "descriptor", "json-new");
    let pack_backup = transaction_path(out_dir, "descriptor", "pack-old");
    let json_backup = transaction_path(out_dir, "descriptor", "json-old");
    for path in [&pack_temp, &json_temp, &pack_backup, &json_backup] {
        let _ = fs::remove_file(path);
    }
    if let Err(error) = write_synced_new(&pack_temp, pack).and_then(|_| write_synced_new(&json_temp, json)) {
        let _ = fs::remove_file(&pack_temp);
        let _ = fs::remove_file(&json_temp);
        return Err(error);
    }
    let had_pack = pack_path.exists();
    let had_json = json_path.exists();
    let result = (|| {
        if had_pack {
            fs::rename(&pack_path, &pack_backup).map_err(|error| DescribeError(format!("retiring {}: {error}", pack_path.display())))?;
        }
        if had_json {
            if let Err(error) = fs::rename(&json_path, &json_backup) {
                if had_pack {
                    let _ = fs::rename(&pack_backup, &pack_path);
                }
                return Err(DescribeError(format!("retiring {}: {error}", json_path.display())));
            }
        }
        if let Err(error) = fs::rename(&pack_temp, &pack_path) {
            if had_pack {
                let _ = fs::rename(&pack_backup, &pack_path);
            }
            if had_json {
                let _ = fs::rename(&json_backup, &json_path);
            }
            return Err(DescribeError(format!("publishing {}: {error}", pack_path.display())));
        }
        if let Err(error) = fs::rename(&json_temp, &json_path) {
            let _ = fs::remove_file(&pack_path);
            if had_pack {
                let _ = fs::rename(&pack_backup, &pack_path);
            }
            if had_json {
                let _ = fs::rename(&json_backup, &json_path);
            }
            return Err(DescribeError(format!("publishing {}: {error}", json_path.display())));
        }
        Ok(())
    })();
    for path in [&pack_temp, &json_temp, &pack_backup, &json_backup] {
        let _ = fs::remove_file(path);
    }
    result
}

#[cfg(test)]
async fn execute_describe_wasmtime(wasm_bytes: &[u8], source: &Path) -> Result<Vec<u8>, DescribeError> {
    let execution_bytes = interpreter::wasm_execution_binary(wasm_bytes).map_err(|error| DescribeError(format!("normalizing {} for execution: {error}", source.display())))?;
    let mut config = wasmtime::Config::new();
    config.wasm_component_model_async(true);
    config.consume_fuel(true);
    let engine = wasmtime::Engine::new(&config).map_err(|error| DescribeError(format!("building wasmtime engine: {error}")))?;
    let mut linker = wasmtime::component::Linker::<DescribeHostState>::new(&engine);
    actor_bindings::Actor::add_to_linker::<DescribeHostState, wasmtime::component::HasSelf<DescribeHostState>>(&mut linker, |state: &mut DescribeHostState| state).map_err(|error| DescribeError(format!("linking `world actor` imports: {error}")))?;
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(|error| DescribeError(format!("linking wasi preview 2: {error}")))?;
    let component = wasmtime::component::Component::from_binary(&engine, &execution_bytes).map_err(|error| DescribeError(format!("parsing {} as a wasm component: {error}", source.display())))?;
    let mut store = wasmtime::Store::new(&engine, DescribeHostState { wasi_ctx: wasmtime_wasi::WasiCtxBuilder::new().build(), resource_table: wasmtime::component::ResourceTable::new() });
    store.set_fuel(DESCRIBE_FUEL_BUDGET).map_err(|error| DescribeError(format!("setting fuel budget: {error}")))?;
    let bindings = actor_bindings::Actor::instantiate_async(&mut store, &component, &linker).await.map_err(|error| DescribeError(format!("instantiating {}: {error}", source.display())))?;
    store
        .run_concurrent(async |accessor| bindings.semio_framework_describe().call_describe(accessor).await)
        .await
        .map_err(|error| DescribeError(format!("calling describe() on {}: {error}", source.display())))?
        .map_err(|error| DescribeError(format!("calling describe() on {}: {error}", source.display())))
}

async fn execute_describe_owned(wasm_bytes: &[u8], source: &Path) -> Result<Vec<u8>, DescribeError> {
    let started = std::time::Instant::now();
    eprintln!("[describe] owned phase=compile bytes={} elapsed_ms=0", wasm_bytes.len());
    let runtime = OwnedRuntime::new();
    let package = PackageRef { package: PackageId(source.display().to_string()), hash: PackageHash([0; 32]) };
    let compiled = runtime.compile(&package, wasm_bytes).await.map_err(|error| DescribeError(format!("compiling {} with the owned interpreter: {error}", source.display())))?;
    eprintln!("[describe] owned phase=execute fuel=0 elapsed_ms={}", started.elapsed().as_millis());
    runtime
        .describe_observed(
            &compiled,
            semio_framework::kernel::Budget { fuel: DESCRIBE_FUEL_BUDGET, deadline_ms: DESCRIBE_DEADLINE_MS, max_effects: 0, max_patch_bytes: 0, max_frames: 0 },
            |fuel, elapsed| eprintln!("[describe] owned phase=execute fuel={fuel} elapsed_ms={}", elapsed.as_millis()),
        )
        .await
        .map_err(|error| DescribeError(format!("calling owned describe() on {}: {error}", source.display())))
}

/// 🛂️ Instantiates `wasm_path` once (fuel-capped, `pure`-only imports), calls its `describe()`
/// export, patches independent raw-component and extracted-core `hashes` in, and writes
/// both output files under `out_dir`. Returns the patched descriptor for the caller to print/verify.
pub async fn describe_component(wasm_path: &Path, core_wasm_path: &Path, out_dir: &Path) -> Result<PackageDescriptor, DescribeError> {
    let (wasm_bytes, wasm_sha256) = read_artifact(wasm_path, "raw component")?;
    let (_, core_wasm_sha256) = read_artifact(core_wasm_path, "extracted core module")?;
    let (wasm_sha256, core_wasm_sha256) = artifact_hashes(wasm_sha256, core_wasm_sha256)?;
    let descriptor_bytes = execute_describe_owned(&wasm_bytes, wasm_path).await?;

    let decoded = store::pack_rt::decode_wire_value(&descriptor_bytes).map_err(|error| DescribeError(format!("decoding describe() output as a pack: {error}")))?;
    let mut descriptor: PackageDescriptor = dsl::from_dsl_value(decoded).map_err(|error| DescribeError(format!("decoding describe() output as a PackageDescriptor: {error}")))?;

    descriptor.hashes.wasm_sha256 = wasm_sha256;
    descriptor.hashes.core_wasm_sha256 = core_wasm_sha256;
    // 🪪️ `descriptor_sha256` self-hashes the descriptor's own encoded pack MINUS this very field
    // (a self-referential hash cannot include itself) — encode once with an empty
    // `descriptor_sha256`, hash THAT, then patch the real value in before the final write. Any
    // consumer re-deriving `descriptor_sha256` for verification must reproduce this exact two-pass
    // convention.
    descriptor.hashes.descriptor_sha256 = String::new();
    let prehash_value = dsl::to_dsl_value(&descriptor).map_err(|error| DescribeError(format!("encoding descriptor for hashing: {error}")))?;
    let prehash_bytes = store::pack_rt::encode_wire_value(&prehash_value);
    descriptor.hashes.descriptor_sha256 = semio_framework_hash::sha256_hex(&prehash_bytes);

    let final_value = dsl::to_dsl_value(&descriptor).map_err(|error| DescribeError(format!("encoding final descriptor: {error}")))?;
    let final_bytes = store::pack_rt::encode_wire_value(&final_value);
    let final_json = store::json::to_string_pretty(&store::json::from_dsl_value(&final_value));

    // 🛡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (registrar): refuse to write a descriptor whose
    // assembly failed. `plugin_manifest()` mints a `pluginId: "assembly-failed"` stub when
    // `PLUGIN_ASSEMBLY_ERROR` is set, carrying the real error in `label` — a shape that looks like a
    // descriptor, passes JSON parsing, and feeds the generated registry catalog with fabricated
    // contributions. Three were committed this session by packets that emitted and then stalled
    // before verifying: the "never commit a placeholder" rule held only while an agent reached its
    // verification step, and enforced nothing when it did not. Failing at the writer makes the
    // invalid state unrepresentable instead of relying on every caller to remember.
    if descriptor.manifest.plugin_id == ASSEMBLY_FAILED_PLUGIN_ID {
        return Err(DescribeError(format!("refusing to write a placeholder descriptor for {}: plugin assembly failed — {}", wasm_path.display(), descriptor.manifest.label)));
    }
    write_descriptor_pair_atomic(out_dir, &final_bytes, format!("{final_json}\n").as_bytes())?;

    Ok(descriptor)
}
//#endregion 🔖️Describe

//#region 🔖️Cli
/// 🧬️ One document kind a built component answers `world actor`'s `codec` interface for.
pub struct ComponentCodecRow {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub pack_schema_hash: String,
}

/// 🧬️ Asks a built component itself which document kinds it owns and what each one's structural
/// snapshot fingerprint is, so a catalog builder never has to transcribe a plugin's pack record
/// specification by hand. Both answers come out of the same hash-identified component bytes the
/// caller is about to publish:
///
/// * `codec.genesis(kind, probe-id)` mints the kind's canonical empty document. Its `spr` is a
///   `HistoryLog` whose `schema` field IS `A::DOCUMENT_SCHEMA` — the string `store::ArtifactCodec`
///   and the hub's trusted catalog are keyed by, and the only place a package publishes it.
/// * `codec.pack-schema-hash(schema)` then returns that kind's 32-byte record fingerprint.
///
/// `kinds` are dialect artifact kinds as a descriptor's `manifest.apps[].dialect.artifact_kind`
/// spells them; a kind the bundle owns no app for is an error, not a silent omission.
pub async fn component_codec_rows(wasm_path: &Path, kinds: &[String]) -> Result<Vec<ComponentCodecRow>, DescribeError> {
    let (wasm_bytes, _) = read_artifact(wasm_path, "raw component")?;
    let runtime = OwnedRuntime::new();
    let package = PackageRef { package: PackageId(wasm_path.display().to_string()), hash: PackageHash([0; 32]) };
    let compiled = runtime.compile(&package, &wasm_bytes).await.map_err(|error| DescribeError(format!("compiling {} with the owned interpreter: {error}", wasm_path.display())))?;
    let budget = semio_framework::kernel::Budget { fuel: DESCRIBE_FUEL_BUDGET, deadline_ms: DESCRIBE_DEADLINE_MS, max_effects: 0, max_patch_bytes: 0, max_frames: 0 };
    let mut rows = Vec::with_capacity(kinds.len());
    for kind in kinds {
        let pair = runtime.codec_genesis(&compiled, kind, CODEC_PROBE_DOCUMENT_ID, budget).await.map_err(|error| DescribeError(format!("codec.genesis({kind}) on {}: {error}", wasm_path.display())))?;
        let history = store::os_spr::decode_history(&pair.spr, &store::os_spr::DecodeOptions::default())
            .await
            .map_err(|error| DescribeError(format!("decoding the genesis history of {kind}: {error}")))?;
        if history.doc_id != CODEC_PROBE_DOCUMENT_ID || !history.edits.is_empty() {
            return Err(DescribeError(format!("genesis of {kind} is not the canonical empty document at the requested id")));
        }
        if history.schema.is_empty() || history.schema.len() > 256 {
            return Err(DescribeError(format!("genesis of {kind} carries no bounded document schema")));
        }
        let hash = runtime.codec_pack_schema_hash(&compiled, &history.schema, budget).await.map_err(|error| DescribeError(format!("codec.pack-schema-hash({}) on {}: {error}", history.schema, wasm_path.display())))?;
        if hash == [0; 32] {
            return Err(DescribeError(format!("document schema {} has no structural record specification", history.schema)));
        }
        rows.push(ComponentCodecRow { artifact_kind: kind.clone(), artifact_schema: history.schema, pack_schema_hash: semio_framework_hash::hex_lower(&hash) });
    }
    Ok(rows)
}

/// ⌨️ `describe <component.wasm> --core <core.wasm> --out <dir>` and `codecs <component.wasm>
/// --kinds <k1,k2,…> --out <file.json>`. Returns the process exit code (0 success, 1 a failure,
/// 2 a usage error).
pub async fn run(args: Vec<String>) -> i32 {
    let mut rest = args.into_iter();
    match rest.next().as_deref() {
        Some("describe") => run_describe(rest.collect()).await,
        Some("codecs") => run_codecs(rest.collect()).await,
        Some(other) => {
            eprintln!("semio-framework-plugin-describe: unknown command {other:?} (expected \"describe\" or \"codecs\")");
            2
        }
        None => {
            eprintln!("usage: semio-framework-plugin-describe describe <component.wasm> --core <core.wasm> --out <dir>");
            2
        }
    }
}

async fn run_codecs(args: Vec<String>) -> i32 {
    let mut wasm_path: Option<PathBuf> = None;
    let mut kinds: Vec<String> = Vec::new();
    let mut out_path: Option<PathBuf> = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--kinds" => kinds = iter.next().map(|value| value.split(',').filter(|part| !part.is_empty()).map(str::to_string).collect()).unwrap_or_default(),
            "--out" => out_path = iter.next().map(PathBuf::from),
            _ if wasm_path.is_none() => wasm_path = Some(PathBuf::from(arg)),
            other => {
                eprintln!("semio-framework-plugin-describe codecs: unexpected argument {other:?}");
                return 2;
            }
        }
    }
    let (Some(wasm_path), Some(out_path)) = (wasm_path, out_path) else {
        eprintln!("usage: semio-framework-plugin-describe codecs <component.wasm> --kinds <k1,k2,…> --out <file.json>");
        return 2;
    };
    if kinds.is_empty() {
        eprintln!("semio-framework-plugin-describe codecs: --kinds must name at least one dialect artifact kind");
        return 2;
    }
    match component_codec_rows(&wasm_path, &kinds).await {
        Ok(rows) => {
            let body = rows
                .iter()
                .map(|row| format!("{{\"artifactKind\":\"{}\",\"artifactSchema\":\"{}\",\"packSchemaHash\":\"{}\"}}", row.artifact_kind, row.artifact_schema, row.pack_schema_hash))
                .collect::<Vec<_>>()
                .join(",");
            let document = format!("{{\"schema\":\"semio.plugin.component-codec-rows/v1\",\"rows\":[{body}]}}\n");
            if let Some(parent) = out_path.parent() {
                if let Err(error) = fs::create_dir_all(parent) {
                    eprintln!("semio-framework-plugin-describe codecs: creating {}: {error}", parent.display());
                    return 1;
                }
            }
            match fs::write(&out_path, document.as_bytes()) {
                Ok(()) => {
                    println!("codecs {} -> {} ({} rows)", wasm_path.display(), out_path.display(), rows.len());
                    0
                }
                Err(error) => {
                    eprintln!("semio-framework-plugin-describe codecs: writing {}: {error}", out_path.display());
                    1
                }
            }
        }
        Err(error) => {
            eprintln!("semio-framework-plugin-describe codecs: {error}");
            1
        }
    }
}

async fn run_describe(args: Vec<String>) -> i32 {
    let mut wasm_path: Option<PathBuf> = None;
    let mut core_wasm_path: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--core" => core_wasm_path = iter.next().map(PathBuf::from),
            "--out" => out_dir = iter.next().map(PathBuf::from),
            _ if wasm_path.is_none() => wasm_path = Some(PathBuf::from(arg)),
            other => {
                eprintln!("semio-framework-plugin-describe describe: unexpected argument {other:?}");
                return 2;
            }
        }
    }
    let (Some(wasm_path), Some(core_wasm_path), Some(out_dir)) = (wasm_path, core_wasm_path, out_dir) else {
        eprintln!("usage: semio-framework-plugin-describe describe <component.wasm> --core <core.wasm> --out <dir>");
        return 2;
    };
    match describe_component(&wasm_path, &core_wasm_path, &out_dir).await {
        Ok(descriptor) => {
            println!("described {} with core {} ({:?}, role={:?}) -> {}/🛂️.descriptor.semio + 🔣️.json (wasm_sha256={}, core_wasm_sha256={})", wasm_path.display(), core_wasm_path.display(), descriptor.manifest.plugin_id, descriptor.role, out_dir.display(), descriptor.hashes.wasm_sha256, descriptor.hashes.core_wasm_sha256);
            0
        }
        Err(error) => {
            eprintln!("semio-framework-plugin-describe describe: {error}");
            1
        }
    }
}
//#endregion 🔖️Cli

#[cfg(test)]
#[path = "../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
