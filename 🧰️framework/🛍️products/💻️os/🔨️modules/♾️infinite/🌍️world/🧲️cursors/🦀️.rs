//! 🧲️ World3d interaction cursors: ray, object and component picking, marquee selection and gumball gestures.
//!
//! Crate `semio-framework-os-infinite-world-cursors`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{ROTATE_SELECTION_ACTION_ID, SCALE_SELECTION_ACTION_ID, TRANSLATE_SELECTION_ACTION_ID, WORLD_INTERACTION_BYTE_CAPACITY, WORLD_INTERACTION_ID_BYTE_CAPACITY, WORLD_INTERACTION_MARQUEE_POINT_CAPACITY, WORLD_INTERACTION_MESH_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, world_pick_keys_match, world_pick_target_ray_score, WorldCancelReason, WorldFlatAction, WorldFlatActionKind, WorldInteractionMeshRegistry, WorldInteractionMeshToken, WorldInteractionObjectKind, WorldInteractionObjectRegistry, WorldInteractionObjectSlot, WorldInteractionObjectToken, WorldInteractionPlan, WorldInteractionSpan, WorldInteractionState, WorldInteractionStep, WorldMeshSource, WorldSessionState};
use semio_framework_os_infinite_world_components::{GumballHandle, world3d_declares_action, World3dSceneGumballConfigRecord, WORLD_COMPONENT_TARGET_BYTE_CAPACITY, world_mesh_component_id, world_mesh_triangle, world_mesh_vertex, WorldCameraState, WorldComponentAddress, WorldGumballState, WorldSceneState, WorldSelectionState};
use ui_wgpu::wgpu::{ActionDescriptor, axis_rotate_angle, gumball_extent, gumball_eye, gumball_project_ray_onto_axis, interpolate_mesh_uv, Mat4, Mesh3dField, ray_aabb_slab, ray_plane_point, ray_segment_distance, Rect, Vec3};
use semio_framework::{MergeMode, SelectionMethod};
use semio_framework_ui_viewport::Viewport3dProjectionSpec;

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
    pub bytes: [u8; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY],
    pub len: usize,
}

impl Default for WorldInteractionText {
    fn default() -> Self { Self { bytes: [0; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY], len: 0 } }
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

pub struct WorldMarqueeGesture {
    pub revision: u64,
    pub start_generation: u64,
    pub points: Box<[Option<[f32; 2]>; WORLD_INTERACTION_MARQUEE_POINT_CAPACITY]>,
    pub len: u16,
    pub retiring: bool,
}

pub const WORLD_MARQUEE_RESULT_PAGE_CAPACITY: usize = 64;

pub const WORLD_MARQUEE_RESULT_PAGE_COUNT: usize = 16;

pub const WORLD_COMPONENT_MARQUEE_CAPACITY: usize = 64;

//#region 🧵️WorldInteractionTransaction
#[derive(Clone, Copy)]
pub enum WorldMarqueeResult {
    Object(WorldInteractionObjectToken),
    Component { object: WorldInteractionObjectToken, id: u32 },
}

pub struct WorldMarqueeResultPages {
    pub pages: Box<[[Option<WorldMarqueeResult>; WORLD_MARQUEE_RESULT_PAGE_CAPACITY]; WORLD_MARQUEE_RESULT_PAGE_COUNT]>,
    pub lens: [u8; WORLD_MARQUEE_RESULT_PAGE_COUNT],
    id_bytes: [u16; WORLD_MARQUEE_RESULT_PAGE_COUNT],
    pub page_len: u8,
}

impl Default for WorldMarqueeResultPages {
    fn default() -> Self {
        Self { pages: Box::new([[None; WORLD_MARQUEE_RESULT_PAGE_CAPACITY]; WORLD_MARQUEE_RESULT_PAGE_COUNT]), lens: [0; WORLD_MARQUEE_RESULT_PAGE_COUNT], id_bytes: [0; WORLD_MARQUEE_RESULT_PAGE_COUNT], page_len: 0 }
    }
}

impl WorldMarqueeResultPages {
    pub fn push(&mut self, result: WorldMarqueeResult, id_bytes: usize) -> bool {
        if let WorldMarqueeResult::Component { object, id } = result {
            if self.pages[0][..usize::from(self.lens[0])].iter().flatten().any(|entry| matches!(entry, WorldMarqueeResult::Component { object: existing_object, id: existing } if *existing_object == object && *existing == id)) {
                return true;
            }
        }
        if matches!(result, WorldMarqueeResult::Component { .. }) && (self.page_len > 1 || self.page_len == 1 && usize::from(self.lens[0]) == WORLD_COMPONENT_MARQUEE_CAPACITY) {
            return false;
        }
        let mut page = usize::from(self.page_len.saturating_sub(1));
        if self.page_len == 0 || usize::from(self.lens[page]) == WORLD_MARQUEE_RESULT_PAGE_CAPACITY {
            page = usize::from(self.page_len);
            if page == WORLD_MARQUEE_RESULT_PAGE_COUNT {
                return false;
            }
            self.page_len += 1;
        }
        let item = usize::from(self.lens[page]);
        let Some(bytes) = usize::from(self.id_bytes[page]).checked_add(id_bytes).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY) else {
            return false;
        };
        self.pages[page][item] = Some(result);
        self.lens[page] += 1;
        self.id_bytes[page] = bytes as u16;
        true
    }

    pub fn close_step(&mut self) -> bool {
        if self.page_len == 0 {
            return true;
        }
        let page = usize::from(self.page_len - 1);
        if self.lens[page] > 0 {
            self.lens[page] -= 1;
            self.pages[page][usize::from(self.lens[page])] = None;
            return false;
        }
        self.id_bytes[page] = 0;
        self.page_len -= 1;
        false
    }
}

pub struct WorldMarqueePickCursor {
    pub generation: u64,
    pub gesture: WorldMarqueeGesture,
    pub viewport: Rect,
    pub view_projection: Mat4,
    pub projection_spec: Viewport3dProjectionSpec,
    pub rectangle: bool,
    pub component_kind: Option<WorldComponentKind>,
    pub crossing: Option<bool>,
    direction_index: u16,
    pub slot: u16,
    pub current: Option<WorldInteractionObjectToken>,
    pub topology: u32,
    polygon_edge: u16,
    pub candidate: Option<WorldMarqueeCandidate>,
    winding: [i16; 3],
    intersects: bool,
    any_visible: bool,
    pub results: WorldMarqueeResultPages,
    pub complete: bool,
    pub faulted: bool,
}

#[derive(Clone, Copy)]
enum WorldMarqueeCandidate {
    Point { point: [f32; 2], component: Option<u32> },
    Segment { points: [[f32; 2]; 2], component: u32 },
    Triangle { points: [[f32; 2]; 3], component: Option<u32> },
}

impl WorldMarqueePickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, generation: u64, gesture: WorldMarqueeGesture) -> Option<Self> {
        let viewport = render_pick_viewport(session_state);
        if viewport.w <= 0.0 || viewport.h <= 0.0 || gesture.len < 2 {
            return None;
        }
        let camera = camera_state.orbit.to_camera();
        let view_projection = ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, viewport.w, viewport.h);
        let rectangle = selection_state.selection_method != "lasso";
        let component_kind = match selection_state.granularity.as_str() {
            "vertex" => Some(WorldComponentKind::Vertex),
            "edge" => Some(WorldComponentKind::Edge),
            "face" | "component" => Some(WorldComponentKind::Face),
            _ => None,
        };
        let crossing = rectangle.then(|| {
            let start = gesture.points[0].expect("marquee start");
            let end = gesture.points[usize::from(gesture.len - 1)].expect("marquee end");
            end[0] < start[0]
        });
        Some(Self {
            generation,
            gesture,
            viewport,
            view_projection,
            projection_spec: camera_state.projection_spec,
            rectangle,
            component_kind,
            crossing,
            direction_index: 1,
            slot: 0,
            current: None,
            topology: 0,
            polygon_edge: 0,
            candidate: None,
            winding: [0; 3],
            intersects: false,
            any_visible: false,
            results: WorldMarqueeResultPages::default(),
            complete: false,
            faulted: false,
        })
    }

    fn local_point(&self, index: usize) -> [f32; 2] {
        let point = self.gesture.points[index].expect("marquee point within retained length");
        [point[0] - self.viewport.x, point[1] - self.viewport.y]
    }

    fn admit_candidate(&mut self, interaction_state: &WorldInteractionState, result: WorldMarqueeResult) -> WorldInteractionStep {
        let token = match result {
            WorldMarqueeResult::Object(token) | WorldMarqueeResult::Component { object: token, .. } => token,
        };
        let Some(entry) = interaction_state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let id_bytes = if matches!(result, WorldMarqueeResult::Object(_)) { entry.id.as_str().len() } else { 0 };
        if !self.results.push(result, id_bytes) {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        }
        WorldInteractionStep::Pending
    }

    fn orient(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    }

    fn on_segment(point: [f32; 2], a: [f32; 2], b: [f32; 2]) -> bool {
        point[0] >= a[0].min(b[0]) && point[0] <= a[0].max(b[0]) && point[1] >= a[1].min(b[1]) && point[1] <= a[1].max(b[1])
    }

    fn segments_intersect(a0: [f32; 2], a1: [f32; 2], b0: [f32; 2], b1: [f32; 2]) -> bool {
        let o1 = Self::orient(a0, a1, b0);
        let o2 = Self::orient(a0, a1, b1);
        let o3 = Self::orient(b0, b1, a0);
        let o4 = Self::orient(b0, b1, a1);
        (o1 == 0.0 && Self::on_segment(b0, a0, a1)) || (o2 == 0.0 && Self::on_segment(b1, a0, a1)) || (o3 == 0.0 && Self::on_segment(a0, b0, b1)) || (o4 == 0.0 && Self::on_segment(a1, b0, b1)) || ((o1 > 0.0) != (o2 > 0.0) && (o3 > 0.0) != (o4 > 0.0))
    }

    pub fn rect(&self) -> [f32; 4] {
        let start = self.local_point(0);
        let end = self.local_point(usize::from(self.gesture.len - 1));
        [start[0].min(end[0]), start[1].min(end[1]), start[0].max(end[0]), start[1].max(end[1])]
    }

    fn rect_contains(rect: [f32; 4], point: [f32; 2]) -> bool {
        point[0] >= rect[0] && point[0] <= rect[2] && point[1] >= rect[1] && point[1] <= rect[3]
    }

    fn triangle_intersects_rect(points: [[f32; 2]; 3], rect: [f32; 4]) -> bool {
        if points.iter().any(|point| Self::rect_contains(rect, *point)) {
            return true;
        }
        let corners = [[rect[0], rect[1]], [rect[2], rect[1]], [rect[2], rect[3]], [rect[0], rect[3]]];
        (0..3).any(|edge| (0..4).any(|side| Self::segments_intersect(points[edge], points[(edge + 1) % 3], corners[side], corners[(side + 1) % 4])))
    }

    fn begin_candidate(&mut self, candidate: WorldMarqueeCandidate) {
        self.candidate = Some(candidate);
        self.polygon_edge = 0;
        self.winding = [0; 3];
        self.intersects = false;
    }

    fn candidate_step(&mut self) -> Option<bool> {
        let candidate = self.candidate?;
        let edge = usize::from(self.polygon_edge);
        if edge == usize::from(self.gesture.len) {
            let selected = match candidate {
                WorldMarqueeCandidate::Point { .. } => self.winding[0] != 0,
                WorldMarqueeCandidate::Segment { .. } => {
                    if self.crossing.expect("marquee direction resolved") {
                        self.intersects || self.winding[0] != 0 || self.winding[1] != 0
                    } else {
                        self.winding[0] != 0 && self.winding[1] != 0
                    }
                }
                WorldMarqueeCandidate::Triangle { .. } => {
                    if self.crossing.expect("marquee direction resolved") {
                        self.intersects || self.winding.iter().any(|winding| *winding != 0)
                    } else {
                        self.winding.iter().all(|winding| *winding != 0)
                    }
                }
            };
            self.candidate = None;
            self.polygon_edge = 0;
            self.winding = [0; 3];
            self.intersects = false;
            return Some(selected);
        }
        let a = self.local_point(edge);
        let b = self.local_point((edge + 1) % usize::from(self.gesture.len));
        let points = match candidate {
            WorldMarqueeCandidate::Point { point, .. } => [point, point, point],
            WorldMarqueeCandidate::Segment { points, .. } => [points[0], points[1], points[1]],
            WorldMarqueeCandidate::Triangle { points, .. } => points,
        };
        let count = match candidate {
            WorldMarqueeCandidate::Point { .. } => 1,
            WorldMarqueeCandidate::Segment { .. } => 2,
            WorldMarqueeCandidate::Triangle { .. } => 3,
        };
        for (index, point) in points.iter().copied().take(count).enumerate() {
            if a[1] <= point[1] {
                if b[1] > point[1] && Self::orient(a, b, point) > 0.0 {
                    self.winding[index] += 1;
                }
            } else if b[1] <= point[1] && Self::orient(a, b, point) < 0.0 {
                self.winding[index] -= 1;
            }
        }
        match candidate {
            WorldMarqueeCandidate::Segment { points, .. } => {
                self.intersects |= Self::segments_intersect(points[0], points[1], a, b);
            }
            WorldMarqueeCandidate::Triangle { points, .. } => {
                self.intersects |= (0..3).any(|index| Self::segments_intersect(points[index], points[(index + 1) % 3], a, b));
            }
            WorldMarqueeCandidate::Point { .. } => {}
        }
        self.polygon_edge += 1;
        None
    }

    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.gesture.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.crossing.is_none() {
            let index = usize::from(self.direction_index);
            if index < usize::from(self.gesture.len) {
                let start = self.local_point(0);
                let point = self.local_point(index);
                self.direction_index += 1;
                if (point[0] - start[0]).abs() >= 2.0 {
                    self.crossing = Some(point[0] < start[0]);
                }
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let start = self.local_point(0);
            let end = self.local_point(usize::from(self.gesture.len - 1));
            self.crossing = Some(end[0] < start[0]);
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if self.candidate.is_some() {
            let candidate = self.candidate.expect("retained marquee candidate");
            let Some(selected) = self.candidate_step() else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            let token = self.current.expect("marquee candidate object");
            let component = match candidate {
                WorldMarqueeCandidate::Point { component, .. } | WorldMarqueeCandidate::Triangle { component, .. } => component,
                WorldMarqueeCandidate::Segment { component, .. } => Some(component),
            };
            if let Some(id) = component {
                let outcome = if selected { self.admit_candidate(interaction_state, WorldMarqueeResult::Component { object: token, id }) } else { WorldInteractionStep::Pending };
                context.consume_fuel(1);
                return outcome;
            }
            if !selected && !self.crossing.expect("crossing resolved") {
                self.current = None;
                self.topology = 0;
                self.any_visible = false;
            }
            let outcome = if selected && self.crossing.expect("crossing resolved") {
                self.current = None;
                self.topology = 0;
                self.any_visible = false;
                self.admit_candidate(interaction_state, WorldMarqueeResult::Object(token))
            } else {
                WorldInteractionStep::Pending
            };
            context.consume_fuel(1);
            return outcome;
        }
        if self.current.is_none() {
            let index = usize::from(self.slot);
            if index >= usize::from(interaction_state.interaction_objects.instance_len) {
                if self.results.page_len == 0 {
                    self.results.page_len = 1;
                }
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.slot += 1;
            let Some(token) = interaction_state.interaction_objects.instance_order[index] else {
                return WorldInteractionStep::Fault;
            };
            let Some(entry) = interaction_state.interaction_objects.resolve(token).filter(|entry| entry.revision == self.gesture.revision && entry.kind == WorldInteractionObjectKind::Instance) else {
                return WorldInteractionStep::Stale;
            };
            if self.component_kind.is_some() && selection_state.active_object_id.as_deref().is_some_and(|active| active != entry.id.as_str()) {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.current = Some(token);
            self.topology = 0;
            self.any_visible = false;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.current.expect("marquee current object");
        let Some(entry) = interaction_state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(mesh_token) = entry.mesh else {
            return WorldInteractionStep::Fault;
        };
        let Some(admitted) = interaction_state.interaction_meshes.resolve(mesh_token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(&mesh) = scene_state.meshes.get(admitted.id.as_str()) else {
            return WorldInteractionStep::Fault;
        };
        let Ok(schema) = mesh.schema() else {
            return WorldInteractionStep::Stale;
        };
        if admitted.vertices != schema.vertices || admitted.edges != if schema.edge_ids == schema.edges { schema.edges } else { 0 } || admitted.triangles != schema.indices / 3 {
            return WorldInteractionStep::Stale;
        }
        let crossing = self.crossing.expect("crossing resolved");
        if let Some(kind) = self.component_kind {
            let count = match kind {
                WorldComponentKind::Vertex => if schema.vertex_ids == schema.vertices { admitted.vertices } else { 0 },
                WorldComponentKind::Edge => admitted.edges,
                WorldComponentKind::Face => admitted.triangles,
            };
            if self.topology >= count {
                self.current = None;
                self.topology = 0;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let index = self.topology;
            self.topology += 1;
            let id = match kind {
                WorldComponentKind::Vertex => world_mesh_component_id(mesh, Mesh3dField::VertexIds, index),
                WorldComponentKind::Edge => world_mesh_component_id(mesh, Mesh3dField::EdgeIds, index),
                WorldComponentKind::Face => world_mesh_component_id(mesh, Mesh3dField::FaceIds, index),
            };
            if kind == WorldComponentKind::Vertex && id == u32::MAX {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let mut selected = false;
            match kind {
                WorldComponentKind::Vertex => {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let world = entry.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                    let Some(screen) = ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    };
                    if self.rectangle {
                        selected = Self::rect_contains(self.rect(), screen);
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Point { point: screen, component: Some(id) });
                    }
                }
                WorldComponentKind::Edge => {
                    let Ok(edge) = mesh.edge(index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let a = entry.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = entry.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    let Some(segment) = ui_wgpu::wgpu::projection_spec_project_segment(self.view_projection, self.projection_spec, a, b, self.viewport.w, self.viewport.h) else {
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    };
                    let [screen_a, screen_b] = segment.screen;
                    if self.rectangle {
                        selected = if crossing {
                            Self::rect_contains(self.rect(), screen_a) || Self::rect_contains(self.rect(), screen_b) || {
                                let rect = self.rect();
                                let corners = [[rect[0], rect[1]], [rect[2], rect[1]], [rect[2], rect[3]], [rect[0], rect[3]]];
                                (0..4).any(|side| Self::segments_intersect(screen_a, screen_b, corners[side], corners[(side + 1) % 4]))
                            }
                        } else {
                            Self::rect_contains(self.rect(), screen_a) && Self::rect_contains(self.rect(), screen_b)
                        };
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Segment { points: [screen_a, screen_b], component: id });
                    }
                }
                WorldComponentKind::Face => {
                    let Some(indices) = world_mesh_triangle(mesh, index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let mut points = [[0.0; 2]; 3];
                    for corner in 0..3 {
                        let Some(world) = world_mesh_vertex(mesh, indices[corner]).map(|vertex| entry.model.transform_point(vertex)) else {
                            return WorldInteractionStep::Stale;
                        };
                        let Some(screen) = ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                            context.consume_fuel(1);
                            return WorldInteractionStep::Pending;
                        };
                        points[corner] = screen;
                    }
                    if self.rectangle {
                        selected = if crossing { Self::triangle_intersects_rect(points, self.rect()) } else { points.iter().all(|point| Self::rect_contains(self.rect(), *point)) };
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Triangle { points, component: Some(id) });
                    }
                }
            }
            let outcome = if selected { self.admit_candidate(interaction_state, WorldMarqueeResult::Component { object: token, id }) } else { WorldInteractionStep::Pending };
            context.consume_fuel(1);
            return outcome;
        }
        if !crossing {
            if self.topology >= admitted.vertices {
                self.current = None;
                self.topology = 0;
                let selected = self.any_visible;
                self.any_visible = false;
                let outcome = if selected { self.admit_candidate(interaction_state, WorldMarqueeResult::Object(token)) } else { WorldInteractionStep::Pending };
                context.consume_fuel(1);
                return outcome;
            }
            let index = self.topology;
            self.topology += 1;
            let Some(world) = world_mesh_vertex(mesh, index).map(|vertex| entry.model.transform_point(vertex)) else {
                return WorldInteractionStep::Stale;
            };
            let Some(point) = ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            self.any_visible = true;
            if self.rectangle {
                if !Self::rect_contains(self.rect(), point) {
                    self.current = None;
                    self.topology = 0;
                    self.any_visible = false;
                }
            } else {
                self.begin_candidate(WorldMarqueeCandidate::Point { point, component: None });
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if self.topology >= admitted.triangles {
            self.current = None;
            self.topology = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let triangle = self.topology;
        self.topology += 1;
        let Some(indices) = world_mesh_triangle(mesh, triangle) else {
            return WorldInteractionStep::Stale;
        };
        let mut points = [[0.0; 2]; 3];
        for index in 0..3 {
            let Some(world) = world_mesh_vertex(mesh, indices[index]).map(|vertex| entry.model.transform_point(vertex)) else {
                return WorldInteractionStep::Stale;
            };
            let Some(point) = ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            points[index] = point;
        }
        let selected = if self.rectangle {
            Self::triangle_intersects_rect(points, self.rect())
        } else {
            self.begin_candidate(WorldMarqueeCandidate::Triangle { points, component: None });
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let outcome = if selected {
            self.current = None;
            self.topology = 0;
            self.admit_candidate(interaction_state, WorldMarqueeResult::Object(token))
        } else {
            WorldInteractionStep::Pending
        };
        context.consume_fuel(1);
        outcome
    }

    pub fn close_step(&mut self) -> bool {
        if self.candidate.take().is_some() {
            self.polygon_edge = 0;
            self.winding = [0; 3];
            self.intersects = false;
            return false;
        }
        if self.current.take().is_some() {
            self.topology = 0;
            self.any_visible = false;
            return false;
        }
        if !self.results.close_step() {
            return false;
        }
        self.gesture.close_step()
    }

    #[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
    pub fn finish(self, session_state: &WorldSessionState, generation: u64) -> Result<(WorldMarqueeGesture, WorldMarqueeResultPages), (Self, WorldInteractionStep)> {
        if !self.complete {
            return Err((self, WorldInteractionStep::Pending));
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation {
            return Err((self, WorldInteractionStep::Stale));
        }
        Ok((self.gesture, self.results))
    }
}

pub struct WorldMarqueePublishJob {
    pub generation: u64,
    pub gesture: WorldMarqueeGesture,
    pub results: WorldMarqueeResultPages,
    pub merge: &'static str,
    pub prepared: Option<ui_wgpu::wgpu::PreparedClaimedActionBatch>,
    pub draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    /// 🎯️ The page's `targets` value BEING BUILT — one JSON-encoded string, assembled one target per
    /// bounded step. See [`INTERACTION_TARGETS_OPEN`] for why the arg is text rather than an array.
    pub targets: Box<[u8; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY]>,
    pub targets_len: u16,
    pub page: u8,
    pub stage: u16,
    pub published: bool,
}

/// 🔎️ First index of `needle` in `haystack`, or `None` — a fixed-window scan, no allocation, for
/// [`WorldMarqueePublishJob::targets_contain`].
fn world_find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).find(|start| &haystack[*start..*start + needle.len()] == needle)
}

impl WorldMarqueePublishJob {
    pub fn new(generation: u64, gesture: WorldMarqueeGesture, results: WorldMarqueeResultPages, shift: bool, ctrl: bool) -> Self {
        let merge = world_merge_mode(shift, ctrl).wire_label();
        Self { generation, gesture, results, merge, prepared: None, draft: None, targets: Box::new([0; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY]), targets_len: 0, page: 0, stage: 0, published: false }
    }

    fn push_targets(&mut self, parts: &[&str]) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        let mut len = usize::from(self.targets_len);
        for part in parts {
            let end = len.checked_add(part.len()).filter(|end| *end <= ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
            self.targets[len..end].copy_from_slice(part.as_bytes());
            len = end;
        }
        self.targets_len = len as u16;
        Ok(())
    }

    fn targets_text(&self) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        core::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }

    /// 🎯️ Whether this page's `targets` already names the exact `(granularity, id)` pair.
    ///
    /// ⚖️ A selection is a SET of topology target pairs: several instances can share one target,
    /// while the same id under two declared granularities remains two distinct targets. React's
    /// `world3dInstanceInteractionTargets` dedups on the same rule, and
    /// `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`'s `marquee-release-replaces` case pins it.
    fn targets_contain(&self, granularity: &str, id: &[&str]) -> bool {
        let text = &self.targets[..usize::from(self.targets_len)];
        let mut cursor = 0usize;
        while let Some(found) = world_find_bytes(&text[cursor..], INTERACTION_TARGETS_OPEN.as_bytes()) {
            let start = cursor + found;
            cursor = start + INTERACTION_TARGETS_OPEN.len();
            let mut offset = start;
            let matches = [INTERACTION_TARGETS_OPEN, granularity, INTERACTION_TARGETS_MIDDLE]
                .into_iter()
                .chain(id.iter().copied())
                .chain([INTERACTION_TARGETS_TAIL])
                .all(|part| {
                    let Some(end) = offset.checked_add(part.len()) else { return false };
                    let matched = text.get(offset..end) == Some(part.as_bytes());
                    offset = end;
                    matched
                });
            if matches {
                return true;
            }
        }
        false
    }

    /// 🎯️ The `targets` VALUE is JSON text now, so its own punctuation is part of the reservation —
    /// including the two bytes of `[]` a page with NO targets still writes. Charging the array
    /// delimiters per target instead of once left an empty page two bytes short, and the shortfall
    /// surfaced on the LAST string of the action (`method`), faulting the whole publish
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`).
    pub fn page_credit(&self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, page: usize) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        let target_count = usize::from(self.results.lens[page]);
        let target_keys = target_count
            .checked_mul(INTERACTION_TARGETS_OPEN.len() + INTERACTION_TARGETS_MIDDLE.len() + INTERACTION_TARGETS_TAIL.len() + 1)
            .and_then(|bytes| bytes.checked_add(INTERACTION_TARGETS_ARRAY_OPEN.len() + INTERACTION_TARGETS_ARRAY_CLOSE.len()))
            .ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let granularities = target_count.checked_mul(maximum_instance_interaction_granularity_bytes(session_state, interaction_state)).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let id_prefix = if session_state.bound_domain_id.is_some() { 0 } else { target_count.checked_mul(session_state.surface_id.len() + WORLD_ITEM_PATH_DELIMITER.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)? };
        ui_wgpu::wgpu::checked_action_string_bytes(&[&session_state.controller_id, "interactionSelect", "domainId", resolved_domain_id(session_state), "targets", "merge", self.merge, "method", selection_method_wire_str(SelectionMethod::Rectangle)])?
            .checked_add(target_keys)
            .and_then(|bytes| bytes.checked_add(granularities))
            .and_then(|bytes| bytes.checked_add(id_prefix))
            .and_then(|bytes| bytes.checked_add(usize::from(self.results.id_bytes[page])))
            .filter(|bytes| *bytes <= ui_wgpu::wgpu::ACTION_ITEM_BYTE_CAPACITY)
            .ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)
    }

    /// 💥️ `then`, never `then_some`: `then_some`'s argument is evaluated EAGERLY, so `self.stage - 3`
    /// ran on stages 0, 1 and 2 as well and underflowed the `u16` — an arithmetic panic that aborted
    /// the wasm instance. Measured on 6118: every marquee release on the World3d preview killed the
    /// renderer outright, and because the Worker installed no panic hook the only visible symptom was
    /// the next `#[wasm_bindgen] &mut self` call failing with `recursive use of an object`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`).
    ///
    /// 🎯️ One TARGET per bounded step, appended to this page's `targets` text — the whole value is
    /// handed to the builder once, at `target_stage_end`. The id is the TOPOLOGY target the rendered
    /// instance stands for, deduplicated, exactly as a pick's is.
    pub fn step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if self.published {
            if !self.results.close_step() {
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            if !self.gesture.close_step() {
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            return Ok(WorldInteractionStep::Complete);
        }
        if self.prepared.is_none() {
            let mut credits = [0usize; WORLD_MARQUEE_RESULT_PAGE_COUNT];
            for (page, credit) in credits.iter_mut().enumerate().take(usize::from(self.results.page_len)) {
                *credit = self.page_credit(session_state, interaction_state, page)?;
            }
            let claims = input.claim_actions(&credits[..usize::from(self.results.page_len)])?;
            self.prepared = Some(ui_wgpu::wgpu::PreparedClaimedActionBatch::new(claims));
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if usize::from(self.page) == usize::from(self.results.page_len) {
            input.publish_prepared_claimed_actions(self.prepared.take().expect("terminal marquee claimed batch"))?;
            self.published = true;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let page = usize::from(self.page);
        if self.draft.is_none() {
            let claim = self.prepared.as_ref().and_then(|batch| batch.claim(page)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            self.draft = Some(input.draft_claimed_action(claim, &session_state.controller_id, "interactionSelect")?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let target_count = u16::from(self.results.lens[page]);
        let target_stage_end = 3 + target_count;
        let target_index = (self.stage >= 3 && self.stage < target_stage_end).then(|| usize::from(self.stage - 3));
        let token = target_index
            .and_then(|index| self.results.pages[page][index])
            .map(|result| match result {
                WorldMarqueeResult::Object(token) => Ok(token),
                WorldMarqueeResult::Component { .. } => Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
            })
            .transpose()?;
        let entry = token.map(|token| interaction_state.interaction_objects.resolve(token).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)).transpose()?;
        if self.stage >= 3 && self.stage < target_stage_end {
            let entry = entry.expect("marquee token resolved");
            let id = instance_interaction_id(interaction_state, entry.id.as_str());
            let granularity = instance_interaction_granularity_id(session_state, interaction_state, entry.id.as_str());
            let separator: &str = if self.targets_len > 1 { "," } else { "" };
            let target: [&str; 8] = if session_state.bound_domain_id.is_some() {
                [separator, INTERACTION_TARGETS_OPEN, granularity, INTERACTION_TARGETS_MIDDLE, id, "", "", INTERACTION_TARGETS_TAIL]
            } else {
                [separator, INTERACTION_TARGETS_OPEN, granularity, INTERACTION_TARGETS_MIDDLE, &session_state.surface_id, WORLD_ITEM_PATH_DELIMITER, id, INTERACTION_TARGETS_TAIL]
            };
            if !self.targets_contain(granularity, &target[4..7]) {
                self.push_targets(&target)?;
            }
            self.stage += 1;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.stage == 2 {
            self.targets_len = 0;
            self.push_targets(&["["])?;
            self.stage += 1;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.stage == target_stage_end {
            self.push_targets(&["]"])?;
            let targets = core::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            self.draft.as_mut().expect("marquee page draft").builder().string(Some("targets"), targets)?;
            self.stage += 1;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let draft = self.draft.as_mut().expect("marquee page draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("domainId"), resolved_domain_id(session_state))?,
            stage if stage == target_stage_end + 1 => draft.builder().string(Some("merge"), self.merge)?,
            stage if stage == target_stage_end + 2 => draft.builder().string(Some("method"), selection_method_wire_str(SelectionMethod::Rectangle))?,
            stage if stage == target_stage_end + 3 => draft.builder().end_container()?,
            _ => {
                let action = self.draft.take().expect("terminal marquee page draft").finish()?;
                self.prepared.as_mut().expect("marquee claimed batch").push(action)?;
                self.page += 1;
                self.stage = 0;
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(prepared) = self.prepared.as_mut() {
            if let Some(claim) = prepared.take_last_claim() {
                if input.release_action_claim(claim).is_err() {
                    prepared.restore_last_claim(claim).expect("marquee close restores retained claim slot");
                    return false;
                }
                return false;
            }
            self.prepared = None;
            return false;
        }
        if !self.results.close_step() {
            return false;
        }
        self.gesture.close_step()
    }
}

pub struct WorldComponentMarqueePublishJob {
    pub generation: u64,
    pub gesture: WorldMarqueeGesture,
    pub results: WorldMarqueeResultPages,
    pub kind: WorldComponentKind,
    pub merge: &'static str,
    pub targets: Box<[u8; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY]>,
    pub targets_len: u16,
    result_cursor: u8,
    targets_ready: bool,
    pub claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    pub draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    pub prepared: Option<ui_wgpu::wgpu::PreparedClaimedAction>,
    pub stage: u16,
    pub published: bool,
}

impl WorldComponentMarqueePublishJob {
    pub fn new(generation: u64, gesture: WorldMarqueeGesture, results: WorldMarqueeResultPages, kind: WorldComponentKind, shift: bool, ctrl: bool) -> Self {
        Self { generation, gesture, results, kind, merge: world_merge_mode(shift, ctrl).wire_label(), targets: Box::new([0; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY]), targets_len: 0, result_cursor: 0, targets_ready: false, claim: None, draft: None, prepared: None, stage: 0, published: false }
    }

    fn push_targets(&mut self, text: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
        let end = usize::from(self.targets_len).checked_add(text.len()).filter(|end| *end <= self.targets.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
        self.targets[usize::from(self.targets_len)..end].copy_from_slice(text.as_bytes()); self.targets_len = end as u16; Ok(())
    }

    fn targets_text(&self) -> &str { std::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).expect("component targets retain valid UTF-8") }

    fn targets_step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        if self.targets_len == 0 { self.push_targets("[")?; return Ok(false); }
        let index = usize::from(self.result_cursor);
        if index < usize::from(self.results.lens[0]) {
            let Some(WorldMarqueeResult::Component { object, id }) = self.results.pages[0][index] else { return Err(ui_wgpu::wgpu::BoundedActionFault::Structure); };
            let entry = interaction_state.interaction_objects.resolve(object).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let target = WorldComponentTarget::new(interaction_state, scene_state, entry, self.kind, id)?;
            let mut row = WorldInteractionText::default();
            let parts = if session_state.bound_domain_id.is_some() { ["", "", target.as_str()] } else { [&session_state.surface_id, WORLD_ITEM_PATH_DELIMITER, target.as_str()] };
            world_write_interaction_target(&mut row, self.kind.as_str(), &parts).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
            if index > 0 { self.push_targets(",")?; }
            self.push_targets(row.as_str())?;
            self.result_cursor += 1;
            return Ok(false);
        }
        self.push_targets("]")?;
        Ok(true)
    }

    pub fn step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() { return Ok(WorldInteractionStep::Pending); }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation { return Ok(WorldInteractionStep::Stale); }
        if self.published {
            if self.targets_len > 0 { self.targets_len = 0; context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
            if !self.results.close_step() || !self.gesture.close_step() { context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
            return Ok(WorldInteractionStep::Complete);
        }
        if !self.targets_ready { self.targets_ready = self.targets_step(session_state, interaction_state, scene_state)?; context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        if self.claim.is_none() {
            let credits = ui_wgpu::wgpu::checked_action_string_bytes(&[&session_state.controller_id, "interactionSelect", "domainId", resolved_domain_id(session_state), "targets", self.targets_text(), "merge", self.merge, "method", selection_method_wire_str(SelectionMethod::Rectangle)])?;
            self.claim = Some(input.claim_action(credits)?); context.consume_fuel(1); return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() && self.prepared.is_none() { self.draft = Some(input.draft_claimed_action(self.claim.expect("component marquee claim"), &session_state.controller_id, "interactionSelect")?); context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        if let Some(prepared) = self.prepared.take() { input.publish_prepared_claimed_action(prepared)?; self.claim = None; self.published = true; context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        let targets = std::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let draft = self.draft.as_mut().expect("component marquee draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("domainId"), resolved_domain_id(session_state))?,
            2 => draft.builder().string(Some("targets"), targets)?,
            3 => draft.builder().string(Some("merge"), self.merge)?,
            4 => draft.builder().string(Some("method"), selection_method_wire_str(SelectionMethod::Rectangle))?,
            5 => draft.builder().end_container()?,
            _ => { self.prepared = Some(self.draft.take().expect("terminal component marquee draft").finish()?); context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        }
        self.stage += 1; context.consume_fuel(1); Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.prepared.take().is_some() || self.draft.take().is_some() { return false; }
        if let Some(claim) = self.claim.take() { if input.release_action_claim(claim).is_err() { self.claim = Some(claim); } return false; }
        if self.targets_len > 0 { self.targets_len = 0; return false; }
        if !self.results.close_step() { return false; }
        self.gesture.close_step()
    }
}

impl WorldMarqueeGesture {
    pub fn new(revision: u64, generation: u64, point: [f32; 2]) -> Self {
        let mut points = Box::new([None; WORLD_INTERACTION_MARQUEE_POINT_CAPACITY]);
        points[0] = Some(point);
        Self { revision, start_generation: generation, points, len: 1, retiring: false }
    }

    pub fn push(&mut self, point: [f32; 2]) -> bool {
        let index = usize::from(self.len);
        if self.retiring || index == WORLD_INTERACTION_MARQUEE_POINT_CAPACITY {
            return false;
        }
        self.points[index] = Some(point);
        self.len += 1;
        true
    }

    pub fn is_click(&self, end: [f32; 2]) -> bool {
        let start = self.points[0].expect("marquee gesture owns start point");
        let dx = end[0] - start[0];
        let dy = end[1] - start[1];
        (dx * dx + dy * dy).sqrt() <= CLICK_DRAG_THRESHOLD_PX
    }

    pub fn close_step(&mut self) -> bool {
        self.retiring = true;
        if self.len == 0 {
            return true;
        }
        self.len -= 1;
        self.points[usize::from(self.len)] = None;
        false
    }
}
//#endregion 🧵️WorldInteractionTransaction

//#region 🧵️WorldInteractionTransaction
pub struct WorldComponentTarget {
    pub bytes: [u8; WORLD_COMPONENT_TARGET_BYTE_CAPACITY],
    pub len: usize,
}

impl std::fmt::Write for WorldComponentTarget {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.len.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(std::fmt::Error)?;
        self.bytes[self.len..end].copy_from_slice(text.as_bytes()); self.len = end; Ok(())
    }
}

impl WorldComponentTarget {
    pub fn new(interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, entry: &WorldInteractionObjectSlot, kind: WorldComponentKind, group: u32) -> Result<Self, ui_wgpu::wgpu::BoundedActionFault> {
        use std::fmt::Write;
        let mut target = Self { bytes: [0; WORLD_COMPONENT_TARGET_BYTE_CAPACITY], len: 0 };
        write!(target, "{}.{}.{}", entry.id.as_str(), kind.as_str(), group).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
        if let Some(source) = entry.component_source {
            let admitted = entry.mesh.and_then(|mesh| interaction_state.interaction_meshes.resolve(mesh)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let mesh = scene_state.meshes.get(admitted.id.as_str()).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let field = match kind { WorldComponentKind::Vertex => Mesh3dField::VertexIds, WorldComponentKind::Edge => Mesh3dField::EdgeIds, WorldComponentKind::Face => Mesh3dField::FaceIds };
            let label = mesh.component_label(field, group).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?.ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            write!(target, "~{}~{}~{}", source.handle(), label, source.revision()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
        }
        Ok(target)
    }

    pub fn as_str(&self) -> &str { std::str::from_utf8(&self.bytes[..self.len]).expect("component target retains valid UTF-8") }
}

impl WorldComponentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Edge => "edge",
            Self::Face => "face",
        }
    }
}

pub const WORLD_GUMBALL_SELECTED_CAPACITY: usize = ui_wgpu::wgpu::ACTION_NODE_CAPACITY - 11;

#[derive(Clone, Copy)]
pub struct WorldGumballTarget {
    pub object: WorldInteractionObjectToken,
    source_index: Option<u32>,
    captured: [u8; WORLD_COMPONENT_TARGET_BYTE_CAPACITY],
    captured_len: u16,
}

impl WorldGumballTarget {
    pub fn new(object: WorldInteractionObjectToken, source_index: Option<u32>, text: &str) -> Option<Self> {
        if text.len() > WORLD_COMPONENT_TARGET_BYTE_CAPACITY { return None; }
        let mut target = Self { object, source_index, captured: [0; WORLD_COMPONENT_TARGET_BYTE_CAPACITY], captured_len: u16::try_from(text.len()).ok()? };
        target.captured[..text.len()].copy_from_slice(text.as_bytes());
        Some(target)
    }

    fn captured_text(&self) -> &str { std::str::from_utf8(&self.captured[..usize::from(self.captured_len)]).expect("captured gumball target is valid UTF-8") }

    pub fn text<'a>(&self, gumball_state: &'a WorldGumballState, interaction_state: &'a WorldInteractionState, scene_state: &'a WorldSceneState, mode: Option<WorldComponentKind>) -> Option<&'a str> {
        let entry = interaction_state.interaction_objects.resolve(self.object)?;
        match (self.source_index, mode) {
            (None, None) => Some(entry.id.as_str()),
            (Some(index), Some(kind)) => {
                let id = gumball_state.gumball_selection_ids.get(index as usize)?.as_str();
                let address = WorldComponentAddress::parse(id, kind.as_str())?;
                (address.object == entry.id.as_str() && address.matches_registry(interaction_state, scene_state, entry)).then_some(id)
            }
            _ => None,
        }
    }
}

pub struct WorldGumballPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub x: f32,
    pub y: f32,
    pub slot: u16,
    source_cursor: usize,
    source_probe: u16,
    pub mode: Option<WorldComponentKind>,
    pub selected: Box<[Option<WorldGumballTarget>; WORLD_GUMBALL_SELECTED_CAPACITY]>,
    pub selected_len: u16,
    pub selected_bytes: u16,
    pub sum: Vec3,
    pub pivot: Option<Vec3>,
    pub origin: Vec3,
    pub direction: Vec3,
    pub eye: Vec3,
    pub extent: f32,
    pub handle: u8,
    validate: u16,
    pub best: Option<(f32, GumballHandle)>,
    pub complete: bool,
    pub faulted: bool,
}

pub struct WorldGumballGesture {
    pub revision: u64,
    pub start_generation: u64,
    pub handle: GumballHandle,
    pub pivot: Vec3,
    pub anchor: f32,
    pub start: Vec3,
    pub translate: Vec3,
    pub angle: f32,
    pub scale: Vec3,
    pub mode: Option<WorldComponentKind>,
    pub selected: Box<[Option<WorldGumballTarget>; WORLD_GUMBALL_SELECTED_CAPACITY]>,
    pub selected_len: u16,
    pub selected_bytes: u16,
    pub validation: u16,
    pub pending: Option<WorldGumballUpdate>,
    pub live: bool,
    pub streamed: bool,
    pub sent_translate: Vec3,
    pub sent_angle: f32,
    pub sent_scale: Vec3,
}

#[derive(Clone, Copy)]
struct WorldGumballUpdate {
    pub generation: u64,
    pub x: f32,
    pub y: f32,
}

/// 🎚️ The verb one gumball handle commits as — the single handle→action mapping, read by the pick
/// (which never offers a handle whose verb the window kind does not declare) and by the commit job
/// that mints it.
pub fn gumball_handle_action_id(handle: GumballHandle) -> &'static str {
    if handle.is_translate() {
        TRANSLATE_SELECTION_ACTION_ID
    } else if handle.is_rotate() {
        ROTATE_SELECTION_ACTION_ID
    } else {
        SCALE_SELECTION_ACTION_ID
    }
}

impl WorldGumballPickCursor {
    pub fn new(session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64, x: f32, y: f32) -> Self {
        Self {
            revision: session_state.interaction_revision,
            generation,
            x,
            y,
            slot: 0,
            source_cursor: 0,
            source_probe: 0,
            mode: match selection_state.granularity.as_str() { "vertex" => Some(WorldComponentKind::Vertex), "edge" => Some(WorldComponentKind::Edge), "face" | "component" => Some(WorldComponentKind::Face), _ => None },
            selected: Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]),
            selected_len: 0,
            selected_bytes: 0,
            sum: Vec3::ZERO,
            pivot: None,
            origin: Vec3::ZERO,
            direction: Vec3::ZERO,
            eye: Vec3::ZERO,
            extent: 0.0,
            handle: 0,
            validate: 0,
            best: None,
            complete: false,
            faulted: false,
        }
    }

    pub fn retain(&mut self, target: WorldGumballTarget, bytes: usize, position: Vec3) -> bool {
        let index = usize::from(self.selected_len);
        let Some(bytes) = usize::from(self.selected_bytes).checked_add(bytes).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY) else { return false; };
        let Some(slot) = self.selected.get_mut(index) else { return false; };
        *slot = Some(target);
        self.selected_bytes = bytes as u16;
        self.selected_len += 1;
        self.sum = self.sum.add(position);
        true
    }

    /// 🎛️ ONE gate for every handle: the resolved `gumballConfig` (authored, else the mode's
    /// fallback) ∩ the drafting plane — React's `gumballHandleEnabled`. A handle the gumball does
    /// not SHOW must not be pickable either.
    pub fn step(&mut self, session_state: &WorldSessionState, camera_state: &WorldCameraState, gumball_state: &WorldGumballState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.pivot.is_none() {
            if let Some(kind) = self.mode {
                if let Some(id) = gumball_state.gumball_selection_ids.get(self.source_cursor) {
                    let Some(address) = WorldComponentAddress::parse(id, kind.as_str()) else {
                        self.faulted = true;
                        return WorldInteractionStep::Fault;
                    };
                    if usize::from(self.source_probe) == WORLD_INTERACTION_OBJECT_CAPACITY {
                        self.faulted = true;
                        return WorldInteractionStep::Fault;
                    }
                    let index = (WorldInteractionObjectRegistry::hash(WorldInteractionObjectKind::Instance, address.object) + usize::from(self.source_probe)) % WORLD_INTERACTION_OBJECT_CAPACITY;
                    self.source_probe += 1;
                    if let Some(entry) = interaction_state.interaction_objects.slots[index].as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance && entry.id.as_str() == address.object) {
                        let Some(source_index) = u32::try_from(self.source_cursor).ok() else { self.faulted = true; return WorldInteractionStep::Fault; };
                        let Some(target) = WorldGumballTarget::new(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision }, Some(source_index), id) else { self.faulted = true; return WorldInteractionStep::Fault; };
                        if !address.matches_registry(interaction_state, scene_state, entry) || !self.retain(target, id.len(), Vec3::new(entry.values[3], entry.values[4], entry.values[5])) {
                            self.faulted = true;
                            return WorldInteractionStep::Fault;
                        }
                        self.source_cursor += 1;
                        self.source_probe = 0;
                    }
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
            } else {
                let index = usize::from(self.slot);
                if let Some(entry) = interaction_state.interaction_objects.slots.get(index) {
                    self.slot += 1;
                    if let Some(entry) = entry.as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance && entry.values[2] != 0.0) {
                        let Some(target) = WorldGumballTarget::new(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision }, None, entry.id.as_str()) else { self.faulted = true; return WorldInteractionStep::Fault; };
                        if !self.retain(target, entry.id.as_str().len(), Vec3::new(entry.values[3], entry.values[4], entry.values[5])) {
                            self.faulted = true;
                            return WorldInteractionStep::Fault;
                        }
                    }
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
            }
            if self.selected_len == 0 {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let pivot = gumball_state.gumball_target.map_or_else(|| self.sum.scale(1.0 / f32::from(self.selected_len)), |target| Vec3::new(target[0], target[1], target[2]));
            let Some((local_x, local_y, viewport)) = pointer_in_pick_rect(session_state, self.x, self.y) else {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            let camera = camera_state.orbit.to_camera();
            let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
            self.pivot = Some(pivot);
            self.origin = origin;
            self.direction = direction;
            self.eye = gumball_eye(&camera, pivot);
            self.extent = gumball_extent(camera.position.sub(pivot).length());
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let handles = [
            GumballHandle::MoveX,
            GumballHandle::MoveY,
            GumballHandle::MoveZ,
            GumballHandle::MoveXY,
            GumballHandle::MoveYZ,
            GumballHandle::MoveXZ,
            GumballHandle::RotateX,
            GumballHandle::RotateY,
            GumballHandle::RotateZ,
            GumballHandle::ScaleX,
            GumballHandle::ScaleY,
            GumballHandle::ScaleZ,
        ];
        let Some(handle) = handles.get(usize::from(self.handle)).copied() else {
            if self.validate < self.selected_len {
                let target = self.selected[usize::from(self.validate)].expect("gumball selected target");
                self.validate += 1;
                if target.text(gumball_state, interaction_state, scene_state, self.mode).is_none() {
                    return WorldInteractionStep::Stale;
                }
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        self.handle += 1;
        if !world3d_declares_action(session_state, gumball_handle_action_id(handle)) {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if !world3d_gumball_config(gumball_state).admits(handle) {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let pivot = self.pivot.expect("gumball pivot resolved");
        let pick_radius = self.extent * 0.08;
        let candidate = if handle.is_translate() && handle.axis_dir().is_some() {
            let axis = handle.axis_dir().expect("translation axis");
            ray_segment_distance(self.origin, self.direction, pivot, pivot.add(axis.scale(self.extent))).filter(|distance| *distance <= pick_radius)
        } else if matches!(handle, GumballHandle::MoveXY | GumballHandle::MoveYZ | GumballHandle::MoveXZ) {
            let normal = handle.plane_normal().expect("translation plane");
            ray_plane_point(self.origin, self.direction, pivot, normal).and_then(|hit| {
                let offset = hit.sub(pivot);
                let u = if normal.z.abs() > 0.9 {
                    offset.x.abs()
                } else if normal.x.abs() > 0.9 {
                    offset.y.abs()
                } else {
                    offset.x.abs()
                };
                let v = if normal.z.abs() > 0.9 { offset.y.abs() } else { offset.z.abs() };
                (u <= self.extent * 0.35 && v <= self.extent * 0.35).then_some(self.origin.sub(hit).length())
            })
        } else if handle.is_rotate() {
            let normal = handle.plane_normal().expect("rotation plane");
            ray_plane_point(self.origin, self.direction, pivot, normal).and_then(|hit| {
                let distance = (hit.sub(pivot).length() - self.extent * 0.85).abs();
                (distance <= pick_radius * 2.0).then_some(distance)
            })
        } else if handle.is_scale() {
            let axis = handle.axis_dir().expect("scale axis");
            ray_segment_distance(self.origin, self.direction, pivot, pivot.add(axis.scale(self.extent * 1.1))).filter(|distance| *distance <= pick_radius)
        } else {
            None
        };
        if let Some(distance) = candidate {
            if self.best.is_none_or(|(best, _)| distance < best) {
                self.best = Some((distance, handle));
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path."))]
    pub fn finish(mut self, session_state: &WorldSessionState, gumball_state: &WorldGumballState, generation: u64) -> Result<Option<WorldGumballGesture>, (Self, WorldInteractionStep)> {
        if !self.complete {
            return Err((self, WorldInteractionStep::Pending));
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err((self, WorldInteractionStep::Stale));
        }
        let Some((_, handle)) = self.best else {
            return Ok(None);
        };
        let pivot = self.pivot.expect("gumball terminal pivot");
        let anchor = handle.axis_dir().and_then(|axis| gumball_project_ray_onto_axis(self.origin, self.direction, pivot, axis, self.eye)).unwrap_or(0.0);
        let start = handle.plane_normal().and_then(|normal| ray_plane_point(self.origin, self.direction, pivot, normal)).map_or(pivot, |point| if handle.is_rotate() { point.sub(pivot) } else { point });
        Ok(Some(WorldGumballGesture {
            revision: self.revision,
            start_generation: self.generation,
            handle,
            pivot,
            anchor,
            start,
            translate: Vec3::ZERO,
            angle: 0.0,
            scale: Vec3::new(1.0, 1.0, 1.0),
            mode: self.mode,
            selected: std::mem::replace(&mut self.selected, Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY])),
            selected_len: self.selected_len,
            selected_bytes: self.selected_bytes,
            validation: 0,
            pending: None,
            live: gumball_state.gumball_live_dispatch,
            streamed: false,
            sent_translate: Vec3::ZERO,
            sent_angle: 0.0,
            sent_scale: Vec3::new(1.0, 1.0, 1.0),
        }))
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.selected_len > 0 {
            self.selected_len -= 1;
            self.selected[usize::from(self.selected_len)] = None;
            return false;
        }
        self.selected_bytes = 0;
        self.complete = true;
        true
    }
}

/// 🛠️ Where one gumball publication sits in the tool protocol — the wgpu twin of `🌐️World3dHost` `worldGumballStep`
/// (`🧫️fixtures/🛠️gumball-live-protocol.json`): `Skip` publishes nothing (a local drag that never moved along its handle),
/// `Once` is a local gesture's ONE net delta, `Stream`/`Commit` are a live gesture's ticks and tail, `Abort` drops a live
/// gesture that streamed with zero trace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldGumballPhase {
    Skip,
    Once,
    Stream,
    Commit,
    Abort(WorldCancelReason),
}

impl WorldGumballPhase {
    /// 🏷️ The `phase` (and an abort's `reason`) a publication carries after its motion.
    pub fn tail(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Skip | Self::Once => &[],
            Self::Stream => &[("phase", "stream")],
            Self::Commit => &[("phase", "commit")],
            Self::Abort(WorldCancelReason::Blur) => &[("phase", "abort"), ("reason", "blur")],
            Self::Abort(WorldCancelReason::CaptureLost) => &[("phase", "abort"), ("reason", "captureLost")],
        }
    }
}

/// 🎚️ The protocol's motion tolerance: a pose delta below it along the handle moves nothing.
const GUMBALL_MOTION_EPSILON: f32 = 1e-6;

/// 🧮️ One gumball motion — offset, angle about the handle's axis, factors.
type WorldGumballMotion = (Vec3, f32, Vec3);

/// 🫥️ The motion that moves nothing: what a live tail that moved nothing commits and what an abort names.
pub fn world_gumball_identity_motion() -> WorldGumballMotion {
    (Vec3::ZERO, 0.0, Vec3::new(1.0, 1.0, 1.0))
}

impl WorldGumballGesture {
    /// 🧮️ The motion still owed to the guest — from the pose the last streamed tick reached to the current one: offsets
    /// subtract, angles subtract, factors divide (a local gesture never streams, so it owes its whole motion).
    pub fn owed(&self) -> WorldGumballMotion {
        let ratio = |current: f32, sent: f32| current / if sent.abs() > GUMBALL_MOTION_EPSILON { sent } else { GUMBALL_MOTION_EPSILON };
        (self.translate.sub(self.sent_translate), self.angle - self.sent_angle, Vec3::new(ratio(self.scale.x, self.sent_scale.x), ratio(self.scale.y, self.sent_scale.y), ratio(self.scale.z, self.sent_scale.z)))
    }

    /// ⚖️ Whether `motion` moves along this gesture's handle, beyond [`GUMBALL_MOTION_EPSILON`].
    fn moves(&self, (translate, angle, scale): WorldGumballMotion) -> bool {
        if self.handle.is_translate() {
            [translate.x, translate.y, translate.z].iter().any(|value| value.abs() >= GUMBALL_MOTION_EPSILON)
        } else if self.handle.is_rotate() {
            angle.abs() >= GUMBALL_MOTION_EPSILON
        } else {
            [scale.x, scale.y, scale.z].iter().any(|value| (value - 1.0).abs() >= GUMBALL_MOTION_EPSILON)
        }
    }

    /// 🌊️ After a completed pose update: a live gesture owes a `Stream` tick of its owed motion, which it marks sent;
    /// a local gesture or a pose that moved nothing owes nothing.
    pub fn stream(&mut self) -> Option<(WorldGumballPhase, WorldGumballMotion)> {
        let owed = self.owed();
        if !self.live || self.selected_len == 0 || !self.moves(owed) {
            return None;
        }
        self.sent_translate = self.translate;
        self.sent_angle = self.angle;
        self.sent_scale = self.scale;
        self.streamed = true;
        Some((WorldGumballPhase::Stream, owed))
    }

    /// 🏁️ The release: a live gesture commits its owed tail (the identity when the tail moved nothing), a local gesture
    /// publishes its ONE net delta, and a gesture that grabbed nothing or never moved along its handle publishes nothing.
    pub fn release(&self) -> (WorldGumballPhase, WorldGumballMotion) {
        let owed = self.owed();
        match (self.selected_len > 0, self.live, self.moves(owed)) {
            (true, true, true) => (WorldGumballPhase::Commit, owed),
            (true, true, false) => (WorldGumballPhase::Commit, world_gumball_identity_motion()),
            (true, false, true) => (WorldGumballPhase::Once, owed),
            _ => (WorldGumballPhase::Skip, owed),
        }
    }

    /// 🧯️ A host cancel: a live gesture that streamed aborts with the identity motion; anything else is untouched.
    pub fn cancel(&self, reason: WorldCancelReason) -> Option<(WorldGumballPhase, WorldGumballMotion)> {
        (self.live && self.streamed).then(|| (WorldGumballPhase::Abort(reason), world_gumball_identity_motion()))
    }

    /// 📨️ This gesture's handle and targets carrying `motion` — what a publication job publishes.
    pub fn publication(&self, (translate, angle, scale): WorldGumballMotion) -> Self {
        Self {
            revision: self.revision,
            start_generation: self.start_generation,
            handle: self.handle,
            pivot: self.pivot,
            anchor: self.anchor,
            start: self.start,
            translate,
            angle,
            scale,
            mode: self.mode,
            selected: self.selected.clone(),
            selected_len: self.selected_len,
            selected_bytes: self.selected_bytes,
            validation: 0,
            pending: None,
            live: self.live,
            streamed: self.streamed,
            sent_translate: self.sent_translate,
            sent_angle: self.sent_angle,
            sent_scale: self.sent_scale,
        }
    }

    pub fn begin_update(&mut self, generation: u64, x: f32, y: f32) -> WorldInteractionStep {
        if generation <= self.start_generation {
            return WorldInteractionStep::Stale;
        }
        match self.pending {
            None => {
                self.pending = Some(WorldGumballUpdate { generation, x, y });
                self.validation = 0;
                WorldInteractionStep::Pending
            }
            Some(pending) if pending.generation == generation && pending.x == x && pending.y == y => WorldInteractionStep::Pending,
            Some(_) => WorldInteractionStep::Fault,
        }
    }

    pub fn update_step(&mut self, session_state: &WorldSessionState, camera_state: &WorldCameraState, gumball_state: &WorldGumballState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState) -> WorldInteractionStep {
        if self.revision != session_state.interaction_revision {
            return WorldInteractionStep::Stale;
        }
        let Some(pending) = self.pending else {
            return WorldInteractionStep::Fault;
        };
        if self.validation < self.selected_len {
            let target = self.selected[usize::from(self.validation)].expect("gumball selected target");
            self.validation += 1;
            return if self.live && self.streamed || target.text(gumball_state, interaction_state, scene_state, self.mode).is_some() { WorldInteractionStep::Pending } else { WorldInteractionStep::Stale };
        }
        let Some((local_x, local_y, viewport)) = pointer_in_pick_rect(session_state, pending.x, pending.y) else {
            self.pending = None;
            self.validation = 0;
            return WorldInteractionStep::Pending;
        };
        let camera = camera_state.orbit.to_camera();
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        let eye = gumball_eye(&camera, self.pivot);
        self.translate = Vec3::ZERO;
        self.angle = 0.0;
        self.scale = Vec3::new(1.0, 1.0, 1.0);
        if self.handle.is_translate() {
            if let Some(axis) = self.handle.axis_dir() {
                if let Some(current) = gumball_project_ray_onto_axis(origin, direction, self.pivot, axis, eye) {
                    self.translate = axis.normalize().scale(current - self.anchor);
                }
            } else if let Some(normal) = self.handle.plane_normal() {
                if let Some(current) = ray_plane_point(origin, direction, self.pivot, normal) {
                    self.translate = current.sub(self.start);
                }
            }
        } else if self.handle.is_rotate() {
            if let Some(normal) = self.handle.plane_normal() {
                if let Some(current) = ray_plane_point(origin, direction, self.pivot, normal) {
                    self.angle = axis_rotate_angle(self.start, current.sub(self.pivot), normal);
                }
            }
        } else if let Some(axis) = self.handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, direction, self.pivot, axis, eye) {
                let factor = if self.anchor.abs() > 1e-4 { (current / self.anchor).clamp(0.05, 20.0) } else { 1.0 };
                match self.handle {
                    GumballHandle::ScaleX => self.scale.x = factor,
                    GumballHandle::ScaleY => self.scale.y = factor,
                    GumballHandle::ScaleZ => self.scale.z = factor,
                    _ => {}
                }
            }
        }
        self.pending = None;
        self.validation = 0;
        WorldInteractionStep::Complete
    }

    pub fn close_step(&mut self) -> bool {
        if self.pending.take().is_some() {
            self.validation = 0;
            return false;
        }
        if self.selected_len > 0 {
            self.selected_len -= 1;
            self.selected[usize::from(self.selected_len)] = None;
            return false;
        }
        self.selected_bytes = 0;
        true
    }
}

pub struct WorldGumballCommitJob {
    pub generation: u64,
    pub gesture: WorldGumballGesture,
    pub phase: WorldGumballPhase,
    pub claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    pub draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    pub stage: u16,
    pub complete: bool,
}

pub const WORLD_BRUSH_COPY_CHUNK_BYTES: usize = 256;

#[derive(Clone, Copy)]
enum WorldBrushScale {
    Null,
    Number(f64),
    Array { values: [f64; 3], len: u8 },
}

pub struct WorldBrushCommitJob {
    pub generation: u64,
    pub revision: u64,
    pub bytes: Box<[u8; WORLD_INTERACTION_BYTE_CAPACITY]>,
    pub target: WorldInteractionSpan,
    pub kind: WorldInteractionSpan,
    source_index: usize,
    pub origin: [f64; 3],
    pub orientation: [f64; 4],
    pub scale: WorldBrushScale,
    copy_field: u8,
    copy_offset: u16,
    pub validating: bool,
    pub claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    pub draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    pub stage: u8,
    scale_stage: u8,
    pub complete: bool,
}

impl WorldBrushCommitJob {
    pub fn new(session_state: &WorldSessionState, gumball_state: &WorldGumballState, generation: u64) -> Result<Option<Self>, ui_wgpu::wgpu::BoundedActionFault> {
        let Some(preview) = gumball_state.brush_preview.as_ref() else {
            return Ok(None);
        };
        let (Some(target), Some(kind), Some(source_index)) = (preview.target_vortex_full_id.as_deref(), preview.object_kind_id.as_deref(), preview.source_vortex_index) else {
            return Ok(None);
        };
        if source_index > u32::MAX as usize {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        if target.len() > ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY || kind.len() > ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        let kind_start = target.len();
        kind_start.checked_add(kind.len()).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let scale = match preview.scale.as_ref() {
            None | Some(serde_json::Value::Null) => WorldBrushScale::Null,
            Some(serde_json::Value::Number(value)) => WorldBrushScale::Number(value.as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?),
            Some(serde_json::Value::Array(values)) if values.len() <= 3 => {
                let mut scalars = [0.0; 3];
                for index in 0..values.len() {
                    scalars[index] = values[index].as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                }
                WorldBrushScale::Array { values: scalars, len: values.len() as u8 }
            }
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        };
        Ok(Some(Self {
            generation,
            revision: session_state.interaction_revision,
            bytes: Box::new([0; WORLD_INTERACTION_BYTE_CAPACITY]),
            target: WorldInteractionSpan { start: 0, len: target.len() as u16 },
            kind: WorldInteractionSpan { start: kind_start as u16, len: kind.len() as u16 },
            source_index,
            origin: preview.origin.unwrap_or([0.0, 0.0, 0.0]),
            orientation: preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
            scale,
            copy_field: 0,
            copy_offset: 0,
            validating: false,
            claim: None,
            draft: None,
            stage: 0,
            scale_stage: 0,
            complete: false,
        }))
    }

    pub fn text(&self, span: WorldInteractionSpan) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        let start = usize::from(span.start);
        let end = start.checked_add(usize::from(span.len)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        std::str::from_utf8(self.bytes.get(start..end).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }

    pub fn source<'a>(&self, gumball_state: &'a WorldGumballState) -> Result<(&'a str, &'a str), ui_wgpu::wgpu::BoundedActionFault> {
        let preview = gumball_state.brush_preview.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let target = preview.target_vortex_full_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let kind = preview.object_kind_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if target.len() != usize::from(self.target.len) || kind.len() != usize::from(self.kind.len) || preview.source_vortex_index != Some(self.source_index) {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        Ok((target, kind))
    }

    fn copy_step(&mut self, gumball_state: &WorldGumballState) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        let (target, kind) = self.source(gumball_state)?;
        let (source, span) = if self.copy_field == 0 { (target.as_bytes(), self.target) } else { (kind.as_bytes(), self.kind) };
        let offset = usize::from(self.copy_offset);
        if offset == source.len() {
            self.copy_field += 1;
            self.copy_offset = 0;
            if self.copy_field == 2 {
                if self.validating {
                    return Ok(true);
                }
                self.validating = true;
                self.copy_field = 0;
            }
            return Ok(false);
        }
        let take = (source.len() - offset).min(WORLD_BRUSH_COPY_CHUNK_BYTES);
        let destination = usize::from(span.start) + offset;
        let existing = &mut self.bytes[destination..destination + take];
        if self.validating {
            if existing != &source[offset..offset + take] {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
        } else {
            existing.copy_from_slice(&source[offset..offset + take]);
        }
        self.copy_offset += take as u16;
        Ok(false)
    }

    pub fn credit(&self, session_state: &WorldSessionState) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        ui_wgpu::wgpu::checked_action_string_bytes(&[&session_state.controller_id, "addBrushObject", "targetVortexFullId", self.text(self.target)?, "objectKindId", self.text(self.kind)?, "sourceVortexIndex", "origin", "orientation", "scale"])
    }

    pub fn step(&mut self, session_state: &WorldSessionState, gumball_state: &WorldGumballState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if !self.complete {
            self.complete = self.copy_step(gumball_state)?;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.claim.is_none() {
            self.claim = Some(input.claim_action(self.credit(session_state)?)?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() {
            self.draft = Some(input.draft_claimed_action(self.claim.expect("brush claim"), &session_state.controller_id, "addBrushObject")?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let target_start = usize::from(self.target.start);
        let target_end = target_start + usize::from(self.target.len);
        let kind_start = usize::from(self.kind.start);
        let kind_end = kind_start + usize::from(self.kind.len);
        let target = std::str::from_utf8(&self.bytes[target_start..target_end]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let kind = std::str::from_utf8(&self.bytes[kind_start..kind_end]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let draft = self.draft.as_mut().expect("brush draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("targetVortexFullId"), target)?,
            2 => draft.builder().string(Some("objectKindId"), kind)?,
            3 => draft.builder().number(Some("sourceVortexIndex"), self.source_index as f64)?,
            4 => draft.builder().begin_array(Some("origin"))?,
            5..=7 => draft.builder().number(None, self.origin[usize::from(self.stage - 5)])?,
            8 => draft.builder().end_container()?,
            9 => draft.builder().begin_array(Some("orientation"))?,
            10..=13 => draft.builder().number(None, self.orientation[usize::from(self.stage - 10)])?,
            14 => draft.builder().end_container()?,
            15 => match self.scale {
                WorldBrushScale::Null => draft.builder().null(Some("scale"))?,
                WorldBrushScale::Number(value) => draft.builder().number(Some("scale"), value)?,
                WorldBrushScale::Array { .. } => draft.builder().begin_array(Some("scale"))?,
            },
            16 if matches!(self.scale, WorldBrushScale::Array { .. }) => {
                let WorldBrushScale::Array { values, len } = self.scale else { unreachable!("array scale") };
                if self.scale_stage < len {
                    draft.builder().number(None, values[usize::from(self.scale_stage)])?;
                    self.scale_stage += 1;
                    context.consume_fuel(1);
                    return Ok(WorldInteractionStep::Pending);
                }
                draft.builder().end_container()?;
            }
            16 => draft.builder().end_container()?,
            17 if matches!(self.scale, WorldBrushScale::Array { .. }) => draft.builder().end_container()?,
            _ => {
                let prepared = self.draft.take().expect("terminal brush draft").finish()?;
                input.publish_prepared_claimed_action(prepared)?;
                self.claim = None;
                return Ok(WorldInteractionStep::Complete);
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(claim) = self.claim.take() {
            if input.release_action_claim(claim).is_err() {
                self.claim = Some(claim);
            }
            return false;
        }
        true
    }
}

impl WorldGumballCommitJob {
    pub fn new(generation: u64, gesture: WorldGumballGesture, phase: WorldGumballPhase) -> Self {
        Self { generation, gesture, phase, claim: None, draft: None, stage: 0, complete: false }
    }

    /// 🔢️ How many motion numbers this gesture's verb carries: `dx dy dz`, `ax ay az angle` or `sx sy sz`.
    fn motion_len(&self) -> usize {
        if self.gesture.handle.is_rotate() {
            4
        } else {
            3
        }
    }

    pub fn action_id(&self) -> &'static str {
        gumball_handle_action_id(self.gesture.handle)
    }

    fn string_bytes(&self, session_state: &WorldSessionState) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        let keys: &[&str] = if self.gesture.handle.is_translate() {
            &["surfaceId", "windowId", "mode", "ids", "dx", "dy", "dz"]
        } else if self.gesture.handle.is_rotate() {
            &["surfaceId", "windowId", "mode", "ids", "ax", "ay", "az", "angle"]
        } else {
            &["surfaceId", "windowId", "mode", "ids", "sx", "sy", "sz"]
        };
        let mut bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&session_state.controller_id, self.action_id(), &session_state.surface_id, &session_state.surface_id, self.gesture.mode.map_or("mesh", WorldComponentKind::as_str)])?;
        for key in keys {
            bytes = bytes.checked_add(key.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        }
        bytes = bytes.checked_add(usize::from(self.gesture.selected_bytes)).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        for (key, value) in self.phase.tail() {
            bytes = bytes.checked_add(key.len() + value.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        }
        if bytes > WORLD_INTERACTION_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        Ok(bytes)
    }

    pub fn numeric(&self, index: usize) -> Option<(&'static str, f64)> {
        if self.gesture.handle.is_translate() {
            [("dx", self.gesture.translate.x as f64), ("dy", self.gesture.translate.y as f64), ("dz", self.gesture.translate.z as f64)].get(index).copied()
        } else if self.gesture.handle.is_rotate() {
            let axis = self.gesture.handle.axis_dir().unwrap_or(Vec3::ZERO);
            [("ax", axis.x as f64), ("ay", axis.y as f64), ("az", axis.z as f64), ("angle", self.gesture.angle as f64)].get(index).copied()
        } else {
            [("sx", self.gesture.scale.x as f64), ("sy", self.gesture.scale.y as f64), ("sz", self.gesture.scale.z as f64)].get(index).copied()
        }
    }

    /// 🪟️ A transform verb is WINDOW-OWNED, exactly like `setCamera`: the app declares it on
    /// its preview window kinds only, and the shell resolves `ActionAddress::window_instance_id`
    /// from this argument before it falls back to the focused window
    /// (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `dispatch_action`). A World3d surface is keyed BY its
    /// window instance, so its own id is that address. Without it a gumball drag in a
    /// multi-window mode was addressed to whatever had focus, measured verbatim on 6118 as
    /// `handle_action promise failed: window kind generation3d-generate-form does not own
    /// action translateSelection` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn step(&mut self, session_state: &WorldSessionState, gumball_state: &WorldGumballState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.phase == WorldGumballPhase::Skip {
            self.complete = true;
        }
        if self.complete {
            if self.gesture.selected_len > 0 {
                self.gesture.selected_len -= 1;
                self.gesture.selected[usize::from(self.gesture.selected_len)] = None;
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            self.gesture.selected_bytes = 0;
            return Ok(WorldInteractionStep::Complete);
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if self.claim.is_none() {
            let claim = input.claim_action(self.string_bytes(session_state)?)?;
            self.claim = Some(claim);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() {
            self.draft = Some(input.draft_claimed_action(self.claim.expect("gumball claim"), &session_state.controller_id, self.action_id())?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let selected_end = 5 + self.gesture.selected_len;
        let selected = if self.stage >= 5 && self.stage < selected_end {
            let target = self.gesture.selected[usize::from(self.stage - 5)].as_ref().expect("gumball selected target");
            Some(if self.gesture.live { target.captured_text() } else { target.text(gumball_state, interaction_state, scene_state, self.gesture.mode).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)? })
        } else {
            None
        };
        let index = (self.stage > selected_end).then(|| usize::from(self.stage - selected_end - 1));
        let numeric = index.and_then(|index| self.numeric(index));
        let tail = index.and_then(|index| index.checked_sub(self.motion_len())).and_then(|index| self.phase.tail().get(index).copied());
        let draft = self.draft.as_mut().expect("gumball draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("surfaceId"), &session_state.surface_id)?,
            2 => draft.builder().string(Some("windowId"), &session_state.surface_id)?,
            3 => draft.builder().string(Some("mode"), self.gesture.mode.map_or("mesh", WorldComponentKind::as_str))?,
            4 => draft.builder().begin_array(Some("ids"))?,
            stage if usize::from(stage - 5) < usize::from(self.gesture.selected_len) => {
                draft.builder().string(None, selected.expect("selected id resolved"))?;
            }
            stage if stage == 5 + self.gesture.selected_len => draft.builder().end_container()?,
            _ => {
                if let Some((key, value)) = numeric {
                    draft.builder().number(Some(key), value)?;
                } else if let Some((key, value)) = tail {
                    draft.builder().string(Some(key), value)?;
                } else {
                    draft.builder().end_container()?;
                    let prepared = self.draft.take().expect("complete gumball draft").finish()?;
                    input.publish_prepared_claimed_action(prepared)?;
                    self.claim = None;
                    self.complete = true;
                    context.consume_fuel(1);
                    return Ok(WorldInteractionStep::Pending);
                }
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(claim) = self.claim.take() {
            if input.release_action_claim(claim).is_err() {
                self.claim = Some(claim);
            }
            return false;
        }
        self.gesture.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && self.claim.is_none() && self.draft.is_none() && self.gesture.selected_len == 0 && self.gesture.selected_bytes == 0 && self.gesture.pending.is_none()
    }
}
//#endregion 🧵️WorldInteractionTransaction

//#region 🧵️WorldInteractionTransaction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionAuthorityStep {
    Idle,
    Pending,
    OutputBlocked,
    Complete,
    Stale,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldRayPickPurpose {
    Instance,
    Hover,
    Paint,
    Surface,
}

#[derive(Clone, Copy, Debug)]
struct WorldRayHitRef {
    pub mesh: WorldInteractionMeshToken,
    pub draw: u16,
    pub instance: u16,
    pub triangle: Option<u32>,
    pub distance: f32,
    pub bary_u: f32,
    pub bary_v: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub struct WorldRayPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub purpose: WorldRayPickPurpose,
    pub origin: Vec3,
    pub direction: Vec3,
    pub local_x: f32,
    pub local_y: f32,
    pub viewport: Rect,
    pub view_projection: Mat4,
    pub projection_spec: Viewport3dProjectionSpec,
    pub draw: usize,
    pub instance: usize,
    pub primitive: usize,
    pub mesh: Option<WorldInteractionMeshToken>,
    pub mesh_probe: u16,
    pub merge: u8,
    pub best: Option<WorldRayHitRef>,
    /// 🧲️ How many of `state.pick_targets` this cursor has scored — the sub-object PRE-PASS, one
    /// row per step, run before the first draw so the two compete in one ordering.
    pub pick_target: usize,
    /// 🧲️ `(score, index)` of the winning pick target, React's `targetRayScore` ordering.
    pub best_target: Option<(f32, u16)>,
    pub complete: bool,
    pub faulted: bool,
}

impl WorldRayPickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, generation: u64, purpose: WorldRayPickPurpose, x: f32, y: f32) -> Option<Self> {
        let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
        let camera = camera_state.orbit.to_camera();
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        Some(Self {
            revision: session_state.interaction_revision,
            generation,
            purpose,
            origin,
            direction,
            local_x,
            local_y,
            viewport,
            view_projection: ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, viewport.w, viewport.h),
            projection_spec: camera_state.projection_spec,
            draw: 0,
            instance: 0,
            primitive: 0,
            mesh: None,
            mesh_probe: 0,
            merge: 0,
            best: None,
            pick_target: 0,
            best_target: None,
            complete: false,
            faulted: false,
        })
    }

    /// ⏭️ Advances past a draw the ray cannot test — one whose mesh the guest has not published yet.
    fn skip_draw(&mut self) {
        self.draw += 1;
        self.instance = 0;
        self.primitive = 0;
        self.mesh = None;
        self.mesh_probe = 0;
    }

    /// 🧲️ Sub-object PRE-PASS. One retained pick target per step (the set is capped at
    /// `WORLD_PICK_TARGET_CAPACITY` when the lane is parsed, so this loop is bounded by
    /// construction), scored with React's own `targetRayScore`. A `Paint`/`Surface` gesture aims
    /// at the model itself, never at a pick proxy, so it skips the pass entirely.
    ///
    /// 🫥️ Unpublished mesh ⇒ an uninteractable draw, never a fault — see the same arm of
    /// `WorldInteractionRegistryBuildCursor::step`.
    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if interaction_state.interaction_meshes.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if matches!(self.purpose, WorldRayPickPurpose::Instance | WorldRayPickPurpose::Hover) && self.pick_target < selection_state.pick_targets.len() {
            let index = self.pick_target;
            self.pick_target += 1;
            let target = &selection_state.pick_targets[index];
            if target.is_selectable() {
                if let (Some(score), Ok(slot)) = (world_pick_target_ray_score(target, self.origin, self.direction), u16::try_from(index)) {
                    if self.best_target.is_none_or(|(best, _)| score < best) {
                        self.best_target = Some((score, slot));
                    }
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let Some(draw) = scene_state.draws.get(self.draw) else {
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        if self.mesh.is_none() {
            if draw.mesh_key.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY || usize::from(self.mesh_probe) == WORLD_INTERACTION_MESH_CAPACITY {
                self.faulted = true;
                return WorldInteractionStep::Fault;
            }
            let slot = (WorldInteractionMeshRegistry::hash(&draw.mesh_key) + usize::from(self.mesh_probe)) % WORLD_INTERACTION_MESH_CAPACITY;
            match interaction_state.interaction_meshes.slots[slot] {
                Some(entry) if entry.id.as_str() == draw.mesh_key => {
                    self.mesh = Some(WorldInteractionMeshToken { slot: slot as u16, generation: entry.generation });
                    self.mesh_probe = 0;
                }
                Some(_) => self.mesh_probe += 1,
                None => {
                    self.skip_draw();
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.mesh.expect("world pick mesh token resolved above");
        let Some(admitted) = interaction_state.interaction_meshes.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        if admitted.id.as_str() != draw.mesh_key || admitted.version != live_world3d_mesh_version(scene_state, &draw.mesh_key) {
            return WorldInteractionStep::Stale;
        }
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            self.skip_draw();
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let Ok(schema) = mesh.schema() else {
            return WorldInteractionStep::Stale;
        };
        if admitted.vertices != schema.vertices || admitted.triangles != schema.indices / 3 {
            return WorldInteractionStep::Stale;
        }
        let Some(instance) = draw.instances.get(self.instance) else {
            self.draw += 1;
            self.instance = 0;
            self.primitive = 0;
            self.mesh = None;
            self.mesh_probe = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        if schema.indices == 0 && matches!(self.purpose, WorldRayPickPurpose::Instance | WorldRayPickPurpose::Hover) {
            let count = if schema.edges > 0 { schema.edges } else if schema.vertex_ids == schema.vertices { schema.vertices } else { 0 };
            if self.primitive >= count as usize {
                self.instance += 1;
                self.primitive = 0;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let index = self.primitive as u32;
            let project = |point| ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, point, self.viewport.w, self.viewport.h);
            let point = if schema.edges > 0 {
                mesh.edge(index).ok().and_then(|edge| {
                    let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    let segment = ui_wgpu::wgpu::projection_spec_project_segment(self.view_projection, self.projection_spec, a, b, self.viewport.w, self.viewport.h)?;
                    let [screen_a, screen_b] = segment.screen;
                    (ui_wgpu::wgpu::screen_segment_distance(self.local_x, self.local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]) <= PICK_EDGE_SCREEN_PX).then(|| ui_wgpu::wgpu::ray_segment_closest(self.origin, self.direction, segment.points[0], segment.points[1]).map(|closest| closest.segment_point)).flatten()
                })
            } else if world_mesh_component_id(mesh, Mesh3dField::VertexIds, index) != u32::MAX {
                world_mesh_vertex(mesh, index).and_then(|point| {
                    let point = instance.model.transform_point(point);
                    let screen = project(point)?;
                    ((screen[0] - self.local_x).hypot(screen[1] - self.local_y) <= PICK_VERTEX_SCREEN_PX).then_some(point)
                })
            } else { None };
            if let Some(point) = point {
                let distance = point.sub(self.origin).dot(self.direction);
                if distance.is_finite() && distance >= 0.0 && self.best.is_none_or(|best| distance < best.distance) {
                    let (Ok(draw), Ok(instance)) = (u16::try_from(self.draw), u16::try_from(self.instance)) else { self.faulted = true; return WorldInteractionStep::Fault; };
                    self.best = Some(WorldRayHitRef { mesh: token, draw, instance, triangle: None, distance, bary_u: 0.0, bary_v: 0.0, point, normal: Vec3::ZERO });
                }
            }
            self.primitive += 1;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let Some(indices) = u32::try_from(self.primitive).ok().and_then(|triangle| world_mesh_triangle(mesh, triangle)) else {
            self.instance += 1;
            self.primitive = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let Some(a) = world_mesh_vertex(mesh, indices[0]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let Some(b) = world_mesh_vertex(mesh, indices[1]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let Some(c) = world_mesh_vertex(mesh, indices[2]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let a = instance.model.transform_point(a);
        let b = instance.model.transform_point(b);
        let c = instance.model.transform_point(c);
        if let Some((distance, bary_u, bary_v)) = world_ray_triangle_barycentric(self.origin, self.direction, a, b, c) {
            if self.best.is_none_or(|best| distance < best.distance) {
                let point = self.origin.add(self.direction.scale(distance));
                let mut normal = b.sub(a).cross(c.sub(a));
                if normal.length() > 1e-6 {
                    normal = normal.normalize();
                }
                let Ok(draw_index) = u16::try_from(self.draw) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                let Ok(instance_index) = u16::try_from(self.instance) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                let Ok(triangle_index) = u32::try_from(self.primitive) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                self.best = Some(WorldRayHitRef { mesh: token, draw: draw_index, instance: instance_index, triangle: Some(triangle_index), distance, bary_u, bary_v, point, normal });
            }
        }
        self.primitive += 1;
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    /// 🧲️ ONE ordering across the sub-object proxies and the instance meshes, exactly as r3f
    /// raycasts `SpatialPickHitTarget` and the model in the same scene: the lower score wins, and
    /// a tie goes to the pick target (React's `targetRayScore` already folded the kind's
    /// tie-break into the score, so `<=` here is that ladder's last rung).
    pub fn finish_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if self.faulted {
            return Err(WorldInteractionStep::Fault);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if let Some((score, slot)) = self.best_target {
            if score <= self.best.map_or(f32::INFINITY, |hit| hit.distance) {
                return self.pick_target_plan(session_state, selection_state, generation, slot);
            }
        }
        let Some(hit) = self.best else {
            return match self.purpose {
                WorldRayPickPurpose::Hover => {
                    if selection_state.local_hover_id.is_none() && selection_state.pick_hover_key.is_none() {
                        return Ok(None);
                    }
                    let mut plan = WorldInteractionPlan::new(self.revision, generation);
                    let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
                    let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                    let action = WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), None, None, Some(domain), None, None, None, None], numbers: [0.0; 10] };
                    plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
                }
                WorldRayPickPurpose::Instance => {
                    let mut plan = WorldInteractionPlan::new(self.revision, generation);
                    let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
                    let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                    let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                    let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                    let action = WorldFlatAction { kind: WorldFlatActionKind::Select, strings: [Some(controller), None, None, Some(domain), None, Some(merge), Some(method), None], numbers: [0.0; 10] };
                    plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
                }
                _ => Ok(None),
            };
        };
        let draw = scene_state.draws.get(usize::from(hit.draw)).ok_or(WorldInteractionStep::Fault)?;
        let admitted = interaction_state.interaction_meshes.resolve(hit.mesh).ok_or(WorldInteractionStep::Stale)?;
        if admitted.id.as_str() != draw.mesh_key || admitted.version != live_world3d_mesh_version(scene_state, &draw.mesh_key) {
            return Err(WorldInteractionStep::Stale);
        }
        let mesh = *scene_state.meshes.get(&draw.mesh_key).ok_or(WorldInteractionStep::Fault)?;
        let instance = draw.instances.get(usize::from(hit.instance)).ok_or(WorldInteractionStep::Fault)?;
        let target_id = instance_interaction_id(interaction_state, instance.id.as_str());
        let target_granularity_id = instance_interaction_granularity_id(session_state, interaction_state, instance.id.as_str());
        if self.purpose == WorldRayPickPurpose::Hover && selection_state.local_hover_id.as_deref() == Some(target_id) && selection_state.local_hover_granularity_id.as_deref() == Some(target_granularity_id) {
            return Ok(None);
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object = plan.push_string(target_id).ok_or(WorldInteractionStep::Fault)?;
        let action = match self.purpose {
            WorldRayPickPurpose::Paint => {
                let (u, v) = interpolate_mesh_uv(mesh, hit.triangle.ok_or(WorldInteractionStep::Fault)? as usize, hit.bary_u, hit.bary_v).map_err(|fault|if fault==ui_wgpu::wgpu::Mesh3dFault::Busy{WorldInteractionStep::Pending}else{WorldInteractionStep::Fault})?.ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction { kind: WorldFlatActionKind::PaintAt, strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None], numbers: [u as f64, v as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] }
            }
            WorldRayPickPurpose::Surface => WorldFlatAction {
                kind: WorldFlatActionKind::SurfacePlace,
                strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None],
                numbers: [hit.point.x as f64, hit.point.y as f64, hit.point.z as f64, hit.normal.x as f64, hit.normal.y as f64, hit.normal.z as f64, 0.0, 0.0, 0.0, 0.0],
            },
            WorldRayPickPurpose::Instance => {
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = plan.push_string(target_granularity_id).ok_or(WorldInteractionStep::Fault)?;
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Select,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), Some(merge), Some(method), None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
            WorldRayPickPurpose::Hover => {
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = plan.push_string(target_granularity_id).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Hover,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), None, None, None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    /// 🧲️ The `interactionSelect`/`interactionHover` one WINNING sub-object pick target dispatches —
    /// React's own payload for a `SpatialPickHitTarget` pick, with the target's KIND as the
    /// granularity (`vertex`/`edge`/`face`/`object`) and its kernel entity id as the target id.
    ///
    /// 🪪️ Alias-aware, so a guest that echoes `solid:foo` for a host that dispatched `object:foo`
    /// does not make every move re-publish the same hover.
    fn pick_target_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64, slot: u16) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        let target = selection_state.pick_targets.get(usize::from(slot)).ok_or(WorldInteractionStep::Stale)?;
        let key = target.key();
        if self.purpose == WorldRayPickPurpose::Hover && world_pick_keys_match(selection_state.pick_hover_key.as_deref(), Some(key.as_str())) {
            return Ok(None);
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object = plan.push_string(&target.id).ok_or(WorldInteractionStep::Fault)?;
        let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
        let granularity = plan.push_string(&target.kind).ok_or(WorldInteractionStep::Fault)?;
        let bare = if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 };
        let action = match self.purpose {
            WorldRayPickPurpose::Instance => {
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Select,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), Some(merge), Some(method), None],
                    numbers: [bare, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
            _ => WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), None, None, None], numbers: [bare, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] },
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && self.best.is_none() && self.best_target.is_none()
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.best_target.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldObjectPickPurpose {
    VortexHover,
    VortexSelect,
    ReferenceHover,
    ReferenceSelect,
}

pub struct WorldObjectPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub purpose: WorldObjectPickPurpose,
    pub origin: Vec3,
    pub direction: Vec3,
    pub slot: u16,
    pub merge: u8,
    pub best: Option<(WorldInteractionObjectToken, f32)>,
    pub complete: bool,
}

impl WorldObjectPickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, generation: u64, purpose: WorldObjectPickPurpose, x: f32, y: f32) -> Option<Self> {
        let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
        let camera = camera_state.orbit.to_camera();
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        Some(Self { revision: session_state.interaction_revision, generation, purpose, origin, direction, slot: 0, merge: 0, best: None, complete: false })
    }

    pub fn from_ray(revision: u64, generation: u64, purpose: WorldObjectPickPurpose, origin: Vec3, direction: Vec3) -> Self {
        Self { revision, generation, purpose, origin, direction, slot: 0, merge: 0, best: None, complete: false }
    }

    pub fn step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        let Some((index, entry)) = interaction_state.interaction_objects.next_occupied(self.slot) else {
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        self.slot = index as u16 + 1;
        if entry.revision != self.revision {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let distance = match (self.purpose, entry.kind) {
            (WorldObjectPickPurpose::VortexHover | WorldObjectPickPurpose::VortexSelect, WorldInteractionObjectKind::Vortex) => {
                let center = Vec3::new(entry.values[0], entry.values[1], entry.values[2]);
                let radius = entry.values[6].max(0.0);
                ray_aabb_slab(self.origin, self.direction, [center.x - radius, center.y - radius, center.z - radius], [center.x + radius, center.y + radius, center.z + radius])
            }
            (WorldObjectPickPurpose::ReferenceHover | WorldObjectPickPurpose::ReferenceSelect, WorldInteractionObjectKind::Reference) => {
                let center = Vec3::new(entry.values[0], entry.values[1], entry.values[2]);
                ray_plane_point(self.origin, self.direction, center, Vec3::new(0.0, 0.0, 1.0)).and_then(|hit| {
                    let offset = hit.sub(center);
                    (offset.x.abs() <= entry.values[3] * 0.5 && offset.y.abs() <= entry.values[4] * 0.5).then_some(self.origin.sub(hit).length())
                })
            }
            _ => None,
        };
        if let Some(distance) = distance {
            if self.best.is_none_or(|(_, best)| distance < best) {
                self.best = Some((WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision }, distance));
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub fn finish_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        let entry = match self.best {
            Some((token, _)) => Some(interaction_state.interaction_objects.resolve(token).ok_or(WorldInteractionStep::Stale)?),
            None => None,
        };
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let action = match self.purpose {
            WorldObjectPickPurpose::VortexHover => {
                let hit = entry.map(|entry| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
                WorldFlatAction { kind: WorldFlatActionKind::VortexHover, strings: [Some(controller), Some(surface), hit, None, None, None, None, None], numbers: [0.0; 10] }
            }
            WorldObjectPickPurpose::VortexSelect => {
                let Some(entry) = entry else {
                    return Ok(None);
                };
                let hit = plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)?;
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction { kind: WorldFlatActionKind::VortexSelect, strings: [Some(controller), Some(surface), Some(hit), Some(merge), None, None, None, None], numbers: [0.0; 10] }
            }
            WorldObjectPickPurpose::ReferenceHover => {
                let hit = entry.map(|entry| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
                if hit.is_none() && selection_state.local_hover_id.is_none() {
                    return Ok(None);
                }
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = match hit {
                    Some(_) => Some(plan.push_string("reference").ok_or(WorldInteractionStep::Fault)?),
                    None => None,
                };
                WorldFlatAction {
                    kind: WorldFlatActionKind::Hover,
                    strings: [Some(controller), Some(surface), hit, Some(domain), granularity, None, None, None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
            WorldObjectPickPurpose::ReferenceSelect => {
                let hit = entry.map(|entry| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = match hit {
                    Some(_) => Some(plan.push_string("reference").ok_or(WorldInteractionStep::Fault)?),
                    None => None,
                };
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Select,
                    strings: [Some(controller), Some(surface), hit, Some(domain), granularity, Some(merge), Some(method), None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldComponentKind {
    Vertex,
    Edge,
    Face,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldComponentPickPurpose {
    Hover,
    Select,
}

#[derive(Clone, Copy)]
pub struct WorldComponentHit {
    pub object: WorldInteractionObjectToken,
    pub id: u32,
    pub primary: f32,
    pub secondary: f32,
}

pub struct WorldComponentPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub purpose: WorldComponentPickPurpose,
    pub kind: WorldComponentKind,
    pub local_x: f32,
    pub local_y: f32,
    pub viewport: Rect,
    pub view_projection: Mat4,
    pub projection_spec: Viewport3dProjectionSpec,
    pub origin: Vec3,
    pub direction: Vec3,
    pub slot: u16,
    pub current: Option<WorldInteractionObjectToken>,
    pub topology: u32,
    pub merge: u8,
    pub best: Option<WorldComponentHit>,
    pub complete: bool,
}

impl WorldComponentPickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, generation: u64, purpose: WorldComponentPickPurpose, x: f32, y: f32) -> Option<Self> {
        let kind = match selection_state.granularity.as_str() {
            "vertex" => WorldComponentKind::Vertex,
            "edge" => WorldComponentKind::Edge,
            "face" | "component" => WorldComponentKind::Face,
            _ => return None,
        };
        if selection_state.active_object_id.as_ref().is_some_and(|id| id.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY) {
            return None;
        }
        let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
        let camera = camera_state.orbit.to_camera();
        let view_projection = ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, viewport.w, viewport.h);
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        Some(Self { revision: session_state.interaction_revision, generation, purpose, kind, local_x, local_y, viewport, view_projection, projection_spec: camera_state.projection_spec, origin, direction, slot: 0, current: None, topology: 0, merge: 0, best: None, complete: false })
    }

    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.current.is_none() {
            let index = usize::from(self.slot);
            let Some(entry) = interaction_state.interaction_objects.slots.get(index) else {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            self.slot += 1;
            if let Some(entry) = entry.as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance) {
                if selection_state.active_object_id.as_deref().is_none_or(|active| active == entry.id.as_str()) {
                    self.current = Some(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision });
                    self.topology = 0;
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.current.expect("component cursor current token");
        let Some(instance) = interaction_state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(mesh_token) = instance.mesh else {
            return WorldInteractionStep::Fault;
        };
        let Some(admitted) = interaction_state.interaction_meshes.resolve(mesh_token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(&mesh) = scene_state.meshes.get(admitted.id.as_str()) else {
            return WorldInteractionStep::Fault;
        };
        let Ok(schema) = mesh.schema() else { return WorldInteractionStep::Stale; };
        let count = match self.kind {
            WorldComponentKind::Vertex => if schema.vertex_ids == schema.vertices { admitted.vertices } else { 0 },
            WorldComponentKind::Edge => admitted.edges,
            WorldComponentKind::Face => admitted.triangles,
        };
        if self.topology >= count {
            self.current = None;
            self.topology = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let index = self.topology;
        self.topology += 1;
        if self.kind == WorldComponentKind::Vertex && world_mesh_component_id(mesh, Mesh3dField::VertexIds, index) == u32::MAX {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let candidate = match self.kind {
            WorldComponentKind::Vertex => mesh.vec3(Mesh3dField::Positions, index).ok().and_then(|point| {
                let world = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h).and_then(|screen| {
                    let dx = screen[0] - self.local_x;
                    let dy = screen[1] - self.local_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    (distance <= PICK_VERTEX_SCREEN_PX).then_some((distance, 0.0))
                })
            }),
            WorldComponentKind::Edge => mesh.edge(index).ok().and_then(|edge| {
                let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                let segment = ui_wgpu::wgpu::projection_spec_project_segment(self.view_projection, self.projection_spec, a, b, self.viewport.w, self.viewport.h)?;
                let [screen_a, screen_b] = segment.screen;
                let screen_distance = ui_wgpu::wgpu::screen_segment_distance(self.local_x, self.local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                if screen_distance > PICK_EDGE_SCREEN_PX { return None; }
                let closest = ui_wgpu::wgpu::ray_segment_closest(self.origin, self.direction, segment.points[0], segment.points[1])?;
                let depth = closest.segment_point.sub(self.origin).dot(self.direction);
                (depth.is_finite() && depth >= 0.0).then_some((depth, closest.distance))
            }),
            WorldComponentKind::Face => world_mesh_triangle(mesh, index).and_then(|indices| {
                let a = instance.model.transform_point(world_mesh_vertex(mesh, indices[0])?);
                let b = instance.model.transform_point(world_mesh_vertex(mesh, indices[1])?);
                let c = instance.model.transform_point(world_mesh_vertex(mesh, indices[2])?);
                world_ray_triangle_barycentric(self.origin, self.direction, a, b, c).map(|(distance, _, _)| (distance, 0.0))
            }),
        };
        if let Some((primary, secondary)) = candidate {
            let better = self.best.is_none_or(|best| primary < best.primary - 1e-4 || ((primary - best.primary).abs() <= 1e-4 && secondary < best.secondary));
            if better {
                let id = match self.kind {
                    WorldComponentKind::Vertex => world_mesh_component_id(mesh, Mesh3dField::VertexIds, index),
                    WorldComponentKind::Edge => world_mesh_component_id(mesh, Mesh3dField::EdgeIds, index),
                    WorldComponentKind::Face => world_mesh_component_id(mesh, Mesh3dField::FaceIds, index),
                };
                self.best = Some(WorldComponentHit { object: token, id, primary, secondary });
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub fn finish_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        let object = match self.best {
            Some(hit) => Some((interaction_state.interaction_objects.resolve(hit.object).ok_or(WorldInteractionStep::Stale)?, hit.id)),
            None => None,
        };
        if self.purpose == WorldComponentPickPurpose::Hover {
            let unchanged = match object {
                Some((entry, id)) => {
                    selection_state.hovered_component_id.as_deref().and_then(|value| value.parse::<u32>().ok()) == Some(id)
                        && selection_state.hovered_component_object_id.as_deref() == Some(entry.id.as_str())
                        && selection_state.hovered_component_mode.as_deref() == Some(self.kind.as_str())
                }
                None => selection_state.hovered_component_id.is_none() && selection_state.hovered_component_object_id.is_none() && selection_state.hovered_component_mode.is_none(),
            };
            if unchanged {
                return Ok(None);
            }
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object_span = object.map(|(entry, _)| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
        let mode = plan.push_string(self.kind.as_str()).ok_or(WorldInteractionStep::Fault)?;
        let merge = if self.purpose == WorldComponentPickPurpose::Select { Some(plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?) } else { None };
        let target = object.map(|(entry, group)| WorldComponentTarget::new(interaction_state, scene_state, entry, self.kind, group).map_err(|_| WorldInteractionStep::Fault)).transpose()?;
        let target_span = target.as_ref().map(|target| plan.push_string(target.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
        let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
        let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
        let action = WorldFlatAction {
            kind: if self.purpose == WorldComponentPickPurpose::Hover { WorldFlatActionKind::Hover } else { WorldFlatActionKind::Select },
            strings: [Some(controller), Some(surface), target_span, Some(domain), Some(mode), merge, Some(method), object_span],
            numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 1.0, 1.0, object.map_or(0.0, |(_, id)| id as f64), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.current.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}
//#endregion 🧵️WorldInteractionTransaction

/// 🔢️ The version `publish_world3d_mesh_lease` admitted this mesh key under. `mesh_versions` and
/// `interaction_meshes` are written in ONE transaction there, so this is the freshness witness every
/// interaction-registry lookup must match — never `SceneDraw3d::mesh_version`, which the typed
/// snapshot apply admits before the mesh's own publication ladder has run and which therefore stays
/// at its `0` placeholder for every bridged draw. Matching the stale field faulted the whole
/// interaction authority — every pick, hover, orbit and wheel on that surface.
pub fn live_world3d_mesh_version(scene_state: &WorldSceneState, mesh_key: &str) -> u64 {
    match scene_state.meshes.0.get(mesh_key){Some(WorldMeshSource::Native(source)) if !source.frozen=>source.loan.identity().cache_epoch,_=>scene_state.mesh_versions.get(mesh_key).copied().unwrap_or(0)}
}
