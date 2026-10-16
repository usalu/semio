//! 🪝️ `shell-window-actions` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl<'ctx> WindowActionsCtx<'ctx> {
    /// 🧩️ The guest answers `setContributions` with the effects it could not arm before
    /// its extension registry moved — a `flowEvalTick` per attached preview window. React's
    /// `ShellHost` dispatches exactly these through `applyHostEffects`; this target dropped
    /// the whole answer on the floor, so the guest's latch stayed armed and no preview ever
    /// evaluated (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn push_contributions(&mut self) -> Result<(), String> {
        let Some(session) = self.docs.session.clone() else {
            return Ok(());
        };
        if !app_owns_command(&session.app, "setContributions") {
            debug_log(&format!("[TRACE] contributions push {}", serde_json::json!({ "plugin": session.plugin_id, "app": session.app.id, "skipped": "app-owns-no-setContributions", "encoding": "pack", "crossings": 0 })));
            return Ok(());
        }
        declare_boot_subphase("shell-boot:contributions-push", "enter", 0.0);
        let started = instant_now_ms();
        #[cfg(target_arch = "wasm32")]
        {
            let view_json = serde_json::to_string(&self.as_session().live_view_state(&session)).unwrap_or_else(|_| "{}".into());
            if let Some(plugin) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() {
                debug_log(&format!("[TRACE] contributions push {}", serde_json::json!({ "plugin": plugin.plugin_id, "app": session.app.id, "active": true, "encoding": "pack", "crossings": 1 })));
                match plugin.push_scoped_contributions(session.instance_id, &session.app.id, &view_json).await {
                    Ok(result) => {
                        debug_log(&format!("[TRACE] setContributions deferred effects {}", serde_json::json!({ "plugin": plugin.plugin_id, "app": session.app.id, "effects": result.requested_effects.len() })));
                        self.queue_host_effects(&session.app.controller_id, result.requested_effects);
                    }
                    Err(error) => debug_log(&format!("[TRACE] setContributions command failed {} {error}", plugin.plugin_id)),
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = session;
            debug_log("[TRACE] contributions push skipped native-no-pack-sender");
        }
        declare_boot_subphase("shell-boot:contributions-push", "leave", (instant_now_ms() - started).max(0.0));
        Ok(())
    }

    /// 🐢️ One refresh pass. `ask` is the [`UiDirtyScope`] the dispatch that caused it declared,
    /// unioned with everything owed since the last pass ([`Self::owe_refresh`]).
    ///
    /// **The scope is honoured**: a window body, a panel body, the engagements section and the
    /// measures section are re-rendered exactly when the union names them, over the same kernel
    /// predicates ([`UiDirtyScope::wants_window_body`] and siblings) whose TypeScript twins React's
    /// `🛠️ShellHelpers/🟦️.tsx` runs against the same fixture — one selection law per language, never
    /// one per renderer.
    ///
    /// ⚖️ Narrowing was measured as a product STALL before the settle pump existed, and the reason is
    /// worth keeping: a window body's render is also the guest CROSSING that funds the guest's own
    /// background evaluation (`render_with_document` submits a turn, and that turn is what pumps the
    /// guest's worker pool), so a refresh that skips every surface withdraws compute from a brep solve
    /// the UI has nothing to do with — 14 of 16 battery examples froze mid-solve, `box-fillet-preview`
    /// never converging in 180 s at 36 renders against 21.13 s at 96 with the scope forced to `Full`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4).
    ///
    /// That funding is no longer this function's job. [`ShellSettlePump`] owns it: the frame loop
    /// takes one settle step per frame and, whenever nothing is armed and a producer still reports
    /// `computing`, spends exactly one crossing on that producer's own window body. The refresh is
    /// therefore free to render only what a settle actually dirtied, which on a converging edit is
    /// nothing at all for all seven `flowEvalTick` hops and the whole shell only for `toolRunStart`.
    ///
    /// ⚠️ A surface's retirement (`window_ui` remove → `retire_one_surface_document`) stays INSIDE the
    /// per-surface loop, never ahead of it, so a skipped surface keeps the exact document it already
    /// owns instead of being retired and never re-minted.
    pub async fn refresh_ui(&mut self, ask: UiDirtyScope) -> Result<(), String> {
        self.refresh_ui_rendered(ask, None).await
    }

    /// 🖼️ [`Self::refresh_ui`] with the guest bodies and reserved sections some of whose reads already
    /// happened off the shell: a surface or section found in `rendered` takes that document, every other
    /// wanted one is read here, and a read no longer wanted — or read for an instance that is no longer
    /// the session's — is handed back by its own `Drop`.
    ///
    /// 🧾️ The SHELL-owned leaves (the Command dock, the five Settings leaves, the Marketplace leaf,
    /// every `tool.<id>` leaf and the chat transcript) are not guest-published: they are retained
    /// beside the guest's own panel surfaces so `retire_documents_outside` never sweeps them.
    ///
    /// 🧰️ The utility bar is derived from the app's declared `AppDefinition.utilities` (scoped to the active
    /// window kind) via `ui_wgpu::wgpu::derive_utility_nodes` — the old per-call `plugin.utilities()` fetch and the
    /// `find_active_utility_id` "first pressed toggle" heuristic are gone (Architecture Decision 5).
    /// 🩸️ The `framework.sync.{file,folder,remote}` toggles used to be appended here and painted in
    /// the footer. React builds the same three leaves but hands them to the bottom-left `s-sync-status`
    /// tab's `SyncAttachCard` (`🏛️ShellHost/🟦️.tsx:9076`) — its footer never carries them.
    ///
    /// 🛍️ Deliberately unscoped: the GUEST fetch inside is already claimed to once per app
    /// instance, and what remains is the local republish onto every live node-graph engine host —
    /// the moment a surface minted by THIS pass gets its palette. Skipping that on a partial scope
    /// is how a spotlight silently opens empty.
    ///
    /// 🎬️ Deliberately UNSCOPED, like the shell-owned panel leaves above: these two bodies are the
    /// shell's own projection of state the user moves without any guest round trip (the expanded
    /// action, its staged buffer, the possibles chevron), so a scope that skipped them would leave
    /// a pressed row painting its previous frame forever. The ingress is revision-keyed, so an
    /// unchanged body still pays nothing.
    ///
    /// 🛠️ The TOOL half of the same reader — React's `toolMeasuresByToolId` ref, refreshed on the
    /// scope that names tools so a slider tick inside an armed tool's options repaints its leaf.
    pub async fn refresh_ui_rendered(&mut self, ask: UiDirtyScope, rendered: Option<ShellRenderedRefresh>) -> Result<(), String> {
        let mut latency = semio_framework_os_renderer_wgpu_host_base::frame_latency::FrameLatencyTimer::start(semio_framework_os_renderer_wgpu_host_base::frame_latency::latest_frame_authority(), semio_framework_os_renderer_wgpu_host_base::frame_latency::FrameLatencyStage::ShellRefresh, 1);
        let scope = core::mem::replace(&mut self.settle.owed_refresh_scope, UiDirtyScope::None).merged_with(ask);
        let Some(session) = self.docs.session.clone() else {
            return Ok(());
        };
        let mut read_ahead = rendered.filter(|rendered| rendered.instance_id == session.instance_id).unwrap_or_else(|| ShellRenderedRefresh::empty(session.instance_id));
        self.docs.drain_retained_document_arenas();
        self.as_session().sync_dock();
        let view_state = self.as_session().live_view_state(&session);
        let live_windows = self.dock.dock.window_instances();
        latency.set_work_items(live_windows.len());
        let measure_windows: Vec<String> = live_windows.iter().map(|(window_id, _)| window_id.clone()).collect();
        let mut refresh_effects = Vec::new();
        let mut faults: Vec<(String, String, semio_framework_os_renderer_wgpu_program_bridge::ProgramFault)> = Vec::new();
        let mut rendered = 0usize;
        let mut visited: Vec<String> = Vec::new();
        {
            let program = self.plugins.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
            let retained: Vec<String> = live_windows.iter().map(|(window_id, _)| window_id.clone()).collect();
            self.docs.retire_documents_outside(&retained, true, &mut self.chrome)?;
            for (window_id, window_kind_id) in live_windows {
                let kind = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).ok_or_else(|| format!("window kind '{}' is absent from the app", window_kind_id))?;
                if !scope.wants_window_body(&kind.body_key) {
                    continue;
                }
                let window_view = view_state.for_window_instance(&window_id).ok_or_else(|| format!("window '{}' is absent from the live view", window_id))?;
                let previous = self.docs.window_ui.remove(&window_id);
                self.docs.retire_one_surface_document(previous)?;
                rendered += 1;
                visited.push(window_id.clone());
                debug_log(&format!("[TRACE] wgpu-shell render begin surface={window_id} body={}", kind.body_key));
                let render_started = instant_now_ms();
                declare_boot_subphase(&format!("shell-boot:render:{window_id}"), "enter", 0.0);
                let document = match read_ahead.surfaces.remove(&window_id) {
                    Some((document, mut effects)) => {
                        refresh_effects.append(&mut effects);
                        document
                    }
                    None => program.render_with_document(session.instance_id, &window_id, &kind.body_key, &window_view, None, Some(&mut refresh_effects)).await,
                };
                match document {
                    Ok(document) => {
                        self.docs.window_ui.insert(window_id.clone(), document);
                    }
                    Err(error) => faults.push((window_id.clone(), kind.body_key.clone(), error.into())),
                }
                let elapsed = (instant_now_ms() - render_started).max(0.0);
                declare_boot_subphase(&format!("shell-boot:render:{window_id}"), "leave", elapsed);
                debug_log(&format!("[TRACE] wgpu-shell render leave surface={window_id} {elapsed:.0} ms{}", resident_census()));
            }
        }
        let program = self.plugins.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
        if let Err(error) = self.as_session().refresh_app_document_identity(&program, session.instance_id).await {
            faults.push(("document-identity".into(), "document-identity".into(), error.into()));
        }
        let panel_view = view_state.for_panel();
        let panel_leaves: Vec<(String, String)> = flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().filter_map(|tab| tab.body_key.as_deref().map(|body_key| (tab.id().to_string(), body_key.to_string()))).collect();
        let shell_leaves: Vec<String> = self.docs.shell_owned_panel_leaves(&self.chrome, &self.plugins, &self.prefs);
        let retained: Vec<String> = panel_leaves.iter().map(|(id, _)| id.clone()).chain(shell_leaves.iter().cloned()).collect();
        self.docs.retire_documents_outside(&retained, false, &mut self.chrome)?;
        for tab_id in shell_leaves {
            match self.as_session().publish_shell_panel_document(&tab_id) {
                Ok(Some(document)) => {
                    let previous = self.docs.panel_documents.insert(tab_id.clone(), document);
                    self.docs.retire_one_surface_document(previous)?;
                }
                Ok(None) => {}
                Err(error) => faults.push((tab_id.clone(), tab_id.clone(), error.into())),
            }
        }
        for (tab_id, body_key) in panel_leaves {
            if !scope.wants_panel_body(&body_key) {
                continue;
            }
            let previous = self.docs.panel_documents.remove(&tab_id);
            self.docs.retire_one_surface_document(previous)?;
            rendered += 1;
            visited.push(tab_id.clone());
            debug_log(&format!("[TRACE] wgpu-shell render begin surface={tab_id} body={body_key}"));
            let render_started = instant_now_ms();
            declare_boot_subphase(&format!("shell-boot:render:{tab_id}"), "enter", 0.0);
            let document = match read_ahead.surfaces.remove(&tab_id) {
                Some((document, mut effects)) => {
                    refresh_effects.append(&mut effects);
                    document
                }
                None => program.render_with_document(session.instance_id, &tab_id, &body_key, &panel_view, None, Some(&mut refresh_effects)).await,
            };
            match document {
                Ok(document) => {
                    let reveal = tab_id == FRAMEWORK_PANEL_TAB_TOOL_RUN_ID && tool_run_panel_reveals(&mut self.plugins.tool_run_panel_runs, &document)?;
                    self.docs.panel_documents.insert(tab_id.clone(), document);
                    if reveal {
                        self.dock.chrome_tour_reveal_panel_tab(&session, &tab_id, &mut self.settle, &self.plugins);
                    }
                }
                Err(error) => faults.push((tab_id.clone(), body_key.clone(), error.into())),
            }
            let elapsed = (instant_now_ms() - render_started).max(0.0);
            declare_boot_subphase(&format!("shell-boot:render:{tab_id}"), "leave", elapsed);
            debug_log(&format!("[TRACE] wgpu-shell render leave surface={tab_id} {elapsed:.0} ms{}", resident_census()));
        }
        self.windows.active_utilities = self.dock.derive_utility_nodes(&session, &self.prefs, &self.windows);
        self.as_session().refresh_app_catalogue(&program, session.instance_id, &panel_view, read_ahead.catalogue.take()).await;
        if scope.wants_section(UiDirtySection::Engagements) {
            visited.push(semio_framework::UiRefreshSection::Engagements.body_key().to_string());
            self.windows.refresh_window_engagements(&program, session.instance_id, &view_state, read_ahead.engagements.take(), &mut faults, &mut self.docs).await?;
        }
        visited.extend(measure_windows.iter().map(|window_id| window_actions_surface_id(window_id)));
        visited.extend(measure_windows.iter().map(|window_id| window_search_surface_id(window_id)));
        self.refresh_window_action_panes(&measure_windows, &mut faults)?;
        if scope.wants_section(UiDirtySection::Measures) {
            visited.push(semio_framework::UiRefreshSection::Measures.body_key().to_string());
            visited.extend(measure_windows.iter().map(|window_id| window_measures_surface_id(window_id)));
            self.windows.refresh_window_measures(&program, session.instance_id, &view_state, read_ahead.measures.take(), &measure_windows, &mut faults, &mut self.docs).await?;
        }
        if scope.wants_section(UiDirtySection::Tools) && !self.docs.tool_panel_tabs(&self.prefs).is_empty() {
            visited.push(semio_framework::UiRefreshSection::Tools.body_key().to_string());
            self.windows.refresh_tool_measures(&program, session.instance_id, &view_state, read_ahead.tools.take(), &mut faults, &mut self.docs).await?;
        }
        let previous_spawned_session = self.docs.spawned_session.clone();
        if let Some(document) = self.docs.spawned_ui.take() {
            if let Err(document) = self.docs.retain_document_for_close(document) {
                self.docs.spawned_ui = Some(document);
                return Err("shell: spawned document retirement registry refused the exact prior owner".to_string());
            }
        }
        self.docs.spawned_session = None;
        if self.plugins.space_mode {
            if let Some(panel) = panel_state_from_view(&session.view_state)? {
                if let Some(spawned) = panel.active_spawned_id.as_ref().and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id)) {
                    if let Some(spawn_plugin) = self.plugins.plugins.iter().find(|p| p.plugin_id == spawned.plugin_id).cloned() {
                        let spawned_app = spawn_plugin.manifest.apps.iter().find(|app| app.id == spawned.app_id).cloned();
                        if let Some(app) = spawned_app {
                            if let Err(error) = self.as_session().refresh_app_document_identity(&spawn_plugin, spawned.instance_id).await {
                                faults.push((spawned.id.clone(), "document-identity".into(), error.into()));
                            }
                            let body_key = app.window_kinds.first().body_key.clone();
                            let fresh_view = ViewModel {
                                active_mode_id: Some(app.default_mode_id.clone()),
                                active_window_kind_id: Some(app.window_kinds.first().id.clone()),
                                active_utility_id: None,
                                panel_json: None,
                                extension_input_json: None,
                                session_identity: self.directory.session_identity_view(),
                                locale: self.prefs.active_locale(),
                                terminology: self.prefs.active_terminology(),
                                window_id: Some(spawned.id.clone()),
                                focused_window_id: Some(spawned.id.clone()),
                                window_instances: vec![semio_framework::ViewWindowInstance { id: spawned.id.clone(), window_kind_id: app.window_kinds.first().id.clone() }],
                                active_tool_id: None,
                                active_utility_by_window_id: Default::default(),
                                tool_run_trace_cursor_by_window_id: Default::default(),
                                tree_windows: Vec::new(),
                                tree_viewport_rows: None,
                            };
                            let mut view_state = previous_spawned_session.as_ref().filter(|owner| owner.plugin_id == spawned.plugin_id && owner.instance_id == spawned.instance_id && owner.app.id == app.id).map(|owner| owner.view_state.clone()).unwrap_or(fresh_view);
                            view_state.locale = self.prefs.active_locale();
                            view_state.terminology = self.prefs.active_terminology();
                            view_state.session_identity = self.directory.session_identity_view();
                            visited.push(spawned.id.clone());
                            match spawn_plugin.render(spawned.instance_id, &spawned.id, &body_key, &view_state).await {
                                Ok(document) => {
                                    self.docs.spawned_ui = Some(document);
                                    self.docs.spawned_session = Some(ActiveSession { plugin_id: spawned.plugin_id.clone(), instance_id: spawned.instance_id, app, view_state });
                                }
                                Err(error) => faults.push((spawned.id.clone(), body_key.clone(), error.into())),
                            }
                        }
                    }
                }
            }
        }
        let earned = self.docs.host_effect_earned_scope(&refresh_effects);
        self.queue_host_effects(&session.app.controller_id, refresh_effects);
        self.settle.owe_refresh(earned);
        self.prefs.settle_surface_faults(faults, &visited, &mut self.chrome, &mut self.plugins);
        if self.windows.window_topology_refresh_owed {
            self.as_session().complete_window_topology_refresh();
        }
        debug_log(&format!("[TRACE] wgpu-shell refresh scope={} rendered={rendered}", refresh_scope_label(&scope)));
        Ok(())
    }
}

//#endregion ShellLifecycle

