//! 🧲️ World3d interaction cursors: ray, object and component picking, marquee selection and gumball gestures.
//!
//! Crate `semio-os-infinite-world-cursors`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_os_infinite_world_model::{ROTATE_SELECTION_ACTION_ID, SCALE_SELECTION_ACTION_ID, TRANSLATE_SELECTION_ACTION_ID, WORLD_INTERACTION_BYTE_CAPACITY, WORLD_INTERACTION_ID_BYTE_CAPACITY, WORLD_INTERACTION_MARQUEE_POINT_CAPACITY, WORLD_INTERACTION_MESH_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, world_pick_keys_match, world_pick_target_ray_score, WorldCancelReason, WorldFlatAction, WorldFlatActionKind, WorldInteractionMeshRegistry, WorldInteractionMeshToken, WorldInteractionObjectKind, WorldInteractionObjectRegistry, WorldInteractionObjectSlot, WorldInteractionObjectToken, WorldInteractionPlan, WorldInteractionSpan, WorldInteractionState, WorldInteractionStep, WorldMeshSource, WorldSessionState};
use semio_os_infinite_world_components::{GumballHandle, world3d_declares_action, World3dSceneGumballConfigRecord, WORLD_COMPONENT_TARGET_BYTE_CAPACITY, world_mesh_component_id, world_mesh_triangle, world_mesh_vertex, WorldCameraState, WorldComponentAddress, WorldGumballState, WorldSceneState, WorldSelectionState};
use semio_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{axis_rotate_angle, gumball_extent, gumball_eye, gumball_project_ray_onto_axis, interpolate_mesh_uv, Mat4, Mesh3dField, ray_aabb_slab, ray_plane_point, ray_segment_distance, Vec3};
use semio_ui_theme::geometry::Rect;
use semio_framework::{MergeMode, SelectionMethod};
use semio_ui_viewport::Viewport3dProjectionSpec;

/// 🎛️ The gumball config THIS surface offers: the plugin-authored one when it published one,
/// otherwise the single-mode fallback for its `transformMode`.
///
/// 🩸️ Before ticket 26/09/17 packet W14g nothing on this target read `selection.gumballConfig` at
/// all, and the handle gate was a `transform_mode` string match that admitted the three MOVE axes
/// and the three move planes UNCONDITIONALLY — so a `rotate`-mode gumball still offered translation
/// where React offers only the rings, and a plugin that authored several groups at once (the
/// contract puzzle3d's Move/Rotate relies on) got the single-mode set.
pub fn world3d_gumball_config(gumball_state: &WorldGumballState) -> World3dSceneGumballConfigRecord {
    gumball_state.gumball_config.clone().unwrap_or_else(|| World3dSceneGumballConfigRecord::for_transform_mode(&gumball_state.transform_mode))
}

pub const CLICK_DRAG_THRESHOLD_PX: f32 = 4.0;

pub const PICK_VERTEX_SCREEN_PX: f32 = 14.0;

pub const PICK_EDGE_SCREEN_PX: f32 = 18.0;

pub fn world_pick_rect(session_state: &WorldSessionState) -> Rect {
    if session_state.pick_bounds.w > 0.0 && session_state.pick_bounds.h > 0.0 {
        session_state.pick_bounds
    } else {
        session_state.bounds
    }
}

pub fn render_pick_viewport(session_state: &WorldSessionState) -> Rect {
    session_state.bounds
}

pub fn pointer_in_pick_rect(session_state: &WorldSessionState, x: f32, y: f32) -> Option<(f32, f32, Rect)> {
    let clip = world_pick_rect(session_state);
    if !clip.contains(x, y) {
        return None;
    }
    let viewport = render_pick_viewport(session_state);
    if !viewport.contains(x, y) {
        return None;
    }
    Some((x - viewport.x, y - viewport.y, viewport))
}

/// 🎯️ Intersects the live viewport ray with a horizontal world plane.
pub fn world3d_ground_plane_pick(session: &WorldSessionState, camera: &WorldCameraState, x: f32, y: f32, plane_z: f32) -> Option<[f32; 3]> {
    world3d_axis_plane_pick(session, camera, x, y, 2, plane_z)
}

/// 🎯️ Intersects the live viewport ray with the world plane whose `axis` coordinate equals `value`.
pub fn world3d_axis_plane_pick(session: &WorldSessionState, camera: &WorldCameraState, x: f32, y: f32, axis: usize, value: f32) -> Option<[f32; 3]> {
    let (x, y, viewport) = pointer_in_pick_rect(session, x, y)?;
    let (origin, direction) = ui_scene::math::projection_spec_ray_from_screen(&camera.orbit.to_camera(), camera.projection_spec, x, y, viewport.w, viewport.h);
    let (origin, direction) = (origin.to_array(), direction.to_array());
    let distance = (value - origin[axis]) / direction[axis];
    if direction[axis].abs() < 1e-5 || !distance.is_finite() || distance < 0.0 {
        return None;
    }
    let position = [0, 1, 2].map(|index| origin[index] + direction[index] * distance);
    position.iter().all(|coordinate| coordinate.is_finite()).then_some(position)
}

/// 📍️ Captures a bounded ground point with the explicit window and scene addresses.
pub fn plan_world3d_engagement_pointer(session: &WorldSessionState, camera: &WorldCameraState, intent: &semio_os_infinite_world_model::WorldInteractionIntent) -> Option<WorldInteractionPlan> {
    let (kind, action_id) = if intent.phase == semio_os_infinite_world_model::WorldInteractionPhase::PointerButton {
        (WorldFlatActionKind::EngagementPointerDown, "worldPointerDown")
    } else {
        (WorldFlatActionKind::EngagementPointerMove, "worldPointerMove")
    };
    let window = session.window_instance_id.as_deref()?;
    if !world3d_declares_action(session, action_id)
        || [window, &session.surface_id, &session.controller_id].into_iter().any(|id| id.is_empty() || id.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY)
    {
        return None;
    }
    let [x, y, _] = world3d_ground_plane_pick(session, camera, intent.x, intent.y, 0.0)?;
    let mut plan = WorldInteractionPlan::new(session.interaction_revision, intent.generation);
    let controller = plan.push_string(&session.controller_id)?;
    let surface = plan.push_string(&session.surface_id)?;
    let window = plan.push_string(window)?;
    let mut numbers = [0.0; 10];
    numbers[..5].copy_from_slice(&[x.into(), y.into(), u8::from(intent.shift).into(), u8::from(intent.ctrl).into(), u8::from(intent.meta).into()]);
    plan.push_action(WorldFlatAction { kind, strings: [Some(controller), Some(surface), Some(window), None, None, None, None, None], numbers }).then_some(plan)
}

const _: () = assert!(semio_os_infinite_world_model::WORLD_TERRAIN_STROKE_CAPACITY * 3 + 9 <= semio_ui_action::ACTION_NODE_CAPACITY);

/// 🏜️ Publishes one window-scoped elevation edit with an optional path of integer grid cells.
pub fn publish_world3d_terrain_edit(plan: &WorldInteractionPlan, action: WorldFlatAction, input: &mut semio_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<(), semio_ui_action::BoundedActionFault> {
    let controller = plan.string(action.strings[0].expect("terrain controller span"));
    let action_id = plan.string(action.strings[1].expect("terrain action span"));
    let operation = plan.string(action.strings[2].expect("terrain operation span"));
    let window = plan.string(action.strings[3].expect("terrain window span"));
    let surface = plan.string(action.strings[4].expect("terrain surface span"));
    let samples = plan.terrain_stroke.as_ref().expect("terrain stroke accompanies its action").samples();
    let fields = ["column", "row", "radius", "value"];
    let stroke_key = if samples.len() > 1 { "stroke" } else { "" };
    let bytes = semio_ui_action::checked_action_string_bytes(&[
        controller, action_id, "operation", operation, fields[0], fields[1], fields[2], fields[3],
        "windowId", window, "surfaceId", surface, stroke_key,
    ])?;
    let mut reservation = input.reserve_action(controller, action_id, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("windowId"), window)?;
    builder.string(Some("surfaceId"), surface)?;
    builder.string(Some("operation"), operation)?;
    for (field, value) in fields.into_iter().zip(action.numbers) {
        builder.integer(Some(field), value as i64)?;
    }
    if samples.len() > 1 {
        builder.begin_array(Some(stroke_key))?;
        for &[column, row] in samples {
            builder.begin_array(None)?;
            builder.integer(None, i64::from(column))?;
            builder.integer(None, i64::from(row))?;
            builder.end_container()?;
        }
        builder.end_container()?;
    }
    builder.end_container()?;
    reservation.publish()
}

/// 📮️ Publishes the flat engagement event if its source window and surface still own the action.
pub fn publish_world3d_engagement_pointer(session: &WorldSessionState, selection: &WorldSelectionState, plan: &WorldInteractionPlan, action: WorldFlatAction, input: &mut semio_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_ui_action::BoundedActionFault> {
    let controller = plan.string(action.strings[0].expect("engagement controller span"));
    let surface = plan.string(action.strings[1].expect("engagement surface span"));
    let window = plan.string(action.strings[2].expect("engagement window span"));
    let pane = surface.rsplit('/').next().expect("engagement pane suffix");
    let press = action.kind == WorldFlatActionKind::EngagementPointerDown;
    let action_id = if press { "worldPointerDown" } else { "worldPointerMove" };
    if !selection.engagement_session_active || session.window_instance_id.as_deref() != Some(window)
        || session.surface_id != surface || session.controller_id != controller || !world3d_declares_action(session, action_id)
    {
        return Ok(false);
    }
    let modifiers = ["shiftKey", "ctrlKey", "metaKey"];
    let mut bytes = semio_ui_action::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "windowId", window, "pane", pane, "position"])?;
    if press {
        bytes += semio_ui_action::checked_action_string_bytes(&modifiers)?;
    }
    let mut reservation = input.reserve_action(controller, action_id, bytes)?;
    let builder = reservation.builder();
    builder.begin_object(None)?;
    builder.string(Some("surfaceId"), surface)?;
    builder.string(Some("windowId"), window)?;
    builder.string(Some("pane"), pane)?;
    builder.begin_array(Some("position"))?;
    for value in [action.numbers[0], action.numbers[1], 0.0] {
        builder.number(None, value)?;
    }
    builder.end_container()?;
    if press {
        for (key, value) in modifiers.into_iter().zip(&action.numbers[2..5]) {
            builder.boolean(Some(key), *value != 0.0)?;
        }
    }
    builder.end_container()?;
    reservation.publish()?;
    Ok(true)
}

//#region 🔖️WorldInteractionDomain
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the OS's own bare-board `world`
/// domain — plain (non-component) 3D-world object picking/hover for a world3d window that binds NO
/// app `InteractionDefinition`. Granularity `"item"` targets are `"{surfaceId}/{objectId}"`, split by
/// [`HierarchyProvider::PathDelimited`] — the cross-surface addressing a shared, multi-board OS domain
/// needs (`"surface"` covers a future whole-surface selection, not emitted by pointer picking today).
///
/// Most world3d apps bind their OWN domain instead (`WindowKindDefinition.interactions` /
/// `.window_kind_interactions(...)`) — e.g. CAD's `"cad"` domain with `HierarchyProvider::Flat`. That
/// binding reaches this file only through `World3dScene.domain_id`/`domain_granularity_id` (set by the
/// app's own `render()`, the only party that knows both its `window_kind_id` and which domain it bound
/// there — mirrors `UiTreeNode.interaction_domain`/`PanelTreeBuilder::interaction_domain`, extended to
/// `Scene` surfaces), captured onto `World3dState.bound_domain_id`/`bound_domain_granularity_id` by
/// `sync_world3d_state`. `resolved_domain_id`/`resolved_domain_granularity_id`/`resolved_item_id` below
/// are what every plain pick/hover emit site actually reads — never `WORLD_INTERACTION_DOMAIN_ID`
/// directly, so a bound app domain and this OS domain never both light up for the same click (the
/// two-selection-universe bug this indirection exists to prevent). A bound domain is inherently
/// single-surface-scoped (one app document per window), so its ids are always bare — no `surfaceId/`
/// prefix, unlike `world`'s own multi-surface `PathDelimited` shape.
///
/// Component-level targets retain their original render instance, analytic source and exact label
/// in the same interaction domain as ordinary object targets.
pub const WORLD_INTERACTION_DOMAIN_ID: &str = "world";

pub const WORLD_ITEM_GRANULARITY_ID: &str = "item";

pub const WORLD_ITEM_PATH_DELIMITER: &str = "/";

/// 🪟️ The interaction domain a plain pick/hover on this surface targets: the app-bound domain from
/// `World3dScene.domain_id`, falling back to the OS's own `world` board domain when unbound.
pub fn resolved_domain_id(session_state: &WorldSessionState) -> &str {
    session_state.bound_domain_id.as_deref().unwrap_or(WORLD_INTERACTION_DOMAIN_ID)
}

/// 🎯️ The scene's default granularity for a plain instance target — the app-declared
/// granularity when a domain is bound, else the `world` domain's own `"item"` granularity.
pub fn resolved_domain_granularity_id(session_state: &WorldSessionState) -> &str {
    session_state.bound_domain_granularity_id.as_deref().unwrap_or(WORLD_ITEM_GRANULARITY_ID)
}

/// 🎯️ The TOPOLOGY target a rendered instance stands for.
///
/// ⚖️ A surface renders one channel as many instances (`extrude@solid#0`, `extrude@solid#1`), and the
/// guest's selection/hover is over the CHANNEL (`extrude@solid`). Publishing the render id makes the
/// guest silently drop the target: it is not in its topology. Measured on 6118 —
/// `interactionSelect targets=[{"id":"extrude@solid#0"}]` left `selectedIds` empty across hover,
/// click and marquee (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-world3d-interaction-2026-09-13.md`). React resolves the same way
/// (`world3dInstanceInteractionTarget`), and the fixture
/// `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` is the shared oracle.
pub fn instance_interaction_id<'a>(interaction_state: &'a WorldInteractionState, render_id: &'a str) -> &'a str {
    interaction_state.instance_interaction_ids.get(render_id).map_or(render_id, String::as_str)
}

/// 🎯️ The granularity of one rendered instance's interaction target. An explicit record
/// override wins; otherwise pick, hover and marquee all use the scene default, matching React's
/// `world3dInstanceInteractionTarget`.
pub fn instance_interaction_granularity_id<'a>(session_state: &'a WorldSessionState, interaction_state: &'a WorldInteractionState, render_id: &str) -> &'a str {
    interaction_state.instance_interaction_granularity_ids.get(render_id).map_or_else(|| resolved_domain_granularity_id(session_state), String::as_str)
}

/// 🧮️ Upper bound used by marquee action reservation. Every retained override is already
/// capped by [`WORLD_INTERACTION_ID_BYTE_CAPACITY`] at bridge admission.
pub(crate) fn maximum_instance_interaction_granularity_bytes(session_state: &WorldSessionState, interaction_state: &WorldInteractionState) -> usize {
    interaction_state.instance_interaction_granularity_ids.values().map(String::len).max().unwrap_or(0).max(resolved_domain_granularity_id(session_state).len())
}

/// 🕹️ The ONE modifier → merge rule this surface has, in the ONE vocabulary of
/// `🕹️interaction/🧬️schema/🔣️.json`, put on the wire verbatim.
///
/// ⚖️ `shift+ctrl` = invertive, `shift` = additive, `ctrl` (and its platform twin `meta`) =
/// subtractive, nothing = replace — identical to React's `resolveWorldMergeMode`
/// (`🌐️World3dHost/🟦️.tsx`) and pinned by `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`. Six
/// call sites in this file each open-coded `shift → additive, ctrl → INVERTIVE, else replace`, which
/// has no `subtractive` at all and spells ctrl's mode wrong; they now all read this.
pub fn world_merge_mode(shift: bool, ctrl_or_meta: bool) -> MergeMode {
    match (shift, ctrl_or_meta) {
        (true, true) => MergeMode::Invertive,
        (true, false) => MergeMode::Additive,
        (false, true) => MergeMode::Subtractive,
        (false, false) => MergeMode::Replace,
    }
}

pub fn world_merge_wire_label(code: u8) -> &'static str {
    match code {
        1 => MergeMode::Additive.wire_label(),
        2 => MergeMode::Subtractive.wire_label(),
        3 => MergeMode::Invertive.wire_label(),
        _ => MergeMode::Replace.wire_label(),
    }
}

pub fn selection_method_wire_str(method: SelectionMethod) -> &'static str {
    match method {
        SelectionMethod::Pick => "pick",
        SelectionMethod::Rectangle => "rectangle",
        SelectionMethod::Lasso => "lasso",
    }
}
//#endregion 🔖️WorldInteractionDomain

pub(crate) fn world_ray_triangle_barycentric(origin: Vec3, direction: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, f32, f32)> {
    let edge1 = b.sub(a);
    let edge2 = c.sub(a);
    let h = direction.cross(edge2);
    let determinant = edge1.dot(h);
    if determinant.abs() < 1e-8 {
        return None;
    }
    let inverse = 1.0 / determinant;
    let offset = origin.sub(a);
    let u = inverse * offset.dot(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = offset.cross(edge1);
    let v = inverse * direction.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = inverse * edge2.dot(q);
    (distance > 1e-6).then_some((distance, u, v))
}

/// 🎯️ `interactionSelect`/`interactionHover` carry `targets` as a JSON-encoded STRING, never as a
/// structured array.
///
/// ⚖️ The framework's own decoder is `parse_interaction_targets`
/// (`💻️os/🔨️modules/🔌️plugin/🦀️.rs`): it reads the arg with `DslValue::as_str` and hands the text to
/// `serde_json`, and the manifest declares the arg as `ActionArgDef::text("targets")`. An ARRAY makes
/// `as_str` answer `None`, so the guest faults the job with `missing required arg targets` — and
/// because a reserved tool job's fault never reaches the dispatch promise, the shell saw a clean
/// dispatch and the selection simply never moved. Measured on 6118 across hover, click, shift-click,
/// empty-click and marquee (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-world3d-interaction-2026-09-13.md`). React encodes the same way —
/// `world3dSelectionActionArgs`/`world3dHoverActionArgs` both `JSON.stringify(targets)`.
///
/// The three spans are written through `string_joined`, so the JSON text costs no allocation and
/// stays inside the action's own byte credits.
pub(crate) const INTERACTION_TARGETS_ARRAY_OPEN: &str = "[";

pub(crate) const INTERACTION_TARGETS_ARRAY_CLOSE: &str = "]";

pub(crate) const INTERACTION_TARGETS_OPEN: &str = "{\"granularity\":\"";

pub(crate) const INTERACTION_TARGETS_MIDDLE: &str = "\",\"id\":\"";

pub(crate) const INTERACTION_TARGETS_TAIL: &str = "\"}";

/// 🎯️ Writes the `targets` arg of one `interactionSelect`/`interactionHover` — see
/// [`INTERACTION_TARGETS_OPEN`]. `bare_id` is true when a real app domain is bound, in which case the
/// target id is the topology id alone; the shared `world` domain is `PathDelimited` and prefixes the
/// surface. A miss on any of the three spans is an EMPTY target list — the clear, which the fixture's
/// `background-click-clears` case requires to be a dispatch rather than a silence.
pub struct WorldInteractionText {
    pub bytes: [u8; semio_ui_action::ACTION_STRING_BYTE_CAPACITY],
    pub len: usize,
}

impl Default for WorldInteractionText {
    fn default() -> Self { Self { bytes: [0; semio_ui_action::ACTION_STRING_BYTE_CAPACITY], len: 0 } }
}

impl std::fmt::Write for WorldInteractionText {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.len.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(std::fmt::Error)?;
        self.bytes[self.len..end].copy_from_slice(text.as_bytes()); self.len = end; Ok(())
    }
}

impl WorldInteractionText {
    pub fn as_str(&self) -> &str { std::str::from_utf8(&self.bytes[..self.len]).expect("interaction text retains valid UTF-8") }
}

fn world_write_json_string(output: &mut impl std::fmt::Write, parts: &[&str]) -> std::fmt::Result {
    output.write_char('"')?;
    for character in parts.iter().flat_map(|part| part.chars()) {
        match character {
            '"' => output.write_str("\\\"")?,
            '\\' => output.write_str("\\\\")?,
            '\n' => output.write_str("\\n")?,
            '\r' => output.write_str("\\r")?,
            '\t' => output.write_str("\\t")?,
            '\u{0008}' => output.write_str("\\b")?,
            '\u{000c}' => output.write_str("\\f")?,
            character if character < ' ' => write!(output, "\\u{:04x}", character as u32)?,
            character => output.write_char(character)?,
        }
    }
    output.write_char('"')
}

pub fn world_write_interaction_target(output: &mut impl std::fmt::Write, granularity: &str, id: &[&str]) -> std::fmt::Result {
    output.write_str("{\"granularity\":")?;
    world_write_json_string(output, &[granularity])?;
    output.write_str(",\"id\":")?;
    world_write_json_string(output, id)?;
    output.write_char('}')
}

pub fn world_hover_action_string_bytes(controller: &str, domain: &str, targets: &str, window: Option<&str>) -> Result<usize, semio_ui_action::BoundedActionFault> {
    semio_ui_action::checked_action_string_bytes(&[controller, "interactionHover", "domainId", domain, "channel", "pointer", "targets", targets, window.map_or("", |_| "windowId"), window.unwrap_or_default()])
}

pub fn world_authored_hover_targets<'a>(session: &WorldSessionState, interaction: &'a WorldInteractionState, instance_id: &str) -> Option<&'a str> {
    interaction.instance_hover_targets.get(instance_id).map(String::as_str).filter(|targets| {
        world_hover_action_string_bytes(&session.controller_id, resolved_domain_id(session), targets, session.window_instance_id.as_deref()).is_ok()
    })
}

#[path = "🔲️marquee/🦀️.rs"]
mod marquee;
pub use marquee::*;

#[path = "🕹️gumball-and-brush/🦀️.rs"]
mod gumball_and_brush;
pub use gumball_and_brush::*;

#[path = "🎯️ray-and-component-pick/🦀️.rs"]
mod ray_and_component_pick;
pub use ray_and_component_pick::*;

/// 🔢️ The version `publish_world3d_mesh_lease` admitted this mesh key under. `mesh_versions` and
/// `interaction_meshes` are written in ONE transaction there, so this is the freshness witness every
/// interaction-registry lookup must match — never `SceneDraw3d::mesh_version`, which the typed
/// snapshot apply admits before the mesh's own publication ladder has run and which therefore stays
/// at its `0` placeholder for every bridged draw. Matching the stale field faulted the whole
/// interaction authority — every pick, hover, orbit and wheel on that surface.
pub fn live_world3d_mesh_version(scene_state: &WorldSceneState, mesh_key: &str) -> u64 {
    match scene_state.meshes.0.get(mesh_key){Some(WorldMeshSource::Native(source)) if !source.frozen=>source.loan.identity().cache_epoch,_=>scene_state.mesh_versions.get(mesh_key).copied().unwrap_or(0)}
}
