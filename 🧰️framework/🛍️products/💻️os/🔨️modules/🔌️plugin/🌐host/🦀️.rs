//! 🌐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2/sdk-async, design-abi.md §4): the async host-
//! capability API, replacing `host_port`/`component::host_*`. `log`/`now_ms`/`trace_span` stay
//! synchronous (they wrap the `pure` WIT import, the world's only import for BOTH worlds); every
//! other `Host` method is a match over `HostBackend`:
//!
//! - `HostBackend::Poll` (`world actor`): builds a `semio_framework::kernel::Effect`, hands it to a
//!   `🌐host/📮️requests::RequestRegistry`, and awaits the matching `Event::Completed`/`Event::HttpChunk`/
//!   `Event::JobCompleted` on a LATER `reactor::poll` call.
//!
//! `Host` is constructed per app instance (never a process-global singleton — see
//! `important.md`'s "Replace, never wrap" list: `set_host_backbone_channel` is explicitly one of
//! the things a pooled multi-instance actor cannot keep). Cloning is cheap (`Poll` wraps an `Rc`).
extern crate semio_framework_os_kernel as store;

use requests::RequestRegistry;
use semio_framework_value::DslValue;
use semio_framework::kernel::{CapabilityId, ClipboardFragment, Effect, IconRenderExportItem, JobPlacement, RequestId, RequestOutcome, WindowHandle, WindowKindId};
use semio_framework::{Fault, FaultCode, FaultOrigin, MediaType};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "📖️body/🦀️.rs"]
pub mod body;
#[path = "📮️requests/🦀️.rs"]
pub mod requests;
pub use body::BodyReader;
#[path = "../⚛️reactor/🧵️executor/🦀️.rs"]
pub mod executor;
#[path = "../📡️backbone/🔗️binding/🦀️.rs"]
pub mod document_backbone_binding;
#[path = "🛬️lifted/💼️jobs/🦀️.rs"]
mod lifted_jobs;
pub use lifted_jobs::*;

/// 📦️ Encodes a fault as the ABI `pack` `completion-result.fault` and every `host-async` Err arm carry.
/// Schema: `🔌️plugin/🧬️schema/📜️.wit` `type pack = list<u8>` — no JSON string on this data path.
#[doc(hidden)]
pub fn encode_fault_pack(fault: &Fault) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(fault))
}

/// 📦️ Inverse of [`encode_fault_pack`] — the guest decode of a host-written fault pack.
fn decode_fault_pack(bytes: &[u8]) -> Fault {
    match store::pack_rt::decode_wire_value(bytes) {
        Ok(value) => semio_framework_value::FromValue::from_value(value).unwrap_or_else(|_| Fault::new(FaultOrigin::Os, FaultCode::new("os.fault.decode"), String::from_utf8_lossy(bytes).into_owned())),
        Err(_) => Fault::new(FaultOrigin::Os, FaultCode::new("os.fault.decode"), String::from_utf8_lossy(bytes).into_owned()),
    }
}

/// 📦️ Decodes a `RequestOutcome` into the `Result<Vec<u8>, Fault>` every `host::*` async call
/// resolves to. `Err` bytes are a `pack` of the fault value — the same shape the shell writes with
/// `encodePackValue(fault)` and the native host writes with [`encode_fault_pack`].
// 🚫️async: E1 pure decode consumed by `⚛️reactor/🦀️.rs`'s sync `world actor` boundary — R9.
#[doc(hidden)]
pub fn outcome_to_result(outcome: RequestOutcome) -> Result<Vec<u8>, Fault> {
    match outcome {
        RequestOutcome::Ok(bytes) => Ok(bytes),
        RequestOutcome::Err(bytes) => Err(decode_fault_pack(&bytes)),
    }
}

/// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): which world this `Host` handle talks
/// to — see module doc.
#[derive(Clone)]
enum HostBackend {
    Poll(RequestRegistry),
}

impl Default for HostBackend {
    fn default() -> Self {
        HostBackend::Poll(RequestRegistry::default())
    }
}

/// 🌐️ Per-instance async host-capability handle — see module doc.
#[derive(Clone, Default)]
pub struct Host {
    backend: HostBackend,
}

/// 🌊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the envelope `http_fetch` resolves to —
/// `body` arrives already fully reassembled by `🌐host/📮️requests::RequestRegistry::append_chunk`
/// (see `Host::http_fetch`'s own doc).
pub struct HttpFetchResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: BodyReader,
}

/// 🩹️ The poll world's `Event::Completed` payload for `Effect::HttpRequest` — JSON-encoded by
/// `🖥️host/⚡️effects/🦀️.rs`'s `encode_http_response`/`HttpResponseWire` (that struct is
/// private to the host crate; this is the guest-side decode counterpart, field-for-field the same
/// shape, never imported directly since they live in different crates on either side of the wasm
/// boundary).
#[derive(serde::Serialize, ToValue, serde::Deserialize, FromValue)]
struct HttpResponseWire {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Host {
    pub fn new(registry: RequestRegistry) -> Self {
        Self { backend: HostBackend::Poll(registry) }
    }

    /// 📮️ The request registry this handle correlates completions through.
    pub async fn registry(&self) -> &RequestRegistry {
        match &self.backend {
            HostBackend::Poll(registry) => registry,
        }
    }

    /// 🔥️ Fire-and-forget dispatch shared by every non-completable method below — queues `effect`
    /// exactly as `RequestRegistry::emit` always did.
    async fn emit(&self, effect: Effect) {
        match &self.backend {
            HostBackend::Poll(registry) => registry.emit(effect),
        }
    }

    //#region 🔖️Blobs
    pub async fn blob_load(&self, hash: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let hash = hash.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::BlobLoad { req, hash }).await,
        }
    }

    pub async fn blob_write(&self, media_type: MediaType, bytes: Vec<u8>) -> Result<Vec<u8>, Fault> {
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::BlobWrite { req, media_type, bytes }).await,
        }
    }

    /// 🌊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the STREAMING counterpart to
    /// `blob_load` — no chunked blob backend exists anywhere in this codebase yet (host report,
    /// `terra-async-imports`: "`blob-read` calls the same buffered path and hands back an already-
    /// `done`, one-item `ChunkStreamProducer`"), so the Poll arm below reuses the identical
    /// `Effect::BlobLoad` round-trip `blob_load` uses and wraps the WHOLE result as a single-chunk
    /// `BodyReader` — genuinely honest, not a stand-in (real chunked blob delivery needs a new host
    /// service neither this packet nor the host's own owns).
    pub async fn blob_read(&self, hash: impl Into<String>) -> Result<BodyReader, Fault> {
        let hash = hash.into();
        match &self.backend {
            HostBackend::Poll(registry) => match registry.request(move |req| Effect::BlobLoad { req, hash }).await {
                Ok(bytes) => Ok(BodyReader::poll_buffered(bytes).await),
                Err(fault) => Err(fault),
            },
        }
    }
    //#endregion 🔖️Blobs

    //#region 🔖️Http
    #[allow(clippy::too_many_arguments)]
    pub async fn http_request(&self, method: impl Into<String>, url: impl Into<String>, headers: Vec<(String, String)>, body: Option<Vec<u8>>, stream: bool) -> Result<Vec<u8>, Fault> {
        let method = method.into();
        let url = url.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::HttpRequest { req, method, url, headers, body, stream }).await,
        }
    }

    /// 🌊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the STREAMING counterpart to
    /// `http_request` — `host-async`'s `http-fetch` renames the poll world's `http-request` and
    /// resolves to a `stream<u8>` body (design-abi.md §4's fix for `Event::HttpChunk`'s discarded-
    /// chunk bug, see `🌐host/📮️requests::RequestRegistry::append_chunk`). Poll arm reuses the
    /// SAME `Effect::HttpRequest` round-trip `http_request` uses (`append_chunk` has already fully
    /// reassembled the body by the time this resolves — a poll-world completion is one shot, there
    /// is nothing left to stream), decodes the JSON `HttpResponseWire` envelope
    /// `🖥️host/⚡️effects/🦀️.rs::encode_http_response` writes, and hands back the WHOLE body
    /// as one chunk.
    pub async fn http_fetch(&self, method: impl Into<String>, url: impl Into<String>, headers: Vec<(String, String)>, body: Option<Vec<u8>>) -> Result<HttpFetchResponse, Fault> {
        let method = method.into();
        let url = url.into();
        match &self.backend {
            HostBackend::Poll(registry) => {
                let bytes = registry.request(move |req| Effect::HttpRequest { req, method, url, headers, body, stream: true }).await?;
                let bytes_text = std::str::from_utf8(&bytes).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.host.http-decode-error"), format!("could not decode the http-request completion envelope: {error}")))?;
                let wire: HttpResponseWire = semio_framework_pack_json::from_json_str(bytes_text, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.host.http-decode-error"), format!("could not decode the http-request completion envelope: {error}")))?;
                Ok(HttpFetchResponse { status: wire.status, headers: wire.headers, body: BodyReader::poll_buffered(wire.body).await })
            }
        }
    }
    //#endregion 🔖️Http

    //#region 🔖️Documents
    pub async fn artifact_read(&self, doc: semio_framework::kernel::ArtifactHandle, lane: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let lane = lane.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::DocumentRead { req, doc, lane }).await,
        }
    }

    pub async fn artifact_write(&self, doc: semio_framework::kernel::ArtifactHandle, lane: impl Into<String>, ops: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let lane = lane.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::DocumentWrite { req, doc, lane, ops }).await,
        }
    }
    //#endregion 🔖️Documents

    //#region 🔖️Links
    /// 🧷️ Captures a linked artifact reply for a retained Send job through the same actor registry.
    pub fn resolve_link_retained(&self, link: String, wake: std::task::Waker) -> Result<requests::SharedRequestReceiver, Fault> {
        match &self.backend {
            HostBackend::Poll(registry) => registry.request_retained(wake, move |req| Effect::LinkResolve { req, link }),
        }
    }

    pub async fn resolve_link(&self, link: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let link = link.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::LinkResolve { req, link }).await,
        }
    }
    //#endregion 🔖️Links

    //#region 🔖️Io — CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM absorption
    /// 🌉️ Absorbs the old host import `io-routes`: `kind = "io-routes"`, `filter` is the
    /// `{source, target}` query — see `effects.wit`'s `registry-query`.
    pub async fn io_routes(&self, source: &str, target: &str) -> Result<Vec<u8>, Fault> {
        #[derive(serde::Serialize, ToValue)]
        struct IoRoutesFilter<'a> {
            source: &'a str,
            target: &'a str,
        }
        let filter = Some(semio_framework_value::ToValue::to_value(&IoRoutesFilter { source, target }));
        self.registry_query("io-routes", filter).await
    }

    /// 🌉️ Absorbs the old host import `io-identify`: `kind = "io-identify"`.
    pub async fn io_identify(&self, payload: &[u8]) -> Result<Vec<u8>, Fault> {
        #[derive(serde::Serialize, ToValue)]
        struct IoIdentifyFilter {
            payload: Vec<u8>,
        }
        let filter = Some(semio_framework_value::ToValue::to_value(&IoIdentifyFilter { payload: payload.to_vec() }));
        self.registry_query("io-identify", filter).await
    }

    /// 🌉️ Absorbs the old host import `io-run` (multi-hop, cross-plugin) — distinct from the
    /// `semio.io-run` COLD JOB kind, which is the single-hop, THIS-plugin-only registry lookup
    /// (see `⚛️reactor/💼️jobs`).
    ///
    /// 🚧️ The Poll arm below is UNCHANGED (byte-for-byte, per this packet's own mandate) and stays
    /// a documented APPROXIMATION: `semio_framework::kernel::Effect` has no `IoRun` variant yet
    /// (the same A3 gap `🦀️.rs`'s own `wit_effect_to_kernel` already documents, and the
    /// host report's own honest gaps confirm), so it synthesizes a one-hop `IoCompose` instead of
    /// a real multi-hop route.
    pub async fn io_run(&self, source: impl Into<String>, target: impl Into<String>, payload: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let source = source.into();
        let target = target.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::IoCompose { req, key: format!("{source}->{target}"), sources: vec![String::from_utf8_lossy(&payload).into_owned()] }).await,
        }
    }

    /// 🌉️ One-hop compose, unchanged semantics from the old `io-compose` host import.
    pub async fn io_compose(&self, key: impl Into<String>, sources: Vec<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::IoCompose { req, key, sources }).await,
        }
    }
    //#endregion 🔖️Io

    //#region 🔖️Registry
    pub async fn registry_query(&self, kind: impl Into<String>, filter: Option<DslValue>) -> Result<Vec<u8>, Fault> {
        let kind = kind.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RegistryQuery { req, kind, filter }).await,
        }
    }
    //#endregion 🔖️Registry

    //#region 🔖️Cache
    pub async fn cache_derive(&self, engine_id: impl Into<String>, input: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let engine_id = engine_id.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::CacheDerive { req, engine_id, input }).await,
        }
    }

    pub async fn cache_read(&self, engine_id: impl Into<String>, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let engine_id = engine_id.into();
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::CacheRead { req, engine_id, key }).await,
        }
    }
    //#endregion 🔖️Cache

    //#region 🔖️Extensions / Messaging / Respond
    pub async fn invoke_extension(&self, extension_id: impl Into<String>, capability: impl Into<String>, request_json: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let extension_id = extension_id.into();
        let capability = capability.into();
        let request_json = request_json.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::invoke_extension(req, extension_id, capability, request_json)).await,
        }
    }

    pub async fn send_message(&self, target: semio_framework::kernel::MessageEndpoint, payload: Vec<u8>) {
        self.emit(Effect::SendMessage { target, payload }).await;
    }

    pub async fn publish_event(&self, topic: impl Into<String>, payload: Vec<u8>) {
        self.emit(Effect::PublishEvent { topic: topic.into(), payload }).await;
    }

    pub async fn subscribe(&self, topic: impl Into<String>) {
        self.emit(Effect::Subscribe { topic: topic.into() }).await;
    }

    pub async fn unsubscribe(&self, topic: impl Into<String>) {
        self.emit(Effect::Unsubscribe { topic: topic.into() }).await;
    }

    /// ↩️ Answers an inbound `Event::Request{req, ..}` — must be called within the bounded number
    /// of turns the host allows, or the caller sees a timeout fault.
    pub async fn respond(&self, req: RequestId, result: Result<Vec<u8>, Vec<u8>>) {
        let result = match result {
            Ok(bytes) => RequestOutcome::Ok(bytes),
            Err(bytes) => RequestOutcome::Err(bytes),
        };
        self.emit(Effect::Respond { req, result }).await;
    }
    //#endregion 🔖️Extensions

    //#region 🔖️Timers / Jobs
    pub async fn set_timer(&self, id: u64, after_ms: u64, repeat: bool) {
        self.emit(Effect::SetTimer { id, after_ms, repeat }).await;
    }

    /// 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): moves a genuinely long-
    /// running computation off this turn's own budget. Poll arm allocates its `job` id from the
    /// SAME `RequestRegistry` counter every other `host::*` call uses (`self.call`'s `RequestId`) —
    /// `job == req.0` is the correlation the host's completion, `Event::JobCompleted{job, ..}`,
    /// resolves against (`⚛️reactor/🦀️.rs`'s `Event::JobCompleted` routing step), so no
    /// separate job/request mapping table is needed on either side of the component boundary. The
    /// host drives `start-job`/`step-job` under a `JobBudget` across as many turns as the job
    /// needs — see `🖥️host/🧵️shard/🦀️.rs`'s `ShardLoop::pump`.
    pub async fn spawn_job(&self, kind: impl Into<String>, input: Vec<u8>, placement: JobPlacement) -> Result<Vec<u8>, Fault> {
        let kind = kind.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::SpawnJob { job: req.0, kind, input, placement }).await,
        }
    }

    /// 🛑️ `job` must be one this SAME instance's own `host::jobs::spawn` call returned — cancelling
    /// an id minted by the `RequestRegistry`'s `req` counter, exactly as `respond`/every other
    /// `req`-carrying effect already assumes about ids it did not itself allocate.
    pub async fn cancel_job(&self, job: u64) {
        self.emit(Effect::CancelJob { job }).await;
    }
    //#endregion 🔖️Timers

    //#region 🔖️Ui / Shell
    pub async fn open_window(&self, kind: impl Into<String>, params: DslValue) -> Result<Vec<u8>, Fault> {
        let kind = WindowKindId(kind.into());
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::OpenWindow { req, kind, params }).await,
        }
    }

    pub async fn close_window(&self, window: WindowHandle) {
        self.emit(Effect::CloseWindow { window }).await;
    }

    pub async fn notify(&self, message: impl Into<String>) {
        self.emit(Effect::Notify { message: message.into() }).await;
    }

    pub async fn clipboard_write(&self, fragment: ClipboardFragment) {
        self.emit(Effect::ClipboardWrite { fragment }).await;
    }

    pub async fn navigate(&self, uri: impl Into<String>) {
        self.emit(Effect::Navigate { uri: uri.into() }).await;
    }

    pub async fn open_external_url(&self, url: impl Into<String>) {
        self.emit(Effect::OpenExternalUrl { url: url.into() }).await;
    }

    pub async fn set_panel(&self, panel_json: impl Into<String>) {
        self.emit(Effect::SetPanel { panel_json: panel_json.into() }).await;
    }

    pub async fn set_active_utility(&self, window_id: impl Into<String>, utility_id: impl Into<String>) {
        self.emit(Effect::SetActiveUtility { window_id: window_id.into(), utility_id: utility_id.into() }).await;
    }

    pub async fn set_active_tool(&self, tool_id: impl Into<String>) {
        self.emit(Effect::SetActiveTool { tool_id: tool_id.into() }).await;
    }

    pub async fn replay_shell_command(&self, action_id: impl Into<String>, args: Option<DslValue>) {
        self.emit(Effect::ReplayShellCommand { action_id: action_id.into(), args }).await;
    }

    pub async fn download_media_export(&self, filename: impl Into<String>, mime_type: impl Into<String>, data: impl Into<String>, encoding: Option<String>) {
        self.emit(Effect::DownloadMediaExport { filename: filename.into(), mime_type: mime_type.into(), data: data.into(), encoding }).await;
    }

    pub async fn icon_render_export(&self, items: Vec<IconRenderExportItem>) {
        self.emit(Effect::IconRenderExport { items }).await;
    }

    pub async fn request_file_open(&self, accept: impl Into<String>, read_as: Option<String>, import_action: impl Into<String>, multiple: bool) -> Result<Vec<u8>, Fault> {
        let accept = accept.into();
        let import_action = import_action.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RequestFileOpen { req, accept, read_as, import_action, multiple, args: None }).await,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn request_media_frames(
        &self,
        accept: impl Into<String>,
        frame_action: impl Into<String>,
        done_action: impl Into<String>,
        fallback_action: impl Into<String>,
        sample_stride: u32,
        max_frames: u32,
        max_long_edge_px: u32,
        fps_hint: f64,
        payload: Option<String>,
        args: Option<DslValue>,
    ) -> Result<Vec<u8>, Fault> {
        let accept = accept.into();
        let frame_action = frame_action.into();
        let done_action = done_action.into();
        let fallback_action = fallback_action.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args }).await,
        }
    }

    pub async fn load_document(&self, pack: Vec<u8>, spr: Vec<u8>) {
        self.emit(Effect::LoadDocument { pack, spr }).await;
    }

    pub async fn spawn_plugin_instance(&self, plugin_id: impl Into<String>, app_id: impl Into<String>, os_instance_id: Option<String>, label: Option<String>, document_json: Option<String>) -> Result<Vec<u8>, Fault> {
        let plugin_id = plugin_id.into();
        let app_id = app_id.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json }).await,
        }
    }

    pub async fn open_plugin_instance(&self, plugin_id: impl Into<String>, app_id: impl Into<String>, os_instance_id: Option<String>) {
        self.emit(Effect::OpenPluginInstance { plugin_id: plugin_id.into(), app_id: app_id.into(), os_instance_id }).await;
    }

    /// 🕹️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): NEW — `effects.wit`'s `dispatch-
    /// action-effect`/`host-async`'s `dispatch-action` import both existed before this packet, but
    /// no `Host` method ever built the effect (every existing caller of `Effect::DispatchAction`
    /// is host-side, replaying a shell action — see `⚛️reactor/🦀️.rs`'s
    /// `kernel_effect_to_wit`). Added so the guest SDK can dispatch one too, matching the 24-import
    /// table this packet's report carries.
    pub async fn dispatch_action(&self, action: impl Into<String>, args: Option<DslValue>, delay_ms: u64) -> Result<Vec<u8>, Fault> {
        let action = action.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::DispatchAction { req, action, args, delay_ms }).await,
        }
    }

    pub async fn open_dialog(&self, dialog_id: impl Into<String>, args: Option<DslValue>) -> Result<Vec<u8>, Fault> {
        let dialog_id = dialog_id.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::OpenDialog { req, dialog_id, args }).await,
        }
    }

    pub async fn request_sync(&self) {
        self.emit(Effect::RequestSync).await;
    }
    //#endregion 🔖️Ui

    //#region 🔖️Storage / Capabilities
    pub async fn storage_read(&self, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::StorageRead { req, key }).await,
        }
    }

    pub async fn storage_write(&self, key: impl Into<String>, bytes: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::StorageWrite { req, key, bytes }).await,
        }
    }

    pub async fn storage_delete(&self, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::StorageDelete { req, key }).await,
        }
    }

    pub async fn request_capability(&self, capability: semio_framework::kernel::CapabilityRequest) -> Result<Vec<u8>, Fault> {
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RequestCapability { req, capability }).await,
        }
    }

    pub async fn release_capability(&self, id: CapabilityId) {
        self.emit(Effect::ReleaseCapability { id }).await;
    }
    //#endregion 🔖️Storage
}

/// 📝️ Synchronous — wraps the `pure` WIT import `log`. Native/test builds (no wasm32-wasip2
/// target) fall back to `eprintln!`, mirroring `host_port::host_now_ms`'s own
/// fallback shape.
pub async fn log(level: &str, message: &str) {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    {
        semio_framework_plugin_abi::wasip2::semio::framework::pure::log(level, message);
    }
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    eprintln!("[{level}] {message}");
}

/// ⏱️ Synchronous — wraps the `pure` WIT import `now-ms`.
pub async fn now_ms() -> i64 {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    {
        semio_framework_plugin_abi::wasip2::semio::framework::pure::now_ms()
    }
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as i64)
}

/// 📏️ Synchronous — wraps the `pure` WIT import `trace-span`.
pub async fn trace_span(name: &str) {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    {
        semio_framework_plugin_abi::wasip2::semio::framework::pure::trace_span(name);
    }
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    let _ = name;
}
