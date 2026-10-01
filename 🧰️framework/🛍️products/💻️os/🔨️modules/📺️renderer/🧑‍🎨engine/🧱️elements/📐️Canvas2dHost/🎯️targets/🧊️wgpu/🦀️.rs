//! 🧭️ wgpu twin of `📐️Canvas2dHost/🟦️GumballOverlay.tsx`: the Canvas2d transform gumball a plugin arms with its
//! `meta:gumball` layer (`📐️Canvas2dHost/🧬️schema/🔣️gumball-meta`). The handle geometry, the press hit test and the
//! gesture algebra are the overlay's, replayed over the shared corpus `📐️Canvas2dHost/🧫️fixtures/🧫️gumball-dispatch`: a
//! live gumball streams its gesture (`phase` stream/commit/abort) into the app's ONE open tool transaction, a non-live
//! one previews a local ghost and dispatches ONE one-shot pose delta on release (ticket
//! `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, design §5, decision 2026-09-30 21:36). `🎞️Scenes` routes its Canvas2d
//! pointer, cancel and paint seams through [`pointer_button_into`], [`pointer_move_into`], [`cancel_into`] and [`paint`].

use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use ui_wgpu::wgpu::{ActionDescriptor, Rect, Rgba, UiComponentSceneNode};

//#region 📐️Geometry
/// 📏️ Screen pixels from the pivot to the move knobs; the scale knobs sit at three quarters of it on the diagonals.
pub(crate) const HANDLE_LENGTH: f64 = 56.0;
/// ⭕️ Screen radius of the turn ring; the uniform-scale knob sits on it at 0.7 of the radius per axis.
pub(crate) const ROTATE_RADIUS: f64 = 44.0;
/// 🎯️ Screen radius a knob is pressed within.
pub(crate) const HIT_RADIUS: f64 = 10.0;
/// 💍️ Screen half-width of the turn ring's press band.
pub(crate) const RING_HIT_BAND: f64 = 4.0;
const MOVE_EPSILON: f64 = 1e-9;
const TURN_EPSILON: f64 = 1e-6;

/// 🎛️ The handle set a gumball arms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GumballConfig {
    pub move_axes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

/// 🗺️ The affine map from model to layer units per axis: `layer = model × scale + offset`.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub(crate) struct ModelToLayer {
    pub scale: [f64; 2],
    pub offset: [f64; 2],
}

const IDENTITY_MAP: ModelToLayer = ModelToLayer { scale: [1.0, 1.0], offset: [0.0, 0.0] };

/// 🧿️ The `gumball` record of a `meta:gumball` layer.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GumballMeta {
    pub active: bool,
    #[serde(default)]
    pub live_dispatch: bool,
    pub pivot_layer: [f64; 2],
    #[serde(default)]
    pub model_to_layer: Option<ModelToLayer>,
    pub selection_ids: Vec<String>,
    pub config: GumballConfig,
}

#[derive(Deserialize)]
struct MetaLayer {
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    gumball: Option<GumballMeta>,
}

/// 🧩️ The active `meta:gumball` layer of a scene's `layersJson`.
pub(crate) fn parse_meta(layers_json: &str) -> Option<GumballMeta> {
    let layers: Vec<MetaLayer> = serde_json::from_str(layers_json).ok()?;
    layers.into_iter().filter(|layer| layer.role.as_deref() == Some("meta")).find_map(|layer| layer.gumball.filter(|gumball| gumball.active))
}

/// 🔭️ The host view the gumball is painted and measured in: the camera and the canvas's logical size, points local to
/// the canvas's top-left corner (`Canvas2dHost`'s `worldToScreenLogical`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct GumballView {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub width: f64,
    pub height: f64,
}

impl GumballView {
    fn zoom(&self) -> f64 {
        if self.zoom == 0.0 || self.zoom.is_nan() {
            1.0
        } else {
            self.zoom
        }
    }

    fn world_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.camera_x) * self.zoom() + self.width * 0.5, (y - self.camera_y) * self.zoom() + self.height * 0.5)
    }

    fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.width * 0.5) / self.zoom() + self.camera_x, (y - self.height * 0.5) / self.zoom() + self.camera_y)
    }
}

/// ✋️ One gumball handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GumballHandle {
    MoveX,
    MoveY,
    Rotate,
    ScaleX,
    ScaleY,
    ScaleUniform,
}

impl GumballHandle {
    /// 🎚️ The verb the handle drags with.
    pub(crate) fn verb(self) -> &'static str {
        match self {
            Self::MoveX | Self::MoveY => "translateSelection",
            Self::Rotate => "rotateSelection",
            Self::ScaleX | Self::ScaleY | Self::ScaleUniform => "scaleSelection",
        }
    }

    fn enabled(self, config: &GumballConfig) -> bool {
        match self {
            Self::MoveX | Self::MoveY => config.move_axes,
            Self::ScaleX | Self::ScaleY => config.scale_axes,
            Self::ScaleUniform => config.scale_uniform,
            Self::Rotate => config.rotate,
        }
    }
}

/// 📍️ The pivot in local screen pixels under `view`.
pub(crate) fn pivot_screen(pivot_layer: [f64; 2], view: &GumballView) -> (f64, f64) {
    view.world_to_screen(pivot_layer[0], pivot_layer[1])
}

/// 🔘️ Every knob's screen centre about `pivot`, topmost first — the order a press resolves them in.
pub(crate) fn knobs(pivot: (f64, f64)) -> [(GumballHandle, f64, f64); 5] {
    let diagonal = HANDLE_LENGTH * 0.75;
    let uniform = ROTATE_RADIUS * 0.7;
    [
        (GumballHandle::ScaleUniform, pivot.0 + uniform, pivot.1 - uniform),
        (GumballHandle::ScaleY, pivot.0 - diagonal, pivot.1 + diagonal),
        (GumballHandle::ScaleX, pivot.0 + diagonal, pivot.1 + diagonal),
        (GumballHandle::MoveY, pivot.0, pivot.1 - HANDLE_LENGTH),
        (GumballHandle::MoveX, pivot.0 + HANDLE_LENGTH, pivot.1),
    ]
}

/// 👆️ The handle a press at local `(x, y)` grabs: the topmost enabled knob within its hit radius, else the turn ring
/// within its band.
pub(crate) fn handle_at(meta: &GumballMeta, view: &GumballView, x: f64, y: f64) -> Option<GumballHandle> {
    if !meta.active {
        return None;
    }
    let pivot = pivot_screen(meta.pivot_layer, view);
    knobs(pivot)
        .into_iter()
        .find(|(kind, knob_x, knob_y)| kind.enabled(&meta.config) && (x - knob_x).hypot(y - knob_y) <= HIT_RADIUS)
        .map(|(kind, _, _)| kind)
        .or_else(|| (meta.config.rotate && ((x - pivot.0).hypot(y - pivot.1) - ROTATE_RADIUS).abs() <= RING_HIT_BAND).then_some(GumballHandle::Rotate))
}
//#endregion 📐️Geometry

//#region 🧮️Algebra
/// 🎬️ A pose delta in model units: an offset, a turn or a pair of factors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum GumballMotion {
    Translate { dx: f64, dy: f64 },
    Rotate { angle: f64 },
    Scale { sx: f64, sy: f64 },
}

/// 🚦️ Where a dispatch sits in a streamed gesture; absent = a one-shot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GumballPhase {
    Stream,
    Commit,
    Abort(&'static str),
}

/// 📤️ One gumball dispatch: the verb, the pinned ids, the pose delta (absent for an abort) and the phase.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GumballDispatch {
    pub verb: &'static str,
    pub ids: Vec<String>,
    pub motion: Option<GumballMotion>,
    pub phase: Option<GumballPhase>,
}

impl GumballDispatch {
    /// 🧾️ The dispatch's action args, exactly as `🟦️GumballOverlay.tsx` sends them.
    pub(crate) fn args(&self) -> Value {
        let mut args = serde_json::Map::new();
        args.insert("ids".into(), json!(self.ids));
        match self.motion {
            Some(GumballMotion::Translate { dx, dy }) => {
                args.insert("dx".into(), json!(dx));
                args.insert("dy".into(), json!(dy));
                args.insert("dz".into(), json!(0));
            }
            Some(GumballMotion::Rotate { angle }) => {
                args.insert("ax".into(), json!(0));
                args.insert("ay".into(), json!(0));
                args.insert("az".into(), json!(1));
                args.insert("angle".into(), json!(angle));
            }
            Some(GumballMotion::Scale { sx, sy }) => {
                args.insert("sx".into(), json!(sx));
                args.insert("sy".into(), json!(sy));
                args.insert("sz".into(), json!(1));
            }
            None => {}
        }
        match self.phase {
            Some(GumballPhase::Stream) => {
                args.insert("phase".into(), json!("stream"));
            }
            Some(GumballPhase::Commit) => {
                args.insert("phase".into(), json!("commit"));
            }
            Some(GumballPhase::Abort(reason)) => {
                args.insert("phase".into(), json!("abort"));
                args.insert("reason".into(), json!(reason));
            }
            None => {}
        }
        Value::Object(args)
    }
}

/// 🤏️ One gumball gesture in flight: the handle, the local press point, the pivot, the map, the pinned ids, whether it
/// streams, the last cumulative pose delta and whether a stream tick went out.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GumballGesture {
    pub kind: GumballHandle,
    pub start: (f64, f64),
    pub pivot_layer: [f64; 2],
    pub model_to_layer: ModelToLayer,
    pub ids: Vec<String>,
    pub live: bool,
    pub total: Option<GumballMotion>,
    pub streamed: bool,
}

/// ✊️ The gesture a press of `kind` at local `(x, y)` opens.
pub(crate) fn begin(meta: &GumballMeta, kind: GumballHandle, x: f64, y: f64) -> GumballGesture {
    GumballGesture { kind, start: (x, y), pivot_layer: meta.pivot_layer, model_to_layer: meta.model_to_layer.unwrap_or(IDENTITY_MAP), ids: meta.selection_ids.clone(), live: meta.live_dispatch, total: None, streamed: false }
}

fn layer_to_model(map: &ModelToLayer, (x, y): (f64, f64)) -> (f64, f64) {
    ((x - map.offset[0]) / map.scale[0], (y - map.offset[1]) / map.scale[1])
}

fn wrap_angle(angle: f64) -> f64 {
    angle - 2.0 * std::f64::consts::PI * (angle / (2.0 * std::f64::consts::PI)).round()
}

/// 🧮️ The cumulative pose delta from the press to local `(x, y)` — a turn unwrapped against the last total; `None` when
/// it moves nothing.
pub(crate) fn total(gesture: &GumballGesture, view: &GumballView, x: f64, y: f64) -> Option<GumballMotion> {
    let pivot = pivot_screen(gesture.pivot_layer, view);
    let (start_x, start_y) = gesture.start;
    match gesture.kind {
        GumballHandle::MoveX | GumballHandle::MoveY => {
            let before = layer_to_model(&gesture.model_to_layer, view.screen_to_world(start_x, start_y));
            let after = layer_to_model(&gesture.model_to_layer, view.screen_to_world(x, y));
            let dx = if gesture.kind == GumballHandle::MoveX { after.0 - before.0 } else { 0.0 };
            let dy = if gesture.kind == GumballHandle::MoveY { after.1 - before.1 } else { 0.0 };
            (dx.abs() >= MOVE_EPSILON || dy.abs() >= MOVE_EPSILON).then_some(GumballMotion::Translate { dx, dy })
        }
        GumballHandle::Rotate => {
            let raw = (y - pivot.1).atan2(x - pivot.0) - (start_y - pivot.1).atan2(start_x - pivot.0);
            let previous = match gesture.total {
                Some(GumballMotion::Rotate { angle }) => angle,
                _ => 0.0,
            };
            let angle = previous + wrap_angle(raw - previous);
            (angle.abs() >= TURN_EPSILON).then_some(GumballMotion::Rotate { angle })
        }
        GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleUniform => {
            let reach0 = (start_x - pivot.0).hypot(start_y - pivot.1);
            let reach1 = (x - pivot.0).hypot(y - pivot.1);
            let ratio = if reach0 > 1e-6 { reach1 / reach0 } else { 1.0 };
            let (sx, sy) = match gesture.kind {
                GumballHandle::ScaleUniform => (ratio, ratio),
                GumballHandle::ScaleX => (ratio, 1.0),
                _ => (1.0, ratio),
            };
            ((ratio - 1.0).abs() >= TURN_EPSILON).then_some(GumballMotion::Scale { sx, sy })
        }
    }
}

/// ➖️ The increment from `previous` to `current` — offsets and angles subtract, factors divide; `None` when it moves
/// nothing.
pub(crate) fn increment(previous: Option<GumballMotion>, current: GumballMotion) -> Option<GumballMotion> {
    match (previous, current) {
        (Some(GumballMotion::Translate { dx: before_x, dy: before_y }), GumballMotion::Translate { dx, dy }) => {
            let (dx, dy) = (dx - before_x, dy - before_y);
            (dx.abs() >= MOVE_EPSILON || dy.abs() >= MOVE_EPSILON).then_some(GumballMotion::Translate { dx, dy })
        }
        (Some(GumballMotion::Rotate { angle: before }), GumballMotion::Rotate { angle }) => {
            let angle = angle - before;
            (angle.abs() >= TURN_EPSILON).then_some(GumballMotion::Rotate { angle })
        }
        (Some(GumballMotion::Scale { sx: before_x, sy: before_y }), GumballMotion::Scale { sx, sy }) => {
            let (sx, sy) = (sx / before_x, sy / before_y);
            ((sx - 1.0).abs() >= TURN_EPSILON || (sy - 1.0).abs() >= TURN_EPSILON).then_some(GumballMotion::Scale { sx, sy })
        }
        _ => Some(current),
    }
}

fn identity(kind: GumballHandle) -> GumballMotion {
    match kind {
        GumballHandle::MoveX | GumballHandle::MoveY => GumballMotion::Translate { dx: 0.0, dy: 0.0 },
        GumballHandle::Rotate => GumballMotion::Rotate { angle: 0.0 },
        GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleUniform => GumballMotion::Scale { sx: 1.0, sy: 1.0 },
    }
}

fn dispatch(gesture: &GumballGesture, motion: Option<GumballMotion>, phase: Option<GumballPhase>) -> GumballDispatch {
    GumballDispatch { verb: gesture.kind.verb(), ids: gesture.ids.clone(), motion, phase }
}

/// 🌊️ A pointer move: a live gesture streams the increment since its last tick; a non-live one records its total for the
/// ghost.
pub(crate) fn drag(gesture: &mut GumballGesture, view: &GumballView, x: f64, y: f64) -> Option<GumballDispatch> {
    let current = total(gesture, view, x, y)?;
    if !gesture.live {
        gesture.total = Some(current);
        return None;
    }
    let step = increment(gesture.total, current)?;
    gesture.total = Some(current);
    gesture.streamed = true;
    Some(dispatch(gesture, Some(step), Some(GumballPhase::Stream)))
}

/// 💾️ The release at local `(x, y)`: a live gesture commits its ONE transaction with the remaining tail (the identity
/// when nothing is left; nothing when it never moved); a non-live one dispatches its whole pose delta as ONE one-shot.
pub(crate) fn release(gesture: &GumballGesture, view: &GumballView, x: f64, y: f64) -> Option<GumballDispatch> {
    let current = total(gesture, view, x, y);
    if !gesture.live {
        return current.map(|motion| dispatch(gesture, Some(motion), None));
    }
    let tail = current.and_then(|current| increment(gesture.total, current));
    (gesture.streamed || tail.is_some()).then(|| dispatch(gesture, Some(tail.unwrap_or_else(|| identity(gesture.kind))), Some(GumballPhase::Commit)))
}

/// 🧯️ A host cancel: a live gesture that streamed drops its open transaction with zero trace; anything else sends nothing.
pub(crate) fn cancel(gesture: &GumballGesture, reason: &'static str) -> Option<GumballDispatch> {
    (gesture.live && gesture.streamed).then(|| dispatch(gesture, None, Some(GumballPhase::Abort(reason))))
}
//#endregion 🧮️Algebra

//#region 🖱️Input
/// 🪪️ The one gumball gesture the canvas owns: its pointer, window, host and document generation, the scene's address,
/// the gesture and the last local pointer (the ghost's anchor).
#[derive(Clone, Debug)]
struct GumballSlot {
    pointer_id: ui_render::PointerId,
    window_id: String,
    host_id: String,
    document_generation: u64,
    controller_id: String,
    surface_id: String,
    view: GumballView,
    gesture: GumballGesture,
    pointer: (f64, f64),
    cancellation_requested: bool,
}

thread_local! {
    static GUMBALL: RefCell<Option<GumballSlot>> = const { RefCell::new(None) };
}

fn view_of(inner: Rect, camera: (f64, f64, f64)) -> GumballView {
    GumballView { camera_x: camera.0, camera_y: camera.1, zoom: camera.2, width: f64::from(inner.w), height: f64::from(inner.h) }
}

fn local(inner: Rect, x: f32, y: f32) -> (f64, f64) {
    (f64::from(x - inner.x), f64::from(y - inner.y))
}

/// 🟢️ Whether the scene's Transform utility is armed — React shows the overlay under exactly this utility.
fn armed(scene: &UiComponentSceneNode) -> bool {
    crate::scenes::canvas_active_utility(scene).as_deref() == Some("transform")
}

fn publish(slot: &GumballSlot, dispatch: Option<GumballDispatch>, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, commit: impl FnOnce()) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let Some(dispatch) = dispatch else {
        commit();
        return Ok(());
    };
    let mut args = dispatch.args();
    if let Value::Object(entries) = &mut args {
        entries.insert("surfaceId".into(), json!(slot.surface_id));
    }
    let action = ActionDescriptor { controller_id: slot.controller_id.clone(), action: dispatch.verb.into(), args: Some(semio_framework::DslValue::from(&args)) };
    crate::scenes::write_scene_action_batch(input, &[action], commit)
}

fn clear_slot() {
    GUMBALL.with(|cell| {
        cell.borrow_mut().take();
    });
}

fn owns(slot: &GumballSlot, pointer_id: ui_render::PointerId, window_id: &str, host_id: &str, document_generation: u64) -> bool {
    slot.pointer_id == pointer_id && slot.window_id == window_id && slot.host_id == host_id && slot.document_generation == document_generation
}

/// 🖱️ The Canvas2d pointer button seam: under the Transform utility a primary press grabs the handle it hits (or
/// nothing — the press never reaches the document, as in React) and the release ends the gesture it owns. `None` leaves
/// the event to the canvas's own gestures (middle-button pan, other utilities).
#[allow(clippy::too_many_arguments)]
pub(crate) fn pointer_button_into(
    scene: &UiComponentSceneNode,
    inner: Rect,
    pointer_id: ui_render::PointerId,
    window_id: &str,
    document_generation: u64,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    camera: (f64, f64, f64),
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<Option<bool>, ui_wgpu::wgpu::BoundedActionFault> {
    if !down {
        let Some(slot) = GUMBALL.with(|cell| cell.borrow().clone().filter(|slot| owns(slot, pointer_id, window_id, &scene.host_id, document_generation))) else { return Ok(None) };
        let (local_x, local_y) = local(inner, x, y);
        publish(&slot, release(&slot.gesture, &slot.view, local_x, local_y), input, clear_slot)?;
        return Ok(Some(true));
    }
    if button != 0 || !armed(scene) {
        return Ok(None);
    }
    let view = view_of(inner, camera);
    let (local_x, local_y) = local(inner, x, y);
    let Some(meta) = scene.canvas_2d.as_ref().and_then(|canvas| parse_meta(canvas.layers_json.as_str())) else { return Ok(Some(true)) };
    let Some(kind) = handle_at(&meta, &view, local_x, local_y) else { return Ok(Some(true)) };
    if GUMBALL.with(|cell| cell.borrow().is_some()) {
        cancel_into(None, "captureLost", input)?;
    }
    let slot = GumballSlot {
        pointer_id,
        window_id: window_id.to_string(),
        host_id: scene.host_id.clone(),
        document_generation,
        controller_id: scene.controller_id.clone(),
        surface_id: scene.surface_id.clone(),
        view,
        gesture: begin(&meta, kind, local_x, local_y),
        pointer: (local_x, local_y),
        cancellation_requested: false,
    };
    GUMBALL.with(|cell| *cell.borrow_mut() = Some(slot));
    Ok(Some(true))
}

/// 🐾️ The Canvas2d pointer move seam: the gesture this pointer owns follows it anywhere (pointer capture), streaming a
/// live tick or moving the local ghost. `None` when the pointer owns no gumball gesture.
#[allow(clippy::too_many_arguments)]
pub(crate) fn pointer_move_into(
    scene: &UiComponentSceneNode,
    inner: Rect,
    pointer_id: ui_render::PointerId,
    window_id: &str,
    document_generation: u64,
    x: f32,
    y: f32,
    camera: (f64, f64, f64),
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<Option<bool>, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(mut slot) = GUMBALL.with(|cell| cell.borrow().clone().filter(|slot| owns(slot, pointer_id, window_id, &scene.host_id, document_generation))) else { return Ok(None) };
    slot.view = view_of(inner, camera);
    slot.pointer = local(inner, x, y);
    let step = drag(&mut slot.gesture, &slot.view, slot.pointer.0, slot.pointer.1);
    let next = slot.clone();
    publish(&slot, step, input, move || GUMBALL.with(|cell| *cell.borrow_mut() = Some(next)))?;
    Ok(Some(true))
}

/// 🛑️ Cancels the gumball gesture `pointer_id` owns (any, when `None`): a live gesture that streamed sends its abort with
/// `reason`; the slot clears either way. `true` when a gesture was cancelled.
pub(crate) fn cancel_into(pointer_id: Option<ui_render::PointerId>, reason: &'static str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(slot) = GUMBALL.with(|cell| cell.borrow().clone().filter(|slot| pointer_id.is_none_or(|pointer_id| slot.pointer_id == pointer_id))) else { return Ok(false) };
    if crate::interpreter::ui_document_close_pending_for(&slot.window_id) || crate::scenes::scene_host_retiring(&slot.host_id) {
        clear_slot();
        return Ok(true);
    }
    publish(&slot, cancel(&slot.gesture, reason), input, clear_slot)?;
    Ok(true)
}

/// 🥀️ Cancels a gesture of `window_id`/`host_id` opened on another document generation than the live one.
pub(crate) fn cancel_stale_into(window_id: &str, host_id: &str, document_generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let stale = GUMBALL.with(|cell| cell.borrow().as_ref().filter(|slot| slot.window_id == window_id && slot.host_id == host_id && slot.document_generation != document_generation).map(|slot| slot.pointer_id));
    if let Some(pointer_id) = stale {
        cancel_into(Some(pointer_id), "captureLost", input)?;
    }
    Ok(())
}

/// 🚩️ Asks the next terminal step to cancel the gesture `matches(window_id, host_id, document_generation)` selects.
pub(crate) fn request_cancel(mut matches: impl FnMut(&str, &str, u64) -> bool) {
    GUMBALL.with(|cell| {
        if let Some(slot) = cell.borrow_mut().as_mut().filter(|slot| matches(&slot.window_id, &slot.host_id, slot.document_generation)) {
            slot.cancellation_requested = true;
        }
    });
}

/// ⏰️ The pointer whose gesture a terminal step must cancel now.
pub(crate) fn cancel_due() -> Option<ui_render::PointerId> {
    GUMBALL.with(|cell| cell.borrow().as_ref().filter(|slot| slot.cancellation_requested).map(|slot| slot.pointer_id))
}

/// 🧹️ Drops the gesture of a window entering close (the app's `retiring` host event aborts its transaction).
pub(crate) fn retire_window(window_id: &str) -> bool {
    GUMBALL.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.as_ref().is_some_and(|slot| slot.window_id == window_id) {
            *slot = None;
            return true;
        }
        false
    })
}
//#endregion 🖱️Input

//#region 🎨️Paint
const CIRCLE_SEGMENTS: usize = 32;
const RING_DASHES: usize = 36;

fn circle_points(cx: f32, cy: f32, radius: f32, segments: usize) -> Vec<[f32; 2]> {
    (0..segments).map(|index| index as f32 / segments as f32 * std::f32::consts::TAU).map(|angle| [cx + radius * angle.cos(), cy + radius * angle.sin()]).collect()
}

fn disc(draw: &mut ui_wgpu::wgpu::DrawList, cx: f32, cy: f32, radius: f32, color: Rgba) {
    draw.push_triangle_fan(&circle_points(cx, cy, radius, CIRCLE_SEGMENTS), color);
}

fn ring(draw: &mut ui_wgpu::wgpu::DrawList, cx: f32, cy: f32, radius: f32, color: Rgba, width: f32, dashed: bool) {
    let points = circle_points(cx, cy, radius, if dashed { RING_DASHES * 2 } else { CIRCLE_SEGMENTS });
    for index in (0..points.len()).step_by(if dashed { 2 } else { 1 }) {
        let (a, b) = (points[index], points[(index + 1) % points.len()]);
        draw.push_line(a[0], a[1], b[0], b[1], color, width);
    }
}

/// 🎨️ Paints the armed gumball of `scene` over its canvas `inner` under `camera` — the handles of
/// `🟦️GumballOverlay.tsx`, the press marker while a gesture is open and the ghost of a non-live gesture.
pub(crate) fn paint(scene: &UiComponentSceneNode, inner: Rect, camera: (f64, f64, f64), draw: &mut ui_wgpu::wgpu::DrawList) {
    if !armed(scene) || inner.w <= 0.0 || inner.h <= 0.0 {
        return;
    }
    let Some(meta) = scene.canvas_2d.as_ref().and_then(|canvas| parse_meta(canvas.layers_json.as_str())) else { return };
    let view = view_of(inner, camera);
    let (px, py) = pivot_screen(meta.pivot_layer, &view);
    let (cx, cy) = (inner.x + px as f32, inner.y + py as f32);
    let axis = |index: u8, alpha: f32| ui_wgpu::wgpu::draw_types::gizmo::spatial_axis_rgba(index, alpha);
    let knob = |kind: GumballHandle| knobs((px, py)).into_iter().find(|(candidate, _, _)| *candidate == kind).map_or((cx, cy), |(_, x, y)| (inner.x + x as f32, inner.y + y as f32));
    let radius = HIT_RADIUS as f32;
    if meta.config.rotate {
        ring(draw, cx, cy, ROTATE_RADIUS as f32, axis(1, 1.0), 1.5, true);
    }
    if meta.config.move_axes {
        for (kind, index) in [(GumballHandle::MoveX, 0), (GumballHandle::MoveY, 1)] {
            let (x, y) = knob(kind);
            draw.push_line(cx, cy, x, y, axis(index, 1.0), 2.0);
            disc(draw, x, y, radius, axis(index, 1.0));
        }
    }
    if meta.config.scale_axes {
        for (kind, index) in [(GumballHandle::ScaleX, 0), (GumballHandle::ScaleY, 1)] {
            let (x, y) = knob(kind);
            disc(draw, x, y, radius, axis(index, 0.85));
        }
    }
    if meta.config.scale_uniform {
        let (x, y) = knob(GumballHandle::ScaleUniform);
        disc(draw, x, y, radius, axis(2, 1.0));
    }
    disc(draw, cx, cy, 4.0, Rgba::from_srgb8(255, 255, 255, 255));
    ring(draw, cx, cy, 4.0, Rgba::from_srgb8(148, 163, 184, 255), 1.0, false);
    let Some(slot) = GUMBALL.with(|cell| cell.borrow().clone().filter(|slot| slot.host_id == scene.host_id)) else { return };
    let marker = Rgba::from_srgb8(250, 204, 21, 230);
    if !slot.gesture.live {
        let (pointer_x, pointer_y) = (inner.x + slot.pointer.0 as f32, inner.y + slot.pointer.1 as f32);
        let (start_x, start_y) = (inner.x + slot.gesture.start.0 as f32, inner.y + slot.gesture.start.1 as f32);
        match slot.gesture.kind {
            GumballHandle::MoveX | GumballHandle::MoveY => {
                let x = if slot.gesture.kind == GumballHandle::MoveX { cx + pointer_x - start_x } else { cx };
                let y = if slot.gesture.kind == GumballHandle::MoveY { cy + pointer_y - start_y } else { cy };
                draw.push_line(cx, cy, x, y, marker, 1.5);
                ring(draw, x, y, 5.0, marker, 2.0, false);
            }
            GumballHandle::Rotate => draw.push_line(cx, cy, pointer_x, pointer_y, marker, 2.0),
            GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleUniform => ring(draw, cx, cy, (pointer_x - cx).hypot(pointer_y - cy), marker, 1.5, true),
        }
    }
    disc(draw, cx, cy, 3.0, marker);
}
//#endregion 🎨️Paint

#[cfg(test)]
#[path = "../../🧪️tests/🧪️wgpu-gumball-dispatch/🦀️.rs"]
mod tests;
