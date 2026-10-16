//! 🖐️ `shell-pointer` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

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
        self.opening.icon_export.as_ref().is_some_and(icon_export::IconExportBatch::running) || semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending() || settle_pump_owes(self.settle.settling, self.docs.session.is_some(), armed_work, &self.settle.owed_refresh_scope, self.settle.settle_pump.owed, computing)
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
                let actions: Vec<ui_wgpu::wgpu::ShellMenuAction> = window_kind
                    .map(|kind| semio_framework::resolve_window_actions(&session.app, kind))
                    .unwrap_or_default()
                    .into_iter()
                    .map(|action| ui_wgpu::wgpu::ShellMenuAction {
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
                items = ui_wgpu::wgpu::build_shell_context_menu_specs(&actions, true).into_iter().map(|spec| shell_context_menu_item_from_spec(spec, &controller_id, is_de)).collect();
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

    /// 🪟️ Escape closes exactly the topmost overlay first — matches ui_wgpu's overlay-manager
    /// precedence (`EventRouter::close_topmost_overlay`, report-w1d-events-overlay.md: "Escape
    /// closes only the topmost") even though these ad-hoc chrome overlays predate that stack and
    /// aren't routed through it yet. A Select dropdown is the most local/transient overlay, so it
    /// wins over the context menu; neither used to close on Escape at all before this fix.
    ///
    /// 🎓️ Tour keys take precedence over every other chord below (mirrors Escape closing the topmost
    /// overlay above) and — unlike every other chord in this function — fire even while a field has
    /// focus: React binds all three with `enableOnFormTags: true` (`UIIntroduction`), because a veil
    /// that owns every pointer must always be answerable from the keyboard. The sync-attach card
    /// still outranks them, since it is the one overlay painted above the tour.
    ///
    /// ⌨️ Hardcoded shell chords never fire while a text field (or the sync-attach draft buffer)
    /// has focus — matches `os-shell.tsx`'s `isEditableEventTarget`/`useActionHotkey` (backed by
    /// react-hotkeys-hook, which by default does not fire on form tags): "hotkeys never fire while
    /// the user is typing". Previously these six chords fired unconditionally, so e.g. Ctrl+B while
    /// typing in a focused Input would silently toggle the left panel instead of inserting "b".
    ///
    /// 🛡️ Every chord is now one row of `SHELL_SHORTCUT_ROWS` (the transcription of React's
    /// `SHELL_KEYBINDINGS`) rather than an open-coded `meta && Char("b")` ladder: the palette/find
    /// toggles, the three nav verbs, all eight panel anchors and the three window verbs resolve
    /// through the same table `is_reserved_shell_chord` reserves, so a chord can no longer be
    /// reserved-but-dead (which is exactly how the six missing panel anchors and the three window
    /// verbs read before this wave).
    ///
    /// ⌨️ An ARMED Keybindings row swallows the next key event and becomes that row's override —
    /// React's `chordFromKeyboardEvent` on the capture button, ahead of every other rung so a chord
    /// being remapped never also fires its own verb.
    ///
    /// ⌨️ Tab/Shift+Tab walks the keyboard ring ([`Self::keyboard_ring`]) whenever nothing else claims
    /// focus: every focusable chrome control, then every dock window. This renderer has no DOM, so
    /// before this the chrome had no keyboard order at all and Tab only cycled windows. Content-level
    /// Tab among a window's own widgets stays the retained content's (`EventRouter`), routed above.
    ///
    /// 📌️ ticket §C5 item 3 — same shell-owned keyboard-routed draft-field idiom as
    /// `sync_card_kind` above, for the check-in message-prompt card.
    pub fn handle_keyboard(&mut self, action: ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, input: &mut InputState<ActionDescriptor>) {
        if self.chrome.chrome_build.dialog_open() && self.input.handle_chrome_dialog_key(&action, modifiers, &mut self.chrome, &mut self.settle) {
            return;
        }
        if action == ui_wgpu::wgpu::KeyAction::Escape {
            if semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_interactions(input) {
                if let Some((window_id, pointer_id)) = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_any_pointer_capture() {
                    semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_pointer_event(&window_id, pointer_id, ui_wgpu::wgpu::UiEvent::PointerCancel, input);
                }
                return;
            }
            if semio_framework_os_renderer_wgpu_scene_render::cancel_scene_list_transfer() {
                self.input.chrome_pointer_owner = None;
                return;
            }
            if self.dock.dock_drag.take().is_some() || self.dock.pending_dock_drag.take().is_some() {
                self.input.chrome_pointer_owner = None;
                self.input.pending_chrome_release = None;
                return;
            }
            if self.input.dismiss_dismissable_layers() {
                return;
            }
            if self.input.context_menu.is_some() {
                match self.input.context_menu_handle_key(ui_wgpu::wgpu::KeyAction::Escape) {
                    ContextMenuKeyOutcome::Activate(_) => return,
                    ContextMenuKeyOutcome::Ignored => {}
                    ContextMenuKeyOutcome::Consumed | ContextMenuKeyOutcome::CloseMenu => return,
                }
            }
        }
        if self.sync.sync_card_kind.is_none() {
            if let Some(step) = self.docs.chrome_tour_active_step(&self.chrome) {
                match action {
                    ui_wgpu::wgpu::KeyAction::Escape => {
                        if let Some(seen_key) = self.docs.introduction_seen_key() {
                            self.chrome.chrome_build.dismiss_introduction(&seen_key);
                        } else {
                            self.chrome.chrome_build.skip_introduction();
                        }
                        return;
                    }
                    ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::ArrowRight => {
                        if step.interactions.is_empty() {
                            self.docs.chrome_tour_advance_current_step(&step, &mut self.chrome);
                            return;
                        }
                    }
                    ui_wgpu::wgpu::KeyAction::ArrowLeft => {
                        self.chrome.chrome_build.back_introduction();
                        return;
                    }
                    _ => {}
                }
            }
        }
        let editing = content_is_editing(input.focused_id.as_deref(), self.sync.sync_card_kind.is_some());
        let palette_open = matches!(self.input.overlay_state, OverlayState::Search | OverlayState::Find);
        if self.input.keybinding_capture_control_id.is_some() {
            if self.input.capture_keybinding_chord(&action, modifiers, &mut self.chrome) {
                return;
            }
        }
        if !editing {
            if let Some(shortcut) = shell_shortcut_for_in(&self.chrome.shortcut_table(), &action, modifiers) {
                if self.as_session().apply_sync_shell_shortcut(shortcut, palette_open, input) {
                    return;
                }
            }
        }
        if !editing && !palette_open && self.dock.dock_drag.is_none() && action == ui_wgpu::wgpu::KeyAction::Tab {
            self.advance_keyboard_ring(!modifiers.shift, input);
            return;
        }
        if self.sync.sync_card_kind.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.sync.sync_card_kind = None;
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    self.settle.deferred_actions.push(ActionDescriptor {
                        controller_id: "framework.sync".into(),
                        action: "attach".into(),
                        args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({
                            "path": self.sync.sync_card_draft,
                            "kind": self.sync.sync_card_kind,
                        }),
                    });
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    self.sync.sync_card_draft.push_str(&key);
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    self.sync.sync_card_draft.pop();
                    return;
                }
                _ => {}
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.sync.checkin_dialog_draft.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.sync.checkin_dialog_draft = None;
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    let message = self.sync.checkin_dialog_draft.clone().unwrap_or_default();
                    self.sync.checkin_dialog_draft = None;
                    self.settle.deferred_actions.push(ActionDescriptor { controller_id: "framework.checkin".into(), action: "submit".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "message": message }) });
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    if let Some(draft) = self.sync.checkin_dialog_draft.as_mut() {
                        draft.push_str(&key);
                    }
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    if let Some(draft) = self.sync.checkin_dialog_draft.as_mut() {
                        draft.pop();
                    }
                    return;
                }
                _ => {}
            }
        }
        if palette_open {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.commands.close_command_palette(input, &mut self.input);
                }
                ui_wgpu::wgpu::KeyAction::ArrowDown => {
                    if self.input.overlay_state == OverlayState::Search {
                        let len = self.as_session().filtered_search_items().len();
                        if len > 0 {
                            self.commands.search_selected = (self.commands.search_selected + 1).min(len - 1);
                        }
                    } else {
                        let len = self.commands.filtered_find_items().len();
                        if len > 0 {
                            self.commands.find_selected = (self.commands.find_selected + 1).min(len - 1);
                        }
                    }
                }
                ui_wgpu::wgpu::KeyAction::ArrowUp => {
                    if self.input.overlay_state == OverlayState::Search {
                        self.commands.search_selected = self.commands.search_selected.saturating_sub(1);
                    } else {
                        self.commands.find_selected = self.commands.find_selected.saturating_sub(1);
                    }
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    let runtime = ();
                    let _ = runtime;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    let kind = if self.input.overlay_state == OverlayState::Search { ShellPaletteKind::Search } else { ShellPaletteKind::Find };
                    let mut query = self.commands.palette_query(kind).to_string();
                    query.push_str(&key);
                    self.commands.set_palette_query(kind, query, input, &mut self.input);
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    let kind = if self.input.overlay_state == OverlayState::Search { ShellPaletteKind::Search } else { ShellPaletteKind::Find };
                    let mut query = self.commands.palette_query(kind).to_string();
                    query.pop();
                    self.commands.set_palette_query(kind, query, input, &mut self.input);
                }
                _ => {}
            }
            return;
        }
        if input.focused_id.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    if let Some(ch) = key.chars().next() {
                        input.insert_char(ch);
                    }
                }
                ui_wgpu::wgpu::KeyAction::Backspace => input.backspace(),
                ui_wgpu::wgpu::KeyAction::Delete => input.delete_forward(),
                _ => {}
            }
        }
    }

    /// 📋️⎋️ Dismissing the menu does NOT consume the chord. React's menu is a DOM popover: the
    /// same Escape keydown closes it AND reaches `handleAppKeydown`, so the app's own binding still
    /// fires — the reference journals `engagementAbort` for the `context-menu-dismiss` step
    /// (`🗑️generated/parity-run-2/steps.json`), where this target journalled nothing at all
    /// because `CloseMenu` returned here. `Consumed` is the menu's own navigation (arrows, Enter)
    /// and stays consumed, exactly as React's menu stops propagation for those.
    ///
    /// ⌨️ Enter/Space press the chrome control the keyboard ring focused — through the same accessibility
    /// activation a platform assistive technology and the browser mirror use, so all three press it alike.
    ///
    /// 🩸️ A CONTENT field's Enter/Escape commits it. The shell's own overlay query fields are not
    /// content: claiming Escape here swallowed the quick-search palette's own Escape arm, so the
    /// palette — already unable to close on its opening chord — had no keyboard route out at all.
    ///
    /// ⎋️ A dismissable layer closes on this very keydown and does NOT consume it — see
    /// [`Self::dismiss_dismissable_layers`]. Dismissing here, ahead of `idle`, is what lets the
    /// app's own Escape binding still fire under an open picker, as React's does.
    ///
    /// 👁️✏️🔁️ The `SHELL_SHORTCUT_ROWS` verbs that need the async funnel — the surface-role axis
    /// (`mod+alt+e`/`mod+alt+v`) and the mode-cycle axis (`mod+alt+→`/`mod+alt+←`). They are
    /// matched here, ahead of every app-declared chord, exactly as
    /// `is_reserved_shell_chord` reserves them. Every other row of the same table is a synchronous
    /// chrome-state transition answered by `handle_keyboard`'s own table dispatch below.
    ///
    /// 🎯️🕹️ Content-focus routing (w2-input-wiring): whenever the active window's retained
    /// content — not chrome — is the one holding focus, real keys belong there via
    /// `interpreter::dispatch_ui_event` (Escape/Tab/edit keys/clipboard chords/text), taking
    /// priority over the idle-Escape-deactivate-utility and app-keybinding dispatch below, both
    /// of which are chrome-level concerns. `content_has_focus` is this module's own best-effort
    /// tracker (see its doc comment for the one documented gap: pointer-click-driven focus
    /// changes, entirely inside the off-limits `interpreter` region, never reach it). `Tab`
    /// reaching `dispatch_event` here is ALSO the full Tab-traversal fix: `events::EventRouter::
    /// dispatch`'s own `KeyDown{key:"Tab"}` arm already calls `FocusState::focus_next`/
    /// `focus_prev` internally — nothing else to wire for that. When content does NOT have
    /// tracked focus, this is a no-operation and `handle_keyboard`'s keyboard ring
    /// (`advance_keyboard_ring`) takes Tab below.
    ///
    /// 🧰️🛠️ Escape deactivates the focused window's active utility (P5) — and, when no utility is
    /// active, the active MODE-LEVEL TOOL. Both rungs and their order are React's
    /// (`🏛️ShellHost/🟦️.tsx` `handleAppKeydown`: utility first, then `SET_ACTIVE_TOOL_ACTION_ID` with
    /// an empty id); the tool rung was missing here, so Escape left an armed tool armed.
    ///
    /// ⌨️ App-declared keybinding dispatch (Architecture Decision 8, P4) — NET-NEW for the wgpu
    /// shell, which previously only handled hardcoded shell chords. Reserved shell chords still
    /// win, now inside `match_app_keybinding` per CHORD exactly as React's own loop reserves them,
    /// so a binding whose second chord is the shell's still fires on its first.
    ///
    /// ⌨️ An app binding that ANSWERED ends the ladder; one whose verb no mounted window owns
    /// does NOT, because React's own keybinding loop `continue`s past an unresolved target and
    /// its `mod+z`/`mod+shift+z` tail still runs. Measured live: after the journey's
    /// `window-cap-close` steps the wgpu dock held zero windows, the app's own `undo`/`redo`/
    /// `escape` bindings all resolved, every one became the unowned-chord banner, and the
    /// framework tail below never ran — so `chord-escape`, `chord-undo` and `chord-redo`
    /// journalled NOTHING where React journals `engagementAbort`, `undo` and `redo`
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`,
    /// `🗑️generated/w12c-parity-run-17/wgpu/console.txt`'s `chord gate … docked=0`).
    ///
    /// 🖐️ A chord is a user gesture exactly as a click is: `mod+o` is the ONLY door the
    /// import picker has, and a browser grants user activation to the task that handled the
    /// key and to nothing after it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🗨️ An open chrome dialog owns the keyboard before every rung below: a focused chrome control, a retained
    /// content field, a shell chord or an app binding never takes a key from a modal — the finalize prompt of a
    /// history edit is operated by keyboard alone ([`Self::handle_chrome_dialog_key`]).
    ///
    /// ⏪️ Framework-universal undo/redo chords — LAST, after the app-keybinding loop, so an app that
    /// declares `mod+z` itself shadows them, exactly as React's `handleAppKeydown` orders its own
    /// `mod+z`/`mod+shift+z`/`mod+y` tail. Routed through the same `dispatch_action` funnel the
    /// History panel's Undo/Redo rows and the palette's `studio.undo`/`studio.redo` entries use, so
    /// the three doors to the verb cannot diverge. Before this wave the wgpu shell had NO undo chord
    /// at all: undo was reachable only by opening the palette and typing "undo".
    pub async fn handle_keyboard_async(&mut self, action: ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        if self.chrome.chrome_build.dialog_open() {
            self.handle_keyboard(action, modifiers, input);
            self.settle.owe_settle();
            return Ok(());
        }
        if self.input.context_menu.is_some() {
            match self.input.context_menu_handle_key(action.clone()) {
                ContextMenuKeyOutcome::Ignored | ContextMenuKeyOutcome::CloseMenu => {}
                ContextMenuKeyOutcome::Consumed => return Ok(()),
                ContextMenuKeyOutcome::Activate(descriptor) => {
                    self.as_actions().dispatch_action(descriptor).await?;
                    return Ok(());
                }
            }
        }
        if matches!(action, ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Space(true)) {
            if let Some(target) = self.input.keyboard_focused_chrome_target(input, &self.chrome) {
                self.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, input).await?;
                return Ok(());
            }
        }
        if matches!(self.input.overlay_state, OverlayState::Search) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_search_item(self.commands.search_selected).await?;
            input.blur_input();
            self.input.accessibility_focused_control_id = None;
            return Ok(());
        }
        if matches!(self.input.overlay_state, OverlayState::Find) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_find_item(self.commands.find_selected).await?;
            input.blur_input();
            self.input.accessibility_focused_control_id = None;
            return Ok(());
        }
        if content_is_editing(input.focused_id.as_deref(), false) {
            match action {
                ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Escape => {
                    self.commit_focused_input(input).await?;
                    return Ok(());
                }
                _ => {}
            }
        }
        if action == ui_wgpu::wgpu::KeyAction::Escape {
            self.input.dismiss_dismissable_layers();
        }
        let idle = input.focused_id.is_none() && self.input.overlay_state == OverlayState::None && self.sync.sync_card_kind.is_none() && self.dock.dock_drag.is_none();
        debug_log(&format!(
            "[TRACE] wgpu-shell chord gate action={action:?} idle={idle} focused={:?} overlay={:?} syncCard={} dockDrag={} menu={} window={:?} session={:?} editVerb={:?} appBinding={:?} reserved={} docked={}",
            input.focused_id,
            self.input.overlay_state,
            self.sync.sync_card_kind.is_some(),
            self.dock.dock_drag.is_some(),
            self.input.context_menu.is_some(),
            self.windows.active_window_id,
            self.docs.session.as_ref().map(|session| session.app.controller_id.clone()),
            shell_edit_verb_for(&action, modifiers).map(ShellEditVerb::action_id),
            self.docs.match_app_keybinding(&action, modifiers, &self.chrome).map(|descriptor| descriptor.action),
            is_reserved_shell_chord_in(&self.chrome.shortcut_table(), &action, modifiers),
            self.dock.dock.window_instances().len()
        ));
        if idle {
            if let Some(shortcut) = shell_shortcut_for_in(&self.chrome.shortcut_table(), &action, modifiers) {
                if shortcut.is_async() && self.apply_async_shell_shortcut(shortcut).await? {
                    return Ok(());
                }
            }
        }
        if idle && !is_reserved_shell_chord_in(&self.chrome.shortcut_table(), &action, modifiers) {
            let platform = command_host_platform();
            let command = self.docs
                .resolved_commands(&self.chrome, &self.plugins)
                .into_iter()
                .rev()
                .find(|entry| entry.definition.in_palette && entry.definition.keybindings.iter().any(|binding| binding.platform.is_none_or(|declared| declared == platform) && key_event_matches_chord(&action, modifiers, &binding.chord)));
            if let Some(entry) = command {
                if entry.definition.args.is_empty() {
                    if matches!(entry.address.owner, semio_framework::manifest::CommandOwnerAddress::Os) {
                        self.as_actions().apply_os_command(&entry.definition.id, None).await?;
                    } else {
                        let invocation = semio_framework::manifest::CommandInvocation { address: entry.address, arguments: BTreeMap::new() };
                        self.as_actions().dispatch_command(invocation).await?;
                    }
                } else {
                    self.input.overlay_state = OverlayState::Search;
                    self.commands.search_query = entry.definition.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string();
                    self.commands.search_selected = 0;
                }
                return Ok(());
            }
        }
        if idle {
            if let Some((surface, node)) = self.as_session().retained_keyboard_focus() {
                debug_log(&format!("[TRACE] wgpu-shell key routing surface={surface} node={node:?} action={action:?}"));
                let focused_select = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_node_is_select(&surface, node);
                if let Some(event) = ui_event_from_key_action(&action, modifiers, focused_select) {
                    let commands = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_event(&surface, event, input);
                    self.chrome.chrome_build.note_content_focus_commands(&commands);
                    return Ok(());
                }
            }
        }
        if idle && action == ui_wgpu::wgpu::KeyAction::Escape {
            if let Some(window_id) = self.windows.active_window_id.clone() {
                if self.windows.active_utility_by_window.remove(&window_id).is_some() {
                    self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
                    return Ok(());
                }
            }
            let armed_tool = self.docs.session.as_ref().and_then(|session| session.view_state.active_tool_id.clone()).is_some_and(|tool_id| !tool_id.is_empty());
            if armed_tool {
                let controller_id = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
                self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "setActiveTool".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "toolId": "" }) }).await?;
                return Ok(());
            }
        }
        if idle {
            if let Some(descriptor) = self.docs.match_app_keybinding(&action, modifiers, &self.chrome) {
                if self.dispatch_app_keybinding(descriptor).await? {
                    self.as_actions().drain_gesture_bound_work().await;
                    self.settle.owe_settle();
                    return Ok(());
                }
            }
        }
        if idle {
            if let Some(verb) = shell_edit_verb_for(&action, modifiers) {
                let controller_id = self.docs.session.as_ref().map(|session| session.app.controller_id.clone());
                if let Some(controller_id) = controller_id {
                    debug_log(&format!("[TRACE] wgpu-shell edit chord verb={} controller={controller_id}", verb.action_id()));
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: verb.action_id().into(), args: None }).await?;
                    self.settle.owe_settle();
                    return Ok(());
                }
            }
        }
        self.handle_keyboard(action, modifiers, input);
        self.as_actions().drain_gesture_bound_work().await;
        self.settle.owe_settle();
        Ok(())
    }

    /// ⌨️ Moves keyboard focus one stop along the ring: onto a chrome control (the accessibility focus the
    /// platform adapter and the mirror set, painted as the focus ring) or onto a dock window (the active
    /// window, whose own content focus takes the keyboard again).
    pub fn advance_keyboard_ring(&mut self, forward: bool, input: &mut InputState<ActionDescriptor>) {
        let ring = self.dock.keyboard_ring(&self.chrome);
        if ring.is_empty() {
            return;
        }
        let current = match self.input.accessibility_focused_control_id.as_deref() {
            Some(key) => ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Chrome(control) if control == key)),
            None => self.windows.active_window_id.as_deref().and_then(|active| ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Window(_, window) if window == active))),
        };
        let next = match current {
            Some(index) if forward => (index + 1) % ring.len(),
            Some(index) => (index + ring.len() - 1) % ring.len(),
            None if forward => 0,
            None => ring.len() - 1,
        };
        match ring[next].clone() {
            ShellKeyboardStop::Chrome(key) => {
                input.blur_input();
                self.chrome.chrome_build.focused_retained_surface = None;
                self.input.accessibility_focused_control_id = Some(key);
            }
            ShellKeyboardStop::Window(path, window_id) => {
                self.input.accessibility_focused_control_id = None;
                self.dock.dock.set_stack_active(&path, &window_id);
                self.dock.dock.sync_active_window(&window_id);
                self.windows.active_window_id = Some(window_id);
            }
        }
    }
}

/// ⌨️ Whether the USER is typing into app CONTENT — the one gate the hardcoded shell chords and
/// the focused-input Enter/Escape commit both sit behind. React expresses it as
/// `isEditableEventTarget` (react-hotkeys-hook: "hotkeys never fire while the user is typing").
///
/// 🩸️ It used to be a bare `focused_id.is_some()`, and the chord that OPENS the quick-search
/// palette focuses the palette's own query field — so opening it disabled every keyboard route
/// out of it: `mod+p` no longer toggled (it fell through to the open palette's `Char` arm and
/// typed a literal `p` into the query) and Escape was claimed by the focused-input commit before
/// the palette's own Escape arm could run. The shell's own overlay fields are chrome, not
/// content (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// `Self`-less so its law drives the real body without a `ShellState` fixture.
pub fn content_is_editing(focused_id: Option<&str>, sync_card_open: bool) -> bool {
    focused_id.is_some_and(|id| !SHELL_OVERLAY_INPUT_IDS.contains(&id)) || sync_card_open
}

//#endregion ShellInput

//#region ShellInput
impl<'ctx> PointerCtx<'ctx> {
    /// 🫀️ The crossing half of a step: fund every producer that is still advancing, and drive a
    /// producer whose own witness has frozen to a terminal state.
    ///
    /// 🏁️ A working producer this shell has no window body to render is a producer it cannot
    /// fund at all, so it stands down instead of asking the frame loop for a step per frame it
    /// has no way to spend.
    pub async fn settle_pump_cross(&mut self) -> ShellSettleStep {
        let live: Vec<(String, World3dComputeStatus)> = self.surfaces.live_compute_surfaces().collect();
        if live.is_empty() {
            self.settle.settle_pump.watches.clear();
            self.settle.settle_pump.owed = false;
            return ShellSettleStep::Quiescent;
        }
        let mut wedged: Vec<(String, World3dComputeStatus)> = Vec::new();
        let mut funded: Vec<String> = Vec::new();
        for (surface_id, status) in live {
            let watch = self.settle.settle_pump.watches.entry(surface_id.clone()).or_default();
            match settle_watch_verdict(watch, &status) {
                ShellSettleVerdict::Terminal => wedged.push((surface_id, status)),
                ShellSettleVerdict::Fund => funded.push(surface_id),
                ShellSettleVerdict::Stand => {}
            }
        }
        let watched: Vec<String> = self.surfaces.world3d_states.keys().cloned().collect();
        self.settle.settle_pump.watches.retain(|surface_id, _| watched.contains(surface_id));
        for (host_id, status) in wedged {
            let Some((controller_id, surface_id)) = self.surfaces.world3d_states.get(&host_id).map(|world| (world.controller_id.clone(), world.surface_id.clone())) else { continue };
            debug_log(&format!(
                "[TRACE] wgpu-shell settle pump wedge {}",
                serde_json::json!({ "host": host_id, "surface": surface_id, "phase": status.phase, "unitsDone": status.units_done, "unitsTotal": status.units_total, "inFlight": status.in_flight, "action": status.cancel_action, "steps": SHELL_SETTLE_STALL_STEPS })
            ));
            let mut args = status.cancel_args;
            args.insert("surfaceId".to_string(), Value::String(surface_id));
            let descriptor = ActionDescriptor { controller_id, action: status.cancel_action, args: Some(semio_framework::DslValue::from(Value::Object(args))) };
            if let Err(error) = self.as_actions().dispatch_action(descriptor).await {
                debug_log(&format!("[TRACE] wgpu-shell settle pump terminal drive failed: {error}"));
            }
            self.settle.settle_pump.owed = true;
            return ShellSettleStep::Wedged;
        }
        let bodies: Vec<String> = funded.iter().filter_map(|host_id| self.surfaces.world3d_window_ids.get(host_id)).filter_map(|window_id| self.dock.window_body_key(window_id, &self.docs)).collect();
        if bodies.is_empty() {
            for surface_id in funded {
                if let Some(watch) = self.settle.settle_pump.watches.get_mut(&surface_id) {
                    watch.standing = true;
                }
            }
            self.settle.settle_pump.owed = false;
            return ShellSettleStep::Quiescent;
        }
        self.settle.settle_pump.crossings = self.settle.settle_pump.crossings.saturating_add(1);
        let scope = UiDirtyScope::Partial { window_bodies: bodies, panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false };
        if let Err(error) = self.as_window_actions().refresh_ui(scope).await {
            debug_log(&format!("[TRACE] wgpu-shell settle pump crossing failed: {error}"));
        }
        self.settle.settle_pump.owed = true;
        ShellSettleStep::Crossed
    }

    /// 📥️ One extension round trip, with its answer's own effects folded back into this shell —
    /// the wgpu twin of React's `dispatchInvokeExtensionEffect(..., publish)` callback, which ends in
    /// `applyHostEffects(response.requestedEffects, ...)`.
    #[cfg(target_arch = "wasm32")]
    pub async fn run_extension_invocation(&mut self, invocation: PendingExtensionInvocation) {
        let Some(session) = self.docs.session.clone() else { return };
        let Some(plugin) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        let PendingExtensionInvocation { req, extension_id, capability, request_json } = invocation;
        debug_log(&format!("[TRACE] wgpu-shell invokeExtension dispatch extension={extension_id} capability={capability} req={req}"));
        match plugin.dispatch_invoke_extension(session.instance_id, &extension_id, &capability, &request_json, req).await {
            Ok(result) => {
                debug_log(&format!("[TRACE] wgpu-shell invokeExtension answered req={req} extension={extension_id} capability={capability} effects={}", result.requested_effects.len()));
                let controller_id = session.app.controller_id.clone();
                let scope = result.ui_scope.clone().merged_with(self.docs.host_effect_earned_scope(&result.requested_effects));
                self.as_window_actions().queue_host_effects(&controller_id, result.requested_effects);
                let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
                if let Err(error) = self.as_actions().apply_mutations(&operations, scope).await {
                    debug_log(&format!("[TRACE] wgpu-shell invokeExtension mutations req={req} failed: {error}"));
                }
            }
            Err(error) => debug_log(&format!("[TRACE] wgpu-shell invokeExtension failed req={req} extension={extension_id}: {error}")),
        }
    }

    pub async fn dispatch_widget_drag_values(&mut self, input: &InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.drag.target_id.as_deref() else {
            return Ok(());
        };
        if let Some(value) = self.input.widget_maps.slider_live_values.get(id).copied() {
            if let Some(meta) = self.input.widget_maps.slider_metas.get(id).cloned() {
                self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": value }) }).await?;
            }
        } else if let Some(value) = self.input.widget_maps.ring_live_values.get(id).copied() {
            if let Some(meta) = self.input.widget_maps.ring_metas.get(id).cloned() {
                self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": value }) }).await?;
            }
        }
        Ok(())
    }

    /// 🔢️ The chrome shell's own commit goes through `InputMeta::commit_value`, which is the
    /// one place the field's `min`/`max`/`step` are applied — React's `<input type="number">`
    /// refuses an out-of-range or off-step value outright, and this path used to dispatch the
    /// raw buffer text instead, so a numeric shell field could commit what React never would
    /// (ticket 26/09/17 packet W15a, W1m gap G6). `None` means a `number` buffer that does not
    /// parse at all: the guest sees NO commit rather than an invented number, exactly as the
    /// primitive's own doc comment specifies. The caret still blurs either way, because the
    /// gesture is over whichever answer the constraint gave.
    pub async fn commit_focused_input(&mut self, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.focused_id.clone() else {
            return Ok(());
        };
        if id.ends_with(".input") {
            let base = id.trim_end_matches(".input");
            if let Some(meta) = self.input.widget_maps.stepper_metas.get(base).cloned() {
                let parsed = input.text_view().parse::<f64>().unwrap_or(meta.value);
                self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_absolute.controller_id, action: meta.on_absolute.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": parsed }) }).await?;
                input.blur_input();
                return Ok(());
            }
        }
        if let Some(meta) = self.input.widget_maps.input_metas.get(&id).cloned() {
            let committed = meta.commit_value(input.text_view());
            input.blur_input();
            if let Some(value) = committed {
                let args = merge_committed_args(meta.on_change.args.as_ref(), value);
                self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args }).await?;
            }
        }
        Ok(())
    }

    pub async fn finish_tree_drag(&mut self, x: f32, y: f32, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(drag) = self.input.tree_drag.take() else {
            return Ok(());
        };
        if self.input.finish_canvas2d_catalogue_drop(x, y, &drag.drag_data, input) {
            semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_clear_all_ghost_widgets();
            self.surfaces.clear_world3d_catalogue_drop_previews();
            return Ok(());
        }
        let surfaces = self.surfaces.node_graph_states.iter().map(|(id, surface)| (id.as_str(), surface.bounds, surface.controller_id.as_str())).collect::<Vec<_>>();
        if let Some(action) = semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_flow_widget_drop_action(x, y, &drag.drag_data, &surfaces) {
            semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_clear_all_ghost_widgets();
            self.surfaces.clear_world3d_catalogue_drop_previews();
            self.as_actions().dispatch_action(action).await?;
            return Ok(());
        }
        if let Some(action) = self.surfaces.world3d_catalogue_drop_action(x, y, &drag.drag_data) {
            semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_clear_all_ghost_widgets();
            self.surfaces.clear_world3d_catalogue_drop_previews();
            self.as_actions().dispatch_action(action).await?;
            return Ok(());
        }
        if let Some(action) = semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_catalogue_drop_action(x, y, &drag.drag_data, &surfaces) {
            semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_clear_all_ghost_widgets();
            self.surfaces.clear_world3d_catalogue_drop_previews();
            self.as_actions().dispatch_action(action).await?;
            return Ok(());
        }
        semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_clear_all_ghost_widgets();
        self.surfaces.clear_world3d_catalogue_drop_previews();
        Ok(())
    }

    pub async fn select_panel_tab(&mut self, anchor: PanelAnchor, tab_id: &str) -> Result<(), String> {
        let was_visible = self.dock.anchor_open(anchor);
        let was_path = self.dock.anchor_state(anchor).path.first().cloned();
        let outcome = self.as_actions().select_panel_tab_step(anchor, tab_id).await;
        self.as_actions().journal_panel_selection(anchor, tab_id, was_visible, was_path).await?;
        outcome
    }

    /// 📇️ P3 redirect: focus the hosting window, unfold its Actions rail, expand the form.
    ///
    /// 🎛️ Os-level command redirect (see `apply_os_command` in `shell::ActionPanelAndUtilities`)
    /// — `"os.commandId"` for zero-arg commands, `"os.commandId:optionValue"` for the
    /// per-option-expanded select-arg commands built by `command_search_items`.
    ///
    /// 🎛️ P3 redirect for an arg-carrying command — React opens the Command panel at the
    /// command's category and expands its staged form (`SET_PANEL_VISIBLE`/`SET_PANEL_PATH`/
    /// `SET_COMMAND_EXPANDED`). `reveal_dock_tab` is this target's spelling of the first two;
    /// `expanded_command_id` is the third, and the Commands dock reads it to open that form.
    pub async fn activate_search_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.as_session().filtered_search_items();
        debug_log(&format!(
            "[TRACE] wgpu-shell palette activate index={index} query={:?} items={} offered={:?} chose={:?}",
            self.commands.search_query,
            items.len(),
            items.iter().take(8).map(|item| item.id.as_str()).collect::<Vec<_>>(),
            items.get(index).map(|item| (item.id.as_str(), item.action.as_deref()))
        ));
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(action) = item.dispatch_action.clone() {
            self.as_actions().dispatch_action(action).await?;
        } else if let Some(action) = &item.action {
            if let Some(window_id) = action.strip_prefix("window:") {
                self.windows.active_window_id = Some(window_id.to_string());
            } else if let Some(rest) = action.strip_prefix("action-panel:") {
                if let Some((window_id, action_id)) = rest.split_once(':') {
                    self.windows.active_window_id = Some(window_id.to_string());
                    self.windows.action_panel_folded.insert(window_id.to_string(), false);
                    self.windows.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
                }
            } else if let Some(rest) = action.strip_prefix("os-command:") {
                let (command_id, option_value) = match rest.split_once(':') {
                    Some((command_id, value)) => (command_id.to_string(), Some(value.to_string())),
                    None => (rest.to_string(), None),
                };
                self.as_actions().apply_os_command(&command_id, option_value.as_deref()).await?;
            } else if let Some(command_json) = action.strip_prefix("command:") {
                let invocation: semio_framework::manifest::CommandInvocation = semio_framework_pack_json::from_json_str(command_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
                self.as_actions().dispatch_command(invocation).await?;
            } else if let Some(command_key) = action.strip_prefix("command-form:") {
                let command_key = command_key.to_string();
                let category_tab = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().find(|entry| command_address_stable_key(&entry.address) == command_key).map(|entry| format!("{FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX}{}", entry.definition.category));
                let category_tab = category_tab.as_deref().unwrap_or(FRAMEWORK_CATEGORY_COMMAND_ID);
                let previous_expanded = self.commands.expanded_command_id.replace(command_key);
                if let Err(error) = self.as_session().republish_shell_panel_document(category_tab) {
                    self.commands.expanded_command_id = previous_expanded;
                    return Err(error);
                }
                self.dock.reveal_dock_tab(category_tab);
            }
        }
        self.commands.search_open = false;
        self.input.overlay_state = OverlayState::None;
        self.commands.search_query.clear();
        self.commands.search_selected = 0;
        Ok(())
    }

    pub async fn activate_find_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.commands.filtered_find_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) {
            self.as_actions().dispatch_action(ActionDescriptor {
                controller_id,
                action: "setMediaNodeSelection".into(),
                args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({
                    "surfaceId": item.surface_id,
                    "nodeIds": [item.node_id],
                }),
            })
            .await?;
        }
        self.commands.find_open = false;
        self.input.overlay_state = OverlayState::None;
        self.commands.find_query.clear();
        self.commands.find_selected = 0;
        Ok(())
    }

    /// ⌨️ Applies the P4 keybinding rule: arg-less actions dispatch directly; an arg-carrying action's
    /// hotkey opens its form, or — if that form is already expanded in the active window — executes it
    /// with the staged/validated args (never silent-fires defaults from a cold keystroke).
    ///
    /// ⌨️ An app-wide chord for a WINDOW-OWNED verb addresses the window of the ACTIVE MODE that owns
    /// it, focused or not; a verb no mounted window owns is a HINTED no-op, never a dispatch at a
    /// window whose kind never declared it (`resolve_keybinding_target_window_v1`, ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🧯️ Packet W1j: React raises this through `showTransientNotice(..., "info",
    /// KEYBINDING_UNOWNED_CODE)`, not through a persistent error line — an unowned chord is a
    /// momentary "nothing here answers that", which is exactly what the banner is for.
    pub async fn dispatch_app_keybinding(&mut self, descriptor: ActionDescriptor) -> Result<bool, String> {
        let Some(session) = self.docs.session.clone() else {
            return Ok(false);
        };
        let mounted: Vec<WindowScopeInstanceV1> = self.dock.dock.window_instances().into_iter().map(|(id, window_kind_id)| WindowScopeInstanceV1 { id, window_kind_id }).collect();
        let focused = self.windows.active_window_id.clone().or_else(|| session.view_state.active_window_kind_id.clone());
        let (target_kind, target_window_id) = resolve_keybinding_target_window_v1(&session.app, &mounted, focused.as_deref(), &descriptor.action);
        let Some(window_id) = target_window_id else {
            let label = session
                .app
                .window_kinds
                .iter()
                .flat_map(|kind| kind.actions.iter())
                .find(|action| action.id == descriptor.action)
                .map_or_else(|| descriptor.action.clone(), |action| action.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string());
            let chord = session.app.keybindings.iter().find(|binding| binding.action.action == descriptor.action).map_or_else(String::new, |binding| binding.keys.clone());
            let text = keybinding_unowned_text_v1(self.prefs.active_locale().as_str(), &chord, &label);
            self.chrome.show_transient_notice(text, semio_framework::Severity::Info, Some(KEYBINDING_UNOWNED_CODE));
            return Ok(false);
        };
        if target_kind == WindowScopeTargetKindV1::Owner {
            self.dock.dock.sync_active_window(&window_id);
            self.windows.active_window_id = Some(window_id.clone());
        }
        let window_kind_id = mounted.iter().find(|instance| instance.id == window_id).map_or_else(|| window_id.clone(), |instance| instance.window_kind_id.clone());
        let action_def = window_action_definition(&session.app, &window_kind_id, &descriptor.action).cloned();
        let has_args = action_def.as_ref().is_some_and(|action| !action.args.is_empty());
        if !has_args {
            self.as_actions().dispatch_action(descriptor).await?;
            return Ok(true);
        }
        let action_id = action_def.expect("checked has_args").id;
        let already_expanded = self.windows.active_window_id.as_deref() == Some(window_id.as_str()) && self.windows.action_panel_expanded.get(&window_id).map(String::as_str) == Some(action_id.as_str());
        if already_expanded {
            self.as_actions().execute_staged_action(&window_id, &action_id).await?;
        } else {
            self.windows.active_window_id = Some(window_id.clone());
            self.windows.action_panel_folded.insert(window_id.clone(), false);
            self.windows.action_panel_expanded.insert(window_id, action_id);
            self.as_window_actions().refresh_ui(UiDirtyScope::Full).await?;
        }
        Ok(true)
    }
}
//#endregion ShellInput

//#region ShellInput
//#region ⌨️ShellShortcutDispatch
impl<'ctx> PointerCtx<'ctx> {
    /// 🛠️ A selected `tool.<id>` leaf IS that tool's activation — React's `reconcileToolTabSelection`,
    /// whose whole point is that activation and selection are one state with two representations, so
    /// every route into the path (a press, a restored arrangement, a tour step) ends at the same place.
    /// A path that selects no tool leaf is left alone rather than deactivating, so browsing another
    /// category keeps the armed tool live exactly as React's "skip while not the active root" rule does.
    pub async fn arm_tool_for_panel_tab(&mut self, path: &[String]) -> Result<(), String> {
        let Some(tool_id) = path.last().and_then(|id| id.strip_prefix(FRAMEWORK_TOOL_PANEL_TAB_PREFIX)).map(ToOwned::to_owned) else {
            return Ok(());
        };
        let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) else {
            return Ok(());
        };
        if self.docs.session.as_ref().and_then(|session| session.view_state.active_tool_id.as_deref()) == Some(tool_id.as_str()) {
            return Ok(());
        }
        self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "setActiveTool".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "toolId": tool_id }) }).await
    }

    /// ⌨️ Applies the shell verbs that need the async funnel — the other half of the same table.
    pub async fn apply_async_shell_shortcut(&mut self, shortcut: ShellShortcut) -> Result<bool, String> {
        match shortcut {
            ShellShortcut::ModeStep(step) => {
                self.as_window_actions().apply_mode_step(step).await?;
                Ok(true)
            }
            ShellShortcut::SurfaceRole(role) => {
                self.as_actions().switch_to_session_role(role).await?;
                Ok(true)
            }
            ShellShortcut::TimeTravel(verb) => match self.docs.time_travel_shortcut_action(verb, &self.chrome, &self.history) {
                Some(action) => {
                    self.as_actions().dispatch_action(action).await?;
                    Ok(true)
                }
                None => Ok(false),
            },
            _ => Ok(false),
        }
    }
}
//#endregion ⌨️ShellShortcutDispatch
//#endregion ShellInput
