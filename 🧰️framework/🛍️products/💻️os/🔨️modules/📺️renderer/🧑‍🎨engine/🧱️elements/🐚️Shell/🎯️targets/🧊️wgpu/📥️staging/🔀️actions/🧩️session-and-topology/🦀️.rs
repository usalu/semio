//! 🔀️ `shell-actions` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl<'ctx> ActionsCtx<'ctx> {

    pub async fn boot(&mut self) -> Result<(), String> {
        self.as_session().load_persisted_dock();
        self.plugins.plugin_faults.clear();
        self.chrome.error = None;
        debug_log("[TRACE] wgpu-shell boot: begin");
        #[cfg(target_arch = "wasm32")]
        if let Err(error) = self.plugins.refresh_extension_store().await {
            debug_log(&format!("[TRACE] wgpu extension inventory unavailable: {error}"));
        }
        if let Some(cfg) = self.plugins.host_config() {
            let host_plugin_id = cfg.plugin_id.to_string();
            let semio_s_plugin_space = self.plugins.plugins.iter().find(|p| p.plugin_id == host_plugin_id).ok_or("host program missing")?;
            let s_app = semio_s_plugin_space.manifest.apps.iter().find(|app| app.id == cfg.landing_app_id).or_else(|| semio_s_plugin_space.manifest.apps.first()).ok_or("host program missing landing app")?.clone();
            let panel_state = self.as_session().default_host_panel_state()?;
            let Some(instance_id) = self.prefs.open_boot_instance(&host_plugin_id, &s_app.id, &mut self.chrome, &mut self.plugins).await else {
                return self.settle_boot().await;
            };
            let view_state = ViewModel {
                active_mode_id: Some(s_app.default_mode_id.clone()),
                active_window_kind_id: Some(s_app.window_kinds.first().id.clone()),
                active_utility_id: None,
                panel_json: Some(panel_json(&panel_state)?),
                extension_input_json: None,
                session_identity: None,
                locale: self.prefs.active_locale(),
                terminology: self.prefs.active_terminology(),
                window_id: None,
                focused_window_id: Some(s_app.window_kinds.first().id.clone()),
                window_instances: Vec::new(),
                active_tool_id: None,
                active_utility_by_window_id: Default::default(),
                tool_run_trace_cursor_by_window_id: Default::default(),
                tree_windows: Vec::new(),
                tree_viewport_rows: None,
            };
            self.windows.active_window_id = Some(s_app.window_kinds.first().id.clone());
            let session = ActiveSession { plugin_id: host_plugin_id, instance_id, app: s_app, view_state };
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.directory.directory_home = Some(DirectoryHomeProjection::new(session.plugin_id.clone(), session.instance_id, session.app.clone(), session.view_state.clone()).map_err(|error| error.to_string())?);
            }
            self.docs.session = Some(session);
        } else {
            let selection = {
                let programs: Vec<(&str, &semio_framework::PluginManifest)> = self.plugins.plugins.iter().map(|entry| (entry.plugin_id.as_str(), &entry.manifest)).collect();
                let request = BootSelectionRequest { plugin_id: resolve_registry_plugin_id(&self.plugins.plugin_filter), app_id: resolve_playground_app_id(&self.plugins.plugin_filter) };
                select_boot_program(&programs, request, boot_app_role(), semio_framework_os_renderer_wgpu_host_base::boot_app_id().as_deref())
            };
            let Some((index, app)) = selection else {
                let plugin_id = resolve_registry_plugin_id(&self.plugins.plugin_filter).to_string();
                let app_id = resolve_playground_app_id(&self.plugins.plugin_filter).unwrap_or("").to_string();
                self.prefs.record_plugin_fault(&plugin_id, &app_id, "the requested plugin contributed no app to this boot".to_string(), &mut self.chrome, &mut self.plugins);
                return self.settle_boot().await;
            };
            let plugin_id = self.plugins.plugins[index].plugin_id.clone();
            let Some(instance_id) = self.prefs.open_boot_instance(&plugin_id, &app.id, &mut self.chrome, &mut self.plugins).await else {
                return self.settle_boot().await;
            };
            self.windows.active_window_id = Some(app.window_kinds.first().id.clone());
            self.docs.session = Some(ActiveSession {
                plugin_id,
                instance_id,
                app: app.clone(),
                view_state: ViewModel {
                    active_mode_id: Some(app.default_mode_id.clone()),
                    active_window_kind_id: self.windows.active_window_id.clone(),
                    active_utility_id: None,
                    panel_json: None,
                    extension_input_json: None,
                    session_identity: None,
                    locale: self.prefs.active_locale(),
                    terminology: self.prefs.active_terminology(),
                    window_id: None,
                    focused_window_id: self.windows.active_window_id.clone(),
                    window_instances: Vec::new(),
                    active_tool_id: None,
                    active_utility_by_window_id: Default::default(),
                    tool_run_trace_cursor_by_window_id: Default::default(),
                    tree_windows: Vec::new(),
                    tree_viewport_rows: None,
                },
            });
        }
        self.as_session().apply_boot_mode();
        if let Some(session) = &self.docs.session {
            debug_log(&format!("[TRACE] wgpu-shell boot: program={} app={} mode={:?}", session.plugin_id, session.app.id, session.view_state.active_mode_id));
        }
        self.settle_boot().await
    }

    /// 📇️ ticket §1 — non-blocking (submits finite shared-pool turns and returns immediately, see
    /// `bootstrap_identity`'s own doc): a slow/unreachable hub must never delay the first frame.
    ///
    /// 🌐️ Browser: there is no credential to claim and no pool future to submit — only the boot
    /// environment to record. `poll_browser_identity` (every-frame, retry-floored) does the one
    /// `GET /auth/sessions/me` round trip, so this stays as non-blocking as the native arm.
    ///
    /// 🧩️ Contributions cross BEFORE the first render, the way React's `refreshUi` starts its
    /// publisher before its own first await: a surface rendered against an EMPTY flow-extension
    /// registry arms nothing at all (the guest's `may_rearm` gate — a graph whose operator kinds
    /// the registry cannot serve owes no continuation), so pushing after the refresh left every
    /// preview window with no tick and no chain to re-arm
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🫀️ The boot ARMS the settle lane and returns; it does not converge. `flush_deferred_actions`
    /// used to run the whole chain to a fixed point right here, so an `?example=` boot spent its
    /// entire evaluation INSIDE `boot_shell` — measured on 6118 as `boot_shell leave 7 226 ms` with
    /// the first chrome at 9 212 ms against 1 880 ms / 4 101 ms for the same build with no example,
    /// i.e. 5.3 s of convergence with nothing painted and no status pill to show it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md` §8.1).
    /// The frame loop takes it from here, one [`Self::settle_pump_step`] per frame, which is what
    /// React's refresh lane has always done with its owed slot.
    pub async fn settle_boot(&mut self) -> Result<(), String> {
        self.as_session().sync_dock();
        self.as_session().sync_session_chrome();
        #[cfg(not(target_arch = "wasm32"))]
        self.directory.bootstrap_identity();
        #[cfg(target_arch = "wasm32")]
        {
            self.directory.identity_env = resolve_identity_env();
        }
        self.as_window_actions().push_contributions().await?;
        self.announce_session_example().await?;
        declare_boot_subphase("shell-boot:refresh-ui", "enter", 0.0);
        let refresh_started = instant_now_ms();
        self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
        declare_boot_subphase("shell-boot:refresh-ui", "leave", (instant_now_ms() - refresh_started).max(0.0));
        self.settle.owe_settle();
        Ok(())
    }

    /// 📚️ Announces the RESOLVED example to a freshly mounted instance — the wgpu twin of React's
    /// per-instance boot-example effect (`🏛️ShellHost/🟦️.tsx`), which runs for EVERY new session
    /// instance: the boot one (from `settle_boot`, AFTER `push_contributions` armed the guest's
    /// flow-extension registry and before the first `refresh_ui`) and every successor of a role switch.
    /// `setActiveExample` is dispatched exactly as `handle_control_command`'s `shell.example.*` arm
    /// does for a click, so the guest opens the document the navbar picker shows and journals it.
    ///
    /// 🩸️ The boot used to announce only an example a `?example=` query named: without the query the
    /// picker showed the dialect's first example while the guest still held its empty genesis document —
    /// an empty board and a History with no row where React boots on "Set Active Example"
    /// (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, live fault F11). Before that a role switch
    /// journalled nothing (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, packet W10b).
    /// `sync_session_chrome` has already run `resolve_boot_example_id` for the instance's dialect (a
    /// query id the dialect does not author falls back, never a boot failure), so the id announced here
    /// is exactly the one the navbar picker shows.
    pub async fn announce_session_example(&mut self) -> Result<(), String> {
        let Some(example_id) = self.plugins.active_example_id.clone().filter(|id| !id.is_empty()) else {
            return Ok(());
        };
        let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) else {
            return Ok(());
        };
        debug_log(&format!("[TRACE] wgpu-shell session example {}", serde_json::json!({ "exampleId": example_id, "controller": controller_id })));
        self.dispatch_action(ActionDescriptor { controller_id, action: "setActiveExample".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "exampleId": example_id }) }).await
    }
}

//#endregion ShellLifecycle

