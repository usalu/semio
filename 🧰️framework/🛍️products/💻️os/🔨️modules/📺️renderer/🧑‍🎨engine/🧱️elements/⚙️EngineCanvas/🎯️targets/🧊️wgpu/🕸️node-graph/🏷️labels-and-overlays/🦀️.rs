//! 🏷️ Labels and overlays: the label chrome, the legible font clamp, the overlay rows, the node graph labels, the overlay snapshot and the selection marquee and bounds. See [`super`].

use super::*;

//#region NodeGraph
const DAG_LABEL_SCREEN_PX: f32 = 11.0;

const LABEL_INSET: f32 = 0.88;

struct LabelInteractionChrome {
    selected_ids: HashSet<String>,
    highlighted_ids: HashSet<String>,
    pub(in super::super) hovered_id: Option<String>,
    dimmed_ids: Vec<String>,
}

fn label_chrome_from_flow(host: &FlowHost) -> LabelInteractionChrome {
    let selected=host.selected_widget_ids();
    let preselect: Value = serde_json::from_str(&host.preselect_widget_ids_json()).unwrap_or(json!({}));
    let pre_ids: Vec<String> = preselect.get("ids").and_then(|v| v.as_array()).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let removed: Vec<String> = preselect.get("removedIds").and_then(|v| v.as_array()).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default();
    let (selected_ids, highlighted_ids) = if pre_ids.is_empty() && removed.is_empty() { (selected.into_iter().collect(), HashSet::new()) } else { (pre_ids.into_iter().collect(), removed.into_iter().collect()) };
    LabelInteractionChrome { selected_ids, highlighted_ids, hovered_id: host.hovered_widget_id(), dimmed_ids: host.preview_off_widget_ids() }
}

fn label_chrome_from_graph(host: &GraphHost) -> LabelInteractionChrome {
    let selected = host.dag.graph.selected_node_ids();
    let pre_ids = host.dag.graph.preselect_widget_ids();
    let removed = host.dag.graph.preselect_removed_widget_ids();
    let (selected_ids, highlighted_ids) = if pre_ids.is_empty() && removed.is_empty() { (selected.into_iter().collect(), HashSet::new()) } else { (pre_ids.into_iter().collect(), removed.into_iter().collect()) };
    LabelInteractionChrome { selected_ids, highlighted_ids, hovered_id: hovered_node_id(&host.dag.viewport, &host.dag.graph), dimmed_ids: Vec::new() }
}

/// 🔠️ Smallest font an overlay label is allowed to shrink to. Below this a caption is a smudge, not
/// a word — which is why width is answered by `canvas::text::ellipsize_by_measure` and never by
/// shrinking further.
const LABEL_LEGIBLE_MIN_PX: f32 = ui_styling::metrics::label::LEGIBLE_MIN_PX as f32;

/// 📐️ Overlay label font size: the row's own target, shrunk ONLY to fit the row's height, never
/// below {@link LABEL_LEGIBLE_MIN_PX}. Width is not a font decision — see
/// {@link fit_overlay_label_text}. This used to binary-search the font down to 4 px on width too,
/// which is how a node title wider than its body became an unreadable smear.
fn clamp_label_font_px(atlas: &mut FontAtlas, text: &str, target_px: f32, max_h: f32) -> f32 {
    let px = target_px.max(LABEL_LEGIBLE_MIN_PX).round();
    if atlas.measure_text(text, px).1 * 1.2 <= max_h {
        return px;
    }
    let mut low = LABEL_LEGIBLE_MIN_PX;
    let mut high = px;
    let mut best = LABEL_LEGIBLE_MIN_PX;
    while low <= high {
        let mid = ((low + high) * 0.5).floor();
        if atlas.measure_text(text, mid).1 * 1.2 <= max_h {
            best = mid;
            low = mid + 1.0;
        } else {
            high = mid - 1.0;
        }
    }
    best
}

/// ✂️ The wgpu shell's half of the shared caption law: clip by the GLYPH ATLAS's own measure. Its
/// browser twin is `dagEllipsizeOverlayLabel` in `🕸️NodeGraph/🟦️.tsx`, and both are pinned to the
/// rows of `🧰️framework/🔨️modules/🖼️canvas/🧫️fixtures/🏷️label-fit/🔣️.json`.
fn fit_overlay_label_text(atlas: &mut FontAtlas, text: &str, font_px: f32, max_w: f32) -> String {
    canvas::text::ellipsize_by_measure(text, f64::from(max_w), |candidate| f64::from(atlas.measure_text(candidate, font_px).0))
}

fn label_overlay_fill(theme: &Theme, node_id: &str, ghost: bool, chrome: &LabelInteractionChrome) -> Rgba {
    if ghost {
        return theme.text_muted;
    }
    if chrome.dimmed_ids.iter().any(|id| id == node_id) {
        return theme.text_muted.with_alpha(0.5);
    }
    if chrome.selected_ids.contains(node_id) {
        return theme.active_foreground;
    }
    if chrome.highlighted_ids.contains(node_id) {
        return theme.text_muted;
    }
    if chrome.hovered_id.as_deref() == Some(node_id) {
        return theme.active_foreground;
    }
    theme.text_element
}

/// 📐️ The host publishes the caption's own screen budget (`maxScreenW`) because only it knows
/// whether the caption sits INSIDE the node body or above it; `nodeW` is the fallback.
fn paint_label_overlay_row(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, cam_x: f64, cam_y: f64, zoom: f64, row: &Value, chrome: &LabelInteractionChrome) {
    let Some(text) = row.get("text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()) else {
        return;
    };
    let wx = row.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let wy = row.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let node_w = row.get("nodeW").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let node_h = row.get("nodeH").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let align = row.get("align").and_then(|v| v.as_str());
    let ghost = row.get("ghost").and_then(|v| v.as_bool()).unwrap_or(false);
    let node_id = row.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let is_port = row.get("kind").and_then(|v| v.as_str()) == Some("port") || matches!(align, Some("left") | Some("right"));
    let zoom_f = zoom.max(0.05) as f32;
    let max_w = row.get("maxScreenW").and_then(|v| v.as_f64()).filter(|w| *w > 0.0).unwrap_or_else(|| (node_w * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0)) as f32;
    let max_h = if is_port {
        row.get("maxScreenH").and_then(|v| v.as_f64()).filter(|h| *h > 0.0).map(|h| h as f32).unwrap_or((node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32)
    } else {
        (node_h * f64::from(zoom_f) * f64::from(LABEL_INSET)).max(4.0) as f32
    };
    let target_px = row.get("fontScreenPx").and_then(|v| v.as_f64()).filter(|px| *px > 0.0).map(|px| px as f32).unwrap_or(DAG_LABEL_SCREEN_PX);
    let font_px = clamp_label_font_px(&mut ctx.atlas, text, target_px, max_h);
    let fitted = fit_overlay_label_text(&mut ctx.atlas, text, font_px, max_w);
    let text = fitted.as_str();
    let (anchor_x, anchor_y) = world_to_screen_inner(inner, cam_x, cam_y, zoom, wx, wy);
    let (text_w, text_h) = ctx.atlas.measure_text(text, font_px);
    let tx = match align {
        Some("left") => anchor_x,
        Some("right") => anchor_x - text_w,
        _ => anchor_x - text_w * 0.5,
    };
    let ty = anchor_y + text_h * 0.5;
    let fill = label_overlay_fill(ctx.theme, node_id, ghost, chrome);
    let alpha = if ghost {
        0.85
    } else if chrome.dimmed_ids.iter().any(|id| id == node_id) {
        0.5
    } else {
        1.0
    };
    draw_text_overlay(ctx, text, tx, ty, font_px, fill.with_alpha(fill.a * alpha));
}

pub fn paint_node_graph_labels(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, inner: Rect) {
    let snapshot = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(&scene.host_id)?;
        match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => {
                let state_json = host.label_overlay_paint_state_json().ok()?;
                Some((state_json, label_chrome_from_flow(host)))
            }
            Some(NodeGraphEngine::Dag(host)) => {
                let state_json = host.label_overlay_paint_state_json().ok()?;
                Some((state_json, label_chrome_from_graph(host)))
            }
            None => None,
        }
    });
    let Some((state_json, chrome)) = snapshot else {
        return;
    };
    let Ok(state) = serde_json::from_str::<Value>(&state_json) else {
        return;
    };
    let cam = state.get("camera").cloned().unwrap_or(json!({}));
    let cam_x = cam.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let cam_y = cam.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let zoom = cam.get("zoom").and_then(|v| v.as_f64()).unwrap_or(1.0);
    let labels = state.get("labels").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    for row in &labels {
        paint_label_overlay_row(ctx, inner, cam_x, cam_y, zoom, row, &chrome);
    }
}

struct NodeGraphOverlaySnapshot {
    preview_points_json: String,
    preview_crossing: bool,
    preview_method: String,
    selection_bounds_json: String,
}

fn node_graph_overlay_snapshot(surface_id: &str) -> Option<NodeGraphOverlaySnapshot> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let entry = map.get(surface_id)?;
        match entry.node_graph.as_ref() {
            Some(NodeGraphEngine::Flow(host)) => Some(NodeGraphOverlaySnapshot {
                preview_points_json: host.selection_preview_points_json(),
                preview_crossing: host.selection_preview_crossing(),
                preview_method: host.selection_preview_method().to_string(),
                selection_bounds_json: host.selection_union_bounds_screen_json(),
            }),
            Some(NodeGraphEngine::Dag(host)) => Some(NodeGraphOverlaySnapshot {
                preview_points_json: host.dag.graph.selection_preview_points_json(),
                preview_crossing: host.dag.graph.selection_preview_crossing(),
                preview_method: host.dag.graph.selection_preview_method().to_string(),
                selection_bounds_json: selection_union_bounds_screen_json(&host.dag.viewport, &host.dag.graph),
            }),
            None => None,
        }
    })
}

fn parse_selection_preview_points(json: &str) -> Vec<(f32, f32)> {
    serde_json::from_str::<Vec<[f64; 2]>>(json).unwrap_or_default().into_iter().map(|point| (point[0] as f32, point[1] as f32)).collect()
}

fn paint_node_graph_selection_marquee(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, points: &[(f32, f32)], crossing: bool, method: &str, theme: &Theme) {
    if points.len() < 2 {
        return;
    }
    let lasso = method == "lasso";
    let global: Vec<[f32; 2]> = points.iter().map(|(x, y)| [inner.x + x, inner.y + y]).collect();
    semio_framework_ui_draw_contract::paint_selection_marquee(&mut ctx.draw, theme, crossing, lasso, &global, true);
}

/// 🖌️🧭️ The two paint-2d overlays React layers OVER its canvas: a live marquee
/// (`SelectionMarquee`, `🖌️Paint2dHost/🟦️.tsx:178`) and the navigator's "you are here" viewport
/// rectangle (`overlayRect`, `:275`). Both are chrome — neither belongs in the vello raster the
/// composite phase pushes — so they are painted here, in the scene's own overlay phase.
pub fn paint_paint2d_overlays(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, inner: Rect) {
    let Some(paint) = scene.paint_2d.as_ref() else {
        return;
    };
    let theme = *ctx.theme;
    if let Some(rect) = paint2d_navigator_overlay_rect(scene) {
        let bounds = [inner.x + rect[0], inner.y + rect[1], rect[2], rect[3]];
        ctx.draw.push_solid(bounds, theme.accent.with_alpha(0.12));
        ctx.draw.push_line(bounds[0], bounds[1], bounds[0] + bounds[2], bounds[1], theme.accent, theme.stroke_hairline);
        ctx.draw.push_line(bounds[0], bounds[1] + bounds[3], bounds[0] + bounds[2], bounds[1] + bounds[3], theme.accent, theme.stroke_hairline);
        ctx.draw.push_line(bounds[0], bounds[1], bounds[0], bounds[1] + bounds[3], theme.accent, theme.stroke_hairline);
        ctx.draw.push_line(bounds[0] + bounds[2], bounds[1], bounds[0] + bounds[2], bounds[1] + bounds[3], theme.accent, theme.stroke_hairline);
        return;
    }
    if let Some((points, crossing, lasso)) = paint2d_marquee_overlay(&scene.host_id, &paint.active_utility) {
        semio_framework_ui_draw_contract::paint_selection_marquee(ctx.draw, &theme, crossing, lasso, &points, true);
    }
}

fn paint_node_graph_selection_bounds(ctx: &mut FrameworkWidgetContext<'_>, inner: Rect, bounds_json: &str, theme: &Theme) {
    if bounds_json.trim() == "null" {
        return;
    }
    let Ok(value) = serde_json::from_str::<Value>(bounds_json) else {
        return;
    };
    let x = value.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let y = value.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let w = value.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let h = value.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let rx = inner.x + x;
    let ry = inner.y + y;
    let stroke = theme.text_element.with_alpha(0.95);
    ctx.draw.push_line_overlay(rx, ry, rx + w, ry, stroke, 1.0);
    ctx.draw.push_line_overlay(rx + w, ry, rx + w, ry + h, stroke, 1.0);
    ctx.draw.push_line_overlay(rx + w, ry + h, rx, ry + h, stroke, 1.0);
    ctx.draw.push_line_overlay(rx, ry + h, rx, ry, stroke, 1.0);
}

pub fn paint_node_graph_overlays(ctx: &mut FrameworkWidgetContext<'_>, scene: &UiComponentSceneNode, inner: Rect) {
    let Some(snapshot) = node_graph_overlay_snapshot(&scene.host_id) else {
        return;
    };
    let points = parse_selection_preview_points(&snapshot.preview_points_json);
    paint_node_graph_selection_marquee(ctx, inner, &points, snapshot.preview_crossing, &snapshot.preview_method, ctx.theme);
    paint_node_graph_selection_bounds(ctx, inner, &snapshot.selection_bounds_json, ctx.theme);
}
//#endregion NodeGraph
