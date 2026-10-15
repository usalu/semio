//! 🖱️ Scene interaction steps and the mounted scene host.
//!
//! Driving one scene interaction step, the presented input candidates, the surface context menu and the framework
//! scene host with its mounted layout text step.

use super::ink_and_vfs_focus::{focus_ink_editor, focus_ink_surface, focus_vfs_control};
use super::retained_ui_engine::{
    ui_document_close_pending_for, SceneIntentEvent, SceneInteractionIntent, SCENE_INTENTS,
    UI_ENGINE,
};
use super::text_and_table_focus::{
    clear_focused_table_editable_text, commit_focused_table_editable_text_with_engine,
    focus_table_editable_text, focus_table_stepper, focused_table_editable_text_address,
    install_text_editor_focus, reset_text_editor_caret, text_editor_focus_in,
};
use super::ui_image_loading::{framework_widget_context, render_ui_image_step};
use crate::element_contracts::ScenePointerTarget;
use crate::scenes::render_component_scene_step;
use ui_wgpu::wgpu::{ActionDescriptor, Rect, Theme, UiNode};

pub fn drive_scene_interaction_step(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    crate::scenes::request_stale_canvas_authority_cancel(|window_id, generation| UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id) == Some(generation)));
    if crate::scenes::canvas_pointer_terminal_step(input) {
        return true;
    }
    let Some(mut intent) = SCENE_INTENTS.with(|cell| cell.borrow_mut().pop_front()) else {
        return false;
    };
    match process_scene_interaction(&mut intent, input) {
        Ok(SceneIntentProgress::Complete) => {}
        Ok(SceneIntentProgress::Pending) => SCENE_INTENTS.with(|cell| cell.borrow_mut().push_front(intent)),
        Err(fault) => {
            intent.retiring = true;
            if !intent.close_step() {
                SCENE_INTENTS.with(|cell| cell.borrow_mut().push_front(intent));
            }
            input.record_action_fault(fault);
        }
    }
    true
}

pub fn close_scene_interaction_step() -> bool {
    SCENE_INTENTS.with(|cell| cell.borrow_mut().close_step())
}

pub fn scene_interaction_terminal_is_empty() -> bool {
    SCENE_INTENTS.with(|cell| cell.borrow().is_empty())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PresentedInputCandidateProgress {
    Stale,
    Pending { advanced: bool },
    Ready,
}

pub(crate) fn progress_presented_input_candidate(witness: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> PresentedInputCandidateProgress {
    let blocked = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if !engine.presented_input_candidate_matches(witness) {
            return None;
        }
        Some(SCENE_INTENTS.with(|cell| cell.borrow().blocks_presented_candidate(&engine, witness)))
    });
    match blocked {
        None => PresentedInputCandidateProgress::Stale,
        Some(false) => PresentedInputCandidateProgress::Ready,
        Some(true) => {
            let advanced = drive_scene_interaction_step(input);
            PresentedInputCandidateProgress::Pending { advanced }
        }
    }
}

//#region 🖱️SurfaceContextMenu
/** 🖱️ One component scene resolved for a right-click: which surface the pointer is over, and
 * that kind's own `hits`/`selection`/`text` — the `surface` half of React's
 * `openSurfaceContextMenu({ surface: { surfaceId, kind, hits, selection } })`. */
pub struct SurfaceContextMenuTarget {
    pub surface_id: String,
    pub kind: ui_wgpu::wgpu::SurfaceKind,
    pub rect: Rect,
    pub target: crate::scenes::SceneContextMenuTarget,
}

/** 🖱️ The component scene under a WINDOW-LOCAL point, already resolved into its context-menu
 * surface target. `None` when the point is over chrome that no scene owns — the caller then falls
 * back to the shell's own `"window"` menu, exactly as React's `ShellContextMenu` does when no scene
 * host claimed the event.
 *
 * `Ui::scene_at` is a READ-ONLY query: it resolves the leaf through the same reverse-paint-order walk
 * a press uses without moving focus or arming a capture, which a right-click must not do (React's
 * `onContextMenu` calls `preventDefault()` and never lets the press through either).
 *
 * `SurfaceKind::World3d` answers with an EMPTY target on purpose: a world surface's hover/selection
 * authority is `World3dState`, which lives in the shell's own `world3d_states` map, so the shell
 * fills it from `infinite_world::world::world3d_context_menu_surface`. */
pub fn surface_context_menu_target(window_id: &str, x: f32, y: f32) -> Option<SurfaceContextMenuTarget> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let hit = engine.scene_at(window_id, x, y)?;
        let tree = engine.tree(window_id)?;
        let UiNode::ComponentScene(scene) = &tree.node(hit.node)?.spec.0 else { return None };
        let target = crate::scenes::scene_context_menu_target(scene, hit.rect, x, y);
        Some(SurfaceContextMenuTarget { surface_id: hit.surface_id, kind: hit.kind, rect: hit.rect, target })
    })
}

/** 🖱️ Parks a context-menu row on the text-editor surface under a WINDOW-LOCAL point when
 * that row is one the editor answers itself, and reports whether it did — React's
 * `dispatchTextEditorMenu`, which runs an id present in its own `localActions` map instead of
 * dispatching it to the guest. `false` for every other surface kind and every other row. */
pub fn text_editor_claim_menu_action(window_id: &str, x: f32, y: f32, action: &str) -> bool {
    UI_ENGINE
        .with(|cell| {
            let engine = cell.borrow();
            let hit = engine.scene_at(window_id, x, y)?;
            (hit.kind == ui_wgpu::wgpu::SurfaceKind::TextEditor).then(|| crate::scenes::text_editor_queue_menu_action(&hit.surface_id, action, x, y))
        })
        .unwrap_or(false)
}

/** 📋️ An ALT-held secondary press on a text editor opens the completions dropdown instead of
 * a context menu — React's `event.altKey && completions.length > 0` branch in the host's own
 * `onContextMenu` (`🧱️elements/✏️TextEditor/🟦️.tsx:413`). `false` leaves the press to the menu. */
pub fn text_editor_claim_alt_completions(window_id: &str, x: f32, y: f32) -> bool {
    UI_ENGINE
        .with(|cell| {
            let engine = cell.borrow();
            let hit = engine.scene_at(window_id, x, y)?;
            if hit.kind != ui_wgpu::wgpu::SurfaceKind::TextEditor {
                return None;
            }
            let tree = engine.tree(window_id)?;
            let UiNode::ComponentScene(scene) = &tree.node(hit.node)?.spec.0 else { return None };
            Some(crate::scenes::text_editor_open_completions(scene))
        })
        .unwrap_or(false)
}
//#endregion 🖱️SurfaceContextMenu

enum SceneIntentProgress {
    Pending,
    Complete,
}

/// 🍿️ A press on an open completions dropdown COMMITS that row (and a press
/// anywhere else dismisses it) before the caret path moves the caret — React's
/// popup is a real element above the canvas and swallows the press the same way.
///
/// 🖱️ The two modifier flags were literal `false` here — `paint2d_pointer_button_into`
/// has asked for shift/ctrl since it was written, and this, its only production caller,
/// could not answer. React reads them off the pointer event
/// (`🖌️Paint2dHost/🟦️.tsx:503`'s `marqueeModeFromModifiers`).
fn process_scene_interaction(intent: &mut SceneInteractionIntent, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<SceneIntentProgress, ui_wgpu::wgpu::BoundedActionFault> {
    let operation_generation = intent.generation;
    if operation_generation == 0 {
        intent.retiring = true;
    }
    let window_id = intent.window_id.as_str();
    let node = intent.node;
    let kind = intent.kind;
    let rect = intent.rect;
    let live = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        engine.tree_revision(window_id) == Some(intent.tree_revision)
            && engine.surface_generation(window_id) == Some(intent.surface_generation)
            && engine.tree(window_id).and_then(|tree| tree.node(node)).and_then(|retained| match &retained.spec.0 {
                UiNode::ComponentScene(scene) => Some(scene.component_kind == kind && scene.host_id == intent.host_id),
                _ => None,
            }) == Some(true)
    });
    if !live || ui_document_close_pending_for(window_id) || crate::scenes::scene_host_retiring(&intent.host_id) {
        intent.retiring = true;
    }
    if intent.retiring {
        return Ok(if intent.close_step() { SceneIntentProgress::Complete } else { SceneIntentProgress::Pending });
    }
    if crate::scenes::scene_has_bespoke_pointer_dispatch(kind) {
        return Ok(SceneIntentProgress::Complete);
    }
    if kind == ui_wgpu::wgpu::SurfaceKind::TextEditor {
        let (result, focus) = UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            let Some(tree) = engine.tree(window_id) else {
                return (Ok(SceneIntentProgress::Complete), None);
            };
            let Some(retained) = tree.node(node) else {
                return (Ok(SceneIntentProgress::Complete), None);
            };
            let UiNode::ComponentScene(scene) = &retained.spec.0 else {
                return (Ok(SceneIntentProgress::Complete), None);
            };
            let focus = matches!(intent.event, SceneIntentEvent::PointerDown { .. }).then(|| text_editor_focus_in(&engine, window_id, intent.surface_generation, node, &scene.host_id)).flatten();
            let result = match intent.event {
                SceneIntentEvent::PointerDown { x, y, button, .. } => {
                    match crate::scenes::text_editor_popup_pointer(scene, rect, x, y, false, input) {
                        Ok(true) => return (Ok(SceneIntentProgress::Complete), focus),
                        Ok(false) => {}
                        Err(fault) => return (Err(fault), focus),
                    }
                    crate::engine_canvas::text_editor_pointer_button_into(scene, rect, x, y, button, true, input)
                }
                SceneIntentEvent::PointerUp { x, y, button, .. } => crate::engine_canvas::text_editor_pointer_button_into(scene, rect, x, y, button, false, input),
                SceneIntentEvent::PointerMove { x, y, .. } => crate::engine_canvas::text_editor_pointer_move_into(scene, rect, x, y, input),
                SceneIntentEvent::Scroll { delta_y, .. } => Ok(crate::engine_canvas::text_editor_wheel_into(scene, delta_y)),
            };
            (result.map(|_| SceneIntentProgress::Complete), focus)
        });
        if let Some(focus) = focus {
            install_text_editor_focus(focus.clone());
            reset_text_editor_caret(&focus);
        }
        return result;
    }
    if kind == ui_wgpu::wgpu::SurfaceKind::Paint2d {
        return UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            let Some(tree) = engine.tree(window_id) else {
                return Ok(SceneIntentProgress::Complete);
            };
            let Some(retained) = tree.node(node) else {
                return Ok(SceneIntentProgress::Complete);
            };
            let UiNode::ComponentScene(scene) = &retained.spec.0 else {
                return Ok(SceneIntentProgress::Complete);
            };
            let result = match intent.event {
                SceneIntentEvent::PointerDown { x, y, button, modifiers } => crate::engine_canvas::paint2d_pointer_button_into(scene, rect, x, y, true, button, modifiers.shift, modifiers.ctrl || modifiers.meta, input),
                SceneIntentEvent::PointerUp { x, y, button, modifiers } => crate::engine_canvas::paint2d_pointer_button_into(scene, rect, x, y, false, button, modifiers.shift, modifiers.ctrl || modifiers.meta, input),
                SceneIntentEvent::PointerMove { x, y, .. } => crate::engine_canvas::paint2d_pointer_move_into(scene, rect, x, y, input),
                SceneIntentEvent::Scroll { x, y, delta_y, .. } if delta_y.abs() >= 0.01 => crate::engine_canvas::paint2d_wheel_into(scene, rect, x, y, delta_y, input),
                SceneIntentEvent::Scroll { .. } => Ok(false),
            };
            result.map(|_| SceneIntentProgress::Complete)
        });
    }
    if kind == ui_wgpu::wgpu::SurfaceKind::Canvas2d {
        return UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            let Some(tree) = engine.tree(window_id) else {
                return Ok(SceneIntentProgress::Complete);
            };
            let Some(retained) = tree.node(node) else {
                return Ok(SceneIntentProgress::Complete);
            };
            let UiNode::ComponentScene(scene) = &retained.spec.0 else {
                return Ok(SceneIntentProgress::Complete);
            };
            let result = match intent.event {
                SceneIntentEvent::PointerDown { x, y, button, modifiers } => crate::scenes::canvas_pointer_button_into(
                    scene,
                    rect,
                    intent.pointer_id.ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?,
                    window_id,
                    intent.surface_generation,
                    x,
                    y,
                    true,
                    button,
                    crate::scenes::SceneModifiers::from(modifiers),
                    input,
                ),
                SceneIntentEvent::PointerUp { x, y, button, modifiers } => crate::scenes::canvas_pointer_button_into(
                    scene,
                    rect,
                    intent.pointer_id.ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?,
                    window_id,
                    intent.surface_generation,
                    x,
                    y,
                    false,
                    button,
                    crate::scenes::SceneModifiers::from(modifiers),
                    input,
                ),
                SceneIntentEvent::PointerMove { x, y, modifiers } => {
                    let pointer_id = intent.pointer_id.ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
                    let result = crate::scenes::canvas_pointer_move_into(scene, rect, pointer_id, window_id, intent.surface_generation, x, y, crate::scenes::SceneModifiers::from(modifiers), input);
                    if result.is_ok() {
                        crate::scenes::set_scene_last_pointer_pos(&scene.host_id, x, y);
                    }
                    result
                }
                SceneIntentEvent::Scroll { x, y, delta_y, .. } => Ok(crate::scenes::canvas_wheel_into(scene, rect, x, y, delta_y)),
            };
            result.map(|_| SceneIntentProgress::Complete)
        });
    }
    if kind == ui_wgpu::wgpu::SurfaceKind::InkCanvas {
        return UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            let Some(tree) = engine.tree(window_id) else {
                return Ok(SceneIntentProgress::Complete);
            };
            let Some(retained) = tree.node(node) else {
                return Ok(SceneIntentProgress::Complete);
            };
            let UiNode::ComponentScene(scene) = &retained.spec.0 else {
                return Ok(SceneIntentProgress::Complete);
            };
            if matches!(intent.event, SceneIntentEvent::PointerDown { button: 0, .. }) {
                if let Some(generation) = engine.surface_generation(window_id) {
                    focus_ink_surface(window_id, generation, node, &scene.host_id, rect);
                }
            }
            match intent.event {
                SceneIntentEvent::Scroll { x, y, delta_y, .. } if delta_y.abs() >= 0.01 => crate::scenes::ink_wheel_into(scene, rect, x, y, delta_y, input).map(|_| SceneIntentProgress::Complete),
                SceneIntentEvent::Scroll { .. } => Ok(SceneIntentProgress::Complete),
                event => {
                    if intent.ink_job.is_none() {
                        let event = match event {
                            SceneIntentEvent::PointerDown { x, y, button, modifiers } => crate::scenes::InkInteractionEvent::PointerDown { x, y, button, shift: modifiers.shift },
                            SceneIntentEvent::PointerUp { x, y, .. } => crate::scenes::InkInteractionEvent::PointerUp { x, y },
                            SceneIntentEvent::PointerMove { x, y, .. } => crate::scenes::InkInteractionEvent::PointerMove { x, y },
                            SceneIntentEvent::Scroll { .. } => unreachable!(),
                        };
                        intent.ink_job = crate::scenes::InkInteractionJob::new(operation_generation, intent.surface_generation, intent.pointer_id, scene, event)?;
                        if intent.ink_job.is_none() {
                            return Ok(SceneIntentProgress::Complete);
                        }
                        return Ok(SceneIntentProgress::Pending);
                    }
                    match intent.ink_job.as_mut().expect("ink job retained").step(operation_generation, scene, rect, input)? {
                        crate::scenes::InkInteractionStep::Pending => Ok(SceneIntentProgress::Pending),
                        crate::scenes::InkInteractionStep::Complete => {
                            if intent.ink_job.as_ref().is_some_and(crate::scenes::InkInteractionJob::opened_edit) {
                                if let Some(generation) = engine.surface_generation(window_id) {
                                    focus_ink_editor(window_id, generation, node, &scene.host_id);
                                }
                            }
                            intent.retiring = true;
                            Ok(SceneIntentProgress::Pending)
                        }
                    }
                }
            }
        });
    }
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let Some(tree) = engine.tree(window_id) else {
            return Ok(SceneIntentProgress::Complete);
        };
        let Some(n) = tree.node(node) else {
            return Ok(SceneIntentProgress::Complete);
        };
        let UiNode::ComponentScene(scene) = &n.spec.0 else {
            return Ok(SceneIntentProgress::Complete);
        };
        match intent.event {
            SceneIntentEvent::PointerDown { x, y, button, modifiers } => {
                if kind == ui_wgpu::wgpu::SurfaceKind::Table && button == 0 {
                    if let Some(target) = crate::scenes::table_editable_text_focus_target(scene, rect, x, y, intent.driver_drag) {
                        focus_table_editable_text(&engine, window_id, intent.surface_generation, node, &scene.host_id, target, input)?;
                    } else {
                        let same_table = focused_table_editable_text_address().is_some_and(|(focused_window, focused_generation, focused_node, focused_host, _, _, _, _, _)| {
                            focused_window == window_id && focused_generation == intent.surface_generation && focused_node == node && focused_host == scene.host_id
                        });
                        if same_table {
                            match commit_focused_table_editable_text_with_engine(&engine, input).transpose()? {
                                Some(crate::scenes::TableEditableTextCommitOutcome::Conflict | crate::scenes::TableEditableTextCommitOutcome::Publishing | crate::scenes::TableEditableTextCommitOutcome::AwaitingEcho) => {}
                                _ => clear_focused_table_editable_text(input),
                            }
                        }
                    }
                    if let Some(target) = crate::scenes::table_stepper_focus_target(scene, rect, x, y, intent.driver_drag) {
                        focus_table_stepper(window_id, intent.surface_generation, node, &scene.host_id, target);
                    }
                }
                if kind == ui_wgpu::wgpu::SurfaceKind::VirtualFileSystem && button == 0 {
                    if let Some(target) = crate::scenes::vfs_accessibility_focus_target(scene, x, y) {
                        focus_vfs_control(window_id, intent.surface_generation, node, &scene.host_id, target);
                    }
                }
                let pointer_id = intent.pointer_id.ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
                crate::scenes::passive_scene_pointer_button(scene, rect, pointer_id, x, y, true, button, crate::scenes::SceneModifiers::from(modifiers), window_id, intent.surface_generation, intent.driver_drag, input)?;
            }
            SceneIntentEvent::PointerUp { x, y, button, modifiers } => {
                let pointer_id = intent.pointer_id.ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
                crate::scenes::passive_scene_pointer_button(scene, rect, pointer_id, x, y, false, button, crate::scenes::SceneModifiers::from(modifiers), window_id, intent.surface_generation, intent.driver_drag, input)?;
            }
            SceneIntentEvent::PointerMove { x, y, .. } => {
                let pointer_id = intent.pointer_id.ok_or(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)?;
                crate::scenes::passive_scene_pointer_move(scene, rect, pointer_id, x, y, window_id, intent.surface_generation, intent.driver_drag);
            }
            SceneIntentEvent::Scroll { x, y, delta_y, .. } => {
                if delta_y.abs() >= 0.01 {
                    crate::scenes::passive_scene_wheel(scene, rect, x, y, delta_y);
                }
            }
        }
        Ok(SceneIntentProgress::Complete)
    })
}

/** 🎬️ The `SceneHost` implementor closing the SceneHost gap `paint_unbridged_scene_and_image_leaves`
 * used to paper over: reads a `SceneSlot`'s payload — a `&UiComponentSceneNode`/`&UiImageNode`
 * borrowed straight from `ui_wgpu`'s own retained tree, never a second copy — and dispatches to the
 * UNCHANGED `render_component_scene`/`render_ui_image`, the exact functions the deleted shadow walk
 * used to call, now reached through the real bridge instead of a second immediate-mode layout pass.
 * `collect_scene_slots` (inside `Ui::frame`) already does a full, unconditional tree walk, so a scene
 * nested under `Group`/`Tree`/any other container resolves here too — not just `Stack`/`Section`/
 * `Field`, the shadow walk's own hard-coded set.
 *
 * Constructed fresh once per `render_ui_node` call, borrowing exactly the per-frame state that call
 * already has in scope (never stored longer than that one call) — mirrors why `Ui::frame` itself
 * takes `scene_host` as a parameter rather than a stored field (see that method's doc comment):
 * `gpu`/the per-surface-kind state maps aren't anything a `Ui`-owned `Box<dyn SceneHost>` could hold. */
pub(super) struct FrameworkSceneHost<'ctx> {
    pub(super) input: &'ctx mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    pub(super) theme: &'ctx Theme,
    pub(super) scroll_offsets: &'ctx mut std::collections::HashMap<String, f32>,
    pub(super) collapsed_sections: &'ctx mut std::collections::HashMap<String, bool>,
    pub(super) open_selects: &'ctx mut std::collections::HashMap<String, bool>,
    pub(super) chrome_labels: crate::scenes::SceneChromeLabels,
    /// 🌍️ The shell's own `World3dState` map and this frame's world build context — the two pieces a
    /// `SurfaceKind::World3d` slot needs and a `Ui`-owned `Box<dyn SceneHost>` could never hold.
    pub(super) world3d_states: &'ctx mut crate::scenes::AdmittedSurfaceMap<infinite_world::world::World3dState>,
    pub(super) world_resources: &'ctx mut infinite_world::world::World3dBuildContext,
    /// 🪟️ The window instance this paint belongs to — carried so every engine surface the walk
    /// attaches names its retention owner (`🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`).
    pub(super) window_id: &'ctx str,
    pub(super) driver_drag: ui_wgpu::wgpu::UiDriverDrag,
    pub(super) document_generation: u64,
    pub(super) window_generation: u64,
}

impl ui_wgpu::wgpu::SceneHost for FrameworkSceneHost<'_> {
    /// 🖌️ `draw`/`atlas`/`icons` are `Ui::frame`'s own per-tick parameters, reborrowed fresh by the
    /// engine for each slot (see `scene_slots::SceneHost::paint_slot`'s doc comment) — `draw` is the
    /// retained window's own `DrawList`, in that window's local `(0,0)`-origin space, the same space
    /// `slot.rect` is expressed in (the caller composites/offsets the WHOLE retained `DrawList` by
    /// `bounds.x`/`bounds.y` afterward via `composite_retained_draw_list`, exactly like every other
    /// retained-paint call — so real scene/image pixels painted here land in the right place for free).
    fn paint_slot_step(
        &mut self,
        slot: &ui_wgpu::wgpu::SceneSlot<'_>,
        cursor: &mut ui_wgpu::wgpu::ScenePaintCursor,
        draw: &mut ui_wgpu::wgpu::DrawList,
        atlas: &mut ui_wgpu::wgpu::FontAtlas,
        icons: Option<&ui_wgpu::wgpu::IconAtlas>,
    ) -> ui_wgpu::wgpu::ScenePaintStep {
        match cursor.bind(slot.node) {
            Ok(true) => {}
            Ok(false) => return ui_wgpu::wgpu::ScenePaintStep::Pending,
            Err(_) => return ui_wgpu::wgpu::ScenePaintStep::Fault,
        }
        let viewport_height = draw.screen_height();
        let mut ctx = framework_widget_context(draw, None, atlas, icons, self.input, self.theme, self.scroll_offsets, self.collapsed_sections, self.open_selects, None, viewport_height);
        let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: self.chrome_labels, world3d_states: &mut *self.world3d_states, world_resources: &mut *self.world_resources, window_id: self.window_id };
        match &slot.content {
            ui_wgpu::wgpu::SlotContent::Scene(scene) => {
                if cursor.phase() == 0 && cursor.byte() == 0 {
                    let Some(document_id) = slot.document_id else { return ui_wgpu::wgpu::ScenePaintStep::Fault };
                    let owner = ScenePointerTarget {
                        document_id,
                        host_id: scene.host_id.clone(),
                        window_id: self.window_id.into(),
                        window_generation: self.window_generation,
                        component_generation: slot.component_generation,
                        node: slot.node,
                        key: slot.key.clone(),
                        kind: scene.component_kind,
                        surface_id: scene.surface_id.clone(),
                    };
                    if !crate::scenes::mount_scene_identity(&owner) || scene.component_kind == ui_wgpu::wgpu::SurfaceKind::Canvas2d && !crate::scenes::mount_canvas_camera(scene, owner) {
                        return ui_wgpu::wgpu::ScenePaintStep::Fault;
                    }
                }
                render_component_scene_step(scene, slot.rect, &mut ctx, cursor, &mut hosts, self.driver_drag, self.window_generation)
            }
            ui_wgpu::wgpu::SlotContent::Image(image) => render_ui_image_step(image, slot.rect, &mut ctx, cursor),
        }
    }
}

/// ⏱️ Whether one layout scheduling call was `window_id`'s own work: a step of its own job (or an `Idle` stall of a dirty layout)
/// is; a wait on the worker pool or a step of another surface's job is not.
pub(super) fn layout_step_is_window_work(step: &ui_wgpu::wgpu::UiLayoutStep, window_id: &str) -> bool {
    match step {
        ui_wgpu::wgpu::UiLayoutStep::Awaiting { .. } => false,
        ui_wgpu::wgpu::UiLayoutStep::Refused { .. } => false,
        ui_wgpu::wgpu::UiLayoutStep::Idle => true,
        ui_wgpu::wgpu::UiLayoutStep::Yielded { window_id: stepped, .. } | ui_wgpu::wgpu::UiLayoutStep::Ready { window_id: stepped, .. } | ui_wgpu::wgpu::UiLayoutStep::Cancelled { window_id: stepped, .. } => AsRef::<str>::as_ref(stepped) == window_id,
    }
}

pub(super) fn drive_mounted_layout_text_one(engine:&mut ui_wgpu::wgpu::Ui,atlas:&mut ui_wgpu::wgpu::FontAtlas,cx:&mut semio_framework_job::StepContext<'_>)->ui_wgpu::wgpu::UiLayoutStep{
    let pool=crate::renderer_worker_pool();
    let progress=engine.step_layouts(&pool,atlas,cx);
    #[cfg(target_arch="wasm32")]
    if let Some(now)=cx.now_us(){let _=pool.pump(now/1_000);}
    progress
}
