//! 🛂️ `semio-framework-plugin-describe` — the build-time-only descriptor emitter
//! (`📓️design-abi.md` §3, packet E1-describe). `describe <component.wasm> --out <dir>`
//! instantiates the built `world actor` component exactly once, with ONLY its `pure` import
//! satisfied and a fuel cap, calls the `describe()` export, decodes the packed
//! `semio_framework::PackageDescriptor` it returns, patches in the content hashes (which the guest
//! itself cannot compute — it doesn't know its own already-built wasm bytes), and writes
//! `🛂️descriptor.semio` (pack bytes) + `🔣️descriptor.json` (readable mirror) to `--out <dir>`.
//! Never instantiated or invoked by the OS at runtime — see `🧬️schema/📜️component.wit`'s
//! `describe` interface doc.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;

use std::fs;
use std::path::{Path, PathBuf};

use semio_framework::{PackageDescriptor, ASSEMBLY_FAILED_PLUGIN_ID};
use sha2::{Digest, Sha256};

//#region 🔖️ActorBindings
mod actor_bindings {
    // 🐛️ `additional_derives` intentionally omitted — see the identical note in
    // `🔌️plugin/🖥️host/🦀️component.rs`'s own `mod actor_bindings`: wasmtime-wit-bindgen 22.0.1
    // hand-writes `Debug` for every WIT record/variant/enum regardless, so requesting it again here
    // would conflict.
    wasmtime::component::bindgen!({
        world: "actor",
        path: "../../../🧬️schema",
    });
}

/// 🧬️ `pure` (`📜️pure.wit`) is `world actor`'s ONLY import — `log`/`now-ms`/`trace-span`, none
/// fallible, none async. This emitter's host state carries nothing: `describe()` never legitimately
/// needs to log, read the clock, or trace — a component that does so during `describe()` is doing
/// something the descriptor contract does not ask for, but the calls are still satisfied (never
/// trapped) so a component that calls them for its own bookkeeping still completes.
struct DescribeHostState {
    wasi_ctx: wasmtime_wasi::WasiCtx,
    resource_table: wasmtime::component::ResourceTable,
}

/// 🌐️ WASI Preview 2, required even though `world actor` declares no wasi import: a real
/// `wasm32-wasip2` build pulls `wasi:io/poll` and friends in transitively via the Rust target's own
/// runtime shim, so `pure` alone leaves the linker short and instantiation fails. Sandboxed default
/// ctx — `describe()` is a pure metadata read and is granted no stdio, filesystem or network.
impl wasmtime_wasi::WasiView for DescribeHostState {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        wasmtime_wasi::WasiCtxView { ctx: &mut self.wasi_ctx, table: &mut self.resource_table }
    }
}

// 🚫️async: E1 — `wasmtime::component::bindgen!` generates this `Host` trait from the WIT, which
// declares `log`/`now-ms`/`trace-span` sync; the signature is external and fixed, not chosen here.
// See R9/R2 E1.
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
impl actor_bindings::semio::framework::types::Host for DescribeHostState {}
impl actor_bindings::semio::framework::capabilities::Host for DescribeHostState {}
impl actor_bindings::semio::framework::effects::Host for DescribeHostState {}
impl actor_bindings::semio::framework::events::Host for DescribeHostState {}
impl actor_bindings::semio::framework::ui::Host for DescribeHostState {}

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
fn describe_must_be_pure(name: &str) -> Vec<u8> {
    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("describe.impure"), format!("host-async {name} is not available during describe() — the descriptor contract requires describe() to be pure")))
}

/// 🚪️ `emit`/`emit-patch`, the fire-and-forget doors. Dropped with a loud stderr line rather than
/// silently: nothing consumes effects at describe time, and a `describe()` that emits one is the
/// same contract violation the 24 refusals above cover.
impl actor_bindings::semio::framework::host_async::Host for DescribeHostState {
    // 🚫️async: E1 — the WIT declares both sync (deliberate one-way doors); `bindgen!` mirrors that.
    fn emit(&mut self, _value: actor_bindings::semio::framework::effects::Effect) {
        eprintln!("[describe] ignoring host-async emit(): describe() must be pure");
    }

    fn emit_patch(&mut self, _patch: actor_bindings::semio::framework::ui::UiPatch) {
        eprintln!("[describe] ignoring host-async emit-patch(): describe() must be pure");
    }
}

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

    async fn document_read(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::DocumentReadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("document-read"))
    }

    async fn document_write(_accessor: &wasmtime::component::Accessor<DescribeHostState, Self>, _params: actor_bindings::semio::framework::effects::DocumentWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(describe_must_be_pure("document-write"))
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
/// must be sized against; `2_000_000_000` leaves ~21× headroom over the measured figure while still
/// bounding a runaway to seconds. Re-measure, do not re-estimate, if a larger plugin trips it.
const DESCRIBE_FUEL_BUDGET: u64 = 2_000_000_000;

/// 🚨️ Every way `describe_component` can fail, rendered as a plain message for the CLI's stderr.
#[derive(Debug)]
pub struct DescribeError(pub String);

impl std::fmt::Display for DescribeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// #️⃣ Lowercase hex SHA-256 of `bytes` — the literal algorithm `PackageHashes`' field names name
/// (`wasm_sha256`/`core_wasm_sha256`/`descriptor_sha256`), not this repo's usual `blake3`-based
/// `semio-framework-hash` (a different, Merkle-oriented content-addressing scheme with a different
/// hex alphabet) — package consumers outside this repo (registries, CI caches) expect literal
/// SHA-256 for a `*_sha256`-named field.
async fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex_encode(&hasher.finalize()).await
}

async fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// 🛂️ Instantiates `wasm_path` once (fuel-capped, `pure`-only imports), calls its `describe()`
/// export, patches `hashes` in (the guest cannot know its own already-built wasm bytes), and writes
/// both output files under `out_dir`. Returns the patched descriptor for the caller to print/verify.
///
/// ⚠️ `hashes.core_wasm_sha256` is set equal to `hashes.wasm_sha256`: this emitter receives exactly
/// one file (the already-componentized `.wasm`), not the pre-`wasm-tools component new` core module
/// it may have been assembled from, so it cannot independently hash a "core" module that was never
/// handed to it. A future caller that has both files can re-run with a lower-level flag; documented
/// here rather than silently left as an empty string (which would make the registry `check` gate's
/// `hashes.wasm_sha256` verification the only one ever meaningfully populated).
pub async fn describe_component(wasm_path: &Path, out_dir: &Path) -> Result<PackageDescriptor, DescribeError> {
    let wasm_bytes = fs::read(wasm_path).map_err(|error| DescribeError(format!("reading {}: {error}", wasm_path.display())))?;

    let mut config = wasmtime::Config::new();
    // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `describe()` is `async func`
    // now and `world actor` imports `host-async`, so the Store MUST be component-model-async — a
    // sync `func` export is uncallable on such a Store and an async one is uncallable without it.
    config.wasm_component_model_async(true);
    config.consume_fuel(true);
    let engine = wasmtime::Engine::new(&config).map_err(|error| DescribeError(format!("building wasmtime engine: {error}")))?;

    let mut linker = wasmtime::component::Linker::<DescribeHostState>::new(&engine);
    // 🧬️ Whole-world linker call: defines `pure` AND `host-async` together. Linking only `pure`
    // leaves 24 imports unresolved and every instantiation fails outright.
    actor_bindings::Actor::add_to_linker::<DescribeHostState, wasmtime::component::HasSelf<DescribeHostState>>(&mut linker, |state: &mut DescribeHostState| state).map_err(|error| DescribeError(format!("linking `world actor` imports: {error}")))?;
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(|error| DescribeError(format!("linking wasi preview 2: {error}")))?;

    let component = wasmtime::component::Component::from_binary(&engine, &wasm_bytes).map_err(|error| DescribeError(format!("parsing {} as a wasm component: {error}", wasm_path.display())))?;

    let mut store = wasmtime::Store::new(&engine, DescribeHostState { wasi_ctx: wasmtime_wasi::WasiCtxBuilder::new().build(), resource_table: wasmtime::component::ResourceTable::new() });
    store.set_fuel(DESCRIBE_FUEL_BUDGET).map_err(|error| DescribeError(format!("setting fuel budget: {error}")))?;

    let bindings = actor_bindings::Actor::instantiate_async(&mut store, &component, &linker).await.map_err(|error| DescribeError(format!("instantiating {}: {error}", wasm_path.display())))?;

    // 🧬️ `describe()` is `async func`, so it is driven through `Store::run_concurrent`'s `Accessor`
    // — the only shape wasmtime offers for an async-lifted export. Two error layers: the outer one
    // is `run_concurrent` itself, the inner one the call's own trap.
    let descriptor_bytes = store
        .run_concurrent(async |accessor| bindings.semio_framework_describe().call_describe(accessor).await)
        .await
        .map_err(|error| DescribeError(format!("calling describe() on {}: {error}", wasm_path.display())))?
        .map_err(|error| DescribeError(format!("calling describe() on {}: {error}", wasm_path.display())))?;

    let decoded = store::pack_rt::decode_wire_value(&descriptor_bytes).await.map_err(|error| DescribeError(format!("decoding describe() output as a pack: {error}")))?;
    let mut descriptor: PackageDescriptor = dsl::from_dsl_value(decoded).map_err(|error| DescribeError(format!("decoding describe() output as a PackageDescriptor: {error}")))?;

    let wasm_sha256 = sha256_hex(&wasm_bytes).await;
    descriptor.hashes.wasm_sha256 = wasm_sha256.clone();
    descriptor.hashes.core_wasm_sha256 = wasm_sha256;
    // 🪪️ `descriptor_sha256` self-hashes the descriptor's own encoded pack MINUS this very field
    // (a self-referential hash cannot include itself) — encode once with an empty
    // `descriptor_sha256`, hash THAT, then patch the real value in before the final write. Any
    // consumer re-deriving `descriptor_sha256` for verification must reproduce this exact two-pass
    // convention.
    descriptor.hashes.descriptor_sha256 = String::new();
    let prehash_value = dsl::to_dsl_value(&descriptor).map_err(|error| DescribeError(format!("encoding descriptor for hashing: {error}")))?;
    let prehash_bytes = store::pack_rt::encode_wire_value(&prehash_value).await;
    descriptor.hashes.descriptor_sha256 = sha256_hex(&prehash_bytes).await;

    let final_value = dsl::to_dsl_value(&descriptor).map_err(|error| DescribeError(format!("encoding final descriptor: {error}")))?;
    let final_bytes = store::pack_rt::encode_wire_value(&final_value).await;
    let final_json = serde_json::to_string_pretty(&descriptor).map_err(|error| DescribeError(format!("encoding descriptor as JSON: {error}")))?;

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
    fs::create_dir_all(out_dir).map_err(|error| DescribeError(format!("creating {}: {error}", out_dir.display())))?;
    fs::write(out_dir.join("🛂️descriptor.semio"), &final_bytes).map_err(|error| DescribeError(format!("writing 🛂️descriptor.semio: {error}")))?;
    fs::write(out_dir.join("🔣️descriptor.json"), format!("{final_json}\n")).map_err(|error| DescribeError(format!("writing 🔣️descriptor.json: {error}")))?;

    Ok(descriptor)
}
//#endregion 🔖️Describe

//#region 🔖️Cli
/// ⌨️ `describe <component.wasm> --out <dir>` — the only subcommand this crate has. Returns the
/// process exit code (0 success, 1 a `describe_component` failure, 2 a usage error).
pub async fn run(args: Vec<String>) -> i32 {
    let mut rest = args.into_iter();
    match rest.next().as_deref() {
        Some("describe") => run_describe(rest.collect()).await,
        Some(other) => {
            eprintln!("semio-framework-plugin-describe: unknown command {other:?} (expected \"describe\")");
            2
        }
        None => {
            eprintln!("usage: semio-framework-plugin-describe describe <component.wasm> --out <dir>");
            2
        }
    }
}

async fn run_describe(args: Vec<String>) -> i32 {
    let mut wasm_path: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--out" => out_dir = iter.next().map(PathBuf::from),
            _ if wasm_path.is_none() => wasm_path = Some(PathBuf::from(arg)),
            other => {
                eprintln!("semio-framework-plugin-describe describe: unexpected argument {other:?}");
                return 2;
            }
        }
    }
    let (Some(wasm_path), Some(out_dir)) = (wasm_path, out_dir) else {
        eprintln!("usage: semio-framework-plugin-describe describe <component.wasm> --out <dir>");
        return 2;
    };
    match describe_component(&wasm_path, &out_dir).await {
        Ok(descriptor) => {
            println!("described {} ({:?}, role={:?}) -> {}/🛂️descriptor.semio + 🔣️descriptor.json (wasm_sha256={})", wasm_path.display(), descriptor.manifest.plugin_id, descriptor.role, out_dir.display(), descriptor.hashes.wasm_sha256);
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sha256_hex_matches_known_vector() {
        // "" -> e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 (well-known empty-input SHA-256)
        assert_eq!(sha256_hex(b"").await, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[semio_framework_async_macros::async_test]
    async fn run_with_no_args_returns_usage_exit_code() {
        assert_eq!(run(Vec::new()).await, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn run_with_unknown_command_returns_usage_exit_code() {
        assert_eq!(run(vec!["not-describe".to_string()]).await, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn run_describe_without_out_flag_returns_usage_exit_code() {
        assert_eq!(run(vec!["describe".to_string(), "component.wasm".to_string()]).await, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn run_describe_on_missing_file_returns_failure_exit_code() {
        let code = run(vec!["describe".to_string(), "/nonexistent/component.wasm".to_string(), "--out".to_string(), "/tmp/does-not-matter".to_string()]).await;
        assert_eq!(code, 1);
    }
}
