//! 🎮️ Interaction state: the state a frame checks out to every worker (shell, input, pointer capture, text streams) with its keyboard and pointer entry points, and the normalized-event dispatch that drives it; the frame-actions crate of the renderer partition, below the runtime mailbox (plan section 5.3, inversion I2).
use super::*;
use semio_framework_os_renderer_wgpu_host_plumbing::input_wire::{event_modifiers_to_pointer, key_action_from_dispatch, pointer_button_to_i16};
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_session::SessionCtx;
use semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx;
use ui_render::{DispatchEvent, ImeEvent, PointerInfo, TextEditTarget};

//#region 📮️RuntimeMailbox
//#region 🎮️AppInteractionState
const TEXT_STREAM_CAPACITY: usize = 64;

#[derive(Clone, Copy)]
pub struct AppTextStream {
    pub id: u64,
    pub token: ui_contract::TextIngressToken,
}

pub struct AppInteractionState {
    pub shell: ShellState,
    pub input: InputState<ActionDescriptor>,
    pub theme: Theme,
    pub theme_dark: bool,
    pub last_pointer_x: f32,
    pub last_pointer_y: f32,
    pub pointer_down: bool,
    pub pointer_button: i16,
    /// 🎯️ The layer that owns the pointer SEQUENCE in flight — set on the press from
    /// [`ShellState::pointer_owner_at`] and cleared on the release. React gets this from the DOM: the
    /// element a `pointerdown` resolved to receives the whole gesture, so a press that landed on
    /// chrome keeps every move and the release away from the canvas underneath it, whatever the
    /// pointer travels over in between.
    pub pointer_capture: PointerCapture,
    pub modifiers: PointerModifiers,
    pub space_pressed: bool,
    pub wheel_zoom_deadline_ms: f64,
    pub text_streams: [Option<AppTextStream>; TEXT_STREAM_CAPACITY],
    pub text_fault: Option<String>,
    pub frame_fault: Option<String>,
    pub text_cancel_pending: bool,
    pub last_sync_pump_ms: f64,
}

impl AppInteractionState {
    pub fn start_text_operation(&mut self, stream: u64, declared_bytes: usize) -> Result<(), String> {
        if self.text_streams.iter().any(|entry| entry.is_some_and(|entry| entry.id == stream)) {
            return Err("text stream was already started".to_string());
        }
        let slot = self.text_streams.iter().position(Option::is_none).ok_or_else(|| "text stream slots exhausted".to_string())?;
        let generation = self.input.text_buffer.generation();
        let token = self.input.text_buffer.begin(generation, declared_bytes, self.input.cursor_pos, self.input.cursor_pos).map_err(|fault| format!("text admission failed: {fault:?}"))?;
        self.text_streams[slot] = Some(AppTextStream { id: stream, token });
        Ok(())
    }

    pub fn push_text_operation(&mut self, stream: u64, text: String) -> Result<(), String> {
        let token = self.text_streams.iter().flatten().find(|entry| entry.id == stream).map(|entry| entry.token).ok_or_else(|| "text stream chunk arrived before start".to_string())?;
        self.input.text_buffer.push(token, text).map_err(|fault| format!("text chunk admission failed: {fault:?}"))
    }

    pub fn commit_text_operation(&mut self, stream: u64) -> Result<(), String> {
        let slot = self.text_streams.iter().position(|entry| entry.is_some_and(|entry| entry.id == stream)).ok_or_else(|| "text stream commit arrived before start".to_string())?;
        let token = self.text_streams[slot].expect("text stream slot").token;
        self.input.text_buffer.commit(token).map_err(|fault| format!("text stream commit failed: {fault:?}"))?;
        self.text_streams[slot] = None;
        Ok(())
    }

    pub fn abort_text_operation(&mut self, stream: u64) -> Result<(), String> {
        let slot = self.text_streams.iter().position(|entry| entry.is_some_and(|entry| entry.id == stream)).ok_or_else(|| "text stream abort arrived before start".to_string())?;
        let token = self.text_streams[slot].take().expect("text stream slot").token;
        self.input.text_buffer.abort(token).map_err(|fault| format!("text stream abort failed: {fault:?}"))
    }

    pub fn enqueue_text_operation(&mut self, text: String) -> Result<(), String> {
        let generation = self.input.text_buffer.generation();
        self.input.text_buffer.enqueue_owned(generation, text, self.input.cursor_pos, self.input.cursor_pos).map_err(|fault| format!("text admission failed: {fault:?}"))
    }

    pub fn cancel_text_operations(&mut self) {
        self.text_cancel_pending = true;
    }

    /// ⌨️ `mod+z` belongs to a FOCUSED text control, and to nothing else.
    ///
    /// 🩸️ It used to belong to the text buffer whenever that buffer held any undo state at all — and
    /// the buffer keeps its state after a control loses focus, so once the journey had opened the pane's
    /// Search box, every later `mod+z` was swallowed here and the shell's own undo chord (which
    /// dispatches `undo` to the app, exactly as React does) never ran (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` steps 27–28).
    pub fn undo_text_operation(&mut self) -> bool {
        if self.input.focused_id.is_none() {
            return false;
        }
        let Some(cursor) = self.input.text_buffer.undo() else { return false };
        self.input.cursor_pos = cursor.min(self.input.text_buffer.len());
        true
    }

    pub fn has_pending_text_work(&self) -> bool {
        self.input.text_buffer.runnable_work_pending() || self.input.retained_action_pending() || self.text_cancel_pending || semio_framework_os_renderer_wgpu_engine_text::text_editor::has_pending_text_editor_outbox()
    }

    pub fn drive_text_operation(&mut self) {
        if self.text_cancel_pending {
            let generation = self.input.text_buffer.generation();
            let _ = self.input.text_buffer.step(generation, 1, true);
            self.text_cancel_pending = self.input.text_buffer.reserved_bytes() != 0;
            return;
        }
        if let Err(fault) = self.input.drive_text_step() {
            self.text_fault = Some(format!("text edit step failed: {fault:?}"));
        }
        semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::sync_focused_table_editable_text_draft(&self.input);
    }

    /// 📐️ The content root is LOGICAL (CSS) pixels, exactly like every chrome constant
    /// (`chrome_px`, navbar/tab/footer heights, resize-handle hit widths) and like the React host's
    /// DOM. `dpr` never reaches layout — it only sizes the GPU surface, the projection divisor and
    /// the glyph atlas raster. See ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g.
    pub fn resize(&mut self, css_width: f32, css_height: f32, _dpr: f32) {
        self.shell.chrome.screen_w = css_width.max(1.0);
        self.shell.chrome.screen_h = css_height.max(1.0);
    }

    /// ⌨️ Offers one key to every board pane the pointer is inside, newest-first, and answers `true`
    /// as soon as one consumes it. Scoped to the pointer like React's `hoverActiveRef` gate, so a
    /// board in another pane never eats a chord meant for the focused window.
    fn board2d_key(&mut self, action: &KeyAction, modifiers: &PointerModifiers) -> bool {
        if !matches!(action, KeyAction::Escape | KeyAction::Tab) {
            return false;
        }
        let surfaces: Vec<(String, String)> = self.shell.surfaces.board2d_states.keys().filter_map(|surface_id| self.shell.surfaces.board2d_states.get(surface_id).map(|surface| (surface_id.to_string(), surface.controller_id.clone()))).collect();
        for (surface_id, controller_id) in surfaces {
            match semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_key_into(&surface_id, &controller_id, action, modifiers, &mut self.input) {
                Ok(true) => return true,
                Ok(false) => {}
                Err(fault) => {
                    self.input.record_action_fault(fault);
                    return true;
                }
            }
        }
        false
    }

    /// ✍️ A focused text editor owns the keyboard before the shell's chord table does — the same
    /// precedence React gets from DOM focus on its `<textarea>`. Without this the editor's
    /// `text_editor_apply_key_into` had no production caller at all and typing into a TextEditor
    /// surface only ever fired global accelerators.
    ///
    /// ⎋️🚚️ A live World3d relocate drag owns `Escape` before ANY chord table does — React binds it
    /// as a CAPTURE-phase `keydown` while `relocateMode` is on and calls `event.stopPropagation()`
    /// only when a drag was actually cancelled (`🌐️World3dHost/🟦️.tsx:6873-6881`). The drag is
    /// dropped, its ghost cleared, and nothing is dispatched.
    ///
    /// ⌨️ A board pane under the pointer owns `Escape` (cancel area-select) and `Tab`/`shift+Tab`
    /// (walk the brush slot's candidates) before the shell's chord table does — React gets the same
    /// precedence from a CAPTURE-phase listener that calls `preventDefault` (`🖥️Board2dHost/🟦️.tsx`
    /// :1292-1313, :1338-1339). Both chords had NO wgpu path at all before this.
    ///
    /// 🔌️ w2-input-wiring: spawns the ASYNC `handle_keyboard_async` (mirrors this fn's own
    /// `on_button`/`on_move` sibling callbacks above, and the `spawn_app_task` pattern this fn
    /// used to hand-roll just for search/find-Enter-activation) instead of calling the sync
    /// `handle_keyboard` directly. Before this fix `handle_keyboard_async` was entirely dead code
    /// (see `report-w3-shell-input-cutover.md`'s "MAJOR FINDING"): the P4 app-keybinding dispatch,
    /// P5 idle-Escape-deactivates-utility, and — worst — committing a focused `Input`'s typed text
    /// via Enter/Escape never fired. `handle_keyboard_async`'s own top already reimplements the
    /// exact search/find-Enter-activation this fn used to hand-duplicate around the sync call, so
    /// that duplication is gone, not just moved.
    pub async fn handle_key(&mut self, action: KeyAction, modifiers: PointerModifiers) {
        if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::apply_focused_vfs_control_key(&action, &mut self.input) {
            return;
        }
        if semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_table_editable_text_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_table_stepper_key(&action, &mut self.input) {
            return;
        }
        if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::apply_focused_ink_editor_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::apply_focused_ink_surface_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if let KeyAction::Space(pressed) = &action {
            if *pressed && (self.shell.input.context_menu.is_some() || SessionCtx::of(&mut self.shell).retained_select_owns_keyboard()) {
                if let Err(err) = PointerCtx::of(&mut self.shell).handle_keyboard_async(KeyAction::Space(true), &modifiers, &mut self.input).await {
                    log_debug(&format!("keyboard failed: {err}"));
                }
                return;
            }
            if semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_text_editor_key(&action, &modifiers, &mut self.input) {
                return;
            }
            self.space_pressed = *pressed;
            return;
        }
        if semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_node_graph_note_key(&action, &modifiers) {
            return;
        }
        if semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_text_editor_key(&action, &modifiers, &mut self.input) {
            return;
        }
        if action == KeyAction::Escape && self.shell.surfaces.world3d_states.values_mut().fold(false, |cancelled, state| infinite_world::world::world3d_cancel_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene) || cancelled) {
            return;
        }
        if self.board2d_key(&action, &modifiers) {
            return;
        }
        if let Err(err) = PointerCtx::of(&mut self.shell).handle_keyboard_async(action, &modifiers, &mut self.input).await {
            log_debug(&format!("keyboard failed: {err}"));
        }
    }

    /// 🛑️ Clears pointer ownership without manufacturing a successful release.
    pub fn handle_pointer_cancel(&mut self, pointer: ui_render::PointerInfo) {
        if let Some([x, y]) = self.pointer_capture.position(pointer.id) {
            self.input.pointer_x = x;
            self.input.pointer_y = y;
        }
        if pointer.kind == ui_render::PointerKind::Touch {
            if let Some(target) = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::captured_scene_pointer(pointer.id) {
                let outcome = match target.kind {
                    ui_wgpu::wgpu::SurfaceKind::TiledMap => self
                        .shell
                        .surfaces.tiled_map_states
                        .get(&target.host_id)
                        .filter(|surface| surface.window_id == target.window_id)
                        .map(|surface| semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(&target.host_id, &surface.controller_id, pointer.id, &mut self.input)),
                    ui_wgpu::wgpu::SurfaceKind::Board2d => self
                        .shell
                        .surfaces.board2d_states
                        .get(&target.host_id)
                        .filter(|surface| surface.window_id == target.window_id)
                        .map(|surface| semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_up_into(&target.host_id, &surface.controller_id, pointer.id, &mut self.input)),
                    _ => None,
                };
                if let Some(Err(fault)) = outcome {
                    self.input.record_action_fault(fault);
                }
            }
        }
        self.pointer_capture.release(pointer.id);
        self.pointer_down = self.pointer_capture.any_active();
        PointerCtx::of(&mut self.shell).handle_pointer_cancel_for(pointer.id, &mut self.input);
    }

    /// 🎡️ Applies one owned normalized wheel event before the next dispatch can change its target.
    pub fn handle_pointer_wheel(&mut self, x: f32, y: f32, delta_x: f32, delta_y: f32, modifiers: PointerModifiers) {
        self.modifiers = modifiers.clone();
        self.input.modifiers = modifiers.clone();
        let target = self.shell.input.scene_pointer_target_at(x, y, &self.input, &self.theme);
        let fault = self.shell.surfaces.world3d_states.take_fault().or_else(|| self.shell.surfaces.node_graph_states.take_fault()).or_else(|| self.shell.surfaces.tiled_map_states.take_fault()).or_else(|| self.shell.surfaces.board2d_states.take_fault());
        if let Some(fault) = fault {
            self.frame_fault = Some(fault.to_owned());
            return;
        }
        if let Some(target) = target {
            if let Err(fault) = self.apply_scene_wheel(&target, x, y, delta_y, &modifiers) {
                self.frame_fault = Some(fault.to_owned());
            }
        } else {
            self.shell.input.handle_pointer_wheel(x, y, delta_x, delta_y, &mut self.input);
        }
    }

    /// 🎡️ Applies one notch to the captured published scene, never to overlapping peers.
    pub fn apply_scene_wheel(&mut self, target: &semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget, x: f32, y: f32, delta: f32, modifiers: &PointerModifiers) -> Result<(), &'static str> {
        use ui_wgpu::wgpu::SurfaceKind;
        if !self.shell.input.scene_pointer_target_is_published(target) {
            return Ok(());
        }
        let surface_id = &target.surface_id;
        let host_id = &target.host_id;
        if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
            return Err("scene wheel identifier exceeded fixed credits");
        }
        let outcome = match target.kind {
            SurfaceKind::World3d => self.shell.surfaces.world3d_states.get_mut(host_id).map(|state| {
                if state.session.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::wheel(x, y, delta, modifiers)).map(|_| ()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits)
            }),
            SurfaceKind::NodeGraph => self.shell.surfaces.node_graph_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                let outcome = semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_wheel_into(host_id, &surface.controller_id, surface.bounds, x, y, delta, modifiers.ctrl, &mut self.input).map(|_| ());
                if outcome.is_ok() {
                    self.wheel_zoom_deadline_ms = app_now_ms() + 120.0;
                }
                outcome
            }),
            SurfaceKind::TiledMap => self.shell.surfaces.tiled_map_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                semio_framework_os_renderer_wgpu_engine_board::tiled_map::tiled_map_wheel_into(host_id, &surface.controller_id, surface.bounds, x, y, delta, modifiers.ctrl, &mut self.input).map(|_| ())
            }),
            SurfaceKind::Board2d => self.shell.surfaces.board2d_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                }
                semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_wheel_into(host_id, &surface.controller_id, surface.bounds, x, y, delta, &mut self.input).map(|_| ())
            }),
            _ => None,
        };
        if let Some(Err(fault)) = outcome {
            self.input.record_action_fault(fault);
            return Err("scene wheel action admission failed");
        }
        Ok(())
    }

    pub async fn handle_pointer_button(&mut self, pointer: ui_render::PointerInfo, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        use ui_wgpu::wgpu::SurfaceKind;
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        self.input.modifiers = modifiers.clone();
        self.input.pointer_x = x;
        self.input.pointer_y = y;
        self.input.pointer_down = down;
        if down && button == 0 {
            semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::blur_focused_ink_editor_at(x, y, &mut self.input);
        }
        let target = if down { self.shell.input.scene_pointer_target_at(x, y, &self.input, &self.theme) } else { None };
        semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::blur_focused_table_editable_text_for_pointer(target.as_ref(), down, button, &mut self.input);
        semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::blur_focused_text_editor_for_pointer(target.as_ref(), down, button);
        semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::blur_focused_node_graph_caret_for_pointer(target.as_ref(), down, button);
        semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::blur_focused_table_stepper_for_pointer(down, button);
        semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::blur_focused_vfs_control_for_pointer(down, button);
        let owner = if down {
            let Some(owner) = self.pointer_capture.press(pointer.id, if target.is_some() { PointerHitOwner::Surface } else { PointerHitOwner::Chrome }, x, y) else {
                self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                return;
            };
            owner
        } else {
            self.pointer_capture.release(pointer.id)
        };
        self.pointer_down = self.pointer_capture.any_active();
        if down {
            self.shell.input.activate_window_under_pointer(x, y, &self.theme, &mut self.shell.windows, &mut self.shell.dock);
        }
        if owner == PointerHitOwner::Chrome {
            if let Err(err) = ChromeViewCtx::of(&mut self.shell).handle_pointer_button_for(pointer.id, x, y, down, button, &mut self.input, &self.theme).await {
                log_debug(&format!("pointer failed: {err}"));
            }
            return;
        }
        let Some(target) = (if down { target } else { semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::release_scene_pointer(pointer.id) }).filter(|target| self.shell.input.scene_pointer_target_is_published(target)) else { return };
        if down && !semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::claim_scene_pointer_owner(target.clone(), pointer.id) {
            self.pointer_capture.release(pointer.id);
            self.pointer_down = self.pointer_capture.any_active();
            self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
            return;
        }
        if down && target.kind == SurfaceKind::World3d && button == 2 && !modifiers.shift && !modifiers.alt && !modifiers.meta {
            if let Err(err) = ChromeViewCtx::of(&mut self.shell).handle_pointer_button_for(pointer.id, x, y, down, button, &mut self.input, &self.theme).await {
                log_debug(&format!("pointer failed: {err}"));
            }
        }
        let host_id = &target.host_id;
        let outcome = match target.kind {
            SurfaceKind::World3d => {
                self.shell.surfaces.world3d_states.get_mut(host_id).map(|state| enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_button(x, y, down, button, &modifiers)).map(|_| ()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::ItemCredits))
            }
            SurfaceKind::NodeGraph => {
                let outcome = self.shell.surfaces.node_graph_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                    if down {
                        semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_pointer_down_into(host_id, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, self.space_pressed, &mut self.input).map(|_| ())
                    } else {
                        semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_pointer_up_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())
                    }
                });
                semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::refresh_node_graph_caret_after_pointer(&target);
                outcome
            }
            SurfaceKind::TiledMap => self.shell.surfaces.tiled_map_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                let touch_handled = pointer.kind == ui_render::PointerKind::Touch
                    && if down {
                        semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_down(host_id, surface.bounds, pointer.id, x, y)
                    } else {
                        match semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_up_into(host_id, &surface.controller_id, pointer.id, &mut self.input) {
                            Ok(handled) => handled,
                            Err(fault) => return Err(fault),
                        }
                    };
                if touch_handled {
                    Ok(())
                } else if down {
                    semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_down_into(&target, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), &surface.selection_method, &mut self.input).map(|_| ())
                } else {
                    semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_up_into(host_id, &surface.surface_id, &surface.controller_id, surface.bounds, x, y, &mut self.input).map(|_| ())
                }
            }),
            SurfaceKind::Board2d => self.shell.surfaces.board2d_states.get(host_id).filter(|surface| surface.window_id == target.window_id).map(|surface| {
                let touch_handled = pointer.kind == ui_render::PointerKind::Touch
                    && if down {
                        match semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_down_into(host_id, surface.bounds, pointer.id, x, y, &mut self.input) {
                            Ok(handled) => handled,
                            Err(fault) => return Err(fault),
                        }
                    } else {
                        match semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_up_into(host_id, &surface.controller_id, pointer.id, &mut self.input) {
                            Ok(handled) => handled,
                            Err(fault) => return Err(fault),
                        }
                    };
                if touch_handled {
                    Ok(())
                } else if down {
                    semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_pointer_down_into(host_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), &mut self.input)
                } else {
                    semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_pointer_up_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())
                }
            }),
            _ => None,
        };
        if let Some(Err(fault)) = outcome {
            self.input.record_action_fault(fault);
        }
    }

    pub async fn handle_pointer_move(&mut self, pointer: ui_render::PointerInfo, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        use ui_wgpu::wgpu::SurfaceKind;
        let (drag_dx, drag_dy) = self.pointer_capture.advance(pointer.id, x, y).unwrap_or((0.0, 0.0));
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = self.pointer_capture.any_active();
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        self.input.modifiers = modifiers.clone();
        self.input.pointer_x = x;
        self.input.pointer_y = y;
        self.input.pointer_down = down;
        self.shell.directory.presence_pointer = Some((x, y));
        let target = match self.pointer_capture.holder(pointer.id) {
            Some(PointerHitOwner::Surface) => semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::captured_scene_pointer(pointer.id),
            Some(PointerHitOwner::Chrome) => None,
            None => self.shell.input.scene_pointer_target_at(x, y, &self.input, &self.theme),
        }
        .filter(|target| self.shell.input.scene_pointer_target_is_published(target));
        if target.is_none() && self.pointer_capture.holder(pointer.id) != Some(PointerHitOwner::Surface) {
            PointerCtx::of(&mut self.shell).handle_pointer_move_for(pointer.id, x, y, down, &mut self.input, &self.theme);
        }
        let world_id = target.as_ref().filter(|target| target.kind == SurfaceKind::World3d).map(|target| target.host_id.as_str());
        for (host_id, state) in self.shell.surfaces.world3d_states.iter_mut() {
            let intent = if Some(host_id.as_str()) == world_id {
                WorldInteractionIntent::pointer_move(x, y, drag_dx, drag_dy, down, button, &modifiers)
            } else if world3d_hover_is_published(&state.selection) {
                WorldInteractionIntent::pointer_leave(x, y)
            } else {
                continue;
            };
            if enqueue_world3d_event(&mut state.selection, &mut state.authority, intent).is_err() {
                self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                return;
            }
        }
        for (host_id, surface) in &self.shell.surfaces.board2d_states {
            if target.as_ref().is_some_and(|target| target.kind == SurfaceKind::Board2d && target.host_id == *host_id) {
                continue;
            }
            if let Err(fault) = semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_pointer_leave_into(host_id, &surface.controller_id, modifiers.alt, &mut self.input) {
                self.input.record_action_fault(fault);
                return;
            }
        }
        for (host_id, surface) in &self.shell.surfaces.tiled_map_states {
            if target.as_ref().is_some_and(|target| target.kind == SurfaceKind::TiledMap && target.host_id == *host_id && target.window_id == surface.window_id) {
                continue;
            }
            if let Err(fault) = semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_leave_into(host_id, &surface.surface_id, &surface.controller_id, &mut self.input) {
                self.input.record_action_fault(fault);
                return;
            }
        }
        let Some(target) = target else { return };
        let host_id = &target.host_id;
        let outcome = match target.kind {
            SurfaceKind::NodeGraph => self
                .shell
                .surfaces.node_graph_states
                .get(host_id)
                .filter(|surface| surface.window_id == target.window_id)
                .map(|surface| semio_framework_os_renderer_wgpu_engine_graph::node_graph::node_graph_pointer_move_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())),
            SurfaceKind::TiledMap => self
                .shell
                .surfaces.tiled_map_states
                .get(host_id)
                .filter(|surface| surface.window_id == target.window_id)
                .map(|surface| {
                    if pointer.kind == ui_render::PointerKind::Touch && semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_touch_pointer_move(host_id, surface.bounds, pointer.id, x, y) {
                        Ok(())
                    } else {
                        semio_framework_os_renderer_wgpu_scene_feeds::tiled_map_pointer_move_into(&target, &surface.controller_id, surface.bounds, x, y, down, &mut self.input).map(|_| ())
                    }
                }),
            SurfaceKind::Board2d => self
                .shell
                .surfaces.board2d_states
                .get(host_id)
                .filter(|surface| surface.window_id == target.window_id)
                .map(|surface| {
                    if pointer.kind == ui_render::PointerKind::Touch && semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_touch_pointer_move(host_id, surface.bounds, pointer.id, x, y) {
                        Ok(())
                    } else {
                        semio_framework_os_renderer_wgpu_scene_text_vfs::board2d_pointer_move_into(host_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input).map(|_| ())
                    }
                }),
            _ => None,
        };
        if let Some(Err(fault)) = outcome {
            self.input.record_action_fault(fault);
        }
    }
}

/// 🔀️ One normalized `ui_host::DispatchEvent` → the existing `AppRuntime` pointer/keyboard entry
/// points, unchanged from before this packet (`handle_pointer_move`/`handle_pointer_button`/
/// `handle_key`). `PointerButton::{Primary,Secondary,Middle}` → `{0,2,1}` matches the DOM
/// `MouseEvent.button` convention `AppRuntime`'s own `i16` button parameter already assumed
/// (`dispatch_actions`/world3d/graph/map/board call sites all branch on `0`/`1`/`2` verbatim).
/// 📤️ P3a: the runtime mailbox invokes this for one retained cursor item per worker turn.
pub async fn dispatch_normalized_event(app: &mut AppInteractionState, event: &mut DispatchEvent) {
    semio_framework_os_renderer_wgpu_host_base::log_debug(&format!("[TRACE] os_host dispatch_normalized_event {event:?}"));
    match event {
        DispatchEvent::PointerMove { pointer, x, y, modifiers } => {
            let (down, button, modifiers) = (app.pointer_down, app.pointer_button, event_modifiers_to_pointer(*modifiers));
            app.handle_pointer_move(*pointer, *x, *y, down, button, modifiers).await;
        }
        DispatchEvent::PointerDown { pointer, x, y, button, modifiers } => {
            let modifiers = event_modifiers_to_pointer(*modifiers);
            app.handle_pointer_button(*pointer, *x, *y, true, pointer_button_to_i16(*button), modifiers).await;
        }
        DispatchEvent::PointerUp { pointer, x, y, button, modifiers } => {
            let modifiers = event_modifiers_to_pointer(*modifiers);
            app.handle_pointer_button(*pointer, *x, *y, false, pointer_button_to_i16(*button), modifiers).await;
        }
        DispatchEvent::PointerCancel { pointer } => app.handle_pointer_cancel(*pointer),
        DispatchEvent::Scroll { x, y, delta_x, delta_y, modifiers } => {
            app.handle_pointer_wheel(*x, *y, *delta_x, *delta_y, event_modifiers_to_pointer(*modifiers));
        }
        DispatchEvent::KeyDown { key, modifiers } => {
            app.modifiers = event_modifiers_to_pointer(*modifiers);
            if (modifiers.ctrl || modifiers.meta) && key.eq_ignore_ascii_case("z") && app.undo_text_operation() {
                return;
            }
            if key.as_str() == "Escape" {
                app.cancel_text_operations();
            }
            if let Some(action) = key_action_from_dispatch(key, true) {
                app.handle_key(action, event_modifiers_to_pointer(*modifiers)).await;
            }
        }
        DispatchEvent::KeyUp { key, modifiers } => {
            app.modifiers = event_modifiers_to_pointer(*modifiers);
            if let Some(action) = key_action_from_dispatch(key, false) {
                app.handle_key(action, event_modifiers_to_pointer(*modifiers)).await;
            }
        }
        DispatchEvent::Paste { text } => {
            if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::apply_focused_ink_paste(text, false, &mut app.input) {
                return;
            }
            if semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_text_editor_text(text, &mut app.input) {
                return;
            }
            if let Err(error) = app.enqueue_text_operation(std::mem::take(text)) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::TextInput { text } => {
            if let Err(error) = app.enqueue_text_operation(std::mem::take(text)) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::TextEditStart { stream, target, declared_bytes } if matches!(*target, TextEditTarget::Paste | TextEditTarget::PasteImageDataUrl) => {
            match semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::start_focused_ink_clipboard_stream(*stream, *target == TextEditTarget::PasteImageDataUrl, *declared_bytes) {
                Ok(true) => {}
                Ok(false) => {
                    let text_editor = if *target == TextEditTarget::Paste { semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::start_focused_text_editor_stream(*stream, *declared_bytes) } else { Ok(false) };
                    match text_editor {
                        Ok(true) => {}
                        Ok(false) => {
                            if let Err(error) = app.start_text_operation(*stream, *declared_bytes) {
                                app.text_fault = Some(error);
                            }
                        }
                        Err(error) => app.text_fault = Some(error.to_string()),
                    }
                }
                Err(error) => app.text_fault = Some(error.to_string()),
            }
        }
        DispatchEvent::TextEditStart { stream, target, declared_bytes } => match semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::start_focused_text_editor_stream(*stream, *declared_bytes) {
            Ok(true) => {}
            Ok(false) => {
                if let Err(error) = app.start_text_operation(*stream, *declared_bytes) {
                    app.text_fault = Some(error);
                }
            }
            Err(error) => app.text_fault = Some(error.to_string()),
        },
        DispatchEvent::TextEditChunk { stream, text } => match semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::push_focused_ink_clipboard_stream(*stream, text) {
            Ok(true) => {}
            Ok(false) => match semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::push_focused_text_editor_stream(*stream, text) {
                Ok(true) => {}
                Ok(false) => {
                    if let Err(error) = app.push_text_operation(*stream, std::mem::take(text)) {
                        app.text_fault = Some(error)
                    }
                }
                Err(error) => app.text_fault = Some(error.to_string()),
            },
            Err(error) => app.text_fault = Some(error.to_string()),
        },
        DispatchEvent::TextEditCommit { stream } => {
            if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::commit_focused_ink_clipboard_stream(*stream, &mut app.input) {
                return;
            }
            if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::commit_focused_text_editor_stream(*stream, &mut app.input) {
                return;
            }
            if let Err(error) = app.commit_text_operation(*stream) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::TextEditAbort { stream } => {
            if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::abort_focused_ink_clipboard_stream(*stream) {
                return;
            }
            if semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::abort_focused_text_editor_stream(*stream) {
                return;
            }
            if let Err(error) = app.abort_text_operation(*stream) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::Ime(ImeEvent::Commit { text }) => {
            if semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_text_editor_text(text, &mut app.input) {
                return;
            }
            if let Err(error) = app.enqueue_text_operation(std::mem::take(text)) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::Ime(_) => {}
        DispatchEvent::Accessibility { target, event } => {
            if let Err(error) = PointerCtx::of(&mut app.shell).handle_accessibility_event(target, event, &mut app.input).await {
                app.frame_fault = Some(error);
            }
        }
    }
}
//#endregion 🎮️AppInteractionState
//#endregion 📮️RuntimeMailbox
