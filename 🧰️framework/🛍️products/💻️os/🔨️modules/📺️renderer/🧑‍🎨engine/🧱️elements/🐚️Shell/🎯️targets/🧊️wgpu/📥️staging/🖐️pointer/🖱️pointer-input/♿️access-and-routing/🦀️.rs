//! ♿️ Accessibility and routing: the accessibility event handler, the chrome press ownership, the pointer hit owner, the drag capture and the retained pointer routing. See [`super`].

use super::*;

//#region ShellInput
impl<'ctx> PointerCtx<'ctx> {
    pub async fn handle_accessibility_event(&mut self, target: &ui_render::AccessibilityTarget, event: &ui_render::AccessibilityEvent, input: &mut InputState<ActionDescriptor>) -> Result<bool, String> {
        if target.window_id != semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID {
            let event = match event {
                ui_render::AccessibilityEvent::Focus => semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus,
                ui_render::AccessibilityEvent::Blur => semio_framework_ui_wgpu_events::AccessibilityUiEvent::Blur,
                ui_render::AccessibilityEvent::Activate => semio_framework_ui_wgpu_events::AccessibilityUiEvent::Activate,
                ui_render::AccessibilityEvent::Value(value) => semio_framework_ui_wgpu_events::AccessibilityUiEvent::Value(value.clone()),
            };
            let focused = matches!(event, semio_framework_ui_wgpu_events::AccessibilityUiEvent::Focus);
            let Some(commands) = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_accessibility_event(&target.window_id, target.window_generation, target.node_id, &target.node_key, event, input) else { return Ok(false) };
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
        if target.node_key == CONTROL_ID {
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
                            self.as_actions().dispatch_action(ActionDescriptor { controller_id: meta.on_absolute.controller_id, action: meta.on_absolute.action, args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": value }) }).await?;
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
pub fn drag_captures_pointer(drag: &semio_framework_ui_wgpu_input::DragState) -> bool {
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
    pub async fn route_retained_pointer_press(
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
                semio_framework_ui_wgpu_events::UiEvent::PointerDown { x: x - body.x, y: y - body.y, button: retained_pointer_button(button), modifiers: retained_event_modifiers(input) }
            } else {
                semio_framework_ui_wgpu_events::UiEvent::PointerUp { x: x - body.x, y: y - body.y, button: retained_pointer_button(button), modifiers: retained_event_modifiers(input) }
            };
            let commands = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_presented_ui_pointer_event(window_id, pointer_id, event, input);
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
