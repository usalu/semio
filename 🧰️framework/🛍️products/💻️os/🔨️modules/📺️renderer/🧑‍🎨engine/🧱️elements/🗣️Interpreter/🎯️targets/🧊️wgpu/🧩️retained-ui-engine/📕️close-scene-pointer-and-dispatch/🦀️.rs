//! 📕️ Close, scene pointer and dispatch: the ui document close queue, the scene targets, the pointer owner claims and cancellation, the clipped hit registration, the catalogue drag, the window clock and the ui event dispatch. See [`super`].

use super::*;

//#region RetainedEngineCutover
/// 🧹️ Retains the exact mounted document generation that a closed window must retire.
pub fn request_ui_document_close(window_id: &str) -> bool {
    let Some((token, generation)) = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        Some((engine.surface_token(window_id)?, engine.surface_generation(window_id)?))
    }) else {
        return true;
    };
    let Ok(window_id) = SurfaceId::try_from(window_id) else { return false };
    let admitted = UI_DOCUMENT_CLOSE_QUEUE.with(|cell| {
        let mut queue = cell.borrow_mut();
        queue.try_upsert(UiDocumentCloseOwner { window_id, token, generation })
    });
    admitted && UI_ENGINE.with(|cell| cell.borrow_mut().begin_surface_close(token))
}

/// 🧵️ Advances one exact retained-window retirement unit for the frame work lane.
pub fn close_ui_document_one(cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    if cx.should_yield(){return Ok(false)}let mut failure=None;
    let advanced=(||{
    if close_retiring_focus_clipboard_one() {
        return true;
    }
    if semio_framework_os_renderer_wgpu_scene_state::close_retired_scene_surface_one() {
        return true;
    }
    let Some(owner) = UI_DOCUMENT_CLOSE_QUEUE.with(|cell| cell.borrow().owners.front().cloned()) else { return false };
    let window_id = owner.window_id.as_ref();
    if UI_ENGINE.with(|cell|{let engine=cell.borrow();engine.layout_retirement_refusal(window_id).is_some()&&!engine.layout_retirement_failure_pending(window_id)}){return false;}
    if close_window_focus_clipboard_one(window_id, owner.generation) {
        return true;
    }
    if SCENE_INTENTS.with(|cell| cell.borrow_mut().close_window_step(window_id, owner.generation)) {
        return true;
    }
    if SCENE_POINTER_OWNERS.with(|cell| {
        let mut owners = cell.borrow_mut();
        let Some(index) = owners.slots.iter().position(|entry| entry.target.window_id == window_id && entry.target.window_generation == owner.generation) else { return false };
        owners.slots.remove(index);
        true
    }) {
        return true;
    }
    let mounted = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        engine.surface_token(window_id) == Some(owner.token) && engine.surface_generation(window_id) == Some(owner.generation)
    });
    if mounted && semio_framework_os_renderer_wgpu_scene_render::retire_canvas_window_step(window_id) {
        return true;
    }
    if !semio_framework_os_renderer_wgpu_scene_state::scene_surface_retirement_can_admit() {
        return true;
    }
    let (complete, retirement) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        if engine.surface_token(window_id) != Some(owner.token) || engine.surface_generation(window_id) != Some(owner.generation) {
            return (true, None);
        }
        match engine.close_surface_one(owner.token,cx){
            Err(error)=>{failure=Some(error);(false,None)},
            Ok(step)=>match step{
            semio_framework_ui_wgpu_engine_surface::UiSurfaceCloseStep::Refused(refusal) => {
                document_debug_log(&format!("[TRACE] ui-doc retained close refusal window={window_id} stage={} kind={}", refusal.stage, refusal.kind.as_str()));
                (false, None)
            }
            semio_framework_ui_wgpu_engine_surface::UiSurfaceCloseStep::Pending => (false, None),
            semio_framework_ui_wgpu_engine_surface::UiSurfaceCloseStep::RetiredScene(retirement) => (false, Some(retirement)),
            semio_framework_ui_wgpu_engine_surface::UiSurfaceCloseStep::Complete => (true, None),
        },
        }
    });
    if let Some(retired) = retirement {
        let owner = ScenePointerTarget::from(retired);
        semio_framework_os_renderer_wgpu_scene_state::retire_scene_identity(&owner);
    }
    if complete {
        UI_DOCUMENT_CLOSE_QUEUE.with(|cell| {
            let mut queue = cell.borrow_mut();
            if queue.owners.front() == Some(&owner) {
                queue.owners.pop_front();
            }
        });
    }
    true
})();
    match failure{Some(error)=>Err(error),None=>Ok(advanced)}
}

/// 🪪️ Captures the exact retained scene identity published by one document node.
/// 🪟️ Every windowed tree container the retained surface `window_id` presents, measured against its scroll viewport
/// (`Ui::tree_window_measures`) — the Shell's tree window observer reads the shared engine through this.
pub fn tree_window_measures(window_id: &str) -> Option<(f64, Vec<semio_framework_ui_wgpu_light::tree_window::TreeWindowContainerMeasure>)> {
    UI_ENGINE.with(|cell| cell.borrow().tree_window_measures(window_id))
}

pub fn retained_scene_target(window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {
    UI_ENGINE.with(|cell| retained_scene_target_in(&cell.borrow(), window_id, node))
}

pub fn retained_scene_target_at(window_id: &str, x: f32, y: f32) -> Option<ScenePointerTarget> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let node = engine.scene_at(window_id, x, y)?.node;
        retained_scene_target_in(&engine, window_id, node)
    })
}

fn retained_scene_target_in(engine: &semio_framework_ui_wgpu_engine::Ui, window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {
    let retained = engine.tree(window_id)?.node(node)?;
    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
    if window_id.len() > SCENE_INTENT_ID_BYTES || scene.host_id.is_empty() || scene.host_id.len() > SCENE_INTENT_ID_BYTES || scene.surface_id.len() > SCENE_INTENT_ID_BYTES {
        return None;
    }
    Some(ScenePointerTarget {
        document_id: engine.retained_document_id(window_id, node)?,
        host_id: scene.host_id.clone(),
        window_id: window_id.into(),
        surface_id: scene.surface_id.clone(),
        kind: scene.component_kind,
        node,
        key: retained.key.clone(),
        window_generation: engine.surface_generation(window_id)?,
        component_generation: retained.component_generation(),
    })
}

fn candidate_scene_target_in(engine: &semio_framework_ui_wgpu_engine::Ui, window_id: &str, node: NodeId) -> Option<ScenePointerTarget> {
    let retained = engine.candidate_tree(window_id)?.node(node)?;
    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
    if window_id.len() > SCENE_INTENT_ID_BYTES || scene.host_id.is_empty() || scene.host_id.len() > SCENE_INTENT_ID_BYTES || scene.surface_id.len() > SCENE_INTENT_ID_BYTES {
        return None;
    }
    Some(ScenePointerTarget {
        document_id: engine.candidate_document_id(window_id, node)?,
        host_id: scene.host_id.clone(),
        window_id: window_id.into(),
        surface_id: scene.surface_id.clone(),
        kind: scene.component_kind,
        node,
        key: retained.key.clone(),
        window_generation: engine.candidate_surface_generation(window_id)?,
        component_generation: retained.component_generation(),
    })
}

/// 🔒️ A replacement key, kind, document generation, or component mount cannot inherit a captured gesture.
pub fn scene_pointer_target_is_live(target: &ScenePointerTarget) -> bool {
    if ui_document_close_pending_for(&target.window_id) || semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&target.host_id) {
        return false;
    }
    let retained = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_generation(&target.window_id) != Some(target.window_generation) {
            return false;
        }
        let Some(retained) = engine.tree(&target.window_id).and_then(|tree| tree.node(target.node)) else { return false };
        let UiNode::ComponentScene(scene) = &retained.spec.0 else { return false };
        engine.retained_document_id(&target.window_id, target.node) == Some(target.document_id)
            && retained.key == target.key
            && retained.component_generation() == target.component_generation
            && scene.component_kind == target.kind
            && scene.host_id == target.host_id
            && scene.surface_id == target.surface_id
    });
    retained && UI_DOCUMENT_CLOSE_QUEUE.with(|cell| !cell.borrow().owners.iter().any(|owner| owner.window_id.as_ref() == target.window_id && owner.generation == target.window_generation))
}

pub fn claim_scene_pointer_owner(target: ScenePointerTarget, pointer_id: ui_render::PointerId) -> bool {
    if !scene_pointer_target_is_live(&target) {
        return false;
    }
    SCENE_POINTER_OWNERS.with(|cell| {
        let mut owners = cell.borrow_mut();
        owners.slots.retain(|owner| owner.pointer_id != pointer_id && scene_pointer_target_is_live(&owner.target));
        if owners.slots.len() == SCENE_POINTER_OWNER_CAPACITY {
            return false;
        }
        owners.slots.push(ScenePointerOwner { pointer_id, target });
        true
    })
}

pub fn claim_scene_pointer_at(window_id: &str, x: f32, y: f32, pointer_id: ui_render::PointerId) -> bool {
    retained_scene_target_at(window_id, x, y).is_some_and(|target| claim_scene_pointer_owner(target, pointer_id))
}

pub fn captured_scene_pointer(pointer_id: ui_render::PointerId) -> Option<ScenePointerTarget> {
    SCENE_POINTER_OWNERS.with(|cell| cell.borrow().slots.iter().find(|owner| owner.pointer_id == pointer_id).map(|owner| owner.target.clone())).filter(scene_pointer_target_is_live)
}

pub fn release_scene_pointer(pointer_id: ui_render::PointerId) -> Option<ScenePointerTarget> {
    SCENE_POINTER_OWNERS
        .with(|cell| {
            let mut owners = cell.borrow_mut();
            let index = owners.slots.iter().position(|owner| owner.pointer_id == pointer_id)?;
            Some(owners.slots.remove(index).target)
        })
        .filter(scene_pointer_target_is_live)
}

pub fn cancel_scene_pointer(pointer_id: ui_render::PointerId, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<ScenePointerTarget> {
    let owners = SCENE_POINTER_OWNERS.with(|cell| {
        let mut registry = cell.borrow_mut();
        let mut cancelled = Vec::new();
        registry.slots.retain(|owner| {
            if owner.pointer_id == pointer_id {
                cancelled.push(owner.clone());
                false
            } else {
                true
            }
        });
        cancelled
    });
    let owners: Vec<_> = owners.into_iter().filter(|owner| scene_pointer_target_is_live(&owner.target)).collect();
    SCENE_INTENTS.with(|cell| cell.borrow_mut().cancel_pointer(pointer_id, &owners));
    for owner in &owners {
        let owner = &owner.target;
        match owner.kind {
            semio_framework_ui_declaration::ui::SurfaceKind::TextEditor => {
                semio_framework_os_renderer_wgpu_engine_text::text_editor::text_editor_pointer_cancel_into(&owner.host_id);
            }
            semio_framework_ui_declaration::ui::SurfaceKind::Paint2d => {
                semio_framework_os_renderer_wgpu_engine_board::paint2d::paint2d_pointer_cancel_into(&owner.host_id);
            }
            semio_framework_ui_declaration::ui::SurfaceKind::InkCanvas => {
                let controller_id = UI_ENGINE.with(|cell| {
                    let engine = cell.borrow();
                    match &engine.tree(&owner.window_id).and_then(|tree| tree.node(owner.node))?.spec.0 {
                        UiNode::ComponentScene(scene) => Some(scene.controller_id.clone()),
                        _ => None,
                    }
                });
                if let Some(controller_id) = controller_id {
                    let _ = semio_framework_os_renderer_wgpu_scene_ink::ink_gesture_abort_into(&controller_id, &owner.surface_id, &owner.host_id, input);
                }
                semio_framework_os_renderer_wgpu_scene_ink::ink_pointer_cancel_into(&owner.host_id);
            }
            _ => {}
        }
    }
    owners.into_iter().map(|owner| owner.target).collect()
}

/// 👆️ Last-seen `(pointer_down, pointer_button)` per `window_id`, so `dispatch_pointer_events` can
/// detect Down/Up edges from `InputState`'s per-frame aggregate.
#[cfg(any(test, feature = "testkit"))]
static POINTER_EDGE_STATE: WorkerCell<std::collections::HashMap<String, (bool, i16)>> = WorkerCell::new(Default::default);

/** 🖇️ Public hook for the sibling `w3-shell-input-cutover` workstream (region `shell::ShellInput`,
 * which this ticket must not touch): routes a fully-formed `ui_wgpu::wgpu::UiEvent` (built from raw
 * winit input, e.g. real key events/IME/focus-scoped routing this function's own per-frame
 * pointer-only synthesis below deliberately does not attempt) into the same process-wide retained
 * engine `render_ui_node` itself drives, and forwards any resulting `UiCommand::App` action into the
 * same reserved bounded-action/`app.dispatch_actions` pipeline other actions already use. Returns the raw
 * command list too, for `FocusChanged`/`OverlayClosed`/`DropCommitted`/clipboard commands a caller may
 * want to react to itself (e.g. an actual OS clipboard read for `ClipboardPasteRequested` is a `host`
 * concern, not this function's). */
/** 🎯️ Pushes `window_id`'s retained pointer registry — one entry per interactive node of its last
 * published paint, at the rect that paint drew it at — into the host's own `InputState`, and answers
 * the control ids it minted so the caller can tell a document hit from a chrome hit.
 *
 * 🩸️ Without this the shell's registry held the chrome and nothing else: a pointer inside a
 * published `mounted_layout` row rect resolved the WINDOW's own `HitKind::ScrollRegion`, so no row
 * action dispatched, no wheel propagated to a scene and no hover ever resolved (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6). The
 * window's own `ScrollRegion` is registered by the chrome BEFORE the body paints, so every body
 * child outranks it in `InputState::hit_at`'s reverse scan. */
pub fn register_retained_hit_targets(window_id: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<String> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let registrations = engine.window_hit_targets(window_id);
        let mut ids = Vec::with_capacity(registrations.len());
        for registration in registrations {
            ids.push(registration.control_id.clone());
            input.register_hit(registration.to_hit_target());
        }
        ids
    })
}

pub fn register_clipped_retained_hit_targets(window_id: &str, clip: semio_framework_ui_theme::geometry::Rect, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<(String, Option<ScenePointerTarget>, Rect)> {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let mut ids = Vec::new();
        for registration in engine.window_hit_targets(window_id) {
            let mut target = registration.to_hit_target();
            if !registration.overlay {
                let x = registration.rect.x.max(clip.x);
                let y = registration.rect.y.max(clip.y);
                let right = (registration.rect.x + registration.rect.w).min(clip.x + clip.w);
                let bottom = (registration.rect.y + registration.rect.h).min(clip.y + clip.h);
                if right <= x || bottom <= y {
                    continue;
                }
                target.rect = semio_framework_ui_theme::geometry::Rect::new(x, y, right - x, bottom - y);
            }
            let scene = registration.scene.as_ref().and_then(|_| candidate_scene_target_in(&engine, window_id, registration.node));
            ids.push((registration.control_id.clone(), scene, target.rect));
            input.register_hit(target);
        }
        ids
    })
}

/// 🖱️ The retained window whose content holds pointer capture right now — a host routes every move
/// there until release, so a `Slider`/`Ring` drag survives leaving the control's own rect.
pub fn retained_pointer_capture_window(pointer_id: ui_render::PointerId) -> Option<String> {
    UI_ENGINE.with(|cell| cell.borrow().window_with_pointer_capture(pointer_id.0).map(str::to_owned))
}

pub fn retained_any_pointer_capture() -> Option<(String, ui_render::PointerId)> {
    UI_ENGINE.with(|cell| cell.borrow().any_pointer_capture().map(|(window_id, pointer_id)| (window_id.to_owned(), ui_render::PointerId(pointer_id))))
}

/// 📐️ The SOLVED content height of one retained surface — what a floating panel hugs up to
/// `calc(100% - 2 · spacing)`, React's `<Panel>` sizing (`🖼️Panel/🟦️.tsx`). `None` until that surface's
/// layout has been accepted once, so the caller keeps its own band for the first frame.
pub fn retained_content_height(window_id: &str) -> Option<f32> {
    UI_ENGINE.with(|cell| cell.borrow().surface_content_height(window_id))
}

#[cfg(any(test, feature = "testkit"))]
/// 🌳️ Whether production reconcile retained `window_id` with a Tree root.
pub fn retained_root_is_tree(window_id: &str) -> bool {
    UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let Some(tree) = engine.tree(window_id) else { return false };
        tree.root.and_then(|root| tree.node(root)).is_some_and(|node| matches!(&node.spec.0, semio_framework_ui_declaration::ui::UiNode::Tree(_)))
    })
}

/// 🫳️ Retained panel/catalogue drags (`EventRouter::DragSession`) the legacy shell `tree_drag` path never sees.
pub fn active_retained_drag_sessions() -> Vec<(String, f32, f32, std::collections::HashMap<String, String>)> {
    UI_ENGINE.with(|cell| cell.borrow().active_drag_sessions())
}

pub fn canvas_catalogue_drag_over(window_id: &str, x: f32, y: f32, drag_data: &std::collections::HashMap<String, String>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    if !drag_data.contains_key(semio_framework_os_renderer_wgpu_scene_render::CANVAS_CATALOGUE_DRAG_MIME) {
        return semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_catalogue_drag(input);
    }
    let mut types = drag_data.keys().cloned().collect::<Vec<_>>();
    types.sort();
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let hit = engine.scene_at(window_id, x, y)?;
        if hit.kind != semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d {
            return None;
        }
        let generation = engine.surface_generation(window_id)?;
        let tree = engine.tree(window_id)?;
        let UiNode::ComponentScene(scene) = &tree.node(hit.node)?.spec.0 else { return None };
        Some(semio_framework_os_renderer_wgpu_scene_render::canvas_catalogue_drag_over_into(scene, hit.rect, window_id, generation, x, y, &types, input))
    });
    match outcome {
        Some(Ok(accepted)) => accepted,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            false
        }
        None => {
            semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_catalogue_drag(input);
            false
        }
    }
}

pub fn canvas_catalogue_drop(window_id: &str, x: f32, y: f32, drag_data: &std::collections::HashMap<String, String>, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
    let Some(raw_payload) = drag_data.get(semio_framework_os_renderer_wgpu_scene_render::CANVAS_CATALOGUE_DRAG_MIME) else {
        return false;
    };
    let outcome = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        let hit = engine.scene_at(window_id, x, y)?;
        if hit.kind != semio_framework_ui_declaration::ui::SurfaceKind::Canvas2d {
            return None;
        }
        let generation = engine.surface_generation(window_id)?;
        let tree = engine.tree(window_id)?;
        let UiNode::ComponentScene(scene) = &tree.node(hit.node)?.spec.0 else { return None };
        Some(semio_framework_os_renderer_wgpu_scene_render::canvas_catalogue_drop_into(scene, hit.rect, window_id, generation, x, y, raw_payload, input))
    });
    match outcome {
        Some(Ok(accepted)) => accepted,
        Some(Err(fault)) => {
            input.record_action_fault(fault);
            false
        }
        None => {
            semio_framework_os_renderer_wgpu_scene_render::cancel_canvas_catalogue_drag(input);
            false
        }
    }
}

pub(super) fn stamp_window_clock(engine: &mut semio_framework_ui_wgpu_engine::Ui, window_id: &str) -> Option<semio_framework_ui_wgpu_engine_surface::UiWindowClockStep> {
    let seconds = semio_framework_job::default_now_us()? as f64 / 1_000_000.0;
    engine.advance_window_clock(window_id, seconds)
}

pub(super) fn apply_window_clock_caret(window_id: &str, step: Option<&semio_framework_ui_wgpu_engine_surface::UiWindowClockStep>) {
    if let Some(caret) = step.and_then(|step| step.caret) {
        apply_presented_scene_caret(window_id, caret);
    }
}

/// ⏲️ Advances one accepted surface and delivers its timer commands through the ordinary boundary.
pub fn advance_retained_window_clock(window_id: &str, surface: semio_framework_ui_wgpu_engine::UiSurfaceToken, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Option<semio_framework_ui_wgpu_engine_surface::UiWindowClockStep> {
    let (step, mut commands) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        if engine.surface_token(window_id) != Some(surface) {
            return (None, Vec::new());
        }
        let step = stamp_window_clock(&mut engine, window_id);
        (step, engine.drain_commands())
    });
    apply_ui_commands(&mut commands, None, input);
    apply_window_clock_caret(window_id, step.as_ref());
    step
}

pub fn dispatch_ui_event(window_id: &str, event: semio_framework_ui_wgpu_events::UiEvent, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<semio_framework_ui_wgpu_events::UiCommand> {
    let (mut commands, clock) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        let clock = stamp_window_clock(&mut engine, window_id);
        let mut commands = engine.drain_commands();
        commands.extend(engine.dispatch_event(window_id, event));
        (commands, clock)
    });
    apply_window_clock_caret(window_id, clock.as_ref());
    apply_ui_commands(&mut commands, None, input);
    commands
}

pub fn dispatch_ui_pointer_event(window_id: &str, pointer_id: ui_render::PointerId, event: semio_framework_ui_wgpu_events::UiEvent, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<semio_framework_ui_wgpu_events::UiCommand> {
    dispatch_presented_ui_pointer_event(window_id, pointer_id, event, input)
}

/// 🎞️ Routes pointer state through the exact presented retained tree and router.
pub fn dispatch_presented_ui_pointer_event(window_id: &str, pointer_id: ui_render::PointerId, event: semio_framework_ui_wgpu_events::UiEvent, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<semio_framework_ui_wgpu_events::UiCommand> {
    if matches!(event, semio_framework_ui_wgpu_events::UiEvent::PointerDown { .. }) {
        FOCUSED_TABLE_STEPPER.with(|cell| *cell.borrow_mut() = None);
        FOCUSED_VFS_CONTROL.with(|cell| *cell.borrow_mut() = None);
    }
    let (mut commands, clock) = UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        let clock = stamp_window_clock(&mut engine, window_id);
        let mut commands = engine.drain_commands();
        commands.extend(engine.dispatch_pointer_event(window_id, pointer_id.0, event));
        (commands, clock)
    });
    apply_window_clock_caret(window_id, clock.as_ref());
    apply_ui_commands(&mut commands, Some(pointer_id), input);
    commands
}
//#endregion RetainedEngineCutover
