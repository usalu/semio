//! 🖍️ Paint state and palette of the DAG host: the paint state methods, the canvas palette setter and the label overlay rows of a node spec. See [`super`].

use super::*;

impl DagPaintState {
    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    pub(super) fn paint_preview_image_content(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, src: &str, label_fill: canvas::Color, bg: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::Point;
        if src.is_empty() {
            let (x0, y0, x1, y1) = preview_content_bounds(node);
            let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
            append_label(scene, "Image…", pos, DAG_LABEL_COMPACT_SCREEN_PX, label_fill, bg);
            return;
        }
        let (x0, y0, x1, y1) = preview_content_bounds(node);
        let center = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
        let w = (x1 - x0).max(1.0);
        let h = (y1 - y0).max(1.0);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, src, graph::IconScreenRect { center, width: w, height: h }, label_fill, bg, true);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    pub(super) fn paint_preview_content(
        &self,
        scene: &mut canvas::Scene,
        cam: &canvas::camera::Camera,
        viewport: &canvas::camera::Viewport,
        node: &DagNodeSpec,
        content: &DagPreviewContent,
        expanded: &DagExpandedPaths,
        paint_px: f64,
        label_fill: canvas::Color,
        label_halo: canvas::Color,
        bg: canvas::Color,
    ) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::Point;
        match content {
            DagPreviewContent::Empty => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, "—", pos, paint_px, label_fill, label_halo);
            }
            DagPreviewContent::Scalar { text } => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let display = if text.is_empty() { "—" } else { text.as_str() };
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, display, pos, paint_px * 1.05, label_fill, label_halo);
            }
            DagPreviewContent::Image { src } => {
                self.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, bg);
            }
            DagPreviewContent::Tree { json } => {
                let (x0, y0, x1, _) = preview_content_bounds(node);
                let rows = preview_tree_rows(json, expanded, "", 0);
                let row_font_px = paint_px * 0.9;
                for (index, row) in rows.iter().enumerate() {
                    let row_y = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT + DAG_PREVIEW_ROW_HEIGHT * 0.5;
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    if row.has_children {
                        let glyph = if row.expanded { "▾️" } else { "▸️" };
                        let toggle_pos = world_to_screen(cam, viewport, Point::new(x0 + indent + DAG_PREVIEW_TOGGLE_WIDTH * 0.5, row_y));
                        append_label(scene, glyph, toggle_pos, row_font_px, label_fill, label_halo);
                    }
                    let text_x = x0 + indent + if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 } + 2.0;
                    let line = if row.has_children && row.expanded { row.label.clone() } else { format!("{}: {}", row.label, row.summary) };
                    let max_w = (x1 - text_x).max(1.0);
                    let shown = truncate_label_to_fit_width(&line, max_w, row_font_px);
                    let text_pos = world_to_screen(cam, viewport, Point::new(text_x, row_y));
                    append_label(scene, &shown, text_pos, row_font_px, label_fill, label_halo);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    pub(super) fn paint_node_lod_icon(&self, scene: &mut canvas::Scene, lod: DagDrawLod, center_screen: canvas::Point, node: &DagNodeSpec, zoom: f64, fg: canvas::Color, bg: canvas::Color) {
        if !should_paint_node_lod_icon(node, lod) {
            return;
        }
        let icon = node.icon.trim();
        if icon.is_empty() {
            return;
        }
        let screen_w = node.width * zoom.max(0.05);
        let screen_h = node.height * zoom.max(0.05);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, icon, graph::IconScreenRect { center: center_screen, width: screen_w, height: screen_h }, fg, bg, false);
    }

    pub(super) fn paint_computing_active_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_active_anim_phase.get(), false);
    }

    pub(super) fn paint_computing_stale_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_stale_anim_phase.get(), true);
    }
}

pub fn dag_set_canvas_palette(viewport: &mut DagViewportState, paint: &mut DagPaintState, overlay:&BoardPaletteOverlay) {
    viewport.canvas_theme.apply_overlay(overlay);
    paint.icon_paint_cache.clear();
}

pub fn label_overlay_rows_for_node_spec(viewport: &DagViewportState, graph: &DagGraphState, paint: &DagPaintState, node: &DagNodeSpec, ghost: bool) -> Vec<Value> {
    let lod = draw_lod_for_frame(viewport, graph);
    let zoom = graph.host_snapshot.camera.zoom;
    let lod_index = dag_lod_index(zoom);
    let engine_nid = graph.node_id_for_widget_id(&node.id);
    label_overlay_rows_for_node(node, lod, zoom, lod_index, ghost, engine_nid, &paint.unresolved_input_ports)
}
