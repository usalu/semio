//! 🔁️ Deferred actions and menus: the deferred action flush, the ui chain settling, the settle pump steps, the owed refresh and the context menu. See [`super`].

use super::*;

//#region ShellInput
impl<'ctx> PointerCtx<'ctx> {
    /// 🔁️ Drains the deferred halves AND re-enters the window bodies until nothing new is armed.
    ///
    /// ⚖️ A guest's preview job is driven by its own window render, so a chain has THREE halves, not
    /// two: the actions an effect deferred, the extension calls those actions asked for, and the
    /// RENDER that turns a settled answer into the next `flowEvalTick`. Draining only the first two
    /// parks the chain the moment the work one render armed quiesces — `meshingFaces 6/8` on 6118.
    /// React converges because a mutation re-renders the shell; this is that, bounded
    /// (`SHELL_SETTLE_ROUNDS`) and reentrancy-guarded so a nested flush stays a plain drain.
    pub async fn flush_deferred_actions(&mut self) -> Result<(), String> {
        if self.settle.settling {
            self.drain_deferred_actions().await?;
            return Ok(());
        }
        self.settle.settling = true;
        let result = self.settle_ui_chain().await;
        self.settle.settling = false;
        result
    }

    /// ♻️ One settle round = drain everything armed, then re-enter the window bodies ONCE. A round
    /// whose drain executed nothing is the fixed point: no render can arm what no work produced, so
    /// the loop stops without spending a refresh (which is also why a bare pointer move, whose flush
    /// executes nothing, never pays for one).
    ///
    /// 🐢️ The round's own pass asks for NOTHING of its own: every action it drained already
    /// declared what it dirtied (`owe_refresh`), and a round whose work dirtied nothing is a
    /// round that owes no render. This used to be an unconditional whole-shell refresh — the
    /// dominant half of the 116-of-137 `patched=0` renders per converging edit
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7).
    pub async fn settle_ui_chain(&mut self) -> Result<(), String> {
        for round in 0..SHELL_SETTLE_ROUNDS {
            let worked = self.drain_deferred_actions().await?;
            if worked == 0 {
                if round > 0 {
                    debug_log(&format!("[TRACE] wgpu-shell ui chain settled after {round} round(s){}", resident_census()));
                }
                return Ok(());
            }
            if self.docs.session.is_none() {
                return Ok(());
            }
            self.as_window_actions().refresh_ui(UiDirtyScope::None).await?;
        }
        debug_log(&format!("[TRACE] wgpu-shell ui chain exhausted {SHELL_SETTLE_ROUNDS} settle round(s) with {} action(s) left", self.settle.deferred_actions.len()));
        self.drain_deferred_actions().await?;
        Ok(())
    }

    /// 🪟️ The one place every activation site converges, whoever wrote `active_window_id` — a dock
    /// tab select, a retained body press, a pane chip, a palette `window:` row or a keybinding's
    /// own owner activation. React reaches the same point through `Mode`'s single
    /// `onActiveWindowChange` callback; this renderer has no such callback, so the change is
    /// WITNESSED here rather than announced by each writer (see `arm_window_activation_note`).
    ///
    /// 🔁️ ONE convergence loop for the two halves of a guest chain: the actions an effect
    /// deferred, and the extension calls those actions asked for. An answer commonly arms the next
    /// hop (`flowEvalTick` → evaluate → `flowEvalResolve` → tessellate → `flowTessellateResolve`),
    /// so draining either half once leaves the chain parked — which is exactly what a single-pass
    /// flush plus a detached extension task did (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🧯 One program action's failure is that action's failure, never the shell's: React's
    /// `applyHostEffects` logs a dropped dispatch and carries on, and this loop runs inside
    /// `settle_boot`, where a propagated error aborted the whole boot
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn drain_deferred_actions(&mut self) -> Result<usize, String> {
        self.as_window_actions().arm_window_activation_note();
        self.windows.arm_window_blur_host_event(&mut self.settle, &self.plugins, &self.docs);
        self.as_window_actions().arm_host_window_blur();
        let mut worked = self.step_import_transfers().await;
        for _ in 0..SHELL_DEFERRED_CHAIN_ROUNDS {
            let actions = std::mem::take(&mut self.settle.deferred_actions);
            #[cfg(target_arch = "wasm32")]
            let invocations = std::mem::take(&mut self.plugins.pending_extension_invocations);
            #[cfg(not(target_arch = "wasm32"))]
            let invocations: Vec<()> = Vec::new();
            #[cfg(target_arch = "wasm32")]
            let file_opens = std::mem::take(&mut self.opening.pending_file_opens);
            #[cfg(not(target_arch = "wasm32"))]
            let file_opens: Vec<()> = Vec::new();
            if actions.is_empty() && invocations.is_empty() && file_opens.is_empty() {
                break;
            }
            worked = worked.saturating_add(actions.len()).saturating_add(invocations.len()).saturating_add(file_opens.len());
            for action in actions {
                let id = action.action.clone();
                if let Err(error) = self.as_actions().dispatch_action(action).await {
                    debug_log(&format!("[TRACE] wgpu-shell deferred action {id} failed: {error}"));
                }
            }
            #[cfg(target_arch = "wasm32")]
            for invocation in invocations {
                self.run_extension_invocation(invocation).await;
            }
            #[cfg(target_arch = "wasm32")]
            for request in file_opens {
                self.opening.run_file_open_request(request, &self.docs).await;
            }
        }
        if !self.settle.deferred_actions.is_empty() {
            debug_log(&format!("[TRACE] wgpu-shell deferred chain exhausted {SHELL_DEFERRED_CHAIN_ROUNDS} rounds with {} action(s) left", self.settle.deferred_actions.len()));
        }
        if self.opening.pending_shell_uri_apply {
            self.opening.pending_shell_uri_apply = false;
            self.apply_pending_shell_uri().await?;
            worked = worked.saturating_add(1);
        }
        Ok(worked)
    }

    /// 🫀️ Whether the frame loop owes this shell a settle step. Derived, never a latch alone: armed
    /// work, an owed refresh scope and a producer that reports `computing` each keep the pump running
    /// on their own, so a step that forgot to re-arm cannot silently park a live chain.
    ///
    /// ⚠️ A synchronous flush (`flush_deferred_actions`, still the door a native/winit host and the
    /// tests use) owns the chain while it runs; the pump stands aside rather than crossing into the
    /// same guest from two places.
    ///
    /// 🏁️ A producer the watchdog has stood down from is NOT "computing" as far as the frame loop
    /// is concerned: its witness is frozen, crossings were measured not to move it, and believing
    /// its own flag would pin the host at one settle step per frame forever — which the served
    /// generation3d preview does, publishing `computing:true` on every sample of a 140 s window in
    /// which nothing evaluated at all (`📓️wgpu-progress-visibility-2026-09-14.md` §3.1).
    pub fn settle_pump_pending(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        let parked = !self.plugins.pending_extension_invocations.is_empty() || !self.opening.pending_file_opens.is_empty();
        #[cfg(not(target_arch = "wasm32"))]
        let parked = false;
        let armed_work = !self.settle.deferred_actions.is_empty() || self.opening.pending_shell_uri_apply || parked || host_window_blur_pending() || !self.opening.import_transfers.is_empty();
        let computing = self.surfaces.live_compute_surfaces().any(|(surface_id, _)| !self.settle.settle_pump.watches.get(&surface_id).is_some_and(|watch| watch.standing));
        self.opening.icon_export.as_ref().is_some_and(IconExportBatch::running) || semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() || settle_pump_owes(self.settle.settling, self.docs.session.is_some(), armed_work, &self.settle.owed_refresh_scope, self.settle.settle_pump.owed, computing)
    }

    /// 🫀️ ONE settle step, and the whole of what the frame loop asks of this shell per frame.
    ///
    /// The order is the lane's own law: a topology refresh that publishes a new window body first,
    /// then armed work, and a crossing only when nothing is armed. Ordinary work still takes the
    /// refresh pass it owed and stops there — the next frame takes the next step, which is what lets
    /// the chrome, the GPU present and the status pill stay live through a convergence that used to
    /// run to a fixed point inside `boot_shell`.
    ///
    /// A step that found nothing armed is the interesting one: the guest is off computing, and on this
    /// renderer a background solve only advances on a host crossing, so the step spends exactly one —
    /// scoped to the window bodies whose producers report `computing`, never the whole shell. That is
    /// what pays for [`Self::refresh_ui`] honouring `UiDirtyScope` without withdrawing compute from a
    /// solve the UI has nothing to do with.
    ///
    /// 🩺️ One line per CLASS transition plus a heartbeat, never one per step: `Drained` and
    /// `Crossed` alternate every frame of a converging edit and a line apiece would bury the
    /// console the probes read.
    pub async fn settle_pump_step(&mut self) -> ShellSettleStep {
        self.as_session().pump_tree_windows();
        let step = self.settle_pump_step_inner().await;
        let entering = matches!(step, ShellSettleStep::Wedged | ShellSettleStep::Quiescent) || matches!(self.settle.settle_pump.traced, None | Some(ShellSettleStep::Quiescent));
        if (entering && self.settle.settle_pump.traced != Some(step)) || self.settle.settle_pump.steps % SHELL_SETTLE_HEARTBEAT_STEPS == 0 {
            self.settle.settle_pump.traced = Some(step);
            debug_log(&format!("[TRACE] wgpu-shell settle pump {}", serde_json::json!({ "step": format!("{step:?}"), "steps": self.settle.settle_pump.steps, "crossings": self.settle.settle_pump.crossings, "watching": self.settle.settle_pump.watches.len() })));
        }
        step
    }

    /// 🪟️ The first body of a newly created window is a prerequisite for its own journal note.
    /// Once a cohort refresh releases any ready peer, that peer dispatches before the next retry
    /// refresh; an unrelated refusing body cannot head-of-line block its acknowledged transfer.
    pub async fn settle_pump_step_inner(&mut self) -> ShellSettleStep {
        self.settle.settle_pump.steps = self.settle.settle_pump.steps.saturating_add(1);
        if self.docs.session.is_none() {
            self.settle.settle_pump.owed = false;
            return ShellSettleStep::Quiescent;
        }
        if self.windows.window_topology_refresh_owed && !self.windows.window_topology_journal_dispatch_owed {
            if let Err(error) = self.as_window_actions().refresh_ui(UiDirtyScope::None).await {
                self.settle.owe_refresh(UiDirtyScope::Full);
                debug_log(&format!("[TRACE] wgpu-shell window topology refresh failed: {error}"));
            }
            self.settle.settle_pump.owed = true;
            return ShellSettleStep::Drained;
        }
        let worked = match self.drain_deferred_actions().await {
            Ok(worked) => worked,
            Err(error) => {
                debug_log(&format!("[TRACE] wgpu-shell settle pump drain failed: {error}"));
                0
            }
        };
        self.windows.window_topology_journal_dispatch_owed = false;
        if worked > 0 || !self.settle.owed_refresh_scope.asks_for_nothing() || self.settle.settle_pump.rendering.is_some() {
            if !self.settle.owed_refresh_scope.asks_for_nothing() || self.settle.settle_pump.rendering.is_some() {
                self.advance_owed_refresh().await;
            }
            self.settle.settle_pump.owed = true;
            return ShellSettleStep::Drained;
        }
        self.settle_pump_cross().await
    }

    /// 🖼️ The settle lane's owed refresh without holding the shell across a guest turn: the first step
    /// hands every guest read the owed scope names — its bodies and its reserved sections — to one detached
    /// read ([`render_refresh_detached`]) and returns; a later step, once they answered, runs the refresh
    /// with those documents ([`Self::refresh_ui_rendered`]), which then only applies them. Before, the
    /// whole refresh ran inside one step: every read held the frame build — 3.5 s per full refresh of a
    /// block2d session in a debug build, then still ~0.75 s for the three section reads alone (ticket
    /// 26/09/23 slice WG8).
    pub async fn advance_owed_refresh(&mut self) {
        if let Some(rendering) = self.settle.settle_pump.rendering.as_ref() {
            let Some(rendered) = rendering.take() else { return };
            self.settle.settle_pump.rendering = None;
            if let Err(error) = self.as_window_actions().refresh_ui_rendered(UiDirtyScope::None, Some(rendered)).await {
                debug_log(&format!("[TRACE] wgpu-shell settle pump refresh failed: {error}"));
            }
            return;
        }
        let Some(session) = self.docs.session.clone() else {
            if let Err(error) = self.as_window_actions().refresh_ui(UiDirtyScope::None).await {
                debug_log(&format!("[TRACE] wgpu-shell settle pump refresh failed: {error}"));
            }
            return;
        };
        let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        self.docs.drain_retained_document_arenas();
        self.as_session().sync_dock();
        let scope = self.settle.owed_refresh_scope.clone();
        let view_state = self.as_session().live_view_state(&session);
        let mut jobs = Vec::new();
        for (window_id, window_kind_id) in self.dock.dock.window_instances() {
            let Some(kind) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id) else { continue };
            if !scope.wants_window_body(&kind.body_key) {
                continue;
            }
            if let Some(window_view) = view_state.for_window_instance(&window_id) {
                jobs.push((window_id, kind.body_key.clone(), window_view));
            }
        }
        let panel_view = view_state.for_panel();
        for tab in flatten_panel_tab_leaves(&session.app.panel_tabs) {
            if let Some(body_key) = tab.body_key.as_deref().filter(|body_key| scope.wants_panel_body(body_key)) {
                jobs.push((tab.id().to_string(), body_key.to_string(), panel_view.clone()));
            }
        }
        let sections = ShellRefreshSections {
            catalogue: claim_app_catalogue_fetch(&mut self.plugins.app_catalogue_instance, session.instance_id),
            engagements: scope.wants_section(UiDirtySection::Engagements),
            measures: scope.wants_section(UiDirtySection::Measures),
            tools: scope.wants_section(UiDirtySection::Tools) && !self.docs.tool_panel_tabs(&self.prefs).is_empty(),
        };
        self.settle.settle_pump.rendering = Some(ShellDetached::spawn(render_refresh_detached(program, session.instance_id, jobs, sections, panel_view, view_state)));
    }

    pub async fn open_context_menu(&mut self, x: f32, y: f32, hit: Option<HitTarget<ActionDescriptor>>) {
        let node_id = hit.as_ref().and_then(|hit| hit.control_id.as_deref().and_then(|id| id.rsplit_once(".node.").map(|(_, node_id)| node_id.to_string())));
        let edge_id = hit.as_ref().and_then(|hit| hit.control_id.as_deref().and_then(|id| id.rsplit_once(".edge.").map(|(_, edge_id)| edge_id.to_string())));
        let ContextMenuSurfaceResolution { surface_id, kind, hits, selection: selection_groups, text: text_context } = self.input.resolve_context_menu_surface(x, y, node_id.as_deref(), edge_id.as_deref(), &self.surfaces);
        let window_instance_id = self.input.context_window_instance_id(x, y).map(str::to_string);
        let is_de = self.prefs.locale_id == "de";
        let mut items = Vec::new();
        if let Some(session) = self.docs.session.clone() {
            let shortcut_by_action: HashMap<String, String> = session.app.keybindings.iter().map(|binding| (binding.action.action.clone(), binding.keys.clone())).collect();
            let view_state = self.as_session().live_view_state(&session);
            let selection = selection_groups.clone();
            let text: Option<Value> = text_context.as_ref().and_then(|text| serde_json::to_value(text).ok());
            let request = serde_json::json!({
                "menu": { "id": kind.clone() },
                "viewState": view_state,
                "windowInstanceId": window_instance_id,
                "surface": {
                    "surfaceId": surface_id,
                    "kind": kind.clone(),
                    "hits": hits,
                    "selection": selection,
                    "text": text,
                },
                "point": { "x": x as f64, "y": y as f64 },
            });
            if let Some(program) = self.plugins.plugins.iter().find(|plugin| plugin.plugin_id == session.plugin_id) {
                match program.context_menu(session.instance_id, request).await {
                    Ok(specs) => {
                        items = specs
                            .into_iter()
                            .map(|spec| {
                                let mut item = shell_context_menu_item_from_spec(spec, &session.app.controller_id, is_de);
                                if item.shortcut.is_none() {
                                    if let Some(action) = item.action.as_ref() {
                                        item.shortcut = shortcut_by_action.get(&action.action).map(|keys| format_keybinding_shortcut(keys));
                                    }
                                } else if let Some(shortcut) = item.shortcut.as_ref() {
                                    item.shortcut = Some(format_keybinding_shortcut(shortcut));
                                }
                                item
                            })
                            .collect();
                    }
                    Err(error) => {
                        self.chrome.error = Some(error);
                    }
                }
            }
        }
        if items.is_empty() {
            if let Some(session) = &self.docs.session {
                let window_kind = window_instance_id.as_deref().and_then(|window_id| self.dock.live_window_kind_id(session, window_id)).and_then(|window_kind_id| session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id));
                let actions: Vec<semio_framework_ui_declaration::layout::ShellMenuAction> = window_kind
                    .map(|kind| semio_framework::resolve_window_actions(&session.app, kind))
                    .unwrap_or_default()
                    .into_iter()
                    .map(|action| semio_framework_ui_declaration::layout::ShellMenuAction {
                        id: action.id.clone(),
                        label: action.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string(),
                        icon: Some(action.icon_id.as_str().to_string()),
                        keys: action.keys.clone(),
                        kind: context_menu_action_kind_str(action.kind),
                        category: action.category.clone(),
                        in_palette: action.in_palette,
                        arg_carrying: !action.args.is_empty(),
                    })
                    .collect();
                let controller_id = session.app.controller_id.clone();
                items = semio_framework_ui_declaration::layout::build_shell_context_menu_specs(&actions, true).into_iter().map(|spec| shell_context_menu_item_from_spec(spec, &controller_id, is_de)).collect();
            }
        }
        if let Some(window_id) = window_instance_id.as_deref() {
            scope_context_menu_items(&mut items, window_id);
        }
        if let Some(controller_id) = self.plugins.host_controller_id() {
            items.push(ContextMenuItem {
                id: "shell.context.home".into(),
                label: shell_chrome_string("contextMenu.goHome", is_de).to_string(),
                icon: None,
                destructive: false,
                action: Some(ActionDescriptor { controller_id, action: "goHome".into(), args: None }),
                ..Default::default()
            });
        }
        self.input.context_menu = Some(ContextMenuState { x, y, items, active: Vec::new(), submenu_collapsed_at: None, scroll_offset: 0.0 });
        self.input.overlay_state = OverlayState::None;
    }

}
//#endregion ShellInput
