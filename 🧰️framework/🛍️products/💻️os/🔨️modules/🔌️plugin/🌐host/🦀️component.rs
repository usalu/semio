//! 🌐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): the async host-capability
//! API, replacing `host_port`/`component::host_*`. `log`/`now_ms`/`trace_span` stay synchronous
//! (they wrap the `pure` WIT import, the world's only import); everything else is an `async fn`
//! that builds a `semio_framework::kernel::Effect`, hands it to a `⚛️reactor/📮️requests::
//! RequestRegistry`, and awaits the matching `Event::Completed`.
//!
//! `Host` is constructed per app instance (never a process-global singleton — see
//! `important.md`'s "Replace, never wrap" list: `set_host_backbone_channel` is explicitly one of
//! the things a pooled multi-instance actor cannot keep). Cloning is a cheap `Rc` bump (the
//! wrapped `RequestRegistry` already is one).

use crate::reactor::requests::RequestRegistry;
use dsl::DslValue;
use semio_framework::kernel::{CapabilityId, ClipboardFragment, Effect, IconRenderExportItem, JobPlacement, RequestId, RequestOutcome, WindowHandle, WindowKindId};
use semio_framework::{Fault, MediaType};

/// 🩹️ Decodes a `RequestOutcome` into the `Result<Vec<u8>, Fault>` every `host::*` async call
/// resolves to — `Err` bytes are `dsl::encode_fault_bytes` output, the SAME convention every
/// synchronous `host_*` wrapper already used. Called from `⚛️reactor/🦀️component.rs`'s
/// `Event::Completed` routing step before it hands the result to `RequestRegistry::resolve` —
/// that routing step lives in `wit_bridge`, so this is gated identically (native never reaches it).
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub(crate) fn outcome_to_result(outcome: RequestOutcome) -> Result<Vec<u8>, Fault> {
    match outcome {
        RequestOutcome::Ok(bytes) => Ok(bytes),
        RequestOutcome::Err(bytes) => Err(dsl::decode_fault_bytes(&bytes)),
    }
}

/// 🌐️ Per-instance async host-capability handle — see module doc.
#[derive(Clone, Default)]
pub struct Host {
    registry: RequestRegistry,
}

impl Host {
    pub fn new(registry: RequestRegistry) -> Self {
        Self { registry }
    }

    pub fn registry(&self) -> &RequestRegistry {
        &self.registry
    }

    async fn call(&self, build: impl FnOnce(RequestId) -> Effect) -> Result<Vec<u8>, Fault> {
        let bytes = self.registry.request(build).await?;
        // `request()`'s `Ok` is already the raw completion payload for effects whose `Event::
        // Completed.result` the reactor turn loop unwraps before calling `resolve` — see
        // `⚛️reactor/🦀️component.rs`'s event-routing step. Kept as a passthrough here so a future
        // reactor change (deferring the `RequestOutcome` unwrap to this call site instead) is a
        // one-line edit, not a signature break.
        Ok(bytes)
    }

    //#region 🔖️Blobs
    pub async fn blob_load(&self, hash: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let hash = hash.into();
        self.call(move |req| Effect::BlobLoad { req, hash }).await
    }

    pub async fn blob_write(&self, media_type: MediaType, bytes: Vec<u8>) -> Result<Vec<u8>, Fault> {
        self.call(move |req| Effect::BlobWrite { req, media_type, bytes }).await
    }
    //#endregion 🔖️Blobs

    //#region 🔖️Http
    #[allow(clippy::too_many_arguments)]
    pub async fn http_request(&self, method: impl Into<String>, url: impl Into<String>, headers: Vec<(String, String)>, body: Option<Vec<u8>>, stream: bool) -> Result<Vec<u8>, Fault> {
        let method = method.into();
        let url = url.into();
        self.call(move |req| Effect::HttpRequest { req, method, url, headers, body, stream }).await
    }
    //#endregion 🔖️Http

    //#region 🔖️Documents
    pub async fn document_read(&self, doc: semio_framework::kernel::ArtifactHandle, lane: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let lane = lane.into();
        self.call(move |req| Effect::DocumentRead { req, doc, lane }).await
    }

    pub async fn document_write(&self, doc: semio_framework::kernel::ArtifactHandle, lane: impl Into<String>, ops: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let lane = lane.into();
        self.call(move |req| Effect::DocumentWrite { req, doc, lane, ops }).await
    }
    //#endregion 🔖️Documents

    //#region 🔖️Links
    pub async fn resolve_link(&self, link: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let link = link.into();
        self.call(move |req| Effect::LinkResolve { req, link }).await
    }
    //#endregion 🔖️Links

    //#region 🔖️Io — CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM absorption
    /// 🌉️ Absorbs the old host import `io-routes`: `kind = "io-routes"`, `filter` is the
    /// `{source, target}` query — see `effects.wit`'s `registry-query`.
    pub async fn io_routes(&self, source: &str, target: &str) -> Result<Vec<u8>, Fault> {
        #[derive(serde::Serialize)]
        struct IoRoutesFilter<'a> {
            source: &'a str,
            target: &'a str,
        }
        let filter = dsl::to_dsl_value(&IoRoutesFilter { source, target }).ok();
        self.registry_query("io-routes", filter).await
    }

    /// 🌉️ Absorbs the old host import `io-identify`: `kind = "io-identify"`.
    pub async fn io_identify(&self, payload: &[u8]) -> Result<Vec<u8>, Fault> {
        #[derive(serde::Serialize)]
        struct IoIdentifyFilter {
            payload: Vec<u8>,
        }
        let filter = dsl::to_dsl_value(&IoIdentifyFilter { payload: payload.to_vec() }).ok();
        self.registry_query("io-identify", filter).await
    }

    /// 🌉️ Absorbs the old host import `io-run` (multi-hop, cross-plugin) — distinct from the
    /// `semio.io-run` COLD JOB kind, which is the single-hop, THIS-plugin-only registry lookup
    /// (see `⚛️reactor/💼️jobs`).
    pub async fn io_run(&self, source: impl Into<String>, target: impl Into<String>, payload: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let source = source.into();
        let target = target.into();
        self.call(move |req| Effect::IoCompose { req, key: format!("{source}->{target}"), sources: vec![String::from_utf8_lossy(&payload).into_owned()] }).await
    }

    /// 🌉️ One-hop compose, unchanged semantics from the old `io-compose` host import.
    pub async fn io_compose(&self, key: impl Into<String>, sources: Vec<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        self.call(move |req| Effect::IoCompose { req, key, sources }).await
    }
    //#endregion 🔖️Io

    //#region 🔖️Registry
    pub async fn registry_query(&self, kind: impl Into<String>, filter: Option<DslValue>) -> Result<Vec<u8>, Fault> {
        let kind = kind.into();
        self.call(move |req| Effect::RegistryQuery { req, kind, filter }).await
    }
    //#endregion 🔖️Registry

    //#region 🔖️Cache
    pub async fn cache_derive(&self, engine_id: impl Into<String>, input: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let engine_id = engine_id.into();
        self.call(move |req| Effect::CacheDerive { req, engine_id, input }).await
    }

    pub async fn cache_read(&self, engine_id: impl Into<String>, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let engine_id = engine_id.into();
        let key = key.into();
        self.call(move |req| Effect::CacheRead { req, engine_id, key }).await
    }
    //#endregion 🔖️Cache

    //#region 🔖️Extensions / Messaging / Respond
    pub async fn invoke_extension(&self, extension_id: impl Into<String>, capability: impl Into<String>, request_json: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let extension_id = extension_id.into();
        let capability = capability.into();
        let request_json = request_json.into();
        self.call(move |req| Effect::InvokeExtension { req, extension_id, capability, request_json }).await
    }

    pub fn send_message(&self, target: semio_framework::kernel::MessageEndpoint, payload: Vec<u8>) {
        self.registry.emit(Effect::SendMessage { target, payload });
    }

    pub fn publish_event(&self, topic: impl Into<String>, payload: Vec<u8>) {
        self.registry.emit(Effect::PublishEvent { topic: topic.into(), payload });
    }

    pub fn subscribe(&self, topic: impl Into<String>) {
        self.registry.emit(Effect::Subscribe { topic: topic.into() });
    }

    pub fn unsubscribe(&self, topic: impl Into<String>) {
        self.registry.emit(Effect::Unsubscribe { topic: topic.into() });
    }

    /// ↩️ Answers an inbound `Event::Request{req, ..}` — must be called within the bounded number
    /// of turns the host allows, or the caller sees a timeout fault.
    pub fn respond(&self, req: RequestId, result: Result<Vec<u8>, Vec<u8>>) {
        let result = match result {
            Ok(bytes) => RequestOutcome::Ok(bytes),
            Err(bytes) => RequestOutcome::Err(bytes),
        };
        self.registry.emit(Effect::Respond { req, result });
    }
    //#endregion 🔖️Extensions

    //#region 🔖️Timers / Jobs
    pub fn set_timer(&self, id: u64, after_ms: u64, repeat: bool) {
        self.registry.emit(Effect::SetTimer { id, after_ms, repeat });
    }

    /// 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): moves a genuinely long-
    /// running computation off this turn's own budget. Allocates its `job` id from the SAME
    /// `RequestRegistry` counter every other `host::*` call uses (`self.call`'s `RequestId`) —
    /// `job == req.0` is the correlation the host's completion, `Event::JobCompleted{job, ..}`,
    /// resolves against (`⚛️reactor/🦀️component.rs`'s `Event::JobCompleted` routing step), so no
    /// separate job/request mapping table is needed on either side of the component boundary. The
    /// host drives `start-job`/`step-job` under a `JobBudget` across as many turns as the job
    /// needs — see `🖥️host/🧵️shard/🦀️component.rs`'s `ShardLoop::pump`, the generic executor that
    /// was previously missing entirely (`📓️terra-M5-report.md` §4(a): "no code anywhere reads a
    /// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ...}` and spawns/drives a job
    /// for it").
    pub async fn spawn_job(&self, kind: impl Into<String>, input: Vec<u8>, placement: JobPlacement) -> Result<Vec<u8>, Fault> {
        let kind = kind.into();
        self.call(move |req| Effect::SpawnJob { job: req.0, kind, input, placement }).await
    }

    /// 🛑️ `job` must be one this SAME instance's own `host::jobs::spawn` call returned — cancelling
    /// an id minted by the `RequestRegistry`'s `req` counter, exactly as `respond`/every other
    /// `req`-carrying effect already assumes about ids it did not itself allocate.
    pub fn cancel_job(&self, job: u64) {
        self.registry.emit(Effect::CancelJob { job });
    }
    //#endregion 🔖️Timers

    //#region 🔖️Ui / Shell
    pub async fn open_window(&self, kind: impl Into<String>, params: DslValue) -> Result<Vec<u8>, Fault> {
        let kind = WindowKindId(kind.into());
        self.call(move |req| Effect::OpenWindow { req, kind, params }).await
    }

    pub fn close_window(&self, window: WindowHandle) {
        self.registry.emit(Effect::CloseWindow { window });
    }

    pub fn notify(&self, message: impl Into<String>) {
        self.registry.emit(Effect::Notify { message: message.into() });
    }

    pub fn clipboard_write(&self, fragment: ClipboardFragment) {
        self.registry.emit(Effect::ClipboardWrite { fragment });
    }

    pub fn navigate(&self, uri: impl Into<String>) {
        self.registry.emit(Effect::Navigate { uri: uri.into() });
    }

    pub fn open_external_url(&self, url: impl Into<String>) {
        self.registry.emit(Effect::OpenExternalUrl { url: url.into() });
    }

    pub fn set_panel(&self, panel_json: impl Into<String>) {
        self.registry.emit(Effect::SetPanel { panel_json: panel_json.into() });
    }

    pub fn set_active_utility(&self, window_id: impl Into<String>, utility_id: impl Into<String>) {
        self.registry.emit(Effect::SetActiveUtility { window_id: window_id.into(), utility_id: utility_id.into() });
    }

    pub fn set_active_tool(&self, tool_id: impl Into<String>) {
        self.registry.emit(Effect::SetActiveTool { tool_id: tool_id.into() });
    }

    pub fn patch_world3d_chrome(&self, selection_json: impl Into<String>, vortices_json: Option<String>, document_selected_ids: Vec<String>, document_highlighted_ids: Option<Vec<String>>) {
        self.registry.emit(Effect::PatchWorld3dChrome { selection_json: selection_json.into(), vortices_json, document_selected_ids, document_highlighted_ids });
    }

    pub fn replay_shell_command(&self, action_id: impl Into<String>, args: Option<DslValue>) {
        self.registry.emit(Effect::ReplayShellCommand { action_id: action_id.into(), args });
    }

    pub fn download_media_export(&self, filename: impl Into<String>, mime_type: impl Into<String>, data: impl Into<String>, encoding: Option<String>) {
        self.registry.emit(Effect::DownloadMediaExport { filename: filename.into(), mime_type: mime_type.into(), data: data.into(), encoding });
    }

    pub fn icon_render_export(&self, items: Vec<IconRenderExportItem>) {
        self.registry.emit(Effect::IconRenderExport { items });
    }

    pub async fn request_file_open(&self, accept: impl Into<String>, read_as: Option<String>, import_action: impl Into<String>, multiple: bool) -> Result<Vec<u8>, Fault> {
        let accept = accept.into();
        let import_action = import_action.into();
        self.call(move |req| Effect::RequestFileOpen { req, accept, read_as, import_action, multiple }).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn request_media_frames(
        &self, accept: impl Into<String>, frame_action: impl Into<String>, done_action: impl Into<String>, fallback_action: impl Into<String>, sample_stride: u32, max_frames: u32, max_long_edge_px: u32, fps_hint: f64, payload: Option<String>, args: Option<DslValue>,
    ) -> Result<Vec<u8>, Fault> {
        let accept = accept.into();
        let frame_action = frame_action.into();
        let done_action = done_action.into();
        let fallback_action = fallback_action.into();
        self.call(move |req| Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args }).await
    }

    pub fn load_document(&self, pack: Vec<u8>, spr: Vec<u8>) {
        self.registry.emit(Effect::LoadDocument { pack, spr });
    }

    pub async fn spawn_plugin_instance(&self, plugin_id: impl Into<String>, app_id: impl Into<String>, os_instance_id: Option<String>, label: Option<String>, document_json: Option<String>) -> Result<Vec<u8>, Fault> {
        let plugin_id = plugin_id.into();
        let app_id = app_id.into();
        self.call(move |req| Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json }).await
    }

    pub fn open_plugin_instance(&self, plugin_id: impl Into<String>, app_id: impl Into<String>, os_instance_id: Option<String>) {
        self.registry.emit(Effect::OpenPluginInstance { plugin_id: plugin_id.into(), app_id: app_id.into(), os_instance_id });
    }

    pub async fn open_dialog(&self, dialog_id: impl Into<String>, args: Option<DslValue>) -> Result<Vec<u8>, Fault> {
        let dialog_id = dialog_id.into();
        self.call(move |req| Effect::OpenDialog { req, dialog_id, args }).await
    }

    pub fn request_sync(&self) {
        self.registry.emit(Effect::RequestSync);
    }
    //#endregion 🔖️Ui

    //#region 🔖️Storage / Capabilities
    pub async fn storage_read(&self, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        self.call(move |req| Effect::StorageRead { req, key }).await
    }

    pub async fn storage_write(&self, key: impl Into<String>, bytes: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        self.call(move |req| Effect::StorageWrite { req, key, bytes }).await
    }

    pub async fn storage_delete(&self, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        self.call(move |req| Effect::StorageDelete { req, key }).await
    }

    pub async fn request_capability(&self, capability: semio_framework::kernel::CapabilityRequest) -> Result<Vec<u8>, Fault> {
        self.call(move |req| Effect::RequestCapability { req, capability }).await
    }

    pub fn release_capability(&self, id: CapabilityId) {
        self.registry.emit(Effect::ReleaseCapability { id });
    }
    //#endregion 🔖️Storage
}

/// 📝️ Synchronous — wraps the `pure` WIT import `log`. Native/test builds (no `component-guest`
/// wasm32-wasip2 target) fall back to `eprintln!`, mirroring `host_port::host_now_ms`'s own
/// fallback shape.
pub fn log(level: &str, message: &str) {
    #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
    {
        crate::component::component::semio::framework::pure::log(level, message);
        return;
    }
    #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
    eprintln!("[{level}] {message}");
}

/// ⏱️ Synchronous — wraps the `pure` WIT import `now-ms`.
pub fn now_ms() -> i64 {
    #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
    {
        return crate::component::component::semio::framework::pure::now_ms();
    }
    #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as i64).unwrap_or(0)
}

/// 📏️ Synchronous — wraps the `pure` WIT import `trace-span`.
pub fn trace_span(name: &str) {
    #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
    {
        crate::component::component::semio::framework::pure::trace_span(name);
        return;
    }
    #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
    let _ = name;
}
