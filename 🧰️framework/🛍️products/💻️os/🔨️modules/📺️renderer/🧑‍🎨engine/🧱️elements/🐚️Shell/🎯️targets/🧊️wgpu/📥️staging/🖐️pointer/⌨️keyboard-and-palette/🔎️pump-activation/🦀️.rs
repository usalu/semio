//! 🔎️ Pump cross and activation: the editing check, the cross settle pump, the extension invocation, the widget drag values, the focused input commit, the tree drag finish, the panel tab, the search and find activation, the app keybindings and the shortcut dispatch. See [`super`].

use super::*;

//#region ShellInput
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
