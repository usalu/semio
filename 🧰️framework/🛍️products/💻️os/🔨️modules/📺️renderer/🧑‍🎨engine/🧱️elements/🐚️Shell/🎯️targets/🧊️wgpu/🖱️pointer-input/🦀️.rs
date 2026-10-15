//! 🖱️ Shell input, part one: pointer handling and the retained hit registry.
//!
//! The chrome build-state input impl, key-action translation, tree helpers, host window blur, the pointer
//! handlers, the retained hit registry and the presented-input candidate.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellInput
impl ShellChromeBuildState {
    pub(crate) fn content_has_focus(&self, window_id: &str) -> bool {
        self.content_focus_node(window_id).is_some()
    }

    /// 🎯️ WHICH retained node holds focus in `window_id` — React's `surfaceActiveRoot`. A chrome verb
    /// that must address the focused content node (a panel-level Copy, an inspector commit) reads this
    /// instead of re-deriving focus from the tree.
    fn content_focus_node(&self, window_id: &str) -> Option<ui_wgpu::wgpu::NodeId> {
        self.content_focus.get(window_id).and_then(Option::as_ref).map(|focus| focus.node)
    }

    pub(crate) fn note_content_focus_commands(&mut self, commands: &[ui_wgpu::wgpu::UiCommand]) {
        for command in commands {
            if let ui_wgpu::wgpu::UiCommand::FocusChanged { window_id, node } = command {
                ShellState::debug_log(&format!("[TRACE] wgpu-shell content focus window={window_id} node={node:?}"));
                let focus = node.and_then(|node| crate::interpreter::retained_node_focus_identity(window_id, node).map(|(key, kind)| RetainedContentFocus { node, key, kind }));
                if focus.is_some() {
                    if let Some(previous) = self.focused_retained_surface.replace(window_id.clone()).filter(|previous| previous != window_id) {
                        self.content_focus.insert(previous, None);
                    }
                } else if self.focused_retained_surface.as_deref() == Some(window_id.as_str()) {
                    self.focused_retained_surface = None;
                }
                self.content_focus.insert(window_id.clone(), focus);
            }
        }
    }

    pub(crate) fn clear_content_focus(&mut self, surface: &str) {
        self.content_focus.remove(surface);
        if self.focused_retained_surface.as_deref() == Some(surface) {
            self.focused_retained_surface = None;
        }
    }

    fn clear_sibling_pane_focus(&mut self, surface: &str) {
        let owner = window_pane_owner(surface).unwrap_or(surface);
        for sibling in [owner.to_string(), window_measures_surface_id(owner), window_actions_surface_id(owner), window_search_surface_id(owner), window_projection_surface_id(owner)] {
            if sibling != surface {
                self.clear_content_focus(&sibling);
            }
        }
    }
}

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

impl ShellState {
    pub(crate) fn retained_keyboard_focus(&self) -> Option<(String, ui_wgpu::wgpu::NodeId)> {
        let surface = self.chrome_build.focused_retained_surface.as_ref()?;
        let owner = window_pane_owner(surface).unwrap_or(surface);
        if !self.retained_surface_is_panel(surface) && self.active_window_id.as_deref() != Some(owner) {
            return None;
        }
        let focus = self.chrome_build.content_focus.get(surface)?.as_ref()?;
        crate::interpreter::retained_node_has_visible_focus(surface, focus.node, &focus.key, focus.kind).then(|| (surface.clone(), focus.node))
    }

    pub(crate) fn retained_select_owns_keyboard(&self) -> bool {
        self.retained_keyboard_focus().is_some_and(|(surface, node)| crate::interpreter::retained_node_is_select(&surface, node))
    }
}

fn tree_drag_handle_item_id(control_id: &str) -> Option<&str> {
    control_id.strip_prefix("tree.drag.sort.").or_else(|| control_id.strip_prefix("tree.drag.transfer."))
}

pub(crate) fn tree_hit_item_id(control_id: &str) -> Option<&str> {
    control_id.strip_prefix("tree.label.").or_else(|| tree_drag_handle_item_id(control_id))
}

/// 🌳️ Whether this frame painted a fold chevron for `item_id` — the row has children.
fn tree_row_has_chevron(input: &InputState<ActionDescriptor>, item_id: &str) -> bool {
    let chevron = format!("tree.chevron.{item_id}");
    input.hits().iter().chain(input.staged_hits().iter()).any(|hit| hit.control_id.as_deref() == Some(chevron.as_str()))
}

//#region 🫥️HostWindowBlur
/// 🧯️ Frames a closed World3d window waits at most for its capture-lost cancel to drain before it retires regardless.
pub(crate) const WORLD3D_CLOSE_CANCEL_SIGHTINGS: u8 = 8;

/// 🫥️ Whether the host window lost the system focus since the shell last looked (native winit `Focused(false)`, the browser
/// page's `window` blur through the `semioWgpuHostWindowBlur` door). A worker cell, so a law's thread owns its own.
pub(crate) static HOST_WINDOW_BLUR_PENDING: crate::element_contracts::WorkerCell<bool> = crate::element_contracts::WorkerCell::new(Default::default);

/// 🫥️ The host window lost the system focus: every open text-editor typing run ends at once (`blur`, design §13.2 of ticket
/// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), and the shell's next drain blurs the focused pane for its program
/// ([`ShellState::arm_host_window_blur`]) — what React's pane `onBlur` does when the page loses focus.
pub fn note_host_window_blur() {
    crate::engine_canvas::end_every_text_editor_typing(crate::engine_canvas::TextEditorTypingEnd::Blur);
    *HOST_WINDOW_BLUR_PENDING.borrow_mut() = true;
}

/// 🫥️ Whether a host window blur waits for the shell's drain.
pub(crate) fn host_window_blur_pending() -> bool {
    *HOST_WINDOW_BLUR_PENDING.borrow()
}

/// 🫥️ The browser frame Worker's window-blur door (`host-window-blur`): the page's `window` lost focus.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuHostWindowBlur)]
pub fn semio_wgpu_host_window_blur() {
    note_host_window_blur();
}
//#endregion 🫥️HostWindowBlur

/// 📨️ `hostEvent{windowId, kind}` on `controller_id`: the window fact the program answers by ending an open gesture there —
/// the wgpu twin of React's `windowHostEventHandlersV1` (a pane that loses the activation is blurred, a cancelled pointer
/// sequence loses its capture).
pub(crate) fn window_host_event_action(controller_id: &str, window_id: &str, kind: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: semio_framework::HOST_EVENT_ACTION_ID.into(), args: crate::action_args_json!({ "windowId": window_id, "kind": kind }) }
}

/// 🌳️ Retained controls whose complete pointer gesture belongs to `EventRouter`.
fn retained_router_owns_pointer(kind: HitKind) -> bool {
    matches!(kind, HitKind::Button | HitKind::Input | HitKind::Select | HitKind::Toggle | HitKind::Slider | HitKind::NumberStepper | HitKind::Ring | HitKind::IconSelect | HitKind::TreeItem | HitKind::TreeDragHandle | HitKind::ComponentScene)
}

impl ShellState {
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
        if let Some(controller_id) = self.shell_command_controller_id() {
            for window_id in capture_lost {
                self.deferred_actions.push(window_host_event_action(&controller_id, window_id, semio_framework::HOST_EVENT_KIND_CAPTURE_LOST));
            }
        }
        for cancelled in cancelled_scenes {
            match cancelled.kind {
                ui_wgpu::wgpu::SurfaceKind::World3d => {
                    if let Some(state) = self.world3d_states.get_mut(&cancelled.host_id) {
                        infinite_world::world::world3d_cancel_relocate_drag(state);
                    }
                }
                ui_wgpu::wgpu::SurfaceKind::NodeGraph => {
                    crate::engine_canvas::node_graph_pointer_cancel_into(&cancelled.host_id);
                }
                ui_wgpu::wgpu::SurfaceKind::Board2d => {
                    crate::engine_canvas::puzzle_board_pointer_cancel_into(&cancelled.host_id, input);
                }
                ui_wgpu::wgpu::SurfaceKind::TiledMap => {
                    if let Some(surface) = self.tiled_map_states.get(&cancelled.host_id).filter(|surface| surface.window_id == cancelled.window_id) {
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
        let chrome_cancelled = self.chrome_pointer_owner == Some(pointer_id);
        if chrome_cancelled {
            self.chrome_pointer_owner = None;
            self.dock_tab_drag = None;
            self.dock_drag = None;
            self.pending_dock_drag = None;
            self.tree_drag = None;
            self.pending_tree_drag = None;
            self.pending_chrome_release = None;
            self.right_click = RightClickState::default();
        }
        if retained_cancelled.is_some() || scene_cancelled || canvas_cancelled || transfer_cancelled || chrome_cancelled {
            input.pointer_down = false;
            input.hovered_id = None;
            input.drag = Default::default();
        }
    }

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
        Self::debug_log(&format!(
            "[TRACE] wgpu-shell pointer button x={x} y={y} down={down} button={button} targets={} staged={} gen={} hit={:?}",
            input.hits().len(),
            input.staged_hits().len(),
            input.hit_generation(),
            input.hit_at(x, y).map(|target| (target.kind, target.control_id.clone(), target.event.as_ref().map(|descriptor| descriptor.action.clone())))
        ));
        if !down && self.chrome_pointer_owner.is_some_and(|owner| owner != pointer_id) {
            return Ok(());
        }
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.pointer_button = button;
        if !down {
            if self.chrome_pointer_owner == Some(pointer_id) {
                self.chrome_pointer_owner = None;
            }
            let retained_canvas_drag = crate::interpreter::active_retained_drag_sessions().into_iter().map(|(_, _, _, drag_data)| drag_data).find(|drag_data| drag_data.contains_key(crate::scenes::CANVAS_CATALOGUE_DRAG_MIME));
            if let Some((tab_id, from_anchor)) = self.dock_tab_drag.take() {
                self.finish_dock_tab_drag(&tab_id, from_anchor, x, y, theme);
            }
            let dock_drop_result = if self.dock_drag.is_some() {
                self.finish_dock_drag(x, y, input).await
            } else {
                self.pending_dock_drag = None;
                Ok(())
            };
            if let Some(control_id) = self.pending_chrome_release.take() {
                let same_control = input.hit_at(x, y).and_then(|hit| hit.control_id.as_deref()) == Some(control_id.as_str());
                if let Some((kind, index)) = ShellPaletteKind::row_index(&control_id) {
                    if same_control && button == 0 {
                        match kind {
                            ShellPaletteKind::Search => self.activate_search_item(index).await?,
                            ShellPaletteKind::Find => self.activate_find_item(index).await?,
                        }
                        input.blur_input();
                        self.accessibility_focused_control_id = None;
                    }
                    crate::scenes::cancel_scene_list_transfer();
                    return dock_drop_result;
                }
                if ShellPaletteKind::from_close_id(&control_id).is_some() {
                    if same_control && button == 0 {
                        self.close_command_palette(input);
                    }
                    crate::scenes::cancel_scene_list_transfer();
                    return dock_drop_result;
                }
                if let Some(window_id) = dock_tab_select_window_id(&control_id).filter(|_| same_control) {
                    if self.dock.activate_window(window_id) {
                        self.active_window_id = Some(window_id.to_string());
                    }
                }
            }
            if self.tree_drag.is_some() {
                self.finish_tree_drag(x, y, input).await?;
            } else if let Some((item_id, _)) = self.pending_tree_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) == Some(item_id.as_str()) {
                        if tree_row_has_chevron(input, &item_id) {
                            self.toggle_immediate_tree_item(&item_id);
                        }
                        self.dispatch_tree_selection(&item_id);
                        if let Some(action) = hit.event.clone() {
                            self.dispatch_action(action).await?;
                        }
                    }
                }
            }
            let canvas_drop_accepted = retained_canvas_drag.as_ref().is_some_and(|drag_data| self.finish_canvas2d_catalogue_drop(x, y, drag_data, input));
            if input.drag.active {
                let drag_target = input.drag.target_id.clone();
                self.dispatch_widget_drag_values(input).await?;
                input.end_drag();
                if drag_target.as_deref().is_some_and(|id| id.starts_with("dock.split.") || id.starts_with("dock.corner.")) {
                    self.persist_dock_layout();
                    if let Some(controller_id) = self.shell_command_controller_id() {
                        let note = Self::note_shell_command_action(&controller_id, "shell.windowResize", shell_chrome_string("command.resizeWindow", self.locale_id == "de"), None);
                        self.dispatch_action(note).await?;
                    }
                }
            }
            let captured_release = crate::interpreter::retained_pointer_capture_window(pointer_id).and_then(|window_id| self.retained_window_body_rect(&window_id).map(|body| (window_id, body)));
            let retained_release = input
                .hit_at(x, y)
                .cloned()
                .and_then(|hit| self.retained_hit_window(&hit).map(|(window_id, body)| (window_id, body, hit.kind, hit.event)))
                .or_else(|| captured_release.clone().map(|(window_id, body)| (window_id, body, HitKind::ComponentScene, None)));
            if canvas_drop_accepted {
                if let Some((captured_window, captured_body)) = captured_release {
                    self.route_retained_pointer_press(&captured_window, captured_body, captured_body.x - 1.0, captured_body.y - 1.0, false, button, HitKind::ComponentScene, None, pointer_id, input).await?;
                }
            } else if let Some((window_id, body, kind, action)) = retained_release {
                let released_window = window_id.clone();
                self.route_retained_pointer_press(&window_id, body, x, y, false, button, kind, action, pointer_id, input).await?;
                if let Some((captured_window, captured_body)) = captured_release.filter(|(captured_window, _)| captured_window != &released_window) {
                    self.route_retained_pointer_press(&captured_window, captured_body, x, y, false, button, HitKind::ComponentScene, None, pointer_id, input).await?;
                }
            }
            crate::scenes::cancel_scene_list_transfer();
            return dock_drop_result;
        }
        if button == 2 {
            let hit = input.hit_at(x, y).cloned();
            if input.modifiers.alt && self.claim_text_editor_alt_completions(x, y) {
                self.right_click = RightClickState { pending: true, x, y };
                return Ok(());
            }
            if let Some((window_id, body)) = hit.as_ref().and_then(|hit| self.retained_canvas2d_hit_window(hit, x, y)) {
                let scene_hit = hit.as_ref().expect("the retained Canvas2d owner was resolved from this hit");
                self.route_retained_pointer_press(&window_id, body, x, y, true, button, scene_hit.kind, scene_hit.event.clone(), pointer_id, input).await?;
            }
            self.open_context_menu(x, y, hit).await;
            self.right_click = RightClickState { pending: true, x, y };
            return Ok(());
        }
        if self.dismiss_overlays(x, y, input) {
            return Ok(());
        }
        if input.hit_at(x, y).is_none() && self.presented_input_geometry.dock_canvas_bounds.contains(x, y) {
            self.deactivate_active_window();
        }
        if let Some(hit) = input.hit_at(x, y).cloned() {
            if let Some((window_id, body)) = self.retained_hit_window(&hit) {
                self.begin_window_template_drag_from_hit(pointer_id, &hit, x, y);
                self.route_retained_pointer_press(&window_id, body, x, y, true, button, hit.kind, hit.event.clone(), pointer_id, input).await?;
                return Ok(());
            }
            if hit.kind == HitKind::PanelResize {
                if !self.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(window_id) = id.strip_prefix("shell.measures.resize.") {
                        self.measures_resize_window_id = Some(window_id.to_string());
                        self.measures_resize_origin_width = *self.measures_width.get(window_id).unwrap_or(&Theme::default().window_measures_default_width);
                        input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal), Some(hit.kind));
                        return Ok(());
                    }
                }
                let body = self.body_rect(theme);
                let anchor = hit.control_id.as_deref().and_then(panel_resize_anchor).map(|(anchor, _)| anchor).unwrap_or(PanelAnchor::TopLeft);
                self.panel_resize_origin_width = floating_panel_width(self.anchor_state(anchor).size, body, theme);
                input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal), Some(hit.kind));
                return Ok(());
            }
            if matches!(hit.kind, HitKind::DockSplit | HitKind::DockJoinCorner) {
                if !self.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                self.deactivate_active_window();
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(rest) = id.strip_prefix("dock.corner.r/") {
                        if let Some((row_part, col_part)) = rest.split_once("/c/") {
                            if let Some((row_path_str, row_index_str)) = row_part.rsplit_once('/') {
                                if let Some((col_path_str, col_index_str)) = col_part.rsplit_once('/') {
                                    let row_path = parse_path(row_path_str);
                                    let col_path = parse_path(col_path_str);
                                    self.split_resize_path = Some(row_path.clone());
                                    self.split_resize_index = row_index_str.parse().unwrap_or(0);
                                    self.split_resize_secondary_path = Some(col_path.clone());
                                    self.split_resize_secondary_index = col_index_str.parse().unwrap_or(0);
                                    self.split_resize_origin = self.dock.begin_split_drag(&row_path);
                                    self.split_resize_secondary_origin = self.dock.begin_split_drag(&col_path);
                                    let geometry = &self.presented_input_geometry;
                                    self.split_resize_axis_total = self.dock.split_axis_extent(&row_path, geometry.dock_canvas_bounds, geometry.dock_axis_separator).unwrap_or(geometry.dock_canvas_bounds.w);
                                    self.split_resize_secondary_axis_total = self.dock.split_axis_extent(&col_path, geometry.dock_canvas_bounds, geometry.dock_axis_separator).unwrap_or(geometry.dock_canvas_bounds.h);
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
                            self.split_resize_path = Some(path.clone());
                            self.split_resize_index = index;
                            self.split_resize_origin = self.dock.begin_split_drag(&path);
                            let geometry = &self.presented_input_geometry;
                            self.split_resize_axis_total = self.dock.split_axis_extent(&path, geometry.dock_canvas_bounds, geometry.dock_axis_separator).unwrap_or_else(|| match hit.drag_axis {
                                Some(DragAxis::Vertical) => geometry.dock_canvas_bounds.h,
                                _ => geometry.dock_canvas_bounds.w,
                            });
                            input.begin_drag(x, y, button, Some(id.to_string()), hit.drag_axis, Some(hit.kind));
                            return Ok(());
                        }
                    }
                }
            }
            if self.begin_window_template_drag_from_hit(pointer_id, &hit, x, y) {
                return Ok(());
            }
            if let Some((item_id, drag_data)) = hit.control_id.as_deref().and_then(tree_hit_item_id).zip(hit.drag_data.clone()) {
                if !self.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                self.tree_drag_origin = (x, y);
                self.pending_tree_drag = Some((item_id.to_string(), drag_data));
                return Ok(());
            }
            if button == 0 && hit.control_id.as_deref().is_some_and(|id| ShellPaletteKind::row_index(id).is_some() || ShellPaletteKind::from_close_id(id).is_some()) {
                if !self.claim_chrome_pointer(pointer_id) {
                    return Ok(());
                }
                self.pending_chrome_release = hit.control_id.clone();
                return Ok(());
            }
            if self.handle_shell_hit(&hit, input).await? {
                if let Some(kind) = hit.control_id.as_deref().and_then(ShellPaletteKind::from_input_id) {
                    self.set_palette_query(kind, self.palette_query(kind).to_string(), input);
                } else if hit.control_id.as_deref().is_some_and(|id| matches!(id, "ui.search.toggle" | "ui.find.toggle")) {
                    if let Some(kind) = match self.overlay_state {
                        OverlayState::Search => Some(ShellPaletteKind::Search),
                        OverlayState::Find => Some(ShellPaletteKind::Find),
                        _ => None,
                    } {
                        self.set_palette_query(kind, self.palette_query(kind).to_string(), input);
                    } else {
                        input.blur_input();
                        self.accessibility_focused_control_id = None;
                    }
                } else if ShellPaletteKind::row_index(hit.control_id.as_deref().unwrap_or_default()).is_some() {
                    input.blur_input();
                    self.accessibility_focused_control_id = None;
                }
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if let Some(window_id) = dock_tab_select_window_id(id) {
                    if !self.claim_chrome_pointer(pointer_id) {
                        return Ok(());
                    }
                    self.pending_chrome_release = hit.control_id.clone();
                    return Ok(());
                }
                if let Some(rest) = id.strip_prefix("dock.tab.").and_then(|rest| rest.strip_suffix(".drag")) {
                    if let Some((path_str_value, window_id)) = rest.split_once('.') {
                        let path = parse_path(path_str_value);
                        let tab_index = self.dock.tab_index(&path, window_id).unwrap_or(0);
                        let window_kind_id = self.dock.window_kind_id(window_id).unwrap_or(window_id).to_string();
                        let template_id = self.dock.window_template_id(window_id).map(str::to_string);
                        let ghost_label =
                            self.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == window_kind_id).map(|k| k.label.resolve(self.active_terminology(), self.active_locale()).to_string())).unwrap_or_else(|| window_id.to_string());
                        self.begin_pending_dock_drag(pointer_id, DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.to_string(), window_kind_id, template_id, source_path: path, tab_index, ghost_label }, x, y);
                        return Ok(());
                    }
                }
                if let Some(path_str_value) = id.strip_prefix("dock.stack.") {
                    let path = parse_path(path_str_value);
                    let windows = self.dock.stack_windows_at_path(&path).unwrap_or_default();
                    let active = windows.iter().find(|wid| self.active_window_id.as_deref() == Some(wid.as_str())).or_else(|| windows.first()).cloned().unwrap_or_default();
                    if !active.is_empty() {
                        let tab_index = self.dock.tab_index(&path, &active).unwrap_or(0);
                        let window_kind_id = self.dock.window_kind_id(&active).unwrap_or(&active).to_string();
                        let ghost_label =
                            self.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == window_kind_id).map(|k| k.label.resolve(self.active_terminology(), self.active_locale()).to_string())).unwrap_or_else(|| active.clone());
                        self.begin_pending_dock_drag(pointer_id, DockDragPayload { kind: DockDragKind::Stack, window_id: active, window_kind_id, template_id: None, source_path: path, tab_index, ghost_label }, x, y);
                        return Ok(());
                    }
                }
            }
            if hit.kind == HitKind::PanelTab {
                if let Some((tab_id, anchor)) = hit.control_id.as_deref().and_then(|id| Some((id.to_string(), self.panel_tab_anchor(id)?))) {
                    if !self.claim_chrome_pointer(pointer_id) {
                        return Ok(());
                    }
                    self.dock_tab_drag = Some((tab_id, anchor));
                    input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Both), Some(hit.kind));
                }
            }
            if hit.kind == HitKind::Slider {
                if let Some(id) = hit.control_id.clone() {
                    if !self.claim_chrome_pointer(pointer_id) {
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
                        self.dispatch_action(ActionDescriptor {
                            controller_id: self.session.as_ref().map(|s| s.app.controller_id.clone()).unwrap_or_default(),
                            action: "selectRows".into(),
                            args: crate::action_args_json!({ "surfaceId": surface_id, "ids": ids }),
                        })
                        .await?;
                        return Ok(());
                    }
                }
            }
            if let Some(action) = hit.event.clone() {
                self.dispatch_action(action).await?;
            } else if hit.kind == HitKind::Input {
                if let Some(id) = hit.control_id {
                    let seed = ShellPaletteKind::from_input_id(&id).map(|kind| self.palette_query(kind).to_string()).or_else(|| self.widget_maps.input_metas.get(&id).map(|meta| meta.value.clone())).unwrap_or_default();
                    input.focus_input_owned(id, seed);
                }
            }
        }
        self.drain_gesture_bound_work().await;
        self.owe_settle();
        Ok(())
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
        if self.chrome_pointer_owner.is_some_and(|owner| owner != pointer_id) {
            return;
        }
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        self.presence_pointer = Some((x, y));
        input.update_hover(x, y);
        if let Some((kind, index)) = input.hovered_id.as_deref().and_then(ShellPaletteKind::row_index) {
            match kind {
                ShellPaletteKind::Search => self.search_selected = index,
                ShellPaletteKind::Find => self.find_selected = index,
            }
        }
        self.route_retained_pointer_move(pointer_id, x, y, input);
        self.sync_context_menu_hover(input);
        self.update_tree_hover(input);
        if let Some((ref item_id, ref drag_data)) = self.pending_tree_drag {
            if down {
                let dx = x - self.tree_drag_origin.0;
                let dy = y - self.tree_drag_origin.1;
                if self.tree_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    self.tree_drag = Some(TreeDragState { source_id: item_id.clone(), drag_data: drag_data.clone(), x, y, drop_target_id: None, drop_position: TreeDropPosition::Inside });
                    self.pending_tree_drag = None;
                }
            }
        }
        let tree_drag_payload = self.tree_drag.as_mut().map(|drag| {
            drag.x = x;
            drag.y = y;
            drag.drag_data.clone()
        });
        if let Some(ref drag_data) = tree_drag_payload {
            let graph_surfaces = self.node_graph_states.iter().map(|(id, surface)| (id.as_str(), surface.bounds)).collect::<Vec<_>>();
            crate::engine_canvas::node_graph_sync_flow_widget_ghost(x, y, drag_data, &graph_surfaces);
            drop(graph_surfaces);
            self.sync_world3d_catalogue_drop_preview(x, y, drag_data);
        } else {
            self.sync_world3d_catalogue_drop_preview_from_retained_ui();
        }
        let retained_drag_data = tree_drag_payload
            .is_none()
            .then(|| crate::interpreter::active_retained_drag_sessions())
            .and_then(|sessions| sessions.into_iter().map(|(_, _, _, drag_data)| drag_data).find(|drag_data| drag_data.contains_key(crate::scenes::CANVAS_CATALOGUE_DRAG_MIME)));
        if let Some(drag_data) = tree_drag_payload.as_ref().or(retained_drag_data.as_ref()) {
            self.sync_canvas2d_catalogue_drag(x, y, drag_data, input);
        } else {
            crate::scenes::cancel_canvas_catalogue_drag(input);
        }
        if let Some(drag) = &mut self.tree_drag {
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
        if let Some((payload, origin)) = &self.pending_dock_drag {
            if down && !self.dock.mobile {
                let dx = x - origin.0;
                let dy = y - origin.1;
                if self.dock_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    let payload = payload.clone();
                    let origin = *origin;
                    self.pending_dock_drag = None;
                    self.dock_drag = Some(DockDragState { payload, x: origin.0, y: origin.1, drop_zone: None });
                }
            }
        }
        if let Some(drag) = &mut self.dock_drag {
            drag.x = x;
            drag.y = y;
            let geometry = &self.presented_input_geometry;
            drag.drop_zone = compute_dock_drop_zone(x, y, &geometry.dock_drop_tab_bars, &geometry.dock_drop_bodies, geometry.dock_canvas_bounds);
        }
        if input.drag.active && down {
            input.update_drag(x, y);
            if let Some(id) = input.drag.target_id.as_deref() {
                let dx = x - input.drag.start_x;
                let dy = y - input.drag.start_y;
                match id {
                    id if id.starts_with("shell.measures.resize.") => {
                        if let Some(window_id) = self.measures_resize_window_id.clone() {
                            let next = (self.measures_resize_origin_width - dx).clamp(theme.panel_min_width, theme.panel_max_width);
                            self.measures_width.insert(window_id, next);
                        }
                    }
                    id if id.starts_with("panel.resize.") => {
                        if let Some((anchor, inner)) = panel_resize_anchor(id) {
                            let body = self.body_rect(theme);
                            let factor = if anchor.horizontal() == "middle" { 2.0 } else { 1.0 } * if inner { -1.0 } else { 1.0 };
                            let next = (self.panel_resize_origin_width + dx * factor).clamp(theme.panel_min_width, floating_panel_max_width(body, theme));
                            self.anchor_state_mut(anchor).size = next;
                        }
                    }
                    dock_id if dock_id.starts_with("dock.corner.") => {
                        if let Some(path) = self.split_resize_path.clone() {
                            self.dock.apply_split_drag_with_origin(&path, self.split_resize_index, dx, self.split_resize_axis_total, &self.split_resize_origin);
                        }
                        if let Some(path) = self.split_resize_secondary_path.clone() {
                            self.dock.apply_split_drag_with_origin(&path, self.split_resize_secondary_index, dy, self.split_resize_secondary_axis_total, &self.split_resize_secondary_origin);
                        }
                    }
                    dock_id if dock_id.starts_with("dock.split.") => {
                        if let (Some(path), axis) = (&self.split_resize_path, input.drag.axis) {
                            let delta = match axis {
                                Some(DragAxis::Horizontal) => dx,
                                Some(DragAxis::Vertical) => dy,
                                _ => dx,
                            };
                            self.dock.apply_split_drag_with_origin(path, self.split_resize_index, delta, self.split_resize_axis_total, &self.split_resize_origin);
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
        let Some(mut drag) = self.dock_drag.take() else {
            return Ok(());
        };
        drag.x = x;
        drag.y = y;
        if drag.drop_zone.is_none() {
            let geometry = &self.presented_input_geometry;
            drag.drop_zone = compute_dock_drop_zone(x, y, &geometry.dock_drop_tab_bars, &geometry.dock_drop_bodies, geometry.dock_canvas_bounds);
        }
        if let Some(zone) = drag.drop_zone {
            let body_key = self.session.as_ref().and_then(|session| session.app.window_kinds.iter().find(|kind| kind.id == drag.payload.window_kind_id)).map(|kind| kind.body_key.clone());
            if body_key.is_some() && !self.window_topology_publication_has_credit() {
                return Err("window topology publication item credits exceeded".to_string());
            }
            let controller_id = self.shell_command_controller_id();
            if controller_id.is_some() && body_key.is_none() {
                return Err(format!("window kind '{}' has no retained body", drag.payload.window_kind_id));
            }
            let journal_token = if let Some(controller_id) = controller_id {
                let (command_id, label, detail) = if drag.payload.kind == DockDragKind::NewWindow {
                    ("shell.windowSplit", shell_chrome_string("command.splitWindow", self.locale_id == "de"), serde_json::json!({ "windowKindId": drag.payload.window_kind_id, "instanceId": drag.payload.window_id }))
                } else {
                    ("shell.windowMove", shell_chrome_string("command.moveWindow", self.locale_id == "de"), serde_json::json!({ "windowId": drag.payload.window_id }))
                };
                let note = Self::note_shell_command_action(&controller_id, command_id, label, Some(detail));
                Some(self.reserve_window_topology_action(note).map_err(|error| format!("window topology journal refused admission before dock mutation: {error:?}"))?)
            } else {
                None
            };
            if !self.dock.apply_drop(&drag.payload, &zone) {
                if let Some(token) = journal_token {
                    let _ = self.take_window_topology_action(token);
                }
                return Ok(());
            }
            if drag.payload.kind == DockDragKind::NewWindow {
                self.seed_new_window_projection(&drag.payload.window_id, drag.payload.template_id.as_deref());
            }
            self.active_window_id = Some(drag.payload.window_id.clone());
            self.dock.sync_active_window(&drag.payload.window_id);
            self.persist_dock_layout();
            if let Some(body_key) = body_key.as_deref() {
                self.commit_window_topology_publication(&drag.payload.window_id, body_key, journal_token);
            } else {
                self.owe_window_topology_refresh();
            }
        }
        let _ = input;
        Ok(())
    }

    //#region 🎯️RetainedPointerRouting
    /// 🎯️ The retained window body that minted `hit`, with the body rect the registry was minted
    /// against — `None` for every chrome target. See `retained_hit_windows`.
    pub(crate) fn retained_hit_window(&self, hit: &HitTarget<ActionDescriptor>) -> Option<(String, Rect)> {
        self.retained_hit_windows.get(hit.control_id.as_deref()?).cloned()
    }

    /// 🎬️ Admits only the exact published scene hit while its originating surface remains live.
    fn retained_scene_hit_is_live(&self, hit: &HitTarget<ActionDescriptor>, x: f32, y: f32) -> bool {
        use ui_wgpu::wgpu::SurfaceKind;
        let Some(control) = hit.control_id.as_deref() else { return false };
        let Some((rect, scene)) = self.retained_scene_hits.get(control) else { return false };
        let window = &scene.window_id;
        if *rect != hit.rect || !rect.contains(x, y) || !self.retained_hit_windows.get(control).is_some_and(|(owner, body)| owner == window && body.contains(x, y)) {
            return false;
        }
        let kind_matches = match scene.kind {
            SurfaceKind::World3d => hit.kind == HitKind::World3d,
            SurfaceKind::NodeGraph | SurfaceKind::Board2d | SurfaceKind::TiledMap => hit.kind == HitKind::ScrollRegion,
            _ => false,
        };
        kind_matches && crate::interpreter::scene_pointer_target_is_live(scene)
    }

    /// 🖼️ Resolves the exact live Canvas2d scene beneath a secondary press before its context menu opens.
    pub(crate) fn retained_canvas2d_hit_window(&self, hit: &HitTarget<ActionDescriptor>, x: f32, y: f32) -> Option<(String, Rect)> {
        let control = hit.control_id.as_deref()?;
        let scene_entry = self.retained_scene_hits.get(control);
        let owner_entry = self.retained_hit_windows.get(control);
        let live = owner_entry.and_then(|(window_id, body)| crate::interpreter::retained_scene_target_at(window_id, x - body.x, y - body.y));
        let (rect, scene) = scene_entry?;
        let (window_id, body) = owner_entry?;
        let live = live?;
        (*rect == hit.rect && rect.contains(x, y) && body.contains(x, y) && scene.kind == ui_wgpu::wgpu::SurfaceKind::Canvas2d && scene.window_id == *window_id && live.kind == ui_wgpu::wgpu::SurfaceKind::Canvas2d && live == *scene)
            .then(|| (window_id.clone(), *body))
    }

    /// 🧭️ A retained panel tab owns pointer routing but is not an application window instance.
    /// 🧭️ Whether a retained surface is shell chrome rather than a plugin window: a panel open in an
    /// anchor, or any shell-owned leaf wherever it is shown (the `/hub` workspace overlay is the
    /// `framework.hub` leaf with no anchor). Focusing or pressing chrome never makes it the active
    /// window — every plugin action resolves its window kind from `active_window_id`, and a chrome id
    /// there has none, which faulted the frame.
    pub(crate) fn retained_surface_is_panel(&self, surface_id: &str) -> bool {
        self.open_anchors().into_iter().any(|anchor| self.anchor_state(anchor).active_tab() == Some(surface_id)) || self.shell_owned_panel_leaves().iter().any(|leaf| leaf == surface_id)
    }

    /// 📜️ Registers a docked or mobile panel's whole content rect as its scroll region `<surface>.scroll`, under the node hits
    /// [`Self::register_retained_body_hits`] registers next, and routes it to the retained body — so a wheel over the gap between
    /// rows scrolls the panel as React's overflow container does, and a probe finds the panel's scroll region in `dumpChrome`.
    pub(crate) fn register_retained_panel_scroll_region(&mut self, surface: &str, content: Rect, input: &mut InputState<ActionDescriptor>) {
        let control_id = format!("{surface}{RETAINED_PANEL_SCROLL_SUFFIX}");
        input.register_hit(HitTarget { rect: content, event: None, control_id: Some(control_id.clone()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        self.retained_hit_windows_staging.insert(control_id, (surface.to_string(), content));
    }

    /// 🎯️ Publishes one retained body's own registry into the host's `InputState` and remembers who
    /// minted each id. Called by every body that paints a retained document — a dock window and a
    /// panel alike — right after its paint, so the entries land above the window region the chrome
    /// registered before it.
    pub(crate) fn register_retained_body_hits(&mut self, window_id: &str, body: Rect, input: &mut InputState<ActionDescriptor>) {
        if let Some((_, rect)) = self.retained_body_rects_staging.iter_mut().find(|(id, _)| id == window_id) {
            *rect = body;
        } else if self.retained_body_rects_staging.len() < 256 {
            self.retained_body_rects_staging.push((window_id.to_string(), body));
        }
        crate::interpreter::note_accessibility_visible_document(window_id);
        for (control_id, scene, rect) in crate::interpreter::register_clipped_retained_hit_targets(window_id, body, input) {
            if let Some(scene) = scene {
                self.retained_scene_hits_staging.insert(control_id.clone(), (rect, scene));
            } else {
                self.retained_scene_hits_staging.remove(&control_id);
            }
            self.retained_hit_windows_staging.insert(control_id, (window_id.to_string(), body));
        }
    }

    /// 🎬️ Lends the accepted original geometry and owner roster to the same caller's media producer.
    pub(crate) fn drive_presented_media_slots(&self,cursor:&mut crate::media_slots::original_collect::OriginalPresentedRoster,output:&mut Vec<crate::media_slots::PresentedMediaSlot>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let geometry=&self.presented_input_geometry;let source=crate::media_slots::original_collect::PresentedRosterSource{bodies:&geometry.retained_body_rects,surfaces:&geometry.retained_surfaces,owners:&geometry.media_owners,panels:&geometry.panel_rects,docks:&geometry.dock_window_plan,blocked:geometry.media_blocked};crate::interpreter::drive_presented_media_slots(&source,cursor,output,cx)
    }

    /// 🧭 Resolves one timer owner from this shell's GPU-accepted retained surface roster.
    pub(crate) fn retained_clock_surface(&self, index: usize) -> Option<(&str, ui_wgpu::wgpu::UiSurfaceToken)> {
        self.presented_input_geometry.retained_surfaces.get(index).map(|(id, surface)| (id.as_str(), *surface))
    }

    pub(crate) fn presented_retained_clock_deadline(&self) -> Option<f64> {
        crate::interpreter::presented_retained_clock_deadline(&self.presented_input_geometry.retained_surfaces)
    }

    /// 🪪️ Refreshes bounded app ownership independently from authored media resources.
    pub(crate) async fn refresh_app_document_identity(&mut self, program: &ProgramBridgeEntry, instance_id: u32) -> Result<(), String> {
        let key = (program.plugin_id.clone(), instance_id);
        let spawned = self.session.as_ref().and_then(|session| Self::panel_state_from_view(&session.view_state).ok().flatten()).map(|panel| panel.spawned_apps).unwrap_or_default();
        let active = self.session.as_ref().map(|session| (session.plugin_id.clone(), session.instance_id));
        let mounted = self.spawned_session.as_ref().map(|session| (session.plugin_id.clone(), session.instance_id));
        self.app_document_identities.retain(|owner| Some(owner) == active.as_ref() || Some(owner) == mounted.as_ref() || *owner == key || spawned.iter().any(|app| app.plugin_id == owner.0 && app.instance_id == owner.1));
        let request = self.app_document_identities.begin(key.clone()).ok_or_else(|| "app document identity registry request authority unavailable".to_string())?;
        let identity = match program.read_app_document_identity(instance_id).await {
            Ok(identity) => identity,
            Err(error) => {
                self.app_document_identities.remove(&key);
                return Err(self.note_refused_open(error));
            }
        };
        if !self.app_document_identities.accept(&key, request, identity.app_instance_id, identity.parent_document_id) {
            return Err("app document identity reply is stale or foreign".into());
        }
        Ok(())
    }

    fn media_owner_for_session(&self, session: &ActiveSession) -> Option<crate::media_slots::PresentedMediaOwner> {
        let parent_document_id = self.app_document_identities.document(&(session.plugin_id.clone(), session.instance_id))?.to_string();
        Some(crate::media_slots::PresentedMediaOwner { plugin_id: session.plugin_id.clone(), controller_id: session.app.controller_id.clone(), app_instance_id: session.instance_id, parent_document_id })
    }

    fn candidate_input_geometry(&self, theme: &Theme) -> PresentedInputGeometry {
        let body = self.body_rect(theme);
        let panel_rects = PanelAnchor::ALL.into_iter().filter(|anchor| self.anchor_open(*anchor)).map(|anchor| self.anchor_rect(anchor, body, theme)).collect();
        let active_owner = self.session.as_ref().and_then(|session| self.media_owner_for_session(session));
        let spawned_owner = self.spawned_session.as_ref().and_then(|session| self.media_owner_for_session(session));
        let spawned = self.session.as_ref().and_then(|session| Self::panel_state_from_view(&session.view_state).ok().flatten()).map(|panel| panel.spawned_apps).unwrap_or_default();
        let media_owners = self.retained_body_rects_staging.iter().map(|(id, _)| {
            let owner = if id == "spawned" || spawned.iter().any(|app| &app.id == id) { spawned_owner.clone() } else { active_owner.clone() };
            (id.clone(), owner)
        }).collect();
        let media_blocked = self.candidate_pointer_input_is_modal() || self.overlay_state != OverlayState::None || self.sync_card_kind.is_some() || self.dock_drag.is_some()
            || self.chrome_build.tooltip_hover.as_ref().is_some_and(|hover| self.chrome_build.tooltip_shows(false) && chrome_tooltip_ready(hover, chrome_now_ms()));
        PresentedInputGeometry {
            retained_body_rects: self.retained_body_rects_staging.clone(),
            media_owners,
            media_blocked,
            retained_surfaces: crate::interpreter::staged_retained_clock_surfaces(),
            dock_canvas_bounds: self.dock_canvas_bounds,
            dock_axis_separator: self.dock_axis_separator,
            dock_drop_tab_bars: self.dock_drop_tab_bars.clone(),
            dock_drop_bodies: self.dock_drop_bodies.clone(),
            dock_window_plan: self.dock_window_plan.clone(),
            window_content_rects: self.window_content_rects.clone(),
            window_silhouettes: self.window_silhouettes.clone(),
            modal: self.candidate_pointer_input_is_modal(),
            panel_rects,
        }
    }

    /// 🎞️ Seals the hit, owner, widget-binding and geometry candidates minted by one completed
    /// chrome walk. They remain non-interactive until the matching GPU packet is acknowledged.
    pub(crate) fn seal_presented_input_candidate(&mut self, theme: &Theme) -> Result<PresentedInputCandidateWitness, &'static str> {
        let epoch = self.next_presented_input_epoch.ok_or("presented input candidate generation exhausted")?;
        self.next_presented_input_epoch = epoch.checked_add(1);
        let witness = PresentedInputCandidateWitness(epoch);
        self.presented_input_geometry_staging = self.candidate_input_geometry(theme);
        self.presented_input_candidate = Some(witness);
        if !crate::interpreter::seal_presented_input_candidate(witness.0) {
            self.presented_input_candidate = None;
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

    pub(crate) fn presented_input_candidate_matches(&self, witness: PresentedInputCandidateWitness) -> bool {
        self.presented_input_candidate == Some(witness) && crate::interpreter::presented_input_candidate_matches(witness.0)
    }

    pub(crate) fn progress_presented_input_candidate(&self, input: &mut InputState<ActionDescriptor>, witness: PresentedInputCandidateWitness) -> crate::interpreter::PresentedInputCandidateProgress {
        if self.presented_input_candidate != Some(witness) {
            return crate::interpreter::PresentedInputCandidateProgress::Stale;
        }
        crate::interpreter::progress_presented_input_candidate(witness.0, input)
    }

    /// 🏁️ Promotes one GPU-accepted frame's complete input authority in one runtime lock. The chrome's
    /// accessibility publication is replaced (and its generation advanced) only when its projection
    /// changed, so the mirror's addresses stay valid across frames that present the same chrome.
    ///
    /// ♿️ The chrome's accessible names ride the SAME promotion as its hit registry (packet W15d)
    /// — a production path, not a diagnostics one, so an assistive technology reads the chrome
    /// whether or not `SEMIO_RUNTIME_DIAGNOSTICS` is armed.
    pub(crate) fn acknowledge_presented_input(&mut self, input: &mut InputState<ActionDescriptor>, witness: PresentedInputCandidateWitness) -> bool {
        if !self.presented_input_candidate_matches(witness) {
            return false;
        }
        if !crate::interpreter::acknowledge_presented_input(witness.0) {
            return false;
        }
        crate::scenes::acknowledge_host_temporal_candidates(witness.0);
        crate::scenes::acknowledge_table_stepper_accessibility_candidates(witness.0);
        crate::scenes::acknowledge_table_editable_text_accessibility_candidates(witness.0);
        crate::scenes::acknowledge_table_button_accessibility_candidates(witness.0);
        crate::scenes::acknowledge_vfs_accessibility_candidates(witness.0);
        crate::scenes::acknowledge_block_list_accessibility_candidates(witness.0);
        crate::scenes::acknowledge_event_feed_accessibility_candidates(witness.0);
        crate::scenes::acknowledge_world3d_accessibility_candidates(witness.0);
        crate::scenes::acknowledge_graph_timeline_accessibility_candidates(witness.0);
        std::mem::swap(&mut self.retained_hit_windows, &mut self.retained_hit_windows_staging);
        std::mem::swap(&mut self.retained_scene_hits, &mut self.retained_scene_hits_staging);
        std::mem::swap(&mut self.widget_maps, &mut self.widget_maps_staging);
        std::mem::swap(&mut self.presented_input_geometry, &mut self.presented_input_geometry_staging);
        input.publish_hits();
        self.presented_input_epoch = witness.0;
        self.presented_input_candidate = None;
        crate::interpreter::publish_accessibility_visible_documents();
        self.resolve_time_travel_focus(input);
        let chrome_accessibility = self.chrome_accessibility_nodes(input.hits());
        let focus_changed = self.presented_chrome_accessibility.iter_mut().zip(&chrome_accessibility).fold(false, |changed, (previous, current)| {
            let focus_changed = previous.focused != current.focused;
            previous.focused = current.focused;
            changed || focus_changed
        });
        let authority_changed = self.presented_chrome_accessibility_generation == 0 || chrome_accessibility != self.presented_chrome_accessibility;
        if authority_changed {
            self.presented_chrome_accessibility = chrome_accessibility;
            self.presented_chrome_accessibility_generation = witness.0;
        }
        if authority_changed || focus_changed {
            crate::interpreter::note_chrome_accessibility(self.presented_chrome_accessibility_generation, self.presented_chrome_accessibility.clone());
        }
        crate::interpreter::note_chrome_hit_registry(input.hits(), &self.retained_hit_windows);
        crate::interpreter::note_chrome_surfaces(&self.chrome_surface_census());
        #[cfg(not(target_arch = "wasm32"))]
        crate::native_accessibility::publish_native_accessibility(crate::native_accessibility::NativeAccessibilityPublication { title: crate::boot_window_title(), windows: crate::interpreter::native_accessibility_windows() });
        true
    }

    pub(crate) fn discard_presented_input_candidate(&mut self, witness: PresentedInputCandidateWitness) -> bool {
        if self.presented_input_candidate != Some(witness) {
            return false;
        }
        if !crate::interpreter::discard_presented_input_candidate(witness.0) {
            return false;
        }
        crate::scenes::discard_host_temporal_candidates(witness.0);
        crate::scenes::discard_table_stepper_accessibility_candidates(witness.0);
        crate::scenes::discard_table_editable_text_accessibility_candidates(witness.0);
        crate::scenes::discard_table_button_accessibility_candidates(witness.0);
        crate::scenes::discard_vfs_accessibility_candidates(witness.0);
        crate::scenes::discard_block_list_accessibility_candidates(witness.0);
        crate::scenes::discard_event_feed_accessibility_candidates(witness.0);
        crate::scenes::discard_world3d_accessibility_candidates(witness.0);
        crate::scenes::discard_graph_timeline_accessibility_candidates(witness.0);
        self.presented_input_candidate = None;
        true
    }

    #[cfg(test)]
    pub(crate) fn publish_retained_hit_registry(&mut self, input: &mut InputState<ActionDescriptor>) {
        self.publish_retained_input_for_test(input, &Theme::default());
    }

    #[cfg(test)]
    pub(crate) fn publish_retained_input_for_test(&mut self, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
        let witness = self.seal_presented_input_candidate(theme).expect("test input candidate witness");
        assert!(self.acknowledge_presented_input(input, witness));
    }

    /// 🪟️ Every surface this frame carries, at the LEVEL React's DOM states through `data-level` and
    /// under React's OWN element id — the census `dumpChrome` publishes beside the hit registry.
    ///
    /// ⚖️ Three levels, because React's shell has three: a window instance the dock planned, a DOCKED
    /// PANEL (one per open anchor — its active tab, named `panelTabElementId(tabId)`) and a dialog
    /// that owns the screen. This renderer models a panel AS a window instance, so a probe reading
    /// only window ids saw every panel a step opened as a new WINDOW where React reported a panel,
    /// and no panel step could match (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w9c-behaviour-parity-run-2.md` steps 3–8).
    pub(crate) fn chrome_surface_census(&self) -> Vec<(String, &'static str, String)> {
        let mut rows: Vec<(String, &'static str, String)> = self.dock_window_plan.iter().map(|(window_id, _)| (window_id.clone(), "window", window_id.clone())).collect();
        rows.extend(self.open_anchors().into_iter().filter_map(|anchor| self.anchor_state(anchor).active_tab().map(|tab| (tab.to_string(), "panel", panel_tab_introduction_element_id(tab)))));
        if self.chrome_build.tour_state.is_some() {
            rows.push((UI_INTRODUCTION_DIALOG_ELEMENT_ID.to_string(), "dialog", UI_INTRODUCTION_DIALOG_ELEMENT_ID.to_string()));
        }
        if let Some(request) = self.chrome_build.dialog_stack.last() {
            rows.push((request.accessibility_key(), "dialog", request.accessibility_key()));
        }
        if let Some(kind) = match self.overlay_state {
            OverlayState::Search => Some(ShellPaletteKind::Search),
            OverlayState::Find => Some(ShellPaletteKind::Find),
            _ => None,
        } {
            rows.push((kind.dialog_id().to_string(), "dialog", kind.dialog_id().to_string()));
        }
        rows
    }

    /// ⌨️🖱️ The live modifier set the retained router's POINTER events carry — the aggregate
    /// `InputState::modifiers` the ingress refreshes on every pointer and key event, projected onto
    /// `events`' own type. React reads the very same four flags straight off the DOM `PointerEvent`
    /// (`⚙️VirtualFileSystem/🟦️.tsx:520-521`), so a retained press that dropped them could never
    /// express shift-extend or ctrl-toggle on any list surface
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-scenes-residual.md` C1).
    fn retained_event_modifiers(input: &InputState<ActionDescriptor>) -> ui_wgpu::wgpu::EventModifiers {
        ui_wgpu::wgpu::EventModifiers { shift: input.modifiers.shift, ctrl: input.modifiers.ctrl, alt: input.modifiers.alt, meta: input.modifiers.meta }
    }

    /// 🖱️ DOM-standard `MouseEvent.button` code onto the retained event's own button identity.
    fn retained_pointer_button(button: i16) -> ui_wgpu::wgpu::PointerButton {
        match button {
            1 => ui_wgpu::wgpu::PointerButton::Middle,
            2 => ui_wgpu::wgpu::PointerButton::Secondary,
            _ => ui_wgpu::wgpu::PointerButton::Primary,
        }
    }

    /// 👆️ Routes a pointer move over a retained window body into that body's own
    /// `events::EventRouter` — the ONE owner of `NodeFlags::HOVERED`, the hover bubble chain and the
    /// per-surface `UiCommand::Scene` lane. Coordinates are window-local, because the retained tree's
    /// root sits at its own origin while the flat registry above is page-space.
    ///
    /// 🖱️ A live gesture keeps its window until release, the way a browser keeps a captured
    /// pointer with the element that captured it — otherwise a `Slider`/`Ring` drag froze the
    /// instant the pointer left the control's own rect and the registry answered something else.
    ///
    /// 👋️ The body that last owned the hover must be told about the move that LEAVES it. Routing
    /// only the moves that land ON a retained target left `NodeFlags::HOVERED` set forever on the
    /// last row the pointer touched — the router is edge-triggered on the move it receives, so a
    /// move it never receives is a hover it never clears (measured on 6118: hovering
    /// `Add Generation` then moving 300 px away left all three nodes of its bubble chain hovered,
    /// `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.2). The leave carries the pointer's REAL
    /// window-local coordinates, so `events::hit_test` resolves what is actually under it —
    /// nothing, or a plain container — and `update_hover` clears the chain by its own rule. No
    /// sentinel coordinate, and no second notion of "outside".
    fn route_retained_pointer_move(&mut self, pointer_id: ui_render::PointerId, x: f32, y: f32, input: &mut InputState<ActionDescriptor>) {
        let captured = crate::interpreter::retained_pointer_capture_window(pointer_id).and_then(|window_id| self.retained_window_body_rect(&window_id).map(|body| (window_id, body)));
        let current = captured.or_else(|| input.hit_at(x, y).cloned().and_then(|hit| self.retained_hit_window(&hit)));
        if let Some(previous) = self.retained_hover_window.clone() {
            if current.as_ref().map(|(window_id, _)| window_id.as_str()) != Some(previous.as_str()) {
                if let Some(body) = self.retained_window_body_rect(&previous) {
                    let _ = crate::interpreter::dispatch_ui_pointer_event(&previous, pointer_id, ui_wgpu::wgpu::UiEvent::PointerMove { x: x - body.x, y: y - body.y, modifiers: Self::retained_event_modifiers(input) }, input);
                }
            }
        }
        self.retained_hover_window = current.as_ref().map(|(window_id, _)| window_id.clone());
        let Some((window_id, body)) = current else { return };
        let _ = crate::interpreter::dispatch_ui_pointer_event(&window_id, pointer_id, ui_wgpu::wgpu::UiEvent::PointerMove { x: x - body.x, y: y - body.y, modifiers: Self::retained_event_modifiers(input) }, input);
    }

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
        Self::debug_log(&format!("[TRACE] wgpu-shell retained press window={window_id} kind={kind:?} down={down} action={:?}", action.as_ref().map(|descriptor| descriptor.action.clone())));
        if down && !self.retained_surface_is_panel(window_id) {
            let owner = window_pane_owner(window_id).unwrap_or(window_id).to_string();
            self.chrome_build.clear_sibling_pane_focus(window_id);
            self.dock.sync_active_window(&owner);
            self.active_window_id = Some(owner);
        }
        if retained_router_owns_pointer(kind) {
            let event = if down {
                ui_wgpu::wgpu::UiEvent::PointerDown { x: x - body.x, y: y - body.y, button: Self::retained_pointer_button(button), modifiers: Self::retained_event_modifiers(input) }
            } else {
                ui_wgpu::wgpu::UiEvent::PointerUp { x: x - body.x, y: y - body.y, button: Self::retained_pointer_button(button), modifiers: Self::retained_event_modifiers(input) }
            };
            let commands = crate::interpreter::dispatch_presented_ui_pointer_event(window_id, pointer_id, event, input);
            self.chrome_build.note_content_focus_commands(&commands);
        } else if !down {
            if let Some(mut action) = action {
                scope_action_to_window(&mut action, window_id);
                self.dispatch_action(action).await?;
            }
        }
        self.drain_gesture_bound_work().await;
        self.owe_settle();
        Ok(())
    }
    //#endregion 🎯️RetainedPointerRouting

    /// 🎬️ Whether a live engine surface is painted under this point — the wheel's twin of the press
    /// path's own `state.bounds.contains(x, y)` claim, and the same set of surfaces the four wheel
    /// phases go on to offer the notch to.
    pub fn scene_surface_contains(&self, x: f32, y: f32) -> bool {
        self.world3d_states.values().any(|state| state.bounds.contains(x, y))
            || self.node_graph_states.values().any(|surface| surface.bounds.contains(x, y))
            || self.tiled_map_states.values().any(|surface| surface.bounds.contains(x, y))
            || self.board2d_states.values().any(|surface| surface.bounds.contains(x, y))
    }

    /// 🎡️ A published scene hit owns its notch after modal and anchored-panel precedence.
    pub fn wheel_reaches_scene_surface(&self, x: f32, y: f32, input: &InputState<ActionDescriptor>, theme: &Theme) -> bool {
        !self.pointer_input_is_modal() && !self.pointer_is_over_open_panel(x, y, theme) && input.hit_at(x, y).is_some_and(|hit| self.retained_scene_hit_is_live(hit, x, y))
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
        Self::pointer_hit_owner(hit) == PointerHitOwner::Chrome
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

    /// 🚧️ Whether an overlay is MODAL for pointer input — React's menus, dropdowns, dialogs and the
    /// tour render into a layer above everything with their own backdrop, so while one is open no
    /// pointer of any phase reaches the canvas, wherever it lands. The tour's veil is already a
    /// full-screen hit target (`ui.introduction.veil`), so it answers through
    /// [`Self::pointer_hit_owner`] too; the other three publish hit rows for their own items only,
    /// and the point BESIDE an open menu is exactly the one that used to reach the world.
    fn candidate_pointer_input_is_modal(&self) -> bool {
        self.context_menu.is_some() || matches!(self.overlay_state, OverlayState::Search | OverlayState::Find) || self.chrome_build.dialog_open() || self.chrome_build.tour_is_open() || self.open_selects.values().any(|open| *open)
    }

    pub fn pointer_input_is_modal(&self) -> bool {
        self.presented_input_geometry.modal
    }

    /// 📑️ Whether this point is inside an OPEN anchored panel's painted box. A panel is an opaque
    /// layer over the dock's body — React's is a DOM element above the `<canvas>`, so the pointer
    /// never reaches the pane beneath it even where the panel's own content registers nothing (its
    /// padding, a gap between tree rows, the empty space under the last one). The boxes are this
    /// frame's own, from the same [`Self::anchor_rect`] the paint, the hit test and the published
    /// panel boxes read.
    pub fn pointer_is_over_open_panel(&self, x: f32, y: f32, _theme: &Theme) -> bool {
        self.presented_input_geometry.panel_rects.iter().any(|rect| rect.contains(x, y))
    }

    /// 🎯️ [`Self::pointer_hit_owner`] with this shell's live overlay LAYERS folded in — a modal
    /// overlay, an open panel's box, the retained body that minted the topmost hit, then the static
    /// hit classifier. The answer the renderer's ingress uses for a real point.
    pub fn pointer_owner_at(&self, x: f32, y: f32, input: &InputState<ActionDescriptor>, theme: &Theme) -> PointerHitOwner {
        if self.pointer_input_is_modal() || self.pointer_is_over_open_panel(x, y, theme) {
            return PointerHitOwner::Chrome;
        }
        let hit = input.hit_at(x, y);
        if let Some(hit) = hit.filter(|hit| self.retained_hit_window(hit).is_some()) {
            return if self.retained_scene_hit_is_live(hit, x, y) { PointerHitOwner::Surface } else { PointerHitOwner::Chrome };
        }
        PointerHitOwner::Chrome
    }

    /// 🧲️ Resolves one published scene identity after the shell's modal and panel layers.
    pub fn scene_pointer_target_at(&self, x: f32, y: f32, input: &InputState<ActionDescriptor>, theme: &Theme) -> Option<crate::element_contracts::ScenePointerTarget> {
        if self.pointer_input_is_modal() || self.pointer_is_over_open_panel(x, y, theme) {
            return None;
        }
        let hit = input.hit_at(x, y)?;
        if !self.retained_scene_hit_is_live(hit, x, y) {
            return None;
        }
        let (_, target) = self.retained_scene_hits.get(hit.control_id.as_deref()?)?;
        crate::interpreter::scene_pointer_target_is_live(target).then(|| target.clone())
    }

    /// 🖼️ Captured input may outlive its rectangle, but never its published scene owner.
    pub fn scene_pointer_target_is_published(&self, target: &crate::element_contracts::ScenePointerTarget) -> bool {
        self.retained_scene_hits.values().any(|(_, published)| published == target) && crate::interpreter::scene_pointer_target_is_live(target)
    }

    /// 🪟️ A press anywhere inside a window's own box ACTIVATES that window — React's
    /// `🪟️Window/🟦️.tsx`'s `onPointerDownCapture={… onActivate?.()}`, a capture-phase handler on the
    /// whole window element, so it fires for the window's chrome AND for the scene canvas filling its
    /// body, before any inner handler runs. It reaches `Mode`'s `activateWindow`
    /// (`🎨️Canvas/🟦️.tsx:1446`), whose `onActiveWindowChange` is the one funnel
    /// `🏛️ShellHost/🟦️.tsx:10389`'s `handleActiveWindowChange` notes `shell.windowActivate` from.
    ///
    /// 🩸️ This renderer had eleven sites writing `active_window_id` and not one of them was a press
    /// into a window BODY, so orbiting a pane never activated it: React's `orbit-drag` journals a
    /// `noteShellCommand` for the press into the perspective pane and the wgpu pane journalled none
    /// (`🗑️generated/w12c-parity-run-19/steps.json` step 19, ticket 26/09/17 packet W13c §1).
    ///
    /// The boxes are `dock_window_plan` — this frame's own solved window bodies, the same rects the
    /// dock paints and hit-tests. A modal layer owns every pointer and activates nothing (React's
    /// menus and dialogs are portaled OUTSIDE the window element, so no capture handler of theirs
    /// ever fires), and the empty-id row the dock plans after a cap close names no window.
    /// [`Self::arm_window_activation_note`] turns the change into the history note on the next drain.
    pub fn activate_window_under_pointer(&mut self, x: f32, y: f32, theme: &Theme) -> bool {
        if self.pointer_input_is_modal() || self.pointer_is_over_open_panel(x, y, theme) {
            return false;
        }
        let Some(window_id) = self.presented_input_geometry.dock_window_plan.iter().rev().find(|(id, rect)| !id.is_empty() && rect.contains(x, y)).map(|(id, _)| id.clone()) else {
            return false;
        };
        if self.active_window_id.as_deref() == Some(window_id.as_str()) {
            return false;
        }
        self.dock.sync_active_window(&window_id);
        self.active_window_id = Some(window_id);
        true
    }

    /// 🖱️ Whether a LIVE drag has captured the pointer for the shell's own chrome — the MOVE twin of
    /// [`Self::pointer_press_belongs_to_shell_chrome`], and React's `setPointerCapture` on a resize
    /// handle. The dock's structural drags travel across whatever the pointer sweeps over, so the
    /// engine surfaces under it must not also receive those moves.
    pub fn drag_captures_pointer(drag: &ui_wgpu::wgpu::DragState) -> bool {
        drag.active && matches!(drag.kind, Some(HitKind::DockSplit | HitKind::DockJoinCorner | HitKind::PanelResize | HitKind::PanelTab))
    }

    /// 📜️ The open context menu's own scroll offset lives on `ContextMenuState`, not the generic
    /// per-control `scroll_offsets` map — see `render_context_menu_level`'s clip/scroll handling.
    pub fn handle_pointer_wheel(&mut self, x: f32, y: f32, delta_x: f32, delta_y: f32, input: &mut InputState<ActionDescriptor>) -> bool {
        if self.widget_maps.scroll_select_popup_at(input, &mut self.scroll_offsets, x, y, delta_y) {
            return true;
        }
        let Some(hit) = input.hit_at(x, y).cloned() else {
            return false;
        };
        if self.retained_scene_hit_is_live(&hit, x, y) {
            return false;
        }
        if let Some((surface, body)) = self.retained_hit_window(&hit) {
            let modifiers = Self::retained_event_modifiers(input);
            let _ = crate::interpreter::dispatch_ui_event(&surface, ui_wgpu::wgpu::UiEvent::Scroll { x: x - body.x, y: y - body.y, delta_x, delta_y, modifiers }, input);
            return true;
        }
        if hit.kind != HitKind::ScrollRegion {
            return false;
        }
        let Some(id) = &hit.control_id else {
            return false;
        };
        if id == "shell.context.menu.scroll" {
            let Some(menu) = self.context_menu.as_mut() else {
                return false;
            };
            menu.scroll_offset = (menu.scroll_offset + delta_y).max(0.0);
            return true;
        }
        let entry = self.scroll_offsets.entry(id.clone()).or_insert(0.0);
        let delta = if id.starts_with("shell.panel.tabs.") && delta_x != 0.0 { delta_x } else { delta_y };
        *entry = (*entry + delta).max(0.0);
        true
    }

    pub async fn handle_world3d_input(&mut self, x: f32, y: f32, down: bool, button: i16, shift: bool, ctrl: bool, alt: bool, meta: bool, wheel_delta: f32, drag_dx: f32, drag_dy: f32) -> Result<(), String> {
        let modifiers = PointerModifiers { shift, ctrl, alt, meta };
        for state in self.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            let wheel = WorldInteractionIntent::wheel(x, y, wheel_delta, &modifiers);
            let button_intent = WorldInteractionIntent::pointer_button(x, y, down, button, &modifiers);
            let move_intent = WorldInteractionIntent::pointer_move(x, y, drag_dx, drag_dy, down, button, &modifiers);
            let mut drag = move_intent;
            drag.phase = WorldInteractionPhase::PointerDrag;
            let has_wheel = wheel_delta.abs() > 0.0;
            let has_drag = down && (drag_dx.abs() > 0.0 || drag_dy.abs() > 0.0);
            let admitted = match (has_wheel, has_drag) {
                (true, true) => enqueue_world3d_events(state, [wheel, drag, button_intent, move_intent]).is_ok(),
                (true, false) => enqueue_world3d_events(state, [wheel, button_intent, move_intent]).is_ok(),
                (false, true) => enqueue_world3d_events(state, [drag, button_intent, move_intent]).is_ok(),
                (false, false) => enqueue_world3d_events(state, [button_intent, move_intent]).is_ok(),
            };
            if !admitted {
                return Err("world3d fixed interaction credits exhausted".into());
            }
        }
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
            self.chrome_build.note_content_focus_commands(&commands);
            if focused && !self.retained_surface_is_panel(&target.window_id) {
                let owner = window_pane_owner(&target.window_id).unwrap_or(&target.window_id).to_string();
                self.chrome_build.clear_sibling_pane_focus(&target.window_id);
                self.dock.sync_active_window(&owner);
                self.active_window_id = Some(owner);
            }
            return Ok(true);
        }
        if target.window_generation != self.presented_chrome_accessibility_generation {
            return Ok(false);
        }
        if !self.presented_chrome_accessibility.iter().any(|node| node.node_id == target.node_id && node.key == target.node_key) {
            return Ok(false);
        }
        if self.handle_chrome_dialog_accessibility_event(&target.node_key, event) {
            return Ok(true);
        }
        if target.node_key == icon_export::CONTROL_ID {
            match event {
                ui_render::AccessibilityEvent::Activate => self.cancel_icon_export(),
                ui_render::AccessibilityEvent::Focus => {
                    input.blur_input();
                    self.accessibility_focused_control_id = Some(target.node_key.clone());
                }
                ui_render::AccessibilityEvent::Blur => self.accessibility_focused_control_id = None,
                ui_render::AccessibilityEvent::Value(_) => return Ok(false),
            }
            return Ok(true);
        }
        let palette_kind = match self.overlay_state {
            OverlayState::Search => Some(ShellPaletteKind::Search),
            OverlayState::Find => Some(ShellPaletteKind::Find),
            _ => None,
        };
        if let Some(kind) = palette_kind {
            match event {
                ui_render::AccessibilityEvent::Focus if target.node_key == kind.input_id() => {
                    let query = self.palette_query(kind).to_string();
                    self.set_palette_query(kind, query, input);
                }
                ui_render::AccessibilityEvent::Focus if target.node_key == kind.close_id() => {
                    input.blur_input();
                    self.accessibility_focused_control_id = Some(target.node_key.clone());
                }
                ui_render::AccessibilityEvent::Focus => {}
                ui_render::AccessibilityEvent::Blur if target.node_key == kind.input_id() => {
                    input.blur_input();
                    self.accessibility_focused_control_id = None;
                }
                ui_render::AccessibilityEvent::Blur => {}
                ui_render::AccessibilityEvent::Activate if target.node_key == kind.close_id() => {
                    self.close_command_palette(input);
                }
                ui_render::AccessibilityEvent::Activate if target.node_key == kind.input_id() => {
                    let query = self.palette_query(kind).to_string();
                    self.set_palette_query(kind, query, input);
                }
                ui_render::AccessibilityEvent::Activate => {
                    let index = match kind {
                        ShellPaletteKind::Search => self.filtered_search_items().iter().position(|item| item.id == target.node_key),
                        ShellPaletteKind::Find => self.filtered_find_items().iter().position(|item| item.id == target.node_key),
                    };
                    if let Some(index) = index {
                        match kind {
                            ShellPaletteKind::Search => self.activate_search_item(index).await?,
                            ShellPaletteKind::Find => self.activate_find_item(index).await?,
                        }
                        input.blur_input();
                        self.accessibility_focused_control_id = None;
                    }
                }
                ui_render::AccessibilityEvent::Value(value) if target.node_key == kind.input_id() => {
                    self.set_palette_query(kind, value.clone(), input);
                }
                ui_render::AccessibilityEvent::Value(_) => {}
            }
            return Ok(true);
        }
        if let Some(anchor) = self.panel_tab_anchor(&target.node_key) {
            match event {
                ui_render::AccessibilityEvent::Focus => {
                    input.blur_input();
                    self.accessibility_focused_control_id = Some(target.node_key.clone());
                    self.reveal_panel_tab_row(anchor, &target.node_key);
                }
                ui_render::AccessibilityEvent::Blur => {
                    if self.accessibility_focused_control_id.as_deref() == Some(target.node_key.as_str()) {
                        self.accessibility_focused_control_id = None;
                    }
                }
                ui_render::AccessibilityEvent::Activate => {
                    input.blur_input();
                    self.accessibility_focused_control_id = Some(target.node_key.clone());
                    self.reveal_panel_tab_row(anchor, &target.node_key);
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
                    self.accessibility_focused_control_id = Some(target.node_key.clone());
                }
                ui_render::AccessibilityEvent::Blur => {
                    if self.accessibility_focused_control_id.as_deref() == Some(target.node_key.as_str()) {
                        self.accessibility_focused_control_id = None;
                    }
                }
                ui_render::AccessibilityEvent::Activate => {
                    input.blur_input();
                    self.accessibility_focused_control_id = Some(target.node_key.clone());
                    if self.dock.activate_window(&window_id) {
                        self.active_window_id = Some(window_id);
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
        self.accessibility_focused_control_id = Some(target.node_key.clone());
        match event {
            ui_render::AccessibilityEvent::Focus => {
                if hit.kind == HitKind::Input {
                    let seed = ShellPaletteKind::from_input_id(&target.node_key).map(|kind| self.palette_query(kind).to_string()).or_else(|| self.widget_maps.input_metas.get(&target.node_key).map(|meta| meta.value.clone())).unwrap_or_default();
                    input.focus_input_owned(target.node_key.clone(), seed);
                }
            }
            ui_render::AccessibilityEvent::Blur => {
                if input.focused_id.as_deref() == Some(target.node_key.as_str()) {
                    self.commit_focused_input(input).await?;
                }
                if self.accessibility_focused_control_id.as_deref() == Some(target.node_key.as_str()) {
                    self.accessibility_focused_control_id = None;
                }
            }
            ui_render::AccessibilityEvent::Activate => {
                if self.handle_shell_hit(&hit, input).await? {
                    if matches!(target.node_key.as_str(), "ui.search.toggle" | "ui.find.toggle") {
                        if let Some(kind) = match self.overlay_state {
                            OverlayState::Search => Some(ShellPaletteKind::Search),
                            OverlayState::Find => Some(ShellPaletteKind::Find),
                            _ => None,
                        } {
                            self.set_palette_query(kind, self.palette_query(kind).to_string(), input);
                        }
                    } else if ShellPaletteKind::row_index(&target.node_key).is_some() {
                        input.blur_input();
                        self.accessibility_focused_control_id = None;
                    }
                } else {
                    if let Some(action) = hit.event {
                        self.dispatch_action(action).await?;
                    }
                }
            }
            ui_render::AccessibilityEvent::Value(value) => {
                if let Some(kind) = ShellPaletteKind::from_input_id(&target.node_key) {
                    self.set_palette_query(kind, value.clone(), input);
                    return Ok(true);
                }
                if target.node_key.ends_with(".input") {
                    let base = target.node_key.trim_end_matches(".input");
                    if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                        if let Ok(value) = value.parse::<f64>() {
                            self.dispatch_action(ActionDescriptor { controller_id: meta.on_absolute.controller_id, action: meta.on_absolute.action, args: crate::action_args_json!({ "value": value }) }).await?;
                        }
                        return Ok(true);
                    }
                }
                if let Some(meta) = self.widget_maps.input_metas.get(&target.node_key).cloned() {
                    if let Some(value) = meta.commit_value(value) {
                        let args = merge_committed_args(meta.on_change.args.as_ref(), value);
                        self.dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args }).await?;
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
        if let Some((window_id, chip)) = self.window_pane_chip_target(id) {
            self.toggle_window_pane_chip(&window_id, chip)?;
            return Ok(true);
        }
        if ui_wgpu::wgpu::arm_select_scroll(&mut self.scroll_offsets, id) {
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
                self.push_uri("/hub".to_string());
                self.apply_shell_uri("/hub").await?;
                return Ok(true);
            }
            "ui.nav.back" => {
                if self.uri_index > 0 {
                    self.uri_index -= 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.forward" => {
                if self.uri_index + 1 < self.uri_history.len() {
                    self.uri_index += 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.up" => {
                let uri = self.shell_uri();
                if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                    if !parent.is_empty() {
                        self.push_uri(parent);
                    }
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "playground.navbar.fixture" => {
                let open = matches!(&self.overlay_state, OverlayState::Dropdown(id) if id == "example");
                self.overlay_state = if open { OverlayState::None } else { OverlayState::Dropdown("example".to_string()) };
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.roles.") => {
                let requested = match id.trim_start_matches("playground.navbar.roles.") {
                    "editor" => semio_framework::manifest::AppRole::Editor,
                    "viewer" => semio_framework::manifest::AppRole::Viewer,
                    _ => return Ok(true),
                };
                self.switch_to_session_role(requested).await?;
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.modes.") => {
                let mode_id = id.trim_start_matches("playground.navbar.modes.").to_string();
                self.apply_navbar_mode(&mode_id).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.world3d.cancel::") => {
                let host_id = id.trim_start_matches("shell.world3d.cancel::").to_string();
                let action = world3d_cancel_affordance(self.world3d_status.get(&host_id).map(String::as_str));
                let owner = self.world3d_states.get(&host_id).map(|world| (world.controller_id.clone(), world.surface_id.clone()));
                if let (true, Some((controller_id, surface_id))) = (action.cancellable, owner) {
                    Self::debug_log(&format!("[TRACE] shell world3d cancel {}", serde_json::json!({ "host": host_id, "surface": surface_id, "action": action.cancel_action })));
                    let mut args = action.cancel_args;
                    args.insert("surfaceId".to_string(), Value::String(surface_id));
                    self.dispatch_action(ActionDescriptor { controller_id, action: action.cancel_action, args: Some(semio_framework::DslValue::from(Value::Object(args))) }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("framework.utility.collection.") => {
                let collection_id = id.trim_start_matches("framework.utility.collection.");
                let expanded = self.utility_collection_expanded.get(collection_id).copied().unwrap_or(false);
                self.utility_collection_expanded.insert(collection_id.to_string(), !expanded);
                return Ok(true);
            }
            id if id.starts_with("shell.example.") => {
                if !self.session_example_rows().iter().any(|row| row.control_id == id) {
                    return Ok(true);
                }
                let example_id = id.trim_start_matches("shell.example.");
                self.active_example_id = Some(example_id.to_string());
                self.overlay_state = OverlayState::None;
                if let Some(session) = &self.session {
                    self.dispatch_action(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "setActiveExample".into(), args: crate::action_args_json!({ "exampleId": example_id }) }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.engagement.toggle.") => {
                let window_id = id.trim_start_matches("shell.engagement.toggle.");
                let activated = self.engagement_activated.get(window_id).copied().unwrap_or(false);
                self.engagement_activated.insert(window_id.to_string(), !activated);
                self.engagement_expanded.insert(window_id.to_string(), !activated);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.focus.") => {
                let window_id = id.trim_start_matches("shell.measures.focus.");
                let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
                self.measures_expanded.insert(window_id.to_string(), !expanded);
                if !expanded {
                    self.engagement_activated.remove(window_id);
                    self.engagement_expanded.insert(window_id.to_string(), false);
                }
                return Ok(true);
            }
            "ui.search.toggle" => {
                self.search_open = !self.search_open;
                self.find_open = false;
                self.overlay_state = if self.search_open { OverlayState::Search } else { OverlayState::None };
                return Ok(true);
            }
            "ui.find.toggle" => {
                self.find_open = !self.find_open;
                self.search_open = false;
                self.overlay_state = if self.find_open { OverlayState::Find } else { OverlayState::None };
                return Ok(true);
            }
            "ui.panelToggle.display" => {
                self.toggle_anchor(PanelAnchor::BottomLeft);
                return Ok(true);
            }
            "ui.panelToggle.workbench" => {
                self.toggle_anchor(PanelAnchor::TopLeft);
                return Ok(true);
            }
            "ui.panelToggle.details" => {
                self.toggle_anchor(PanelAnchor::TopRight);
                self.note_panel_toggle_command(id, "details").await?;
                return Ok(true);
            }
            "ui.panelToggle.settings" => {
                self.toggle_anchor(PanelAnchor::BottomRight);
                self.note_panel_toggle_command(id, "settings").await?;
                return Ok(true);
            }
            "ui.panelToggle.chat" => {
                self.toggle_anchor_tab(PanelAnchor::TopRight, FRAMEWORK_CHAT_PANEL_ID);
                self.note_panel_toggle_command(id, "chat").await?;
                return Ok(true);
            }
            "ui.panelToggle.command" => {
                self.toggle_anchor_tab(PanelAnchor::BottomMiddle, FRAMEWORK_CATEGORY_COMMAND_ID);
                return Ok(true);
            }
            "ui.fullscreen.toggle" => {
                self.apply_os_command("os.toggleFullscreen", None).await?;
                return Ok(true);
            }
            "space.canvas.home" => {
                let controller_id = self.host_controller_id().unwrap_or_default();
                self.dispatch_action(ActionDescriptor { controller_id, action: "goHome".into(), args: None }).await?;
                return Ok(true);
            }
            "space.canvas.back" => {
                let has_focused_instance = if let Some(session) = self.session.as_ref() { Self::panel_state_from_view(&session.view_state)?.is_some_and(|panel| panel.active_spawned_id.is_some()) } else { false };
                if has_focused_instance {
                    let controller_id = self.host_controller_id().unwrap_or_default();
                    self.dispatch_action(ActionDescriptor { controller_id, action: "closeFocusedInstance".into(), args: None }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".focus") => {
                if let Some((path, window_id)) = Self::parse_dock_tab_action_id(id, "focus") {
                    self.dock.set_stack_active(&path, window_id);
                    self.active_window_id = Some(window_id.to_string());
                    let mut order = Vec::new();
                    Self::dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
                    if order.len() > 1 {
                        self.dock.toggle_maximize(&path);
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".close") => {
                if let Some((_, window_id)) = Self::parse_dock_tab_action_id(id, "close") {
                    let window_id = window_id.to_string();
                    self.close_dock_window_and_arm(&window_id, id);
                }
                return Ok(true);
            }
            id if id.starts_with("shell.layout.") => {
                let layout_id = id.trim_start_matches("shell.layout.");
                if let Some(layout) = self.named_layout_by_id(layout_id) {
                    self.layout_override = Some(layout);
                    self.sync_dock();
                    self.active_window_id = self.dock.active_window_id.clone();
                    self.note_control_command(id, Some(serde_json::json!({ "layoutId": layout_id }))).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.mode.") => {
                let mode_id = id.trim_start_matches("shell.mode.");
                self.dispatch_action(ActionDescriptor { controller_id: self.session.as_ref().map(|s| s.app.controller_id.clone()).unwrap_or_default(), action: "setMode".into(), args: crate::action_args_json!({ "modeId": mode_id }) }).await?;
                return Ok(true);
            }
            id if id.starts_with("framework.settings.appearance.") => {
                self.appearance_id = id.trim_start_matches("framework.settings.appearance.").to_string();
                return Ok(true);
            }
            UI_INTRODUCTION_VEIL_CONTROL_ID | UI_INTRODUCTION_SKIP_CONTROL_ID => {
                match self.introduction_seen_key() {
                    Some(seen_key) => self.chrome_build.dismiss_introduction(&seen_key),
                    None => self.chrome_build.skip_introduction(),
                }
                return Ok(true);
            }
            UI_INTRODUCTION_NEXT_CONTROL_ID => {
                if let Some(step) = self.chrome_tour_active_step() {
                    self.chrome_tour_advance_current_step(&step);
                }
                return Ok(true);
            }
            UI_INTRODUCTION_BACK_CONTROL_ID => {
                self.chrome_build.back_introduction();
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.mobile.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.mobile.").to_string();
                let tabs = self.mobile_panel_tabs();
                self.mobile_panel_path = mobile_panel_reconcile_path(&tabs, &[tab_id.clone()]);
                self.mobile_panel_visible = true;
                self.arm_tool_for_panel_tab(&self.mobile_panel_path.clone()).await?;
                return Ok(true);
            }
            "ui.mobilePanel.toggle" => {
                self.mobile_panel_visible = !self.mobile_panel_visible;
                return Ok(true);
            }
            id if self.panel_tab_anchor(id).is_some() => {
                if let Some(anchor) = self.panel_tab_anchor(id) {
                    let tab_id = id.to_string();
                    if hit.kind == HitKind::Toggle {
                        let was_visible = self.anchor_open(anchor);
                        let was_path = self.anchor_state(anchor).path.first().cloned();
                        self.toggle_anchor_tab(anchor, &tab_id);
                        self.journal_panel_selection(anchor, &tab_id, was_visible, was_path).await?;
                    } else {
                        self.select_panel_tab(anchor, &tab_id).await?;
                    }
                    self.arm_tool_for_panel_tab(&self.anchor_state(anchor).path.clone()).await?;
                }
                return Ok(true);
            }
            id if self.context_menu.as_ref().is_some_and(|menu| {
                let mut prefix = Vec::new();
                context_menu_path_for_item_id(&menu.items, id, &mut prefix).is_some()
            }) =>
            {
                let disabled = self.context_menu.as_ref().is_some_and(|menu| {
                    let mut prefix = Vec::new();
                    context_menu_path_for_item_id(&menu.items, id, &mut prefix).and_then(|path| context_menu_item_at_path(&menu.items, &path)).is_some_and(|item| item.disabled)
                });
                if disabled {
                    return Ok(true);
                }
                let submenu_path = self.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    (!context_menu_item_at_path(&menu.items, &path)?.children.is_empty()).then_some(path)
                });
                if let Some(path) = submenu_path {
                    if let Some(menu) = self.context_menu.as_mut() {
                        menu.active = path;
                        menu.submenu_collapsed_at = None;
                    }
                    return Ok(true);
                }
                let action = self.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    context_menu_item_at_path(&menu.items, &path)?.action.clone()
                });
                let origin = self.context_menu.as_ref().map(|menu| (menu.x, menu.y));
                self.context_menu = None;
                if let Some(action) = action {
                    let claimed = origin.is_some_and(|(x, y)| self.claim_text_editor_menu_action(&action, x, y));
                    if !claimed {
                        self.dispatch_action(action).await?;
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("section.chevron.") => {
                let section_id = id.trim_start_matches("section.chevron.");
                let key = format!("section.{section_id}");
                let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
                let next_open = !self.canonical_tree_open(section_id, !collapsed);
                self.collapsed_sections.insert(key, !next_open);
                self.set_canonical_tree_open(section_id, next_open);
                return Ok(true);
            }
            id if id.starts_with("tree.chevron.") => {
                let item_id = id.trim_start_matches("tree.chevron.");
                self.toggle_immediate_tree_item(item_id);
                return Ok(true);
            }
            id if id.contains(".vfs.chevron.") => {
                if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.chevron.") {
                    toggle_vfs_row_expanded(surface_id, row_id);
                    return Ok(true);
                }
            }
            id if self.widget_maps.select_metas.contains_key(id) => {
                let opening = !self.open_selects.get(id).copied().unwrap_or(false);
                self.close_open_selects();
                self.open_selects.insert(id.to_string(), opening);
                return Ok(true);
            }
            id if id.contains(".item.") => {
                if let Some((select_id, value)) = id.rsplit_once(".item.") {
                    if let Some(action) = self.widget_maps.select_metas.get(select_id).cloned() {
                        ui_wgpu::wgpu::clear_select_scroll(&mut self.scroll_offsets, select_id);
                        self.open_selects.insert(select_id.to_string(), false);
                        let args = merge_committed_args(action.args.as_ref(), semio_framework_value::DslValue::String(value.to_string()));
                        self.dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args }).await?;
                        return Ok(true);
                    }
                }
            }
            id if self.widget_maps.toggle_metas.contains_key(id) => {
                if let Some((pressed, action)) = self.widget_maps.toggle_metas.get(id).cloned() {
                    self.dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: crate::action_args_json!({ "pressed": !pressed }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".minus") => {
                let base = id.trim_end_matches(".minus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: crate::action_args_json!({ "delta": -meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".plus") => {
                let base = id.trim_end_matches(".plus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: crate::action_args_json!({ "delta": meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.starts_with("tree.label.") => {
                let item_id = id.trim_start_matches("tree.label.");
                if hit.drag_data.is_some() {
                    return Ok(true);
                }
                if tree_row_has_chevron(input, item_id) {
                    self.toggle_immediate_tree_item(item_id);
                }
                self.queue_tree_selection(item_id);
                return Ok(false);
            }
            id if tree_drag_handle_item_id(id).is_some() => return Ok(true),
            _ => {}
        }
        Ok(false)
    }
}
//#endregion ShellInput
