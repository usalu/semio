//! 🧭️ Node graph and tiled map scenes.

use super::*;

//#region NodeGraph
#[derive(Clone, Debug)]
pub struct NodeGraphSurface {
    pub surface_id: String,
    pub bounds: Rect,
    pub controller_id: String,
    /// 🪟️ The window instance that owns this surface — the ONLY thing that retires it. See
    /// [`NodeGraphSurface`]'s oracle, `🧑‍🎨engine/🧫️fixtures/🧲️engine-surface-retention/🔣️.json`.
    pub window_id: String,
}

//#endregion NodeGraph

//#region TiledMap
#[derive(Clone, Debug)]
pub struct TiledMapSurface {
    pub surface_id: String,
    pub bounds: Rect,
    pub controller_id: String,
    pub selection_method: String,
    pub window_id: String,
}

pub fn query_map_feature_hits(host: &semio_framework_surface::tiled_map::MapHost, method: &str, points: &[(f32, f32)], crossing: bool) -> Vec<String> {
    if method == "lasso" && points.len() >= 3 {
        let payload: Vec<[f64; 2]> = points.iter().map(|(x, y)| [*x as f64, *y as f64]).collect();
        let points_json = serde_json::to_string(&payload).unwrap_or_else(|_| "[]".into());
        semio_framework_os_renderer_wgpu_engine_board::tiled_map::parse_map_feature_hit(&host.features_in_polygon_json(&points_json, crossing))
    } else if points.len() >= 2 {
        let (x0, y0) = points[0];
        let (x1, y1) = points[points.len() - 1];
        let (min_x, max_x) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        semio_framework_os_renderer_wgpu_engine_board::tiled_map::parse_map_feature_hit(&host.features_in_rect_json(min_x as f64, min_y as f64, max_x as f64, max_y as f64, crossing))
    } else {
        Vec::new()
    }
}

/** 🗺️ The map context-menu surface target for a screen-space right-click: the feature
 * under the point as a `ContextMenuHit`, plus the live feature selection grouped per pickable overlay
 * granularity. Port of `onContextMenu` in `🧭️TiledMapHost/🟦️.tsx` — every domain is an overlay
 * layer's granularity, and the groups decide whether the app's `clearSelection` row renders enabled. */
pub fn tiled_map_context_menu_target(surface_id: &str, inner: Rect, x: f32, y: f32) -> (Vec<semio_framework_ui_declaration::ui::ContextMenuHit>, Vec<semio_framework_ui_declaration::ui::ContextMenuSelectionGroup>) {
    #[derive(Deserialize)]
    struct HitRow {
        kind: String,
        id: String,
    }
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    let hit_json = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_host(surface_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
    let hits = serde_json::from_str::<Option<HitRow>>(&hit_json).ok().flatten().map(|row| semio_framework_ui_declaration::ui::ContextMenuHit { domain: row.kind, id: row.id, label: None }).into_iter().collect();
    let selection: Vec<semio_framework_ui_declaration::ui::ContextMenuSelectionGroup> = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_host(surface_id, |host| {
        host.pickable_overlays()
            .filter_map(|layer| layer.granularity.as_deref())
            .filter_map(|granularity| {
                let ids: Vec<String> = host.selected_ids(granularity).map(str::to_owned).collect();
                (!ids.is_empty()).then(|| semio_framework_ui_declaration::ui::ContextMenuSelectionGroup { domain: granularity.into(), ids })
            })
            .collect()
    })
    .unwrap_or_default();
    (hits, selection)
}

/// 🕹️ The framework-owned `"features"` interaction domain a map app addresses, and its feature
/// granularity — the constants React's `🧭️TiledMapHost` spells `MAP_INTERACTION_DOMAIN`/
/// `MAP_FEATURE_GRANULARITY`/`MAP_HOVER_CHANNEL`, and the domain a map app declares
/// (`InteractionDefinition { id: "features", … }`).
const MAP_INTERACTION_DOMAIN: &str = "features";
const MAP_FEATURE_GRANULARITY: &str = "feature";
const MAP_HOVER_CHANNEL: &str = "pointer";

/// 🎯️ `interactionSelect`'s `targets` are JSON TEXT, not a node array — the wire shape
/// `mapFeatureSelectionActionArgs`/`select_feature_action_args` both build.
fn map_feature_targets_json(ids: impl Iterator<Item = impl AsRef<str>>) -> Result<String, semio_framework_ui_action::BoundedActionFault> {
    let rows: Vec<Value> = ids.map(|id| serde_json::json!({ "granularity": MAP_FEATURE_GRANULARITY, "id": id.as_ref() })).collect();
    serde_json::to_string(&rows).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)
}

/// 🕹️ A map pick/marquee release: `interactionSelect` against the `"features"` domain, or
/// `clearSelection` on a miss. Mirrors `emitFeatureSelection` in `🧭️TiledMapHost/🟦️.tsx` — including
/// its hit/miss split, because `next_selection` treats an empty `targets` as a no-op, not a clear.
/// The bespoke `setFeatureSelection` this used to publish was DELETED from the map apps (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): every such dispatch was refused.
fn write_tiled_map_selection(
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
    controller_id: &str,
    surface_id: &str,
    ids: &[String],
    merge: &str,
    method: &str,
    commit: impl FnOnce(),
) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    if ids.is_empty() {
        let action = semio_framework_ui_declaration::ui::tiled_map_actions::CLEAR_SELECTION;
        let bytes = semio_framework_ui_action::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id])?;
        let mut reservation = input.reserve_action(controller_id, action, bytes)?;
        let builder = reservation.builder();
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), surface_id)?;
        builder.end_container()?;
        return reservation.publish_with(commit);
    }
    let targets = map_feature_targets_json(ids.iter())?;
    if targets.len() > semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::StringCredits);
    }
    let action = "interactionSelect";
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id, "domainId", MAP_INTERACTION_DOMAIN, "targets", targets.as_str(), "merge", merge, "method", method])?;
    let mut reservation = input.reserve_action(controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    builder.string(Some("domainId"), MAP_INTERACTION_DOMAIN)?;
    builder.string(Some("targets"), targets.as_str())?;
    builder.string(Some("merge"), merge)?;
    builder.string(Some("method"), method)?;
    builder.end_container()?;
    reservation.publish_with(commit)
}

/// 🐁️ `interactionHover` on the `"pointer"` channel — `mapFeatureHoverActionArgs`'s twin; a cleared
/// hover publishes an EMPTY `targets` array, which is what clears the guest's hover.
fn write_tiled_map_hover(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, controller_id: &str, surface_id: &str, hover: Option<(&str, &str)>, commit: impl FnOnce()) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    let targets = map_feature_targets_json(hover.map(|(_, id)| id).into_iter())?;
    if targets.len() > semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY {
        return Err(semio_framework_ui_action::BoundedActionFault::StringCredits);
    }
    let action = "interactionHover";
    let bytes = semio_framework_ui_action::checked_action_string_bytes(&[controller_id, action, "surfaceId", surface_id, "domainId", MAP_INTERACTION_DOMAIN, "channel", MAP_HOVER_CHANNEL, "targets", targets.as_str()])?;
    let mut reservation = input.reserve_action(controller_id, action, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface_id)?;
    builder.string(Some("domainId"), MAP_INTERACTION_DOMAIN)?;
    builder.string(Some("channel"), MAP_HOVER_CHANNEL)?;
    builder.string(Some("targets"), targets.as_str())?;
    builder.end_container()?;
    reservation.publish_with(commit)
}

pub fn touch_pointer_down(host_id: &str, pointer_id: ui_render::PointerId, x: f64, y: f64) -> GestureVerdict {
    let mut verdict = GestureVerdict::Single;
    mutate_scene_state(host_id, |state| {
        verdict = state.touch_gesture.down(GesturePointer { id: pointer_id.0, x, y });
    });
    verdict
}

pub fn touch_pointer_move(host_id: &str, pointer_id: ui_render::PointerId, x: f64, y: f64) -> GestureVerdict {
    let mut verdict = GestureVerdict::Single;
    mutate_scene_state(host_id, |state| {
        verdict = state.touch_gesture.move_to(GesturePointer { id: pointer_id.0, x, y });
    });
    verdict
}

pub fn touch_pointer_up(host_id: &str, pointer_id: ui_render::PointerId) -> GestureVerdict {
    let mut verdict = GestureVerdict::Single;
    mutate_scene_state(host_id, |state| {
        verdict = state.touch_gesture.up(pointer_id.0);
    });
    verdict
}

pub fn pinch_camera(current: [f64; 3], step: semio_framework::interaction::gesture::PinchStep, viewport: [f64; 2], bounds: ZoomBounds) -> [f64; 3] {
    let camera = apply_pinch_to_camera(Camera2d { x: current[0], y: current[1], zoom: current[2] }, step, viewport, bounds);
    [camera.x, camera.y, camera.zoom]
}

/// 🤏️ Transfers a map's first-touch lane to pinch ownership when contact two lands.
pub fn tiled_map_touch_pointer_down(host_id: &str, inner: Rect, pointer_id: ui_render::PointerId, x: f32, y: f32) -> bool {
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    match touch_pointer_down(host_id, pointer_id, sx, sy) {
        GestureVerdict::Single => false,
        GestureVerdict::PinchBegin(_) => {
            clear_tiled_map_interaction(host_id);
            true
        }
        _ => true,
    }
}

/// 🤏️ Applies one silent map pinch step; the single-pointer lane stays suppressed while latched.
pub fn tiled_map_touch_pointer_move(host_id: &str, inner: Rect, pointer_id: ui_render::PointerId, x: f32, y: f32) -> bool {
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    match touch_pointer_move(host_id, pointer_id, sx, sy) {
        GestureVerdict::Single => false,
        GestureVerdict::Pinch(step) => {
            if let Some(current) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::tiled_map_camera(host_id) {
                let next = pinch_camera(
                    current,
                    step,
                    [inner.w as f64, inner.h as f64],
                    ZoomBounds { min: semio_framework_surface::tiled_map::MAP_CAMERA_ZOOM_MIN, max: semio_framework_surface::tiled_map::MAP_CAMERA_ZOOM_MAX },
                );
                semio_framework_os_renderer_wgpu_engine_board::tiled_map::tiled_map_set_camera_silent(host_id, next);
            }
            true
        }
        _ => true,
    }
}

/// 🏁️ Suppresses pinch lifts and publishes exactly one settled map camera on the final lift.
pub fn tiled_map_touch_pointer_up_into(
    host_id: &str,
    controller_id: &str,
    pointer_id: ui_render::PointerId,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    match touch_pointer_up(host_id, pointer_id) {
        GestureVerdict::Single => Ok(false),
        GestureVerdict::PinchEnd => semio_framework_os_renderer_wgpu_engine_board::tiled_map::tiled_map_publish_camera_into(host_id, controller_id, input).map(|_| true),
        _ => Ok(true),
    }
}

pub fn tiled_map_pointer_down_into(
    owner: &crate::element_contracts::ScenePointerTarget,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    button: i16,
    shift: bool,
    ctrl_or_meta: bool,
    selection_method: &str,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let host_id = owner.host_id.as_str();
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    if button == 0 {
        if selection_method.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
            return Err(semio_framework_ui_action::BoundedActionFault::StringCredits);
        }
        mutate_scene_state(host_id, |state| {
            state.drag = Some(SceneDrag { mode: SceneDragMode::MapMarquee { start_x: sx as f32, start_y: sy as f32, method: selection_method.to_owned(), merge_mode: semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_marquee_mode(shift, ctrl_or_meta).to_owned() } });
            state.map_marquee_points = vec![(sx as f32, sy as f32)];
            state.map_marquee_active = false;
            state.map_interaction_owner = Some(owner.clone());
        });
        semio_framework_os_renderer_wgpu_engine_board::tiled_map::stamp_map_interaction_owner(host_id, owner);
        return Ok(true);
    }
    if button == 1 {
        let published = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_interaction_into(host_id, controller_id, input, semio_framework_surface::tiled_map::MapInteractionIntent::PointerDown { sx, sy, button: 1 })?;
        if published {
            mutate_scene_state(host_id, |state| {
                state.drag = Some(SceneDrag { mode: SceneDragMode::MapPan });
                state.map_interaction_owner = Some(owner.clone());
            });
            semio_framework_os_renderer_wgpu_engine_board::tiled_map::stamp_map_interaction_owner(host_id, owner);
        }
        return Ok(published);
    }
    Ok(false)
}

fn clear_tiled_map_interaction(host_id: &str) {
    let owner = SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let Some(state) = states.get_mut(host_id) else { return None };
        state.drag = None;
        state.map_marquee_points.clear();
        state.map_marquee_active = false;
        state.map_interaction_owner.take()
    });
    if let Some(owner) = owner {
        semio_framework_os_renderer_wgpu_engine_board::tiled_map::retire_map_interaction_owner(&owner);
    }
}

pub fn tiled_map_pointer_move_into(
    owner: &crate::element_contracts::ScenePointerTarget,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    down: bool,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let host_id = owner.host_id.as_str();
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    if down {
        let state = scene_state(host_id);
        if let Some(drag) = &state.drag {
            match &drag.mode {
                SceneDragMode::MapPan => return semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_interaction_into(host_id, controller_id, input, semio_framework_surface::tiled_map::MapInteractionIntent::PointerMove { sx, sy }),
                SceneDragMode::MapMarquee { start_x, start_y, method, .. } => {
                    let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
                    mutate_scene_state(host_id, |state| {
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
        return Ok(true);
    }
    #[derive(Deserialize)]
    struct HoverRow {
        kind: String,
        id: String,
    }
    let hit_json = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_host(host_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
    let hover = serde_json::from_str::<Option<HoverRow>>(&hit_json).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
    let hover_json = hover.as_ref().map(|row| format!("{}:{}", row.kind, row.id)).unwrap_or_else(|| "null".into());
    if scene_state(host_id).map_last_hover_json.as_deref() == Some(hover_json.as_str()) {
        return Ok(false);
    }
    write_tiled_map_hover(input, controller_id, &owner.surface_id, hover.as_ref().map(|row| (row.kind.as_str(), row.id.as_str())), || {
        mutate_scene_state(host_id, |state| {
            state.map_last_hover_json = Some(hover_json);
            state.map_hover_owner = Some(owner.clone());
        });
    })?;
    Ok(true)
}

/// 🛑️ Cancels only the active Map gesture: pan settles its retained camera once, while marquee
/// cancellation is local and cannot publish a selection.
pub fn tiled_map_pointer_cancel_into(host_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let Some(drag) = SCENE_STATE.with(|cell| cell.borrow().get(host_id).and_then(|state| state.drag.clone())) else {
        return Ok(false);
    };
    match drag.mode {
        SceneDragMode::MapPan => {
            let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
            let published = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_interaction_into(host_id, controller_id, input, semio_framework_surface::tiled_map::MapInteractionIntent::PointerUp { sx, sy })?;
            if published {
                clear_tiled_map_interaction(host_id);
            }
            Ok(published)
        }
        SceneDragMode::MapMarquee { .. } => {
            clear_tiled_map_interaction(host_id);
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// 👋️ Publishes one empty Map hover transition from the cached non-null witness.
pub fn tiled_map_pointer_leave_into(host_id: &str, surface_id: &str, controller_id: &str, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    if !SCENE_STATE.with(|cell| cell.borrow().get(host_id).is_some_and(|state| state.map_last_hover_json.as_deref().is_some_and(|hover| hover != "null"))) {
        return Ok(false);
    }
    write_tiled_map_hover(input, controller_id, surface_id, None, || {
        mutate_scene_state(host_id, |state| {
            state.map_last_hover_json = Some("null".into());
            state.map_hover_owner = None;
        });
    })?;
    Ok(true)
}

/// 🎯️ `method` is `"pick"` for a click and the marquee's own method for a drag — the exact
/// split `emitFeatureSelection`'s two call sites make in `🧭️TiledMapHost/🟦️.tsx`.
pub fn tiled_map_pointer_up_into(host_id: &str, surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    let state = scene_state(host_id);
    let Some(drag) = state.drag.as_ref() else {
        return Ok(false);
    };
    match &drag.mode {
        SceneDragMode::MapPan => {
            let published = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_interaction_into(host_id, controller_id, input, semio_framework_surface::tiled_map::MapInteractionIntent::PointerUp { sx, sy })?;
            if published {
                clear_tiled_map_interaction(host_id);
            }
            Ok(published)
        }
        SceneDragMode::MapMarquee { start_x, start_y, method, merge_mode } => {
            let distance = ((sx as f32 - *start_x).powi(2) + (sy as f32 - *start_y).powi(2)).sqrt();
            let mut ids = Vec::new();
            if state.map_marquee_active && distance >= MAP_MARQUEE_THRESHOLD_PX {
                let mut points = state.map_marquee_points.clone();
                if method == "lasso" {
                    points.push((sx as f32, sy as f32));
                } else {
                    points = vec![(*start_x, *start_y), (sx as f32, sy as f32)];
                }
                let crossing = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_marquee_crossing(method, *start_x, sx as f32);
                ids = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_host(host_id, |host| query_map_feature_hits(host, method, &points, crossing)).unwrap_or_default();
            } else if distance < MAP_MARQUEE_THRESHOLD_PX {
                #[derive(Deserialize)]
                struct HitRow {
                    id: String,
                }
                let hit_json = semio_framework_os_renderer_wgpu_engine_board::tiled_map::with_map_host(host_id, |host| host.hit_test_feature_json(sx, sy)).unwrap_or_else(|| "null".into());
                if let Some(hit) = serde_json::from_str::<Option<HitRow>>(&hit_json).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)? {
                    ids.push(hit.id);
                }
            }
            if ids.is_empty() && distance >= MAP_MARQUEE_THRESHOLD_PX && !state.map_marquee_active {
                clear_tiled_map_interaction(host_id);
                return Ok(false);
            }
            let select_method = if state.map_marquee_active && distance >= MAP_MARQUEE_THRESHOLD_PX { method.as_str() } else { "pick" };
            write_tiled_map_selection(input, controller_id, surface_id, &ids, merge_mode, select_method, || {
                clear_tiled_map_interaction(host_id);
            })?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn tiled_map_drag_active(surface_id: &str) -> bool {
    scene_state(surface_id).drag.as_ref().is_some_and(|drag| matches!(drag.mode, SceneDragMode::MapMarquee { .. } | SceneDragMode::MapPan))
}

//#endregion TiledMap
