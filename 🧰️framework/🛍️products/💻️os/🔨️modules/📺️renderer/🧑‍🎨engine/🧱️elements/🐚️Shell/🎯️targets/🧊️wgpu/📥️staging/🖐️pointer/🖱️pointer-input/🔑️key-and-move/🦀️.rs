//! 🔑️ Key and move: the key action to ui event mapping, the tree chevron and host blur checks and the pointer cancel, move and dock drag finish handlers. See [`super`].

use super::*;

//#region ShellInput
/// ⌨️ Maps a chrome-level `ui_wgpu::wgpu::KeyAction` (+ modifiers) to the `ui_wgpu::wgpu::UiEvent` the retained
/// content engine's `events::EventRouter::dispatch` expects — mirrors that fn's `UiEvent::KeyDown`
/// key-string vocabulary exactly (`"ArrowLeft"`/`"Backspace"`/`"c"`+ctrl for copy/etc., see
/// `ui_wgpu`'s `🔖️EditRouting`/`🔖️UiCommand` regions). A `Char` held with Ctrl/Cmd routes as
/// `KeyDown` (so `c`/`x`/`v` clipboard chords reach `route_edit_key` instead of being literally
/// inserted as text). A focused Select owns printable typeahead and the Space press as `KeyDown`;
/// editable controls keep plain characters as `TextInput`.
pub fn ui_event_from_key_action(action: &semio_framework_ui_wgpu_input::KeyAction, modifiers: &PointerModifiers, focused_select: bool) -> Option<semio_framework_ui_wgpu_events::UiEvent> {
    let event_modifiers = semio_framework_ui_wgpu_events::EventModifiers { shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta };
    match action {
        semio_framework_ui_wgpu_input::KeyAction::Char(ch) => {
            if modifiers.ctrl_or_meta() || focused_select {
                Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: ch.clone(), modifiers: event_modifiers })
            } else {
                Some(semio_framework_ui_wgpu_events::UiEvent::TextInput { text: ch.clone() })
            }
        }
        semio_framework_ui_wgpu_input::KeyAction::Backspace => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "Backspace".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Delete => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "Delete".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Enter => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "Enter".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Escape => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "Escape".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::ArrowLeft => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::ArrowRight => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "ArrowRight".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::ArrowUp => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "ArrowUp".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::ArrowDown => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "ArrowDown".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Home => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "Home".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::End => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "End".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::PageUp => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "PageUp".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::PageDown => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "PageDown".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Function(number) => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: format!("F{number}"), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Tab => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "Tab".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Space(true) if focused_select => Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: " ".into(), modifiers: event_modifiers }),
        semio_framework_ui_wgpu_input::KeyAction::Space(_) => None,
    }
}

/// 🌳️ Whether this frame painted a fold chevron for `item_id` — the row has children.
pub fn tree_row_has_chevron(input: &InputState<ActionDescriptor>, item_id: &str) -> bool {
    let chevron = format!("tree.chevron.{item_id}");
    input.hits().iter().chain(input.staged_hits().iter()).any(|hit| hit.control_id.as_deref() == Some(chevron.as_str()))
}

//#region 🫥️HostWindowBlur
/// 🫥️ Whether a host window blur waits for the shell's drain.
pub fn host_window_blur_pending() -> bool {
    *HOST_WINDOW_BLUR_PENDING.borrow()
}
//#endregion 🫥️HostWindowBlur

impl<'ctx> PointerCtx<'ctx> {
    /// 🛑️ Retires a cancelled pointer sequence without a click, drop, or release commit.
    pub fn handle_pointer_cancel(&mut self, input: &mut InputState<ActionDescriptor>) {
        self.handle_pointer_cancel_for(ui_render::PointerId(1), input);
    }

    pub fn handle_pointer_cancel_for(&mut self, pointer_id: ui_render::PointerId, input: &mut InputState<ActionDescriptor>) {
        let retained_cancelled = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(pointer_id);
        if let Some(window_id) = retained_cancelled.as_ref() {
            semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_pointer_event(&window_id, pointer_id, semio_framework_ui_wgpu_events::UiEvent::PointerCancel, input);
        }
        let (pointer_x, pointer_y) = (input.pointer_x, input.pointer_y);
        let cancelled_scenes = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::cancel_scene_pointer(pointer_id, input);
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
                semio_framework_ui_declaration::ui::SurfaceKind::World3d => {
                    if let Some(state) = self.surfaces.world3d_states.get_mut(&cancelled.host_id) {
                        infinite_world::world::world3d_cancel_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene);
                    }
                }
                semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph => {
                    semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_pointer_cancel_into(&cancelled.host_id);
                }
                semio_framework_ui_declaration::ui::SurfaceKind::Board2d => {
                    semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_pointer_cancel_into(&cancelled.host_id, input);
                }
                semio_framework_ui_declaration::ui::SurfaceKind::TiledMap => {
                    if let Some(surface) = self.surfaces.tiled_map_states.get(&cancelled.host_id).filter(|surface| surface.window_id == cancelled.window_id) {
                        if let Err(fault) = semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_cancel_into(&cancelled.host_id, &surface.controller_id, surface.bounds, pointer_x, pointer_y, &mut *input) {
                            input.record_action_fault(fault);
                        }
                    }
                }
                _ => {}
            }
        }
        let canvas_cancelled = semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_pointer_gesture_for(pointer_id, input);
        let transfer_cancelled = semio_framework_os_renderer_wgpu_scene_render::cancel_scene_list_transfer_for_pointer(pointer_id);
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
            semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_sync_flow_widget_ghost(x, y, drag_data, &graph_surfaces);
            drop(graph_surfaces);
            self.surfaces.sync_world3d_catalogue_drop_preview(x, y, drag_data);
        } else {
            self.input.sync_world3d_catalogue_drop_preview_from_retained_ui(&mut self.surfaces);
        }
        let retained_drag_data = tree_drag_payload
            .is_none()
            .then(|| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::active_retained_drag_sessions())
            .and_then(|sessions| sessions.into_iter().map(|(_, _, _, drag_data)| drag_data).find(|drag_data| drag_data.contains_key(semio_framework_os_renderer_wgpu_scene_render::CANVAS_CATALOGUE_DRAG_MIME)));
        if let Some(drag_data) = tree_drag_payload.as_ref().or(retained_drag_data.as_ref()) {
            self.input.sync_canvas2d_catalogue_drag(x, y, drag_data, input);
        } else {
            semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_catalogue_drag(input);
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
    pub async fn finish_dock_drag(&mut self, x: f32, y: f32, input: &InputState<ActionDescriptor>) -> Result<(), String> {
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

}
//#endregion ShellInput
