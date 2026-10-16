//! ⌨️ Scene input: canvas pointer wire, cancellation, catalogue drag, row transfer, passive pointer and list hits.

use super::*;

/// 🖱️ ONE canvas-2d pointer sample in the shape `📐️Canvas2dHost`'s gesture lane publishes it
/// (`🧱️elements/📐️Canvas2dHost/🟦️.tsx:426-446`): `x`/`y` are SCREEN-LOGICAL pixels inside the surface
/// rect and `width`/`height` its logical size, because every plugin reader converts them itself —
/// `🖍️draw`'s `canvas_point_to_world(viewport, x, y, width, height)` and `📏️layout`'s
/// `hit_test_at(doc, cfg, x, y, width, height)`. `world_x`/`world_y` ride along as the optional world
/// lane `🖍️draw`'s own payload already declares, and `surface_id`/`extend` stay because `📏️layout`'s
/// `CanvasPointerDown` reads them.
struct CanvasPointerWire {
    screen_x: f32,
    screen_y: f32,
    pub(crate) world_x: f32,
    pub(crate) world_y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) button: Option<i16>,
    pub(crate) modifiers: SceneModifiers,
    cancelled: Option<bool>,
    sampled: bool,
}

impl CanvasPointerWire {
    pub(crate) fn new(inner: Rect, viewport: &Viewport, x: f32, y: f32) -> Self {
        let (world_x, world_y) = viewport.screen_to_world(x, y, inner);
        Self { screen_x: x - inner.x, screen_y: y - inner.y, world_x, world_y, width: inner.w, height: inner.h, button: None, modifiers: SceneModifiers::default(), cancelled: None, sampled: false }
    }

    pub(crate) fn button(mut self, button: i16, modifiers: SceneModifiers) -> Self {
        self.button = Some(button);
        self.modifiers = modifiers;
        self
    }

    fn release(mut self, modifiers: SceneModifiers, cancelled: bool) -> Self {
        self.modifiers = modifiers;
        self.cancelled = Some(cancelled);
        self
    }

    fn sampled(mut self,modifiers: SceneModifiers) -> Self {
        self.modifiers = modifiers;
        self.sampled = true;
        self
    }
}

const CANVAS_POINTER_WIRE_KEYS: &[&str] = &["surfaceId", "x", "y", "width", "height", "shift", "ctrl", "meta", "alt", "extend", "worldX", "worldY", "button", "cancelled", "samples", "worldSamples"];

fn canvas_pointer_action_bytes(scene: &UiComponentSceneNode, action: &str) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    canvas_pointer_action_address_bytes(&scene.controller_id, &scene.surface_id, action)
}

fn canvas_pointer_action_address_bytes(controller_id: &str, surface_id: &str, action: &str) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    let mut parts: Vec<&str> = vec![controller_id, action, surface_id];
    parts.extend_from_slice(CANVAS_POINTER_WIRE_KEYS);
    ui_wgpu::wgpu::checked_action_string_bytes(&parts)
}

fn write_canvas_pointer_action(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, scene: &UiComponentSceneNode, action: &str, wire: &CanvasPointerWire) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    write_canvas_pointer_action_address(batch, &scene.controller_id, &scene.surface_id, action, wire)
}

fn write_canvas_pointer_action_address(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, surface_id: &str, action: &str, wire: &CanvasPointerWire) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = canvas_pointer_action_address_bytes(controller_id, surface_id, action)?;
    batch.action(controller_id, action, bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.number(Some("x"), f64::from(wire.screen_x))?;
        builder.number(Some("y"), f64::from(wire.screen_y))?;
        builder.number(Some("width"), f64::from(wire.width))?;
        builder.number(Some("height"), f64::from(wire.height))?;
        builder.boolean(Some("shift"), wire.modifiers.shift)?;
        builder.boolean(Some("ctrl"), wire.modifiers.ctrl)?;
        builder.boolean(Some("meta"), wire.modifiers.meta)?;
        builder.boolean(Some("alt"), wire.modifiers.alt)?;
        builder.boolean(Some("extend"), wire.modifiers.shift)?;
        builder.number(Some("worldX"), f64::from(wire.world_x))?;
        builder.number(Some("worldY"), f64::from(wire.world_y))?;
        if let Some(button) = wire.button {
            builder.number(Some("button"), f64::from(button))?;
        }
        if let Some(cancelled) = wire.cancelled {
            builder.boolean(Some("cancelled"), cancelled)?;
        }
        if wire.sampled {
            builder.begin_array(Some("samples"))?;
            builder.begin_array(None)?;
            builder.number(None, f64::from(wire.screen_x))?;
            builder.number(None, f64::from(wire.screen_y))?;
            builder.end_container()?;
            builder.end_container()?;
            builder.begin_array(Some("worldSamples"))?;
            builder.begin_array(None)?;
            builder.number(None, f64::from(wire.world_x))?;
            builder.number(None, f64::from(wire.world_y))?;
            builder.end_container()?;
            builder.end_container()?;
        }
        builder.end_container()
    })
}

const CANVAS_DOUBLE_CLICK_KEYS: &[&str] = &["surfaceId", "x", "y", "width", "height"];

fn canvas_double_click_action_bytes(controller_id: &str, surface_id: &str) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
    let mut parts = vec![controller_id, "canvasDoubleClick", surface_id];
    parts.extend_from_slice(CANVAS_DOUBLE_CLICK_KEYS);
    ui_wgpu::wgpu::checked_action_string_bytes(&parts)
}

fn write_canvas_double_click_action(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, surface_id: &str, inner: Rect, x: f32, y: f32) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = canvas_double_click_action_bytes(controller_id, surface_id)?;
    batch.action(controller_id, "canvasDoubleClick", bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.number(Some("x"), f64::from(x - inner.x))?;
        builder.number(Some("y"), f64::from(y - inner.y))?;
        builder.number(Some("width"), f64::from(inner.w))?;
        builder.number(Some("height"), f64::from(inner.h))?;
        builder.end_container()
    })
}

pub fn canvas_state_snapshot(surface_id: &str) -> (Viewport, bool) {
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).map(|state| (state.viewport, matches!(state.drag.as_ref().map(|drag| &drag.mode), Some(SceneDragMode::PanViewport)))).unwrap_or((Viewport::default(), false)))
}

/// 🔭️ The live camera `(x, y, zoom)` of one Canvas2d host as its input path reads it — the view the gumball twin
/// (`📐️Canvas2dHost/🎯️targets/🧊️wgpu`) measures a gesture in.
fn canvas_camera(surface_id: &str) -> (f64, f64, f64) {
    let (viewport, _) = canvas_state_snapshot(surface_id);
    (f64::from(viewport.x), f64::from(viewport.y), f64::from(viewport.zoom))
}

fn canvas_gesture_matches(active: &CanvasGesture, pointer_id: ui_render::PointerId, window_id: &str, host_id: &str, document_generation: u64) -> bool {
    active.pointer_id == pointer_id && active.window_id == window_id && active.host_id == host_id && active.document_generation == document_generation
}

fn cancel_canvas_pointer_gesture_into(pointer_id: Option<ui_render::PointerId>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if semio_framework_os_renderer_wgpu_canvas2d_host::cancel_into(pointer_id, "captureLost", input)? {
        return Ok(true);
    }
    let Some(active) = CANVAS_GESTURE.with(|cell| {
        let gestures = cell.borrow();
        match pointer_id {
            Some(pointer_id) => gestures.get(pointer_id),
            None => gestures.slots.iter().flatten().next(),
        }
        .cloned()
    }) else {
        return Ok(false);
    };
    if crate::element_contracts::ui_document_close_pending_for(&active.window_id) {
        CANVAS_GESTURE.with(|cell| cell.borrow_mut().take(active.pointer_id));
        return Ok(true);
    }
    if active.kind == CanvasGestureKind::Pan {
        let (retired, pan_remains) = CANVAS_GESTURE.with(|cell| {
            let mut current = cell.borrow_mut();
            let retired = current
                .get(active.pointer_id)
                .is_some_and(|candidate| candidate.window_id == active.window_id && candidate.host_id == active.host_id && candidate.document_generation == active.document_generation && candidate.kind == CanvasGestureKind::Pan);
            if retired {
                current.take(active.pointer_id);
            }
            let pan_remains =
                current.slots.iter().flatten().any(|candidate| candidate.window_id == active.window_id && candidate.host_id == active.host_id && candidate.document_generation == active.document_generation && candidate.kind == CanvasGestureKind::Pan);
            (retired, pan_remains)
        });
        if !retired {
            return Ok(false);
        }
        if !pan_remains {
            mutate_scene_state(&active.host_id, |state| {
                state.pointer_was_down = false;
                state.paint_stroke_active = false;
                state.drag = None;
                state.canvas_click = None;
            });
        }
        return Ok(true);
    }
    let wire = CanvasPointerWire::new(active.rect, &active.viewport, active.last_x, active.last_y).release(SceneModifiers::default(), true);
    let bytes = canvas_pointer_action_address_bytes(&active.controller_id, &active.surface_id, "canvasPointerUp")?;
    let mut batch = input.reserve_actions(1, bytes)?;
    write_canvas_pointer_action_address(&mut batch, &active.controller_id, &active.surface_id, "canvasPointerUp", &wire)?;
    batch.publish_with(|| {
        CANVAS_GESTURE.with(|cell| {
            let mut current = cell.borrow_mut();
            if current.get(active.pointer_id).is_some_and(|candidate| candidate.window_id == active.window_id && candidate.host_id == active.host_id && candidate.document_generation == active.document_generation) {
                current.take(active.pointer_id);
            }
        });
        mutate_scene_state(&active.host_id, |state| {
            state.pointer_was_down = false;
            state.paint_stroke_active = false;
            state.drag = None;
            state.canvas_click = None;
        });
    })?;
    Ok(true)
}

pub fn cancel_canvas_pointer_gesture(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    match cancel_canvas_pointer_gesture_into(None, input) {
        Ok(cancelled) => cancelled,
        Err(fault) => {
            input.record_action_fault(fault);
            false
        }
    }
}

pub fn cancel_canvas_pointer_gesture_for(pointer_id: ui_render::PointerId, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    match cancel_canvas_pointer_gesture_into(Some(pointer_id), input) {
        Ok(cancelled) => cancelled,
        Err(fault) => {
            input.record_action_fault(fault);
            false
        }
    }
}

/// 🧹️ Releases one Canvas input owner after the exact window token has entered close.
pub fn retire_canvas_window_step(window_id: &str) -> bool {
    if semio_framework_os_renderer_wgpu_canvas2d_host::retire_window(window_id) {
        return true;
    }
    if CANVAS_GESTURE.with(|cell| {
        let mut gestures = cell.borrow_mut();
        let Some(slot) = gestures.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|active| active.window_id == window_id)) else { return false };
        slot.take();
        true
    }) {
        return true;
    }
    CANVAS_CATALOGUE_HOVER.with(|cell| {
        let mut hover = cell.borrow_mut();
        if hover.as_ref().is_some_and(|active| active.window_id == window_id) {
            hover.take();
            return true;
        }
        false
    })
}

pub fn request_canvas_pointer_gesture_cancel_for_window(window_id: &str) {
    semio_framework_os_renderer_wgpu_scene_state::request_cancel(|window, _, _| window == window_id);
    CANVAS_GESTURE.with(|cell| {
        for active in cell.borrow_mut().slots.iter_mut().flatten().filter(|active| active.window_id == window_id) {
            active.cancellation_requested = true;
        }
    });
    CANVAS_CATALOGUE_HOVER.with(|cell| {
        if let Some(active) = cell.borrow_mut().as_mut().filter(|active| active.window_id == window_id) {
            active.cancellation_requested = true;
        }
    });
}

pub fn request_stale_canvas_authority_cancel(mut generation_is_live: impl FnMut(&str, u64) -> bool) {
    semio_framework_os_renderer_wgpu_scene_state::request_cancel(|window, _, generation| !generation_is_live(window, generation));
    CANVAS_GESTURE.with(|cell| {
        for active in cell.borrow_mut().slots.iter_mut().flatten() {
            if !generation_is_live(&active.window_id, active.document_generation) {
                active.cancellation_requested = true;
            }
        }
    });
    CANVAS_CATALOGUE_HOVER.with(|cell| {
        if let Some(active) = cell.borrow_mut().as_mut().filter(|active| !generation_is_live(&active.window_id, active.document_generation)) {
            active.cancellation_requested = true;
        }
    });
}

pub fn canvas_pointer_terminal_step(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    if let Some(pointer_id) = semio_framework_os_renderer_wgpu_canvas2d_host::cancel_due() {
        return cancel_canvas_pointer_gesture_for(pointer_id, input);
    }
    let gesture_due = CANVAS_GESTURE.with(|cell| cell.borrow().slots.iter().flatten().find(|active| active.cancellation_requested).map(|active| active.pointer_id));
    if let Some(pointer_id) = gesture_due {
        return cancel_canvas_pointer_gesture_for(pointer_id, input);
    }
    let hover_due = CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().as_ref().is_some_and(|active| active.cancellation_requested));
    if hover_due {
        return cancel_canvas_catalogue_drag(input);
    }
    false
}

pub(crate) fn cancel_stale_canvas_authority(window_id: &str, host_id: &str, document_generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    semio_framework_os_renderer_wgpu_canvas2d_host::cancel_stale_into(window_id, host_id, document_generation, input)?;
    let stale_gesture = CANVAS_GESTURE.with(|cell| cell.borrow().slots.iter().flatten().find(|active| active.window_id == window_id && active.host_id == host_id && active.document_generation != document_generation).map(|active| active.pointer_id));
    if let Some(pointer_id) = stale_gesture {
        cancel_canvas_pointer_gesture_into(Some(pointer_id), input)?;
    }
    let stale_hover = CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().as_ref().is_some_and(|active| active.window_id == window_id && active.host_id == host_id && active.document_generation != document_generation));
    if stale_hover {
        cancel_canvas_catalogue_drag_into(input)?;
    }
    Ok(())
}

pub fn canvas_pointer_move_into(
    scene: &UiComponentSceneNode,
    inner: Rect,
    pointer_id: ui_render::PointerId,
    window_id: &str,
    document_generation: u64,
    x: f32,
    y: f32,
    modifiers: SceneModifiers,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    cancel_stale_canvas_authority(window_id, &scene.host_id, document_generation, input)?;
    if let Some(handled) = semio_framework_os_renderer_wgpu_canvas2d_host::pointer_move_into(scene, inner, pointer_id, window_id, document_generation, x, y, canvas_camera(&scene.host_id), input)? {
        return Ok(handled);
    }
    let active = CANVAS_GESTURE.with(|cell| cell.borrow().get(pointer_id).filter(|active| canvas_gesture_matches(active, pointer_id, window_id, &scene.host_id, document_generation)).cloned());
    let down = active.is_some();
    if !inner.contains(x, y) {
        if down {
            cancel_canvas_pointer_gesture_into(Some(pointer_id), input)?;
        }
        return Ok(false);
    }
    let (viewport, _) = canvas_state_snapshot(&scene.host_id);
    let (drag_dx, drag_dy) = active.as_ref().map(|active| (x - active.last_x, y - active.last_y)).unwrap_or((0.0, 0.0));
    let pan = active.as_ref().is_some_and(|active| active.kind == CanvasGestureKind::Pan);
    let action = down && !pan;
    let wire = CanvasPointerWire::new(inner, &viewport, x, y).sampled(modifiers);
    let mut batch = action
        .then(|| {
            let bytes = canvas_pointer_action_bytes(scene, "canvasPointerMove")?;
            input.reserve_actions(1, bytes)
        })
        .transpose()?;
    if pan {
        mutate_scene_state(&scene.host_id, |state| {
            state.viewport.x -= drag_dx / viewport.zoom.max(0.01);
            state.viewport.y -= drag_dy / viewport.zoom.max(0.01);
            state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
        });
        schedule_scene_camera_dispatch(&scene.host_id, &scene.surface_id);
    }
    if let Some(batch) = batch.as_mut() {
        write_canvas_pointer_action(batch, scene, "canvasPointerMove", &wire)?;
    }
    if let Some(batch) = batch {
        batch.publish_with(|| {
            CANVAS_GESTURE.with(|cell| {
                if let Some(active) = cell.borrow_mut().get_mut(pointer_id).filter(|active| canvas_gesture_matches(active, pointer_id, window_id, &scene.host_id, document_generation)) {
                    active.rect = inner;
                    active.viewport = viewport;
                    active.last_x = x;
                    active.last_y = y;
                }
            });
        })?;
    } else if down {
        CANVAS_GESTURE.with(|cell| {
            if let Some(active) = cell.borrow_mut().get_mut(pointer_id).filter(|active| canvas_gesture_matches(active, pointer_id, window_id, &scene.host_id, document_generation)) {
                active.rect = inner;
                active.viewport = viewport;
                active.last_x = x;
                active.last_y = y;
            }
        });
    }
    Ok(true)
}

pub fn canvas_pointer_button_into(
    scene: &UiComponentSceneNode,
    inner: Rect,
    pointer_id: ui_render::PointerId,
    window_id: &str,
    document_generation: u64,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    modifiers: SceneModifiers,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    cancel_stale_canvas_authority(window_id, &scene.host_id, document_generation, input)?;
    if down && !inner.contains(x, y) {
        return Ok(false);
    }
    if down && CANVAS_GESTURE.with(|cell| cell.borrow().get(pointer_id).is_some()) {
        cancel_canvas_pointer_gesture_into(Some(pointer_id), input)?;
    }
    if let Some(handled) = semio_framework_os_renderer_wgpu_canvas2d_host::pointer_button_into(scene, inner, pointer_id, window_id, document_generation, x, y, down, button, canvas_camera(&scene.host_id), input)? {
        return Ok(handled);
    }
    if down && !CANVAS_GESTURE.with(|cell| cell.borrow().has_capacity_for(pointer_id)) {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    let active = CANVAS_GESTURE.with(|cell| cell.borrow().get(pointer_id).filter(|active| canvas_gesture_matches(active, pointer_id, window_id, &scene.host_id, document_generation)).cloned());
    if !down && active.is_none() {
        return Ok(false);
    }
    let (viewport, _) = canvas_state_snapshot(&scene.host_id);
    let pan = if down { button == 1 || button == 0 && canvas_active_utility(scene).as_deref() == Some("transformMove") } else { active.as_ref().is_some_and(|active| active.kind == CanvasGestureKind::Pan) };
    if pan {
        mutate_scene_state(&scene.host_id, |state| {
            state.pointer_was_down = down;
            state.paint_stroke_active = false;
            state.drag = down.then_some(SceneDrag { mode: SceneDragMode::PanViewport });
            if !down {
                state.canvas_click = None;
            }
        });
        CANVAS_GESTURE.with(|cell| {
            if down {
                let admitted = cell.borrow_mut().insert(CanvasGesture {
                    pointer_id,
                    kind: CanvasGestureKind::Pan,
                    window_id: window_id.to_string(),
                    document_generation,
                    host_id: scene.host_id.clone(),
                    surface_id: scene.surface_id.clone(),
                    controller_id: scene.controller_id.clone(),
                    rect: inner,
                    viewport,
                    last_x: x,
                    last_y: y,
                    button,
                    cancellation_requested: false,
                });
                debug_assert!(admitted);
            } else if cell.borrow().get(pointer_id).is_some_and(|active| canvas_gesture_matches(active, pointer_id, window_id, &scene.host_id, document_generation)) {
                cell.borrow_mut().take(pointer_id);
            }
        });
        return Ok(true);
    }
    let cancelled = !down && !inner.contains(x, y);
    let (wire_x, wire_y) = active.as_ref().filter(|_| cancelled).map(|active| (active.last_x, active.last_y)).unwrap_or((x, y));
    let base = CanvasPointerWire::new(inner, &viewport, wire_x, wire_y);
    let wire = if down { base.button(button, modifiers) } else { base.release(if cancelled { SceneModifiers::default() } else { modifiers }, cancelled) };
    let pointer_action = if down { "canvasPointerDown" } else { "canvasPointerUp" };
    let pointer_bytes = canvas_pointer_action_bytes(scene, pointer_action)?;
    let now_ms = semio_framework_job::default_now_ms().expect("Canvas input requires the installed monotonic host clock") as f64;
    let prior_click = SCENE_STATE.with(|cell| cell.borrow().get(&scene.host_id).and_then(|state| state.canvas_click));
    let double_click = !down
        && !cancelled
        && button == 0
        && active.as_ref().is_some_and(|active| active.button == button)
        && prior_click.is_some_and(|prior| prior.button == button && now_ms - prior.at_ms <= 400.0 && (prior.x - x).abs() <= 4.0 && (prior.y - y).abs() <= 4.0);
    let double_bytes = double_click.then(|| canvas_double_click_action_bytes(&scene.controller_id, &scene.surface_id)).transpose()?.unwrap_or(0);
    let mut batch = input.reserve_actions(1 + usize::from(double_click), pointer_bytes.checked_add(double_bytes).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?)?;
    write_canvas_pointer_action(&mut batch, scene, pointer_action, &wire)?;
    if double_click {
        write_canvas_double_click_action(&mut batch, &scene.controller_id, &scene.surface_id, inner, x, y)?;
    }
    batch.publish_with(|| {
        mutate_scene_state(&scene.host_id, |state| {
            state.pointer_was_down = down;
            if !down {
                state.paint_stroke_active = false;
                state.drag = None;
                state.canvas_click = if cancelled || double_click || button != 0 { None } else { Some(CanvasClick { x, y, button, at_ms: now_ms }) };
            }
        });
        CANVAS_GESTURE.with(|cell| {
            if down {
                let admitted = cell.borrow_mut().insert(CanvasGesture {
                    pointer_id,
                    kind: CanvasGestureKind::Document,
                    window_id: window_id.to_string(),
                    document_generation,
                    host_id: scene.host_id.clone(),
                    surface_id: scene.surface_id.clone(),
                    controller_id: scene.controller_id.clone(),
                    rect: inner,
                    viewport,
                    last_x: x,
                    last_y: y,
                    button,
                    cancellation_requested: false,
                });
                debug_assert!(admitted);
            } else if cell.borrow().get(pointer_id).is_some_and(|active| canvas_gesture_matches(active, pointer_id, window_id, &scene.host_id, document_generation)) {
                cell.borrow_mut().take(pointer_id);
            }
        });
    })?;
    Ok(true)
}

pub const CANVAS_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
const CANVAS_CATALOGUE_TYPE_CAPACITY: usize = 32;
const CANVAS_CATALOGUE_PAYLOAD_BYTE_CAPACITY: usize = 64 * 1024;
const CANVAS_DRAG_OVER_THROTTLE_MS: f64 = 50.0;
const CANVAS_DRAG_OVER_THROTTLE_DISTANCE: f32 = 4.0;

fn canvas_addressed_action(controller_id: &str, surface_id: &str, action: &str, fields: semio_framework::DslValue) -> ActionDescriptor {
    let mut args = vec![("surfaceId".to_string(), semio_framework::DslValue::String(surface_id.to_string()))];
    if let semio_framework::DslValue::Object(fields) = fields {
        args.extend(fields.into_iter().filter(|(key, _)| key != "surfaceId"));
    }
    ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: Some(semio_framework::DslValue::Object(args)) }
}

fn canvas_catalogue_leave_action(hover: &CanvasCatalogueHover) -> ActionDescriptor {
    canvas_addressed_action(&hover.controller_id, &hover.surface_id, "canvasDragLeave", semio_framework::dsl_value!({}))
}

pub fn canvas_catalogue_drag_over_into(
    scene: &UiComponentSceneNode,
    inner: Rect,
    window_id: &str,
    document_generation: u64,
    x: f32,
    y: f32,
    types: &[String],
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if !inner.contains(x, y) || !types.iter().any(|mime| mime == CANVAS_CATALOGUE_DRAG_MIME) {
        cancel_canvas_catalogue_drag_into(input)?;
        return Ok(false);
    }
    if types.len() > CANVAS_CATALOGUE_TYPE_CAPACITY {
        return Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
    }
    ui_wgpu::wgpu::checked_action_string_bytes(&types.iter().map(String::as_str).collect::<Vec<_>>())?;
    let now_ms = semio_framework_job::default_now_ms().expect("Canvas input requires the installed monotonic host clock") as f64;
    let prior = CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().clone());
    let same_target = prior.as_ref().is_some_and(|active| active.window_id == window_id && active.document_generation == document_generation && active.host_id == scene.host_id);
    if same_target
        && prior.as_ref().is_some_and(|active| now_ms - active.last_dispatch_ms < CANVAS_DRAG_OVER_THROTTLE_MS && (x - active.last_x).abs() < CANVAS_DRAG_OVER_THROTTLE_DISTANCE && (y - active.last_y).abs() < CANVAS_DRAG_OVER_THROTTLE_DISTANCE)
    {
        return Ok(true);
    }
    let mut actions = Vec::with_capacity(2);
    if let Some(active) = prior.as_ref().filter(|_| !same_target) {
        actions.push(canvas_catalogue_leave_action(active));
    }
    actions.push(scene_action(
        scene,
        "canvasDragOver",
        semio_framework::dsl_value!({
            "surfaceId": scene.surface_id,
            "x": x - inner.x,
            "y": y - inner.y,
            "width": inner.w,
            "height": inner.h,
            "types": types,
        }),
    ));
    write_scene_action_batch(input, &actions, || {
        CANVAS_CATALOGUE_HOVER.with(|cell| {
            *cell.borrow_mut() = Some(CanvasCatalogueHover {
                window_id: window_id.to_string(),
                document_generation,
                host_id: scene.host_id.clone(),
                surface_id: scene.surface_id.clone(),
                controller_id: scene.controller_id.clone(),
                last_x: x,
                last_y: y,
                last_dispatch_ms: now_ms,
                cancellation_requested: false,
            });
        });
    })?;
    Ok(true)
}

fn cancel_canvas_catalogue_drag_into(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(active) = CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().clone()) else { return Ok(false) };
    if crate::element_contracts::ui_document_close_pending_for(&active.window_id) || scene_host_retiring(&active.host_id) {
        CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow_mut().take());
        return Ok(true);
    }
    write_scene_action_batch(input, &[canvas_catalogue_leave_action(&active)], || {
        CANVAS_CATALOGUE_HOVER.with(|cell| {
            let mut current = cell.borrow_mut();
            if current.as_ref().is_some_and(|candidate| candidate.window_id == active.window_id && candidate.host_id == active.host_id && candidate.document_generation == active.document_generation) {
                *current = None;
            }
        });
    })?;
    Ok(true)
}

pub fn cancel_canvas_catalogue_drag(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    match cancel_canvas_catalogue_drag_into(input) {
        Ok(cancelled) => cancelled,
        Err(fault) => {
            input.record_action_fault(fault);
            false
        }
    }
}

pub fn canvas_catalogue_drop_into(
    scene: &UiComponentSceneNode,
    inner: Rect,
    window_id: &str,
    document_generation: u64,
    x: f32,
    y: f32,
    raw_payload: &str,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    if raw_payload.is_empty() || raw_payload.len() > CANVAS_CATALOGUE_PAYLOAD_BYTE_CAPACITY || !inner.contains(x, y) {
        cancel_canvas_catalogue_drag_into(input)?;
        return Ok(false);
    }
    cancel_stale_canvas_authority(window_id, &scene.host_id, document_generation, input)?;
    let leave = CANVAS_CATALOGUE_HOVER.with(|cell| cell.borrow().clone()).as_ref().map(canvas_catalogue_leave_action).unwrap_or_else(|| canvas_addressed_action(&scene.controller_id, &scene.surface_id, "canvasDragLeave", semio_framework::dsl_value!({})));
    let drop = scene_action(
        scene,
        "canvasDrop",
        semio_framework::dsl_value!({
            "surfaceId": scene.surface_id,
            "x": x - inner.x,
            "y": y - inner.y,
            "width": inner.w,
            "height": inner.h,
            "dragData": raw_payload,
        }),
    );
    write_scene_action_batch(input, &[leave, drop], || {
        CANVAS_CATALOGUE_HOVER.with(|cell| *cell.borrow_mut() = None);
    })?;
    Ok(true)
}

pub fn cancel_canvas_interactions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
    let gesture = cancel_canvas_pointer_gesture(input);
    let catalogue = cancel_canvas_catalogue_drag(input);
    gesture || catalogue
}

pub fn canvas_wheel_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32) -> bool {
    if !inner.contains(x, y) {
        return false;
    }
    mutate_scene_state(&scene.host_id, |state| {
        let before = Viewport { zoom: if state.viewport.zoom == 0.0 || state.viewport.zoom.is_nan() { 1.0 } else { state.viewport.zoom }, ..state.viewport };
        let anchor = before.screen_to_world(x, y, inner);
        let zoom = (before.zoom * if delta < 0.0 { 1.1 } else { 0.9 }).clamp(0.05, 32.0);
        state.viewport = Viewport { x: anchor.0 - (x - inner.x - inner.w * 0.5) / zoom, y: anchor.1 - (y - inner.y - inner.h * 0.5) / zoom, zoom };
        state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
    });
    schedule_scene_camera_dispatch(&scene.host_id, &scene.surface_id);
    true
}

pub fn passive_scene_wheel(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, delta: f32) -> bool {
    if !bounds.contains(x, y) {
        return false;
    }
    match scene.component_kind {
        SurfaceKind::Table => set_scroll_offset(&scene.host_id, "body", scroll_offset(&scene.host_id, "body") + delta * 0.5),
        SurfaceKind::VirtualFileSystem => set_scroll_offset(&scene.host_id, "vfs", scroll_offset(&scene.host_id, "vfs") + delta * 0.5),
        SurfaceKind::GraphTimeline => set_scroll_offset(&scene.host_id, "history", scroll_offset(&scene.host_id, "history") + delta * 0.5),
        SurfaceKind::DiffView => set_scroll_offset(&scene.host_id, "diff", scroll_offset(&scene.host_id, "diff") + delta * 0.5),
        SurfaceKind::EventFeed => set_scroll_offset(&scene.host_id, "feed", scroll_offset(&scene.host_id, "feed") + delta * 0.5),
        SurfaceKind::BlockList => set_scroll_offset(&scene.host_id, "blockList", scroll_offset(&scene.host_id, "blockList") + delta * 0.5),
        _ => return false,
    }
    true
}

/// 📏️ How far a press must travel before it counts as a drag rather than a click — the same 5 px the
/// shell's own tree drag uses, and the reason a plain row click still selects instead of transferring.
const SCENE_TRANSFER_DRAG_SLOP_SQUARED: f32 = 25.0;
const SCENE_TRANSFER_PAYLOAD_BYTE_CAPACITY: usize = 64 * 1024;

fn scene_list_transfer_start(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme, driver_drag: UiDriverDrag) -> Option<SceneListTransferStart> {
    match scene.component_kind {
        SurfaceKind::Table => table_transfer_start(scene, bounds, x, y, theme, driver_drag),
        SurfaceKind::BlockList => block_list_transfer_start(scene, bounds, x, y, theme, driver_drag),
        _ => None,
    }
}

fn arm_scene_list_transfer(pointer_id: ui_render::PointerId, window_id: &str, scene: &UiComponentSceneNode, document_generation: u64, driver_drag: UiDriverDrag, start: SceneListTransferStart, x: f32, y: f32) -> bool {
    if window_id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY || scene.surface_id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
        return false;
    }
    let payload_bytes = match &start.source {
        SceneListTransferSource::TableRow { row_id, mime, payload } => row_id.len().saturating_add(mime.len()).saturating_add(payload.len()),
        SceneListTransferSource::BlockListStep { step_id, .. } => step_id.len(),
        SceneListTransferSource::BlockListBlock { step_id, block_id, .. } => step_id.len().saturating_add(block_id.len()),
        SceneListTransferSource::BlockListPalette { kind, mime, payload } => kind.len().saturating_add(mime.len()).saturating_add(payload.len()),
    };
    if payload_bytes > SCENE_TRANSFER_PAYLOAD_BYTE_CAPACITY {
        return false;
    }
    SCENE_LIST_TRANSFER.with(|cell| {
        let mut authority = cell.borrow_mut();
        let Some(next_generation) = authority.next_generation.checked_add(1) else {
            authority.active = None;
            return false;
        };
        let authority_generation = authority.next_generation;
        authority.next_generation = next_generation;
        authority.active = Some(SceneListTransferSession {
            pointer_id,
            authority_generation,
            source_window_id: window_id.to_string(),
            source_host_id: scene.host_id.clone(),
            source_document_generation: document_generation,
            driver_drag,
            source: start.source,
            source_rect: start.rect,
            start_x: x,
            start_y: y,
            promoted: false,
        });
        true
    })
}

pub fn cancel_scene_list_transfer() -> bool {
    let retired = SCENE_LIST_TRANSFER.with(|cell| cell.borrow_mut().active.take());
    if let Some(session) = retired.as_ref() {
        mutate_scene_state(&session.source_host_id, |state| state.pointer_was_down = false);
    }
    retired.is_some()
}

pub fn cancel_scene_list_transfer_for_pointer(pointer_id: ui_render::PointerId) -> bool {
    let retired = SCENE_LIST_TRANSFER.with(|cell| {
        let mut authority = cell.borrow_mut();
        if authority.active.as_ref().is_some_and(|active| active.pointer_id == pointer_id) {
            authority.active.take()
        } else {
            None
        }
    });
    if let Some(session) = retired.as_ref() {
        mutate_scene_state(&session.source_host_id, |state| state.pointer_was_down = false);
    }
    retired.is_some()
}

pub fn cancel_scene_list_transfer_for_window(window_id: &str) -> bool {
    let retired = SCENE_LIST_TRANSFER.with(|cell| {
        let mut authority = cell.borrow_mut();
        if authority.active.as_ref().is_some_and(|active| active.source_window_id == window_id) {
            authority.active.take()
        } else {
            None
        }
    });
    if let Some(session) = retired.as_ref() {
        mutate_scene_state(&session.source_host_id, |state| state.pointer_was_down = false);
    }
    retired.is_some()
}

pub(crate) fn cancel_scene_list_transfer_if_stale(window_id: &str, surface_id: &str, document_generation: u64, driver_drag: UiDriverDrag) -> bool {
    let retired = SCENE_LIST_TRANSFER.with(|cell| {
        let mut authority = cell.borrow_mut();
        let stale = authority.active.as_ref().is_some_and(|active| active.driver_drag != driver_drag || (active.source_window_id == window_id && active.source_host_id == surface_id && active.source_document_generation != document_generation));
        if stale {
            authority.active.take()
        } else {
            None
        }
    });
    if let Some(session) = retired.as_ref() {
        mutate_scene_state(&session.source_host_id, |state| state.pointer_was_down = false);
    }
    retired.is_some()
}

fn update_scene_list_transfer(pointer_id: ui_render::PointerId, window_id: &str, surface_id: &str, document_generation: u64, driver_drag: UiDriverDrag, x: f32, y: f32) {
    cancel_scene_list_transfer_if_stale(window_id, surface_id, document_generation, driver_drag);
    SCENE_LIST_TRANSFER.with(|cell| {
        let mut authority = cell.borrow_mut();
        let Some(active) = authority.active.as_mut() else { return };
        if active.pointer_id != pointer_id || active.source_window_id != window_id || active.source_host_id != surface_id {
            return;
        }
        let dx = x - active.start_x;
        let dy = y - active.start_y;
        if dx * dx + dy * dy > SCENE_TRANSFER_DRAG_SLOP_SQUARED {
            active.promoted = true;
        }
    });
}

fn scene_list_transfer_drop_action(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, window_id: &str, theme: &Theme, session: &SceneListTransferSession) -> Option<ActionDescriptor> {
    if !bounds.contains(x, y) {
        return None;
    }
    match (&session.source, scene.component_kind) {
        (SceneListTransferSource::TableRow { mime, payload, .. }, SurfaceKind::Table) if mime.starts_with("application/x-semio-") => {
            let table = scene.table.as_ref()?;
            let descriptor: ActionDescriptor = serde_json::from_str(table.drop_action_json.as_deref()?).ok()?;
            let patch: Value = serde_json::from_str(payload).ok()?;
            patch.is_object().then(|| merge_action_args(&descriptor, patch))
        }
        (_, SurfaceKind::BlockList) => block_list_transfer_drop_action(scene, bounds, x, y, window_id, theme, session),
        _ => None,
    }
}

fn release_scene_list_transfer(pointer_id: ui_render::PointerId, scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, window_id: &str, document_generation: u64, driver_drag: UiDriverDrag, theme: &Theme) -> Option<SceneListTransferRelease> {
    cancel_scene_list_transfer_if_stale(window_id, &scene.host_id, document_generation, driver_drag);
    let session = SCENE_LIST_TRANSFER.with(|cell| {
        let mut authority = cell.borrow_mut();
        authority.active.as_ref().filter(|active| active.pointer_id == pointer_id)?;
        authority.active.take()
    })?;
    mutate_scene_state(&session.source_host_id, |state| state.pointer_was_down = false);
    let suppress_click = session.promoted || session.driver_drag == UiDriverDrag::Handle;
    if !session.promoted || session.driver_drag != driver_drag || session.authority_generation == 0 {
        return Some(SceneListTransferRelease { suppress_click, action: None });
    }
    let action = scene_list_transfer_drop_action(scene, bounds, x, y, window_id, theme, &session);
    Some(SceneListTransferRelease { suppress_click: true, action })
}
//#endregion SceneRowTransfer

/** 🖱️ The generic (non-bespoke, non-Canvas2d/Ink/TextEditor/Paint2d) per-event press route.
 * Every list kind resolves the press against its own painted row
 * geometry and publishes the action React's matching host dispatches — a row select, a header sort, a
 * stepper delta, a feed activation, a checkpoint checkout, a block add/remove/move, a virtual file
 * system double-click. The action is published on RELEASE, mirroring a DOM `click`. */
pub fn passive_scene_pointer_button(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    pointer_id: ui_render::PointerId,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    modifiers: SceneModifiers,
    window_id: &str,
    document_generation: u64,
    driver_drag: UiDriverDrag,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    cancel_scene_list_transfer_if_stale(window_id, &scene.host_id, document_generation, driver_drag);
    if SCENE_LIST_TRANSFER.with(|cell| cell.borrow().active.as_ref().is_some_and(|active| active.pointer_id != pointer_id)) {
        return Ok(false);
    }
    if !bounds.contains(x, y) {
        if !down {
            mutate_scene_state(&scene.host_id, |state| {
                state.pointer_was_down = false;
                state.drag = None;
            });
        }
        return Ok(false);
    }
    if !scene_kind_is_list(scene.component_kind) {
        return Ok(false);
    }
    if down {
        cancel_scene_list_transfer_for_pointer(pointer_id);
        let source = (button == 0).then(|| scene_list_transfer_start(scene, bounds, x, y, &scene_input_theme(), driver_drag)).flatten();
        if let Some(source) = source {
            arm_scene_list_transfer(pointer_id, window_id, scene, document_generation, driver_drag, source, x, y);
        }
        mutate_scene_state(&scene.host_id, |state| {
            state.pointer_was_down = true;
        });
        return Ok(true);
    }
    let pointer_was_down = scene_state(&scene.host_id).pointer_was_down;
    mutate_scene_state(&scene.host_id, |state| {
        state.pointer_was_down = false;
    });
    if button != 0 {
        return Ok(true);
    }
    if let Some(release) = release_scene_list_transfer(pointer_id, scene, bounds, x, y, window_id, document_generation, driver_drag, &scene_input_theme()) {
        if let Some(action) = release.action {
            write_scene_action(input, &action)?;
        }
        if release.suppress_click {
            return Ok(true);
        }
    }
    if !pointer_was_down {
        return Ok(true);
    }
    if let Some(action) = scene_double_click_action(scene, bounds, x, y) {
        write_scene_action(input, &action)?;
        return Ok(true);
    }
    let Some(hit) = scene_list_hit(scene, bounds, x, y, &scene_input_theme(), true, modifiers, driver_drag) else {
        return Ok(true);
    };
    if let Some(row_id) = hit.toggle_expanded {
        toggle_vfs_row_expanded(&scene.host_id, &row_id);
        return Ok(true);
    }
    if let Some(action) = hit.action {
        write_scene_action(input, &action)?;
    }
    Ok(true)
}

/** 🖱️ The generic per-event move route: every list kind's own hover authority
 * (`SceneSurfaceState::hovered_control_id`), which the next paint reads back to highlight the row
 * under the pointer. */
pub fn passive_scene_pointer_move(scene: &UiComponentSceneNode, bounds: Rect, pointer_id: ui_render::PointerId, x: f32, y: f32, window_id: &str, document_generation: u64, driver_drag: UiDriverDrag) -> bool {
    update_scene_list_transfer(pointer_id, window_id, &scene.host_id, document_generation, driver_drag, x, y);
    if !bounds.contains(x, y) {
        return false;
    }
    if scene_kind_is_list(scene.component_kind) {
        let hovered = scene_list_hit(scene, bounds, x, y, &scene_input_theme(), false, SceneModifiers::default(), driver_drag).map(|hit| hit.control_id);
        mutate_scene_state(&scene.host_id, |state| {
            state.hovered_control_id = hovered;
            state.last_pointer_pos = (x, y);
        });
        return true;
    }
    false
}

//#region SceneRuntime
/** 🎯️ One resolved pointer target inside a list-like component scene: the control id the paint
 * drew it under (this module's hover authority — see `scene_hovered_control_id`) and the action a
 * primary-button RELEASE on it dispatches, byte-for-byte what the matching React host's own
 * `onClick`/`onSort`/`onSelectionChange` handler sends. A `None` action is a target that only owns
 * hover/selection state: a stepper's read-only centre segment, a disabled reorder button — React
 * stops propagation on exactly those, so the row click underneath must not fire either. */
//#endregion SceneRuntime

/** 📃️ Surface kinds this module paints as a scrolling list of rows and resolves pointer input
 * for by re-deriving that same row geometry. They register no usable retained hit target of their own
 * (`retained_scene_hit` mints ONE `HitKind::Generic` entry per `ComponentScene` leaf, and the chrome
 * walk stages it after this paint's rows, so it shadows every one of them) — so the press that lands
 * on the surface arrives here through `UiCommand::Scene` and is resolved against the payload. */
pub(crate) fn scene_kind_is_list(kind: SurfaceKind) -> bool {
    matches!(kind, SurfaceKind::Table | SurfaceKind::VirtualFileSystem | SurfaceKind::GraphTimeline | SurfaceKind::DiffView | SurfaceKind::EventFeed | SurfaceKind::BlockList)
}

/// 🎯️ Resolves `(x, y)` inside a list surface. `activate` is `false` for a hover sample: the action
/// is then not built at all, which keeps a pointer move free of both the allocation and — for the
/// virtual file system — the selection-anchor mutation `vfs_selection_for_click` performs.
pub fn scene_list_hit(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme, activate: bool, modifiers: SceneModifiers, driver_drag: UiDriverDrag) -> Option<SceneListHit> {
    match scene.component_kind {
        SurfaceKind::Table => table_hit(scene, bounds, x, y, theme, modifiers, driver_drag),
        SurfaceKind::VirtualFileSystem => vfs_hit(scene, bounds, x, y, theme, activate, modifiers),
        SurfaceKind::GraphTimeline => graph_timeline_hit(scene, bounds, x, y, theme),
        SurfaceKind::EventFeed => event_feed_hit(scene, bounds, x, y, theme),
        SurfaceKind::BlockList => block_list_hit(scene, bounds, x, y, theme, driver_drag),
        _ => None,
    }
}

/// 🖱️ Double-click edge detection for the one list kind that has a double-click verb (the virtual
/// file system's `navigateUri`). Records this press as the surface's last click either way, so the
/// NEXT press within the 400 ms window can answer it.
pub fn scene_double_click_action(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Option<ActionDescriptor> {
    let target = hit_double_click_target(scene, inner, x, y)?;
    let now = now_ms();
    let prior = scene_state(&scene.host_id);
    let repeat = prior.last_click_target.as_deref() == Some(target.as_str()) && now - prior.last_click_ms < 400.0;
    let action = repeat.then(|| double_click_action(scene, &target, inner, x, y)).flatten();
    mutate_scene_state(&scene.host_id, |state| {
        state.last_click_target = Some(target);
        state.last_click_ms = now;
    });
    action
}

//#endregion SceneInput
