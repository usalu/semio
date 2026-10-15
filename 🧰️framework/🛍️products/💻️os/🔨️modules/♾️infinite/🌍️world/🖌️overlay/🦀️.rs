//! 🖌️ World pick and component overlays, gumball paint, camera projection, mesh admission and snapshot apply.

use super::*;

#[cfg(test)]
fn selection_mode_label(state: &World3dState) -> &'static str {
    match state.granularity.as_str() {
        "vertex" | "edge" | "face" => "component",
        _ => "mesh",
    }
}

pub(super) fn component_mode_active(state: &World3dState) -> bool {
    matches!(state.granularity.as_str(), "vertex" | "edge" | "face" | "component")
}

pub(super) const CLICK_DRAG_THRESHOLD_PX: f32 = 4.0;
pub(super) const PICK_VERTEX_SCREEN_PX: f32 = 14.0;
pub(super) const PICK_EDGE_SCREEN_PX: f32 = 18.0;
pub(super) const FACE_OVERLAY_OFFSET: f32 = 0.003;

pub(super) fn world_pick_rect(state: &World3dState) -> Rect {
    if state.pick_bounds.w > 0.0 && state.pick_bounds.h > 0.0 {
        state.pick_bounds
    } else {
        state.bounds
    }
}

pub(super) fn render_pick_viewport(state: &World3dState) -> Rect {
    state.bounds
}

pub(super) fn pointer_in_pick_rect(state: &World3dState, x: f32, y: f32) -> Option<(f32, f32, Rect)> {
    let clip = world_pick_rect(state);
    if !clip.contains(x, y) {
        return None;
    }
    let viewport = render_pick_viewport(state);
    if !viewport.contains(x, y) {
        return None;
    }
    Some((x - viewport.x, y - viewport.y, viewport))
}

pub(super) fn pick_targets_instance(state: &World3dState, instance_id: &str) -> bool {
    state.active_object_id.as_deref().is_none_or(|active_id| active_id == instance_id)
}

#[cfg(test)]
pub(super) fn pointer_drag_distance(state: &World3dState, x: f32, y: f32) -> f32 {
    let Some(start) = state.marquee_points.first() else {
        return 0.0;
    };
    let dx = x - start[0];
    let dy = y - start[1];
    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
pub(super) fn is_click_gesture(state: &World3dState, x: f32, y: f32) -> bool {
    pointer_drag_distance(state, x, y) <= CLICK_DRAG_THRESHOLD_PX
}

fn push_line_segment(lines: &mut Vec<LineVertex3d>, from: Vec3, to: Vec3, color: [f32; 4]) {
    lines.push(LineVertex3d { position: from.to_array(), color });
    lines.push(LineVertex3d { position: to.to_array(), color });
}

pub(super) const VERTEX_MARKER_MESH: &str = "vertex-marker";

/// 🔘️ The shell-owned placeholder plane the gumball's three drag quads are drawn with. Named here
/// beside the other shell placeholders because it must appear in THREE places that used to disagree:
/// the placeholder build, `sync_mesh_pool`'s `PINNED` list (it belongs to no document draw, so the
/// pool would otherwise evict it as stale every frame) and the GPU `ensure_mesh` the draw needs.
pub(super) const GUMBALL_PLANE_MESH: &str = "gumball-plane";
pub(super) const VERTEX_BASE_SCALE: f32 = 0.05;
pub(super) const VERTEX_HOVER_SCALE: f32 = 0.09;
const VERTEX_SELECT_SCALE: f32 = 0.09;

/// 🔘️ React's `WORLD_VERTEX_DOT_PX` / `WORLD_VERTEX_MARK_PX` (`🌐️World3dHost/🟦️.tsx:2499`) — the
/// plain and the hovered/selected vertex marker, in SCREEN PIXELS, drawn with
/// `sizeAttenuation={false}`.
///
/// 🩸️ These markers were world-unit crosses of `VERTEX_BASE_SCALE * 0.15` = 0.0075 world units,
/// which is sub-pixel on any scene larger than a few metres — the SAME defect React shipped and
/// fixed on lowpoly (`pointsMaterial size` in world units → `sizeAttenuation={false}` pixels,
/// ticket 26/08/29 round 4). The scales above stay as the RELATIVE weight of the three states.
pub(super) const WORLD_VERTEX_DOT_PX: f32 = 6.0;
const WORLD_VERTEX_MARK_PX: f32 = 11.0;

/// 📏️ World units one screen pixel spans at `distance` from the eye — the inverse of
/// `sizeAttenuation`. Perspective opens with distance; a parallel frustum is distance-invariant and
/// reads straight off `orthographic_half_extent`.
pub(super) fn world_units_per_pixel(camera: &Camera3d, viewport: Rect, distance: f32) -> f32 {
    let height = viewport.h.max(1.0);
    if camera.projection.is_parallel() {
        let (_, half_height) = camera.orthographic_half_extent(viewport.w, height);
        return (half_height * 2.0 / height).max(1e-6);
    }
    let half = (camera.fov_y * 0.5).tan() / camera.zoom.max(1e-6);
    (2.0 * distance.abs().max(1e-3) * half / height).max(1e-6)
}

/// 🔘️ Half the world-space span of one vertex marker cross at `centre` — React's pixel size,
/// converted through [`world_units_per_pixel`] so the marker keeps its apparent size at every zoom.
pub(super) fn vertex_marker_half_extent(camera: &Camera3d, viewport: Rect, centre: Vec3, scale: f32, mark_px: Option<f32>) -> f32 {
    let pixels = if scale > VERTEX_BASE_SCALE { mark_px.unwrap_or(WORLD_VERTEX_MARK_PX) } else { WORLD_VERTEX_DOT_PX };
    pixels * 0.5 * world_units_per_pixel(camera, viewport, camera.position.sub(centre).length())
}

/// 🖍️ Resolves the scene's sub-element highlight tokens through the live theme with the shared `world3d_sub_element_paint` table; a granularity without a token keeps the built-in palette.
pub(super) fn world3d_sub_element_paint_native(theme: &ui_wgpu::wgpu::Theme, highlight: Option<&ui_wgpu::wgpu::World3dHighlight>) -> WorldSubElementPaint {
    let defaults = ui_wgpu::wgpu::World3dSubElementDefaults { select: None, hover: None, edge_hover: None, edge_width: 0.0, vertex_mark_px: 0.0 };
    let paint = ui_wgpu::wgpu::world3d_sub_element_paint(highlight, |tone| Some(world3d_tone_rgba(theme, tone)), defaults);
    WorldSubElementPaint {
        face_select: paint.face_select,
        face_hover: paint.face_hover,
        edge_select: paint.edge_select,
        edge_hover: paint.edge_hover,
        vertex_select: paint.vertex_select,
        vertex_hover: paint.vertex_hover,
        vertex_mark_px: highlight.and_then(|highlight| highlight.vertex).and_then(|style| style.width_px).map(|width| width as f32),
    }
}

/// 🎨️ A theme tone token as an opaque colour: `neutral` is the foreground, `primary`/`secondary`/`tertiary` the celebration triad, the rest the outcome palette of the same name.
pub(super) fn world3d_tone_rgba(theme: &ui_wgpu::wgpu::Theme, tone: ui_wgpu::wgpu::World3dTone) -> [f32; 4] {
    use ui_wgpu::wgpu::World3dTone;
    let color = match tone {
        World3dTone::Neutral => theme.text,
        World3dTone::Primary => theme.celebrate[0],
        World3dTone::Secondary => theme.celebrate[1],
        World3dTone::Tertiary => theme.celebrate[2],
        World3dTone::Success => theme.success,
        World3dTone::Warning => theme.warning,
        World3dTone::Danger => theme.error,
        World3dTone::Info => Rgba::from_token(&ui_styling::colors::INFO),
    };
    [color.r, color.g, color.b, 1.0]
}

pub(super) fn component_overlay_color(id: &str, selected: &HashSet<String>, preview: &HashSet<String>, hovered: &Option<String>, tokens: (Option<[f32; 4]>, Option<[f32; 4]>)) -> Option<([f32; 4], f32)> {
    let (select, hover) = tokens;
    if preview.contains(id) {
        return Some((hover.unwrap_or([1.0, 0.85, 0.35, 1.0]), VERTEX_HOVER_SCALE));
    }
    if hovered.as_deref() == Some(id) {
        return Some((hover.map_or([0.35, 0.75, 1.0, 0.9], |color| [color[0], color[1], color[2], 0.9]), VERTEX_HOVER_SCALE));
    }
    if selected.contains(id) {
        return Some((select.unwrap_or([0.35, 0.75, 1.0, 1.0]), VERTEX_SELECT_SCALE));
    }
    None
}

pub(super) fn mesh_face_id(mesh: Mesh3dLease, tri_index: u32) -> String {
    world_mesh_component_id(mesh, Mesh3dField::FaceIds, tri_index).to_string()
}

fn face_component_mode_active(state: &World3dState) -> bool {
    state.selection_targets.face || state.granularity == "face"
}

pub(super) fn mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    world_mesh_vertex(mesh, index)
}

fn instance_hovered_component_id(state: &World3dState, instance_id: &str) -> Option<String> {
    if !pick_targets_instance(state, instance_id) {
        return None;
    }
    if state.hovered_component_mode.as_deref() != Some(state.granularity.as_str()) {
        return None;
    }
    if state.hovered_component_object_id.as_deref().is_some_and(|object_id| object_id != instance_id) {
        return None;
    }
    state.hovered_component_id.clone()
}

pub(super) fn append_component_vertex_spheres(_state: &mut World3dState) -> Vec<Instance3d> {
    Vec::new()
}

/// 📐️ Axis-aligned box wireframe as 12 line segments between `min` and `max`.
fn append_box_wireframe_lines(lines: &mut Vec<LineVertex3d>, min: Vec3, max: Vec3, color: [f32; 4]) {
    let corner = |ix: usize, iy: usize, iz: usize| Vec3::new(if ix == 0 { min.x } else { max.x }, if iy == 0 { min.y } else { max.y }, if iz == 0 { min.z } else { max.z });
    const EDGES: [((usize, usize, usize), (usize, usize, usize)); 12] = [
        ((0, 0, 0), (1, 0, 0)),
        ((1, 0, 0), (1, 1, 0)),
        ((1, 1, 0), (0, 1, 0)),
        ((0, 1, 0), (0, 0, 0)),
        ((0, 0, 1), (1, 0, 1)),
        ((1, 0, 1), (1, 1, 1)),
        ((1, 1, 1), (0, 1, 1)),
        ((0, 1, 1), (0, 0, 1)),
        ((0, 0, 0), (0, 0, 1)),
        ((1, 0, 0), (1, 0, 1)),
        ((1, 1, 0), (1, 1, 1)),
        ((0, 1, 0), (0, 1, 1)),
    ];
    for ((ax, ay, az), (bx, by, bz)) in EDGES {
        lines.push(LineVertex3d { position: corner(ax, ay, az).to_array(), color });
        lines.push(LineVertex3d { position: corner(bx, by, bz).to_array(), color });
    }
}

const WORLD_REFERENCE_LOCKED_OPACITY_SCALE: f32 = 0.35;
const WORLD_REFERENCE_SELECTED_CONTENT_OPACITY_SCALE: f32 = 0.5;
const WORLD_REFERENCE_OUTLINE_SCALE: f32 = 1.002;

#[derive(Clone, Copy)]
struct WorldReferenceVisual {
    pub(super) background: [f32; 4],
    pub(super) content_opacity: f32,
    pub(super) outline: Option<[f32; 4]>,
}

fn world_reference_state_id_matches(value: &str, id: &str) -> bool {
    value == id || value.strip_prefix("reference:") == Some(id)
}

pub(super) fn world_reference_visual(state: &World3dState, reference: &WorldReferenceRecord, theme: &ui_wgpu::wgpu::Theme) -> WorldReferenceVisual {
    let id = reference.id.as_deref().or(reference.url.as_deref()).unwrap_or_default();
    let locked = reference.locked.unwrap_or(false);
    let selected = !locked && state.selected_ids.iter().any(|value| world_reference_state_id_matches(value, id));
    let hovered = !locked && state.local_hover_id.as_deref().is_some_and(|value| world_reference_state_id_matches(value, id));
    let base = (reference.opacity.unwrap_or(1.0) as f32).clamp(0.0, 1.0) * if locked { WORLD_REFERENCE_LOCKED_OPACITY_SCALE } else { 1.0 };
    if selected {
        return WorldReferenceVisual {
            background: [theme.selected.r, theme.selected.g, theme.selected.b, theme.selected.a],
            content_opacity: base * WORLD_REFERENCE_SELECTED_CONTENT_OPACITY_SCALE,
            outline: Some([theme.celebrate[0].r, theme.celebrate[0].g, theme.celebrate[0].b, 1.0]),
        };
    }
    if hovered {
        return WorldReferenceVisual {
            background: [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, theme.row_hover.a],
            content_opacity: base,
            outline: Some([theme.celebrate[1].r, theme.celebrate[1].g, theme.celebrate[1].b, 0.9]),
        };
    }
    WorldReferenceVisual { background: [0.0; 4], content_opacity: base, outline: None }
}

pub(super) fn append_world_reference_outline(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], width: f32, height: f32, color: [f32; 4]) {
    let half_width = width * 0.5 * WORLD_REFERENCE_OUTLINE_SCALE;
    let half_height = height * 0.5 * WORLD_REFERENCE_OUTLINE_SCALE;
    let center = Vec3::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
    let bottom_left = center.add(Vec3::new(-half_width, -half_height, 0.0));
    let bottom_right = center.add(Vec3::new(half_width, -half_height, 0.0));
    let top_right = center.add(Vec3::new(half_width, half_height, 0.0));
    let top_left = center.add(Vec3::new(-half_width, half_height, 0.0));
    push_line_segment(lines, bottom_left, bottom_right, color);
    push_line_segment(lines, bottom_right, top_right, color);
    push_line_segment(lines, top_right, top_left, color);
    push_line_segment(lines, top_left, bottom_left, color);
}

/// 🎨️ Resolves one `pick_targets_json` style reference — a theme TOKEN id or a `#rrggbb` literal —
/// against this appearance. The token set is the CAD picking engine's `SpatialScenePalette`
/// (`✏️editor/⚙️engine/🧲️picking/🦀️.rs:1108`), resolved through the same CSS chain React does
/// (`🎨️styling/🖌️ui/🎨️.css`): `--color-changed-selected` → `--accent`, `--active-base` and
/// `--color-changed-hovered`/`--accent-secondary` → the primary/secondary celebrate stops, the
/// `--hover-*` grays → the one hover fill this theme carries. An unknown token falls back rather
/// than painting black.
fn world_pick_style_color(theme: &ui_wgpu::wgpu::Theme, reference: Option<&str>, fallback: [f32; 4]) -> [f32; 4] {
    let rgba = |color: Rgba| [color.r, color.g, color.b, color.a];
    let Some(reference) = reference.filter(|value| !value.is_empty()) else { return fallback };
    if reference.starts_with('#') {
        return parse_color(reference);
    }
    match reference {
        "--color-changed-selected" => rgba(theme.accent),
        "--active-base" => rgba(theme.celebrate[0]),
        "--color-changed-hovered" | "--accent-secondary" => rgba(theme.celebrate[1]),
        "--foreground" => rgba(theme.text),
        "--muted-foreground" => rgba(theme.text_muted),
        "--border-normal-color" => rgba(theme.border_normal),
        "--hover-base" | "--hover-panel" | "--hover-window" => rgba(theme.row_hover),
        _ => fallback,
    }
}

/// 🧲️ Paints the HOVERED sub-object pick target in its own `target_style` colour — the wgpu twin of
/// React's `SpatialPickTargetNode` for the one target under the pointer.
///
/// ⚖️ Only the hovered row, deliberately: the whole overlay is already drawn by the
/// `engagementPreview` lane (packet W2f), and painting every pick target a second time here would
/// double the line budget of a Concrete-Forest pane for nothing. A vertex target draws the same
/// screen-sized cross the component markers do; every other kind draws the wireframe of the box the
/// hit test itself uses, so what the user sees IS what was hit.
pub(super) fn append_pick_target_hover_lines(state: &World3dState, theme: &ui_wgpu::wgpu::Theme, camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) {
    let Some(key) = state.pick_hover_key.as_deref() else { return };
    let Some(target) = state.pick_targets.iter().find(|target| world_pick_keys_match(Some(target.key().as_str()), Some(key))) else { return };
    let style = target.style.as_ref();
    let mut color = world_pick_style_color(theme, style.and_then(|style| style.color.as_deref()), [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, 1.0]);
    color[3] = style.and_then(|style| style.opacity).map_or(color[3], |opacity| (opacity as f32).clamp(0.0, 1.0));
    let Some((min, max)) = world_pick_target_bounds(target) else { return };
    if target.kind == "vertex" {
        let centre = Vec3::new((min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5, (min[2] + max[2]) * 0.5);
        let d = vertex_marker_half_extent(camera, viewport, centre, VERTEX_HOVER_SCALE, None);
        push_line_segment(lines, centre.sub(Vec3::new(d, 0.0, 0.0)), centre.add(Vec3::new(d, 0.0, 0.0)), color);
        push_line_segment(lines, centre.sub(Vec3::new(0.0, d, 0.0)), centre.add(Vec3::new(0.0, d, 0.0)), color);
        push_line_segment(lines, centre.sub(Vec3::new(0.0, 0.0, d)), centre.add(Vec3::new(0.0, 0.0, d)), color);
        return;
    }
    append_box_wireframe_lines(lines, Vec3::new(min[0], min[1], min[2]), Vec3::new(max[0], max[1], max[2]), color);
}

/// 🤝️ Paints the `engagementPreview` lane as world-space lines — the wgpu twin of React's
/// `EngagementPreviewLayer` (`🌐️World3dHost/🟦️.tsx:4027`), which draws the same four kinds with
/// `colors.hover` (the `hover-interactive-fill` token, `theme.row_hover` here).
///
/// 🖍️ A `point` becomes a three-axis cross rather than a sphere and a `box-preview` a wireframe
/// rather than a wireframe mesh: this target has one line pipeline and no per-item mesh, and React
/// already draws the box `wireframe`, so the silhouette matches. The `0.05` floors on width/depth/
/// height and the "height extrudes up from `cornerA.z`, never from `cornerB.z`" rule are React's.
pub(super) fn append_engagement_preview_lines(state: &World3dState, lines: &mut Vec<LineVertex3d>, color: [f32; 4]) {
    const POINT_CROSS_EXTENT: f32 = 0.08;
    const PREVIEW_MIN_EXTENT: f64 = 0.05;
    for item in &state.engagement_preview {
        match item.kind.as_deref().unwrap_or_default() {
            "point" => {
                let Some(position) = item.position else { continue };
                let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
                for axis in [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)] {
                    lines.push(LineVertex3d { position: center.sub(axis.scale(POINT_CROSS_EXTENT)).to_array(), color });
                    lines.push(LineVertex3d { position: center.add(axis.scale(POINT_CROSS_EXTENT)).to_array(), color });
                }
            }
            "segment" => {
                let (Some(from), Some(to)) = (item.from, item.to) else { continue };
                lines.push(LineVertex3d { position: [from[0] as f32, from[1] as f32, from[2] as f32], color });
                lines.push(LineVertex3d { position: [to[0] as f32, to[1] as f32, to[2] as f32], color });
            }
            "box-preview" => {
                let (Some(corner_a), Some(corner_b)) = (item.corner_a, item.corner_b) else { continue };
                let half_width = (corner_b[0] - corner_a[0]).abs().max(PREVIEW_MIN_EXTENT) * 0.5;
                let half_depth = (corner_b[1] - corner_a[1]).abs().max(PREVIEW_MIN_EXTENT) * 0.5;
                let height = item.height.unwrap_or(PREVIEW_MIN_EXTENT).abs().max(PREVIEW_MIN_EXTENT);
                let center_x = (corner_a[0] + corner_b[0]) * 0.5;
                let center_y = (corner_a[1] + corner_b[1]) * 0.5;
                let min = Vec3::new((center_x - half_width) as f32, (center_y - half_depth) as f32, corner_a[2] as f32);
                let max = Vec3::new((center_x + half_width) as f32, (center_y + half_depth) as f32, (corner_a[2] + height) as f32);
                append_box_wireframe_lines(lines, min, max, color);
            }
            "linear-handle" => {
                let (Some(axis), Some(origin)) = (item.axis, item.origin) else { continue };
                let start = Vec3::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
                let end = start.add(Vec3::new(axis[0] as f32, axis[1] as f32, axis[2] as f32));
                lines.push(LineVertex3d { position: start.to_array(), color });
                lines.push(LineVertex3d { position: end.to_array(), color });
            }
            _ => {}
        }
    }
}

pub(super) fn append_component_overlays(state: &World3dState, theme: &ui_wgpu::wgpu::Theme, camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) {
    let wire_color = [0.55, 0.65, 0.8, 0.75];
    let outline_color = state.show_edges.then(|| environment_outline_color(&state.environment, theme)).flatten();
    let requested_wire = state.interaction_mode == "paint" || component_mode_active(state) || outline_color.is_some() || state.selection_targets.edge || (state.granularity == "mesh" && !state.component_ids.is_empty());
    {
        for draw in &state.draws {
            let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            let primary_wire = schema.indices == 0 && schema.edges > 0;
            let semantic = schema.edge_ids == schema.edges;
            if schema.edges == 0 || (!requested_wire && !primary_wire) || (outline_color.is_none() && !semantic && !primary_wire) {
                continue;
            }
            for instance in &draw.instances {
                let color = if primary_wire { mesh_style_paint(theme, resolve_mesh_style(world_instance_style(state, instance))).line } else { outline_color.unwrap_or(wire_color) };
                for edge_index in 0..schema.edges {
                    let Ok(edge) = mesh.edge(edge_index) else { continue };
                    let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    lines.push(LineVertex3d { position: a.to_array(), color });
                    lines.push(LineVertex3d { position: b.to_array(), color });
                }
            }
        }
    }
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok(schema) = mesh.schema() else { continue };
        if schema.edge_ids != schema.edges {
            continue;
        }
        for instance in &draw.instances {
            let selected = world_instance_selected_components(state,instance,&state.granularity);
            let preview: HashSet<String> = state.marquee_preview_ids.iter().filter(|_|state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
            let hovered = instance_hovered_component_id(state, &instance.id);
            if state.granularity.as_str() != "edge" {
                continue;
            }
            if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                continue;
            }
            for edge_index in 0..schema.edges {
                let Ok(edge) = mesh.edge(edge_index) else { continue };
                let id = world_mesh_component_id(mesh, Mesh3dField::EdgeIds, edge_index).to_string();
                let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered, (state.sub_element_paint.edge_select, state.sub_element_paint.edge_hover)) else {
                    continue;
                };
                let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                push_line_segment(lines, a, b, color);
            }
        }
    }
    if face_component_mode_active(state) {
        for draw in &state.draws {
            let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            for instance in &draw.instances {
                let selected = world_instance_selected_components(state,instance,&state.granularity);
                let preview: HashSet<String> = state.marquee_preview_ids.iter().filter(|_|state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
                let hovered = instance_hovered_component_id(state, &instance.id);
                if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                    continue;
                }
                for tri_index in 0..schema.indices / 3 {
                    let Some(tri) = world_mesh_triangle(mesh, tri_index) else { continue };
                    let id = mesh_face_id(mesh, tri_index);
                    let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered, (state.sub_element_paint.face_select, state.sub_element_paint.face_hover)) else {
                        continue;
                    };
                    let Some(verts) = Option::zip(Option::zip(mesh_vertex(mesh, tri[0]), mesh_vertex(mesh, tri[1])), mesh_vertex(mesh, tri[2]))
                        .map(|((a, b), c)| [instance.model.transform_point(a), instance.model.transform_point(b), instance.model.transform_point(c)])
                    else {
                        continue;
                    };
                    push_line_segment(lines, verts[0], verts[1], color);
                    push_line_segment(lines, verts[1], verts[2], color);
                    push_line_segment(lines, verts[2], verts[0], color);
                }
            }
        }
    }
    {
        let wire_color = [0.55, 0.65, 0.8, 0.9];
        for draw in &state.draws {
            let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            if schema.vertex_ids != schema.vertices { continue; }
            let primary_point = schema.indices == 0 && schema.edges == 0;
            if !primary_point && !state.selection_targets.vertex && state.granularity != "vertex" { continue; }
            for instance in &draw.instances {
                let selected = world_instance_selected_components(state,instance,"vertex");
                let preview: HashSet<String> = state.marquee_preview_ids.iter().filter(|_|state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
                let hovered = instance_hovered_component_id(state, &instance.id);
                for vertex_index in 0..schema.vertices {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                    let group = world_mesh_component_id(mesh, Mesh3dField::VertexIds, vertex_index);
                    if group == u32::MAX { continue; }
                    let id = group.to_string();
                    let center = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                    let base_color = if primary_point { mesh_style_paint(theme, resolve_mesh_style(world_instance_style(state, instance))).line } else { wire_color };
                    let vertex_tokens = (state.sub_element_paint.vertex_select, state.sub_element_paint.vertex_hover);
                    let (color, scale) = component_overlay_color(&id, &selected, &preview, &hovered, vertex_tokens).unwrap_or((base_color, VERTEX_BASE_SCALE));
                    if !primary_point && !state.selection_targets.vertex && component_overlay_color(&id, &selected, &preview, &hovered, vertex_tokens).is_none() {
                        continue;
                    }
                    let d = vertex_marker_half_extent(camera, viewport, center, scale, state.sub_element_paint.vertex_mark_px);
                    push_line_segment(lines, center.sub(Vec3::new(d, 0.0, 0.0)), center.add(Vec3::new(d, 0.0, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, d, 0.0)), center.add(Vec3::new(0.0, d, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, 0.0, d)), center.add(Vec3::new(0.0, 0.0, d)), color);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldFaceOverlayPhase {
    Count,
    Prepare,
    Allocate,
    Geometry,
    Indices,
    Seal,
    Publish,
    NextBucket,
    Complete,
}

#[derive(Clone, Copy)]
struct WorldFaceOverlayMatch {
    pub(super) draw: u16,
    pub(super) instance: u16,
    pub(super) triangle: u32,
    pub(super) category: u8,
    pub(super) hovered: bool,
}

#[derive(Clone, Copy, Default)]
pub(super) struct WorldFaceOverlayScan {
    pub(super) draw: u16,
    pub(super) instance: u16,
    pub(super) triangle: u32,
    pub(super) candidate: Option<u32>,
    pub(super) hovered: bool,
    preview: usize,
    pub(super) selected: usize,
}

pub(super) enum WorldFaceOverlayScanStep {
    Pending,
    Match(WorldFaceOverlayMatch),
    Complete,
}

impl WorldFaceOverlayScan {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn finish_candidate(&mut self, category: Option<u8>) -> WorldFaceOverlayScanStep {
        let matched = category.map(|category| WorldFaceOverlayMatch { draw: self.draw, instance: self.instance, triangle: self.triangle, category, hovered: self.hovered });
        self.triangle = self.triangle.saturating_add(1);
        self.candidate = None;
        self.hovered = false;
        self.preview = 0;
        self.selected = 0;
        matched.map_or(WorldFaceOverlayScanStep::Pending, WorldFaceOverlayScanStep::Match)
    }

    pub(super) fn step(&mut self, state: &World3dState) -> WorldFaceOverlayScanStep {
        if let Some(face_id) = self.candidate {
            let instance = &state.draws.get(usize::from(self.draw)).expect("retained overlay draw").instances[usize::from(self.instance)];
            if self.preview < state.marquee_preview_ids.len() {
                let matches = state.active_object_id.as_deref()==Some(&instance.id) && decimal_component_id_matches(&state.marquee_preview_ids[self.preview], face_id);
                self.preview += 1;
                return if matches { self.finish_candidate(Some(0)) } else { WorldFaceOverlayScanStep::Pending };
            }
            if self.hovered {
                return self.finish_candidate(Some(1));
            }
            if self.selected < state.gumball_selection_ids.len() {
                let matches = world_component_preview_target(state,&state.gumball_selection_ids[self.selected],"face").is_some_and(|(object,group)|object==instance.id && group==face_id);
                self.selected += 1;
                return if matches { self.finish_candidate(Some(2)) } else { WorldFaceOverlayScanStep::Pending };
            }
            let numeric = self.selected - state.gumball_selection_ids.len();
            if instance.component_source.is_none() && state.active_object_id.as_deref()==Some(&instance.id) && numeric < state.component_ids.len() {
                let matches = decimal_component_id_matches(&state.component_ids[numeric],face_id);
                self.selected += 1;
                return if matches { self.finish_candidate(Some(2)) } else { WorldFaceOverlayScanStep::Pending };
            }
            return self.finish_candidate(None);
        }
        if usize::from(self.draw) >= usize::from(state.draws.len) {
            return WorldFaceOverlayScanStep::Complete;
        }
        let Some(draw) = state.draws.get(usize::from(self.draw)) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if usize::from(self.instance) >= draw.instances.len() {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        let Ok(schema) = mesh.schema() else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if self.triangle >= schema.indices / 3 {
            self.instance += 1;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let face_id = world_mesh_component_id(mesh, Mesh3dField::FaceIds, self.triangle);
        self.candidate = Some(face_id);
        self.hovered = instance_hovered_component_matches(state, &draw.instances[usize::from(self.instance)].id, face_id);
        WorldFaceOverlayScanStep::Pending
    }
}

#[derive(Clone, Copy)]
struct WorldFaceOverlayGeometry {
    pub(super) vertices: [Vec3; 3],
    pub(super) normal: Vec3,
}

pub(super) enum WorldFaceOverlayMeshStep {
    Pending,
    Ready { index: usize, color: [f32; 4], key: String, lease: Mesh3dLease },
    Complete { generation: u64, revision: u64, draw_generation: u64, colors: [Option<[f32; 4]>; 3] },
    Stale,
    Fault,
}

pub(super) struct WorldFaceOverlayMeshCursor {
    pub(super) surface_id: String,
    pub(super) generation: u64,
    pub(super) revision: u64,
    pub(super) draw_generation: u64,
    pub(super) phase: WorldFaceOverlayPhase,
    scan: WorldFaceOverlayScan,
    pub(super) counts: [u32; 3],
    order: [u8; 3],
    order_len: u8,
    pub(super) published_colors: [Option<[f32; 4]>; 3],
    bucket: u8,
    geometry: Option<WorldFaceOverlayGeometry>,
    geometry_item: u8,
    index_item: u32,
    pub(super) owner: WorldPlaceholderOwner,
    retry_key: String,
    pub(super) close_started: bool,
    pub(super) faulted: bool,
    stale: bool,
}

impl WorldFaceOverlayMeshCursor {
    pub(super) fn new(surface_id: &str, generation: u64, revision: u64, draw_generation: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        if surface_id.len().checked_add("component-face-overlay::2".len()).is_none_or(|bytes| bytes > WORLD_DYNAMIC_ID_BYTE_CAPACITY) {
            return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity);
        }
        Ok(Self {
            surface_id: surface_id.to_owned(),
            generation,
            revision,
            draw_generation,
            phase: WorldFaceOverlayPhase::Count,
            scan: WorldFaceOverlayScan::default(),
            counts: [0; 3],
            order: [0; 3],
            order_len: 0,
            published_colors: [None; 3],
            bucket: 0,
            geometry: None,
            geometry_item: 0,
            index_item: 0,
            owner: WorldPlaceholderOwner::Empty,
            retry_key: String::new(),
            close_started: false,
            faulted: false,
            stale: false,
        })
    }

    pub(super) fn key(&self, index: usize) -> String {
        format!("component-face-overlay:{}:{}:{index}", self.surface_id, self.generation)
    }

    pub(super) fn token(&self) -> Result<Mesh3dWriteToken, ui_wgpu::wgpu::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_wgpu::wgpu::Mesh3dFault::Stale),
        }
    }

    pub(super) fn step(&mut self, state: &World3dState) -> WorldFaceOverlayMeshStep {
        if state.interaction_revision != self.revision || state.draw_generation != self.draw_generation {
            self.faulted = true;
            self.stale = true;
        }
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() {
                if self.stale {
                    WorldFaceOverlayMeshStep::Stale
                } else {
                    WorldFaceOverlayMeshStep::Fault
                }
            } else {
                WorldFaceOverlayMeshStep::Pending
            };
        }
        match self.step_live(state) {
            Ok(step) => step,
            Err(_) => {
                self.faulted = true;
                let _ = self.close_step();
                WorldFaceOverlayMeshStep::Pending
            }
        }
    }

    pub(super) fn step_live(&mut self, state: &World3dState) -> Result<WorldFaceOverlayMeshStep, ui_wgpu::wgpu::Mesh3dFault> {
        match self.phase {
            WorldFaceOverlayPhase::Count => match self.scan.step(state) {
                WorldFaceOverlayScanStep::Pending => {}
                WorldFaceOverlayScanStep::Match(found) => {
                    let category = usize::from(found.category);
                    if self.counts[category] == 0 {
                        self.order[usize::from(self.order_len)] = found.category;
                        self.order_len += 1;
                    }
                    self.counts[category] = self.counts[category].checked_add(1).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                }
                WorldFaceOverlayScanStep::Complete => {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Prepare;
                }
            },
            WorldFaceOverlayPhase::Prepare => {
                if self.bucket == self.order_len {
                    self.phase = WorldFaceOverlayPhase::Complete;
                } else {
                    let count = self.counts[usize::from(self.order[usize::from(self.bucket)])];
                    let vertices = count.checked_mul(3).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                    let indices = count.checked_mul(6).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                    let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(), 0, 1);
                    let (token, progress) = mesh3d_begin(self.generation + u64::from(self.bucket), self.revision, Mesh3dSchema::triangle_mesh(vertices, indices), grant)?;
                    self.owner = WorldPlaceholderOwner::Writing(token);
                    if !progress.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity); }
                    self.phase = WorldFaceOverlayPhase::Allocate;
                }
            }
            WorldFaceOverlayPhase::Allocate => {
                let token = self.token()?;
                let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_allocate_capacity_byte_demand(token)?, 0, 1);
                if world_mesh_ownership_complete(grant, mesh3d_allocate_step(token, grant)?)? {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Geometry;
                }
            }
            WorldFaceOverlayPhase::Geometry => {
                if let Some(geometry) = self.geometry {
                    let vertex = usize::from(self.geometry_item / 2);
                    let field = self.geometry_item % 2;
                    if field == 0 {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, geometry.vertices[vertex].to_array())?;
                    } else {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, geometry.normal.to_array())?;
                    }
                    self.geometry_item += 1;
                    if self.geometry_item == 6 {
                        self.geometry_item = 0;
                        self.geometry = None;
                    }
                } else {
                    match self.scan.step(state) {
                        WorldFaceOverlayScanStep::Pending => {}
                        WorldFaceOverlayScanStep::Match(found) if found.category == self.order[usize::from(self.bucket)] => {
                            self.geometry = Some(face_overlay_geometry(state, found).ok_or(ui_wgpu::wgpu::Mesh3dFault::Stale)?);
                        }
                        WorldFaceOverlayScanStep::Match(_) => {}
                        WorldFaceOverlayScanStep::Complete => {
                            self.index_item = 0;
                            self.phase = WorldFaceOverlayPhase::Indices;
                        }
                    }
                }
            }
            WorldFaceOverlayPhase::Indices => {
                let category = usize::from(self.order[usize::from(self.bucket)]);
                let total = self.counts[category] * 6;
                let pattern = [0, 1, 2, 0, 2, 1];
                let value = (self.index_item / 6) * 3 + pattern[(self.index_item % 6) as usize];
                mesh3d_write_u32(self.token()?, Mesh3dField::Indices, value)?;
                self.index_item += 1;
                if self.index_item == total {
                    self.phase = WorldFaceOverlayPhase::Seal;
                }
            }
            WorldFaceOverlayPhase::Seal => {
                self.owner = WorldPlaceholderOwner::Ready(mesh3d_seal(self.token()?)?);
                self.phase = WorldFaceOverlayPhase::Publish;
            }
            WorldFaceOverlayPhase::Publish => {
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_wgpu::wgpu::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                self.phase = WorldFaceOverlayPhase::NextBucket;
                let index = usize::from(self.bucket);
                let key = if self.retry_key.is_empty() { self.key(index) } else { std::mem::take(&mut self.retry_key) };
                return Ok(WorldFaceOverlayMeshStep::Ready { index, color: face_overlay_color(&state.sub_element_paint, self.order[index]), key, lease });
            }
            WorldFaceOverlayPhase::NextBucket => {
                self.bucket += 1;
                self.scan.reset();
                self.phase = WorldFaceOverlayPhase::Prepare;
            }
            WorldFaceOverlayPhase::Complete => {
                self.surface_id.clear();
                return Ok(WorldFaceOverlayMeshStep::Complete { generation: self.generation, revision: self.revision, draw_generation: self.draw_generation, colors: self.published_colors });
            }
        }
        Ok(WorldFaceOverlayMeshStep::Pending)
    }

    pub(super) fn close_step(&mut self) -> bool {
        self.geometry = None;
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_abort_demands(token, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_abort_step(token, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_close_demands(lease, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(lease, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.retry_key.pop().is_some() {
                    return false;
                }
                if self.surface_id.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        matches!(self.owner, WorldPlaceholderOwner::Empty) && self.surface_id.is_empty() && self.retry_key.is_empty() && self.geometry.is_none()
    }
}

#[cfg(not(test))]
impl Drop for WorldFaceOverlayMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "face overlay mesh cursor reached Drop before its exact authority witness");
    }
}

fn decimal_component_id_matches(value: &str, mut expected: u32) -> bool {
    if expected == 0 {
        return value == "0";
    }
    let digits = (expected.ilog10() + 1) as usize;
    if value.len() != digits || !value.is_ascii() {
        return false;
    }
    for digit in value.as_bytes().iter().rev() {
        if *digit != b'0' + (expected % 10) as u8 {
            return false;
        }
        expected /= 10;
    }
    true
}

fn instance_hovered_component_matches(state: &World3dState, instance_id: &str, face_id: u32) -> bool {
    pick_targets_instance(state, instance_id)
        && state.hovered_component_mode.as_deref() == Some(state.granularity.as_str())
        && state.hovered_component_object_id.as_deref().is_none_or(|object_id| object_id == instance_id)
        && state.hovered_component_id.as_deref().is_some_and(|id| decimal_component_id_matches(id, face_id))
}

pub(super) fn face_overlay_color(paint: &WorldSubElementPaint, category: u8) -> [f32; 4] {
    let with_alpha = |color: [f32; 4], alpha: f32| [color[0], color[1], color[2], alpha];
    match category {
        0 => with_alpha(paint.face_hover.unwrap_or([1.0, 0.85, 0.35, 1.0]), 0.36),
        1 => with_alpha(paint.face_hover.unwrap_or([0.35, 0.75, 1.0, 1.0]), 0.48),
        _ => with_alpha(paint.face_select.unwrap_or([0.35, 0.75, 1.0, 1.0]), 0.62),
    }
}

fn face_overlay_geometry(state: &World3dState, found: WorldFaceOverlayMatch) -> Option<WorldFaceOverlayGeometry> {
    let draw = state.draws.get(usize::from(found.draw))?;
    let instance = draw.instances.get(usize::from(found.instance))?;
    let mesh = *state.meshes.get(&draw.mesh_key)?;
    let triangle = world_mesh_triangle(mesh, found.triangle)?;
    let vertices = [instance.model.transform_point(mesh_vertex(mesh, triangle[0])?), instance.model.transform_point(mesh_vertex(mesh, triangle[1])?), instance.model.transform_point(mesh_vertex(mesh, triangle[2])?)];
    let normal = vertices[1].sub(vertices[0]).cross(vertices[2].sub(vertices[0])).normalize();
    let offset = if found.hovered { FACE_OVERLAY_OFFSET } else { FACE_OVERLAY_OFFSET * 0.5 };
    Some(WorldFaceOverlayGeometry { vertices: vertices.map(|vertex| vertex.add(normal.scale(offset))), normal })
}

pub(super) fn step_component_face_overlay_build(state: &mut World3dState) {
    let Some(mut cursor) = state.face_overlay_build.take() else { return };
    match cursor.step(state) {
        WorldFaceOverlayMeshStep::Pending => state.face_overlay_build = Some(cursor),
        WorldFaceOverlayMeshStep::Ready { index, color, key, lease } => match publish_world3d_mesh_lease(state, key, lease) {
            Ok(()) => {
                cursor.published_colors[index] = Some(color);
                state.face_overlay_build = Some(cursor);
            }
            Err(rejected) => {
                cursor.owner = WorldPlaceholderOwner::Ready(rejected.value);
                cursor.retry_key = rejected.id;
                cursor.phase = WorldFaceOverlayPhase::Publish;
                cursor.close_started = false;
                state.face_overlay_build = Some(cursor);
            }
        },
        WorldFaceOverlayMeshStep::Complete { generation, revision, draw_generation, colors } => {
            state.face_overlay_retired_generation = state.face_overlay_generation.replace(generation);
            state.face_overlay_colors = colors;
            state.face_overlay_applied_revision = revision;
            state.face_overlay_applied_draw_generation = draw_generation;
        }
        WorldFaceOverlayMeshStep::Stale => {}
        WorldFaceOverlayMeshStep::Fault => mark_world_dynamic_fault(state, WorldDynamicFault::Closing),
    }
}

pub(super) fn append_component_face_translucent_overlays(state: &mut World3dState, gpu: &mut World3dBuildContext, translucent: &mut Vec<SceneDraw3d>) {
    if !face_component_mode_active(state) {
        state.face_overlay_applied_revision = u64::MAX;
        state.face_overlay_applied_draw_generation = u64::MAX;
        if let Some(cursor) = state.face_overlay_build.as_mut() {
            cursor.faulted = true;
            cursor.stale = true;
            step_component_face_overlay_build(state);
            return;
        }
        if state.face_overlay_generation.is_some() {
            state.face_overlay_retired_generation = state.face_overlay_generation.take();
            state.face_overlay_colors = [None; 3];
        }
        return;
    }
    let had_build = state.face_overlay_build.is_some();
    step_component_face_overlay_build(state);
    if !had_build
        && state.face_overlay_build.is_none()
        && state.face_overlay_retired_generation.is_none()
        && (state.face_overlay_applied_revision != state.interaction_revision || state.face_overlay_applied_draw_generation != state.draw_generation)
        && state.dynamic_mesh_close.is_none()
        && state.dynamic_blocked_mesh.is_none()
        && state.snapshot_fault.is_none()
    {
        let generation = state.placeholder_generation.checked_add(3);
        match generation.and_then(|next| WorldFaceOverlayMeshCursor::new(&state.surface_id, state.placeholder_generation + 1, state.interaction_revision, state.draw_generation).ok().map(|cursor| (next, cursor))) {
            Some((next, cursor)) => {
                state.placeholder_generation = next;
                state.face_overlay_build = Some(cursor);
            }
            None => mark_world_dynamic_fault(state, WorldDynamicFault::ByteCapacity),
        }
    }
    let Some(generation) = state.face_overlay_generation else { return };
    for index in 0..3 {
        let Some(color) = state.face_overlay_colors[index] else { continue };
        let mesh_key = format!("component-face-overlay:{}:{generation}:{index}", state.surface_id);
        let Some(mesh) = state.meshes.get(&mesh_key) else { continue };
        let mesh_version = live_world3d_mesh_version(state, &mesh_key);
        gpu.ensure_mesh(&mesh_key, mesh_version, *mesh);
        translucent.push(SceneDraw3d { mesh_key, mesh_version, instances: vec![Instance3d { component_source: None, id: format!("face-overlay-{index}"), model: Mat4::identity(), color, selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    }
}

pub(super) fn selection_centroid(state: &World3dState) -> Option<Vec3> {
    if let Some(target) = state.gumball_target {
        return Some(Vec3::new(target[0], target[1], target[2]));
    }
    if state.selected_ids.is_empty() {
        return None;
    }
    let mut sum = Vec3::ZERO;
    let mut count = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            if state.selected_ids.iter().any(|id| id == &instance.id) {
                let t = instance.model.cols[3];
                sum = sum.add(Vec3::new(t[0], t[1], t[2]));
                count += 1;
            }
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum.scale(1.0 / count as f32))
    }
}

#[cfg(test)]
pub(super) fn pick_gumball_handle_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<GumballHandle> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let pivot = selection_centroid(state)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let pick_radius = extent * 0.08;
    let eye = gumball_eye(&camera, pivot);
    let mut best: Option<(f32, GumballHandle)> = None;
    let axes = [(GumballHandle::MoveX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (GumballHandle::MoveY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (GumballHandle::MoveZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (handle, axis, _) in axes {
        let end = pivot.add(axis.scale(extent));
        if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
            if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                best = Some((dist, handle));
            }
        }
    }
    let planes = [(GumballHandle::MoveXY, Vec3::new(0.0, 0.0, 1.0), extent * 0.35), (GumballHandle::MoveYZ, Vec3::new(1.0, 0.0, 0.0), extent * 0.35), (GumballHandle::MoveXZ, Vec3::new(0.0, 1.0, 0.0), extent * 0.35)];
    for (handle, normal, half) in planes {
        if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
            let offset = hit.sub(pivot);
            let u = if normal.z.abs() > 0.9 {
                offset.x.abs()
            } else if normal.x.abs() > 0.9 {
                offset.y.abs()
            } else {
                offset.x.abs()
            };
            let v = if normal.z.abs() > 0.9 { offset.y.abs() } else { offset.z.abs() };
            if u <= half && v <= half {
                let dist = origin.sub(hit).length();
                if best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    if matches!(state.transform_mode.as_str(), "rotate" | "rotateSelection") {
        for handle in [GumballHandle::RotateX, GumballHandle::RotateY, GumballHandle::RotateZ] {
            let Some(normal) = handle.plane_normal() else {
                continue;
            };
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let radial = hit.sub(pivot);
                let dist_ring = (radial.length() - extent * 0.85).abs();
                if dist_ring <= pick_radius * 2.0 && best.as_ref().is_none_or(|(best_dist, _)| dist_ring < *best_dist) {
                    best = Some((dist_ring, handle));
                }
            }
        }
    }
    if matches!(state.transform_mode.as_str(), "scale" | "scaleSelection") {
        for handle in [GumballHandle::ScaleX, GumballHandle::ScaleY, GumballHandle::ScaleZ] {
            let Some(axis) = handle.axis_dir() else {
                continue;
            };
            let end = pivot.add(axis.scale(extent * 1.1));
            if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
                if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    let _ = eye;
    best.map(|(_, handle)| handle)
}

/// 🎛️ Same gate the PICK uses (`world3d_gumball_config`), so a handle can never be drawn without
/// being pickable or picked without being drawn — React resolves both from one `gumballConfig`.
///
/// 🚨️ The draw this function pushes is answered on the GPU by `mesh_store.get_versioned`, and a
/// miss there is a FATAL `present_step` fault, not a skipped draw. Every other world draw site
/// pairs its `SceneDraw3d` with exactly this `ensure_mesh`; the gumball did not, so the first
/// frame after a selection died with `prepared frame submit step: prepared world mesh was
/// missing` and took the whole frame loop with it. Unreachable until the wgpu host actually ran
/// the reserved tool job that applies a selection (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) fn append_gumball_geometry(
    lines: &mut Vec<LineVertex3d>,
    translucent: &mut Vec<SceneDraw3d>,
    gpu: &mut World3dBuildContext,
    state: &World3dState,
    camera: &Camera3d,
    meshes: &WorldMeshRegistry,
    mesh_versions: &WorldDynamicRegistry<u64, WORLD_DYNAMIC_MESH_CAPACITY>,
) {
    let Some(pivot) = selection_centroid(state) else {
        return;
    };
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let config = world3d_gumball_config(state);
    let axis_colors = [(GumballHandle::MoveX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (GumballHandle::MoveY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (GumballHandle::MoveZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (handle, axis, color) in axis_colors {
        if !config.admits(handle) {
            continue;
        }
        let end = pivot.add(axis.scale(extent));
        lines.push(LineVertex3d { position: pivot.to_array(), color });
        lines.push(LineVertex3d { position: end.to_array(), color });
    }
    let ring_segments = 48usize;
    for (handle, normal, color) in
        [(GumballHandle::RotateX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 0.85]), (GumballHandle::RotateY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 0.85]), (GumballHandle::RotateZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 0.85])]
    {
        if !config.admits(handle) {
            continue;
        }
        let tangent_a = if normal.x.abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let tangent_b = normal.cross(tangent_a).normalize();
        let tangent_a = tangent_b.cross(normal).normalize();
        let radius = extent * 0.85;
        for step in 0..ring_segments {
            let a0 = step as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let a1 = (step + 1) as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let p0 = pivot.add(tangent_a.scale(a0.cos() * radius)).add(tangent_b.scale(a0.sin() * radius));
            let p1 = pivot.add(tangent_a.scale(a1.cos() * radius)).add(tangent_b.scale(a1.sin() * radius));
            lines.push(LineVertex3d { position: p0.to_array(), color });
            lines.push(LineVertex3d { position: p1.to_array(), color });
        }
    }
    if let Some(plane) = meshes.get(GUMBALL_PLANE_MESH) {
        let mesh_version = *mesh_versions.get(GUMBALL_PLANE_MESH).unwrap_or(&0);
        gpu.ensure_mesh(GUMBALL_PLANE_MESH, mesh_version, *plane);
        let half = extent * 0.35;
        let plane_specs = [(GumballHandle::MoveXY, Vec3::new(0.0, 0.0, 1.0), [half, half, 1.0]), (GumballHandle::MoveYZ, Vec3::new(1.0, 0.0, 0.0), [1.0, half, half]), (GumballHandle::MoveXZ, Vec3::new(0.0, 1.0, 0.0), [half, 1.0, half])];
        for (handle, normal, scale) in plane_specs {
            if !config.admits(handle) {
                continue;
            }
            let tangent = if normal.z.abs() > 0.9 {
                Vec3::new(1.0, 0.0, 0.0)
            } else if normal.x.abs() > 0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            let bitangent = normal.cross(tangent).normalize();
            let tangent = bitangent.cross(normal).normalize();
            let rotation = quat_from_basis(tangent, bitangent, normal);
            translucent.push(SceneDraw3d {
                mesh_key: GUMBALL_PLANE_MESH.into(),
                mesh_version,
                instances: vec![Instance3d { component_source: None, id: GUMBALL_PLANE_MESH.into(), model: Instance3d::model_from_trs(pivot.to_array(), rotation, scale), color: [0.75, 0.8, 0.9, 0.22], selected: false, hovered: false, material: Default::default() }],
                shadow_role: Default::default(),
            });
        }
    }
}

#[cfg(test)]
pub(super) fn apply_gumball_preview(state: &mut World3dState) {
    let Some(handle) = state.gumball_handle else {
        return;
    };
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            if state.selected_ids.iter().any(|id| id == &instance.id) {
                instance.model = gumball_preview_in_place(instance.model, handle, state.gumball_preview_translate, state.gumball_preview_angle, state.gumball_preview_scale);
            }
        }
    }
}

/// 🎚️ One selected instance's gumball preview, exactly as the release commits it: a move translates the instance, a
/// turn rotates its basis about the handle's world axis IN PLACE — its own origin stays, because every 3D gumball
/// verb's leaf (puzzle `rotate-selection`, shooting `rotate-assets`, …) turns each target about its own origin — and
/// a scale stretches the instance's own local axis, as the per-axis factors of the scale leaves do. The gumball pivot
/// only anchors the widget; the selection never orbits it, so the multi-object preview equals the committed state.
fn gumball_preview_in_place(model: Mat4, handle: GumballHandle, translate: Vec3, angle: f32, scale: Vec3) -> Mat4 {
    let mut preview = model;
    if translate.length() > 1e-4 {
        let next = Vec3::new(model.cols[3][0], model.cols[3][1], model.cols[3][2]).add(translate);
        preview.cols[3] = [next.x, next.y, next.z, model.cols[3][3]];
    } else if angle.abs() > 1e-6 {
        if let Some(axis) = handle.axis_dir() {
            for column in 0..3 {
                let turned = rotate_vector(Vec3::new(model.cols[column][0], model.cols[column][1], model.cols[column][2]), axis, angle);
                preview.cols[column] = [turned.x, turned.y, turned.z, model.cols[column][3]];
            }
        }
    } else {
        let (column, factor) = match handle {
            GumballHandle::ScaleX => (0, scale.x),
            GumballHandle::ScaleY => (1, scale.y),
            GumballHandle::ScaleZ => (2, scale.z),
            _ => return preview,
        };
        preview.cols[column] = [model.cols[column][0] * factor, model.cols[column][1] * factor, model.cols[column][2] * factor, model.cols[column][3]];
    }
    preview
}

fn retained_gumball_gesture(state: &World3dState) -> Option<&WorldGumballGesture> {
    let authority = state.interaction_authority.as_ref()?;
    authority.gumball.as_ref().or(match authority.active.as_ref() {
        Some(WorldInteractionActive::GumballCommit { job, .. }) => Some(&job.gesture),
        _ => None,
    })
}

pub(super) fn retained_gumball_preview_model(state: &World3dState, draw_index: usize, instance_index: usize, model: Mat4) -> Mat4 {
    let Some(gesture) = retained_gumball_gesture(state).filter(|gesture| !gesture.live) else {
        return model;
    };
    let selected = gesture.selected[..usize::from(gesture.selected_len)]
        .iter()
        .flatten()
        .any(|target| state.interaction_objects.resolve(target.object).is_some_and(|entry| entry.kind == WorldInteractionObjectKind::Instance && entry.values[0] == draw_index as f32 && entry.values[1] == instance_index as f32));
    if !selected {
        return model;
    }
    gumball_preview_in_place(model, gesture.handle, gesture.translate, gesture.angle, gesture.scale)
}

#[cfg(test)]
pub(super) fn reset_gumball_preview(state: &mut World3dState) {
    state.gumball_preview_translate = Vec3::ZERO;
    state.gumball_preview_angle = 0.0;
    state.gumball_preview_scale = Vec3::new(1.0, 1.0, 1.0);
}

#[cfg(test)]
pub(super) fn gumball_commit_action(state: &World3dState) -> Option<ActionDescriptor> {
    let handle = state.gumball_handle?;
    let ids = state.selected_ids.clone();
    if ids.is_empty() {
        return None;
    }
    if handle.is_translate() && state.gumball_preview_translate.length() > 1e-4 {
        let delta = state.gumball_preview_translate;
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "translateSelection".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": state.surface_id,
                "windowId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "dx": delta.x as f64,
                "dy": delta.y as f64,
                "dz": delta.z as f64,
            })),
        });
    }
    if handle.is_rotate() && state.gumball_preview_angle.abs() > 1e-6 {
        let axis = handle.axis_dir()?;
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "rotateSelection".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": state.surface_id,
                "windowId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "ax": axis.x as f64,
                "ay": axis.y as f64,
                "az": axis.z as f64,
                "angle": state.gumball_preview_angle as f64,
            })),
        });
    }
    if handle.is_scale() {
        let scale = match handle {
            GumballHandle::ScaleX => state.gumball_preview_scale.x,
            GumballHandle::ScaleY => state.gumball_preview_scale.y,
            GumballHandle::ScaleZ => state.gumball_preview_scale.z,
            _ => 1.0,
        };
        if (scale - 1.0).abs() > 1e-6 {
            let (sx, sy, sz) = match handle {
                GumballHandle::ScaleX => (scale, 1.0, 1.0),
                GumballHandle::ScaleY => (1.0, scale, 1.0),
                _ => (1.0, 1.0, scale),
            };
            let args = semio_framework::dsl_value!({
                "surfaceId": state.surface_id,
                "windowId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "sx": sx,
                "sy": sy,
                "sz": sz,
            });
            return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "scaleSelection".into(), args: Some(args) });
        }
    }
    None
}

/// 🕒️ `pub` (was crate-private) so the wgpu renderer's wheel-zoom settle-then-dispatch sweep
/// (`AppRuntime::frame`'s `pending_camera_dispatch_deadlines_ms`) can build the same `setCamera`
/// action the pointer-release path below already dispatches — one orbit-to-action mapping, two
/// trigger sites (immediate on release, debounced on wheel settle).
/// 🔎️ The `zoom` scalar of a camera's `setCamera` pose — React's `captureNavigationSnapshot`:
/// `camera instanceof ThreeOrthographicCamera ? camera.zoom : 1`. A perspective pane reports the
/// identity whatever its camera carries; a parallel one reports the live frustum scale, which is the
/// only number its wheel ever moves ([`OrbitController::zoom`]).
pub(super) fn world3d_camera_zoom(camera: &Camera3d) -> f64 {
    if camera.projection.is_parallel() {
        f64::from(camera.zoom)
    } else {
        1.0
    }
}

/// 📦️ Advances camera/view ownership without invalidating a sealed content publication. Snapshot
/// apply exposes its lease one transaction before the retained draw swap; camera reports, initial
/// templates, and projection framing may all legitimately land in that interval.
pub(super) fn advance_world3d_view_revision(state: &mut World3dState) {
    state.interaction_revision = state.interaction_revision.wrapping_add(1);
    if let Some(rebuild) = state.draw_rebuild.as_mut() {
        rebuild.descriptor.revision = state.interaction_revision;
    }
}

/// 🔀️ Applies one locally selected projection while retaining the accepted viewport-owned pose.
/// React's `WorldProjectionSnapDriver` changes family/FOV and rotates only when orientation changes;
/// its content framer stays unmounted after initial ownership. External/initial seeds arm framing in
/// the scene bridge instead.
pub fn apply_world3d_projection_spec(state: &mut World3dState, spec: Viewport3dProjectionSpec) -> bool {
    if state.projection_selected && state.projection_spec == spec {
        return false;
    }
    let orientation_changed = state.projection_spec.orientation != spec.orientation;
    let projection = ui_wgpu::wgpu::projection_spec_family(spec);
    if state.orbit.projection != projection {
        if projection.is_parallel() {
            state.orbit.zoom = ui_wgpu::wgpu::world_projection_matched_ortho_zoom(state.orbit.fov_y.to_degrees(), state.orbit.distance, state.bounds.h);
        }
    }
    state.orbit.projection = projection;
    state.orbit.fov_y = ui_wgpu::wgpu::projection_spec_fov_degrees(spec).to_radians();
    if orientation_changed {
        let (direction, up) = ui_wgpu::wgpu::projection_spec_orientation_look(spec);
        let direction = direction.normalize();
        state.orbit.yaw = direction.y.atan2(direction.x);
        state.orbit.pitch = direction.z.clamp(-1.0, 1.0).asin();
        state.orbit.up = up;
    }
    state.projection_spec = spec;
    state.projection_selected = true;
    state.projection_frame_policy = Viewport3dProjectionFramePolicy::Content;
    state.projection_frame_owed = false;
    advance_world3d_view_revision(state);
    true
}

/// 🧭️ Changes a projection mode while preserving the live gizmo orientation.
pub fn apply_world3d_projection_mode(state: &mut World3dState, mode: Viewport3dProjectionMode) -> bool {
    apply_world3d_projection_spec(state, Viewport3dProjectionSpec { mode, orientation: state.projection_spec.orientation })
}

/// 🧭️ The live orientation retained across projection-mode changes.
pub fn world3d_projection_orientation(state: &World3dState) -> ui_wgpu::wgpu::WorldProjectionOrientation {
    match state.projection_spec.orientation {
        Viewport3dProjectionOrientation::Cardinal { view } => ui_wgpu::wgpu::WorldProjectionOrientation::Cardinal(match view {
            semio_framework_ui_viewport::Viewport3dOrthographicView::Plan | semio_framework_ui_viewport::Viewport3dOrthographicView::Top => ui_wgpu::wgpu::WorldCardinalView::Top,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Bottom => ui_wgpu::wgpu::WorldCardinalView::Bottom,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Front => ui_wgpu::wgpu::WorldCardinalView::Front,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Back => ui_wgpu::wgpu::WorldCardinalView::Back,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Left => ui_wgpu::wgpu::WorldCardinalView::Left,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Right => ui_wgpu::wgpu::WorldCardinalView::Right,
        }),
        _ => ui_wgpu::wgpu::WorldProjectionOrientation::Free,
    }
}

/// 📐️ The complete accepted projection retained across snapshot delivery and remounts.
pub fn world3d_projection_spec(state: &World3dState) -> Viewport3dProjectionSpec { state.projection_spec }

/// 📐️ Applies a display-template seed once, after the delivered camera has landed. React's pending
/// window template seeds `WorldProjectionRig` before `WorldAutoFit`; keeping the live distance and
/// target here is equivalent for the fit because it consumes only this look direction, then replaces
/// both with the measured rendered bounds.
///
/// 📦️ The snapshot lease becomes resident one transaction before its sealed draw rebuild publishes.
/// Shell applies this camera-only template in that interval, so the live rebuild carries the new
/// view-state revision instead of losing the document and its GLB request ledger as stale.
pub fn apply_world3d_initial_projection_seed(
    state: &mut World3dState,
    spec: Viewport3dProjectionSpec,
) -> bool {
    if state.projection_selected || state.snapshot_lease.is_none() {
        return false;
    }
    let (direction, up) = ui_wgpu::wgpu::projection_spec_orientation_look(spec);
    let length = direction.length();
    if !length.is_finite() || length <= 1e-6 || up.to_array().into_iter().any(|value| !value.is_finite()) {
        return false;
    }
    let direction = direction.scale(1.0 / length);
    state.orbit.yaw = direction.y.atan2(direction.x);
    state.orbit.pitch = direction.z.clamp(-1.0, 1.0).asin();
    state.orbit.up = up;
    let projection = ui_wgpu::wgpu::projection_spec_family(spec);
    state.orbit.projection = projection;
    state.orbit.fov_y = ui_wgpu::wgpu::projection_spec_fov_degrees(spec).to_radians();
    state.projection_spec = spec;
    state.projection_selected = true;
    state.projection_frame_policy = Viewport3dProjectionFramePolicy::Content;
    state.projection_frame_owed = true;
    state.projection_frame_key = None;
    state.projection_frame_zoom = None;
    advance_world3d_view_revision(state);
    true
}

/// 🕒️ Whether this surface has a camera intent waiting to publish — the settle a projection switch
/// or a wheel queues.
pub fn world3d_pending_camera_settle(state: &World3dState) -> bool {
    state.camera_sync.owed || state.interaction_authority.as_ref().is_some_and(|authority| authority.queue.len > 0)
}

/// 🔎️ This surface's live camera family, for the pane chip that reports it.
pub fn world3d_camera_projection(state: &World3dState) -> CameraProjection3d {
    ui_wgpu::wgpu::projection_spec_family(state.projection_spec)
}

/// 🎥️ This surface's LIVE orbit in the very shape `setCamera` carries — the row `dumpMeshStats`
/// reports beside the wire camera, so a probe can diff "what the guest published" against "what the
/// pane is looking through". Carries the two members the wire shape has no room for and a local
/// camera move turns on: the projection FAMILY and the framing ORIENTATION.
pub fn world3d_live_camera_json(state: &World3dState) -> serde_json::Value {
    let camera = state.orbit.to_camera();
    let references = state.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)).map(|reference| {
        let url = reference.url.as_deref().unwrap_or_default();
        let [width, height] = world_reference_plane_size(state, url, world_reference_width(reference));
        json!({
            "id": reference.id.as_deref(),
            "url": url,
            "origin": reference.origin.unwrap_or([0.0; 3]),
            "planeSize": [width, height],
        })
    }).collect::<Vec<_>>();
    let content_bounds = world3d_content_bounds(state).map(|(minimum, maximum)| json!({ "minimum": minimum, "maximum": maximum }));
    let lod = scene_lod(state);
    let grid_step = lod_grid_step_world(lod, state.lod.grid_factor);
    json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
        "up": [f64::from(camera.up.x), f64::from(camera.up.y), f64::from(camera.up.z)],
        "zoom": world3d_camera_zoom(&camera),
        "fov": f64::from(camera.fov_y.to_degrees()),
        "projection": state.projection_spec,
        "projectionFrame": state.projection_frame_policy,
        "viewport": [state.bounds.x, state.bounds.y, state.bounds.w, state.bounds.h],
        "contentBounds": content_bounds,
        "projectionFrameOwed": state.projection_frame_owed,
        "references": references,
        "grid": { "lod": lod, "factor": state.lod.grid_factor, "step": grid_step },
        "userMoved": state.camera_user_moved,
    })
}

pub fn orbit_camera_action(state: &World3dState) -> ActionDescriptor {
    let camera = state.orbit.to_camera();
    ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "setCamera".into(),
        args: Some(semio_framework::dsl_value!({
            "windowId": state.surface_id,
            "camera": {
                "position": [
                    camera.position.x as f64,
                    camera.position.y as f64,
                    camera.position.z as f64,
                ],
                "target": [
                    camera.target.x as f64,
                    camera.target.y as f64,
                    camera.target.z as f64,
                ],
                "zoom": world3d_camera_zoom(&camera),
                "up": [f64::from(camera.up.x), f64::from(camera.up.y), f64::from(camera.up.z)],
            }
        })),
    }
}

/// 📥️ Borrows the validated host destination without moving its owner or reconstructing geometry.
pub fn world3d_admit_native_mesh_step(state:&mut World3dState,donor:&mut Option<WorldNativeMeshResource>,receipt:semio_framework::kernel::ActorUiPatchReceipt,revision:WorldNativeUiRevision,resource_ordinal:usize,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{
    let Some(row)=donor.as_ref()else{return Err(WorldNativeMeshFailure::Binding)};
    let identity=row.owner.identity();let key=row.mesh_key.as_str();
    if !receipt.is_valid()||revision.0==0||row.surface.as_str()!=state.surface_id||key.is_empty()||key.len()>WORLD_INTERACTION_ID_BYTE_CAPACITY{return Err(WorldNativeMeshFailure::Binding)}
    if let Some(WorldMeshSource::Native(source))=state.meshes.0.get(key){return if source.receipt==receipt&&source.revision==revision&&source.resource_ordinal==resource_ordinal&&source.loan.identity()==identity&&source.loan.lease()==row.owner.lease()&&!source.frozen{Ok((true,Default::default()))}else{Err(WorldNativeMeshFailure::Binding)}}
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}
    if state.native_mesh_admission.is_none(){
        if state.dynamic_mesh_close.is_some()||state.dynamic_blocked_mesh.is_some(){return Ok((false,Default::default()))}
        let plan=state.meshes.plan_insert(key).map_err(WorldNativeMeshFailure::Registry)?;if plan.occupied{return Err(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))}
        state.native_mesh_admission=Some(WorldNativeMeshAdmission{identity,lease:row.owner.lease().ok_or(WorldNativeMeshFailure::Binding)?,receipt,revision,resource_ordinal,key:String::new(),extent:key.len(),interaction_id:WorldInteractionId{bytes:[0;WORLD_INTERACTION_ID_BYTE_CAPACITY],len:0},interaction_offset:0,allocated:false});
        return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    let cursor=state.native_mesh_admission.as_mut().unwrap();
    if cursor.identity!=identity||Some(cursor.lease)!=row.owner.lease()||cursor.receipt!=receipt||cursor.revision!=revision||cursor.resource_ordinal!=resource_ordinal||cursor.extent!=key.len()||!key.starts_with(&cursor.key){return Err(WorldNativeMeshFailure::Binding)}
    if !cursor.allocated{if grant.maximum_capacity_bytes<cursor.extent{return Ok((false,Default::default()))}cursor.key.try_reserve_exact(cursor.extent).map_err(|_|WorldNativeMeshFailure::Registry(WorldDynamicFault::ByteCapacity))?;cursor.allocated=true;return Ok((false,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:cursor.key.capacity(),..Default::default()}))}
    if cursor.key.len()<cursor.extent{let start=cursor.key.len();let mut end=start.saturating_add(grant.maximum_copy_bytes).min(cursor.extent);while !key.is_char_boundary(end){end-=1;}if end==start{return Ok((false,Default::default()))}cursor.key.push_str(&key[start..end]);return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:end-start,..Default::default()}))}
    if cursor.interaction_offset<cursor.extent{let start=cursor.interaction_offset;let end=start.saturating_add(grant.maximum_copy_bytes).min(cursor.extent);if end==start{return Ok((false,Default::default()))}cursor.interaction_id.bytes[start..end].copy_from_slice(&key.as_bytes()[start..end]);cursor.interaction_offset=end;if end==cursor.extent{cursor.interaction_id.len=end as u16;}return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:end-start,..Default::default()}))}
    let plan=state.meshes.plan_insert(key).map_err(WorldNativeMeshFailure::Registry)?;if plan.occupied{return Err(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))}
    let lease=row.owner.lease().ok_or(WorldNativeMeshFailure::Binding)?;let mut interaction=state.interaction_meshes.plan_admit_id(cursor.interaction_id,identity.cache_epoch,lease).map_err(WorldNativeMeshFailure::Registry)?;interaction.value.generation=interaction.epoch.checked_add(1).ok_or(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))?;
    let Some((loan,progress))=row.owner.loan(grant).map_err(WorldNativeMeshFailure::Mesh)?else{return Ok((false,Default::default()))};
    let cursor=state.native_mesh_admission.take().unwrap();let native=WorldNativeMeshSource{loan,receipt,revision,resource_ordinal,lookup_extent:cursor.extent,frozen:false,gpu_returned:false};
    state.meshes.0.commit_insert(plan,cursor.key,WorldMeshSource::Native(native)).expect("exclusive World transaction preserves its prevalidated empty mesh slot");
    state.interaction_meshes.commit_admit(interaction).expect("exclusive World transaction preserves its prevalidated interaction slot");state.geometry_generation=state.geometry_generation.wrapping_add(1).max(1);
    Ok((true,progress))
}

/// ♻️ Closes a replaced host destination loan only after its exact GPU cache returns.
#[cfg(not(all(target_arch="wasm32",target_env="p2")))]
pub fn world3d_close_native_mesh_step(state:&mut World3dState,key:&str,receipt:semio_framework::kernel::ActorUiPatchReceipt,revision:WorldNativeUiRevision,resource_ordinal:usize,grant:RetainedCloneGrant,retire_gpu:impl FnOnce(&str,&WorldNativeMeshLoan)->Result<Option<ui_wgpu::wgpu::MeshGpuSourceReturn>,&'static str>)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}
    if state.dynamic_mesh_close.is_none(){
        let Some(WorldMeshSource::Native(source))=state.meshes.0.get(key)else{return Ok((true,Default::default()))};
        if source.receipt!=receipt||source.revision!=revision||source.resource_ordinal!=resource_ordinal{return Err(WorldNativeMeshFailure::Binding)}
        let mut entry=state.meshes.remove(key).unwrap();let WorldMeshSource::Native(source)=&mut entry.value else{unreachable!()};source.frozen=true;state.dynamic_mesh_close=Some(entry);return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    let entry=state.dynamic_mesh_close.as_mut().unwrap();let WorldMeshSource::Native(source)=&mut entry.value else{return Err(WorldNativeMeshFailure::Binding)};
    if source.receipt!=receipt||source.revision!=revision||source.resource_ordinal!=resource_ordinal||source.lookup_extent!=key.len()||!key.starts_with(&entry.id){return Err(WorldNativeMeshFailure::Binding)}
    if !source.gpu_returned{let Some(witness)=retire_gpu(key,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?else{return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))};if !witness.validates(key,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?{return Err(WorldNativeMeshFailure::Binding)}source.gpu_returned=true;return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))}
    if !source.loan.terminal_is_empty(){let step=source.loan.return_step(grant).map_err(WorldNativeMeshFailure::Mesh)?;return Ok((false,step.progress))}
    let close=world_close_string(&mut entry.id,grant).map_err(WorldNativeMeshFailure::Close)?;let complete=matches!(close,semio_framework_job::InteractiveJobCloseStep::Complete{..});let progress=if complete{RetainedCloneProgress{copied_items:1,..close.progress()}}else{close.progress()};if complete{state.dynamic_mesh_close.take();}Ok((complete,progress))
}

/// 🧹️ Cancels an unfinished lookup without touching its retained destination owner.
pub fn world3d_close_native_mesh_admission_step(state:&mut World3dState,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}let Some(cursor)=state.native_mesh_admission.as_mut()else{return Ok((true,Default::default()))};let step=world_close_string(&mut cursor.key,grant).map_err(WorldNativeMeshFailure::Close)?;let complete=matches!(step,semio_framework_job::InteractiveJobCloseStep::Complete{..});let progress=if complete{RetainedCloneProgress{copied_items:1,..step.progress()}}else{step.progress()};if complete{state.native_mesh_admission.take();}Ok((complete,progress))}

/// 🧊️ The closing World source retains its loan until the original renderer returns its cache.
#[cfg(not(all(target_arch="wasm32",target_env="p2")))]
pub fn world3d_return_native_mesh_gpu_step(state:&mut World3dState,grant:RetainedCloneGrant,retire_gpu:impl FnOnce(&str,&WorldNativeMeshLoan)->Result<Option<ui_wgpu::wgpu::MeshGpuSourceReturn>,&'static str>)->Result<RetainedCloneProgress,WorldNativeMeshFailure>{
    let Some(entry)=state.dynamic_mesh_close.as_mut()else{return Ok(Default::default())};let WorldMeshSource::Native(source)=&mut entry.value else{return Ok(Default::default())};
    if source.gpu_returned||grant.maximum_items==0||grant.maximum_depth==0{return Ok(Default::default())}source.frozen=true;
    let Some(witness)=retire_gpu(&entry.id,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?else{return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})};if !witness.validates(&entry.id,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?{return Err(WorldNativeMeshFailure::Binding)}source.gpu_returned=true;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
}

pub fn publish_world3d_mesh_lease(state: &mut World3dState, id: String, mesh: Mesh3dLease) -> Result<(), WorldDynamicRejected<Mesh3dLease>> {
    let version = mesh.generation().rotate_left(17) ^ mesh.revision();
    if state.dynamic_mesh_close.is_some() || state.dynamic_blocked_mesh.is_some() {
        return Err(WorldDynamicRejected { fault: WorldDynamicFault::Closing, id, value: mesh });
    }
    let mesh_plan = state.meshes.plan_insert(&id).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let version_plan = state.mesh_versions.plan_insert(&id).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let interaction_plan = state.interaction_meshes.plan_admit(&id, version, mesh).map_err(|fault| WorldDynamicRejected { fault, id: id.clone(), value: mesh })?;
    let (_, previous_mesh) = state.meshes.commit_insert(mesh_plan, id.clone(), mesh).map_err(|rejected| WorldDynamicRejected { fault: rejected.fault, id: rejected.id, value: mesh })?;
    state.interaction_meshes.commit_admit(interaction_plan).expect("world mesh transaction revalidates every fixed interaction slot before its first mutation");
    let (_, previous_version) = state.mesh_versions.commit_insert(version_plan, id.clone(), version).expect("world mesh transaction revalidates every fixed version slot before its first mutation");
    if let Some(mut previous_version) = previous_version {
        previous_version.id.clear();
    }
    if let Some(previous_mesh) = previous_mesh {
        state.dynamic_mesh_close = Some(previous_mesh);
    }
    drop(state.mesh_appearances.remove(&id));
    state.geometry_generation = state.geometry_generation.wrapping_add(1).max(1);
    Ok(())
}

pub(super) fn store_mesh(state: &mut World3dState, id: String, mesh: Mesh3dLease) {
    if let Err(rejected) = publish_world3d_mesh_lease(state, id, mesh) {
        retain_rejected_world_mesh(state, rejected);
    }
}

pub(super) fn retain_rejected_world_mesh(state: &mut World3dState, rejected: WorldDynamicRejected<Mesh3dLease>) {
    let fault = rejected.fault;
    if state.dynamic_blocked_mesh.is_none() {
        state.dynamic_blocked_mesh = Some(WorldDynamicEntry { id: rejected.id, epoch: 0, value: WorldMeshSource::Owned{lease:Some(rejected.value),closing:false} });
    }
    mark_world_dynamic_fault(state, fault);
}

pub(super) fn retain_world_blocked_owner(state: &mut World3dState, owner: WorldOpaqueOwner) {
    assert!(state.dynamic_blocked_owner.is_none(), "world dynamic producers must stop after an observable ownership fault");
    state.dynamic_blocked_owner = Some(owner);
}

#[cfg(test)]
fn set_world_mesh_version(state: &mut World3dState, id: String, version: u64) {
    match state.mesh_versions.insert(id, version) {
        Ok((_, previous)) => drop(previous),
        Err(rejected) => {
            drop(rejected.id);
            mark_world_dynamic_fault(state, rejected.fault);
        }
    }
}

pub(super) fn mark_world_dynamic_fault(state: &mut World3dState, fault: WorldDynamicFault) {
    state.dynamic_fault=Some(fault);
    state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
    if let Some(authority) = state.interaction_authority.as_mut() {
        authority.faulted = true;
    }
}

pub(super) fn retire_world_mesh(state: &mut World3dState, id: &str) -> bool {
    if state.dynamic_mesh_close.is_some() || state.dynamic_blocked_mesh.is_some() {
        return false;
    }
    let Some(entry) = state.meshes.remove(id) else {
        return true;
    };
    state.dynamic_mesh_close = Some(entry);
    drop(state.mesh_versions.remove(id));
    drop(state.mesh_appearances.remove(id));
    state.geometry_generation = state.geometry_generation.wrapping_add(1).max(1);
    true
}

pub(super) fn publish_world_raster(state: &mut World3dState, id: String, lease: SceneRasterLease, paint: bool) -> bool {
    let value = WorldSceneRaster { identity: lease.identity(), pending: Mutex::new(Some(lease)) };
    let registry = if paint { &mut state.mesh_paint_textures } else { &mut state.reference_pixels };
    match registry.insert(id, value) {
        Ok((_, previous)) => {
            drop(previous);
            true
        }
        Err(rejected) => {
            let fault = rejected.fault;
            drop(rejected);
            mark_world_dynamic_fault(state, fault);
            false
        }
    }
}

#[derive(Debug)]
/// 🎨️ Generation-owned pooled paint raster published after asynchronous image decoding.
pub struct DecodedMeshPaintImage {
    pub raster: SceneRasterLease,
}

/// 🎨️ Publishes exact decoded paint pixels for a resident mesh with one UV per vertex.
pub fn apply_decoded_mesh_paint_image(state: &mut World3dState, mesh_key: &str, decoded: DecodedMeshPaintImage) -> bool {
    let descriptor = decoded.raster.identity().descriptor();
    let mesh_version = live_world3d_mesh_version(state,mesh_key);
    let mesh_seal_is_current = state.meshes.get(mesh_key).and_then(|mesh| mesh.schema().ok()).is_some_and(|schema| {
        schema.uvs == schema.vertices
            && schema.uvs > 0
            && descriptor.mesh.is_some_and(|seal| seal.mesh_revision == mesh_version && seal.uv_revision == mesh_version && seal.uv_count == schema.uvs)
    });
    descriptor.profile == SceneRasterProfile::MeshPaintMapNoColorSpace && mesh_seal_is_current && publish_world_raster(state, mesh_key.to_string(), decoded.raster, true)
}

/// 🖼️ Offers ONE reference underlay's decoded payload to the frame, KEEPING the state's own copy.
///
/// ⚖️ The copy stays for two reasons: a sibling surface of the same document holds its own
/// `World3dState` and must be able to offer the same image, and an offer can be LOST — a producer
/// rides the frame it was admitted into, and a frame whose present is ABORTED drops its staged
/// texture (`RasterTextureTable::abort_presented_step`). The re-offer on the next render is what
/// heals that, which is why this lane offers per render rather than once.
///
/// 🩸️ It briefly `mem::take`-d the payload, so the image crossed exactly once ever — and both panes
/// then painted no plan at all, because the one frame that carried it did not commit (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, W9b's `🗑️generated/w9b-final/final.png` against
/// `🗑️generated/w8b-boot-2/final.png`). What made the per-render offer dangerous was never the offer:
/// it was reporting a refused admission as a frame fault, which
/// [`World3dBuildContext::ensure_world_plane_texture`] no longer does.
///
/// ⏭️ The honest end state is an upload ACK from the raster table — the worker cannot see `live`, so
/// it cannot know the texture is resident and must keep offering. Until that exists, the table's own
/// key-dedupe absorbs the repeat.
pub(super) fn world_raster_upload_in_pool(state: &World3dState, id: &str, gpu_key: &str, paint: bool, pool: &SceneRasterPool) -> Option<SceneRasterLease> {
    let registry = if paint { &state.mesh_paint_textures } else { &state.reference_pixels };
    let raster = registry.get(id)?;
    if pool.gpu_resident(gpu_key, raster.identity) {
        raster.pending.lock().ok()?.take();
        return None;
    }
    raster.pending.lock().ok()?.take().or_else(|| pool.acquire(raster.identity).ok())
}

pub(super) fn world_raster_upload(state: &World3dState, id: &str, gpu_key: &str, paint: bool) -> Option<SceneRasterLease> {
    world_raster_upload_in_pool(state, id, gpu_key, paint, &world_scene_raster_pool())
}

pub(super) fn world_reference_raster_ready_in_pool(state: &World3dState, url: &str, pool: &SceneRasterPool) -> bool {
    let Some(raster) = state.reference_pixels.get(url) else { return false };
    if raster.pending.lock().is_ok_and(|pending| pending.is_some()) {
        return true;
    }
    pool.cpu_resident(raster.identity) || pool.gpu_resident(url, raster.identity)
}

pub(super) fn world_reference_raster_ready(state: &World3dState, url: &str) -> bool {
    world_reference_raster_ready_in_pool(state, url, &world_scene_raster_pool())
}

pub(super) fn reference_underlay_upload(state: &World3dState, url: &str) -> Option<SceneRasterLease> {
    world_raster_upload(state, url, url, false)
}

pub(super) fn retire_world_pixels(state: &mut World3dState, id: &str, paint: bool) -> bool {
    let registry = if paint { &mut state.mesh_paint_textures } else { &mut state.reference_pixels };
    drop(registry.remove(id));
    true
}

const WORLD3D_PREPARED_STATUS_BYTES: usize = 192;

#[derive(Clone, Copy)]
pub(super) struct World3dPreparedStatus {
    pub(super) bytes: [u8; WORLD3D_PREPARED_STATUS_BYTES],
    pub(super) len: u8,
    pub(super) progress: [f64; 3],
    pub(super) total: u32,
    pub(super) state: u32,
}

impl World3dPreparedStatus {
    fn from_parts(text: &str, progress: [f64; 3], total: u32, state: u32) -> Option<Self> {
        if text.len() > WORLD3D_PREPARED_STATUS_BYTES {
            return None;
        }
        let mut bytes = [0; WORLD3D_PREPARED_STATUS_BYTES];
        bytes[..text.len()].copy_from_slice(text.as_bytes());
        Some(Self { bytes, len: text.len() as u8, progress, total, state })
    }

    pub(super) fn text(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok()
    }
}

pub(super) struct World3dSnapshotApplyCursor {
    pub(super) lease: World3dSnapshotLease,
    pub(super) page: u16,
    pub(super) item: u8,
    staged_orbit: Option<OrbitController>,
    draw_started: bool,
    /// 🧱️ Which draw the `Instance` items being applied right now belong to. A single-draw producer
    /// (`Mesh` item flags `30`) leaves it at 0; a multi-draw one (flags `31`) re-points it at every
    /// `Mesh` page, which is why those pages interleave with their own instance pages.
    draw_index: u16,
    draw_permit: Option<World3dSnapshotDrawPermit>,
    pub(super) faulted: bool,
    status: [Option<World3dPreparedStatus>; 2],
}

impl World3dSnapshotApplyCursor {
    pub(super) fn new(lease: World3dSnapshotLease) -> Self {
        Self { lease, page: 0, item: 0, staged_orbit: None, draw_started: false, draw_index: 0, draw_permit: None, faulted: false, status: [None; 2] }
    }

    pub(super) fn close_step(&mut self) -> bool {
        if self.staged_orbit.take().is_some() {
            return false;
        }
        self.page = self.lease.page_count;
        self.item = 0;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSnapshotApplyStep {
    Idle,
    Pending,
    Complete,
    Stale,
    Fault,
}

/// 🔢️ The revision this apply INSTALLS, decided BEFORE the draw rebuild it began is sealed.
///
/// 🩸️ `lease.revision` is this world's proposed source-publication revision: the bridge reserves
/// the next interaction revision for each changed payload — `publish_world3d_scene_bridge_snapshot`
/// is its one production writer —
/// so adopting it verbatim moves the counter BACKWARDS whenever anything bumped the revision
/// while the lease was in flight: a document-lane change (`sync_world3d_scene_document_lanes`),
/// a parallel framing (`sync_world3d_projection_content_frame`), a fit, a camera report. The
/// rollback then invalidated the very draws this apply had just built, because
/// `step_world3d_draw_rebuild` refuses a cursor whose `descriptor.revision` no longer equals
/// `state.interaction_revision`. Measured on 6118: generation3d's `procedural-preview` sat at
/// `state-meshes=3 state-instances=0 state-draws=0` with three `world3d draw rebuild step=Stale`
/// lines immediately after `world3d delivery applied`, and the tessellated column never painted
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14b-generation3d-labels-preview-layout.md`).
///
/// 🧱️ The draws this apply built ARE the content of the revision it installs, so they are
/// re-stamped with it rather than left carrying whatever revision happened to be in force
/// when the first `Mesh` page opened the rebuild.
///
/// 📐️ A LOCALLY selected projection is view state the wire never overrides — React holds
/// it in `externalPendingProjectionSpec` and only falls back to the delivered
/// `cameraState.projectionSpec` when the pane has none (`🌐️World3dHost/🟦️.tsx`).
///
/// 🩸️ Without this the switch undid itself within one round trip: the template press
/// queues a settle, the settle publishes `setCamera`, the guest echoes a camera back, and
/// `setCamera`'s payload has no `projection` member at all (`orbit_camera_action`), so the
/// echo always arrives in the DELIVERED family. Measured live: the press moved the pane's
/// target onto the content centre and `liveCamera` still read `perspective` 8 s later
/// (`📓️w9b-projection-pane-framing-grid-materials.md` §1).
pub fn step_world3d_snapshot(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> World3dSnapshotApplyStep {
    if context.should_yield() {
        return World3dSnapshotApplyStep::Pending;
    }
    let Some(mut cursor) = state.snapshot_apply.take() else {
        return World3dSnapshotApplyStep::Idle;
    };
    if cursor.faulted {
        state.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    }
    if cursor.page == cursor.lease.page_count {
        let revision = state.interaction_revision.max(cursor.lease.revision);
        if cursor.draw_started {
            if let Some(rebuild) = state.draw_rebuild.as_mut() {
                rebuild.descriptor.revision = revision;
            }
            if world3d_draw_rebuild_seal(state).is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        if let Some(orbit) = cursor.staged_orbit.take() {
            state.orbit = if state.projection_selected {
                OrbitController { projection: ui_wgpu::wgpu::projection_spec_family(state.projection_spec), fov_y: ui_wgpu::wgpu::projection_spec_fov_degrees(state.projection_spec).to_radians(), ..orbit }
            } else {
                orbit
            };
        }
        state.interaction_revision = revision;
        if let Some(gesture) = state.interaction_authority.as_mut().and_then(|authority| authority.gumball.as_mut()).filter(|gesture| gesture.live && gesture.streamed) {
            gesture.revision = revision;
        }
        state.snapshot_lease = Some(cursor.lease);
        state.prepared_status = cursor.status;
        state.snapshot_fault = None;
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Complete;
    }
    let resolved = world3d_snapshot_with_page(cursor.lease, cursor.page, |page| {
        let item_count = page.item_count();
        let item = page.item(usize::from(cursor.item)).copied();
        let id = item.and_then(|item| item.strings[0]).and_then(|span| page.string(span)).map(str::to_owned);
        let status = item.and_then(|item| {
            let text = item.strings[1].and_then(|span| page.string(span))?;
            World3dPreparedStatus::from_parts(text, [item.numbers[0], item.numbers[1], item.numbers[2]], item.indexes[0], item.indexes[1])
        });
        let component_source = if page.kind() == World3dSnapshotPageKind::Instance {
            item.map(|item| match (item.strings[1], item.strings[2]) {
                (None, None) => Ok(None),
                (Some(handle), Some(revision)) => ComponentSource3d::new(page.string(handle).ok_or(World3dSnapshotFault::PageState)?, page.string(revision).ok_or(World3dSnapshotFault::PageState)?).map(Some).ok_or(World3dSnapshotFault::PageState),
                _ => Err(World3dSnapshotFault::PageState),
            }).transpose()
        } else { Ok(None) };
        (page.kind(), item_count, item, id, status, component_source)
    });
    let (kind, item_count, item, id, status, component_source) = match resolved {
        Ok(resolved) => resolved,
        Err(fault) => {
            state.snapshot_fault = Some(fault);
            cursor.faulted = true;
            state.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Stale;
        }
    };
    if usize::from(cursor.item) == item_count {
        cursor.page += 1;
        cursor.item = 0;
        state.snapshot_apply = Some(cursor);
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Pending;
    }
    let component_source = match component_source {
        Ok(source) => source.flatten(),
        Err(fault) => { state.snapshot_fault = Some(fault); cursor.faulted = true; state.snapshot_apply = Some(cursor); return World3dSnapshotApplyStep::Fault; }
    };
    let Some(item) = item else {
        state.snapshot_fault = Some(World3dSnapshotFault::PageState);
        cursor.faulted = true;
        state.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    };
    match kind {
        World3dSnapshotPageKind::Mesh if item.flags == 30 && item.index_len >= 2 => {
            let Some(mesh_key) = id.as_deref() else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let instance_count = item.indexes[0];
            let byte_count = item.indexes[1].checked_add(size_of::<SceneDraw3d>() as u32).and_then(|bytes| bytes.checked_add(instance_count.checked_mul(size_of::<Instance3d>() as u32)?));
            let Some(byte_count) = byte_count else {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let draw_bytes = item.indexes[1];
            let permit = match world3d_snapshot_claim_draw_permit(cursor.lease, 1, instance_count, draw_bytes) {
                Ok(permit) => permit,
                Err(fault) => {
                    state.snapshot_fault = Some(fault);
                    cursor.faulted = true;
                    state.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                }
            };
            if !scene_mesh_awaits_its_asset(state, mesh_key) {
                begin_world_placeholder_mesh(state, mesh_key, WorldPlaceholderKind::Box);
            }
            let descriptor = WorldDrawRebuildDescriptor { generation: state.draw_generation.wrapping_add(1), revision: state.interaction_revision, draw_count: 1, instance_count, byte_count };
            let mesh_version = live_world3d_mesh_version(state, mesh_key);
            let admitted = begin_world3d_draw_rebuild(state, descriptor).and_then(|()| world3d_draw_rebuild_admit_draw(state, mesh_key, mesh_version, u16::try_from(instance_count).map_err(|_| WorldDynamicFault::InstanceCapacity)?));
            if admitted.is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
            cursor.draw_permit = Some(permit);
            cursor.draw_started = true;
            cursor.draw_index = 0;
        }
        World3dSnapshotPageKind::Mesh if item.flags == 31 && item.index_len >= 6 => {
            let Some(mesh_key) = id.as_deref() else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let (own_instances, ordinal, total_draws, total_instances, total_bytes) = (item.indexes[0], item.indexes[2], item.indexes[3], item.indexes[4], item.indexes[5]);
            let Ok(total_draws) = u16::try_from(total_draws) else {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            if ordinal == 0 {
                let byte_count = total_bytes.checked_add(u32::from(total_draws).saturating_mul(size_of::<SceneDraw3d>() as u32)).and_then(|bytes| bytes.checked_add(total_instances.checked_mul(size_of::<Instance3d>() as u32)?));
                let permit = byte_count.ok_or(World3dSnapshotFault::Capacity).and_then(|_| world3d_snapshot_claim_draw_permit(cursor.lease, total_draws, total_instances, total_bytes));
                let (Some(byte_count), Ok(permit)) = (byte_count, permit) else {
                    state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.faulted = true;
                    state.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                };
                let descriptor = WorldDrawRebuildDescriptor { generation: state.draw_generation.wrapping_add(1), revision: state.interaction_revision, draw_count: total_draws, instance_count: total_instances, byte_count };
                if begin_world3d_draw_rebuild(state, descriptor).is_err() {
                    state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.faulted = true;
                    state.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                }
                cursor.draw_permit = Some(permit);
                cursor.draw_started = true;
            }
            let mesh_version = live_world3d_mesh_version(state, mesh_key);
            let admitted = u16::try_from(own_instances).map_err(|_| WorldDynamicFault::InstanceCapacity).and_then(|count| world3d_draw_rebuild_admit_draw(state, mesh_key, mesh_version, count));
            let Ok(ordinal) = u16::try_from(ordinal) else {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            if admitted.is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
            cursor.draw_index = ordinal;
        }
        World3dSnapshotPageKind::Instance if item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let position = [item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32];
            let rotation = [item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32, item.numbers[6] as f32];
            let scale = [item.numbers[7] as f32, item.numbers[8] as f32, item.numbers[9] as f32];
            let color = [item.numbers[10] as f32, item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let model = Instance3d::model_from_trs(position, rotation, scale);
            if world3d_draw_rebuild_admit_retained_instance(state, cursor.draw_index, id, model, color, SceneColorSource3d::from_snapshot_flag(item.flags), false, false, component_source).is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Camera if item.number_len >= 12 => {
            let projection = if item.numbers[11] >= 0.5 { CameraProjection3d::Orthographic } else { CameraProjection3d::Perspective };
            cursor.staged_orbit = Some(OrbitController::from_camera(&Camera3d {
                position: Vec3::new(item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32),
                target: Vec3::new(item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32),
                up: Vec3::new(item.numbers[6] as f32, item.numbers[7] as f32, item.numbers[8] as f32),
                fov_y: item.numbers[9] as f32 * std::f32::consts::PI / 180.0,
                near: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_NEAR,
                far: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_MIN_FAR,
                projection,
                zoom: item.numbers[10] as f32,
            }));
        }
        World3dSnapshotPageKind::Camera => {
            state.snapshot_fault = Some(World3dSnapshotFault::PageState);
            cursor.faulted = true;
            state.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Fault;
        }
        World3dSnapshotPageKind::Status if (10..=14).contains(&item.flags) && item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let vector = [item.numbers[0], item.numbers[1], item.numbers[2]];
            let magnitude = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt().max(item.numbers[3].abs()).max(1.0e-6) as f32;
            let position = [item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let color = match item.flags {
                10 => [0.957, 0.447, 0.714, 1.0],
                11 => [0.918, 0.702, 0.031, 1.0],
                12 => [0.984, 0.443, 0.522, 1.0],
                13 => [0.133, 0.827, 0.933, 1.0],
                _ => [0.655, 0.545, 0.98, 1.0],
            };
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [0.025, 0.025, magnitude.min(1.0)]);
            if world3d_draw_rebuild_admit_instance(state, 0, id, model, color, false, false).is_err() {
                state.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Status if item.flags == 20 && item.number_len >= 3 && item.index_len >= 2 => {
            let Some(status) = status else {
                state.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            cursor.status[usize::from(id.as_deref() == Some("de"))] = Some(status);
        }
        _ => {}
    }
    cursor.item += 1;
    state.snapshot_apply = Some(cursor);
    context.consume_fuel(1);
    World3dSnapshotApplyStep::Pending
}

pub fn close_world3d_snapshot_apply_step(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    let Some(cursor) = state.snapshot_apply.as_mut() else {
        return true;
    };
    let complete = cursor.close_step();
    context.consume_fuel(1);
    if complete {
        state.snapshot_apply = None;
    }
    complete
}
