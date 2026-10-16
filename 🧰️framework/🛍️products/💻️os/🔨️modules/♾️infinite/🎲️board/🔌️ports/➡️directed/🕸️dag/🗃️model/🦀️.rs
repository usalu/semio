//! 🗃️ Dag board model: geometry, LOD, preview records, errors, edits and the host snapshot loaders.
//!
//! Crate `semio-framework-os-infinite-board-dag-model`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{DagLayoutOptions, field, layered_positions, LayoutControl, refusal, Selection};
use semio_framework_os_infinite_board_engine::{CanvasPalette, DirectedPortGraphEngine, GraphExtension};
use semio_framework_canvas::{Affine, Color, FillRule, Rect, Scene, Stroke};
use semio_framework_value::{DslValue, Number};
use std::collections::{HashMap, HashSet};
use semio_framework_canvas::lod::{Lod, LodScale};
use graph::{NodeId, WorldBox};
use semio_framework_value::numeric_scratch::{NumericIndex, NumericSet};
use geometry::Point;
use semio_framework_value::retirement::{deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence_birth_bytes};
use semio_framework_pack_json::Value;
use ::{graph, semio_framework_canvas as canvas};
use semio_framework_graph_drawing::routing::{handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, rectangle_handle_angle_toward};
use semio_framework_os_infinite_directed_presentation::*;
use semio_framework_value::retirement::*;

pub struct DagInputDisplaced{pub selection:Option<(Selection,Selection,Selection)>,pub chrome:Option<(Option<NodeId>,NumericSet<NodeId>,NumericIndex<NodeId,DagNodeEvalStatusKind>,NumericSet<(NodeId,usize)>)>,pub progress:Option<(Option<NodeId>,NumericSet<NodeId>)>}

impl DagInputDisplaced{pub fn empty()->Self{Self{selection:None,chrome:None,progress:None}}}

pub struct Identity<'a>{pub first:&'a str,pub second:Option<&'a str>}

impl Identity<'_>{pub fn len(&self)->usize{self.first.len()+self.second.map_or(0,|text|text.len()+1)}pub fn byte(&self,index:usize)->Option<u8>{if index<self.first.len(){self.first.as_bytes().get(index).copied()}else if index==self.first.len()&&self.second.is_some(){Some(b'@')}else{self.second?.as_bytes().get(index-self.first.len()-1).copied()}}}

pub enum IdentityStep{Progress,Resolved(u64),Missing}

semio_framework_value::artifact_retire_struct!(DagInputDisplaced{selection,chrome,progress});

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

pub fn dag_stroke_world_step_grid(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, color: canvas::Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
    use canvas::camera::world_to_screen;
    use canvas::{Affine, Point, Stroke};
    let step = world_step * cam.zoom;
    if step < min_step_screen {
        return;
    }
    let stroke = Stroke::new(stroke_px);
    let w = viewport.width as f64;
    let h = viewport.height as f64;
    let origin = world_to_screen(cam, viewport, Point::new(0.0, 0.0));
    let x_off = ((origin.x % step) + step) % step;
    let y_off = ((origin.y % step) + step) % step;
    let mut path = canvas::BezPath::new();
    let mut x = x_off;
    while x <= w {
        path.move_to(Point::new(x, 0.0));
        path.line_to(Point::new(x, h));
        x += step;
    }
    let mut y = y_off;
    while y <= h {
        path.move_to(Point::new(0.0, y));
        path.line_to(Point::new(w, y));
        y += step;
    }
    scene.stroke(&stroke, Affine::IDENTITY, color, None, &path);
}

pub fn parse_snapshot_edge_numeric_id(id: &str) -> Option<u64> {
    id.strip_prefix('e').and_then(|s| s.parse().ok())
}

pub fn dag_port_endpoint_parts(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), String::new()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn dag_port_handle_key(node_id: &str, port_id: &str, input: bool) -> String {
    format!("{}:{}:{}", node_id, if input { "in" } else { "out" }, port_id)
}

pub fn toggle_preview_tree_path(node: &mut DagNodeSpec, path: &str) {
    let DagNodeKind::Preview { content, expanded, .. } = &mut node.kind else {
        return;
    };
    if !matches!(content, DagPreviewContent::Tree { .. }) {
        return;
    }
    if expanded.contains(path) {
        expanded.remove(path);
    } else {
        expanded.insert(path.to_string());
    }
    fit_node_size(node);
}

/// 🌳️ DAG board engine alias.
pub type DagBoardEngine = DirectedPortGraphEngine;

//#region ⚠️ Errors
/// 🚨️ Crate-local error for DAG snapshot parsing, layout, and host-state mutation.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum DagError {
    SnapshotRootNotObject,
    SchemaMismatch,
    NodesMissing,
    UnknownAlignMode(String),
    UnknownWidget(String),
    CanvasTheme(String),
    GridFactorOutOfRange,
    /// 🌉️ Holds the formatted message rather than a codec error type directly — `pack::json`'s
    /// `JsonError` (syntax) and the value derive's `ValueError` (shape) both fold into this,
    /// matching the Display text `serde_json::Error` used to produce.
    Json(String),
}

impl std::fmt::Display for DagError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SnapshotRootNotObject => formatter.write_str("snapshot root must be object"),
            Self::SchemaMismatch => formatter.write_str("schema must be dag.hostDocument"),
            Self::NodesMissing => formatter.write_str("nodes array missing"),
            Self::CanvasTheme(message) => formatter.write_str(message),
            Self::UnknownAlignMode(mode) => write!(formatter, "unknown align mode: {mode}"),
            Self::UnknownWidget(widget) => write!(formatter, "unknown widget: {widget}"),
            Self::GridFactorOutOfRange => formatter.write_str("gridFactor must be finite and in (0, 1e6]"),
            Self::Json(error) => formatter.write_str(error),
        }
    }
}

impl std::error::Error for DagError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<semio_framework_pack_json::JsonError> for DagError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<semio_framework_value::ValueError> for DagError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Json(error.to_string())
    }
}
//#endregion ⚠️ Errors

// #region 🔖️PortSide
/// ↔ Which side of a computation node a variadic insert targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagPortSide {
    Input,
    Output,
}

impl DagPortSide {
    /// 🔤️ The side as the `insertPort` row names it: `input` or `output`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
        }
    }
}
// #endregion 🔖️PortSide

/// 📛️ Reserved title row above computation IO channels.
pub const DAG_COMPUTATION_HEADER_ROWS: usize = 0;

const DAG_NODE_EDGE_INSET: f64 = ui_styling::metrics::dag::NODE_EDGE_INSET;

pub const DAG_IO_COLUMN_WIDTH: f64 = ui_styling::metrics::dag::IO_COLUMN_WIDTH;

pub const DAG_COMPONENT_WIDTH: f64 = DAG_IO_COLUMN_WIDTH * 2.0;

pub const DAG_DISTRIBUTE_MIN_GAP: f64 = ui_styling::metrics::board::LAYOUT_SIBLING_GAP;

const DAG_IO_WIDGET_HEIGHT: f64 = ui_styling::metrics::dag::IO_WIDGET_HEIGHT;

pub const DAG_LABEL_SCREEN_PX: f64 = ui_styling::metrics::label::DAG_DEFAULT_PX;

pub const DAG_LABEL_COMPACT_SCREEN_PX: f64 = ui_styling::metrics::label::DAG_COMPACT_PX;

/// 🔢 How large a slider's value readout is against the board's own label size — the ratio the GPU
/// readout carried before it moved onto the HTML overlay beside the knob it belongs to.
pub const DAG_SLIDER_VALUE_LABEL_RATIO: f64 = 0.9;

pub enum ComputationChannelRowSide {
    Input,
    Output,
}

/// 🔢️ Row count for a computation node body from its IO and variadic flags.
pub fn computation_io_row_count(input_count: usize, output_count: usize, variadic_inputs: bool, variadic_outputs: bool) -> usize {
    let input_rows = input_count + usize::from(variadic_inputs);
    let output_rows = output_count + usize::from(variadic_outputs);
    input_rows.max(output_rows).max(1)
}

/// 📐️ Computation node height from channel row count.
pub fn computation_node_height(input_count: usize, output_count: usize, variadic_inputs: bool, variadic_outputs: bool) -> f64 {
    (computation_io_row_count(input_count, output_count, variadic_inputs, variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS) as f64 * DAG_CHANNEL_ROW_HEIGHT
}

/// 🔤️ Splits a port's declared, comma-joined value schemas into the list the board intersects.
pub fn port_value_types(port: &IoPortSpec) -> Vec<String> {
    port.value_type.as_deref().map(|declared| declared.split(',').filter(|entry| !entry.is_empty()).map(str::to_string).collect()).unwrap_or_default()
}

pub fn io_port_column_width(ports: &[IoPortSpec], _px: f64) -> f64 {
    if ports.is_empty() {
        0.0
    } else {
        DAG_IO_COLUMN_WIDTH
    }
}

/// 📐️ Uniform computation component width shared by every flow component node.
pub fn computation_node_width(_name: &str, _inputs: &[IoPortSpec], _outputs: &[IoPortSpec]) -> f64 {
    DAG_COMPONENT_WIDTH
}

/// 📐️ IO widget width aligned with all flow components.
pub fn io_widget_width(_name: &str) -> f64 {
    DAG_COMPONENT_WIDTH
}

/// 📐️ IO widget height from vertically rotated title metrics plus a control band.
pub fn io_widget_height(name: &str) -> f64 {
    use canvas::text::label_extent;
    let name_px = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_SCALE_MULT;
    let (label_w, _) = label_extent(name, name_px);
    (label_w + DAG_IO_WIDGET_HEIGHT + DAG_NODE_EDGE_INSET * 2.0).max(40.0)
}

/// 📐️ Slider track width aligned with computation components.
pub fn slider_widget_width(_name: &str, _output: &IoPortSpec) -> f64 {
    DAG_COMPONENT_WIDTH
}

/// 📐️ Slider track height — one computation channel row.
pub fn slider_widget_height() -> f64 {
    DAG_CHANNEL_ROW_HEIGHT
}

fn io_widget_label_center(node: &DagNodeSpec) -> (f64, f64) {
    let hh = node.height * 0.5;
    (node.x, node.y - hh * 0.2)
}

fn channel_row_center_y(node_y: f64, node_height: f64, row_index: usize) -> f64 {
    let hh = node_height * 0.5;
    node_y - hh + (row_index as f64 + 0.5) * DAG_CHANNEL_ROW_HEIGHT
}

pub fn channel_row_bounds(node: &DagNodeSpec, row_index: usize) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let y_center = channel_row_center_y(node.y, node.height, row_index);
    let half = DAG_CHANNEL_ROW_HEIGHT * 0.5;
    (node.x - hw, y_center - half, node.x + hw, y_center + half)
}

pub fn input_port_row_hit_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
    let inputs = node.inputs();
    if port_index >= inputs.len() {
        return None;
    }
    let (x0, x1) = if uses_computation_layout(&node.kind) {
        computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Input)
    } else {
        let hw = node.width * 0.5;
        (node.x - hw, node.x)
    };
    let count = inputs.len().max(1);
    let y_center = port_center_y(node, port_index, count);
    let half = if uses_computation_layout(&node.kind) { DAG_CHANNEL_ROW_HEIGHT * 0.5 } else { node.height / count as f64 * 0.5 };
    Some((x0, y_center - half, x1, y_center + half))
}

pub fn output_port_row_hit_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
    let outputs = node.outputs();
    if port_index >= outputs.len() {
        return None;
    }
    let (x0, x1) = if uses_computation_layout(&node.kind) {
        computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output)
    } else {
        let hw = node.width * 0.5;
        (node.x, node.x + hw)
    };
    let count = outputs.len().max(1);
    let y_center = port_center_y(node, port_index, count);
    let half = if uses_computation_layout(&node.kind) { DAG_CHANNEL_ROW_HEIGHT * 0.5 } else { node.height / count as f64 * 0.5 };
    Some((x0, y_center - half, x1, y_center + half))
}

/// 🔌️ Share of a port ROW, from the node edge inward, that is the port's CONNECTOR: the zone a press
/// grabs a wire in, and the rect [`DagHost::entity_screen_json`] publishes as where that port is. One
/// geometry for both — publishing the whole row while grabbing a 6.5-world-unit disc at its edge made
/// the published centre unpressable at every zoom (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️generate-mode-interactions-2026-09-13.md` §4). The remainder of the row stays node body: a
/// computation node has NO header row ([`DAG_COMPUTATION_HEADER_ROWS`] is 0, its name is painted above
/// the rectangle), so without that remainder it would have nowhere left to be dragged by.
const DAG_PORT_CONNECTOR_ROW_SHARE: f64 = 0.4;

/// 🔌️ An input port's connector rect: the outer [`DAG_PORT_CONNECTOR_ROW_SHARE`] of its row, grown
/// outward past the node edge by the radius of the handle cap the engine paints there — so the zone
/// covers exactly what a user sees of the port, inside chrome and outside cap alike.
pub fn input_port_connector_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(node, port_index)?;
    Some((x0 - DAG_HANDLE_WORLD_RADIUS, y0, (x0 + (x1 - x0) * DAG_PORT_CONNECTOR_ROW_SHARE).min(x1), y1))
}

/// 🔌️ An output port's connector rect — the same share measured inward from the node's right edge.
pub fn output_port_connector_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
    let (x0, y0, x1, y1) = output_port_row_hit_bounds(node, port_index)?;
    Some(((x1 - (x1 - x0) * DAG_PORT_CONNECTOR_ROW_SHARE).max(x0), y0, x1 + DAG_HANDLE_WORLD_RADIUS, y1))
}

fn computation_port_center_y(node: &DagNodeSpec, port_index: usize) -> f64 {
    channel_row_center_y(node.y, node.height, port_index + DAG_COMPUTATION_HEADER_ROWS)
}

fn proportional_port_center_y(node: &DagNodeSpec, port_index: usize, count: usize) -> f64 {
    let hh = node.height * 0.5;
    let t = (port_index as f64 + 0.5) / count.max(1) as f64;
    node.y - hh + t * node.height
}

fn port_center_y(node: &DagNodeSpec, port_index: usize, count: usize) -> f64 {
    if uses_computation_layout(&node.kind) {
        computation_port_center_y(node, port_index)
    } else {
        proportional_port_center_y(node, port_index, count)
    }
}

// #region 🔖️PreviewContent
pub const DAG_PREVIEW_PAD: f64 = 4.0;

pub const DAG_PREVIEW_ROW_HEIGHT: f64 = 14.0;

pub const DAG_PREVIEW_TREE_INDENT: f64 = 12.0;

pub const DAG_PREVIEW_TOGGLE_WIDTH: f64 = 10.0;

const DAG_PREVIEW_MAX_IMAGE: f64 = 200.0;

const DAG_PREVIEW_MIN_SIZE: f64 = 20.0;

#[derive(Clone, Debug)]
struct PreviewTreeRow {
    pub path: String,
    pub depth: usize,
    pub label: String,
    pub summary: String,
    pub has_children: bool,
    pub expanded: bool,
}

#[derive(Clone, Debug)]
struct PreviewTreeRowLayout {
    pub path: String,
    pub row_rect: (f64, f64, f64, f64),
}

pub enum WidgetPointerKind {
    SliderDrag,
    SelectClick,
    PreviewToggle(String),
    ClusterExplode,
    ExportClick,
}

fn preview_scalar_text_width(text: &str) -> f64 {
    let px = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_SCALE_MULT;
    port_label_text_width(text, px).max(text.len() as f64 * px * 0.55)
}

fn preview_media_natural_size(src: &str) -> (f64, f64) {
    use canvas::icon_codec::{resolve_icon_kind, ResolvedIcon};
    match resolve_icon_kind(src, |_| None) {
        ResolvedIcon::RasterRgba8 { w, h, .. } => (f64::from(w), f64::from(h)),
        ResolvedIcon::SvgPlain(s) | ResolvedIcon::SvgThemed(s) => {
            if let Ok((_, _, bw, bh)) = canvas::svg_icon::svg_icon_content_bounds_from_str(&s) {
                if bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite() {
                    return (bw, bh);
                }
            }
            (64.0, 48.0)
        }
        ResolvedIcon::None => (64.0, 48.0),
    }
}

fn clamp_preview_image_size(w: f64, h: f64) -> (f64, f64) {
    let max = DAG_PREVIEW_MAX_IMAGE;
    if w <= max && h <= max {
        return (w.max(1.0), h.max(1.0));
    }
    let scale = (max / w).min(max / h);
    ((w * scale).max(1.0), (h * scale).max(1.0))
}

fn preview_tree_collapsed_summary(value: &DslValue) -> String {
    match value {
        DslValue::Object(entries) => format!("{{{} keys}}", entries.len()),
        DslValue::Array(arr) => format!("[{} items]", arr.len()),
        DslValue::String(s) => format!("\"{s}\""),
        DslValue::Bytes(bytes) => format!("<{} bytes>", bytes.len()),
        DslValue::Number(n) => match n {
            semio_framework_value::Number::UInt(v) => v.to_string(),
            semio_framework_value::Number::Int(v) => v.to_string(),
            semio_framework_value::Number::Float(v) => v.to_string(),
        },
        DslValue::Bool(b) => b.to_string(),
        DslValue::Null => "null".into(),
    }
}

fn preview_tree_scalar_display(value: &DslValue) -> String {
    match value {
        DslValue::String(s) => format!("\"{s}\""),
        other => preview_tree_collapsed_summary(other),
    }
}

pub fn truncate_label_to_fit_width(text: &str, max_width: f64, px: f64) -> String {
    use canvas::text::label_extent;
    let trimmed = text.trim();
    if trimmed.is_empty() || max_width <= 0.0 || px < 4.0 {
        return text.to_string();
    }
    let (full_w, _) = label_extent(trimmed, px);
    if full_w <= max_width {
        return text.to_string();
    }
    let ellipsis = "…";
    let mut best = ellipsis.to_string();
    for (byte_idx, ch) in trimmed.char_indices() {
        let end = byte_idx + ch.len_utf8();
        let candidate = format!("{}{ellipsis}", &trimmed[..end]);
        let (w, _) = label_extent(&candidate, px);
        if w <= max_width {
            best = candidate;
        } else {
            break;
        }
    }
    best
}

pub fn note_text_origin_x(node: &DagNodeSpec) -> f64 {
    let (x0, _, _, _) = preview_content_bounds(node);
    x0 + DAG_PREVIEW_PAD
}

pub fn hit_byte_in_note_line(line: &str, world_x: f64, line_origin_x: f64, font_px: f64) -> usize {
    use canvas::text::label_byte_world_x;
    if line.is_empty() {
        return 0;
    }
    let mut boundaries = vec![0usize];
    for (index, _) in line.char_indices() {
        if index > 0 {
            boundaries.push(index);
        }
    }
    if boundaries.last().copied() != Some(line.len()) {
        boundaries.push(line.len());
    }
    for pair in boundaries.windows(2) {
        let start = pair[0];
        let end = pair[1];
        let x0 = label_byte_world_x(line, start, line_origin_x, font_px);
        let x1 = label_byte_world_x(line, end, line_origin_x, font_px);
        if world_x < (x0 + x1) * 0.5 {
            return start;
        }
    }
    line.len()
}

pub fn preview_tree_rows(json: &DslValue, expanded: &DagExpandedPaths, path: &str, depth: usize) -> Vec<PreviewTreeRow> {
    let mut rows = Vec::new();
    match json {
        DslValue::Object(entries) => {
            for (key, val) in entries {
                let row_path = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                let has_children = matches!(val, DslValue::Object(_) | DslValue::Array(_));
                let is_expanded = expanded.contains(&row_path);
                let summary = if has_children { preview_tree_collapsed_summary(val) } else { preview_tree_scalar_display(val) };
                rows.push(PreviewTreeRow { path: row_path.clone(), depth, label: key.clone(), summary, has_children, expanded: is_expanded });
                if has_children && is_expanded {
                    rows.extend(preview_tree_rows(val, expanded, &row_path, depth + 1));
                }
            }
        }
        DslValue::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let key = format!("[{i}]");
                let row_path = if path.is_empty() { key.clone() } else { format!("{path}{key}") };
                let has_children = matches!(val, DslValue::Object(_) | DslValue::Array(_));
                let is_expanded = expanded.contains(&row_path);
                let summary = if has_children { preview_tree_collapsed_summary(val) } else { preview_tree_scalar_display(val) };
                rows.push(PreviewTreeRow { path: row_path.clone(), depth, label: key, summary, has_children, expanded: is_expanded });
                if has_children && is_expanded {
                    rows.extend(preview_tree_rows(val, expanded, &row_path, depth + 1));
                }
            }
        }
        _ => {}
    }
    rows
}

/// 📐️ Measures preview content size in world units.
pub fn measure_preview_content(content: &DagPreviewContent, expanded: &DagExpandedPaths) -> (f64, f64) {
    match content {
        DagPreviewContent::Empty => (DAG_PREVIEW_MIN_SIZE, DAG_PREVIEW_MIN_SIZE),
        DagPreviewContent::Scalar { text } => {
            let tw = preview_scalar_text_width(text);
            (tw.max(DAG_PREVIEW_MIN_SIZE), DAG_PREVIEW_ROW_HEIGHT.max(DAG_PREVIEW_MIN_SIZE))
        }
        DagPreviewContent::Image { src } => clamp_preview_image_size(preview_media_natural_size(src).0, preview_media_natural_size(src).1),
        DagPreviewContent::Tree { json } => {
            let rows = preview_tree_rows(json, expanded, "", 0);
            if rows.is_empty() {
                return (DAG_PREVIEW_MIN_SIZE, DAG_PREVIEW_MIN_SIZE);
            }
            let max_w = rows
                .iter()
                .map(|row| {
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    let toggle = if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 };
                    let line = if row.has_children && row.expanded { row.label.clone() } else { format!("{}: {}", row.label, row.summary) };
                    indent + toggle + port_label_text_width(&line, DAG_LABEL_COMPACT_SCREEN_PX)
                })
                .fold(DAG_PREVIEW_MIN_SIZE, f64::max);
            (max_w, rows.len() as f64 * DAG_PREVIEW_ROW_HEIGHT)
        }
    }
}

fn preview_content_node_size(content: &DagPreviewContent, expanded: &DagExpandedPaths) -> (f64, f64) {
    let (_, ch) = measure_preview_content(content, expanded);
    (DAG_COMPONENT_WIDTH, ch + DAG_PREVIEW_PAD * 2.0)
}

fn preview_image_node_size(src: &str) -> (f64, f64) {
    let inner_w = (DAG_COMPONENT_WIDTH - DAG_PREVIEW_PAD * 2.0).max(1.0);
    let (nw, nh) = preview_media_natural_size(src);
    let (cw, ch) = clamp_preview_image_size(nw, nh);
    let aspect = if cw > 0.0 { ch / cw } else { 1.0 };
    let inner_h = (inner_w * aspect).max(DAG_PREVIEW_MIN_SIZE);
    (DAG_COMPONENT_WIDTH, inner_h + DAG_PREVIEW_PAD * 2.0)
}

/// 📐️ Image input node size from media source.
pub fn image_widget_size(src: &str) -> (f64, f64) {
    preview_image_node_size(src)
}

/// 📐️ Preview node size from typed content and fold state.
pub fn preview_widget_size(content: &DagPreviewContent, expanded: &DagExpandedPaths) -> (f64, f64) {
    preview_content_node_size(content, expanded)
}

pub fn preview_tree_row_layouts(node: &DagNodeSpec, json: &DslValue, expanded: &DagExpandedPaths) -> Vec<PreviewTreeRowLayout> {
    let (x0, y0, x1, _y1) = preview_content_bounds(node);
    preview_tree_rows(json, expanded, "", 0)
        .into_iter()
        .enumerate()
        .map(|(index, row)| {
            let row_y0 = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT;
            let row_y1 = row_y0 + DAG_PREVIEW_ROW_HEIGHT;
            let row_rect = if row.has_children { (x0, row_y0, x0 + (x1 - x0).max(1.0), row_y1) } else { (0.0, 0.0, 0.0, 0.0) };
            PreviewTreeRowLayout { path: row.path, row_rect }
        })
        .collect()
}
// #endregion 🔖️PreviewContent

fn uses_computation_layout(kind: &DagNodeKind) -> bool {
    matches!(
        kind,
        DagNodeKind::Computation { .. }
            | DagNodeKind::Cluster { .. }
            | DagNodeKind::Slider { .. }
            | DagNodeKind::Note { .. }
            | DagNodeKind::Image { .. }
            | DagNodeKind::Preview { .. }
            | DagNodeKind::Action { .. }
            | DagNodeKind::Export { .. }
    )
}

pub const DAG_CLUSTER_EXPLODE_HIT_SIZE: f64 = 14.0;

/// 💥️ World-space hit rect for the cluster explode affordance.
pub fn cluster_explode_hit_rect(node: &DagNodeSpec) -> Option<(f64, f64, f64, f64)> {
    if !matches!(node.kind, DagNodeKind::Cluster { .. }) {
        return None;
    }
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let size = DAG_CLUSTER_EXPLODE_HIT_SIZE;
    let x1 = node.x + hw - DAG_NODE_EDGE_INSET;
    let y0 = node.y - hh + DAG_NODE_EDGE_INSET;
    Some((x1 - size, y0, x1, y0 + size))
}

/// 💥️ Whether a world point hits the cluster explode affordance.
pub fn cluster_explode_hit(node: &DagNodeSpec, world_x: f64, world_y: f64) -> bool {
    cluster_explode_hit_rect(node).is_some_and(|(x0, y0, x1, y1)| point_in_rect(world_x, world_y, x0, y0, x1, y1))
}

pub fn point_in_rect(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
    px >= x0.min(x1) && px <= x0.max(x1) && py >= y0.min(y1) && py <= y0.max(y1)
}

/// 📍️ World-space center of the draggable slider track.
pub fn slider_track_center(node: &DagNodeSpec) -> Option<(f64, f64)> {
    match &node.kind {
        DagNodeKind::Slider { .. } => {
            let (x0, y0, x1, y1) = slider_track_bounds(node);
            Some(((x0 + x1) * 0.5, (y0 + y1) * 0.5))
        }
        _ => None,
    }
}

/// 📐️ Horizontal span of the side that carries ports. A one-sided widget keeps its track in that half.
fn occupied_half_x(node: &DagNodeSpec) -> (f64, f64) {
    let hw = node.width * 0.5;
    let left = node.x - hw;
    let right = node.x + hw;
    let Some(divider) = computation_column_divider_x(node) else {
        return (left, right);
    };
    let has_inputs = !node.inputs().is_empty();
    let has_outputs = !node.outputs().is_empty();
    if has_outputs && !has_inputs {
        (divider, right)
    } else if has_inputs && !has_outputs {
        (left, divider)
    } else {
        (left, right)
    }
}

pub fn slider_track_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let (left, right) = occupied_half_x(node);
    let pad = DAG_NODE_EDGE_INSET;
    let track_y = node.y;
    let hit_h = 8.0;
    (left + pad, track_y - hit_h * 0.5, right - pad, track_y + hit_h * 0.5)
}

pub fn select_control_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    (node.x - hw + DAG_NODE_EDGE_INSET, node.y + hh * 0.12, node.x + hw - 10.0, node.y + hh - DAG_NODE_EDGE_INSET)
}

/// 📐️ Note node size from its text payload.
pub fn note_widget_size(_text: &str) -> (f64, f64) {
    (DAG_COMPONENT_WIDTH, DAG_CHANNEL_ROW_HEIGHT)
}

pub fn preview_content_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_PREVIEW_PAD;
    let (left, right) = if matches!(node.kind, DagNodeKind::Preview { .. }) { occupied_half_x(node) } else { (node.x - hw, node.x + hw) };
    (left + pad, node.y - hh + pad, right - pad, node.y + hh - pad)
}

pub fn action_control_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_NODE_EDGE_INSET;
    (node.x - hw + pad, node.y + hh * 0.08, node.x + hw - pad, node.y + hh - pad)
}

pub fn set_slider_value_from_x(node: &mut DagNodeSpec, world_x: f64) -> Option<f64> {
    let (left, _, right, _) = slider_track_bounds(node);
    let DagNodeKind::Slider { min, max, step, value, .. } = &mut node.kind else {
        return None;
    };
    let span = (right - left).max(1e-6);
    let t = ((world_x - left) / span).clamp(0.0, 1.0);
    let raw = *min + t * (*max - *min);
    let stepped = if *step > 0.0 { (raw / *step).round() * *step } else { raw };
    *value = stepped.clamp(*min, *max);
    Some(*value)
}

/// 📐️ Places input handles on the left and output handles on the right of a rectangle node.
pub fn io_node_handle_angles(input_index: usize, input_count: usize, output_index: usize, output_count: usize) -> (f64, f64) {
    let input_angle = port_angle_on_side(input_index, input_count.max(1), true);
    let output_angle = port_angle_on_side(output_index, output_count.max(1), false);
    (input_angle, output_angle)
}

pub const DAG_VARIADIC_PLUS_ZOOM_THRESHOLD: f64 = 1.5;

fn computation_input_label_x(node: &DagNodeSpec) -> f64 {
    node.x - node.width * 0.5 + DAG_NODE_EDGE_INSET
}

fn computation_output_label_x(node: &DagNodeSpec, label: &str, px: f64) -> f64 {
    let label_w = port_label_text_width(label, px);
    node.x + node.width * 0.5 - DAG_NODE_EDGE_INSET - label_w
}

pub fn computation_column_divider_x(node: &DagNodeSpec) -> Option<f64> {
    let inputs = node.inputs();
    let outputs = node.outputs();
    if inputs.is_empty() && outputs.is_empty() {
        return None;
    }
    if inputs.is_empty() || outputs.is_empty() {
        return Some(node.x);
    }
    let hw = node.width * 0.5;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let left_w = io_port_column_width(inputs, port_px);
    let right_w = io_port_column_width(outputs, port_px);
    let left_end = node.x - hw + DAG_NODE_EDGE_INSET + left_w;
    let right_start = node.x + hw - DAG_NODE_EDGE_INSET - right_w;
    Some(if right_start <= left_end { node.x } else { (left_end + right_start) * 0.5 })
}

pub fn computation_input_column_x_bounds(node: &DagNodeSpec) -> Option<(f64, f64)> {
    let inputs = node.inputs();
    if inputs.is_empty() && !node.variadic_inputs() {
        return None;
    }
    let hw = node.width * 0.5;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let left = node.x - hw + DAG_NODE_EDGE_INSET;
    let right = left + io_port_column_width(inputs, port_px);
    Some((left, right))
}

pub fn computation_output_column_x_bounds(node: &DagNodeSpec) -> Option<(f64, f64)> {
    let outputs = node.outputs();
    let variadic_outputs = matches!(&node.kind, DagNodeKind::Computation { variadic_outputs: true, .. });
    if outputs.is_empty() && !variadic_outputs {
        return None;
    }
    let hw = node.width * 0.5;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let right = node.x + hw - DAG_NODE_EDGE_INSET;
    let left = right - io_port_column_width(outputs, port_px);
    Some((left, right))
}

pub fn computation_io_side_row_counts(node: &DagNodeSpec) -> (usize, usize) {
    match &node.kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => {
            let input_rows = inputs.len() + usize::from(*variadic_inputs);
            let output_rows = outputs.len() + usize::from(*variadic_outputs);
            (input_rows, output_rows)
        }
        DagNodeKind::Cluster { inputs, outputs } => (inputs.len(), outputs.len()),
        DagNodeKind::AppInstance { inputs, outputs, .. } => (inputs.len(), outputs.len()),
        _ => (0, 0),
    }
}

pub fn channel_row_divider_y(node_y: f64, node_height: f64, after_row_index: usize) -> f64 {
    let hh = node_height * 0.5;
    node_y - hh + after_row_index as f64 * DAG_CHANNEL_ROW_HEIGHT
}

pub fn computation_channel_row_divider_x_span(node: &DagNodeSpec, side: ComputationChannelRowSide) -> (f64, f64) {
    let hw = node.width * 0.5;
    let left_edge = node.x - hw;
    let right_edge = node.x + hw;
    match (computation_column_divider_x(node), side) {
        (Some(divider_x), ComputationChannelRowSide::Input) => (left_edge, divider_x),
        (Some(divider_x), ComputationChannelRowSide::Output) => (divider_x, right_edge),
        (None, ComputationChannelRowSide::Input) | (None, ComputationChannelRowSide::Output) => (left_edge, right_edge),
    }
}

pub fn computation_io_side_row_divider_indices(side_rows: usize, grid_rows: usize) -> std::ops::Range<usize> {
    if side_rows == 0 || grid_rows <= 1 {
        return 0..0;
    }
    1..side_rows + usize::from(side_rows < grid_rows)
}

pub fn computation_name_world_center(node: &DagNodeSpec, label: &str, paint_px: f64, zoom: f64) -> (f64, f64) {
    use canvas::text::label_extent;
    let hh = node.height * 0.5;
    let (_, label_h) = label_extent(label, paint_px);
    let z = zoom.max(0.05);
    let screen_gap = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_COMPACT_RATIO;
    let world_offset = (screen_gap + label_h * 0.5) / z;
    (node.x, node.y - hh - world_offset)
}

fn io_widget_name_column_bounds(node: &DagNodeSpec, px: f64) -> (f64, f64, f64, f64) {
    use canvas::text::label_extent;
    let hh = node.height * 0.5;
    let name_px = px * 1.05;
    let (_name_w, name_h) = label_extent(&node.name, name_px);
    let visual_w = name_h;
    let top = node.y - hh + DAG_NODE_EDGE_INSET;
    let bottom = node.y + hh * 0.12;
    let x0 = node.x - visual_w * 0.5;
    let x1 = node.x + visual_w * 0.5;
    (x0, top, x1, bottom)
}

pub fn computation_channel_row_count(node: &DagNodeSpec) -> usize {
    match &node.kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => computation_io_row_count(inputs.len(), outputs.len(), *variadic_inputs, *variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS,
        DagNodeKind::Cluster { inputs, outputs } => computation_io_row_count(inputs.len(), outputs.len(), false, false) + DAG_COMPUTATION_HEADER_ROWS,
        _ => 0,
    }
}

pub fn fit_node_size(node: &mut DagNodeSpec) {
    match &node.kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => {
            node.width = computation_node_width(&node.name, inputs, outputs);
            node.height = computation_node_height(inputs.len(), outputs.len(), *variadic_inputs, *variadic_outputs);
        }
        DagNodeKind::Cluster { inputs, outputs } => {
            node.width = computation_node_width(&node.name, inputs, outputs);
            node.height = computation_node_height(inputs.len(), outputs.len(), false, false);
        }
        DagNodeKind::Slider { output, .. } => {
            node.width = slider_widget_width(&node.name, output);
            node.height = slider_widget_height();
        }
        DagNodeKind::Note { text, .. } => {
            let (w, h) = note_widget_size(text);
            node.width = w;
            node.height = h;
        }
        DagNodeKind::Select { .. } | DagNodeKind::Action { .. } | DagNodeKind::Export { .. } => {
            node.width = io_widget_width(&node.name);
            node.height = io_widget_height(&node.name);
        }
        DagNodeKind::Preview { content, expanded, .. } => {
            let (w, h) = preview_content_node_size(content, expanded);
            node.width = w;
            node.height = h;
        }
        DagNodeKind::Image { src, .. } => {
            let (w, h) = preview_image_node_size(src);
            node.width = w;
            node.height = h;
        }
        DagNodeKind::Screen { .. } => {}
        DagNodeKind::AppInstance { inputs, outputs, .. } => {
            let port_count = inputs.len().max(outputs.len()).max(1);
            node.width = 180.0;
            node.height = 56.0 + port_count as f64 * 18.0;
        }
    }
}

pub fn variadic_input_insert_positions(node: &DagNodeSpec) -> Vec<(usize, f64, f64)> {
    let inputs = node.inputs();
    if !node.variadic_inputs() {
        return vec![];
    }
    let row = inputs.len();
    let port_y = computation_port_center_y(node, row);
    let port_x = computation_input_label_x(node);
    vec![(inputs.len(), port_x, port_y)]
}

pub fn variadic_output_insert_positions(node: &DagNodeSpec) -> Vec<(usize, f64, f64)> {
    if !node.variadic_outputs() {
        return vec![];
    }
    let outputs = node.outputs();
    let row = outputs.len();
    let port_y = computation_port_center_y(node, row);
    let port_x = computation_output_label_x(node, "+", DAG_LABEL_COMPACT_SCREEN_PX);
    vec![(outputs.len(), port_x, port_y)]
}

fn port_angle_on_side(index: usize, count: usize, left: bool) -> f64 {
    let t = (index as f64 + 0.5) / count as f64;
    let y = (t - 0.5) * 0.8;
    if left {
        std::f64::consts::PI + y * std::f64::consts::FRAC_PI_2 * 0.9
    } else {
        y * std::f64::consts::FRAC_PI_2 * 0.9
    }
}

/// 📐️ Rectangle-layout port angle (north-zero CCW) aligned with painted IO labels.
pub fn io_node_rect_port_angle(x: f64, y: f64, width: f64, height: f64, index: usize, count: usize, left: bool) -> f64 {
    use canvas::Point;
    use graph::rectangle_handle_angle_toward;
    let hw = width * 0.5;
    let row_count = computation_io_row_count(count, count, false, false);
    let channel_rows = row_count + DAG_COMPUTATION_HEADER_ROWS;
    let port_y = if channel_rows * (DAG_CHANNEL_ROW_HEIGHT as usize) == height.round() as usize {
        channel_row_center_y(y, height, index + DAG_COMPUTATION_HEADER_ROWS)
    } else {
        let hh = height * 0.5;
        let t = (index as f64 + 0.5) / count.max(1) as f64;
        y - hh + t * height
    };
    let port_x = if left { x - hw } else { x + hw };
    rectangle_handle_angle_toward(Point::new(x, y), width, height, Point::new(port_x, port_y))
}

pub fn io_node_rect_port_angle_for_node(node: &DagNodeSpec, port_index: usize, left: bool) -> f64 {
    use canvas::Point;
    use graph::rectangle_handle_angle_toward;
    let hw = node.width * 0.5;
    let count = if left { node.inputs().len() } else { node.outputs().len() };
    let port_y = port_center_y(node, port_index, count);
    let port_x = if left { node.x - hw } else { node.x + hw };
    rectangle_handle_angle_toward(Point::new(node.x, node.y), node.width, node.height, Point::new(port_x, port_y))
}
// #endregion 🔖️IoNode

// #region 🔖️Acyclicity
/// 🚫️ Returns true when adding `source -> target` would create a cycle.
pub fn would_create_cycle(existing: &[(String, String)], source: &str, target: &str) -> bool {
    ::graph::algorithms::would_create_cycle_ids(existing, source, target)
}
// #endregion 🔖️Acyclicity

/// 🌳️ Applies original layered geometry directly to admitted DAG records.
pub fn apply_dag_layout(snapshot:&mut DagHostSnapshot,opts:&DagLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),DagError>{
 if snapshot.schema!="dag.hostDocument"{return Err(DagError::SchemaMismatch);}
 let ids:HashSet<String>=snapshot.nodes.iter().map(|n|n.id.clone()).collect();
 let mut handle_owners=HashMap::new();for node in &snapshot.nodes{for port in node.inputs().iter().chain(node.outputs().iter()){handle_owners.insert(format!("{}@{}",node.id,port.id),node.id.clone());}}
 let resolve=|key:&str|handle_owners.get(key).cloned().or_else(||ids.contains(key).then(||key.to_owned())).or_else(||key.split('@').next().filter(|base|ids.contains(*base)).map(str::to_owned)).unwrap_or_else(||key.to_owned());
 let directed:Vec<(String,String)>=snapshot.edges.iter().filter_map(|e|{let a=resolve(&e.source);let b=resolve(&e.target);(a!=b&&ids.contains(&a)&&ids.contains(&b)).then_some((a,b))}).collect();
 let positions=layered_positions(&ids,&directed,opts,control).map_err(|e|DagError::Json(e.to_string()))?;
 for node in &mut snapshot.nodes{if let Some(&(x,y))=positions.get(&node.id){node.x=x;node.y=y;}}Ok(())
}
// #endregion 🔖️Layout

// #region 🔖️GraphExtension
/// 🧩️ DAG-specific graph extension marker.
pub struct DagExtension;

impl canvas::CanvasExtension for DagExtension {
    fn extension_id(&self) -> &str {
        "dag"
    }
}

impl GraphExtension for DagExtension {}
// #endregion 🔖️GraphExtension

const DAG_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; fill only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Node icons only.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Horizontal abbreviations.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Vertical names with sections; channel abbreviations on ports.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Channel names on ports, port handles, and control text.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Full channel names on ports and maximum node fidelity.", max_zoom: f64::INFINITY },
];

const DAG_LOD_SCALE: LodScale = LodScale { lods: DAG_LODS };

/// 📶️ Uniform delay before each DAG LOD band activates (requires slightly more zoom for detail tiers).
const DAG_LOD_ZOOM_SHIFT: f64 = ui_styling::metrics::dag::LOD_ZOOM_SHIFT;

const DAG_LOD_BAND_FLOOR_ZOOM: &[f64] = ui_styling::metrics::dag::LOD_BAND_FLOOR_ZOOM;

fn dag_lod_resolve_zoom(zoom: f64) -> f64 {
    (zoom - DAG_LOD_ZOOM_SHIFT).max(0.05)
}

pub fn dag_lod_index(zoom: f64) -> usize {
    DAG_LOD_SCALE.resolve_index(dag_lod_resolve_zoom(zoom))
}

pub fn dag_lod_band_floor_zoom(lod_index: usize) -> f64 {
    let floor = canvas::lod::band_floor_zoom(DAG_LOD_BAND_FLOOR_ZOOM, lod_index, 0.05);
    if lod_index == 0 {
        floor
    } else {
        (floor + DAG_LOD_ZOOM_SHIFT).max(0.05)
    }
}

/// 🔵️ Port dot world radius; screen size grows with camera zoom like node geometry.
pub const DAG_HANDLE_WORLD_RADIUS: f64 = ui_styling::radii::DAG_HANDLE_WORLD;

/// 🏷️ Share of a caption's own slot the text may occupy, leaving a hairline of breathing room. The
/// same number both presentations used to derive privately; it is published on the row now
/// (`maxScreenW`) so the budget is decided once, by the host that knows the layout.
const DAG_LABEL_SCREEN_INSET: f64 = 0.88;

const DAG_NODE_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE;

const DAG_NODE_STROKE_SELECTED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_SELECTED;

const DAG_NODE_STROKE_HOVERED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_HOVERED;

pub const DAG_CHROME_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_CHROME;

pub const DAG_BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;

pub fn dag_world_stroke(screen_px: f64, zoom: f64) -> f64 {
    (screen_px / zoom.max(0.05)).max(1e-3)
}

pub fn dag_label_layout_px() -> f64 {
    DAG_LABEL_SCREEN_PX
}

pub fn dag_label_paint_px(zoom: f64, lod_index: usize) -> f64 {
    canvas::lod::lod_band_label_screen_px(DAG_LABEL_SCREEN_PX, zoom, dag_lod_band_floor_zoom(lod_index))
}

pub fn dag_label_compact_paint_px(zoom: f64, lod_index: usize) -> f64 {
    canvas::lod::lod_band_label_screen_px(DAG_LABEL_COMPACT_SCREEN_PX, zoom, dag_lod_band_floor_zoom(lod_index))
}

fn dag_node_body_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.node_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
    } else if selected {
        theme.node_fill_selected
    } else if highlighted {
        theme.node_fill_selection_exit
    } else if hovered {
        theme.node_fill_hovered
    } else {
        theme.node_fill
    }
}

pub fn dag_node_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.node_stroke, ui_styling::opacities::DIM_STROKE_ALPHA)
    } else if selected {
        theme.node_stroke_selected
    } else if highlighted {
        theme.node_stroke_selection_exit
    } else if hovered {
        theme.node_stroke_hovered
    } else {
        theme.node_stroke
    }
}

pub fn dag_node_label_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.label_fill, ui_styling::opacities::DIM_LABEL_ALPHA)
    } else if selected {
        theme.label_fill_hovered
    } else if highlighted {
        theme.node_stroke_selection_exit
    } else if hovered {
        theme.label_fill_hovered
    } else {
        theme.label_fill
    }
}

/// 🧱️ Internal column/row chrome inside a node body; selection/hover matches label emphasis.
pub fn dag_node_internal_chrome_stroke(body_stroke: canvas::Color, label_fill: canvas::Color, emphasized: bool) -> canvas::Color {
    if emphasized {
        label_fill
    } else {
        body_stroke
    }
}

pub fn dag_handle_body_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.handle_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
    } else if selected {
        theme.handle_fill_selected
    } else if highlighted {
        theme.handle_fill_selection_exit
    } else if hovered {
        theme.handle_fill_hovered
    } else {
        theme.handle_fill
    }
}

pub fn dag_handle_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.handle_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
    } else if selected {
        theme.handle_stroke_selected
    } else if highlighted {
        theme.handle_stroke_selection_exit
    } else if hovered {
        theme.handle_stroke_hovered
    } else {
        theme.handle_stroke
    }
}

pub fn dag_edge_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.edge_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
    } else if selected {
        theme.edge_stroke_selected
    } else if highlighted {
        theme.edge_stroke_selection_exit
    } else if hovered {
        theme.edge_stroke_hovered
    } else {
        theme.edge_stroke
    }
}

pub fn dag_node_stroke_screen_px(dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> f64 {
    if dimmed {
        1.0
    } else if selected {
        DAG_NODE_STROKE_SELECTED_SCREEN_PX
    } else if highlighted || hovered {
        DAG_NODE_STROKE_HOVERED_SCREEN_PX
    } else {
        DAG_NODE_STROKE_SCREEN_PX
    }
}

/// 🎨️ Node body fill when painted; `None` means stroke/text only (board overview+).
pub fn dag_node_paint_fill(lod: DagDrawLod, theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> Option<canvas::Color> {
    if lod == DagDrawLod::Minimap {
        return Some(dag_node_body_stroke(theme, dimmed, selected, highlighted, hovered));
    }
    let chrome = dimmed || selected || highlighted || hovered;
    if chrome {
        Some(dag_node_body_fill(theme, dimmed, selected, highlighted, hovered))
    } else {
        None
    }
}

/// 📶️ Resolves the DAG draw LOD for a camera zoom factor.
pub fn dag_draw_lod(zoom: f64) -> DagDrawLod {
    DagDrawLod::from_scale_index(dag_lod_index(zoom))
}

fn lod_max_zoom_json(max_zoom: f64) -> Value {
    if max_zoom.is_finite() {
        Value::from(max_zoom)
    } else {
        Value::from(f64::MAX)
    }
}

/// 📶️ JSON LOD table for React window chrome (`id`, `name`, `description`, `maxZoom`).
pub fn dag_lod_scale_json() -> String {
    let rows: Vec<Value> = DAG_LODS
        .iter()
        .map(|lod| {
            let max_zoom = if lod.max_zoom.is_finite() { lod.max_zoom + DAG_LOD_ZOOM_SHIFT } else { lod.max_zoom };
            semio_framework_pack_json::object([("id".to_string(), Value::from(lod.id)), ("name".to_string(), Value::from(lod.name)), ("description".to_string(), Value::from(lod.description)), ("maxZoom".to_string(), lod_max_zoom_json(max_zoom))])
        })
        .collect();
    semio_framework_pack_json::to_string(&Value::Array(rows))
}
// #endregion 🔖️Lod

// #region 🔖️NoteEdit

#[derive(Clone, Debug)]
pub struct NoteEditState {
    pub node_id: String,
    pub caret: usize,
    pub anchor: usize,
}

// #endregion 🔖️NoteEdit

// #region 🔖️DagHost

pub const DAG_INTERACTION_NODE_CAPACITY: usize = 256;

pub const DAG_INTERACTION_WORD_CAPACITY: usize = DAG_INTERACTION_NODE_CAPACITY / 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagInteractionPlanFault {
    NodeCredits,
    StringCredits,
    Unsupported,
}

#[derive(Clone, Copy, Debug)]
pub enum DagPointerPhase {
    Down,
    Move,
    Up,
    Leave,
}

#[derive(Clone, Copy, Debug)]
pub struct DagPointerIntent {
    pub phase: DagPointerPhase,
    pub x: f64,
    pub y: f64,
    pub button: u8,
    pub shift: bool,
    pub ctrl_or_meta: bool,
    pub alt: bool,
    pub pan: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct DagNodeMove {
    pub index: u16,
    pub x: f64,
    pub y: f64,
}

/// ✋️ How far a drag moved its nodes, and the press (the projection revision it began at) that names the gesture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DagDragOffset {
    pub press: u64,
    pub dx: f64,
    pub dy: f64,
}

#[derive(Clone, Copy, Debug)]
#[expect(clippy::large_enum_variant, reason = "Pointer plans copy the complete fixed-capacity gesture snapshot before atomic admission; boxing drag state would allocate and break Copy.")]
pub enum DagProjectionGesture {
    Idle,
    Pan { start_x: f64, start_y: f64, camera: [f64; 3] },
    Drag { press: u64, start_x: f64, start_y: f64, starts: [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY], len: u16 },
    Select { start_x: f64, start_y: f64, initial: [u64; DAG_INTERACTION_WORD_CAPACITY] },
}

#[derive(Clone, Copy, Debug)]
pub struct DagInteractionProjection {
    pub revision: u64,
    pub camera: [f64; 3],
    pub selected: [u64; DAG_INTERACTION_WORD_CAPACITY],
    pub hover: Option<u16>,
    pub gesture: DagProjectionGesture,
}

/// 📍️ Owned selection, hover and camera values from an admitted pointer projection.
pub struct DagPointerSnapshot {
    pub node_ids: Vec<String>,
    pub hovered_id: Option<String>,
    pub camera: [f64; 3],
}

#[derive(Clone, Copy, Debug)]
pub struct DagPointerPlan {
    pub expected_revision: u64,
    pub previous_active: bool,
    pub next: DagInteractionProjection,
    pub moves: [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY],
    pub move_len: u16,
    /// 🫃️ The offset this plan's pointer moved the drag by, from its press — `None` while it never left the drag's own
    /// origin. A press-and-release that never moved derives a ZERO-delta drag whose `moves` are the positions the nodes
    /// already hold, and journalling those would spend a guest mutation on a plain click.
    pub drag: Option<DagDragOffset>,
}

pub fn dag_bit_contains(bits: &[u64; DAG_INTERACTION_WORD_CAPACITY], index: usize) -> bool {
    bits.get(index / 64).is_some_and(|word| word & (1_u64 << (index % 64)) != 0)
}

pub fn dag_bit_set(bits: &mut [u64; DAG_INTERACTION_WORD_CAPACITY], index: usize, selected: bool) {
    if let Some(word) = bits.get_mut(index / 64) {
        let mask = 1_u64 << (index % 64);
        if selected {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }
}

impl DagInteractionProjection {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn camera(&self) -> [f64; 3] {
        self.camera
    }

    pub fn selected(&self, index: usize) -> bool {
        dag_bit_contains(&self.selected, index)
    }

    pub fn hover(&self) -> Option<usize> {
        self.hover.map(usize::from)
    }

    /// 🫳️ Whether a BOUNDED gesture — a node drag, a marquee, a pan — is in flight in this projection.
    /// A gesture belongs to the path that STARTED it: without this, a drag whose pointer crossed a port
    /// or an inline widget was handed to the screen path mid-flight, which resynced the projection and
    /// erased the drag (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn gesture_active(&self) -> bool {
        !matches!(self.gesture, DagProjectionGesture::Idle)
    }
}

impl DagPointerPlan {
    pub fn projection(&self) -> &DagInteractionProjection {
        &self.next
    }

    pub fn expected_revision(&self) -> u64 {
        self.expected_revision
    }

    pub fn gesture_active(&self) -> bool {
        !matches!(self.next.gesture, DagProjectionGesture::Idle)
    }

    pub fn previous_gesture_active(&self) -> bool {
        self.previous_active
    }

    pub fn move_len(&self) -> usize {
        usize::from(self.move_len)
    }
}

/// 🔗️ One graph edit a completed gesture performed on this host's own graph, in the guest's own `nodeGraphEdit` row
/// vocabulary (design §13.3) rather than in engine handle/edge ids: `connect`, `disconnect` by synapse id, `move` (the
/// press, the moved node ids and their ONE relative offset), `setSlider` (the value an inline slider was released on) and
/// `insertPort` (a variadic port inserted at `index`). Every renderer — React and wgpu alike — drains these after a
/// gesture and dispatches exactly them ([`dag_graph_edit_rows_json`]); no renderer ever publishes the whole snapshot.
#[derive(Clone, Debug, PartialEq)]
pub enum DagGraphEdit {
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    Disconnect { synapse_id: String },
    Move { gesture_id: String, node_ids: Vec<String>, dx: f64, dy: f64 },
    SetSlider { node_id: String, value: f64 },
    InsertPort { node_id: String, side: DagPortSide, index: usize },
}

/// 🚫️ A gesture refused whole because its rows (`rows`) outgrow one `nodeGraphEdit` dispatch (`limit`,
/// [`DAG_GRAPH_EDIT_CAPACITY`]): nothing was journalled and the embedding host restores the gesture's baseline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DagJournalRefusal {
    pub rows: usize,
    pub limit: usize,
}

/// ✍️ Where the ONE row encoder ([`write_dag_graph_edit_rows`]) writes a row: a JSON answer for React and the flow wasm
/// session ([`dag_graph_edit_rows_json`]), the bounded action builder for wgpu, a string census for credit pricing.
pub trait DagGraphEditRowSink<'a> {
    type Error;
    fn begin_row(&mut self) -> Result<(), Self::Error>;
    fn text(&mut self, field: &'static str, value: &'a str) -> Result<(), Self::Error>;
    fn number(&mut self, field: &'static str, value: f64) -> Result<(), Self::Error>;
    fn integer(&mut self, field: &'static str, value: u64) -> Result<(), Self::Error>;
    fn texts(&mut self, field: &'static str, values: &'a [String]) -> Result<(), Self::Error>;
    fn end_row(&mut self) -> Result<(), Self::Error>;
}

/// 🧾️ THE node-graph row encoder (design §13.3): writes every edit as its `nodeGraphEdit` row into `sink` — the one
/// place a row's operation, fields and their order are spelled: `connect {sourceNodeId, sourcePortId, targetNodeId,
/// targetPortId}`, `disconnect {synapseId}`, `move {gestureId, nodeIds, dx, dy}`, `setSlider {widgetId, value}`,
/// `insertPort {nodeId, side, index}`.
pub fn write_dag_graph_edit_rows<'a, S: DagGraphEditRowSink<'a>>(edits: &'a [DagGraphEdit], sink: &mut S) -> Result<(), S::Error> {
    for edit in edits {
        sink.begin_row()?;
        match edit {
            DagGraphEdit::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                sink.text("operation", "connect")?;
                sink.text("sourceNodeId", source_node_id)?;
                sink.text("sourcePortId", source_port_id)?;
                sink.text("targetNodeId", target_node_id)?;
                sink.text("targetPortId", target_port_id)?;
            }
            DagGraphEdit::Disconnect { synapse_id } => {
                sink.text("operation", "disconnect")?;
                sink.text("synapseId", synapse_id)?;
            }
            DagGraphEdit::Move { gesture_id, node_ids, dx, dy } => {
                sink.text("operation", "move")?;
                sink.text("gestureId", gesture_id)?;
                sink.texts("nodeIds", node_ids)?;
                sink.number("dx", *dx)?;
                sink.number("dy", *dy)?;
            }
            DagGraphEdit::SetSlider { node_id, value } => {
                sink.text("operation", "setSlider")?;
                sink.text("widgetId", node_id)?;
                sink.number("value", *value)?;
            }
            DagGraphEdit::InsertPort { node_id, side, index } => {
                sink.text("operation", "insertPort")?;
                sink.text("nodeId", node_id)?;
                sink.text("side", side.as_str())?;
                sink.integer("index", *index as u64)?;
            }
        }
        sink.end_row()?;
    }
    Ok(())
}

/// 🧾️ The JSON sink of [`write_dag_graph_edit_rows`].
#[derive(Default)]
struct DagGraphEditJsonRows {
    pub rows: Vec<Value>,
    pub row: Vec<(String, Value)>,
}

impl<'a> DagGraphEditRowSink<'a> for DagGraphEditJsonRows {
    type Error = std::convert::Infallible;
    fn begin_row(&mut self) -> Result<(), Self::Error> {
        self.row.clear();
        Ok(())
    }
    fn text(&mut self, field: &'static str, value: &'a str) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), Value::from(value)));
        Ok(())
    }
    fn number(&mut self, field: &'static str, value: f64) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), Value::from(value)));
        Ok(())
    }
    fn integer(&mut self, field: &'static str, value: u64) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), Value::from(value)));
        Ok(())
    }
    fn texts(&mut self, field: &'static str, values: &'a [String]) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), semio_framework_pack_json::array(values.iter().map(|value| Value::from(value.as_str())))));
        Ok(())
    }
    fn end_row(&mut self) -> Result<(), Self::Error> {
        self.rows.push(semio_framework_pack_json::object(std::mem::take(&mut self.row)));
        Ok(())
    }
}

/// 🧮️ The string census sink of [`write_dag_graph_edit_rows`]: every field name and text value a row carries.
#[derive(Default)]
struct DagGraphEditRowStrings<'a>(Vec<&'a str>);

impl<'a> DagGraphEditRowSink<'a> for DagGraphEditRowStrings<'a> {
    type Error = std::convert::Infallible;
    fn begin_row(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn text(&mut self, field: &'static str, value: &'a str) -> Result<(), Self::Error> {
        self.0.extend([field, value]);
        Ok(())
    }
    fn number(&mut self, field: &'static str, _value: f64) -> Result<(), Self::Error> {
        self.0.push(field);
        Ok(())
    }
    fn integer(&mut self, field: &'static str, _value: u64) -> Result<(), Self::Error> {
        self.0.push(field);
        Ok(())
    }
    fn texts(&mut self, field: &'static str, values: &'a [String]) -> Result<(), Self::Error> {
        self.0.push(field);
        self.0.extend(values.iter().map(String::as_str));
        Ok(())
    }
    fn end_row(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// 🧮️ Every field name and text value the rows of `edits` carry, for a bounded writer to price its string credits before
/// it builds them.
pub fn dag_graph_edit_row_strings(edits: &[DagGraphEdit]) -> Vec<&str> {
    let mut strings = DagGraphEditRowStrings::default();
    if let Err(never) = write_dag_graph_edit_rows(edits, &mut strings) {
        match never {}
    }
    strings.0
}

/// 🧾️ A gesture's journal answer as the JSON a renderer reads: the `nodeGraphEdit` arguments `{"operations":[…]}` it
/// dispatches, plus `"refused": {"rows", "limit"}` when the gesture was refused whole ([`DagJournalRefusal`]).
pub fn dag_graph_edit_rows_json(edits: &[DagGraphEdit], refused: Option<DagJournalRefusal>) -> String {
    let mut rows = DagGraphEditJsonRows::default();
    if let Err(never) = write_dag_graph_edit_rows(edits, &mut rows) {
        match never {}
    }
    let refused = refused.map(|refusal| ("refused".to_string(), semio_framework_pack_json::object([("rows".to_string(), Value::from(refusal.rows)), ("limit".to_string(), Value::from(refusal.limit))])));
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object(std::iter::once(("operations".to_string(), semio_framework_pack_json::array(rows.rows))).chain(refused)))
}

/// 🆔️ The gesture id of the drag a press began: `node-drag:<press>`, unique per host for the press's projection revision.
pub fn dag_drag_gesture_id(press: u64) -> String {
    format!("node-drag:{press}")
}

/// 📏️ How many rows one drain may carry: the rows one `nodeGraphEdit` dispatch admits (`NODE_GRAPH_EDIT_MAX_ROWS` of
/// `🛠️tool-machine`). A press journals at most one wire (and the edge it replaced), one slider value and one port; only the
/// moves of a release or an align grow with the graph (one row per distinct offset), so those are reserved whole or the
/// gesture is refused ([`DagJournalRefusal`]) — the journal never drops a row.
pub const DAG_GRAPH_EDIT_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagNodeEvalStatusKind {
    Ok,
    Queued,
    Computing,
    Error,
    Blocked,
}

pub struct MinimapWidgetLayout {
    pub panel: (f64, f64, f64, f64),
    pub world_min_x: f64,
    pub world_min_y: f64,
    pub scale: f64,
    pub map_origin_x: f64,
    pub map_origin_y: f64,
    pub viewport: (f64, f64, f64, f64),
}

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; pointer-event timestamps are a
// browser canvas concept the wasip2 guest has no pointer events to timestamp in the first place,
// so it shares the existing native no-op (`0.0`) rather than reading a WASI clock for no consumer.
pub fn pointer_event_now_ms() -> f64 {
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now()
    }
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        0.0
    }
}

pub fn canvas_color_with_alpha(color: canvas::Color, alpha: u8) -> canvas::Color {
    use canvas::Color;
    let rgba = color.to_rgba8();
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

#[derive(Clone, Copy)]
pub struct DagNodePaintChrome {
    pub is_dimmed: bool,
    pub is_selected: bool,
    pub is_highlighted: bool,
    pub is_hovered: bool,
    pub eval_status: DagNodeEvalStatusKind,
    pub body_fill_alpha: u8,
    pub ghost_tint: bool,
}

impl DagNodePaintChrome {
    pub fn ghost_preview() -> Self {
        Self { is_dimmed: false, is_selected: false, is_highlighted: false, is_hovered: false, eval_status: DagNodeEvalStatusKind::Ok, body_fill_alpha: 255, ghost_tint: true }
    }

    pub fn tint_highlighted(self) -> bool {
        self.is_highlighted || self.ghost_tint
    }

    pub fn has_interaction_chrome(self) -> bool {
        self.is_dimmed || self.is_selected || self.is_highlighted || self.is_hovered
    }
}

/// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::screen_to_world` — see `minimap_camera_fully_shows_content` above.
pub fn minimap_widget_screen_to_world(layout: &MinimapWidgetLayout, sx: f64, sy: f64) -> (f64, f64) {
    ui_wgpu::wgpu::minimap::screen_to_world(layout.map_origin_x, layout.map_origin_y, layout.world_min_x, layout.world_min_y, layout.scale, sx, sy)
}

/// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::point_in_rect` — see `minimap_camera_fully_shows_content` above.
pub fn minimap_widget_point_in_rect(rect: (f64, f64, f64, f64), sx: f64, sy: f64) -> bool {
    ui_wgpu::wgpu::minimap::point_in_rect(rect, sx, sy)
}

// #region 🔖️SelectionAlign
pub fn dag_node_world_bounds(node: &DagNodeSpec) -> WorldBox {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    WorldBox { min_x: node.x - hw, min_y: node.y - hh, max_x: node.x + hw, max_y: node.y + hh }
}

semio_framework_value::artifact_retire_leaf!(DagPortSide);

semio_framework_value::artifact_retire_leaf!(DagJournalRefusal);

semio_framework_value::artifact_retire_leaf!(DagNodeEvalStatusKind);

semio_framework_value::artifact_retire_leaf!(DagInteractionProjection);

semio_framework_value::artifact_retire_struct!(NoteEditState {node_id,caret,anchor});

impl RetireOwned for DagGraphEdit {
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::Connect{source_node_id,source_port_id,target_node_id,target_port_id}=>(source_node_id,source_port_id,target_node_id,target_port_id).retirement(),Self::Disconnect{synapse_id}=>synapse_id.retirement(),Self::Move{gesture_id,node_ids,dx,dy}=>(gesture_id,node_ids,dx,dy).retirement(),Self::SetSlider{node_id,value}=>(node_id,value).retirement(),Self::InsertPort{node_id,side,index}=>(node_id,side,index).retirement()}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::Connect{source_node_id,source_port_id,target_node_id,target_port_id}=>sequence_birth_bytes(&[deferred_birth_bytes_for(source_node_id),deferred_birth_bytes_for(source_port_id),deferred_birth_bytes_for(target_node_id),deferred_birth_bytes_for(target_port_id)]),Self::Disconnect{synapse_id}=>synapse_id.retirement_birth_bytes(),Self::Move{gesture_id,node_ids,dx,dy}=>sequence_birth_bytes(&[deferred_birth_bytes_for(gesture_id),deferred_birth_bytes_for(node_ids),deferred_birth_bytes_for(dx),deferred_birth_bytes_for(dy)]),Self::SetSlider{node_id,value}=>sequence_birth_bytes(&[deferred_birth_bytes_for(node_id),deferred_birth_bytes_for(value)]),Self::InsertPort{node_id,side,index}=>sequence_birth_bytes(&[deferred_birth_bytes_for(node_id),deferred_birth_bytes_for(side),deferred_birth_bytes_for(index)])}}
 fn controlled_retirement_supported()->bool{true}
}
