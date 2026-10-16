//! 🕹️ `shell-input` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl ShellInputState {
    pub fn update_tree_hover(&mut self, input: &InputState<ActionDescriptor>, settle: &mut SettleState) {
        let hovered = input.hovered_id.as_deref().and_then(tree_hit_item_id);
        if self.tree_hovered_id.as_deref() == hovered {
            return;
        }
        if let Some(prev) = self.tree_hovered_id.take() {
            if let Some(action) = self.widget_maps.tree_unhover_commands.get(&prev) {
                settle.deferred_actions.push(action.clone());
            }
        }
        if let Some(id) = hovered {
            if let Some(action) = self.widget_maps.tree_hover_commands.get(id) {
                settle.deferred_actions.push(action.clone());
            }
            self.tree_hovered_id = Some(id.to_string());
        }
    }

    pub fn queue_tree_selection(&mut self, item_id: &str, settle: &mut SettleState) {
        let Some(action) = self.widget_maps.tree_selection_change.clone() else {
            return;
        };
        settle.deferred_actions.push(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "ids": [item_id] }) });
    }

    /// 🌳️ Arms one tree-selection dispatch on the settle lane — a selection is a pointer gesture, and
    /// a gesture never converges its own chain (see [`Self::handle_pointer_button`]).
    pub fn dispatch_tree_selection(&mut self, item_id: &str, settle: &mut SettleState) {
        self.queue_tree_selection(item_id, settle);
        settle.owe_settle();
    }

    pub fn retained_window_body_rect(&self, window_id: &str) -> Option<Rect> {
        self.retained_hit_windows.values().find_map(|(owner, body)| (owner == window_id).then_some(*body))
    }

    /// 👻️ Catalogue ghosts while dragging from a retained document tree (panel catalogue) — not shell `tree_drag`.
    pub fn sync_world3d_catalogue_drop_preview_from_retained_ui(&mut self, surfaces: &mut SurfaceState) {
        let sessions = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::active_retained_drag_sessions();
        if sessions.is_empty() {
            if self.tree_drag.is_none() && self.pending_tree_drag.is_none() {
                surfaces.clear_world3d_catalogue_drop_previews();
            }
            return;
        }
        for (window_id, local_x, local_y, drag_data) in sessions {
            let Some(body) = self.retained_window_body_rect(&window_id) else { continue };
            surfaces.sync_world3d_catalogue_drop_preview(body.x + local_x, body.y + local_y, &drag_data);
        }
    }

    pub fn retained_scene_point(&self, x: f32, y: f32, input: &InputState<ActionDescriptor>) -> Option<(String, f32, f32)> {
        let hit = input.hit_at(x, y)?;
        if hit.kind != HitKind::ComponentScene {
            return None;
        }
        let (window_id, body) = self.retained_hit_window(hit)?;
        Some((window_id, x - body.x, y - body.y))
    }

    pub fn sync_canvas2d_catalogue_drag(&self, x: f32, y: f32, drag_data: &HashMap<String, String>, input: &mut InputState<ActionDescriptor>) {
        let Some((window_id, local_x, local_y)) = self.retained_scene_point(x, y, input) else {
            semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_catalogue_drag(input);
            return;
        };
        semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::canvas_catalogue_drag_over(&window_id, local_x, local_y, drag_data, input);
    }

    pub fn finish_canvas2d_catalogue_drop(&self, x: f32, y: f32, drag_data: &HashMap<String, String>, input: &mut InputState<ActionDescriptor>) -> bool {
        let Some((window_id, local_x, local_y)) = self.retained_scene_point(x, y, input) else {
            semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_catalogue_drag(input);
            return false;
        };
        semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::canvas_catalogue_drop(&window_id, local_x, local_y, drag_data, input)
    }

    /// 🔽️ Closes every open `Select` popup and drops the two `scroll_offsets` slots each one owned —
    /// a reopened popup starts at its first page, like a freshly mounted `SelectContent`, and neither
    /// slot outlives the popup that minted it (ticket 26/09/17 packet W15a). The ONE close path, so
    /// the outside-press, item-commit, Escape and toggle routes cannot disagree about the state they
    /// leave behind.
    pub fn close_open_selects(&mut self) {
        for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
            ui_wgpu::wgpu::clear_select_scroll(&mut self.scroll_offsets, &key);
            self.open_selects.insert(key, false);
        }
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
    pub fn resolve_context_menu_surface(&self, x: f32, y: f32, node_id: Option<&str>, edge_id: Option<&str>, surfaces: &SurfaceState) -> ContextMenuSurfaceResolution {
        let mut fallback_hits = Vec::new();
        if let Some(node_id) = node_id {
            fallback_hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "node".into(), id: node_id.into(), label: None });
        }
        if let Some(edge_id) = edge_id {
            fallback_hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "edge".into(), id: edge_id.into(), label: None });
        }
        if let Some((window_id, body)) = self.retained_context_menu_window(x, y) {
            if let Some(resolved) = semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::surface_context_menu_target(&window_id, x - body.x, y - body.y) {
                let kind = semio_framework_os_renderer_wgpu_scene_render::context_menu_surface_kind_id(resolved.kind).to_string();
                let semio_framework_os_renderer_wgpu_scene_render::SceneContextMenuTarget { mut hits, mut selection, text } = resolved.target;
                if resolved.kind == ui_wgpu::wgpu::SurfaceKind::World3d {
                    if let Some(state) = surfaces.world3d_state_for_window(&window_id) {
                        let (world_hits, world_selection) = infinite_world::world::world3d_context_menu_surface(&state.selection);
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
    pub fn claim_text_editor_menu_action(&self, action: &ActionDescriptor, x: f32, y: f32) -> bool {
        let Some((window_id, body)) = self.retained_context_menu_window(x, y) else {
            return false;
        };
        semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::text_editor_claim_menu_action(&window_id, x - body.x, y - body.y, &action.action)
    }

    /// 📋️ Alt + secondary press on a text editor opens its completions dropdown instead of a menu.
    pub fn claim_text_editor_alt_completions(&self, x: f32, y: f32) -> bool {
        let Some((window_id, body)) = self.retained_context_menu_window(x, y) else {
            return false;
        };
        semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::text_editor_claim_alt_completions(&window_id, x - body.x, y - body.y)
    }

    /// 🪟️ The retained body a right-click landed in, as `(window_id, body_rect)` — the hit registry's
    /// own owner map first (a press on a registered row inside the body), then the dock body the point
    /// falls in, so a click on a scene that registers no target of its own still resolves its window.
    pub fn retained_context_menu_window(&self, x: f32, y: f32) -> Option<(String, Rect)> {
        if let Some(window_id) = self.context_window_instance_id(x, y) {
            if let Some(body) = self.retained_window_body_rect(window_id) {
                return Some((window_id.to_string(), body));
            }
        }
        self.retained_hit_windows.values().find(|(_, body)| body.contains(x, y)).cloned()
    }

    pub fn sync_context_menu_hover(&mut self, input: &InputState<ActionDescriptor>) {
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
    pub fn dismiss_dismissable_layers(&mut self) -> bool {
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

    /// ⌨️ The chrome control the keyboard ring focused, addressed the way an accessibility event addresses it —
    /// `None` when focus is on a window, on a text field, or on a control the presented chrome no longer carries.
    pub fn keyboard_focused_chrome_target(&self, input: &InputState<ActionDescriptor>, chrome: &ChromeState) -> Option<ui_render::AccessibilityTarget> {
        let key = self.accessibility_focused_control_id.as_deref().filter(|_| input.focused_id.is_none())?;
        let node = chrome.presented_chrome_accessibility.iter().find(|node| node.key == key && node.focusable && !node.disabled)?;
        Some(ui_render::AccessibilityTarget { window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: chrome.presented_chrome_accessibility_generation, node_id: node.node_id, node_key: node.key.clone() })
    }

    /// ⌨️🖼️ Where the keyboard focus ring goes this frame: around the focused chrome control's own hit rect.
    pub fn keyboard_focus_ring_rect(&self, input: &InputState<ActionDescriptor>) -> Option<Rect> {
        let key = self.accessibility_focused_control_id.as_deref()?;
        input.staged_hits().iter().rev().find(|hit| hit.control_id.as_deref() == Some(key)).map(|hit| hit.rect)
    }

    //#region ⌨️ShellShortcutDispatch
    /// ⌨️ Writes the armed Keybindings row's override from one key event — the wgpu twin of React's
    /// `chordFromKeyboardEvent`: the fixed `ctrl, meta, alt, shift` modifier order then the key, with
    /// Escape cancelling the capture and a bare modifier press ignored. `true` when the event was
    /// consumed by the capture (which includes the Escape cancel).
    pub fn capture_keybinding_chord(&mut self, action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, chrome: &mut ChromeState) -> bool {
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
        chrome.chrome_build.preferences.keybinding_overrides.insert(control_id.clone(), chord.clone());
        self.keybinding_capture_control_id = None;
        debug_log(&format!("[TRACE] wgpu-shell keybinding override {control_id} -> {chord}"));
        true
    }
    //#endregion ⌨️ShellShortcutDispatch
}

//#endregion ShellInput

