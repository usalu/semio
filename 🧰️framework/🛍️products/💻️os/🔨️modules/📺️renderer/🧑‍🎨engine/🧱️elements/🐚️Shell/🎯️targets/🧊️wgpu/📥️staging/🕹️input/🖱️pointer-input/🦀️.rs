//! 🕹️ `shell-input` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
pub fn tree_drag_handle_item_id(control_id: &str) -> Option<&str> {
    control_id.strip_prefix("tree.drag.sort.").or_else(|| control_id.strip_prefix("tree.drag.transfer."))
}

pub fn tree_hit_item_id(control_id: &str) -> Option<&str> {
    control_id.strip_prefix("tree.label.").or_else(|| tree_drag_handle_item_id(control_id))
}

impl ShellInputState {
    //#region 🎯️RetainedPointerRouting
    /// 🎯️ The retained window body that minted `hit`, with the body rect the registry was minted
    /// against — `None` for every chrome target. See `retained_hit_windows`.
    pub fn retained_hit_window(&self, hit: &HitTarget<ActionDescriptor>) -> Option<(String, Rect)> {
        self.retained_hit_windows.get(hit.control_id.as_deref()?).cloned()
    }

    /// 🎬️ Admits only the exact published scene hit while its originating surface remains live.
    pub fn retained_scene_hit_is_live(&self, hit: &HitTarget<ActionDescriptor>, x: f32, y: f32) -> bool {
        use semio_framework_ui_declaration::ui::SurfaceKind;
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
        kind_matches && semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(scene)
    }

    /// 🖼️ Resolves the exact live Canvas2d scene beneath a secondary press before its context menu opens.
    pub fn retained_canvas2d_hit_window(&self, hit: &HitTarget<ActionDescriptor>, x: f32, y: f32) -> Option<(String, Rect)> {
        let control = hit.control_id.as_deref()?;
        let scene_entry = self.retained_scene_hits.get(control);
        let owner_entry = self.retained_hit_windows.get(control);
        let live = owner_entry.and_then(|(window_id, body)| semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target_at(window_id, x - body.x, y - body.y));
        let (rect, scene) = scene_entry?;
        let (window_id, body) = owner_entry?;
        let live = live?;
        (*rect == hit.rect && rect.contains(x, y) && body.contains(x, y) && scene.kind == semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d && scene.window_id == *window_id && live.kind == semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d && live == *scene)
            .then(|| (window_id.clone(), *body))
    }

    /// 📜️ Registers a docked or mobile panel's whole content rect as its scroll region `<surface>.scroll`, under the node hits
    /// [`Self::register_retained_body_hits`] registers next, and routes it to the retained body — so a wheel over the gap between
    /// rows scrolls the panel as React's overflow container does, and a probe finds the panel's scroll region in `dumpChrome`.
    pub fn register_retained_panel_scroll_region(&mut self, surface: &str, content: Rect, input: &mut InputState<ActionDescriptor>) {
        let control_id = format!("{surface}{RETAINED_PANEL_SCROLL_SUFFIX}");
        input.register_hit(HitTarget { rect: content, event: None, control_id: Some(control_id.clone()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        self.retained_hit_windows_staging.insert(control_id, (surface.to_string(), content));
    }

    /// 🎯️ Publishes one retained body's own registry into the host's `InputState` and remembers who
    /// minted each id. Called by every body that paints a retained document — a dock window and a
    /// panel alike — right after its paint, so the entries land above the window region the chrome
    /// registered before it.
    pub fn register_retained_body_hits(&mut self, window_id: &str, body: Rect, input: &mut InputState<ActionDescriptor>) {
        if let Some((_, rect)) = self.retained_body_rects_staging.iter_mut().find(|(id, _)| id == window_id) {
            *rect = body;
        } else if self.retained_body_rects_staging.len() < 256 {
            self.retained_body_rects_staging.push((window_id.to_string(), body));
        }
        semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_accessibility_visible_document(window_id);
        for (control_id, scene, rect) in semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::register_clipped_retained_hit_targets(window_id, body, input) {
            if let Some(scene) = scene {
                self.retained_scene_hits_staging.insert(control_id.clone(), (rect, scene));
            } else {
                self.retained_scene_hits_staging.remove(&control_id);
            }
            self.retained_hit_windows_staging.insert(control_id, (window_id.to_string(), body));
        }
    }

    /// 🎬️ Lends the accepted original geometry and owner roster to the same caller's media producer.
    pub fn drive_presented_media_slots(&self,cursor:&mut semio_framework_os_renderer_wgpu_media_slots::original_collect::OriginalPresentedRoster,output:&mut Vec<semio_framework_os_renderer_wgpu_media_slots::PresentedMediaSlot>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let geometry=&self.presented_input_geometry;let source=semio_framework_os_renderer_wgpu_media_slots::original_collect::PresentedRosterSource{bodies:&geometry.retained_body_rects,surfaces:&geometry.retained_surfaces,owners:&geometry.media_owners,panels:&geometry.panel_rects,docks:&geometry.dock_window_plan,blocked:geometry.media_blocked};semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::drive_presented_media_slots(&source,cursor,output,cx)
    }

    /// 🧭 Resolves one timer owner from this shell's GPU-accepted retained surface roster.
    pub fn retained_clock_surface(&self, index: usize) -> Option<(&str, semio_framework_ui_wgpu_engine::UiSurfaceToken)> {
        self.presented_input_geometry.retained_surfaces.get(index).map(|(id, surface)| (id.as_str(), *surface))
    }

    pub fn presented_retained_clock_deadline(&self) -> Option<f64> {
        semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::presented_retained_clock_deadline(&self.presented_input_geometry.retained_surfaces)
    }

    pub fn presented_input_candidate_matches(&self, witness: PresentedInputCandidateWitness) -> bool {
        self.presented_input_candidate == Some(witness) && semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::presented_input_candidate_matches(witness.0)
    }

    pub fn progress_presented_input_candidate(&self, input: &mut InputState<ActionDescriptor>, witness: PresentedInputCandidateWitness) -> semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress {
        if self.presented_input_candidate != Some(witness) {
            return semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress::Stale;
        }
        semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::progress_presented_input_candidate(witness.0, input)
    }

    pub fn discard_presented_input_candidate(&mut self, witness: PresentedInputCandidateWitness) -> bool {
        if self.presented_input_candidate != Some(witness) {
            return false;
        }
        if !semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::discard_presented_input_candidate(witness.0) {
            return false;
        }
        semio_framework_os_renderer_wgpu_scene_feeds::discard_host_temporal_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_table_stepper_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_table_editable_text_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_table_button_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_vfs_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_block_list_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_event_feed_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_world3d_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::discard_graph_timeline_accessibility_candidates(witness.0);
        self.presented_input_candidate = None;
        true
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
    pub fn chrome_surface_census(&self, chrome: &ChromeState, dock: &ShellDockState) -> Vec<(String, &'static str, String)> {
        let mut rows: Vec<(String, &'static str, String)> = dock.dock_window_plan.iter().map(|(window_id, _)| (window_id.clone(), "window", window_id.clone())).collect();
        rows.extend(dock.open_anchors().into_iter().filter_map(|anchor| dock.anchor_state(anchor).active_tab().map(|tab| (tab.to_string(), "panel", panel_tab_introduction_element_id(tab)))));
        if chrome.chrome_build.tour_state.is_some() {
            rows.push((UI_INTRODUCTION_DIALOG_ELEMENT_ID.to_string(), "dialog", UI_INTRODUCTION_DIALOG_ELEMENT_ID.to_string()));
        }
        if let Some(request) = chrome.chrome_build.dialog_stack.last() {
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
    pub fn route_retained_pointer_move(&mut self, pointer_id: ui_render::PointerId, x: f32, y: f32, input: &mut InputState<ActionDescriptor>) {
        let captured = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_pointer_capture_window(pointer_id).and_then(|window_id| self.retained_window_body_rect(&window_id).map(|body| (window_id, body)));
        let current = captured.or_else(|| input.hit_at(x, y).cloned().and_then(|hit| self.retained_hit_window(&hit)));
        if let Some(previous) = self.retained_hover_window.clone() {
            if current.as_ref().map(|(window_id, _)| window_id.as_str()) != Some(previous.as_str()) {
                if let Some(body) = self.retained_window_body_rect(&previous) {
                    let _ = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_pointer_event(&previous, pointer_id, semio_framework_ui_wgpu_events::UiEvent::PointerMove { x: x - body.x, y: y - body.y, modifiers: retained_event_modifiers(input) }, input);
                }
            }
        }
        self.retained_hover_window = current.as_ref().map(|(window_id, _)| window_id.clone());
        let Some((window_id, body)) = current else { return };
        let _ = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_pointer_event(&window_id, pointer_id, semio_framework_ui_wgpu_events::UiEvent::PointerMove { x: x - body.x, y: y - body.y, modifiers: retained_event_modifiers(input) }, input);
    }

    //#endregion 🎯️RetainedPointerRouting
    /// 🎡️ A published scene hit owns its notch after modal and anchored-panel precedence.
    pub fn wheel_reaches_scene_surface(&self, x: f32, y: f32, input: &InputState<ActionDescriptor>, theme: &Theme) -> bool {
        !self.pointer_input_is_modal() && !self.pointer_is_over_open_panel(x, y, theme) && input.hit_at(x, y).is_some_and(|hit| self.retained_scene_hit_is_live(hit, x, y))
    }

    /// 🚧️ Whether an overlay is MODAL for pointer input — React's menus, dropdowns, dialogs and the
    /// tour render into a layer above everything with their own backdrop, so while one is open no
    /// pointer of any phase reaches the canvas, wherever it lands. The tour's veil is already a
    /// full-screen hit target (`ui.introduction.veil`), so it answers through
    /// [`Self::pointer_hit_owner`] too; the other three publish hit rows for their own items only,
    /// and the point BESIDE an open menu is exactly the one that used to reach the world.
    pub fn candidate_pointer_input_is_modal(&self, chrome: &ChromeState) -> bool {
        self.context_menu.is_some() || matches!(self.overlay_state, OverlayState::Search | OverlayState::Find) || chrome.chrome_build.dialog_open() || chrome.chrome_build.tour_is_open() || self.open_selects.values().any(|open| *open)
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
    pub fn scene_pointer_target_at(&self, x: f32, y: f32, input: &InputState<ActionDescriptor>, theme: &Theme) -> Option<semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget> {
        if self.pointer_input_is_modal() || self.pointer_is_over_open_panel(x, y, theme) {
            return None;
        }
        let hit = input.hit_at(x, y)?;
        if !self.retained_scene_hit_is_live(hit, x, y) {
            return None;
        }
        let (_, target) = self.retained_scene_hits.get(hit.control_id.as_deref()?)?;
        semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(target).then(|| target.clone())
    }

    /// 🖼️ Captured input may outlive its rectangle, but never its published scene owner.
    pub fn scene_pointer_target_is_published(&self, target: &semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget) -> bool {
        self.retained_scene_hits.values().any(|(_, published)| published == target) && semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::scene_pointer_target_is_live(target)
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
    pub fn activate_window_under_pointer(&mut self, x: f32, y: f32, theme: &Theme, windows: &mut WindowState, dock: &mut ShellDockState) -> bool {
        if self.pointer_input_is_modal() || self.pointer_is_over_open_panel(x, y, theme) {
            return false;
        }
        let Some(window_id) = self.presented_input_geometry.dock_window_plan.iter().rev().find(|(id, rect)| !id.is_empty() && rect.contains(x, y)).map(|(id, _)| id.clone()) else {
            return false;
        };
        if windows.active_window_id.as_deref() == Some(window_id.as_str()) {
            return false;
        }
        dock.dock.sync_active_window(&window_id);
        windows.active_window_id = Some(window_id);
        true
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
            let modifiers = retained_event_modifiers(input);
            let _ = semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::dispatch_ui_event(&surface, semio_framework_ui_wgpu_events::UiEvent::Scroll { x: x - body.x, y: y - body.y, delta_x, delta_y, modifiers }, input);
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
}

//#endregion ShellInput

