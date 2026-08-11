//! 🛡️ Sandboxed wasmtime component plugin host with capability-gated imports.

use semio_framework::{
    kernel::{ArtifactKind, CapabilityRequirement, Rights, Scope},
    PluginManifest, TopicContribution, ViewModel,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use ui_wgpu::wgpu::{UtilityNode, WindowEngagement, WindowMeasure};
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

const PLUGIN_FUEL_BUDGET: u64 = 50_000_000;

bindgen!({
    world: "plugin-world",
    path: "../../../📦️packages/🦀️rust/📜️wit",
    async: false,
});

//#region ⚠️ Errors
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
    #[error("{0} lock poisoned")]
    LockPoisoned(&'static str),
}

//#endregion ⚠️ Errors

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
        let Some(codec) = store::document_codec(schema) else {
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
            Err(error) => {
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
}

impl ArtifactSession {
    /// 🏗️ Empty per-instance session (no packs yet).
    pub fn new() -> Self {
        Self::default()
    }
}

const DEFAULT_ENGINE_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;
//#endregion 🔖️ArtifactSession

//#region 🔖️IoRouter
/// 🌉️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION (D3): the host
/// half of cross-plugin artifact reuse. Each `WasmPluginRuntime`'s own in-guest `IO_REGISTRY` only
/// ever sees composers registered inside ITS OWN wasm linear memory; this router is what makes a
/// key owned by plugin B actually reachable from plugin A's `host.io-compose` import — a single
/// shared table (keyed exactly like `semio_framework::IoKey`) built by calling `list-artifact-
/// dialects` on every plugin as it loads, mapping each key to the plugin id that owns it, plus a
/// handle to that plugin's own `WasmPluginRuntime` to actually forward the call.
pub struct IoRouter {
    routes: Mutex<HashMap<semio_framework::IoKey, String>>,
    runtimes: Mutex<HashMap<String, Arc<WasmPluginRuntime>>>,
}

impl IoRouter {
    pub fn new() -> Self {
        Self { routes: Mutex::new(HashMap::new()), runtimes: Mutex::new(HashMap::new()) }
    }

    /// 📌️ Registers one already-loaded plugin's runtime + merges its composer roster into the
    /// shared route table. Call once per plugin, after `WasmPluginRuntime::load` succeeds.
    pub fn register_plugin(&self, plugin_id: &str, runtime: Arc<WasmPluginRuntime>) -> Result<(), PluginHostError> {
        let wire_bytes = runtime.list_artifact_dialects()?;
        let entries: Vec<(semio_framework::ArtifactDialect, Vec<semio_framework::ArtifactDialect>)> =
            serde_json::from_slice(&wire_bytes).map_err(PluginHostError::Json)?;
        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("io router routes"))?;
        for (writes, reads) in entries {
            for read in &reads {
                routes.insert(
                    semio_framework::IoKey {
                        artifact_kind: writes.artifact_kind.clone(),
                        standard: writes.standard.clone(),
                        subset: writes.subset.clone(),
                        direction: semio_framework::IoDirection::Import,
                        format_kind: read.artifact_kind.clone(),
                        format_standard: read.standard.clone(),
                        format_subset: read.subset.clone(),
                    },
                    plugin_id.to_string(),
                );
                routes.insert(
                    semio_framework::IoKey {
                        artifact_kind: read.artifact_kind.clone(),
                        standard: read.standard.clone(),
                        subset: read.subset.clone(),
                        direction: semio_framework::IoDirection::Export,
                        format_kind: writes.artifact_kind.clone(),
                        format_standard: writes.standard.clone(),
                        format_subset: writes.subset.clone(),
                    },
                    plugin_id.to_string(),
                );
            }
        }
        drop(routes);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("io router runtimes"))?.insert(plugin_id.to_string(), runtime);
        Ok(())
    }

    /// 📊️ `N plugins / M keys` — logged at boot so a dev-boot smoke test can confirm the router
    /// actually picked up more than zero cross-plugin routes.
    pub fn stats(&self) -> (usize, usize) {
        let plugins = self.runtimes.lock().map(|r| r.len()).unwrap_or(0);
        let keys = self.routes.lock().map(|r| r.len()).unwrap_or(0);
        (plugins, keys)
    }

    /// 🌉️ Routes `key`/`sources` (JSON wire bytes) to whichever OTHER plugin owns `key`. Refuses to
    /// route back into `calling_plugin_id` itself: the target plugin's `artifact-compose` guest
    /// handler is local-only by construction (see `io::wire_artifact_compose`'s own doc comment) and
    /// never calls `io-compose` again, so every route is exactly one hop — the self-route guard is
    /// what keeps a plugin from ever needing to reason about calling back into its own in-flight
    /// `Store` mutex (which would deadlock, since that mutex is already held by the caller of this
    /// very host call).
    pub fn compose(&self, calling_plugin_id: &str, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let key: semio_framework::IoKey = serde_json::from_slice(key_bytes).map_err(|e| format!("bad io key wire bytes: {e}"))?;
        let owner = {
            let routes = self.routes.lock().map_err(|_| "io router routes lock poisoned".to_string())?;
            routes.get(&key).cloned()
        }
        .ok_or_else(|| format!("no plugin registered for {}/{}/{} {:?} {}/{}/{}", key.artifact_kind, key.standard, key.subset, key.direction, key.format_kind, key.format_standard, key.format_subset))?;
        if owner == calling_plugin_id {
            return Err(format!("io-compose refused: plugin `{calling_plugin_id}` would be routing to itself (should have resolved locally)"));
        }
        let runtime = {
            let runtimes = self.runtimes.lock().map_err(|_| "io router runtimes lock poisoned".to_string())?;
            runtimes.get(&owner).cloned()
        }
        .ok_or_else(|| format!("plugin `{owner}` owns this key but its runtime is not registered with the router"))?;
        runtime.artifact_compose(key_bytes, sources_bytes).map_err(|error| error.to_string())
    }

    /// 📚️ Every dialect ANY loaded plugin can move `artifact_kind` through in `direction`
    /// ("import"|"export"), JSON `Vec<ArtifactDialect>` bytes.
    pub fn dialects(&self, artifact_kind: &str, direction: &str) -> Result<Vec<u8>, String> {
        let direction = match direction {
            "import" => semio_framework::IoDirection::Import,
            "export" => semio_framework::IoDirection::Export,
            other => return Err(format!("unknown io direction `{other}` (expected \"import\" or \"export\")")),
        };
        let routes = self.routes.lock().map_err(|_| "io router routes lock poisoned".to_string())?;
        let dialects: Vec<semio_framework::ArtifactDialect> = routes
            .keys()
            .filter(|key| key.artifact_kind == artifact_kind && key.direction == direction)
            .map(|key| semio_framework::ArtifactDialect { artifact_kind: key.format_kind.clone(), standard: key.format_standard.clone(), subset: key.format_subset.clone() })
            .collect();
        serde_json::to_vec(&dialects).map_err(|error| error.to_string())
    }
}

impl Default for IoRouter {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️IoRouter

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
        let handle = self
            .engines
            .derive(&engine_id, &input)
            .map_err(|error| host_fault_bytes("os.host.engine-derive", error.to_string()))?;
        Ok(handle.key.0.to_vec())
    }

    fn io_dialects(&mut self, artifact_kind: String, direction: String) -> Result<Vec<u8>, Vec<u8>> {
        let router = self.io_router.as_ref().ok_or_else(|| host_fault_bytes("os.host.io-dialects", "no host io router registered; call register_host_io_router"))?;
        router.dialects(&artifact_kind, &direction).map_err(|error| host_fault_bytes("os.host.io-dialects", error))
    }

    fn io_compose(&mut self, key: Vec<u8>, sources: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        let router = self.io_router.as_ref().ok_or_else(|| host_fault_bytes("os.host.io-compose", "no host io router registered; call register_host_io_router"))?;
        router.compose(&self.plugin_id, &key, &sources).map_err(|error| host_fault_bytes("os.host.io-compose", error))
    }

    fn engine_read(&mut self, engine_id: String, key: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        if !self.has_engine_access(Rights::Read) {
            return Err(host_fault_bytes("os.host.engine-read", "engine read capability missing"));
        }
        let key_bytes: [u8; 32] = key
            .as_slice()
            .try_into()
            .map_err(|_| host_fault_bytes("os.host.engine-read", format!("engine key must be 32 bytes, got {}", key.len())))?;
        let handle = store::EngineHandle {
            key: store::EngineKey(key_bytes),
            engine_id,
        };
        self.engines
            .read(&handle)
            .map_err(|error| host_fault_bytes("os.host.engine-read", error.to_string()))
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

    fn prepare_call(store: &mut Store<HostState>) {
        store.set_fuel(PLUGIN_FUEL_BUDGET).ok();
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
    pub fn bind_session_schemas(
        &self,
        instance_id: u32,
        document_schema: impl Into<Option<String>>,
        config_schema: impl Into<Option<String>>,
        draft_schema: impl Into<Option<String>>,
    ) {
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
                    let document_schema = session
                        .document_schema
                        .clone()
                        .or_else(|| store::lane_schema_from_spr(&session.document.spr));
                    let config_schema = session.config_schema.clone().or_else(|| store::lane_schema_from_spr(&session.config.spr));
                    let draft_schema = session.draft_schema.clone().or_else(|| store::lane_schema_from_spr(&session.draft.spr));
                    session.document.apply_emit_ops(document_schema.as_deref(), document_ops);
                    session.config.apply_emit_ops(config_schema.as_deref(), config_ops);
                    session.draft.apply_emit_ops(draft_schema.as_deref(), draft_ops);
                    session.command_log_len = session.command_log_len.saturating_add(1);
                    session.generation = session.generation.saturating_add(1);
                }
                _ => {}
            }
        }
    }

    /// 🖱️ On-demand context menu via `AppCommand::ContextMenu` on the plugin exchange channel.
    pub fn context_menu(&self, instance_id: u32, request: serde_json::Value) -> Result<Vec<ui_wgpu::wgpu::ContextMenuItemSpec>, PluginHostError> {
        use protocol::{decode_app_frame, encode_app_command, AppCommand, AppFrame};
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEQ: AtomicU64 = AtomicU64::new(1);
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
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
                AppFrame::Error { in_reply_to, fault } if in_reply_to == Some(seq) => {
                    let decoded = dsl::decode_fault_bytes(&fault);
                    return Err(PluginHostError::Plugin(decoded.message));
                }
                _ => {}
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
        bindings.semio_framework_plugin().call_list_artifact_dialects(&mut *store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))
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
        let manifest = PluginManifest { plugin_id: "unknown".into(), label: "Unknown".into(), version: "0.0.0".into(), apps: vec![], examples: vec![], capabilities: vec![], topic_contributions: vec![], commands: vec![], artifact_kinds: vec![] };
        let mut store = Store::new(engine, Self::host_state("bootstrap", &manifest));
        let (bindings, _instance) = PluginWorld::instantiate(&mut store, component, linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let wire_bytes = bindings.semio_framework_plugin().call_manifest(&mut store).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let value = store::pack_rt::decode_wire_value(&wire_bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        let value = store::pack_rt::renormalize_whole_number_floats(value);
        Ok(dsl::from_dsl_value(value).map_err(|error| PluginHostError::Plugin(error))?)
    }

    fn instantiate(mut store: Store<HostState>, component: &Component, linker: &Linker<HostState>) -> Result<(Store<HostState>, PluginWorld), PluginHostError> {
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
        world: "extension-world",
        path: "../../../📦️packages/🦀️rust/📜️wit",
        async: false,
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
        let (bindings, _instance) =
            extension_bindings::ExtensionWorld::instantiate(&mut store, &component, &self.linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let manifest = Self::decode_manifest(&mut store, &bindings)?;
        store.data_mut().extension_id = manifest.extension_id.clone();
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
            instances
                .get(extension_id)
                .cloned()
                .ok_or_else(|| dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("extension.unknown"), format!("no extension loaded with id `{extension_id}`")))?
        };
        let mut store = loaded.store.lock().map_err(|_| dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("extension.lock-poisoned"), "extension store lock poisoned"))?;
        store.set_fuel(PLUGIN_FUEL_BUDGET).ok();
        let result = loaded
            .bindings
            .semio_framework_extension()
            .call_invoke(&mut *store, capability, request)
            .map_err(|error| dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new("extension.wasmtime"), error.to_string()))?;
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
        let _ = std::mem::size_of::<WasmPluginRuntime>();
    }

    #[test]
    fn extension_runtime_constructs_engine_and_linker() {
        let runtime = ExtensionRuntime::new().expect("extension runtime engine/linker build");
        assert!(runtime.manifest("nonexistent").is_none());
        let error = runtime.extension_invoke("nonexistent", "noop", &[]).expect_err("unknown extension id must fault");
        assert_eq!(error.code.0, "extension.unknown");
    }
}
