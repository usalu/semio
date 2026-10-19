//! 📨️ Dispatch and app switch: the command and session command dispatch, the mutation and op application, the managed app and session role switch, the extension install, enable and uninstall and the shell uri. See [`super`].

use super::*;

//#region ShellActions
impl<'ctx> ActionsCtx<'ctx> {
    /// 🎛️ Sends one owner-qualified non-OS command through the program command boundary.
    ///
    /// 🪟️ The LIVE view, exactly as `dispatch_action` and React's `makeEffectDispatchOne`
    /// (`resolveViewState(baseSession)`) read it: the session's stored `view_state` carries no
    /// window-instance roster, so a window-addressed command — every `flowEvalTick` the guest
    /// re-arms — was refused with `target window is absent from the exact ViewModel window
    /// instance roster` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🧾️ ticket §C5 — same fold `dispatch_action` performs; a command-boundary edit (e.g. a
    /// plugin-owned command dispatched from the command palette) is just as real an uncommitted
    /// edit as an action-boundary one.
    ///
    /// 🧾️ ONE host-effect funnel for both dispatch paths. This arm used to be a SECOND,
    /// shorter hand-rolled fold, and every effect it did not name was dropped without a trace —
    /// including `InvokeExtension`, so the `flowEvalTick` reply that finally asked for an
    /// extension call went nowhere and the preview chain stopped one hop before its first
    /// evaluate (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). React has the same single funnel:
    /// `makeEffectDispatchOne` hands BOTH `handleAction` and `handleCommand` results to
    /// `applyHostEffects`. Only the two effects `queue_host_effects` cannot own — a `Navigate`
    /// that must also RESOLVE the uri, and the replay relay — stay here, because both are
    /// `async` and that funnel is deliberately a plain `fn`.
    pub fn dispatch_command<'a>(&'a mut self, invocation: semio_framework::manifest::CommandInvocation) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            if matches!(invocation.address.owner, semio_framework::manifest::CommandOwnerAddress::Os) {
                return Err("os commands must be dispatched by the shell".into());
            }
            let window_id = invocation.arguments.get("windowId").and_then(semio_framework_value::DslValue::as_str);
            let session = self.docs.command_session(&invocation.address.owner, window_id, &self.plugins).cloned().ok_or("command has no mounted app owner")?;
            self.dispatch_session_command(invocation, session).await
        })
    }

    /// 🧵️ A retained command keeps the exact session selected by its source document.
    pub fn dispatch_session_command<'a>(&'a mut self, invocation: semio_framework::manifest::CommandInvocation, session: ActiveSession) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            let owner_plugin_id = match &invocation.address.owner {
                semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id } | semio_framework::manifest::CommandOwnerAddress::App { plugin_id, .. } | semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id, .. } => plugin_id,
                semio_framework::manifest::CommandOwnerAddress::Os => return Err("os commands must be dispatched by the shell".into()),
            };
            let program = self.plugins.plugins.iter().find(|entry| entry.plugin_id == *owner_plugin_id).cloned().ok_or("command program missing")?;
            let command_json = semio_framework_pack_json::to_json_string(&invocation);
            let live_view_state = self.as_session().live_view_state(&session);
            let answer = program.handle_command(session.instance_id, &command_json, &live_view_state).await;
            let mut result = answer.map_err(|refusal| self.prefs.refused_guest_call(refusal, &session.app, &mut self.chrome))?;
            debug_log(&format!("[TRACE] wgpu-shell command {} settled effects={} mutations={}", invocation.address.command_id, result.requested_effects.len(), result.mutations.len()));
            self.observe_invocation_history(result.history_patch.as_ref()).await;
            self.input.observe_interaction_output(&result.output);
            let mut queued = Vec::with_capacity(result.requested_effects.len());
            for effect in core::mem::take(&mut result.requested_effects) {
                match effect {
                    semio_framework::kernel::Effect::Navigate { uri } => {
                        self.opening.push_uri(uri.clone());
                        self.apply_shell_uri(&uri).await?;
                    }
                    semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {
                        self.handle_replay_shell_command(&action_id, args.as_ref()).await;
                    }
                    other => queued.push(other),
                }
            }
            let scope = result.ui_scope.clone().merged_with(self.docs.host_effect_earned_scope(&queued));
            debug_log(&format!("[TRACE] wgpu-shell dispatch command={} scope={}", invocation.address.command_id, refresh_scope_label(&scope)));
            self.as_window_actions().queue_host_effects(&session.app.controller_id.clone(), queued);
            let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
            self.apply_ops_inner(&operations, true, scope, Some(session)).await
        })
    }

}

impl<'ctx> ActionsCtx<'ctx> {
    /// 🐢️ `scope` is what the dispatch that produced these mutations declared it dirtied, unioned
    /// with what applying its own effects dirtied — [`UiDirtyScope::Full`] for every host-owned
    /// caller that speaks for no guest.
    pub fn apply_mutations<'a>(&'a mut self, operations: &'a [String], scope: UiDirtyScope) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            self.apply_ops_inner(operations, true, scope, self.docs.session.clone()).await
        })
    }

    /// 🔗️ Document sync flows through the `framework/sync` `ArtifactHost` actor + the program
    /// store's `ChannelBackbone` (see `attach_sync_backbone`), never through a shell mutation.
    ///
    /// 🪟️ `setPanel` names no body of its own and `panel_json` feeds EVERY section, so
    /// the widest scope is the only honest one — the same widening React's
    /// `hostEffectRefreshScopeV1` performs for the `setPanel`/`loadDocument` effects.
    ///
    /// 🗑️ The `downloadMediaExport`/`requestFileOpen` MUTATION arms lived here — two CRUD-era
    /// document operations no producer in the repo emits (`grep -rn '"requestFileOpen"'`
    /// answered this shell and nothing else). They were the only reader this renderer had
    /// for either journey, so `Export Document…` and `Import Document…` looked wired and
    /// reached nobody; both are now real `Effect` arms of the ONE host-effect funnel
    /// (`queue_host_effects`), which every dispatch path feeds
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn apply_ops_inner(&mut self, operations: &[String], allow_navigate: bool, scope: UiDirtyScope, owner: Option<ActiveSession>) -> Result<(), String> {
        let mut pending: Vec<String> = operations.to_vec();
        let mut view_state = owner.as_ref().map(|s| s.view_state.clone());
        let mut panel_rewritten = false;
        let mut navigate_uri: Option<String> = None;
        while !pending.is_empty() {
            let batch = std::mem::take(&mut pending);
            for operation_json in batch {
                let operation: Value = serde_json::from_str(&operation_json).unwrap_or(Value::Null);
                if operation.get("operation").and_then(|v| v.as_str()) == Some("setPanel") {
                    panel_rewritten = true;
                    if let Some(panel) = operation.get("panel") {
                        if let Some(mut vs) = view_state.take() {
                            vs.panel_json = Some(panel.to_string());
                            view_state = Some(vs);
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("requestFileSave") {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let (Some(filename), Some(data), Some(space_id)) = (operation.get("filename").and_then(|v| v.as_str()), operation.get("data").and_then(|v| v.as_str()), operation.get("spaceId").and_then(|v| v.as_str())) {
                        if let Some(session) = owner.clone() {
                            let (filename, data, space_id) = (filename.to_string(), data.to_string(), space_id.to_string());
                            self.settle.submit_shell_io_future(async move {
                                let Some(path) = request_file_save(&filename).await else { return ShellIoCompletion::Finished };
                                use std::fs as system_fs;
                                if system_fs::write(&path, data.as_bytes()).is_err() {
                                    return ShellIoCompletion::Finished;
                                }
                                ShellIoCompletion::Actions(vec![ActionDescriptor {
                                    controller_id: session.app.controller_id,
                                    action: "bindSpaceFile".into(),
                                    args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "spaceId": space_id, "filePath": path.display().to_string() }),
                                }])
                            });
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("requestFolderPick") {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(import_action) = operation.get("importAction").and_then(|v| v.as_str()) {
                        if let Some(session) = owner.clone() {
                            let import_action = import_action.to_string();
                            let mut args = operation.get("args").cloned().unwrap_or_else(|| serde_json::json!({}));
                            self.settle.submit_shell_io_future(async move {
                                let Some(folder_path) = pick_folder().await else { return ShellIoCompletion::Finished };
                                if let Some(obj) = args.as_object_mut() {
                                    obj.insert("folderPath".into(), serde_json::json!(folder_path));
                                }
                                ShellIoCompletion::Actions(vec![ActionDescriptor { controller_id: session.app.controller_id, action: import_action, args: Some(semio_framework::DslValue::from(args)) }])
                            });
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("spawnProgram") {
                    if let (Some(plugin_id), Some(view_state)) = (operation.get("pluginId").and_then(|v| v.as_str()), self.docs.session.as_ref().map(|session| session.view_state.clone())) {
                        self.as_session().spawn_plugin(plugin_id, view_state).await?;
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("navigate") {
                    if let Some(uri) = operation.get("uri").and_then(|v| v.as_str()) {
                        navigate_uri = Some(uri.to_string());
                    }
                }
            }
        }
        if allow_navigate {
            if let Some(uri) = navigate_uri.take() {
                self.opening.push_uri(uri.clone());
                self.apply_shell_uri(&uri).await?;
                return Ok(());
            }
        }
        let scope = if panel_rewritten { UiDirtyScope::Full } else { scope };
        if let (Some(owner), Some(vs)) = (owner.as_ref(), view_state) {
            if let Some(session) = self.docs.session.as_mut().filter(|session| same_session(session, owner)) {
                session.view_state = vs;
                self.as_session().sync_session_chrome();
            } else if let Some(session) = self.docs.spawned_session.as_mut().filter(|session| same_session(session, owner)) {
                session.view_state = vs;
            } else {
                return Err("mutation owner is no longer mounted".into());
            }
            self.as_window_actions().refresh_ui(scope).await?;
        } else {
            self.settle.owe_refresh(scope);
        }
        Ok(())
    }

    /// 🏠️🧳️ Generic replacement for the old `switch_to_s_app` — switches to either the host plugin's
    /// landing or host app by id (both resolved via `host_config()`, never a specific app's identity).
    ///
    /// 🎬️ The host plugin is resident on every healthy boot, so this is a one-scan no-op then. It is
    /// not decoration: `mountPluginHandles` skips a plugin whose module load faulted, and a boot
    /// that lost the HOST plugin that way used to answer `host program missing` for every later
    /// switch with no way back. One lazy install is the way back, on both targets.
    pub async fn switch_to_managed_app(&mut self, app_id: &str, view_state: Option<ViewModel>) -> Result<(), String> {
        let cfg = self.plugins.host_config().ok_or("host config missing")?;
        self.plugins.install_plugin(&cfg.plugin_id).await?;
        let semio_s_plugin_space = self.plugins.plugins.iter().find(|program| program.plugin_id == cfg.plugin_id).ok_or("host program missing")?;
        let app = semio_s_plugin_space.manifest.apps.iter().find(|candidate| candidate.id == app_id).ok_or("host app missing")?.clone();
        if let Some(session) = &self.docs.session {
            if session.plugin_id == semio_s_plugin_space.plugin_id && session.app.id == app_id {
                if let Some(next_view_state) = view_state {
                    if let Some(mut current) = self.docs.session.take() {
                        current.view_state = next_view_state;
                        self.docs.session = Some(current);
                        self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
                    }
                }
                return Ok(());
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if app_id == cfg.landing_app_id {
            if let Some(home) = self.directory.directory_home.as_mut().filter(|home| home.plugin_id == semio_s_plugin_space.plugin_id && home.app.id == app_id) {
                if let Some(next_view_state) = view_state {
                    home.view_state = next_view_state;
                }
                let session = home.active_session();
                self.windows.active_window_id = Some(session.app.window_kinds.first().id.clone());
                self.opening.open_space_id = None;
                self.docs.session = Some(session);
                return self.as_window_actions().refresh_ui(UiDirtyScope::Full).await;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let (Some(home), Some(current)) = (self.directory.directory_home.as_mut(), self.docs.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
            }
        }
        let instance_id = semio_s_plugin_space.create_app(&app.id).await?;
        let panel_state = self.as_session().default_host_panel_state()?;
        let default_view_state = ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(app.window_kinds.first().id.clone()),
            active_utility_id: None,
            panel_json: Some(panel_json(&panel_state)?),
            extension_input_json: None,
            session_identity: None,
            locale: self.prefs.active_locale(),
            terminology: self.prefs.active_terminology(),
            window_id: None,
            focused_window_id: Some(app.window_kinds.first().id.clone()),
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: Default::default(),
            tool_run_trace_cursor_by_window_id: Default::default(),
            tree_windows: Vec::new(),
            tree_viewport_rows: None,
        };
        let next_view_state = view_state.unwrap_or(default_view_state);
        self.windows.active_window_id = Some(app.window_kinds.first().id.clone());
        if app_id == cfg.landing_app_id {
            self.opening.open_space_id = None;
        }
        self.docs.session = Some(ActiveSession { plugin_id: semio_s_plugin_space.plugin_id.clone(), instance_id, app, view_state: next_view_state });
        self.as_window_actions().refresh_ui(UiDirtyScope::Full).await
    }

    /// 👁️✏️ Opens the sibling surface of the OPEN document's dialect — the wgpu twin of React's
    /// `switchToSessionRole` (`🧱️elements/🏛️ShellHost/🟦️.tsx`). A role the dialect does not declare,
    /// and the role already mounted, are both no-operations: `role_switch_target` answers `None` and
    /// nothing is created, retired or refreshed.
    pub async fn switch_to_session_role(&mut self, requested: semio_framework::manifest::AppRole) -> Result<(), String> {
        let Some(session) = self.docs.session.clone() else {
            return Ok(());
        };
        let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else {
            return Ok(());
        };
        let Some(target) = role_switch_target(&program.manifest.apps, &session.app.dialect, session.app.role, requested).cloned() else {
            return Ok(());
        };
        self.run_session_app_switch(&program, target).await
    }

    /// 🔀️ The ORDERED in-place session switch — `create → seal → retire → publish → seed → refresh`.
    /// The successor instance exists before the predecessor is retired, so no render ever observes
    /// two live instances of one plugin and no switch leaks a `create_app`. Twin of
    /// `runSessionAppSwitchV1` (`🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts`), whose `switch`
    /// fixture rows pin both the order and the `created − retired === 0` invariant. "Seal" is this
    /// target's own shape of React's close ladder: every retained surface document of the outgoing
    /// app is driven to terminal before its instance goes, because the resident aggregate is a fixed
    /// process-wide permit and a leaked root refuses every later surface with `Capacity`.
    pub async fn run_session_app_switch(&mut self, program: &ProgramBridgeEntry, app: AppDefinition) -> Result<(), String> {
        let Some(previous) = self.docs.session.clone() else {
            return Ok(());
        };
        let instance_id = program.create_app(&app.id).await.map_err(|refusal| self.prefs.note_refused_open(refusal, &mut self.chrome))?;
        debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "create", "plugin": program.plugin_id, "from": previous.app.id, "to": app.id, "instance": instance_id })));
        self.docs.retire_documents_outside(&[], true, &mut self.chrome)?;
        self.docs.retire_documents_outside(&[], false, &mut self.chrome)?;
        debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "seal", "instance": previous.instance_id })));
        program.destroy_app(previous.instance_id);
        debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "retire", "instance": previous.instance_id })));
        let landing_window_id = app.window_kinds.first().id.clone();
        let view_state = ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(landing_window_id.clone()),
            active_utility_id: None,
            panel_json: None,
            extension_input_json: None,
            session_identity: None,
            locale: self.prefs.active_locale(),
            terminology: self.prefs.active_terminology(),
            window_id: None,
            focused_window_id: Some(landing_window_id.clone()),
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: Default::default(),
            tool_run_trace_cursor_by_window_id: Default::default(),
            tree_windows: Vec::new(),
            tree_viewport_rows: None,
        };
        self.dock.layout_override = None;
        self.windows.active_window_id = Some(landing_window_id);
        self.docs.session = Some(ActiveSession { plugin_id: previous.plugin_id.clone(), instance_id, app, view_state });
        self.as_session().sync_session_chrome();
        self.as_session().sync_dock();
        self.as_window_actions().push_contributions().await?;
        self.announce_session_example().await?;
        debug_log(&format!("[TRACE] shell session switch {}", serde_json::json!({ "step": "publish", "app": self.docs.session.as_ref().map(|session| session.app.id.clone()) })));
        self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
        debug_log(&format!(
            "[TRACE] shell session switch {}",
            serde_json::json!({ "step": "refresh", "app": self.docs.session.as_ref().map(|session| session.app.id.clone()), "role": self.docs.session.as_ref().map(|session| session.app.role.as_str()) })
        ));
        Ok(())
    }

    pub async fn install_extension_from_store(&mut self, request: serde_json::Value) -> Result<bool, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let answer = extension_store_answer(request).await?;
            if answer.get("cancelled").and_then(serde_json::Value::as_bool) == Some(true) {
                return Ok(false);
            }
            let record: ShellExtensionStoreRecord = serde_json::from_value(answer.get("extension").cloned().ok_or("extension store install missing extension")?).map_err(|error| error.to_string())?;
            let load = self.plugins.load_extension_record(record).await;
            self.as_window_actions().push_contributions().await?;
            self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
            load.map(|_| true)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = request;
            Err("native extension installation is unavailable".into())
        }
    }

    pub fn set_extension_enabled<'a>(&'a mut self, extension_id: &'a str, enabled: bool) -> ShellTurn<'a, Result<bool, String>> {

        shell_turn(async move {
            let Some(index) = self.plugins.extensions.iter().position(|entry| entry.record.extension_id == extension_id) else { return Ok(false) };
            let program = self.plugins.extensions[index].program.clone();
            self.plugins.extensions[index].enabled = enabled;
            self.plugins.plugins.retain(|entry| entry.plugin_id != extension_id);
            if enabled {
                if let Some(program) = program {
                    self.plugins.plugins.push(program);
                }
            }
            self.as_window_actions().push_contributions().await?;
            self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
            Ok(true)

        })

    }

    pub async fn uninstall_extension(&mut self, extension_id: &str) -> Result<bool, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = extension_id;
            return Err("native extension uninstallation is unavailable".into());
        }
        #[cfg(target_arch = "wasm32")]
        {
            let Some(index) = self.plugins.extensions.iter().position(|entry| entry.record.extension_id == extension_id) else { return Ok(false) };
            let record = self.plugins.extensions[index].record.clone();
            self.plugins.plugins.retain(|entry| entry.plugin_id != extension_id);
            self.as_window_actions().push_contributions().await?;
            self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
            if let Err(error) = semio_framework_os_renderer_wgpu_program_bridge::retire_js_extension(extension_id).await {
                let recovery = self.plugins.load_extension_record(record.clone()).await;
                self.as_window_actions().push_contributions().await?;
                self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
                if let Err(recovery) = recovery {
                    return Err(format!("{error}; recovery failed: {recovery}"));
                }
                return Err(error);
            }
            if let Err(error) = extension_store_answer(serde_json::json!({ "op": "extension-store-uninstall", "extensionId": extension_id })).await {
                let recovery = self.plugins.load_extension_record(record).await;
                self.as_window_actions().push_contributions().await?;
                self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
                if let Err(recovery) = recovery {
                    return Err(format!("{error}; recovery failed: {recovery}"));
                }
                return Err(error);
            }
            self.plugins.extensions.remove(index);
            self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
            Ok(true)
        }
    }

    /// 📇️ ticket §5/§6 — "/spaces/{id}/studio" keeps the pre-existing studio behaviour (the ONLY
    /// behaviour this route had before this lane); a bare "/spaces/{id}" now opens the `s.space`
    /// artifact-index app instead, mirroring the React shell's `applyShellUri` (`📓️w2-c-report.md`).
    ///
    /// 🧭️ Pin before the async switch so a concurrent chrome sync cannot boot the demo example over
    /// an explicit `/spaces/:id` route.
    ///
    /// 🎠️ H3-wgpu-native — `load_app_document_pack` is now async.
    pub fn apply_shell_uri<'a>(&'a mut self, uri: &'a str) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            let path = uri.split('?').next().unwrap_or(uri);
            if path.trim_end_matches('/') == "/hub" {
                self.directory.hub_workspace_open = true;
                self.directory.project_verified_hub_authority();
                if self.directory.hub_workspace.session.phase == HubSessionPhase::SignedIn {
                    self.directory.start_hub_spaces_reload();
                }
                return self.as_window_actions().refresh_ui(UiDirtyScope::Full).await;
            }
            let Some(cfg) = self.plugins.host_config() else {
                return Ok(());
            };
            let raw = path.strip_prefix("/spaces/").map(|value| value.trim_end_matches('/').to_string()).filter(|value| !value.is_empty());
            if raw.is_none() {
                self.opening.open_space_id = None;
                if self.docs.session.as_ref().map(|session| session.app.id.as_str()) != Some(cfg.landing_app_id) {
                    self.switch_to_managed_app(cfg.landing_app_id, None).await?;
                }
                return Ok(());
            }
            let raw = raw.expect("space route");
            let space_route = match raw.split_once('/') {
                Some((id, "studio")) => (id.to_string(), true),
                _ => (raw.clone(), false),
            };
            let space_id = space_route.0;
            if !space_route.1 {
                self.opening.open_space_id = Some(space_id.clone());
                let host_program = self.plugins.plugins.iter().find(|program| program.plugin_id == cfg.plugin_id).cloned();
                let space_index = host_program.as_ref().and_then(program_space_index);
                let space_app = host_program
                    .as_ref()
                    .and_then(|program| find_space_index_app(program, semio_framework::manifest::AppRole::Editor).or_else(|| find_space_index_app(program, semio_framework::manifest::AppRole::Viewer)))
                    .cloned();
                let (Some(space_index), Some(space_app)) = (space_index, space_app) else {
                    debug_log(&format!("[TRACE] wgpu shell apply_shell_uri: the host plugin names no space index app (os.space-index) — not loaded yet"));
                    return Ok(());
                };
                self.as_window_actions().switch_to_app(cfg.plugin_id, space_app).await?;
                let (bindings, surface) = self.opening.default_bindings_for_current_session(&self.directory, &self.docs);
                return self.open_document(SPACE_INDEX_DOCUMENT_ID.to_string(), space_index.document_schema, bindings, surface, None).await;
            }
            let studio_changed = self.opening.open_space_id.as_deref() != Some(space_id.as_str());
            self.opening.open_space_id = Some(space_id.clone());
            self.switch_to_managed_app(cfg.host_app_id, None).await?;
            if !studio_changed {
                return Ok(());
            }
            let session = self.docs.session.clone().ok_or("space session missing")?;
            let program = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).ok_or("space program missing")?;
            let action = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "openSpace".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "spaceId": space_id }) };
            let action_json = serde_json::to_string(&action).map_err(|err| err.to_string())?;
            let result = program.handle_action(session.instance_id, &action_json, &session.view_state).await.map_err(|refusal| self.prefs.note_refused_open(refusal, &mut self.chrome))?;
            for effect in &result.requested_effects {
                if let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect {
                    program.load_app_document_pack(session.instance_id, pack, spr).await.map_err(|refusal| self.prefs.note_refused_open(refusal, &mut self.chrome))?;
                }
            }
            self.as_session().sync_session_chrome();
            self.as_window_actions().refresh_ui(UiDirtyScope::Full).await

        })

    }

}
//#endregion ShellActions
