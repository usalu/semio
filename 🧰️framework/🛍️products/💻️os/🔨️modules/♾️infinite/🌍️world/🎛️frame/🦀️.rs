//! 🎛️ World frame sync, presentation, tool-run trace draws, modelling overlay and `render_world_3d`.

use super::*;

pub fn sync_world3d_state(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    state.bounds = bounds;
    let Some(world) = scene.world_3d.as_ref() else {
        state.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
        return;
    };
    state.bound_domain_id = world.domain_id.clone();
    state.bound_domain_granularity_id = world.domain_granularity_id.clone();
    state.presentation = world3d_presentation(world.presentation_json.as_deref());
    state.environment = world.environment_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    sync_world3d_tool_run_trace(state, world);
    sync_world3d_scene_fit(state, world.fit_json.as_deref());
    sync_world3d_projection_content_frame(state);
    sync_world3d_scene_selection(state, &world.selection_json);
    apply_world3d_pick_filter(state, world.modelling_options.as_ref().and_then(|options| options.pick_filter));
    sync_world3d_scene_document_lanes(state, world);
    sync_world3d_brush_mesh_feedback(state, world.interaction_json.as_deref());
    let lease = match world.snapshot {
        Some(lease) => lease,
        None => {
            stage_world3d_scene_bridge(state, world);
            let Some(lease) = state.scene_bridge_lease else {
                if state.scene_bridge.is_none() {
                    state.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
                }
                return;
            };
            lease
        }
    };
    if state.snapshot_lease == Some(lease) || state.snapshot_apply.as_ref().is_some_and(|cursor| cursor.lease == lease) {
        return;
    }
    if state.snapshot_apply.is_some() {
        state.snapshot_fault = Some(World3dSnapshotFault::Closing);
        return;
    }
    state.snapshot_apply = Some(World3dSnapshotApplyCursor::new(lease));
    state.snapshot_fault = None;
}

fn world3d_presentation(json: Option<&str>) -> World3dPresentation {
    let Some(json) = json else { return World3dPresentation::default() };
    let Ok(value) = serde_json::from_str::<World3dPresentation>(json) else { return World3dPresentation::default() };
    if value.source_aspect.is_some_and(|aspect| !aspect.is_finite() || aspect <= 0.0) {
        return World3dPresentation::default();
    }
    value
}

/// ⏯️ Feeds `World3dScene.tool_run_trace` into the resident trace layer and re-derives the mesh index →
/// mesh key table the trace subjects address (`meshes_json` order) only when that lane changed.
fn sync_world3d_tool_run_trace(state: &mut World3dState, world: &ui_wgpu::wgpu::World3dScene) {
    let _ = state.tool_run_trace.apply_lane(world.tool_run_trace.as_deref());
    if state.tool_run_trace.is_empty() {
        return;
    }
    let digest = world3d_scene_digest(&[&world.meshes_json]);
    if state.tool_run_trace_meshes_digest == Some(digest) {
        return;
    }
    #[derive(Deserialize)]
    struct MeshKey {
        id: String,
    }
    state.tool_run_trace_mesh_keys = serde_json::from_str::<Vec<MeshKey>>(&world.meshes_json).map(|meshes| meshes.into_iter().map(|mesh| mesh.id).collect()).unwrap_or_default();
    state.tool_run_trace_meshes_digest = Some(digest);
}

/// 🧭️ The cursor this surface echoes as `toolRunTraceCursorByWindowId[window]` in its window instance view state.
pub fn world3d_tool_run_trace_cursor(state: &World3dState) -> Option<semio_framework_tool_run::ToolRunTraceCursor> {
    state.tool_run_trace.cursor()
}

/// 🧭️ Every `(window instance, cursor)` echo the given world surfaces owe their view state — surfaces without
/// a bound window or without an applied trace lane echo nothing.
pub fn world3d_tool_run_trace_cursors<'a>(states: impl IntoIterator<Item = &'a World3dState>) -> HashMap<String, semio_framework_tool_run::ToolRunTraceCursor> {
    states.into_iter().filter_map(|state| Some((state.tool_run_trace_window_id.clone()?, state.tool_run_trace.cursor()?))).collect()
}

/// 🎨️ Appends the trace layer's instanced `(mesh, verdict)` draws; a mesh index with no published mesh
/// falls back to the placeholder box, exactly like the brush ghost.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn append_tool_run_trace_draws(state: &mut World3dState, gpu: &mut World3dBuildContext, theme: &ui_wgpu::wgpu::Theme, translucent_draws: &mut Vec<SceneDraw3d>) {
    if state.tool_run_trace.is_empty() {
        return;
    }
    let palette = tool_run_trace::ToolRunTracePalette::from_theme(theme);
    for draw in state.tool_run_trace.draws(&palette, state.tool_run_trace_visibility) {
        let mesh_key = state.tool_run_trace_mesh_keys.get(draw.mesh as usize).filter(|key| state.meshes.contains_key(key.as_str())).cloned().unwrap_or_else(|| brush_preview_mesh_id(None));
        if !state.meshes.contains_key(&mesh_key) {
            begin_world_placeholder_mesh(state, &mesh_key, WorldPlaceholderKind::Box);
        }
        let mesh_version = live_world3d_mesh_version(state, &mesh_key);
        if let Some(mesh) = state.meshes.get(&mesh_key).copied() {
            gpu.ensure_mesh(&mesh_key, mesh_version, mesh);
            translucent_draws.push(SceneDraw3d { mesh_key, mesh_version, instances: draw.instances, shadow_role: Default::default() });
        }
    }
}

/// 🎯️ A published id may name the rendered INSTANCE or the TOPOLOGY target it stands for, and both
/// reach this state: the guest's `selectionJson.ids` carries render ids (pinned by
/// `🧫️fixtures/🌉️scene-bridge/🔣️.json`'s `selectedIds`), while this surface's OWN optimistic
/// `local_hover_id` is the topology id it just dispatched in `interactionHover`. Matching only one
/// of the two leaves half the highlights dark. React resolves the same pair
/// (`useWorldInstanceChrome(instance.id)` over a selection the host maps back through
/// `interactionId`). See [`instance_interaction_id`].
///
/// 🎨️ Selection/hover flags must follow the live selection snapshot — OR-ing with the
/// instancesJson bits left deselected meshes painted selected until a later hover rebuild.
pub(super) fn apply_runtime_draw_flags(state: &mut World3dState) {
    let granularity = state.granularity.clone();
    let component_ids: HashSet<String> = state.component_ids.iter().cloned().collect();
    let local_hover_id = state.local_hover_id.clone();
    let hovered_component_object_id = state.hovered_component_object_id.clone();
    let selected_ids: HashSet<String> = state.selected_ids.iter().cloned().collect();
    let interaction_ids = state.instance_interaction_ids.clone();
    let mut object_index_map = HashMap::new();
    let mut index = 0u32;
    for draw in &state.draws {
        for instance in &draw.instances {
            object_index_map.insert(instance.id.clone(), index);
            index += 1;
        }
    }
    let component_mode = component_mode_active(state);
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            let mesh_selected = granularity == "mesh" && object_index_map.get(&instance.id).is_some_and(|object_index| component_ids.contains(&object_index.to_string()));
            let target_id = interaction_ids.get(&instance.id).map_or(instance.id.as_str(), String::as_str);
            let local_hovered = if component_mode { false } else { local_hover_id.as_deref() == Some(instance.id.as_str()) || local_hover_id.as_deref() == Some(target_id) || hovered_component_object_id.as_deref() == Some(instance.id.as_str()) };
            let local_selected = selected_ids.contains(&instance.id) || selected_ids.contains(target_id) || mesh_selected;
            instance.hovered = local_hovered;
            instance.selected = local_selected;
        }
    }
}

/// ✂️ Closes the cut of a capped section plane over the clipped solids, the native twin of the React `World3dSectionClip` cap: the clipped solids are drawn once with `StencilToggle`
/// (each fragment inverts the stencil's section bit, so after both faces of a cut solid the bit is set exactly where the cut exposes its inside), then the flat cap plane in the cap tone
/// is drawn only where the bit is set, then the solids are drawn a second time to invert the bit back to zero. The three passes ride the translucent draw list in that order,
/// after the opaque solids, so the cap is depth-tested against the retained surface.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn append_world3d_section_cap(state: &mut World3dState, gpu: &mut World3dBuildContext, theme: &ui_wgpu::wgpu::Theme, section: &ui_wgpu::wgpu::World3dSection, solids: &[SceneDraw3d], translucent: &mut Vec<SceneDraw3d>) {
    let Some(cap) = section.cap else { return };
    if solids.iter().all(|draw| draw.instances.is_empty()) {
        return;
    }
    if !state.meshes.contains_key(GUMBALL_PLANE_MESH) {
        begin_world_placeholder_mesh(state, GUMBALL_PLANE_MESH, WorldPlaceholderKind::Plane);
    }
    let Some(plane) = state.meshes.get(GUMBALL_PLANE_MESH).copied() else { return };
    let plane_version = live_world3d_mesh_version(state, GUMBALL_PLANE_MESH);
    gpu.ensure_mesh(GUMBALL_PLANE_MESH, plane_version, plane);
    let (mut low, mut high) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    for draw in solids {
        let Some(Ok((mesh_low, mesh_high))) = state.meshes.get(&draw.mesh_key).map(|mesh| mesh.aabb()) else { continue };
        for instance in &draw.instances {
            let (instance_low, instance_high) = transform_aabb(instance.model, mesh_low, mesh_high);
            for axis in 0..3 {
                low[axis] = low[axis].min(instance_low[axis]);
                high[axis] = high[axis].max(instance_high[axis]);
            }
        }
    }
    if !(0..3).all(|axis| low[axis].is_finite() && high[axis].is_finite()) {
        return;
    }
    let diagonal = Vec3::new(high[0] - low[0], high[1] - low[1], high[2] - low[2]).length();
    let side = (4.0 * diagonal).max(2.0);
    let normal = section.unit_normal();
    let normal = Vec3::new(normal[0] as f32, normal[1] as f32, normal[2] as f32);
    let helper = if normal.y.abs() < 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
    let tangent = helper.cross(normal).normalize();
    let rotation = quat_from_basis(tangent, normal, tangent.cross(normal));
    let origin = [section.origin[0] as f32, section.origin[1] as f32, section.origin[2] as f32];
    let toggled = || -> Vec<SceneDraw3d> {
        solids
            .iter()
            .filter(|draw| !draw.instances.is_empty())
            .map(|draw| SceneDraw3d {
                mesh_key: draw.mesh_key.clone(),
                mesh_version: draw.mesh_version,
                instances: draw.instances.iter().map(|instance| Instance3d { material: SceneInstanceMaterial3d { section: SceneSectionRole3d::StencilToggle, ..instance.material }, ..instance.clone() }).collect(),
                shadow_role: Default::default(),
            })
            .collect()
    };
    translucent.extend(toggled());
    translucent.push(SceneDraw3d {
        mesh_key: GUMBALL_PLANE_MESH.into(),
        mesh_version: plane_version,
        instances: vec![Instance3d {
            component_source: None,
            id: "section-cap".into(),
            model: Instance3d::model_from_trs(origin, rotation, [side, 1.0, side]),
            color: world3d_tone_rgba(theme, cap.tone),
            selected: false,
            hovered: false,
            material: SceneInstanceMaterial3d { section: SceneSectionRole3d::Cap, ..Default::default() },
        }],
        shadow_role: Default::default(),
    });
    translucent.extend(toggled());
}

//#region 📏️Modelling overlay
const WORLD_OVERLAY_ARC_SEGMENT_PX: f64 = 6.0;
const WORLD_OVERLAY_LEGEND_MARGIN_PX: f32 = 8.0;
const WORLD_OVERLAY_LEGEND_PADDING_PX: f32 = 6.0;
const WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX: f32 = 12.0;
const WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX: f32 = 96.0;
const WORLD_OVERLAY_LEGEND_BAR_SLICES: usize = 32;
const WORLD_OVERLAY_LEGEND_GAP_PX: f32 = 6.0;
const WORLD_OVERLAY_LABEL_PADDING_X_PX: f32 = 4.0;
const WORLD_OVERLAY_LABEL_PADDING_Y_PX: f32 = 1.0;

/// 📐️ The three corners of a dimension arrowhead whose tip sits at `(x, y)` and points along `angle` — React's `arrowPoints`.
fn world3d_arrow_points(x: f64, y: f64, angle: f64) -> [[f32; 2]; 3] {
    let length = ui_wgpu::wgpu::WORLD3D_ANNOTATION_ARROW_PX;
    let (cos, sin) = (angle.cos(), angle.sin());
    let (base_x, base_y) = (x - cos * length, y - sin * length);
    let half = length * 0.35;
    [[x as f32, y as f32], [(base_x - sin * half) as f32, (base_y + cos * half) as f32], [(base_x + sin * half) as f32, (base_y - cos * half) as f32]]
}

/// 🌙️ The polyline of an angle arc, one vertex per `WORLD_OVERLAY_ARC_SEGMENT_PX` of arc length (at least four, at most thirty-two segments).
fn world3d_arc_points(arc: &ui_wgpu::wgpu::World3dProjectedArc) -> Vec<[f32; 2]> {
    let segments = ((arc.sweep.abs() * arc.radius / WORLD_OVERLAY_ARC_SEGMENT_PX).ceil() as usize).clamp(4, 32);
    (0..=segments)
        .map(|index| {
            let angle = arc.start + arc.sweep * index as f64 / segments as f64;
            [(arc.cx + angle.cos() * arc.radius) as f32, (arc.cy + angle.sin() * arc.radius) as f32]
        })
        .collect()
}

fn world3d_circle_points(x: f64, y: f64, radius: f64) -> Vec<[f32; 2]> {
    (0..=16).map(|index| index as f64 / 16.0 * std::f64::consts::TAU).map(|angle| [(x + angle.cos() * radius) as f32, (y + angle.sin() * radius) as f32]).collect()
}

fn world3d_overlay_color(theme: &ui_wgpu::wgpu::Theme, tone: ui_wgpu::wgpu::World3dTone) -> Rgba {
    let color = world3d_tone_rgba(theme, tone);
    Rgba::new(color[0], color[1], color[2], color[3])
}

/// 📏️ Paints the projected annotation layer on top of the world pass: lines, arrowheads, arcs and marks at screen-constant size in the tone colours of the live theme,
/// and the caller's text in the active language on a translucent plate. Clipped to the viewport by the caller's scissor.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn paint_world3d_annotations(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, layer: &ui_wgpu::wgpu::World3dAnnotationLayer, view_proj: &[f32; 16], inner: Rect, locale: &str) {
    use ui_wgpu::wgpu::{World3dLabelAlign, World3dMarkerShape};
    let theme = ctx.theme;
    let stroke = ui_wgpu::wgpu::WORLD3D_ANNOTATION_STROKE_PX as f32;
    let marker_radius = ui_wgpu::wgpu::WORLD3D_ANNOTATION_MARKER_PX / 2.0;
    let size = theme.font_size_small;
    let (origin_x, origin_y) = (f64::from(inner.x), f64::from(inner.y));
    let at = |points: Vec<[f32; 2]>| -> Vec<[f32; 2]> { points.into_iter().map(|point| [point[0] + inner.x, point[1] + inner.y]).collect() };
    let projected = ui_wgpu::wgpu::project_world3d_annotations(layer, view_proj, inner.w, inner.h);
    for (item, entry) in layer.items.iter().zip(&projected).filter(|(_, entry)| entry.visible) {
        let color = world3d_overlay_color(theme, entry.tone);
        for line in &entry.lines {
            ctx.draw.push_line_overlay(inner.x + line[0] as f32, inner.y + line[1] as f32, inner.x + line[2] as f32, inner.y + line[3] as f32, color, stroke);
        }
        for arrow in &entry.arrows {
            ctx.draw.push_triangle_fan_overlay(&at(world3d_arrow_points(arrow.x, arrow.y, arrow.angle).to_vec()), color);
        }
        if let Some(arc) = entry.arc.as_ref() {
            for pair in at(world3d_arc_points(arc)).windows(2) {
                ctx.draw.push_line_overlay(pair[0][0], pair[0][1], pair[1][0], pair[1][1], color, stroke);
            }
        }
        if let Some(marker) = entry.marker {
            let (x, y) = (marker.x + origin_x, marker.y + origin_y);
            match marker.shape {
                World3dMarkerShape::Dot => ctx.draw.push_rounded_overlay([(x - marker_radius / 2.0) as f32, (y - marker_radius / 2.0) as f32, marker_radius as f32, marker_radius as f32], color, marker_radius as f32 / 2.0),
                World3dMarkerShape::Cross => {
                    for (dx, dy) in [(-1.0, -1.0), (-1.0, 1.0)] {
                        ctx.draw.push_line_overlay((x + dx * marker_radius) as f32, (y + dy * marker_radius) as f32, (x - dx * marker_radius) as f32, (y - dy * marker_radius) as f32, color, stroke);
                    }
                }
                World3dMarkerShape::Ring => {
                    for pair in world3d_circle_points(x, y, marker_radius).windows(2) {
                        ctx.draw.push_line_overlay(pair[0][0], pair[0][1], pair[1][0], pair[1][1], color, stroke);
                    }
                }
            }
        }
        if let Some(label) = entry.label {
            let text = item.text().resolve(locale);
            let (width, height) = ctx.atlas.measure_text(text, size);
            let left = match label.align {
                World3dLabelAlign::Middle => label.x - f64::from(width) / 2.0,
                World3dLabelAlign::Start => label.x,
                World3dLabelAlign::End => label.x - f64::from(width),
            } as f32
                + inner.x;
            let top = label.y as f32 + inner.y - height / 2.0;
            let plate = Rgba::new(theme.background.r, theme.background.g, theme.background.b, 0.8);
            ctx.draw.push_rounded_overlay([left - WORLD_OVERLAY_LABEL_PADDING_X_PX, top - WORLD_OVERLAY_LABEL_PADDING_Y_PX, width + 2.0 * WORLD_OVERLAY_LABEL_PADDING_X_PX, height + 2.0 * WORLD_OVERLAY_LABEL_PADDING_Y_PX], plate, theme.border_radius.min(4.0));
            ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, left, label.y as f32 + inner.y + size * 0.35, size, color);
        }
    }
}

/// 🌡️ The panel rectangle of a scalar-field legend (or its mismatch message) pinned to the viewport's lower left, `content` pixels wide and tall.
fn world3d_legend_rect(inner: Rect, content: (f32, f32)) -> [f32; 4] {
    let (width, height) = (content.0 + 2.0 * WORLD_OVERLAY_LEGEND_PADDING_PX, content.1 + 2.0 * WORLD_OVERLAY_LEGEND_PADDING_PX);
    [inner.x + WORLD_OVERLAY_LEGEND_MARGIN_PX, inner.y + inner.h - WORLD_OVERLAY_LEGEND_MARGIN_PX - height, width, height]
}

/// 🌡️ Paints the scalar-field legend: the title with its unit, the ramp as a vertical gradient (range minimum at the bottom), the evenly spaced tick values beside it and the no-data caption.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn paint_world3d_legend(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, field: &ui_wgpu::wgpu::World3dScalarField, inner: Rect, locale: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_small;
    let lines = field.legend_lines(locale);
    let (title_width, title_height) = ctx.atlas.measure_text(&lines.title, size);
    let tick_width = lines.ticks.iter().map(|tick| ctx.atlas.measure_text(&tick.text, size).0).fold(0.0_f32, f32::max);
    let no_data = lines.no_data.map(|text| (text, ctx.atlas.measure_text(text, size)));
    let ramp_width = WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX + WORLD_OVERLAY_LEGEND_GAP_PX + tick_width;
    let no_data_width = no_data.map_or(0.0, |(_, (width, _))| 10.0 + WORLD_OVERLAY_LEGEND_GAP_PX + width);
    let no_data_height = no_data.map_or(0.0, |(_, (_, height))| WORLD_OVERLAY_LEGEND_GAP_PX + height.max(10.0));
    let content = (title_width.max(ramp_width).max(no_data_width), title_height + WORLD_OVERLAY_LEGEND_GAP_PX + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX + no_data_height);
    let rect = world3d_legend_rect(inner, content);
    ctx.draw.push_rounded_overlay(rect, Rgba::new(theme.panel.r, theme.panel.g, theme.panel.b, 0.9), theme.border_radius.min(4.0));
    let (left, top) = (rect[0] + WORLD_OVERLAY_LEGEND_PADDING_PX, rect[1] + WORLD_OVERLAY_LEGEND_PADDING_PX);
    ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, &lines.title, left, top + size, size, theme.text);
    let bar_top = top + title_height + WORLD_OVERLAY_LEGEND_GAP_PX;
    let slice = WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX / WORLD_OVERLAY_LEGEND_BAR_SLICES as f32;
    for index in 0..WORLD_OVERLAY_LEGEND_BAR_SLICES {
        let [r, g, b] = field.ramp.sample(1.0 - (index as f64 + 0.5) / WORLD_OVERLAY_LEGEND_BAR_SLICES as f64);
        ctx.draw.push_solid_overlay([left, bar_top + index as f32 * slice, WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX, slice + 0.5], Rgba::from_srgb8(r, g, b, 255));
    }
    let steps = lines.ticks.len().saturating_sub(1).max(1) as f32;
    for (index, tick) in lines.ticks.iter().enumerate() {
        let y = bar_top + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX - index as f32 / steps * WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX;
        ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, &tick.text, left + WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX + WORLD_OVERLAY_LEGEND_GAP_PX, y + size * 0.35, size, theme.text);
    }
    if let Some((text, (_, height))) = no_data {
        let y = bar_top + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX + WORLD_OVERLAY_LEGEND_GAP_PX;
        let [r, g, b] = ui_wgpu::wgpu::WORLD3D_SCALAR_NO_DATA_RGB;
        ctx.draw.push_solid_overlay([left, y, 10.0, 10.0], Rgba::from_srgb8(r, g, b, 255));
        ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, left + 10.0 + WORLD_OVERLAY_LEGEND_GAP_PX, y + height.max(10.0) * 0.5 + size * 0.35, size, theme.text);
    }
}

/// 🌡️ Announces on the viewport that the scalar field does not fit the mesh it names, in place of a wrong heatmap.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn paint_world3d_scalar_mismatch(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, inner: Rect, locale: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_small;
    let text = ui_wgpu::wgpu::world3d_modelling_strings(locale).mismatch;
    let (width, height) = ctx.atlas.measure_text(text, size);
    let rect = world3d_legend_rect(inner, (width, height));
    ctx.draw.push_rounded_overlay(rect, Rgba::new(theme.panel.r, theme.panel.g, theme.panel.b, 0.9), theme.border_radius.min(4.0));
    ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, rect[0] + WORLD_OVERLAY_LEGEND_PADDING_PX, rect[1] + WORLD_OVERLAY_LEGEND_PADDING_PX + size, size, theme.error);
}

/// ♿️ One node of the accessibility tree a world surface publishes for its modelling overlays: a stable key, an ARIA role, the text a screen reader speaks and its nesting below the surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dAccessibilityEntry {
    pub key: String,
    pub role: &'static str,
    pub label: String,
    pub depth: u8,
    pub polite: bool,
}

/// ♿️ The text alternative of everything `render_world_3d` paints for the modelling layer, in the surface's language and with the same structure as the React host: a `list` of annotations
/// named by the layer title, one `listitem` per annotation (`Dimension: Width 40 mm`), a `group` for an applied scalar-field legend with its scale ticks and no-data caption, or a polite `status`
/// when the field does not fit its mesh. The shell hands these to the accessibility mirror (the screen reader's `aria-live` region), so nothing drawn here is pointer-only.
pub fn world3d_modelling_accessibility(state: &World3dState, scene: &UiComponentSceneNode) -> Vec<World3dAccessibilityEntry> {
    let Some(world) = scene.world_3d.as_ref() else { return Vec::new() };
    let locale = state.modelling_locale;
    let strings = ui_wgpu::wgpu::world3d_modelling_strings(locale);
    let host = scene.host_id.as_str();
    let entry = |key: String, role: &'static str, label: String, depth: u8, polite: bool| World3dAccessibilityEntry { key, role, label, depth, polite };
    let mut entries = Vec::new();
    if let Some(layer) = world.annotations.as_ref().filter(|layer| !layer.items.is_empty()) {
        entries.push(entry(format!("{host}::annotations"), "list", layer.title.as_ref().map_or(strings.annotations, |title| title.resolve(locale)).to_string(), 0, false));
        entries.extend(layer.items.iter().map(|item| entry(format!("{host}::annotation::{}", item.id()), "listitem", item.accessible_name(locale), 1, false)));
    }
    if let Some(field) = world.scalar_field.as_ref() {
        match state.scalar_field_status {
            World3dScalarFieldStatus::Applied => {
                let lines = field.legend_lines(locale);
                entries.push(entry(format!("{host}::legend"), "group", format!("{}: {}", strings.legend, field.legend.title.resolve(locale)), 0, false));
                entries.push(entry(format!("{host}::legend::scale"), "list", strings.scale.to_string(), 1, false));
                entries.extend(lines.ticks.iter().enumerate().map(|(index, tick)| entry(format!("{host}::legend::tick::{index}"), "listitem", tick.text.clone(), 2, false)));
                if let Some(caption) = lines.no_data {
                    entries.push(entry(format!("{host}::legend::no-data"), "paragraph", caption.to_string(), 1, false));
                }
            }
            World3dScalarFieldStatus::Mismatch => entries.push(entry(format!("{host}::scalar-field-mismatch"), "status", strings.mismatch.to_string(), 0, true)),
            World3dScalarFieldStatus::None | World3dScalarFieldStatus::Missing => {}
        }
    }
    entries
}
//#endregion 📏️Modelling overlay

// 🖥️ Native/browser rendering-host entry point only (called from `📺️renderer`'s engine, never from
// wasip2 plugin guest logic — confirmed by grepping every `render_world_3d` call site in the repo).
// `WidgetContext` bundles the font/icon atlases, which are genuinely GPU-adjacent (real `wgpu`
// crate reachable through `wgpu-engine`), so this one function stays excluded from
// `wasm32-wasip2`: `target_arch = "wasm32"` is TRUE for wasip2 too, hence `not(target_env = "p2")`.
/// 👻️ Mirrors `BrushPreviewGhost`: renders whenever `origin` is present, regardless of
/// `meshUrl` — a translucent unit box is the fallback ghost when there's no mesh URL (or
/// its GLB hasn't resolved into `state.meshes` yet), not "nothing at all". The box itself is
/// a placeholder LEASE, so the ghost appears on the frame that lease lands, never before it.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn render_world_3d(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>,
    state: &mut World3dState,
    gpu: &mut World3dBuildContext,
    shadow_profile: World3dShadowProfile,
) {
    use ui_wgpu::wgpu::widgets::{draw_text, gizmo as gpu_gizmo};
    let theme = ctx.theme;
    let modelling = scene.world_3d.as_ref();
    let section = modelling.and_then(|world| world.modelling_options.as_ref()).and_then(|options| options.section);
    let sub_element_paint = world3d_sub_element_paint_native(theme, modelling.and_then(|world| world.modelling_options.as_ref()).and_then(|options| options.highlight.as_ref()));
    if state.sub_element_paint != sub_element_paint {
        state.sub_element_paint = sub_element_paint;
        advance_world3d_view_revision(state);
    }
    step_world_placeholder_mesh(state);
    state.pick_bounds = ctx.pick_clip.unwrap_or(bounds);
    sync_world3d_state(state, scene, bounds);
    let _ = step_world3d_camera_fit(state);
    apply_terrain_style_if_changed(state, gpu);
    let current_lod = scene_lod(state);
    let lod_changed = state.resolved_lod_pick.is_none_or(|previous| (previous - current_lod).abs() > WORLD_LOD_EPSILON);
    if lod_changed {
        state.resolved_lod_pick = Some(current_lod);
    }
    apply_runtime_draw_flags(state);
    let inner = bounds;
    let clear_color = match state.presentation.clear {
        World3dPresentationClear::Theme => Some(environment_clear_color(&state.environment, theme.canvas_clear)),
        World3dPresentationClear::Transparent => environment_explicit_clear_color(&state.environment),
    };
    if state.presentation.viewport_mask == SceneViewportMask3d::Rectangle {
        if let Some(clear_color) = clear_color {
            ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], clear_color);
        }
    }
    let camera = state.orbit.to_camera();
    let light_dir = environment_light_dir(&state.environment);
    let lighting = environment_scene_lighting(&state.environment);
    let neutral_material = environment_scene_material(&state.environment);
    let shadow = environment_scene_shadow(&state.environment, light_dir, shadow_profile);
    let mut terrain_draws = sync_terrain(state, gpu, &camera);
    for draw in &mut terrain_draws {
        draw.shadow_role = world3d_shadow_role(World3dShadowGeometry::Terrain, shadow.enabled);
    }
    update_visible_chunks(state, camera.position);
    let projection_height = state.presentation.source_aspect.map_or(inner.h, |aspect| inner.w / aspect as f32);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.projection_spec, inner.w, projection_height);
    let planes = frustum_planes(view_proj);
    let shadow_planes = directional_shadow_frustum_planes(light_dir);
    let mut culled_draws = Vec::new();
    let mut material_draws = Vec::new();
    let mut shadow_draws = Vec::new();
    let mut culled_count = 0u32;
    let mut needed_mesh_keys = HashSet::new();
    offer_missing_mesh_fetches(state);
    for (draw_index, draw) in state.draws.iter().enumerate() {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok((mesh_min, mesh_max)) = mesh.aabb() else { continue };
        let mesh_version = live_world3d_mesh_version(state, &draw.mesh_key);
        let mut instances = Vec::new();
        let mut painted_instances = Vec::new();
        let mut celebration_instances = Vec::new();
        let mut authored_instances = Vec::new();
        let mut shadow_instances = Vec::new();
        let schema = mesh.schema().ok();
        let has_vertex_colors = schema.is_some_and(|schema| schema.colors > 0);
        let paint_texture = schema.filter(|schema| schema.uvs == schema.vertices && schema.uvs > 0).and_then(|_| state.mesh_paint_textures.get(&draw.mesh_key)).map(|raster| raster.identity);
        let [metalness, roughness] = world3d_standard_material_for_mesh(state, &draw.mesh_key, neutral_material);
        for (instance_index, source) in draw.instances.iter().enumerate() {
            let style = world_instance_style(state, source);
            let mut instance = world3d_style_paint(theme, style, has_vertex_colors, source.clone());
            if section.is_some() {
                instance.material.section = SceneSectionRole3d::Clipped;
            }
            let style_kind = resolve_mesh_style(style);
            instance.material.metalness = metalness;
            instance.material.roughness = roughness;
            instance.model = retained_gumball_preview_model(state, draw_index, instance_index, instance.model);
            let position = state.instance_positions.get(&instance.id).copied().unwrap_or([0.0, 0.0, 0.0]);
            if !instance_chunk_visible(state, position) {
                continue;
            }
            let (min, max) = transform_aabb(instance.model, mesh_min, mesh_max);
            if shadow.enabled && aabb_intersects_frustum(&shadow_planes, min, max) {
                shadow_instances.push(instance.clone());
            }
            if aabb_intersects_frustum(&planes, min, max) {
                if matches!(style_kind,MeshStyleKind::Neutral|MeshStyleKind::Disabled) && state.mesh_appearances.contains_key(&draw.mesh_key) {
                    authored_instances.push(instance);
                } else if style_kind == MeshStyleKind::Celebrated && !instance.material.preserve_vertex_color {
                    celebration_instances.push(instance);
                } else if paint_texture.is_some() {
                    painted_instances.push(instance);
                } else {
                    instances.push(instance);
                }
            } else {
                culled_count += 1;
            }
        }
        if !instances.is_empty() || !painted_instances.is_empty() || !celebration_instances.is_empty() || !authored_instances.is_empty() || !shadow_instances.is_empty() {
            needed_mesh_keys.insert(draw.mesh_key.clone());
            gpu.ensure_mesh(&draw.mesh_key, mesh_version, mesh);
        }
        if paint_texture.is_some() && !painted_instances.is_empty() {
            let texture_key = format!("mesh-paint:{}", draw.mesh_key);
            if let Some(lease) = world_raster_upload(state, &draw.mesh_key, &texture_key, true) {
                gpu.ensure_world_plane_texture(&texture_key, lease);
            }
        }
        if !authored_instances.is_empty() {
            let pool = world_scene_raster_pool();
            let override_material = environment_authored_material(&state.environment);
            if let Some(appearance) = state.mesh_appearances.get(&draw.mesh_key) {
                for primitive in appearance.primitives() {
                    let material = override_material.clone().unwrap_or_else(|| primitive.material.clone());
                    for texture_key in SceneMaterialKind3d::Authored(material.clone()).texture_keys() {if let Some(lease)=appearance.texture_upload(texture_key,&pool){gpu.ensure_world_plane_texture(texture_key,lease);}}
                    material_draws.push(SceneMaterialDraw3d {
                        mesh_key: draw.mesh_key.clone(),
                        mesh_version,
                        first_index: primitive.first_index,
                        index_count: primitive.index_count,
                        instances: authored_instances.clone(),
                        translucent: material.alpha == SceneMaterialAlpha3d::Blend,
                        material: SceneMaterialKind3d::Authored(material),
                    });
                }
            }
        }
        let glb_shadow_role = world3d_shadow_role(World3dShadowGeometry::Glb, shadow.enabled);
        if !shadow_instances.is_empty() {
            shadow_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances: shadow_instances, shadow_role: glb_shadow_role });
        }
        let instances = instances.into_iter().map(|instance| {
            if state.provisional_instance_ids.contains(&instance.id) {
                Instance3d { selected: false, hovered: false, ..instance }
            } else {
                instance
            }
        });
        let (translucent, opaque): (Vec<_>, Vec<_>) = instances.partition(|instance| instance.color[3] < 1.0);
        if !opaque.is_empty() {
            culled_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances: opaque, shadow_role: glb_shadow_role });
        }
        if !translucent.is_empty() {
            material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Standard, translucent: true });
        }
        if !painted_instances.is_empty() {
            let texture_key = format!("mesh-paint:{}", draw.mesh_key);
            let (opaque, translucent): (Vec<_>, Vec<_>) = painted_instances.into_iter().partition(|instance| instance.color[3] >= 1.0);
            if !opaque.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: opaque, material: SceneMaterialKind3d::Painted { texture_key: texture_key.clone() }, translucent: false });
            }
            if !translucent.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Painted { texture_key }, translucent: true });
            }
        }
        if !celebration_instances.is_empty() {
            let stops = mesh_style_paint(theme, MeshStyleKind::Celebrated).conic.expect("celebrated style owns the conic triad");
            let angle = (ctx.draw.clock_seconds() / CELEBRATE_CONIC_SPIN_SECONDS).rem_euclid(1.0) * std::f32::consts::TAU;
            let (opaque, translucent): (Vec<_>, Vec<_>) = celebration_instances.into_iter().partition(|instance| instance.color[3] >= 1.0);
            if !opaque.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: opaque, material: SceneMaterialKind3d::Celebration { stops, angle }, translucent: false });
            }
            if !translucent.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Celebration { stops, angle }, translucent: true });
            }
        }
    }
    sync_mesh_pool(state, &needed_mesh_keys, gpu);
    let procedural_grid = if state.presentation.show_grid && state.lod.show_grid {
        let datum = state.lod.grid_datum.unwrap_or([0.0, 0.0, 0.0]);
        lod_grid_step_world(current_lod, state.lod.grid_factor).and_then(|step| {
            let cell_size = step as f32;
            if !cell_size.is_finite() || cell_size <= 0.0 {
                return None;
            }
            let plane_z = datum[2] as f32 + 0.001;
            Some(ProceduralGrid3d {
                plane_z,
                camera_plane_projection: [camera.position.x, camera.position.y, plane_z],
                cell_size,
                fade_distance: camera_grid_fade_distance(&camera, datum[2] as f32, step, inner.w, inner.h),
                cell_color: [theme.text_element.r, theme.text_element.g, theme.text_element.b],
            })
        })
    } else {
        None
    };
    let mut line_vertices = Vec::new();
    append_component_overlays(state, theme, &camera, inner, &mut line_vertices);
    append_engagement_preview_lines(state, &mut line_vertices, [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, 1.0]);
    append_pick_target_hover_lines(state, theme, &camera, inner, &mut line_vertices);
    for attraction in &state.attractions {
        let Some(from) = attraction.from else { continue };
        let Some(to) = attraction.to else { continue };
        let color = parse_color(attraction.color.as_deref().unwrap_or("#60a5fa"));
        line_vertices.push(LineVertex3d { position: [from[0] as f32, from[1] as f32, from[2] as f32], color });
        line_vertices.push(LineVertex3d { position: [to[0] as f32, to[1] as f32, to[2] as f32], color });
    }
    for volume in &state.target_volumes {
        append_box_wireframe(&mut line_vertices, volume.origin.unwrap_or([0.0, 0.0, 0.0]), volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), volume.scale.unwrap_or([1.0, 1.0, 1.0]), parse_color(volume.color.as_deref().unwrap_or("#f472b6")));
    }
    let mut extra_draws = Vec::new();
    append_vortex_arrow_draws(state, gpu, &mut extra_draws);
    let vertex_instances = append_component_vertex_spheres(state);
    if !vertex_instances.is_empty() {
        let mesh_version = live_world3d_mesh_version(state, VERTEX_MARKER_MESH);
        if let Some(mesh) = state.meshes.get(VERTEX_MARKER_MESH) {
            gpu.ensure_mesh(VERTEX_MARKER_MESH, mesh_version, *mesh);
            extra_draws.push(SceneDraw3d { mesh_key: VERTEX_MARKER_MESH.into(), mesh_version, instances: vertex_instances, shadow_role: Default::default() });
        }
    }
    let mut translucent_draws = Vec::new();
    append_component_face_translucent_overlays(state, gpu, &mut translucent_draws);
    append_tool_run_trace_draws(state, gpu, theme, &mut translucent_draws);
    if let Some(preview) = state.catalogue_drop_preview.clone() {
        let mesh_id = ghost_mesh_id(state, preview.mesh_url.as_deref());
        if !state.meshes.contains_key(&mesh_id) {
            begin_world_placeholder_mesh(state, &mesh_id, WorldPlaceholderKind::Box);
        }
        let mesh_version = live_world3d_mesh_version(state, &mesh_id);
        if let Some(mesh) = state.meshes.get(&mesh_id).copied() {
            gpu.ensure_mesh(&mesh_id, mesh_version, mesh);
            translucent_draws.push(SceneDraw3d {
                mesh_key: mesh_id,
                mesh_version,
                instances: vec![Instance3d { component_source: None,
                    id: "catalogue-drop-preview".into(),
                    model: Instance3d::model_from_trs([preview.origin[0] as f32, preview.origin[1] as f32, preview.origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]),
                    color: {
                        let mut color = parse_color("#59bfff");
                        color[3] = 0.55;
                        color
                    },
                    selected: false,
                    hovered: false,
                 material: Default::default() }],
                shadow_role: Default::default(),
            });
        }
    }
    if let Some(preview) = state.brush_preview.clone() {
        if let Some(origin) = preview.origin {
            let mesh_id = ghost_mesh_id(state, preview.mesh_url.as_deref());
            if !state.meshes.contains_key(&mesh_id) {
                begin_world_placeholder_mesh(state, &mesh_id, WorldPlaceholderKind::Box);
            }
            let rotation = preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let scale = preview_scale(preview.scale.as_ref());
            let mesh_version = live_world3d_mesh_version(state, &mesh_id);
            if let Some(mesh) = state.meshes.get(&mesh_id).copied() {
                gpu.ensure_mesh(&mesh_id, mesh_version, mesh);
                translucent_draws.push(SceneDraw3d {
                    mesh_key: mesh_id,
                    mesh_version,
                    instances: vec![Instance3d { component_source: None,
                        id: "brush-preview".into(),
                        model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [rotation[0] as f32, rotation[1] as f32, rotation[2] as f32, rotation[3] as f32], scale),
                        color: {
                            let mut color = parse_color(preview.color.as_deref().unwrap_or("#59bfff"));
                            color[3] = 0.45;
                            color
                        },
                        selected: false,
                        hovered: false,
                     material: Default::default() }],
                    shadow_role: Default::default(),
                });
            }
        }
    }
    let mut textured_draws = Vec::new();
    if !state.meshes.contains_key("reference-plane") {
        begin_world_placeholder_mesh(state, "reference-plane", WorldPlaceholderKind::Plane);
    }
    let mut textured_instances = Vec::new();
    let reference_image_urls: Vec<String> = state
        .references
        .iter()
        .filter(|reference| !reference.hidden.unwrap_or(false))
        .filter_map(|reference| reference.url.clone())
        .filter(|url| !world_reference_raster_ready(state, url))
        .filter(|url| !state.asset_url_misses.contains(url))
        .collect();
    for url in reference_image_urls {
        let _ = reserve_world3d_asset_request(state, WorldAssetRequestKind::ReferenceImage, &url);
    }
    let visible_references: Vec<WorldReferenceRecord> = state.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)).cloned().collect();
    for reference in visible_references {
        let Some(url) = reference.url.as_deref() else { continue };
        let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let width = world_reference_width(&reference);
        let [width, height] = world_reference_plane_size(state, url, width);
        let visual = world_reference_visual(state, &reference, theme);
        textured_instances.push(TexturedInstance3d {
            texture_key: url.to_string(),
            model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [width, height, 1.0]),
            background: visual.background,
            appearance: [visual.content_opacity, 0.0, 0.0, 0.0],
        });
        if let Some(outline) = visual.outline {
            append_world_reference_outline(&mut line_vertices, origin, width, height, outline);
        }
        if let Some(lease) = reference_underlay_upload(state, url) {
            gpu.ensure_world_plane_texture(url, lease);
        }
    }
    if !textured_instances.is_empty() {
        textured_draws.push(TexturedDraw3d { instances: textured_instances });
    }
    if !state.meshes.contains_key(GUMBALL_PLANE_MESH) {
        begin_world_placeholder_mesh(state, GUMBALL_PLANE_MESH, WorldPlaceholderKind::Plane);
    }
    if !state.selected_ids.is_empty() && state.active_utility == "select" && world3d_offers_transform_gumball(state) {
        append_gumball_geometry(&mut line_vertices, &mut translucent_draws, gpu, state, &camera, &state.meshes, &state.mesh_versions);
    }
    let paint_shadow_role = world3d_shadow_role(World3dShadowGeometry::Paint, shadow.enabled);
    for draw in extra_draws.iter_mut().chain(translucent_draws.iter_mut()).filter(|draw| draw.shadow_role == SceneShadowRole3d::default()) {
        draw.shadow_role = paint_shadow_role;
    }
    if let Some(section) = section.filter(|section| section.cap.is_some()) {
        append_world3d_section_cap(state, gpu, theme, &section, &culled_draws, &mut translucent_draws);
    }
    culled_draws.extend(extra_draws);
    culled_draws.extend(terrain_draws);
    sort_world3d_translucent_material_draws(&state.meshes, view_proj, &mut material_draws);
    retain_ensured_world_draws(gpu, &mut shadow_draws);
    retain_ensured_world_draws(gpu, &mut culled_draws);
    retain_ensured_world_draws(gpu, &mut translucent_draws);
    retain_ensured_world_material_draws(gpu, &mut material_draws);
    ctx.draw.push_scene_pass(ScenePass3d {
        viewport: [inner.x, inner.y, inner.w, inner.h],
        viewport_mask: state.presentation.viewport_mask,
        render_profile: shadow_profile.render_profile(),
        clear_color: (state.presentation.viewport_mask == SceneViewportMask3d::Ellipse).then(|| clear_color.map(|color| [color.r, color.g, color.b, color.a])).flatten(),
        view_proj: view_proj.to_cols_array(),
        camera_position: camera.position.to_array(),
        light_dir,
        lighting,
        neutral_material,
        shadow,
        section_clip: section.map(|section| section.clip_plane().map(|component| component as f32)),
        procedural_grid,
        shadow_draws,
        draws: culled_draws,
        line_draws: if line_vertices.is_empty() { Vec::new() } else { vec![LineDraw3d { vertices: line_vertices }] },
        translucent_draws,
        material_draws,
        textured_draws,
        curvilinear: match state.projection_spec.mode {
            Viewport3dProjectionMode::Curvilinear { fov, strength, .. } => Some(ui_wgpu::wgpu::SceneCurvilinear3d { fov_radians: (fov as f32).min(160.0).to_radians(), strength: strength as f32 }),
            _ => None,
        },
        ..Default::default()
    });
    if let Some(world) = modelling {
        let paints_legend = world.scalar_field.is_some() && matches!(state.scalar_field_status, World3dScalarFieldStatus::Applied | World3dScalarFieldStatus::Mismatch);
        if world.annotations.is_some() || paints_legend {
            let locale = state.modelling_locale;
            ctx.draw.push_scissor(inner);
            if let Some(layer) = world.annotations.as_ref() {
                paint_world3d_annotations(ctx, layer, &view_proj.to_cols_array(), inner, locale);
            }
            if let Some(field) = world.scalar_field.as_ref() {
                match state.scalar_field_status {
                    World3dScalarFieldStatus::Applied => paint_world3d_legend(ctx, field, inner, locale),
                    World3dScalarFieldStatus::Mismatch => paint_world3d_scalar_mismatch(ctx, inner, locale),
                    World3dScalarFieldStatus::None | World3dScalarFieldStatus::Missing => {}
                }
            }
            ctx.draw.pop_scissor();
        }
    }
    if state.presentation.interactive {
        if let Some(points) = world_marquee_overlay_points(state) {
        let lasso = state.selection_method == "lasso";
        let crossing = marquee_is_crossing_from_path(&points, lasso);
        paint_selection_marquee(ctx.draw, theme, crossing, lasso, &points, true);
        }
    }
    if state.presentation.interactive && state.presentation.show_gizmo {
        gpu_gizmo::paint_orbit_view_gizmo(ctx, &camera, inner, state.gizmo_hovered_tip);
    }
    for (index, status) in state.prepared_status.iter().flatten().enumerate() {
        if let Some(text) = status.text() {
            let y = inner.y + 20.0 + index as f32 * (theme.font_size_small + 6.0);
            draw_text(ctx, text, inner.x + 12.0, y, theme.font_size_small, theme.text);
        }
    }
    if let Some(status) = state.prepared_status[0] {
        let total = f64::from(status.total).max(1.0);
        let ratio = (status.progress[2].min(total) / total) as f32;
        let tone = if status.state == 6 { theme.text } else { theme.text_element };
        ctx.draw.push_solid([inner.x + 12.0, inner.y + 54.0, 160.0 * ratio, 3.0], tone);
    }
    if scene.world_3d.is_none() {
        draw_text(ctx, "world-3d (empty)", inner.x + 12.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
    }
    if world3d_cursor_work_pending(state) {
        gpu.request_cursor_wake();
    }
    if state.presentation.interactive {
        ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(state.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None });
    }
}
