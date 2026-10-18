//! 🖍️ Labels and paint helpers of the DAG model: handle caps, port and node label rows, the variadic plus controls, the node name paint, the channel borders, the cluster affordances, the computing border and the note caret. See [`super`].

use super::*;

pub fn handle_cap_peak(center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::Point {
    match shape {
        PortShape::Semicircle => handle_exterior_cap_peak(center, outward, radius),
        PortShape::Triangle => handle_exterior_cap_triangle_peak(center, outward, radius),
    }
}

pub fn handle_cap_fill_path(center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::BezPath {
    match shape {
        PortShape::Semicircle => handle_exterior_cap_fill_path(center, outward, radius),
        PortShape::Triangle => handle_exterior_cap_triangle_fill_path(center, outward, radius),
    }
}

pub fn handle_cap_stroke_path(center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::BezPath {
    match shape {
        PortShape::Semicircle => handle_exterior_cap_stroke_path(center, outward, radius),
        PortShape::Triangle => handle_exterior_cap_triangle_stroke_path(center, outward, radius),
    }
}

/// 🏷️ `above_body` is the computation/slider layout, where the caption is drawn ABOVE the
/// node rather than inside it (`computation_name_world_center`) — nothing there is clipped
/// by the body's own width, and nodes are laid out far enough apart to overhang it. Its
/// budget is therefore `NODE_TITLE_WIDTH_FACTOR` node widths, while a caption INSIDE the
/// body (the compact tier) and a rotated one keep the extent they actually sit in.
pub fn label_overlay_rows_for_node(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize, ghost: bool, engine_nid: Option<NodeId>, unresolved_input_ports: &NumericSet<(NodeId,usize)>) -> Vec<Value> {
    let paint_px = dag_label_paint_px(zoom, lod_index);
    let mut labels = Vec::new();
    if let Some(text) = node_label_text(node, lod).map(str::to_string) {
        let above_body = !lod.node_label_is_horizontal()
            && ((uses_computation_layout(&node.kind) && lod.shows_computation_layout()) || (matches!(node.kind, DagNodeKind::Slider { .. }) && lod.shows_controls()));
        let (layout, x, y) = if lod.node_label_is_horizontal() {
            ("horizontal", node.x, node.y)
        } else if above_body {
            let (lx, ly) = computation_name_world_center(node, &text, paint_px, zoom);
            ("horizontal", lx, ly)
        } else {
            let (lx, ly) = io_widget_label_center(node);
            ("vertical", lx, ly)
        };
        let world_budget = if above_body {
            node.width * ui_styling::metrics::dag::NODE_TITLE_WIDTH_FACTOR
        } else if layout == "vertical" {
            node.height
        } else {
            node.width
        };
        labels.push(semio_framework_pack_json::object([
            ("id".to_string(), Value::from(node.id.clone())),
            ("text".to_string(), Value::from(text)),
            ("layout".to_string(), Value::from(layout)),
            ("x".to_string(), Value::from(x)),
            ("y".to_string(), Value::from(y)),
            ("nodeW".to_string(), Value::from(node.width)),
            ("nodeH".to_string(), Value::from(node.height)),
            ("maxScreenW".to_string(), Value::from(world_budget * zoom.max(0.05) * DAG_LABEL_SCREEN_INSET)),
            ("fontScreenPx".to_string(), Value::from(paint_px)),
            ("ghost".to_string(), Value::from(ghost)),
        ]));
    }
    if lod.shows_port_labels() && !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
        for mut row in port_label_overlay_rows(node, lod, zoom, lod_index, engine_nid, unresolved_input_ports) {
            if let Some(obj) = row.as_object_mut() {
                obj.insert("ghost", Value::Bool(ghost));
            }
            labels.push(row);
        }
    }
    labels
}

fn port_label_overlay_rows(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize, engine_nid: Option<NodeId>, unresolved_input_ports: &NumericSet<(NodeId,usize)>) -> Vec<Value> {
    use canvas::text::label_extent;
    let hw = node.width * 0.5;
    let handle_inset = 8.0 / zoom.max(0.05);
    let inputs = node.inputs();
    let outputs = node.outputs();
    let computation = uses_computation_layout(&node.kind);
    let port_layout_px = if computation { dag_label_compact_paint_px(zoom, lod_index) } else { dag_label_paint_px(zoom, lod_index) };
    let mut rows = Vec::new();
    let input_column_w = if computation { io_port_column_width(inputs, port_layout_px) } else { (hw - handle_inset).max(8.0) };
    let output_column_w = if computation { io_port_column_width(outputs, port_layout_px) } else { (hw - handle_inset).max(8.0) };
    let input_port_label = |index:usize,port: &IoPortSpec| -> String {
        let mut port = port.clone();
        if let Some(nid) = engine_nid {
            if unresolved_input_ports.contains(&(nid,index)) {
                port.resolved = Some(false);
            }
        }
        port.label_with_cardinality(lod)
    };
    for (i, port) in inputs.iter().enumerate() {
        if port.shape == PortShape::Triangle || !port.visible {
            continue;
        }
        let label = input_port_label(i,port);
        if label.trim().is_empty() {
            continue;
        }
        let world_y = port_center_y(node, i, inputs.len());
        let world_x = if computation { computation_input_label_x(node) } else { node.x - hw + handle_inset };
        rows.push(semio_framework_pack_json::object([
            ("id".to_string(), Value::from(node.id.clone())),
            ("kind".to_string(), Value::from("port")),
            ("text".to_string(), Value::from(label)),
            ("layout".to_string(), Value::from("horizontal")),
            ("align".to_string(), Value::from("left")),
            ("x".to_string(), Value::from(world_x)),
            ("y".to_string(), Value::from(world_y)),
            ("nodeW".to_string(), Value::from(input_column_w)),
            ("nodeH".to_string(), Value::from(DAG_CHANNEL_ROW_HEIGHT)),
            ("maxScreenW".to_string(), Value::from(input_column_w * zoom.max(0.05) * DAG_LABEL_SCREEN_INSET)),
            ("fontScreenPx".to_string(), Value::from(port_layout_px)),
            ("maxScreenH".to_string(), Value::from(port_layout_px * 1.3)),
        ]));
    }
    for (i, port) in outputs.iter().enumerate() {
        if port.shape == PortShape::Triangle || !port.visible {
            continue;
        }
        let label = port.label_with_cardinality(lod);
        if label.trim().is_empty() {
            continue;
        }
        let world_y = port_center_y(node, i, outputs.len());
        let (world_x, column_w) = if computation {
            let left = computation_output_label_x(node, &label, port_layout_px);
            (left + port_label_text_width(&label, port_layout_px), output_column_w)
        } else {
            let (label_w, _) = label_extent(&label, port_layout_px);
            (node.x + hw - handle_inset, label_w / zoom.max(0.05))
        };
        rows.push(semio_framework_pack_json::object([
            ("id".to_string(), Value::from(node.id.clone())),
            ("kind".to_string(), Value::from("port")),
            ("text".to_string(), Value::from(label)),
            ("layout".to_string(), Value::from("horizontal")),
            ("align".to_string(), Value::from("right")),
            ("x".to_string(), Value::from(world_x)),
            ("y".to_string(), Value::from(world_y)),
            ("nodeW".to_string(), Value::from(column_w)),
            ("nodeH".to_string(), Value::from(DAG_CHANNEL_ROW_HEIGHT)),
            ("maxScreenW".to_string(), Value::from(column_w * zoom.max(0.05) * DAG_LABEL_SCREEN_INSET)),
            ("fontScreenPx".to_string(), Value::from(port_layout_px)),
            ("maxScreenH".to_string(), Value::from(port_layout_px * 1.3)),
        ]));
    }
    rows
}

pub fn paint_variadic_plus_controls(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, px: f64, fill: canvas::Color, halo: canvas::Color) {
    use canvas::camera::world_to_screen;
    use canvas::text::append_label;
    for (_, px_world, py_world) in variadic_input_insert_positions(node) {
        let screen = world_to_screen(cam, viewport, canvas::Point::new(px_world, py_world));
        append_label(scene, "+", screen, px * 0.95, fill, halo);
    }
    for (_, px_world, py_world) in variadic_output_insert_positions(node) {
        let screen = world_to_screen(cam, viewport, canvas::Point::new(px_world, py_world));
        append_label(scene, "+", screen, px * 0.95, fill, halo);
    }
}

fn paint_node_name_vertical(scene: &mut canvas::Scene, center_screen: canvas::Point, name: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
    use canvas::text::{append_label, label_extent};
    use canvas::{Affine, Point};
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return;
    }
    let (w, h) = label_extent(trimmed, px);
    let mut label_scene = canvas::Scene::new();
    append_label(&mut label_scene, trimmed, Point::new(0.0, 0.0), px, label_fill, label_halo);
    let rot = Affine::IDENTITY.translate((center_screen.x, center_screen.y)) * Affine::IDENTITY.rotate(-std::f64::consts::FRAC_PI_2) * Affine::IDENTITY.translate((-w * 0.5, -h * 0.5));
    scene.append(&label_scene, Some(rot));
}

pub fn paint_computation_column_divider(scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: canvas::Color) {
    use canvas::{Line, Point, Stroke};
    let Some(divider_x) = computation_column_divider_x(node) else {
        return;
    };
    let hh = node.height * 0.5;
    let top = node.y - hh;
    let bottom = node.y + hh;
    let stroke_style = Stroke::new(chrome_stroke);
    scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(divider_x, top), Point::new(divider_x, bottom)));
}

pub fn paint_io_widget_channel_borders(scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, px: f64, chrome_stroke: f64, stroke: canvas::Color) {
    use canvas::{Line, Point, Stroke};
    let (name_left, top, name_right, bottom) = io_widget_name_column_bounds(node, px);
    let stroke_style = Stroke::new(chrome_stroke);
    scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_left, top), Point::new(name_left, bottom)));
    scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_right, top), Point::new(name_right, bottom)));
}

pub fn node_label_text(node: &DagNodeSpec, lod: DagDrawLod) -> Option<&str> {
    if matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
        return None;
    }
    let text = match lod.node_label() {
        DagNodeLabel::None => return None,
        DagNodeLabel::Name => node.name.trim(),
    };
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

pub fn node_caption_delegated_to_js_overlay(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
    node_label_text(node, lod).is_some()
}

pub fn should_paint_node_lod_icon(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
    if !lod.node_icon_visible() {
        return false;
    }
    !uses_computation_layout(&node.kind) || !lod.shows_computation_layout()
}

pub fn paint_cluster_affordances(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, paint_px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
    use canvas::camera::world_to_screen;
    use canvas::text::append_label;
    use canvas::Point;
    let (name_x, name_y) = computation_name_world_center(node, &node.name, paint_px, cam.zoom);
    let glyph_pos = world_to_screen(cam, viewport, Point::new(name_x - paint_px * 0.55, name_y));
    append_label(scene, "🧩️", glyph_pos, paint_px * 0.85, label_fill, label_halo);
    if let Some((x0, y0, x1, y1)) = cluster_explode_hit_rect(node) {
        let cx = (x0 + x1) * 0.5;
        let cy = (y0 + y1) * 0.5;
        let explode_pos = world_to_screen(cam, viewport, Point::new(cx, cy));
        append_label(scene, "⤢", explode_pos, paint_px * 0.75, label_fill, label_halo);
    }
}

#[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
pub fn paint_computation_node_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
    use canvas::camera::world_to_screen;
    use canvas::Point;
    let (label_x, label_y) = computation_name_world_center(node, label, px, cam.zoom);
    let anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
    paint_node_name_horizontal(scene, anchor, label, px, label_fill, label_halo);
}

#[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
pub fn paint_slider_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
    paint_computation_node_name(scene, cam, viewport, node, label, px, label_fill, label_halo);
}

#[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
pub fn paint_io_widget_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, lod: DagDrawLod, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
    use canvas::camera::world_to_screen;
    use canvas::Point;
    if lod.node_label() == DagNodeLabel::None && !lod.shows_controls() {
        return;
    }
    let (label_x, label_y) = io_widget_label_center(node);
    let name_anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
    paint_node_name_vertical(scene, name_anchor, label, px, label_fill, label_halo);
}

pub fn paint_node_name_horizontal(scene: &mut canvas::Scene, center_screen: canvas::Point, name: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
    use canvas::text::{append_label, label_extent};
    use canvas::Point;
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return;
    }
    let (w, h) = label_extent(trimmed, px);
    append_label(scene, trimmed, Point::new(center_screen.x - w * 0.5, center_screen.y - h * 0.5), px, label_fill, label_halo);
}

#[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
pub fn paint_computing_border_arc(scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color, start_t: f64, dashed: bool) {
    use canvas::{BezPath, Stroke};
    const SEGMENTS: usize = 40;
    const ARC_FRACTION: f64 = 0.24;
    let mut path = BezPath::new();
    for i in 0..=SEGMENTS {
        let local = i as f64 / SEGMENTS as f64;
        let t = (start_t + local * ARC_FRACTION).fract();
        let p = rect_perimeter_point(rect, t);
        if i == 0 {
            path.move_to(p);
        } else {
            path.line_to(p);
        }
    }
    let stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX * 1.75, cam_zoom);
    let mut stroke = Stroke::new(stroke_px);
    if dashed {
        stroke.set_dash_pattern(vec![stroke_px * 2.5, stroke_px * 2.0]);
    }
    scene.stroke(&stroke, *aff, color, None, &path);
}

pub fn paint_eval_status_border(scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color, dashed: bool) {
    paint_computing_border_arc(scene, aff, rect, cam_zoom, color, 0.0, dashed);
}

fn rect_perimeter_point(rect: &canvas::Rect, t: f64) -> canvas::Point {
    use canvas::Point;
    let t = t.fract();
    let w = rect.width();
    let h = rect.height();
    let perim = 2.0 * (w + h);
    let mut d = t * perim;
    if d <= w {
        return Point::new(rect.x0() + d, rect.y0());
    }
    d -= w;
    if d <= h {
        return Point::new(rect.x1(), rect.y0() + d);
    }
    d -= h;
    if d <= w {
        return Point::new(rect.x1() - d, rect.y1());
    }
    d -= w;
    Point::new(rect.x0(), rect.y1() - d)
}

#[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
pub fn paint_note_caret_bar(scene: &mut canvas::Scene, aff: &canvas::Affine, node: &DagNodeSpec, caret_byte: usize, text: &str, font_px: f64, fill: canvas::Color, zoom: f64) {
    use canvas::text::label_byte_world_x;
    use canvas::FillRule;
    use canvas::Rect;
    let (x0, y0, _x1, y1) = preview_content_bounds(node);
    let origin_x = x0 + DAG_PREVIEW_PAD;
    let caret_x = label_byte_world_x(text, caret_byte.min(text.len()), origin_x, font_px);
    let caret_y = (y0 + y1) * 0.5;
    let lh = font_px * 1.2;
    let bar_w = 1.5 / zoom.max(0.05);
    let rect = Rect::new(caret_x, caret_y - lh * 0.4, caret_x + bar_w, caret_y + lh * 0.4);
    scene.fill(FillRule::NonZero, *aff, fill, None, &rect);
}
