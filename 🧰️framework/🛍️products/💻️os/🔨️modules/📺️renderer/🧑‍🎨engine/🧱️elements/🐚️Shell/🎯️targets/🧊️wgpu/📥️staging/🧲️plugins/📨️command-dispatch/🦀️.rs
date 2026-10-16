//! 🧲️ `shell-plugins` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl PluginState {
    //#region 🔖️DirectoryAndIdentity
    //#region 💡️InferencePort
    #[cfg(not(target_arch = "wasm32"))]
    pub fn installed_document_http_port(&self, owner:&str, service_id:&str) -> Option<CompiledDocumentHttpPortV1> {
        let mut found=None;
        for plugin in &self.plugins {
            if plugin.plugin_id!=owner {continue;}
            for contribution in &plugin.manifest.topic_contributions {
                if contribution.topic!=DOCUMENT_HTTP_PORT_TOPIC {continue;}
                let declaration=DocumentHttpPortDeclarationV1::from_value(contribution.payload.clone()).ok()?;
                if declaration.service_id!=service_id || declaration.owner!=owner {continue;}
                if found.is_some() {return None;}
                found=Some(CompiledDocumentHttpPortV1::compile(owner,declaration).ok()?);
            }
        }
        found
    }

    //#endregion 💡️InferencePort
    //#endregion 🔖️DirectoryAndIdentity
    pub fn project_extension_record(&mut self, record: ShellExtensionStoreRecord) -> Result<usize, String> {
        validate_extension_record(&record)?;
        if let Some(index) = self.extensions.iter().position(|entry| entry.record.extension_id == record.extension_id) {
            self.extensions[index].record = record;
            return Ok(index);
        }
        if self.extensions.len() >= SHELL_EXTENSION_LEDGER_CAPACITY {
            return Err("extension projection capacity exceeded".into());
        }
        self.extensions.push(ShellExtensionProjection { record, enabled: true, load_status: ShellExtensionLoadStatus::Available, program: None });
        Ok(self.extensions.len() - 1)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn load_extension_record(&mut self, record: ShellExtensionStoreRecord) -> Result<(), String> {
        let index = self.project_extension_record(record.clone())?;
        let record_json = serde_json::to_string(&record).map_err(|error| error.to_string())?;
        match semio_framework_os_renderer_wgpu_program_bridge::install_js_extension(&record_json, &record.extension_id, &record.version).await {
            Ok(program) => {
                let enabled = self.extensions[index].enabled;
                self.plugins.retain(|entry| entry.plugin_id != record.extension_id);
                if enabled {
                    self.plugins.push(program.clone());
                }
                self.extensions[index].program = Some(program);
                self.extensions[index].load_status = ShellExtensionLoadStatus::Loaded;
                Ok(())
            }
            Err(error) => {
                self.extensions[index].program = None;
                self.extensions[index].load_status = ShellExtensionLoadStatus::Failed(error.clone());
                Err(error)
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn refresh_extension_store(&mut self) -> Result<(), String> {
        let answer = extension_store_answer(serde_json::json!({ "op": "extension-store-list" })).await?;
        let records: Vec<ShellExtensionStoreRecord> = serde_json::from_value(answer.get("extensions").cloned().ok_or("extension store list missing extensions")?).map_err(|error| error.to_string())?;
        if records.len() > SHELL_EXTENSION_LEDGER_CAPACITY {
            return Err("extension store list exceeds projection capacity".into());
        }
        let retired: Vec<String> = self.extensions.iter().filter(|entry| !records.iter().any(|record| record.extension_id == entry.record.extension_id)).map(|entry| entry.record.extension_id.clone()).collect();
        self.plugins.retain(|entry| !retired.contains(&entry.plugin_id));
        self.extensions.retain(|entry| !retired.contains(&entry.record.extension_id));
        for record in records {
            let _ = self.load_extension_record(record).await;
        }
        Ok(())
    }

    /// 🎯️ Refuses an install this target could never complete, BEFORE any install record is minted —
    /// native needs a resident program to read the `<modules_root>/<plugin_id>/<file>.wasm` root back
    /// off, the browser needs the frame Worker's own module door. A refusal here is not a phase: there
    /// is nothing in flight to report, and chrome must not show a cancellable install that never began.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn plugin_install_precondition(&self) -> Result<(), String> {
        plugin_modules_root_of(&self.plugins).map(|_| ()).ok_or_else(|| "plugin modules root is unavailable".to_string())
    }

    /// 🎯️ Browser twin of the native precondition — see that method's own doc.
    #[cfg(target_arch = "wasm32")]
    pub fn plugin_install_precondition(&self) -> Result<(), String> {
        semio_framework_os_renderer_wgpu_program_bridge::js_plugin_install_door_available().then_some(()).ok_or_else(|| "this isolate installs no semioWgpuInstallPlugin door".to_string())
    }

    /// 📦️ The one step that differs per target: hand back a `ProgramBridgeEntry` for `plugin_id`.
    /// Native re-reads the runtime manifest `load_wasm_plugins` publishes (instantiating nothing —
    /// `from_wasm` compiles at `create_app`); the browser asks the frame Worker's own module loader for
    /// the SAME `pluginHandleForBridge` handle the boot plan's eager mount produces, because the wasm
    /// renderer cannot fetch a module itself.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn acquire_plugin_program(&mut self, plugin_id: &str) -> Result<ProgramBridgeEntry, String> {
        let modules_root = plugin_modules_root_of(&self.plugins).ok_or_else(|| "plugin modules root is unavailable".to_string())?;
        let plugin_filter = self.plugin_filter.clone();
        let entries = semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins(&plugin_filter, &modules_root).await?;
        entries.into_iter().find(|entry| entry.plugin_id == plugin_id).ok_or_else(|| format!("native runtime manifest declares no module for {plugin_id}"))
    }

    /// 📦️ Browser twin of the native acquisition — see that method's own doc.
    #[cfg(target_arch = "wasm32")]
    pub async fn acquire_plugin_program(&mut self, plugin_id: &str) -> Result<ProgramBridgeEntry, String> {
        semio_framework_os_renderer_wgpu_program_bridge::install_js_plugin(plugin_id).await
    }

    /// 🎬️ Makes `plugin_id` resident, the wgpu twin of React's `installPlugin`
    /// (`🏛️ShellHost/🟦️.tsx`), on BOTH targets. A hub session lists every registered plugin in its
    /// session but instantiates none of them eagerly on native — `ProgramBridgeEntry::from_wasm`
    /// compiles nothing until `create_app` — so a native miss means the manifest this shell booted
    /// from predates the module, which one re-read fixes. In the browser the boot plan DOES eager-mount
    /// the variant's plugins, so a miss means the owner was outside that plan or its mount faulted and
    /// was skipped (`mountPluginHandles`'s per-plugin isolation); either way the frame Worker can still
    /// load one more module on demand. Already-resident is the common case and costs one scan.
    ///
    /// ⏱️ Progress and cancellation ride [`ShellState::plugin_install`]: the phase advances
    /// `Resolving` → `Loading` → cleared, and every step boundary checks the retained `CancelToken`, so
    /// [`Self::cancel_plugin_install`] stops the install before it commits an entry rather than after.
    /// One install at a time — a second request for the SAME plugin while one runs is refused rather
    /// than racing a second acquisition against the first one's commit.
    pub async fn install_plugin(&mut self, plugin_id: &str) -> Result<(), String> {
        if self.plugins.iter().any(|entry| entry.plugin_id == plugin_id) {
            return Ok(());
        }
        if self.plugin_install.is_some() {
            return Err(format!("plugin install already in flight, {plugin_id} not admitted"));
        }
        self.plugin_install_precondition()?;
        let cancel = CancelToken::root_now();
        self.plugin_install = Some(ShellPluginInstall { plugin_id: plugin_id.to_string(), phase: ShellPluginInstallPhase::Resolving, cancel: cancel.clone() });
        if let Some(install) = self.plugin_install.as_mut() {
            install.phase = ShellPluginInstallPhase::Loading;
        }
        let loaded = self.acquire_plugin_program(plugin_id).await;
        let settled = match loaded {
            _ if cancel.is_cancelled_now() => Err(ShellPluginInstallPhase::Cancelled),
            Err(error) => Err(ShellPluginInstallPhase::Failed(error)),
            Ok(entry) => Ok(entry),
        };
        match settled {
            Ok(entry) => {
                if !self.plugins.iter().any(|resident| resident.plugin_id == plugin_id) {
                    self.plugins.push(entry);
                }
                self.plugin_install = None;
                Ok(())
            }
            Err(phase) => {
                let message = match &phase {
                    ShellPluginInstallPhase::Cancelled => format!("plugin install cancelled: {plugin_id}"),
                    ShellPluginInstallPhase::Failed(error) => error.clone(),
                    _ => format!("plugin install refused: {plugin_id}"),
                };
                if let Some(install) = self.plugin_install.as_mut() {
                    install.phase = phase;
                }
                debug_log(&format!("[TRACE] wgpu shell plugin install failed: {message}"));
                Err(message)
            }
        }
    }

    /// 🛑️ Cancels the retained lazy install. The phase does NOT move here: only the install's own next
    /// step boundary may report `Cancelled`, so an install that already committed is never misreported
    /// as having been stopped. Clearing a settled `Failed`/`Cancelled` record is the same call. In the
    /// browser the frame Worker's own request is withdrawn too, so a cancel during a long module fetch
    /// settles the awaited promise now instead of at the end of the fetch.
    pub fn cancel_plugin_install(&mut self) {
        let settled = matches!(self.plugin_install.as_ref().map(|install| &install.phase), Some(ShellPluginInstallPhase::Failed(_)) | Some(ShellPluginInstallPhase::Cancelled));
        if settled {
            self.plugin_install = None;
            return;
        }
        if let Some(install) = self.plugin_install.as_ref() {
            install.cancel.cancel_now();
            #[cfg(target_arch = "wasm32")]
            semio_framework_os_renderer_wgpu_program_bridge::cancel_js_plugin_install(&install.plugin_id);
        }
    }

    /// 🎬️ The plugin that opens `dialect`, from the catalog's declared `on-artifact-kind:` rows — the
    /// wgpu twin of React's `artifactKindActivationOwner` lookup, and the only resolution that works
    /// before the owner's manifest is resident. Returns the owner and its app for `role` once that
    /// owner is installed, so the relay below can hand both straight to [`Self::switch_to_app`].
    /// Both targets: the table is generated into this crate and the install is [`Self::install_plugin`],
    /// which O2 made target-neutral.
    pub async fn resolve_activation_owner_app(&mut self, dialect: &semio_framework_artifact_reference::ArtifactDialect, role: semio_framework::manifest::AppRole) -> Option<(String, AppDefinition)> {
        let owner = semio_framework_os_renderer_wgpu_program_bridge::resolve_artifact_kind_activation_owner(&dialect.artifact_kind, semio_framework_os_renderer_wgpu_program_bridge::plugin_artifact_kind_activations())?;
        if let Err(error) = self.install_plugin(owner).await {
            debug_log(&format!("[TRACE] wgpu shell os.open-artifact could not install {owner}: {error}"));
            return None;
        }
        let program = self.plugins.iter().find(|entry| entry.plugin_id == owner)?;
        let app = find_dialect_app(program, dialect, role).or_else(|| find_dialect_app(program, dialect, semio_framework::manifest::AppRole::Editor))?;
        Some((owner.to_string(), app.clone()))
    }
}

//#endregion ShellActions

#[cfg(target_arch = "wasm32")]
pub async fn extension_store_answer(request: serde_json::Value) -> Result<serde_json::Value, String> {
    let answer = host_io_call(&request.to_string(), None).await?;
    let decoded: serde_json::Value = serde_json::from_str(&answer).map_err(|error| format!("extension store answer: {error}"))?;
    if let Some(error) = decoded.get("error").and_then(|value| value.as_str()) {
        return Err(error.to_string());
    }
    Ok(decoded)
}

pub fn validate_extension_record(record: &ShellExtensionStoreRecord) -> Result<(), String> {
    if record.extension_id.is_empty() || record.directory_name.is_empty() || record.version.is_empty() || record.extends_host.is_empty() || record.module_url.is_empty() || record.package_hash.is_empty() {
        return Err("extension store returned an incomplete record".into());
    }
    Ok(())
}
