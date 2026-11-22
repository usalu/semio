//! 🌉️ Program bridge entry: the program identity and ephemeral snapshot, the bridge backend and entry, the fixture document, the `ProgramBridgeEntry` methods and the window, tool and engagement section helpers. See [`super`].

use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "🎛️handle-doors.rs"]
mod handle_doors;

/// 🪪️ The package a browser program mounts, as the JS bridge's `packageIdentity` reads it off the
/// served package descriptor the module was admitted with.
#[cfg(target_arch = "wasm32")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProgramPackageIdentityV1 {
    pub(super) package_id: String,
    component_sha256: String,
}

/// 👥️ One guest instance's last published ephemeral state (`AppFrame::Ephemeral`), decoded for the presence
/// wire: the app-owned presence pack (absent while the guest never published one), its declared-broadcast
/// selection and hover, and its tool run summary.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProgramEphemeralSnapshot {
    pub presence: Option<Vec<u8>>,
    pub interaction: Option<protocol::PresenceInteraction>,
    pub tool_run: Option<protocol::PresenceToolRun>,
    pub history_edit: Option<protocol::PresenceHistoryEdit>,
}

enum ProgramBridgeBackend {
    #[cfg(target_arch = "wasm32")]
    Js(Rc<JsValue>),
    /// 🎠️ H3-wgpu-native — replaces `Arc<WasmPluginRuntime>`. `KernelClient` is a cheap channel
    /// handle to the dedicated kernel thread (`crate::kernel_runtime`); `wasm_path` is carried here
    /// (not resolved through the client) because instantiation is now lazy — `create_app` is the
    /// first moment the kernel thread actually reads+compiles the component, per item 3 of this
    /// packet ("no eager loading").
    #[cfg(not(target_arch = "wasm32"))]
    Wasm { client: KernelClient, wasm_path: std::path::PathBuf },
}

impl Clone for ProgramBridgeBackend {
    fn clone(&self) -> Self {
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Js(handle) => Self::Js(handle.clone()),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Wasm { client, wasm_path } => Self::Wasm { client: client.clone(), wasm_path: wasm_path.clone() },
        }
    }
}

#[derive(Clone)]
pub struct ProgramBridgeEntry {
    pub plugin_id: String,
    pub package_id: Option<String>,
    /// 🪪️ The SHA-256 of the component this entry mounts, as its verified package descriptor names it —
    /// what a hub-selected execution-target lease must name for this entry to carry its document.
    pub component_sha256: Option<String>,
    pub manifest: PluginManifest,
    #[cfg(not(target_arch = "wasm32"))]
    native_consumes: Vec<String>,
    backend: ProgramBridgeBackend,
    #[cfg(not(target_arch="wasm32"))]
    native_observed_mtime:Option<std::time::SystemTime>,
    #[cfg(any(test, feature = "testkit"))]
    fixture_render: Option<fn(u32, &str, &str, &ViewModel, Option<&str>, Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String>>,
    #[cfg(any(test, feature = "testkit"))]
    fixture_action: Option<fn(u32, &str, &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String>>,
    #[cfg(any(test, feature = "testkit"))]
    fixture_publications: Option<fn(u32) -> Vec<OperationPublication>>,
    #[cfg(any(test, feature = "testkit"))]
    fixture_document: Option<ProgramFixtureDocument>,
}

/// 🧪️ A law's stand-in for a program's document doors: loading an archive and answering the history snapshot.
#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Copy)]
pub struct ProgramFixtureDocument {
    pub load: fn(u32, &protocol::DocumentArchivePack) -> Result<(), String>,
    pub archive: fn(u32) -> Result<protocol::DocumentArchivePack, String>,
    pub history: fn(u32) -> Result<semio_framework::kernel::HistoryPatch, String>,
}

impl ProgramBridgeEntry {
    #[cfg(target_arch = "wasm32")]
    pub fn from_js(plugin_id: String, handle: JsValue) -> Result<Self, String> {
        let manifest_fn = Reflect::get(&handle, &JsValue::from_str("manifest")).map_err(|_| "missing manifest")?;
        let manifest_fn: Function = manifest_fn.dyn_into().map_err(|_| "manifest not fn")?;
        let manifest_json = manifest_fn.call0(&JsValue::NULL).map_err(|_| "manifest call failed")?.as_string().ok_or("manifest not string")?;
        let manifest: PluginManifest = serde_json::from_str(&manifest_json).map_err(|err| format!("manifest parse: {err}"))?;
        let _create_app = get_fn(&handle, "createApp")?;
        let identity_json = get_fn(&handle, "packageIdentity")?.call0(&JsValue::NULL).map_err(|_| "packageIdentity call failed")?.as_string().ok_or("packageIdentity not string")?;
        let identity: ProgramPackageIdentityV1 = serde_json::from_str(&identity_json).map_err(|err| format!("packageIdentity parse: {err}"))?;
        Ok(Self {
            plugin_id,
            package_id: Some(identity.package_id),
            component_sha256: Some(identity.component_sha256),
            manifest,
            backend: ProgramBridgeBackend::Js(Rc::new(handle)),
            #[cfg(any(test, feature = "testkit"))]
            fixture_render: None,
            #[cfg(any(test, feature = "testkit"))]
            fixture_action: None,
            #[cfg(any(test, feature = "testkit"))]
            fixture_publications: None,
            #[cfg(any(test, feature = "testkit"))]
            fixture_document: None,
        })
    }

    /// 🎠️ H3-wgpu-native — no longer instantiates anything (see `load_wasm_plugins` below, item 3
    /// "no eager loading"): `manifest` is whatever `load_wasm_plugins` could establish without
    /// running the component (a build-time `PackageDescriptor` when one exists, an honest empty
    /// placeholder otherwise — no plugin has migrated to emit one yet, E1-describe is a sibling
    /// packet still in flight). The kernel thread only ever sees `wasm_path` once `create_app` is
    /// actually called.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_wasm(plugin_id: String, package_id: Option<String>, component_sha256: Option<String>, wasm_path: std::path::PathBuf, manifest: PluginManifest) -> Result<Self, String> {
        Ok(Self {
            plugin_id: plugin_id.clone(),
            package_id,
            component_sha256,
            manifest,
            backend: ProgramBridgeBackend::Wasm { client: KernelClient::get(), wasm_path },
            native_consumes: Vec::new(),
            native_observed_mtime:None,
            #[cfg(any(test, feature = "testkit"))]
            fixture_render: None,
            #[cfg(any(test, feature = "testkit"))]
            fixture_action: None,
            #[cfg(any(test, feature = "testkit"))]
            fixture_publications: None,
            #[cfg(any(test, feature = "testkit"))]
            fixture_document: None,
        })
    }

    /// 🧾️ Returns the installed runtime registry's consumed topics for this native program.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn native_consumes(&self) -> &[String] {
        &self.native_consumes
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn with_native_consumes(mut self, consumes: Vec<String>) -> Self {
        self.native_consumes = consumes;
        self
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_fixture_render(&mut self, render: fn(u32, &str, &str, &ViewModel, Option<&str>, Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String>) {
        self.fixture_render = Some(render);
    }

    /// 🕰️ The observation belongs to this exact program path and uses only inline scalar state.
    #[cfg(not(target_arch="wasm32"))]
    pub fn observe_native_modification(&mut self,path:&std::path::Path,time:std::time::SystemTime)->bool{let ProgramBridgeBackend::Wasm{wasm_path,..}=&self.backend;if wasm_path.as_path()!=path{return false}let changed=self.native_observed_mtime.is_some_and(|previous|previous!=time);self.native_observed_mtime=Some(time);changed}

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_fixture_action(&mut self, action: fn(u32, &str, &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String>) {
        self.fixture_action = Some(action);
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_fixture_document(&mut self, document: ProgramFixtureDocument) {
        self.fixture_document = Some(document);
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_fixture_publications(&mut self, publications: fn(u32) -> Vec<OperationPublication>) {
        self.fixture_publications = Some(publications);
    }

    /// 🎠️ H3-wgpu-native — replaces the old `Arc<WasmPluginRuntime>`-returning `wasm_runtime()`.
    /// `register_host_backbone`/`deregister_host_backbone` (its only real callers,
    /// `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) had no in-process guest handle to call anyway once the guest moved to
    /// a separate kernel thread — that mechanism is the process-global `HostBackboneChannel`
    /// `📓️design-abi.md` §4 replaces with a per-instance `EffectBackbone`, flagged as an
    /// unimplemented critical-path gap in `📓️status.md`'s "A2-abi-sdk — honest partial" entry
    /// ("Registrar decision needed before W2 — critical path for both renderer packets"). Not this
    /// packet's to invent; callers get an honest `None`/gap message instead of a dangling type.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn wasm_artifact_path(&self) -> Option<&std::path::Path> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { wasm_path, .. } => Some(wasm_path.as_path()),
            #[cfg(target_arch = "wasm32")]
            _ => None,
        }
    }

    /// 🧬️ The document schema `app_id` opens: its own declared schema, or — for a viewer, which declares
    /// none — the schema its package declares for the same dialect, so a spectator's mount makes the
    /// component the kind's codec exactly as an author's does. Empty for an app that opens no document.
    fn app_document_schema(&self, app_id: &str) -> &str {
        let Some(app) = self.manifest.apps.iter().find(|app| app.id == app_id) else { return "" };
        if !app.io.artifact_schema.is_empty() {
            return &app.io.artifact_schema;
        }
        self.manifest.apps.iter().find(|other| other.dialect == app.dialect && !other.io.artifact_schema.is_empty()).map(|other| other.io.artifact_schema.as_str()).unwrap_or("")
    }

    /// 🐣️ Opens one instance of `app_id`. A refusal keeps the structured fault behind it when the host or the browser bridge
    /// raised one — the host's guest admission (`plugin.channel-mismatch`) — so the shell tells it as its localized notice.
    pub async fn create_app(&self, app_id: &str) -> Result<u32, ProgramFault> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let instance_id = create_app_js(handle, app_id).await?;
                let schema = self.app_document_schema(app_id);
                if !schema.is_empty() {
                    browser_component_codec::register(handle.clone(), schema.to_owned())?;
                }
                Ok(instance_id)
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, wasm_path } => client.create_app(wasm_path, &self.plugin_id, app_id, self.app_document_schema(app_id)).await.map_err(ProgramFault::from),
        }
    }

    pub fn destroy_app(&self, instance_id: u32) {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                browser_ephemeral::forget(instance_id);
                destroy_app_js(handle, instance_id);
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => {
                wasm_program_exchange::forget_ephemeral_snapshot(instance_id);
                native_handle_doors::forget_document_pack(instance_id);
                client.destroy_app(instance_id);
            }
        }
    }

    pub async fn handle_action(&self, instance_id: u32, action_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
        #[cfg(any(test, feature = "testkit"))]
        if let Some(action) = self.fixture_action {
            return action(instance_id, action_json, view_state).map_err(ProgramFault::from);
        }
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let result = handle_action_js(handle, instance_id, action_json, view_state).await;
                browser_ephemeral::observe(handle, instance_id).await;
                result
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::handle_action(client, instance_id, action_json, view_state).await,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn dispatch_invoke_extension(&self, instance_id: u32, extension_id: &str, capability: &str, request_json: &str, req: u64) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
        match &self.backend {
            ProgramBridgeBackend::Js(handle) => dispatch_invoke_extension_js(handle, instance_id, extension_id, capability, request_json, req).await.map_err(ProgramFault::from),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn dispatch_invoke_extension(&self, instance_id: u32, extension_id: &str, capability: &str, request_json: &str, req: u64) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
        extension_invoke::dispatch_native(self, instance_id, extension_id, capability, request_json, req).await
    }

    /// 📨️ Sends one inbound `Event::Request` (from the shell instance `origin`) to this extension's live `instance` and reads the actor's `Effect::Respond` for it out of the same exchange; a refused or failed exchange is `Failed`.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) async fn answer_extension_request(&self, instance: u32, origin: u32, req: semio_framework::kernel::RequestId, capability: &str, payload: Vec<u8>) -> extension_invoke::ExtensionAnswer {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        wasm_program_exchange::answer_extension_request(client, instance, origin, req, capability, payload).await
    }

    /// ✅️ Completes the requester's parked request once: delivers `Event::Completed` to `instance` and answers the effects that turn raised.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) async fn complete_extension_request(&self, instance: u32, req: u64, result: semio_framework::kernel::RequestOutcome) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        wasm_program_exchange::deliver_extension_completion(client, instance, req, result).await
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn push_scoped_contributions(&self, instance_id: u32, app_id: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
        match &self.backend {
            ProgramBridgeBackend::Js(handle) => push_scoped_contributions_js(handle, instance_id, app_id, view_state_json).await.map_err(ProgramFault::from),
        }
    }

    pub async fn handle_command(&self, instance_id: u32, command_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let result = handle_command_js(handle, instance_id, command_json, view_state).await;
                browser_ephemeral::observe(handle, instance_id).await;
                result
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::handle_command(client, instance_id, command_json, view_state).await,
        }
    }

    /// 🖱️ On-demand context menu rows for the given surface hit and selection snapshot, asked of the guest through `AppCommand::ContextMenu`.
    pub async fn context_menu(&self, instance_id: u32, request: serde_json::Value) -> Result<Vec<semio_ui_declaration::ui::ContextMenuItemSpec>, ProgramFault> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => context_menu_js(handle, instance_id, &request).await.map_err(ProgramFault::from),
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => native_handle_doors::context_menu(client, instance_id, request).await,
        }
    }

    /// 🗃️ Loads one plain document `(pack, spr)` — an archive without members — through the stepped, ACK-owned archive load
    /// (admit, poll to a terminal state, acknowledge; `📓️api-stepped-document-load.md` §2 of ticket
    /// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), the one whole-document load: a long history folds across turns and a cancel
    /// leaves the previous document.
    pub async fn load_app_document_pack(&self, instance_id: u32, pack: &[u8], spr: &[u8]) -> Result<(), ProgramFault> {
        let loaded = self.load_app_document_archive(instance_id, &protocol::DocumentArchivePack { parent_pack: pack.to_vec(), parent_spr: spr.to_vec(), members: Vec::new() }).await;
        #[cfg(not(target_arch = "wasm32"))]
        {
            if loaded.is_ok() {
                native_handle_doors::remember_document_pack(instance_id, pack, spr);
            }
        }
        loaded
    }

    /// 🪪️ Scalar document ownership query shared by native and browser bridges.
    pub async fn read_app_document_identity(&self, instance_id: u32) -> Result<protocol::AppDocumentIdentity, ProgramFault> {
        let identity = match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::read_app_document_identity(client, instance_id).await?,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let args = Array::new();
                args.push(&JsValue::from_f64(f64::from(instance_id)));
                let result = call_js(handle, "readAppDocumentIdentity", &args).await?;
                let json = result.as_string().ok_or_else(|| "document identity reply is not a JSON string".to_string())?;
                if json.len() > 4_096 {
                    return Err("document identity reply byte budget exceeded".into());
                }
                serde_json::from_str::<protocol::AppDocumentIdentity>(&json).map_err(|error| error.to_string())?
            }
        };
        if identity.app_instance_id != instance_id || identity.parent_document_id.as_ref().is_some_and(|id| !(1..=512).contains(&id.chars().count())) {
            return Err("document identity reply authority mismatch".into());
        }
        Ok(identity)
    }

    /// 📋 Resolves the checked export from this program's own mounted app declaration.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn declared_native_media_save(&self, app_id: &str, action_id: &str) -> Option<NativeMediaDeclaration> {
        let mut candidates = self.manifest.apps.iter().filter(|app| app.id == app_id);
        let app = candidates.next()?;
        if candidates.next().is_some() { return None; }
        NativeMediaDeclaration::for_app(app, action_id)
    }

    /// 💾 Streams one source-checked binary export to the selected native destination.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn save_native_media(
        &self, owner: &protocol::AppDocumentIdentity, declaration: &NativeMediaDeclaration, destination: &std::path::Path,
        cancelled: impl Fn() -> bool, progress: impl FnMut(u64),
    ) -> Result<NativeSaveOutcome, String> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        native_save::save_media(client, owner, declaration, destination, cancelled, progress).await
    }

    /// 📏 The largest natural file the format's import command admits, as the browser bridge reports it.
    #[cfg(target_arch = "wasm32")]
    pub async fn natural_file_maximum_bytes(&self, format_kind: &str) -> Result<usize, String> {
        let ProgramBridgeBackend::Js(handle) = &self.backend;
        let answer = call_js(handle, "naturalFileMaximumBytes", &Array::of1(&JsValue::from_str(format_kind))).await?;
        answer.as_f64().filter(|bytes| bytes.is_finite() && *bytes >= 0.0).map(|bytes| bytes as usize).ok_or_else(|| "naturalFileMaximumBytes returned no byte count".to_string())
    }

    /// 📥 Imports one picked natural file into a fresh instance through the browser bridge's typed media door.
    #[cfg(target_arch = "wasm32")]
    pub async fn import_natural_file(&self, instance_id: u32, format_kind: &str, bytes: &[u8]) -> Result<(), String> {
        let ProgramBridgeBackend::Js(handle) = &self.backend;
        let args = Array::new();
        args.push(&JsValue::from_f64(f64::from(instance_id)));
        args.push(&JsValue::from_str(format_kind));
        args.push(&js_sys::Uint8Array::from(bytes));
        call_js(handle, "importNaturalFile", &args).await.map(|_| ())
    }

    /// 📥 Imports natural binary input through the mounted program's typed media door.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn import_native_file(
        &self, instance: u32, declaration: &NativeMediaDeclaration, bytes: Vec<u8>,
    ) -> Result<(), String> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        native_save::import_media(client, instance, declaration, bytes).await
    }

    /// 📂 Launches one fresh instance for the selected extent and streams its source after issuance.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn import_native_pdf_source(
        &self, app_id: &str, declaration: &NativeMediaDeclaration, source: std::fs::File,
        extent: u32, cancel: &semio_framework_async::CancelToken, progress: impl FnMut(u64, u64),
        mut retain_instance: impl FnMut(u32),
    ) -> Result<Option<semio_framework::kernel::NaturalSourceWireAck>, String> {
        use semio_framework::kernel::{ActorInstanceOpenPurpose, NaturalSourcePrepare};
        let declaration = declaration.natural_pdf_source().ok_or("Open File requires the mounted PDF declaration")?;
        let ProgramBridgeBackend::Wasm { client, wasm_path } = &self.backend;
        if cancel.is_cancelled_now() { return Ok(None); }
        let sequence = semio_renderer_wgpu_kernel_protocol::next_seq()?;
        let instance = client.create_app_with_purpose(
            wasm_path, &self.plugin_id, app_id, self.app_document_schema(app_id),
            ActorInstanceOpenPurpose::NaturalSource { declaration: declaration.clone(), extent },
        ).await.map_err(|error| error.to_string())?;
        retain_instance(instance);
        let prepare = NaturalSourcePrepare { instance_id: instance, seq: sequence, declaration, extent };
        super::native_natural_source::stream(prepare, source, |event, closing| async move {
            let request = client.exchange_original_media(instance, event);
            let outcome = if closing {
                request.await
            } else {
                match semio_framework_async::select2(cancel.cancelled(), request).await {
                    semio_framework_async::Either::Left(()) => return Err("natural-source.cancelled".to_owned()),
                    semio_framework_async::Either::Right(outcome) => outcome,
                }
            }.map_err(|error| error.to_string())?;
            wasm_program_exchange::observe_ephemeral(instance, &outcome.frames).await;
            Ok(outcome.command_ingress)
        }, || !cancel.is_cancelled_now(), progress).await
    }

    /// 📤 Delivers one selected source through the same active component and exact picker loan.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn deliver_original_media<'a, F: std::future::Future<Output = ()> + Send + 'a>(
        &'a self, request: &'a super::NativeOriginalMediaRequest,
        source: Result<Option<(impl std::io::Read + Send + 'a, String, &'static str, u64)>, String>,
        observe: impl Fn(Vec<Effect>) -> F + Clone + Send + 'a,
        current: impl Fn() -> bool + Send + Sync + 'a, progress: impl FnMut(u64, u64) + Send + 'a,
    ) -> impl std::future::Future<Output = Result<bool, String>> + Send + 'a {
        async move {
            let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
            let sequence = semio_renderer_wgpu_kernel_protocol::next_seq()?;
            let instance = request.instance_id();
            super::native_original_media::NativeOriginalMediaTransfer::run(
                request, sequence, source, move |event| {
                    let client = client.clone();
                    let observe = observe.clone();
                    async move {
                        let outcome = client.exchange_original_media(instance, event).await.map_err(|error| error.to_string())?;
                        wasm_program_exchange::observe_ephemeral(instance, &outcome.frames).await;
                        observe(outcome.effects).await;
                        Ok(outcome.command_ingress)
                    }
                }, current, progress,
            ).await
        }
    }

    pub async fn read_app_document_archive(&self, instance_id: u32) -> Result<protocol::DocumentArchivePack, ProgramFault> {
        #[cfg(any(test, feature = "testkit"))]
        if let Some(document) = self.fixture_document {
            return (document.archive)(instance_id).map_err(ProgramFault::from);
        }
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::read_app_document_archive(client, instance_id).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let args = Array::new();
                args.push(&JsValue::from_f64(f64::from(instance_id)));
                let result = call_js(handle, "readAppDocumentArchive", &args).await?;
                let bytes = result.dyn_into::<js_sys::Uint8Array>().map_err(|_| "readAppDocumentArchive returned no byte archive".to_string())?.to_vec();
                protocol::decode_document_archive_bytes(&bytes).await.map_err(|error| ProgramFault::from(error.to_string()))
            }
        }
    }

    /// 📬 Sends one scoped folder record request on the native bounded event queue.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn exchange_folder_archive(
        &self, request: protocol::channel::archive_ingress::folder_archive_load::FolderArchiveLoadRequest<'_>,
    ) -> Result<protocol::channel::archive_ingress::folder_archive_load::FolderArchiveLoadReply, String> {
        use protocol::channel::archive_ingress::folder_archive_load::FolderArchiveLoadReply as Reply;
        use semio_framework::kernel::{Event, FolderArchiveLoadEvent, MessageEndpoint, PluginInstanceId};
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        let instance = request.key.instance.to_string();
        let payload = request.encode().ok_or("folder archive request exceeded its wire authority")?;
        let event = Event::FolderArchiveLoad(FolderArchiveLoadEvent { instance: request.key.instance, request: payload });
        let outcome = client.exchange_original_media(request.key.instance, event).await.map_err(|error| error.to_string())?;
        let mut answer = None;
        for effect in outcome.effects {
            if let Effect::SendMessage { target: MessageEndpoint::Shell { instance: target }, payload } = effect {
                if !Reply::is_wire(&payload) { continue; }
                if target.0 != instance || answer.is_some() { return Err("folder archive reply changed its Shell instance".into()); }
                answer = Some(Reply::decode(request, &payload).ok_or("folder archive reply changed its operation")?);
            }
        }
        Ok(answer.unwrap_or(Reply::Pending))
    }

    /// 🧾 Uses the ordinary archive status and acknowledgment path after a verified page transfer.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn finish_paged_folder_archive(&self, instance: u32, operation: u64) -> Result<(), ProgramFault> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        wasm_program_exchange::drive_app_document_archive(client, instance, protocol::DocumentArchiveLoadHost::admitted(operation)).await
    }

    pub async fn load_app_document_archive(&self, instance_id: u32, archive: &protocol::DocumentArchivePack) -> Result<(), ProgramFault> {
        #[cfg(any(test, feature = "testkit"))]
        if let Some(document) = self.fixture_document {
            return (document.load)(instance_id, archive).map_err(ProgramFault::from);
        }
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::load_app_document_archive(client, instance_id, archive).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let bytes = protocol::encode_document_archive_bytes(archive).map_err(|error| error.to_string())?;
                call_js_bytes(handle, "loadAppDocumentArchive", instance_id, &[bytes.as_slice()]).await.map_err(ProgramFault::from)
            }
        }
    }

    pub async fn render(&self, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel) -> Result<UiDocumentLease, ProgramFault> {
        self.render_with_document(instance_id, surface_id, body_key, view_state, None, None).await
    }

    pub async fn render_with_document(&self, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, ProgramFault> {
        let _latency = crate::frame_latency::FrameLatencyTimer::start(crate::frame_latency::latest_frame_authority(), crate::frame_latency::FrameLatencyStage::RetainedExchange, 1);
        #[cfg(any(test, feature = "testkit"))]
        if let Some(render) = self.fixture_render {
            return render(instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects).map_err(ProgramFault::from);
        }
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => render_with_document_js(handle, instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects).await.map_err(ProgramFault::from),
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => {
                let mut effects = Vec::new();
                let lease = wasm_program_exchange::render_with_document(client, instance_id, surface_id, body_key, view_state, document_dsl, Some(&mut effects)).await?;
                let lease = extension_splice::splice_native_extensions(lease, instance_id, view_state, Some(&mut effects)).await?;
                if let Some(sink) = refresh_effects {
                    sink.append(&mut effects);
                }
                Ok(lease)
            }
        }
    }

    /// 🧩️ Activates this extension program in the kernel beside the live instance `parent_instance` of plugin `parent_plugin` — no parent re-creation — and answers the extension's own instance id.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) async fn activate_extension_instance(&self, parent_instance: u32, parent_plugin: &str, capabilities: &[String]) -> Result<u32, ProgramFault> {
        let ProgramBridgeBackend::Wasm { client, wasm_path } = &self.backend;
        let activation = semio_renderer_wgpu_kernel_protocol::ExtensionActivation { parent_instance, parent_plugin: parent_plugin.to_string(), extension: semio_renderer_wgpu_kernel_protocol::ExtensionRecord::installed(&self.plugin_id, capabilities) };
        client.activate_extension(wasm_path, activation).await
    }

    /// 🗑️ Begins retiring this extension's kernel instance (the ordinary app close).
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn begin_retire_extension_instance(&self, instance: u32) -> Result<semio_renderer_wgpu_kernel_protocol::KernelCloseHandle, String> {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        client.begin_destroy_app(instance)
    }

    /// 🧩️ This program's own (non-splicing) render as the records and root id an extension host splices into its parent, with the effects the render raised; `None` when the render fails or publishes nothing.
    pub(super) async fn render_extension_records(&self, instance_id: u32, extension: &str, view_state: &ViewModel) -> Option<(Vec<ui_contract::UiNodeRecord>, u64, Vec<Effect>)> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let (document, effects) = render_browser_document(handle, instance_id, extension, extension, view_state).await.ok()?;
                Some((document.nodes, document.root?, effects))
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => {
                let mut effects = Vec::new();
                let lease = wasm_program_exchange::render_with_document(client, instance_id, extension, extension, view_state, None, Some(&mut effects)).await.ok()?;
                let (header, nodes) = extension_splice::lease_records(&lease)?;
                Some((nodes, header.root.0, effects))
            }
        }
    }

    /// 🎬️ Publishes the instance's reserved `framework.section.engagements` surface through the
    /// canonical retained render route shared by native and browser guests. The caller owns the
    /// returned lease and must retire it after [`window_engagements_from_section`] reads it.
    pub async fn window_engagements_section(&self, instance_id: u32, view_state: &ViewModel) -> Result<UiDocumentLease, ProgramFault> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        self.render_with_document(instance_id, body_key, body_key, view_state, None, None).await
    }

    /// 📏️ Publishes the instance's reserved `framework.section.measures` surface — the ONE retained wire
    /// home window measures have on both backends: the guest's `plugin_render_section` carries
    /// `window_measures(view_state)` as canonical JSON in a paged-text carrier, exactly like the
    /// app-static catalogue. The caller owns the returned lease and retires it through its registry
    /// after [`window_measures_from_section`] has read it.
    pub async fn window_measures_section(&self, instance_id: u32, view_state: &ViewModel) -> Result<UiDocumentLease, ProgramFault> {
        let body_key = semio_framework::UiRefreshSection::Measures.body_key();
        self.render_with_document(instance_id, body_key, body_key, view_state, None, None).await
    }

    /// 🛠️ Publishes the instance's reserved `framework.section.tools` surface — the tool twin of
    /// {@link ProgramBridgeEntry::window_measures_section}, carrying `ArtifactApp::tool_measures`
    /// keyed by TOOL id (`🔌️plugin/🦀️.rs`'s `UiRefreshSection::Tools` arm). This is the reader React's
    /// `toolMeasuresByToolId` ref has had all along and this renderer had not, which is why
    /// `build_tool_panel_ui` could only render the tool's armed state. The caller owns the returned
    /// lease and retires it through its registry after {@link tool_measures_from_section} read it.
    pub async fn tool_measures_section(&self, instance_id: u32, view_state: &ViewModel) -> Result<UiDocumentLease, ProgramFault> {
        let body_key = semio_framework::UiRefreshSection::Tools.body_key();
        self.render_with_document(instance_id, body_key, body_key, view_state, None, None).await
    }

    /// ⚔️ This artifact's currently open conflicts — native-only for the same reason `read_history`
    /// is (the JS backend has no `exchange` door). React's twin is the `AppCommand::ReadConflicts`
    /// seed `🏛️ShellHost/🟦️.tsx` fires on session start/switch.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn read_conflicts(&self, instance_id: u32) -> Result<Vec<protocol::Conflict>, ProgramFault> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::read_conflicts(client, instance_id).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("read_conflicts unavailable".into()),
        }
    }

    /// ⚔️ Accept or discard one open conflict, answering the roster that survives it — React's
    /// `dispatchResolveConflict`.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn resolve_conflict(&self, instance_id: u32, conflict_id: &str, accept: bool) -> Result<Vec<protocol::Conflict>, ProgramFault> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::resolve_conflict(client, instance_id, conflict_id, accept).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("resolve_conflict unavailable".into()),
        }
    }

    pub async fn bind_document_backbone(&self, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, ProgramFault> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::bind_document_backbone(client, instance_id, binding_generation, uri).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => document_backbone_js(handle, instance_id, "bind", binding_generation, uri).await.map_err(ProgramFault::from),
        }
    }

    pub async fn retire_document_backbone(&self, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, ProgramFault> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::retire_document_backbone(client, instance_id, binding_generation, uri).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => document_backbone_js(handle, instance_id, "retire", binding_generation, uri).await.map_err(ProgramFault::from),
        }
    }

    pub async fn receive_document_backbone(&self, instance_id: u32, uri: &str, payload: Vec<u8>) -> Result<Vec<Effect>, ProgramFault> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::receive_document_backbone(client, instance_id, uri, payload).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => receive_document_backbone_js(handle, instance_id, uri, &payload).await.map_err(ProgramFault::from),
        }
    }

    /// 👥️ The ephemeral state this instance's guest last published on a command turn — what the presence
    /// heartbeat carries as the peer's app presence pack, interaction and tool run (React's
    /// `ephemeralSnapshot`). `None` until the guest answered its first command.
    pub fn ephemeral_snapshot(&self, instance_id: u32) -> Option<ProgramEphemeralSnapshot> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::ephemeral_snapshot(instance_id),
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(_) => browser_ephemeral::snapshot(instance_id),
        }
    }

    /// 🏁️ What this instance's guest published between two host calls since the last take — typed-operation completions
    /// and UI-progress frames, React's `subscribeOperationCompletions` and `subscribeOperationProgress` lanes. The
    /// browser takes them off the JS bridge's queue (`takeOperationPublications`); natively every such frame is folded
    /// into the next exchange's reply (`invocation_from_frames`), so there is nothing queued to take.
    pub fn take_operation_publications(&self, instance_id: u32) -> Vec<OperationPublication> {
        #[cfg(any(test, feature = "testkit"))]
        if let Some(publications) = self.fixture_publications {
            return publications(instance_id);
        }
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { .. } => {
                let _ = instance_id;
                Vec::new()
            }
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => take_operation_publications_js(handle, instance_id),
        }
    }

    pub async fn apply_mutations(&self, instance_id: u32, operations: &[u8]) -> Result<(), ProgramFault> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::apply_mutations(client, instance_id, operations).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => call_js_bytes(handle, "applyMutations", instance_id, &[operations]).await.map_err(ProgramFault::from),
        }
    }

    /// 👥️ Native twin of the browser host's `AppChannelClient.pushPresence` (contract-freeze §C7.6).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn push_presence(&self, instance_id: u32, own_color: Option<u8>, peers: &[protocol::PresencePeer]) -> Result<(), ProgramFault> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::push_presence(client, instance_id, own_color, peers).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("push_presence unavailable".into()),
        }
    }

    /// 🧾️ ticket §C5 — full history snapshot for an instance: the native exchange's `ReadHistory`
    /// (`wasm_program_exchange::read_history`), and on the browser build the JS bridge's `readHistory`, the same
    /// command through the guest's `AppChannelClient` — what a restored document archive re-reads its rows and head from.
    pub async fn read_history(&self, instance_id: u32) -> Result<semio_framework::kernel::HistoryPatch, ProgramFault> {
        #[cfg(any(test, feature = "testkit"))]
        if let Some(document) = self.fixture_document {
            return (document.history)(instance_id).map_err(ProgramFault::from);
        }
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::read_history(client, instance_id).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let args = Array::new();
                args.push(&JsValue::from_f64(f64::from(instance_id)));
                let text = call_js(handle, "readHistory", &args).await?.as_string().ok_or_else(|| "readHistory answered no JSON".to_string())?;
                semio_framework_pack_json::from_json_str::<semio_framework::kernel::HistoryPatch>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ProgramFault::from(error.to_string()))
            }
        }
    }
}

/// 🛠️ Reads a published [`ProgramBridgeEntry::tool_measures_section`] document back into the
/// per-TOOL measure trees the guest authored — same paged-text carrier and same canonical JSON as
/// {@link window_measures_from_section}, keyed by tool id instead of window instance id.
pub fn tool_measures_from_section(document: &UiDocumentLease) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
    let payload = document.read_paged_text().map_err(|error| format!("tool measures section unreadable: {error:?}"))?;
    serde_json::from_str(&payload).map_err(|error| format!("tool measures section parse: {error}"))
}

/// 📏️ Reads a published [`ProgramBridgeEntry::window_measures_section`] document back into the
/// per-window-instance measure trees the guest authored.
///
/// See `🧑‍🎨engine/🧫️fixtures/📏️window-measures/🔣️.json`.
pub fn window_measures_from_section(document: &UiDocumentLease) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
    let payload = document.read_paged_text().map_err(|error| format!("window measures section unreadable: {error:?}"))?;
    serde_json::from_str(&payload).map_err(|error| format!("window measures section parse: {error}"))
}

/// 🎬️ Reads the canonical per-window engagement map from its reserved retained section.
///
/// See `🧑‍🎨engine/🧫️fixtures/🎬️window-engagements/🔣️.json`.
pub fn window_engagements_from_section(document: &UiDocumentLease) -> Result<HashMap<String, WindowEngagement>, String> {
    let payload = document.read_paged_text().map_err(|error| format!("window engagements section unreadable: {error:?}"))?;
    serde_json::from_str(&payload).map_err(|error| format!("window engagements section parse: {error}"))
}
