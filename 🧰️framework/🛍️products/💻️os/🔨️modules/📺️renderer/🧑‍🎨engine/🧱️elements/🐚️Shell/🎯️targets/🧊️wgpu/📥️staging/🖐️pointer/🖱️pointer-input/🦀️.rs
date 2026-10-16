//! 🖐️ `shell-pointer` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
/// ⌨️ Maps a chrome-level `ui_wgpu::wgpu::KeyAction` (+ modifiers) to the `ui_wgpu::wgpu::UiEvent` the retained
/// content engine's `events::EventRouter::dispatch` expects — mirrors that fn's `UiEvent::KeyDown`
/// key-string vocabulary exactly (`"ArrowLeft"`/`"Backspace"`/`"c"`+ctrl for copy/etc., see
/// `ui_wgpu`'s `🔖️EditRouting`/`🔖️UiCommand` regions). A `Char` held with Ctrl/Cmd routes as
/// `KeyDown` (so `c`/`x`/`v` clipboard chords reach `route_edit_key` instead of being literally
/// inserted as text). A focused Select owns printable typeahead and the Space press as `KeyDown`;
/// editable controls keep plain characters as `TextInput`.
pub(crate) fn ui_event_from_key_action(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, focused_select: bool) -> Option<ui_wgpu::wgpu::UiEvent> {
    let event_modifiers = ui_wgpu::wgpu::EventModifiers { shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta };
    match action {
        ui_wgpu::wgpu::KeyAction::Char(ch) => {
            if modifiers.ctrl_or_meta() || focused_select {
                Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: ch.clone(), modifiers: event_modifiers })
            } else {
                Some(ui_wgpu::wgpu::UiEvent::TextInput { text: ch.clone() })
            }
        }
        ui_wgpu::wgpu::KeyAction::Backspace => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Backspace".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Delete => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Delete".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Enter => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Enter".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Escape => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Escape".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowLeft => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowRight => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowRight".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowUp => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowUp".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowDown => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowDown".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Home => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Home".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::End => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "End".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::PageUp => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "PageUp".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::PageDown => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "PageDown".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Function(number) => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: format!("F{number}"), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Tab => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Space(true) if focused_select => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: " ".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Space(_) => None,
    }
}

/// 🌳️ Whether this frame painted a fold chevron for `item_id` — the row has children.
pub(super) fn tree_row_has_chevron(input: &InputState<ActionDescriptor>, item_id: &str) -> bool {
    let chevron = format!("tree.chevron.{item_id}");
    input.hits().iter().chain(input.staged_hits().iter()).any(|hit| hit.control_id.as_deref() == Some(chevron.as_str()))
}

//#region 🫥️HostWindowBlur
/// 🫥️ Whether a host window blur waits for the shell's drain.
pub(crate) fn host_window_blur_pending() -> bool {
    *HOST_WINDOW_BLUR_PENDING.borrow()
}

//#endregion 🫥️HostWindowBlur
impl<'ctx> PointerCtx<'ctx> {
    /// 🛑️ Retires a cancelled pointer sequence without a click, drop, or release commit.
    pub fn handle_pointer_cancel(&mut self, input: &mut InputState<ActionDescriptor>) {
        self.handle_pointer_cancel_for(ui_render::PointerId(1), input);
    }

    pub fn handle_pointer_cancel_for(&mut self, pointer_id: ui_render::PointerId, input: &mut InputState<ActionDescriptor>) {
        let retained_cancelled = crate::interpreter::retained_pointer_capture_window(pointer_id);
        if let Some(window_id) = retained_cancelled.as_ref() {
            crate::interpreter::dispatch_ui_pointer_event(&window_id, pointer_id, ui_wgpu::wgpu::UiEvent::PointerCancel, input);
        }
        let (pointer_x, pointer_y) = (input.pointer_x, input.pointer_y);
        let cancelled_scenes = crate::interpreter::cancel_scene_pointer(pointer_id, input);
        let scene_cancelled = !cancelled_scenes.is_empty();
        let mut capture_lost: Vec<&str> = retained_cancelled.iter().map(String::as_str).chain(cancelled_scenes.iter().map(|cancelled| cancelled.window_id.as_str())).collect();
        capture_lost.sort_unstable();
        capture_lost.dedup();
        if let Some(controller_id) = self.docs.shell_command_controller_id(&self.plugins) {
            for window_id in capture_lost {
                self.settle.deferred_actions.push(window_host_event_action(&controller_id, window_id, semio_framework::HOST_EVENT_KIND_CAPTURE_LOST));
            }
        }
        for cancelled in cancelled_scenes {
            match cancelled.kind {
                ui_wgpu::wgpu::SurfaceKind::World3d => {
                    if let Some(state) = self.surfaces.world3d_states.get_mut(&cancelled.host_id) {
                        infinite_world::world::world3d_cancel_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene);
                    }
                }
                ui_wgpu::wgpu::SurfaceKind::NodeGraph => {
                    crate::engine_canvas::node_graph_pointer_cancel_into(&cancelled.host_id);
                }
                ui_wgpu::wgpu::SurfaceKind::Board2d => {
                    crate::engine_canvas::board2d_pointer_cancel_into(&cancelled.host_id, input);
                }
                ui_wgpu::wgpu::SurfaceKind::TiledMap => {
                    if let Some(surface) = self.surfaces.tiled_map_states.get(&cancelled.host_id).filter(|surface| surface.window_id == cancelled.window_id) {
                        if let Err(fault) = crate::scenes::tiled_map_pointer_cancel_into(&cancelled.host_id, &surface.controller_id, surface.bounds, pointer_x, pointer_y, &mut *input) {
                            input.record_action_fault(fault);
                        }
                    }
                }
                _ => {}
            }
        }
        let canvas_cancelled = crate::scenes::cancel_canvas_pointer_gesture_for(pointer_id, input);
        let transfer_cancelled = crate::scenes::cancel_scene_list_transfer_for_pointer(pointer_id);
        let chrome_cancelled = self.input.chrome_pointer_owner == Some(pointer_id);
        if chrome_cancelled {
            self.input.chrome_pointer_owner = None;
            self.dock.dock_tab_drag = None;
            self.dock.dock_drag = None;
            self.dock.pending_dock_drag = None;
            self.input.tree_drag = None;
            self.input.pending_tree_drag = None;
            self.input.pending_chrome_release = None;
            self.input.right_click = RightClickState::default();
        }
        if retained_cancelled.is_some() || scene_cancelled || canvas_cancelled || transfer_cancelled || chrome_cancelled {
            input.pointer_down = false;
            input.hovered_id = None;
            input.drag = Default::default();
        }
    }

    pub fn handle_pointer_move(&mut self, x: f32, y: f32, down: bool, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
        self.handle_pointer_move_for(ui_render::PointerId(1), x, y, down, input, theme);
    }

    /// 🖱️ The two ghost-preview syncs take `&mut self` (`sync_world3d_catalogue_drop_preview` walks
    /// `world3d_states`), so the drag's own payload is snapshotted first and the `&mut self.tree_drag`
    /// borrow is re-taken afterwards — the previous shape held both at once and did not compile.
    ///
    /// 📱️ Below React's breakpoint a tab cannot travel at all — the dock context hands
    /// `startTabDrag: mobile ? noopDrag : startTabDrag` (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1852`),
    /// because a mobile mode is ONE flat stack with nothing to drop into.
    ///
    /// 🩸️ Promotion used to REMOVE the dragged window from the committed tree here; it is now a
    /// pure state transition and the removal lives in `DockState::render_view`'s derivation.
    ///
    /// ↔ `delta_factor` from React's `PanelResizeHandle`: an outer (canvas-facing) handle
    /// grows the panel with the pointer, an inner one against it, and a centred
    /// middle-column panel grows from both edges at once, hence the doubled step.
    pub fn handle_pointer_move_for(&mut self, pointer_id: ui_render::PointerId, x: f32, y: f32, down: bool, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
        if self.input.chrome_pointer_owner.is_some_and(|owner| owner != pointer_id) {
            return;
        }
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        self.directory.presence_pointer = Some((x, y));
        input.update_hover(x, y);
        if let Some((kind, index)) = input.hovered_id.as_deref().and_then(ShellPaletteKind::row_index) {
            match kind {
                ShellPaletteKind::Search => self.commands.search_selected = index,
                ShellPaletteKind::Find => self.commands.find_selected = index,
            }
        }
        self.input.route_retained_pointer_move(pointer_id, x, y, input);
        self.input.sync_context_menu_hover(input);
        self.input.update_tree_hover(input, &mut self.settle);
        if let Some((ref item_id, ref drag_data)) = self.input.pending_tree_drag {
            if down {
                let dx = x - self.input.tree_drag_origin.0;
                let dy = y - self.input.tree_drag_origin.1;
                if self.input.tree_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    self.input.tree_drag = Some(TreeDragState { source_id: item_id.clone(), drag_data: drag_data.clone(), x, y, drop_target_id: None, drop_position: TreeDropPosition::Inside });
                    self.input.pending_tree_drag = None;
                }
            }
        }
        let tree_drag_payload = self.input.tree_drag.as_mut().map(|drag| {
            drag.x = x;
            drag.y = y;
            drag.drag_data.clone()
        });
        if let Some(ref drag_data) = tree_drag_payload {
            let graph_surfaces = self.surfaces.node_graph_states.iter().map(|(id, surface)| (id.as_str(), surface.bounds)).collect::<Vec<_>>();
            crate::engine_canvas::node_graph_sync_flow_widget_ghost(x, y, drag_data, &graph_surfaces);
            drop(graph_surfaces);
            self.surfaces.sync_world3d_catalogue_drop_preview(x, y, drag_data);
        } else {
            self.input.sync_world3d_catalogue_drop_preview_from_retained_ui(&mut self.surfaces);
        }
        let retained_drag_data = tree_drag_payload
            .is_none()
            .then(|| crate::interpreter::active_retained_drag_sessions())
            .and_then(|sessions| sessions.into_iter().map(|(_, _, _, drag_data)| drag_data).find(|drag_data| drag_data.contains_key(crate::scenes::CANVAS_CATALOGUE_DRAG_MIME)));
        if let Some(drag_data) = tree_drag_payload.as_ref().or(retained_drag_data.as_ref()) {
            self.input.sync_canvas2d_catalogue_drag(x, y, drag_data, input);
        } else {
            crate::scenes::cancel_canvas_catalogue_drag(input);
        }
        if let Some(drag) = &mut self.input.tree_drag {
            if let Some(hit) = input.hit_at(x, y) {
                if let Some(target_id) = hit.control_id.as_deref().and_then(tree_hit_item_id) {
                    drag.drop_target_id = Some(target_id.to_string());
                    let rel = (y - hit.rect.y) / hit.rect.h.max(1.0);
                    drag.drop_position = if rel < 0.25 {
                        TreeDropPosition::Before
                    } else if rel > 0.75 {
                        TreeDropPosition::After
                    } else {
                        TreeDropPosition::Inside
                    };
                } else if hit.kind == HitKind::World3d || hit.kind == HitKind::Window {
                    drag.drop_target_id = hit.control_id.clone();
                    drag.drop_position = TreeDropPosition::Inside;
                } else {
                    drag.drop_target_id = None;
                }
            }
        }
        if let Some((payload, origin)) = &self.dock.pending_dock_drag {
            if down && !self.dock.dock.mobile {
                let dx = x - origin.0;
                let dy = y - origin.1;
                if self.dock.dock_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    let payload = payload.clone();
                    let origin = *origin;
                    self.dock.pending_dock_drag = None;
                    self.dock.dock_drag = Some(DockDragState { payload, x: origin.0, y: origin.1, drop_zone: None });
                }
            }
        }
        if let Some(drag) = &mut self.dock.dock_drag {
            drag.x = x;
            drag.y = y;
            let geometry = &self.input.presented_input_geometry;
            drag.drop_zone = compute_dock_drop_zone(x, y, &geometry.dock_drop_tab_bars, &geometry.dock_drop_bodies, geometry.dock_canvas_bounds);
        }
        if input.drag.active && down {
            input.update_drag(x, y);
            if let Some(id) = input.drag.target_id.as_deref() {
                let dx = x - input.drag.start_x;
                let dy = y - input.drag.start_y;
                match id {
                    id if id.starts_with("shell.measures.resize.") => {
                        if let Some(window_id) = self.windows.measures_resize_window_id.clone() {
                            let next = (self.windows.measures_resize_origin_width - dx).clamp(theme.panel_min_width, theme.panel_max_width);
                            self.windows.measures_width.insert(window_id, next);
                        }
                    }
                    id if id.starts_with("panel.resize.") => {
                        if let Some((anchor, inner)) = panel_resize_anchor(id) {
                            let body = self.as_session().body_rect(theme);
                            let factor = if anchor.horizontal() == "middle" { 2.0 } else { 1.0 } * if inner { -1.0 } else { 1.0 };
                            let next = (self.dock.panel_resize_origin_width + dx * factor).clamp(theme.panel_min_width, floating_panel_max_width(body, theme));
                            self.dock.anchor_state_mut(anchor).size = next;
                        }
                    }
                    dock_id if dock_id.starts_with("dock.corner.") => {
                        if let Some(path) = self.dock.split_resize_path.clone() {
                            self.dock.dock.apply_split_drag_with_origin(&path, self.dock.split_resize_index, dx, self.dock.split_resize_axis_total, &self.dock.split_resize_origin);
                        }
                        if let Some(path) = self.dock.split_resize_secondary_path.clone() {
                            self.dock.dock.apply_split_drag_with_origin(&path, self.dock.split_resize_secondary_index, dy, self.dock.split_resize_secondary_axis_total, &self.dock.split_resize_secondary_origin);
                        }
                    }
                    dock_id if dock_id.starts_with("dock.split.") => {
                        if let (Some(path), axis) = (&self.dock.split_resize_path, input.drag.axis) {
                            let delta = match axis {
                                Some(DragAxis::Horizontal) => dx,
                                Some(DragAxis::Vertical) => dy,
                                _ => dx,
                            };
                            self.dock.dock.apply_split_drag_with_origin(path, self.dock.split_resize_index, delta, self.dock.split_resize_axis_total, &self.dock.split_resize_origin);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// 🎯️ An abandoned or refused drop is simply nothing: the committed tree was never touched, so
    /// there is no snapshot to re-apply and no `sync_dock` to pay for.
    pub(crate) async fn finish_dock_drag(&mut self, x: f32, y: f32, input: &InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(mut drag) = self.dock.dock_drag.take() else {
            return Ok(());
        };
        drag.x = x;
        drag.y = y;
        if drag.drop_zone.is_none() {
            let geometry = &self.input.presented_input_geometry;
            drag.drop_zone = compute_dock_drop_zone(x, y, &geometry.dock_drop_tab_bars, &geometry.dock_drop_bodies, geometry.dock_canvas_bounds);
        }
        if let Some(zone) = drag.drop_zone {
            let body_key = self.docs.session.as_ref().and_then(|session| session.app.window_kinds.iter().find(|kind| kind.id == drag.payload.window_kind_id)).map(|kind| kind.body_key.clone());
            if body_key.is_some() && !self.windows.window_topology_publication_has_credit() {
                return Err("window topology publication item credits exceeded".to_string());
            }
            let controller_id = self.docs.shell_command_controller_id(&self.plugins);
            if controller_id.is_some() && body_key.is_none() {
                return Err(format!("window kind '{}' has no retained body", drag.payload.window_kind_id));
            }
            let journal_token = if let Some(controller_id) = controller_id {
                let (command_id, label, detail) = if drag.payload.kind == DockDragKind::NewWindow {
                    ("shell.windowSplit", shell_chrome_string("command.splitWindow", self.prefs.locale_id == "de"), serde_json::json!({ "windowKindId": drag.payload.window_kind_id, "instanceId": drag.payload.window_id }))
                } else {
                    ("shell.windowMove", shell_chrome_string("command.moveWindow", self.prefs.locale_id == "de"), serde_json::json!({ "windowId": drag.payload.window_id }))
                };
                let note = note_shell_command_action(&controller_id, command_id, label, Some(detail));
                Some(self.windows.reserve_window_topology_action(note).map_err(|error| format!("window topology journal refused admission before dock mutation: {error:?}"))?)
            } else {
                None
            };
            if !self.dock.dock.apply_drop(&drag.payload, &zone) {
                if let Some(token) = journal_token {
                    let _ = self.windows.take_window_topology_action(token);
                }
                return Ok(());
            }
            if drag.payload.kind == DockDragKind::NewWindow {
                self.dock.seed_new_window_projection(&drag.payload.window_id, drag.payload.template_id.as_deref(), &self.docs, &mut self.windows);
            }
            self.windows.active_window_id = Some(drag.payload.window_id.clone());
            self.dock.dock.sync_active_window(&drag.payload.window_id);
            self.dock.persist_dock_layout();
            if let Some(body_key) = body_key.as_deref() {
                self.windows.commit_window_topology_publication(&drag.payload.window_id, body_key, journal_token, &mut self.settle);
            } else {
                self.windows.owe_window_topology_refresh(&mut self.settle);
            }
        }
        let _ = input;
        Ok(())
    }

    pub(crate) async fn handle_accessibility_event(&mut self, target: &ui_render::AccessibilityTarget, event: &ui_render::AccessibilityEvent, input: &mut InputState<ActionDescriptor>) -> Result<bool, String> {
        if target.window_id != crate::interpreter::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID {
            let event = match event {
                ui_render::AccessibilityEvent::Focus => ui_wgpu::wgpu::AccessibilityUiEvent::Focus,
                ui_render::AccessibilityEvent::Blur => ui_wgpu::wgpu::AccessibilityUiEvent::Blur,
                ui_render::AccessibilityEvent::Activate => ui_wgpu::wgpu::AccessibilityUiEvent::Activate,
                ui_render::AccessibilityEvent::Value(value) => ui_wgpu::wgpu::AccessibilityUiEvent::Value(value.clone()),
            };
            let focused = matches!(event, ui_wgpu::wgpu::AccessibilityUiEvent::Focus);
            let Some(commands) = crate::interpreter::dispatch_accessibility_event(&target.window_id, target.window_generation, target.node_id, &target.node_key, event, input) else { return Ok(false) };
            self.chrome.chrome_build.note_content_focus_commands(&commands);
            if focused && !self.as_session().retained_surface_is_panel(&target.window_id) {
                let owner = window_pane_owner(&target.window_id).unwrap_or(&target.window_id).to_string();
                self.chrome.chrome_build.clear_sibling_pane_focus(&target.window_id);
                self.dock.dock.sync_active_window(&owner);
                self.windows.active_window_id = Some(owner);
            }
            return Ok(true);
        }
        if target.window_generation != self.chrome.presented_chrome_accessibility_generation {
            return Ok(false);
        }
        if !self.chrome.presented_chrome_accessibility.iter().any(|node| node.node_id == target.node_id && node.key == target.node_key) {
            return Ok(false);
        }
        if self.input.handle_chrome_dialog_accessibility_event(&target.node_key, event, &mut self.chrome, &mut self.settle) {
            return Ok(true);
        }
        if target.node_key == icon_export::CONTROL_ID {
            match event {
                ui_render::AccessibilityEvent::Activate => self.opening.cancel_icon_export(),
                ui_render::AccessibilityEvent::Focus => {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = Some(target.node_key.clone());
                }
                ui_render::AccessibilityEvent::Blur => self.input.accessibility_focused_control_id = None,
                ui_render::AccessibilityEvent::Value(_) => return Ok(false),
            }
            return Ok(true);
        }
        let palette_kind = match self.input.overlay_state {
            OverlayState::Search => Some(ShellPaletteKind::Search),
            OverlayState::Find => Some(ShellPaletteKind::Find),
            _ => None,
        };
        if let Some(kind) = palette_kind {
            match event {
                ui_render::AccessibilityEvent::Focus if target.node_key == kind.input_id() => {
                    let query = self.commands.palette_query(kind).to_string();
                    self.commands.set_palette_query(kind, query, input, &mut self.input);
                }
                ui_render::AccessibilityEvent::Focus if target.node_key == kind.close_id() => {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = Some(target.node_key.clone());
                }
                ui_render::AccessibilityEvent::Focus => {}
                ui_render::AccessibilityEvent::Blur if target.node_key == kind.input_id() => {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = None;
                }
                ui_render::AccessibilityEvent::Blur => {}
                ui_render::AccessibilityEvent::Activate if target.node_key == kind.close_id() => {
                    self.commands.close_command_palette(input, &mut self.input);
                }
                ui_render::AccessibilityEvent::Activate if target.node_key == kind.input_id() => {
                    let query = self.commands.palette_query(kind).to_string();
                    self.commands.set_palette_query(kind, query, input, &mut self.input);
                }
                ui_render::AccessibilityEvent::Activate => {
                    let index = match kind {
                        ShellPaletteKind::Search => self.as_session().filtered_search_items().iter().position(|item| item.id == target.node_key),
                        ShellPaletteKind::Find => self.commands.filtered_find_items().iter().position(|item| item.id == target.node_key),
                    };
                    if let Some(index) = index {
                        match kind {
                            ShellPaletteKind::Search => self.activate_search_item(index).await?,
                            ShellPaletteKind::Find => self.activate_find_item(index).await?,
                        }
                        input.blur_input();
                        self.input.accessibility_focused_control_id = None;
                    }
                }
                ui_render::AccessibilityEvent::Value(value) if target.node_key == kind.input_id() => {
                    self.commands.set_palette_query(kind, value.clone(), input, &mut self.input);
                }
                ui_render::AccessibilityEvent::Value(_) => {}
            }
            return Ok(true);
        }
        if let Some(anchor) = self.dock.panel_tab_anchor(&target.node_key) {
            match event {
                ui_render::AccessibilityEvent::Focus => {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = Some(target.node_key.clone());
                    self.input.reveal_panel_tab_row(anchor, &target.node_key, &self.dock);
                }
                ui_render::AccessibilityEvent::Blur => {
                    if self.input.accessibility_focused_control_id.as_deref() == Some(target.node_key.as_str()) {
                        self.input.accessibility_focused_control_id = None;
                    }
                }
                ui_render::AccessibilityEvent::Activate => {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = Some(target.node_key.clone());
                    self.input.reveal_panel_tab_row(anchor, &target.node_key, &self.dock);
                    self.select_panel_tab(anchor, &target.node_key).await?;
                }
                ui_render::AccessibilityEvent::Value(_) => return Ok(false),
            }
            return Ok(true);
        }
        if let Some(window_id) = dock_tab_select_window_id(&target.node_key).map(str::to_string) {
            match event {
                ui_render::AccessibilityEvent::Focus => {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = Some(target.node_key.clone());
                }
                ui_render::AccessibilityEvent::Blur => {
                    if self.input.accessibility_focused_control_id.as_deref() == Some(target.node_key.as_str()) {
                        self.input.accessibility_focused_control_id = None;
                    }
                }
                ui_render::AccessibilityEvent::Activate => {
                    input.blur_input();
                    self.input.accessibility_focused_control_id = Some(target.node_key.clone());
                    if self.dock.dock.activate_window(&window_id) {
                        self.windows.active_window_id = Some(window_id);
                    }
                }
                ui_render::AccessibilityEvent::Value(_) => return Ok(false),
            }
            return Ok(true);
        }
        let Some(hit) = input.hits().iter().find(|hit| hit.control_id.as_deref() == Some(target.node_key.as_str())).cloned() else { return Ok(false) };
        if input.focused_id.as_deref().is_some_and(|id| id != target.node_key) {
            self.commit_focused_input(input).await?;
        }
        if hit.kind != HitKind::Input {
            input.blur_input();
        }
        self.input.accessibility_focused_control_id = Some(target.node_key.clone());
        match event {
            ui_render::AccessibilityEvent::Focus => {
                if hit.kind == HitKind::Input {
                    let seed = ShellPaletteKind::from_input_id(&target.node_key).map(|kind| self.commands.palette_query(kind).to_string()).or_else(|| self.input.widget_maps.input_metas.get(&target.node_key).map(|meta| meta.value.clone())).unwrap_or_default();
                    input.focus_input_owned(target.node_key.clone(), seed);
                }
            }
            ui_render::AccessibilityEvent::Blur => {
                if input.focused_id.as_deref() == Some(target.node_key.as_str()) {
                    self.commit_focused_input(input).await?;
                }
                if self.input.accessibility_focused_control_id.as_deref() == Some(target.node_key.as_str()) {
                    self.input.accessibility_focused_control_id = None;
                }
            }
            ui_render::AccessibilityEvent::Activate => {
                if self.handle_shell_hit(&hit, input).await? {
                    if matches!(target.node_key.as_str(), "ui.search.toggle" | "ui.find.toggle") {
                        if let Some(kind) = match self.input.overlay_state {
                            OverlayState::Search => Some(ShellPaletteKind::Search),
                            OverlayState::Find => Some(ShellPaletteKind::Find),
                            _ => None,
                        } {
                            self.commands.set_palette_query(kind, self.commands.palette_query(kind).to_string(), input, &mut self.input);
                        }
                    } else if ShellPaletteKind::row_index(&target.node_key).is_some() {
                        input.blur_input();
                        self.input.accessibility_focused_control_id = None;
                    }
                } else {
                    if let Some(action) = hit.event {
                        self.as_actions().dispatch_action(action).await?;
                    }
                }
            }
            ui_render::AccessibilityEvent::Value(value) => {
                if let Some(kind) = ShellPaletteKind::from_input_id(&target.node_key) {
                    self.commands.set_palette_query(kind, value.clone(), input, &mut self.input);
                    return Ok(true);
                }
                if target.node_key.ends_with(".input") {
                    let base = target.node_key.trim_end_matches(".input");
                    if let Some(meta) = self.input.widget_maps.stepper_metas.get(base).cloned() {
                        if let Ok(value) = value.parse::<f64>() {
                            self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_absolute.controller_id, action: meta.on_absolute.action, args: crate::action_args_json!({ "value": value }) }).await?;
                        }
                        return Ok(true);
                    }
                }
                if let Some(meta) = self.input.widget_maps.input_metas.get(&target.node_key).cloned() {
                    if let Some(value) = meta.commit_value(value) {
                        let args = merge_committed_args(meta.on_change.args.as_ref(), value);
                        self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args }).await?;
                    }
                }
            }
        }
        Ok(true)
    }

    /// 🪟️ The five pane chips answer FIRST and by decode, not by prefix: their ids carry the window
    /// instance in the middle (React's `framework.window.<segment>.<pane>.<verb>`), so a
    /// `starts_with` arm cannot name them. See [`WindowPaneChip::control_id`].
    ///
    /// 🔼️ A `Select` popup's scroll chevron (`{select}.scroll.up`/`.down`). Answered BEFORE the
    /// `match` for the same reason the pane chips are: the id is decoded, not prefixed, and the
    /// press handler cannot clamp — it only arms a direction that the popup's own painter prices
    /// in `select_scroll_step` pixels and clamps with `select_clamped_scroll`, because only the
    /// painter holds the item count and the collision-resolved popup height
    /// (ticket 26/09/17 packet W15a; the chevrons painted and did nothing before it).
    ///
    /// 👁️✏️ The surface-role group. Switching roles is a TRANSACTIONAL session switch, not a
    /// view-state flip: the successor instance is created before the predecessor is retired,
    /// so no render ever observes two live instances of one plugin.
    ///
    /// 🛑️ The World3d compute cancel — dispatches whatever id the surface's own status
    /// document named, never a verb this shell knows.
    ///
    /// 🔀️ The pane's own Projection chip — React's `framework.worldOrbit.projection.<segment>.pane.fold`,
    /// ONE id on both sides of the fold (`Pane`'s `chromeToggleId = toggleId ?? childElementId(id,
    /// "pane", "fold")`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10006`), flipping the pane's own local
    /// `folded` state and nothing else.
    ///
    /// 🪟️ React gates the cap on `canMaximize` — a mode with ONE window has nothing to
    /// maximize away from, exactly as `focus_active_window` already refuses there.
    ///
    /// 🖥️ React's `applyNamedLayout` searches `[...builtinLayouts, ...userLayouts]`
    /// (`📌️ChromePanels/🟦️.tsx`'s `useNamedLayoutHost`), so a layout the user SAVED resolves
    /// exactly like one the app declared. This arm used to search the app's declarations
    /// alone, which is why a restored `namedLayouts` entry was inert.
    ///
    /// 🎓️ The introduction's own four controls, answered HERE rather than by a rect test inside
    /// the painter — React wires them as ordinary `onClick`/`onPointerDown` handlers on real
    /// elements, and routing them through the one press funnel is what makes the veil able to
    /// swallow a press instead of letting it through to the surface below.
    ///
    /// 📱️ The mobile panel's flattened tab row selects a path in the ONE mobile panel instead
    /// of an anchor's, and a tool leaf arms its tool exactly as the desktop row does.
    ///
    /// 📑️ A panel tab is addressed by React's own BARE id (`framework.panel.artifact`), so the
    /// anchor is derived here from whichever anchor's tab tree carries it — exactly as React
    /// derives it from the tab's own `group` — instead of being carried in the control id.
    ///
    /// 🖱️ Group rows (`menu.group.<category>`, but any row with `children`) open their submenu on
    /// click, not just hover — matched by path (not a flat top-level id scan) so nested rows work.
    ///
    /// 🍿️ A row the surface answers ITSELF never reaches the guest — React's
    /// `dispatchTextEditorMenu` short-circuits its own `localActions` the same way.
    ///
    /// 🔀️ The picked value is MERGED into whatever the node authored, never
    /// substituted for it — see `merge_committed_args` (packet W15d).
    pub(crate) async fn handle_shell_hit(&mut self, hit: &HitTarget<ActionDescriptor>, input: &InputState<ActionDescriptor>) -> Result<bool, String> {
        let Some(id) = hit.control_id.as_deref() else {
            return Ok(false);
        };
        if let Some((window_id, chip)) = self.dock.window_pane_chip_target(id, &self.docs) {
            self.dock.toggle_window_pane_chip(&window_id, chip, &self.surfaces, &mut self.docs, &mut self.windows)?;
            return Ok(true);
        }
        if ui_wgpu::wgpu::arm_select_scroll(&mut self.input.scroll_offsets, id) {
            return Ok(true);
        }
        if let Some((kind, index)) = ShellPaletteKind::row_index(id) {
            match kind {
                ShellPaletteKind::Search => self.activate_search_item(index).await?,
                ShellPaletteKind::Find => self.activate_find_item(index).await?,
            }
            return Ok(true);
        }
        match id {
            "framework.hub.scrim" => return Ok(true),
            "framework.hub.signIn" => {
                self.opening.push_uri("/hub".to_string());
                self.as_actions().apply_shell_uri("/hub").await?;
                return Ok(true);
            }
            "ui.nav.back" => {
                if self.opening.uri_index > 0 {
                    self.opening.uri_index -= 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.forward" => {
                if self.opening.uri_index + 1 < self.opening.uri_history.len() {
                    self.opening.uri_index += 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.up" => {
                let uri = self.opening.shell_uri(&self.docs);
                if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                    if !parent.is_empty() {
                        self.opening.push_uri(parent);
                    }
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "playground.navbar.fixture" => {
                let open = matches!(&self.input.overlay_state, OverlayState::Dropdown(id) if id == "example");
                self.input.overlay_state = if open { OverlayState::None } else { OverlayState::Dropdown("example".to_string()) };
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.roles.") => {
                let requested = match id.trim_start_matches("playground.navbar.roles.") {
                    "editor" => semio_framework::manifest::AppRole::Editor,
                    "viewer" => semio_framework::manifest::AppRole::Viewer,
                    _ => return Ok(true),
                };
                self.as_actions().switch_to_session_role(requested).await?;
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.modes.") => {
                let mode_id = id.trim_start_matches("playground.navbar.modes.").to_string();
                self.as_window_actions().apply_navbar_mode(&mode_id).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.world3d.cancel::") => {
                let host_id = id.trim_start_matches("shell.world3d.cancel::").to_string();
                let action = world3d_cancel_affordance(self.surfaces.world3d_status.get(&host_id).map(String::as_str));
                let owner = self.surfaces.world3d_states.get(&host_id).map(|world| (world.controller_id.clone(), world.surface_id.clone()));
                if let (true, Some((controller_id, surface_id))) = (action.cancellable, owner) {
                    debug_log(&format!("[TRACE] shell world3d cancel {}", serde_json::json!({ "host": host_id, "surface": surface_id, "action": action.cancel_action })));
                    let mut args = action.cancel_args;
                    args.insert("surfaceId".to_string(), Value::String(surface_id));
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: action.cancel_action, args: Some(semio_framework::DslValue::from(Value::Object(args))) }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("framework.utility.collection.") => {
                let collection_id = id.trim_start_matches("framework.utility.collection.");
                let expanded = self.windows.utility_collection_expanded.get(collection_id).copied().unwrap_or(false);
                self.windows.utility_collection_expanded.insert(collection_id.to_string(), !expanded);
                return Ok(true);
            }
            id if id.starts_with("shell.example.") => {
                if !self.docs.session_example_rows(&self.plugins, &self.prefs).iter().any(|row| row.control_id == id) {
                    return Ok(true);
                }
                let example_id = id.trim_start_matches("shell.example.");
                self.plugins.active_example_id = Some(example_id.to_string());
                self.input.overlay_state = OverlayState::None;
                if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "setActiveExample".into(), args: crate::action_args_json!({ "exampleId": example_id }) }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.engagement.toggle.") => {
                let window_id = id.trim_start_matches("shell.engagement.toggle.");
                let activated = self.windows.engagement_activated.get(window_id).copied().unwrap_or(false);
                self.windows.engagement_activated.insert(window_id.to_string(), !activated);
                self.windows.engagement_expanded.insert(window_id.to_string(), !activated);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.focus.") => {
                let window_id = id.trim_start_matches("shell.measures.focus.");
                let expanded = self.windows.measures_expanded.get(window_id).copied().unwrap_or(false);
                self.windows.measures_expanded.insert(window_id.to_string(), !expanded);
                if !expanded {
                    self.windows.engagement_activated.remove(window_id);
                    self.windows.engagement_expanded.insert(window_id.to_string(), false);
                }
                return Ok(true);
            }
            "ui.search.toggle" => {
                self.commands.search_open = !self.commands.search_open;
                self.commands.find_open = false;
                self.input.overlay_state = if self.commands.search_open { OverlayState::Search } else { OverlayState::None };
                return Ok(true);
            }
            "ui.find.toggle" => {
                self.commands.find_open = !self.commands.find_open;
                self.commands.search_open = false;
                self.input.overlay_state = if self.commands.find_open { OverlayState::Find } else { OverlayState::None };
                return Ok(true);
            }
            "ui.panelToggle.display" => {
                self.dock.toggle_anchor(PanelAnchor::BottomLeft);
                return Ok(true);
            }
            "ui.panelToggle.workbench" => {
                self.dock.toggle_anchor(PanelAnchor::TopLeft);
                return Ok(true);
            }
            "ui.panelToggle.details" => {
                self.dock.toggle_anchor(PanelAnchor::TopRight);
                self.note_panel_toggle_command(id, "details").await?;
                return Ok(true);
            }
            "ui.panelToggle.settings" => {
                self.dock.toggle_anchor(PanelAnchor::BottomRight);
                self.note_panel_toggle_command(id, "settings").await?;
                return Ok(true);
            }
            "ui.panelToggle.chat" => {
                self.dock.toggle_anchor_tab(PanelAnchor::TopRight, FRAMEWORK_CHAT_PANEL_ID);
                self.note_panel_toggle_command(id, "chat").await?;
                return Ok(true);
            }
            "ui.panelToggle.command" => {
                self.dock.toggle_anchor_tab(PanelAnchor::BottomMiddle, FRAMEWORK_CATEGORY_COMMAND_ID);
                return Ok(true);
            }
            "ui.fullscreen.toggle" => {
                self.as_actions().apply_os_command("os.toggleFullscreen", None).await?;
                return Ok(true);
            }
            "space.canvas.home" => {
                let controller_id = self.plugins.host_controller_id().unwrap_or_default();
                self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "goHome".into(), args: None }).await?;
                return Ok(true);
            }
            "space.canvas.back" => {
                let has_focused_instance = if let Some(session) = self.docs.session.as_ref() { panel_state_from_view(&session.view_state)?.is_some_and(|panel| panel.active_spawned_id.is_some()) } else { false };
                if has_focused_instance {
                    let controller_id = self.plugins.host_controller_id().unwrap_or_default();
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: "closeFocusedInstance".into(), args: None }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".focus") => {
                if let Some((path, window_id)) = parse_dock_tab_action_id(id, "focus") {
                    self.dock.dock.set_stack_active(&path, window_id);
                    self.windows.active_window_id = Some(window_id.to_string());
                    let mut order = Vec::new();
                    dock_window_order(&self.dock.dock.root, &mut Vec::new(), &mut order);
                    if order.len() > 1 {
                        self.dock.dock.toggle_maximize(&path);
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".close") => {
                if let Some((_, window_id)) = parse_dock_tab_action_id(id, "close") {
                    let window_id = window_id.to_string();
                    self.as_session().close_dock_window_and_arm(&window_id, id);
                }
                return Ok(true);
            }
            id if id.starts_with("shell.layout.") => {
                let layout_id = id.trim_start_matches("shell.layout.");
                if let Some(layout) = self.dock.named_layout_by_id(layout_id, &self.docs) {
                    self.dock.layout_override = Some(layout);
                    self.as_session().sync_dock();
                    self.windows.active_window_id = self.dock.dock.active_window_id.clone();
                    self.as_actions().note_control_command(id, Some(serde_json::json!({ "layoutId": layout_id }))).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.mode.") => {
                let mode_id = id.trim_start_matches("shell.mode.");
                ({ let __arg0 = ActionDescriptor { controller_id: self.docs.session.as_ref().map(|s| s.app.controller_id.clone()).unwrap_or_default(), action: "setMode".into(), args: crate::action_args_json!({ "modeId": mode_id }) }; self.as_actions().dispatch_action(__arg0) }).await?;
                return Ok(true);
            }
            id if id.starts_with("framework.settings.appearance.") => {
                self.prefs.appearance_id = id.trim_start_matches("framework.settings.appearance.").to_string();
                return Ok(true);
            }
            UI_INTRODUCTION_VEIL_CONTROL_ID | UI_INTRODUCTION_SKIP_CONTROL_ID => {
                match self.docs.introduction_seen_key() {
                    Some(seen_key) => self.chrome.chrome_build.dismiss_introduction(&seen_key),
                    None => self.chrome.chrome_build.skip_introduction(),
                }
                return Ok(true);
            }
            UI_INTRODUCTION_NEXT_CONTROL_ID => {
                if let Some(step) = self.docs.chrome_tour_active_step(&self.chrome) {
                    self.docs.chrome_tour_advance_current_step(&step, &mut self.chrome);
                }
                return Ok(true);
            }
            UI_INTRODUCTION_BACK_CONTROL_ID => {
                self.chrome.chrome_build.back_introduction();
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.mobile.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.mobile.").to_string();
                let tabs = self.as_session().mobile_panel_tabs();
                self.dock.mobile_panel_path = mobile_panel_reconcile_path(&tabs, &[tab_id.clone()]);
                self.dock.mobile_panel_visible = true;
                self.arm_tool_for_panel_tab(&self.dock.mobile_panel_path.clone()).await?;
                return Ok(true);
            }
            "ui.mobilePanel.toggle" => {
                self.dock.mobile_panel_visible = !self.dock.mobile_panel_visible;
                return Ok(true);
            }
            id if self.dock.panel_tab_anchor(id).is_some() => {
                if let Some(anchor) = self.dock.panel_tab_anchor(id) {
                    let tab_id = id.to_string();
                    if hit.kind == HitKind::Toggle {
                        let was_visible = self.dock.anchor_open(anchor);
                        let was_path = self.dock.anchor_state(anchor).path.first().cloned();
                        self.dock.toggle_anchor_tab(anchor, &tab_id);
                        self.as_actions().journal_panel_selection(anchor, &tab_id, was_visible, was_path).await?;
                    } else {
                        self.select_panel_tab(anchor, &tab_id).await?;
                    }
                    self.arm_tool_for_panel_tab(&self.dock.anchor_state(anchor).path.clone()).await?;
                }
                return Ok(true);
            }
            id if self.input.context_menu.as_ref().is_some_and(|menu| {
                let mut prefix = Vec::new();
                context_menu_path_for_item_id(&menu.items, id, &mut prefix).is_some()
            }) =>
            {
                let disabled = self.input.context_menu.as_ref().is_some_and(|menu| {
                    let mut prefix = Vec::new();
                    context_menu_path_for_item_id(&menu.items, id, &mut prefix).and_then(|path| context_menu_item_at_path(&menu.items, &path)).is_some_and(|item| item.disabled)
                });
                if disabled {
                    return Ok(true);
                }
                let submenu_path = self.input.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    (!context_menu_item_at_path(&menu.items, &path)?.children.is_empty()).then_some(path)
                });
                if let Some(path) = submenu_path {
                    if let Some(menu) = self.input.context_menu.as_mut() {
                        menu.active = path;
                        menu.submenu_collapsed_at = None;
                    }
                    return Ok(true);
                }
                let action = self.input.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    context_menu_item_at_path(&menu.items, &path)?.action.clone()
                });
                let origin = self.input.context_menu.as_ref().map(|menu| (menu.x, menu.y));
                self.input.context_menu = None;
                if let Some(action) = action {
                    let claimed = origin.is_some_and(|(x, y)| self.input.claim_text_editor_menu_action(&action, x, y));
                    if !claimed {
                        self.as_actions().dispatch_action(action).await?;
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("section.chevron.") => {
                let section_id = id.trim_start_matches("section.chevron.");
                let key = format!("section.{section_id}");
                let collapsed = self.input.collapsed_sections.get(&key).copied().unwrap_or(false);
                let next_open = !self.input.canonical_tree_open(section_id, !collapsed);
                self.input.collapsed_sections.insert(key, !next_open);
                self.input.set_canonical_tree_open(section_id, next_open);
                return Ok(true);
            }
            id if id.starts_with("tree.chevron.") => {
                let item_id = id.trim_start_matches("tree.chevron.");
                self.input.toggle_immediate_tree_item(item_id);
                return Ok(true);
            }
            id if id.contains(".vfs.chevron.") => {
                if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.chevron.") {
                    toggle_vfs_row_expanded(surface_id, row_id);
                    return Ok(true);
                }
            }
            id if self.input.widget_maps.select_metas.contains_key(id) => {
                let opening = !self.input.open_selects.get(id).copied().unwrap_or(false);
                self.input.close_open_selects();
                self.input.open_selects.insert(id.to_string(), opening);
                return Ok(true);
            }
            id if id.contains(".item.") => {
                if let Some((select_id, value)) = id.rsplit_once(".item.") {
                    if let Some(action) = self.input.widget_maps.select_metas.get(select_id).cloned() {
                        ui_wgpu::wgpu::clear_select_scroll(&mut self.input.scroll_offsets, select_id);
                        self.input.open_selects.insert(select_id.to_string(), false);
                        let args = merge_committed_args(action.args.as_ref(), semio_framework_value::DslValue::String(value.to_string()));
                        self.as_actions().dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args }).await?;
                        return Ok(true);
                    }
                }
            }
            id if self.input.widget_maps.toggle_metas.contains_key(id) => {
                if let Some((pressed, action)) = self.input.widget_maps.toggle_metas.get(id).cloned() {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: crate::action_args_json!({ "pressed": !pressed }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".minus") => {
                let base = id.trim_end_matches(".minus");
                if let Some(meta) = self.input.widget_maps.stepper_metas.get(base).cloned() {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: crate::action_args_json!({ "delta": -meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".plus") => {
                let base = id.trim_end_matches(".plus");
                if let Some(meta) = self.input.widget_maps.stepper_metas.get(base).cloned() {
                    self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: crate::action_args_json!({ "delta": meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.starts_with("tree.label.") => {
                let item_id = id.trim_start_matches("tree.label.");
                if hit.drag_data.is_some() {
                    return Ok(true);
                }
                if tree_row_has_chevron(input, item_id) {
                    self.input.toggle_immediate_tree_item(item_id);
                }
                self.input.queue_tree_selection(item_id, &mut self.settle);
                return Ok(false);
            }
            id if tree_drag_handle_item_id(id).is_some() => return Ok(true),
            _ => {}
        }
        Ok(false)
    }
}

/// 🛑️🖱️ Whether a PRESS at this hit belongs to the shell's own chrome rather than to the engine
/// surface underneath it.
///
/// 🩸️ The World3d overlay row — the compute-status pill and the cancel control
/// ([`surface_overlay_controls_for`]) — is painted INSIDE the surface's own rect, and the
/// renderer's press path claimed a point for a World3d surface on `bounds.contains` alone and
/// returned before the shell ever saw it (`🧊️renderer/🦀️.rs`, the `world_consumed` guard). The
/// RELEASE path calls the shell first, but `handle_shell_hit` deliberately fires on the press —
/// so the only pointer phase that reached the shell over a World3d surface was the one it
/// ignores. Measured on 6118: the cancel control paints, hit-tests as
/// `NavbarItem("shell.world3d.cancel::procedural-preview")`, and a click on it dispatched
/// NOTHING, every time (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-progress-visibility-2026-09-14.md`). A cancel that cannot be pressed is not a cancel.
///
/// ⚖️ Chrome kinds, plus the two NAMESPACES the shell alone mints. It is still never a list of
/// control ids — `shell.` and `ui.introduction.` are prefixes no engine surface can produce,
/// because every surface-minted target is keyed by its own `surface_id`
/// (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`'s `scroll_key(&scene.surface_id, …)` and siblings).
///
/// 🩸️ Kinds ALONE were not enough, and the gap swallowed a whole control family. A pane's
/// overlay chips — `Actions`, `Search`, `Window Options`, `Utilities` and `Projection` — are
/// `HitKind::Toggle` painted at React's `Pane` anchors INSIDE a world pane's rect
/// ([`Self::window_pane_chips`]), so a press on any of them resolved to a chrome target, failed
/// this predicate, and was handed to `enqueue_world3d_event` instead: `handle_shell_hit` never
/// ran, the fold never flipped, and the unfolded body therefore never painted. Measured live at
/// 1440×900 (`🗑️generated/w9b-projection-live/console.txt`): the press on
/// `shell.projection.fold.puzzle3d-main-perspective` produced exactly ONE
/// `wgpu-shell pointer button` line, `down=false` — the release the shell ignores — and the hit
/// ledger stayed at 43 rows through ten 1.5 s retries.
///
/// 🧯️ React has no such gap to close: a `Pane` chip is a DOM button in a layer ABOVE the
/// `<canvas>` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `Pane`), so the canvas never sees the press at
/// all. The `ui.introduction.` namespace is that same rule for the tour's veil and its three
/// card controls, which own EVERY pointer in the app while they block
/// (`📓️w5b-interaction-parity-probe.md` §4.3).
/// 🩸️ The dock's own STRUCTURE was the second family the kind list missed. A split gutter is
/// registered at the seam between two panes, and a pane's rect starts ON that seam
/// (`puzzle3d-main-perspective@1063x914+534,54` against `dock.split..0@20x936+524,32`), so the
/// press on the gutter resolved to `HitKind::DockSplit`, failed this predicate, and was enqueued
/// into the world surface instead — `begin_drag` never ran, the split never moved and the
/// release journalled no `shell.windowResize` where React journals one
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 15).
/// `PanelResize`, `PanelTab` and the join corner share the geometry and therefore the rule; the
/// `dock.` namespace is the dock's twin of `shell.`, minted by [`crate::dock`] alone.
pub fn pointer_press_belongs_to_shell_chrome(hit: Option<&HitTarget<ActionDescriptor>>) -> bool {
    pointer_hit_owner(hit) == PointerHitOwner::Chrome
}

/// 🎯️ WHO owns a pointer at this hit — the whole DOM stacking rule in one answer, and the single
/// routing predicate the renderer's press, move, release and wheel ingress all ask.
///
/// ⚖️ The topmost hit is the owner: chrome painted over an engine surface takes the pointer from
/// it, exactly as a DOM element above the `<canvas>` does, and a point that hits nothing is the
/// surface's. [`Self::pointer_press_belongs_to_shell_chrome`] is this answer read as a bool. A retained pane body's rows are
/// keyed `<surface>/<id>` (`PanelProjection::key`), so it is their surface-local id that names them chrome.
pub fn pointer_hit_owner(hit: Option<&HitTarget<ActionDescriptor>>) -> PointerHitOwner {
    let chrome = hit.is_some_and(|hit| {
        matches!(hit.kind, HitKind::NavbarItem | HitKind::DropdownItem | HitKind::ContextMenu | HitKind::Select | HitKind::DockSplit | HitKind::DockJoinCorner | HitKind::PanelResize | HitKind::PanelTab)
            || hit.control_id.as_deref().is_some_and(|id| {
                id.starts_with("shell.")
                    || id.starts_with("ui.introduction.")
                    || id.starts_with("dock.")
                    || id.starts_with("panel.resize.")
                    || id.starts_with(WINDOW_PANE_CHIP_PARENT)
                    || id.starts_with(WORLD_PROJECTION_PANE_PARENT)
                    || id.split_once('/').is_some_and(|(_, local)| local.starts_with(WORLD_PROJECTION_PANE_PARENT))
                    || id.starts_with(WINDOW_UTILITY_RAIL_PARENT)
            })
    });
    if chrome {
        PointerHitOwner::Chrome
    } else {
        PointerHitOwner::Surface
    }
}

/// 🖱️ Whether a LIVE drag has captured the pointer for the shell's own chrome — the MOVE twin of
/// [`Self::pointer_press_belongs_to_shell_chrome`], and React's `setPointerCapture` on a resize
/// handle. The dock's structural drags travel across whatever the pointer sweeps over, so the
/// engine surfaces under it must not also receive those moves.
pub fn drag_captures_pointer(drag: &ui_wgpu::wgpu::DragState) -> bool {
    drag.active && matches!(drag.kind, Some(HitKind::DockSplit | HitKind::DockJoinCorner | HitKind::PanelResize | HitKind::PanelTab))
}

//#endregion ShellInput

//#region ShellInput
//#region 🎯️RetainedPointerRouting
impl<'ctx> PointerCtx<'ctx> {
    /// 👆️ A press on a retained body target. A control the retained router itself owns (a text
    /// `Input`'s focus, a `Select`'s popup and options, a tree row's selection or drag handle) is routed there;
    /// everything else dispatches the action the node's own spec declared through the
    /// same `dispatch_action` funnel every chrome hit uses, so a document row and a chrome row are
    /// one dispatch path.
    ///
    /// 🎛️ EVERY retained interactive control routes into the retained router, which is the ONE
    /// authority that turns a press on one into its own guest action (`events`' 🔖️Commit region)
    /// and retires any overlay the gesture owned. Select options are synthesized retained Stacks
    /// published as `HitKind::Button`; dispatching that hit directly fired `setAppearance` while
    /// bypassing `OverlayClosed`, leaving the listbox painted over the updated Settings panel.
    /// Restricting this to `Input`/`Select` left a `Toggle`/`Slider`/`NumberStepper`/`Ring`/
    /// `IconSelect` press falling into the `action`-dispatch branch below with `action: None` —
    /// i.e. dispatching nothing at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️audit-wgpu-parity-2026-09-13.md` gaps #1/#6).
    ///
    /// 🪟️ Pressing a window's BODY activates that window, on the press, exactly as pressing its
    /// chrome does. The keyboard follows `active_window_id`
    /// (`handle_keyboard_async`'s content-focus routing reads `content_has_focus(active)`), and a
    /// retained body press was the one way into a window that never set it: focus landed on the
    /// pressed `Input` — `content focus window=generation3d-generations node=Some(..)` — while
    /// every keystroke was still routed at `procedural-main`, the app's first window kind, whose
    /// content had no focus. So an inline rename editor could be opened and never typed into, on
    /// any window but the first (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-generation-publication-2026-09-13.md`).
    ///
    /// 🪟️ React's `activateWindow` is `setActiveWindowInLayout(prev, windowId)` plus
    /// `onActiveWindowChange` (`🧱️elements/🎨️Canvas/🟦️.tsx:1446-1452`) — the DOCK's notion of the
    /// active stack moves with the shell's active window, which is what paints the stack accent.
    /// Setting `active_window_id` alone left `dock.active_stack` on the previously focused
    /// stack, so a body press focused a window whose stack never lit up.
    ///
    /// 👆️ A click is press AND release, and the action fires on the RELEASE — the same rule
    /// `events::EventRouter` already applies to a `Button` (`PointerUp` inside the pressed
    /// node), and the same rule a browser applies.
    ///
    /// 🩸️ This used to fire on the press instead, and on 6118 that meant it never fired at
    /// all: `InputState`'s pointer registry is drained one entry per frame-build boundary
    /// step (`FrameBuildPhase::InputFrame`), so a `PointerDown` arriving inside that window
    /// resolves an EMPTY registry. Browser-measured, 2026-09-13: every `PointerDown` reported
    /// `targets=0 hit=None` while the matching `PointerUp` reported `targets=36` and the
    /// correct row — so `Add Generation` resolved perfectly and dispatched nothing
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-retained-controls-wires-2026-09-13.md` §7).
    ///
    /// 🪟️ The row belongs to THIS window instance, so its dispatch must be addressed to it
    /// rather than to whatever window happens to be focused.
    pub(crate) async fn route_retained_pointer_press(
        &mut self,
        window_id: &str,
        body: Rect,
        x: f32,
        y: f32,
        down: bool,
        button: i16,
        kind: HitKind,
        action: Option<ActionDescriptor>,
        pointer_id: ui_render::PointerId,
        input: &mut InputState<ActionDescriptor>,
    ) -> Result<(), String> {
        debug_log(&format!("[TRACE] wgpu-shell retained press window={window_id} kind={kind:?} down={down} action={:?}", action.as_ref().map(|descriptor| descriptor.action.clone())));
        if down && !self.as_session().retained_surface_is_panel(window_id) {
            let owner = window_pane_owner(window_id).unwrap_or(window_id).to_string();
            self.chrome.chrome_build.clear_sibling_pane_focus(window_id);
            self.dock.dock.sync_active_window(&owner);
            self.windows.active_window_id = Some(owner);
        }
        if retained_router_owns_pointer(kind) {
            let event = if down {
                ui_wgpu::wgpu::UiEvent::PointerDown { x: x - body.x, y: y - body.y, button: retained_pointer_button(button), modifiers: retained_event_modifiers(input) }
            } else {
                ui_wgpu::wgpu::UiEvent::PointerUp { x: x - body.x, y: y - body.y, button: retained_pointer_button(button), modifiers: retained_event_modifiers(input) }
            };
            let commands = crate::interpreter::dispatch_presented_ui_pointer_event(window_id, pointer_id, event, input);
            self.chrome.chrome_build.note_content_focus_commands(&commands);
        } else if !down {
            if let Some(mut action) = action {
                scope_action_to_window(&mut action, window_id);
                self.as_actions().dispatch_action(action).await?;
            }
        }
        self.as_actions().drain_gesture_bound_work().await;
        self.settle.owe_settle();
        Ok(())
    }
}
//#endregion 🎯️RetainedPointerRouting
//#endregion ShellInput
