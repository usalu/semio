//! ⌨️ Shell input, part two: keyboard, palette, context menus and dock ring.
//!
//! Tree hover and selection, world3d catalogue drops, context menus, search/find items, the command palette,
//! keyboard handling, the dock and keyboard ring and shortcut capture.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellInput
impl ShellState {
    pub(crate) fn update_tree_hover(&mut self, input: &InputState<ActionDescriptor>) {
        let hovered = input.hovered_id.as_deref().and_then(tree_hit_item_id);
        if self.tree_hovered_id.as_deref() == hovered {
            return;
        }
        if let Some(prev) = self.tree_hovered_id.take() {
            if let Some(action) = self.widget_maps.tree_unhover_commands.get(&prev) {
                self.deferred_actions.push(action.clone());
            }
        }
        if let Some(id) = hovered {
            if let Some(action) = self.widget_maps.tree_hover_commands.get(id) {
                self.deferred_actions.push(action.clone());
            }
            self.tree_hovered_id = Some(id.to_string());
        }
    }

    pub(crate) fn queue_tree_selection(&mut self, item_id: &str) {
        let Some(action) = self.widget_maps.tree_selection_change.clone() else {
            return;
        };
        self.deferred_actions.push(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: crate::action_args_json!({ "ids": [item_id] }) });
    }

    /// 🌳️ Arms one tree-selection dispatch on the settle lane — a selection is a pointer gesture, and
    /// a gesture never converges its own chain (see [`Self::handle_pointer_button`]).
    pub(crate) fn dispatch_tree_selection(&mut self, item_id: &str) {
        self.queue_tree_selection(item_id);
        self.owe_settle();
    }

    /// 🔁️ Drains the deferred halves AND re-enters the window bodies until nothing new is armed.
    ///
    /// ⚖️ A guest's preview job is driven by its own window render, so a chain has THREE halves, not
    /// two: the actions an effect deferred, the extension calls those actions asked for, and the
    /// RENDER that turns a settled answer into the next `flowEvalTick`. Draining only the first two
    /// parks the chain the moment the work one render armed quiesces — `meshingFaces 6/8` on 6118.
    /// React converges because a mutation re-renders the shell; this is that, bounded
    /// (`SHELL_SETTLE_ROUNDS`) and reentrancy-guarded so a nested flush stays a plain drain.
    pub async fn flush_deferred_actions(&mut self) -> Result<(), String> {
        if self.settling {
            self.drain_deferred_actions().await?;
            return Ok(());
        }
        self.settling = true;
        let result = self.settle_ui_chain().await;
        self.settling = false;
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
    async fn settle_ui_chain(&mut self) -> Result<(), String> {
        for round in 0..SHELL_SETTLE_ROUNDS {
            let worked = self.drain_deferred_actions().await?;
            if worked == 0 {
                if round > 0 {
                    Self::debug_log(&format!("[TRACE] wgpu-shell ui chain settled after {round} round(s){}", Self::resident_census()));
                }
                return Ok(());
            }
            if self.session.is_none() {
                return Ok(());
            }
            self.refresh_ui(UiDirtyScope::None).await?;
        }
        Self::debug_log(&format!("[TRACE] wgpu-shell ui chain exhausted {SHELL_SETTLE_ROUNDS} settle round(s) with {} action(s) left", self.deferred_actions.len()));
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
    async fn drain_deferred_actions(&mut self) -> Result<usize, String> {
        self.arm_window_activation_note();
        self.arm_window_blur_host_event();
        self.arm_host_window_blur();
        let mut worked = self.step_import_transfers().await;
        for _ in 0..SHELL_DEFERRED_CHAIN_ROUNDS {
            let actions = std::mem::take(&mut self.deferred_actions);
            #[cfg(target_arch = "wasm32")]
            let invocations = std::mem::take(&mut self.pending_extension_invocations);
            #[cfg(not(target_arch = "wasm32"))]
            let invocations: Vec<()> = Vec::new();
            #[cfg(target_arch = "wasm32")]
            let file_opens = std::mem::take(&mut self.pending_file_opens);
            #[cfg(not(target_arch = "wasm32"))]
            let file_opens: Vec<()> = Vec::new();
            if actions.is_empty() && invocations.is_empty() && file_opens.is_empty() {
                break;
            }
            worked = worked.saturating_add(actions.len()).saturating_add(invocations.len()).saturating_add(file_opens.len());
            for action in actions {
                let id = action.action.clone();
                if let Err(error) = self.dispatch_action(action).await {
                    Self::debug_log(&format!("[TRACE] wgpu-shell deferred action {id} failed: {error}"));
                }
            }
            #[cfg(target_arch = "wasm32")]
            for invocation in invocations {
                self.run_extension_invocation(invocation).await;
            }
            #[cfg(target_arch = "wasm32")]
            for request in file_opens {
                self.run_file_open_request(request).await;
            }
        }
        if !self.deferred_actions.is_empty() {
            Self::debug_log(&format!("[TRACE] wgpu-shell deferred chain exhausted {SHELL_DEFERRED_CHAIN_ROUNDS} rounds with {} action(s) left", self.deferred_actions.len()));
        }
        if self.pending_shell_uri_apply {
            self.pending_shell_uri_apply = false;
            self.apply_pending_shell_uri().await?;
            worked = worked.saturating_add(1);
        }
        Ok(worked)
    }

    /// 🖐️ Runs the work a browser only permits inside the gesture that asked for it — today exactly
    /// the parked file-open pickers — and nothing else.
    ///
    /// ⚖️ Everything a gesture arms goes to the settle pump, which takes it on a LATER frame. A file
    /// picker cannot wait for one: `input.click()` is gated on USER ACTIVATION, which the browser
    /// grants to the task that handles the click and to no task after it. This is the one exception,
    /// and it is deliberately the narrowest possible: only `pending_file_opens`, only from a pointer
    /// gesture, and still bounded (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎨️ The theme IMPORT picker is the second gesture-bound door (packet W15d) and obeys the same
    /// rule: opened from the gesture that armed it, never from a later settle step.
    #[cfg(target_arch = "wasm32")]
    pub(crate) async fn drain_gesture_bound_work(&mut self) {
        if std::mem::take(&mut self.pending_theme_import) {
            self.run_theme_import_request().await;
        }
        for _ in 0..SHELL_DEFERRED_CHAIN_ROUNDS {
            let file_opens = std::mem::take(&mut self.pending_file_opens);
            if file_opens.is_empty() {
                break;
            }
            for request in file_opens {
                self.run_file_open_request(request).await;
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn drain_gesture_bound_work(&mut self) {}

    /// 🫀️ Arms the runtime-owned settle lane — the settle twin of [`Self::owe_refresh`], and the ONE
    /// way a producer says "there is a chain here to converge". Declaring is all a producer may do:
    /// the frame loop is the only caller of [`Self::settle_pump_step`], so nothing converges inside a
    /// boot, a dispatch or a pointer handler any more.
    pub fn owe_settle(&mut self) {
        self.settle_pump.owed = true;
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
        let parked = !self.pending_extension_invocations.is_empty() || !self.pending_file_opens.is_empty();
        #[cfg(not(target_arch = "wasm32"))]
        let parked = false;
        let armed_work = !self.deferred_actions.is_empty() || self.pending_shell_uri_apply || parked || host_window_blur_pending() || !self.import_transfers.is_empty();
        let computing = self.live_compute_surfaces().any(|(surface_id, _)| !self.settle_pump.watches.get(&surface_id).is_some_and(|watch| watch.standing));
        self.icon_export.as_ref().is_some_and(icon_export::IconExportBatch::running) || crate::interpreter::ui_document_close_pending() || settle_pump_owes(self.settling, self.session.is_some(), armed_work, &self.owed_refresh_scope, self.settle_pump.owed, computing)
    }

    /// ⏳️ Every live World3d surface whose own published status says its producer is working, with
    /// that status — the one input both halves of a pump step read.
    fn live_compute_surfaces(&self) -> impl Iterator<Item = (String, World3dComputeStatus)> + '_ {
        self.world3d_status.iter().filter(|(surface_id, _)| self.world3d_states.contains_key(surface_id.as_str())).filter_map(|(surface_id, json)| {
            let status = world3d_compute_status(Some(json.as_str()));
            status.computing.then(|| (surface_id.clone(), status))
        })
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
        self.pump_tree_windows();
        let step = self.settle_pump_step_inner().await;
        let entering = matches!(step, ShellSettleStep::Wedged | ShellSettleStep::Quiescent) || matches!(self.settle_pump.traced, None | Some(ShellSettleStep::Quiescent));
        if (entering && self.settle_pump.traced != Some(step)) || self.settle_pump.steps % SHELL_SETTLE_HEARTBEAT_STEPS == 0 {
            self.settle_pump.traced = Some(step);
            Self::debug_log(&format!("[TRACE] wgpu-shell settle pump {}", serde_json::json!({ "step": format!("{step:?}"), "steps": self.settle_pump.steps, "crossings": self.settle_pump.crossings, "watching": self.settle_pump.watches.len() })));
        }
        step
    }

    /// 🪟️ The first body of a newly created window is a prerequisite for its own journal note.
    /// Once a cohort refresh releases any ready peer, that peer dispatches before the next retry
    /// refresh; an unrelated refusing body cannot head-of-line block its acknowledged transfer.
    pub(crate) async fn settle_pump_step_inner(&mut self) -> ShellSettleStep {
        self.settle_pump.steps = self.settle_pump.steps.saturating_add(1);
        if self.session.is_none() {
            self.settle_pump.owed = false;
            return ShellSettleStep::Quiescent;
        }
        if self.window_topology_refresh_owed && !self.window_topology_journal_dispatch_owed {
            if let Err(error) = self.refresh_ui(UiDirtyScope::None).await {
                self.owe_refresh(UiDirtyScope::Full);
                Self::debug_log(&format!("[TRACE] wgpu-shell window topology refresh failed: {error}"));
            }
            self.settle_pump.owed = true;
            return ShellSettleStep::Drained;
        }
        let worked = match self.drain_deferred_actions().await {
            Ok(worked) => worked,
            Err(error) => {
                Self::debug_log(&format!("[TRACE] wgpu-shell settle pump drain failed: {error}"));
                0
            }
        };
        self.window_topology_journal_dispatch_owed = false;
        if worked > 0 || !self.owed_refresh_scope.asks_for_nothing() || self.settle_pump.rendering.is_some() {
            if !self.owed_refresh_scope.asks_for_nothing() || self.settle_pump.rendering.is_some() {
                self.advance_owed_refresh().await;
            }
            self.settle_pump.owed = true;
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
    async fn advance_owed_refresh(&mut self) {
        if let Some(rendering) = self.settle_pump.rendering.as_ref() {
            let Some(rendered) = rendering.take() else { return };
            self.settle_pump.rendering = None;
            if let Err(error) = self.refresh_ui_rendered(UiDirtyScope::None, Some(rendered)).await {
                Self::debug_log(&format!("[TRACE] wgpu-shell settle pump refresh failed: {error}"));
            }
            return;
        }
        let Some(session) = self.session.clone() else {
            if let Err(error) = self.refresh_ui(UiDirtyScope::None).await {
                Self::debug_log(&format!("[TRACE] wgpu-shell settle pump refresh failed: {error}"));
            }
            return;
        };
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        self.drain_retained_document_arenas();
        self.sync_dock();
        let scope = self.owed_refresh_scope.clone();
        let view_state = self.live_view_state(&session);
        let mut jobs = Vec::new();
        for (window_id, window_kind_id) in self.dock.window_instances() {
            let Some(kind) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id) else { continue };
            if !scope.wants_window_body(&kind.body_key) {
                continue;
            }
            if let Some(window_view) = view_state.for_window_instance(&window_id) {
                jobs.push((window_id, kind.body_key.clone(), window_view));
            }
        }
        let panel_view = view_state.for_panel();
        for tab in Self::flatten_panel_tab_leaves(&session.app.panel_tabs) {
            if let Some(body_key) = tab.body_key.as_deref().filter(|body_key| scope.wants_panel_body(body_key)) {
                jobs.push((tab.id().to_string(), body_key.to_string(), panel_view.clone()));
            }
        }
        let sections = ShellRefreshSections {
            catalogue: claim_app_catalogue_fetch(&mut self.app_catalogue_instance, session.instance_id),
            engagements: scope.wants_section(UiDirtySection::Engagements),
            measures: scope.wants_section(UiDirtySection::Measures),
            tools: scope.wants_section(UiDirtySection::Tools) && !self.tool_panel_tabs().is_empty(),
        };
        self.settle_pump.rendering = Some(ShellDetached::spawn(render_refresh_detached(program, session.instance_id, jobs, sections, panel_view, view_state)));
    }

    /// 🫀️ The crossing half of a step: fund every producer that is still advancing, and drive a
    /// producer whose own witness has frozen to a terminal state.
    ///
    /// 🏁️ A working producer this shell has no window body to render is a producer it cannot
    /// fund at all, so it stands down instead of asking the frame loop for a step per frame it
    /// has no way to spend.
    async fn settle_pump_cross(&mut self) -> ShellSettleStep {
        let live: Vec<(String, World3dComputeStatus)> = self.live_compute_surfaces().collect();
        if live.is_empty() {
            self.settle_pump.watches.clear();
            self.settle_pump.owed = false;
            return ShellSettleStep::Quiescent;
        }
        let mut wedged: Vec<(String, World3dComputeStatus)> = Vec::new();
        let mut funded: Vec<String> = Vec::new();
        for (surface_id, status) in live {
            let watch = self.settle_pump.watches.entry(surface_id.clone()).or_default();
            match settle_watch_verdict(watch, &status) {
                ShellSettleVerdict::Terminal => wedged.push((surface_id, status)),
                ShellSettleVerdict::Fund => funded.push(surface_id),
                ShellSettleVerdict::Stand => {}
            }
        }
        let watched: Vec<String> = self.world3d_states.keys().cloned().collect();
        self.settle_pump.watches.retain(|surface_id, _| watched.contains(surface_id));
        for (host_id, status) in wedged {
            let Some((controller_id, surface_id)) = self.world3d_states.get(&host_id).map(|world| (world.controller_id.clone(), world.surface_id.clone())) else { continue };
            Self::debug_log(&format!(
                "[TRACE] wgpu-shell settle pump wedge {}",
                serde_json::json!({ "host": host_id, "surface": surface_id, "phase": status.phase, "unitsDone": status.units_done, "unitsTotal": status.units_total, "inFlight": status.in_flight, "action": status.cancel_action, "steps": SHELL_SETTLE_STALL_STEPS })
            ));
            let mut args = status.cancel_args;
            args.insert("surfaceId".to_string(), Value::String(surface_id));
            let descriptor = ActionDescriptor { controller_id, action: status.cancel_action, args: Some(semio_framework::DslValue::from(Value::Object(args))) };
            if let Err(error) = self.dispatch_action(descriptor).await {
                Self::debug_log(&format!("[TRACE] wgpu-shell settle pump terminal drive failed: {error}"));
            }
            self.settle_pump.owed = true;
            return ShellSettleStep::Wedged;
        }
        let bodies: Vec<String> = funded.iter().filter_map(|host_id| self.world3d_window_ids.get(host_id)).filter_map(|window_id| self.window_body_key(window_id)).collect();
        if bodies.is_empty() {
            for surface_id in funded {
                if let Some(watch) = self.settle_pump.watches.get_mut(&surface_id) {
                    watch.standing = true;
                }
            }
            self.settle_pump.owed = false;
            return ShellSettleStep::Quiescent;
        }
        self.settle_pump.crossings = self.settle_pump.crossings.saturating_add(1);
        let scope = UiDirtyScope::Partial { window_bodies: bodies, panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false };
        if let Err(error) = self.refresh_ui(scope).await {
            Self::debug_log(&format!("[TRACE] wgpu-shell settle pump crossing failed: {error}"));
        }
        self.settle_pump.owed = true;
        ShellSettleStep::Crossed
    }

    /// 🪟️ The body key one live window instance renders — the address a scoped refresh names a window
    /// by, resolved through the dock plan and the app's own window kinds.
    fn window_body_key(&self, window_id: &str) -> Option<String> {
        let session = self.session.as_ref()?;
        let kind_id = self.dock.window_instances().into_iter().find(|(id, _)| id == window_id).map(|(_, kind_id)| kind_id)?;
        session.app.window_kinds.iter().find(|kind| kind.id == kind_id).map(|kind| kind.body_key.clone())
    }

    /// 📤️ One file-open round trip: opens a REAL browser picker, then hands the picked files to an [`ImportTransfer`] whose
    /// chunks the drain dispatches in order through the normal action path — so each chunk's own effects, mutations and
    /// history patch are folded in exactly as a user-pressed action's are, and a person can cancel it between chunks.
    /// A cancelled picker dispatches nothing and is not an error.
    #[cfg(target_arch = "wasm32")]
    async fn run_file_open_request(&mut self, request: PendingFileOpen) {
        let PendingFileOpen { controller_id, accept, read_as, import_action, args, multiple } = request;
        let opened = request_file_open(&accept, read_as.as_deref(), multiple).await;
        Self::debug_log(&format!("[TRACE] wgpu-shell file open accept={accept} action={import_action} files={} bytes={}", opened.len(), opened.iter().map(|file| file.contents.len()).sum::<usize>()));
        self.begin_import_transfer(PickedImport { controller_id, import_action, opened, args, multiple });
    }

    /// 📥️ One extension round trip, with its answer's own effects folded back into this shell —
    /// the wgpu twin of React's `dispatchInvokeExtensionEffect(..., publish)` callback, which ends in
    /// `applyHostEffects(response.requestedEffects, ...)`.
    #[cfg(target_arch = "wasm32")]
    async fn run_extension_invocation(&mut self, invocation: PendingExtensionInvocation) {
        let Some(session) = self.session.clone() else { return };
        let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        let PendingExtensionInvocation { req, extension_id, capability, request_json } = invocation;
        Self::debug_log(&format!("[TRACE] wgpu-shell invokeExtension dispatch extension={extension_id} capability={capability} req={req}"));
        match plugin.dispatch_invoke_extension(session.instance_id, &extension_id, &capability, &request_json, req).await {
            Ok(result) => {
                Self::debug_log(&format!("[TRACE] wgpu-shell invokeExtension answered req={req} extension={extension_id} capability={capability} effects={}", result.requested_effects.len()));
                let controller_id = session.app.controller_id.clone();
                let scope = result.ui_scope.clone().merged_with(self.host_effect_earned_scope(&result.requested_effects));
                self.queue_host_effects(&controller_id, result.requested_effects);
                let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
                if let Err(error) = self.apply_mutations(&operations, scope).await {
                    Self::debug_log(&format!("[TRACE] wgpu-shell invokeExtension mutations req={req} failed: {error}"));
                }
            }
            Err(error) => Self::debug_log(&format!("[TRACE] wgpu-shell invokeExtension failed req={req} extension={extension_id}: {error}")),
        }
    }

    pub(crate) async fn dispatch_widget_drag_values(&mut self, input: &InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.drag.target_id.as_deref() else {
            return Ok(());
        };
        if let Some(value) = self.widget_maps.slider_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.slider_metas.get(id).cloned() {
                self.dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args: crate::action_args_json!({ "value": value }) }).await?;
            }
        } else if let Some(value) = self.widget_maps.ring_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.ring_metas.get(id).cloned() {
                self.dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args: crate::action_args_json!({ "value": value }) }).await?;
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
    pub(crate) async fn commit_focused_input(&mut self, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.focused_id.clone() else {
            return Ok(());
        };
        if id.ends_with(".input") {
            let base = id.trim_end_matches(".input");
            if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                let parsed = input.text_view().parse::<f64>().unwrap_or(meta.value);
                self.dispatch_action(ActionDescriptor { controller_id: meta.on_absolute.controller_id, action: meta.on_absolute.action, args: crate::action_args_json!({ "value": parsed }) }).await?;
                input.blur_input();
                return Ok(());
            }
        }
        if let Some(meta) = self.widget_maps.input_metas.get(&id).cloned() {
            let committed = meta.commit_value(input.text_view());
            input.blur_input();
            if let Some(value) = committed {
                let args = merge_committed_args(meta.on_change.args.as_ref(), value);
                self.dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args }).await?;
            }
        }
        Ok(())
    }

    pub(crate) fn sync_world3d_catalogue_drop_preview(&mut self, x: f32, y: f32, drag_data: &HashMap<String, String>) {
        let Some((object_kind, mesh_url)) = crate::engine_canvas::puzzle3d_catalogue_drag_payload(drag_data) else {
            self.clear_world3d_catalogue_drop_previews();
            return;
        };
        let mesh_url_ref = mesh_url.as_deref();
        for state in self.world3d_states.values_mut() {
            if state.bounds.contains(x, y) {
                world3d_update_catalogue_drop_preview(state, x, y, &object_kind, mesh_url_ref);
            } else {
                world3d_clear_catalogue_drop_preview(state);
            }
        }
    }

    pub(crate) fn retained_window_body_rect(&self, window_id: &str) -> Option<Rect> {
        self.retained_hit_windows.values().find_map(|(owner, body)| (owner == window_id).then_some(*body))
    }

    /// 👻️ Catalogue ghosts while dragging from a retained document tree (panel catalogue) — not shell `tree_drag`.
    pub(crate) fn sync_world3d_catalogue_drop_preview_from_retained_ui(&mut self) {
        let sessions = crate::interpreter::active_retained_drag_sessions();
        if sessions.is_empty() {
            if self.tree_drag.is_none() && self.pending_tree_drag.is_none() {
                self.clear_world3d_catalogue_drop_previews();
            }
            return;
        }
        for (window_id, local_x, local_y, drag_data) in sessions {
            let Some(body) = self.retained_window_body_rect(&window_id) else { continue };
            self.sync_world3d_catalogue_drop_preview(body.x + local_x, body.y + local_y, &drag_data);
        }
    }

    fn clear_world3d_catalogue_drop_previews(&mut self) {
        for state in self.world3d_states.values_mut() {
            world3d_clear_catalogue_drop_preview(state);
        }
    }

    fn retained_scene_point(&self, x: f32, y: f32, input: &InputState<ActionDescriptor>) -> Option<(String, f32, f32)> {
        let hit = input.hit_at(x, y)?;
        if hit.kind != HitKind::ComponentScene {
            return None;
        }
        let (window_id, body) = self.retained_hit_window(hit)?;
        Some((window_id, x - body.x, y - body.y))
    }

    pub(crate) fn sync_canvas2d_catalogue_drag(&self, x: f32, y: f32, drag_data: &HashMap<String, String>, input: &mut InputState<ActionDescriptor>) {
        let Some((window_id, local_x, local_y)) = self.retained_scene_point(x, y, input) else {
            crate::scenes::cancel_canvas_catalogue_drag(input);
            return;
        };
        crate::interpreter::canvas_catalogue_drag_over(&window_id, local_x, local_y, drag_data, input);
    }

    pub(crate) fn finish_canvas2d_catalogue_drop(&self, x: f32, y: f32, drag_data: &HashMap<String, String>, input: &mut InputState<ActionDescriptor>) -> bool {
        let Some((window_id, local_x, local_y)) = self.retained_scene_point(x, y, input) else {
            crate::scenes::cancel_canvas_catalogue_drag(input);
            return false;
        };
        crate::interpreter::canvas_catalogue_drop(&window_id, local_x, local_y, drag_data, input)
    }

    fn world3d_catalogue_drop_action(&self, x: f32, y: f32, drag_data: &HashMap<String, String>) -> Option<ActionDescriptor> {
        let (object_kind, _mesh_url) = crate::engine_canvas::puzzle3d_catalogue_drag_payload(drag_data)?;
        for state in self.world3d_states.values() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            let Some(origin) = world3d_catalogue_drop_origin(state, x, y) else {
                return None;
            };
            return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "addObjectKind".into(), args: crate::action_args_json!({ "objectKind": object_kind, "origin": origin }) });
        }
        None
    }

    pub(crate) async fn finish_tree_drag(&mut self, x: f32, y: f32, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(drag) = self.tree_drag.take() else {
            return Ok(());
        };
        if self.finish_canvas2d_catalogue_drop(x, y, &drag.drag_data, input) {
            crate::engine_canvas::node_graph_clear_all_ghost_widgets();
            self.clear_world3d_catalogue_drop_previews();
            return Ok(());
        }
        let surfaces = self.node_graph_states.iter().map(|(id, surface)| (id.as_str(), surface.bounds, surface.controller_id.as_str())).collect::<Vec<_>>();
        if let Some(action) = crate::engine_canvas::node_graph_flow_widget_drop_action(x, y, &drag.drag_data, &surfaces) {
            crate::engine_canvas::node_graph_clear_all_ghost_widgets();
            self.clear_world3d_catalogue_drop_previews();
            self.dispatch_action(action).await?;
            return Ok(());
        }
        if let Some(action) = self.world3d_catalogue_drop_action(x, y, &drag.drag_data) {
            crate::engine_canvas::node_graph_clear_all_ghost_widgets();
            self.clear_world3d_catalogue_drop_previews();
            self.dispatch_action(action).await?;
            return Ok(());
        }
        if let Some(action) = crate::engine_canvas::node_graph_catalogue_drop_action(x, y, &drag.drag_data, &surfaces) {
            crate::engine_canvas::node_graph_clear_all_ghost_widgets();
            self.clear_world3d_catalogue_drop_previews();
            self.dispatch_action(action).await?;
            return Ok(());
        }
        crate::engine_canvas::node_graph_clear_all_ghost_widgets();
        self.clear_world3d_catalogue_drop_previews();
        Ok(())
    }

    pub(crate) fn render_tree_drag_overlay_node(&self, overlay: &mut DrawList, input: &InputState<ActionDescriptor>, theme: &Theme) {
        let Some(drag) = &self.tree_drag else {
            return;
        };
        overlay.push_solid([drag.x - 60.0, drag.y - 12.0, 120.0, 24.0], theme.selected.with_alpha(0.85));
        if let Some(hit) = input.hit_at(drag.x, drag.y) {
            if let Some(target_id) = hit.control_id.as_deref().and_then(tree_hit_item_id) {
                let _ = target_id;
                match drag.drop_position {
                    TreeDropPosition::Before => overlay.push_solid([hit.rect.x, hit.rect.y, hit.rect.w, 2.0], theme.accent),
                    TreeDropPosition::After => overlay.push_solid([hit.rect.x, hit.rect.y + hit.rect.h - 2.0, hit.rect.w, 2.0], theme.accent),
                    TreeDropPosition::Inside => overlay.push_rounded([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h], theme.accent.with_alpha(0.15), theme.border_radius),
                }
            }
        }
    }

    /// 📑️ One tab-bar press: a branch folds/unfolds in place, a leaf opens its anchor at that leaf —
    /// routed through the host's own `setActivePanelTab` action first whenever this session claims that
    /// leaf, so the host panel document stays the single authority on which leaf is active.
    /// 📑️ Which anchor owns `tab_id` — the wgpu twin of React reading a tab's own `group`
    /// (`panelAnchorForGroup`), and the reason a panel-tab control id no longer has to spell its
    /// anchor out. `None` for anything that is not a dock tab, which is what makes it safe as the
    /// guard of `handle_shell_hit`'s panel-tab arm.
    pub(crate) fn panel_tab_anchor(&self, tab_id: &str) -> Option<PanelAnchor> {
        PanelAnchor::ALL.into_iter().find(|anchor| self.dock_tabs.tabs(*anchor).iter().any(|node| dock_tab_node_carries(node, tab_id)))
    }

    /// 📑️ Journals what React journals for the same gesture: `shell.panelTab` when the anchor's path
    /// moves, `shell.panelToggle` when its visibility flips (`🏛️ShellHost/🟦️.tsx:10014`, `:10042`).
    ///
    /// 🩸️ This renderer used to journal NOTHING for a panel press — the surface appeared and
    /// disappeared in `dumpStructure` with no action crossing `dispatch_action` at all, so the parity
    /// probe read every panel step as a difference (`📓️w5b-interaction-parity-probe.md` §4.4).
    pub(crate) async fn journal_panel_selection(&mut self, anchor: PanelAnchor, tab_id: &str, was_visible: bool, was_path: Option<String>) -> Result<(), String> {
        let Some(controller_id) = self.session.as_ref().map(|session| session.app.controller_id.clone()) else { return Ok(()) };
        let is_de = self.locale_id == "de";
        let visible = self.anchor_open(anchor);
        let path = self.anchor_state(anchor).path.first().cloned();
        if visible != was_visible {
            let label = shell_chrome_string("shellCommand.panelToggle", is_de);
            let detail = serde_json::json!({ "anchor": anchor.as_str(), "visible": visible });
            self.dispatch_action(Self::note_shell_command_action(&controller_id, "shell.panelToggle", label, Some(detail))).await?;
        }
        if path != was_path {
            let label = shell_chrome_string("shellCommand.panelTab", is_de);
            let detail = serde_json::json!({ "anchor": anchor.as_str(), "tabId": tab_id });
            self.dispatch_action(Self::note_shell_command_action(&controller_id, "shell.panelTab", label, Some(detail))).await?;
        }
        Ok(())
    }

    pub(crate) async fn select_panel_tab(&mut self, anchor: PanelAnchor, tab_id: &str) -> Result<(), String> {
        let was_visible = self.anchor_open(anchor);
        let was_path = self.anchor_state(anchor).path.first().cloned();
        let outcome = self.select_panel_tab_step(anchor, tab_id).await;
        self.journal_panel_selection(anchor, tab_id, was_visible, was_path).await?;
        outcome
    }

    /// 🌳️ A branch already showing its children row folds that row; otherwise the press opens
    /// the anchor at that branch — React's `PanelTabRow` drill-down, one press per level.
    async fn select_panel_tab_step(&mut self, anchor: PanelAnchor, tab_id: &str) -> Result<(), String> {
        if self.dock_tabs.locate(tab_id).and_then(|(found, path)| self.dock_tabs.node_at(found, &path)).is_some_and(DockTabNode::is_branch) {
            if self.anchor_open(anchor) && self.anchor_state(anchor).path.first().is_some_and(|id| id == tab_id) && !self.dock_branch_collapsed(anchor, tab_id) {
                self.toggle_dock_branch(anchor, tab_id);
            } else {
                self.dock_collapsed_branches.remove(&format!("{}/{tab_id}", anchor.as_str()));
                self.toggle_anchor_tab(anchor, tab_id);
            }
            return Ok(());
        }
        let claimed = self.session.as_ref().filter(|_| self.host_config().is_some()).map(|session| session.app.controller_id.clone());
        let is_app_leaf = self.session.as_ref().is_some_and(|session| Self::flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().any(|leaf| leaf.id() == tab_id));
        if let Some(controller_id) = claimed.filter(|_| is_app_leaf) {
            self.dispatch_action(ActionDescriptor { controller_id, action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab_id }) }).await?;
            return Ok(());
        }
        self.toggle_anchor_tab(anchor, tab_id);
        Ok(())
    }

    /// 🔽️ Closes every open `Select` popup and drops the two `scroll_offsets` slots each one owned —
    /// a reopened popup starts at its first page, like a freshly mounted `SelectContent`, and neither
    /// slot outlives the popup that minted it (ticket 26/09/17 packet W15a). The ONE close path, so
    /// the outside-press, item-commit, Escape and toggle routes cannot disagree about the state they
    /// leave behind.
    pub(crate) fn close_open_selects(&mut self) {
        for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
            ui_wgpu::wgpu::clear_select_scroll(&mut self.scroll_offsets, &key);
            self.open_selects.insert(key, false);
        }
    }

    pub(crate) fn dismiss_overlays(&mut self, x: f32, y: f32, input: &mut InputState<ActionDescriptor>) -> bool {
        let hit = input.hit_at(x, y);
        let on_palette = hit.and_then(|hit| hit.control_id.as_deref()).is_some_and(|id| [ShellPaletteKind::Search, ShellPaletteKind::Find].into_iter().any(|kind| kind.owns_control_id(id)));
        if matches!(self.overlay_state, OverlayState::Search | OverlayState::Find) && !on_palette {
            self.close_command_palette(input);
            return true;
        }
        let on_overlay = on_palette || hit.is_some_and(|h| matches!(h.kind, HitKind::ContextMenu | HitKind::DropdownItem | HitKind::NavbarItem | HitKind::Select | HitKind::Input));
        if self.open_selects.values().any(|open| *open) && !on_overlay {
            self.close_open_selects();
            return true;
        }
        if self.context_menu.is_some() && !on_overlay {
            self.context_menu = None;
            return true;
        }
        if self.overlay_state != OverlayState::None && !on_overlay {
            self.close_command_palette(input);
            return true;
        }
        false
    }

    pub(crate) async fn open_context_menu(&mut self, x: f32, y: f32, hit: Option<HitTarget<ActionDescriptor>>) {
        let node_id = hit.as_ref().and_then(|hit| hit.control_id.as_deref().and_then(|id| id.rsplit_once(".node.").map(|(_, node_id)| node_id.to_string())));
        let edge_id = hit.as_ref().and_then(|hit| hit.control_id.as_deref().and_then(|id| id.rsplit_once(".edge.").map(|(_, edge_id)| edge_id.to_string())));
        let ContextMenuSurfaceResolution { surface_id, kind, hits, selection: selection_groups, text: text_context } = self.resolve_context_menu_surface(x, y, node_id.as_deref(), edge_id.as_deref());
        let window_instance_id = self.context_window_instance_id(x, y).map(str::to_string);
        let is_de = self.locale_id == "de";
        let mut items = Vec::new();
        if let Some(session) = self.session.clone() {
            let shortcut_by_action: HashMap<String, String> = session.app.keybindings.iter().map(|binding| (binding.action.action.clone(), binding.keys.clone())).collect();
            let view_state = self.live_view_state(&session);
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
            if let Some(program) = self.plugins.iter().find(|plugin| plugin.plugin_id == session.plugin_id) {
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
                        self.error = Some(error);
                    }
                }
            }
        }
        if items.is_empty() {
            if let Some(session) = &self.session {
                let window_kind = window_instance_id.as_deref().and_then(|window_id| self.live_window_kind_id(session, window_id)).and_then(|window_kind_id| session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id));
                let actions: Vec<ui_wgpu::wgpu::ShellMenuAction> = window_kind
                    .map(|kind| semio_framework::resolve_window_actions(&session.app, kind))
                    .unwrap_or_default()
                    .into_iter()
                    .map(|action| ui_wgpu::wgpu::ShellMenuAction {
                        id: action.id.clone(),
                        label: action.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
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
        if let Some(controller_id) = self.host_controller_id() {
            items.push(ContextMenuItem {
                id: "shell.context.home".into(),
                label: shell_chrome_string("contextMenu.goHome", is_de).to_string(),
                icon: None,
                destructive: false,
                action: Some(ActionDescriptor { controller_id, action: "goHome".into(), args: None }),
                ..Default::default()
            });
        }
        self.context_menu = Some(ContextMenuState { x, y, items, active: Vec::new(), submenu_collapsed_at: None, scroll_offset: 0.0 });
        self.overlay_state = OverlayState::None;
    }

    /** 🖱️ Which SURFACE a right-click belongs to, and that kind's own `hits`/`selection`/
     * `text` — the `surface` half of React's `openSurfaceContextMenu` request, resolved for EVERY
     * scene kind rather than the four the shell happens to keep a state map for.
     *
     * The component scene under the pointer is asked first (`interpreter::surface_context_menu_target`
     * → `scenes::scene_context_menu_target`), because every one of the fifteen kinds IS a
     * `ComponentScene` leaf in the window's retained tree — that is the one lookup that covers
     * tables, feeds, canvases, editors, graphs, boards and maps alike, and it resolves against the
     * SAME rect the surface painted at. `World3d` is the single kind whose hover/selection authority
     * lives outside the tree (the shell's own `world3d_states`), so its target is filled from
     * `infinite_world::world::world3d_context_menu_surface` here.
     *
     * A point over chrome no scene owns answers the shell's own `"window"` menu, carrying the
     * node/edge the hit control id named as a fallback (a chrome-registered graph row).
     */
    fn resolve_context_menu_surface(&self, x: f32, y: f32, node_id: Option<&str>, edge_id: Option<&str>) -> ContextMenuSurfaceResolution {
        let mut fallback_hits = Vec::new();
        if let Some(node_id) = node_id {
            fallback_hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "node".into(), id: node_id.into(), label: None });
        }
        if let Some(edge_id) = edge_id {
            fallback_hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "edge".into(), id: edge_id.into(), label: None });
        }
        if let Some((window_id, body)) = self.retained_context_menu_window(x, y) {
            if let Some(resolved) = crate::interpreter::surface_context_menu_target(&window_id, x - body.x, y - body.y) {
                let kind = crate::scenes::context_menu_surface_kind_id(resolved.kind).to_string();
                let crate::scenes::SceneContextMenuTarget { mut hits, mut selection, text } = resolved.target;
                if resolved.kind == ui_wgpu::wgpu::SurfaceKind::World3d {
                    if let Some(state) = self.world3d_state_for_window(&window_id) {
                        let (world_hits, world_selection) = infinite_world::world::world3d_context_menu_surface(state);
                        hits = world_hits;
                        selection = world_selection;
                    }
                }
                if hits.is_empty() {
                    hits = fallback_hits;
                }
                return ContextMenuSurfaceResolution { surface_id: resolved.surface_id, kind, hits, selection, text };
            }
        }
        ContextMenuSurfaceResolution { surface_id: "shell".into(), kind: "window".into(), hits: fallback_hits, selection: Vec::new(), text: None }
    }

    /// 🍿️ Hands one activated context-menu row to the text editor under the menu's own origin when
    /// that row is local to the editor (`interpreter::text_editor_claim_menu_action`). `true` means
    /// the row is answered on the surface and must NOT be dispatched to the guest.
    pub(crate) fn claim_text_editor_menu_action(&self, action: &ActionDescriptor, x: f32, y: f32) -> bool {
        let Some((window_id, body)) = self.retained_context_menu_window(x, y) else {
            return false;
        };
        crate::interpreter::text_editor_claim_menu_action(&window_id, x - body.x, y - body.y, &action.action)
    }

    /// 📋️ Alt + secondary press on a text editor opens its completions dropdown instead of a menu.
    pub(crate) fn claim_text_editor_alt_completions(&self, x: f32, y: f32) -> bool {
        let Some((window_id, body)) = self.retained_context_menu_window(x, y) else {
            return false;
        };
        crate::interpreter::text_editor_claim_alt_completions(&window_id, x - body.x, y - body.y)
    }

    /// 🪟️ The retained body a right-click landed in, as `(window_id, body_rect)` — the hit registry's
    /// own owner map first (a press on a registered row inside the body), then the dock body the point
    /// falls in, so a click on a scene that registers no target of its own still resolves its window.
    fn retained_context_menu_window(&self, x: f32, y: f32) -> Option<(String, Rect)> {
        if let Some(window_id) = self.context_window_instance_id(x, y) {
            if let Some(body) = self.retained_window_body_rect(window_id) {
                return Some((window_id.to_string(), body));
            }
        }
        self.retained_hit_windows.values().find(|(_, body)| body.contains(x, y)).cloned()
    }

    pub(crate) fn sync_context_menu_hover(&mut self, input: &InputState<ActionDescriptor>) {
        let Some(menu) = self.context_menu.as_mut() else {
            return;
        };
        let Some(item_id) = input.hovered_id.as_deref() else {
            return;
        };
        let mut prefix = Vec::new();
        if let Some(path) = context_menu_path_for_item_id(&menu.items, item_id, &mut prefix) {
            menu.active = path;
            menu.submenu_collapsed_at = None;
        }
    }

    /** ⌨️ Routes keyboard input to the open shell context menu. */
    pub fn context_menu_handle_key(&mut self, action: ui_wgpu::wgpu::KeyAction) -> ContextMenuKeyOutcome {
        let Some(menu) = self.context_menu.as_mut() else {
            return ContextMenuKeyOutcome::Ignored;
        };
        let root = menu.items.clone();
        let path = menu.active.clone();
        match action {
            ui_wgpu::wgpu::KeyAction::Escape => {
                if menu.active.len() > 1 {
                    menu.active.pop();
                    return ContextMenuKeyOutcome::Consumed;
                }
                self.context_menu = None;
                return ContextMenuKeyOutcome::CloseMenu;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if key.len() == 1 && key.chars().next().is_some_and(|ch| ch.is_ascii_digit() && ch != '0') => {
                let ordinal = key.parse::<usize>().ok();
                let Some(ordinal) = ordinal else {
                    return ContextMenuKeyOutcome::Ignored;
                };
                if let Some(next) = context_menu_path_for_ordinal(&root, &path, ordinal) {
                    menu.active = next;
                    return ContextMenuKeyOutcome::Consumed;
                }
                return ContextMenuKeyOutcome::Ignored;
            }
            ui_wgpu::wgpu::KeyAction::ArrowUp => {
                menu.active = context_menu_move_active(&root, &path, false);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::ArrowDown => {
                menu.active = context_menu_move_active(&root, &path, true);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::ArrowLeft => {
                if menu.active.len() > 1 {
                    let parent = menu.active[..menu.active.len() - 1].to_vec();
                    menu.active = parent.clone();
                    menu.submenu_collapsed_at = Some(parent);
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::ArrowRight => {
                if let Some(next) = context_menu_open_submenu_path(&root, &menu.active) {
                    menu.active = next;
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Space(true) => {
                let active = menu.active.clone();
                let Some(item) = context_menu_item_at_path(&root, &active) else {
                    return ContextMenuKeyOutcome::Ignored;
                };
                if item.disabled {
                    return ContextMenuKeyOutcome::Ignored;
                }
                if !item.children.is_empty() {
                    if let Some(next) = context_menu_open_submenu_path(&root, &active) {
                        menu.active = next;
                    }
                    return ContextMenuKeyOutcome::Consumed;
                }
                if let Some(action) = item.action.clone() {
                    self.context_menu = None;
                    return ContextMenuKeyOutcome::Activate(action);
                }
                return ContextMenuKeyOutcome::Ignored;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "w" | "W") => {
                menu.active = context_menu_move_active(&root, &path, false);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "s" | "S") => {
                menu.active = context_menu_move_active(&root, &path, true);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "a" | "A") => {
                if menu.active.len() > 1 {
                    menu.active.pop();
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "d" | "D") => {
                if let Some(next) = context_menu_open_submenu_path(&root, &menu.active) {
                    menu.active = next;
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            _ => ContextMenuKeyOutcome::Ignored,
        }
    }

    /// 🔍️ Every palette row, in React's own declaration order (`🏛️ShellHost/🟦️.tsx`'s `searchItems`
    /// memo): panel tabs, window kinds, resolved commands, then the host-app verbs. Declaration order is
    /// what the ranker breaks ties on and what a cold (empty-query) palette shows, so it is part of the
    /// contract, not an accident.
    fn build_search_items(&self) -> Vec<SearchPaletteItem> {
        let is_de = self.locale_id == "de";
        let Some(session) = &self.session else {
            return Vec::new();
        };
        let mut items = Vec::new();
        for tab in Self::flatten_panel_tab_leaves(&session.app.panel_tabs) {
            items.push(SearchPaletteItem {
                id: format!("panel.{}", tab.id()),
                label: tab.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                description: None,
                group: shell_chrome_string("palette.group.panels", is_de).into(),
                dispatch_action: Some(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab.id() }) }),
                action: None,
                category: None,
            });
        }
        for kind in &session.app.window_kinds {
            items.push(SearchPaletteItem {
                id: format!("window.{}", kind.id),
                label: kind.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                description: None,
                group: shell_chrome_string("palette.group.windows", is_de).into(),
                dispatch_action: None,
                action: Some(format!("window:{}", kind.id)),
                category: None,
            });
        }
        items.extend(self.command_search_items());
        let panel = self.space_mode && Self::panel_state_from_view(&session.view_state).ok().flatten().is_some();
        if panel {
            let controller_id = self.host_controller_id().unwrap_or_default();
            for program in self.build_space_workflows() {
                let breadcrumb = self.plugins.iter().find_map(|plugin| plugin.manifest.apps.iter().find(|app| app.id == program.app_id)).map(|app| shell_navbar_title(app, &self.terminology_id)).unwrap_or_else(|| program.breadcrumb.join(" · "));
                items.push(SearchPaletteItem {
                    id: format!("spawn.{}", program.plugin_id),
                    label: format!("{} {breadcrumb}", shell_chrome_string("palette.spawnPrefix", is_de)),
                    description: None,
                    group: shell_chrome_string("palette.group.catalogue", is_de).into(),
                    dispatch_action: Some(ActionDescriptor { controller_id: controller_id.clone(), action: "spawnApp".into(), args: crate::action_args_json!({ "pluginId": program.plugin_id }) }),
                    action: None,
                    category: None,
                });
            }
            for (id, action, label_key, group_key) in
                [("studio.undo", "undo", "palette.undo", "palette.group.hostApp"), ("studio.redo", "redo", "palette.redo", "palette.group.hostApp"), ("studio.home", "goHome", "palette.goHome", "palette.group.navigation")]
            {
                items.push(SearchPaletteItem {
                    id: id.into(),
                    label: shell_chrome_string(label_key, is_de).into(),
                    description: None,
                    group: shell_chrome_string(group_key, is_de).into(),
                    dispatch_action: Some(ActionDescriptor { controller_id: controller_id.clone(), action: action.into(), args: None }),
                    action: None,
                    category: None,
                });
            }
        }
        items
    }

    /// 🔍️ The palette's visible rows — ranked by the SAME algorithm, fields, weights, threshold and
    /// limit React's `UISearch` passes `rankFuzzyItems` (`🔎️ShellSearch/🟦️.tsx`): `label` ×2,
    /// `description` ×1, the group heading ×0.5, threshold `0.4`, limit `20`.
    pub(crate) fn filtered_search_items(&self) -> Vec<SearchPaletteItem> {
        rank_fuzzy_items(
            self.build_search_items(),
            &self.search_query,
            &[(|item: &SearchPaletteItem| Some(item.label.as_str()), 2.0), (|item: &SearchPaletteItem| item.description.as_deref(), 1.0), (|item: &SearchPaletteItem| Some(item.group.as_str()), 0.5)],
            FUZZY_SEARCH_THRESHOLD,
            FUZZY_SEARCH_LIMIT,
        )
    }

    /// 🔍️ Find-in-content rows — React's `UIFind` ranks its own `UIFindItem[]` through the very same
    /// `rankFuzzyItems` call (same fields/weights/threshold/limit), so this is not a second algorithm.
    pub(crate) fn filtered_find_items(&self) -> Vec<ShellFindItem> {
        rank_fuzzy_items(
            self.find_items.iter().cloned().collect(),
            &self.find_query,
            &[(|item: &ShellFindItem| Some(item.label.as_str()), 2.0), (|item: &ShellFindItem| item.description.as_deref(), 1.0), (|item: &ShellFindItem| item.category.as_deref(), 0.5)],
            FUZZY_SEARCH_THRESHOLD,
            FUZZY_SEARCH_LIMIT,
        )
    }

    /// 🔎️ One bounded command-palette layout authority, consumed unchanged by paint, hit
    /// publication, accessibility and input routing. Its dimensions are React's authored
    /// `CommandDialog` contract: `sm:max-w-lg`, `h-large`, `max-h-layout-command`, group `p-single`,
    /// heading `py-single`, and item `py-tiny`.
    pub(crate) fn command_palette_plan(&self, theme: &Theme, width: f32, height: f32) -> Option<ShellPalettePlan> {
        let kind = match self.overlay_state {
            OverlayState::Search => ShellPaletteKind::Search,
            OverlayState::Find => ShellPaletteKind::Find,
            _ => return None,
        };
        let (query, placeholder, empty, rows, selected) = match kind {
            ShellPaletteKind::Search => {
                let rows = self.filtered_search_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.group)).collect::<Vec<_>>();
                (self.search_query.clone(), shell_chrome_string("search.placeholder", self.locale_id == "de").to_string(), shell_chrome_string("search.empty", self.locale_id == "de").to_string(), rows, self.search_selected)
            }
            ShellPaletteKind::Find => {
                let rows = self.filtered_find_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.category.unwrap_or_default())).collect::<Vec<_>>();
                (self.find_query.clone(), shell_chrome_string("find.placeholder", self.locale_id == "de").to_string(), shell_chrome_string("find.empty", self.locale_id == "de").to_string(), rows, self.find_selected)
            }
        };
        let inset = theme.control_height;
        let dialog_width = SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX.min((width - inset * 2.0).max(1.0));
        let input_height = theme.navbar_height;
        let body_line = theme.font_size_body * 1.5;
        let small_line = theme.font_size_small * (10.0 / 7.0);
        let row_padding = theme.gap_standard * SHELL_PALETTE_TINY_UI_SPACING;
        let list_max = theme.gap_standard * SHELL_PALETTE_LIST_MAX_UI_SPACING;
        let mut groups = Vec::<String>::new();
        for (_, _, _, _, group) in &rows {
            if !groups.iter().any(|existing| existing == group) {
                groups.push(group.clone());
            }
        }
        let mut specs = Vec::<(Option<(usize, String, String, Option<String>, bool)>, Option<String>, f32)>::with_capacity(SHELL_PALETTE_PLAN_ENTRY_CAPACITY - 2);
        if rows.is_empty() {
            specs.push((None, None, body_line + theme.gap_standard * SHELL_PALETTE_MEDIUM_UI_SPACING * 2.0));
        } else {
            for group in groups {
                if !group.is_empty() && specs.len() < SHELL_PALETTE_PLAN_ENTRY_CAPACITY - 1 {
                    specs.push((None, Some(group.clone()), small_line + theme.gap_standard * 3.0));
                }
                for (index, item_id, label, description, row_group) in rows.iter().filter(|(_, _, _, _, row_group)| row_group == &group) {
                    if specs.len() >= SHELL_PALETTE_PLAN_ENTRY_CAPACITY - 1 {
                        break;
                    }
                    let height = body_line + description.as_ref().map_or(0.0, |_| small_line) + row_padding * 2.0;
                    specs.push((Some((*index, item_id.clone(), label.clone(), description.clone(), *index == selected)), None, height));
                }
                if let Some(last) = specs.last_mut() {
                    last.2 += theme.gap_standard;
                }
            }
        }
        let natural_list_height = specs.iter().map(|(_, _, height)| *height).sum::<f32>();
        let viewport_list_height = list_max.min((height - inset * 2.0 - input_height).max(1.0)).min(natural_list_height);
        let dialog_height = input_height + viewport_list_height;
        let dialog = Rect::new((width - dialog_width) * 0.5, (height - dialog_height) * 0.5, dialog_width, dialog_height);
        let list_top = dialog.y + input_height;
        let list_bottom = dialog.y + dialog.h;
        let selected_bounds = specs
            .iter()
            .scan(0.0, |y, (row, _, height)| {
                let top = *y;
                *y += *height;
                Some((row.as_ref().is_some_and(|(_, _, _, _, selected)| *selected), top, *y))
            })
            .find(|(selected, _, _)| *selected);
        let scroll = selected_bounds
            .map(|(_, top, bottom)| {
                if bottom > viewport_list_height {
                    bottom - viewport_list_height
                } else if top < 0.0 {
                    top
                } else {
                    0.0
                }
            })
            .unwrap_or(0.0)
            .clamp(0.0, (natural_list_height - viewport_list_height).max(0.0));
        let mut entries = Vec::with_capacity(SHELL_PALETTE_PLAN_ENTRY_CAPACITY);
        entries.push(ShellPalettePlanEntry::Input { rect: Rect::new(dialog.x, dialog.y, dialog.w, input_height), query, placeholder });
        let mut y = list_top - scroll;
        for (row, group, entry_height) in specs {
            let rect = Rect::new(dialog.x + theme.gap_standard, y, (dialog.w - theme.gap_standard * 2.0).max(1.0), entry_height);
            y += entry_height;
            if rect.y < list_top || rect.y + rect.h > list_bottom {
                continue;
            }
            match (row, group) {
                (Some((index, item_id, label, description, selected)), _) => entries.push(ShellPalettePlanEntry::Row { rect, index, item_id, label, description, selected }),
                (None, Some(label)) => entries.push(ShellPalettePlanEntry::Group { rect, label }),
                (None, None) => entries.push(ShellPalettePlanEntry::Empty { rect, label: empty.clone() }),
            }
        }
        let close_size = input_height.min(theme.control_height);
        entries.push(ShellPalettePlanEntry::Close { rect: Rect::new(dialog.x + dialog.w - close_size, dialog.y, close_size, close_size), label: shell_chrome_string("common.close", self.locale_id == "de").to_string() });
        Some(ShellPalettePlan { kind, dialog, list: Rect::new(dialog.x, list_top, dialog.w, viewport_list_height), entries })
    }

    pub(crate) fn palette_query(&self, kind: ShellPaletteKind) -> &str {
        match kind {
            ShellPaletteKind::Search => &self.search_query,
            ShellPaletteKind::Find => &self.find_query,
        }
    }

    pub(crate) fn set_palette_query(&mut self, kind: ShellPaletteKind, value: String, input: &mut InputState<ActionDescriptor>) {
        match kind {
            ShellPaletteKind::Search => {
                self.search_query = value.clone();
                self.search_selected = 0;
            }
            ShellPaletteKind::Find => {
                self.find_query = value.clone();
                self.find_selected = 0;
            }
        }
        input.focus_input_owned(kind.input_id().to_string(), value);
        self.accessibility_focused_control_id = Some(kind.input_id().to_string());
    }

    pub(crate) fn close_command_palette(&mut self, input: &mut InputState<ActionDescriptor>) {
        if self.pending_chrome_release.as_deref().is_some_and(|id| [ShellPaletteKind::Search, ShellPaletteKind::Find].into_iter().any(|kind| kind.owns_control_id(id))) {
            self.pending_chrome_release = None;
        }
        self.overlay_state = OverlayState::None;
        self.search_open = false;
        self.find_open = false;
        input.blur_input();
        self.accessibility_focused_control_id = None;
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
        let items = self.filtered_search_items();
        Self::debug_log(&format!(
            "[TRACE] wgpu-shell palette activate index={index} query={:?} items={} offered={:?} chose={:?}",
            self.search_query,
            items.len(),
            items.iter().take(8).map(|item| item.id.as_str()).collect::<Vec<_>>(),
            items.get(index).map(|item| (item.id.as_str(), item.action.as_deref()))
        ));
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(action) = item.dispatch_action.clone() {
            self.dispatch_action(action).await?;
        } else if let Some(action) = &item.action {
            if let Some(window_id) = action.strip_prefix("window:") {
                self.active_window_id = Some(window_id.to_string());
            } else if let Some(rest) = action.strip_prefix("action-panel:") {
                if let Some((window_id, action_id)) = rest.split_once(':') {
                    self.active_window_id = Some(window_id.to_string());
                    self.action_panel_folded.insert(window_id.to_string(), false);
                    self.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
                }
            } else if let Some(rest) = action.strip_prefix("os-command:") {
                let (command_id, option_value) = match rest.split_once(':') {
                    Some((command_id, value)) => (command_id.to_string(), Some(value.to_string())),
                    None => (rest.to_string(), None),
                };
                self.apply_os_command(&command_id, option_value.as_deref()).await?;
            } else if let Some(command_json) = action.strip_prefix("command:") {
                let invocation: semio_framework::manifest::CommandInvocation = semio_framework_pack_json::from_json_str(command_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
                self.dispatch_command(invocation).await?;
            } else if let Some(command_key) = action.strip_prefix("command-form:") {
                let command_key = command_key.to_string();
                let category_tab = self.resolved_commands().into_iter().find(|entry| command_address_stable_key(&entry.address) == command_key).map(|entry| format!("{FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX}{}", entry.definition.category));
                let category_tab = category_tab.as_deref().unwrap_or(FRAMEWORK_CATEGORY_COMMAND_ID);
                let previous_expanded = self.expanded_command_id.replace(command_key);
                if let Err(error) = self.republish_shell_panel_document(category_tab) {
                    self.expanded_command_id = previous_expanded;
                    return Err(error);
                }
                self.reveal_dock_tab(category_tab);
            }
        }
        self.search_open = false;
        self.overlay_state = OverlayState::None;
        self.search_query.clear();
        self.search_selected = 0;
        Ok(())
    }

    pub async fn activate_find_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.filtered_find_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(session) = &self.session {
            self.dispatch_action(ActionDescriptor {
                controller_id: session.app.controller_id.clone(),
                action: "setMediaNodeSelection".into(),
                args: crate::action_args_json!({
                    "surfaceId": item.surface_id,
                    "nodeIds": [item.node_id],
                }),
            })
            .await?;
        }
        self.find_open = false;
        self.overlay_state = OverlayState::None;
        self.find_query.clear();
        self.find_selected = 0;
        Ok(())
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
    pub(crate) fn content_is_editing(focused_id: Option<&str>, sync_card_open: bool) -> bool {
        focused_id.is_some_and(|id| !SHELL_OVERLAY_INPUT_IDS.contains(&id)) || sync_card_open
    }

    /// ⎋️ Closes every open DISMISSABLE LAYER — a retained widget's `Select` and the navbar's own
    /// example dropdown — and answers whether one was open.
    ///
    /// ⚖️ React's twin is Radix's `DismissableLayer`: it closes on the Escape KEYDOWN and does not
    /// stop that keydown reaching `handleAppKeydown`, so `example-picker-dismiss` journals the app's
    /// `engagementAbort` in the reference while this target journalled nothing at all
    /// (`🗑️generated/w11a-parity-run-14/steps.json`, ticket 26/09/17/WGPU-RENDERER-REACT-PARITY
    /// `📓️w12c`). It is therefore called BEFORE `handle_keyboard_async` computes `idle` and the
    /// chord carries on down the ladder — the same law W10a gave the context menu's `CloseMenu`.
    ///
    /// 🧯️ The quick-search and find overlays are deliberately NOT dismissable layers: they are
    /// focus-trapping dialogs with a focused query field, which is exactly React's `isEditableTarget`
    /// early return in `handleAppKeydown`, and `idle`'s `focused_id` term already reproduces it.
    fn dismiss_dismissable_layers(&mut self) -> bool {
        let mut dismissed = false;
        if self.open_selects.values().any(|open| *open) {
            self.close_open_selects();
            dismissed = true;
        }
        if matches!(self.overlay_state, OverlayState::Dropdown(_)) {
            self.overlay_state = OverlayState::None;
            dismissed = true;
        }
        dismissed
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
        if self.chrome_build.dialog_open() && self.handle_chrome_dialog_key(&action, modifiers) {
            return;
        }
        if action == ui_wgpu::wgpu::KeyAction::Escape {
            if crate::scenes::cancel_canvas_interactions(input) {
                if let Some((window_id, pointer_id)) = crate::interpreter::retained_any_pointer_capture() {
                    crate::interpreter::dispatch_ui_pointer_event(&window_id, pointer_id, ui_wgpu::wgpu::UiEvent::PointerCancel, input);
                }
                return;
            }
            if crate::scenes::cancel_scene_list_transfer() {
                self.chrome_pointer_owner = None;
                return;
            }
            if self.dock_drag.take().is_some() || self.pending_dock_drag.take().is_some() {
                self.chrome_pointer_owner = None;
                self.pending_chrome_release = None;
                return;
            }
            if self.dismiss_dismissable_layers() {
                return;
            }
            if self.context_menu.is_some() {
                match self.context_menu_handle_key(ui_wgpu::wgpu::KeyAction::Escape) {
                    ContextMenuKeyOutcome::Activate(_) => return,
                    ContextMenuKeyOutcome::Ignored => {}
                    ContextMenuKeyOutcome::Consumed | ContextMenuKeyOutcome::CloseMenu => return,
                }
            }
        }
        if self.sync_card_kind.is_none() {
            if let Some(step) = self.chrome_tour_active_step() {
                match action {
                    ui_wgpu::wgpu::KeyAction::Escape => {
                        if let Some(seen_key) = self.introduction_seen_key() {
                            self.chrome_build.dismiss_introduction(&seen_key);
                        } else {
                            self.chrome_build.skip_introduction();
                        }
                        return;
                    }
                    ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::ArrowRight => {
                        if step.interactions.is_empty() {
                            self.chrome_tour_advance_current_step(&step);
                            return;
                        }
                    }
                    ui_wgpu::wgpu::KeyAction::ArrowLeft => {
                        self.chrome_build.back_introduction();
                        return;
                    }
                    _ => {}
                }
            }
        }
        let editing = Self::content_is_editing(input.focused_id.as_deref(), self.sync_card_kind.is_some());
        let palette_open = matches!(self.overlay_state, OverlayState::Search | OverlayState::Find);
        if self.keybinding_capture_control_id.is_some() {
            if self.capture_keybinding_chord(&action, modifiers) {
                return;
            }
        }
        if !editing {
            if let Some(shortcut) = shell_shortcut_for_in(&self.shortcut_table(), &action, modifiers) {
                if self.apply_sync_shell_shortcut(shortcut, palette_open, input) {
                    return;
                }
            }
        }
        if !editing && !palette_open && self.dock_drag.is_none() && action == ui_wgpu::wgpu::KeyAction::Tab {
            self.advance_keyboard_ring(!modifiers.shift, input);
            return;
        }
        if self.sync_card_kind.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.sync_card_kind = None;
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    self.deferred_actions.push(ActionDescriptor {
                        controller_id: "framework.sync".into(),
                        action: "attach".into(),
                        args: crate::action_args_json!({
                            "path": self.sync_card_draft,
                            "kind": self.sync_card_kind,
                        }),
                    });
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    self.sync_card_draft.push_str(&key);
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    self.sync_card_draft.pop();
                    return;
                }
                _ => {}
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.checkin_dialog_draft.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.checkin_dialog_draft = None;
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    let message = self.checkin_dialog_draft.clone().unwrap_or_default();
                    self.checkin_dialog_draft = None;
                    self.deferred_actions.push(ActionDescriptor { controller_id: "framework.checkin".into(), action: "submit".into(), args: crate::action_args_json!({ "message": message }) });
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    if let Some(draft) = self.checkin_dialog_draft.as_mut() {
                        draft.push_str(&key);
                    }
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    if let Some(draft) = self.checkin_dialog_draft.as_mut() {
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
                    self.close_command_palette(input);
                }
                ui_wgpu::wgpu::KeyAction::ArrowDown => {
                    if self.overlay_state == OverlayState::Search {
                        let len = self.filtered_search_items().len();
                        if len > 0 {
                            self.search_selected = (self.search_selected + 1).min(len - 1);
                        }
                    } else {
                        let len = self.filtered_find_items().len();
                        if len > 0 {
                            self.find_selected = (self.find_selected + 1).min(len - 1);
                        }
                    }
                }
                ui_wgpu::wgpu::KeyAction::ArrowUp => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_selected = self.search_selected.saturating_sub(1);
                    } else {
                        self.find_selected = self.find_selected.saturating_sub(1);
                    }
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    let runtime = ();
                    let _ = runtime;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    let kind = if self.overlay_state == OverlayState::Search { ShellPaletteKind::Search } else { ShellPaletteKind::Find };
                    let mut query = self.palette_query(kind).to_string();
                    query.push_str(&key);
                    self.set_palette_query(kind, query, input);
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    let kind = if self.overlay_state == OverlayState::Search { ShellPaletteKind::Search } else { ShellPaletteKind::Find };
                    let mut query = self.palette_query(kind).to_string();
                    query.pop();
                    self.set_palette_query(kind, query, input);
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
        if self.chrome_build.dialog_open() {
            self.handle_keyboard(action, modifiers, input);
            self.owe_settle();
            return Ok(());
        }
        if self.context_menu.is_some() {
            match self.context_menu_handle_key(action.clone()) {
                ContextMenuKeyOutcome::Ignored | ContextMenuKeyOutcome::CloseMenu => {}
                ContextMenuKeyOutcome::Consumed => return Ok(()),
                ContextMenuKeyOutcome::Activate(descriptor) => {
                    self.dispatch_action(descriptor).await?;
                    return Ok(());
                }
            }
        }
        if matches!(action, ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Space(true)) {
            if let Some(target) = self.keyboard_focused_chrome_target(input) {
                self.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, input).await?;
                return Ok(());
            }
        }
        if matches!(self.overlay_state, OverlayState::Search) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_search_item(self.search_selected).await?;
            input.blur_input();
            self.accessibility_focused_control_id = None;
            return Ok(());
        }
        if matches!(self.overlay_state, OverlayState::Find) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_find_item(self.find_selected).await?;
            input.blur_input();
            self.accessibility_focused_control_id = None;
            return Ok(());
        }
        if Self::content_is_editing(input.focused_id.as_deref(), false) {
            match action {
                ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Escape => {
                    self.commit_focused_input(input).await?;
                    return Ok(());
                }
                _ => {}
            }
        }
        if action == ui_wgpu::wgpu::KeyAction::Escape {
            self.dismiss_dismissable_layers();
        }
        let idle = input.focused_id.is_none() && self.overlay_state == OverlayState::None && self.sync_card_kind.is_none() && self.dock_drag.is_none();
        Self::debug_log(&format!(
            "[TRACE] wgpu-shell chord gate action={action:?} idle={idle} focused={:?} overlay={:?} syncCard={} dockDrag={} menu={} window={:?} session={:?} editVerb={:?} appBinding={:?} reserved={} docked={}",
            input.focused_id,
            self.overlay_state,
            self.sync_card_kind.is_some(),
            self.dock_drag.is_some(),
            self.context_menu.is_some(),
            self.active_window_id,
            self.session.as_ref().map(|session| session.app.controller_id.clone()),
            shell_edit_verb_for(&action, modifiers).map(ShellEditVerb::action_id),
            self.match_app_keybinding(&action, modifiers).map(|descriptor| descriptor.action),
            is_reserved_shell_chord_in(&self.shortcut_table(), &action, modifiers),
            self.dock.window_instances().len()
        ));
        if idle {
            if let Some(shortcut) = shell_shortcut_for_in(&self.shortcut_table(), &action, modifiers) {
                if shortcut.is_async() && self.apply_async_shell_shortcut(shortcut).await? {
                    return Ok(());
                }
            }
        }
        if idle && !is_reserved_shell_chord_in(&self.shortcut_table(), &action, modifiers) {
            let platform = command_host_platform();
            let command = self
                .resolved_commands()
                .into_iter()
                .rev()
                .find(|entry| entry.definition.in_palette && entry.definition.keybindings.iter().any(|binding| binding.platform.is_none_or(|declared| declared == platform) && key_event_matches_chord(&action, modifiers, &binding.chord)));
            if let Some(entry) = command {
                if entry.definition.args.is_empty() {
                    if matches!(entry.address.owner, semio_framework::manifest::CommandOwnerAddress::Os) {
                        self.apply_os_command(&entry.definition.id, None).await?;
                    } else {
                        let invocation = semio_framework::manifest::CommandInvocation { address: entry.address, arguments: BTreeMap::new() };
                        self.dispatch_command(invocation).await?;
                    }
                } else {
                    self.overlay_state = OverlayState::Search;
                    self.search_query = entry.definition.label.resolve(self.active_terminology(), self.active_locale()).to_string();
                    self.search_selected = 0;
                }
                return Ok(());
            }
        }
        if idle {
            if let Some((surface, node)) = self.retained_keyboard_focus() {
                Self::debug_log(&format!("[TRACE] wgpu-shell key routing surface={surface} node={node:?} action={action:?}"));
                let focused_select = crate::interpreter::retained_node_is_select(&surface, node);
                if let Some(event) = ui_event_from_key_action(&action, modifiers, focused_select) {
                    let commands = crate::interpreter::dispatch_ui_event(&surface, event, input);
                    self.chrome_build.note_content_focus_commands(&commands);
                    return Ok(());
                }
            }
        }
        if idle && action == ui_wgpu::wgpu::KeyAction::Escape {
            if let Some(window_id) = self.active_window_id.clone() {
                if self.active_utility_by_window.remove(&window_id).is_some() {
                    self.refresh_ui(UiDirtyScope::Full).await?;
                    return Ok(());
                }
            }
            let armed_tool = self.session.as_ref().and_then(|session| session.view_state.active_tool_id.clone()).is_some_and(|tool_id| !tool_id.is_empty());
            if armed_tool {
                let controller_id = self.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
                self.dispatch_action(ActionDescriptor { controller_id, action: "setActiveTool".into(), args: crate::action_args_json!({ "toolId": "" }) }).await?;
                return Ok(());
            }
        }
        if idle {
            if let Some(descriptor) = self.match_app_keybinding(&action, modifiers) {
                if self.dispatch_app_keybinding(descriptor).await? {
                    self.drain_gesture_bound_work().await;
                    self.owe_settle();
                    return Ok(());
                }
            }
        }
        if idle {
            if let Some(verb) = shell_edit_verb_for(&action, modifiers) {
                let controller_id = self.session.as_ref().map(|session| session.app.controller_id.clone());
                if let Some(controller_id) = controller_id {
                    Self::debug_log(&format!("[TRACE] wgpu-shell edit chord verb={} controller={controller_id}", verb.action_id()));
                    self.dispatch_action(ActionDescriptor { controller_id, action: verb.action_id().into(), args: None }).await?;
                    self.owe_settle();
                    return Ok(());
                }
            }
        }
        self.handle_keyboard(action, modifiers, input);
        self.drain_gesture_bound_work().await;
        self.owe_settle();
        Ok(())
    }

    /// ⌨️ The app keybinding matching the current key event, if any.
    ///
    /// 🩸️ A binding's `keys` is a COMMA-SEPARATED chord list — React's `handleAppKeydown` splits it
    /// (`parseKeys`) and puzzle2d's `deleteSelection` declares `"delete,backspace"`. This target fed
    /// the whole string to [`key_event_matches_chord`], which splits on `+` alone, so every
    /// multi-chord binding in the repo was dead on this renderer: `"delete,backspace"` resolved the
    /// key token `"backspace"` with a literal `delete,` prefix and matched neither key.
    ///
    /// 🛡️ The reserved check is PER CHORD, exactly where React puts it (`if (reservedChords.has(chord))
    /// continue`), so a binding whose second chord is the shell's still fires on its first.
    fn match_app_keybinding(&self, action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ActionDescriptor> {
        let session = self.session.as_ref()?;
        let reserved = reserved_shell_chords_v1(&self.shortcut_table(), &[]);
        session
            .app
            .keybindings
            .iter()
            .find(|binding| binding.keys.split(',').map(|chord| chord.trim().to_ascii_lowercase()).filter(|chord| !chord.is_empty()).any(|chord| key_event_matches_chord(action, modifiers, &chord) && !reserved.contains(&chord)))
            .map(|binding| binding.action.clone())
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
    async fn dispatch_app_keybinding(&mut self, descriptor: ActionDescriptor) -> Result<bool, String> {
        let Some(session) = self.session.clone() else {
            return Ok(false);
        };
        let mounted: Vec<WindowScopeInstanceV1> = self.dock.window_instances().into_iter().map(|(id, window_kind_id)| WindowScopeInstanceV1 { id, window_kind_id }).collect();
        let focused = self.active_window_id.clone().or_else(|| session.view_state.active_window_kind_id.clone());
        let (target_kind, target_window_id) = resolve_keybinding_target_window_v1(&session.app, &mounted, focused.as_deref(), &descriptor.action);
        let Some(window_id) = target_window_id else {
            let label = session
                .app
                .window_kinds
                .iter()
                .flat_map(|kind| kind.actions.iter())
                .find(|action| action.id == descriptor.action)
                .map_or_else(|| descriptor.action.clone(), |action| action.label.resolve(self.active_terminology(), self.active_locale()).to_string());
            let chord = session.app.keybindings.iter().find(|binding| binding.action.action == descriptor.action).map_or_else(String::new, |binding| binding.keys.clone());
            let text = keybinding_unowned_text_v1(self.active_locale().as_str(), &chord, &label);
            self.show_transient_notice(text, semio_framework::Severity::Info, Some(KEYBINDING_UNOWNED_CODE));
            return Ok(false);
        };
        if target_kind == WindowScopeTargetKindV1::Owner {
            self.dock.sync_active_window(&window_id);
            self.active_window_id = Some(window_id.clone());
        }
        let window_kind_id = mounted.iter().find(|instance| instance.id == window_id).map_or_else(|| window_id.clone(), |instance| instance.window_kind_id.clone());
        let action_def = window_action_definition(&session.app, &window_kind_id, &descriptor.action).cloned();
        let has_args = action_def.as_ref().is_some_and(|action| !action.args.is_empty());
        if !has_args {
            self.dispatch_action(descriptor).await?;
            return Ok(true);
        }
        let action_id = action_def.expect("checked has_args").id;
        let already_expanded = self.active_window_id.as_deref() == Some(window_id.as_str()) && self.action_panel_expanded.get(&window_id).map(String::as_str) == Some(action_id.as_str());
        if already_expanded {
            self.execute_staged_action(&window_id, &action_id).await?;
        } else {
            self.active_window_id = Some(window_id.clone());
            self.action_panel_folded.insert(window_id.clone(), false);
            self.action_panel_expanded.insert(window_id, action_id);
            self.refresh_ui(UiDirtyScope::Full).await?;
        }
        Ok(true)
    }

    pub(crate) fn push_uri(&mut self, uri: String) {
        self.uri_history.truncate(self.uri_index + 1);
        self.uri_history.push(uri);
        self.uri_index = self.uri_history.len().saturating_sub(1);
    }

    /// 🪟️ Read-only walk of the dock tree collecting `(stack path, window id)` in tab/visual order
    /// (depth-first Row/Column child order, each Stack's own tab order). Duplicated locally rather than
    /// reusing `dock`'s own module-private `find_stack_path`-style traversal, because that helper is
    /// private to the `dock` module and this region (`shell::ShellInput`) must not edit `dock` to expose
    /// a public equivalent (region-claims.json `must_not_touch`). `DockNode`'s variants/fields are
    /// public, so this is a plain read of already-public state, not a layering violation.
    pub(crate) fn dock_window_order(node: &crate::dock::DockNode, path: &mut Vec<usize>, out: &mut Vec<(Vec<usize>, String)>) {
        match node {
            crate::dock::DockNode::Stack { windows, .. } => {
                for tab in windows {
                    out.push((path.clone(), tab.window_id.clone()));
                }
            }
            crate::dock::DockNode::Row(children) | crate::dock::DockNode::Column(children) => {
                for (index, (child, _)) in children.iter().enumerate() {
                    path.push(index);
                    Self::dock_window_order(child, path, out);
                    path.pop();
                }
            }
        }
    }

    /// ⌨️ The native shell's keyboard ring — React's DOM order has no twin without a DOM, so it is the chrome's
    /// accessibility publication (every focusable, enabled, visible control, in reading order) followed by the
    /// dock's windows. The keyboard, a platform assistive technology and the browser mirror therefore reach
    /// exactly the same controls (ticket 26/09/23 slice WG10).
    pub(crate) fn keyboard_ring(&self) -> Vec<ShellKeyboardStop> {
        let mut ring: Vec<ShellKeyboardStop> = self.presented_chrome_accessibility.iter().filter(|node| node.focusable && !node.disabled && !node.hidden).map(|node| ShellKeyboardStop::Chrome(node.key.clone())).collect();
        let mut order = Vec::new();
        Self::dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        ring.extend(order.into_iter().map(|(path, window_id)| ShellKeyboardStop::Window(path, window_id)));
        ring
    }

    /// ⌨️ Moves keyboard focus one stop along the ring: onto a chrome control (the accessibility focus the
    /// platform adapter and the mirror set, painted as the focus ring) or onto a dock window (the active
    /// window, whose own content focus takes the keyboard again).
    fn advance_keyboard_ring(&mut self, forward: bool, input: &mut InputState<ActionDescriptor>) {
        let ring = self.keyboard_ring();
        if ring.is_empty() {
            return;
        }
        let current = match self.accessibility_focused_control_id.as_deref() {
            Some(key) => ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Chrome(control) if control == key)),
            None => self.active_window_id.as_deref().and_then(|active| ring.iter().position(|stop| matches!(stop, ShellKeyboardStop::Window(_, window) if window == active))),
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
                self.chrome_build.focused_retained_surface = None;
                self.accessibility_focused_control_id = Some(key);
            }
            ShellKeyboardStop::Window(path, window_id) => {
                self.accessibility_focused_control_id = None;
                self.dock.set_stack_active(&path, &window_id);
                self.dock.sync_active_window(&window_id);
                self.active_window_id = Some(window_id);
            }
        }
    }

    /// ⌨️ The chrome control the keyboard ring focused, addressed the way an accessibility event addresses it —
    /// `None` when focus is on a window, on a text field, or on a control the presented chrome no longer carries.
    fn keyboard_focused_chrome_target(&self, input: &InputState<ActionDescriptor>) -> Option<ui_render::AccessibilityTarget> {
        let key = self.accessibility_focused_control_id.as_deref().filter(|_| input.focused_id.is_none())?;
        let node = self.presented_chrome_accessibility.iter().find(|node| node.key == key && node.focusable && !node.disabled)?;
        Some(ui_render::AccessibilityTarget { window_id: crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: self.presented_chrome_accessibility_generation, node_id: node.node_id, node_key: node.key.clone() })
    }

    /// ⌨️🖼️ Where the keyboard focus ring goes this frame: around the focused chrome control's own hit rect.
    pub(crate) fn keyboard_focus_ring_rect(&self, input: &InputState<ActionDescriptor>) -> Option<Rect> {
        let key = self.accessibility_focused_control_id.as_deref()?;
        input.staged_hits().iter().rev().find(|hit| hit.control_id.as_deref() == Some(key)).map(|hit| hit.rect)
    }

    //#region ⌨️ShellShortcutDispatch
    /// 🪟️ The dock stack path holding `window_id`, read off the same order walk Tab cycling uses.
    pub(crate) fn dock_stack_path_for(&self, window_id: &str) -> Option<crate::dock::DockPath> {
        let mut order = Vec::new();
        Self::dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        order.into_iter().find(|(_, id)| id == window_id).map(|(path, _)| path)
    }

    /// 🧭️ Toggles one named panel anchor — the wgpu twin of React's per-anchor `SET_PANEL_VISIBLE`
    /// toggle that `ui.shell.panelAnchor.*` binds. An anchor carrying no tab is a witnessed no-op
    /// (`false`) rather than an empty box appearing over the canvas.
    fn toggle_panel_anchor(&mut self, anchor: PanelAnchor) -> bool {
        if self.dock_tabs.tabs(anchor).is_empty() {
            return false;
        }
        self.toggle_anchor(anchor);
        true
    }

    /// 🧭️ Opens this anchor at its remembered (or default) path, or folds it shut when it is already open.
    pub fn toggle_anchor(&mut self, anchor: PanelAnchor) {
        if self.anchor_open(anchor) {
            self.anchor_state_mut(anchor).visible = false;
            return;
        }
        let path = self.dock_tabs.reconcile_path(anchor, &self.anchor_state(anchor).path.clone());
        let state = self.anchor_state_mut(anchor);
        state.path = path;
        state.visible = true;
    }

    /// 🧭️ `⌘️B`/`⌘️⇧️B` — folds a whole column shut when any of its anchors is open, else reopens its
    /// top corner, the column-level chord the two former `left_panel_open`/`right_panel_open` flags carried.
    pub fn toggle_column(&mut self, left: bool) {
        let anchors = Self::column_anchors(left);
        if anchors.into_iter().any(|anchor| self.anchor_open(anchor)) {
            for anchor in anchors {
                self.anchor_state_mut(anchor).visible = false;
            }
            return;
        }
        self.toggle_anchor(anchors[0]);
    }

    /// 🪟️ Closes one window wherever it sits — the ONE close lane React's `closeWindow(windowId)`
    /// (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1475-1485`) is: the tab's own chip, the `mod+shift+w` chord
    /// and any future caller collapse the layout by the same rule, refocus `children[0]`, and tear
    /// down whatever the window was hosting.
    pub(crate) fn close_dock_window(&mut self, window_id: &str) -> bool {
        if self.dock.window_kind_id(window_id).is_none() {
            return false;
        }
        if !crate::interpreter::request_ui_document_close(window_id) {
            return false;
        }
        if !self.dock.close_window(window_id) {
            return false;
        }
        self.window_icon_overrides.remove(window_id);
        self.window_title_overrides.remove(window_id);
        self.active_window_id = self.dock.active_window_id.clone();
        crate::scenes::request_canvas_pointer_gesture_cancel_for_window(window_id);
        crate::scenes::cancel_scene_list_transfer_for_window(window_id);
        self.teardown_spawned_window(window_id);
        self.persist_dock_layout();
        self.owe_window_topology_refresh();
        true
    }

    /// 🪶️ Destroys the plugin instance a closed window was hosting and drops its panel entry —
    /// React's `onWindowClose` (`🏛️ShellHost/🟦️.tsx:10533-10555`). Without it the guest instance ran
    /// forever behind a window that no longer exists (the teardown gap
    /// REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT documented). A window that hosts no spawned app is
    /// simply nothing to tear down.
    fn teardown_spawned_window(&mut self, window_id: &str) {
        let Some(session) = self.session.as_ref() else {
            return;
        };
        let Ok(Some(mut panel)) = Self::panel_state_from_view(&session.view_state) else {
            return;
        };
        let Some(closed) = Self::take_spawned_entry(&mut panel, window_id) else {
            return;
        };
        if let Ok(json) = Self::panel_json(&panel) {
            if let Some(session) = self.session.as_mut() {
                session.view_state.panel_json = Some(json);
            }
        }
        if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == closed.plugin_id) {
            program.destroy_app(closed.instance_id);
            Self::debug_log(&format!("[TRACE] wgpu-shell closed window {window_id} destroyed spawned {} instance {}", closed.plugin_id, closed.instance_id));
        }
    }

    /// 🪶️ Drops the spawned-app entry a closed window owned and re-points `active_spawned_id` at
    /// whatever is left — the pure half of [`Self::teardown_spawned_window`], mirroring React's
    /// `buildSpacePanelState(nextSpawned, tab, nextSpawned[0]?.id)` (`🏛️ShellHost/🟦️.tsx:10537`).
    pub(crate) fn take_spawned_entry(panel: &mut SpacePanelState, window_id: &str) -> Option<SpawnedAppEntry> {
        let slot = panel.spawned_apps.iter().position(|entry| entry.id == window_id)?;
        let closed = panel.spawned_apps.remove(slot);
        panel.active_spawned_id = panel.spawned_apps.first().map(|entry| entry.id.clone());
        Some(closed)
    }

    /// 🪟️ `mod+shift+w` — closes the active window through [`Self::close_dock_window`], last window of
    /// the mode included.
    fn close_active_window(&mut self) -> bool {
        let Some(window_id) = self.active_window_id.clone() else {
            return false;
        };
        self.close_dock_window_and_arm(&window_id, "ui.window.close")
    }

    pub(crate) fn close_dock_window_and_arm(&mut self, window_id: &str, control_id: &str) -> bool {
        if !self.close_dock_window(window_id) {
            return false;
        }
        self.arm_control_command(control_id, Some(serde_json::json!({ "windowId": window_id })));
        true
    }

    /// 🪟️ `mod+shift+enter` — the same maximize toggle the window's own focus control performs. React
    /// gates it on `canMaximize` (more than one window in the mode); a single-window mode has nothing to
    /// maximize away from, so this refuses there too.
    pub(crate) fn focus_active_window(&mut self) -> bool {
        let Some(window_id) = self.active_window_id.clone() else {
            return false;
        };
        let mut order = Vec::new();
        Self::dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        if order.len() <= 1 {
            return false;
        }
        let Some(path) = order.into_iter().find(|(_, id)| *id == window_id).map(|(path, _)| path) else {
            return false;
        };
        self.dock.set_stack_active(&path, &window_id);
        self.dock.toggle_maximize(&path);
        true
    }

    /// 🪟️ `mod+shift+n` — "Open in New Window" is a second INSTANCE of the active window's kind split to
    /// the right of its stack, never an OS window: the exact shape of React's `onWindowOpenInNewWindow`
    /// (`🏛️ShellHost/🟦️.tsx`), down to the `{kindId}-{n}` instance id and the right-hand split.
    fn open_active_window_in_new_window(&mut self) -> bool {
        let Some(window_id) = self.active_window_id.clone() else {
            return false;
        };
        let Some(session) = self.session.as_ref() else {
            return false;
        };
        let Some(window_kind_id) = self.live_window_kind_id(session, &window_id).map(str::to_string) else {
            return false;
        };
        let Some(body_key) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).map(|kind| kind.body_key.clone()) else {
            return false;
        };
        if !self.window_topology_publication_has_credit() {
            return false;
        }
        let taken: Vec<String> = self.dock.window_instances().into_iter().map(|(id, _)| id).collect();
        let instance_id = (2..).map(|index| format!("{window_kind_id}-{index}")).find(|candidate| !taken.contains(candidate)).expect("an unbounded counter always clears a finite dock roster");
        let Some(path) = self.dock_stack_path_for(&window_id) else {
            return false;
        };
        if !self.dock.split_stack_with_instance(&path, &instance_id, &window_kind_id, crate::dock::DockSide::Right) {
            return false;
        }
        self.active_window_id = Some(instance_id.clone());
        self.persist_dock_layout();
        self.commit_window_topology_publication(&instance_id, &body_key, None);
        true
    }

    /// 🛠️ A selected `tool.<id>` leaf IS that tool's activation — React's `reconcileToolTabSelection`,
    /// whose whole point is that activation and selection are one state with two representations, so
    /// every route into the path (a press, a restored arrangement, a tour step) ends at the same place.
    /// A path that selects no tool leaf is left alone rather than deactivating, so browsing another
    /// category keeps the armed tool live exactly as React's "skip while not the active root" rule does.
    pub(crate) async fn arm_tool_for_panel_tab(&mut self, path: &[String]) -> Result<(), String> {
        let Some(tool_id) = path.last().and_then(|id| id.strip_prefix(FRAMEWORK_TOOL_PANEL_TAB_PREFIX)).map(ToOwned::to_owned) else {
            return Ok(());
        };
        let Some(controller_id) = self.session.as_ref().map(|session| session.app.controller_id.clone()) else {
            return Ok(());
        };
        if self.session.as_ref().and_then(|session| session.view_state.active_tool_id.as_deref()) == Some(tool_id.as_str()) {
            return Ok(());
        }
        self.dispatch_action(ActionDescriptor { controller_id, action: "setActiveTool".into(), args: crate::action_args_json!({ "toolId": tool_id }) }).await
    }

    /// ⌨️ This session's REMAPPED shortcut table — [`SHELL_SHORTCUT_ROWS`] with the persisted
    /// `keybindingOverrides` applied. Every key-event reader goes through this, so a remap takes effect
    /// for dispatch, for reservation and for the shortcut rows W1c paints, from one source.
    pub(crate) fn shortcut_table(&self) -> Vec<(String, String)> {
        shell_shortcut_table(&self.chrome_build.preferences.keybinding_overrides)
    }

    /// ⌨️ Writes the armed Keybindings row's override from one key event — the wgpu twin of React's
    /// `chordFromKeyboardEvent`: the fixed `ctrl, meta, alt, shift` modifier order then the key, with
    /// Escape cancelling the capture and a bare modifier press ignored. `true` when the event was
    /// consumed by the capture (which includes the Escape cancel).
    pub(crate) fn capture_keybinding_chord(&mut self, action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> bool {
        let Some(control_id) = self.keybinding_capture_control_id.clone() else { return false };
        if matches!(action, ui_wgpu::wgpu::KeyAction::Escape) {
            self.keybinding_capture_control_id = None;
            return true;
        }
        let Some(key) = keybinding_capture_key_token(action) else { return true };
        let mut parts: Vec<&str> = Vec::new();
        if modifiers.ctrl {
            parts.push("ctrl");
        }
        if modifiers.meta {
            parts.push("meta");
        }
        if modifiers.alt {
            parts.push("alt");
        }
        if modifiers.shift {
            parts.push("shift");
        }
        let chord = parts.into_iter().chain(std::iter::once(key.as_str())).collect::<Vec<_>>().join("+");
        self.chrome_build.preferences.keybinding_overrides.insert(control_id.clone(), chord.clone());
        self.keybinding_capture_control_id = None;
        Self::debug_log(&format!("[TRACE] wgpu-shell keybinding override {control_id} -> {chord}"));
        true
    }

    /// ⌨️ Applies the shell verbs that are pure chrome-state transitions (no guest round-trip, no os
    /// command) — the synchronous half of the one [`SHELL_SHORTCUT_ROWS`] table. Returns whether the key
    /// was consumed; an unconsumed key falls through to the rest of `handle_keyboard` untouched.
    fn apply_sync_shell_shortcut(&mut self, shortcut: ShellShortcut, palette_open: bool, input: &mut InputState<ActionDescriptor>) -> bool {
        match shortcut {
            ShellShortcut::ToggleSearch => {
                self.search_open = !self.search_open;
                self.find_open = false;
                self.overlay_state = if self.search_open { OverlayState::Search } else { OverlayState::None };
                self.search_selected = 0;
                if self.search_open {
                    let query = self.search_query.clone();
                    self.set_palette_query(ShellPaletteKind::Search, query, input);
                } else {
                    self.close_command_palette(input);
                }
                Self::debug_log(&format!("[TRACE] wgpu-shell palette chord search-open={} overlay={:?} focused={:?}", self.search_open, self.overlay_state, input.focused_id));
                true
            }
            ShellShortcut::ToggleFind => {
                self.find_open = !self.find_open;
                self.search_open = false;
                self.overlay_state = if self.find_open { OverlayState::Find } else { OverlayState::None };
                self.find_selected = 0;
                if self.find_open {
                    let query = self.find_query.clone();
                    self.set_palette_query(ShellPaletteKind::Find, query, input);
                } else {
                    self.close_command_palette(input);
                }
                true
            }
            ShellShortcut::NavBack => {
                if palette_open {
                    return false;
                }
                self.uri_index = self.uri_index.saturating_sub(1);
                self.pending_shell_uri_apply = true;
                true
            }
            ShellShortcut::NavForward => {
                if palette_open {
                    return false;
                }
                if self.uri_index + 1 < self.uri_history.len() {
                    self.uri_index += 1;
                }
                self.pending_shell_uri_apply = true;
                true
            }
            ShellShortcut::NavUp => {
                if palette_open {
                    return false;
                }
                let uri = self.shell_uri();
                if let Some(parent) = uri.rsplit_once('/').map(|(parent, _)| parent.to_string()) {
                    if !parent.is_empty() {
                        self.push_uri(parent);
                    }
                }
                self.pending_shell_uri_apply = true;
                true
            }
            ShellShortcut::TogglePanelAnchor(anchor) => {
                if palette_open {
                    return false;
                }
                self.toggle_panel_anchor(anchor)
            }
            ShellShortcut::CloseWindow => !palette_open && self.close_active_window(),
            ShellShortcut::FocusWindow => !palette_open && self.focus_active_window(),
            ShellShortcut::NewWindow => !palette_open && self.open_active_window_in_new_window(),
            ShellShortcut::ModeStep(_) | ShellShortcut::SurfaceRole(_) | ShellShortcut::TimeTravel(_) => false,
        }
    }

    /// ⌨️ Applies the shell verbs that need the async funnel — the other half of the same table.
    async fn apply_async_shell_shortcut(&mut self, shortcut: ShellShortcut) -> Result<bool, String> {
        match shortcut {
            ShellShortcut::ModeStep(step) => {
                self.apply_mode_step(step).await?;
                Ok(true)
            }
            ShellShortcut::SurfaceRole(role) => {
                self.switch_to_session_role(role).await?;
                Ok(true)
            }
            ShellShortcut::TimeTravel(verb) => match self.time_travel_shortcut_action(verb) {
                Some(action) => {
                    self.dispatch_action(action).await?;
                    Ok(true)
                }
                None => Ok(false),
            },
            _ => Ok(false),
        }
    }
    //#endregion ⌨️ShellShortcutDispatch
}

#[cfg(test)]
#[path = "../../../🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs"]
mod shell_shortcuts_palette_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs"]
mod chord_example_role_parity_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🧲️engine-surface-retention/🦀️.rs"]
mod engine_surface_retention_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎯️wgpu-pointer-hit-ownership/🦀️.rs"]
mod pointer_hit_ownership_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs"]
mod wheel_and_escape_routing_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🛟️panel-window-reservation/🦀️.rs"]
mod panel_window_reservation_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs"]
mod dock_close_reopen_journal_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/👕️board-presence/🦀️.rs"]
mod board_presence_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🚗️driver-editor/🦀️.rs"]
mod driver_editor_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🔗️hub-projection-workspace/🦀️.rs"]
mod hub_projection_workspace_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs"]
mod window_lifecycle_template_drag_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/⚙️settings-general-layout/🦀️.rs"]
mod settings_general_layout_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🔄️detached-sync-task-manager/🦀️.rs"]
mod detached_sync_task_manager_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔄️sync-owner-lifetime/🦀️.rs"]
mod sync_owner_lifetime_tests;
//#endregion ShellInput
