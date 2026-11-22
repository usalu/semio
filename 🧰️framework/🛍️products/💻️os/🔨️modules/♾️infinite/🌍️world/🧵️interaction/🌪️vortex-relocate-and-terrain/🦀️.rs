//! 🌪️ World3d vortex arrows, relocate drag, catalogue drop preview, environment helpers, terrain sync and placeholder mesh steps.

use super::*;
use semio_os_infinite_world_bridge::{srgb_byte_to_linear, world_units_per_pixel};

/// 🌀️ Selects the vortex pointer lane for brush tools and vertex selection on a vortex scene.
pub fn world3d_vortex_pointer_mode(utility: &str, granularity: &str, has_vortices: bool) -> bool {
    utility == "brush" || (utility == "select" && granularity == "vertex" && has_vortices)
}

//#region PlaceholderMeshAuthority
//#region Terrain
//#region Environment
//#region WorldRelocateGesture
//#region VortexArrow
const VORTEX_ARROW_SHAFT_MESH: &str = "cylinder";

const VORTEX_ARROW_HEAD_MESH: &str = "cone";

pub struct VortexArrowLayout {
    point_radius: f32,
    shaft_radius: f32,
    shaft_length: f32,
    head_length: f32,
    shaft_center: [f32; 3],
    head_base: [f32; 3],
    pub rotation: [f32; 4],
}

fn quat_normalize_f32(quat: [f32; 4]) -> [f32; 4] {
    let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
    if len < 1e-9 {
        [0.0, 0.0, 0.0, 1.0]
    } else {
        [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
    }
}

fn quat_from_unit_vectors(from: Vec3, to: Vec3) -> [f32; 4] {
    let from = from.normalize();
    let to = to.normalize();
    let r = from.dot(to) + 1.0;
    let quat = if r < 0.000_001 {
        if from.x.abs() > from.z.abs() {
            [-from.y, from.x, 0.0, r]
        } else {
            [0.0, -from.z, from.y, r]
        }
    } else {
        let cross = from.cross(to);
        [cross.x, cross.y, cross.z, r]
    };
    quat_normalize_f32(quat)
}

fn vortex_unit_direction(direction: Option<[f64; 3]>) -> Vec3 {
    let dir = direction.map_or(Vec3::new(0.0, 0.0, -1.0), |value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32));
    if dir.dot(dir) < 1e-12 {
        Vec3::new(0.0, 0.0, -1.0)
    } else {
        dir.normalize()
    }
}

fn vortex_arrow_layout(position: [f64; 3], direction: Option<[f64; 3]>, radius: f32, display_direction: Option<&str>) -> VortexArrowLayout {
    let dir = vortex_unit_direction(direction);
    let pos = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
    let arrow_length = radius;
    let head_length = radius * 0.28;
    let shaft_length = (arrow_length - head_length).max(radius * 0.2);
    let shaft_radius = radius * 0.055;
    let point_radius = radius * 0.18;
    let outward = !display_direction.is_some_and(|mode| mode == "inwards");
    let rotation = quat_from_unit_vectors(Vec3::new(0.0, 1.0, 0.0), dir);
    let (shaft_center, head_base) = if outward { (pos.add(dir.scale(shaft_length * 0.5)), pos.add(dir.scale(shaft_length))) } else { (pos.sub(dir.scale(head_length + shaft_length * 0.5)), pos.sub(dir.scale(head_length))) };
    VortexArrowLayout { point_radius, shaft_radius, shaft_length, head_length, shaft_center: shaft_center.to_array(), head_base: head_base.to_array(), rotation }
}

fn ensure_primitive_mesh(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, mesh_key: &str) {
    if scene_state.meshes.contains_key(mesh_key) {
        return;
    }
    begin_world_placeholder_mesh(session_state, authority_state, retirement_state, scene_state, mesh_key, WorldPlaceholderKind::resolve(mesh_key));
}

pub fn append_vortex_arrow_draws(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, gpu: &mut World3dBuildContext, extra_draws: &mut Vec<SceneDraw3d>) {
    if scene_state.vortices.is_empty() {
        return;
    }
    let mut point_instances = Vec::new();
    let mut shaft_instances = Vec::new();
    let mut head_instances = Vec::new();
    for vortex in &scene_state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let layout = vortex_arrow_layout(position, vortex.direction, radius, vortex.display_direction.as_deref());
        let color = parse_color(vortex.color.as_deref().unwrap_or("#38bdf8"));
        let hovered = selection_state.hovered_vortex_id.as_deref() == Some(vortex.full_id.as_str());
        let id = vortex.full_id.clone();
        point_instances.push(Instance3d { component_source: None,
            id: format!("{id}:point"),
            model: Instance3d::model_from_trs([position[0] as f32, position[1] as f32, position[2] as f32], [0.0, 0.0, 0.0, 1.0], [layout.point_radius, layout.point_radius, layout.point_radius]),
            color,
            selected: false,
            hovered,
         material: Default::default() });
        shaft_instances.push(Instance3d { component_source: None,
            id: format!("{id}:shaft"),
            model: Instance3d::model_from_trs(layout.shaft_center, layout.rotation, [layout.shaft_radius * 2.0, layout.shaft_length, layout.shaft_radius * 2.0]),
            color,
            selected: false,
            hovered,
         material: Default::default() });
        head_instances.push(Instance3d { component_source: None, id: format!("{id}:head"), model: Instance3d::model_from_trs(layout.head_base, layout.rotation, [layout.shaft_radius * 3.6, layout.head_length, layout.shaft_radius * 3.6]), color, selected: false, hovered , material: Default::default() });
    }
    ensure_primitive_mesh(session_state, authority_state, retirement_state, scene_state, "vortex-marker");
    ensure_primitive_mesh(session_state, authority_state, retirement_state, scene_state, VORTEX_ARROW_SHAFT_MESH);
    ensure_primitive_mesh(session_state, authority_state, retirement_state, scene_state, VORTEX_ARROW_HEAD_MESH);
    for (mesh_key, instances) in [("vortex-marker", point_instances), (VORTEX_ARROW_SHAFT_MESH, shaft_instances), (VORTEX_ARROW_HEAD_MESH, head_instances)] {
        let mesh_version = live_world3d_mesh_version(scene_state, mesh_key);
        if let Some(mesh) = scene_state.meshes.get(mesh_key).copied() {
            gpu.ensure_mesh(mesh_key, mesh_version, mesh);
            extra_draws.push(SceneDraw3d { mesh_key: mesh_key.into(), mesh_version, instances, shadow_role: Default::default() });
        }
    }
}
//#endregion VortexArrow

#[cfg(any(test, feature = "testkit"))]
pub fn pick_vortex_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for vortex in &scene_state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
        let min = [center.x - radius, center.y - radius, center.z - radius];
        let max = [center.x + radius, center.y + radius, center.z + radius];
        if let Some(distance) = ray_aabb_slab(origin, dir, min, max) {
            if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                best = Some((distance, vortex.full_id.clone()));
            }
        }
    }
    best.map(|(_, id)| id)
}

/// 🖼️ Ray-vs-quad hit test against visible reference-image planes, returning the closest hit
/// reference's authored `id`. References are flat rectangles lying in the local XY plane (normal +Z,
/// width along X, height along Y) centered at `origin` — matching this renderer's Z-up convention.
#[cfg(any(test, feature = "testkit"))]
pub(super) fn pick_reference_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let plane_normal = Vec3::new(0.0, 0.0, 1.0);
    let mut best: Option<(f32, String)> = None;
    for reference in &scene_state.references {
        if reference.hidden.unwrap_or(false) || reference.locked.unwrap_or(false) {
            continue;
        }
        let Some(url) = reference.url.as_deref() else {
            continue;
        };
        let plane_position = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let plane_origin = Vec3::new(plane_position[0] as f32, plane_position[1] as f32, plane_position[2] as f32);
        let width = world_reference_width(reference);
        let image_aspect = reference_image_aspect(scene_state, url);
        let height = width / image_aspect.max(0.01);
        let Some(hit) = ray_plane_point(origin, dir, plane_origin, plane_normal) else {
            continue;
        };
        let offset = hit.sub(plane_origin);
        if offset.x.abs() > width * 0.5 || offset.y.abs() > height * 0.5 {
            continue;
        }
        let distance = origin.sub(hit).length();
        if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
            best = Some((distance, reference.id.as_deref().unwrap_or(url).to_string()));
        }
    }
    best.map(|(_, id)| id)
}

/// 📍️ One instance's own world ORIGIN — the translation column of its model matrix. Production since
/// the relocate gesture landed: `world3d_begin_relocate_drag` needs the grabbed object's origin to
/// apply React's `origin + (to - from)` travel (`world3dRelocateDispatchArgsV1`).
pub(super) fn object_world_position(scene_state: &WorldSceneState, object_id: &str) -> Option<[f32; 3]> {
    object_world_instance(scene_state, object_id).map(|(_, model)| {
        let translation = model.cols[3];
        [translation[0], translation[1], translation[2]]
    })
}

/// 🧊️ The mesh key and model matrix of the instance carrying `object_id`.
fn object_world_instance<'a>(scene_state: &'a WorldSceneState, object_id: &str) -> Option<(&'a str, Mat4)> {
    scene_state.draws.iter().find_map(|draw| draw.instances.iter().find(|instance| instance.id == object_id).map(|instance| (draw.mesh_key.as_str(), instance.model)))
}

#[cfg(any(test, feature = "testkit"))]
pub(super) fn update_dragged_instance_position(scene_state: &mut WorldSceneState, object_id: &str, position: [f32; 3]) {
    for draw in &mut scene_state.draws {
        for instance in &mut draw.instances {
            if instance.id == object_id {
                instance.model.cols[3] = [position[0], position[1], position[2], 1.0];
            }
        }
    }
}

/// 📏️ `GUMBALL_TRANSFORM_EPSILON` (`🌐️World3dHost/🟦️.tsx:2496`) — a drag that lands within this of
/// where it started writes no document edit.
const WORLD_RELOCATE_EPSILON: f64 = 1e-6;

/// 🚚️ Which object a relocate press grabs — React `world3dRelocateDragTargetV1`
/// (`🌐️World3dHost/🟦️.tsx:4726`). A press on empty canvas relocates the first selected object; a press
/// with no selection relocates what it landed on; a press INSIDE a selection relocates that object,
/// and a press on something outside the selection relocates nothing (it is a selection gesture, and
/// `clearSelection` stays the way to drop a selection).
pub fn world3d_relocate_drag_target(pressed_id: Option<&str>, selected_ids: &[String]) -> Option<String> {
    let Some(pressed_id) = pressed_id.filter(|id| !id.is_empty()) else {
        return selected_ids.first().cloned();
    };
    if selected_ids.is_empty() {
        return Some(pressed_id.to_string());
    }
    selected_ids.iter().any(|id| id == pressed_id).then(|| pressed_id.to_string())
}

/// 🚚️ The ONE `worldRelocate` payload a finished relocate drag commits — React
/// `world3dRelocateDispatchArgsV1` (`🌐️World3dHost/🟦️.tsx:4738`). The ground travel `from → to` is
/// added to the grabbed object's own origin and snapped exactly like a catalogue drop; a drag that
/// lands within [`WORLD_RELOCATE_EPSILON`] of where it started answers `None`, so a click without
/// travel never writes a document edit.
pub fn world3d_relocate_dispatch_args(session: &World3dRelocateSession, to: [f32; 3], grid_snap_enabled: bool, grid_factor: f64) -> Option<[f64; 3]> {
    let moved = [0, 1, 2].map(|axis| session.origin[axis] + (to[axis] - session.from[axis]));
    let mut position = snap_world_point_to_grid(moved, grid_snap_enabled, grid_factor);
    position[session.plane.axis()] = f64::from(session.origin[session.plane.axis()]);
    let unchanged = (0..3).all(|axis| (position[axis] - f64::from(session.origin[axis])).abs() < WORLD_RELOCATE_EPSILON);
    (!unchanged).then_some(position)
}

/// 🚚️ Whether this surface is in React's `relocateMode` — `activeUtility === "worldRelocate"`
/// (`🌐️World3dHost/🟦️.tsx:5656`).
pub fn world3d_relocate_mode(session_state: &WorldSessionState) -> bool {
    session_state.active_utility == "worldRelocate"
}

/// 🚚️ Arms a relocate drag at a primary press. The pressed object is the one this surface's own hover
/// authority last resolved (`local_hover_id`) — the wgpu stand-in for the r3f pointer event's own
/// `object` target, which is the same object by construction since the hover cursor resolves every
/// move before the press lands. Answers `false` when the press grabs nothing, which leaves the press
/// to the ordinary selection route.
pub fn world3d_begin_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32) -> bool {
    if !world3d_relocate_mode(session_state) {
        return false;
    }
    let Some(object_id) = world3d_relocate_drag_target(selection_state.local_hover_id.as_deref(), &selection_state.selected_ids) else {
        return false;
    };
    let Some(origin) = object_world_position(scene_state, &object_id) else {
        return false;
    };
    let plane = World3dRelocatePlane::from_projection(camera_state.projection_spec);
    let Some(from) = world3d_axis_plane_pick(session_state, camera_state, x, y, plane.axis(), origin[plane.axis()]) else {
        return false;
    };
    let Some((mesh_key, preview_model)) = object_world_instance(scene_state, &object_id).map(|(mesh_key, model)| (mesh_key.to_owned(), model)) else {
        return false;
    };
    gumball_state.relocate = Some(World3dRelocateSession { object_id, origin, from, plane, mesh_key, preview_model });
    true
}

/// 🚚️ Moves the relocate ghost: the grabbed instance's own mesh and model, translated along the plane captured at the press. Nothing is dispatched until the release.
pub fn world3d_update_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32) -> bool {
    let Some(session) = gumball_state.relocate.as_mut() else {
        return false;
    };
    let axis = session.plane.axis();
    let position = world3d_axis_plane_pick(session_state, camera_state, x, y, axis, session.origin[axis]).and_then(|to| world3d_relocate_dispatch_args(session, to, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor)).unwrap_or_else(|| session.origin.map(f64::from));
    session.preview_model.cols[3] = [position[0] as f32, position[1] as f32, position[2] as f32, 1.0];
    true
}

/// 🚚️ Closes the relocate drag and answers the position to commit, or `None` for an abort or a drag
/// that never travelled. `to` is `None` for Escape and for a cancelled pointer — React's
/// `endRelocateDrag(null)`, which drops the ghost and dispatches nothing.
pub fn world3d_end_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, to: Option<(f32, f32)>) -> Option<(String, [f64; 3])> {
    let session = gumball_state.relocate.take()?;
    gumball_state.catalogue_drop_preview = None;
    let (x, y) = to?;
    let axis = session.plane.axis();
    let point = world3d_axis_plane_pick(session_state, camera_state, x, y, axis, session.origin[axis])?;
    let position = world3d_relocate_dispatch_args(&session, point, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor)?;
    Some((session.object_id, position))
}

/// ⎋️ Escape drops a live relocate drag and dispatches nothing — React binds it as a CAPTURE-phase
/// `keydown` while `relocateMode` is on (`🌐️World3dHost/🟦️.tsx:6875`). Answers whether a drag was
/// actually cancelled, so the caller can stop the key there exactly as React calls
/// `event.stopPropagation()` only when `endRelocateDrag` returns true.
pub fn world3d_cancel_relocate_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState) -> bool {
    if gumball_state.relocate.is_none() {
        return false;
    }
    world3d_end_relocate_drag(session_state, camera_state, gumball_state, scene_state, None);
    true
}

/// 🚚️ Whether a relocate drag is live on this surface.
pub fn world3d_relocate_active(gumball_state: &WorldGumballState) -> bool {
    gumball_state.relocate.is_some()
}

/// 🚚️ The `worldRelocate` plan a finished relocate drag publishes.
pub(crate) fn plan_world3d_relocate(session_state: &WorldSessionState, generation: u64, object_id: &str, position: [f64; 3]) -> Option<WorldInteractionPlan> {
    let mut plan = WorldInteractionPlan::new(session_state.interaction_revision, generation);
    let controller = plan.push_string(&session_state.controller_id)?;
    let surface = plan.push_string(&session_state.surface_id)?;
    let object = plan.push_string(object_id)?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::Relocate, strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None], numbers: [position[0], position[1], position[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
    plan.push_action(action).then_some(plan)
}
//#endregion WorldRelocateGesture

fn snap_world_point_to_grid(point: [f32; 3], grid_snap_enabled: bool, grid_factor: f64) -> [f64; 3] {
    if !grid_snap_enabled || grid_factor <= 0.0 {
        return [point[0] as f64, point[1] as f64, point[2] as f64];
    }
    let snap = |value: f32| (value as f64 / grid_factor).round() * grid_factor;
    [snap(point[0]), snap(point[1]), snap(point[2])]
}

/// 👻️ Updates the live catalogue-drop ghost for a wgpu `World3d` pane under the pointer.
pub fn world3d_update_catalogue_drop_preview(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, client_x: f32, client_y: f32, object_kind: &str, mesh_url: Option<&str>) {
    let Some(hit) = world3d_ground_plane_pick(session_state, camera_state, client_x, client_y, 0.0) else {
        gumball_state.catalogue_drop_preview = None;
        return;
    };
    let origin = snap_world_point_to_grid(hit, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor);
    gumball_state.catalogue_drop_preview = Some(WorldCatalogueDropPreviewRecord { object_kind: object_kind.to_string(), mesh_url: mesh_url.filter(|url| !url.is_empty()).map(str::to_string), origin });
}

/// 👻️ Clears the catalogue-drop ghost on one `World3d` pane.
pub fn world3d_clear_catalogue_drop_preview(gumball_state: &mut WorldGumballState) {
    gumball_state.catalogue_drop_preview = None;
}

/// 🎯️ Grid-snapped ground origin for committing a catalogue drop on a wgpu `World3d` pane.
pub fn world3d_catalogue_drop_origin(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, client_x: f32, client_y: f32) -> Option<[f64; 3]> {
    world3d_ground_plane_pick(session_state, camera_state, client_x, client_y, 0.0).map(|hit| snap_world_point_to_grid(hit, scene_state.lod.grid_snap_enabled, scene_state.lod.grid_factor))
}

pub fn preview_scale(scale: Option<&serde_json::Value>) -> [f32; 3] {
    match scale {
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0) as f32, values[1].as_f64().unwrap_or(1.0) as f32, values[2].as_f64().unwrap_or(1.0) as f32],
        _ => [1.0, 1.0, 1.0],
    }
}

/// 👻️ Mesh key for `BrushPreviewGhost`: the real GLB's resolved id when a `meshUrl` is given
/// (loaded lazily, same as any other mesh), else the shared "box" primitive fallback ghost.
pub fn brush_preview_mesh_id(mesh_url: Option<&str>) -> String {
    mesh_url.map_or_else(|| "box".to_string(), mesh_id_from_url)
}

/// 👻️ The key a GHOST actually draws through. React's `BrushPreviewGhost` renders a translucent unit
/// box until its GLB resolves and the real mesh after — but React's ghost owns a separate three.js
/// object, while here every draw shares ONE mesh table keyed by mesh id. Minting the ghost's box
/// under a `mesh:…` id the scene is still waiting on would make that id resident, and
/// `offer_missing_mesh_fetches` offers a url only while its key is NOT resident — the ghost would
/// permanently suppress the fetch for the mesh it is standing in for. So a ghost over an unresolved
/// url draws through the shared `box` primitive instead, and switches to the real id the frame its
/// mesh lands.
pub fn ghost_mesh_id(scene_state: &WorldSceneState, mesh_url: Option<&str>) -> String {
    let id = brush_preview_mesh_id(mesh_url);
    if scene_state.meshes.contains_key(&id) || !scene_mesh_awaits_its_asset(scene_state, &id) {
        return id;
    }
    "box".to_string()
}

pub fn append_box_wireframe(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], orientation: [f64; 4], scale: [f64; 3], color: [f32; 4]) {
    let corners = [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]];
    let model = Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [orientation[0] as f32, orientation[1] as f32, orientation[2] as f32, orientation[3] as f32], [scale[0] as f32, scale[1] as f32, scale[2] as f32]);
    let world_corners: Vec<[f32; 3]> = corners.iter().map(|corner| model.transform_point(Vec3::new(corner[0], corner[1], corner[2])).to_array()).collect();
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    for (a, b) in edges {
        lines.push(LineVertex3d { position: world_corners[a], color });
        lines.push(LineVertex3d { position: world_corners[b], color });
    }
}

/// 🌞️ Resolves React's ambient plus configured-sun branch into one backend-neutral pass record.
pub fn environment_scene_lighting(environment: &WorldEnvironmentRecord) -> SceneLighting3d {
    let ambient = environment.ambient.as_ref();
    let sun = environment.sun.as_ref();
    let ambient_rgba = parse_color(ambient.and_then(|light| light.color.as_deref()).unwrap_or("#ffffff"));
    let sun_rgba = parse_color(sun.and_then(|light| light.color.as_deref()).unwrap_or("#ffffff"));
    SceneLighting3d {
        ambient_color: ambient_rgba[..3].try_into().expect("fixed rgba prefix"),
        ambient_intensity: ambient.and_then(|light| light.intensity).unwrap_or(1.15) as f32,
        sun_color: sun_rgba[..3].try_into().expect("fixed rgba prefix"),
        sun_intensity: sun.and_then(|light| light.intensity).unwrap_or(0.85) as f32,
        sun_enabled: sun.and_then(|light| light.enabled) == Some(true),
    }
}

pub fn environment_authored_material(environment: &WorldEnvironmentRecord) -> Option<SceneAuthoredMaterial3d> {
    let material = environment.material.as_ref()?;
    let color = parse_color(material.color.as_deref()?);
    let scene = environment_scene_material(environment);
    Some(SceneAuthoredMaterial3d {
        base_color: color,
        emissive: [scene.emissive[0] * scene.emissive_intensity, scene.emissive[1] * scene.emissive_intensity, scene.emissive[2] * scene.emissive_intensity],
        metalness: scene.metalness,
        roughness: scene.roughness,
        alpha: if color[3] < 1.0 { SceneMaterialAlpha3d::Blend } else { SceneMaterialAlpha3d::Opaque },
        alpha_cutoff: 0.5,
        double_sided: false,
        preserve_vertex_color: false,
        base_color_texture: None,
        texture_sampler: Default::default(),
        metallic_roughness_texture:None,normal_texture:None,occlusion_texture:None,emissive_texture:None,additional_texture_samplers:[Default::default();4],
    normal_scale:[1.0;2],occlusion_strength:1.0,})
}

/// 🖼️ Resolves the canvas clear color from `environment.background`, falling back to the ambient
/// theme clear color when absent or `"transparent"` (mirrors `isTransparentWorldBackground`).
pub fn environment_clear_color(environment: &WorldEnvironmentRecord, theme_clear: Rgba) -> Rgba {
    environment_explicit_clear_color(environment).unwrap_or(theme_clear)
}

pub fn environment_explicit_clear_color(environment: &WorldEnvironmentRecord) -> Option<Rgba> {
    let background = environment.background.as_deref()?;
    if background.eq_ignore_ascii_case("transparent") {
        return None;
    }
    let [r, g, b, a] = parse_color(background);
    Some(Rgba::new(r, g, b, a))
}
//#endregion Environment

pub fn step_world3d_terrain(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState,grant:semio_framework_value::retained_clone::RetainedCloneGrant,context:&mut semio_framework_job::StepContext<'_>)->WorldTerrainMeshProgress {
    if context.is_cancelled() {if let Some(cursor)=terrain_state.terrain_build.as_mut(){cursor.faulted=Some(WorldDynamicFault::Canceled);}scene_state.dynamic_fault=Some(WorldDynamicFault::Canceled);return WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Pending,ownership:Default::default()};}
    if scene_state.dynamic_fault==Some(WorldDynamicFault::Canceled) {return step_world3d_terrain_retirement(session_state, terrain_state,grant,context);}
    let mut result=WorldTerrainMeshProgress{step:WorldTerrainMeshPublicationStep::Idle,ownership:Default::default()};
    if context.should_yield() || grant.maximum_items==0 {result.step=WorldTerrainMeshPublicationStep::Pending;return result;}
    if terrain_state.terrain_build.is_none() && terrain_state.terrain_pending.is_some() {
        match begin_world_terrain_pending(session_state, terrain_state,grant,context.is_cancelled()) {
            Ok(progress)=>{context.set_stage("terrainMeshAdmission");context.consume_fuel(1);result.step=WorldTerrainMeshPublicationStep::Pending;result.ownership=progress;return result;}
            Err(fault)=>{result.step=WorldTerrainMeshPublicationStep::Fault(fault);return result;}
        }
    }
    let Some(mut cursor)=terrain_state.terrain_build.take() else{return result};
    if terrain_state.terrain_style_pending() || cursor.terrain_revision != terrain_state.terrain_revision || !terrain_state.terrain_visible_tiles.contains(&(cursor.z,cursor.x,cursor.y)) {
        cursor.faulted = Some(WorldDynamicFault::Canceled);
    } else if scene_state.dynamic_blocked_mesh.is_some() || scene_state.dynamic_mesh_close.is_some() {
        terrain_state.terrain_build = Some(cursor);
        result.step = WorldTerrainMeshPublicationStep::Pending;
        return result;
    }
    match cursor.step(terrain_state.terrain_revision,terrain_state.terrain_revision,grant,context) {
        Ok((step,progress))=>{
            result.ownership=progress;
            match step {
                WorldTerrainMeshStep::Pending=>{terrain_state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Pending;}
                WorldTerrainMeshStep::Ready(key,lease)=>{
                    if let Err(rejected)=publish_world3d_mesh_lease(interaction_state, scene_state,key,lease) {
                        cursor.faulted=Some(rejected.fault);retain_rejected_world_mesh(session_state, authority_state, scene_state,rejected);
                    }
                    terrain_state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Pending;
                }
                WorldTerrainMeshStep::Complete(tile)=>{terrain_state.terrain_built_tiles.insert(tile);result.step=WorldTerrainMeshPublicationStep::Complete;}
                WorldTerrainMeshStep::Fault(WorldDynamicFault::Canceled)=>{}
                WorldTerrainMeshStep::Fault(fault)=>{mark_world_dynamic_fault(session_state, authority_state, scene_state,fault);result.step=WorldTerrainMeshPublicationStep::Fault(fault);}
            }
        }
        Err(fault)=>{terrain_state.terrain_build=Some(cursor);result.step=WorldTerrainMeshPublicationStep::Fault(fault);}
    }
    result
}

/// 🏔️ GPU-free half of `sync_terrain`: asks `TerrainSessionCore` which DEM tiles are visible for
/// the current camera, evicts (CPU-side) tiles that scrolled out of view, admits byte-fetches for
/// tiles not yet uploaded (see `reserve_terrain_tile_fetch`), and builds/caches one vertex-coloured
/// mesh per tile whose elevation data is already available. Returns the tiles to draw this frame
/// plus the mesh keys the caller must evict from the GPU.
pub fn sync_terrain_state(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState, assets_state: &mut WorldAssetsState, camera: &Camera3d) -> (Vec<TerrainTileDraw>, Vec<String>) {
    let Some((tile_url_template, min_zoom, max_zoom)) = terrain_state.tile_source() else {
        return (Vec::new(), Vec::new());
    };
    let tile_url_template = tile_url_template.to_owned();
    let camera_json = json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
        "minZoom": min_zoom,
        "maxZoom": max_zoom,
    })
    .to_string();
    let visible_json = terrain_state.terrain_session.visible_terrain_tiles_json(&camera_json);
    let visible_rows: Vec<TerrainVisibleTileRow> = serde_json::from_str(&visible_json).unwrap_or_default();
    let visible_set: HashSet<(u32, u32, u32)> = visible_rows.iter().map(|row| (row.z, row.x, row.y)).collect();

    let stale: Vec<(u32, u32, u32)> = terrain_state.terrain_visible_tiles.iter().copied().filter(|key| !visible_set.contains(key)).collect();
    let mut evicted_mesh_keys = Vec::new();
    for (z, x, y) in stale {
        let mesh_key = terrain_tile_mesh_key(&session_state.surface_id, z, x, y);
        if scene_state.meshes.contains_key(&mesh_key) && !retire_world_mesh(scene_state, &mesh_key) {
            continue;
        }
        terrain_state.forget_tile((z, x, y));
        terrain_state.terrain_visible_tiles.remove(&(z, x, y));
        evicted_mesh_keys.push(mesh_key);
    }
    terrain_state.pending_terrain_tile_urls.retain(|_, tile| visible_set.contains(tile));
    terrain_state.terrain_tile_misses.retain(|tile| visible_set.contains(tile));
    terrain_state.terrain_visible_tiles.extend(visible_set);

    let mut tile_draws = Vec::new();
    for TerrainVisibleTileRow { z, x, y } in visible_rows {
        if !terrain_family_visible(terrain_state, (z, x, y)) {
            if !terrain_state.terrain_session.has_elevation_tile(z, x, y) {
                reserve_terrain_tile_fetch(session_state, terrain_state, assets_state, &tile_url_template, (z, x, y));
            } else if terrain_state.terrain_build.is_none() && terrain_state.terrain_pending.is_none() && scene_state.dynamic_mesh_close.is_none() && scene_state.dynamic_blocked_mesh.is_none() && session_state.snapshot_fault.is_none() {
                if let Some(mesh) = terrain_state.terrain_session.terrain_tile_mesh(z, x, y) {
                    let mesh_payload = TerrainTileMeshPayload { positions: mesh.positions, normals: mesh.normals, indices: mesh.indices, uvs: mesh.uvs };
                    let next = scene_state.placeholder_generation.checked_add(1);
                    if let Some(next) = next {
                        scene_state.placeholder_generation=next;
                        terrain_state.terrain_pending=Some(((z,x,y),mesh_payload,next,terrain_state.terrain_revision,terrain_state.terrain_revision));
                    } else {
                        mark_world_dynamic_fault(session_state, authority_state, scene_state, WorldDynamicFault::StaleToken);
                    }
                }
            }
            continue;
        }
        let mesh_key = terrain_tile_mesh_key(&session_state.surface_id, z, x, y);
        if !scene_state.meshes.contains_key(&mesh_key) {
            continue;
        }
        let mesh_version = live_world3d_mesh_version(scene_state, &mesh_key);
        tile_draws.push(TerrainTileDraw { tile: (z, x, y), mesh_key, mesh_version });
    }
    (tile_draws, evicted_mesh_keys)
}

/// 🏔️ Per-frame terrain sync entry point used by `render_world_3d` — see `sync_terrain_state` for
/// the (unit-testable) tile-visibility/meshing/fetch-admission logic this wraps with GPU upload and
/// eviction calls.
pub fn sync_terrain(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, terrain_state: &mut WorldTerrainState, assets_state: &mut WorldAssetsState, gpu: &mut World3dBuildContext, camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) -> Vec<SceneDraw3d> {
    let (tile_draws, evicted_mesh_keys) = sync_terrain_state(session_state, authority_state, scene_state, terrain_state, assets_state, camera);
    for key in evicted_mesh_keys {
        gpu.evict_mesh(&key);
    }
    let mut overlay_build_available = true;
    tile_draws
        .into_iter()
        .filter_map(|tile| {
            let mesh = scene_state.meshes.get(&tile.mesh_key)?;
            gpu.ensure_mesh(&tile.mesh_key, tile.mesh_version, *mesh);
            if gpu.has_mesh_request(&tile.mesh_key, tile.mesh_version) {
                let needs_build = terrain_state.overlay_tile_needs_build(tile.tile);
                if needs_build && !overlay_build_available {
                    gpu.request_cursor_wake();
                } else {
                    overlay_build_available &= !needs_build;
                    append_terrain_overlay_lines(terrain_state, tile.tile, camera, viewport, lines);
                }
            }
            Some(SceneDraw3d {
                mesh_key: tile.mesh_key.clone(),
                mesh_version: tile.mesh_version,
                instances: vec![Instance3d { component_source: None, id: format!("terrain-{}", tile.mesh_key), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }],
                shadow_role: Default::default(),
            })
        })
        .collect()
}

/// 📍 Appends depth-tested geographic overlay lines for a resident terrain tile.
pub fn append_terrain_overlay_lines(terrain: &mut WorldTerrainState, tile: (u32, u32, u32), camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) {
    let Some(overlays) = terrain.tile_overlays(tile) else { return };
    let color = |[r, g, b]: [u8; 3]| [srgb_byte_to_linear(r), srgb_byte_to_linear(g), srgb_byte_to_linear(b), 1.0];
    let bias = terrain_overlay_bias(tile.0);
    for (paths, color) in [(&overlays.regions, color([0x4f, 0xd4, 0xe8])), (&overlays.routes, color([0xf7, 0xf4, 0xe8]))] {
        lines.extend(paths.iter().flat_map(|path| path.segments.iter().flatten()).map(|[x, y, z]| LineVertex3d { position: [*x, *y, z + bias], color }));
    }
    let [forward, right, up] = terrain_marker_axes(camera);
    let color = color([0xf8, 0xd9, 0x4e]);
    for marker in &overlays.positions {
        let (tip, pixel) = terrain_marker_tip(camera, viewport, marker.point, bias, forward);
        let shoulder = tip.add(up.scale(7.0 * pixel));
        let points = [tip, shoulder.add(right.scale(4.0 * pixel)), tip.add(up.scale(12.0 * pixel)), shoulder.sub(right.scale(4.0 * pixel)), tip];
        lines.extend(points.windows(2).flatten().map(|point| LineVertex3d { position: point.to_array(), color }));
    }
}

fn terrain_overlay_bias(zoom: u32) -> f32 {
    (40_075_200.0 / (1_u64 << zoom) as f32 * 0.0002).clamp(0.1, 20.0)
}

fn terrain_marker_axes(camera: &Camera3d) -> [Vec3; 3] {
    let forward = camera.target.sub(camera.position).normalize();
    let right = forward.cross(camera.up);
    let right = if right.length() > 1e-6 { right.normalize() } else { Vec3::new(1.0, 0.0, 0.0) };
    [forward, right, right.cross(forward)]
}

fn terrain_marker_tip(camera: &Camera3d, viewport: Rect, [x, y, z]: [f32; 3], bias: f32, forward: Vec3) -> (Vec3, f32) {
    let tip = Vec3::new(x, y, z + bias);
    let pixel = world_units_per_pixel(camera, viewport, tip.sub(camera.position).dot(forward));
    (tip, pixel)
}

pub(super) fn pick_terrain_position_marker<'a>(terrain: &'a WorldTerrainState, camera: &Camera3d, viewport: Rect, origin: Vec3, direction: Vec3, terrain_depth: Option<f32>) -> Option<&'a str> {
    let [forward, right, up] = terrain_marker_axes(camera);
    let alignment = direction.dot(forward);
    if alignment <= 1e-6 {
        return None;
    }
    terrain.visible_overlay_tiles()
        .flat_map(|((zoom, _, _), tile)| tile.positions.iter().map(move |marker| (zoom, marker)))
        .filter_map(|(zoom, marker)| {
            let (tip, pixel) = terrain_marker_tip(camera, viewport, marker.point, terrain_overlay_bias(zoom), forward);
            let depth = tip.sub(origin).dot(forward) / alignment;
            if marker.id.is_empty() || marker.id.len() > 256 || !depth.is_finite() || depth < 0.0 || terrain_depth.is_some_and(|ground| ground < depth) {
                return None;
            }
            let offset = origin.add(direction.scale(depth)).sub(tip);
            let height = offset.dot(up) / pixel;
            let width = offset.dot(right).abs() / pixel;
            let half_width = 4.0 * (height / 7.0).min((12.0 - height) / 5.0);
            (width <= half_width + 2.0 && (-2.0..=14.0).contains(&height)).then_some((depth, marker.id.as_str()))
        })
        .min_by(|(depth_a, id_a), (depth_b, id_b)| depth_a.total_cmp(depth_b).then_with(|| id_a.cmp(id_b)))
        .map(|(_, id)| id)
}
//#endregion Terrain

pub fn begin_world_placeholder_mesh(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, key: &str, kind: WorldPlaceholderKind) {
    if scene_state.meshes.contains_key(key) || scene_state.placeholder_build.is_some() || retirement_state.dynamic_retirement.is_some() {
        return;
    }
    scene_state.placeholder_generation = scene_state.placeholder_generation.wrapping_add(1).max(1);
    match WorldPlaceholderMeshCursor::new(key, kind, scene_state.placeholder_generation, session_state.interaction_revision) {
        Ok(cursor) => scene_state.placeholder_build = Some(cursor),
        Err(_) => mark_world_dynamic_fault(session_state, authority_state, scene_state, WorldDynamicFault::ByteCapacity),
    }
}

pub fn step_world_placeholder_mesh(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, authority_state: &mut WorldAuthorityState, scene_state: &mut WorldSceneState, effect:&mut semio_framework_value::RetainedAllocationEffect<'_>) {
    let Some(mut cursor) = scene_state.placeholder_build.take() else { return };
    match cursor.step(effect) {
        WorldPlaceholderMeshStep::Pending => scene_state.placeholder_build = Some(cursor),
        WorldPlaceholderMeshStep::Ready(key,lease,appearance) => {if let Some(appearance)=appearance {let asset=World3dMeshAsset{mesh:lease,appearance};if let Err(rejected)=publish_world3d_inline_asset(session_state, interaction_state, scene_state,key,asset){cursor.owner=WorldPlaceholderOwner::Ready(rejected.value.mesh);cursor.appearance=Some(rejected.value.appearance);cursor.faulted=true;scene_state.placeholder_build=Some(cursor);mark_world_dynamic_fault(session_state, authority_state, scene_state,rejected.fault);}}else{store_mesh(session_state, interaction_state, authority_state, scene_state,key,lease)}},
        WorldPlaceholderMeshStep::Fault => mark_world_dynamic_fault(session_state, authority_state, scene_state, WorldDynamicFault::Closing),
    }
}
//#endregion PlaceholderMeshAuthority
