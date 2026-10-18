//! 🎨️ Node visual paint of the DAG host: painting one node with its body, label, handles, status border and chrome. See [`super`].

use super::*;

/// 🎚️ Track, knob AND value readout live on the HTML `GraphSliderOverlays` control,
/// which renders all three off ONE published row (`slider_overlay_state_json`). A
/// readout painted here was a SECOND producer of the same number: it could only
/// change when a GPU frame was drawn, while `aria-valuenow` moved with the DOM
/// control, so a released knob read `10` to a screen reader and `0.0` on the canvas
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️slider-reevaluation-correctness-2026-09-15.md`).
#[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
pub(super) fn paint_node_visual(viewport_state: &DagViewportState, graph: &DagGraphState, paint: &DagPaintState, notes: &DagNotesState, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod, lod_index: usize, node: &DagNodeSpec, chrome: DagNodePaintChrome) {
    use canvas::camera::world_to_screen;
    use canvas::text::append_label;
    use canvas::FillRule;
    use canvas::{Point, Rect, Stroke};

    let theme = &viewport_state.canvas_theme;
    let label_halo = theme.label_halo;
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
    let tint = chrome.tint_highlighted();
    let fill = dag_node_paint_fill(lod, theme, chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered).map(|color| canvas_color_with_alpha(color, chrome.body_fill_alpha));
    let stroke = dag_node_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
    let label_fill = dag_node_label_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
    let internal_chrome_stroke = dag_node_internal_chrome_stroke(stroke, label_fill, chrome.is_hovered || chrome.is_selected || chrome.is_highlighted);
    let stroke_screen_px = dag_node_stroke_screen_px(chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered);
    if let Some(fill) = fill {
        scene.fill(FillRule::NonZero, *aff, fill, None, &rect);
    }
    if !chrome.is_selected {
        scene.stroke(&Stroke::new(dag_world_stroke(stroke_screen_px, cam.zoom)), *aff, stroke, None, &rect);
    }
    match chrome.eval_status {
        DagNodeEvalStatusKind::Computing => paint.paint_computing_active_border(scene, aff, &rect, cam.zoom, theme.node_stroke_computing),
        DagNodeEvalStatusKind::Queued => paint.paint_computing_stale_border(scene, aff, &rect, cam.zoom, theme.node_stroke_stale),
        DagNodeEvalStatusKind::Error => paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_error, false),
        DagNodeEvalStatusKind::Blocked => paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_blocked, true),
        DagNodeEvalStatusKind::Ok => {}
    }
    let layout_px = dag_label_layout_px();
    let paint_px = dag_label_paint_px(cam.zoom, lod_index);
    let chrome_stroke = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
    let center_screen = world_to_screen(cam, viewport, Point::new(node.x, node.y));
    let node_fill = fill.unwrap_or(theme.node_fill);
    if !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
        paint.paint_node_lod_icon(scene, lod, center_screen, node, cam.zoom, label_fill, node_fill);
    }
    let label_text = node_label_text(node, lod);
    let caption_on_overlay = node_caption_delegated_to_js_overlay(node, lod);
    if lod.node_label_is_horizontal() {
        if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
            paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
        }
    } else {
        match &node.kind {
            DagNodeKind::Computation { .. } | DagNodeKind::Cluster { .. } => {
                if lod.shows_computation_layout() {
                    let channel_row_pick = lod.uses_channel_row_pick();
                    if channel_row_pick {
                        graph.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
                    }
                    paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_chrome_stroke);
                    graph.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_chrome_stroke, stroke, label_fill, channel_row_pick);
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        paint_computation_node_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                    }
                    if matches!(node.kind, DagNodeKind::Cluster { .. }) {
                        paint_cluster_affordances(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                    }
                    if let DagNodeKind::Computation { variadic_inputs, variadic_outputs, .. } = &node.kind {
                        if cam.zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD && (*variadic_inputs || *variadic_outputs) {
                            paint_variadic_plus_controls(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                        }
                    }
                }
            }
            DagNodeKind::Slider { .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if let Some(label) = label_text.filter(|_| lod.shows_controls() && !caption_on_overlay) {
                    paint_slider_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                }
            }
            DagNodeKind::Select { options, selected, .. } => {
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (cx0, cy0, cx1, cy1) = select_control_bounds(node);
                    let control = Rect::new(cx0, cy0, cx1, cy1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                    if lod.shows_detail_text() {
                        let option = usize::try_from(*selected).ok().and_then(|index| options.get(index)).map_or("—", String::as_str);
                        let option_pos = world_to_screen(cam, viewport, Point::new((cx0 + cx1) * 0.5, (cy0 + cy1) * 0.5));
                        append_label(scene, option, option_pos, paint_px * 0.95, label_fill, label_halo);
                        let chevron = world_to_screen(cam, viewport, Point::new(cx1 - 6.0 / cam.zoom.max(0.05), (cy0 + cy1) * 0.5));
                        append_label(scene, "▾️", chevron, paint_px, label_fill, label_halo);
                    }
                }
            }
            DagNodeKind::Screen { media, .. } => {
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let inset = 8.0 / cam.zoom.max(0.05);
                    let frame = Rect::new(node.x - hw + inset, node.y - hh + hh * 0.35, node.x + hw - inset, node.y + hh - inset);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke_selection_exit, None, &frame);
                }
                if lod.shows_detail_text() {
                    if let Some(media) = media {
                        let kind_label = match media.kind {
                            DagMediaKind::Image => "image",
                            DagMediaKind::Svg => "svg",
                            DagMediaKind::Pdf => "pdf",
                            DagMediaKind::Video => "video",
                        };
                        let hint = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.1));
                        append_label(scene, kind_label, hint, paint_px * 0.85, canvas_color_with_alpha(label_fill, ui_styling::opacities::KIND_HINT_ALPHA), label_halo);
                    }
                }
            }
            DagNodeKind::Note { text, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_detail_text() || lod.shows_controls() {
                    let (x0, y0, x1, y1) = preview_content_bounds(node);
                    let font_px = paint_px * 1.05;
                    let text_x = x0 + DAG_PREVIEW_PAD;
                    let max_w = (x1 - text_x).max(1.0);
                    let display = if text.is_empty() { "…" } else { text.as_str() };
                    let shown = truncate_label_to_fit_width(display, max_w, font_px);
                    let pos = world_to_screen(cam, viewport, Point::new(text_x, (y0 + y1) * 0.5));
                    append_label(scene, &shown, pos, font_px, label_fill, label_halo);
                    if let Some(edit) = &notes.editing_note {
                        if edit.node_id == node.id && notes.caret_visible {
                            paint_note_caret_bar(scene, aff, node, edit.caret, text, font_px, label_fill, cam.zoom);
                        }
                    }
                }
            }
            DagNodeKind::Image { src, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (x0, y0, x1, y1) = preview_content_bounds(node);
                    let frame = Rect::new(x0, y0, x1, y1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &frame);
                }
                if lod.shows_detail_text() || lod.shows_controls() {
                    paint.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, theme.raster_clear);
                }
            }
            DagNodeKind::Preview { content, expanded, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_detail_text() || lod.shows_controls() {
                    paint.paint_preview_content(scene, cam, viewport, node, content, expanded, paint_px, label_fill, label_halo, theme.raster_clear);
                }
            }
            DagNodeKind::Action { label, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    let control = Rect::new(x0, y0, x1, y1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                    if lod.shows_detail_text() {
                        let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                        append_label(scene, label, pos, paint_px * 0.95, label_fill, label_halo);
                    }
                }
            }
            DagNodeKind::Export { format, .. } => {
                graph.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                if lod.shows_controls() {
                    paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                }
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_controls() {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    let control = Rect::new(x0, y0, x1, y1);
                    scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                    if lod.shows_detail_text() {
                        let export_label = format.to_uppercase();
                        let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                        append_label(scene, &export_label, pos, paint_px * 0.95, label_fill, label_halo);
                    }
                }
            }
            DagNodeKind::AppInstance { plugin_id, app_id, .. } => {
                if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                    paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
                }
                if lod.shows_detail_text() {
                    let subtitle = format!("{plugin_id}/{app_id}");
                    let subtitle_pos = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.22));
                    append_label(scene, &subtitle, subtitle_pos, paint_px * 0.85, label_fill, label_halo);
                }
            }
        }
    }
}
