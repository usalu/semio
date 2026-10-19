//! 🖌️ World3d overlays: vertex markers, reference outlines, component overlays, gumball geometry, picking and commit.

use super::*;

//#region 🌉️World3dSceneBridge
fn push_line_segment(lines: &mut Vec<LineVertex3d>, from: Vec3, to: Vec3, color: [f32; 4]) {
    lines.push(LineVertex3d { position: from.to_array(), color });
    lines.push(LineVertex3d { position: to.to_array(), color });
}

pub const VERTEX_MARKER_MESH: &str = "vertex-marker";

pub const VERTEX_BASE_SCALE: f32 = 0.05;

pub const VERTEX_HOVER_SCALE: f32 = 0.09;

const VERTEX_SELECT_SCALE: f32 = 0.09;

/// 🔘️ React's `WORLD_VERTEX_DOT_PX` / `WORLD_VERTEX_MARK_PX` (`🌐️World3dHost/🟦️.tsx:2499`) — the
/// plain and the hovered/selected vertex marker, in SCREEN PIXELS, drawn with
/// `sizeAttenuation={false}`.
///
/// 🩸️ These markers were world-unit crosses of `VERTEX_BASE_SCALE * 0.15` = 0.0075 world units,
/// which is sub-pixel on any scene larger than a few metres — the SAME defect React shipped and
/// fixed on lowpoly (`pointsMaterial size` in world units → `sizeAttenuation={false}` pixels,
/// ticket 26/08/29 round 4). The scales above stay as the RELATIVE weight of the three states.
pub const WORLD_VERTEX_DOT_PX: f32 = 6.0;

const WORLD_VERTEX_MARK_PX: f32 = 11.0;

/// 📏️ World units one screen pixel spans at `distance` from the eye — the inverse of
/// `sizeAttenuation`. Perspective opens with distance; a parallel frustum is distance-invariant and
/// reads straight off `orthographic_half_extent`.
pub fn world_units_per_pixel(camera: &Camera3d, viewport: Rect, distance: f32) -> f32 {
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
pub fn vertex_marker_half_extent(camera: &Camera3d, viewport: Rect, centre: Vec3, scale: f32, mark_px: Option<f32>) -> f32 {
    let pixels = if scale > VERTEX_BASE_SCALE { mark_px.unwrap_or(WORLD_VERTEX_MARK_PX) } else { WORLD_VERTEX_DOT_PX };
    pixels * 0.5 * world_units_per_pixel(camera, viewport, camera.position.sub(centre).length())
}

/// 🖍️ Resolves the scene's sub-element highlight tokens through the live theme with the shared `world3d_sub_element_paint` table; a granularity without a token keeps the built-in palette.
pub fn world3d_sub_element_paint_native(theme: &semio_framework_ui_theme::Theme, highlight: Option<&semio_framework_ui_declaration::ui::World3dHighlight>) -> WorldSubElementPaint {
    let defaults = semio_framework_ui_declaration::ui::World3dSubElementDefaults { select: None, hover: None, edge_hover: None, edge_width: 0.0, vertex_mark_px: 0.0 };
    let paint = semio_framework_ui_declaration::ui::world3d_sub_element_paint(highlight, |tone| Some(world3d_tone_rgba(theme, tone)), defaults);
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
pub fn world3d_tone_rgba(theme: &semio_framework_ui_theme::Theme, tone: semio_framework_ui_declaration::ui::World3dTone) -> [f32; 4] {
    use semio_framework_ui_declaration::ui::World3dTone;
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

pub fn component_overlay_color(id: &str, selected: &HashSet<String>, preview: &HashSet<String>, hovered: &Option<String>, tokens: (Option<[f32; 4]>, Option<[f32; 4]>)) -> Option<([f32; 4], f32)> {
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

pub fn mesh_face_id(mesh: Mesh3dLease, tri_index: u32) -> String {
    world_mesh_component_id(mesh, Mesh3dField::FaceIds, tri_index).to_string()
}

pub fn face_component_mode_active(selection_state: &WorldSelectionState) -> bool {
    selection_state.selection_targets.face || selection_state.granularity == "face"
}

fn instance_hovered_component_id(selection_state: &WorldSelectionState, instance_id: &str) -> Option<String> {
    if !pick_targets_instance(selection_state, instance_id) {
        return None;
    }
    if selection_state.hovered_component_mode.as_deref() != Some(selection_state.granularity.as_str()) {
        return None;
    }
    if selection_state.hovered_component_object_id.as_deref().is_some_and(|object_id| object_id != instance_id) {
        return None;
    }
    selection_state.hovered_component_id.clone()
}

pub fn append_component_vertex_spheres() -> Vec<Instance3d> {
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
    pub background: [f32; 4],
    pub content_opacity: f32,
    pub outline: Option<[f32; 4]>,
}

fn world_reference_state_id_matches(value: &str, id: &str) -> bool {
    value == id || value.strip_prefix("reference:") == Some(id)
}

pub fn world_reference_visual(selection_state: &WorldSelectionState, reference: &WorldReferenceRecord, theme: &semio_framework_ui_theme::Theme) -> WorldReferenceVisual {
    let id = reference.id.as_deref().or(reference.url.as_deref()).unwrap_or_default();
    let locked = reference.locked.unwrap_or(false);
    let selected = !locked && selection_state.selected_ids.iter().any(|value| world_reference_state_id_matches(value, id));
    let hovered = !locked && selection_state.local_hover_id.as_deref().is_some_and(|value| world_reference_state_id_matches(value, id));
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

pub fn append_world_reference_outline(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], width: f32, height: f32, color: [f32; 4]) {
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
/// (`✏️editor/⚙️engine/🧲️picking/🎨️target-style/🦀️.rs`), resolved through the same CSS chain React does
/// (`🎨️styling/🖌️ui/🎨️.css`): `--color-changed-selected` → `--accent`, `--active-base` and
/// `--color-changed-hovered`/`--accent-secondary` → the primary/secondary celebrate stops, the
/// `--hover-*` grays → the one hover fill this theme carries. An unknown token falls back rather
/// than painting black.
fn world_pick_style_color(theme: &semio_framework_ui_theme::Theme, reference: Option<&str>, fallback: [f32; 4]) -> [f32; 4] {
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
pub fn append_pick_target_hover_lines(selection_state: &WorldSelectionState, theme: &semio_framework_ui_theme::Theme, camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) {
    let Some(key) = selection_state.pick_hover_key.as_deref() else { return };
    let Some(target) = selection_state.pick_targets.iter().find(|target| world_pick_keys_match(Some(target.key().as_str()), Some(key))) else { return };
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
pub fn append_engagement_preview_lines(scene_state: &WorldSceneState, lines: &mut Vec<LineVertex3d>, color: [f32; 4]) {
    const POINT_CROSS_EXTENT: f32 = 0.08;
    const PREVIEW_MIN_EXTENT: f64 = 0.05;
    for item in &scene_state.engagement_preview {
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

pub fn append_component_overlays(selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, overlay_state: &WorldOverlayState, theme: &semio_framework_ui_theme::Theme, camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) {
    let wire_color = [0.55, 0.65, 0.8, 0.75];
    let outline_color = selection_state.show_edges.then(|| environment_outline_color(&scene_state.environment, theme)).flatten();
    let requested_wire = selection_state.interaction_mode == "paint" || component_mode_active(selection_state) || outline_color.is_some() || selection_state.selection_targets.edge || (selection_state.granularity == "mesh" && !selection_state.component_ids.is_empty());
    {
        for draw in &scene_state.draws {
            let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            let primary_wire = schema.indices == 0 && schema.edges > 0;
            let semantic = schema.edge_ids == schema.edges;
            if schema.edges == 0 || (!requested_wire && !primary_wire) || (outline_color.is_none() && !semantic && !primary_wire) {
                continue;
            }
            for instance in &draw.instances {
                let color = if primary_wire { mesh_style_paint(theme, resolve_mesh_style(world_instance_style(selection_state, instance))).line } else { outline_color.unwrap_or(wire_color) };
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
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok(schema) = mesh.schema() else { continue };
        if schema.edge_ids != schema.edges {
            continue;
        }
        for instance in &draw.instances {
            let selected = world_instance_selected_components(selection_state, gumball_state, scene_state,instance,&selection_state.granularity);
            let preview: HashSet<String> = selection_state.marquee_preview_ids.iter().filter(|_|selection_state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
            let hovered = instance_hovered_component_id(selection_state, &instance.id);
            if selection_state.granularity.as_str() != "edge" {
                continue;
            }
            if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                continue;
            }
            for edge_index in 0..schema.edges {
                let Ok(edge) = mesh.edge(edge_index) else { continue };
                let id = world_mesh_component_id(mesh, Mesh3dField::EdgeIds, edge_index).to_string();
                let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered, (overlay_state.sub_element_paint.edge_select, overlay_state.sub_element_paint.edge_hover)) else {
                    continue;
                };
                let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                push_line_segment(lines, a, b, color);
            }
        }
    }
    if face_component_mode_active(selection_state) {
        for draw in &scene_state.draws {
            let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            for instance in &draw.instances {
                let selected = world_instance_selected_components(selection_state, gumball_state, scene_state,instance,&selection_state.granularity);
                let preview: HashSet<String> = selection_state.marquee_preview_ids.iter().filter(|_|selection_state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
                let hovered = instance_hovered_component_id(selection_state, &instance.id);
                if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                    continue;
                }
                for tri_index in 0..schema.indices / 3 {
                    let Some(tri) = world_mesh_triangle(mesh, tri_index) else { continue };
                    let id = mesh_face_id(mesh, tri_index);
                    let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered, (overlay_state.sub_element_paint.face_select, overlay_state.sub_element_paint.face_hover)) else {
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
        for draw in &scene_state.draws {
            let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            if schema.vertex_ids != schema.vertices { continue; }
            let primary_point = schema.indices == 0 && schema.edges == 0;
            if !primary_point && !selection_state.selection_targets.vertex && selection_state.granularity != "vertex" { continue; }
            for instance in &draw.instances {
                let selected = world_instance_selected_components(selection_state, gumball_state, scene_state,instance,"vertex");
                let preview: HashSet<String> = selection_state.marquee_preview_ids.iter().filter(|_|selection_state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
                let hovered = instance_hovered_component_id(selection_state, &instance.id);
                for vertex_index in 0..schema.vertices {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                    let group = world_mesh_component_id(mesh, Mesh3dField::VertexIds, vertex_index);
                    if group == u32::MAX { continue; }
                    let id = group.to_string();
                    let center = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                    let base_color = if primary_point { mesh_style_paint(theme, resolve_mesh_style(world_instance_style(selection_state, instance))).line } else { wire_color };
                    let vertex_tokens = (overlay_state.sub_element_paint.vertex_select, overlay_state.sub_element_paint.vertex_hover);
                    let (color, scale) = component_overlay_color(&id, &selected, &preview, &hovered, vertex_tokens).unwrap_or((base_color, VERTEX_BASE_SCALE));
                    if !primary_point && !selection_state.selection_targets.vertex && component_overlay_color(&id, &selected, &preview, &hovered, vertex_tokens).is_none() {
                        continue;
                    }
                    let d = vertex_marker_half_extent(camera, viewport, center, scale, overlay_state.sub_element_paint.vertex_mark_px);
                    push_line_segment(lines, center.sub(Vec3::new(d, 0.0, 0.0)), center.add(Vec3::new(d, 0.0, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, d, 0.0)), center.add(Vec3::new(0.0, d, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, 0.0, d)), center.add(Vec3::new(0.0, 0.0, d)), color);
                }
            }
        }
    }
}

pub fn selection_centroid(selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState) -> Option<Vec3> {
    if let Some(target) = gumball_state.gumball_target {
        return Some(Vec3::new(target[0], target[1], target[2]));
    }
    if selection_state.selected_ids.is_empty() {
        return None;
    }
    let mut sum = Vec3::ZERO;
    let mut count = 0u32;
    for draw in &scene_state.draws {
        for instance in &draw.instances {
            if selection_state.selected_ids.iter().any(|id| id == &instance.id) {
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

#[cfg(any(test, feature = "testkit"))]
pub fn pick_gumball_handle_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<GumballHandle> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let pivot = selection_centroid(selection_state, gumball_state, scene_state)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
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
    if matches!(gumball_state.transform_mode.as_str(), "rotate" | "rotateSelection") {
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
    if matches!(gumball_state.transform_mode.as_str(), "scale" | "scaleSelection") {
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
pub fn append_gumball_geometry(
    lines: &mut Vec<LineVertex3d>,
    translucent: &mut Vec<SceneDraw3d>,
    gpu: &mut World3dBuildContext,
    selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState,
    camera: &Camera3d,
    meshes: &WorldMeshRegistry,
    mesh_versions: &WorldDynamicRegistry<u64, WORLD_DYNAMIC_MESH_CAPACITY>,
) {
    let Some(pivot) = selection_centroid(selection_state, gumball_state, scene_state) else {
        return;
    };
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let config = world3d_gumball_config(gumball_state);
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

#[cfg(any(test, feature = "testkit"))]
pub fn apply_gumball_preview(selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState) {
    let Some(handle) = gumball_state.gumball_handle else {
        return;
    };
    for draw in &mut scene_state.draws {
        for instance in &mut draw.instances {
            if selection_state.selected_ids.iter().any(|id| id == &instance.id) {
                instance.model = gumball_preview_in_place(instance.model, handle, gumball_state.gumball_preview_translate, gumball_state.gumball_preview_angle, gumball_state.gumball_preview_scale);
            }
        }
    }
}

/// 🎚️ One selected instance's gumball preview, exactly as the release commits it: a move translates the instance, a
/// turn rotates its basis about the handle's world axis IN PLACE — its own origin stays, because every 3D gumball
/// verb's leaf (puzzle `rotate-selection`, shooting `rotate-assets`, …) turns each target about its own origin — and
/// a scale stretches the instance's own local axis, as the per-axis factors of the scale leaves do. The gumball pivot
/// only anchors the widget; the selection never orbits it, so the multi-object preview equals the committed state.
pub fn gumball_preview_in_place(model: Mat4, handle: GumballHandle, translate: Vec3, angle: f32, scale: Vec3) -> Mat4 {
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

#[cfg(any(test, feature = "testkit"))]
pub fn reset_gumball_preview(gumball_state: &mut WorldGumballState) {
    gumball_state.gumball_preview_translate = Vec3::ZERO;
    gumball_state.gumball_preview_angle = 0.0;
    gumball_state.gumball_preview_scale = Vec3::new(1.0, 1.0, 1.0);
}

#[cfg(any(test, feature = "testkit"))]
pub fn gumball_commit_action(session_state: &WorldSessionState, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState) -> Option<ActionDescriptor> {
    let handle = gumball_state.gumball_handle?;
    let ids = selection_state.selected_ids.clone();
    if ids.is_empty() {
        return None;
    }
    if handle.is_translate() && gumball_state.gumball_preview_translate.length() > 1e-4 {
        let delta = gumball_state.gumball_preview_translate;
        return Some(ActionDescriptor {
            controller_id: session_state.controller_id.clone(),
            action: "translateSelection".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "windowId": session_state.surface_id,
                "mode": selection_mode_label(selection_state),
                "ids": ids,
                "dx": delta.x as f64,
                "dy": delta.y as f64,
                "dz": delta.z as f64,
            })),
        });
    }
    if handle.is_rotate() && gumball_state.gumball_preview_angle.abs() > 1e-6 {
        let axis = handle.axis_dir()?;
        return Some(ActionDescriptor {
            controller_id: session_state.controller_id.clone(),
            action: "rotateSelection".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "windowId": session_state.surface_id,
                "mode": selection_mode_label(selection_state),
                "ids": ids,
                "ax": axis.x as f64,
                "ay": axis.y as f64,
                "az": axis.z as f64,
                "angle": gumball_state.gumball_preview_angle as f64,
            })),
        });
    }
    if handle.is_scale() {
        let scale = match handle {
            GumballHandle::ScaleX => gumball_state.gumball_preview_scale.x,
            GumballHandle::ScaleY => gumball_state.gumball_preview_scale.y,
            GumballHandle::ScaleZ => gumball_state.gumball_preview_scale.z,
            _ => 1.0,
        };
        if (scale - 1.0).abs() > 1e-6 {
            let (sx, sy, sz) = match handle {
                GumballHandle::ScaleX => (scale, 1.0, 1.0),
                GumballHandle::ScaleY => (1.0, scale, 1.0),
                _ => (1.0, 1.0, scale),
            };
            let args = semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "windowId": session_state.surface_id,
                "mode": selection_mode_label(selection_state),
                "ids": ids,
                "sx": sx,
                "sy": sy,
                "sz": sz,
            });
            return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "scaleSelection".into(), args: Some(args) });
        }
    }
    None
}
//#endregion 🌉️World3dSceneBridge
