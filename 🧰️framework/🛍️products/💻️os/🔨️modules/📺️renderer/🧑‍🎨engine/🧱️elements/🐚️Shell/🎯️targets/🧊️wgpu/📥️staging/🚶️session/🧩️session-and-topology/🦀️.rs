//! 🚶️ `shell-session` staging of `session-and-topology`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl<'ctx> SessionCtx<'ctx> {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn poll_shell_io(&mut self) -> bool {
        const MAX_COMPLETIONS_PER_FRAME: usize = 8;
        let mut changed = false;
        for _ in 0..MAX_COMPLETIONS_PER_FRAME {
            let result = match self.settle.shell_io_pending.front() {
                Some(pending) => pending.receiver.try_recv(),
                None => break,
            };
            match result {
                Ok(ShellIoCompletion::Actions(actions)) => {
                    if let Some(pending) = self.settle.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                    changed |= !actions.is_empty();
                    self.settle.deferred_actions.extend(actions);
                }
                Ok(ShellIoCompletion::Import(picked)) => {
                    if let Some(pending) = self.settle.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                    changed |= self.opening.begin_import_transfer(picked, &self.docs);
                }
                Ok(ShellIoCompletion::MediaFrames { controller_id, frames }) => {
                    if let Some(pending) = self.settle.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                    changed |= self.opening.begin_media_frames_transfer(controller_id, frames, &self.docs);
                }
                Ok(ShellIoCompletion::Program(result)) => {
                    if let Err(refusal) = result { self.prefs.note_refused_open(refusal, &mut self.chrome); changed = true; }
                    if let Some(pending) = self.settle.shell_io_pending.pop_front() { if let Some(task) = pending.task { task.cancel(); } }
                }
                Ok(ShellIoCompletion::Finished) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    if let Some(pending) = self.settle.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
            }
        }
        changed
    }

    //#region 🏠️🧳️PluginHostConfig

    //#endregion 🏠️🧳️PluginHostConfig

    pub fn default_host_panel_state(&self) -> Result<SpacePanelState, String> {
        let active_panel_tab = self.plugins.host_catalogue_tab_id().ok_or_else(|| "host-panel.missing-configured-leaf".to_string())?;
        let state = SpacePanelState { active_panel_tab, spawned_apps: vec![], active_spawned_id: None };
        validate_panel_state(&state)?;
        Ok(state)
    }

    /// 🧯️ A retained document whose per-frame cursor reached its TERMINAL fault phase must release the
    /// chrome step, not hold it. `render_ui_document_step` answers `false` for "not complete", and the
    /// chrome walk used to read that as "come back next opportunity" — which for a terminal fault is
    /// never, so one unreadable body wedged the whole frame transaction and the shell presented
    /// nothing at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-blank-paint-2026-09-12.md`).
    /// The surface's own fault card is the right consequence; a black canvas is not.
    pub fn record_document_paint_fault(&mut self, surface_id: &str) {
        if self.plugins.surface_faults.iter().any(|fault| fault.surface_id == surface_id) {
            return;
        }
        let body_key = self.dock.document_body_key(surface_id, &self.docs).unwrap_or_else(|| surface_id.to_string());
        self.prefs.record_surface_fault(surface_id, &body_key, "retained document ingress reached its terminal fault".into(), &mut self.chrome, &mut self.plugins);
    }

    /// 🪟️ One retained body finished a paint: its paint fault (if any) clears, and its windowed tree containers are measured and
    /// reported ([`Self::observe_tree_windows`]).
    pub fn note_retained_body_painted(&mut self, surface_id: &str) {
        if self.plugins.surface_faults.iter().any(|fault| fault.surface_id == surface_id) {
            self.plugins.surface_faults.retain(|fault| fault.surface_id != surface_id);
            self.chrome.error = self.prefs.fault_status(&self.plugins);
        }
        self.dock.observe_tree_windows(surface_id, &self.docs);
    }

    /// 🎭️ Applies the boot-requested mode (`?mode=`) to the session the boot just opened: the mode
    /// becomes active AND its authored layout becomes the dock's, exactly as
    /// `handle_control_command`'s `playground.navbar.modes.*` arm does for a click. A mode the open
    /// app does not declare is ignored — a url is not a place to hard-fail a shell, the same rule
    /// `ChromeRole::from_boot_env` follows for `?role=`.
    pub fn apply_boot_mode(&mut self) {
        let Some(mode_id) = semio_framework_os_renderer_wgpu_host_base::boot_app_mode() else { return };
        let Some(session) = self.docs.session.as_mut() else { return };
        if !session.app.modes.iter().any(|mode| mode.id == mode_id) {
            debug_log(&format!("[TRACE] wgpu-shell boot mode {mode_id:?} is not declared by app {} — keeping {}", session.app.id, session.app.default_mode_id));
            return;
        }
        session.view_state.active_mode_id = Some(mode_id.clone());
        let layout = semio_framework::resolve_layout_for_mode(&session.app, &mode_id);
        if let Some(layout) = layout {
            self.dock.layout_override = Some(layout);
            self.sync_dock();
            self.windows.active_window_id = self.dock.dock.active_window_id.clone();
        }
    }

    /// 🧭️ The dock's tab sources are the session's own panel tabs, so every chrome sync rebuilds it.
    /// The persisted override layer is keyed by app id, so a NEW app additionally reloads that app's
    /// layer — reloading on every sync would clobber the reveal `apply_host_panel_selection` just made.
    pub fn sync_session_chrome(&mut self) {
        let Some(session) = &self.docs.session else {
            return;
        };
        let examples: Vec<String> = self.plugins
            .plugins
            .iter()
            .find(|entry| entry.plugin_id == session.plugin_id)
            .map(|entry| shell_offered_examples(&entry.manifest.examples, &session.app).into_iter().map(|example| example.id.clone()).collect())
            .unwrap_or_default();
        let resolved = resolve_boot_example_id(self.plugins.active_example_id.as_deref().unwrap_or(""), &examples, semio_framework_os_renderer_wgpu_host_base::boot_app_example().as_deref());
        self.plugins.active_example_id = (!resolved.is_empty()).then_some(resolved);
        let app_id = self.docs.dock_store_app_id();
        if self.dock.dock_loaded_app_id != app_id {
            self.dock.dock_loaded_app_id = app_id;
            self.load_persisted_dock();
        } else {
            self.sync_dock_tabs();
        }
    }

    /// 🧭️ Where the tab lives NOW wins over its declared group — a tab dragged to another anchor
    /// must open where the user put it, which is exactly why `reveal_dock_tab` locates it in the live
    /// dock first and only falls back on `PanelGroup::anchor()` when the dock has no node for it yet.
    pub fn apply_host_panel_selection(&mut self, action: &ActionDescriptor) -> Option<Result<(), String>> {
        if self.plugins.host_config().is_none() || action.action != "setActivePanelTab" {
            return None;
        }
        let target_app = self.docs.session.as_ref().filter(|session| session.app.controller_id == action.controller_id).map(|session| &session.app).or_else(|| self.plugins.host_app().filter(|app| app.controller_id == action.controller_id));
        let Some(target_app) = target_app else {
            return None;
        };
        let tab_id = match action.args.as_ref().and_then(|args| args.get("tabId")).and_then(semio_framework_value::DslValue::as_str) {
            Some(tab_id) if panel_identifier(tab_id) => tab_id.to_string(),
            _ => return Some(Err("host-panel.invalid-tab-id".into())),
        };
        let Some(group) = flatten_panel_tab_leaves(&target_app.panel_tabs).into_iter().find(|tab| tab.id() == tab_id).map(|tab| tab.group) else {
            return Some(Err("host-panel.tab-is-not-configured-leaf".into()));
        };
        let Some(current) = self.docs.session.as_ref() else {
            return Some(Err("host-panel.session-missing".into()));
        };
        let mut panel = match panel_state_from_view(&current.view_state) {
            Ok(Some(panel)) => panel,
            Ok(None) => match self.default_host_panel_state() {
                Ok(panel) => panel,
                Err(error) => return Some(Err(error)),
            },
            Err(error) => return Some(Err(error)),
        };
        panel.active_panel_tab = tab_id.clone();
        let panel_json = match panel_json(&panel) {
            Ok(panel_json) => panel_json,
            Err(error) => return Some(Err(error)),
        };
        let current_identity = (current.plugin_id.clone(), current.instance_id);
        if let Some(session) = self.docs.session.as_mut() {
            session.view_state.panel_json = Some(panel_json.clone());
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(home) = self.directory.directory_home.as_mut().filter(|home| home.is_instance(&current_identity.0, current_identity.1)) {
            home.view_state.panel_json = Some(panel_json);
        }
        if self.dock.reveal_dock_tab(&tab_id).is_none() {
            let anchor = PanelAnchor::from_group(group);
            let state = self.dock.anchor_state_mut(anchor);
            state.path = vec![tab_id];
            state.visible = true;
        }
        self.sync_session_chrome();
        Some(Ok(()))
    }

    pub fn sync_dock(&mut self) {
        if let Some(session) = &self.docs.session {
            let owner=(session.plugin_id.clone(),session.instance_id,session.app.id.clone());
            let incoming_layout = self.dock.layout_override.as_ref().or(session.app.default_layout.as_ref());
            let roster = session_window_instances(session, &self.dock.dock).into_iter().map(|instance| instance.id).collect::<Vec<_>>();
            let owner_changed = self.dock.dock_instance_owner.as_ref().is_none_or(|(plugin_id, instance_id, app_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id || app_id != &session.app.id);
            let source_changed = owner_changed || self.dock.dock_input_identity.as_ref().is_none_or(|input| input.source.as_ref() != incoming_layout);
            let mode_layout = incoming_layout.map(dock_mode_layout_identity);
            let input_changed = owner_changed || self.dock.dock_input_identity.as_ref().is_none_or(|input| input.mode_layout != mode_layout || input.roster != roster);
            if source_changed || (incoming_layout.is_none() && input_changed) {
                let maximized = self.dock.dock.maximized_stack.clone();
                if let Some(layout) = &self.dock.layout_override {
                    self.dock.dock.apply_layout_diff(layout);
                    if self.dock.dock.active_window_id.is_none() {
                        self.dock.dock.active_window_id = self.windows.active_window_id.clone().or_else(|| session.view_state.active_window_kind_id.clone());
                    }
                } else {
                    let mobile = self.dock.dock.mobile;
                    self.dock.dock = DockState::from_app(&session.app, self.windows.active_window_id.as_deref());
                    self.dock.dock.mobile = mobile;
                }
                if !input_changed {
                    self.dock.dock.maximized_stack = maximized;
                }
            }
            if source_changed || input_changed {
                self.dock.dock.reconcile_app_windows(&session.app);
            }
            if input_changed {
                self.dock.dock.maximized_stack = None;
            }
            let roster = session_window_instances(session, &self.dock.dock).into_iter().map(|instance| instance.id).collect();
            self.dock.dock_input_identity = Some(DockInputIdentity { source: incoming_layout.cloned(), mode_layout, roster });
            if let Some(seed) = dock_seed_active_window_id_v1(&mode_layout_stacks_v1(&self.dock.dock.root), self.windows.active_window_id.as_deref()) {
                self.windows.active_window_id = Some(seed);
            }
            if let Some(id) = &self.windows.active_window_id {
                self.dock.dock.sync_active_window(id);
            }
            let live = self.dock.dock.collect_window_ids();
            if owner_changed {
                self.windows.world_projection_template.clear();
            }
            let retired_projection_documents: Vec<String> = self.docs
                .window_projection_documents
                .keys()
                .filter(|window_id| owner_changed || !live.contains(*window_id))
                .cloned()
                .collect();
            for window_id in retired_projection_documents {
                let previous = self.docs.window_projection_documents.remove(&window_id);
                if let Err(error) = self.docs.retire_one_surface_document(previous) {
                    self.chrome.error = Some(error);
                }
            }
            self.windows.world_projection_template.retain(|window_id, _| live.contains(window_id));
            self.windows.window_icon_overrides.retain(|window_id, icon| icon.plugin_id == owner.0 && icon.app_id == owner.2 && icon.app_instance_id == owner.1 && live.contains(window_id));
            self.windows.window_title_overrides.retain(|window_id, title| title.plugin_id == owner.0 && title.app_id == owner.2 && title.app_instance_id == owner.1 && live.contains(window_id));
            for window_id in live {
                if let Some(template_id) = self.dock.dock.window_template_id(&window_id) {
                    self.windows.world_projection_template.entry(window_id).or_insert_with(|| template_id.to_string());
                }
            }
            self.dock.dock_instance_owner = Some(owner);
        } else {
            self.windows.world_projection_template.clear();
            self.windows.window_icon_overrides.clear();
            self.windows.window_title_overrides.clear();
            self.dock.dock_instance_owner = None;
            self.dock.dock_input_identity = None;
        }
    }

    /// 🪟️ The host-owned fields every refresh restamps onto the session's view state — including the tree windows this shell's
    /// observer measured (`tree_windows`/`tree_viewport_rows`, [`tree_windows::TreeWindowScheduler::view_state_fields`]), the
    /// same pair React's `TreeWindowHostV1::viewStateFields` stamps (`🛠️ShellHelpers/🟦️.tsx`).
    pub fn live_view_state(&self, session: &ActiveSession) -> ViewModel {
        let mut view_state = session.view_state.clone();
        view_state.locale = self.prefs.active_locale();
        view_state.terminology = self.prefs.active_terminology();
        let spawned = self.docs.spawned_session.as_ref().is_some_and(|owner| same_session(owner, session));
        if !spawned {
            view_state.window_instances = session_window_instances(session, &self.dock.dock);
        }
        view_state.active_utility_by_window_id = self.windows.active_utility_by_window.clone();
        view_state.tool_run_trace_cursor_by_window_id = infinite_world::world::world3d_tool_run_trace_cursors(self.surfaces.world3d_states.values());
        view_state.tool_run_trace_cursor_by_window_id.extend(semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::board2d_tool_run_trace_cursors());
        if !spawned {
            view_state.focused_window_id = self.windows.active_window_id.clone();
        }
        view_state.session_identity = self.directory.session_identity_view();
        (view_state.tree_windows, view_state.tree_viewport_rows) = self.dock.tree_windows.view_state_fields();
        view_state
    }

    /// 📯️ Releases each transfer journal exactly once only after its required retained body exists.
    /// Each refresh visits the cohort present at entry once, so a refused body retains its own token
    /// without blocking a later ready body. The retry ceiling retires both owners with one fault.
    pub fn complete_window_topology_refresh(&mut self) {
        let live: Vec<String> = self.dock.dock.window_instances().into_iter().map(|(window_id, _)| window_id).collect();
        let cohort = self.windows.window_topology_publications.len();
        for _ in 0..cohort {
            let Some(mut publication) = self.windows.window_topology_publications.pop_front() else {
                break;
            };
            if self.docs.window_ui.contains_key(&publication.window_id) {
                if let Some(token) = publication.journal_token {
                    match self.windows.take_window_topology_action(token) {
                        Some(action) => match action.into_descriptor() {
                            Ok(action) => {
                                self.settle.deferred_actions.push(action);
                                self.windows.window_topology_journal_dispatch_owed = true;
                            }
                            Err(error) => self.prefs.record_surface_fault(&publication.window_id, &publication.body_key, format!("window topology journal decode refused: {error:?}").into(), &mut self.chrome, &mut self.plugins),
                        },
                        None => self.prefs.record_surface_fault(&publication.window_id, &publication.body_key, "window topology journal owner was absent".into(), &mut self.chrome, &mut self.plugins),
                    }
                }
                continue;
            }
            if !live.iter().any(|window_id| window_id == &publication.window_id) {
                self.windows.discard_window_topology_journal(&publication, &mut self.chrome, &mut self.plugins, &mut self.prefs);
                continue;
            }
            publication.attempts = publication.attempts.saturating_add(1);
            if publication.attempts < WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS {
                self.windows.window_topology_publications.push_back(publication);
                continue;
            }
            self.windows.discard_window_topology_journal(&publication, &mut self.chrome, &mut self.plugins, &mut self.prefs);
            self.prefs.record_surface_fault(&publication.window_id, &publication.body_key, format!("initial window body publication refused after {} attempts; topology journal retired without dispatch", WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS).into(), &mut self.chrome, &mut self.plugins);
        }
        self.windows.window_topology_refresh_owed = !self.windows.window_topology_publications.is_empty();
        if self.windows.window_topology_refresh_owed {
            self.settle.owe_refresh(UiDirtyScope::Full);
            self.settle.owe_settle();
        } else if !self.settle.deferred_actions.is_empty() {
            self.settle.owe_settle();
        }
    }

    /// 🛍️ Fetches the app-static operator/palette catalogue once per app instance and pushes it into
    /// every live node-graph engine host, so the canvas spotlight and the full palette have the same
    /// operator set the React shell threads through `AppCatalogueContext`.
    ///
    /// The catalogue rides the reserved `framework.section.catalogue` retained surface, which is the
    /// ordinary `SurfaceVisible` → `AdvanceRetained` protocol every window body already uses — no
    /// second channel, and no `UiDirtyScope` concept the native shell does not have. A fetch that
    /// fails leaves the previous catalogue in place rather than blanking the palette.
    ///
    /// ⚠️ The instance is marked ATTEMPTED before the fetch, not after a successful one: the refresh
    /// that calls this runs on every settled command, so recording only successes turned one failed
    /// catalogue into a re-fetch per refresh. The second such fetch never returned from its own
    /// `submit_turn`, which wedged `boot_shell` itself and took every later pointer event with it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md`). "Once
    /// per app instance" is what the first line of this doc promises; the failure path now keeps it.
    pub async fn refresh_app_catalogue(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, semio_framework_os_renderer_wgpu_program_bridge::ProgramFault>>) {
        let body_key = semio_framework::UiRefreshSection::Catalogue.body_key();
        let fetched = match read_ahead {
            Some(fetched) => fetched,
            None if claim_app_catalogue_fetch(&mut self.plugins.app_catalogue_instance, instance_id) => program.render_with_document(instance_id, body_key, body_key, view_state, None, None).await,
            None => {
                self.plugins.publish_app_catalogue(&self.surfaces);
                return;
            }
        };
        let document = match fetched {
            Ok(document) => document,
            Err(error) => {
                let detail = self.prefs.note_refused_open(error, &mut self.chrome);
                debug_log(&format!("[TRACE] wgpu shell app catalogue fetch failed: {detail}"));
                return;
            }
        };
        let payload = document.read_paged_text();
        if let Err(document) = self.docs.retain_document_for_close(document) {
            drop(document);
        }
        match payload {
            Ok(payload) => {
                debug_log(&format!("[TRACE] wgpu shell app catalogue ready bytes={}", payload.len()));
                self.plugins.app_catalogue_json = payload;
                self.plugins.publish_app_catalogue(&self.surfaces);
            }
            Err(error) => debug_log(&format!("[TRACE] wgpu shell app catalogue reassembly failed: {error:?}")),
        }
    }
}

/// 🪪️ Session equality follows the plugin's exact instance and declared app.
pub fn same_session(left: &ActiveSession, right: &ActiveSession) -> bool {
    left.plugin_id == right.plugin_id && left.instance_id == right.instance_id && left.app.id == right.app.id
}

pub fn panel_identifier(value: &str) -> bool {
    !value.is_empty() && value.chars().count() <= 256 && !value.chars().any(|character| character <= '\u{1f}' || character == '\u{7f}')
}

pub fn validate_panel_state(state: &SpacePanelState) -> Result<(), String> {
    if !panel_identifier(&state.active_panel_tab) {
        return Err("host-panel.invalid-active-tab".into());
    }
    if state.spawned_apps.len() > 64 {
        return Err("host-panel.spawned-capacity".into());
    }
    let mut ids = std::collections::BTreeSet::new();
    for spawned in &state.spawned_apps {
        if !panel_identifier(&spawned.id) || !panel_identifier(&spawned.plugin_id) || !panel_identifier(&spawned.app_id) {
            return Err("host-panel.invalid-spawned-identity".into());
        }
        if !ids.insert(spawned.id.as_str()) {
            return Err("host-panel.duplicate-spawned-identity".into());
        }
        if spawned.label.chars().count() > 65_536 || spawned.breadcrumb.len() > 64 || spawned.breadcrumb.iter().any(|part| part.chars().count() > 65_536) {
            return Err("host-panel.spawned-text-capacity".into());
        }
    }
    if state.active_spawned_id.as_ref().is_some_and(|id| !panel_identifier(id) || !ids.contains(id.as_str())) {
        return Err("host-panel.invalid-active-spawned".into());
    }
    Ok(())
}

pub fn panel_state_from_view(view_state: &ViewModel) -> Result<Option<SpacePanelState>, String> {
    let Some(json) = view_state.panel_json.as_ref() else {
        return Ok(None);
    };
    if json.chars().count() > 65_536 {
        return Err("host-panel.json-capacity".into());
    }
    let state: SpacePanelState = serde_json::from_str(json).map_err(|_| "host-panel.invalid-json".to_string())?;
    validate_panel_state(&state)?;
    Ok(Some(state))
}

pub fn panel_json(state: &SpacePanelState) -> Result<String, String> {
    validate_panel_state(state)?;
    let json = serde_json::to_string(state).map_err(|_| "host-panel.encode".to_string())?;
    if json.chars().count() > 65_536 {
        return Err("host-panel.json-capacity".into());
    }
    Ok(json)
}

/// 👁️✏️ The boot-resolved surface role, as `AppRole`. The renderer keeps it as the UI crate's
/// `ChromeRole` (that is the type its chrome reads); the boot app selection needs the manifest's
/// own spelling, and the two are wire-identical by contract (`"viewer"`/`"editor"`).
pub fn boot_app_role() -> semio_framework::manifest::AppRole {
    match semio_framework_os_renderer_wgpu_host_base::boot_app_role() {
        ui_wgpu::wgpu::component::role_chrome::ChromeRole::Viewer => semio_framework::manifest::AppRole::Viewer,
        ui_wgpu::wgpu::component::role_chrome::ChromeRole::Editor => semio_framework::manifest::AppRole::Editor,
    }
}

pub fn declare_boot_subphase(phase: &str, state: &str, elapsed_ms: f64) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let global = js_sys::global();
        if let Ok(func) = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("semioDeclareBootSubphase")) {
            if let Ok(func) = func.dyn_into::<js_sys::Function>() {
                let _ = func.call3(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(phase), &wasm_bindgen::JsValue::from_str(state), &wasm_bindgen::JsValue::from_f64(elapsed_ms));
            }
        }
    }
    let _ = (phase, state, elapsed_ms);
}

pub fn app_owns_command(app: &semio_framework::AppDefinition, command_id: &str) -> bool {
    app.commands.iter().any(|command| command.id == command_id)
}

/// 🪟️ The window roster a `ViewModel` carries — the Rust twin of React's `sessionWindowInstances`
/// (`🛠️ShellHelpers/🟦️.tsx:4927`): EVERY window kind the open app declares, then the live dock
/// instances that are not one of those kinds (a spawned or duplicated pane).
///
/// 🩸️ It used to be the dock alone. `focused_window_id` is seeded from `app.window_kinds.first()`
/// and a mode layout need not place that kind at all, so in generate mode the roster held only
/// `generation3d-{generations,generate-form,generate-preview}` while the focus named
/// `procedural-main` — and `WindowConfigOwnerRegistry::capture` (`🔌️plugin/🪟️window/🎚️config/🦀️.rs:541`)
/// answers a roster miss with `window-config.window-context`. That fault aborted the whole
/// `setContributions` crossing, so the guest's flow-extension registry stayed EMPTY and every
/// generate-mode preview faulted `flow.extension-not-contributed {"extensionId":"brep"}` with
/// `meshesHead="[]"` forever — measured verbatim on 6118
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-journey-probe-2026-09-13.md` §3).
pub fn session_window_instances(session: &ActiveSession, dock: &DockState) -> Vec<semio_framework::ViewWindowInstance> {
    let mut instances: Vec<semio_framework::ViewWindowInstance> = session.app.window_kinds.iter().map(|kind| semio_framework::ViewWindowInstance { id: kind.id.clone(), window_kind_id: kind.id.clone() }).collect();
    for (id, window_kind_id) in dock.window_instances() {
        if instances.iter().any(|instance| instance.id == id) || !session.app.window_kinds.iter().any(|kind| kind.id == window_kind_id) {
            continue;
        }
        instances.push(semio_framework::ViewWindowInstance { id, window_kind_id });
    }
    instances
}

//#endregion ShellLifecycle

