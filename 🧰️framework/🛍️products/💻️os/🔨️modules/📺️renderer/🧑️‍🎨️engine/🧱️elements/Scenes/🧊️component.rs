//! 🎬️ framework/products/os/modules/renderer/engine/elements/Scenes/component.rs — wgpu render
//! implementation for the Scenes element, extracted from lib.rs's inline `pub mod scenes { ... }`
//! body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via `#[path =
//! "../../../../🧱️elements/Scenes/🧊️component.rs"] pub mod scenes;` in lib.rs in place of the former
//! inline block; the module name `scenes` is unchanged, so every existing `crate::scenes::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🎬️ Native component scene hosts for canvas-2d, tables, graphs, and 3D views.

use crate::engine_canvas;
use crate::interpreter::{validate_component_scene, FrameworkWidgetContext, RENDER_PLAN_LIMITS};
use crate::shell::{push_find_item, ShellFindItem};
use base64::Engine;
use infinite_world::{render_world_3d, World3dState};
use semio_framework::IconName;
use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ui_wgpu::wgpu::input::{DragAxis, KeyAction};
use ui_wgpu::wgpu::{draw_text, draw_text_wrapped, render_widget, HitKind, HitTarget, Rect, Rgba, Theme, WidgetNode};
use ui_wgpu::wgpu::{ActionDescriptor, SurfaceKind, UiComponentSceneNode, UiPresence};

//#region SceneRuntime
#[derive(Clone, Copy, Debug, Default)]
struct Viewport {
    x: f32,
    y: f32,
    zoom: f32,
}

impl Viewport {
    fn from_json(raw: &str) -> Self {
        serde_json::from_str::<Value>(raw)
            .ok()
            .map(|value| Self { x: value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, y: value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32, zoom: value.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32 })
            .unwrap_or_default()
    }

    fn from_typed(viewport: Option<&ui_wgpu::wgpu::NodeGraphViewport>) -> Self {
        viewport.map(|viewport| Self { x: viewport.x as f32, y: viewport.y as f32, zoom: viewport.zoom as f32 }).unwrap_or(Self { x: 0.0, y: 0.0, zoom: 1.0 })
    }

    fn screen_to_world(&self, sx: f32, sy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        ((sx - cx) / self.zoom + self.x, (sy - cy) / self.zoom + self.y)
    }

    fn world_to_screen(&self, wx: f32, wy: f32, origin: Rect) -> (f32, f32) {
        let cx = origin.x + origin.w * 0.5;
        let cy = origin.y + origin.h * 0.5;
        (cx + (wx - self.x) * self.zoom, cy + (wy - self.y) * self.zoom)
    }
}

#[derive(Clone, Debug)]
enum SceneDragMode {
    PanViewport,
    MoveNode { node_id: String, grab_x: f32, grab_y: f32 },
    ConnectPort { source_node_id: String, source_port_id: String, is_output: bool },
    Marquee,
    MapMarquee { start_x: f32, start_y: f32, method: String, merge_mode: String },
    MapPan,
    InkPan { start_x: f32, start_y: f32, camera_x: f64, camera_y: f64, zoom: f64 },
    InkMove { origins: HashMap<String, (f64, f64)>, start_x: f32, start_y: f32 },
    InkResize { handle: String, from: InkBoundsF, start_x: f32, start_y: f32, selected_ids: Vec<String> },
    InkStroke { block_id: String },
    InkEraser { mode: String },
    InkMarqueeDrag { start_x: f32, start_y: f32 },
}

#[derive(Clone, Debug)]
struct SceneDrag {
    mode: SceneDragMode,
    button: i16,
}

#[derive(Clone, Debug, Default)]
struct SceneSurfaceState {
    scroll_offsets: HashMap<String, f32>,
    viewport: Viewport,
    drag: Option<SceneDrag>,
    pointer_was_down: bool,
    last_click_ms: f64,
    last_click_target: Option<String>,
    editor_cursor: usize,
    node_positions: HashMap<String, (f32, f32)>,
    selected_ids: HashSet<String>,
    hover_row_id: Option<String>,
    raster_digest: Option<u64>,
    pending_raster: Option<PendingRasterUpload>,
    pending_raster_uploads: Vec<PendingRasterUpload>,
    canvas_image_digests: HashMap<String, u64>,
    canvas_image_src_digests: HashMap<String, u64>,
    paint_stroke_active: bool,
    vfs_expanded_ids: HashSet<String>,
    vfs_selection_anchor: Option<String>,
    map_marquee_points: Vec<(f32, f32)>,
    map_marquee_active: bool,
    map_last_hover_json: Option<String>,
    ink_camera: Option<(f64, f64, f64)>,
    ink_overrides: HashMap<String, Value>,
    ink_marquee_points: Vec<(f32, f32)>,
    //#region GenericPointerDispatch
    last_pointer_pos: (f32, f32),
    //#endregion GenericPointerDispatch
    /// 🕒️ The controller id a Canvas2d/Paint2d surface's settled `setCamera` dispatch should target —
    /// stashed here (rather than threaded through `SCENE_CAMERA_DISPATCH_DEADLINES_MS`) since that map
    /// only needs a bare surface-id -> deadline shape to stay a drop-in match for
    /// `sweep_expired_camera_dispatch_deadlines`. Set on every wheel/pan mutation, read (never cleared)
    /// by `sweep_expired_scene_camera_dispatches` at expiry.
    camera_dispatch_controller_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PendingRasterUpload {
    pub key: String,
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

thread_local! {
    static SCENE_STATE: RefCell<HashMap<String, SceneSurfaceState>> = RefCell::new(HashMap::new());
    static GRAPH_NODE_CTX: RefCell<HashMap<String, Option<String>>> = RefCell::new(HashMap::new());
    /// 🕒️ Canvas2d/Paint2d's settle-then-dispatch deadline map — surface id -> the timestamp its
    /// debounced `setCamera` should fire at. Same shape/sweep (`sweep_expired_camera_dispatch_deadlines`)
    /// as `AppRuntime`'s `world3d_camera_dispatch_deadlines_ms`; kept thread-local here rather than on
    /// `AppRuntime` because `handle_scene_wheel`/`handle_scene_pointer_move` (this module's wheel/pan
    /// mutators) only ever see a `&UiComponentSceneNode`, never `AppRuntime` itself.
    static SCENE_CAMERA_DISPATCH_DEADLINES_MS: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());
}

/** @emoji 🕸️ Clears per-frame graph node metadata used by context menus. */
pub fn clear_graph_node_context() {
    GRAPH_NODE_CTX.with(|cell| cell.borrow_mut().clear());
}

/** @emoji 🕸️ Registers a graph node instance mapping for context-menu dispatch. */
pub fn register_graph_node(node_id: &str, instance_id: Option<&str>) {
    GRAPH_NODE_CTX.with(|cell| {
        cell.borrow_mut().insert(node_id.to_string(), instance_id.map(str::to_string));
    });
}

/** @emoji 🕸️ Resolves a graph node instance id for context-menu actions. */
pub fn graph_node_instance(node_id: &str) -> Option<String> {
    GRAPH_NODE_CTX.with(|cell| cell.borrow().get(node_id).cloned().flatten())
}

/** @emoji 📁️ Toggles VFS row expand/collapse in scene-local state. */
pub fn toggle_vfs_row_expanded(surface_id: &str, row_id: &str) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.contains(row_id) {
            state.vfs_expanded_ids.remove(row_id);
        } else {
            state.vfs_expanded_ids.insert(row_id.to_string());
        }
    });
}

/** @emoji 📁️ Seeds default expanded VFS roots on first render. */
pub fn seed_vfs_expanded(surface_id: &str, row_ids: &[String]) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.is_empty() {
            for id in row_ids {
                state.vfs_expanded_ids.insert(id.clone());
            }
        }
    });
}

/** @emoji 📁️ Computes VFS multi-select ids for shift/meta click semantics. */
pub fn vfs_selection_for_click(surface_id: &str, row_id: &str, ordered_ids: &[String], shift: bool, additive: bool) -> Vec<String> {
    let mut state = scene_state(surface_id);
    if shift {
        let anchor = state.vfs_selection_anchor.clone().unwrap_or_else(|| row_id.to_string());
        let a = ordered_ids.iter().position(|id| id == &anchor);
        let b = ordered_ids.iter().position(|id| id == row_id);
        if let (Some(a), Some(b)) = (a, b) {
            let (start, end) = if a <= b { (a, b) } else { (b, a) };
            let ids: Vec<String> = ordered_ids[start..=end].to_vec();
            state.vfs_selection_anchor = Some(anchor);
            mutate_scene_state(surface_id, |state| {
                state.vfs_selection_anchor = Some(row_id.to_string());
            });
            return ids;
        }
    }
    mutate_scene_state(surface_id, |state| {
        state.vfs_selection_anchor = Some(row_id.to_string());
    });
    if additive {
        let mut ids: Vec<String> = scene_state(surface_id).selected_ids.into_iter().collect();
        if ids.iter().any(|id| id == row_id) {
            ids.retain(|id| id != row_id);
        } else {
            ids.push(row_id.to_string());
        }
        return ids;
    }
    vec![row_id.to_string()]
}

fn scene_state(surface_id: &str) -> SceneSurfaceState {
    SCENE_STATE.with(|cell| cell.borrow_mut().entry(surface_id.to_string()).or_default().clone())
}

fn mutate_scene_state(surface_id: &str, f: impl FnOnce(&mut SceneSurfaceState)) {
    SCENE_STATE.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.entry(surface_id.to_string()).or_default();
        f(entry);
    });
}

//#region SceneCameraDispatch
/// 🕒️ Pushes a Canvas2d/Paint2d surface's settled `setCamera` deadline ~350ms out — called on every
/// wheel/pan mutation (see `handle_scene_wheel`'s `Canvas2d`/`Paint2d` arms and
/// `handle_scene_pointer_move`'s `PanViewport` arm), same 350ms settle window as
/// `AppRuntime::world3d_camera_dispatch_deadlines_ms`.
fn schedule_scene_camera_dispatch(surface_id: &str) {
    SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| {
        cell.borrow_mut().insert(surface_id.to_string(), crate::app_now_ms() + 350.0);
    });
}

/// 🕒️ A Canvas2d/Paint2d `setCamera` action from a bare surface id/controller id/viewport — same
/// `{surfaceId, camera: {x, y, zoom}}` shape `ink_set_camera_action` already uses for `InkCanvas`'s own
/// camera dispatch (this crate's one other camera-from-viewport builder), reused here since neither
/// `Paint2dHost`'s React source nor this repo's own Ink precedent key it any other way.
fn scene_camera_action(surface_id: &str, controller_id: &str, viewport: Viewport) -> ActionDescriptor {
    ActionDescriptor { controller_id: controller_id.to_string(), action: "setCamera".into(), args: crate::action_args_json!({ "surfaceId": surface_id, "camera": { "x": viewport.x, "y": viewport.y, "zoom": viewport.zoom } }) }
}

/// 🕒️ `AppRuntime::frame`'s per-frame hook into this module's settle-then-dispatch deadlines —
/// sweeps `SCENE_CAMERA_DISPATCH_DEADLINES_MS` via the shared pure
/// `sweep_expired_camera_dispatch_deadlines`, then builds each expired surface's `setCamera` action
/// from its last-known viewport + stashed controller id (`camera_dispatch_controller_id`; `None` only
/// if a deadline outlives its `SCENE_STATE` entry, which never happens in practice since both are
/// written together in `schedule_scene_camera_dispatch`'s call sites).
pub fn sweep_expired_scene_camera_dispatches(now_ms: f64) -> Vec<ActionDescriptor> {
    let expired = SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| crate::sweep_expired_camera_dispatch_deadlines(&mut cell.borrow_mut(), now_ms));
    expired
        .into_iter()
        .filter_map(|surface_id| {
            let state = scene_state(&surface_id);
            state.camera_dispatch_controller_id.clone().map(|controller_id| scene_camera_action(&surface_id, &controller_id, state.viewport))
        })
        .collect()
}
//#endregion SceneCameraDispatch

/** @emoji 🖱️ Cheap read of a surface's pointer edge-detection fields, avoiding a full `SceneSurfaceState` clone. `pub(crate)` so `interpreter::apply_scene_ui_command` (the real per-event `UiCommand::Scene` handler — the sole caller of `handle_scene_pointer_button`/`handle_scene_pointer_move` now, `RenderEntry`'s own once-per-render-frame `apply_scene_wheel`/`apply_scene_pointer` having been deleted once every generic-fallback surface was proven reachable through this path) can read `pointer_was_down`/`last_pointer_pos` to derive `handle_scene_pointer_move`'s `down`/drag-delta parameters. */
pub(crate) fn scene_pointer_edge_state(surface_id: &str) -> (bool, f32, f32) {
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).map(|state| (state.pointer_was_down, state.last_pointer_pos.0, state.last_pointer_pos.1)).unwrap_or((false, 0.0, 0.0)))
}

/** @emoji 🖱️ Records `surface_id`'s latest known pointer position — the write half of `scene_pointer_edge_state`, `pub(crate)` for the same reason (see that fn's own doc comment). */
pub(crate) fn set_scene_last_pointer_pos(surface_id: &str, x: f32, y: f32) {
    mutate_scene_state(surface_id, |state| {
        state.last_pointer_pos = (x, y);
    });
}

fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.into(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

fn surface_args(scene: &UiComponentSceneNode) -> Value {
    json!({ "surfaceId": scene.surface_id })
}

fn scroll_key(surface_id: &str, suffix: &str) -> String {
    format!("{surface_id}.{suffix}")
}

fn scroll_offset(surface_id: &str, suffix: &str) -> f32 {
    let key = scroll_key(surface_id, suffix);
    SCENE_STATE.with(|cell| cell.borrow().get(surface_id).and_then(|state| state.scroll_offsets.get(&key).copied()).unwrap_or(0.0)).max(0.0)
}

fn set_scroll_offset(surface_id: &str, suffix: &str, value: f32) {
    let key = scroll_key(surface_id, suffix);
    mutate_scene_state(surface_id, |state| {
        state.scroll_offsets.insert(key, value.max(0.0));
    });
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window().and_then(|window| window.performance()).map(|perf| perf.now()).unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    0.0
}

fn digest_pixels(pixels: &[u8]) -> u64 {
    pixels.iter().fold(0u64, |acc, byte| acc.wrapping_mul(31).wrapping_add(*byte as u64))
}

pub fn drain_pending_raster_uploads() -> Vec<PendingRasterUpload> {
    let mut uploads = Vec::new();
    SCENE_STATE.with(|cell| {
        for state in cell.borrow_mut().values_mut() {
            if let Some(pending) = state.pending_raster.take() {
                uploads.push(pending);
            }
            uploads.append(&mut state.pending_raster_uploads);
        }
    });
    uploads
}
//#endregion SceneRuntime

fn canvas_world_pointer_json(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, extra: Value) -> Value {
    let state = scene_state(&scene.surface_id);
    let (wx, wy) = state.viewport.screen_to_world(x, y, inner);
    let mut payload = json!({
        "surfaceId": scene.surface_id,
        "x": wx,
        "y": wy,
    });
    if let (Some(base), Some(patch)) = (payload.as_object_mut(), extra.as_object()) {
        for (key, value) in patch {
            base.insert(key.clone(), value.clone());
        }
    }
    payload
}

//#region SceneInput
const MAP_MARQUEE_THRESHOLD_PX: f32 = 6.0;

pub fn handle_scene_wheel(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, delta: f32, ctrl: bool) -> Vec<ActionDescriptor> {
    if !bounds.contains(x, y) {
        return Vec::new();
    }
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    match scene.component_kind {
        SurfaceKind::Table => {
            let current = scroll_offset(&scene.surface_id, "body");
            set_scroll_offset(&scene.surface_id, "body", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::TextEditor => engine_canvas::text_editor_wheel(scene, delta),
        SurfaceKind::VirtualFileSystem => {
            let current = scroll_offset(&scene.surface_id, "vfs");
            set_scroll_offset(&scene.surface_id, "vfs", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::Canvas2d => {
            mutate_scene_state(&scene.surface_id, |state| {
                let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.125, 8.0);
                state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
            });
            schedule_scene_camera_dispatch(&scene.surface_id);
            Vec::new()
        }
        SurfaceKind::Paint2d => {
            if let Some(paint_2d) = &scene.paint_2d {
                let doc: Paint2dDocSyncJson = serde_json::from_str(&paint_2d.document_sync_json).unwrap_or_default();
                mutate_scene_state(&scene.surface_id, |state| {
                    if state.viewport.zoom <= 0.0 {
                        state.viewport = Viewport { x: doc.camera.x as f32, y: doc.camera.y as f32, zoom: doc.camera.zoom as f32 };
                    }
                    let factor = (1.0 - delta * 0.001).clamp(0.5, 2.0);
                    state.viewport.zoom = (state.viewport.zoom * factor).clamp(0.05, 32.0);
                    state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
                });
                schedule_scene_camera_dispatch(&scene.surface_id);
            }
            Vec::new()
        }
        SurfaceKind::NodeGraph => engine_canvas::node_graph_wheel(&scene.surface_id, &scene.controller_id, inner, x, y, delta, ctrl),
        SurfaceKind::TiledMap => engine_canvas::tiled_map_wheel(&scene.surface_id, &scene.controller_id, inner, x, y, delta, ctrl),
        SurfaceKind::InkCanvas => ink_wheel(scene, inner, x, y, delta),
        SurfaceKind::GraphTimeline => {
            let current = scroll_offset(&scene.surface_id, "history");
            set_scroll_offset(&scene.surface_id, "history", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::DiffView => {
            let current = scroll_offset(&scene.surface_id, "diff");
            set_scroll_offset(&scene.surface_id, "diff", current + delta * 0.5);
            Vec::new()
        }
        SurfaceKind::EventFeed => {
            let current = scroll_offset(&scene.surface_id, "feed");
            set_scroll_offset(&scene.surface_id, "feed", current + delta * 0.5);
            Vec::new()
        }
        _ => Vec::new(),
    }
}

pub fn handle_scene_pointer_move(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, down: bool, _button: i16, drag_dx: f32, drag_dy: f32) -> Vec<ActionDescriptor> {
    let inner = bounds;
    if !inner.contains(x, y) {
        return Vec::new();
    }
    let mut actions = Vec::new();
    let state = scene_state(&scene.surface_id);
    if down {
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::PanViewport => {
                    let vp = state.viewport;
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.viewport.x -= drag_dx / vp.zoom.max(0.01);
                        state.viewport.y -= drag_dy / vp.zoom.max(0.01);
                        state.camera_dispatch_controller_id = Some(scene.controller_id.clone());
                    });
                    schedule_scene_camera_dispatch(&scene.surface_id);
                }
                SceneDragMode::MoveNode { node_id, grab_x, grab_y } => {
                    let vp = state.viewport;
                    let (wx, wy) = vp.screen_to_world(x, y, inner);
                    let nx = wx - grab_x;
                    let ny = wy - grab_y;
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.node_positions.insert(node_id.clone(), (nx, ny));
                    });
                }
                SceneDragMode::ConnectPort { .. } => {}
                SceneDragMode::Marquee => {}
                SceneDragMode::MapMarquee { start_x, start_y, method, .. } => {
                    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
                    let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(&scene.surface_id, |state| {
                        if distance >= MAP_MARQUEE_THRESHOLD_PX {
                            state.map_marquee_active = true;
                        }
                        if state.map_marquee_active {
                            if method == "lasso" {
                                state.map_marquee_points.push((sx as f32, sy as f32));
                            } else {
                                state.map_marquee_points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                            }
                        }
                    });
                }
                SceneDragMode::MapPan => {}
                SceneDragMode::InkPan { start_x, start_y, camera_x, camera_y, zoom } => {
                    let dx = (x - start_x) as f64;
                    let dy = (y - start_y) as f64;
                    let next = InkCameraF { x: camera_x + dx, y: camera_y + dy, zoom: *zoom };
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.ink_camera = Some((next.x, next.y, next.zoom));
                    });
                    actions.push(ink_set_camera_action(scene, next));
                }
                SceneDragMode::InkMove { origins, start_x, start_y } => {
                    let camera = ink_current_camera(scene);
                    let dx = (x - start_x) as f64 / camera.zoom.max(0.0001);
                    let dy = (y - start_y) as f64 / camera.zoom.max(0.0001);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let mut events = Vec::new();
                    let mut new_overrides = Vec::new();
                    for (id, (ox, oy)) in origins.iter() {
                        if let Some(block) = find_ink_item(&doc.blocks, id) {
                            let updated = ink_item_with_position(block, ox + dx, oy + dy);
                            events.push(json!({ "operation": "updateBlock", "blockId": id, "block": updated }));
                            new_overrides.push((id.clone(), updated));
                        }
                    }
                    if !events.is_empty() {
                        mutate_scene_state(&scene.surface_id, |state| {
                            for (id, block) in new_overrides {
                                state.ink_overrides.insert(id, block);
                            }
                        });
                        actions.push(ink_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::InkResize { handle, from, start_x, start_y, selected_ids } => {
                    let camera = ink_current_camera(scene);
                    let dx = (x - start_x) as f64 / camera.zoom.max(0.0001);
                    let dy = (y - start_y) as f64 / camera.zoom.max(0.0001);
                    let to = ink_resize_bounds(*from, handle, dx, dy, 8.0);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let mut events = Vec::new();
                    let mut new_overrides = Vec::new();
                    for id in selected_ids {
                        if let Some(block) = find_ink_item(&doc.blocks, id) {
                            let updated = scale_ink_item(block, *from, to);
                            events.push(json!({ "operation": "updateBlock", "blockId": id, "block": updated }));
                            new_overrides.push((id.clone(), updated));
                        }
                    }
                    if !events.is_empty() {
                        mutate_scene_state(&scene.surface_id, |state| {
                            for (id, block) in new_overrides {
                                state.ink_overrides.insert(id, block);
                            }
                        });
                        actions.push(ink_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::InkStroke { block_id } => {
                    let camera = ink_current_camera(scene);
                    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let current = state.ink_overrides.get(block_id).cloned().or_else(|| find_ink_item(&doc.blocks, block_id).cloned());
                    if let Some(mut block) = current {
                        let bx = ink_item_num(&block, "x");
                        let by = ink_item_num(&block, "y");
                        let local = json!([world_x - bx, world_y - by]);
                        if let Some(obj) = block.as_object_mut() {
                            let mut points = obj.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
                            points.push(local);
                            obj.insert("points".into(), Value::Array(points));
                        }
                        let block_id = block_id.clone();
                        mutate_scene_state(&scene.surface_id, |state| {
                            state.ink_overrides.insert(block_id.clone(), block.clone());
                        });
                        actions.push(ink_apply_events_action(scene, &[json!({ "operation": "updateBlock", "blockId": block_id, "block": block })], "live", None));
                    }
                }
                SceneDragMode::InkEraser { mode } => {
                    let camera = ink_current_camera(scene);
                    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);
                    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
                    let events = if mode == "eraserStroke" { erase_ink_stroke_events(&doc.blocks, world_x, world_y, 8.0) } else { erase_ink_stroke_points_events(&doc.blocks, world_x, world_y, doc.eraser_radius.unwrap_or(12.0)) };
                    if !events.is_empty() {
                        actions.push(ink_apply_events_action(scene, &events, "live", None));
                    }
                }
                SceneDragMode::InkMarqueeDrag { start_x, start_y } => {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.ink_marquee_points = vec![(*start_x, *start_y), (x, y)];
                    });
                }
            }
        }
    }
    match scene.component_kind {
        SurfaceKind::InkCanvas if !down => {
            actions.extend(ink_hover_move(scene, inner, x, y));
        }
        SurfaceKind::Canvas2d if down => {
            actions.push(scene_action(scene, "canvasPointerMove", canvas_world_pointer_json(scene, inner, x, y, json!({}))));
        }
        SurfaceKind::NodeGraph if down => {
            actions.extend(engine_canvas::node_graph_pointer_move(&scene.surface_id, &scene.controller_id, inner, x, y, false, false, false));
        }
        SurfaceKind::TextEditor if down => {
            actions.extend(engine_canvas::text_editor_pointer_move(scene, inner, x, y));
        }
        SurfaceKind::NodeGraph | SurfaceKind::TextEditor if !down => {
            actions.extend(match scene.component_kind {
                SurfaceKind::NodeGraph => engine_canvas::node_graph_pointer_move(&scene.surface_id, &scene.controller_id, inner, x, y, false, false, false),
                _ => engine_canvas::text_editor_pointer_move(scene, inner, x, y),
            });
        }
        _ => {}
    }
    actions
}

pub fn handle_scene_pointer_button(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, down: bool, button: i16, shift: bool) -> Vec<ActionDescriptor> {
    let inner = bounds;
    if !inner.contains(x, y) {
        if !down {
            mutate_scene_state(&scene.surface_id, |state| {
                state.drag = None;
                state.pointer_was_down = false;
            });
        }
        return Vec::new();
    }
    let mut actions = Vec::new();
    if down {
        mutate_scene_state(&scene.surface_id, |state| {
            state.pointer_was_down = true;
        });
        match scene.component_kind {
            SurfaceKind::Canvas2d => {
                if button == 0 {
                    mutate_scene_state(&scene.surface_id, |state| {
                        if !state.paint_stroke_active {
                            state.paint_stroke_active = true;
                        }
                    });
                    actions.push(scene_action(scene, "paintStrokeBegin", json!({ "surfaceId": scene.surface_id })));
                }
                actions.push(scene_action(scene, "canvasPointerDown", canvas_world_pointer_json(scene, inner, x, y, json!({ "button": button, "extend": shift }))));
                if button == 1 || button == 2 {
                    mutate_scene_state(&scene.surface_id, |state| {
                        state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport, button });
                    });
                }
            }
            SurfaceKind::Paint2d => {
                if button == 1 || button == 2 {
                    if let Some(paint_2d) = &scene.paint_2d {
                        let doc: Paint2dDocSyncJson = serde_json::from_str(&paint_2d.document_sync_json).unwrap_or_default();
                        mutate_scene_state(&scene.surface_id, |state| {
                            if state.viewport.zoom <= 0.0 {
                                state.viewport = Viewport { x: doc.camera.x as f32, y: doc.camera.y as f32, zoom: doc.camera.zoom as f32 };
                            }
                            state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport, button });
                        });
                    }
                }
            }
            SurfaceKind::NodeGraph => {
                actions.extend(engine_canvas::node_graph_pointer_down(&scene.surface_id, &scene.controller_id, inner, x, y, button, shift, false, false, false));
            }
            SurfaceKind::TextEditor => {
                actions.extend(engine_canvas::text_editor_pointer_down(scene, inner, x, y, button));
            }
            SurfaceKind::InkCanvas => {
                actions.extend(ink_pointer_down(scene, inner, x, y, button, shift));
            }
            _ => {}
        }
    } else {
        match scene.component_kind {
            SurfaceKind::InkCanvas => {
                actions.extend(ink_pointer_up(scene, inner, x, y));
            }
            SurfaceKind::Canvas2d => {
                actions.push(scene_action(scene, "canvasPointerUp", canvas_world_pointer_json(scene, inner, x, y, json!({}))));
                mutate_scene_state(&scene.surface_id, |state| {
                    if state.paint_stroke_active {
                        state.paint_stroke_active = false;
                    }
                });
                actions.push(scene_action(scene, "paintStrokeEnd", json!({ "surfaceId": scene.surface_id })));
            }
            SurfaceKind::NodeGraph => {
                actions.extend(engine_canvas::node_graph_pointer_up(&scene.surface_id, &scene.controller_id, inner, x, y, shift, false, false));
            }
            SurfaceKind::TextEditor => {
                actions.extend(engine_canvas::text_editor_pointer_up(scene, inner, x, y));
            }
            _ => {}
        }
        if let Some(target) = hit_double_click_target(scene, inner, x, y) {
            let now = now_ms();
            let prior = scene_state(&scene.surface_id);
            if prior.last_click_target.as_deref() == Some(target.as_str()) && now - prior.last_click_ms < 400.0 {
                if let Some(action) = double_click_action(scene, &target, inner, x, y) {
                    actions.push(action);
                }
            }
            mutate_scene_state(&scene.surface_id, |state| {
                state.last_click_target = Some(target);
                state.last_click_ms = now;
            });
        }
        mutate_scene_state(&scene.surface_id, |state| {
            if let Some(SceneDrag { mode: SceneDragMode::MoveNode { node_id, .. }, .. }) = state.drag.as_ref() {
                if let Some((nx, ny)) = state.node_positions.get(node_id).copied() {
                    actions.push(scene_action(scene, "moveMediaNode", json!({ "surfaceId": scene.surface_id, "nodeId": node_id, "x": nx, "y": ny })));
                }
            }
            state.drag = None;
            state.pointer_was_down = false;
        });
    }
    actions
}

fn hit_double_click_target(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Option<String> {
    match scene.component_kind {
        SurfaceKind::VirtualFileSystem => {
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let body_y = inner.y + 24.0;
            let index = ((y - body_y + scroll) / row_h).floor() as i32;
            if index < 0 {
                return None;
            }
            Some(format!("{}.vfs.index.{index}", scene.surface_id))
        }
        SurfaceKind::NodeGraph => hit_graph_node(scene, inner, x, y).map(|id| format!("{}.node.{}", scene.surface_id, id)),
        _ => None,
    }
}

fn double_click_action(scene: &UiComponentSceneNode, target: &str, inner: Rect, _x: f32, y: f32) -> Option<ActionDescriptor> {
    match scene.component_kind {
        SurfaceKind::VirtualFileSystem => {
            let vfs = scene.virtual_file_system.as_ref()?;
            let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).ok()?;
            let row_h = 22.0;
            let scroll = scroll_offset(&scene.surface_id, "vfs");
            let index = ((y - inner.y - 24.0 + scroll) / row_h).floor() as usize;
            rows.get(index).and_then(|row| vfs_double_click_action(scene, row))
        }
        SurfaceKind::NodeGraph => {
            let node_id = target.strip_prefix(&format!("{}.node.", scene.surface_id))?;
            let record = find_graph_node(scene, node_id)?;
            let instance_id = record.instance_id.as_deref()?;
            Some(scene_action(scene, "openInstance", json!({ "surfaceId": scene.surface_id, "instanceId": instance_id })))
        }
        _ => None,
    }
}
//#endregion SceneInput

//#region RenderEntry
pub fn render_component_scene(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::wgpu::GpuContext,
    world3d_states: &mut HashMap<String, World3dState>,
    node_graph_states: &mut HashMap<String, NodeGraphSurface>,
    tiled_map_states: &mut HashMap<String, TiledMapSurface>,
    icon_render_states: &mut HashMap<String, World3dState>,
    board2d_states: &mut HashMap<String, Board2dSurface>,
) {
    if let Err(message) = validate_component_scene(scene, &RENDER_PLAN_LIMITS) {
        let theme = ctx.theme;
        ctx.draw.set_screen_height(bounds.y + bounds.h);
        ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius);
        draw_text(ctx, &format!("Render plan rejected: {message}"), bounds.x + 12.0, bounds.y + 24.0, theme.font_size_body, theme.text_muted);
        return;
    }
    let theme = ctx.theme;
    ctx.draw.set_screen_height(bounds.y + bounds.h);
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius);
    match scene.component_kind {
        SurfaceKind::Paint2d => render_paint_2d(scene, bounds, ctx, gpu),
        SurfaceKind::Table => render_table(scene, bounds, ctx),
        SurfaceKind::Canvas2d => render_canvas_2d(scene, bounds, ctx),
        SurfaceKind::NodeGraph => render_node_graph(scene, bounds, ctx, gpu, node_graph_states),
        SurfaceKind::TiledMap => render_tiled_map(scene, bounds, ctx, gpu, tiled_map_states),
        SurfaceKind::VirtualFileSystem => render_vfs(scene, bounds, ctx),
        SurfaceKind::TextEditor => render_text_editor(scene, bounds, ctx, gpu),
        SurfaceKind::InkCanvas => render_ink_canvas(scene, bounds, ctx, gpu),
        SurfaceKind::World3d => {
            let state = world3d_states.entry(scene.surface_id.clone()).or_insert_with(|| World3dState::new(scene.surface_id.clone(), scene.controller_id.clone()));
            render_world_3d(scene, bounds, ctx, state, gpu);
        }
        SurfaceKind::IconRender => render_icon_render(scene, bounds, ctx, gpu, icon_render_states),
        SurfaceKind::Board2d => render_board2d(scene, bounds, ctx, gpu, board2d_states),
        SurfaceKind::GraphTimeline => render_graph_timeline(scene, bounds, ctx),
        SurfaceKind::BlockList => render_block_list(scene, bounds, ctx),
        SurfaceKind::DiffView => render_diff_view(scene, bounds, ctx),
        SurfaceKind::EventFeed => render_event_feed(scene, bounds, ctx),
        // 🚨️ Deliberately exhaustive, no `_` wildcard: every `SurfaceKind` variant has a real arm
        // above, so a future variant addition fails to compile here until it's wired up, instead of
        // silently falling through to `render_placeholder` forever (see `region-claims.json`/this
        // ticket's task 5 — `render_placeholder` itself is kept for other callers that still want an
        // explicit "unimplemented" chrome, e.g. an unresolved `ExternalSlot`).
    }
    // 🐛️➡️✅️ W4 (`.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`): this used to
    // end with `apply_scene_wheel(scene, bounds, ctx); apply_scene_pointer(scene, bounds, ctx);` — a
    // once-per-render-frame sample of the aggregate `InputState` with its own manual "was it down last
    // frame" edge detection, which could drop fast clicks/double-clicks and had asymmetries (e.g. a
    // right-click passthrough silently inert because `pointer_down_screen` no-op'd unless `button ==
    // 0`). Real pointer/wheel input for these 11 surfaces now arrives per real event, hit-tested by
    // `ui_wgpu::wgpu::events::EventRouter::dispatch` and routed here via `UiCommand::Scene` ->
    // `interpreter::apply_scene_ui_command`, which calls the SAME `handle_scene_wheel`/
    // `handle_scene_pointer_button`/`handle_scene_pointer_move` below — so nothing is called from this
    // render pass any longer.
}

/** @emoji 🧭️ Surface kinds that already receive pointer/wheel input through their own bespoke per-frame host state (`world3d_states`/`node_graph_states`/`tiled_map_states`/`board2d_states`, driven directly by the OS event loop) and must not be double-dispatched through the generic `handle_scene_*` handlers below. `pub(crate)` so `interpreter::apply_scene_ui_command` (the real per-event `UiCommand::Scene` handler, and now the ONLY caller of `handle_scene_wheel`/`handle_scene_pointer_button`/`handle_scene_pointer_move` — see that fn's own doc comment) applies this SAME exclusion list. */
pub(crate) fn scene_has_bespoke_pointer_dispatch(kind: SurfaceKind) -> bool {
    matches!(kind, SurfaceKind::World3d | SurfaceKind::NodeGraph | SurfaceKind::TiledMap | SurfaceKind::Board2d)
}

#[cfg(test)]
mod render_entry_tests {
    use super::*;

    #[test]
    fn bespoke_surfaces_are_excluded_from_generic_dispatch() {
        for kind in [SurfaceKind::World3d, SurfaceKind::NodeGraph, SurfaceKind::TiledMap, SurfaceKind::Board2d] {
            assert!(scene_has_bespoke_pointer_dispatch(kind), "{kind:?} should stay on its own bespoke host");
        }
        for kind in [SurfaceKind::Canvas2d, SurfaceKind::Paint2d, SurfaceKind::TextEditor, SurfaceKind::InkCanvas, SurfaceKind::GraphTimeline, SurfaceKind::Table, SurfaceKind::VirtualFileSystem, SurfaceKind::DiffView, SurfaceKind::EventFeed] {
            assert!(!scene_has_bespoke_pointer_dispatch(kind), "{kind:?} previously received no interaction at all and must use the generic handlers");
        }
    }
}
//#endregion RenderEntry

fn render_placeholder(kind: &str, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    draw_text(ctx, &format!("{kind} host"), bounds.x + 12.0, bounds.y + 24.0, theme.font_size_body, theme.text_muted);
}

//#region Paint2d
#[derive(Deserialize, Clone, Copy)]
struct Paint2dCameraFields {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "paint2d_default_one")]
    zoom: f64,
}

impl Default for Paint2dCameraFields {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct Paint2dTransformFields {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "paint2d_default_one")]
    scale_x: f64,
    #[serde(default = "paint2d_default_one")]
    scale_y: f64,
}

impl Default for Paint2dTransformFields {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0 }
    }
}

fn paint2d_default_one() -> f64 {
    1.0
}

fn paint2d_default_true() -> bool {
    true
}

fn paint2d_default_opacity() -> f32 {
    1.0
}

#[derive(Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum Paint2dLayerJson {
    #[serde(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        #[serde(default = "paint2d_default_true")]
        visible: bool,
        #[serde(default = "paint2d_default_opacity")]
        opacity: f32,
        #[serde(default)]
        transform: Paint2dTransformFields,
        width: Option<u32>,
        height: Option<u32>,
        image_key: Option<String>,
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        #[serde(default = "paint2d_default_true")]
        visible: bool,
        #[serde(default = "paint2d_default_opacity")]
        opacity: f32,
        #[serde(default)]
        transform: Paint2dTransformFields,
        #[serde(default)]
        children: Vec<Paint2dLayerJson>,
    },
    #[serde(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {},
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Paint2dDocSyncJson {
    #[serde(default)]
    camera: Paint2dCameraFields,
    #[serde(default)]
    layers: Vec<Paint2dLayerJson>,
}

#[derive(Deserialize)]
struct Paint2dAssetJson {
    mime: String,
    data: String,
}

struct Paint2dFlatLayer {
    id: String,
    image_key: Option<String>,
    x: f64,
    y: f64,
    scale_x: f64,
    scale_y: f64,
    opacity: f32,
    width: u32,
    height: u32,
}

fn collect_paint2d_pixel_layers(layers: &[Paint2dLayerJson], parent_x: f64, parent_y: f64, parent_sx: f64, parent_sy: f64, parent_opacity: f32, out: &mut Vec<Paint2dFlatLayer>) {
    for layer in layers {
        match layer {
            Paint2dLayerJson::Pixel { id, visible, opacity, transform, width, height, image_key } => {
                if !*visible {
                    continue;
                }
                out.push(Paint2dFlatLayer {
                    id: id.clone(),
                    image_key: image_key.clone(),
                    x: parent_x + transform.x * parent_sx,
                    y: parent_y + transform.y * parent_sy,
                    scale_x: parent_sx * transform.scale_x,
                    scale_y: parent_sy * transform.scale_y,
                    opacity: opacity * parent_opacity,
                    width: width.unwrap_or(512),
                    height: height.unwrap_or(512),
                });
            }
            Paint2dLayerJson::Group { visible, opacity, transform, children } => {
                if !*visible {
                    continue;
                }
                collect_paint2d_pixel_layers(children, parent_x + transform.x * parent_sx, parent_y + transform.y * parent_sy, parent_sx * transform.scale_x, parent_sy * transform.scale_y, opacity * parent_opacity, out);
            }
            Paint2dLayerJson::Adjustment { .. } => {}
        }
    }
}

//#region Paint2dNavigator
const PAINT2D_NAVIGATOR_PADDING: f32 = 24.0;

/** 🧭️ Fits a camera to the document's pixel-layer bounds so a `viewMode === "navigator"` surface
 * shows the whole composition — a port of `RasterHost::navigator_fit_camera_json`
 * (`framework/surface/paint/rs/lib.rs`'s premigration `rasterNavigatorFitCamera`; that crate is a
 * sibling used by the React `Paint2dHost`'s WASM raster session and is not wired into this wgpu
 * renderer's dependency graph, so the fit math is reimplemented here with this file's own
 * `Viewport`/`Rect`). Falls back to a neutral centered camera when the document has no pixel content. */
fn paint2d_navigator_fit_viewport(flat: &[Paint2dFlatLayer], inner: Rect) -> Viewport {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for layer in flat {
        let w = layer.width as f64 * layer.scale_x;
        let h = layer.height as f64 * layer.scale_y;
        let x0 = (layer.x - w * 0.5) as f32;
        let y0 = (layer.y - h * 0.5) as f32;
        let x1 = (layer.x + w * 0.5) as f32;
        let y1 = (layer.y + h * 0.5) as f32;
        min_x = min_x.min(x0.min(x1));
        min_y = min_y.min(y0.min(y1));
        max_x = max_x.max(x0.max(x1));
        max_y = max_y.max(y0.max(y1));
    }
    if !min_x.is_finite() || !min_y.is_finite() || !max_x.is_finite() || !max_y.is_finite() {
        return Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
    }
    let content_w = (max_x - min_x).max(1.0);
    let content_h = (max_y - min_y).max(1.0);
    let inner_w = (inner.w - PAINT2D_NAVIGATOR_PADDING * 2.0).max(1.0);
    let inner_h = (inner.h - PAINT2D_NAVIGATOR_PADDING * 2.0).max(1.0);
    let zoom = (inner_w / content_w).min(inner_h / content_h).clamp(0.05, 32.0);
    Viewport { x: min_x + (max_x - min_x) * 0.5, y: min_y + (max_y - min_y) * 0.5, zoom }
}

/** 🧭️ Maps the main (composite) viewport's visible world rect into the navigator's own fitted
 * screen space, producing the "you are here" overlay rectangle — a port of
 * `RasterHost::navigator_viewport_overlay_json`. `content_camera_json` is the main surface's
 * `Paint2dScene.cameraJson` (echoed into the navigator's own scene payload by the owning program) and
 * `content_viewport_json` its reported `compositeViewportJson` (`{width,height}` in CSS px, set via
 * the React reference's `setCompositeViewport` action / this renderer's `ResizeObserver` equivalent
 * — see report for the pointer/resize wiring gap notes). Returns `None` when the main viewport size
 * hasn't been reported yet. */
fn paint2d_navigator_overlay_rect(content_camera_json: &str, content_viewport_json: Option<&str>, navigator_viewport: &Viewport, navigator_inner: Rect) -> Option<Rect> {
    let content_viewport_json = content_viewport_json?;
    let content_camera = Viewport::from_json(content_camera_json);
    let size: Value = serde_json::from_str(content_viewport_json).ok()?;
    let content_w = size.get("width").and_then(Value::as_f64).unwrap_or(0.0) as f32;
    let content_h = size.get("height").and_then(Value::as_f64).unwrap_or(0.0) as f32;
    if content_w <= 0.0 || content_h <= 0.0 {
        return None;
    }
    let content_rect = Rect::new(0.0, 0.0, content_w, content_h);
    let (wx0, wy0) = content_camera.screen_to_world(0.0, 0.0, content_rect);
    let (wx1, wy1) = content_camera.screen_to_world(content_w, content_h, content_rect);
    let (sx0, sy0) = navigator_viewport.world_to_screen(wx0, wy0, navigator_inner);
    let (sx1, sy1) = navigator_viewport.world_to_screen(wx1, wy1, navigator_inner);
    Some(Rect::new(sx0.min(sx1), sy0.min(sy1), (sx1 - sx0).abs(), (sy1 - sy0).abs()))
}
//#endregion Paint2dNavigator

/** 🖼️ Composites paint-2d document layers as textured quads; blend modes, masks and adjustment layers are not yet applied (see FIX-LOWPOLY-DEV-BOOT sibling ticket 26/07/11/WGPU-RENDERER-FULL-PARITY for follow-up scope). `viewMode === "navigator"` renders the same layer stack fit-to-view with a composite-viewport overlay instead of following the local/camera viewport. */
fn render_paint_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::wgpu::GpuContext) {
    let theme = ctx.theme;
    let Some(paint_2d) = &scene.paint_2d else {
        return render_placeholder("paint-2d", bounds, ctx);
    };
    let _ = gpu;
    let inner = bounds;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let doc: Paint2dDocSyncJson = serde_json::from_str(&paint_2d.document_sync_json).unwrap_or_default();
    let assets: HashMap<String, Paint2dAssetJson> = serde_json::from_str(&paint_2d.assets_json).unwrap_or_default();
    let is_navigator = paint_2d.view_mode == "navigator";
    let mut flat = Vec::new();
    collect_paint2d_pixel_layers(&doc.layers, 0.0, 0.0, 1.0, 1.0, 1.0, &mut flat);
    let viewport = if is_navigator {
        paint2d_navigator_fit_viewport(&flat, inner)
    } else {
        let mut vp = Viewport::from_json(&paint_2d.camera_json);
        if vp.zoom <= 0.0 {
            vp = Viewport { x: doc.camera.x as f32, y: doc.camera.y as f32, zoom: doc.camera.zoom as f32 };
        }
        let local = scene_state(&scene.surface_id);
        if local.viewport.zoom > 0.0 {
            vp = local.viewport;
        }
        vp
    };
    draw_checkerboard(ctx.draw, &viewport, inner, theme, 4096.0);
    if flat.is_empty() {
        draw_text(ctx, "Empty paint-2d document", inner.x + 8.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
    }
    for layer in &flat {
        let w = (layer.width as f32 * layer.scale_x as f32 * viewport.zoom).max(1.0);
        let h = (layer.height as f32 * layer.scale_y as f32 * viewport.zoom).max(1.0);
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let quad = [sx - w * 0.5, sy - h * 0.5, w, h];
        if let Some(image_key) = &layer.image_key {
            let Some(asset) = assets.get(image_key) else {
                ctx.draw.push_solid(quad, theme.panel.with_alpha(layer.opacity.clamp(0.0, 1.0)));
                continue;
            };
            let data_url = format!("data:{};base64,{}", asset.mime, asset.data);
            let Some(key) = queue_canvas_image_upload(&scene.surface_id, &layer.id, &data_url) else {
                ctx.draw.push_solid(quad, theme.panel.with_alpha(layer.opacity.clamp(0.0, 1.0)));
                continue;
            };
            ctx.draw.push_raster_quad(&key, quad, [0.0, 0.0, 1.0, 1.0], layer.opacity);
        } else {
            ctx.draw.push_solid(quad, theme.panel.with_alpha(layer.opacity.clamp(0.0, 1.0)));
        }
    }
    // 🧭️ "You are here" overlay: the main surface's visible world rect, mapped into this
    // navigator's fitted screen space — matches the React reference's `overlayRect` border div.
    if is_navigator {
        if let Some(overlay) = paint2d_navigator_overlay_rect(&paint_2d.camera_json, paint_2d.composite_viewport_json.as_deref(), &viewport, inner) {
            draw_ink_rect_outline(ctx.draw, overlay.x, overlay.y, overlay.w, overlay.h, theme.accent, 2.0);
        }
    }
    ctx.input.register_hit(HitTarget {
        rect: inner,
        event: Some(scene_action(scene, "paint2dClick", json!({ "surfaceId": scene.surface_id, "activeUtility": paint_2d.active_utility, "brushSize": paint_2d.brush_size, "brushOpacity": paint_2d.brush_opacity }))),
        control_id: Some(scene.surface_id.clone()),
        kind: HitKind::Generic,
        drag_axis: None,
        drag_data: None,
    });
}
//#endregion Paint2d

//#region Table
#[derive(Deserialize)]
struct TableColumn {
    id: String,
    label: String,
    #[serde(default)]
    sortable: bool,
}

/// 🔀️ Mirrors `sourcing::TableSort`'s wire format (`{columnId, direction}`) — the active sort
/// state for a [`TableScene`] whose columns opt into `sortable`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableSortJson {
    column_id: String,
    direction: String,
}

/// 🧾️ Mirrors `ui_wgpu::wgpu::TableCell` — a typed table cell value parsed out of a row's raw JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum TableCellPayload {
    Text { value: String },
    Number { value: f64 },
    Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
    Buttons { buttons: Vec<TableCellButtonPayload> },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TableCellButtonPayload {
    icon_id: String,
    #[serde(default)]
    label: Option<String>,
    action: ActionDescriptor,
}

/// 🔗️ Merges `patch` into `base`'s existing args (rather than replacing them), so a stepper/button cell keeps its row-identifying args (e.g. `objectId`) alongside the delta/click patch.
fn merge_action_args(base: &ActionDescriptor, patch: Value) -> ActionDescriptor {
    let mut args = match &base.args {
        Some(dsl) => match semio_framework::from_dsl_value::<Value>(dsl.clone()) {
            Ok(Value::Object(map)) => map.clone(),
            _ => serde_json::Map::new(),
        },
        None => serde_json::Map::new(),
    };
    if let Value::Object(patch_map) = patch {
        args.extend(patch_map);
    }
    ActionDescriptor { controller_id: base.controller_id.clone(), action: base.action.clone(), args: semio_framework::optional_json_to_dsl(Some(Value::Object(args))) }
}

/// 🧾️ Renders a table cell's interactive controls (stepper/buttons) directly, or returns the plain text to draw for text/number/legacy-string cells.
fn render_table_cell(cell: &Value, rect: Rect, ctx: &mut FrameworkWidgetContext<'_>) -> Option<String> {
    let Ok(payload) = serde_json::from_value::<TableCellPayload>(cell.clone()) else {
        return Some(match cell {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        });
    };
    match payload {
        TableCellPayload::Text { value } => Some(value),
        TableCellPayload::Number { value } => Some(value.to_string()),
        TableCellPayload::Stepper { value, min, max, step, action } => {
            let seg = rect.w / 3.0;
            let minus = Rect::new(rect.x, rect.y, seg, rect.h);
            let center = Rect::new(rect.x + seg, rect.y, seg, rect.h);
            let plus = Rect::new(rect.x + seg * 2.0, rect.y, seg, rect.h);
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "−".into(), event: (value > min).then(|| merge_action_args(&action, json!({ "delta": -step }))) }, minus, ctx);
            render_widget(&WidgetNode::Text { value: format!("{value:.0}"), emphasize: false }, center, ctx);
            render_widget(&WidgetNode::Button { id: None, icon_id: None, label: "+".into(), event: (value < max).then(|| merge_action_args(&action, json!({ "delta": step }))) }, plus, ctx);
            None
        }
        TableCellPayload::Buttons { buttons } => {
            let seg = if buttons.is_empty() { rect.w } else { rect.w / buttons.len() as f32 };
            for (index, button) in buttons.iter().enumerate() {
                let button_rect = Rect::new(rect.x + index as f32 * seg, rect.y, seg, rect.h);
                render_widget(&WidgetNode::Button { id: None, icon_id: IconName::from_str(&button.icon_id), label: button.label.clone().unwrap_or_default(), event: Some(button.action.clone()) }, button_rect, ctx);
            }
            None
        }
    }
}

fn render_table(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(table) = &scene.table else {
        return render_placeholder("table", bounds, ctx);
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let selected_ids: Vec<String> =
        table.selection_json.as_deref().and_then(|json| serde_json::from_str::<Value>(json).ok()).and_then(|value| value.get("selectedIds").cloned()).and_then(|value| serde_json::from_value(value).ok()).unwrap_or_default();
    let sort: Option<TableSortJson> = table.sort_json.as_deref().and_then(|json| serde_json::from_str(json).ok());
    let inner = bounds;
    let header_h = theme.control_height * 1.33;
    let row_h = theme.control_height;
    let pad = theme.padding_standard;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    let col_w = if columns.is_empty() { inner.w } else { inner.w / columns.len() as f32 };
    for (index, column) in columns.iter().enumerate() {
        let x = inner.x + index as f32 * col_w;
        let sorted_here = sort.as_ref().filter(|s| s.column_id == column.id);
        let label = match sorted_here {
            Some(s) if s.direction == "desc" => format!("{} \u{25BC}", column.label),
            Some(_) => format!("{} \u{25B2}", column.label),
            None => column.label.clone(),
        };
        draw_text(ctx, &label, x + pad, inner.y + header_h * 0.65, theme.font_size_small, if sorted_here.is_some() { theme.text } else { theme.text_muted });
        if column.sortable {
            let next_direction = match sorted_here {
                Some(s) if s.direction == "asc" => "desc",
                _ => "asc",
            };
            ctx.input.register_hit(HitTarget {
                rect: Rect::new(x, inner.y, col_w, header_h),
                event: Some(scene_action(scene, "sortTable", json!({ "surfaceId": scene.surface_id, "columnId": column.id, "direction": next_direction }))),
                control_id: Some(format!("{}.header.{}", scene.surface_id, column.id)),
                kind: HitKind::Generic,
                drag_axis: None,
                drag_data: None,
            });
        }
    }
    ctx.draw.push_line(inner.x, inner.y + header_h, inner.x + inner.w, inner.y + header_h, theme.separator, 1.0);
    let body = Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h);
    let scroll = scroll_offset(&scene.surface_id, "body");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.surface_id, "body")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    let hovered_row = ctx.input.hovered_id.clone();
    if rows.is_empty() {
        let message = "No rows";
        draw_text(ctx, message, body.x + body.w * 0.5 - 40.0, body.y + body.h * 0.5, theme.font_size_small, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = row.get("id").or_else(|| row.get("pluginId")).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let control_id = format!("{}.row.{}", scene.surface_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let selected = selected_ids.iter().any(|id| id == &row_id);
        if selected {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);
        for (col_index, column) in columns.iter().enumerate() {
            let x = body.x + col_index as f32 * col_w;
            let cell_rect = Rect::new(x + pad, y, col_w - pad * 2.0, row_h);
            let text = match row.get(&column.id) {
                Some(value) => render_table_cell(value, cell_rect, ctx),
                None => Some("—".into()),
            };
            if let Some(text) = text {
                draw_text(ctx, &text, x + pad, y + row_h * 0.65, theme.font_size_small, if selected || hovered { theme.active_foreground } else { theme.text });
            }
        }
        let drag_data = table.row_drag_mime.as_ref().and_then(|mime| row.get("_drag").map(|payload| HashMap::from([(mime.clone(), payload.to_string())])));
        ctx.input.register_hit(HitTarget { rect: row_rect, event: Some(scene_action(scene, "selectRow", json!({ "surfaceId": scene.surface_id, "row": row }))), control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data });
    }
    ctx.draw.pop_scissor();
}
//#endregion Table

//#region TableTests
#[cfg(test)]
mod table_tests {
    use super::*;
    use ui_wgpu::wgpu::{DrawList, FontAtlas, IconAtlas, InputState, TableScene};

    fn table_scene(surface_id: &str, table: TableScene) -> UiComponentSceneNode {
        UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::Table,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: Some(table),
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: None,
            menu: None,
        }
    }

    /// 🧪️ Renders `node` and returns the `InputState` so tests can inspect registered hit targets.
    fn render(node: &UiComponentSceneNode) -> InputState<ActionDescriptor> {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            render_table(node, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
        }
        input
    }

    fn hit<'a>(input: &'a InputState<ActionDescriptor>, control_id: &str) -> &'a HitTarget<ActionDescriptor> {
        input.hit_targets.iter().find(|target| target.control_id.as_deref() == Some(control_id)).unwrap_or_else(|| panic!("no hit target registered for control_id {control_id:?}"))
    }

    fn columns_json(entries: &[(&str, &str, bool)]) -> String {
        json!(entries.iter().map(|(id, label, sortable)| json!({ "id": id, "label": label, "sortable": sortable })).collect::<Vec<_>>()).to_string()
    }

    #[test]
    fn header_click_on_unsorted_sortable_column_requests_ascending() {
        let table = TableScene::base(columns_json(&[("name", "Name", true)]), "[]".to_string());
        let node = table_scene("s1", table);
        let input = render(&node);
        let target = hit(&input, "s1.header.name");
        let action = target.event.as_ref().expect("sortTable action");
        assert_eq!(action.action, "sortTable");
        assert_eq!(action.args.as_ref().and_then(|args| args.get("columnId")).and_then(semio_framework::DslValue::as_str), Some("name"));
        assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("asc"));
    }

    #[test]
    fn header_click_toggles_ascending_to_descending() {
        let mut table = TableScene::base(columns_json(&[("name", "Name", true)]), "[]".to_string());
        table.sort_json = Some(json!({ "columnId": "name", "direction": "asc" }).to_string());
        let node = table_scene("s1", table);
        let input = render(&node);
        let target = hit(&input, "s1.header.name");
        let action = target.event.as_ref().expect("sortTable action");
        assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("desc"));
    }

    #[test]
    fn header_click_cycles_descending_back_to_ascending() {
        let mut table = TableScene::base(columns_json(&[("name", "Name", true)]), "[]".to_string());
        table.sort_json = Some(json!({ "columnId": "name", "direction": "desc" }).to_string());
        let node = table_scene("s1", table);
        let input = render(&node);
        let target = hit(&input, "s1.header.name");
        let action = target.event.as_ref().expect("sortTable action");
        assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("asc"));
    }

    #[test]
    fn sorting_a_column_does_not_reset_a_different_column_to_desc() {
        let mut table = TableScene::base(columns_json(&[("name", "Name", true), ("age", "Age", true)]), "[]".to_string());
        table.sort_json = Some(json!({ "columnId": "age", "direction": "asc" }).to_string());
        let node = table_scene("s1", table);
        let input = render(&node);
        let target = hit(&input, "s1.header.name");
        let action = target.event.as_ref().expect("sortTable action");
        // 🔀️ "name" isn't the currently-sorted column, so clicking it must start a fresh ascending sort, not toggle.
        assert_eq!(action.args.as_ref().and_then(|args| args.get("direction")).and_then(semio_framework::DslValue::as_str), Some("asc"));
    }

    #[test]
    fn non_sortable_column_registers_no_header_hit() {
        let table = TableScene::base(columns_json(&[("name", "Name", false)]), "[]".to_string());
        let node = table_scene("s1", table);
        let input = render(&node);
        assert!(input.hit_targets.iter().all(|target| target.control_id.as_deref() != Some("s1.header.name")), "a non-sortable column must not register a header sort hit target");
    }

    #[test]
    fn row_click_dispatches_select_row_with_full_row_payload() {
        let rows = json!([{ "id": "r1", "name": { "kind": "text", "value": "Alpha" } }]).to_string();
        let table = TableScene::base(columns_json(&[("name", "Name", false)]), rows);
        let node = table_scene("s1", table);
        let input = render(&node);
        let target = hit(&input, "s1.row.r1");
        let action = target.event.as_ref().expect("selectRow action");
        assert_eq!(action.action, "selectRow");
        assert_eq!(action.args.as_ref().and_then(|args| args.get("row")).and_then(|row| row.get("id")).and_then(semio_framework::DslValue::as_str), Some("r1"));
    }
}
//#endregion TableTests

//#region BlockList
/// 🧩️ Mirrors `playbook::PlaybookBlock`'s renderer-relevant fields — a typed block inside a
/// [`BlockListScene`] step. Unknown/extra JSON fields (the block-kind-specific property editor
/// fields owned by the host app) are ignored by `serde` since this crate never edits them.
#[derive(Deserialize)]
struct BlockListBlockJson {
    id: String,
    label: String,
    kind: String,
    #[serde(default)]
    description: Option<String>,
}

/// 🧩️ Mirrors `playbook::PlaybookStep`'s renderer-relevant fields.
#[derive(Deserialize)]
struct BlockListStepJson {
    id: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    blocks: Vec<BlockListBlockJson>,
}

/// 🧩️ Mirrors `ui_wgpu::wgpu::BlockPaletteEntry`'s wire format (`{blockKind, label, iconId}`).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockListPaletteEntryJson {
    block_kind: String,
    label: String,
    #[serde(default)]
    icon_id: String,
}

/// 🧩️ Renders the strict-list Blockly-like block-list builder ([`SurfaceKind::BlockList`]):
/// steps stacked vertically (each with its ordered blocks) plus a palette rail for inserting new
/// blocks, mirroring `block-list-host.tsx`'s layout and action verbs (`addStep`/`removeStep`/
/// `moveStep`/`addBlock`/`removeBlock`/`moveBlock`, see `playbook::builder_kit` and
/// `playbook-plugin`'s `handle_action`). Reordering dispatches `moveStep`/`moveBlock` from
/// move-up/move-down hit targets rather than free-form pointer drag (unlike the React host's
/// dnd-kit drag-and-drop): this renderer's click-dispatch model (see `render_table`'s row/header
/// hits) has no established cross-frame drag-position-tracking primitive for list reordering, and
/// building one is out of this ticket's scope (`w2-scene-wiring` owns generic pointer routing).
/// Selection highlighting reads `selected_id` directly (mirrors `render_table`'s selected-row
/// highlight) even though `block-list-host.tsx` does not yet render it either.
fn render_block_list(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(block_list) = &scene.block_list else {
        return render_placeholder("block-list", bounds, ctx);
    };
    let steps: Vec<BlockListStepJson> = serde_json::from_str(&block_list.steps_json).unwrap_or_default();
    let palette: Vec<BlockListPaletteEntryJson> = serde_json::from_str(&block_list.palette_json).unwrap_or_default();
    let selected_id = block_list.selected_id.as_deref();

    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    let btn_w = 26.0;
    let palette_w = (bounds.w * 0.22).clamp(140.0, 220.0);
    let main = Rect::new(bounds.x, bounds.y, (bounds.w - palette_w).max(0.0), bounds.h);
    let palette_rect = Rect::new(main.x + main.w, bounds.y, palette_w, bounds.h);

    //#region Steps
    let header_rect = Rect::new(main.x, main.y, main.w, row_h);
    draw_text(ctx, "Steps", header_rect.x + pad, header_rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    let add_step_rect = Rect::new(header_rect.x + header_rect.w - 108.0, header_rect.y + 4.0, 104.0, row_h - 8.0);
    render_widget(&WidgetNode::Button { id: Some(format!("{}.addStep", scene.surface_id)), icon_id: Some("plus".into()), label: "Add Step".into(), event: Some(scene_action(scene, "addStep", json!({}))) }, add_step_rect, ctx);

    let body = Rect::new(main.x, main.y + row_h, main.w, (main.h - row_h).max(0.0));
    let scroll = scroll_offset(&scene.surface_id, "blockList");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.surface_id, "blockList")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    if steps.is_empty() {
        draw_text(ctx, "No steps", body.x + pad, body.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    let step_count = steps.len();
    let mut y = body.y - scroll;
    for (step_index, step) in steps.iter().enumerate() {
        let block_count = step.blocks.len();
        let description_h = step.description.as_ref().map_or(0.0, |_| theme.font_size_small + pad);
        let step_h = row_h + description_h + block_count as f32 * row_h + pad * 2.0;
        if y + step_h >= body.y && y <= body.y + body.h {
            let step_rect = Rect::new(body.x + pad, y, (body.w - pad * 2.0).max(0.0), step_h);
            let step_selected = selected_id == Some(step.id.as_str());
            ctx.draw.push_rounded([step_rect.x, step_rect.y, step_rect.w, step_rect.h], if step_selected { theme.selected } else { theme.button }, theme.border_radius);
            // 🖼️ Full four-side stroke, matching `StepCard`'s `border border-border` box in
            // `index.tsx` — previously only top/bottom hairlines were drawn, so cards had no visible
            // left/right edge.
            draw_ink_rect_outline(ctx.draw, step_rect.x, step_rect.y, step_rect.w, step_rect.h, theme.border_normal, theme.stroke_hairline);

            let title_color = if step_selected { theme.active_foreground } else { theme.text };
            draw_text(ctx, &step.title, step_rect.x + pad, step_rect.y + row_h * 0.65, theme.font_size_body, title_color);

            let btn_y = step_rect.y + (row_h - theme.control_height_small) * 0.5;
            let up_rect = Rect::new(step_rect.x + step_rect.w - pad - btn_w * 3.0, btn_y, btn_w, theme.control_height_small);
            let down_rect = Rect::new(step_rect.x + step_rect.w - pad - btn_w * 2.0, btn_y, btn_w, theme.control_height_small);
            let remove_rect = Rect::new(step_rect.x + step_rect.w - pad - btn_w, btn_y, btn_w, theme.control_height_small);
            render_widget(
                &WidgetNode::Button {
                    id: Some(format!("{}.step.{}.moveUp", scene.surface_id, step.id)),
                    icon_id: Some("chevron-up".into()),
                    label: String::new(),
                    event: (step_index > 0).then(|| scene_action(scene, "moveStep", json!({ "stepId": step.id, "index": step_index - 1 }))),
                },
                up_rect,
                ctx,
            );
            render_widget(
                &WidgetNode::Button {
                    id: Some(format!("{}.step.{}.moveDown", scene.surface_id, step.id)),
                    icon_id: Some("chevron-down".into()),
                    label: String::new(),
                    event: (step_index + 1 < step_count).then(|| scene_action(scene, "moveStep", json!({ "stepId": step.id, "index": step_index + 1 }))),
                },
                down_rect,
                ctx,
            );
            render_widget(
                &WidgetNode::Button { id: Some(format!("{}.step.{}.remove", scene.surface_id, step.id)), icon_id: Some("trash-2".into()), label: String::new(), event: Some(scene_action(scene, "removeStep", json!({ "stepId": step.id }))) },
                remove_rect,
                ctx,
            );

            let mut inner_y = step_rect.y + row_h;
            if let Some(description) = &step.description {
                draw_text(ctx, description, step_rect.x + pad, inner_y + theme.font_size_small, theme.font_size_small, theme.text_muted);
                inner_y += description_h;
            }
            for (block_index, block) in step.blocks.iter().enumerate() {
                let block_rect = Rect::new(step_rect.x + pad, inner_y, (step_rect.w - pad * 2.0).max(0.0), row_h);
                let block_selected = selected_id == Some(block.id.as_str());
                if block_selected {
                    ctx.draw.push_rounded([block_rect.x, block_rect.y, block_rect.w, block_rect.h], theme.selected, theme.border_radius.min(4.0));
                }
                let block_color = if block_selected { theme.active_foreground } else { theme.text };
                draw_text(ctx, &block.label, block_rect.x + pad, block_rect.y + row_h * 0.65, theme.font_size_small, block_color);
                draw_text(ctx, &block.kind, block_rect.x + block_rect.w * 0.5, block_rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);

                let bblock_btn_y = block_rect.y + (row_h - theme.control_height_small) * 0.5;
                let bup_rect = Rect::new(block_rect.x + block_rect.w - pad - btn_w * 3.0, bblock_btn_y, btn_w, theme.control_height_small);
                let bdown_rect = Rect::new(block_rect.x + block_rect.w - pad - btn_w * 2.0, bblock_btn_y, btn_w, theme.control_height_small);
                let bremove_rect = Rect::new(block_rect.x + block_rect.w - pad - btn_w, bblock_btn_y, btn_w, theme.control_height_small);
                render_widget(
                    &WidgetNode::Button {
                        id: Some(format!("{}.block.{}.moveUp", scene.surface_id, block.id)),
                        icon_id: Some("chevron-up".into()),
                        label: String::new(),
                        event: (block_index > 0).then(|| scene_action(scene, "moveBlock", json!({ "blockId": block.id, "fromStepId": step.id, "toStepId": step.id, "index": block_index - 1 }))),
                    },
                    bup_rect,
                    ctx,
                );
                render_widget(
                    &WidgetNode::Button {
                        id: Some(format!("{}.block.{}.moveDown", scene.surface_id, block.id)),
                        icon_id: Some("chevron-down".into()),
                        label: String::new(),
                        event: (block_index + 1 < block_count).then(|| scene_action(scene, "moveBlock", json!({ "blockId": block.id, "fromStepId": step.id, "toStepId": step.id, "index": block_index + 1 }))),
                    },
                    bdown_rect,
                    ctx,
                );
                render_widget(
                    &WidgetNode::Button {
                        id: Some(format!("{}.block.{}.remove", scene.surface_id, block.id)),
                        icon_id: Some("trash-2".into()),
                        label: String::new(),
                        event: Some(scene_action(scene, "removeBlock", json!({ "stepId": step.id, "blockId": block.id }))),
                    },
                    bremove_rect,
                    ctx,
                );
                inner_y += row_h;
            }
        }
        y += step_h + theme.gap_standard;
    }
    ctx.draw.pop_scissor();
    //#endregion Steps

    //#region Palette
    ctx.draw.push_line(palette_rect.x, palette_rect.y, palette_rect.x, palette_rect.y + palette_rect.h, theme.separator, theme.stroke_hairline);
    draw_text(ctx, "Palette", palette_rect.x + pad, palette_rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    let mut py = palette_rect.y + row_h;
    for entry in &palette {
        let entry_rect = Rect::new(palette_rect.x + pad, py, (palette_rect.w - pad * 2.0).max(0.0), row_h);
        render_widget(
            &WidgetNode::Button {
                id: Some(format!("{}.palette.{}", scene.surface_id, entry.block_kind)),
                icon_id: IconName::from_str(&entry.icon_id),
                label: entry.label.clone(),
                event: Some(scene_action(scene, "addBlock", json!({ "kind": entry.block_kind }))),
            },
            entry_rect,
            ctx,
        );
        py += row_h + 2.0;
    }
    //#endregion Palette
}
//#endregion BlockList

//#region BlockListTests
#[cfg(test)]
mod block_list_tests {
    use super::*;
    use ui_wgpu::wgpu::{BlockListScene, DrawList, FontAtlas, IconAtlas, InputState};

    fn block_list_scene(surface_id: &str, block_list: BlockListScene) -> UiComponentSceneNode {
        UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::BlockList,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: Some(block_list),
            menu: None,
        }
    }

    fn step_json(id: &str, blocks: &[(&str, &str, &str)]) -> Value {
        json!({
            "id": id,
            "title": format!("Step {id}"),
            "blocks": blocks.iter().map(|(bid, label, kind)| json!({ "id": bid, "label": label, "kind": kind })).collect::<Vec<_>>(),
        })
    }

    /// 🧪️ Renders `node` and returns the `InputState` so tests can inspect registered hit targets.
    fn render(node: &UiComponentSceneNode) -> InputState<ActionDescriptor> {
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            render_block_list(node, Rect::new(0.0, 0.0, 600.0, 400.0), &mut ctx);
        }
        input
    }

    fn hit<'a>(input: &'a InputState<ActionDescriptor>, control_id: &str) -> &'a HitTarget<ActionDescriptor> {
        input.hit_targets.iter().find(|target| target.control_id.as_deref() == Some(control_id)).unwrap_or_else(|| panic!("no hit target registered for control_id {control_id:?}"))
    }

    fn find_hit<'a>(input: &'a InputState<ActionDescriptor>, control_id: &str) -> Option<&'a HitTarget<ActionDescriptor>> {
        input.hit_targets.iter().find(|target| target.control_id.as_deref() == Some(control_id))
    }

    #[test]
    fn missing_scene_renders_placeholder_without_panicking() {
        let node = UiComponentSceneNode {
            surface_id: "s1".into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::BlockList,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: None,
            menu: None,
        };
        render(&node);
    }

    #[test]
    fn add_step_button_dispatches_add_step() {
        let scene = BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: None, dragging_id: None };
        let node = block_list_scene("s1", scene);
        let input = render(&node);
        let target = hit(&input, "s1.addStep");
        let action = target.event.as_ref().expect("addStep action");
        assert_eq!(action.action, "addStep");
    }

    #[test]
    fn palette_entry_dispatches_add_block_with_kind() {
        let palette = json!([{ "blockKind": "text", "label": "Text", "iconId": "type" }]).to_string();
        let scene = BlockListScene { steps_json: "[]".into(), palette_json: palette, selected_id: None, dragging_id: None };
        let node = block_list_scene("s1", scene);
        let input = render(&node);
        let target = hit(&input, "s1.palette.text");
        let action = target.event.as_ref().expect("addBlock action");
        assert_eq!(action.action, "addBlock");
        assert_eq!(action.args.as_ref().and_then(|args| args.get("kind")).and_then(semio_framework::DslValue::as_str), Some("text"));
    }

    #[test]
    fn first_step_has_no_move_up_but_has_move_down_when_a_second_step_exists() {
        let steps = json!([step_json("a", &[]), step_json("b", &[])]).to_string();
        let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None };
        let node = block_list_scene("s1", scene);
        let input = render(&node);
        assert!(hit(&input, "s1.step.a.moveUp").event.is_none(), "the first step must not be able to move further up");
        let down = hit(&input, "s1.step.a.moveDown");
        let action = down.event.as_ref().expect("moveStep action");
        assert_eq!(action.action, "moveStep");
        assert_eq!(action.args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
        assert_eq!(action.args.as_ref().and_then(|args| args.get("index")).and_then(semio_framework::DslValue::as_f64), Some(1.0));
    }

    #[test]
    fn last_step_has_no_move_down() {
        let steps = json!([step_json("a", &[]), step_json("b", &[])]).to_string();
        let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None };
        let node = block_list_scene("s1", scene);
        let input = render(&node);
        assert!(hit(&input, "s1.step.b.moveDown").event.is_none(), "the last step must not be able to move further down");
    }

    #[test]
    fn remove_step_button_dispatches_remove_step_with_step_id() {
        let steps = json!([step_json("a", &[])]).to_string();
        let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None };
        let node = block_list_scene("s1", scene);
        let input = render(&node);
        let target = hit(&input, "s1.step.a.remove");
        let action = target.event.as_ref().expect("removeStep action");
        assert_eq!(action.action, "removeStep");
        assert_eq!(action.args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
    }

    #[test]
    fn block_move_and_remove_dispatch_expected_action_shapes() {
        let steps = json!([step_json("a", &[("b1", "Block One", "text"), ("b2", "Block Two", "number")])]).to_string();
        let scene = BlockListScene { steps_json: steps, palette_json: "[]".into(), selected_id: None, dragging_id: None };
        let node = block_list_scene("s1", scene);
        let input = render(&node);

        assert!(hit(&input, "s1.block.b1.moveUp").event.is_none(), "the first block in a step must not move further up");
        let move_down = hit(&input, "s1.block.b1.moveDown");
        let move_action = move_down.event.as_ref().expect("moveBlock action");
        assert_eq!(move_action.action, "moveBlock");
        assert_eq!(move_action.args.as_ref().and_then(|args| args.get("blockId")).and_then(semio_framework::DslValue::as_str), Some("b1"));
        assert_eq!(move_action.args.as_ref().and_then(|args| args.get("fromStepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
        assert_eq!(move_action.args.as_ref().and_then(|args| args.get("toStepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
        assert_eq!(move_action.args.as_ref().and_then(|args| args.get("index")).and_then(semio_framework::DslValue::as_f64), Some(1.0));

        assert!(hit(&input, "s1.block.b2.moveDown").event.is_none(), "the last block in a step must not move further down");

        let remove = hit(&input, "s1.block.b1.remove");
        let remove_action = remove.event.as_ref().expect("removeBlock action");
        assert_eq!(remove_action.action, "removeBlock");
        assert_eq!(remove_action.args.as_ref().and_then(|args| args.get("stepId")).and_then(semio_framework::DslValue::as_str), Some("a"));
        assert_eq!(remove_action.args.as_ref().and_then(|args| args.get("blockId")).and_then(semio_framework::DslValue::as_str), Some("b1"));
    }

    #[test]
    fn empty_steps_registers_no_step_hit_targets() {
        let scene = BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: None, dragging_id: None };
        let node = block_list_scene("s1", scene);
        let input = render(&node);
        assert!(find_hit(&input, "s1.step.a.remove").is_none());
    }

    //#region BlockListPaintTests
    #[test]
    fn step_card_draws_a_full_four_sided_border_not_just_top_and_bottom() {
        // 🖼️ Unit-tests `draw_ink_rect_outline` directly (the helper `render_block_list`'s step-card
        // border calls) rather than filtering `render_block_list`'s full draw output by color: `theme
        // .separator` and `theme.border_normal` are byte-identical by design (both derive from
        // `chrome.border_normal` in `Theme::from_chrome`), and the card's right edge sits only a few
        // px from the unrelated main/palette divider line — too tight a margin for a position filter
        // to reliably separate the two from the full scene, so isolate the helper instead.
        let mut draw = DrawList::default();
        let color = Theme::default().border_normal;
        draw_ink_rect_outline(&mut draw, 10.0, 20.0, 200.0, 80.0, color, 1.0);
        let border_vertex_count = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).filter(|v| v.color == [color.r, color.g, color.b, color.a]).count();
        assert_eq!(border_vertex_count, 24, "4 lines (top/right/bottom/left) * 6 vertices should emit 24, got {border_vertex_count}");
    }
    //#endregion BlockListPaintTests
}
//#endregion BlockListTests

//#region DiffView
#[derive(Clone, Copy, PartialEq, Debug)]
enum DiffLineOperation {
    Equal,
    Removed,
    Added,
}

#[derive(Clone, Copy, Debug)]
struct DiffLine<'a> {
    operation: DiffLineOperation,
    text: &'a str,
}

/// 🧮️ Above this many `before.len() * after.len()` DP cells, [`diff_lines`] skips the LCS table and
/// falls back to a positional compare so a single huge [`SurfaceKind::DiffView`] payload can't blow
/// up per-frame recompute cost (this crate re-derives the diff every render pass, mirroring how
/// `render_graph_timeline` re-parses `columns_json` every frame rather than caching it).
const DIFF_LCS_CELL_BUDGET: usize = 200_000;

/// 🔀️ Line-level LCS diff (classic DP backtrace). Falls back to a naive positional compare above
/// [`DIFF_LCS_CELL_BUDGET`] cells.
fn diff_lines<'a>(before: &[&'a str], after: &[&'a str]) -> Vec<DiffLine<'a>> {
    let (n, m) = (before.len(), after.len());
    if n.saturating_mul(m) > DIFF_LCS_CELL_BUDGET {
        let mut out = Vec::with_capacity(n + m);
        for i in 0..n.max(m) {
            match (before.get(i).copied(), after.get(i).copied()) {
                (Some(b), Some(a)) if b == a => out.push(DiffLine { operation: DiffLineOperation::Equal, text: b }),
                (Some(b), Some(a)) => {
                    out.push(DiffLine { operation: DiffLineOperation::Removed, text: b });
                    out.push(DiffLine { operation: DiffLineOperation::Added, text: a });
                }
                (Some(b), None) => out.push(DiffLine { operation: DiffLineOperation::Removed, text: b }),
                (None, Some(a)) => out.push(DiffLine { operation: DiffLineOperation::Added, text: a }),
                (None, None) => {}
            }
        }
        return out;
    }
    let mut table = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            table[i][j] = if before[i] == after[j] { table[i + 1][j + 1] + 1 } else { table[i + 1][j].max(table[i][j + 1]) };
        }
    }
    let mut out = Vec::with_capacity(n + m);
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if before[i] == after[j] {
            out.push(DiffLine { operation: DiffLineOperation::Equal, text: before[i] });
            i += 1;
            j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i] });
            i += 1;
        } else {
            out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j] });
            j += 1;
        }
    }
    while i < n {
        out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i] });
        i += 1;
    }
    while j < m {
        out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j] });
        j += 1;
    }
    out
}

/// 🩹️ Renders [`SurfaceKind::DiffView`]: a line-level diff of `before`/`after` text, either as a
/// single scrolling column with `+`/`-` markers (default, or `mode: "unified"`) or as two aligned
/// columns (`mode: "split"`). Text-only — no syntax highlighting, gutter line numbers, or monospace
/// font for `language` yet (the latter two need capabilities this crate's `draw_text`/layout don't
/// have). Add/remove **text** is tinted with the theme's `accent`/`error` tokens (equal text stays
/// full-brightness `theme.text`), matching `DIFF_LINE_CLASS`'s per-line text-color classes in
/// `diff-view-host.tsx` — this used to instead wash the whole row background and dim the *unchanged*
/// majority of lines, the opposite of what the React source of truth does.
fn render_diff_view(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(diff) = &scene.diff_view else {
        return render_placeholder("diff-view", bounds, ctx);
    };
    let before_lines: Vec<&str> = diff.before.split('\n').collect();
    let after_lines: Vec<&str> = diff.after.split('\n').collect();
    let operations = diff_lines(&before_lines, &after_lines);
    let inner = bounds;
    let pad = theme.padding_standard;
    let row_h = theme.font_size_small + pad * 0.5;
    let split = diff.mode.as_deref() == Some("split");

    let scroll = scroll_offset(&scene.surface_id, "diff");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "diff")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    if operations.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        ctx.draw.pop_scissor();
        return;
    }

    let col_w = if split { (inner.w * 0.5).max(1.0) } else { inner.w };
    let right_x = inner.x + col_w;
    for (row_index, line) in operations.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        if split {
            match line.operation {
                DiffLineOperation::Removed => {
                    draw_text(ctx, line.text, inner.x + pad, y + row_h * 0.7, theme.font_size_small, theme.error);
                }
                DiffLineOperation::Added => {
                    draw_text(ctx, line.text, right_x + pad, y + row_h * 0.7, theme.font_size_small, theme.accent);
                }
                DiffLineOperation::Equal => {
                    draw_text(ctx, line.text, inner.x + pad, y + row_h * 0.7, theme.font_size_small, theme.text);
                    draw_text(ctx, line.text, right_x + pad, y + row_h * 0.7, theme.font_size_small, theme.text);
                }
            }
            ctx.draw.push_line(right_x, y, right_x, y + row_h, theme.separator, theme.stroke_hairline);
        } else {
            let (marker, color) = match line.operation {
                DiffLineOperation::Added => ('+', theme.accent),
                DiffLineOperation::Removed => ('-', theme.error),
                DiffLineOperation::Equal => (' ', theme.text),
            };
            draw_text(ctx, &format!("{marker} {}", line.text), inner.x + pad, y + row_h * 0.7, theme.font_size_small, color);
        }
    }
    ctx.draw.pop_scissor();
}
//#endregion DiffView

//#region DiffViewTests
#[cfg(test)]
mod diff_view_tests {
    use super::*;

    #[test]
    fn identical_inputs_produce_only_equal_operations() {
        let before = vec!["a", "b", "c"];
        let after = vec!["a", "b", "c"];
        let operations = diff_lines(&before, &after);
        assert_eq!(operations.len(), 3);
        assert!(operations.iter().all(|line| line.operation == DiffLineOperation::Equal));
    }

    #[test]
    fn pure_addition_is_all_added_after_the_shared_prefix() {
        let before = vec!["a"];
        let after = vec!["a", "b", "c"];
        let operations = diff_lines(&before, &after);
        assert_eq!(operations[0].operation, DiffLineOperation::Equal);
        assert_eq!(operations[1].operation, DiffLineOperation::Added);
        assert_eq!(operations[2].operation, DiffLineOperation::Added);
    }

    #[test]
    fn pure_removal_is_all_removed_after_the_shared_prefix() {
        let before = vec!["a", "b", "c"];
        let after = vec!["a"];
        let operations = diff_lines(&before, &after);
        assert_eq!(operations[0].operation, DiffLineOperation::Equal);
        assert_eq!(operations[1].operation, DiffLineOperation::Removed);
        assert_eq!(operations[2].operation, DiffLineOperation::Removed);
    }

    #[test]
    fn changed_line_shows_as_a_remove_add_pair() {
        let before = vec!["a", "old", "c"];
        let after = vec!["a", "new", "c"];
        let operations = diff_lines(&before, &after);
        assert_eq!(operations.iter().filter(|line| line.operation == DiffLineOperation::Removed).count(), 1);
        assert_eq!(operations.iter().filter(|line| line.operation == DiffLineOperation::Added).count(), 1);
        assert_eq!(operations.iter().filter(|line| line.operation == DiffLineOperation::Equal).count(), 2);
    }

    #[test]
    fn empty_inputs_produce_no_operations() {
        assert!(diff_lines(&[], &[]).is_empty());
    }

    #[test]
    fn oversized_inputs_use_the_positional_fallback_without_panicking() {
        let before: Vec<&str> = vec!["x"; 2000];
        let after: Vec<&str> = vec!["x"; 2000];
        let operations = diff_lines(&before, &after);
        assert_eq!(operations.len(), 2000);
        assert!(operations.iter().all(|line| line.operation == DiffLineOperation::Equal));
    }

    //#region DiffViewPaintTests
    /// 🧰️ Renders a `render_diff_view` scene in `mode` and returns the `DrawList` so paint-level
    /// assertions (glyph tint, absence of row-background fills) can inspect it directly, same
    /// technique as `render_entry_tests::Fixture`.
    fn render_diff(before: &str, after: &str, mode: Option<&str>) -> (ui_wgpu::wgpu::DrawList, Theme) {
        let scene = UiComponentSceneNode {
            surface_id: "diff-paint-test".into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::DiffView,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: Some(ui_wgpu::wgpu::DiffViewScene { before: before.into(), after: after.into(), language: None, mode: mode.map(str::to_string) }),
            event_feed: None,
            block_list: None,
            menu: None,
        };
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            render_diff_view(&scene, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
        }
        (draw, theme)
    }

    fn glyph_colors(draw: &ui_wgpu::wgpu::DrawList) -> Vec<Rgba> {
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| Rgba::new(instance.color[0], instance.color[1], instance.color[2], instance.color[3])).collect()
    }

    #[test]
    fn unified_added_line_text_is_tinted_accent_not_a_row_background() {
        let (draw, theme) = render_diff("a\n", "a\nnew\n", Some("unified"));
        let colors = glyph_colors(&draw);
        assert!(colors.contains(&theme.accent), "added line's glyph text should be tinted theme.accent, got {colors:?}");
        // 🚫️ No translucent full-row wash left over — the old background-fill mechanism pushed a
        // `push_solid` at `theme.accent.with_alpha(0.16)` for every added row.
        assert!(!colors.contains(&theme.accent.with_alpha(0.16)), "added rows must no longer paint a translucent background wash");
    }

    #[test]
    fn unified_removed_line_text_is_tinted_error() {
        let (draw, theme) = render_diff("old\n", "\n", Some("unified"));
        let colors = glyph_colors(&draw);
        assert!(colors.contains(&theme.error), "removed line's glyph text should be tinted theme.error, got {colors:?}");
    }

    #[test]
    fn unchanged_lines_stay_full_brightness_not_dimmed() {
        let (draw, theme) = render_diff("same\n", "same\n", Some("unified"));
        let colors = glyph_colors(&draw);
        assert!(colors.contains(&theme.text), "an unchanged line must render at full theme.text brightness (React never dims equal lines), got {colors:?}");
        assert!(!colors.contains(&theme.text_muted), "unchanged lines must not be dimmed to theme.text_muted, got {colors:?}");
    }

    #[test]
    fn split_mode_added_and_removed_columns_use_accent_and_error_text() {
        let (draw, theme) = render_diff("old\n", "new\n", Some("split"));
        let colors = glyph_colors(&draw);
        assert!(colors.contains(&theme.accent), "split-mode added column text should be theme.accent");
        assert!(colors.contains(&theme.error), "split-mode removed column text should be theme.error");
    }
    //#endregion DiffViewPaintTests
}
//#endregion DiffViewTests

//#region EventFeed
/// 🪶️ Mirrors a `SurfaceKind::EventFeed` entry (`{id, timestampMs, iconId, title, detail?, tone?}`,
/// `ui_wgpu::wgpu::EventFeedScene`'s doc comment / `EventFeedEntry` in `framework/core/js/index.ts`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventFeedEntryJson {
    id: String,
    #[serde(default)]
    timestamp_ms: i64,
    #[serde(default)]
    icon_id: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    detail: Option<String>,
    #[serde(default)]
    tone: Option<String>,
}

/// 🕒️ Renders a ms-since-epoch timestamp as a bare `HH:MM:SS` UTC time-of-day — no calendar/timezone
/// library in this crate, so this deliberately doesn't attempt a full date.
fn event_feed_time_of_day_utc(timestamp_ms: i64) -> String {
    let ms_in_day = timestamp_ms.rem_euclid(86_400_000);
    let total_seconds = ms_in_day / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

/// 🎨️ Maps an entry's free-form `tone` to an existing theme token — no new color literals. Only the
/// tones this crate already has a token for get a distinct color; anything else (including no tone)
/// stays neutral. Widen this as more tones prove common once the host apps start emitting them.
fn event_feed_tone_color(tone: Option<&str>, theme: &Theme) -> Rgba {
    match tone {
        Some("error") | Some("danger") => theme.error,
        Some("success") | Some("positive") => theme.accent,
        Some("pending") | Some("warning") => theme.temporary,
        _ => theme.text_muted,
    }
}

fn event_feed_row_height(entry: &EventFeedEntryJson, row_h: f32, theme: &Theme) -> f32 {
    row_h + entry.detail.as_ref().map_or(0.0, |_| theme.font_size_small + theme.padding_standard * 0.25)
}

/// 📜️ Renders [`SurfaceKind::EventFeed`]: a scrollable list of text rows (`entries_json`), each a
/// tone dot + optional icon + time-of-day + title, with an optional detail line beneath. When
/// `follow` is set the feed snaps to its bottom every frame (log-tail behavior); a manual
/// wheel-scroll on a following feed is overridden on the next render, same tradeoff a live log tail
/// makes. Rows dispatch `activate_action` (when set) with `{ "entryId": ... }`, mirroring
/// `render_graph_timeline`'s per-row `checkoutCheckpoint` hit.
fn render_event_feed(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(feed) = &scene.event_feed else {
        return render_placeholder("event-feed", bounds, ctx);
    };
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let inner = bounds;
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    if entries.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }

    let heights: Vec<f32> = entries.iter().map(|entry| event_feed_row_height(entry, row_h, theme)).collect();
    let content_h: f32 = heights.iter().sum();
    if feed.follow.unwrap_or(false) {
        set_scroll_offset(&scene.surface_id, "feed", (content_h - inner.h).max(0.0));
    }
    let scroll = scroll_offset(&scene.surface_id, "feed");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "feed")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    let hovered_row = ctx.input.hovered_id.clone();
    let mut y = inner.y - scroll;
    for entry in entries.iter() {
        let entry_h = event_feed_row_height(entry, row_h, theme);
        if y + entry_h < inner.y || y > inner.y + inner.h {
            y += entry_h;
            continue;
        }
        let control_id = format!("{}.feed.{}", scene.surface_id, entry.id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, entry_h);
        if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);

        let tone_color = event_feed_tone_color(entry.tone.as_deref(), theme);
        // ℹ `FEED_TONE_CLASS`'s `info` case is `text-foreground`, not a muted tone — only
        // success/warning/error get an actual tone color on the title.
        let title_tone_color = match entry.tone.as_deref() {
            None | Some("info") => theme.text,
            _ => tone_color,
        };
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([inner.x + pad, dot_y - 3.0, 6.0, 6.0], tone_color, 3.0);
        let mut title_x = inner.x + pad + 6.0 + pad * 0.5;

        if !entry.icon_id.is_empty() {
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(&entry.icon_id) {
                    ctx.draw.push_textured([title_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, theme.text_element);
                    title_x += 14.0 + pad * 0.5;
                }
            }
        }

        if entry.timestamp_ms != 0 {
            let time_label = event_feed_time_of_day_utc(entry.timestamp_ms);
            draw_text(ctx, &time_label, title_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
            title_x += 56.0;
        }
        // 🎨️ `FEED_TONE_CLASS` tints the title span itself in `event-feed-host.tsx`
        // (info→foreground, success/warning/error→their tone color) — plain `theme.text` here
        // previously dropped the tone cue from the one place React actually shows it.
        draw_text(ctx, &entry.title, title_x, y + row_h * 0.65, theme.font_size_small, title_tone_color);
        if let Some(detail) = &entry.detail {
            draw_text(ctx, detail, inner.x + pad, y + row_h + theme.font_size_small * 0.9, theme.font_size_small, theme.text_muted);
        }
        if let Some(action) = &feed.activate_action {
            ctx.input.register_hit(HitTarget { rect: row_rect, event: Some(scene_action(scene, action, json!({ "entryId": entry.id }))), control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data: None });
        }
        y += entry_h;
    }
    ctx.draw.pop_scissor();
}
//#endregion EventFeed

//#region EventFeedTests
#[cfg(test)]
mod event_feed_tests {
    use super::*;

    #[test]
    fn entries_json_tolerates_missing_optional_fields() {
        let json = r#"[{"id":"only-required"}]"#;
        let entries: Vec<EventFeedEntryJson> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "only-required");
        assert_eq!(entries[0].timestamp_ms, 0);
        assert!(entries[0].icon_id.is_empty());
        assert!(entries[0].title.is_empty());
        assert!(entries[0].detail.is_none());
        assert!(entries[0].tone.is_none());
    }

    #[test]
    fn entries_json_parses_the_full_wire_shape() {
        let json = r#"[{"id":"e1","timestampMs":1000,"iconId":"bell","title":"Built","detail":"3 warnings","tone":"success"}]"#;
        let entries: Vec<EventFeedEntryJson> = serde_json::from_str(json).unwrap();
        assert_eq!(entries[0].timestamp_ms, 1000);
        assert_eq!(entries[0].icon_id, "bell");
        assert_eq!(entries[0].title, "Built");
        assert_eq!(entries[0].detail.as_deref(), Some("3 warnings"));
        assert_eq!(entries[0].tone.as_deref(), Some("success"));
    }

    #[test]
    fn row_height_grows_when_detail_is_present() {
        let theme = Theme::dark();
        let without_detail = EventFeedEntryJson { id: "a".into(), timestamp_ms: 0, icon_id: String::new(), title: "a".into(), detail: None, tone: None };
        let with_detail = EventFeedEntryJson { id: "b".into(), timestamp_ms: 0, icon_id: String::new(), title: "b".into(), detail: Some("more".into()), tone: None };
        let row_h = theme.control_height;
        assert!(event_feed_row_height(&with_detail, row_h, &theme) > event_feed_row_height(&without_detail, row_h, &theme));
    }

    #[test]
    fn time_of_day_wraps_within_a_day() {
        assert_eq!(event_feed_time_of_day_utc(0), "00:00:00");
        assert_eq!(event_feed_time_of_day_utc(3_661_000), "01:01:01");
        assert_eq!(event_feed_time_of_day_utc(86_400_000), "00:00:00");
    }

    #[test]
    fn known_tones_resolve_to_distinct_theme_tokens() {
        let theme = Theme::dark();
        assert_eq!(event_feed_tone_color(Some("error"), &theme), theme.error);
        assert_eq!(event_feed_tone_color(Some("success"), &theme), theme.accent);
        assert_eq!(event_feed_tone_color(None, &theme), theme.text_muted);
        assert_eq!(event_feed_tone_color(Some("unknown-tone"), &theme), theme.text_muted);
    }

    //#region EventFeedPaintTests
    /// 🧰️ Renders `render_event_feed` with one entry of the given `tone` and returns its `DrawList`,
    /// so the title glyph's tint can be inspected directly — matches `FEED_TONE_CLASS`'s title-span
    /// coloring in `event-feed-host.tsx`.
    fn render_feed_entry(tone: Option<&str>) -> (ui_wgpu::wgpu::DrawList, Theme) {
        let entry = json!({ "id": "e1", "title": "Built", "tone": tone });
        let scene = UiComponentSceneNode {
            surface_id: "feed-paint-test".into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::EventFeed,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: Some(ui_wgpu::wgpu::EventFeedScene { entries_json: json!([entry]).to_string(), follow: None, activate_action: None }),
            block_list: None,
            menu: None,
        };
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            render_event_feed(&scene, Rect::new(0.0, 0.0, 400.0, 200.0), &mut ctx);
        }
        (draw, theme)
    }

    fn instance_colors(draw: &ui_wgpu::wgpu::DrawList) -> Vec<Rgba> {
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| Rgba::new(instance.color[0], instance.color[1], instance.color[2], instance.color[3])).collect()
    }

    #[test]
    fn info_tone_title_is_full_brightness_foreground_not_muted() {
        let (draw, theme) = render_feed_entry(None);
        let colors = instance_colors(&draw);
        assert!(colors.contains(&theme.text), "an info/no-tone title must render at full theme.text brightness, matching FEED_TONE_CLASS's `info` case, got {colors:?}");
    }

    #[test]
    fn error_tone_title_is_tinted_theme_error() {
        let (draw, theme) = render_feed_entry(Some("error"));
        let colors = instance_colors(&draw);
        assert!(colors.contains(&theme.error), "an error-tone title must be tinted theme.error, got {colors:?}");
    }
    //#endregion EventFeedPaintTests
}
//#endregion EventFeedTests

//#region GraphTimeline
/** @emoji 🗄️ Mirrors `store::HistoryColumn` / React `HistoryColumn` (`ui/js/react/index.tsx:19116`). */
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryColumnAuthorJson {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryColumnJson {
    checkpoint_id: String,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    authors: Vec<HistoryColumnAuthorJson>,
    #[serde(default)]
    parent_checkpoint_id: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    lane: usize,
}

const HISTORY_LANE_PITCH: f32 = 16.0;
const HISTORY_LANE_PAD: f32 = 8.0;
const HISTORY_AUTHOR_SLOT: f32 = 40.0;

/** Ports `historyLaneCount` (`ui/js/react/index.tsx:19141`). */
fn history_lane_count(columns: &[HistoryColumnJson]) -> usize {
    columns.iter().map(|column| column.lane + 1).max().unwrap_or(1).max(1)
}

/** Ports `historyGraphWidth` (`ui/js/react/index.tsx:19145`). */
fn history_graph_width(lane_count: usize) -> f32 {
    (HISTORY_LANE_PAD * 2.0 + lane_count as f32 * HISTORY_LANE_PITCH).max(56.0)
}

/** Ports `historyLaneX` (`ui/js/react/index.tsx:19153`). */
fn history_lane_x(lane: usize, lane_count: usize, graph_width: f32) -> f32 {
    if lane_count <= 1 {
        return graph_width * 0.5;
    }
    HISTORY_LANE_PAD + lane as f32 * HISTORY_LANE_PITCH + HISTORY_LANE_PITCH * 0.5
}

/** Ports `historyRowLaneGuides` (`ui/js/react/index.tsx:19162`): per-row, per-lane guide-line
 * visibility, including the elbow-row propagation when a checkpoint's parent sits on another lane. */
fn history_row_lane_guides(columns: &[HistoryColumnJson], lane_count: usize) -> Vec<Vec<bool>> {
    let mut guides = vec![vec![false; lane_count]; columns.len()];
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();
    for (row_index, column) in columns.iter().enumerate() {
        if column.lane < lane_count {
            guides[row_index][column.lane] = true;
        }
        let Some(parent_row) = column.parent_checkpoint_id.as_deref().and_then(|id| row_by_id.get(id).copied()) else {
            continue;
        };
        let parent_lane = columns[parent_row].lane;
        if column.lane == parent_lane {
            for row in (row_index + 1)..parent_row {
                guides[row][column.lane] = true;
            }
            continue;
        }
        let elbow_row = if row_index + 1 < parent_row { row_index + 1 } else { parent_row };
        for row in (row_index + 1)..=elbow_row {
            if column.lane < lane_count {
                guides[row][column.lane] = true;
            }
        }
        for row in elbow_row..parent_row {
            if parent_lane < lane_count {
                guides[row][parent_lane] = true;
            }
        }
    }
    guides
}

/// 🔤️ Two-letter initials from the first two words of an author name (e.g. "Jane Doe" → "JD"),
/// matching the avatar-initials helper in `index.tsx` — previously the caller only took the very
/// first character of the whole string (e.g. "Jane Doe" → "J").
fn graph_timeline_avatar_initials(name: &str) -> String {
    let letters: String = name.split_whitespace().filter_map(|word| word.chars().next()).take(2).flat_map(char::to_uppercase).collect();
    if letters.is_empty() {
        "?".to_string()
    } else {
        letters
    }
}

fn render_graph_timeline(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(history) = &scene.graph_timeline else {
        return render_placeholder("graph-timeline", bounds, ctx);
    };
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let inner = bounds;
    let row_h = theme.control_height * 1.33;
    let pad = theme.padding_standard;
    if columns.is_empty() {
        draw_text(ctx, "—", inner.x + pad, inner.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        return;
    }
    let lane_count = history_lane_count(&columns);
    let graph_width = history_graph_width(lane_count);
    let graph_col_w = graph_width + HISTORY_AUTHOR_SLOT;
    let labels_col_w = (inner.w * 0.28).max(96.0);
    let guides = history_row_lane_guides(&columns, lane_count);
    let row_by_id: HashMap<&str, usize> = columns.iter().enumerate().map(|(index, column)| (column.checkpoint_id.as_str(), index)).collect();

    let scroll = scroll_offset(&scene.surface_id, "history");
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scroll_key(&scene.surface_id, "history")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(inner);
    let hovered_row = ctx.input.hovered_id.clone();
    let graph_x0 = inner.x + labels_col_w;
    let desc_x = inner.x + labels_col_w + graph_col_w;

    for (row_index, column) in columns.iter().enumerate() {
        let y = inner.y + row_index as f32 * row_h - scroll;
        if y + row_h < inner.y || y > inner.y + inner.h {
            continue;
        }
        let control_id = format!("{}.history.{}", scene.surface_id, column.checkpoint_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        let row_rect = Rect::new(inner.x, y, inner.w, row_h);
        if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);

        let mut label_x = inner.x + pad;
        if column.labels.is_empty() {
            draw_text(ctx, "checkpoint", label_x, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        } else {
            for label in &column.labels {
                let chip_w = (label.len() as f32 * 6.0 + pad * 2.0).min((inner.x + labels_col_w - label_x).max(0.0));
                if chip_w <= 0.0 {
                    break;
                }
                ctx.draw.push_rounded([label_x, y + row_h * 0.5 - 9.0, chip_w, 18.0], theme.accent, 4.0);
                draw_text(ctx, label, label_x + 4.0, y + row_h * 0.5 + 4.0, theme.font_size_small, theme.active_foreground);
                label_x += chip_w + 4.0;
            }
        }

        // 🪢️ `color-mix(in oklab, var(--muted-foreground) 40%, transparent)` on the lane guides and
        // parent connectors in `graph-timeline-host.tsx` — a translucent line; opaque `theme.separator`
        // previously read as visibly heavier than React's thin, faded rail.
        let guide_stroke = theme.separator.with_alpha(theme.separator.a * 0.4);
        for lane in 0..lane_count {
            if guides[row_index][lane] {
                let lx = graph_x0 + history_lane_x(lane, lane_count, graph_width);
                ctx.draw.push_line(lx, y, lx, y + row_h, guide_stroke, 1.0);
            }
        }
        if let Some(parent_id) = column.parent_checkpoint_id.as_deref() {
            if let Some(&parent_row) = row_by_id.get(parent_id) {
                let x0 = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
                let parent_lane = columns[parent_row].lane;
                let x1 = graph_x0 + history_lane_x(parent_lane, lane_count, graph_width);
                let y0 = y + row_h * 0.5;
                let y1 = inner.y + parent_row as f32 * row_h - scroll + row_h * 0.5;
                if (x0 - x1).abs() < 0.5 {
                    ctx.draw.push_line(x0, y0, x1, y1, guide_stroke, 1.5);
                } else {
                    let elbow_y = y + row_h;
                    ctx.draw.push_line(x0, y0, x0, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x0, elbow_y, x1, elbow_y, guide_stroke, 1.5);
                    ctx.draw.push_line(x1, elbow_y, x1, y1, guide_stroke, 1.5);
                }
            }
        }
        let dot_x = graph_x0 + history_lane_x(column.lane, lane_count, graph_width);
        let dot_y = y + row_h * 0.5;
        ctx.draw.push_rounded([dot_x - 3.0, dot_y - 3.0, 6.0, 6.0], theme.text, 3.0);

        let avatar_size = 20.0;
        let avatar_x = graph_x0 + graph_width + 4.0;
        let avatar_y = y + row_h * 0.5 - avatar_size * 0.5;
        let initial = column.authors.first().map(|author| graph_timeline_avatar_initials(&author.name)).unwrap_or_else(|| "?".into());
        ctx.draw.push_rounded([avatar_x, avatar_y, avatar_size, avatar_size], theme.button, avatar_size * 0.5);
        let initial_x_frac = if initial.chars().count() >= 2 { 0.18 } else { 0.32 };
        draw_text(ctx, &initial, avatar_x + avatar_size * initial_x_frac, avatar_y + avatar_size * 0.7, theme.font_size_small, theme.text);

        if let Some(description) = &column.description {
            draw_text(ctx, description, desc_x + pad, y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        }

        ctx.input.register_hit(HitTarget {
            rect: row_rect,
            event: Some(scene_action(scene, "checkoutCheckpoint", json!({ "checkpointId": column.checkpoint_id }))),
            control_id: Some(control_id),
            kind: HitKind::Generic,
            drag_axis: None,
            drag_data: None,
        });
    }
    ctx.draw.pop_scissor();
}
//#endregion GraphTimeline

//#region GraphTimelineTests
#[cfg(test)]
mod graph_timeline_tests {
    use super::*;

    fn column(id: &str, lane: usize, parent: Option<&str>) -> HistoryColumnJson {
        HistoryColumnJson { checkpoint_id: id.to_string(), labels: Vec::new(), authors: Vec::new(), parent_checkpoint_id: parent.map(str::to_string), description: None, lane }
    }

    #[test]
    fn lane_count_is_max_lane_plus_one() {
        let columns = vec![column("a", 0, None), column("b", 2, Some("a"))];
        assert_eq!(history_lane_count(&columns), 3);
    }

    #[test]
    fn lane_count_defaults_to_one_for_empty_columns() {
        assert_eq!(history_lane_count(&[]), 1);
    }

    #[test]
    fn lane_x_centers_graph_when_single_lane() {
        let width = history_graph_width(1);
        assert_eq!(history_lane_x(0, 1, width), width * 0.5);
    }

    #[test]
    fn linear_history_guides_stay_on_single_lane() {
        let columns = vec![column("c", 0, Some("b")), column("b", 0, Some("a")), column("a", 0, None)];
        let guides = history_row_lane_guides(&columns, 1);
        assert!(guides.iter().all(|row| row[0]), "a linear single-lane history must keep every row's lane-0 guide active");
    }

    #[test]
    fn fork_guides_propagate_through_elbow_row() {
        let columns = vec![column("c", 1, Some("a")), column("b", 0, None), column("a", 0, None)];
        let guides = history_row_lane_guides(&columns, 2);
        assert!(guides[0][1], "the forking checkpoint's own row must show its lane");
        assert!(guides[1][1] || guides[1][0], "the elbow row must carry a guide on at least one of the two connected lanes");
    }

    #[test]
    fn columns_json_tolerates_missing_optional_fields() {
        let json = r#"[{"checkpointId":"only-required"}]"#;
        let columns: Vec<HistoryColumnJson> = serde_json::from_str(json).unwrap();
        assert_eq!(columns.len(), 1);
        assert_eq!(columns[0].checkpoint_id, "only-required");
        assert_eq!(columns[0].lane, 0);
        assert!(columns[0].labels.is_empty());
        assert!(columns[0].authors.is_empty());
        assert!(columns[0].parent_checkpoint_id.is_none());
        assert!(columns[0].description.is_none());
    }

    //#region GraphTimelinePaintTests
    #[test]
    fn avatar_initials_take_the_first_letter_of_the_first_two_words() {
        assert_eq!(graph_timeline_avatar_initials("Jane Doe"), "JD");
        assert_eq!(graph_timeline_avatar_initials("cher"), "C");
        assert_eq!(graph_timeline_avatar_initials("  "), "?");
        assert_eq!(graph_timeline_avatar_initials(""), "?");
        assert_eq!(graph_timeline_avatar_initials("Ada Lovelace Byron"), "AL");
    }

    #[test]
    fn lane_guide_lines_are_translucent_not_the_opaque_separator_token() {
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        let scene = UiComponentSceneNode {
            surface_id: "timeline-paint-test".into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::GraphTimeline,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: Some(ui_wgpu::wgpu::GraphTimelineScene {
                columns_json: json!([
                    { "checkpointId": "b", "lane": 0, "parentCheckpointId": "a" },
                    { "checkpointId": "a", "lane": 0 },
                ])
                .to_string(),
            }),
            diff_view: None,
            event_feed: None,
            block_list: None,
            menu: None,
        };
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            render_graph_timeline(&scene, Rect::new(0.0, 0.0, 400.0, 200.0), &mut ctx);
        }
        // 🖊️ Note: the per-row bottom hairline (a separate, legitimate divider) still uses the fully
        // opaque `theme.separator`, so this only checks that the translucent guide stroke is present
        // among the parent-connector line's vertices, not that opaque `theme.separator` is absent.
        let guide_stroke = theme.separator.with_alpha(theme.separator.a * 0.4);
        let vertex_colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).map(|v| v.color).collect();
        assert!(vertex_colors.contains(&[guide_stroke.r, guide_stroke.g, guide_stroke.b, guide_stroke.a]), "the parent-connector line must use the translucent guide stroke, got {vertex_colors:?}");
    }
    //#endregion GraphTimelinePaintTests
}
//#endregion GraphTimelineTests

//#region Canvas2d
/** 🪣️ A `CanvasLayerRecord["fill"]` mirror — solid color or linear/radial gradient stops, matches
 * `fillStyleToPaint` in `canvas-2d-host.tsx`. Coordinates (`x1/y1/x2/y2`, `cx/cy/r`) are in the same
 * local space as the owning layer's `x`/`y` (this renderer has no per-layer transform matrix yet, so
 * they are treated as offsets from the layer's own `x`/`y` origin). */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasFillJson {
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    color: Option<Vec<f64>>,
    #[serde(default)]
    x1: f64,
    #[serde(default)]
    y1: f64,
    #[serde(default)]
    x2: f64,
    #[serde(default)]
    y2: f64,
    #[serde(default)]
    cx: f64,
    #[serde(default)]
    cy: f64,
    #[serde(default)]
    r: f64,
    #[serde(default)]
    stops: Vec<CanvasGradientStopJson>,
}

/** 🎨️ One `CanvasGradientStop` — `offset` in `[0,1]`, `color` an `[r,g,b,a?]` channel array in `0..1`. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasGradientStopJson {
    #[serde(default)]
    offset: f64,
    #[serde(default)]
    color: Option<Vec<f64>>,
}

/** 🖊️ A `CanvasLayerRecord["stroke"]` mirror. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasStrokeJson {
    #[serde(default)]
    color: Option<Vec<f64>>,
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    dash: Option<Vec<f64>>,
}

/** 🖼️ A `CanvasLayerRecord["image"]` mirror — the nested per-node image field (as opposed to the
 * legacy top-level `dataUrl` used by `kind === "image"` records). */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasImageFieldJson {
    #[serde(default)]
    src: Option<String>,
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    height: Option<f64>,
}

/** 📝️ A `CanvasLayerRecord["text"]` mirror. */
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct CanvasTextFieldJson {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    size: Option<f64>,
}

#[derive(Deserialize)]
struct CanvasLayer {
    #[serde(default)]
    kind: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    width: f64,
    #[serde(default)]
    height: f64,
    #[serde(default)]
    x0: Option<f64>,
    #[serde(default)]
    y0: Option<f64>,
    #[serde(default)]
    x1: Option<f64>,
    #[serde(default)]
    y1: Option<f64>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    target: Option<String>,
    #[serde(default, rename = "dataUrl")]
    data_url: Option<String>,
    #[serde(default)]
    points: Option<Vec<[f64; 2]>>,
    #[serde(default)]
    seams: Option<Vec<u8>>,
    /** 🗒️ `role === "meta"` marks a non-visual bookkeeping record (e.g. carries the host's active
     * pointer-tool id) — filtered from rendering, matches `JsonLayersCanvasSession.renderFrame`'s
     * `records.find(role === "meta")` / `layers.filter(role !== "meta")` split. */
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    visible: Option<bool>,
    #[serde(default)]
    opacity: Option<f32>,
    #[serde(default, rename = "blendMode")]
    blend_mode: Option<String>,
    #[serde(default)]
    selected: Option<bool>,
    #[serde(default)]
    fill: Option<CanvasFillJson>,
    #[serde(default)]
    stroke: Option<CanvasStrokeJson>,
    #[serde(default)]
    image: Option<CanvasImageFieldJson>,
    #[serde(default)]
    text: Option<CanvasTextFieldJson>,
}

fn canvas_layer_should_render(layer: &CanvasLayer) -> bool {
    layer.role.as_deref() != Some("meta") && layer.visible.unwrap_or(true)
}

pub(crate) fn decode_canvas_image(data_url: &str) -> Option<(Vec<u8>, u32, u32)> {
    let payload = data_url.strip_prefix("data:image/png;base64,").or_else(|| data_url.strip_prefix("data:image/jpeg;base64,")).unwrap_or(data_url);
    let bytes = base64::engine::general_purpose::STANDARD.decode(payload).ok()?;
    let image = image::load_from_memory(&bytes).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some((rgba.into_raw(), width, height))
}

/** Hashes the raw (still-encoded) `data_url` before touching base64/PNG decode, so an unchanged
 * image layer costs one cheap byte-hash per frame instead of a full decode — decode only runs when
 * the source string actually changes. Continuous rAF renderers (e.g. paint-2d) would otherwise redo
 * base64+PNG decode every frame for every image layer regardless of whether anything changed. */
pub(crate) fn queue_canvas_image_upload(surface_id: &str, layer_id: &str, data_url: &str) -> Option<String> {
    let key = format!("canvas-image:{surface_id}:{layer_id}");
    let src_key = format!("canvas-image-src:{surface_id}:{layer_id}");
    let src_digest = digest_pixels(data_url.as_bytes());
    let unchanged = scene_state(surface_id).canvas_image_src_digests.get(&src_key).copied() == Some(src_digest);
    if unchanged {
        return Some(key);
    }
    let (pixels, width, height) = decode_canvas_image(data_url)?;
    let expected = (width as usize).saturating_mul(height as usize).saturating_mul(4);
    if pixels.len() < expected {
        return None;
    }
    let digest = digest_pixels(&pixels[..expected]);
    mutate_scene_state(surface_id, |state| {
        state.canvas_image_src_digests.insert(src_key.clone(), src_digest);
        let prior = state.canvas_image_digests.get(&key).copied();
        if prior != Some(digest) {
            state.canvas_image_digests.insert(key.clone(), digest);
            state.pending_raster_uploads.push(PendingRasterUpload { key: key.clone(), pixels: pixels[..expected].to_vec(), width, height });
        }
    });
    Some(key)
}

/** Clamps checkerboard cell iteration to the world-space rect actually visible through `inner`
 * (intersected with the full `±extent/2` grid) instead of always walking the whole grid — a
 * continuously-rendering surface (paint-2d) was pushing up to `(extent/cell)^2` solid quads every
 * single frame regardless of zoom/pan, which starves headless WebGPU frame pacing. */
fn draw_checkerboard(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme, extent: f32) {
    let cell = 16.0;
    let half = extent * 0.5;
    let light = theme.checker_light;
    let dark = theme.checker_dark;
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (-half, half, -half, half);
    if viewport.zoom > 0.0 {
        let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
        let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
        min_x = min_x.max(wx0.min(wx1) - cell);
        max_x = max_x.min(wx0.max(wx1) + cell);
        min_y = min_y.max(wy0.min(wy1) - cell);
        max_y = max_y.min(wy0.max(wy1) + cell);
    }
    let start_row = ((min_y - (-half)) / cell).floor().max(0.0) as i64;
    let start_col = ((min_x - (-half)) / cell).floor().max(0.0) as i64;
    let mut row = start_row;
    let mut wy = -half + start_row as f32 * cell;
    while wy < max_y {
        let mut col = start_col;
        let mut wx = -half + start_col as f32 * cell;
        while wx < max_x {
            let color = if (row + col) % 2 == 0 { light } else { dark };
            let (sx, sy) = viewport.world_to_screen(wx, wy, inner);
            let (sx1, sy1) = viewport.world_to_screen(wx + cell, wy + cell, inner);
            let w = (sx1 - sx).abs().max(1.0);
            let h = (sy1 - sy).abs().max(1.0);
            draw.push_solid([sx.min(sx1), sy.min(sy1), w, h], color);
            wx += cell;
            col += 1;
        }
        wy += cell;
        row += 1;
    }
}

/** 📐️ Theme-aware LOD world grid for canvas-2d — same large/medium/small/micro steps as flow and infinite boards. */
fn draw_canvas_infinite_grid(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, theme: &Theme) {
    if viewport.zoom <= 0.0 {
        return;
    }
    let (wx0, wy0) = viewport.screen_to_world(inner.x, inner.y, inner);
    let (wx1, wy1) = viewport.screen_to_world(inner.x + inner.w, inner.y + inner.h, inner);
    let min_x = wx0.min(wx1);
    let max_x = wx0.max(wx1);
    let min_y = wy0.min(wy1);
    let max_y = wy0.max(wy1);
    let color = theme.separator.with_alpha((theme.separator.a * 0.35).max(0.08));
    let steps: [(f32, f32, f32); 4] = [(10.0, 1.0, 0.0), (2.5, 0.72, 8.0), (0.5, 0.48, 10.0), (0.1, 0.32, 12.0)];
    for (world_step, stroke_px, min_screen) in steps {
        let screen = world_step * viewport.zoom;
        if screen < min_screen {
            continue;
        }
        let width = (stroke_px).max(0.5);
        let start_x = (min_x / world_step).floor() * world_step;
        let start_y = (min_y / world_step).floor() * world_step;
        let mut x = start_x;
        while x <= max_x {
            let (sx0, sy0) = viewport.world_to_screen(x, min_y, inner);
            let (sx1, sy1) = viewport.world_to_screen(x, max_y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            x += world_step;
        }
        let mut y = start_y;
        while y <= max_y {
            let (sx0, sy0) = viewport.world_to_screen(min_x, y, inner);
            let (sx1, sy1) = viewport.world_to_screen(max_x, y, inner);
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
            y += world_step;
        }
    }
}

fn draw_dashed_line(draw: &mut ui_wgpu::wgpu::DrawList, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let dash = 4.0f32;
    let gap = 4.0f32;
    let mut traveled = 0.0f32;
    let mut drawing = true;
    while traveled < len {
        let segment = if drawing { dash } else { gap };
        let next = (traveled + segment).min(len);
        if drawing {
            let sx0 = x0 + ux * traveled;
            let sy0 = y0 + uy * traveled;
            let sx1 = x0 + ux * next;
            let sy1 = y0 + uy * next;
            draw.push_line(sx0, sy0, sx1, sy1, color, width);
        }
        traveled = next;
        drawing = !drawing;
    }
}

//#region Canvas2dFillBlend
/** 🎨️ Reads an `[r,g,b,a?]` channel array (each `0..1`, matches `rgbaToCss` in `canvas-2d-host.tsx`)
 * into an `Rgba`, multiplying alpha by the layer's resolved `opacity`. Missing channels fall back to
 * a neutral slate gray (matches the React reference's `rgba(148, 163, 184, opacity)` default). */
fn canvas_color_channels(v: &[f64], opacity: f32) -> Rgba {
    let r = v.first().copied().unwrap_or(0.58) as f32;
    let g = v.get(1).copied().unwrap_or(0.64) as f32;
    let b = v.get(2).copied().unwrap_or(0.72) as f32;
    let a = v.get(3).copied().unwrap_or(1.0) as f32;
    Rgba::new(r, g, b, (a * opacity).clamp(0.0, 1.0))
}

fn canvas_mix_rgba(a: Rgba, b: Rgba, t: f32) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    Rgba::new(a.r + (b.r - a.r) * t, a.g + (b.g - a.g) * t, a.b + (b.b - a.b) * t, a.a + (b.a - a.a) * t)
}

/** 🌈️ Samples a `CanvasGradientStop[]` list at `t ∈ [0,1]`, linearly interpolating between the
 * bracketing stops — mirrors `CanvasGradient.addColorStop` sampling semantics. */
fn canvas_gradient_color_at(stops: &[CanvasGradientStopJson], t: f32, opacity: f32) -> Rgba {
    if stops.is_empty() {
        return Rgba::new(0.58, 0.64, 0.72, 0.95 * opacity);
    }
    let t = t.clamp(0.0, 1.0);
    if stops.len() == 1 {
        return canvas_color_channels(stops[0].color.as_deref().unwrap_or(&[]), opacity);
    }
    let last = stops.len() - 1;
    for i in 0..last {
        let a_off = stops[i].offset.clamp(0.0, 1.0) as f32;
        let b_off = stops[i + 1].offset.clamp(0.0, 1.0) as f32;
        if t <= b_off || i == last - 1 {
            let span = (b_off - a_off).max(0.0001);
            let local = ((t - a_off) / span).clamp(0.0, 1.0);
            let ca = canvas_color_channels(stops[i].color.as_deref().unwrap_or(&[]), opacity);
            let cb = canvas_color_channels(stops[i + 1].color.as_deref().unwrap_or(&[]), opacity);
            return canvas_mix_rgba(ca, cb, local);
        }
    }
    canvas_color_channels(stops[last].color.as_deref().unwrap_or(&[]), opacity)
}

/** 🌗️ A single separable-blend-mode channel formula (W3C Compositing and Blending Level 1 §5.2) —
 * `cb` is the backdrop channel, `cs` the source channel, both `0..1`. */
fn canvas_blend_channel(mode: &str, cb: f32, cs: f32) -> f32 {
    match mode {
        "multiply" => cb * cs,
        "screen" => cb + cs - cb * cs,
        "overlay" => canvas_blend_channel("hardLight", cs, cb),
        "darken" => cb.min(cs),
        "lighten" => cb.max(cs),
        "colorDodge" => {
            if cb <= 0.0 {
                0.0
            } else if cs >= 1.0 {
                1.0
            } else {
                (cb / (1.0 - cs)).min(1.0)
            }
        }
        "colorBurn" => {
            if cb >= 1.0 {
                1.0
            } else if cs <= 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - cb) / cs).min(1.0)
            }
        }
        "hardLight" => {
            if cs <= 0.5 {
                2.0 * cb * cs
            } else {
                1.0 - 2.0 * (1.0 - cb) * (1.0 - cs)
            }
        }
        "softLight" => {
            let d = if cb <= 0.25 { ((16.0 * cb - 12.0) * cb + 4.0) * cb } else { cb.sqrt() };
            if cs <= 0.5 {
                cb - (1.0 - 2.0 * cs) * cb * (1.0 - cb)
            } else {
                cb + (2.0 * cs - 1.0) * (d - cb)
            }
        }
        "difference" => (cb - cs).abs(),
        "exclusion" => cb + cs - 2.0 * cb * cs,
        _ => cs,
    }
}

fn canvas_lum(c: [f32; 3]) -> f32 {
    0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]
}

fn canvas_clip_color(c: [f32; 3]) -> [f32; 3] {
    let l = canvas_lum(c);
    let n = c[0].min(c[1]).min(c[2]);
    let x = c[0].max(c[1]).max(c[2]);
    let mut out = c;
    if n < 0.0 {
        for ch in out.iter_mut() {
            *ch = l + (*ch - l) * l / (l - n).max(1e-6);
        }
    }
    if x > 1.0 {
        for ch in out.iter_mut() {
            *ch = l + (*ch - l) * (1.0 - l) / (x - l).max(1e-6);
        }
    }
    out
}

fn canvas_set_lum(c: [f32; 3], l: f32) -> [f32; 3] {
    let d = l - canvas_lum(c);
    canvas_clip_color([c[0] + d, c[1] + d, c[2] + d])
}

fn canvas_sat(c: [f32; 3]) -> f32 {
    c[0].max(c[1]).max(c[2]) - c[0].min(c[1]).min(c[2])
}

fn canvas_set_sat(c: [f32; 3], s: f32) -> [f32; 3] {
    let mut idx = [0usize, 1, 2];
    idx.sort_by(|&a, &b| c[a].partial_cmp(&c[b]).unwrap_or(std::cmp::Ordering::Equal));
    let (imin, imid, imax) = (idx[0], idx[1], idx[2]);
    let mut out = [0.0f32; 3];
    if c[imax] > c[imin] {
        out[imid] = (c[imid] - c[imin]) * s / (c[imax] - c[imin]);
        out[imax] = s;
    }
    out[imin] = 0.0;
    out
}

/** 🎨️ Approximates the React `Canvas2dScene`'s 16 CSS blend modes (`BLEND_MODE_TO_COMPOSITE` in
 * `canvas-2d-host.tsx`) by pre-blending the resolved fill/stroke color against `backdrop` — a
 * stand-in for true per-pixel GPU compositing, which would require a `wgpu::BlendState` change per
 * draw call in the shared `ui_wgpu` pipeline (out of scope for this ticket's Canvas2d/Paint2d
 * regions; see ticket 26/07/11/WGPU-RENDERER-FULL-PARITY). The four non-separable modes
 * (hue/saturation/color/luminosity) follow the W3C SetLum/SetSat algorithm exactly. */
fn canvas_apply_blend_mode(mode: Option<&str>, backdrop: Rgba, source: Rgba) -> Rgba {
    let mode = match mode {
        None | Some("") | Some("normal") => return source,
        Some(m) => m,
    };
    let cb = [backdrop.r, backdrop.g, backdrop.b];
    let cs = [source.r, source.g, source.b];
    let blended = match mode {
        "hue" => canvas_set_lum(canvas_set_sat(cs, canvas_sat(cb)), canvas_lum(cb)),
        "saturation" => canvas_set_lum(canvas_set_sat(cb, canvas_sat(cs)), canvas_lum(cb)),
        "color" => canvas_set_lum(cs, canvas_lum(cb)),
        "luminosity" => canvas_set_lum(cb, canvas_lum(cs)),
        _ => [canvas_blend_channel(mode, cb[0], cs[0]), canvas_blend_channel(mode, cb[1], cs[1]), canvas_blend_channel(mode, cb[2], cs[2])],
    };
    Rgba::new(blended[0].clamp(0.0, 1.0), blended[1].clamp(0.0, 1.0), blended[2].clamp(0.0, 1.0), source.a)
}
//#endregion Canvas2dFillBlend

//#region Canvas2dShapes
const CANVAS_GRADIENT_BANDS: usize = 10;
const CANVAS_CIRCLE_SEGMENTS: usize = 28;

fn canvas_circle_points(cx: f32, cy: f32, radius: f32, segments: usize) -> Vec<[f32; 2]> {
    (0..segments)
        .map(|i| {
            let a = (i as f32 / segments as f32) * std::f32::consts::TAU;
            [cx + a.cos() * radius, cy + a.sin() * radius]
        })
        .collect()
}

fn push_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, is_circle: bool) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        draw.push_triangle_fan(&canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    } else {
        draw.push_rounded([rect.x, rect.y, rect.w, rect.h], color, 4.0);
    }
}

fn push_circle_outline(draw: &mut ui_wgpu::wgpu::DrawList, cx: f32, cy: f32, radius: f32, color: Rgba, width: f32) {
    let points = canvas_circle_points(cx, cy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS);
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        draw.push_line(a[0], a[1], b[0], b[1], color, width);
    }
}

fn push_shape_outline(draw: &mut ui_wgpu::wgpu::DrawList, rect: Rect, color: Rgba, width: f32, is_circle: bool, dash: Option<&[f64]>) {
    if is_circle {
        let cx = rect.x + rect.w * 0.5;
        let cy = rect.y + rect.h * 0.5;
        let radius = rect.w.min(rect.h) * 0.5;
        push_circle_outline(draw, cx, cy, radius, color, width);
        return;
    }
    if dash.is_some_and(|d| !d.is_empty()) {
        draw_dashed_line(draw, rect.x, rect.y, rect.x + rect.w, rect.y, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y, rect.x + rect.w, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x + rect.w, rect.y + rect.h, rect.x, rect.y + rect.h, color, width);
        draw_dashed_line(draw, rect.x, rect.y + rect.h, rect.x, rect.y, color, width);
    } else {
        draw_ink_rect_outline(draw, rect.x, rect.y, rect.w, rect.h, color, width);
    }
}

/** 🌈️ Bands a linear gradient across `clip` (scissor-bounded to the shape's screen bbox) as
 * `CANVAS_GRADIENT_BANDS` solid quads perpendicular to the `(x1,y1)-(x2,y2)` axis — `ui_wgpu::wgpu::
 * DrawList` has no per-vertex gradient primitive, see `canvas_apply_blend_mode` doc comment. */
fn push_linear_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (sx1, sy1) = viewport.world_to_screen((origin_x + fill.x1) as f32, (origin_y + fill.y1) as f32, inner);
    let (sx2, sy2) = viewport.world_to_screen((origin_x + fill.x2) as f32, (origin_y + fill.y2) as f32, inner);
    let dx = sx2 - sx1;
    let dy = sy2 - sy1;
    let len = (dx * dx + dy * dy).sqrt();
    draw.push_scissor(clip);
    if len < 0.5 {
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, 1.0, opacity));
        draw.push_rounded([clip.x, clip.y, clip.w, clip.h], color, 0.0);
        draw.pop_scissor();
        return;
    }
    let ux = dx / len;
    let uy = dy / len;
    let px = -uy;
    let py = ux;
    let overhang = (clip.w + clip.h).max(32.0);
    for i in 0..CANVAS_GRADIENT_BANDS {
        let t0 = i as f32 / CANVAS_GRADIENT_BANDS as f32;
        let t1 = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, (t0 + t1) * 0.5, opacity));
        let a0 = if i == 0 { -overhang } else { t0 * len };
        let a1 = if i == CANVAS_GRADIENT_BANDS - 1 { len + overhang } else { t1 * len };
        let bx = sx1 + ux * a0;
        let by = sy1 + uy * a0;
        let ex = sx1 + ux * a1;
        let ey = sy1 + uy * a1;
        let points = [[bx + px * overhang, by + py * overhang], [ex + px * overhang, ey + py * overhang], [ex - px * overhang, ey - py * overhang], [bx - px * overhang, by - py * overhang]];
        draw.push_triangle_fan(&points, color);
    }
    draw.pop_scissor();
}

/** 🌈️ Bands a radial gradient as `CANVAS_GRADIENT_BANDS` concentric circles painted outer-to-inner
 * (painter's algorithm — smaller/later circles overpaint the center), scissor-bounded to `clip`. */
fn push_radial_gradient_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, clip: Rect, origin_x: f64, origin_y: f64, fill: &CanvasFillJson, opacity: f32, blend: Option<&str>, backdrop: Rgba) {
    let (scx, scy) = viewport.world_to_screen((origin_x + fill.cx) as f32, (origin_y + fill.cy) as f32, inner);
    let sr = (fill.r as f32 * viewport.zoom).max(0.5);
    draw.push_scissor(clip);
    let outer_radius = sr.max(clip.w.max(clip.h));
    for i in (0..CANVAS_GRADIENT_BANDS).rev() {
        let t = (i + 1) as f32 / CANVAS_GRADIENT_BANDS as f32;
        let radius = if i == CANVAS_GRADIENT_BANDS - 1 { outer_radius } else { sr * t };
        let color = canvas_apply_blend_mode(blend, backdrop, canvas_gradient_color_at(&fill.stops, t, opacity));
        draw.push_triangle_fan(&canvas_circle_points(scx, scy, radius.max(0.5), CANVAS_CIRCLE_SEGMENTS), color);
    }
    draw.pop_scissor();
}

/** 🖌️ Resolves and draws a Canvas2dScene draw record's `fill` (solid / linear / radial gradient) and
 * `stroke`, matching `drawSceneNode`'s fill/stroke resolution in `canvas-2d-host.tsx`. */
fn render_canvas_shape_fill(draw: &mut ui_wgpu::wgpu::DrawList, viewport: &Viewport, inner: Rect, shape_rect: Rect, layer: &CanvasLayer, opacity: f32, fallback_fill: Rgba, backdrop: Rgba, is_circle: bool) {
    let blend = layer.blend_mode.as_deref();
    match &layer.fill {
        Some(fill) if fill.kind.as_deref() == Some("linearGradient") && !fill.stops.is_empty() => {
            push_linear_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.kind.as_deref() == Some("radialGradient") && !fill.stops.is_empty() => {
            push_radial_gradient_fill(draw, viewport, inner, shape_rect, layer.x, layer.y, fill, opacity, blend, backdrop);
        }
        Some(fill) if fill.color.is_some() => {
            let solid = canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(fill.color.as_deref().unwrap_or(&[]), opacity));
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
        _ => {
            let solid = canvas_apply_blend_mode(blend, backdrop, fallback_fill);
            push_shape_fill(draw, shape_rect, solid, is_circle);
        }
    }
    if let Some(stroke) = &layer.stroke {
        let color = stroke.color.as_deref().map(|c| canvas_apply_blend_mode(blend, backdrop, canvas_color_channels(c, opacity))).unwrap_or_else(|| Rgba::new(0.58, 0.64, 0.72, 0.9 * opacity));
        let width = (stroke.width.unwrap_or(1.0) as f32).max(1.0);
        push_shape_outline(draw, shape_rect, color, width, is_circle, stroke.dash.as_deref());
    }
}
//#endregion Canvas2dShapes

/** 🟡️ Selection-ring colors ported verbatim from `drawBoundsLayer`'s literal
 * `"rgba(251, 191, 36, 0.95|0.28)"` strings in `canvas-2d-host.tsx` — an amber that isn't backed by
 * any `Theme` token, so it's kept local to this region rather than mapped onto `theme.accent`
 * (which resolves to the app's red/crimson accent and previously made the ring the wrong hue). */
const CANVAS2D_SELECTION_RING: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.95);
const CANVAS2D_SELECTION_GLOW: Rgba = Rgba::new(0.984_314, 0.749_02, 0.141_176, 0.28);

fn render_canvas_2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(canvas) = &scene.canvas_2d else {
        return render_placeholder("canvas-2d", bounds, ctx);
    };
    let layers: Vec<CanvasLayer> = serde_json::from_str(&canvas.layers_json).unwrap_or_default();
    let inner = bounds;
    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    let mut viewport = Viewport { x: canvas.camera_x as f32, y: canvas.camera_y as f32, zoom: canvas.zoom as f32 };
    let local = scene_state(&scene.surface_id);
    if local.viewport.zoom > 0.0 && scene.component_kind == SurfaceKind::Canvas2d {
        viewport = local.viewport;
    }
    draw_canvas_infinite_grid(ctx.draw, &viewport, inner, theme);
    let has_polyline = layers.iter().any(|layer| layer.kind == "polyline");
    if has_polyline {
        draw_checkerboard(ctx.draw, &viewport, inner, ctx.theme, 1024.0);
    }
    for (index, layer) in layers.iter().enumerate() {
        // 🗒️ `role === "meta"` (activeUtility bookkeeping) and `visible === false` records are
        // non-visual — skip rendering entirely, matches `layers.filter(role !== "meta")` in
        // `canvas-2d-host.tsx`'s `JsonLayersCanvasSession.renderFrame`.
        if !canvas_layer_should_render(layer) {
            continue;
        }
        let opacity = layer.opacity.unwrap_or(1.0).clamp(0.0, 1.0);
        let blend = layer.blend_mode.as_deref();
        if layer.kind == "image" {
            let source = layer.data_url.clone().or_else(|| layer.image.as_ref().and_then(|image| image.src.clone()));
            if let Some(data_url) = source.filter(|src| src.starts_with("data:")) {
                if let Some(key) = queue_canvas_image_upload(&scene.surface_id, &layer.id, &data_url) {
                    let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
                    let iw = layer.image.as_ref().and_then(|image| image.width).unwrap_or(layer.width);
                    let ih = layer.image.as_ref().and_then(|image| image.height).unwrap_or(layer.height);
                    let w = iw as f32 * viewport.zoom;
                    let h = ih as f32 * viewport.zoom;
                    ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], opacity);
                }
            }
            continue;
        }
        if layer.kind == "polyline" {
            if let Some(points) = &layer.points {
                let stroke = theme.diagram_stroke.with_alpha(theme.diagram_stroke.a * opacity);
                let seam_stroke = theme.diagram_seam.with_alpha(theme.diagram_seam.a * opacity);
                let width = (1.5 * viewport.zoom).max(1.0);
                for (edge_index, chunk) in points.chunks(2).enumerate() {
                    if chunk.len() < 2 {
                        continue;
                    }
                    let (x0, y0) = viewport.world_to_screen(chunk[0][0] as f32, chunk[0][1] as f32, inner);
                    let (x1, y1) = viewport.world_to_screen(chunk[1][0] as f32, chunk[1][1] as f32, inner);
                    let is_seam = layer.seams.as_ref().and_then(|seams| seams.get(edge_index)).copied().unwrap_or(0) != 0;
                    if is_seam {
                        draw_dashed_line(ctx.draw, x0, y0, x1, y1, seam_stroke, width);
                    } else {
                        ctx.draw.push_line(x0, y0, x1, y1, stroke, width);
                    }
                }
            }
            continue;
        }
        let hue = (index * 47 % 360) as f32;
        if layer.kind == "line" || layer.x0.is_some() {
            let x0 = layer.x0.unwrap_or(layer.x) as f32;
            let y0 = layer.y0.unwrap_or(layer.y) as f32;
            let x1 = layer.x1.unwrap_or(layer.x + layer.width) as f32;
            let y1 = layer.y1.unwrap_or(layer.y + layer.height) as f32;
            let (sx0, sy0) = viewport.world_to_screen(x0, y0, inner);
            let (sx1, sy1) = viewport.world_to_screen(x1, y1, inner);
            let base_stroke = layer
                .stroke
                .as_ref()
                .and_then(|stroke| stroke.color.as_deref())
                .map(|c| canvas_color_channels(c, opacity))
                .unwrap_or_else(|| Rgba::new(theme.diagram_accent.r + hue / 720.0, theme.diagram_accent.g, theme.diagram_accent.b, theme.diagram_accent.a * opacity));
            let stroke = canvas_apply_blend_mode(blend, theme.canvas_clear, base_stroke);
            ctx.draw.push_line(sx0, sy0, sx1, sy1, stroke, (2.0 * viewport.zoom).max(1.0));
            continue;
        }
        // 🖼️ Generic bounds-rect (or `kind === "circle"`) draw record — resolves solid/gradient
        // fill, blend-mode approximation, and stroke, matching `drawSceneNode`'s bounds-layer path.
        let (sx, sy) = viewport.world_to_screen(layer.x as f32, layer.y as f32, inner);
        let w = (layer.width as f32 * viewport.zoom).max(8.0);
        let h = (layer.height as f32 * viewport.zoom).max(8.0);
        let shape_rect = Rect::new(sx, sy, w, h);
        let is_circle = layer.kind == "circle";
        let fallback_fill = Rgba::new(theme.diagram_accent_fill.r + hue / 720.0, theme.diagram_accent_fill.g, theme.diagram_accent_fill.b, theme.diagram_accent_fill.a * opacity);
        render_canvas_shape_fill(ctx.draw, &viewport, inner, shape_rect, layer, opacity, fallback_fill, theme.canvas_clear, is_circle);
        // 🖊️ Overlay annotation: a two-pass selection highlight (soft outer glow + crisp amber ring)
        // drawn on top of the shape, matches `drawBoundsLayer`'s `isSelected` glow+ring pair in
        // `canvas-2d-host.tsx` (glow at +4px/width 5, ring at +0px/width 2.5, both amber).
        if layer.selected.unwrap_or(false) {
            if is_circle {
                let cx = sx + w * 0.5;
                let cy = sy + h * 0.5;
                let r = w.min(h) * 0.5;
                push_circle_outline(ctx.draw, cx, cy, r + 4.0, CANVAS2D_SELECTION_GLOW, 5.0);
                push_circle_outline(ctx.draw, cx, cy, r, CANVAS2D_SELECTION_RING, 2.5);
            } else {
                draw_ink_rect_outline(ctx.draw, sx - 4.0, sy - 4.0, w + 8.0, h + 8.0, CANVAS2D_SELECTION_GLOW, 5.0);
                draw_ink_rect_outline(ctx.draw, sx, sy, w, h, CANVAS2D_SELECTION_RING, 2.5);
            }
        }
        if let Some(text) = layer.text.as_ref().and_then(|text| text.content.as_deref()) {
            let size = layer.text.as_ref().and_then(|t| t.size).unwrap_or(14.0) as f32;
            draw_text(ctx, text, sx + 2.0, sy + size.max(8.0), size.max(8.0), theme.text);
        } else {
            let label = if layer.name.is_empty() { layer.id.as_str() } else { layer.name.as_str() };
            if !label.is_empty() {
                draw_text(ctx, label, sx + 4.0, sy + 14.0, theme.font_size_small, theme.text);
            }
        }
    }
    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::Generic, drag_axis: Some(DragAxis::Both), drag_data: None });
}
//#endregion Canvas2d

//#region Canvas2dTests
#[cfg(test)]
mod canvas2d_tests {
    use super::*;

    fn canvas_scene(surface_id: &str, layers_json: String) -> UiComponentSceneNode {
        UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::Canvas2d,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: Some(ui_wgpu::wgpu::Canvas2dScene { camera_x: 0.0, camera_y: 0.0, zoom: 1.0, layers_json }),
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: None,
            menu: None,
        }
    }

    /// 🖊️ A shape-selection ring should be tinted amber (matching `drawBoundsLayer`'s hardcoded
    /// `"rgba(251, 191, 36, ...)"` literals in `canvas-2d-host.tsx`), not `theme.accent` (the app's
    /// red/crimson design token) — see `CANVAS2D_SELECTION_RING`/`CANVAS2D_SELECTION_GLOW`.
    #[test]
    fn selected_shape_draws_the_amber_ring_and_glow_not_theme_accent() {
        let layers = json!([{ "kind": "rectangle", "id": "r1", "x": 10.0, "y": 10.0, "width": 40.0, "height": 20.0, "selected": true }]);
        let node = canvas_scene("s1", layers.to_string());
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            render_canvas_2d(&node, Rect::new(0.0, 0.0, 400.0, 300.0), &mut ctx);
        }
        let vertex_colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).map(|v| v.color).collect();
        let ring = CANVAS2D_SELECTION_RING;
        let glow = CANVAS2D_SELECTION_GLOW;
        assert!(vertex_colors.contains(&[ring.r, ring.g, ring.b, ring.a]), "expected the crisp amber ring color among vertices, got {vertex_colors:?}");
        assert!(vertex_colors.contains(&[glow.r, glow.g, glow.b, glow.a]), "expected the soft amber glow color among vertices, got {vertex_colors:?}");
        assert!(!vertex_colors.contains(&[theme.accent.r, theme.accent.g, theme.accent.b, theme.accent.a]), "the selection ring must no longer use theme.accent (the app's red/crimson token), got {vertex_colors:?}");
    }

    /// 🕒️ `scene_camera_action`'s exact key names — `{surfaceId, camera: {x, y, zoom}}`, matching
    /// `ink_set_camera_action`'s own shape (this crate's other camera-from-viewport builder).
    #[test]
    fn scene_camera_action_uses_surface_id_and_nested_camera_xyz_keys() {
        let action = scene_camera_action("s-cam-1", "controller-1", Viewport { x: 12.5, y: -3.0, zoom: 2.0 });
        assert_eq!(action.controller_id, "controller-1");
        assert_eq!(action.action, "setCamera");
        let args = action.args.expect("setCamera always carries args");
        assert_eq!(args.get("surfaceId").and_then(semio_framework::DslValue::as_str), Some("s-cam-1"));
        assert_eq!(args.get("camera").and_then(|value| value.get("x")).and_then(semio_framework::DslValue::as_f64), Some(12.5));
        assert_eq!(args.get("camera").and_then(|value| value.get("y")).and_then(semio_framework::DslValue::as_f64), Some(-3.0));
        assert_eq!(args.get("camera").and_then(|value| value.get("zoom")).and_then(semio_framework::DslValue::as_f64), Some(2.0));
    }

    /// 🕒️ A Canvas2d wheel tick mutates the viewport immediately but only SCHEDULES its `setCamera` —
    /// nothing is due yet, and nothing is dispatched, until the deadline sweep says otherwise.
    #[test]
    fn canvas2d_wheel_schedules_a_settled_camera_dispatch_without_firing_immediately() {
        let surface_id = "wheel-settle-canvas2d";
        let node = canvas_scene(surface_id, "[]".to_string());
        let actions = handle_scene_wheel(&node, Rect::new(0.0, 0.0, 400.0, 300.0), 50.0, 50.0, -100.0, false);
        assert!(actions.is_empty(), "wheel-zoom never dispatches inline anymore");
        let immediate = sweep_expired_scene_camera_dispatches(crate::app_now_ms());
        assert!(immediate.iter().all(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) != Some(surface_id)), "sweeping immediately (before the ~350ms settle window) must not yet report this surface");
        let due = sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
        let matched = due.iter().find(|action| action.args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str) == Some(surface_id)).expect("this surface's setCamera fires once its deadline has passed");
        assert_eq!(matched.controller_id, "controller");
        assert_eq!(matched.action, "setCamera");
    }

    /// 🕒️ A Canvas2d pan-drag (`SceneDragMode::PanViewport`) gets the identical settle-then-dispatch
    /// treatment as wheel-zoom — same deadline map, same sweep.
    #[test]
    fn canvas2d_pan_drag_schedules_a_settled_camera_dispatch() {
        let surface_id = "pan-settle-canvas2d";
        let node = canvas_scene(surface_id, "[]".to_string());
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag { mode: SceneDragMode::PanViewport, button: 1 });
        });
        let bounds = Rect::new(0.0, 0.0, 400.0, 300.0);
        let actions = handle_scene_pointer_move(&node, bounds, 60.0, 60.0, true, 1, 5.0, 5.0);
        assert!(
            actions.iter().all(|action| action.action != "setCamera"),
            "the pan itself never dispatches setCamera inline, even though Canvas2d's own \
         canvasPointerMove tracking action still fires alongside it"
        );
        let due = sweep_expired_scene_camera_dispatches(crate::app_now_ms() + 400.0);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(semio_framework::DslValue::as_str), Some(surface_id));
    }
}
//#endregion Canvas2dTests

//#region InkCanvas
// 📝️ Direct DrawList painting for ink-canvas, ported from ink-canvas-host.tsx (framework/renderer/react).

//#region InkCanvasModel
static INK_HOST_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn create_ink_host_id(prefix: &str) -> String {
    let next = INK_HOST_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    format!("{prefix}-host-{next}")
}

#[derive(Clone, Copy, Debug, Default)]
struct InkCameraF {
    x: f64,
    y: f64,
    zoom: f64,
}

impl From<InkCameraJson> for InkCameraF {
    fn from(camera: InkCameraJson) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InkCameraJson {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "ink_default_zoom")]
    zoom: f64,
}

fn ink_default_zoom() -> f64 {
    1.0
}

impl Default for InkCameraJson {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct InkDocumentJson {
    schema: String,
    id: String,
    camera: InkCameraJson,
    blocks: Vec<Value>,
    active_utility: Option<String>,
    grid_visible: Option<bool>,
    grid_spacing: Option<f64>,
    grid_subdivisions: Option<f64>,
    grid_opacity: Option<f64>,
    snap_enabled: Option<bool>,
    snap_grid_spacing: Option<f64>,
    pencil_width: Option<f64>,
    eraser_radius: Option<f64>,
    assets: HashMap<String, Value>,
}

impl Default for InkDocumentJson {
    fn default() -> Self {
        Self {
            schema: "ink.document".into(),
            id: "empty".into(),
            camera: InkCameraJson::default(),
            blocks: Vec::new(),
            active_utility: Some("selectDirect".into()),
            grid_visible: None,
            grid_spacing: None,
            grid_subdivisions: None,
            grid_opacity: None,
            snap_enabled: None,
            snap_grid_spacing: None,
            pencil_width: None,
            eraser_radius: None,
            assets: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct InkBoundsF {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl InkBoundsF {
    fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    fn intersects(&self, other: &InkBoundsF) -> bool {
        self.x < other.x + other.w && self.x + self.w > other.x && self.y < other.y + other.h && self.y + self.h > other.y
    }
}

fn ink_item_str<'a>(block: &'a Value, key: &str) -> &'a str {
    block.get(key).and_then(Value::as_str).unwrap_or("")
}

fn ink_item_id(block: &Value) -> &str {
    ink_item_str(block, "id")
}

fn ink_item_kind(block: &Value) -> &str {
    ink_item_str(block, "kind")
}

fn ink_item_visible(block: &Value) -> bool {
    block.get("visible").and_then(Value::as_bool).unwrap_or(true)
}

fn ink_item_locked(block: &Value) -> bool {
    block.get("locked").and_then(Value::as_bool).unwrap_or(false)
}

fn ink_item_num(block: &Value, key: &str) -> f64 {
    block.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn ink_item_bounds(block: &Value) -> InkBoundsF {
    let x = ink_item_num(block, "x");
    let y = ink_item_num(block, "y");
    let w = ink_item_num(block, "width");
    let h = ink_item_num(block, "height");
    if ink_item_kind(block) == "stroke" {
        if let Some(points) = block.get("points").and_then(Value::as_array) {
            if !points.is_empty() {
                let mut min_x = f64::INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_x = f64::NEG_INFINITY;
                let mut max_y = f64::NEG_INFINITY;
                for point in points {
                    let px = point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    min_x = min_x.min(px);
                    min_y = min_y.min(py);
                    max_x = max_x.max(px);
                    max_y = max_y.max(py);
                }
                return InkBoundsF { x: x + min_x, y: y + min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) };
            }
        }
    }
    InkBoundsF { x, y, w, h }
}

fn ink_effective_bounds(block: &Value, overrides: &HashMap<String, Value>) -> InkBoundsF {
    match overrides.get(ink_item_id(block)) {
        Some(over) => ink_item_bounds(over),
        None => ink_item_bounds(block),
    }
}

fn flatten_ink_items(blocks: &[Value]) -> Vec<&Value> {
    let mut out = Vec::new();
    fn visit<'a>(blocks: &'a [Value], out: &mut Vec<&'a Value>) {
        for block in blocks {
            out.push(block);
            if ink_item_kind(block) == "group" {
                if let Some(children) = block.get("children").and_then(Value::as_array) {
                    visit(children, out);
                }
            }
        }
    }
    visit(blocks, &mut out);
    out
}

fn find_ink_item<'a>(blocks: &'a [Value], id: &str) -> Option<&'a Value> {
    flatten_ink_items(blocks).into_iter().find(|block| ink_item_id(block) == id)
}

fn ink_items_at_point<'a>(blocks: &'a [Value], overrides: &HashMap<String, Value>, x: f64, y: f64) -> Vec<&'a Value> {
    let mut flat = flatten_ink_items(blocks);
    flat.reverse();
    flat.into_iter().filter(|block| ink_effective_bounds(block, overrides).contains_point(x, y)).collect()
}

fn ink_items_intersecting_rect(blocks: &[Value], overrides: &HashMap<String, Value>, rect: InkBoundsF) -> Vec<String> {
    flatten_ink_items(blocks).into_iter().filter(|block| ink_effective_bounds(block, overrides).intersects(&rect)).map(|block| ink_item_id(block).to_string()).collect()
}

fn ink_selection_bounds(blocks: &[Value], overrides: &HashMap<String, Value>, ids: &[String]) -> Option<InkBoundsF> {
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let selected: Vec<InkBoundsF> = flatten_ink_items(blocks).into_iter().filter(|block| id_set.contains(ink_item_id(block))).map(|block| ink_effective_bounds(block, overrides)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for bounds in &selected {
        min_x = min_x.min(bounds.x);
        min_y = min_y.min(bounds.y);
        max_x = max_x.max(bounds.x + bounds.w);
        max_y = max_y.max(bounds.y + bounds.h);
    }
    Some(InkBoundsF { x: min_x, y: min_y, w: (max_x - min_x).max(1.0), h: (max_y - min_y).max(1.0) })
}

fn ink_scale_value(v: f64, from_min: f64, from_size: f64, to_min: f64, to_size: f64) -> f64 {
    if from_size <= 0.0 {
        return to_min;
    }
    to_min + ((v - from_min) / from_size) * to_size
}

fn scale_ink_item(block: &Value, from: InkBoundsF, to: InkBoundsF) -> Value {
    let bounds = ink_item_bounds(block);
    let next_x = ink_scale_value(bounds.x, from.x, from.w, to.x, to.w);
    let next_y = ink_scale_value(bounds.y, from.y, from.h, to.y, to.h);
    let next_w = (ink_scale_value(bounds.x + bounds.w, from.x, from.w, to.x, to.w) - next_x).max(8.0);
    let next_h = (ink_scale_value(bounds.y + bounds.h, from.y, from.h, to.y, to.h) - next_y).max(8.0);
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(next_x));
        obj.insert("y".into(), json!(next_y));
        obj.insert("width".into(), json!(next_w));
        obj.insert("height".into(), json!(next_h));
        if ink_item_kind(block) == "stroke" {
            let scale_x = if from.w > 0.0 { to.w / from.w } else { 1.0 };
            let scale_y = if from.h > 0.0 { to.h / from.h } else { 1.0 };
            if let Some(points) = block.get("points").and_then(Value::as_array) {
                let scaled: Vec<Value> = points
                    .iter()
                    .map(|p| {
                        let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0) * scale_x;
                        let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0) * scale_y;
                        json!([px, py])
                    })
                    .collect();
                obj.insert("points".into(), Value::Array(scaled));
            }
        }
    }
    cloned
}

fn ink_resize_bounds(from: InkBoundsF, handle: &str, dx: f64, dy: f64, min_size: f64) -> InkBoundsF {
    let mut x = from.x;
    let mut y = from.y;
    let mut w = from.w;
    let mut h = from.h;
    if handle.contains('e') {
        w = (w + dx).max(min_size);
    }
    if handle.contains('w') {
        let next_w = (w - dx).max(min_size);
        x += w - next_w;
        w = next_w;
    }
    if handle.contains('s') {
        h = (h + dy).max(min_size);
    }
    if handle.contains('n') {
        let next_h = (h - dy).max(min_size);
        y += h - next_h;
        h = next_h;
    }
    InkBoundsF { x, y, w, h }
}

fn ink_snap_coordinate(v: f64, spacing: f64) -> f64 {
    if spacing <= 0.0 {
        v
    } else {
        (v / spacing).round() * spacing
    }
}

fn ink_snap_point(x: f64, y: f64, spacing: f64) -> (f64, f64) {
    (ink_snap_coordinate(x, spacing), ink_snap_coordinate(y, spacing))
}

fn ink_maybe_snap(doc: &InkDocumentJson, x: f64, y: f64) -> (f64, f64) {
    if doc.snap_enabled.unwrap_or(false) {
        ink_snap_point(x, y, doc.snap_grid_spacing.unwrap_or(8.0))
    } else {
        (x, y)
    }
}

fn ink_item_with_position(block: &Value, x: f64, y: f64) -> Value {
    let mut cloned = block.clone();
    if let Some(obj) = cloned.as_object_mut() {
        obj.insert("x".into(), json!(x));
        obj.insert("y".into(), json!(y));
    }
    cloned
}

fn create_ink_item(kind: &str, x: f64, y: f64) -> Value {
    let id = create_ink_host_id(kind);
    match kind {
        "image" => json!({
            "id": id, "name": "Image", "kind": "image", "x": x, "y": y, "width": 240.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false, "imageKey": "placeholder",
        }),
        "table" => json!({
            "id": id, "name": "Table", "kind": "table", "x": x, "y": y, "width": 320.0, "height": 160.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "columns": ["A", "B", "C"],
            "rows": [
                [{"content": ""}, {"content": ""}, {"content": ""}],
                [{"content": ""}, {"content": ""}, {"content": ""}],
            ],
        }),
        "math" => json!({
            "id": id, "name": "Math", "kind": "math", "x": x, "y": y, "width": 200.0, "height": 80.0,
            "rotation": 0.0, "visible": true, "locked": false, "tex": "E = mc^2", "displayMode": true,
        }),
        "stroke" => json!({
            "id": id, "name": "Ink", "kind": "stroke", "x": x, "y": y, "width": 1.0, "height": 1.0,
            "rotation": 0.0, "visible": true, "locked": false, "points": [], "strokeWidth": 3.0, "color": [0.0, 0.0, 0.0, 1.0],
        }),
        "group" => json!({
            "id": id, "name": "Group", "kind": "group", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false, "children": [],
        }),
        _ => json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": 280.0, "height": 120.0,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [{"runs": [{"text": ""}]}], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        }),
    }
}

fn ink_text_plain(block: &Value) -> String {
    block
        .get("paragraphs")
        .and_then(Value::as_array)
        .map(|paragraphs| {
            paragraphs
                .iter()
                .map(|paragraph| paragraph.get("runs").and_then(Value::as_array).map(|runs| runs.iter().filter_map(|run| run.get("text").and_then(Value::as_str)).collect::<String>()).unwrap_or_default())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}

fn point_segment_distance(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    if dx == 0.0 && dy == 0.0 {
        return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
    }
    let t = (((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((px - (x1 + t * dx)).powi(2) + (py - (y1 + t * dy)).powi(2)).sqrt()
}

fn ink_points(block: &Value) -> Vec<(f64, f64)> {
    let bx = ink_item_num(block, "x");
    let by = ink_item_num(block, "y");
    block
        .get("points")
        .and_then(Value::as_array)
        .map(|points| {
            points
                .iter()
                .map(|p| {
                    let px = p.get(0).and_then(Value::as_f64).unwrap_or(0.0);
                    let py = p.get(1).and_then(Value::as_f64).unwrap_or(0.0);
                    (bx + px, by + py)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn ink_hits_point(block: &Value, x: f64, y: f64, threshold: f64) -> bool {
    let points = ink_points(block);
    let stroke_width = ink_item_num(block, "strokeWidth");
    if points.len() < 2 {
        return points.first().map(|p| ((x - p.0).powi(2) + (y - p.1).powi(2)).sqrt() <= threshold).unwrap_or(false);
    }
    points.windows(2).any(|w| point_segment_distance(x, y, w[0].0, w[0].1, w[1].0, w[1].1) <= threshold + stroke_width / 2.0)
}

fn erase_ink_stroke_events(blocks: &[Value], x: f64, y: f64, threshold: f64) -> Vec<Value> {
    flatten_ink_items(blocks).into_iter().filter(|block| ink_item_kind(block) == "stroke" && ink_hits_point(block, x, y, threshold)).map(|block| json!({ "operation": "removeBlock", "blockId": ink_item_id(block) })).collect()
}

fn erase_ink_stroke_points_in_item(block: &Value, x: f64, y: f64, radius: f64) -> Vec<Value> {
    let bx = ink_item_num(block, "x");
    let by = ink_item_num(block, "y");
    let points = block.get("points").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut kept_indices = Vec::new();
    for (index, point) in points.iter().enumerate() {
        let px = bx + point.get(0).and_then(Value::as_f64).unwrap_or(0.0);
        let py = by + point.get(1).and_then(Value::as_f64).unwrap_or(0.0);
        if ((px - x).powi(2) + (py - y).powi(2)).sqrt() > radius {
            kept_indices.push(index);
        }
    }
    if kept_indices.len() == points.len() {
        return vec![block.clone()];
    }
    if kept_indices.is_empty() {
        return Vec::new();
    }
    let mut runs: Vec<Vec<Value>> = Vec::new();
    let mut current: Vec<Value> = vec![points[kept_indices[0]].clone()];
    for window in kept_indices.windows(2) {
        if window[1] - window[0] > 1 {
            if current.len() >= 2 {
                runs.push(current);
            }
            current = vec![points[window[1]].clone()];
        } else {
            current.push(points[window[1]].clone());
        }
    }
    if current.len() >= 2 {
        runs.push(current);
    }
    let name = ink_item_str(block, "name").to_string();
    runs.into_iter()
        .enumerate()
        .map(|(index, pts)| {
            let mut cloned = block.clone();
            if let Some(obj) = cloned.as_object_mut() {
                if index > 0 {
                    obj.insert("id".into(), json!(create_ink_host_id("stroke")));
                    obj.insert("name".into(), json!(format!("{name} fragment")));
                }
                obj.insert("points".into(), Value::Array(pts));
            }
            cloned
        })
        .collect()
}

fn erase_ink_stroke_points_events(blocks: &[Value], x: f64, y: f64, radius: f64) -> Vec<Value> {
    let mut events = Vec::new();
    for block in flatten_ink_items(blocks) {
        if ink_item_kind(block) != "stroke" {
            continue;
        }
        let fragments = erase_ink_stroke_points_in_item(block, x, y, radius);
        if fragments.len() == 1 && fragments[0] == *block {
            continue;
        }
        events.push(json!({ "operation": "removeBlock", "blockId": ink_item_id(block) }));
        for fragment in fragments {
            events.push(json!({ "operation": "addBlock", "block": fragment }));
        }
    }
    events
}

fn ink_screen_to_world(camera: InkCameraF, inner: Rect, sx: f32, sy: f32) -> (f64, f64) {
    let lx = (sx - inner.x) as f64;
    let ly = (sy - inner.y) as f64;
    ((lx - camera.x) / camera.zoom, (ly - camera.y) / camera.zoom)
}

fn ink_world_to_screen(camera: InkCameraF, inner: Rect, wx: f64, wy: f64) -> (f32, f32) {
    (inner.x + (wx * camera.zoom + camera.x) as f32, inner.y + (wy * camera.zoom + camera.y) as f32)
}

fn positive_mod_f32(v: f32, m: f32) -> f32 {
    if m <= 0.0 {
        0.0
    } else {
        ((v % m) + m) % m
    }
}
//#endregion InkCanvasModel

//#region InkCanvasState
fn ink_current_camera(scene: &UiComponentSceneNode) -> InkCameraF {
    let state = scene_state(&scene.surface_id);
    if let Some((x, y, zoom)) = state.ink_camera {
        return InkCameraF { x, y, zoom };
    }
    scene.ink_canvas.as_ref().and_then(|ink| serde_json::from_str::<InkDocumentJson>(&ink.document_json).ok()).map(|doc| InkCameraF::from(doc.camera)).unwrap_or_default()
}

fn ink_events_json(events: &[Value]) -> String {
    Value::Array(events.to_vec()).to_string()
}

fn ink_apply_events_action(scene: &UiComponentSceneNode, events: &[Value], phase: &str, select_ids: Option<&[String]>) -> ActionDescriptor {
    let mut args = json!({
        "surfaceId": scene.surface_id,
        "eventsJson": ink_events_json(events),
        "phase": phase,
    });
    if let Some(ids) = select_ids {
        args["selectIds"] = json!(ids);
    }
    scene_action(scene, "inkApplyEvents", args)
}

fn ink_set_selection_action(scene: &UiComponentSceneNode, ids: &[String]) -> ActionDescriptor {
    scene_action(scene, "setSelection", json!({ "surfaceId": scene.surface_id, "ids": ids }))
}

fn ink_set_hover_action(scene: &UiComponentSceneNode, id: Option<&str>) -> ActionDescriptor {
    scene_action(scene, "setHover", json!({ "surfaceId": scene.surface_id, "id": id }))
}

fn ink_set_camera_action(scene: &UiComponentSceneNode, camera: InkCameraF) -> ActionDescriptor {
    scene_action(scene, "setCamera", json!({ "surfaceId": scene.surface_id, "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom } }))
}

const INK_RESIZE_HANDLES: [&str; 8] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];

fn ink_resize_handle_screen_pos(handle: &str, sx: f32, sy: f32, w: f32, h: f32, size: f32) -> (f32, f32) {
    let half = size * 0.5;
    let x = if handle.contains('w') {
        sx - half
    } else if handle.contains('e') {
        sx + w - half
    } else {
        sx + w * 0.5 - half
    };
    let y = if handle.contains('n') {
        sy - half
    } else if handle.contains('s') {
        sy + h - half
    } else {
        sy + h * 0.5 - half
    };
    (x, y)
}

fn ink_resize_handle_at(bounds: InkBoundsF, camera: InkCameraF, inner: Rect, sx: f32, sy: f32, hit_radius: f32) -> Option<&'static str> {
    let (bx, by) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    for handle in INK_RESIZE_HANDLES {
        let (hx, hy) = ink_resize_handle_screen_pos(handle, bx, by, w, h, 8.0);
        let cx = hx + 4.0;
        let cy = hy + 4.0;
        if ((sx - cx).powi(2) + (sy - cy).powi(2)).sqrt() <= hit_radius {
            return Some(handle);
        }
    }
    None
}

/** @emoji 📝️ Pointer-down entry point for ink-canvas: mirrors handlePointerDown in ink-canvas-host.tsx. */
fn ink_pointer_down(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, button: i16, shift: bool) -> Vec<ActionDescriptor> {
    let Some(ink) = &scene.ink_canvas else {
        return Vec::new();
    };
    if ink.view_mode == "navigator" || !ink.interactive {
        return Vec::new();
    }
    let doc: InkDocumentJson = serde_json::from_str(&ink.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&ink.selection_json).unwrap_or_default();
    let state = scene_state(&scene.surface_id);
    let camera = state.ink_camera.map(|(cx, cy, cz)| InkCameraF { x: cx, y: cy, zoom: cz }).unwrap_or_else(|| InkCameraF::from(doc.camera.clone()));
    let utility = doc.active_utility.clone().unwrap_or_else(|| "selectDirect".into());
    let mut actions = Vec::new();

    let selection_bounds = ink_selection_bounds(&doc.blocks, &state.ink_overrides, &selected_ids);
    let show_handles = (utility == "selectDirect" || utility == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if button == 0 && show_handles {
        if let Some(bounds) = selection_bounds {
            if let Some(handle) = ink_resize_handle_at(bounds, camera, inner, x, y, 8.0) {
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag { mode: SceneDragMode::InkResize { handle: handle.to_string(), from: bounds, start_x: x, start_y: y, selected_ids: selected_ids.clone() }, button });
                });
                return actions;
            }
        }
    }

    if utility == "pan" || button == 1 {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkPan { start_x: x, start_y: y, camera_x: camera.x, camera_y: camera.y, zoom: camera.zoom }, button });
        });
        return actions;
    }

    if button != 0 {
        return actions;
    }

    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);

    if utility == "eraserStroke" || utility == "eraserPoint" {
        let events = if utility == "eraserStroke" { erase_ink_stroke_events(&doc.blocks, world_x, world_y, 8.0) } else { erase_ink_stroke_points_events(&doc.blocks, world_x, world_y, doc.eraser_radius.unwrap_or(12.0)) };
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkEraser { mode: utility.clone() }, button });
        });
        if !events.is_empty() {
            actions.push(ink_apply_events_action(scene, &events, "begin", None));
        }
        return actions;
    }

    if utility == "selectMarquee" {
        mutate_scene_state(&scene.surface_id, |s| {
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkMarqueeDrag { start_x: x, start_y: y }, button });
            s.ink_marquee_points = vec![(x, y)];
        });
        return actions;
    }

    if utility == "pencil" {
        let block = create_ink_item("stroke", world_x, world_y);
        let block_id = ink_item_id(&block).to_string();
        mutate_scene_state(&scene.surface_id, |s| {
            s.ink_overrides.insert(block_id.clone(), block.clone());
            s.drag = Some(SceneDrag { mode: SceneDragMode::InkStroke { block_id: block_id.clone() }, button });
        });
        actions.push(ink_apply_events_action(scene, &[json!({ "operation": "addBlock", "block": block })], "begin", Some(&[block_id])));
        return actions;
    }

    if utility == "text" || utility == "image" || utility == "table" || utility == "math" {
        let (px, py) = ink_maybe_snap(&doc, world_x, world_y);
        let block = create_ink_item(&utility, px, py);
        let block_id = ink_item_id(&block).to_string();
        actions.push(ink_apply_events_action(scene, &[json!({ "operation": "addBlock", "block": block })], "atomic", Some(&[block_id])));
        return actions;
    }

    let hits = ink_items_at_point(&doc.blocks, &state.ink_overrides, world_x, world_y);
    let top = hits.first().copied();
    match top {
        Some(top_block) if !ink_item_locked(top_block) => {
            if utility == "selectDirect" {
                let top_id = ink_item_id(top_block).to_string();
                let next_selection = if shift {
                    let mut ids: Vec<String> = selected_ids.clone();
                    if !ids.contains(&top_id) {
                        ids.push(top_id.clone());
                    }
                    ids
                } else {
                    vec![top_id.clone()]
                };
                actions.push(ink_set_selection_action(scene, &next_selection));
                let move_ids: Vec<String> = if selected_ids.contains(&top_id) { selected_ids.clone() } else { vec![top_id.clone()] };
                let mut origins = HashMap::new();
                for id in &move_ids {
                    if let Some(b) = find_ink_item(&doc.blocks, id) {
                        let eff = state.ink_overrides.get(id).unwrap_or(b);
                        origins.insert(id.clone(), (ink_item_num(eff, "x"), ink_item_num(eff, "y")));
                    }
                }
                mutate_scene_state(&scene.surface_id, |s| {
                    s.drag = Some(SceneDrag { mode: SceneDragMode::InkMove { origins, start_x: x, start_y: y }, button });
                });
            }
        }
        _ => {
            if utility == "selectDirect" {
                actions.push(ink_set_selection_action(scene, &[]));
            }
        }
    }
    actions
}

/** @emoji 📝️ Pointer-up entry point for ink-canvas: commits the active gesture and finalizes marquee selection. */
fn ink_pointer_up(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    let state = scene_state(&scene.surface_id);
    let Some(drag) = state.drag.clone() else {
        return actions;
    };
    let doc: InkDocumentJson = scene.ink_canvas.as_ref().map(|n| serde_json::from_str(&n.document_json).unwrap_or_default()).unwrap_or_default();
    match &drag.mode {
        SceneDragMode::InkMove { origins, .. } => {
            let mut events = Vec::new();
            for id in origins.keys() {
                if let Some(block) = state.ink_overrides.get(id).cloned().or_else(|| find_ink_item(&doc.blocks, id).cloned()) {
                    let updated = if doc.snap_enabled.unwrap_or(false) {
                        let spacing = doc.snap_grid_spacing.unwrap_or(8.0);
                        let (sx, sy) = ink_snap_point(ink_item_num(&block, "x"), ink_item_num(&block, "y"), spacing);
                        ink_item_with_position(&block, sx, sy)
                    } else {
                        block
                    };
                    events.push(json!({ "operation": "updateBlock", "blockId": id, "block": updated }));
                }
            }
            actions.push(ink_apply_events_action(scene, &events, "commit", None));
        }
        SceneDragMode::InkResize { selected_ids, .. } => {
            let mut events = Vec::new();
            for id in selected_ids {
                if let Some(block) = state.ink_overrides.get(id).cloned() {
                    events.push(json!({ "operation": "updateBlock", "blockId": id, "block": block }));
                }
            }
            actions.push(ink_apply_events_action(scene, &events, "commit", None));
        }
        SceneDragMode::InkStroke { block_id } => {
            if let Some(block) = state.ink_overrides.get(block_id).cloned() {
                actions.push(ink_apply_events_action(scene, &[json!({ "operation": "updateBlock", "blockId": block_id, "block": block })], "commit", None));
            } else {
                actions.push(ink_apply_events_action(scene, &[], "commit", None));
            }
        }
        SceneDragMode::InkEraser { .. } => {
            actions.push(ink_apply_events_action(scene, &[], "commit", None));
        }
        SceneDragMode::InkMarqueeDrag { start_x, start_y } => {
            let x0 = start_x.min(x);
            let y0 = start_y.min(y);
            let w = (x - start_x).abs();
            let h = (y - start_y).abs();
            if w >= 4.0 || h >= 4.0 {
                let camera = ink_current_camera(scene);
                let (wx0, wy0) = ink_screen_to_world(camera, inner, x0, y0);
                let (wx1, wy1) = ink_screen_to_world(camera, inner, x0 + w, y0 + h);
                let world_rect = InkBoundsF { x: wx0.min(wx1), y: wy0.min(wy1), w: (wx1 - wx0).abs(), h: (wy1 - wy0).abs() };
                let ids = ink_items_intersecting_rect(&doc.blocks, &state.ink_overrides, world_rect);
                actions.push(ink_set_selection_action(scene, &ids));
            }
        }
        _ => {}
    }
    mutate_scene_state(&scene.surface_id, |s| {
        s.drag = None;
        s.ink_marquee_points.clear();
    });
    actions
}

/** @emoji 📝️ Pointer-move hover entry point for ink-canvas: mirrors the `!dragState` hover branch of handlePointerMove. */
fn ink_hover_move(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let Some(ink) = &scene.ink_canvas else {
        return Vec::new();
    };
    if ink.view_mode == "navigator" || !ink.interactive {
        return Vec::new();
    }
    let doc: InkDocumentJson = serde_json::from_str(&ink.document_json).unwrap_or_default();
    let camera = ink_current_camera(scene);
    let (wx, wy) = ink_screen_to_world(camera, inner, x, y);
    let state = scene_state(&scene.surface_id);
    let hits = ink_items_at_point(&doc.blocks, &state.ink_overrides, wx, wy);
    let top_id = hits.first().map(|block| ink_item_id(block).to_string());
    if ink.hovered_id.as_deref() == top_id.as_deref() {
        return Vec::new();
    }
    vec![ink_set_hover_action(scene, top_id.as_deref())]
}

/** @emoji 📝️ Wheel entry point for ink-canvas: zoom-at-cursor, mirrors handleWheel in ink-canvas-host.tsx. */
fn ink_wheel(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    let Some(ink) = &scene.ink_canvas else {
        return Vec::new();
    };
    if ink.view_mode == "navigator" {
        return Vec::new();
    }
    let camera = ink_current_camera(scene);
    let zoom_factor: f64 = if delta < 0.0 { 1.08 } else { 0.92 };
    let next_zoom = (camera.zoom * zoom_factor).clamp(0.1, 8.0);
    let (wx, wy) = ink_screen_to_world(camera, inner, x, y);
    let next = InkCameraF { x: (x - inner.x) as f64 - wx * next_zoom, y: (y - inner.y) as f64 - wy * next_zoom, zoom: next_zoom };
    mutate_scene_state(&scene.surface_id, |s| {
        s.ink_camera = Some((next.x, next.y, next.zoom));
    });
    vec![ink_set_camera_action(scene, next)]
}
//#endregion InkCanvasState

//#region InkCanvasRender
fn draw_ink_rect_outline(draw: &mut ui_wgpu::wgpu::DrawList, x: f32, y: f32, w: f32, h: f32, color: Rgba, width: f32) {
    draw.push_line(x, y, x + w, y, color, width);
    draw.push_line(x + w, y, x + w, y + h, color, width);
    draw.push_line(x + w, y + h, x, y + h, color, width);
    draw.push_line(x, y + h, x, y, color, width);
}

fn draw_ink_grid(draw: &mut ui_wgpu::wgpu::DrawList, camera: InkCameraF, inner: Rect, theme: &Theme, spacing: f64, subdivisions: u32, opacity: f64) {
    let major_px = (spacing * camera.zoom) as f32;
    if major_px < 2.0 {
        return;
    }
    let minor_px = major_px / subdivisions.max(1) as f32;
    let offset_x = positive_mod_f32(camera.x as f32, major_px);
    let offset_y = positive_mod_f32(camera.y as f32, major_px);
    let color = theme.separator.with_alpha((theme.separator.a * opacity as f32).max(0.05));
    let minor_color = color.with_alpha(color.a * 0.55);

    let mut wx = inner.x + positive_mod_f32(offset_x, major_px) - major_px;
    while wx < inner.x + inner.w {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let mx = wx + s as f32 * minor_px;
                if mx >= inner.x && mx <= inner.x + inner.w {
                    draw.push_line(mx, inner.y, mx, inner.y + inner.h, minor_color, 0.5);
                }
            }
        }
        if wx >= inner.x && wx <= inner.x + inner.w {
            draw.push_line(wx, inner.y, wx, inner.y + inner.h, color, 1.0);
        }
        wx += major_px;
    }
    let mut wy = inner.y + positive_mod_f32(offset_y, major_px) - major_px;
    while wy < inner.y + inner.h {
        if subdivisions > 1 {
            for s in 1..subdivisions {
                let my = wy + s as f32 * minor_px;
                if my >= inner.y && my <= inner.y + inner.h {
                    draw.push_line(inner.x, my, inner.x + inner.w, my, minor_color, 0.5);
                }
            }
        }
        if wy >= inner.y && wy <= inner.y + inner.h {
            draw.push_line(inner.x, wy, inner.x + inner.w, wy, color, 1.0);
        }
        wy += major_px;
    }
}

fn draw_ink_table(ctx: &mut FrameworkWidgetContext<'_>, block: &Value, sx: f32, sy: f32, w: f32, h: f32, theme: &Theme) {
    let columns: Vec<String> = block.get("columns").and_then(Value::as_array).map(|c| c.iter().filter_map(|v| v.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let rows: Vec<Vec<String>> = block
        .get("rows")
        .and_then(Value::as_array)
        .map(|rows| rows.iter().map(|row| row.as_array().map(|cells| cells.iter().map(|cell| cell.get("content").and_then(Value::as_str).unwrap_or("").to_string()).collect()).unwrap_or_default()).collect())
        .unwrap_or_default();
    let col_count = columns.len().max(1);
    let row_count = rows.len() + 1;
    let col_w = w / col_count as f32;
    let row_h = h / row_count as f32;
    let font = theme.font_size_small.min(row_h * 0.6).max(6.0);
    for (index, label) in columns.iter().enumerate() {
        draw_text(ctx, label, sx + index as f32 * col_w + 3.0, sy + row_h * 0.7, font, theme.text_muted);
    }
    for (row_index, row) in rows.iter().enumerate() {
        let ry = sy + (row_index + 1) as f32 * row_h;
        for (col_index, cell) in row.iter().enumerate() {
            draw_text(ctx, cell, sx + col_index as f32 * col_w + 3.0, ry + row_h * 0.7, font, theme.text);
        }
    }
    for index in 0..=col_count {
        let x = sx + index as f32 * col_w;
        ctx.draw.push_line(x, sy, x, sy + h, theme.separator, 0.5);
    }
    for index in 0..=row_count {
        let y = sy + index as f32 * row_h;
        ctx.draw.push_line(sx, y, sx + w, y, theme.separator, 0.5);
    }
}

fn draw_ink_image(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, doc: &InkDocumentJson, sx: f32, sy: f32, w: f32, h: f32) {
    let theme = ctx.theme;
    let image_key = ink_item_str(block, "imageKey");
    if let Some(asset) = doc.assets.get(image_key) {
        let mime = asset.get("mime").and_then(Value::as_str).unwrap_or("image/png");
        let data = asset.get("data").and_then(Value::as_str).unwrap_or("");
        let data_url = if data.starts_with("data:") { data.to_string() } else { format!("data:{mime};base64,{data}") };
        if let Some(key) = queue_canvas_image_upload(&scene.surface_id, ink_item_id(block), &data_url) {
            ctx.draw.push_raster_quad(&key, [sx, sy, w.max(1.0), h.max(1.0)], [0.0, 0.0, 1.0, 1.0], 1.0);
            return;
        }
    }
    draw_text(ctx, image_key, sx + 6.0, sy + h * 0.5, theme.font_size_small, theme.text_muted);
}

fn draw_ink_item(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, block: &Value, camera: InkCameraF, inner: Rect, doc: &InkDocumentJson, selected: bool, hovered: bool) {
    let theme = ctx.theme;
    let kind = ink_item_kind(block);
    let bounds = ink_item_bounds(block);
    let (sx, sy) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;

    if kind == "stroke" {
        let points = ink_points(block);
        if points.len() >= 2 {
            let color = block
                .get("color")
                .and_then(Value::as_array)
                .map(|c| {
                    let get = |i: usize| c.get(i).and_then(Value::as_f64).unwrap_or(0.0) as f32;
                    Rgba::new(get(0), get(1), get(2), get(3))
                })
                .unwrap_or(theme.text);
            let stroke_width = (ink_item_num(block, "strokeWidth") as f32 * camera.zoom as f32).max(1.0);
            let screen_points: Vec<(f32, f32)> = points.iter().map(|p| ink_world_to_screen(camera, inner, p.0, p.1)).collect();
            for pair in screen_points.windows(2) {
                ctx.draw.push_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, color, stroke_width);
            }
        }
        return;
    }

    // 🎨️ `bg-background/90` in `ink-canvas-host.tsx` — `theme.panel` (the app-chrome surface token)
    // previously stood in for the canvas-item card token, which is `background`, not `panel`.
    let bg = theme.background;
    ctx.draw.push_rounded([sx, sy, w.max(4.0), h.max(4.0)], bg.with_alpha(0.9), theme.border_radius.min(6.0));

    match kind {
        "text" => {
            let text = ink_text_plain(block);
            let font_size = (ink_item_num(block, "fontSize").max(8.0) as f32 * camera.zoom as f32).max(6.0);
            draw_text_wrapped(ctx, &text, sx + 6.0, sy + 4.0, (w - 12.0).max(1.0), font_size, theme.text);
        }
        "math" => {
            let tex = ink_item_str(block, "tex");
            draw_text(ctx, tex, sx + 8.0, sy + h * 0.5 + 4.0, theme.font_size_body.max(8.0), theme.text);
        }
        "table" => draw_ink_table(ctx, block, sx, sy, w.max(4.0), h.max(4.0), theme),
        "image" => draw_ink_image(ctx, scene, block, doc, sx, sy, w.max(4.0), h.max(4.0)),
        "group" => {
            let children_len = block.get("children").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
            draw_text(ctx, &format!("Group · {children_len} children"), sx + 6.0, sy + 16.0, theme.font_size_small, theme.text_muted);
        }
        _ => {}
    }

    let border = if selected {
        theme.accent
    } else if hovered {
        theme.accent.with_alpha(theme.accent.a * 0.6)
    } else {
        theme.panel_border
    };
    let border_w = if selected { 2.0 } else { 1.0 };
    draw_ink_rect_outline(ctx.draw, sx, sy, w.max(4.0), h.max(4.0), border, border_w);
}

fn draw_ink_selection_chrome(draw: &mut ui_wgpu::wgpu::DrawList, theme: &Theme, camera: InkCameraF, inner: Rect, bounds: InkBoundsF, show_handles: bool) {
    let (sx, sy) = ink_world_to_screen(camera, inner, bounds.x, bounds.y);
    let w = (bounds.w * camera.zoom) as f32;
    let h = (bounds.h * camera.zoom) as f32;
    draw_ink_rect_outline(draw, sx, sy, w, h, theme.accent, 1.5);
    if !show_handles {
        return;
    }
    let handle_size = 8.0;
    for handle in INK_RESIZE_HANDLES {
        let (hx, hy) = ink_resize_handle_screen_pos(handle, sx, sy, w, h, handle_size);
        draw.push_rounded([hx, hy, handle_size, handle_size], theme.background, 1.0);
        draw_ink_rect_outline(draw, hx, hy, handle_size, handle_size, theme.accent, 1.0);
    }
}

fn render_ink_canvas(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::wgpu::GpuContext) {
    let _ = gpu;
    let theme = ctx.theme;
    let Some(ink) = &scene.ink_canvas else {
        return render_placeholder("ink-canvas", bounds, ctx);
    };
    let doc: InkDocumentJson = serde_json::from_str(&ink.document_json).unwrap_or_default();
    let selected_ids: Vec<String> = serde_json::from_str(&ink.selection_json).unwrap_or_default();
    let selected_set: HashSet<&str> = selected_ids.iter().map(String::as_str).collect();
    let hovered_id = ink.hovered_id.clone();
    let is_navigator = ink.view_mode == "navigator";
    let inner = bounds;

    let state = scene_state(&scene.surface_id);
    let camera = state.ink_camera.map(|(x, y, zoom)| InkCameraF { x, y, zoom }).unwrap_or_else(|| InkCameraF::from(doc.camera.clone()));

    ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], theme.canvas_clear);
    ctx.draw.push_scissor(inner);

    if doc.grid_visible.unwrap_or(true) && !is_navigator {
        draw_ink_grid(ctx.draw, camera, inner, theme, doc.grid_spacing.unwrap_or(32.0), doc.grid_subdivisions.unwrap_or(4.0).max(1.0) as u32, doc.grid_opacity.unwrap_or(0.35));
    }

    let overrides = state.ink_overrides.clone();
    let blocks = flatten_ink_items(&doc.blocks);
    for block in blocks.iter().copied() {
        let effective = overrides.get(ink_item_id(block)).unwrap_or(block);
        if !ink_item_visible(effective) {
            continue;
        }
        let id = ink_item_id(block);
        let selected = selected_set.contains(id);
        let hovered = hovered_id.as_deref() == Some(id);
        draw_ink_item(ctx, scene, effective, camera, inner, &doc, selected, hovered);
    }

    let selection_bounds = ink_selection_bounds(&doc.blocks, &overrides, &selected_ids);
    let utility = doc.active_utility.clone().unwrap_or_else(|| "selectDirect".into());
    let show_handles = !is_navigator && (utility == "selectDirect" || utility == "selectMarquee") && selection_bounds.is_some() && !selected_ids.is_empty();
    if let Some(sel) = selection_bounds {
        draw_ink_selection_chrome(ctx.draw, theme, camera, inner, sel, show_handles);
    }

    if state.ink_marquee_points.len() >= 2 {
        let points: Vec<[f32; 2]> = state.ink_marquee_points.iter().map(|p| [p.0, p.1]).collect();
        ui_wgpu::wgpu::paint_selection_marquee(ctx.draw, theme, false, false, &points, false);
    }

    ctx.draw.pop_scissor();

    ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
}
//#endregion InkCanvasRender

//#region RasterFrameCostTests
#[cfg(test)]
mod raster_frame_cost_tests {
    use super::*;

    fn count_solids(draw: &ui_wgpu::wgpu::DrawList) -> usize {
        draw.layers.iter().map(|layer| layer.ui_instances.len()).sum()
    }

    fn tiny_png_data_url(r: u8, g: u8, b: u8) -> String {
        let img = image::RgbaImage::from_pixel(1, 1, image::Rgba([r, g, b, 255]));
        let mut bytes: Vec<u8> = Vec::new();
        image::DynamicImage::ImageRgba8(img).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode tiny test png");
        format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(&bytes))
    }

    #[test]
    fn queue_canvas_image_upload_skips_decode_when_source_unchanged() {
        let surface_id = "raster-frame-cost-test-unchanged";
        let data_url = tiny_png_data_url(10, 20, 30);
        let first = queue_canvas_image_upload(surface_id, "layer-a", &data_url);
        assert!(first.is_some());
        mutate_scene_state(surface_id, |state| {
            state.pending_raster_uploads.clear();
        });
        let second = queue_canvas_image_upload(surface_id, "layer-a", &data_url);
        assert_eq!(first, second, "key must stay stable across frames");
        let pending = scene_state(surface_id).pending_raster_uploads.len();
        assert_eq!(pending, 0, "unchanged data_url must not re-decode/re-queue an upload");
    }

    #[test]
    fn queue_canvas_image_upload_redecodes_when_source_changes() {
        let surface_id = "raster-frame-cost-test-changed";
        let png_a = tiny_png_data_url(10, 20, 30);
        let png_b = tiny_png_data_url(200, 100, 50);
        queue_canvas_image_upload(surface_id, "layer-a", &png_a);
        mutate_scene_state(surface_id, |state| state.pending_raster_uploads.clear());
        queue_canvas_image_upload(surface_id, "layer-a", &png_b);
        let pending = scene_state(surface_id).pending_raster_uploads.len();
        assert_eq!(pending, 1, "changed data_url must re-decode and queue exactly one upload");
    }

    #[test]
    fn draw_checkerboard_clamps_to_visible_viewport() {
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let theme = Theme::default();
        draw_checkerboard(&mut draw, &viewport, inner, &theme, 4096.0);
        let quads = count_solids(&draw);
        assert!(quads > 0, "checkerboard should still draw the visible cells");
        assert!(quads < 4000, "checkerboard must clamp to the viewport instead of the full ±extent/2 grid, got {quads}");
    }

    #[test]
    fn draw_checkerboard_falls_back_to_full_extent_when_zoom_is_zero() {
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 0.0 };
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let theme = Theme::default();
        draw_checkerboard(&mut draw, &viewport, inner, &theme, 64.0);
        let quads = count_solids(&draw);
        assert_eq!(quads, 16, "degenerate zoom must fall back to the full extent grid (4x4 cells for a 64-unit extent)");
    }

    //#region Canvas2dDrawRecordTests
    #[test]
    fn canvas_layer_should_render_filters_meta_role_and_invisible_records() {
        let meta: CanvasLayer = serde_json::from_str(r#"{"role":"meta","utility":"selectDirect"}"#).unwrap();
        assert!(!canvas_layer_should_render(&meta), "role: meta records are non-visual bookkeeping");
        let hidden: CanvasLayer = serde_json::from_str(r#"{"kind":"rect","visible":false}"#).unwrap();
        assert!(!canvas_layer_should_render(&hidden));
        let visible: CanvasLayer = serde_json::from_str(r#"{"kind":"rect"}"#).unwrap();
        assert!(canvas_layer_should_render(&visible), "records default to visible when the field is absent");
    }

    #[test]
    fn canvas_gradient_color_at_interpolates_between_bracketing_stops() {
        let stops = vec![CanvasGradientStopJson { offset: 0.0, color: Some(vec![0.0, 0.0, 0.0, 1.0]) }, CanvasGradientStopJson { offset: 1.0, color: Some(vec![1.0, 1.0, 1.0, 1.0]) }];
        let start = canvas_gradient_color_at(&stops, 0.0, 1.0);
        let mid = canvas_gradient_color_at(&stops, 0.5, 1.0);
        let end = canvas_gradient_color_at(&stops, 1.0, 1.0);
        assert!(start.r < 0.001, "t=0 should sample the first stop");
        assert!((mid.r - 0.5).abs() < 0.001, "t=0.5 should sit halfway between stops");
        assert!(end.r > 0.999, "t=1 should sample the last stop");
    }

    #[test]
    fn canvas_apply_blend_mode_normal_and_none_are_passthrough() {
        let src = Rgba::new(0.2, 0.4, 0.6, 0.8);
        let backdrop = Rgba::new(0.9, 0.9, 0.9, 1.0);
        assert_eq!(canvas_apply_blend_mode(None, backdrop, src).r, src.r);
        assert_eq!(canvas_apply_blend_mode(Some("normal"), backdrop, src).b, src.b);
    }

    #[test]
    fn canvas_apply_blend_mode_multiply_matches_w3c_formula() {
        let src = Rgba::new(0.5, 0.5, 0.5, 1.0);
        let backdrop = Rgba::new(0.4, 0.8, 1.0, 1.0);
        let blended = canvas_apply_blend_mode(Some("multiply"), backdrop, src);
        assert!((blended.r - 0.2).abs() < 0.001, "multiply(0.4,0.5) should be 0.2, got {}", blended.r);
        assert!((blended.g - 0.4).abs() < 0.001, "multiply(0.8,0.5) should be 0.4, got {}", blended.g);
        assert!((blended.b - 0.5).abs() < 0.001, "multiply(1.0,0.5) should be 0.5, got {}", blended.b);
    }

    #[test]
    fn canvas_apply_blend_mode_screen_matches_w3c_formula() {
        let src = Rgba::new(0.5, 0.2, 0.0, 1.0);
        let backdrop = Rgba::new(0.5, 0.5, 1.0, 1.0);
        let blended = canvas_apply_blend_mode(Some("screen"), backdrop, src);
        let expected_r = 0.5 + 0.5 - 0.5 * 0.5;
        assert!((blended.r - expected_r).abs() < 0.001, "screen(a,b)=a+b-ab, got {}", blended.r);
    }

    #[test]
    fn push_linear_gradient_fill_emits_banded_triangle_fan_geometry() {
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let clip = Rect::new(50.0, 50.0, 100.0, 40.0);
        let fill = CanvasFillJson {
            kind: Some("linearGradient".into()),
            x1: 0.0,
            y1: 0.0,
            x2: 100.0,
            y2: 0.0,
            stops: vec![CanvasGradientStopJson { offset: 0.0, color: Some(vec![1.0, 0.0, 0.0, 1.0]) }, CanvasGradientStopJson { offset: 1.0, color: Some(vec![0.0, 0.0, 1.0, 1.0]) }],
            ..Default::default()
        };
        push_linear_gradient_fill(&mut draw, &viewport, inner, clip, 0.0, 0.0, &fill, 1.0, None, Theme::default().canvas_clear);
        let verts: usize = draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum();
        assert!(verts > 0, "linear gradient bands should push triangle-fan geometry");
    }

    #[test]
    fn push_radial_gradient_fill_emits_concentric_ring_geometry() {
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let clip = Rect::new(0.0, 0.0, 80.0, 80.0);
        let fill = CanvasFillJson {
            kind: Some("radialGradient".into()),
            cx: 40.0,
            cy: 40.0,
            r: 40.0,
            stops: vec![CanvasGradientStopJson { offset: 0.0, color: Some(vec![1.0, 1.0, 1.0, 1.0]) }, CanvasGradientStopJson { offset: 1.0, color: Some(vec![0.0, 0.0, 0.0, 1.0]) }],
            ..Default::default()
        };
        push_radial_gradient_fill(&mut draw, &viewport, inner, clip, 0.0, 0.0, &fill, 1.0, None, Theme::default().canvas_clear);
        let verts: usize = draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum();
        assert!(verts > 0, "radial gradient rings should push triangle-fan geometry");
    }

    #[test]
    fn render_canvas_shape_fill_draws_solid_fill_and_stroke_for_plain_records() {
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let shape_rect = Rect::new(10.0, 10.0, 40.0, 20.0);
        let layer: CanvasLayer = serde_json::from_str(r#"{"kind":"rect","fill":{"color":[0.1,0.2,0.3,1.0]},"stroke":{"color":[1.0,1.0,1.0,1.0],"width":2.0}}"#).unwrap();
        render_canvas_shape_fill(&mut draw, &viewport, inner, shape_rect, &layer, 1.0, Rgba::new(0.0, 0.0, 0.0, 1.0), Theme::default().canvas_clear, false);
        let solids = count_solids(&draw);
        let verts: usize = draw.layers.iter().map(|l| l.vector_vertices.len()).sum();
        assert!(solids > 0, "solid fill should push a rounded-rect instance");
        assert!(verts > 0, "stroke should push line geometry");
    }
    //#endregion Canvas2dDrawRecordTests

    //#region Paint2dNavigatorTests
    #[test]
    fn paint2d_navigator_fit_viewport_centers_and_scales_to_content_bounds() {
        let flat = vec![Paint2dFlatLayer { id: "a".into(), image_key: Some("k".into()), x: 100.0, y: 50.0, scale_x: 1.0, scale_y: 1.0, opacity: 1.0, width: 200, height: 100 }];
        let inner = Rect::new(0.0, 0.0, 100.0, 100.0);
        let viewport = paint2d_navigator_fit_viewport(&flat, inner);
        assert!((viewport.x - 100.0).abs() < 0.01, "camera should center on content x, got {}", viewport.x);
        assert!((viewport.y - 50.0).abs() < 0.01, "camera should center on content y, got {}", viewport.y);
        assert!((viewport.zoom - 0.26).abs() < 0.01, "zoom should fit the padded viewport to content, got {}", viewport.zoom);
    }

    #[test]
    fn paint2d_navigator_fit_viewport_falls_back_to_neutral_camera_when_document_is_empty() {
        let viewport = paint2d_navigator_fit_viewport(&[], Rect::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
        assert_eq!(viewport.zoom, 1.0);
    }

    #[test]
    fn paint2d_navigator_overlay_rect_maps_main_viewport_into_navigator_screen_space() {
        let content_camera_json = r#"{"x":0,"y":0,"zoom":1}"#;
        let content_viewport_json = r#"{"width":400,"height":300}"#;
        let navigator_viewport = Viewport { x: 0.0, y: 0.0, zoom: 0.5 };
        let navigator_inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let overlay = paint2d_navigator_overlay_rect(content_camera_json, Some(content_viewport_json), &navigator_viewport, navigator_inner).expect("overlay rect should resolve when a composite viewport size is present");
        assert!((overlay.w - 200.0).abs() < 0.5, "overlay width should track the main viewport's world width, got {}", overlay.w);
        assert!((overlay.h - 150.0).abs() < 0.5, "overlay height should track the main viewport's world height, got {}", overlay.h);
    }

    #[test]
    fn paint2d_navigator_overlay_rect_is_none_without_a_reported_composite_viewport() {
        let navigator_viewport = Viewport { x: 0.0, y: 0.0, zoom: 1.0 };
        let navigator_inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        assert!(paint2d_navigator_overlay_rect("{}", None, &navigator_viewport, navigator_inner).is_none());
    }
    //#endregion Paint2dNavigatorTests
}
//#endregion RasterFrameCostTests

//#region InkCanvasTests
#[cfg(test)]
mod ink_canvas_tests {
    use super::*;
    use ui_wgpu::wgpu::UiPresence;

    fn sample_block(id: &str, x: f64, y: f64, w: f64, h: f64) -> Value {
        json!({
            "id": id, "name": "Text", "kind": "text", "x": x, "y": y, "width": w, "height": h,
            "rotation": 0.0, "visible": true, "locked": false,
            "paragraphs": [], "fontSize": 18.0, "fontWeight": "normal", "align": "left",
        })
    }

    #[test]
    fn hit_test_prefers_topmost_block() {
        let blocks = vec![sample_block("a", 0.0, 0.0, 100.0, 100.0), sample_block("b", 20.0, 20.0, 100.0, 100.0)];
        let overrides = HashMap::new();
        let hits = ink_items_at_point(&blocks, &overrides, 50.0, 50.0);
        assert_eq!(ink_item_id(hits[0]), "b");
    }

    #[test]
    fn hit_test_misses_outside_bounds() {
        let blocks = vec![sample_block("a", 0.0, 0.0, 10.0, 10.0)];
        let overrides = HashMap::new();
        assert!(ink_items_at_point(&blocks, &overrides, 50.0, 50.0).is_empty());
    }

    #[test]
    fn resize_bounds_east_handle_grows_width_only() {
        let from = InkBoundsF { x: 0.0, y: 0.0, w: 100.0, h: 50.0 };
        let to = ink_resize_bounds(from, "e", 20.0, 0.0, 8.0);
        assert_eq!(to, InkBoundsF { x: 0.0, y: 0.0, w: 120.0, h: 50.0 });
    }

    #[test]
    fn resize_bounds_northwest_handle_moves_origin() {
        let from = InkBoundsF { x: 10.0, y: 10.0, w: 100.0, h: 100.0 };
        let to = ink_resize_bounds(from, "nw", -10.0, -10.0, 8.0);
        assert_eq!(to, InkBoundsF { x: 0.0, y: 0.0, w: 110.0, h: 110.0 });
    }

    #[test]
    fn resize_bounds_respects_minimum_size() {
        let from = InkBoundsF { x: 0.0, y: 0.0, w: 20.0, h: 20.0 };
        let to = ink_resize_bounds(from, "e", -100.0, 0.0, 8.0);
        assert_eq!(to.w, 8.0);
    }

    #[test]
    fn screen_world_roundtrip() {
        let camera = InkCameraF { x: 12.0, y: -8.0, zoom: 1.5 };
        let inner = Rect::new(100.0, 40.0, 400.0, 300.0);
        let (wx, wy) = ink_screen_to_world(camera, inner, 250.0, 150.0);
        let (sx, sy) = ink_world_to_screen(camera, inner, wx, wy);
        assert!((sx - 250.0).abs() < 0.01);
        assert!((sy - 150.0).abs() < 0.01);
    }

    #[test]
    fn snap_rounds_to_nearest_grid_cell() {
        assert_eq!(ink_snap_coordinate(13.0, 8.0), 16.0);
        assert_eq!(ink_snap_coordinate(3.0, 8.0), 0.0);
    }

    #[test]
    fn ink_block_bounds_from_points() {
        let block = json!({
            "id": "i1", "kind": "stroke", "x": 10.0, "y": 10.0, "width": 1.0, "height": 1.0,
            "points": [[0.0, 0.0], [5.0, 10.0], [-5.0, 2.0]], "strokeWidth": 3.0, "color": [0, 0, 0, 1],
        });
        let bounds = ink_item_bounds(&block);
        assert_eq!(bounds.x, 5.0);
        assert_eq!(bounds.y, 10.0);
        assert_eq!(bounds.w, 10.0);
        assert_eq!(bounds.h, 10.0);
    }

    //#region InkCanvasPaintTests
    #[test]
    fn item_card_background_uses_the_background_token_not_panel() {
        let block = json!({
            "id": "i1", "kind": "text", "x": 0.0, "y": 0.0, "width": 100.0, "height": 40.0,
            "paragraphs": [], "fontSize": 16.0,
        });
        let doc: InkDocumentJson = serde_json::from_str("{}").unwrap();
        let scene = UiComponentSceneNode {
            surface_id: "ink-paint-test".into(),
            controller_id: "controller".into(),
            component_kind: SurfaceKind::InkCanvas,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: None,
            menu: None,
        };
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            let camera = InkCameraF { x: 0.0, y: 0.0, zoom: 1.0 };
            let inner = Rect::new(0.0, 0.0, 400.0, 300.0);
            draw_ink_item(&mut ctx, &scene, &block, camera, inner, &doc, false, false);
        }
        // 🎨️ `bg-background/90` in `ink-canvas-host.tsx` — the card's fill must resolve to
        // `theme.background`, not `theme.panel` (the app-chrome surface token).
        let expected = theme.background.with_alpha(0.9);
        let colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
        assert!(colors.contains(&[expected.r, expected.g, expected.b, expected.a]), "expected an item card fill at theme.background@0.9, got {colors:?}");
        let stale = theme.panel.with_alpha(0.92);
        assert!(!colors.contains(&[stale.r, stale.g, stale.b, stale.a]), "the item card must no longer fill with the stale theme.panel@0.92 token");
    }
    //#endregion InkCanvasPaintTests
}
//#endregion InkCanvasTests
//#endregion InkCanvas

//#region NodeGraph
#[derive(Clone, Debug)]
pub struct NodeGraphSurface {
    pub bounds: Rect,
    pub controller_id: String,
}

/** @emoji 🕸️ Applies node-hit context to a scene context-menu action. */
pub fn resolve_graph_context_action(action: &ActionDescriptor, node_id: Option<&str>) -> ActionDescriptor {
    let Some(node_id) = node_id else {
        return action.clone();
    };
    let mut resolved = action.clone();
    match action.action.as_str() {
        "setMediaNodeSelection" => {
            resolved.args = crate::action_args_json!({ "nodeIds": [node_id] });
        }
        "removeAppInstance" => {
            if let Some(instance_id) = graph_node_instance(node_id) {
                resolved.args = crate::action_args_json!({ "instanceId": instance_id });
            }
        }
        "selectNode" => {
            resolved.args = crate::action_args_json!({ "nodeId": node_id });
        }
        _ => {}
    }
    resolved
}

fn find_graph_node(scene: &UiComponentSceneNode, node_id: &str) -> Option<ui_wgpu::wgpu::NodeGraphNodeRecord> {
    scene.node_graph.as_ref().and_then(|graph| graph.nodes.iter().find(|n| n.id == node_id).cloned())
}

fn hit_graph_node(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> Option<String> {
    let graph = scene.node_graph.as_ref()?;
    let state = scene_state(&scene.surface_id);
    let viewport = if state.viewport.zoom > 0.0 { state.viewport } else { Viewport::from_typed(graph.viewport.as_ref()) };
    for node in graph.nodes.iter().rev() {
        let (nx, ny) = state.node_positions.get(&node.id).copied().unwrap_or((node.x as f32, node.y as f32));
        let (sx, sy) = viewport.world_to_screen(nx, ny, inner);
        let w = node.width as f32 * viewport.zoom;
        let h = node.height as f32 * viewport.zoom;
        let rect = Rect::new(sx, sy, w, h);
        if rect.contains(x, y) {
            return Some(node.id.clone());
        }
    }
    None
}

fn push_bezier(ctx: &mut FrameworkWidgetContext<'_>, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
    let cx0 = x0 + (x1 - x0) * 0.5;
    let cy0 = y0;
    let cx1 = x0 + (x1 - x0) * 0.5;
    let cy1 = y1;
    let segments = 16usize;
    let mut last = (x0, y0);
    for step in 1..=segments {
        let t = step as f32 / segments as f32;
        let u = 1.0 - t;
        let px = u * u * u * x0 + 3.0 * u * u * t * cx0 + 3.0 * u * t * t * cx1 + t * t * t * x1;
        let py = u * u * u * y0 + 3.0 * u * u * t * cy0 + 3.0 * u * t * t * cy1 + t * t * t * y1;
        ctx.draw.push_line(last.0, last.1, px, py, color, width);
        last = (px, py);
    }
}

fn render_node_graph(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::wgpu::GpuContext, node_graph_states: &mut HashMap<String, NodeGraphSurface>) {
    let Some(graph) = &scene.node_graph else {
        return render_placeholder("node-graph", bounds, ctx);
    };
    for node in &graph.nodes {
        register_graph_node(&node.id, node.instance_id.as_deref());
        let label = node.label.as_deref().or(node.instance_id.as_deref()).unwrap_or(&node.id);
        push_find_item(ShellFindItem { id: node.id.clone(), label: label.to_string(), description: node.instance_id.clone(), category: Some("Nodes".into()), surface_id: scene.surface_id.clone(), node_id: node.id.clone() });
    }
    let inner = bounds;
    node_graph_states.insert(scene.surface_id.clone(), NodeGraphSurface { bounds: inner, controller_id: scene.controller_id.clone() });
    engine_canvas::paint_node_graph(gpu, ctx, scene, inner);
    engine_canvas::paint_node_graph_labels(ctx, scene, inner);
    engine_canvas::paint_node_graph_overlays(ctx, scene, inner);
}

fn node_screen_pos(node: &ui_wgpu::wgpu::NodeGraphNodeRecord, state: &SceneSurfaceState, viewport: &Viewport, inner: Rect) -> (f32, f32) {
    let (nx, ny) = state.node_positions.get(&node.id).copied().unwrap_or((node.x as f32, node.y as f32));
    viewport.world_to_screen(nx, ny, inner)
}
//#endregion NodeGraph

//#region TiledMap
#[derive(Clone, Debug)]
pub struct TiledMapSurface {
    pub bounds: Rect,
    pub controller_id: String,
    pub selection_method: String,
}

fn query_map_feature_hits(host: &framework_surface_tiled_map::tiled_map::MapHost, method: &str, points: &[(f32, f32)], crossing: bool) -> (Vec<String>, Vec<String>) {
    if method == "lasso" && points.len() >= 3 {
        let payload: Vec<[f64; 2]> = points.iter().map(|(x, y)| [*x as f64, *y as f64]).collect();
        let points_json = serde_json::to_string(&payload).unwrap_or_else(|_| "[]".into());
        engine_canvas::parse_map_feature_hit(&host.features_in_polygon_json(&points_json, crossing))
    } else if points.len() >= 2 {
        let (x0, y0) = points[0];
        let (x1, y1) = points[points.len() - 1];
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        engine_canvas::parse_map_feature_hit(&host.features_in_rect_json(min_x as f64, min_y as f64, max_x as f64, max_y as f64, crossing))
    } else {
        (Vec::new(), Vec::new())
    }
}

fn paint_tiled_map_marquee(ctx: &mut FrameworkWidgetContext<'_>, surface_id: &str, inner: Rect, theme: &Theme) {
    let state = scene_state(surface_id);
    if !state.map_marquee_active {
        return;
    }
    let points = state.map_marquee_points;
    if points.len() < 2 {
        return;
    }
    let method = match &state.drag {
        Some(SceneDrag { mode: SceneDragMode::MapMarquee { method, .. }, .. }) => method.as_str(),
        _ => "rectangle",
    };
    let lasso = method == "lasso" && points.len() >= 3;
    let global: Vec<[f32; 2]> = points.iter().map(|(x, y)| [inner.x + x, inner.y + y]).collect();
    let crossing = ui_wgpu::wgpu::marquee_is_crossing_from_path(&global, lasso);
    ui_wgpu::wgpu::paint_selection_marquee(&mut ctx.draw, theme, crossing, lasso, &global, false);
}

/** @emoji 🗺️ Pushes GIS map context-menu items for a screen-space hit. */

pub fn tiled_map_pointer_down(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool, selection_method: &str) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if button == 0 {
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag { mode: SceneDragMode::MapMarquee { start_x: sx as f32, start_y: sy as f32, method: selection_method.to_string(), merge_mode: engine_canvas::map_marquee_mode(shift, ctrl_or_meta).to_string() }, button });
            state.map_marquee_points = vec![(sx as f32, sy as f32)];
            state.map_marquee_active = false;
        });
        return Vec::new();
    }
    if button == 1 {
        engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_down_screen(sx, sy, 1));
        mutate_scene_state(surface_id, |state| {
            state.drag = Some(SceneDrag { mode: SceneDragMode::MapPan, button: 1 });
        });
        return engine_canvas::with_map_host_mut(surface_id, |host| engine_canvas::map_interaction_actions(surface_id, controller_id, host)).unwrap_or_default();
    }
    let _ = controller_id;
    Vec::new()
}

pub fn tiled_map_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, down: bool) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    if down {
        let state = scene_state(surface_id);
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::MapPan => {
                    engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_move_screen(sx, sy));
                    return engine_canvas::with_map_host_mut(surface_id, |host| engine_canvas::map_interaction_actions(surface_id, controller_id, host)).unwrap_or_default();
                }
                SceneDragMode::MapMarquee { start_x, start_y, method, .. } => {
                    let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(surface_id, |state| {
                        if distance >= MAP_MARQUEE_THRESHOLD_PX {
                            state.map_marquee_active = true;
                        }
                        if state.map_marquee_active {
                            if method == "lasso" {
                                if state.map_marquee_points.last().copied() != Some((sx as f32, sy as f32)) {
                                    state.map_marquee_points.push((sx as f32, sy as f32));
                                }
                            } else {
                                state.map_marquee_points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                            }
                        }
                    });
                }
                _ => {}
            }
        }
        return Vec::new();
    }
    let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
    let hover = engine_canvas::parse_map_hover(&hit_json);
    let hover_json = if hover.is_null() { "null".into() } else { hover.to_string() };
    let prior = scene_state(surface_id).map_last_hover_json;
    if prior.as_deref() == Some(hover_json.as_str()) {
        return Vec::new();
    }
    mutate_scene_state(surface_id, |state| {
        state.map_last_hover_json = Some(hover_json.clone());
    });
    vec![engine_canvas::map_action(controller_id, ui_wgpu::wgpu::tiled_map_actions::SET_HOVER, json!({ "surfaceId": surface_id, "hover": hover }))]
}

pub fn tiled_map_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32) -> Vec<ActionDescriptor> {
    let (sx, sy) = engine_canvas::map_local_pointer(inner, x, y);
    let state = scene_state(surface_id);
    let Some(drag) = state.drag.clone() else {
        return Vec::new();
    };
    let mut actions = Vec::new();
    match drag.mode {
        SceneDragMode::MapPan => {
            engine_canvas::with_map_host_mut(surface_id, |host| host.pointer_up_screen(sx, sy));
            actions.extend(engine_canvas::with_map_host_mut(surface_id, |host| engine_canvas::map_interaction_actions(surface_id, controller_id, host)).unwrap_or_default());
        }
        SceneDragMode::MapMarquee { start_x, start_y, method, merge_mode } => {
            let distance = ((sx as f32 - start_x).powi(2) + (sy as f32 - start_y).powi(2)).sqrt();
            if state.map_marquee_active && distance >= MAP_MARQUEE_THRESHOLD_PX {
                let mut points = state.map_marquee_points.clone();
                if method == "lasso" {
                    points.push((sx as f32, sy as f32));
                } else {
                    points = vec![(start_x, start_y), (sx as f32, sy as f32)];
                }
                let crossing = engine_canvas::map_marquee_crossing(&method, start_x, sx as f32);
                let (positions, routes) = engine_canvas::with_map_host(surface_id, |host| query_map_feature_hits(host, &method, &points, crossing)).unwrap_or_default();
                actions.push(engine_canvas::map_action(
                    controller_id,
                    ui_wgpu::wgpu::tiled_map_actions::SET_FEATURE_SELECTION,
                    json!({
                        "surfaceId": surface_id,
                        "positions": positions,
                        "routes": routes,
                        "mode": merge_mode,
                    }),
                ));
            } else if distance < MAP_MARQUEE_THRESHOLD_PX {
                let hit_json = engine_canvas::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
                let hit: Value = serde_json::from_str(&hit_json).unwrap_or(Value::Null);
                let (kind, id) = (hit.get("kind").and_then(|value| value.as_str()), hit.get("id").and_then(|value| value.as_str()));
                if let (Some(kind), Some(id)) = (kind, id) {
                    actions.push(engine_canvas::map_action(
                        controller_id,
                        ui_wgpu::wgpu::tiled_map_actions::SET_FEATURE_SELECTION,
                        json!({
                            "surfaceId": surface_id,
                            "positions": if kind == "position" { vec![id] } else { Vec::<&str>::new() },
                            "routes": if kind == "route" { vec![id] } else { Vec::<&str>::new() },
                            "mode": merge_mode,
                        }),
                    ));
                }
            }
        }
        _ => {}
    }
    mutate_scene_state(surface_id, |state| {
        state.drag = None;
        state.map_marquee_points.clear();
        state.map_marquee_active = false;
    });
    actions
}

pub fn tiled_map_drag_active(surface_id: &str) -> bool {
    scene_state(surface_id).drag.as_ref().is_some_and(|drag| matches!(drag.mode, SceneDragMode::MapMarquee { .. } | SceneDragMode::MapPan))
}

fn render_tiled_map(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::wgpu::GpuContext, tiled_map_states: &mut HashMap<String, TiledMapSurface>) {
    let Some(map_scene) = &scene.tiled_map else {
        return render_placeholder("tiled-map", bounds, ctx);
    };
    let inner = bounds;
    tiled_map_states.insert(scene.surface_id.clone(), TiledMapSurface { bounds: inner, controller_id: scene.controller_id.clone(), selection_method: map_scene.selection_method.clone() });
    engine_canvas::paint_tiled_map(gpu, ctx, scene, inner);
    paint_tiled_map_marquee(ctx, &scene.surface_id, inner, ctx.theme);
}
//#endregion TiledMap

//#region IconRender
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderCameraFields {
    position: [f64; 3],
    target: [f64; 3],
    #[serde(default = "icon_render_default_zoom")]
    zoom: f64,
    #[serde(default)]
    fov: Option<f64>,
    #[serde(default)]
    up: Option<[f64; 3]>,
}

fn icon_render_default_zoom() -> f64 {
    1.0
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct IconRenderLightsFields {
    #[serde(default)]
    ambient_intensity: f64,
    #[serde(default)]
    ambient_color: Option<String>,
    #[serde(default)]
    sun_azimuth: f64,
    #[serde(default)]
    sun_elevation: f64,
    #[serde(default)]
    sun_intensity: f64,
    #[serde(default)]
    sun_color: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderMaterialFields {
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    metalness: Option<f64>,
    #[serde(default)]
    roughness: Option<f64>,
    #[serde(default)]
    emissive: Option<String>,
    #[serde(default)]
    emissive_intensity: Option<f64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IconRenderRequestFields {
    asset_url: String,
    camera: IconRenderCameraFields,
    #[serde(default)]
    lights: Option<IconRenderLightsFields>,
    width: f64,
    height: f64,
    #[serde(default)]
    shape: Option<String>,
    #[serde(default)]
    background: Option<String>,
    #[serde(default)]
    shadow_enabled: Option<bool>,
    #[serde(default)]
    material: Option<IconRenderMaterialFields>,
}

/** @emoji 🎥️ Folds the request's three.js `zoom` into an equivalent vertical FOV, since the native orbit camera has no independent zoom factor, see https://threejs.org/docs/#api/en/cameras/PerspectiveCamera.zoom. */
fn icon_render_camera_json(camera: &IconRenderCameraFields) -> String {
    let fov = camera.fov.unwrap_or(50.0).max(1.0);
    let zoom = if camera.zoom.abs() > 1e-6 { camera.zoom } else { 1.0 };
    let effective_fov = if (zoom - 1.0).abs() > 1e-6 {
        let half = (fov * 0.5).to_radians();
        (2.0 * (half.tan() / zoom).atan()).to_degrees()
    } else {
        fov
    };
    let up = camera.up.unwrap_or([0.0, 0.0, 1.0]);
    json!({
        "position": camera.position,
        "target": camera.target,
        "up": up,
        "fov": effective_fov,
    })
    .to_string()
}

fn icon_render_environment_json(request: &IconRenderRequestFields) -> String {
    let lights = request.lights.clone().unwrap_or_default();
    let mut value = json!({
        "ambient": { "intensity": lights.ambient_intensity, "color": lights.ambient_color },
        "sun": {
            "azimuth": lights.sun_azimuth,
            "elevation": lights.sun_elevation,
            "intensity": lights.sun_intensity,
            "color": lights.sun_color,
        },
        "shadow": { "enabled": request.shadow_enabled.unwrap_or(false) },
    });
    if let Some(object) = value.as_object_mut() {
        if let Some(material) = &request.material {
            object.insert(
                "material".into(),
                json!({
                    "color": material.color,
                    "metalness": material.metalness,
                    "roughness": material.roughness,
                    "emissive": material.emissive,
                    "emissiveIntensity": material.emissive_intensity,
                }),
            );
        }
        if let Some(background) = &request.background {
            object.insert("background".into(), json!(background));
        }
    }
    value.to_string()
}

fn render_icon_render_empty(bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, message: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_body;
    let width = ctx.atlas.measure_text(message, size).0;
    draw_text(ctx, message, bounds.x + (bounds.w - width) * 0.5, bounds.y + bounds.h * 0.5, size, theme.text_muted);
}

/** @emoji 🖼️ Native counterpart of framework/renderer/react/components/icon-render-host.tsx: reframes the request into a synthetic World3dScene and delegates the actual GLB draw to infinite_world::render_world_3d, then paints the aspect-fit frame/badge/footer chrome on top. */
fn render_icon_render(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::wgpu::GpuContext, icon_render_states: &mut HashMap<String, World3dState>) {
    let Some(icon_render) = &scene.icon_render else {
        return render_icon_render_empty(bounds, ctx, "No shot");
    };
    let Ok(request) = serde_json::from_str::<IconRenderRequestFields>(&icon_render.request_json) else {
        return render_icon_render_empty(bounds, ctx, "No shot");
    };

    let shape = request.shape.clone().unwrap_or_else(|| "rectangle".into());
    let width = request.width.max(1.0) as f32;
    let height = request.height.max(1.0) as f32;
    let fit_scale = (bounds.w / width).min(bounds.h / height).max(0.01);
    let frame_w = width * fit_scale;
    let frame_h = height * fit_scale;
    let frame = Rect::new(bounds.x + (bounds.w - frame_w) * 0.5, bounds.y + (bounds.h - frame_h) * 0.5, frame_w, frame_h);

    let mesh_id = semio_framework_plugin::world3d_mesh_id_from_url(&request.asset_url);
    let instances_json = json!([{
        "id": "icon-render-subject",
        "meshId": mesh_id,
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
    }])
    .to_string();
    let mut synthetic_world = semio_framework_plugin::world3d_scene(
        icon_render_camera_json(&request.camera),
        semio_framework_plugin::world3d_meshes_json_from_urls(std::slice::from_ref(&request.asset_url)),
        instances_json,
        ui_wgpu::wgpu::world3d_default_selection_json(),
        &semio_framework_plugin::WorldSunConfig::default(),
    );
    synthetic_world.environment_json = Some(icon_render_environment_json(&request));

    let synthetic_scene = UiComponentSceneNode {
        presence: UiPresence::default(),
        surface_id: scene.surface_id.clone(),
        controller_id: scene.controller_id.clone(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(synthetic_world),
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        diff_view: None,
        event_feed: None,
        block_list: None,
        menu: None,
    };

    let state = icon_render_states.entry(scene.surface_id.clone()).or_insert_with(|| World3dState::new(scene.surface_id.clone(), scene.controller_id.clone()));
    render_world_3d(&synthetic_scene, frame, ctx, state, gpu);

    paint_icon_render_chrome(ctx, bounds, frame, &request, &shape, icon_render.footer.as_deref());
}

/// 🖼️ The aspect-fit frame border, size/shape badge, and optional footer caption painted on top of
/// the delegated `render_world_3d` GLB draw — split out from `render_icon_render` (which needs a
/// live `GpuContext` and so can't run in a headless unit test) so this chrome-only paint can be
/// exercised directly against a `DrawList`.
fn paint_icon_render_chrome(ctx: &mut FrameworkWidgetContext<'_>, bounds: Rect, frame: Rect, request: &IconRenderRequestFields, shape: &str, footer: Option<&str>) {
    let theme = ctx.theme;
    // 🖼️ 2px, matching `IconShotFrame`'s `border-2 border-accent` in `icon-render-host.tsx` —
    // `theme.stroke_hairline` (1px) previously halved the frame width relative to React.
    let hair = 2.0_f32;
    ctx.draw.push_solid([frame.x, frame.y, frame.w, hair], theme.accent);
    ctx.draw.push_solid([frame.x, frame.y + frame.h - hair, frame.w, hair], theme.accent);
    ctx.draw.push_solid([frame.x, frame.y, hair, frame.h], theme.accent);
    ctx.draw.push_solid([frame.x + frame.w - hair, frame.y, hair, frame.h], theme.accent);

    let badge = format!("{}×{} · {}", request.width.round() as i64, request.height.round() as i64, shape);
    let badge_size = theme.font_size_small;
    let (badge_text_w, badge_text_h) = ctx.atlas.measure_text(&badge, badge_size);
    let pad = 4.0;
    let badge_w = badge_text_w + pad * 2.0;
    let badge_h = badge_text_h + pad * 2.0;
    let badge_x = frame.x + frame.w - badge_w - 4.0;
    let badge_y = frame.y + frame.h - badge_h - 4.0;
    // 🏷️ `bg-background/80`, not `panel` — the badge chip sits on the transparent canvas frame in
    // `icon-render-host.tsx`, not on a panel surface.
    ctx.draw.push_rounded([badge_x, badge_y, badge_w, badge_h], theme.background.with_alpha(0.8), 2.0);
    draw_text(ctx, &badge, badge_x + pad, badge_y + pad + badge_text_h * 0.8, badge_size, theme.text_muted);

    if let Some(footer) = footer {
        let footer_size = theme.font_size_small;
        let footer_w = ctx.atlas.measure_text(footer, footer_size).0;
        draw_text(ctx, footer, bounds.x + (bounds.w - footer_w) * 0.5, bounds.y + bounds.h - 8.0, footer_size, theme.text_muted);
    }
}
//#endregion IconRender

//#region IconRenderTests
#[cfg(test)]
mod icon_render_tests {
    use super::*;

    #[test]
    fn frame_border_is_two_px_and_badge_uses_background_token() {
        let request: IconRenderRequestFields = serde_json::from_str(r#"{"assetUrl":"mesh://x","camera":{"position":[0,0,5],"target":[0,0,0]},"width":64.0,"height":64.0,"shape":"rectangle"}"#).unwrap();
        let bounds = Rect::new(0.0, 0.0, 200.0, 200.0);
        let frame = Rect::new(20.0, 20.0, 160.0, 160.0);

        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        let mut scroll = HashMap::new();
        let mut collapsed = HashMap::new();
        let mut selects = HashMap::new();
        {
            let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
            paint_icon_render_chrome(&mut ctx, bounds, frame, &request, "rectangle", None);
        }
        // 🖼️ The top border strip is `[frame.x, frame.y, frame.w, hair]` — its rect's height (index 3)
        // must be exactly 2.0, matching React's `border-2`.
        let top_border = draw
            .layers
            .iter()
            .flat_map(|layer| layer.ui_instances.iter())
            .find(|instance| instance.rect[0] == frame.x && instance.rect[1] == frame.y && instance.rect[2] == frame.w)
            .unwrap_or_else(|| panic!("expected the top frame-border strip to be pushed"));
        assert_eq!(top_border.rect[3], 2.0, "the frame border must be 2px, matching border-2 in icon-render-host.tsx");

        let expected_badge_bg = theme.background.with_alpha(0.8);
        let stale_badge_bg = theme.panel.with_alpha(0.8);
        let colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
        assert!(colors.contains(&[expected_badge_bg.r, expected_badge_bg.g, expected_badge_bg.b, expected_badge_bg.a]), "expected the badge chip to use theme.background@0.8, got {colors:?}");
        assert!(!colors.contains(&[stale_badge_bg.r, stale_badge_bg.g, stale_badge_bg.b, stale_badge_bg.a]), "the badge chip must no longer use the stale theme.panel@0.8 token");
    }
}
//#endregion IconRenderTests

//#region Board2d
pub struct Board2dSurface {
    pub bounds: Rect,
    pub controller_id: String,
    pub fixture_json: String,
}

fn render_board2d(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::wgpu::GpuContext, board2d_states: &mut HashMap<String, Board2dSurface>) {
    let Some(board_scene) = &scene.board2d else {
        return render_placeholder("board-2d", bounds, ctx);
    };
    let inner = bounds;
    board2d_states.insert(scene.surface_id.clone(), Board2dSurface { bounds: inner, controller_id: scene.controller_id.clone(), fixture_json: board_scene.fixture_json.clone() });
    engine_canvas::paint_puzzle_board(gpu, ctx, scene, inner);
}

pub fn puzzle_board_pointer_down(surface_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool) {
    engine_canvas::puzzle_board_pointer_down(surface_id, inner, x, y, button, shift, ctrl_or_meta);
}

pub fn puzzle_board_pointer_move(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_move(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt)
}

pub fn puzzle_board_pointer_up(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, shift: bool, ctrl_or_meta: bool, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_up(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt)
}

pub fn puzzle_board_pointer_leave(surface_id: &str, controller_id: &str, alt: bool) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_pointer_leave(surface_id, controller_id, alt)
}

pub fn board2d_drag_active(surface_id: &str) -> bool {
    engine_canvas::board_drag_active(surface_id)
}

pub fn puzzle_board_wheel(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32) -> Vec<ActionDescriptor> {
    engine_canvas::puzzle_board_wheel(surface_id, controller_id, inner, x, y, delta)
}

//#region Puzzle2dSelectionMenu
pub struct Puzzle2dSelectionMenuItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub action: String,
    pub args: Option<Value>,
    pub disabled: bool,
    pub destructive: bool,
}

fn puzzle2d_entity_flag(entity: &Value, key: &str) -> bool {
    entity.get(key).and_then(Value::as_bool).unwrap_or(false)
}

/// @emoji 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors `buildPuzzle2dSelectionMenuItems` in the React host.
pub fn build_puzzle2d_selection_menu_items(fixture_json: &str, selection_ids: &[String]) -> Vec<Puzzle2dSelectionMenuItem> {
    let fixture: Value = serde_json::from_str(fixture_json).unwrap_or(Value::Null);
    if selection_ids.is_empty() {
        return vec![Puzzle2dSelectionMenuItem { id: "selectAll".into(), label: "Select all".into(), icon: "maximize-2".into(), action: "selectAll".into(), args: None, disabled: false, destructive: false }];
    }
    let selected: HashSet<&str> = selection_ids.iter().map(String::as_str).collect();
    let nodes = fixture.get("nodes").and_then(Value::as_array).cloned().unwrap_or_default();
    let edges = fixture.get("edges").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut selected_entities: Vec<Value> = Vec::new();
    let mut has_selected_node = false;
    for node in &nodes {
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(node.clone());
                has_selected_node = true;
            }
        }
        if let Some(handles) = node.get("handles").and_then(Value::as_array) {
            for handle in handles {
                if let Some(id) = handle.get("id").and_then(Value::as_str) {
                    if selected.contains(id) {
                        selected_entities.push(handle.clone());
                    }
                }
            }
        }
    }
    for edge in &edges {
        if let Some(id) = edge.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(edge.clone());
            }
        }
    }
    let any_visible = selected_entities.iter().any(|entity| !puzzle2d_entity_flag(entity, "hidden"));
    let any_unlocked = selected_entities.iter().any(|entity| !puzzle2d_entity_flag(entity, "locked"));
    vec![
        Puzzle2dSelectionMenuItem {
            id: "toggleHidden".into(),
            label: (if any_visible { "Hide" } else { "Show" }).into(),
            icon: (if any_visible { "eye-off" } else { "eye" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "hidden", "value": any_visible })),
            disabled: false,
            destructive: false,
        },
        Puzzle2dSelectionMenuItem {
            id: "toggleLocked".into(),
            label: (if any_unlocked { "Lock" } else { "Unlock" }).into(),
            icon: (if any_unlocked { "lock" } else { "lock-open" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "locked", "value": any_unlocked })),
            disabled: false,
            destructive: false,
        },
        Puzzle2dSelectionMenuItem { id: "duplicate".into(), label: "Duplicate".into(), icon: "copy".into(), action: "duplicateSelection".into(), args: None, disabled: !has_selected_node, destructive: false },
        Puzzle2dSelectionMenuItem { id: "selectSameKind".into(), label: "Select all of same kind".into(), icon: "layers".into(), action: "selectSameKind".into(), args: None, disabled: false, destructive: false },
        Puzzle2dSelectionMenuItem { id: "focusSelection".into(), label: "Zoom to selection".into(), icon: "crosshair".into(), action: "focusSelection".into(), args: None, disabled: false, destructive: false },
        Puzzle2dSelectionMenuItem { id: "deleteSelection".into(), label: "Delete".into(), icon: "trash".into(), action: "deleteSelection".into(), args: None, disabled: false, destructive: true },
    ]
}
//#endregion Puzzle2dSelectionMenu

/// @emoji 🧩️ Pushes board-2d context-menu items for a screen-space hit, eagerly selecting the clicked target if it isn't already selected (mirrors the React host's `onContextMenu`).

//#endregion Board2d

//#region VirtualFileSystem
#[derive(Deserialize)]
struct VfsDescriptorKind {
    #[serde(default)]
    presentation: String,
}

#[derive(Deserialize)]
struct VfsFileNodeKind {
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    descriptors: Vec<VfsDescriptorColumn>,
}

#[derive(Deserialize)]
struct VfsDescriptorColumn {
    id: String,
    #[serde(default)]
    label: String,
    #[serde(rename = "descriptorKindId", default)]
    descriptor_kind_id: String,
}

#[derive(Deserialize)]
struct VfsSchema {
    #[serde(rename = "descriptorColumnIds", default)]
    descriptor_column_ids: Vec<String>,
    #[serde(rename = "descriptorKinds", default)]
    descriptor_kinds: HashMap<String, VfsDescriptorKind>,
    #[serde(rename = "fileNodeKinds", default)]
    file_node_kinds: HashMap<String, VfsFileNodeKind>,
}

#[derive(Clone)]
struct VfsVisibleRow {
    row: Value,
    level: u32,
    has_children: bool,
    expanded: bool,
}

fn vfs_children_by_parent(rows: &[Value]) -> HashMap<String, Vec<Value>> {
    let mut map: HashMap<String, Vec<Value>> = HashMap::new();
    for row in rows {
        let parent = row.get("parentId").and_then(|v| v.as_str()).unwrap_or("").to_string();
        map.entry(parent).or_default().push(row.clone());
    }
    map
}

fn build_vfs_visible_rows(rows: &[Value], expanded_ids: &HashSet<String>) -> Vec<VfsVisibleRow> {
    let children_by_parent = vfs_children_by_parent(rows);
    let mut visible = Vec::new();
    fn visit(node: &Value, level: u32, out: &mut Vec<VfsVisibleRow>, children_by_parent: &HashMap<String, Vec<Value>>, expanded_ids: &HashSet<String>) {
        let id = node.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let has_children = node.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or_else(|| children_by_parent.get(&id).is_some_and(|c| !c.is_empty()));
        let expanded = has_children && expanded_ids.contains(&id);
        out.push(VfsVisibleRow { row: node.clone(), level, has_children, expanded });
        if !expanded {
            return;
        }
        if let Some(children) = children_by_parent.get(&id) {
            for child in children {
                visit(child, level + 1, out, children_by_parent, expanded_ids);
            }
        }
    }
    let roots: Vec<Value> = rows.iter().filter(|row| row.get("parentId").map(|v| v.is_null() || v.as_str() == Some("")).unwrap_or(true)).cloned().collect();
    for root in roots {
        if root.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or(false) {
            let root_id = root.get("id").and_then(|v| v.as_str()).unwrap_or("");
            if let Some(children) = children_by_parent.get(root_id) {
                for child in children {
                    visit(child, 0, &mut visible, &children_by_parent, expanded_ids);
                }
            }
        } else {
            visit(&root, 0, &mut visible, &children_by_parent, expanded_ids);
        }
    }
    visible
}

/// 🗂️ Resolves the row glyph, matching `VirtualFileSystemNodeGlyph`'s kind→icon lookup in
/// `index.tsx`. Previously a configured `fileNodeKinds[kindId].icon` only gated an `.is_some()`
/// check and the *actual* configured icon id was discarded in favor of a hardcoded `"folder"` —
/// any non-folder kind with its own icon (e.g. a custom "asset" kind) rendered the wrong glyph.
/// Extension-based file-type glyphs (React's ~40-entry `zip`→file-archive table) are not ported
/// here: the native icon atlas's available id set overlaps an in-flight `IconName` migration in
/// another session, so guessing unverified ids risks silently blank icons — left as a known gap.
fn vfs_glyph_icon<'a>(schema: &'a VfsSchema, row: &Value) -> &'a str {
    let kind_id = row.get("fileNodeKindId").and_then(|v| v.as_str()).unwrap_or("file");
    if let Some(icon) = schema.file_node_kinds.get(kind_id).and_then(|k| k.icon.as_deref()) {
        return icon;
    }
    match kind_id {
        "root" | "studio" | "folder" => "folder",
        "instance" => "box",
        _ => "file-text",
    }
}

fn vfs_descriptor_label(schema: &VfsSchema, column_id: &str) -> String {
    for kind in schema.file_node_kinds.values() {
        if let Some(col) = kind.descriptors.iter().find(|c| c.id == column_id) {
            if !col.label.is_empty() {
                return col.label.clone();
            }
        }
    }
    column_id.to_string()
}

fn vfs_descriptor_value(schema: &VfsSchema, row: &Value, column_id: &str) -> String {
    let raw = row
        .get("descriptorValues")
        .and_then(|values| values.get(column_id))
        .map(|v| match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .unwrap_or_default();
    let kind_id = schema.file_node_kinds.values().flat_map(|kind| kind.descriptors.iter()).find(|col| col.id == column_id).map(|col| col.descriptor_kind_id.as_str()).unwrap_or("text");
    let presentation = schema.descriptor_kinds.get(kind_id).map(|k| k.presentation.as_str()).unwrap_or("text");
    if presentation == "time" {
        if let Ok(ms) = raw.parse::<f64>() {
            let secs = (ms / 1000.0) as i64;
            let mins = secs / 60;
            let hours = mins / 60;
            return format!("{:02}:{:02}:{:02}", hours, mins % 60, secs % 60);
        }
    }
    raw
}

fn render_vfs(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(vfs) = &scene.virtual_file_system else {
        return render_placeholder("virtualFileSystem", bounds, ctx);
    };
    let schema: VfsSchema = serde_json::from_str(&vfs.schema_json).unwrap_or(VfsSchema { descriptor_column_ids: vec![], descriptor_kinds: HashMap::new(), file_node_kinds: HashMap::new() });
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
    let root_expand_ids: Vec<String> = rows.iter().filter(|row| row.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or(false)).filter_map(|row| row.get("id").and_then(|v| v.as_str()).map(str::to_string)).collect();
    seed_vfs_expanded(&scene.surface_id, &root_expand_ids);
    let selected: HashSet<String> = vfs.selected_row_ids_json.as_deref().and_then(|json| serde_json::from_str::<Vec<String>>(json).ok()).unwrap_or_default().into_iter().collect();
    let state = scene_state(&scene.surface_id);
    let expanded_ids = state.vfs_expanded_ids;
    let visible_rows = build_vfs_visible_rows(&rows, &expanded_ids);
    let inner = bounds;
    let header_h = theme.control_height * 1.33;
    let row_h = theme.control_height;
    let pad = theme.padding_standard;
    let name_col_w = inner.w * 0.32;
    let descriptor_ids: Vec<String> = if schema.descriptor_column_ids.is_empty() { vec![] } else { schema.descriptor_column_ids.clone() };
    let descriptor_col_w = if descriptor_ids.is_empty() { 0.0 } else { (inner.w - name_col_w) / descriptor_ids.len() as f32 };
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    draw_text(ctx, "Name", inner.x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    for (index, column_id) in descriptor_ids.iter().enumerate() {
        let x = inner.x + name_col_w + index as f32 * descriptor_col_w;
        draw_text(ctx, &vfs_descriptor_label(&schema, column_id), x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    let body = Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h);
    let scroll = scroll_offset(&scene.surface_id, "vfs");
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.surface_id, "vfs")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    let hovered_row = vfs.hovered_row_id.clone().or_else(|| ctx.input.hovered_id.clone());
    if visible_rows.is_empty() {
        let message = vfs.empty_message.as_deref().unwrap_or("No file system nodes");
        draw_text(ctx, message, body.x + pad, body.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    for entry in &visible_rows {
        let row = &entry.row;
        let row_index = visible_rows.iter().position(|v| v.row.get("id") == row.get("id")).unwrap_or(0);
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = row.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let control_id = format!("{}.vfs.{}", scene.surface_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let selected_row = selected.contains(&row_id);
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        if selected_row {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);
        let indent = entry.level as f32 * 14.0;
        let mut name_x = body.x + pad + indent;
        if entry.has_children {
            let chevron_rect = Rect::new(name_x, y, 14.0, row_h);
            let chevron = if entry.expanded { "chevron-down" } else { "chevron-right" };
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(chevron) {
                    ctx.draw.push_textured([chevron_rect.x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, ctx.theme.text_element);
                }
            }
            ctx.input.register_hit(HitTarget { rect: chevron_rect, event: None, control_id: Some(format!("{}.vfs.chevron.{}", scene.surface_id, row_id)), kind: HitKind::Generic, drag_axis: None, drag_data: None });
            name_x += 14.0;
        }
        let icon_id = vfs_glyph_icon(&schema, row);
        if let Some(icons) = ctx.icons {
            if let Some(uv) = icons.icon_uv(icon_id) {
                ctx.draw.push_textured([name_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, ctx.theme.text_element);
            }
        }
        name_x += 18.0;
        let name = row.get("name").and_then(|v| v.as_str()).unwrap_or("—");
        draw_text(ctx, name, name_x, y + row_h * 0.65, theme.font_size_small, if selected_row || hovered { theme.active_foreground } else { theme.text });
        for (col_index, column_id) in descriptor_ids.iter().enumerate() {
            let x = body.x + name_col_w + col_index as f32 * descriptor_col_w;
            let value = vfs_descriptor_value(&schema, row, column_id);
            draw_text(ctx, &value, x + pad, y + row_h * 0.65, theme.font_size_small, if selected_row { theme.active_foreground } else { theme.text_muted });
        }
        let drag_data = if vfs.drag_drop_enabled.unwrap_or(false) {
            let mut data = HashMap::new();
            data.insert("application/x-semio-vfs-node".into(), serde_json::to_string(row).unwrap_or_default());
            Some(data)
        } else {
            None
        };
        ctx.input.register_hit(HitTarget { rect: row_rect, event: None, control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data });
    }
    ctx.draw.pop_scissor();
}

fn vfs_double_click_action(scene: &UiComponentSceneNode, row: &Value) -> Option<ActionDescriptor> {
    let uri = row.get("navigateUri").and_then(|v| v.as_str())?;
    if uri.starts_with("os://instance/") {
        return Some(scene_action(
            scene,
            "openInstance",
            json!({
                "surfaceId": scene.surface_id,
                "instanceId": uri.trim_start_matches("os://instance/"),
            }),
        ));
    }
    if uri.starts_with("os://export/") {
        let parts: Vec<&str> = uri.split('/').collect();
        if parts.len() >= 5 {
            return Some(scene_action(
                scene,
                "exportMedia",
                json!({
                    "surfaceId": scene.surface_id,
                    "instanceId": parts[2],
                    "format": parts[4],
                }),
            ));
        }
    }
    if uri.starts_with("/spaces/") {
        let space_id = uri.split('/').nth(2)?;
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", json!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    if let Some(space_id) = uri.strip_prefix("studio:") {
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", json!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    None
}
//#endregion VirtualFileSystem

//#region VirtualFileSystemTests
#[cfg(test)]
mod virtual_file_system_tests {
    use super::*;

    /// 🗂️ A configured `fileNodeKinds[kindId].icon` must win over the kind-name fallback table —
    /// previously `vfs_glyph_icon` only checked `.is_some()` and always returned `"folder"` for any
    /// kind with a custom icon configured, discarding the actual icon id.
    #[test]
    fn configured_kind_icon_is_used_verbatim_not_collapsed_to_folder() {
        let schema: VfsSchema = serde_json::from_str(r#"{"fileNodeKinds":{"asset":{"icon":"box"}}}"#).unwrap();
        let row = json!({ "fileNodeKindId": "asset" });
        assert_eq!(vfs_glyph_icon(&schema, &row), "box");
    }

    #[test]
    fn folder_and_instance_kinds_fall_back_to_their_built_in_glyphs() {
        let schema: VfsSchema = serde_json::from_str("{}").unwrap();
        assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "folder" })), "folder");
        assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "instance" })), "box");
        assert_eq!(vfs_glyph_icon(&schema, &json!({ "fileNodeKindId": "other" })), "file-text");
    }

    #[test]
    fn missing_file_node_kind_id_defaults_to_the_file_kind() {
        let schema: VfsSchema = serde_json::from_str("{}").unwrap();
        assert_eq!(vfs_glyph_icon(&schema, &json!({})), "file-text");
    }
}
//#endregion VirtualFileSystemTests

//#region TextEditor
//#region State
/// 🗂️ Per-surface interaction state for double-click-to-select-word / completions / context-menu / rename.
///
/// 🔀️ Mid-session reconciliation note (concurrent `w2-scene-wiring` session): this region originally also
/// drove plain single-click-to-caret and drag-to-select itself, reading `ctx.input` pointer state directly
/// during render (mirroring the pre-existing focus-on-click code below), because at the time nothing
/// called `SceneInput::handle_scene_pointer_button`/`handle_scene_pointer_move` for any surface kind.
/// `w2-scene-wiring` landed `apply_scene_pointer` in `RenderEntry` next, calling those for every
/// non-bespoke surface kind including `TextEditor` from a once-per-render-frame `InputState` sample —
/// and `w4-scene-input` (`.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`) has
/// since replaced THAT with a real per-event route (`ui_wgpu::wgpu::UiCommand::Scene` ->
/// `interpreter::apply_scene_ui_command`, calling the same two handlers), deleting `apply_scene_pointer`
/// itself. Plain click/drag still reaches `EditorHost` via that generic path today, just per real event
/// now rather than sampled once per frame. That single-click/drag code was removed here to avoid
/// double-dispatching `textSelect`/`textEdit`; what remains below (double-click word-select, right-click
/// context menu, completions, rename) is *not* covered by the generic path and
/// stays. 🐛️➡️✅️ W4 fix (`.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w4-scene-input.md`):
/// `EditorHost::pointer_down_screen` used to no-operate entirely for `button != 0`, so both the generic
/// path's raw-button-passthrough call AND this region's own right-click handling had to force `button`
/// to `0` to reposition the caret at all. `pointer_down_screen` now repositions the caret for every
/// button (only a primary press also starts a drag-selection), so this region's `pointer_down`/
/// `pointer_up` pair below passes the real button through instead of forcing it.
#[derive(Clone, Debug, Default)]
struct TextEditorUiState {
    was_pointer_down: bool,
    /// 🖱️ `now_ms()`-based double-click detection — same convention (and the same always-`0.0`-on-native
    /// limitation) as `hit_double_click_target` elsewhere in this file.
    last_click_ms: f64,
    last_click_offset: Option<usize>,
    completions_open: bool,
    completion_index: usize,
    context_menu: Option<TextEditorContextMenu>,
    rename_active: bool,
    rename_occurrences: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
struct TextEditorContextMenu {
    x: f32,
    y: f32,
    items: Vec<TextEditorMenuItem>,
}

#[derive(Clone, Copy, Debug)]
struct TextEditorMenuItem {
    id: &'static str,
    label: &'static str,
}

/// 📋️ Mirrors `CompletionItem` (`text-editor-host.tsx`); `insertText` has no producer yet anywhere in the
/// codebase (`jack_completions_json` only ever emits `label`/`detail`) so `insert_text` falls back to
/// `label`, exactly like `identifierPrefixStart`/`applyCompletion` do on the React side.
#[derive(Clone, Deserialize)]
struct TextEditorCompletionItem {
    label: String,
    #[serde(default)]
    detail: Option<String>,
    #[serde(default, rename = "insertText")]
    insert_text: Option<String>,
}

#[derive(Clone, Copy, Deserialize)]
struct TextEditorSpan {
    start: usize,
    end: usize,
}

/// ✏️ Mirrors `RenameInfo` (`text-editor-host.tsx`), parsed from `TextEditorScene::rename_json`.
#[derive(Clone, Deserialize)]
struct TextEditorRenameInfo {
    name: String,
    occurrences: Vec<TextEditorSpan>,
}

thread_local! {
    static TEXT_EDITOR_UI_STATE: RefCell<HashMap<String, TextEditorUiState>> = RefCell::new(HashMap::new());
}

fn text_editor_ui_state(surface_id: &str) -> TextEditorUiState {
    TEXT_EDITOR_UI_STATE.with(|cell| cell.borrow().get(surface_id).cloned().unwrap_or_default())
}

fn store_text_editor_ui_state(surface_id: &str, state: TextEditorUiState) {
    TEXT_EDITOR_UI_STATE.with(|cell| {
        cell.borrow_mut().insert(surface_id.to_string(), state);
    });
}

fn text_editor_completions(editor: &ui_wgpu::wgpu::TextEditorScene) -> Vec<TextEditorCompletionItem> {
    editor.completions_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default()
}

fn text_editor_rename_info(editor: &ui_wgpu::wgpu::TextEditorScene) -> Option<TextEditorRenameInfo> {
    editor.rename_json.as_deref().and_then(|json| serde_json::from_str(json).ok())
}

/// ✂️ Identifier-prefix scan back from `caret`, mirroring `identifierPrefixStart`
/// (`framework/renderer/react/components/text-editor-host.tsx`) for the completion-commit replacement range.
fn identifier_prefix_start(text: &str, caret: usize) -> usize {
    let bytes = text.as_bytes();
    let mut start = caret.min(bytes.len());
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    start
}

/// 📏️ `[start, end)` byte range of the buffer line containing `cursor`, via the (previously unwired)
/// `line_col_at` helper below — mirrors `lineRangeAt` (`text-editor-host.tsx`)'s "Select Line" semantics.
fn text_editor_line_range(buffer: &str, cursor: usize) -> (usize, usize) {
    let (line_index, _) = line_col_at(buffer, cursor);
    let mut offset = 0usize;
    for (index, line) in buffer.lines().enumerate() {
        let end = offset + line.len();
        if index == line_index {
            return (offset, end);
        }
        offset = end + 1;
    }
    (buffer.len(), buffer.len())
}

/// 🧭️ Right-click menu rows, mirroring `buildTextEditorContextMenuItems` (`text-editor-host.tsx`) minus
/// clipboard (no OS clipboard binding exists anywhere in this crate yet — see `ui_wgpu::wgpu::events`'
/// `UiCommand::ClipboardCopy/Cut/PasteRequested`, which even there says the OS read/write is a
/// "host-region concern" still unwired) and the domain-specific "pick target" rows (those need a new
/// `EditorHost::pick_targets_at_screen_json` wrapper; deferred, noted in the ticket report).
fn text_editor_context_menu_items(editor: &ui_wgpu::wgpu::TextEditorScene) -> Vec<TextEditorMenuItem> {
    let mut items = Vec::new();
    if !text_editor_completions(editor).is_empty() {
        items.push(TextEditorMenuItem { id: "suggest", label: "Suggest Completions" });
    }
    items.push(TextEditorMenuItem { id: "select-token", label: "Select Token" });
    items.push(TextEditorMenuItem { id: "select-line", label: "Select Line" });
    items.push(TextEditorMenuItem { id: "select-all", label: "Select All" });
    if text_editor_rename_info(editor).is_some() {
        items.push(TextEditorMenuItem { id: "rename", label: "Rename" });
    }
    items.push(TextEditorMenuItem { id: "format", label: "Format Document" });
    items.push(TextEditorMenuItem { id: "lint", label: "Lint Document" });
    items
}

/// ▶️ Executes one context-menu row. `inner` re-derives the click point in surface-local screen space for
/// "Select Token"/"Select Line"; "Select All" reuses `engine_canvas::text_editor_apply_key`'s existing
/// Ctrl/Cmd+A path instead of adding a sixth wrapper.
fn text_editor_run_menu_action(scene: &UiComponentSceneNode, editor: &ui_wgpu::wgpu::TextEditorScene, inner: Rect, menu: &TextEditorContextMenu, action_id: &str, ctx: &mut FrameworkWidgetContext<'_>, ui_state: &mut TextEditorUiState) {
    match action_id {
        "suggest" => {
            ui_state.completions_open = true;
            ui_state.completion_index = 0;
        }
        "select-token" => {
            for action in engine_canvas::text_editor_select_span_at_screen(scene, inner, menu.x, menu.y) {
                ctx.input.queue_event(action);
            }
        }
        "select-line" => {
            let offset = cursor_from_click(scene, inner, menu.x, menu.y, 0.0);
            let (start, end) = text_editor_line_range(&editor.buffer, offset);
            for action in engine_canvas::text_editor_set_selection(scene, start, end) {
                ctx.input.queue_event(action);
            }
        }
        "select-all" => {
            let modifiers = ui_wgpu::wgpu::PointerModifiers { ctrl: true, ..Default::default() };
            for action in engine_canvas::text_editor_apply_key(scene, KeyAction::Char("a".to_string()), &modifiers) {
                ctx.input.queue_event(action);
            }
        }
        "rename" => {
            if let Some(info) = text_editor_rename_info(editor) {
                ctx.input.focus_input(&format!("{}.editor.rename", scene.surface_id), &info.name);
                ui_state.rename_active = true;
                ui_state.rename_occurrences = info.occurrences.iter().map(|span| (span.start, span.end)).collect();
            }
        }
        "format" => {
            ctx.input.queue_event(scene_action(scene, "formatDocument", json!({ "surfaceId": scene.surface_id })));
        }
        "lint" => {
            ctx.input.queue_event(scene_action(scene, "lintDocument", json!({ "surfaceId": scene.surface_id })));
        }
        _ => {}
    }
}
//#endregion State

//#region Popups
/// 📍️ Anchor (surface-local screen space) for the completions dropdown: near the caret, falling back to a
/// fixed offset if the host isn't ready yet — mirrors `WasmEditorSurface`'s `position ? ... : { left: 12, top: 12 }`.
fn text_editor_completion_anchor(scene: &UiComponentSceneNode, inner: Rect) -> (f32, f32) {
    engine_canvas::text_editor_caret_screen(scene, inner).unwrap_or((inner.x + 12.0, inner.y + 12.0))
}

fn text_editor_completion_row_rect(anchor: (f32, f32), theme: &Theme, index: usize) -> Rect {
    let row_h = theme.control_height_small;
    Rect::new(anchor.0, anchor.1 + 18.0 + index as f32 * row_h, 220.0, row_h)
}

fn text_editor_completion_hit(scene: &UiComponentSceneNode, inner: Rect, theme: &Theme, len: usize, x: f32, y: f32) -> Option<usize> {
    let anchor = text_editor_completion_anchor(scene, inner);
    (0..len).find(|&index| text_editor_completion_row_rect(anchor, theme, index).contains(x, y))
}

fn text_editor_menu_row_rect(menu: &TextEditorContextMenu, theme: &Theme, index: usize) -> Rect {
    let row_h = theme.control_height;
    Rect::new(menu.x + 4.0, menu.y + 4.0 + index as f32 * row_h, 200.0 - 8.0, row_h)
}

fn text_editor_menu_hit(menu: &TextEditorContextMenu, theme: &Theme, x: f32, y: f32) -> Option<usize> {
    (0..menu.items.len()).find(|&index| text_editor_menu_row_rect(menu, theme, index).contains(x, y))
}

/// 🍿️ Local fallback popup: `ui_wgpu::wgpu::events::{OverlayKind, open_overlay}` (the w1d-events-overlay
/// workstream) is `pub(crate)` inside `ui_wgpu` — not reachable from this crate yet ("None of the new
/// `EventRouter` API or new public types are called/re-exported from `engine`/crate-root yet", per that
/// workstream's own report) — so this draws directly via `ctx.draw`, same convention as row-list surfaces
/// elsewhere in this module (`render_vfs`'s `theme.selected`/`theme.row_hover` rows). Known limitation:
/// unlike `shell`'s own `render_context_menu` (drawn into a dedicated top-level overlay `DrawList`), this
/// draws into the regular in-flow layer, so it can't guarantee being on top of *other* panels — only of
/// this surface's own content and anything already drawn earlier in the frame.
fn render_text_editor_completions(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, scene: &UiComponentSceneNode, completions: &[TextEditorCompletionItem], active_index: usize) {
    let theme = ctx.theme;
    let anchor = text_editor_completion_anchor(scene, inner);
    if completions.is_empty() {
        return;
    }
    // 🪟️ Outer popover container — matches `rounded border border-border bg-popover p-1 shadow-md`
    // on the completions list in `text-editor-host.tsx`; previously only per-row backgrounds were
    // drawn, with no enclosing frame at all.
    let pad = 4.0;
    let row_h = text_editor_completion_row_rect(anchor, theme, 0).h;
    let container = Rect::new(anchor.0 - pad, anchor.1 + 18.0 - pad, 220.0 + pad * 2.0, completions.len() as f32 * row_h + pad * 2.0);
    ctx.draw.push_rounded([container.x, container.y, container.w, container.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, container.x, container.y, container.w, container.h, theme.panel_border, 1.0);
    for (index, item) in completions.iter().enumerate() {
        let row = text_editor_completion_row_rect(anchor, theme, index);
        // 🎯️ Active row uses `bg-accent text-accent-foreground` in React — `theme.selected` (the
        // generic row-highlight token) previously stood in for the accent pairing used nowhere else
        // for completion rows.
        let (bg, fg) = if index == active_index { (theme.accent, theme.active_foreground) } else { (theme.panel, theme.text) };
        ctx.draw.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius * 0.5);
        draw_text(ctx, &item.label, row.x + 8.0, row.y + row.h * 0.68, theme.font_size_small, fg);
        if let Some(detail) = &item.detail {
            let label_w = item.label.len() as f32 * theme.font_size_small * 0.6;
            let detail_fg = if index == active_index { fg } else { theme.text_muted };
            draw_text(ctx, detail, row.x + 12.0 + label_w, row.y + row.h * 0.68, theme.font_size_small, detail_fg);
        }
    }
}

fn render_text_editor_context_menu(ctx: &mut FrameworkWidgetContext<'_>, menu: &TextEditorContextMenu) {
    let theme = ctx.theme;
    let row_h = theme.control_height;
    let w = 200.0;
    let h = menu.items.len() as f32 * row_h + 8.0;
    ctx.draw.push_rounded([menu.x, menu.y, w, h], theme.panel, theme.border_radius);
    // 🖊️ `ContextMenuChrome`/`WindowChrome` in React paints this as a titled floating window with a
    // glass material and its own border; the full window-chrome treatment is `shell`-owned (out of
    // scope here — see `render_context_menu`'s own top-level overlay), so this at least adds the
    // border stroke React's window frame always carries instead of a flat, edgeless panel fill.
    draw_ink_rect_outline(ctx.draw, menu.x, menu.y, w, h, theme.panel_border, 1.0);
    for (index, item) in menu.items.iter().enumerate() {
        let row = text_editor_menu_row_rect(menu, theme, index);
        ctx.draw.push_rounded([row.x, row.y, row.w, row.h], theme.button, theme.border_radius * 0.5);
        draw_text(ctx, item.label, row.x + 8.0, row.y + row.h * 0.68, theme.font_size_small, theme.text);
    }
}

fn render_text_editor_rename_input(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, scene: &UiComponentSceneNode) {
    let theme = ctx.theme;
    let text = ctx.input.text_buffer.clone();
    let (x, y) = engine_canvas::text_editor_caret_screen(scene, inner).unwrap_or((inner.x + 12.0, inner.y + 12.0));
    let rect = Rect::new(x, (y - theme.control_height_small * 0.5).max(inner.y), 180.0, theme.control_height_small);
    // 🖊️ `border border-border bg-panel` on the rename input in `text-editor-host.tsx` — previously
    // drawn as an unbordered `theme.input_bg` fill (a different token, and no stroke at all).
    ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.panel, theme.border_radius * 0.5);
    draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, theme.panel_border, 1.0);
    draw_text(ctx, &text, rect.x + 8.0, rect.y + rect.h * 0.68, theme.font_size_small, theme.text);
}
//#endregion Popups

//#region Geometry
fn cursor_from_click(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, scroll: f32) -> usize {
    let Some(editor) = &scene.text_editor else {
        return 0;
    };
    let line_h = 18.0;
    let line_index = ((y - inner.y - 8.0 + scroll) / line_h).max(0.0) as usize;
    let lines: Vec<&str> = editor.buffer.lines().collect();
    let line = lines.get(line_index).copied().unwrap_or("");
    let rel_x = (x - inner.x - 8.0).max(0.0);
    let mut cursor = 0usize;
    let mut width = 0.0f32;
    for (index, ch) in line.chars().enumerate() {
        let advance = if ch == '\t' { 8.0 } else { 7.0 };
        if width + advance * 0.5 > rel_x {
            cursor = index;
            break;
        }
        width += advance;
        cursor = index + 1;
    }
    lines.iter().take(line_index).map(|l| l.len() + 1).sum::<usize>() + cursor
}

fn line_col_at(text: &str, cursor: usize) -> (usize, usize) {
    let mut index = 0usize;
    for (line_index, line) in text.lines().enumerate() {
        let next = index + line.len() + 1;
        if cursor < next {
            return (line_index, cursor.saturating_sub(index));
        }
        index = next;
    }
    let line_count = text.lines().count();
    (line_count.saturating_sub(1), 0)
}
//#endregion Geometry

//#region Render
fn render_text_editor(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, gpu: &mut ui_wgpu::wgpu::GpuContext) {
    let Some(editor) = &scene.text_editor else {
        return render_placeholder("text-editor", bounds, ctx);
    };
    let inner = bounds;
    engine_canvas::paint_text_editor(gpu, ctx, scene, inner);
    let editor_id = format!("{}.editor", scene.surface_id);
    let rename_id = format!("{}.editor.rename", scene.surface_id);
    let seed_focused = ctx.input.focused_id.as_deref() == Some(editor_id.as_str());
    if seed_focused && ctx.input.text_buffer.is_empty() && !editor.buffer.is_empty() {
        ctx.input.focus_input(&editor_id, &editor.buffer);
    }

    let mut ui_state = text_editor_ui_state(&scene.surface_id);
    let hovered = inner.contains(ctx.input.pointer_x, ctx.input.pointer_y);
    let pressed_edge = ctx.input.pointer_down && !ui_state.was_pointer_down;

    //#region PointerInput
    // 🔀️ Plain single-click-to-caret and drag-to-select are handled by the generic real per-event
    // `ui_wgpu::wgpu::UiCommand::Scene` -> `interpreter::apply_scene_ui_command` ->
    // `handle_scene_pointer_button`/`handle_scene_pointer_move` route now (see `TextEditorUiState`'s
    // doc comment) — this block only covers what that path doesn't: double-click word-select and the
    // right-click context menu.
    if pressed_edge {
        let mut consumed_press = false;
        // 🍿️ Completions popup rows take priority: commit on hit; a miss just falls through so the click
        // still positions the caret, matching `WasmEditorSurface`'s document-pane click handling.
        if ui_state.completions_open {
            let completions = text_editor_completions(editor);
            if let Some(index) = text_editor_completion_hit(scene, inner, ctx.theme, completions.len(), ctx.input.pointer_x, ctx.input.pointer_y) {
                if let Some(item) = completions.get(index) {
                    let (_, caret) = engine_canvas::text_editor_caret(scene);
                    let prefix_start = identifier_prefix_start(&editor.buffer, caret);
                    let insert_text = item.insert_text.clone().unwrap_or_else(|| item.label.clone());
                    for action in engine_canvas::text_editor_apply_completion(scene, prefix_start, caret, &insert_text) {
                        ctx.input.queue_event(action);
                    }
                }
                ui_state.completions_open = false;
                consumed_press = true;
            }
        }
        // 🖱️ Context-menu rows: any press while one is open dismisses it; a hit also runs the action.
        if !consumed_press {
            if let Some(menu) = ui_state.context_menu.clone() {
                if let Some(index) = text_editor_menu_hit(&menu, ctx.theme, ctx.input.pointer_x, ctx.input.pointer_y) {
                    if let Some(item) = menu.items.get(index).copied() {
                        text_editor_run_menu_action(scene, editor, inner, &menu, item.id, ctx, &mut ui_state);
                    }
                }
                ui_state.context_menu = None;
                consumed_press = true;
            }
        }
        if !consumed_press && !ui_state.rename_active {
            if ctx.input.pointer_button == 2 && hovered {
                // 🖱️➡️ Reposition the caret first (real button `2`, matching `WasmEditorSurface.onContextMenu`'s
                // `pointerDownScreen(sx, sy, 2)`), then open the menu at the click point. 🐛️➡️✅️ W4 fix: this used
                // to force `button` to `0` since `EditorHost::pointer_down_screen` no-operated entirely for
                // `button != 0` — now that it repositions the caret for every button (see that fn's own doc
                // comment), passing the real button through is both correct AND avoids incorrectly flagging
                // `drag_selecting` for what is not a primary-button press.
                let mut actions = engine_canvas::text_editor_pointer_down(scene, inner, ctx.input.pointer_x, ctx.input.pointer_y, ctx.input.pointer_button);
                actions.extend(engine_canvas::text_editor_pointer_up(scene, inner, ctx.input.pointer_x, ctx.input.pointer_y));
                for action in actions {
                    ctx.input.queue_event(action);
                }
                ctx.input.focus_input(&editor_id, &editor.buffer);
                ui_state.completions_open = false;
                ui_state.context_menu = Some(TextEditorContextMenu { x: ctx.input.pointer_x, y: ctx.input.pointer_y, items: text_editor_context_menu_items(editor) });
            } else if ctx.input.pointer_button == 0 && hovered {
                // ✋️ The generic `UiCommand::Scene` route already repositions the caret / extends the
                // drag-selection for this same press (via `apply_scene_ui_command`); this only tracks
                // double-click timing/offset locally and closes the completions popup / (re)focuses for
                // keyboard routing.
                ctx.input.focus_input(&editor_id, &editor.buffer);
                ui_state.completions_open = false;
                let click_offset = cursor_from_click(scene, inner, ctx.input.pointer_x, ctx.input.pointer_y, 0.0);
                let now = now_ms();
                let is_double = ui_state.last_click_offset == Some(click_offset) && (now - ui_state.last_click_ms).abs() < 400.0;
                if is_double {
                    for action in engine_canvas::text_editor_select_span_at_screen(scene, inner, ctx.input.pointer_x, ctx.input.pointer_y) {
                        ctx.input.queue_event(action);
                    }
                }
                ui_state.last_click_ms = now;
                ui_state.last_click_offset = Some(click_offset);
            }
        }
    }
    ui_state.was_pointer_down = ctx.input.pointer_down;
    //#endregion PointerInput

    //#region Keyboard
    let focused = ctx.input.focused_id.as_deref() == Some(editor_id.as_str());
    let renaming = ui_state.rename_active && ctx.input.focused_id.as_deref() == Some(rename_id.as_str());
    if renaming {
        for key in ctx.input.drain_keys() {
            match key {
                KeyAction::Escape => {
                    ui_state.rename_active = false;
                    ctx.input.blur_input();
                }
                KeyAction::Enter => {
                    let occurrences: Vec<Value> = ui_state.rename_occurrences.iter().map(|(start, end)| json!({ "start": start, "end": end })).collect();
                    ctx.input.queue_event(scene_action(scene, "commitRename", json!({ "surfaceId": scene.surface_id, "occurrences": occurrences, "text": ctx.input.text_buffer.clone() })));
                    ui_state.rename_active = false;
                    ctx.input.blur_input();
                }
                KeyAction::Char(ch) => {
                    for c in ch.chars() {
                        ctx.input.insert_char(c);
                    }
                }
                KeyAction::Backspace => ctx.input.backspace(),
                KeyAction::Delete => ctx.input.delete_forward(),
                _ => {}
            }
        }
    } else if focused {
        let modifiers = ctx.input.modifiers.clone();
        let completions = text_editor_completions(editor);
        for key in ctx.input.drain_keys() {
            if ui_state.completions_open && !completions.is_empty() {
                match key {
                    KeyAction::ArrowDown => {
                        ui_state.completion_index = (ui_state.completion_index + 1) % completions.len();
                        continue;
                    }
                    KeyAction::ArrowUp => {
                        ui_state.completion_index = (ui_state.completion_index + completions.len() - 1) % completions.len();
                        continue;
                    }
                    KeyAction::Tab | KeyAction::Enter => {
                        let item = &completions[ui_state.completion_index.min(completions.len() - 1)];
                        let (_, caret) = engine_canvas::text_editor_caret(scene);
                        let prefix_start = identifier_prefix_start(&editor.buffer, caret);
                        let insert_text = item.insert_text.clone().unwrap_or_else(|| item.label.clone());
                        for action in engine_canvas::text_editor_apply_completion(scene, prefix_start, caret, &insert_text) {
                            ctx.input.queue_event(action);
                        }
                        ui_state.completions_open = false;
                        continue;
                    }
                    KeyAction::Escape => {
                        ui_state.completions_open = false;
                        continue;
                    }
                    _ => {}
                }
            }
            match key {
                KeyAction::Space(true) if (modifiers.meta || modifiers.ctrl) && !completions.is_empty() => {
                    ui_state.completions_open = true;
                    ui_state.completion_index = 0;
                }
                KeyAction::Enter if modifiers.meta || modifiers.ctrl => {
                    ctx.input.queue_event(scene_action(scene, "submit", json!({ "surfaceId": scene.surface_id, "document": editor.buffer })));
                }
                KeyAction::Char(ch) if (modifiers.meta || modifiers.ctrl) && ch.eq_ignore_ascii_case("s") => {
                    ctx.input.queue_event(scene_action(scene, "formatDocument", json!({ "surfaceId": scene.surface_id })));
                }
                KeyAction::Enter | KeyAction::Escape => {
                    ctx.input.queue_event(scene_action(scene, "textEdit", json!({ "surfaceId": scene.surface_id, "document": editor.buffer })));
                    if matches!(key, KeyAction::Escape) {
                        ctx.input.blur_input();
                    }
                }
                KeyAction::Char(_) | KeyAction::Backspace | KeyAction::Delete => {
                    for action in engine_canvas::text_editor_apply_key(scene, key, &modifiers) {
                        ctx.input.queue_event(action);
                    }
                }
                _ => {}
            }
        }
    }
    //#endregion Keyboard

    //#region Popups
    if ui_state.completions_open {
        let completions = text_editor_completions(editor);
        if completions.is_empty() {
            ui_state.completions_open = false;
        } else {
            if ui_state.completion_index >= completions.len() {
                ui_state.completion_index = 0;
            }
            render_text_editor_completions(ctx, inner, scene, &completions, ui_state.completion_index);
        }
    }
    if let Some(menu) = ui_state.context_menu.clone() {
        render_text_editor_context_menu(ctx, &menu);
    }
    if ui_state.rename_active {
        render_text_editor_rename_input(ctx, inner, scene);
    }
    //#endregion Popups

    store_text_editor_ui_state(&scene.surface_id, ui_state);
}
//#endregion Render

#[cfg(test)]
mod text_editor_tests {
    use super::*;
    use crate::interpreter::framework_widget_context;
    use ui_wgpu::wgpu::UiPresence;

    fn test_scene(surface_id: &str, kind: SurfaceKind) -> UiComponentSceneNode {
        UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "controller".into(),
            component_kind: kind,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: None,
            menu: None,
        }
    }

    fn text_editor_scene_payload(buffer: &str, completions_json: Option<&str>, rename_json: Option<&str>) -> ui_wgpu::wgpu::TextEditorScene {
        ui_wgpu::wgpu::TextEditorScene {
            buffer: buffer.to_string(),
            language: None,
            selection_json: None,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: completions_json.map(str::to_string),
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: None,
            camera_json: None,
            hover_json: None,
            newline_gates_json: None,
            rename_json: rename_json.map(str::to_string),
        }
    }

    fn text_editor_scene(surface_id: &str, buffer: &str, completions_json: Option<&str>, rename_json: Option<&str>) -> UiComponentSceneNode {
        let mut scene = test_scene(surface_id, SurfaceKind::TextEditor);
        scene.text_editor = Some(text_editor_scene_payload(buffer, completions_json, rename_json));
        scene
    }

    /// 🧰️ GPU-free `FrameworkWidgetContext` fixture, same construction as `render_entry_tests::Fixture`
    /// (private to that module, so duplicated here rather than reused).
    struct Fixture {
        draw: ui_wgpu::wgpu::DrawList,
        atlas: ui_wgpu::wgpu::FontAtlas,
        theme: Theme,
        input: ui_wgpu::wgpu::InputState<ActionDescriptor>,
        scroll_offsets: HashMap<String, f32>,
        collapsed_sections: HashMap<String, bool>,
        open_selects: HashMap<String, bool>,
    }

    impl Fixture {
        fn new() -> Self {
            Self {
                draw: ui_wgpu::wgpu::DrawList::default(),
                atlas: ui_wgpu::wgpu::FontAtlas::builtin(),
                theme: Theme::default(),
                input: ui_wgpu::wgpu::InputState::<ActionDescriptor>::default(),
                scroll_offsets: HashMap::new(),
                collapsed_sections: HashMap::new(),
                open_selects: HashMap::new(),
            }
        }

        fn ctx(&mut self) -> FrameworkWidgetContext<'_> {
            framework_widget_context(&mut self.draw, None, &mut self.atlas, None, &mut self.input, &self.theme, &mut self.scroll_offsets, &mut self.collapsed_sections, &mut self.open_selects, None)
        }
    }

    //#region ClickToCaretGeometry
    #[test]
    fn cursor_from_click_resolves_the_first_line_offset_at_the_click_x() {
        let scene = text_editor_scene("editor.click", "hello\nworld", None, None);
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        // 8px left padding + ~7px/char advance (see `cursor_from_click`): clicking near x=8 should land at
        // the very start of the line, clicking further right should land later in "hello".
        let start = cursor_from_click(&scene, inner, 8.0, 8.0, 0.0);
        let mid = cursor_from_click(&scene, inner, 8.0 + 7.0 * 3.0, 8.0, 0.0);
        assert_eq!(start, 0);
        assert!(mid >= 2 && mid <= 4, "expected an offset inside \"hello\", got {mid}");
    }

    #[test]
    fn cursor_from_click_accounts_for_line_index_via_y() {
        let scene = text_editor_scene("editor.click.line2", "ab\ncd\nef", None, None);
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        // line_h is 18.0 and the first line starts at y = inner.y + 8.0 (see `cursor_from_click`).
        let offset = cursor_from_click(&scene, inner, 8.0, 8.0 + 18.0 + 2.0, 0.0);
        let (line, col) = line_col_at("ab\ncd\nef", offset);
        assert_eq!((line, col), (1, 0));
    }

    #[test]
    fn line_col_at_reports_line_and_column_for_a_mid_buffer_offset() {
        let (line, col) = line_col_at("alpha\nbeta\ngamma", 7);
        assert_eq!((line, col), (1, 1));
    }
    //#endregion ClickToCaretGeometry

    //#region SelectLine
    #[test]
    fn text_editor_line_range_returns_the_bounds_of_the_containing_line() {
        let buffer = "first\nsecond line\nthird";
        let (start, end) = text_editor_line_range(buffer, 9);
        assert_eq!(&buffer[start..end], "second line");
    }

    #[test]
    fn text_editor_line_range_handles_the_last_line_without_a_trailing_newline() {
        let buffer = "one\ntwo";
        let (start, end) = text_editor_line_range(buffer, 5);
        assert_eq!(&buffer[start..end], "two");
    }
    //#endregion SelectLine

    //#region CompletionPrefix
    #[test]
    fn identifier_prefix_start_stops_at_the_nearest_non_identifier_char() {
        let text = "let value = my_var";
        let caret = text.len();
        assert_eq!(&text[identifier_prefix_start(text, caret)..caret], "my_var");
    }

    #[test]
    fn identifier_prefix_start_returns_caret_when_not_inside_an_identifier() {
        let text = "a = ";
        assert_eq!(identifier_prefix_start(text, text.len()), text.len());
    }
    //#endregion CompletionPrefix

    //#region CompletionsParsing
    #[test]
    fn text_editor_completions_parses_label_and_optional_detail() {
        let scene = text_editor_scene("editor.completions", "", Some(r#"[{"label":"foo","detail":"fn foo()"},{"label":"bar"}]"#), None);
        let items = text_editor_completions(scene.text_editor.as_ref().unwrap());
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].label, "foo");
        assert_eq!(items[0].detail.as_deref(), Some("fn foo()"));
        assert_eq!(items[1].detail, None);
    }

    #[test]
    fn text_editor_completions_is_empty_for_missing_or_malformed_json() {
        let missing = text_editor_scene("editor.completions.missing", "", None, None);
        assert!(text_editor_completions(missing.text_editor.as_ref().unwrap()).is_empty());
        let malformed = text_editor_scene("editor.completions.bad", "", Some("not json"), None);
        assert!(text_editor_completions(malformed.text_editor.as_ref().unwrap()).is_empty());
    }

    #[test]
    fn text_editor_rename_info_parses_name_and_occurrences() {
        let scene = text_editor_scene("editor.rename", "", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5},{"start":10,"end":15}]}"#));
        let info = text_editor_rename_info(scene.text_editor.as_ref().unwrap()).expect("rename info");
        assert_eq!(info.name, "count");
        assert_eq!(info.occurrences.len(), 2);
        assert_eq!((info.occurrences[1].start, info.occurrences[1].end), (10, 15));
    }
    //#endregion CompletionsParsing

    //#region ContextMenuItems
    #[test]
    fn context_menu_items_include_suggest_only_when_completions_are_present() {
        let without = text_editor_scene("editor.menu.no-suggest", "x", None, None);
        let items = text_editor_context_menu_items(without.text_editor.as_ref().unwrap());
        assert!(!items.iter().any(|item| item.id == "suggest"));

        let with = text_editor_scene("editor.menu.suggest", "x", Some(r#"[{"label":"x"}]"#), None);
        let items = text_editor_context_menu_items(with.text_editor.as_ref().unwrap());
        assert!(items.iter().any(|item| item.id == "suggest"));
    }

    #[test]
    fn context_menu_items_include_rename_only_when_rename_info_is_present() {
        let without = text_editor_scene("editor.menu.no-rename", "x", None, None);
        let items = text_editor_context_menu_items(without.text_editor.as_ref().unwrap());
        assert!(!items.iter().any(|item| item.id == "rename"));

        let with = text_editor_scene("editor.menu.rename", "x", None, Some(r#"{"name":"x","occurrences":[]}"#));
        let items = text_editor_context_menu_items(with.text_editor.as_ref().unwrap());
        assert!(items.iter().any(|item| item.id == "rename"));
    }

    #[test]
    fn context_menu_items_always_include_selection_and_document_actions() {
        let scene = text_editor_scene("editor.menu.baseline", "x", None, None);
        let items = text_editor_context_menu_items(scene.text_editor.as_ref().unwrap());
        for expected in ["select-token", "select-line", "select-all", "format", "lint"] {
            assert!(items.iter().any(|item| item.id == expected), "missing {expected}");
        }
    }
    //#endregion ContextMenuItems

    //#region ContextMenuGeometry
    #[test]
    fn menu_row_rects_stack_vertically_without_overlapping() {
        let menu = TextEditorContextMenu { x: 10.0, y: 20.0, items: vec![TextEditorMenuItem { id: "a", label: "A" }, TextEditorMenuItem { id: "b", label: "B" }, TextEditorMenuItem { id: "c", label: "C" }] };
        let theme = Theme::default();
        let first = text_editor_menu_row_rect(&menu, &theme, 0);
        let second = text_editor_menu_row_rect(&menu, &theme, 1);
        assert_eq!(second.y, first.y + theme.control_height);
        assert_eq!(first.x, second.x);
    }

    #[test]
    fn menu_hit_finds_the_row_under_the_point_and_none_outside_it() {
        let menu = TextEditorContextMenu { x: 0.0, y: 0.0, items: vec![TextEditorMenuItem { id: "a", label: "A" }, TextEditorMenuItem { id: "b", label: "B" }] };
        let theme = Theme::default();
        let row_h = theme.control_height;
        assert_eq!(text_editor_menu_hit(&menu, &theme, 8.0, 8.0), Some(0));
        assert_eq!(text_editor_menu_hit(&menu, &theme, 8.0, row_h + 8.0), Some(1));
        assert_eq!(text_editor_menu_hit(&menu, &theme, 8.0, row_h * 10.0), None);
    }
    //#endregion ContextMenuGeometry

    //#region ContextMenuActionDispatch
    #[test]
    fn run_menu_action_format_queues_a_format_document_action() {
        let scene = text_editor_scene("editor.action.format", "hello", None, None);
        let editor = scene.text_editor.as_ref().unwrap().clone();
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
        let mut fixture = Fixture::new();
        let mut ui_state = TextEditorUiState::default();
        {
            let mut ctx = fixture.ctx();
            text_editor_run_menu_action(&scene, &editor, inner, &menu, "format", &mut ctx, &mut ui_state);
        }
        let events = fixture.input.drain_events();
        assert!(events.iter().any(|action| action.action == "formatDocument"));
    }

    #[test]
    fn run_menu_action_lint_queues_a_lint_document_action() {
        let scene = text_editor_scene("editor.action.lint", "hello", None, None);
        let editor = scene.text_editor.as_ref().unwrap().clone();
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
        let mut fixture = Fixture::new();
        let mut ui_state = TextEditorUiState::default();
        {
            let mut ctx = fixture.ctx();
            text_editor_run_menu_action(&scene, &editor, inner, &menu, "lint", &mut ctx, &mut ui_state);
        }
        let events = fixture.input.drain_events();
        assert!(events.iter().any(|action| action.action == "lintDocument"));
    }

    #[test]
    fn run_menu_action_suggest_opens_the_completions_popup_at_index_zero() {
        let scene = text_editor_scene("editor.action.suggest", "hello", Some(r#"[{"label":"a"},{"label":"b"}]"#), None);
        let editor = scene.text_editor.as_ref().unwrap().clone();
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
        let mut fixture = Fixture::new();
        let mut ui_state = TextEditorUiState { completion_index: 3, ..Default::default() };
        {
            let mut ctx = fixture.ctx();
            text_editor_run_menu_action(&scene, &editor, inner, &menu, "suggest", &mut ctx, &mut ui_state);
        }
        assert!(ui_state.completions_open);
        assert_eq!(ui_state.completion_index, 0);
    }

    #[test]
    fn run_menu_action_rename_activates_rename_state_and_focuses_the_rename_input() {
        let scene = text_editor_scene("editor.action.rename", "count", None, Some(r#"{"name":"count","occurrences":[{"start":0,"end":5}]}"#));
        let editor = scene.text_editor.as_ref().unwrap().clone();
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
        let mut fixture = Fixture::new();
        let mut ui_state = TextEditorUiState::default();
        {
            let mut ctx = fixture.ctx();
            text_editor_run_menu_action(&scene, &editor, inner, &menu, "rename", &mut ctx, &mut ui_state);
        }
        assert!(ui_state.rename_active);
        assert_eq!(ui_state.rename_occurrences, vec![(0, 5)]);
        assert_eq!(fixture.input.focused_id.as_deref(), Some("editor.action.rename.editor.rename"));
        assert_eq!(fixture.input.text_buffer, "count");
    }

    #[test]
    fn run_menu_action_rename_is_a_no_op_without_rename_info() {
        let scene = text_editor_scene("editor.action.no-rename", "count", None, None);
        let editor = scene.text_editor.as_ref().unwrap().clone();
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
        let mut fixture = Fixture::new();
        let mut ui_state = TextEditorUiState::default();
        {
            let mut ctx = fixture.ctx();
            text_editor_run_menu_action(&scene, &editor, inner, &menu, "rename", &mut ctx, &mut ui_state);
        }
        assert!(!ui_state.rename_active);
        assert!(fixture.input.focused_id.is_none());
    }

    #[test]
    fn run_menu_action_select_all_reuses_the_ctrl_a_key_path_without_a_registered_engine_surface() {
        // 🛡️ No GPU / `ENGINE_SURFACES` entry exists for this surface_id in a unit test, so this only
        // asserts the dispatch doesn't panic and gracefully no-operations (see `engine_canvas::text_editor_apply_key`).
        let scene = text_editor_scene("editor.action.select-all", "hello", None, None);
        let editor = scene.text_editor.as_ref().unwrap().clone();
        let inner = Rect::new(0.0, 0.0, 200.0, 200.0);
        let menu = TextEditorContextMenu { x: 4.0, y: 4.0, items: vec![] };
        let mut fixture = Fixture::new();
        let mut ui_state = TextEditorUiState::default();
        let mut ctx = fixture.ctx();
        text_editor_run_menu_action(&scene, &editor, inner, &menu, "select-all", &mut ctx, &mut ui_state);
    }
    //#endregion ContextMenuActionDispatch

    //#region PopupChromePaintTests
    #[test]
    fn completions_popup_has_a_bordered_container_and_the_active_row_uses_accent() {
        let scene = text_editor_scene("editor.completions.paint", "", None, None);
        let inner = Rect::new(0.0, 0.0, 300.0, 300.0);
        let completions = vec![TextEditorCompletionItem { label: "alpha".into(), detail: None, insert_text: None }, TextEditorCompletionItem { label: "beta".into(), detail: None, insert_text: None }];
        let mut fixture = Fixture::new();
        {
            let mut ctx = fixture.ctx();
            render_text_editor_completions(&mut ctx, inner, &scene, &completions, 0);
        }
        let theme = fixture.theme;
        // 🪟️ `border border-border bg-popover` — the container's own outline color must appear among
        // the drawn vector-line vertices (`draw_ink_rect_outline`'s 4-line/24-vertex shape).
        let border = theme.panel_border;
        let has_container_border = fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|v| v.color == [border.r, border.g, border.b, border.a]);
        assert!(has_container_border, "expected the completions popup to draw an outer container border");

        // 🎯️ `bg-accent text-accent-foreground` on the active (index 0) row.
        let colors: Vec<[f32; 4]> = fixture.draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
        let accent = theme.accent;
        assert!(colors.contains(&[accent.r, accent.g, accent.b, accent.a]), "expected the active completion row's background to be theme.accent, got {colors:?}");
    }

    #[test]
    fn rename_input_draws_a_bordered_panel_box() {
        let scene = text_editor_scene("editor.rename.paint", "count", None, None);
        let inner = Rect::new(0.0, 0.0, 300.0, 300.0);
        let mut fixture = Fixture::new();
        fixture.input.text_buffer = "count2".into();
        {
            let mut ctx = fixture.ctx();
            render_text_editor_rename_input(&mut ctx, inner, &scene);
        }
        let theme = fixture.theme;
        // 🖊️ `border border-border bg-panel` — previously an unbordered `theme.input_bg` fill.
        let border = theme.panel_border;
        let has_border = fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|v| v.color == [border.r, border.g, border.b, border.a]);
        assert!(has_border, "expected the rename input to draw a border stroke");
        let colors: Vec<[f32; 4]> = fixture.draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
        let panel = theme.panel;
        assert!(colors.contains(&[panel.r, panel.g, panel.b, panel.a]), "expected the rename input fill to use theme.panel, got {colors:?}");
    }

    #[test]
    fn context_menu_draws_a_border_stroke_around_the_flat_panel() {
        let menu = TextEditorContextMenu { x: 10.0, y: 10.0, items: vec![TextEditorMenuItem { id: "rename", label: "Rename" }] };
        let mut fixture = Fixture::new();
        {
            let mut ctx = fixture.ctx();
            render_text_editor_context_menu(&mut ctx, &menu);
        }
        let theme = fixture.theme;
        let border = theme.panel_border;
        let has_border = fixture.draw.layers.iter().flat_map(|layer| layer.vector_vertices.iter()).any(|v| v.color == [border.r, border.g, border.b, border.a]);
        assert!(has_border, "expected the context menu's flat panel to at least draw a border stroke");
    }
    //#endregion PopupChromePaintTests
}
//#endregion TextEditor
