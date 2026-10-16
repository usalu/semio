//! 🍱️ `shell-view-chrome` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl<'ctx> ChromeViewCtx<'ctx> {
    /// 🖱️ One chrome pointer button, dispatched and RETURNED — the chain it armed converges on the
    /// frame loop's settle lane ([`Self::settle_pump_step`]), never inside this turn.
    ///
    /// 🩸️ This used to end in `flush_deferred_actions`, whose `settle_ui_chain` runs up to
    /// `SHELL_SETTLE_ROUNDS` whole-shell refresh passes to a fixed point. The renderer lends its ONE
    /// interaction state to a dispatch for the length of the dispatch's future, so a gesture whose
    /// chain re-solved the graph held that state for the whole convergence and no later input was
    /// dispatched at all. Measured on 6118 (`📓️wgpu-generate-add-port-fit-2026-09-14.md`): a stray
    /// port drag rewired the graph, one pointer move then held `dispatch-event` for **8 515 ms**
    /// across 6 refresh passes, and the `F` the user pressed 1.1 s later reached
    /// `fit_node_graph_camera` **7 632 ms** after the key went down.
    pub async fn handle_pointer_button(&mut self, x: f32, y: f32, down: bool, button: i16, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> Result<(), String> {
        self.handle_pointer_button_for(ui_render::PointerId(1), x, y, down, button, input, theme).await
    }

    /// 📑️ React's tab label is a `<button onClick={() => onSelectTab(tab.id)}>`: the select
    /// lands on RELEASE over the same label, and it is addressed by window ID, never by the
    /// stack path the chip happens to be painted at — the mobile flat stack renders every tab
    /// at the root path.
    ///
    /// 🖱️ A cross-surface transfer must deliver the semantic release to the CURRENT
    /// destination and also retire EventRouter capture on the SOURCE. The destination
    /// runs first and consumes the generation-owned transfer; the second up is outside
    /// the captured source's geometry, so it can only clear its pressed/capture state —
    /// it cannot emit a second drop or source click.
    ///
    /// 📋️ Alt + secondary press on a text editor is the completions gesture, not a menu —
    /// React's own `onContextMenu` takes that branch first (`✏️TextEditor/🟦️.tsx:413`).
    ///
    /// 🌫️ Pressing the mode canvas where no chrome and no window body answers is React's
    /// `handleCanvasBackgroundPointerDown` on `[data-slot="mode-body"]` itself
    /// (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1461-1471`): the mode keeps its layout and loses its focus.
    ///
    /// 🎯️ A retained window body owns its own targets end to end: the node's declared action
    /// fires through `dispatch_action`, a focusable control is routed into that body's
    /// `events::EventRouter`, and none of the chrome id conventions below can claim it.
    ///
    /// 🌫️ A resize gutter is canvas BACKGROUND as far as focus goes: React's
    /// `handleCanvasBackgroundPointerDown` deactivates on any `data-slot="resizable-panel"`
    /// press (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1461-1471`) and still starts the resize.
    ///
    /// 🎬️ Only the tab's grip chip starts a drag, exactly like React's `DragHandle`
    /// `onPointerDown` (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1101`); the label beside it selects
    /// on release and never travels.
    pub async fn handle_pointer_button_for(&mut self, pointer_id: ui_render::PointerId, x: f32, y: f32, down: bool, button: i16, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> Result<(), String> {
        debug_log(&format!(
            "[TRACE] wgpu-shell pointer button x={x} y={y} down={down} button={button} targets={} staged={} gen={} hit={:?}",
            input.hits().len(),
            input.staged_hits().len(),
            input.hit_generation(),
            input.hit_at(x, y).map(|target| (target.kind, target.control_id.clone(), target.event.as_ref().map(|descriptor| descriptor.action.clone())))
        ));
        if !down && self.input.chrome_pointer_owner.is_some_and(|owner| owner != pointer_id) {
            return Ok(());
        }
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.pointer_button = button;
        if !down {
            if self.input.chrome_pointer_owner == Some(pointer_id) {
                self.input.chrome_pointer_owner = None;
            }
            let retained_canvas_drag = crate::interpreter::active_retained_drag_sessions().into_iter().map(|(_, _, _, drag_data)| drag_data).find(|drag_data| drag_data.contains_key(crate::scenes::CANVAS_CATALOGUE_DRAG_MIME));
            if let Some((tab_id, from_anchor)) = self.dock.dock_tab_drag.take() {
                self.finish_dock_tab_drag(&tab_id, from_anchor, x, y, theme);
            }
            let dock_drop_result = if self.dock.dock_drag.is_some() {
                self.as_pointer().finish_dock_drag(x, y, input).await
            } else {
                self.dock.pending_dock_drag = None;
                Ok(())
            };
            if let Some(control_id) = self.input.pending_chrome_release.take() {
                let same_control = input.hit_at(x, y).and_then(|hit| hit.control_id.as_deref()) == Some(control_id.as_str());
                if let Some((kind, index)) = ShellPaletteKind::row_index(&control_id) {
                    if same_control && button == 0 {
                        match kind {
                            ShellPaletteKind::Search => self.as_pointer().activate_search_item(index).await?,
                            ShellPaletteKind::Find => self.as_pointer().activate_find_item(index).await?,
                        }
                        input.blur_input();
                        self.input.accessibility_focused_control_id = None;
                    }
                    crate::scenes::cancel_scene_list_transfer();
                    return dock_drop_result;
                }
                if ShellPaletteKind::from_close_id(&control_id).is_some() {
                    if same_control && button == 0 {
                        self.commands.close_command_palette(input, &mut self.input);
                    }
                    crate::scenes::cancel_scene_list_transfer();
                    return dock_drop_result;
                }
                if let Some(window_id) = dock_tab_select_window_id(&control_id).filter(|_| same_control) {
                    if self.dock.dock.activate_window(window_id) {
                        self.windows.active_window_id = Some(window_id.to_string());
                    }
                }
            }
            if self.input.tree_drag.is_some() {
                self.as_pointer().finish_tree_drag(x, y, input).await?;
            } else if let Some((item_id, _)) = self.input.pending_tree_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) == Some(item_id.as_str()) {
                        if tree_row_has_chevron(input, &item_id) {
                            self.input.toggle_immediate_tree_item(&item_id);
                        }
                        self.input.dispatch_tree_selection(&item_id, &mut self.settle);
                        if let Some(action) = hit.event.clone() {
                            self.as_actions().dispatch_action(action).await?;
                        }
                    }
                }
            }
            let canvas_drop_accepted = retained_canvas_drag.as_ref().is_some_and(|drag_data| self.input.finish_canvas2d_catalogue_drop(x, y, drag_data, input));
            if input.drag.active {
                let drag_target = input.drag.target_id.clone();
                self.as_pointer().dispatch_widget_drag_values(input).await?;
                input.end_drag();
                if drag_target.as_deref().is_some_and(|id| id.starts_with("dock.split.") || id.starts_with("dock.corner.")) {
                    self.dock.persist_dock_layout();
                    if let Some(controller_id) = self.docs.shell_command_controller_id(&self.plugins) {
                        let note = note_shell_command_action(&controller_id, "shell.windowResize", shell_chrome_string("command.resizeWindow", self.prefs.locale_id == "de"), None);
                        self.as_actions().dispatch_action(note).await?;
                    }
                }
            }
            let captured_release = crate::interpreter::retained_pointer_capture_window(pointer_id).and_then(|window_id| self.input.retained_window_body_rect(&window_id).map(|body| (window_id, body)));
            let retained_release = input
                .hit_at(x, y)
                .cloned()
                .and_then(|hit| self.input.retained_hit_window(&hit).map(|(window_id, body)| (window_id, body, hit.kind, hit.event)))
                .or_else(|| captured_release.clone().map(|(window_id, body)| (window_id, body, HitKind::ComponentScene, None)));
            if canvas_drop_accepted {
                if let Some((captured_window, captured_body)) = captured_release {
                    self.as_pointer().route_retained_pointer_press(&captured_window, captured_body, captured_body.x - 1.0, captured_body.y - 1.0, false, button, HitKind::ComponentScene, None, pointer_id, input).await?;
                }
            } else if let Some((window_id, body, kind, action)) = retained_release {
                let released_window = window_id.clone();
                self.as_pointer().route_retained_pointer_press(&window_id, body, x, y, false, button, kind, action, pointer_id, input).await?;
                if let Some((captured_window, captured_body)) = captured_release.filter(|(captured_window, _)| captured_window != &released_window) {
                    self.as_pointer().route_retained_pointer_press(&captured_window, captured_body, x, y, false, button, HitKind::ComponentScene, None, pointer_id, input).await?;
                }
            }
            crate::scenes::cancel_scene_list_transfer();
            return dock_drop_result;
        }
        if button == 2 {
            let hit = input.hit_at(x, y).cloned();
            if input.modifiers.alt && self.input.claim_text_editor_alt_completions(x, y) {
                self.input.right_click = RightClickState { pending: true, x, y };
                return Ok(());
            }
            if let Some((window_id, body)) = hit.as_ref().and_then(|hit| self.input.retained_canvas2d_hit_window(hit, x, y)) {
                let scene_hit = hit.as_ref().expect("the retained Canvas2d owner was resolved from this hit");
                self.as_pointer().route_retained_pointer_press(&window_id, body, x, y, true, button, scene_hit.kind, scene_hit.event.clone(), pointer_id, input).await?;
            }
            self.as_pointer().open_context_menu(x, y, hit).await;
            self.input.right_click = RightClickState { pending: true, x, y };
            return Ok(());
        }
        if self.commands.dismiss_overlays(x, y, input, &mut self.input) {
            return Ok(());
        }
        if input.hit_at(x, y).is_none() && self.input.presented_input_geometry.dock_canvas_bounds.contains(x, y) {
            self.dock.deactivate_active_window(&mut self.windows);
        }
        if let Some(hit) = input.hit_at(x, y).cloned() {
            if let Some((window_id, body)) = self.input.retained_hit_window(&hit) {
                self.input.begin_window_template_drag_from_hit(pointer_id, &hit, x, y, &self.docs, &mut self.dock);
                self.as_pointer().route_retained_pointer_press(&window_id, body, x, y, true, button, hit.kind, hit.event.clone(), pointer_id, input).await?;
                return Ok(());
            }
            if hit.kind == HitKind::PanelResize {
                if !self.input.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(window_id) = id.strip_prefix("shell.measures.resize.") {
                        self.windows.measures_resize_window_id = Some(window_id.to_string());
                        self.windows.measures_resize_origin_width = *self.windows.measures_width.get(window_id).unwrap_or(&Theme::default().window_measures_default_width);
                        input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal), Some(hit.kind));
                        return Ok(());
                    }
                }
                let body = self.as_session().body_rect(theme);
                let anchor = hit.control_id.as_deref().and_then(panel_resize_anchor).map(|(anchor, _)| anchor).unwrap_or(PanelAnchor::TopLeft);
                self.dock.panel_resize_origin_width = floating_panel_width(self.dock.anchor_state(anchor).size, body, theme);
                input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal), Some(hit.kind));
                return Ok(());
            }
            if matches!(hit.kind, HitKind::DockSplit | HitKind::DockJoinCorner) {
                if !self.input.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                self.dock.deactivate_active_window(&mut self.windows);
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(rest) = id.strip_prefix("dock.corner.r/") {
                        if let Some((row_part, col_part)) = rest.split_once("/c/") {
                            if let Some((row_path_str, row_index_str)) = row_part.rsplit_once('/') {
                                if let Some((col_path_str, col_index_str)) = col_part.rsplit_once('/') {
                                    let row_path = parse_path(row_path_str);
                                    let col_path = parse_path(col_path_str);
                                    self.dock.split_resize_path = Some(row_path.clone());
                                    self.dock.split_resize_index = row_index_str.parse().unwrap_or(0);
                                    self.dock.split_resize_secondary_path = Some(col_path.clone());
                                    self.dock.split_resize_secondary_index = col_index_str.parse().unwrap_or(0);
                                    self.dock.split_resize_origin = self.dock.dock.begin_split_drag(&row_path);
                                    self.dock.split_resize_secondary_origin = self.dock.dock.begin_split_drag(&col_path);
                                    let geometry = &self.input.presented_input_geometry;
                                    self.dock.split_resize_axis_total = self.dock.dock.split_axis_extent(&row_path, geometry.dock_canvas_bounds, geometry.dock_axis_separator).unwrap_or(geometry.dock_canvas_bounds.w);
                                    self.dock.split_resize_secondary_axis_total = self.dock.dock.split_axis_extent(&col_path, geometry.dock_canvas_bounds, geometry.dock_axis_separator).unwrap_or(geometry.dock_canvas_bounds.h);
                                    input.begin_drag(x, y, button, Some(id.to_string()), Some(DragAxis::Both), Some(hit.kind));
                                    return Ok(());
                                }
                            }
                        }
                    }
                    if let Some(rest) = id.strip_prefix("dock.split.") {
                        if let Some((path_str, index_str)) = rest.rsplit_once('.') {
                            let path = parse_path(path_str);
                            let index: usize = index_str.parse().unwrap_or(0);
                            self.dock.split_resize_path = Some(path.clone());
                            self.dock.split_resize_index = index;
                            self.dock.split_resize_origin = self.dock.dock.begin_split_drag(&path);
                            let geometry = &self.input.presented_input_geometry;
                            self.dock.split_resize_axis_total = self.dock.dock.split_axis_extent(&path, geometry.dock_canvas_bounds, geometry.dock_axis_separator).unwrap_or_else(|| match hit.drag_axis {
                                Some(DragAxis::Vertical) => geometry.dock_canvas_bounds.h,
                                _ => geometry.dock_canvas_bounds.w,
                            });
                            input.begin_drag(x, y, button, Some(id.to_string()), hit.drag_axis, Some(hit.kind));
                            return Ok(());
                        }
                    }
                }
            }
            if self.input.begin_window_template_drag_from_hit(pointer_id, &hit, x, y, &self.docs, &mut self.dock) {
                return Ok(());
            }
            if let Some((item_id, drag_data)) = hit.control_id.as_deref().and_then(tree_hit_item_id).zip(hit.drag_data.clone()) {
                if !self.input.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                self.input.tree_drag_origin = (x, y);
                self.input.pending_tree_drag = Some((item_id.to_string(), drag_data));
                return Ok(());
            }
            if button == 0 && hit.control_id.as_deref().is_some_and(|id| ShellPaletteKind::row_index(id).is_some() || ShellPaletteKind::from_close_id(id).is_some()) {
                if !self.input.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                self.input.pending_chrome_release = hit.control_id.clone();
                return Ok(());
            }
            if self.as_pointer().handle_shell_hit(&hit, input).await? {
                if let Some(kind) = hit.control_id.as_deref().and_then(ShellPaletteKind::from_input_id) {
                    self.commands.set_palette_query(kind, self.commands.palette_query(kind).to_string(), input, &mut self.input);
                } else if hit.control_id.as_deref().is_some_and(|id| matches!(id, "ui.search.toggle" | "ui.find.toggle")) {
                    if let Some(kind) = match self.input.overlay_state {
                        OverlayState::Search => Some(ShellPaletteKind::Search),
                        OverlayState::Find => Some(ShellPaletteKind::Find),
                        _ => None,
                    } {
                        self.commands.set_palette_query(kind, self.commands.palette_query(kind).to_string(), input, &mut self.input);
                    } else {
                        input.blur_input();
                        self.input.accessibility_focused_control_id = None;
                    }
                } else if ShellPaletteKind::row_index(hit.control_id.as_deref().unwrap_or_default()).is_some() {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = None;
                }
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if let Some(window_id) = dock_tab_select_window_id(id) {
                    if !self.input.claim_chrome_pointer(pointer_id) {
                        return Ok(());
                    }
                    self.input.pending_chrome_release = hit.control_id.clone();
                    return Ok(());
                }
                if let Some(rest) = id.strip_prefix("dock.tab.").and_then(|rest| rest.strip_suffix(".drag")) {
                    if let Some((path_str_value, window_id)) = rest.split_once('.') {
                        let path = parse_path(path_str_value);
                        let tab_index = self.dock.dock.tab_index(&path, window_id).unwrap_or(0);
                        let window_kind_id = self.dock.dock.window_kind_id(window_id).unwrap_or(window_id).to_string();
                        let template_id = self.dock.dock.window_template_id(window_id).map(str::to_string);
                        let ghost_label =
                            self.docs.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == window_kind_id).map(|k| k.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string())).unwrap_or_else(|| window_id.to_string());
                        self.input.begin_pending_dock_drag(pointer_id, DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.to_string(), window_kind_id, template_id, source_path: path, tab_index, ghost_label }, x, y, &mut self.dock);
                        return Ok(());
                    }
                }
                if let Some(path_str_value) = id.strip_prefix("dock.stack.") {
                    let path = parse_path(path_str_value);
                    let windows = self.dock.dock.stack_windows_at_path(&path).unwrap_or_default();
                    let active = windows.iter().find(|wid| self.windows.active_window_id.as_deref() == Some(wid.as_str())).or_else(|| windows.first()).cloned().unwrap_or_default();
                    if !active.is_empty() {
                        let tab_index = self.dock.dock.tab_index(&path, &active).unwrap_or(0);
                        let window_kind_id = self.dock.dock.window_kind_id(&active).unwrap_or(&active).to_string();
                        let ghost_label =
                            self.docs.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == window_kind_id).map(|k| k.label.resolve(self.prefs.active_terminology(), self.prefs.active_locale()).to_string())).unwrap_or_else(|| active.clone());
                        self.input.begin_pending_dock_drag(pointer_id, DockDragPayload { kind: DockDragKind::Stack, window_id: active, window_kind_id, template_id: None, source_path: path, tab_index, ghost_label }, x, y, &mut self.dock);
                        return Ok(());
                    }
                }
            }
            if hit.kind == HitKind::PanelTab {
                if let Some((tab_id, anchor)) = hit.control_id.as_deref().and_then(|id| Some((id.to_string(), self.dock.panel_tab_anchor(id)?))) {
                    if !self.input.claim_chrome_pointer(pointer_id) {
                        return Ok(());
                    }
                    self.dock.dock_tab_drag = Some((tab_id, anchor));
                    input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Both), Some(hit.kind));
                }
            }
            if hit.kind == HitKind::Slider {
                if let Some(id) = hit.control_id.clone() {
                    if !self.input.claim_chrome_pointer(pointer_id) {
                        return Ok(());
                    }
                    input.begin_drag(x, y, button, Some(id), hit.drag_axis, Some(hit.kind));
                    return Ok(());
                }
            }
            if hit.kind == HitKind::Select || hit.kind == HitKind::Toggle || hit.kind == HitKind::DropdownItem {
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if id.contains(".vfs.") && !id.contains(".chevron.") {
                    if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.") {
                        let additive = input.modifiers.meta || input.modifiers.ctrl;
                        let shift = input.modifiers.shift;
                        let ordered = vec![row_id.to_string()];
                        let ids = vfs_selection_for_click(surface_id, row_id, &ordered, shift, additive);
                        ({ let __arg0 = ActionDescriptor {
                            controller_id: self.docs.session.as_ref().map(|s| s.app.controller_id.clone()).unwrap_or_default(),
                            action: "selectRows".into(),
                            args: crate::action_args_json!({ "surfaceId": surface_id, "ids": ids }),
                        }; self.as_actions().dispatch_action(__arg0) })
                        .await?;
                        return Ok(());
                    }
                }
            }
            if let Some(action) = hit.event.clone() {
                self.as_actions().dispatch_action(action).await?;
            } else if hit.kind == HitKind::Input {
                if let Some(id) = hit.control_id {
                    let seed = ShellPaletteKind::from_input_id(&id).map(|kind| self.commands.palette_query(kind).to_string()).or_else(|| self.input.widget_maps.input_metas.get(&id).map(|meta| meta.value.clone())).unwrap_or_default();
                    input.focus_input_owned(id, seed);
                }
            }
        }
        self.as_actions().drain_gesture_bound_work().await;
        self.settle.owe_settle();
        Ok(())
    }

    //#region 🎯️RetainedPointerRouting
    pub(super) fn candidate_input_geometry(&mut self, theme: &Theme) -> PresentedInputGeometry {
        let body = self.as_session().body_rect(theme);
        let panel_rects = PanelAnchor::ALL.into_iter().filter(|anchor| self.dock.anchor_open(*anchor)).map(|anchor| self.anchor_rect(anchor, body, theme)).collect();
        let active_owner = self.docs.session.as_ref().and_then(|session| self.docs.media_owner_for_session(session));
        let spawned_owner = self.docs.spawned_session.as_ref().and_then(|session| self.docs.media_owner_for_session(session));
        let spawned = self.docs.session.as_ref().and_then(|session| panel_state_from_view(&session.view_state).ok().flatten()).map(|panel| panel.spawned_apps).unwrap_or_default();
        let media_owners = self.input.retained_body_rects_staging.iter().map(|(id, _)| {
            let owner = if id == "spawned" || spawned.iter().any(|app| &app.id == id) { spawned_owner.clone() } else { active_owner.clone() };
            (id.clone(), owner)
        }).collect();
        let media_blocked = self.input.candidate_pointer_input_is_modal(&self.chrome) || self.input.overlay_state != OverlayState::None || self.sync.sync_card_kind.is_some() || self.dock.dock_drag.is_some()
            || self.chrome.chrome_build.tooltip_hover.as_ref().is_some_and(|hover| self.chrome.chrome_build.tooltip_shows(false) && chrome_tooltip_ready(hover, chrome_now_ms()));
        PresentedInputGeometry {
            retained_body_rects: self.input.retained_body_rects_staging.clone(),
            media_owners,
            media_blocked,
            retained_surfaces: crate::interpreter::staged_retained_clock_surfaces(),
            dock_canvas_bounds: self.dock.dock_canvas_bounds,
            dock_axis_separator: self.dock.dock_axis_separator,
            dock_drop_tab_bars: self.dock.dock_drop_tab_bars.clone(),
            dock_drop_bodies: self.dock.dock_drop_bodies.clone(),
            dock_window_plan: self.dock.dock_window_plan.clone(),
            window_content_rects: self.dock.window_content_rects.clone(),
            window_silhouettes: self.dock.window_silhouettes.clone(),
            modal: self.input.candidate_pointer_input_is_modal(&self.chrome),
            panel_rects,
        }
    }

    /// 🎞️ Seals the hit, owner, widget-binding and geometry candidates minted by one completed
    /// chrome walk. They remain non-interactive until the matching GPU packet is acknowledged.
    pub(crate) fn seal_presented_input_candidate(&mut self, theme: &Theme) -> Result<PresentedInputCandidateWitness, &'static str> {
        let epoch = self.input.next_presented_input_epoch.ok_or("presented input candidate generation exhausted")?;
        self.input.next_presented_input_epoch = epoch.checked_add(1);
        let witness = PresentedInputCandidateWitness(epoch);
        self.input.presented_input_geometry_staging = self.candidate_input_geometry(theme);
        self.input.presented_input_candidate = Some(witness);
        if !crate::interpreter::seal_presented_input_candidate(witness.0) {
            self.input.presented_input_candidate = None;
            return Err("retained presented input candidate could not be sealed");
        }
        crate::scenes::seal_host_temporal_candidates(witness.0);
        crate::scenes::seal_table_stepper_accessibility_candidates(witness.0);
        crate::scenes::seal_table_editable_text_accessibility_candidates(witness.0);
        crate::scenes::seal_table_button_accessibility_candidates(witness.0);
        crate::scenes::seal_vfs_accessibility_candidates(witness.0);
        crate::scenes::seal_block_list_accessibility_candidates(witness.0);
        crate::scenes::seal_event_feed_accessibility_candidates(witness.0);
        crate::scenes::seal_world3d_accessibility_candidates(witness.0);
        crate::scenes::seal_graph_timeline_accessibility_candidates(witness.0);
        Ok(witness)
    }
    //#endregion 🎯️RetainedPointerRouting
}

//#endregion ShellInput

