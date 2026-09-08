//! 🌳️ Directed acyclic port graph: rectangle IO nodes on infinite canvas.

use std::cell::Cell;
use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};

use dsl::DslValue;
use semio_framework_artifact_infinite_dag::*;
use semio_framework_value_derive::{FromValue, ToValue};

use ::graph::manifest::{flow_dag::flow_dag_manifest, ManifestValidator, PropertyBag, PropertyValue};

pub use crate::infinite::board::ports::directed::{
    self as graph, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak,
    handle_exterior_cap_triangle_stroke_path, handle_outward_at_node_rim, CanvasPalette, DirectedPortGraphEngine, Edge, EdgeId, GraphExtension, Handle, HandleId, HandleRole, InteractionMode, Node, NodeId, RenderSnapshot, Selection,
};
pub use crate::infinite::canvas;
use graph::{handle_position, world_box_from_points, BoardEvent, WorldBox};

/// 🌳️ DAG board engine alias.
pub type DagBoardEngine = DirectedPortGraphEngine;

//#region ⚠️ Errors
/// 🚨️ Crate-local error for DAG fixture parsing, layout, and host-state mutation.
#[derive(Debug)]
pub enum DagError {
    FixtureRootNotObject,
    SchemaMismatch,
    NodesMissing,
    InvalidNodeKind(String),
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
            Self::FixtureRootNotObject => formatter.write_str("fixture root must be object"),
            Self::SchemaMismatch => formatter.write_str("schema must be dag.fixture"),
            Self::NodesMissing => formatter.write_str("nodes array missing"),
            Self::InvalidNodeKind(message) | Self::CanvasTheme(message) => formatter.write_str(message),
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

impl From<os_pack::json::JsonError> for DagError {
    fn from(error: os_pack::json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<dsl::ValueError> for DagError {
    fn from(error: dsl::ValueError) -> Self {
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
// #endregion 🔖️PortSide



/// 📛️ Reserved title row above computation IO channels.
const DAG_COMPUTATION_HEADER_ROWS: usize = 0;

const DAG_NODE_EDGE_INSET: f64 = ui_styling::metrics::dag::NODE_EDGE_INSET;
const DAG_IO_COLUMN_WIDTH: f64 = ui_styling::metrics::dag::IO_COLUMN_WIDTH;
const DAG_COMPONENT_WIDTH: f64 = DAG_IO_COLUMN_WIDTH * 2.0;
const DAG_DISTRIBUTE_MIN_GAP: f64 = ui_styling::metrics::board::LAYOUT_SIBLING_GAP;
const DAG_IO_WIDGET_HEIGHT: f64 = ui_styling::metrics::dag::IO_WIDGET_HEIGHT;
const DAG_LABEL_SCREEN_PX: f64 = ui_styling::metrics::label::DAG_DEFAULT_PX;
const DAG_LABEL_COMPACT_SCREEN_PX: f64 = ui_styling::metrics::label::DAG_COMPACT_PX;

enum ComputationChannelRowSide {
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



fn io_port_column_width(ports: &[IoPortSpec], _px: f64) -> f64 {
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

fn channel_row_bounds(node: &DagNodeSpec, row_index: usize) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let y_center = channel_row_center_y(node.y, node.height, row_index);
    let half = DAG_CHANNEL_ROW_HEIGHT * 0.5;
    (node.x - hw, y_center - half, node.x + hw, y_center + half)
}

fn input_port_row_hit_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
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

fn output_port_row_hit_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
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
const DAG_PREVIEW_PAD: f64 = 4.0;
const DAG_PREVIEW_ROW_HEIGHT: f64 = 14.0;
const DAG_PREVIEW_TREE_INDENT: f64 = 12.0;
const DAG_PREVIEW_TOGGLE_WIDTH: f64 = 10.0;
const DAG_PREVIEW_MAX_IMAGE: f64 = 200.0;
const DAG_PREVIEW_MIN_SIZE: f64 = 20.0;

#[derive(Clone, Debug)]
struct PreviewTreeRow {
    path: String,
    depth: usize,
    label: String,
    summary: String,
    has_children: bool,
    expanded: bool,
}

#[derive(Clone, Debug)]
struct PreviewTreeRowLayout {
    path: String,
    row_rect: (f64, f64, f64, f64),
}

enum WidgetPointerKind {
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
    use canvas::icon_codec::{board_resolve_icon_kind, BoardResolvedIcon};
    match board_resolve_icon_kind(src, |_| None) {
        BoardResolvedIcon::RasterRgba8 { w, h, .. } => (f64::from(w), f64::from(h)),
        BoardResolvedIcon::SvgPlain(s) | BoardResolvedIcon::SvgThemed(s) => {
            if let Ok((_, _, bw, bh)) = canvas::svg_icon::svg_icon_content_bounds_from_str(&s) {
                if bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite() {
                    return (bw, bh);
                }
            }
            (64.0, 48.0)
        }
        BoardResolvedIcon::None => (64.0, 48.0),
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
        DslValue::Number(n) => match n {
            dsl::Number::UInt(v) => v.to_string(),
            dsl::Number::Int(v) => v.to_string(),
            dsl::Number::Float(v) => v.to_string(),
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

fn truncate_label_to_fit_width(text: &str, max_width: f64, px: f64) -> String {
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

fn note_text_origin_x(node: &DagNodeSpec) -> f64 {
    let (x0, _, _, _) = preview_content_bounds(node);
    x0 + DAG_PREVIEW_PAD
}

fn hit_byte_in_note_line(line: &str, world_x: f64, line_origin_x: f64, font_px: f64) -> usize {
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

fn preview_tree_rows(json: &DslValue, expanded: &BTreeSet<String>, path: &str, depth: usize) -> Vec<PreviewTreeRow> {
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
pub fn measure_preview_content(content: &DagPreviewContent, expanded: &BTreeSet<String>) -> (f64, f64) {
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

fn preview_content_node_size(content: &DagPreviewContent, expanded: &BTreeSet<String>) -> (f64, f64) {
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
pub fn preview_widget_size(content: &DagPreviewContent, expanded: &BTreeSet<String>) -> (f64, f64) {
    preview_content_node_size(content, expanded)
}

fn preview_tree_row_layouts(node: &DagNodeSpec, json: &DslValue, expanded: &BTreeSet<String>) -> Vec<PreviewTreeRowLayout> {
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

fn validate_dag_fixture_node_kinds(nodes: &[DagNodeSpec]) -> Result<(), DagError> {
    let manifest = flow_dag_manifest();
    let validator = ManifestValidator::new(&manifest);
    for node in nodes {
        validator.validate_node_kind(dag_node_kind_tag(&node.kind)).map_err(|error| DagError::InvalidNodeKind(format!("{}: {}", error.path, error.message)))?;
    }
    Ok(())
}

fn uses_computation_layout(kind: &DagNodeKind) -> bool {
    matches!(kind, DagNodeKind::Computation { .. } | DagNodeKind::Cluster { .. })
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

fn point_in_rect(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
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

fn slider_track_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let pad = DAG_NODE_EDGE_INSET;
    let track_y = node.y;
    let hit_h = 8.0;
    (node.x - hw + pad, track_y - hit_h * 0.5, node.x + hw - pad, track_y + hit_h * 0.5)
}

fn slider_value_world_center(node: &DagNodeSpec, value_text: &str, paint_px: f64, zoom: f64) -> (f64, f64) {
    use canvas::text::label_extent;
    let hw = node.width * 0.5;
    let (value_w, _) = label_extent(value_text, paint_px);
    let z = zoom.max(0.05);
    let screen_gap = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_RATIO;
    let world_offset = (screen_gap + value_w * 0.5) / z;
    (node.x - hw - world_offset, node.y)
}

fn select_control_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    (node.x - hw + DAG_NODE_EDGE_INSET, node.y + hh * 0.12, node.x + hw - 10.0, node.y + hh - DAG_NODE_EDGE_INSET)
}

/// 📐️ Note node size from its text payload.
pub fn note_widget_size(_text: &str) -> (f64, f64) {
    (DAG_COMPONENT_WIDTH, DAG_CHANNEL_ROW_HEIGHT)
}

fn preview_content_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_PREVIEW_PAD;
    (node.x - hw + pad, node.y - hh + pad, node.x + hw - pad, node.y + hh - pad)
}

fn action_control_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_NODE_EDGE_INSET;
    (node.x - hw + pad, node.y + hh * 0.08, node.x + hw - pad, node.y + hh - pad)
}

fn set_slider_value_from_x(node: &mut DagNodeSpec, world_x: f64) -> Option<f64> {
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

const DAG_VARIADIC_PLUS_ZOOM_THRESHOLD: f64 = 1.5;

fn computation_input_label_x(node: &DagNodeSpec) -> f64 {
    node.x - node.width * 0.5 + DAG_NODE_EDGE_INSET
}

fn computation_output_label_x(node: &DagNodeSpec, label: &str, px: f64) -> f64 {
    let label_w = port_label_text_width(label, px);
    node.x + node.width * 0.5 - DAG_NODE_EDGE_INSET - label_w
}

fn computation_column_divider_x(node: &DagNodeSpec) -> Option<f64> {
    let inputs = node.inputs();
    let outputs = node.outputs();
    if inputs.is_empty() || outputs.is_empty() {
        return None;
    }
    let hw = node.width * 0.5;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let left_w = io_port_column_width(inputs, port_px);
    let right_w = io_port_column_width(outputs, port_px);
    let left_end = node.x - hw + DAG_NODE_EDGE_INSET + left_w;
    let right_start = node.x + hw - DAG_NODE_EDGE_INSET - right_w;
    Some(if right_start <= left_end { node.x } else { (left_end + right_start) * 0.5 })
}

fn computation_input_column_x_bounds(node: &DagNodeSpec) -> Option<(f64, f64)> {
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

fn computation_output_column_x_bounds(node: &DagNodeSpec) -> Option<(f64, f64)> {
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

fn computation_io_side_row_counts(node: &DagNodeSpec) -> (usize, usize) {
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

fn channel_row_divider_y(node_y: f64, node_height: f64, after_row_index: usize) -> f64 {
    let hh = node_height * 0.5;
    node_y - hh + after_row_index as f64 * DAG_CHANNEL_ROW_HEIGHT
}

fn computation_channel_row_divider_x_span(node: &DagNodeSpec, side: ComputationChannelRowSide) -> (f64, f64) {
    let hw = node.width * 0.5;
    let left_edge = node.x - hw;
    let right_edge = node.x + hw;
    match (computation_column_divider_x(node), side) {
        (Some(divider_x), ComputationChannelRowSide::Input) => (left_edge, divider_x),
        (Some(divider_x), ComputationChannelRowSide::Output) => (divider_x, right_edge),
        (None, ComputationChannelRowSide::Input) | (None, ComputationChannelRowSide::Output) => (left_edge, right_edge),
    }
}

fn computation_io_side_row_divider_indices(side_rows: usize, grid_rows: usize) -> std::ops::Range<usize> {
    if side_rows == 0 || grid_rows <= 1 {
        return 0..0;
    }
    1..side_rows + usize::from(side_rows < grid_rows)
}

fn computation_name_world_center(node: &DagNodeSpec, label: &str, paint_px: f64, zoom: f64) -> (f64, f64) {
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

fn computation_channel_row_count(node: &DagNodeSpec) -> usize {
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

fn variadic_input_insert_positions(node: &DagNodeSpec) -> Vec<(usize, f64, f64)> {
    let inputs = node.inputs();
    if !node.variadic_inputs() {
        return vec![];
    }
    let row = inputs.len();
    let port_y = computation_port_center_y(node, row);
    let port_x = computation_input_label_x(node);
    vec![(inputs.len(), port_x, port_y)]
}

fn variadic_output_insert_positions(node: &DagNodeSpec) -> Vec<(usize, f64, f64)> {
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

fn io_node_rect_port_angle_for_node(node: &DagNodeSpec, port_index: usize, left: bool) -> f64 {
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

// #region 🔖️Layout
//#region 🌳️TidyTree
/// 🌲️ Buchheim tidy-tree on string-labeled directed edges.
fn buchheim_positions(roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>) -> HashMap<String, (f64, f64)> {
    let roots_set: HashSet<String> = roots.iter().cloned().collect();
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for (u, v) in directed {
        incoming.entry(v.clone()).or_default().push(u.clone());
    }
    for v in incoming.values_mut() {
        v.sort();
        v.dedup();
    }
    let mut chosen_parent: HashMap<String, String> = HashMap::new();
    let mut all_ids: HashSet<String> = HashSet::new();
    for (u, v) in directed {
        all_ids.insert(u.clone());
        all_ids.insert(v.clone());
    }
    for r in roots {
        all_ids.insert(r.clone());
    }
    for id in &all_ids {
        if roots_set.contains(id) {
            continue;
        }
        let ps = incoming.get(id).cloned().unwrap_or_default();
        if ps.is_empty() {
            continue;
        }
        let best = ps
            .iter()
            .min_by_key(|p| {
                let dp = depth.get(*p).copied().unwrap_or(0);
                (dp, (*p).clone())
            })
            .expect("non-empty ps")
            .clone();
        chosen_parent.insert(id.clone(), best);
    }
    let mut ordered_ids: Vec<String> = all_ids.into_iter().collect();
    ordered_ids.sort();
    if ordered_ids.is_empty() {
        return HashMap::new();
    }
    let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
    let super_idx = ordered_ids.len();
    let mut nodes: Vec<BuchheimNode> =
        ordered_ids.iter().map(|id| BuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
    nodes.push(BuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: "__tree_super__".into(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
    for (i, oid) in ordered_ids.iter().enumerate() {
        let pidx = if roots_set.contains(oid) {
            super_idx
        } else {
            match chosen_parent.get(oid) {
                Some(p) => *id_to_idx.get(p).unwrap_or(&super_idx),
                None => super_idx,
            }
        };
        nodes[i].parent = Some(pidx);
    }
    for node in nodes.iter_mut() {
        node.children.clear();
    }
    for i in 0..super_idx {
        let pi = nodes[i].parent.expect("parent set for every non-super node in the loop above");
        nodes[pi].children.push(i);
    }
    for p in 0..=super_idx {
        let mut ch = nodes[p].children.clone();
        ch.sort_by_key(|&c| nodes[c].id.clone());
        nodes[p].children = ch;
    }
    for p in 0..=super_idx {
        if nodes[p].children.is_empty() {
            continue;
        }
        let ch = nodes[p].children.clone();
        for (k, &c) in ch.iter().enumerate() {
            nodes[c].number = (k + 1) as i32;
            nodes[c].ancestor = c;
        }
    }
    buchheim_first_walk(&mut nodes, super_idx, 1.0);
    let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY);
    if min_x.is_finite() && min_x < 0.0 {
        buchheim_third_walk(&mut nodes, super_idx, -min_x);
    }
    let mut out = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if i == super_idx || n.synthetic {
            continue;
        }
        out.insert(n.id.clone(), (n.x, n.y));
    }
    out
}

#[derive(Debug)]
struct BuchheimNode {
    id: String,
    parent: Option<usize>,
    children: Vec<usize>,
    x: f64,
    y: f64,
    mod_: f64,
    thread: Option<usize>,
    ancestor: usize,
    change: f64,
    shift: f64,
    number: i32,
    synthetic: bool,
}

fn buchheim_left_brother(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    let pos = ch.iter().position(|&c| c == i)?;
    if pos == 0 {
        return None;
    }
    Some(ch[pos - 1])
}

fn buchheim_leftmost_sibling(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    if ch.first() == Some(&i) {
        return None;
    }
    ch.first().copied()
}

fn buchheim_next_right(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.last().copied()
}

fn buchheim_next_left(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.first().copied()
}

fn buchheim_move_subtree(nodes: &mut [BuchheimNode], wl: usize, wr: usize, shift: f64) {
    let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
    if subtrees <= 0.0 {
        return;
    }
    nodes[wr].change -= shift / subtrees;
    nodes[wr].shift += shift;
    nodes[wl].change += shift / subtrees;
    nodes[wr].x += shift;
    nodes[wr].mod_ += shift;
}

fn buchheim_execute_shifts(nodes: &mut [BuchheimNode], v: usize) {
    let mut shift = 0.0f64;
    let mut change = 0.0f64;
    for &w in nodes[v].children.iter().rev() {
        nodes[w].x += shift;
        nodes[w].mod_ += shift;
        change += nodes[w].change;
        shift += nodes[w].shift + change;
    }
}

fn buchheim_apportion(nodes: &mut [BuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
    let w = match buchheim_left_brother(nodes, v) {
        Some(w) => w,
        None => return default_ancestor,
    };
    let mut vir = v;
    let mut vil = w;
    let mut vol = buchheim_leftmost_sibling(nodes, v).unwrap_or(v);
    let mut vor = v;
    let mut sir = nodes[v].mod_;
    let mut sil = nodes[vil].mod_;
    loop {
        let vil_r = buchheim_next_right(nodes, vil);
        let vir_l = buchheim_next_left(nodes, vir);
        if vil_r.is_none() || vir_l.is_none() {
            break;
        }
        vil = vil_r.expect("checked Some above");
        vir = vir_l.expect("checked Some above");
        let vol_l = buchheim_next_left(nodes, vol);
        let vor_r = buchheim_next_right(nodes, vor);
        if vol_l.is_none() || vor_r.is_none() {
            break;
        }
        vol = vol_l.expect("checked Some above");
        vor = vor_r.expect("checked Some above");
        nodes[vor].ancestor = v;
        let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
        if shift > 0.0 {
            buchheim_move_subtree(nodes, default_ancestor, v, shift);
            sir += shift;
        }
        sil += nodes[vil].mod_;
        sir += nodes[vir].mod_;
    }
    default_ancestor
}

fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64) -> usize {
    if nodes[v].children.is_empty() {
        if let Some(lb) = buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[lb].x + distance;
        } else {
            nodes[v].x = 0.0;
        }
        return v;
    }
    let mut default_ancestor = nodes[v].children[0];
    for &w in nodes[v].children.clone().iter() {
        buchheim_first_walk(nodes, w, distance);
        default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
    }
    buchheim_execute_shifts(nodes, v);
    let c0 = nodes[v].children[0];
    let c1 = *nodes[v].children.last().expect("children non-empty per the is_empty check above");
    let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
    if let Some(w) = buchheim_left_brother(nodes, v) {
        nodes[v].x = nodes[w].x + distance;
        nodes[v].mod_ = nodes[v].x - mid;
    } else {
        nodes[v].x = mid;
    }
    v
}

fn buchheim_second_walk(nodes: &mut [BuchheimNode], v: usize, m: f64, depth: i32, min_x: f64) -> f64 {
    nodes[v].x += m;
    nodes[v].y = depth as f64;
    let mut min_x = min_x.min(nodes[v].x);
    for &w in nodes[v].children.clone().iter() {
        min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x);
    }
    min_x
}

fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64) {
    nodes[v].x += n;
    for &c in nodes[v].children.clone().iter() {
        buchheim_third_walk(nodes, c, n);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️tidy-tree/🦀️.rs"]
mod tidy_tree_tests;
//#endregion 🌳️TidyTree

use dsl::os_pack::json::Value;

/// 🧭️ Tree layout flow direction for layered DAG positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum DagLayoutOrientation {
    #[default]
    LeftRight,
    TopBottom,
}

/// 🌲️ Layered DAG layout options for fixture JSON. `ToValue`/`FromValue` added
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass) so
/// `host::FlowHost::reorganize` can route through `pack::json::from_json_str` instead of
/// `serde_json::from_str` — `host::FlowCoreError` only has `From<pack::json::JsonError>`, not
/// `From<serde_json::Error>` (see its own docstring), so the old call never actually compiled
/// with the `?` operator once that conversion impl was dropped.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagLayoutOptions {
    #[value(default = "default_layer_spacing")]
    pub layer_spacing: f64,
    #[value(default = "default_sibling_gap")]
    pub sibling_gap: f64,
    #[value(default)]
    pub orientation: DagLayoutOrientation,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
}

fn default_layer_spacing() -> f64 {
    ui_styling::metrics::board::LAYOUT_LAYER_SPACING
}

fn default_sibling_gap() -> f64 {
    ui_styling::metrics::board::LAYOUT_SIBLING_GAP
}

fn resolve_layout_node_id(handle_to_node: &HashMap<String, String>, key: &str, node_ids: &HashSet<String>) -> String {
    if let Some(nid) = handle_to_node.get(key) {
        return nid.clone();
    }
    if node_ids.contains(key) {
        return key.to_string();
    }
    if let Some(base) = key.split('@').next() {
        if node_ids.contains(base) {
            return base.to_string();
        }
    }
    key.to_string()
}

impl Default for DagLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_layer_spacing(), sibling_gap: default_sibling_gap(), orientation: DagLayoutOrientation::default(), center_x: None, center_y: None }
    }
}

/// 🌳️ Writes node centers from a layered DAG layout into `dag.fixture`.
pub fn apply_dag_layout_to_fixture_v1_value(fixture: &mut Value, opts: &DagLayoutOptions) -> Result<(), DagError> {
    let Some(root) = fixture.as_object_mut() else {
        return Err(DagError::FixtureRootNotObject);
    };
    if root.get("schema").and_then(|v| v.as_str()) != Some("dag.fixture") {
        return Err(DagError::SchemaMismatch);
    }
    let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
        return Err(DagError::NodesMissing);
    };
    if nodes.is_empty() {
        return Ok(());
    }
    let mut handle_to_node: HashMap<String, String> = HashMap::new();
    let mut node_ids: HashSet<String> = HashSet::new();
    for node in nodes.iter() {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        node_ids.insert(nid.to_string());
        if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
            for h in handles {
                if let Some(hid) = h.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
    }
    let mut directed: Vec<(String, String)> = Vec::new();
    for e in &edges_json {
        let Some(eo) = e.as_object() else {
            continue;
        };
        let src = eo.get("source").and_then(|v| v.as_str()).or_else(|| eo.get("sourceHandle").and_then(|v| v.as_str()));
        let tgt = eo.get("target").and_then(|v| v.as_str()).or_else(|| eo.get("targetHandle").and_then(|v| v.as_str()));
        let (Some(src_h), Some(tgt_h)) = (src, tgt) else {
            continue;
        };
        let u = resolve_layout_node_id(&handle_to_node, src_h, &node_ids);
        let v = resolve_layout_node_id(&handle_to_node, tgt_h, &node_ids);
        if u != v && node_ids.contains(&u) && node_ids.contains(&v) {
            directed.push((u, v));
        }
    }
    let mut incoming: HashMap<String, u32> = HashMap::new();
    for id in &node_ids {
        incoming.insert(id.clone(), 0);
    }
    for (_, v) in &directed {
        *incoming.entry(v.clone()).or_insert(0) += 1;
    }
    let roots: Vec<String> = node_ids.iter().filter(|id| incoming.get(*id).copied().unwrap_or(0) == 0).cloned().collect();
    let roots = if roots.is_empty() { node_ids.iter().cloned().collect() } else { roots };
    let mut depth: HashMap<String, i32> = HashMap::new();
    for r in &roots {
        depth.insert(r.clone(), 0);
    }
    for _ in 0..directed.len().saturating_add(node_ids.len()).saturating_add(4) {
        let mut changed = false;
        for (u, v) in &directed {
            let Some(&du) = depth.get(u) else {
                continue;
            };
            let nd = du + 1;
            if depth.get(v).copied().unwrap_or(-1) < nd {
                depth.insert(v.clone(), nd);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let pos = buchheim_positions(&roots, &directed, &depth);
    let mut minx = f64::INFINITY;
    let mut maxx = f64::NEG_INFINITY;
    let mut miny = f64::INFINITY;
    let mut maxy = f64::NEG_INFINITY;
    for (x, y) in pos.values() {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }
    let cx = (minx + maxx) * 0.5;
    let cy = (miny + maxy) * 0.5;
    let gx = opts.center_x.unwrap_or(0.0);
    let gy = opts.center_y.unwrap_or(0.0);
    let (dx, dy) = match opts.orientation {
        DagLayoutOrientation::LeftRight => (gx - cy * opts.layer_spacing, gy - cx * opts.sibling_gap),
        DagLayoutOrientation::TopBottom => (gx - cx * opts.sibling_gap, gy - cy * opts.layer_spacing),
    };
    for node in nodes.iter_mut() {
        let Some(obj) = node.as_object_mut() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some((bx, by)) = pos.get(nid) else {
            continue;
        };
        let (nx, ny) = match opts.orientation {
            DagLayoutOrientation::LeftRight => (by * opts.layer_spacing + dx, bx * opts.sibling_gap + dy),
            DagLayoutOrientation::TopBottom => (bx * opts.sibling_gap + dx, by * opts.layer_spacing + dy),
        };
        obj.insert("x", Value::from(nx));
        obj.insert("y", Value::from(ny));
    }
    Ok(())
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

// #region 🔖️Lod
use canvas::lod::{Lod, LodScale};

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

fn dag_lod_index(zoom: f64) -> usize {
    DAG_LOD_SCALE.resolve_index(dag_lod_resolve_zoom(zoom))
}

fn dag_lod_band_floor_zoom(lod_index: usize) -> f64 {
    let floor = canvas::lod::band_floor_zoom(DAG_LOD_BAND_FLOOR_ZOOM, lod_index, 0.05);
    if lod_index == 0 {
        floor
    } else {
        (floor + DAG_LOD_ZOOM_SHIFT).max(0.05)
    }
}

/// 🔵️ Port dot world radius; screen size grows with camera zoom like node geometry.
const DAG_HANDLE_WORLD_RADIUS: f64 = ui_styling::radii::DAG_HANDLE_WORLD;

const DAG_NODE_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE;
const DAG_NODE_STROKE_SELECTED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_SELECTED;
const DAG_NODE_STROKE_HOVERED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_HOVERED;


const DAG_CHROME_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_CHROME;
const DAG_BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;

fn dag_world_stroke(screen_px: f64, zoom: f64) -> f64 {
    (screen_px / zoom.max(0.05)).max(1e-3)
}

fn dag_label_layout_px() -> f64 {
    DAG_LABEL_SCREEN_PX
}

fn dag_label_paint_px(zoom: f64, lod_index: usize) -> f64 {
    canvas::lod::lod_band_label_screen_px(DAG_LABEL_SCREEN_PX, zoom, dag_lod_band_floor_zoom(lod_index))
}

fn dag_label_compact_paint_px(zoom: f64, lod_index: usize) -> f64 {
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

pub(crate) fn dag_node_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
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

pub(crate) fn dag_node_label_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
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

/// @emoji 🧱️ Internal column/row chrome inside a node body; selection/hover matches label emphasis.
pub(crate) fn dag_node_internal_chrome_stroke(body_stroke: canvas::Color, label_fill: canvas::Color, emphasized: bool) -> canvas::Color {
    if emphasized {
        label_fill
    } else {
        body_stroke
    }
}

pub(crate) fn dag_handle_body_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
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

pub(crate) fn dag_handle_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
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

pub(crate) fn dag_edge_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> canvas::Color {
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

fn dag_node_stroke_screen_px(dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> f64 {
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

/// @emoji 🎨️ Node body fill when painted; `None` means stroke/text only (puzzle 2d overview+).
pub(crate) fn dag_node_paint_fill(lod: DagDrawLod, theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> Option<canvas::Color> {
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
            os_pack::json::object([("id".to_string(), Value::from(lod.id)), ("name".to_string(), Value::from(lod.name)), ("description".to_string(), Value::from(lod.description)), ("maxZoom".to_string(), lod_max_zoom_json(max_zoom))])
        })
        .collect();
    os_pack::json::to_string(&Value::Array(rows))
}
// #endregion 🔖️Lod

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, but wasip2 has real stderr (WASI),
// so it takes the `eprintln!` arm rather than the browser `console.log` one — case (b): wasip2 gets
// its own implementation via the existing native path, not a browser bridge.
fn dag_debug_log(msg: &str) {
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    web_sys::console::log_1(&msg.into());
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    eprintln!("{msg}");
}

//#region 🔖️Grid
const GRID_WORLD_LARGE: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;
const GRID_WORLD_MEDIUM: f64 = ui_styling::metrics::board::GRID_WORLD_MEDIUM;
const GRID_WORLD_SMALL: f64 = ui_styling::metrics::board::GRID_WORLD_SMALL;
const GRID_WORLD_MICRO: f64 = ui_styling::metrics::board::GRID_WORLD_MICRO;
const GRID_FACTOR_DEFAULT: f64 = ui_styling::metrics::board::GRID_FACTOR_DEFAULT;
//#endregion 🔖️Grid

// #region 🔖️ChannelRef
/// 🔌️ Resolved fixture channel from a port handle hover or selection.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagChannelRef {
    pub widget_id: String,
    pub port: String,
    pub direction: String,
}

impl DagChannelRef {
    pub fn is_input(&self) -> bool {
        self.direction == "in"
    }

    pub fn is_output(&self) -> bool {
        self.direction == "out"
    }
}
// #endregion 🔖️ChannelRef

// #region 🔖️NoteEdit

#[derive(Clone, Debug)]
struct NoteEditState {
    node_id: String,
    caret: usize,
    anchor: usize,
}

// #endregion 🔖️NoteEdit

// #region 🔖️DagHost

pub const DAG_INTERACTION_NODE_CAPACITY: usize = 256;
const DAG_INTERACTION_WORD_CAPACITY: usize = DAG_INTERACTION_NODE_CAPACITY / 64;

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
struct DagNodeMove {
    index: u16,
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug)]
#[expect(clippy::large_enum_variant, reason = "Pointer plans copy the complete fixed-capacity gesture snapshot before atomic admission; boxing drag state would allocate and break Copy.")]
enum DagProjectionGesture {
    Idle,
    Pan { start_x: f64, start_y: f64, camera: [f64; 3] },
    Drag { start_x: f64, start_y: f64, starts: [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY], len: u16 },
    Select { start_x: f64, start_y: f64, initial: [u64; DAG_INTERACTION_WORD_CAPACITY] },
}

#[derive(Clone, Copy, Debug)]
pub struct DagInteractionProjection {
    revision: u64,
    camera: [f64; 3],
    selected: [u64; DAG_INTERACTION_WORD_CAPACITY],
    hover: Option<u16>,
    gesture: DagProjectionGesture,
}

/// 📍️ Owned selection, hover and camera values from an admitted pointer projection.
pub struct DagPointerSnapshot {
    pub node_ids: Vec<String>,
    pub hovered_id: Option<String>,
    pub camera: [f64; 3],
}

#[derive(Clone, Copy, Debug)]
pub struct DagPointerPlan {
    expected_revision: u64,
    previous_active: bool,
    next: DagInteractionProjection,
    moves: [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY],
    move_len: u16,
}

pub const DAG_CURSOR_MAX_OUTPUT_BYTES: usize = 65_536;

/// ⛽️ One cancellable semantic-unit grant for a retained DAG cursor.
#[derive(Clone, Copy, Debug)]
pub struct DagCursorGrant {
    pub fuel: u8,
    pub now_milliseconds: u64,
    pub deadline_milliseconds: u64,
    pub cancelled: bool,
    pub interrupted: bool,
}

/// 🚧️ Fail-closed retained DAG cursor faults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagCursorFault {
    Cancelled,
    Interrupted,
    Deadline,
    NoFuel,
    Limit,
    Sealed,
}

/// 📬️ A census, byte, progress, or terminal result from one DAG cursor grant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagCursorStep {
    Progress { completed: usize, total: usize },
    Census { bytes: usize },
    Byte(u8),
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DagSelectedNodesJsonPhase {
    CensusNode,
    CensusText,
    Open,
    Seek,
    Separator,
    QuoteOpen,
    Text,
    Escape,
    QuoteClose,
    Close,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DagSelectedJsonKind {
    Nodes,
    Edges,
}

/// 🎯️ Exact-preflight JSON array encoder that inspects or emits at most one unit per grant.
pub struct DagSelectedNodesJsonCursor {
    kind: DagSelectedJsonKind,
    phase: DagSelectedNodesJsonPhase,
    node_cursor: usize,
    text_cursor: usize,
    selected_count: usize,
    emitted_count: usize,
    census_bytes: usize,
    output_cursor: usize,
    escape: [u8; 6],
    escape_length: u8,
    escape_cursor: u8,
}

impl Default for DagSelectedNodesJsonCursor {
    fn default() -> Self {
        Self {
            kind: DagSelectedJsonKind::Nodes,
            phase: DagSelectedNodesJsonPhase::CensusNode,
            node_cursor: 0,
            text_cursor: 0,
            selected_count: 0,
            emitted_count: 0,
            census_bytes: 2,
            output_cursor: 0,
            escape: [0; 6],
            escape_length: 0,
            escape_cursor: 0,
        }
    }
}

impl DagSelectedNodesJsonCursor {
    pub fn edges() -> Self {
        Self { kind: DagSelectedJsonKind::Edges, ..Self::default() }
    }

    fn guard(grant: DagCursorGrant) -> Result<(), DagCursorFault> {
        if grant.cancelled {
            Err(DagCursorFault::Cancelled)
        } else if grant.interrupted {
            Err(DagCursorFault::Interrupted)
        } else if grant.now_milliseconds >= grant.deadline_milliseconds {
            Err(DagCursorFault::Deadline)
        } else if grant.fuel == 0 {
            Err(DagCursorFault::NoFuel)
        } else {
            Ok(())
        }
    }

    fn selected(host: &DagHost, index: usize) -> bool {
        let Ok(id) = u64::try_from(index.saturating_add(1)) else {
            return false;
        };
        host.node_id_map.get(&id) == Some(&index) && host.engine.selection.node_ids.contains(&id)
    }

    fn count(&self, host: &DagHost) -> usize {
        match self.kind {
            DagSelectedJsonKind::Nodes => host.fixture.nodes.len(),
            DagSelectedJsonKind::Edges => host.fixture.edges.len(),
        }
    }

    fn item<'a>(&self, host: &'a DagHost, index: usize) -> Option<&'a str> {
        match self.kind {
            DagSelectedJsonKind::Nodes => Self::selected(host, index).then(|| host.fixture.nodes.get(index).map(|node| node.id.as_str())).flatten(),
            DagSelectedJsonKind::Edges => host.edge_engine_ids.get(index).and_then(|id| *id).filter(|id| host.engine.selection.edge_ids.contains(id)).and_then(|_| host.fixture.edges.get(index).map(|edge| edge.id.as_str())),
        }
    }

    fn escape(byte: u8) -> ([u8; 6], u8) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        match byte {
            b'"' => ([b'\\', b'"', 0, 0, 0, 0], 2),
            b'\\' => ([b'\\', b'\\', 0, 0, 0, 0], 2),
            0x00..=0x1f => ([b'\\', b'u', b'0', b'0', HEX[usize::from(byte >> 4)], HEX[usize::from(byte & 0x0f)]], 6),
            _ => ([byte, 0, 0, 0, 0, 0], 1),
        }
    }

    pub fn step(&mut self, host: &DagHost, grant: DagCursorGrant) -> Result<DagCursorStep, DagCursorFault> {
        Self::guard(grant)?;
        let count = self.count(host);
        match self.phase {
            DagSelectedNodesJsonPhase::CensusNode => {
                if self.node_cursor == count {
                    if self.census_bytes > DAG_CURSOR_MAX_OUTPUT_BYTES {
                        return Err(DagCursorFault::Limit);
                    }
                    self.node_cursor = 0;
                    self.text_cursor = 0;
                    self.phase = DagSelectedNodesJsonPhase::Open;
                    return Ok(DagCursorStep::Census { bytes: self.census_bytes });
                }
                if self.item(host, self.node_cursor).is_some() {
                    self.census_bytes = self.census_bytes.checked_add(2 + usize::from(self.selected_count != 0)).ok_or(DagCursorFault::Limit)?;
                    self.phase = DagSelectedNodesJsonPhase::CensusText;
                } else {
                    self.node_cursor += 1;
                }
                Ok(DagCursorStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectedNodesJsonPhase::CensusText => {
                let text = self.item(host, self.node_cursor).ok_or(DagCursorFault::Limit)?.as_bytes();
                if self.text_cursor == text.len() {
                    self.selected_count += 1;
                    self.node_cursor += 1;
                    self.text_cursor = 0;
                    self.phase = DagSelectedNodesJsonPhase::CensusNode;
                } else {
                    self.census_bytes = self.census_bytes.checked_add(usize::from(Self::escape(text[self.text_cursor]).1)).ok_or(DagCursorFault::Limit)?;
                    self.text_cursor += 1;
                }
                Ok(DagCursorStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectedNodesJsonPhase::Open => {
                self.phase = DagSelectedNodesJsonPhase::Seek;
                self.output_cursor += 1;
                Ok(DagCursorStep::Byte(b'['))
            }
            DagSelectedNodesJsonPhase::Seek => {
                if self.node_cursor == count {
                    self.phase = DagSelectedNodesJsonPhase::Close;
                } else if self.item(host, self.node_cursor).is_some() {
                    self.phase = if self.emitted_count == 0 { DagSelectedNodesJsonPhase::QuoteOpen } else { DagSelectedNodesJsonPhase::Separator };
                } else {
                    self.node_cursor += 1;
                }
                Ok(DagCursorStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectedNodesJsonPhase::Separator => {
                self.phase = DagSelectedNodesJsonPhase::QuoteOpen;
                self.output_cursor += 1;
                Ok(DagCursorStep::Byte(b','))
            }
            DagSelectedNodesJsonPhase::QuoteOpen => {
                self.phase = DagSelectedNodesJsonPhase::Text;
                self.output_cursor += 1;
                Ok(DagCursorStep::Byte(b'"'))
            }
            DagSelectedNodesJsonPhase::Text => {
                let text = self.item(host, self.node_cursor).ok_or(DagCursorFault::Limit)?.as_bytes();
                if self.text_cursor == text.len() {
                    self.phase = DagSelectedNodesJsonPhase::QuoteClose;
                    return Ok(DagCursorStep::Progress { completed: self.output_cursor, total: self.census_bytes });
                }
                let (escape, length) = Self::escape(text[self.text_cursor]);
                if length == 1 {
                    self.text_cursor += 1;
                    self.output_cursor += 1;
                    Ok(DagCursorStep::Byte(escape[0]))
                } else {
                    self.escape = escape;
                    self.escape_length = length;
                    self.escape_cursor = 0;
                    self.phase = DagSelectedNodesJsonPhase::Escape;
                    Ok(DagCursorStep::Progress { completed: self.output_cursor, total: self.census_bytes })
                }
            }
            DagSelectedNodesJsonPhase::Escape => {
                let byte = self.escape[usize::from(self.escape_cursor)];
                self.escape_cursor += 1;
                self.output_cursor += 1;
                if self.escape_cursor == self.escape_length {
                    self.text_cursor += 1;
                    self.phase = DagSelectedNodesJsonPhase::Text;
                }
                Ok(DagCursorStep::Byte(byte))
            }
            DagSelectedNodesJsonPhase::QuoteClose => {
                self.emitted_count += 1;
                self.node_cursor += 1;
                self.text_cursor = 0;
                self.phase = DagSelectedNodesJsonPhase::Seek;
                self.output_cursor += 1;
                Ok(DagCursorStep::Byte(b'"'))
            }
            DagSelectedNodesJsonPhase::Close => {
                self.phase = DagSelectedNodesJsonPhase::Complete;
                self.output_cursor += 1;
                Ok(DagCursorStep::Byte(b']'))
            }
            DagSelectedNodesJsonPhase::Complete if self.output_cursor == self.census_bytes => Ok(DagCursorStep::Complete),
            DagSelectedNodesJsonPhase::Complete => Err(DagCursorFault::Limit),
        }
    }
}

fn dag_bit_contains(bits: &[u64; DAG_INTERACTION_WORD_CAPACITY], index: usize) -> bool {
    bits.get(index / 64).is_some_and(|word| word & (1_u64 << (index % 64)) != 0)
}

fn dag_bit_set(bits: &mut [u64; DAG_INTERACTION_WORD_CAPACITY], index: usize, selected: bool) {
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

/// 🌳️ Retained DAG host: typed nodes, edges, engine, camera.
pub struct DagHost {
    pub fixture: DagFixture,
    pub engine: DagBoardEngine,
    pub canvas_theme: CanvasPalette,
    width: u32,
    height: u32,
    dpr: f64,
    last_screen_x: f64,
    last_screen_y: f64,
    node_id_map: HashMap<NodeId, usize>,
    handle_key_map: HashMap<HandleId, String>,
    handle_port_shape: HashMap<HandleId, PortShape>,
    handle_port_visible: HashMap<HandleId, bool>,
    edge_id_map: HashMap<EdgeId, String>,
    edge_engine_ids: Vec<Option<EdgeId>>,
    edge_route_style: HashMap<EdgeId, EdgeRouteStyle>,
    widget_drag: Option<usize>,
    pending_port_insert: Option<(DagPortSide, String, usize)>,
    last_logged_lod: Cell<i8>,
    dimmed: HashSet<NodeId>,
    wheel_zoom_active: bool,
    wheel_zoom_render_lod: Option<DagDrawLod>,
    automatic_lod: bool,
    forced_draw_lod: Option<DagDrawLod>,
    grid_visible: bool,
    grid_snap_enabled: bool,
    grid_factor: f64,
    icon_paint_cache: graph::IconPaintCache,
    ghost_node: Option<DagNodeSpec>,
    pending_cluster_explode: Option<String>,
    pending_export_click: Option<String>,
    pending_open_instance_id: Option<String>,
    last_pointer_down_at_ms: f64,
    last_pointer_down_world: (f64, f64),
    last_pointer_down_node_id: Option<String>,
    computing_active: Option<NodeId>,
    computing_stale: HashSet<NodeId>,
    computing_active_anim_phase: Cell<f64>,
    computing_stale_anim_phase: Cell<f64>,
    node_eval_status: HashMap<NodeId, DagNodeEvalStatusKind>,
    unresolved_input_ports: HashSet<(NodeId, String)>,
    editing_note: Option<NoteEditState>,
    caret_visible: bool,
    pan_anchor: Option<(f64, f64, f64, f64)>,
    minimap_widget_visible: bool,
    minimap_widget_hovered: bool,
    minimap_widget_drag: Option<(f64, f64)>,
}

/// 🧮️ One retained retirement turn; `credited_bytes` never exceeds the current grant,
/// while `released_bytes` is the physical backing freed after enough credits were retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagRetirementStep {
    Blocked,
    Pending { released_items: usize, credited_bytes: usize, released_bytes: usize },
    Complete,
}

enum DagRetirementOwner {
    Text(String),
    Bytes { value: Vec<u8>, remaining_backing_bytes: usize },
    Dsl(DslValue),
    DslValues { values: Vec<DslValue>, remaining_backing_bytes: usize },
    DslEntries { values: Vec<(String, DslValue)>, remaining_backing_bytes: usize },
    Property(PropertyValue),
    Properties(PropertyBag),
    PropertyValues { values: Vec<PropertyValue>, remaining_backing_bytes: usize },
    Port(IoPortSpec),
    Ports { values: Vec<IoPortSpec>, remaining_backing_bytes: usize },
    Strings { values: Vec<String>, remaining_backing_bytes: usize },
    Expanded(BTreeSet<String>),
    Preview(DagPreviewContent),
    NodeKind(DagNodeKind),
    FixtureNode(DagNodeSpec),
    FixtureNodes { values: Vec<DagNodeSpec>, remaining_backing_bytes: usize },
    FixtureEdge(DagFixtureEdge),
    FixtureEdges { values: Vec<DagFixtureEdge>, remaining_backing_bytes: usize },
    EngineNode(Node),
    EngineHandle(Handle),
    EngineSemantics(::graph::ElementSemantics),
    EngineEvent(BoardEvent),
    EngineEvents { values: Vec<BoardEvent>, remaining_backing_bytes: usize },
    Ids { values: Vec<u64>, remaining_backing_bytes: usize },
    OptionalIds { values: Vec<Option<u64>>, remaining_backing_bytes: usize },
}

fn dag_vec_backing_bytes<T>(values: &Vec<T>) -> usize {
    values.capacity().saturating_mul(size_of::<T>())
}

struct DagPayloadRetirement {
    owners: std::mem::ManuallyDrop<LinkedList<DagRetirementOwner>>,
}

impl Default for DagPayloadRetirement {
    fn default() -> Self {
        Self { owners: std::mem::ManuallyDrop::new(LinkedList::new()) }
    }
}

impl DagPayloadRetirement {
    fn push(&mut self, owner: DagRetirementOwner) {
        self.owners.push_front(owner);
    }

    fn bytes(&mut self, value: Vec<u8>) {
        let remaining_backing_bytes = value.capacity();
        self.push(DagRetirementOwner::Bytes { value, remaining_backing_bytes });
    }

    fn text(&mut self, value: String) {
        self.bytes(value.into_bytes());
    }

    fn dsl_values(&mut self, values: Vec<DslValue>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::DslValues { values, remaining_backing_bytes });
    }

    fn dsl_entries(&mut self, values: Vec<(String, DslValue)>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::DslEntries { values, remaining_backing_bytes });
    }

    fn property_values(&mut self, values: Vec<PropertyValue>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::PropertyValues { values, remaining_backing_bytes });
    }

    fn ports(&mut self, values: Vec<IoPortSpec>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::Ports { values, remaining_backing_bytes });
    }

    fn strings(&mut self, values: Vec<String>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::Strings { values, remaining_backing_bytes });
    }

    fn fixture_nodes(&mut self, values: Vec<DagNodeSpec>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::FixtureNodes { values, remaining_backing_bytes });
    }

    fn fixture_edges(&mut self, values: Vec<DagFixtureEdge>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::FixtureEdges { values, remaining_backing_bytes });
    }

    fn ids(&mut self, values: Vec<u64>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::Ids { values, remaining_backing_bytes });
    }

    fn engine_events(&mut self, values: Vec<BoardEvent>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::EngineEvents { values, remaining_backing_bytes });
    }

    fn optional_ids(&mut self, values: Vec<Option<u64>>) {
        let remaining_backing_bytes = dag_vec_backing_bytes(&values);
        self.push(DagRetirementOwner::OptionalIds { values, remaining_backing_bytes });
    }

    fn credit_backing<T>(&mut self, values: Vec<T>, remaining_backing_bytes: usize, maximum_bytes: usize, restore: impl FnOnce(Vec<T>, usize) -> DagRetirementOwner) -> Result<(Vec<T>, usize), DagRetirementStep> {
        let credited_bytes = maximum_bytes.min(remaining_backing_bytes);
        let remaining_backing_bytes = remaining_backing_bytes - credited_bytes;
        if remaining_backing_bytes != 0 {
            self.push(restore(values, remaining_backing_bytes));
            return Err(DagRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 });
        }
        Ok((values, credited_bytes))
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> DagRetirementStep {
        if self.owners.is_empty() {
            return DagRetirementStep::Complete;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return DagRetirementStep::Blocked;
        }
        let owner = self.owners.pop_front().expect("nonempty DAG payload retirement");
        match owner {
            DagRetirementOwner::Text(value) => {
                self.text(value);
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::Bytes { value, remaining_backing_bytes } => {
                let credited_bytes = maximum_bytes.min(remaining_backing_bytes);
                let remaining_backing_bytes = remaining_backing_bytes - credited_bytes;
                if remaining_backing_bytes != 0 {
                    self.push(DagRetirementOwner::Bytes { value, remaining_backing_bytes });
                    return DagRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
                }
                let released_bytes = value.capacity();
                drop(value);
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes }
            }
            DagRetirementOwner::Dsl(value) => {
                match value {
                    DslValue::String(value) => self.text(value),
                    DslValue::Array(values) => self.dsl_values(values),
                    DslValue::Object(values) => self.dsl_entries(values),
                    DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => {}
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::DslValues { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::DslValues { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::DslValues { values, remaining_backing_bytes: 0 });
                }
                if let Some(value) = next {
                    self.push(DagRetirementOwner::Dsl(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::DslEntries { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::DslEntries { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::DslEntries { values, remaining_backing_bytes: 0 });
                }
                if let Some((key, value)) = next {
                    self.text(key);
                    self.push(DagRetirementOwner::Dsl(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::Property(value) => {
                match value {
                    PropertyValue::String(value) => self.text(value),
                    PropertyValue::Array(values) => self.property_values(values),
                    PropertyValue::Object(values) => self.push(DagRetirementOwner::Properties(values)),
                    PropertyValue::Null | PropertyValue::Bool(_) | PropertyValue::Number(_) => {}
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::Properties(mut values) => {
                if let Some((key, value)) = values.pop_first() {
                    if !values.is_empty() {
                        self.push(DagRetirementOwner::Properties(values));
                    }
                    self.text(key);
                    self.push(DagRetirementOwner::Property(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::PropertyValues { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::PropertyValues { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::PropertyValues { values, remaining_backing_bytes: 0 });
                }
                if let Some(value) = next {
                    self.push(DagRetirementOwner::Property(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::Port(value) => {
                let IoPortSpec { id, label, code, abbreviation, full_name, value_type, default, value, connected: _, artifact_kind, cardinality, shape: _, visible: _, resolved: _ } = value;
                self.text(id);
                self.text(label);
                self.text(code);
                self.text(abbreviation);
                self.text(full_name);
                if let Some(value) = value_type {
                    self.text(value);
                }
                if let Some(value) = default {
                    self.push(DagRetirementOwner::Dsl(value));
                }
                if let Some(value) = value {
                    self.push(DagRetirementOwner::Dsl(value));
                }
                if let Some(value) = artifact_kind {
                    self.text(value);
                }
                self.text(cardinality);
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::Ports { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::Ports { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::Ports { values, remaining_backing_bytes: 0 });
                }
                if let Some(value) = next {
                    self.push(DagRetirementOwner::Port(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::Strings { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::Strings { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::Strings { values, remaining_backing_bytes: 0 });
                }
                if let Some(value) = next {
                    self.text(value);
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::Expanded(mut values) => {
                if let Some(value) = values.pop_first() {
                    if !values.is_empty() {
                        self.push(DagRetirementOwner::Expanded(values));
                    }
                    self.text(value);
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::Preview(value) => {
                match value {
                    DagPreviewContent::Empty => {}
                    DagPreviewContent::Scalar { text } => self.text(text),
                    DagPreviewContent::Image { src } => self.text(src),
                    DagPreviewContent::Tree { json } => self.push(DagRetirementOwner::Dsl(json)),
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::NodeKind(value) => {
                match value {
                    DagNodeKind::Computation { inputs, outputs, variadic_inputs: _, variadic_outputs: _ } | DagNodeKind::Cluster { inputs, outputs } => {
                        self.ports(inputs);
                        self.ports(outputs);
                    }
                    DagNodeKind::Slider { min: _, max: _, step: _, value: _, output } => self.push(DagRetirementOwner::Port(output)),
                    DagNodeKind::Select { options, selected: _, output } => {
                        self.strings(options);
                        self.push(DagRetirementOwner::Port(output));
                    }
                    DagNodeKind::Screen { media, input } => {
                        if let Some(media) = media {
                            self.text(media.src);
                        }
                        self.push(DagRetirementOwner::Port(input));
                    }
                    DagNodeKind::Note { text, output } => {
                        self.text(text);
                        self.push(DagRetirementOwner::Port(output));
                    }
                    DagNodeKind::Image { src, output } => {
                        self.text(src);
                        self.push(DagRetirementOwner::Port(output));
                    }
                    DagNodeKind::Preview { content, expanded, input } => {
                        self.push(DagRetirementOwner::Preview(content));
                        self.push(DagRetirementOwner::Expanded(expanded));
                        self.push(DagRetirementOwner::Port(input));
                    }
                    DagNodeKind::Action { label, input } => {
                        self.text(label);
                        self.push(DagRetirementOwner::Port(input));
                    }
                    DagNodeKind::Export { label, format, input } => {
                        self.text(label);
                        self.text(format);
                        self.push(DagRetirementOwner::Port(input));
                    }
                    DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs } => {
                        self.text(instance_id);
                        self.text(plugin_id);
                        self.text(app_id);
                        self.text(icon);
                        self.ports(inputs);
                        self.ports(outputs);
                    }
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::FixtureNode(value) => {
                let DagNodeSpec { id, name, abbreviation, icon, x: _, y: _, width: _, height: _, operator_kind, properties, kind } = value;
                self.text(id);
                self.text(name);
                self.text(abbreviation);
                self.text(icon);
                if let Some(value) = operator_kind {
                    self.text(value);
                }
                self.push(DagRetirementOwner::Properties(properties));
                self.push(DagRetirementOwner::NodeKind(kind));
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::FixtureNodes { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::FixtureNodes { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::FixtureNodes { values, remaining_backing_bytes: 0 });
                }
                if let Some(value) = next {
                    self.push(DagRetirementOwner::FixtureNode(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::FixtureEdge(value) => {
                let DagFixtureEdge { id, source, target, route_style: _, properties } = value;
                self.text(id);
                self.text(source);
                self.text(target);
                self.push(DagRetirementOwner::Properties(properties));
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::FixtureEdges { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::FixtureEdges { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::FixtureEdges { values, remaining_backing_bytes: 0 });
                }
                if let Some(value) = next {
                    self.push(DagRetirementOwner::FixtureEdge(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::EngineNode(value) => {
                let Node { id: _, center: _, radius: _, width: _, height: _, shape: _, draggable: _, kind, label, properties } = value;
                if let Some(value) = kind {
                    self.text(value);
                }
                if let Some(value) = label {
                    self.text(value);
                }
                self.push(DagRetirementOwner::Properties(properties));
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::EngineHandle(value) => {
                let Handle { angle: _, id: _, node_id: _, radius: _, role: _, kind, properties } = value;
                if let Some(value) = kind {
                    self.text(value);
                }
                self.push(DagRetirementOwner::Properties(properties));
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::EngineSemantics(value) => {
                let ::graph::ElementSemantics { kind, properties } = value;
                if let Some(value) = kind {
                    self.text(value);
                }
                self.push(DagRetirementOwner::Properties(properties));
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::EngineEvent(value) => {
                match value {
                    BoardEvent::SelectionChanged { edge_ids, handle_ids, node_ids } => {
                        self.ids(edge_ids);
                        self.ids(handle_ids);
                        self.ids(node_ids);
                    }
                    BoardEvent::PreselectChanged { edge_ids, handle_ids, node_ids, removed_edge_ids, removed_handle_ids, removed_node_ids } => {
                        self.ids(edge_ids);
                        self.ids(handle_ids);
                        self.ids(node_ids);
                        self.ids(removed_edge_ids);
                        self.ids(removed_handle_ids);
                        self.ids(removed_node_ids);
                    }
                    BoardEvent::HoverChanged { .. } | BoardEvent::NodeMoved { .. } | BoardEvent::EdgeConnected { .. } | BoardEvent::EdgeRemoved { .. } => {}
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 }
            }
            DagRetirementOwner::EngineEvents { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::EngineEvents { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let next = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::EngineEvents { values, remaining_backing_bytes: 0 });
                }
                if let Some(value) = next {
                    self.push(DagRetirementOwner::EngineEvent(value));
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::Ids { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::Ids { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let _ = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::Ids { values, remaining_backing_bytes: 0 });
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
            DagRetirementOwner::OptionalIds { values, remaining_backing_bytes } => {
                let released_backing_bytes = dag_vec_backing_bytes(&values);
                let (mut values, credited_bytes) = match self.credit_backing(values, remaining_backing_bytes, maximum_bytes, |values, remaining_backing_bytes| DagRetirementOwner::OptionalIds { values, remaining_backing_bytes }) {
                    Ok(values) => values,
                    Err(step) => return step,
                };
                let _ = values.pop();
                let released_backing = values.is_empty();
                if !values.is_empty() {
                    self.push(DagRetirementOwner::OptionalIds { values, remaining_backing_bytes: 0 });
                }
                DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: if released_backing { released_backing_bytes } else { 0 } }
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.is_empty()
    }
}

impl Drop for DagPayloadRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("DAG payload retirement dropped with live owned payloads");
            }
            return;
        }
        unsafe { std::mem::ManuallyDrop::drop(&mut self.owners) };
    }
}

fn retire_empty_hash_map_backing<K, V>(values: &mut HashMap<K, V>, credited: &mut usize, maximum_bytes: usize) -> Option<DagRetirementStep> {
    if !values.is_empty() || values.capacity() == 0 {
        return None;
    }
    let released_bytes = values.capacity().saturating_mul(size_of::<(K, V)>());
    let remaining_bytes = released_bytes.saturating_sub(*credited);
    let credited_bytes = maximum_bytes.min(remaining_bytes);
    *credited = credited.saturating_add(credited_bytes);
    if *credited != released_bytes {
        return Some(DagRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 });
    }
    drop(std::mem::take(values));
    *credited = 0;
    Some(DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes })
}

fn retire_empty_hash_set_backing<T>(values: &mut HashSet<T>, credited: &mut usize, maximum_bytes: usize) -> Option<DagRetirementStep> {
    if !values.is_empty() || values.capacity() == 0 {
        return None;
    }
    let released_bytes = values.capacity().saturating_mul(size_of::<T>());
    let remaining_bytes = released_bytes.saturating_sub(*credited);
    let credited_bytes = maximum_bytes.min(remaining_bytes);
    *credited = credited.saturating_add(credited_bytes);
    if *credited != released_bytes {
        return Some(DagRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 });
    }
    drop(std::mem::take(values));
    *credited = 0;
    Some(DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes })
}

#[doc(hidden)]
pub struct DagHostRetirementState {
    fixture: DagFixture,
    engine: DagBoardEngine,
    node_id_map: HashMap<NodeId, usize>,
    handle_key_map: HashMap<HandleId, String>,
    handle_port_shape: HashMap<HandleId, PortShape>,
    handle_port_visible: HashMap<HandleId, bool>,
    edge_id_map: HashMap<EdgeId, String>,
    edge_engine_ids: Vec<Option<EdgeId>>,
    edge_route_style: HashMap<EdgeId, EdgeRouteStyle>,
    pending_port_insert: Option<(DagPortSide, String, usize)>,
    dimmed: HashSet<NodeId>,
    icon_paint_cache: graph::IconPaintCache,
    ghost_node: Option<DagNodeSpec>,
    pending_cluster_explode: Option<String>,
    pending_export_click: Option<String>,
    pending_open_instance_id: Option<String>,
    last_pointer_down_node_id: Option<String>,
    computing_active: Option<NodeId>,
    computing_stale: HashSet<NodeId>,
    node_eval_status: HashMap<NodeId, DagNodeEvalStatusKind>,
    unresolved_input_ports: HashSet<(NodeId, String)>,
    editing_note: Option<NoteEditState>,
    payload_retirement: DagPayloadRetirement,
    backing_credited_bytes: usize,
    released: bool,
}

/// 🧹️ Retained DAG host owner that releases one admitted graph item or bounded payload bytes per grant.
pub struct DagHostRetirement {
    state: std::mem::ManuallyDrop<DagHostRetirementState>,
}

impl std::ops::Deref for DagHostRetirement {
    type Target = DagHostRetirementState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl std::ops::DerefMut for DagHostRetirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl DagHostRetirement {
    pub fn new(host: DagHost) -> Self {
        let DagHost {
            fixture,
            engine,
            canvas_theme: _,
            width: _,
            height: _,
            dpr: _,
            last_screen_x: _,
            last_screen_y: _,
            node_id_map,
            handle_key_map,
            handle_port_shape,
            handle_port_visible,
            edge_id_map,
            edge_engine_ids,
            edge_route_style,
            widget_drag: _,
            pending_port_insert,
            last_logged_lod: _,
            dimmed,
            wheel_zoom_active: _,
            wheel_zoom_render_lod: _,
            automatic_lod: _,
            forced_draw_lod: _,
            grid_visible: _,
            grid_snap_enabled: _,
            grid_factor: _,
            icon_paint_cache,
            ghost_node,
            pending_cluster_explode,
            pending_export_click,
            pending_open_instance_id,
            last_pointer_down_at_ms: _,
            last_pointer_down_world: _,
            last_pointer_down_node_id,
            computing_active,
            computing_stale,
            computing_active_anim_phase: _,
            computing_stale_anim_phase: _,
            node_eval_status,
            unresolved_input_ports,
            editing_note,
            caret_visible: _,
            pan_anchor: _,
            minimap_widget_visible: _,
            minimap_widget_hovered: _,
            minimap_widget_drag: _,
        } = host;
        Self {
            state: std::mem::ManuallyDrop::new(DagHostRetirementState {
                fixture,
                engine,
                node_id_map,
                handle_key_map,
                handle_port_shape,
                handle_port_visible,
                edge_id_map,
                edge_engine_ids,
                edge_route_style,
                pending_port_insert,
                dimmed,
                icon_paint_cache,
                ghost_node,
                pending_cluster_explode,
                pending_export_click,
                pending_open_instance_id,
                last_pointer_down_node_id,
                computing_active,
                computing_stale,
                node_eval_status,
                unresolved_input_ports,
                editing_note,
                payload_retirement: DagPayloadRetirement::default(),
                backing_credited_bytes: 0,
                released: false,
            }),
        }
    }

    fn credit_owner(&mut self, owner: DagRetirementOwner, maximum_items: usize, maximum_bytes: usize) -> DagRetirementStep {
        self.payload_retirement.push(owner);
        self.payload_retirement.close_step(maximum_items, maximum_bytes)
    }

    #[doc(hidden)]
    pub fn retain_node_payload(&mut self, node: DagNodeSpec) {
        assert!(!self.released, "DAG retirement cannot admit a payload after terminal release");
        self.payload_retirement.push(DagRetirementOwner::FixtureNode(node));
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> DagRetirementStep {
        if self.released {
            return DagRetirementStep::Complete;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return DagRetirementStep::Blocked;
        }
        if !self.payload_retirement.terminal_is_empty() {
            return self.payload_retirement.close_step(maximum_items, maximum_bytes);
        }
        if let Some((_, value)) = self.engine.edge_semantics.pop_first() {
            return self.credit_owner(DagRetirementOwner::EngineSemantics(value), maximum_items, maximum_bytes);
        }
        if self.engine.events.capacity() != 0 {
            let values = std::mem::take(&mut self.engine.events);
            self.payload_retirement.engine_events(values);
            return self.payload_retirement.close_step(maximum_items, maximum_bytes);
        }
        if let Some((_, value)) = self.engine.handles.pop_first() {
            return self.credit_owner(DagRetirementOwner::EngineHandle(value), maximum_items, maximum_bytes);
        }
        if let Some((_, value)) = self.engine.nodes.pop_first() {
            return self.credit_owner(DagRetirementOwner::EngineNode(value), maximum_items, maximum_bytes);
        }
        if !self.engine.selection_options.method.is_empty() {
            let value = std::mem::take(&mut self.engine.selection_options.method);
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if !self.engine.selection_options.mode.is_empty() {
            let value = std::mem::take(&mut self.engine.selection_options.mode);
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if !self.engine.terminal_is_empty() {
            let _ = self.engine.close_step();
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        match self.engine.close_backing_page(maximum_bytes) {
            infinite::board::GraphEngineBackingRetirementStep::Blocked => return DagRetirementStep::Blocked,
            infinite::board::GraphEngineBackingRetirementStep::Pending { credited_bytes, released_bytes } => {
                return DagRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes };
            }
            infinite::board::GraphEngineBackingRetirementStep::Complete => {}
        }
        match self.icon_paint_cache.close_page(maximum_items, maximum_bytes) {
            graph::IconPaintRetirementStep::Blocked => return DagRetirementStep::Blocked,
            graph::IconPaintRetirementStep::Pending { released_items, credited_bytes, released_bytes } => {
                return DagRetirementStep::Pending { released_items, credited_bytes, released_bytes };
            }
            graph::IconPaintRetirementStep::Complete => {}
        }
        if !self.fixture.schema.is_empty() {
            let value = std::mem::take(&mut self.fixture.schema);
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if self.fixture.nodes.capacity() != 0 {
            let values = std::mem::take(&mut self.fixture.nodes);
            self.payload_retirement.fixture_nodes(values);
            return self.payload_retirement.close_step(maximum_items, maximum_bytes);
        }
        if self.fixture.edges.capacity() != 0 {
            let values = std::mem::take(&mut self.fixture.edges);
            self.payload_retirement.fixture_edges(values);
            return self.payload_retirement.close_step(maximum_items, maximum_bytes);
        }
        if self.edge_engine_ids.capacity() != 0 {
            let values = std::mem::take(&mut self.edge_engine_ids);
            self.payload_retirement.optional_ids(values);
            return self.payload_retirement.close_step(maximum_items, maximum_bytes);
        }
        if let Some((_, value, _)) = self.pending_port_insert.take() {
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(value) = self.pending_cluster_explode.take() {
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(value) = self.pending_export_click.take() {
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(value) = self.pending_open_instance_id.take() {
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(value) = self.last_pointer_down_node_id.take() {
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(value) = self.editing_note.take() {
            return self.credit_owner(DagRetirementOwner::Text(value.node_id), maximum_items, maximum_bytes);
        }
        if let Some(key) = self.node_id_map.keys().next().copied() {
            self.node_id_map.remove(&key);
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        if let Some(key) = self.handle_key_map.keys().next().copied() {
            let value = self.handle_key_map.remove(&key).expect("DAG handle key remains present");
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(key) = self.handle_port_shape.keys().next().copied() {
            self.handle_port_shape.remove(&key);
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        if let Some(key) = self.handle_port_visible.keys().next().copied() {
            self.handle_port_visible.remove(&key);
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        if let Some(key) = self.edge_id_map.keys().next().copied() {
            let value = self.edge_id_map.remove(&key).expect("DAG edge key remains present");
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(key) = self.edge_route_style.keys().next().copied() {
            self.edge_route_style.remove(&key);
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        if let Some(key) = self.dimmed.iter().next().copied() {
            self.dimmed.remove(&key);
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        if let Some(key) = self.computing_stale.iter().next().copied() {
            self.computing_stale.remove(&key);
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        if let Some(key) = self.node_eval_status.keys().next().copied() {
            self.node_eval_status.remove(&key);
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        if let Some((_, value)) = self.unresolved_input_ports.extract_if(|_| true).next() {
            return self.credit_owner(DagRetirementOwner::Text(value), maximum_items, maximum_bytes);
        }
        if let Some(value) = self.ghost_node.take() {
            return self.credit_owner(DagRetirementOwner::FixtureNode(value), maximum_items, maximum_bytes);
        }
        if self.computing_active.take().is_some() {
            return DagRetirementStep::Pending { released_items: 1, credited_bytes: 0, released_bytes: 0 };
        }
        let state = &mut *self.state;
        if let Some(step) = retire_empty_hash_map_backing(&mut state.node_id_map, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_map_backing(&mut state.handle_key_map, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_map_backing(&mut state.handle_port_shape, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_map_backing(&mut state.handle_port_visible, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_map_backing(&mut state.edge_id_map, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_map_backing(&mut state.edge_route_style, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_set_backing(&mut state.dimmed, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_set_backing(&mut state.computing_stale, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_map_backing(&mut state.node_eval_status, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        if let Some(step) = retire_empty_hash_set_backing(&mut state.unresolved_input_ports, &mut state.backing_credited_bytes, maximum_bytes) {
            return step;
        }
        debug_assert_eq!(self.backing_credited_bytes, 0);
        self.released = true;
        DagRetirementStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.released
            && self.engine.terminal_is_empty()
            && self.fixture.schema.is_empty()
            && self.fixture.nodes.is_empty()
            && self.fixture.edges.is_empty()
            && self.node_id_map.is_empty()
            && self.handle_key_map.is_empty()
            && self.handle_port_shape.is_empty()
            && self.handle_port_visible.is_empty()
            && self.edge_id_map.is_empty()
            && self.edge_engine_ids.is_empty()
            && self.edge_route_style.is_empty()
            && self.pending_port_insert.is_none()
            && self.dimmed.is_empty()
            && self.icon_paint_cache.terminal_is_empty()
            && self.ghost_node.is_none()
            && self.pending_cluster_explode.is_none()
            && self.pending_export_click.is_none()
            && self.pending_open_instance_id.is_none()
            && self.last_pointer_down_node_id.is_none()
            && self.computing_active.is_none()
            && self.computing_stale.is_empty()
            && self.node_eval_status.is_empty()
            && self.unresolved_input_ports.is_empty()
            && self.editing_note.is_none()
            && self.payload_retirement.terminal_is_empty()
            && self.backing_credited_bytes == 0
    }
}

impl Drop for DagHostRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("DagHostRetirement must reach terminal-empty before release");
            }
            return;
        }
        unsafe { std::mem::ManuallyDrop::drop(&mut self.state) };
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DagNodeEvalStatusKind {
    Ok,
    Stale,
    Queued,
    Computing,
    Error,
    Blocked,
}

struct MinimapWidgetLayout {
    panel: (f64, f64, f64, f64),
    world_min_x: f64,
    world_min_y: f64,
    scale: f64,
    map_origin_x: f64,
    map_origin_y: f64,
    viewport: (f64, f64, f64, f64),
}

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; pointer-event timestamps are a
// browser canvas concept the wasip2 guest has no pointer events to timestamp in the first place,
// so it shares the existing native no-op (`0.0`) rather than reading a WASI clock for no consumer.
fn pointer_event_now_ms() -> f64 {
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now()
    }
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        0.0
    }
}

/// 🧭️ Screen coordinates to world coordinates for the current viewport.
pub fn dag_screen_to_world(host: &DagHost, sx: f64, sy: f64) -> (f64, f64) {
    let point = host.screen_to_world_point(sx, sy);
    (point.x, point.y)
}

/// 🪟️ Takes a pending double-click open request for an app instance node.
pub fn dag_take_pending_open_instance_id(host: &mut DagHost) -> Option<String> {
    host.pending_open_instance_id.take()
}

fn canvas_color_with_alpha(color: canvas::Color, alpha: u8) -> canvas::Color {
    use canvas::Color;
    let rgba = color.to_rgba8();
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

#[derive(Clone, Copy)]
struct DagNodePaintChrome {
    is_dimmed: bool,
    is_selected: bool,
    is_highlighted: bool,
    is_hovered: bool,
    eval_status: DagNodeEvalStatusKind,
    body_fill_alpha: u8,
    ghost_tint: bool,
}

impl DagNodePaintChrome {
    fn ghost_preview() -> Self {
        Self { is_dimmed: false, is_selected: false, is_highlighted: false, is_hovered: false, eval_status: DagNodeEvalStatusKind::Ok, body_fill_alpha: 255, ghost_tint: true }
    }

    fn tint_highlighted(self) -> bool {
        self.is_highlighted || self.ghost_tint
    }

    fn has_interaction_chrome(self) -> bool {
        self.is_dimmed || self.is_selected || self.is_highlighted || self.is_hovered
    }
}

impl DagHost {
    pub fn default_demo() -> Self {
        Self::from_fixture(DagFixture::default())
    }

    pub fn from_fixture(fixture: DagFixture) -> Self {
        Self::from_fixture_with_layout(fixture, false)
    }

    /// 🌳️ Builds a host without running auto-layout (preserves node positions).
    pub fn from_fixture_without_layout(fixture: DagFixture) -> Self {
        Self::from_fixture(fixture)
    }

    /// ♻️ Rebuilds transient DAG state while retaining the exact owner of admitted icon paints.
    pub fn replace_fixture_without_layout(&mut self, fixture: DagFixture) {
        let mut next = Self::from_fixture_without_layout(fixture);
        std::mem::swap(&mut self.icon_paint_cache, &mut next.icon_paint_cache);
        *self = next;
    }

    fn from_fixture_with_layout(fixture: DagFixture, apply_layout: bool) -> Self {
        let mut host = Self {
            fixture,
            engine: DagBoardEngine::new(),
            canvas_theme: CanvasPalette::default(),
            width: 1,
            height: 1,
            dpr: 1.0,
            last_screen_x: 0.0,
            last_screen_y: 0.0,
            node_id_map: HashMap::new(),
            handle_key_map: HashMap::new(),
            handle_port_shape: HashMap::new(),
            handle_port_visible: HashMap::new(),
            edge_id_map: HashMap::new(),
            edge_engine_ids: Vec::new(),
            edge_route_style: HashMap::new(),
            widget_drag: None,
            pending_port_insert: None,
            last_logged_lod: Cell::new(-1),
            dimmed: HashSet::new(),
            wheel_zoom_active: false,
            wheel_zoom_render_lod: None,
            automatic_lod: true,
            forced_draw_lod: None,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: GRID_FACTOR_DEFAULT,
            icon_paint_cache: graph::IconPaintCache::new(),
            ghost_node: None,
            pending_cluster_explode: None,
            pending_export_click: None,
            pending_open_instance_id: None,
            last_pointer_down_at_ms: 0.0,
            last_pointer_down_world: (0.0, 0.0),
            last_pointer_down_node_id: None,
            computing_active: None,
            computing_stale: HashSet::new(),
            computing_active_anim_phase: Cell::new(0.0),
            computing_stale_anim_phase: Cell::new(0.0),
            node_eval_status: HashMap::new(),
            unresolved_input_ports: HashSet::new(),
            editing_note: None,
            caret_visible: true,
            pan_anchor: None,
            minimap_widget_visible: false,
            minimap_widget_hovered: false,
            minimap_widget_drag: None,
        };
        host.rebuild_engine_with_layout(apply_layout);
        host
    }

    fn engine_node_id_for_index(&self, idx: usize) -> Option<NodeId> {
        self.node_id_map.iter().find(|(_, &i)| i == idx).map(|(&nid, _)| nid)
    }

    fn node_id_for_widget_id(&self, widget_id: &str) -> Option<NodeId> {
        let idx = self.fixture.nodes.iter().position(|n| n.id == widget_id)?;
        self.engine_node_id_for_index(idx)
    }

    fn widget_id_for_node_id(&self, node_id: NodeId) -> Option<String> {
        self.widget_id_for_node_id_ref(node_id).map(str::to_owned)
    }

    fn widget_id_for_node_id_ref(&self, node_id: NodeId) -> Option<&str> {
        let idx = *self.node_id_map.get(&node_id)?;
        self.fixture.nodes.get(idx).map(|node| node.id.as_str())
    }

    fn is_node_hovered(&self, node_id: NodeId) -> bool {
        matches!(self.engine.hover, Some(hover) if hover == node_id)
    }

    fn resolve_minimap_hover_node_id(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x >= node.x - hw && world_x <= node.x + hw && world_y >= node.y - hh && world_y <= node.y + hh {
                return self.engine_node_id_for_index(idx);
            }
        }
        None
    }

    fn sync_minimap_pointer_hover(&mut self, world_x: f64, world_y: f64) {
        if !matches!(self.draw_lod_for_frame(), DagDrawLod::Minimap) {
            return;
        }
        if !matches!(self.engine.interaction, InteractionMode::Idle) {
            return;
        }
        self.engine.hover = self.resolve_minimap_hover_node_id(world_x, world_y);
    }

    fn is_node_selected(&self, node_id: NodeId) -> bool {
        self.engine.selection.node_ids.contains(&node_id)
    }

    fn is_node_preselected(&self, node_id: NodeId) -> bool {
        self.engine.preselect.node_ids.contains(&node_id)
    }

    fn is_node_preselect_removed(&self, node_id: NodeId) -> bool {
        self.engine.preselect_removed.node_ids.contains(&node_id)
    }

    fn is_preselect_active(&self) -> bool {
        matches!(self.engine.interaction, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. })
            || !self.engine.preselect.node_ids.is_empty()
            || !self.engine.preselect.handle_ids.is_empty()
            || !self.engine.preselect.edge_ids.is_empty()
    }

    fn node_interaction_chrome(&self, node_id: NodeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.is_node_preselected(node_id), self.is_node_preselect_removed(node_id), false);
        }
        (self.is_node_selected(node_id), false, self.is_node_hovered(node_id))
    }

    fn handle_interaction_chrome(&self, handle_id: HandleId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.handle_ids.contains(&handle_id), self.engine.preselect_removed.handle_ids.contains(&handle_id), false);
        }
        (self.engine.selection.handle_ids.contains(&handle_id), false, self.engine.hover == Some(handle_id))
    }

    fn edge_interaction_chrome(&self, edge_id: EdgeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.edge_ids.contains(&edge_id), self.engine.preselect_removed.edge_ids.contains(&edge_id), false);
        }
        (self.engine.selection.edge_ids.contains(&edge_id), false, self.engine.hover == Some(edge_id))
    }

    fn sync_camera_from_engine(&mut self) {
        let cam = self.engine.camera;
        self.fixture.camera = DagCamera { x: cam.x, y: cam.y, zoom: cam.zoom };
    }

    /// 🎯️ Selected fixture node ids from the engine selection snapshot.
    pub fn selected_node_ids(&self) -> Vec<String> {
        self.selected_node_id_refs().map(str::to_owned).collect()
    }

    pub fn selected_node_id_refs(&self) -> impl Iterator<Item = &str> {
        self.engine.selection.node_ids.iter().filter_map(|&node_id| self.widget_id_for_node_id_ref(node_id))
    }

    pub fn selected_node_count(&self) -> usize {
        self.engine.selection.node_ids.len()
    }

    pub fn bounded_interaction_projection(&self, revision: u64) -> Result<DagInteractionProjection, DagInteractionPlanFault> {
        if self.fixture.nodes.len() > DAG_INTERACTION_NODE_CAPACITY {
            return Err(DagInteractionPlanFault::NodeCredits);
        }
        let mut bytes = 0usize;
        for node in &self.fixture.nodes {
            if node.id.len() > 256 {
                return Err(DagInteractionPlanFault::StringCredits);
            }
            bytes = bytes.checked_add(node.id.len()).ok_or(DagInteractionPlanFault::StringCredits)?;
            if bytes > 16 * 1024 {
                return Err(DagInteractionPlanFault::StringCredits);
            }
        }
        let mut selected = [0; DAG_INTERACTION_WORD_CAPACITY];
        for node_id in &self.engine.selection.node_ids {
            if let Some(index) = self.node_id_map.get(node_id).copied() {
                dag_bit_set(&mut selected, index, true);
            }
        }
        let hover = self.engine.hover.and_then(|node_id| self.node_id_map.get(&node_id).copied()).and_then(|index| u16::try_from(index).ok());
        let camera = &self.fixture.camera;
        Ok(DagInteractionProjection { revision, camera: [camera.x, camera.y, camera.zoom], selected, hover, gesture: DagProjectionGesture::Idle })
    }

    pub fn derive_pointer_plan(&self, projection: DagInteractionProjection, intent: DagPointerIntent) -> Result<DagPointerPlan, DagInteractionPlanFault> {
        if projection.revision == u64::MAX || self.fixture.nodes.len() > DAG_INTERACTION_NODE_CAPACITY {
            return Err(DagInteractionPlanFault::NodeCredits);
        }
        let mut next = projection;
        next.revision = projection.revision + 1;
        let mut moves = [None; DAG_INTERACTION_NODE_CAPACITY];
        let mut move_len = 0u16;
        match intent.phase {
            DagPointerPhase::Down => self.derive_projection_down(&mut next, intent)?,
            DagPointerPhase::Move => self.derive_projection_move(&mut next, &mut moves, &mut move_len, intent)?,
            DagPointerPhase::Up => {
                self.derive_projection_move(&mut next, &mut moves, &mut move_len, intent)?;
                next.gesture = DagProjectionGesture::Idle;
            }
            DagPointerPhase::Leave => {
                next.gesture = DagProjectionGesture::Idle;
                next.hover = None;
            }
        }
        Ok(DagPointerPlan { expected_revision: projection.revision, previous_active: !matches!(projection.gesture, DagProjectionGesture::Idle), next, moves, move_len })
    }

    fn bounded_node_hit_index(&self, sx: f64, sy: f64) -> Result<Option<usize>, DagInteractionPlanFault> {
        if self.minimap_widget_pointer_hit(sx, sy).is_some() {
            return Err(DagInteractionPlanFault::Unsupported);
        }
        let world = self.screen_to_world_point(sx, sy);
        if self.port_insert_hit(world.x, world.y, self.fixture.camera.zoom).is_some() || self.world_hits_handle(world.x, world.y) || self.widget_hit_at(world.x, world.y).is_some() {
            return Err(DagInteractionPlanFault::Unsupported);
        }
        Ok(self.fixture_draggable_node_hit(world.x, world.y).and_then(|node_id| self.node_id_map.get(&node_id).copied()))
    }

    fn derive_projection_down(&self, next: &mut DagInteractionProjection, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
        if intent.pan || intent.button == 1 {
            next.gesture = DagProjectionGesture::Pan { start_x: intent.x, start_y: intent.y, camera: next.camera };
            return Ok(());
        }
        if intent.button != 0 || intent.alt {
            return Err(DagInteractionPlanFault::Unsupported);
        }
        let Some(index) = self.bounded_node_hit_index(intent.x, intent.y)? else {
            next.gesture = DagProjectionGesture::Select { start_x: intent.x, start_y: intent.y, initial: next.selected };
            return Ok(());
        };
        if intent.ctrl_or_meta {
            let selected = !dag_bit_contains(&next.selected, index);
            dag_bit_set(&mut next.selected, index, selected);
            if !selected {
                next.gesture = DagProjectionGesture::Idle;
                return Ok(());
            }
        } else if intent.shift {
            dag_bit_set(&mut next.selected, index, true);
        } else if !dag_bit_contains(&next.selected, index) {
            next.selected = [0; DAG_INTERACTION_WORD_CAPACITY];
            dag_bit_set(&mut next.selected, index, true);
        }
        next.hover = u16::try_from(index).ok();
        let mut starts = [None; DAG_INTERACTION_NODE_CAPACITY];
        let mut len = 0usize;
        for node_index in 0..self.fixture.nodes.len() {
            if !dag_bit_contains(&next.selected, node_index) {
                continue;
            }
            let node = &self.fixture.nodes[node_index];
            starts[len] = Some(DagNodeMove { index: node_index as u16, x: node.x, y: node.y });
            len += 1;
        }
        next.gesture = DagProjectionGesture::Drag { start_x: intent.x, start_y: intent.y, starts, len: len as u16 };
        Ok(())
    }

    fn derive_projection_move(&self, next: &mut DagInteractionProjection, moves: &mut [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY], move_len: &mut u16, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
        match next.gesture {
            DagProjectionGesture::Pan { start_x, start_y, camera } => {
                let zoom = camera[2].max(1e-9);
                next.camera = [camera[0] - (intent.x - start_x) / zoom, camera[1] - (intent.y - start_y) / zoom, camera[2]];
            }
            DagProjectionGesture::Drag { start_x, start_y, starts, len } => {
                let zoom = next.camera[2].max(1e-9);
                let dx = (intent.x - start_x) / zoom;
                let dy = (intent.y - start_y) / zoom;
                for index in 0..usize::from(len) {
                    let Some(start) = starts[index] else {
                        continue;
                    };
                    moves[index] = Some(DagNodeMove { index: start.index, x: start.x + dx, y: start.y + dy });
                }
                *move_len = len;
            }
            DagProjectionGesture::Select { start_x, start_y, initial } => {
                let min_x = start_x.min(intent.x);
                let max_x = start_x.max(intent.x);
                let min_y = start_y.min(intent.y);
                let max_y = start_y.max(intent.y);
                let mut selected = if intent.shift || intent.ctrl_or_meta { initial } else { [0; DAG_INTERACTION_WORD_CAPACITY] };
                for index in 0..self.fixture.nodes.len() {
                    let node = &self.fixture.nodes[index];
                    let screen = self.world_to_screen_point(node.x, node.y);
                    if screen.0 >= min_x && screen.0 <= max_x && screen.1 >= min_y && screen.1 <= max_y {
                        dag_bit_set(&mut selected, index, true);
                    }
                }
                next.selected = selected;
            }
            DagProjectionGesture::Idle => {
                next.hover = self.bounded_node_hit_index(intent.x, intent.y)?.and_then(|index| u16::try_from(index).ok());
            }
        }
        Ok(())
    }

    fn world_to_screen_point(&self, x: f64, y: f64) -> (f64, f64) {
        use canvas::camera::{world_to_screen, Camera, Viewport};
        let camera = Camera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        let point = world_to_screen(&camera, &viewport, canvas::Point::new(x, y));
        (point.x, point.y)
    }

    pub fn apply_pointer_plan(&mut self, plan: &DagPointerPlan) {
        self.set_camera(plan.next.camera[0], plan.next.camera[1], plan.next.camera[2]);
        for index in 0..usize::from(plan.move_len) {
            let Some(delta) = plan.moves[index] else {
                continue;
            };
            let index = usize::from(delta.index);
            if let Some(node) = self.fixture.nodes.get_mut(index) {
                node.x = delta.x;
                node.y = delta.y;
            }
            self.sync_fixture_node_center_to_engine(index);
        }
        self.engine.selection = Selection::default();
        for index in 0..self.fixture.nodes.len() {
            if dag_bit_contains(&plan.next.selected, index) {
                if let Some(node_id) = self.engine_node_id_for_index(index) {
                    self.engine.selection.node_ids.insert(node_id);
                }
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
        self.engine.hover = plan.next.hover.and_then(|index| self.engine_node_id_for_index(usize::from(index)));
    }

    pub fn projection_selected_id_refs<'a>(&'a self, projection: &'a DagInteractionProjection) -> impl Iterator<Item = &'a str> + 'a {
        self.fixture.nodes.iter().enumerate().filter(move |(index, _)| dag_bit_contains(&projection.selected, *index)).map(|(_, node)| node.id.as_str())
    }

    pub fn projection_hovered_id_ref<'a>(&'a self, projection: &DagInteractionProjection) -> Option<&'a str> {
        projection.hover.and_then(|index| self.fixture.nodes.get(usize::from(index))).map(|node| node.id.as_str())
    }

    pub fn pointer_plan_move(&self, plan: &DagPointerPlan, index: usize) -> Option<(&str, f64, f64)> {
        let delta = plan.moves.get(index).and_then(|delta| *delta)?;
        self.fixture.nodes.get(usize::from(delta.index)).map(|node| (node.id.as_str(), delta.x, delta.y))
    }

    /// 🔗️ Selected fixture edge ids (synapse ids) from the engine selection snapshot.
    pub fn selected_edge_ids(&self) -> Vec<String> {
        self.engine.selection.edge_ids.iter().filter_map(|&eid| self.edge_id_map.get(&eid).cloned()).collect()
    }

    /// 🎯️ Nodes, edges, and handles in the current selection as JSON (`nodes`, `edges`, `🐙️handles`).
    pub fn selection_domains_json(&self) -> String {
        #[derive(ToValue, FromValue)]
        struct Domains {
            nodes: Vec<String>,
            edges: Vec<String>,
            handles: Vec<String>,
        }
        let handles: Vec<String> = self.selected_channels().into_iter().map(|channel| format!("{}@{}", channel.widget_id, channel.port)).collect();
        os_pack::json::to_json_string(&Domains { nodes: self.selected_node_ids(), edges: self.selected_edge_ids(), handles })
    }

    fn apply_selection_domains(&mut self, nodes: &[String], edges: &[String], handles: &[String]) {
        self.engine.selection = Selection::default();
        for widget_id in nodes {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.engine.selection.node_ids.insert(nid);
            }
        }
        for edge_id in edges {
            for (&eid, synapse_id) in &self.edge_id_map {
                if synapse_id == edge_id {
                    self.engine.selection.edge_ids.insert(eid);
                    break;
                }
            }
        }
        for handle_key in handles {
            if let Some((node_id, port_id)) = handle_key.split_once('@') {
                if let Some(hid) = self.handle_id_for_port(node_id, port_id) {
                    self.engine.selection.handle_ids.insert(hid);
                }
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
    }

    /// ✅️ Replaces selection from domain JSON (`{ nodes, edges, handles }`) or a legacy node-id array.
    pub fn set_selection_domains_json(&mut self, json: &str) {
        #[derive(Default, ToValue, FromValue)]
        struct Domains {
            nodes: Vec<String>,
            edges: Vec<String>,
            handles: Vec<String>,
        }
        if let Ok(domains) = os_pack::json::from_json_str::<Domains>(json) {
            self.apply_selection_domains(&domains.nodes, &domains.edges, &domains.handles);
            return;
        }
        let ids: Vec<String> = os_pack::json::from_json_str(json).unwrap_or_default();
        self.set_selection(&ids);
    }

    /// ✅️ Whether the engine has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        !self.engine.selection.node_ids.is_empty() || !self.engine.selection.edge_ids.is_empty() || !self.engine.selection.handle_ids.is_empty()
    }

    /// 🖱️ Hovered fixture widget id for node body hover, or parent widget when a channel handle is hovered at detail LOD.
    pub fn hovered_node_id(&self) -> Option<String> {
        self.hovered_node_id_ref().map(str::to_owned)
    }

    pub fn hovered_node_id_ref(&self) -> Option<&str> {
        let hover = self.engine.hover?;
        if self.node_id_map.contains_key(&hover) {
            return self.widget_id_for_node_id_ref(hover);
        }
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            if let Some(handle) = self.engine.handles.get(&hover) {
                return self.widget_id_for_node_id_ref(handle.node_id);
            }
        }
        None
    }

    fn decode_channel_ref(&self, target: u64) -> Option<DagChannelRef> {
        if self.node_id_map.contains_key(&target) {
            return None;
        }
        let key = self.handle_key_map.get(&target)?;
        let (node_id, port_id) = key.split_once('@')?;
        let node = self.fixture.nodes.iter().find(|entry| entry.id == node_id)?;
        let direction = if node.inputs().iter().any(|port| port.id == port_id) {
            "in"
        } else if node.outputs().iter().any(|port| port.id == port_id) {
            "out"
        } else {
            return None;
        };
        Some(DagChannelRef { widget_id: node_id.to_string(), port: port_id.to_string(), direction: direction.to_string() })
    }

    /// 🔌️ Hovered fixture channel when the pointer is over a port row or handle.
    pub fn hovered_channel(&self) -> Option<DagChannelRef> {
        let hover = self.engine.hover?;
        self.decode_channel_ref(hover)
    }

    /// 🔌️ Selected fixture channels from handle picks in the current selection snapshot.
    pub fn selected_channels(&self) -> Vec<DagChannelRef> {
        self.engine.selection.handle_ids.iter().filter_map(|&handle_id| self.decode_channel_ref(handle_id)).collect()
    }

    /// 🔌️ Selected fixture channels as JSON.
    pub fn selected_channels_json(&self) -> String {
        os_pack::json::to_json_string(&self.selected_channels())
    }

    /// 🔌️ Hovered fixture channel as JSON, or `null`.
    pub fn hovered_channel_json(&self) -> String {
        match self.hovered_channel() {
            Some(channel) => os_pack::json::to_json_string(&channel),
            None => "null".into(),
        }
    }

    /// @emoji 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        #[derive(ToValue, FromValue)]
        struct Row {
            domain: String,
            id: String,
            generality: u32,
            #[value(default, skip_serializing_if = "Option::is_none")]
            label: Option<String>,
        }
        let world = self.screen_to_world_point(sx, sy);
        let targets = self.engine.hit_test_pick_targets(world);
        let rows: Vec<Row> = targets
            .into_iter()
            .filter_map(|target| {
                let row = match target.domain.as_str() {
                    "node" => self.widget_id_for_node_id(target.id).map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "edge" => self.edge_id_map.get(&target.id).cloned().map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "handle" => self.decode_channel_ref(target.id).map(|channel| Row {
                        domain: "handle".into(),
                        id: format!("{}@{}", channel.widget_id, channel.port),
                        generality: target.generality,
                        label: Some(format!("{} · {}", channel.widget_id, channel.port)),
                    }),
                    _ => None,
                };
                row
            })
            .collect();
        os_pack::json::to_json_string(&rows)
    }

    /// ✅️ Replaces node selection from fixture widget ids.
    pub fn set_selection(&mut self, widget_ids: &[String]) {
        self.engine.selection = Selection::default();
        for widget_id in widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.engine.selection.node_ids.insert(nid);
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
    }

    /// 🎯️ Configures rectangle/lasso area-select behavior.
    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_handles: bool, select_edges: bool) {
        self.engine.set_selection_options(method, mode, select_nodes, select_handles, select_edges);
    }

    /// 🧿️ Screen-space marquee overlay points for the shared selection overlay.
    pub fn selection_preview_points_json(&self) -> String {
        let points: Vec<[f64; 2]> = self.engine.selection_preview_points().iter().map(|p| [p.x, p.y]).collect();
        os_pack::json::to_json_string(&points)
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.engine.selection_preview_crossing()
    }

    pub fn selection_preview_method(&self) -> &str {
        self.engine.selection_preview_method()
    }

    /// 👁️ Preselected widget ids during an in-flight marquee gesture.
    pub fn preselect_widget_ids(&self) -> Vec<String> {
        self.engine.preselect.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// 👁️ Widget ids highlighted as marquee-exit candidates.
    pub fn preselect_removed_widget_ids(&self) -> Vec<String> {
        self.engine.preselect_removed.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ↩️ Cancels an in-flight area select and restores the pre-drag selection.
    pub fn cancel_area_select(&mut self) -> bool {
        self.engine.cancel_area_select()
    }

    // #region 🔖️MinimapWidget
    /// 🗺️ Toggles the bottom-right flow minimap navigator.
    pub fn set_minimap_widget_visible(&mut self, visible: bool) {
        self.minimap_widget_visible = visible;
    }

    fn minimap_widget_content_bounds(&self) -> Option<WorldBox> {
        if self.fixture.nodes.is_empty() {
            return None;
        }
        let pad = ui_styling::metrics::dag::MINIMAP_WIDGET_CONTENT_PAD;
        let mut union = Self::dag_node_world_bounds(&self.fixture.nodes[0]);
        for node in self.fixture.nodes.iter().skip(1) {
            let b = Self::dag_node_world_bounds(node);
            union.min_x = union.min_x.min(b.min_x);
            union.min_y = union.min_y.min(b.min_y);
            union.max_x = union.max_x.max(b.max_x);
            union.max_y = union.max_y.max(b.max_y);
        }
        Some(WorldBox { min_x: union.min_x - pad, min_y: union.min_y - pad, max_x: union.max_x + pad, max_y: union.max_y + pad })
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::content_fully_visible` — pure layout math relocated there
    /// (see `.🧬semio/🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`).
    fn minimap_camera_fully_shows_content(&self, content: &WorldBox, viewport_w: u32, viewport_h: u32) -> bool {
        let cam = &self.fixture.camera;
        ui_wgpu::wgpu::minimap::content_fully_visible(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, 12.0)
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::layout` — see `minimap_camera_fully_shows_content` above.
    fn minimap_widget_layout(&self, viewport_w: u32, viewport_h: u32) -> Option<MinimapWidgetLayout> {
        if !self.minimap_widget_visible {
            return None;
        }
        let content = self.minimap_widget_content_bounds()?;
        if self.minimap_camera_fully_shows_content(&content, viewport_w, viewport_h) {
            return None;
        }
        let w = ui_styling::metrics::dag::MINIMAP_WIDGET_WIDTH;
        let h = ui_styling::metrics::dag::MINIMAP_WIDGET_HEIGHT;
        let margin = ui_styling::metrics::dag::MINIMAP_WIDGET_MARGIN;
        let ratio = ui_styling::metrics::dag::MINIMAP_WIDGET_MAX_CONTENT_RATIO;
        let cam = &self.fixture.camera;
        let layout =
            ui_wgpu::wgpu::minimap::layout(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, w, h, margin, ratio);
        Some(MinimapWidgetLayout { panel: layout.panel, world_min_x: layout.world_min_x, world_min_y: layout.world_min_y, scale: layout.scale, map_origin_x: layout.map_origin_x, map_origin_y: layout.map_origin_y, viewport: layout.viewport })
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::screen_to_world` — see `minimap_camera_fully_shows_content` above.
    fn minimap_widget_screen_to_world(&self, layout: &MinimapWidgetLayout, sx: f64, sy: f64) -> (f64, f64) {
        ui_wgpu::wgpu::minimap::screen_to_world(layout.map_origin_x, layout.map_origin_y, layout.world_min_x, layout.world_min_y, layout.scale, sx, sy)
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::point_in_rect` — see `minimap_camera_fully_shows_content` above.
    fn minimap_widget_point_in_rect(rect: (f64, f64, f64, f64), sx: f64, sy: f64) -> bool {
        ui_wgpu::wgpu::minimap::point_in_rect(rect, sx, sy)
    }

    fn minimap_widget_pointer_hit(&self, sx: f64, sy: f64) -> Option<(MinimapWidgetLayout, bool)> {
        let layout = self.minimap_widget_layout(self.width, self.height)?;
        if !Self::minimap_widget_point_in_rect(layout.panel, sx, sy) {
            return None;
        }
        let on_viewport = Self::minimap_widget_point_in_rect(layout.viewport, sx, sy);
        Some((layout, on_viewport))
    }

    fn minimap_widget_cursor_hint(&self) -> Option<&'static str> {
        if self.minimap_widget_drag.is_some() {
            return Some("grabbing");
        }
        if let Some((layout, on_viewport)) = self.minimap_widget_pointer_hit(self.last_screen_x, self.last_screen_y) {
            let _ = layout;
            return Some(if on_viewport { "grab" } else { "pointer" });
        }
        None
    }

    fn minimap_widget_json(&self) -> Option<Value> {
        let layout = self.minimap_widget_layout(self.width, self.height)?;
        let (x0, y0, x1, y1) = layout.panel;
        Some(os_pack::json::object([
            ("x".to_string(), Value::from(x0)),
            ("y".to_string(), Value::from(y0)),
            ("width".to_string(), Value::from(x1 - x0)),
            ("height".to_string(), Value::from(y1 - y0)),
            ("cursor".to_string(), Value::from(self.minimap_widget_cursor_hint())),
        ]))
    }

    fn paint_minimap_widget(&self, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32) {
        let Some(layout) = self.minimap_widget_layout(viewport_w, viewport_h) else {
            return;
        };
        use canvas::{Affine, FillRule, Rect, RoundedRect, RoundedRectRadii, Stroke};
        use ui_styling::strokes;
        let theme = &self.canvas_theme;
        let aff = Affine::IDENTITY;
        let radius = ui_styling::metrics::dag::MINIMAP_WIDGET_RADIUS;
        let (px0, py0, px1, py1) = layout.panel;
        let panel = RoundedRect::new(Rect::new(px0, py0, px1, py1), RoundedRectRadii::new(radius, radius, radius, radius));
        scene.fill(FillRule::NonZero, aff, theme.minimap_widget_panel_fill, None, &panel);
        scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_PANEL), aff, theme.minimap_widget_panel_stroke, None, &panel);
        let node_min = ui_styling::metrics::dag::MINIMAP_WIDGET_NODE_MIN_SIZE;
        let lod = DagDrawLod::Minimap;
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let node = node.as_ref();
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.node_interaction_chrome(nid));
            let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) else {
                continue;
            };
            let hw = (node.width * layout.scale * 0.5).max(node_min * 0.5);
            let hh = (node.height * layout.scale * 0.5).max(node_min * 0.5);
            let cx = layout.map_origin_x + (node.x - layout.world_min_x) * layout.scale;
            let cy = layout.map_origin_y + (node.y - layout.world_min_y) * layout.scale;
            let rect = Rect::new(cx - hw, cy - hh, cx + hw, cy + hh);
            scene.fill(FillRule::NonZero, aff, fill, None, &rect);
        }
        let (vx0, vy0, vx1, vy1) = layout.viewport;
        let view_rect = Rect::new(vx0, vy0, vx1, vy1);
        let active = self.minimap_widget_hovered || self.minimap_widget_drag.is_some();
        scene.fill(FillRule::NonZero, aff, theme.minimap_widget_viewport_fill, None, &view_rect);
        let stroke_c = if active { theme.minimap_widget_viewport_stroke_hovered } else { theme.minimap_widget_viewport_stroke };
        scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_VIEWPORT), aff, stroke_c, None, &view_rect);
    }
    // #endregion 🔖️MinimapWidget

    // #region 🔖️SelectionAlign
    fn dag_node_world_bounds(node: &DagNodeSpec) -> WorldBox {
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        WorldBox { min_x: node.x - hw, min_y: node.y - hh, max_x: node.x + hw, max_y: node.y + hh }
    }

    fn selected_fixture_nodes(&self) -> Vec<(usize, DagNodeSpec)> {
        let ids = self.selected_node_ids();
        ids.into_iter().filter_map(|id| self.fixture.nodes.iter().enumerate().find(|(_, node)| node.id == id).map(|(idx, node)| (idx, node.clone()))).collect()
    }

    fn sync_fixture_node_center_to_engine(&mut self, idx: usize) {
        let node = &self.fixture.nodes[idx];
        let Some(nid) = self.node_id_for_widget_id(&node.id) else {
            return;
        };
        if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
            engine_node.center = canvas::Point::new(node.x, node.y);
        }
    }

    /// 📦️ Screen-space union bounds of the current node selection for DOM chrome overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        let selected = self.selected_fixture_nodes();
        if selected.is_empty() {
            return "null".into();
        }
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let pad_world = 4.0 / self.fixture.camera.zoom.max(0.05);
        let mut corners = Vec::new();
        for (_, node) in &selected {
            let hw = node.width * 0.5 + pad_world;
            let hh = node.height * 0.5 + pad_world;
            corners.push(Point::new(node.x - hw, node.y - hh));
            corners.push(Point::new(node.x + hw, node.y + hh));
        }
        let Some(bounds) = world_box_from_points(&corners) else {
            return "null".into();
        };
        let cam = CanvasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let tl = world_to_screen(&cam, &viewport, Point::new(bounds.min_x, bounds.min_y));
        let br = world_to_screen(&cam, &viewport, Point::new(bounds.max_x, bounds.max_y));
        os_pack::json::to_string(&os_pack::json::object([
            ("x".to_string(), Value::from(tl.x)),
            ("y".to_string(), Value::from(tl.y)),
            ("width".to_string(), Value::from((br.x - tl.x).max(1.0))),
            ("height".to_string(), Value::from((br.y - tl.y).max(1.0))),
        ]))
    }

    /// @emoji 🎯️ Screen-space geometry (canvas-local px) for a live entity in the shell's pick-target
    /// grammar (`domain`: `"node"` | `"handle"` | `"edge"`; `id`: a node's widget id, `"widgetId@port"`
    /// for a handle, or a mirrored edge id — `"*"` picks whichever matching entity's screen anchor is
    /// nearest the viewport center) — powers introduction-demonstration semantic targeting
    /// (`IntroductionPoint::Entity`/`Curve`). Never errors: an unresolved domain/id returns
    /// `{"visible":false}`.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        #[derive(ToValue, FromValue)]
        struct EntityGeometry {
            visible: bool,
            #[value(default, skip_serializing_if = "Option::is_none")]
            x: Option<f64>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            y: Option<f64>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            rect: Option<[f64; 4]>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            polyline: Option<Vec<[f64; 2]>>,
        }
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let unresolved = EntityGeometry { visible: false, x: None, y: None, rect: None, polyline: None };
        let cam = CanvasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let viewport_center = (self.width as f64 * 0.5, self.height as f64 * 0.5);

        let world_rect_to_screen = |min_x: f64, min_y: f64, max_x: f64, max_y: f64| -> ([f64; 4], (f64, f64)) {
            let tl = world_to_screen(&cam, &viewport, Point::new(min_x, min_y));
            let br = world_to_screen(&cam, &viewport, Point::new(max_x, max_y));
            ([tl.x, tl.y, (br.x - tl.x).max(1.0), (br.y - tl.y).max(1.0)], ((tl.x + br.x) * 0.5, (tl.y + br.y) * 0.5))
        };
        let handle_world_bounds = |widget_id: &str, port: &str| -> Option<(f64, f64, f64, f64)> {
            let node = self.fixture.nodes.iter().find(|node| node.id == widget_id)?;
            if let Some(index) = node.inputs().iter().position(|candidate| candidate.id == port) {
                return input_port_row_hit_bounds(node, index);
            }
            let index = node.outputs().iter().position(|candidate| candidate.id == port)?;
            output_port_row_hit_bounds(node, index)
        };
        let nearest_center_screen = |candidates: &[(f64, f64, f64, f64)]| -> Option<(f64, f64, f64, f64)> {
            let mut best: Option<((f64, f64, f64, f64), f64)> = None;
            for &bounds in candidates {
                let (_, screen_center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
                let distance = (screen_center.0 - viewport_center.0).hypot(screen_center.1 - viewport_center.1);
                if best.is_none_or(|(_, best_distance)| distance < best_distance) {
                    best = Some((bounds, distance));
                }
            }
            best.map(|(bounds, _)| bounds)
        };

        let bounds_result: Option<(f64, f64, f64, f64)> = match domain {
            "node" => {
                if id == "*" {
                    let all: Vec<(f64, f64, f64, f64)> = self
                        .fixture
                        .nodes
                        .iter()
                        .map(|node| {
                            let b = Self::dag_node_world_bounds(node);
                            (b.min_x, b.min_y, b.max_x, b.max_y)
                        })
                        .collect();
                    nearest_center_screen(&all)
                } else {
                    self.fixture.nodes.iter().find(|node| node.id == id).map(|node| {
                        let b = Self::dag_node_world_bounds(node);
                        (b.min_x, b.min_y, b.max_x, b.max_y)
                    })
                }
            }
            "handle" => {
                if id == "*" {
                    let mut all: Vec<(f64, f64, f64, f64)> = Vec::new();
                    for node in &self.fixture.nodes {
                        for port in node.inputs().iter().chain(node.outputs().iter()) {
                            if let Some(bounds) = handle_world_bounds(&node.id, &port.id) {
                                all.push(bounds);
                            }
                        }
                    }
                    nearest_center_screen(&all)
                } else {
                    id.split_once('@').and_then(|(widget_id, port)| handle_world_bounds(widget_id, port))
                }
            }
            "edge" => {
                let edge = if id == "*" { self.fixture.edges.first() } else { self.fixture.edges.iter().find(|edge| edge.id == id) };
                let Some(edge) = edge else { return os_pack::json::to_json_string(&unresolved) };
                let Some((source_widget, source_port)) = edge.source.split_once('@') else { return os_pack::json::to_json_string(&unresolved) };
                let Some((target_widget, target_port)) = edge.target.split_once('@') else { return os_pack::json::to_json_string(&unresolved) };
                let Some(source_bounds) = handle_world_bounds(source_widget, source_port) else { return os_pack::json::to_json_string(&unresolved) };
                let Some(target_bounds) = handle_world_bounds(target_widget, target_port) else { return os_pack::json::to_json_string(&unresolved) };
                let (_, source_center) = world_rect_to_screen(source_bounds.0, source_bounds.1, source_bounds.2, source_bounds.3);
                let (_, target_center) = world_rect_to_screen(target_bounds.0, target_bounds.1, target_bounds.2, target_bounds.3);
                let midpoint = ((source_center.0 + target_center.0) * 0.5, (source_center.1 + target_center.1) * 0.5);
                return os_pack::json::to_json_string(&EntityGeometry { visible: true, x: Some(midpoint.0), y: Some(midpoint.1), rect: None, polyline: Some(vec![[source_center.0, source_center.1], [target_center.0, target_center.1]]) });
            }
            _ => None,
        };

        let Some(bounds) = bounds_result else { return os_pack::json::to_json_string(&unresolved) };
        let (rect, center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
        os_pack::json::to_json_string(&EntityGeometry { visible: true, x: Some(center.0), y: Some(center.1), rect: Some(rect), polyline: None })
    }

    /// 📐️ Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), DagError> {
        use canvas::Point;
        let mut selected = self.selected_fixture_nodes();
        if selected.is_empty() {
            return Ok(());
        }
        match mode {
            "alignLeft" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_left = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.x = min_left + node.width * 0.5;
                }
            }
            "alignRight" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_right = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.x = max_right - node.width * 0.5;
                }
            }
            "alignTop" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_top = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.y = min_top + node.height * 0.5;
                }
            }
            "alignBottom" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_bottom = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.y = max_bottom - node.height * 0.5;
                }
            }
            "alignHorizontal" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = Self::dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_x = (union.min_x + union.max_x) * 0.5;
                for (_, node) in &mut selected {
                    node.x = center_x;
                }
            }
            "alignVertical" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = Self::dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_y = (union.min_y + union.max_y) * 0.5;
                for (_, node) in &mut selected {
                    node.y = center_y;
                }
            }
            "distributeHorizontal" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap_or(std::cmp::Ordering::Equal));
                let left = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                let right = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                let total_width: f64 = selected.iter().map(|(_, node)| node.width).sum();
                let raw_gap = (right - left - total_width) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = left;
                for (_, node) in &mut selected {
                    node.x = cursor + node.width * 0.5;
                    cursor += node.width + gap;
                }
            }
            "distributeVertical" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(std::cmp::Ordering::Equal));
                let top = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                let bottom = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                let total_height: f64 = selected.iter().map(|(_, node)| node.height).sum();
                let raw_gap = (bottom - top - total_height) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = top;
                for (_, node) in &mut selected {
                    node.y = cursor + node.height * 0.5;
                    cursor += node.height + gap;
                }
            }
            other => return Err(DagError::UnknownAlignMode(other.to_string())),
        }
        for (idx, node) in selected {
            self.fixture.nodes[idx].x = node.x;
            self.fixture.nodes[idx].y = node.y;
            self.sync_fixture_node_center_to_engine(idx);
        }
        Ok(())
    }
    // #endregion 🔖️SelectionAlign

    /// 📍️ Sets a fixture widget position in both the fixture and engine snapshots.
    pub fn set_widget_position(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), DagError> {
        let idx = self.fixture.nodes.iter().position(|node| node.id == widget_id).ok_or_else(|| DagError::UnknownWidget(widget_id.to_string()))?;
        self.fixture.nodes[idx].x = x;
        self.fixture.nodes[idx].y = y;
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return Ok(());
        };
        if let Some(node) = self.engine.nodes.get_mut(&nid) {
            node.center = canvas::Point::new(x, y);
        }
        Ok(())
    }

    /// 🗑️ Deletes the current selection from the fixture.
    pub fn delete_selected(&mut self) {
        let widget_ids = self.selected_node_ids();
        self.engine.delete_selection();
        self.fixture.nodes.retain(|node| !widget_ids.contains(&node.id));
        self.sync_edges_from_engine();
        self.rebuild_engine_with_layout(false);
    }

    /// ⌨️ Selects every fixture node id.
    pub fn select_all_node_ids(&self) -> Vec<String> {
        self.fixture.nodes.iter().map(|node| node.id.clone()).collect()
    }

    pub fn select_all(&mut self) {
        self.engine.select_all();
    }

    /// 🖱️ Sets hover to a fixture widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        let next = widget_id.and_then(|id| self.node_id_for_widget_id(id));
        if self.engine.hover != next {
            self.engine.hover = next;
        }
    }

    /// 🔌️ Sets hover to a fixture channel handle, falling back to node hover below channel LOD.
    pub fn set_hover_channel(&mut self, widget_id: Option<&str>, port_id: Option<&str>) {
        let Some(widget_id) = widget_id else {
            self.set_hover(None);
            return;
        };
        if let Some(port_id) = port_id {
            if self.draw_lod_for_frame().uses_channel_row_pick() {
                if let Some(hid) = self.handle_id_for_port(widget_id, port_id) {
                    if self.engine.hover != Some(hid) {
                        self.engine.hover = Some(hid);
                    }
                    return;
                }
            }
        }
        self.set_hover(Some(widget_id));
    }

    /// 🔌️ Replaces channel handle selection from fixture channel JSON, falling back to node selection below channel LOD.
    pub fn set_selected_channels_json(&mut self, json: &str) {
        let channels: Vec<DagChannelRef> = os_pack::json::from_json_str(json).unwrap_or_default();
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            let mut selection = Selection::default();
            for channel in channels {
                if let Some(hid) = self.handle_id_for_port(&channel.widget_id, &channel.port) {
                    selection.handle_ids.insert(hid);
                }
            }
            self.engine.selection = selection;
            self.engine.preselect = Selection::default();
            self.engine.preselect_removed = Selection::default();
            return;
        }
        let widget_ids: Vec<String> = channels.into_iter().map(|channel| channel.widget_id).collect();
        self.set_selection(&widget_ids);
    }

    /// 🌫️ Marks preview-off nodes as dimmed on the canvas.
    pub fn set_dimmed(&mut self, widget_ids: &[String]) {
        self.dimmed.clear();
        for widget_id in widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.dimmed.insert(nid);
            }
        }
    }

    /// ⚙️ Marks one actively computing widget and downstream widgets as stale.
    pub fn set_computing_progress(&mut self, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
        self.computing_active = active_widget_id.and_then(|widget_id| self.node_id_for_widget_id(widget_id));
        self.computing_stale.clear();
        for widget_id in stale_widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                if self.computing_active != Some(nid) {
                    self.computing_stale.insert(nid);
                }
            }
        }
    }

    /// ✅️ Clears evaluating chrome from all nodes.
    pub fn clear_computing(&mut self) {
        self.computing_active = None;
        self.computing_stale.clear();
        self.node_eval_status.clear();
        self.unresolved_input_ports.clear();
    }

    /// 🚦 Applies per-widget eval status from flow `statusJson` (widget id → `{ status, … }`).
    pub fn set_node_statuses_from_json(&mut self, json: &str) {
        self.computing_active = None;
        self.computing_stale.clear();
        self.node_eval_status.clear();
        self.unresolved_input_ports.clear();
        let Ok(value) = os_pack::json::parse(json) else {
            return;
        };
        let Some(map) = value.as_object() else {
            return;
        };
        for (widget_id, entry) in map {
            let Some(nid) = self.node_id_for_widget_id(widget_id) else {
                continue;
            };
            let status = entry.get("status").and_then(|value| value.as_str()).unwrap_or("ok");
            match status {
                "computing" => {
                    self.computing_active = Some(nid);
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Computing);
                }
                "queued" => {
                    self.computing_stale.insert(nid);
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Queued);
                }
                "stale" => {
                    self.computing_stale.insert(nid);
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Stale);
                }
                "error" => {
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Error);
                }
                "blocked" => {
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Blocked);
                    if let Some(ports) = entry.get("ports").and_then(|value| value.as_array()) {
                        for port in ports.iter().filter_map(|value| value.as_str()) {
                            self.unresolved_input_ports.insert((nid, port.to_string()));
                        }
                    }
                }
                _ => {
                    self.node_eval_status.insert(nid, DagNodeEvalStatusKind::Ok);
                }
            }
        }
    }

    fn tick_computing_animation(&self) {
        if self.computing_active.is_some() {
            let next = (self.computing_active_anim_phase.get() + 0.02) % 1.0;
            self.computing_active_anim_phase.set(next);
        }
        if !self.computing_stale.is_empty() {
            let next = (self.computing_stale_anim_phase.get() + 0.008) % 1.0;
            self.computing_stale_anim_phase.set(next);
        }
    }

    /// 📋️ Preview-off fixture node ids currently dimmed on the canvas.
    pub fn dimmed_node_ids(&self) -> Vec<String> {
        self.dimmed.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ➕️ Returns and clears a pending variadic insert request from the last pointer down.
    pub fn take_pending_port_insert(&mut self) -> Option<(DagPortSide, String, usize)> {
        self.pending_port_insert.take()
    }

    /// 🎯️ Hit-tests variadic `+` controls; returns side, node id, and insert index.
    pub fn port_insert_hit(&self, world_x: f64, world_y: f64, zoom: f64) -> Option<(DagPortSide, String, usize)> {
        if zoom < DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
            return None;
        }
        for node in self.fixture.nodes.iter().rev() {
            if node.variadic_inputs() {
                let inputs = node.inputs();
                let hw = node.width * 0.5;
                let row = inputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, hit_x1, y1) = {
                    let (x0, y0, _x1, y1) = channel_row_bounds(node, row);
                    (x0, y0, node.x - hw * 0.5, y1)
                };
                if point_in_rect(world_x, world_y, x0, y0, hit_x1, y1) {
                    return Some((DagPortSide::Input, node.id.clone(), inputs.len()));
                }
            }
            if node.variadic_outputs() {
                let outputs = node.outputs();
                let row = outputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, x1, y1) = channel_row_bounds(node, row);
                let hit_x0 = computation_output_column_x_bounds(node).map_or(x0, |(left, _)| left);
                if point_in_rect(world_x, world_y, hit_x0, y0, x1, y1) {
                    return Some((DagPortSide::Output, node.id.clone(), outputs.len()));
                }
            }
        }
        None
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    /// 👻️ Sets or clears the placement preview node painted with the normal LOD path.
    pub fn set_ghost_node(&mut self, node: Option<DagNodeSpec>) {
        self.ghost_node = node;
    }

    pub fn ghost_node(&self) -> Option<&DagNodeSpec> {
        self.ghost_node.as_ref()
    }

    pub fn automatic_lod(&self) -> bool {
        self.automatic_lod
    }

    pub fn forced_draw_lod_label(&self) -> Option<&'static str> {
        self.forced_draw_lod.map(|lod| lod.label())
    }

    /// 🔍️ Pins draw LOD while the wheel gesture is active so chrome does not flicker across bands.
    pub fn set_wheel_zoom_active(&mut self, active: bool) {
        if active && !self.wheel_zoom_active {
            self.wheel_zoom_render_lod = Some(self.resolved_draw_lod());
        } else if !active {
            self.wheel_zoom_render_lod = None;
        }
        self.wheel_zoom_active = active;
    }

    fn resolved_draw_lod(&self) -> DagDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        dag_draw_lod(self.fixture.camera.zoom)
    }

    fn draw_lod_for_frame(&self) -> DagDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        if self.wheel_zoom_active {
            if let Some(pinned) = self.wheel_zoom_render_lod {
                return pinned;
            }
        }
        dag_draw_lod(self.fixture.camera.zoom)
    }

    /// 📶️ Active draw LOD tier label (`minimap`, `overview`, …).
    pub fn draw_lod_label(&self) -> &'static str {
        self.draw_lod_for_frame().label()
    }

    /// 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier.
    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    /// 🔗️ World-space proximity radius for channel auto-connect; `0` disables snapping.
    pub fn set_proximity_distance(&mut self, world: f64) {
        self.engine.proximity_distance_world = world.max(0.0);
    }

    /// 🔲️ Toggles LOD-tiered world grid painting.
    pub fn set_grid_visible(&mut self, visible: bool) {
        self.grid_visible = visible;
    }

    /// 🧲️ Toggles node-drag snap to the finest visible LOD grid step.
    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    /// 📐️ Sets the positive multiplier for LOD world grid steps.
    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), DagError> {
        if !factor.is_finite() || factor <= 0.0 || factor > 1e6 {
            return Err(DagError::GridFactorOutOfRange);
        }
        self.grid_factor = factor;
        Ok(())
    }

    /// 🔍️ Frames the current node selection in the viewport camera.
    pub fn focus_selection_camera(&self, pad: f64) -> Option<DagCamera> {
        let selected = self.selected_fixture_nodes();
        if selected.is_empty() {
            return None;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for (_, node) in &selected {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            min_x = min_x.min(node.x - hw);
            min_y = min_y.min(node.y - hh);
            max_x = max_x.max(node.x + hw);
            max_y = max_y.max(node.y + hh);
        }
        if !min_x.is_finite() {
            return None;
        }
        let pad = pad.max(1.05);
        let cx = (min_x + max_x) * 0.5;
        let cy = (min_y + max_y) * 0.5;
        let span_w = (max_x - min_x).max(1.0);
        let span_h = (max_y - min_y).max(1.0);
        let vw = self.width.max(1) as f64;
        let vh = self.height.max(1) as f64;
        let zoom = (vw / (span_w * pad)).min(vh / (span_h * pad)).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
        Some(DagCamera { x: cx, y: cy, zoom })
    }

    fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }

    fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }

    fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }

    fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
        match self.draw_lod_for_frame() {
            DagDrawLod::Minimap => None,
            DagDrawLod::Overview | DagDrawLod::Compact => Some(self.grid_step_large_world()),
            DagDrawLod::Normal => Some(self.grid_step_medium_world()),
            DagDrawLod::Detail => Some(self.grid_step_small_world()),
            DagDrawLod::Micro => Some(self.grid_step_micro_world()),
        }
    }

    fn snap_world_scalar(&self, value: f64) -> f64 {
        if !self.grid_snap_enabled {
            return value;
        }
        let Some(step) = self.lod_visible_grid_snap_step_world() else {
            return value;
        };
        (value / step).round() * step
    }

    fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
        (self.snap_world_scalar(x), self.snap_world_scalar(y))
    }

    fn stroke_world_step_grid(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, color: canvas::Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
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

    fn paint_lod_grid(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod) {
        if !self.grid_visible || self.wheel_zoom_active || lod == DagDrawLod::Minimap {
            return;
        }
        let grid_color = self.canvas_theme.grid_minor_stroke;
        Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
        match lod {
            DagDrawLod::Normal | DagDrawLod::Detail | DagDrawLod::Micro => {
                Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
            }
            DagDrawLod::Minimap | DagDrawLod::Overview | DagDrawLod::Compact => {}
        }
        if matches!(lod, DagDrawLod::Detail | DagDrawLod::Micro) {
            Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
        }
        if lod == DagDrawLod::Micro {
            Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
        }
    }

    /// 📶️ Pins WASM draw LOD when {@link DagHost::set_automatic_lod} is false; pass an empty label to follow zoom bands.
    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = DagDrawLod::from_id(trimmed);
    }

    pub fn load_fixture_json(json: &str) -> Result<Self, DagError> {
        let fixture: DagFixture = os_pack::json::from_json_str(json)?;
        if fixture.schema != "dag.fixture" {
            return Err(DagError::SchemaMismatch);
        }
        validate_dag_fixture_node_kinds(&fixture.nodes)?;
        Ok(Self::from_fixture(fixture))
    }

    pub fn fixture_json(&self) -> Result<String, DagError> {
        Ok(os_pack::json::to_json_string(&self.fixture))
    }

    /// 🌳️ Recomputes node positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions) -> Result<(), DagError> {
        let mut fixture_value = os_pack::json::from_dsl_value(&<DagFixture as dsl::ToValue>::to_value(&self.fixture));
        apply_dag_layout_to_fixture_v1_value(&mut fixture_value, opts)?;
        self.fixture = <DagFixture as dsl::FromValue>::from_value(os_pack::json::to_dsl_value(&fixture_value))?;
        self.rebuild_engine_with_layout(false);
        Ok(())
    }

    fn rebuild_engine_with_layout(&mut self, apply_layout: bool) {
        self.engine = DagBoardEngine::new();
        self.engine.enforce_acyclic = true;
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.handle_port_shape.clear();
        self.handle_port_visible.clear();
        self.edge_id_map.clear();
        self.edge_engine_ids.clear();
        self.edge_engine_ids.resize(self.fixture.edges.len(), None);
        self.edge_route_style.clear();
        for node in &mut self.fixture.nodes {
            fit_node_size(node);
        }
        let (cx, cy, zoom) = (self.fixture.camera.x, self.fixture.camera.y, self.fixture.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        if apply_layout {
            let mut fixture_value = os_pack::json::from_dsl_value(&<DagFixture as dsl::ToValue>::to_value(&self.fixture));
            let _ = apply_dag_layout_to_fixture_v1_value(&mut fixture_value, &DagLayoutOptions::default());
            if let Ok(updated) = <DagFixture as dsl::FromValue>::from_value(os_pack::json::to_dsl_value(&fixture_value)) {
                self.fixture = updated;
            }
        }
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for (idx, node) in self.fixture.nodes.iter().enumerate() {
            let nid = idx as u64 + 1;
            self.node_id_map.insert(nid, idx);
            self.engine.create_rect_node(nid, node.x, node.y, node.width, node.height, true);
            let inputs = node.inputs();
            let outputs = node.outputs();
            for (port_idx, port) in inputs.iter().enumerate() {
                let in_a = io_node_rect_port_angle_for_node(node, port_idx, true);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(Self::dag_port_handle_key(&node.id, &port.id, true), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, in_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Target);
            }
            for (port_idx, port) in outputs.iter().enumerate() {
                let out_a = io_node_rect_port_angle_for_node(node, port_idx, false);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(Self::dag_port_handle_key(&node.id, &port.id, false), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, out_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Source);
            }
        }
        let existing: Vec<(String, String)> = self
            .fixture
            .edges
            .iter()
            .filter_map(|e| {
                let src = e.source.split('@').next()?.to_string();
                let tgt = e.target.split('@').next()?.to_string();
                Some((src, tgt))
            })
            .collect();
        let mut eid: u64 = 100;
        for (edge_index, edge) in self.fixture.edges.iter().enumerate() {
            if would_create_cycle(&existing, edge.source.split('@').next().unwrap_or(""), edge.target.split('@').next().unwrap_or("")) {
                continue;
            }
            let (source_node, source_port) = Self::dag_port_endpoint_parts(&edge.source);
            let (target_node, target_port) = Self::dag_port_endpoint_parts(&edge.target);
            let src = handle_map.get(&Self::dag_port_handle_key(&source_node, &source_port, false)).copied();
            let tgt = handle_map.get(&Self::dag_port_handle_key(&target_node, &target_port, true)).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                let id = Self::parse_fixture_edge_numeric_id(&edge.id).unwrap_or(eid);
                eid = eid.max(id).saturating_add(1);
                self.engine.create_edge(id, s, t);
                self.edge_id_map.insert(id, edge.id.clone());
                self.edge_engine_ids[edge_index] = Some(id);
                self.edge_route_style.insert(id, edge.route_style);
            }
        }
        self.engine.set_next_edge_id(eid);
    }

    fn parse_fixture_edge_numeric_id(id: &str) -> Option<u64> {
        id.strip_prefix('e').and_then(|s| s.parse().ok())
    }

    fn dag_port_endpoint_parts(endpoint: &str) -> (String, String) {
        endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), String::new()), |(node, port)| (node.to_string(), port.to_string()))
    }

    fn dag_port_handle_key(node_id: &str, port_id: &str, input: bool) -> String {
        format!("{}:{}:{}", node_id, if input { "in" } else { "out" }, port_id)
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let cam = CanvasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn sync_node_positions_from_engine(&mut self) {
        for (&nid, &idx) in &self.node_id_map {
            let Some(engine_node) = self.engine.nodes.get(&nid) else {
                continue;
            };
            let (mut x, mut y) = (engine_node.center.x, engine_node.center.y);
            if self.grid_snap_enabled {
                (x, y) = self.snap_world_pair(x, y);
            }
            self.fixture.nodes[idx].x = x;
            self.fixture.nodes[idx].y = y;
            if self.grid_snap_enabled {
                if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
                    engine_node.center = canvas::Point::new(x, y);
                }
            }
        }
    }

    fn node_spec_for_paint<'a>(&'a self, idx: usize, node: &'a DagNodeSpec) -> std::borrow::Cow<'a, DagNodeSpec> {
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return std::borrow::Cow::Borrowed(node);
        };
        let Some(engine_node) = self.engine.nodes.get(&nid) else {
            return std::borrow::Cow::Borrowed(node);
        };
        if (engine_node.center.x - node.x).abs() < 1e-9 && (engine_node.center.y - node.y).abs() < 1e-9 {
            return std::borrow::Cow::Borrowed(node);
        }
        let mut synced = node.clone();
        synced.x = engine_node.center.x;
        synced.y = engine_node.center.y;
        std::borrow::Cow::Owned(synced)
    }

    fn sync_edges_from_engine(&mut self) {
        let mut edges = Vec::with_capacity(self.engine.edges.len());
        let mut edge_engine_ids = Vec::with_capacity(self.engine.edges.len());
        for (eid, edge) in &self.engine.edges {
            let Some(source) = self.handle_key_map.get(&edge.source).cloned() else {
                continue;
            };
            let Some(target) = self.handle_key_map.get(&edge.target).cloned() else {
                continue;
            };
            let id = self.edge_id_map.get(eid).cloned().unwrap_or_else(|| format!("e{eid}"));
            self.edge_id_map.insert(*eid, id.clone());
            edges.push(DagFixtureEdge { id, source, target, ..Default::default() });
            edge_engine_ids.push(Some(*eid));
        }
        self.fixture.edges = edges;
        self.edge_engine_ids = edge_engine_ids;
    }

    fn process_engine_events(&mut self) {
        let events = self.engine.drain_events();
        let mut moved = false;
        let mut wired = false;
        for event in events {
            match event {
                BoardEvent::NodeMoved { id, x, y } => {
                    moved = true;
                    dag_debug_log(&format!("[DEBUG] dag node moved id={id} x={x:.1} y={y:.1}"));
                }
                BoardEvent::EdgeConnected { id, source, target } => {
                    wired = true;
                    dag_debug_log(&format!("[DEBUG] dag edge connected id={id} source={source} target={target}"));
                }
                BoardEvent::EdgeRemoved { id } => {
                    wired = true;
                    dag_debug_log(&format!("[DEBUG] dag edge removed id={id}"));
                }
                BoardEvent::SelectionChanged { node_ids, .. } => {
                    let ids: Vec<String> = node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect();
                    dag_debug_log(&format!("[DEBUG] dag selection changed: {}", ids.join(", ")));
                }
                BoardEvent::PreselectChanged { node_ids, removed_node_ids, .. } => {
                    let ids: Vec<String> = node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect();
                    let removed: Vec<String> = removed_node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect();
                    dag_debug_log(&format!("[DEBUG] dag preselect ids=[{}] removed=[{}]", ids.join(", "), removed.join(", ")));
                }
                BoardEvent::HoverChanged { id } => {
                    let label = id.and_then(|nid| self.widget_id_for_node_id(nid).or_else(|| self.engine.handles.get(&nid).and_then(|handle| self.widget_id_for_node_id(handle.node_id))));
                    dag_debug_log(&format!("[DEBUG] dag hover changed: {}", label.as_deref().unwrap_or("—")));
                }
            }
        }
        if moved {
            self.sync_node_positions_from_engine();
        }
        if wired {
            self.sync_edges_from_engine();
        }
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.fixture.camera = DagCamera { x, y, zoom };
        self.engine.set_camera(x, y, zoom);
    }

    fn handle_id_for_port(&self, node_id: &str, port_id: &str) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
        self.handle_key_map.iter().find(|(_, k)| k.as_str() == key).map(|(&hid, _)| hid)
    }

    fn port_row_handle_hit(&self, world_x: f64, world_y: f64, inputs: bool, outputs: bool) -> Option<HandleId> {
        for node in self.fixture.nodes.iter().rev() {
            if inputs {
                for (port_idx, port) in node.inputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = input_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port(&node.id, &port.id) {
                        return Some(hid);
                    }
                }
            }
            if outputs {
                for (port_idx, port) in node.outputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = output_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port(&node.id, &port.id) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    fn channel_row_handle_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        if !self.draw_lod_for_frame().uses_channel_row_pick() {
            return None;
        }
        self.port_row_handle_hit(world_x, world_y, true, true)
    }

    fn handle_anchor_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        if !self.draw_lod_for_frame().allows_connection_hit_picking() {
            return None;
        }
        use canvas::Point;
        let p = Point::new(world_x, world_y);
        for (&hid, handle) in self.engine.handles.iter().rev() {
            let Some(node) = self.engine.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let dx = p.x - pos.x;
            let dy = p.y - pos.y;
            let tol = handle.radius + 6.0;
            if dx * dx + dy * dy <= tol * tol {
                return Some(hid);
            }
        }
        None
    }

    fn rim_handle_anchor_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        let hid = self.handle_anchor_hit(world_x, world_y)?;
        let lod = self.draw_lod_for_frame();
        if (lod.uses_input_row_connection_hitbox() || lod.uses_channel_row_pick()) && self.port_row_handle_hit(world_x, world_y, true, true).is_some() {
            let handle = self.engine.handles.get(&hid)?;
            let node = self.engine.nodes.get(&handle.node_id)?;
            let pos = handle_position(node, handle);
            let dx = world_x - pos.x;
            let dy = world_y - pos.y;
            let rim_tol = (handle.radius + 1.5).max(3.0);
            if dx * dx + dy * dy > rim_tol * rim_tol {
                return None;
            }
        }
        Some(hid)
    }

    fn fixture_draggable_node_hit(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x < node.x - hw || world_x > node.x + hw || world_y < node.y - hh || world_y > node.y + hh {
                continue;
            }
            let Some(nid) = self.engine_node_id_for_index(idx) else {
                continue;
            };
            if self.engine.nodes.get(&nid).is_some_and(|n| n.draggable) {
                return Some(nid);
            }
        }
        None
    }

    fn connection_hit_world(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        let Some(hid) = self.rim_handle_anchor_hit(world_x, world_y) else {
            return (world_x, world_y);
        };
        let Some(handle) = self.engine.handles.get(&hid) else {
            return (world_x, world_y);
        };
        let Some(node) = self.engine.nodes.get(&handle.node_id) else {
            return (world_x, world_y);
        };
        let pos = handle_position(node, handle);
        (pos.x, pos.y)
    }

    fn world_hits_handle(&self, world_x: f64, world_y: f64) -> bool {
        self.rim_handle_anchor_hit(world_x, world_y).is_some()
    }

    fn sync_channel_row_pointer_hover(&mut self, world_x: f64, world_y: f64) {
        if !self.draw_lod_for_frame().uses_channel_row_pick() {
            return;
        }
        if !matches!(self.engine.interaction, InteractionMode::Idle) {
            return;
        }
        if self.handle_anchor_hit(world_x, world_y).is_some() {
            return;
        }
        if let Some(hid) = self.channel_row_handle_hit(world_x, world_y) {
            self.engine.hover = Some(hid);
        }
    }

    fn try_node_rectangle_pointer_down(&mut self, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) -> bool {
        if button != 0 || alt {
            return false;
        }
        let Some(node_id) = self.fixture_draggable_node_hit(world_x, world_y) else {
            return false;
        };
        use canvas::Point;
        use graph::pick_merge_mode_for_modifiers;

        let point = Point::new(world_x, world_y);
        self.engine.pointer_down_on_draggable_node_at(node_id, point, shift, ctrl_or_meta);
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            if let Some(hid) = self.channel_row_handle_hit(world_x, world_y) {
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.engine.selection_options.mode.as_str());
                self.engine.select_handle_with_mode(hid, merge_mode.as_str());
                self.engine.hover = Some(hid);
            }
        }
        true
    }

    fn widget_hit_at(&self, world_x: f64, world_y: f64) -> Option<(usize, WidgetPointerKind)> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            match &node.kind {
                DagNodeKind::Slider { .. } => {
                    let (x0, y0, x1, y1) = slider_track_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SliderDrag));
                    }
                }
                DagNodeKind::Select { .. } => {
                    let (x0, y0, x1, y1) = select_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SelectClick));
                    }
                }
                DagNodeKind::Preview { content: DagPreviewContent::Tree { json }, expanded, .. } => {
                    for row in preview_tree_row_layouts(node, json, expanded) {
                        let (x0, y0, x1, y1) = row.row_rect;
                        if x1 > x0 && point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                            return Some((idx, WidgetPointerKind::PreviewToggle(row.path)));
                        }
                    }
                }
                DagNodeKind::Cluster { .. } => {
                    if cluster_explode_hit(node, world_x, world_y) {
                        return Some((idx, WidgetPointerKind::ClusterExplode));
                    }
                }
                DagNodeKind::Export { .. } => {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::ExportClick));
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn sync_fixture_node_size_to_engine(&mut self, idx: usize) {
        let node = &self.fixture.nodes[idx];
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return;
        };
        let Some(engine_node) = self.engine.nodes.get_mut(&nid) else {
            return;
        };
        engine_node.width = node.width;
        engine_node.height = node.height;
    }

    fn toggle_preview_tree_path(node: &mut DagNodeSpec, path: &str) {
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

    /// ✏️ Begins inline note text editing at a world-space click position.
    pub fn begin_note_edit(&mut self, node_id: &str, world_x: f64, _world_y: f64) -> bool {
        let Some(node) = self.fixture.nodes.iter().find(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &node.kind else {
            return false;
        };
        let lod_index = dag_lod_index(self.fixture.camera.zoom);
        let font_px = dag_label_paint_px(self.fixture.camera.zoom, lod_index) * 1.05;
        let origin_x = note_text_origin_x(node);
        let offset = hit_byte_in_note_line(text, world_x, origin_x, font_px);
        self.editing_note = Some(NoteEditState { node_id: node_id.to_string(), caret: offset, anchor: offset });
        self.caret_visible = true;
        true
    }

    /// ✏️ Inserts text at the active note caret, replacing any selection.
    pub fn note_insert_text(&mut self, chunk: &str) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        text.replace_range(start..end, chunk);
        let pos = start + chunk.len();
        edit.caret = pos;
        edit.anchor = pos;
        true
    }

    /// ✏️ Deletes the selection or the character before the note caret.
    pub fn note_backspace(&mut self) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        if start < end {
            text.replace_range(start..end, "");
            edit.caret = start;
            edit.anchor = start;
            return true;
        }
        if start == 0 {
            return false;
        }
        let prev = text[..start].char_indices().last().map_or(0, |(i, _)| i);
        text.replace_range(prev..start, "");
        edit.caret = prev;
        edit.anchor = prev;
        true
    }

    /// ✏️ Deletes the selection or the character after the note caret.
    pub fn note_delete_forward(&mut self) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        if start < end {
            text.replace_range(start..end, "");
            edit.caret = start;
            edit.anchor = start;
            return true;
        }
        if start >= text.len() {
            return false;
        }
        let next = text[start..].char_indices().nth(1).map_or(text.len(), |(i, _)| start + i);
        text.replace_range(start..next, "");
        edit.caret = start;
        edit.anchor = start;
        true
    }

    /// ✏️ Moves the note caret (`left` | `right` | `home` | `end`).
    pub fn note_move_caret(&mut self, direction: &str, extend: bool) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let pos = match direction {
            "left" => {
                if edit.caret == 0 {
                    0
                } else {
                    text[..edit.caret].char_indices().last().map_or(0, |(i, _)| i)
                }
            }
            "right" => text[edit.caret..].char_indices().nth(1).map_or(text.len(), |(i, _)| edit.caret + i),
            "home" => 0,
            "end" => text.len(),
            _ => return false,
        };
        if extend {
            edit.caret = pos;
        } else {
            edit.caret = pos;
            edit.anchor = pos;
        }
        true
    }

    /// ✏️ Ends inline note editing.
    pub fn note_commit_edit(&mut self) {
        self.editing_note = None;
    }

    /// ✏️ Toggles native caret visibility for the active note editor.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.caret_visible = visible;
    }

    /// ✏️ Returns the widget id currently being edited inline, if any.
    pub fn editing_note_id(&self) -> Option<&str> {
        self.editing_note.as_ref().map(|edit| edit.node_id.as_str())
    }

    /// 📐️ Recomputes preview and image node sizes after content changes.
    pub fn fit_preview_sizes(&mut self) {
        for idx in 0..self.fixture.nodes.len() {
            let kind = &self.fixture.nodes[idx].kind;
            if matches!(kind, DagNodeKind::Preview { .. } | DagNodeKind::Image { .. }) {
                fit_node_size(&mut self.fixture.nodes[idx]);
                self.sync_fixture_node_size_to_engine(idx);
            }
        }
    }

    /// 📐️ Recomputes note node sizes after text changes.
    pub fn fit_note_sizes(&mut self) {
        for idx in 0..self.fixture.nodes.len() {
            if matches!(self.fixture.nodes[idx].kind, DagNodeKind::Note { .. }) {
                fit_node_size(&mut self.fixture.nodes[idx]);
                self.sync_fixture_node_size_to_engine(idx);
            }
        }
    }

    fn try_widget_pointer_down(&mut self, world_x: f64, world_y: f64) -> bool {
        if !self.draw_lod_for_frame().shows_controls() {
            return false;
        }
        let Some((idx, kind)) = self.widget_hit_at(world_x, world_y) else {
            return false;
        };
        match kind {
            WidgetPointerKind::SliderDrag => {
                let node_id = self.fixture.nodes[idx].id.clone();
                self.widget_drag = Some(idx);
                if let Some(value) = set_slider_value_from_x(&mut self.fixture.nodes[idx], world_x) {
                    dag_debug_log(&format!("[DEBUG] dag slider value id={node_id} value={value:.3}"));
                }
            }
            WidgetPointerKind::SelectClick => {
                let node_id = self.fixture.nodes[idx].id.clone();
                if let Some(label) = advance_select_option(&mut self.fixture.nodes[idx]) {
                    dag_debug_log(&format!("[DEBUG] dag select option id={node_id} label={label}"));
                }
            }
            WidgetPointerKind::PreviewToggle(path) => {
                Self::toggle_preview_tree_path(&mut self.fixture.nodes[idx], &path);
                self.sync_fixture_node_size_to_engine(idx);
            }
            WidgetPointerKind::ClusterExplode => {
                self.pending_cluster_explode = Some(self.fixture.nodes[idx].id.clone());
            }
            WidgetPointerKind::ExportClick => {
                self.pending_export_click = Some(self.fixture.nodes[idx].id.clone());
            }
        }
        true
    }

    /// 💥️ Takes a pending cluster explode request from the last widget hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.pending_cluster_explode.take()
    }

    /// 📤️ Takes a pending export control click from the last widget hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.pending_export_click.take()
    }

    fn sync_connection_hit_picking_for_lod(&mut self) {
        let allow = self.draw_lod_for_frame().allows_connection_hit_picking();
        self.engine.handle_pointer_picking = allow;
        if !allow && matches!(self.engine.interaction, InteractionMode::DrawEdge { .. }) {
            self.engine.interaction = InteractionMode::Idle;
        }
    }

    /// @emoji 🧭️ Minimap LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
    fn lod_uses_bounded_drag(&self) -> bool {
        matches!(self.draw_lod_for_frame(), DagDrawLod::Minimap)
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        self.pointer_down_screen(x, y, 0, extend, false, false, false);
    }

    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo (see infinite/board/rs, framework/editor/rs, layout/rs, etc.)")]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        if button == 0 && !shift && !ctrl_or_meta && !alt && !pan {
            if let Some((layout, on_viewport)) = self.minimap_widget_pointer_hit(sx, sy) {
                let (wx, wy) = self.minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.fixture.camera.zoom;
                if on_viewport {
                    let cam = &self.fixture.camera;
                    self.minimap_widget_drag = Some((cam.x - wx, cam.y - wy));
                } else {
                    self.set_camera(wx, wy, zoom);
                    self.minimap_widget_drag = Some((0.0, 0.0));
                }
                dag_debug_log(&format!("[DEBUG] minimap widget pointer down sx={sx:.1} sy={sy:.1} on_viewport={on_viewport}"));
                return;
            }
        }
        if pan {
            self.pan_anchor = Some((sx, sy, self.fixture.camera.x, self.fixture.camera.y));
            return;
        }
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if button == 0 && !shift && !ctrl_or_meta && !alt {
            if let Some(node_id) = self.fixture_draggable_node_hit(world.x, world.y) {
                if !self.world_hits_handle(world.x, world.y) {
                    if let Some(widget_id) = self.widget_id_for_node_id(node_id) {
                        if let Some(node) = self.fixture.nodes.iter().find(|entry| entry.id == widget_id) {
                            if let DagNodeKind::AppInstance { instance_id, .. } = &node.kind {
                                let now = pointer_event_now_ms();
                                let dist = ((world.x - self.last_pointer_down_world.0).powi(2) + (world.y - self.last_pointer_down_world.1).powi(2)).sqrt();
                                let zoom = self.fixture.camera.zoom.max(1e-9);
                                if self.last_pointer_down_node_id.as_deref() == Some(widget_id.as_str()) && now - self.last_pointer_down_at_ms < 350.0 && dist < 8.0 / zoom {
                                    self.pending_open_instance_id = Some(instance_id.clone());
                                    return;
                                }
                                self.last_pointer_down_at_ms = now;
                                self.last_pointer_down_world = (world.x, world.y);
                                self.last_pointer_down_node_id = Some(widget_id);
                            }
                        }
                    }
                }
            }
        }
        if let Some(hit) = self.port_insert_hit(world.x, world.y, self.fixture.camera.zoom) {
            self.pending_port_insert = Some(hit);
            return;
        }
        let (hit_x, hit_y) = self.connection_hit_world(world.x, world.y);
        if self.world_hits_handle(hit_x, hit_y) {
            self.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
            self.process_engine_events();
            self.sync_camera_from_engine();
            return;
        }
        if self.try_widget_pointer_down(world.x, world.y) {
            return;
        }
        if self.try_node_rectangle_pointer_down(world.x, world.y, button, shift, ctrl_or_meta, alt) {
            self.process_engine_events();
            self.sync_camera_from_engine();
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        if button == 0 && !merge_from_modifiers && self.lod_uses_bounded_drag() {
            let pad = DAG_BOUNDED_DRAG_HIT_PAD_PX / self.fixture.camera.zoom.max(1e-9);
            if self.engine.try_begin_selection_union_drag_at(world, pad) {
                self.process_engine_events();
                self.sync_camera_from_engine();
                return;
            }
        }
        self.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn widget_drag_active(&self) -> bool {
        self.widget_drag.is_some()
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, false, false, false);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        if let Some((ox, oy)) = self.minimap_widget_drag {
            if let Some(layout) = self.minimap_widget_layout(self.width, self.height) {
                let (wx, wy) = self.minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.fixture.camera.zoom;
                self.set_camera(wx + ox, wy + oy, zoom);
                self.last_screen_x = sx;
                self.last_screen_y = sy;
                return;
            }
        }
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.fixture.camera.zoom.max(1e-9);
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            self.last_screen_x = sx;
            self.last_screen_y = sy;
            return;
        }
        let minimap_hovered = self.minimap_widget_pointer_hit(sx, sy).is_some();
        self.minimap_widget_hovered = minimap_hovered;
        if minimap_hovered {
            self.last_screen_x = sx;
            self.last_screen_y = sy;
            return;
        }
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if let Some(idx) = self.widget_drag {
            if let Some(value) = set_slider_value_from_x(&mut self.fixture.nodes[idx], world.x) {
                dag_debug_log(&format!("[DEBUG] dag slider value id={} value={value:.3}", self.fixture.nodes[idx].id));
            }
            return;
        }
        let (hit_x, hit_y) = self.connection_hit_world(world.x, world.y);
        self.engine.pointer_move_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        self.sync_channel_row_pointer_hover(world.x, world.y);
        self.sync_minimap_pointer_hover(world.x, world.y);
        if matches!(self.engine.interaction, InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. }) {
            self.sync_node_positions_from_engine();
        }
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, false, false, false);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.pan_anchor = None;
        self.minimap_widget_drag = None;
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        if self.widget_drag.take().is_some() {
            return;
        }
        let world = self.screen_to_world_point(sx, sy);
        let (hit_x, hit_y) = self.connection_hit_world(world.x, world.y);
        self.engine.pointer_up_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_node_positions_from_engine();
        self.sync_camera_from_engine();
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), DagError> {
        self.canvas_theme.merge_from_json(json).map_err(DagError::CanvasTheme)?;
        self.icon_paint_cache.clear();
        Ok(())
    }

    /// 🖼️ Screen-node overlay rects in CSS pixel space for DOM media layers.
    pub fn node_overlays_json(&self) -> Result<String, DagError> {
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let cam = CanvasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let mut overlays = Vec::new();
        for node in &self.fixture.nodes {
            let DagNodeKind::Screen { media: Some(media), .. } = &node.kind else {
                continue;
            };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let inset = 8.0 / cam.zoom.max(0.05);
            let top = node.y - hh + hh * 0.35;
            let bottom = node.y + hh - inset;
            let left = node.x - hw + inset;
            let right = node.x + hw - inset;
            let tl = world_to_screen(&cam, &viewport, Point::new(left, top));
            let br = world_to_screen(&cam, &viewport, Point::new(right, bottom));
            let media_kind = match media.kind {
                DagMediaKind::Image => "image",
                DagMediaKind::Svg => "svg",
                DagMediaKind::Pdf => "pdf",
                DagMediaKind::Video => "video",
            };
            overlays.push(os_pack::json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("mediaKind".to_string(), Value::from(media_kind)),
                ("src".to_string(), Value::from(media.src.clone())),
                (
                    "rect".to_string(),
                    os_pack::json::object([("x".to_string(), Value::from(tl.x)), ("y".to_string(), Value::from(tl.y)), ("w".to_string(), Value::from((br.x - tl.x).max(1.0))), ("h".to_string(), Value::from((br.y - tl.y).max(1.0)))]),
                ),
            ]));
        }
        Ok(os_pack::json::to_string(&Value::Array(overlays)))
    }

    fn handle_cap_peak(&self, center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::Point {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_peak(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_peak(center, outward, radius),
        }
    }

    fn handle_cap_fill_path(&self, center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::BezPath {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_fill_path(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_fill_path(center, outward, radius),
        }
    }

    fn handle_cap_stroke_path(&self, center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::BezPath {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_stroke_path(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_stroke_path(center, outward, radius),
        }
    }

    fn edge_sharp_path(&self, eid: EdgeId) -> Option<canvas::BezPath> {
        let edge = self.engine.edges.get(&eid)?;
        let source_handle = self.engine.handles.get(&edge.source)?;
        let target_handle = self.engine.handles.get(&edge.target)?;
        let source_node = self.engine.nodes.get(&source_handle.node_id)?;
        let target_node = self.engine.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        let source_out = handle_outward_at_node_rim(source_position, source_node.center, source_node.shape, source_node.radius, source_node.width, source_node.height)?;
        let target_out = handle_outward_at_node_rim(target_position, target_node.center, target_node.shape, target_node.radius, target_node.width, target_node.height)?;
        let source_shape = self.handle_port_shape.get(&edge.source).copied().unwrap_or_default();
        let target_shape = self.handle_port_shape.get(&edge.target).copied().unwrap_or_default();
        let source_wire = self.handle_cap_peak(source_position, source_out, source_handle.radius, source_shape);
        let target_wire = self.handle_cap_peak(target_position, target_out, target_handle.radius, target_shape);
        Some(compute_edge_sharp_sz_path(source_wire, target_wire, source_out, target_out))
    }

    fn paint_node_handles_for_spec(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, node: &DagNodeSpec, chrome: &DagNodePaintChrome) {
        use canvas::FillRule;
        use canvas::Stroke;
        use canvas::{Circle, Point};
        use graph::{handle_outward_at_node_rim, handle_position_on_rectangle, NodeShape};

        let theme = &self.canvas_theme;
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let tint = chrome.tint_highlighted();
        let fill = dag_handle_body_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let stroke_c = dag_handle_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let handle_chrome = chrome.has_interaction_chrome();
        let center = Point::new(node.x, node.y);
        let paint_port = |scene: &mut canvas::Scene, port_idx: usize, inputs: bool, port: &IoPortSpec| {
            if !port.visible {
                return;
            }
            let angle = io_node_rect_port_angle_for_node(node, port_idx, inputs);
            let handle_center = handle_position_on_rectangle(center, node.width, node.height, angle);
            let outward = handle_outward_at_node_rim(handle_center, center, NodeShape::Rectangle, 0.0, node.width, node.height);
            if let Some(out) = outward {
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &self.handle_cap_fill_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &self.handle_cap_stroke_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
            } else {
                let circle = Circle::new(handle_center, DAG_HANDLE_WORLD_RADIUS);
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &circle);
            }
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            paint_port(scene, port_idx, true, port);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            paint_port(scene, port_idx, false, port);
        }
    }

    pub fn label_overlay_rows_for_node_spec(&self, node: &DagNodeSpec, ghost: bool) -> Vec<Value> {
        let lod = self.draw_lod_for_frame();
        let zoom = self.fixture.camera.zoom;
        let lod_index = dag_lod_index(zoom);
        let engine_nid = self.node_id_for_widget_id(&node.id);
        Self::label_overlay_rows_for_node(node, lod, zoom, lod_index, ghost, engine_nid, &self.unresolved_input_ports)
    }

    fn label_overlay_rows_for_node(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize, ghost: bool, engine_nid: Option<NodeId>, unresolved_input_ports: &HashSet<(NodeId, String)>) -> Vec<Value> {
        let paint_px = dag_label_paint_px(zoom, lod_index);
        let mut labels = Vec::new();
        if let Some(text) = Self::node_label_text(node, lod).map(str::to_string) {
            let (layout, x, y) = if lod.node_label_is_horizontal() {
                ("horizontal", node.x, node.y)
            } else if (uses_computation_layout(&node.kind) && lod.shows_computation_layout()) || (matches!(node.kind, DagNodeKind::Slider { .. }) && lod.shows_controls()) {
                let (lx, ly) = computation_name_world_center(node, &text, paint_px, zoom);
                ("horizontal", lx, ly)
            } else {
                let (lx, ly) = io_widget_label_center(node);
                ("vertical", lx, ly)
            };
            labels.push(os_pack::json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("text".to_string(), Value::from(text)),
                ("layout".to_string(), Value::from(layout)),
                ("x".to_string(), Value::from(x)),
                ("y".to_string(), Value::from(y)),
                ("nodeW".to_string(), Value::from(node.width)),
                ("nodeH".to_string(), Value::from(node.height)),
                ("fontScreenPx".to_string(), Value::from(paint_px)),
                ("ghost".to_string(), Value::from(ghost)),
            ]));
        }
        if lod.shows_port_labels() && !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            for mut row in Self::port_label_overlay_rows(node, lod, zoom, lod_index, engine_nid, unresolved_input_ports) {
                if let Some(obj) = row.as_object_mut() {
                    obj.insert("ghost", Value::Bool(ghost));
                }
                labels.push(row);
            }
        }
        labels
    }

    fn port_label_overlay_rows(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize, engine_nid: Option<NodeId>, unresolved_input_ports: &HashSet<(NodeId, String)>) -> Vec<Value> {
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
        let input_port_label = |port: &IoPortSpec| -> String {
            let mut port = port.clone();
            if let Some(nid) = engine_nid {
                if unresolved_input_ports.contains(&(nid, port.id.clone())) {
                    port.resolved = Some(false);
                }
            }
            port.label_with_cardinality(lod)
        };
        for (i, port) in inputs.iter().enumerate() {
            if port.shape == PortShape::Triangle || !port.visible {
                continue;
            }
            let label = input_port_label(port);
            if label.trim().is_empty() {
                continue;
            }
            let world_y = port_center_y(node, i, inputs.len());
            let world_x = if computation { computation_input_label_x(node) } else { node.x - hw + handle_inset };
            rows.push(os_pack::json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("kind".to_string(), Value::from("port")),
                ("text".to_string(), Value::from(label)),
                ("layout".to_string(), Value::from("horizontal")),
                ("align".to_string(), Value::from("left")),
                ("x".to_string(), Value::from(world_x)),
                ("y".to_string(), Value::from(world_y)),
                ("nodeW".to_string(), Value::from(input_column_w)),
                ("nodeH".to_string(), Value::from(DAG_CHANNEL_ROW_HEIGHT)),
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
            rows.push(os_pack::json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("kind".to_string(), Value::from("port")),
                ("text".to_string(), Value::from(label)),
                ("layout".to_string(), Value::from("horizontal")),
                ("align".to_string(), Value::from("right")),
                ("x".to_string(), Value::from(world_x)),
                ("y".to_string(), Value::from(world_y)),
                ("nodeW".to_string(), Value::from(column_w)),
                ("nodeH".to_string(), Value::from(DAG_CHANNEL_ROW_HEIGHT)),
                ("fontScreenPx".to_string(), Value::from(port_layout_px)),
                ("maxScreenH".to_string(), Value::from(port_layout_px * 1.3)),
            ]));
        }
        rows
    }

    /// 🎚️ Slider track anchors for the HTML slider overlay.
    pub fn slider_overlay_state_json(&self) -> Result<String, DagError> {
        let cam = &self.fixture.camera;
        let mut sliders: Vec<Value> = Vec::new();
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let DagNodeKind::Slider { min, max, step, value, .. } = &node.kind else {
                continue;
            };
            let (x0, y0, x1, y1) = slider_track_bounds(&node);
            sliders.push(os_pack::json::object([
                ("widgetId".to_string(), Value::from(fixture_node.id.clone())),
                ("label".to_string(), Value::from(node.name.clone())),
                ("value".to_string(), Value::from(*value)),
                ("min".to_string(), Value::from(*min)),
                ("max".to_string(), Value::from(*max)),
                ("step".to_string(), Value::from(*step)),
                ("x".to_string(), Value::from((x0 + x1) * 0.5)),
                ("y".to_string(), Value::from((y0 + y1) * 0.5)),
                ("w".to_string(), Value::from((x1 - x0).max(1.0))),
                ("h".to_string(), Value::from((y1 - y0).max(1.0))),
            ]));
        }
        Ok(os_pack::json::to_string(&os_pack::json::object([
            ("camera".to_string(), os_pack::json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("width".to_string(), Value::from(self.width)),
            ("height".to_string(), Value::from(self.height)),
            ("sliders".to_string(), Value::Array(sliders)),
        ])))
    }

    /// 🏷️ Camera, draw LOD, and node label anchors for the JS canvas text overlay (must match the last GPU frame).
    pub fn label_overlay_paint_state_json(&self) -> Result<String, DagError> {
        let lod = self.draw_lod_for_frame();
        let cam = &self.fixture.camera;
        let lod_index = dag_lod_index(cam.zoom);
        let mut labels = Vec::new();
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let engine_nid = self.engine_node_id_for_index(idx);
            labels.extend(Self::label_overlay_rows_for_node(node.as_ref(), lod, cam.zoom, lod_index, false, engine_nid, &self.unresolved_input_ports));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            labels.extend(Self::label_overlay_rows_for_node(ghost, lod, cam.zoom, lod_index, true, None, &self.unresolved_input_ports));
        }
        let minimap_widget = self.minimap_widget_json();
        Ok(os_pack::json::to_string(&os_pack::json::object([
            ("camera".to_string(), os_pack::json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("lod".to_string(), Value::from(lod.label())),
            ("width".to_string(), Value::from(self.width)),
            ("height".to_string(), Value::from(self.height)),
            ("labels".to_string(), Value::Array(labels)),
            ("minimapWidget".to_string(), Value::from(minimap_widget)),
        ])))
    }

    fn paint_variadic_plus_controls(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, px: f64, fill: canvas::Color, halo: canvas::Color) {
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

    fn paint_computation_column_divider(scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: canvas::Color) {
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

    fn paint_computation_channel_row_highlights(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, node: &DagNodeSpec, theme: &CanvasPalette, is_dimmed: bool) {
        use canvas::FillRule;
        use canvas::Rect;
        let mut paint_bounds = |(x0, y0, x1, y1): (f64, f64, f64, f64), selected: bool, highlighted: bool, hovered: bool| {
            if !selected && !highlighted && !hovered {
                return;
            }
            let fill = dag_handle_body_fill(theme, is_dimmed, selected, highlighted, hovered);
            scene.fill(FillRule::NonZero, *aff, fill, None, &Rect::new(x0, y0, x1, y1));
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            let Some(bounds) = input_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port(&node.id, &port.id) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            let Some(bounds) = output_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port(&node.id, &port.id) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
    }

    fn computation_channel_row_divider_stroke(&self, node: &DagNodeSpec, port_id: &str, body_stroke: canvas::Color, label_fill: canvas::Color, default_stroke: canvas::Color) -> canvas::Color {
        let Some(hid) = self.handle_id_for_port(&node.id, port_id) else {
            return default_stroke;
        };
        let (selected, _, hovered) = self.handle_interaction_chrome(hid);
        if selected || hovered {
            dag_node_internal_chrome_stroke(body_stroke, label_fill, true)
        } else {
            default_stroke
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_computation_channel_row_dividers(&self, scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: canvas::Color, body_stroke: canvas::Color, label_fill: canvas::Color, channel_row_pick: bool) {
        use canvas::{Line, Point, Stroke};
        let grid_rows = computation_channel_row_count(node);
        if grid_rows <= 1 {
            return;
        }
        let (input_rows, output_rows) = computation_io_side_row_counts(node);
        let stroke_style = Stroke::new(chrome_stroke);
        let row_stroke = |port_id: &str| {
            if channel_row_pick {
                self.computation_channel_row_divider_stroke(node, port_id, body_stroke, label_fill, stroke)
            } else {
                stroke
            }
        };
        if computation_input_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Input);
            let inputs = node.inputs();
            for row in computation_io_side_row_divider_indices(input_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = inputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
        if computation_output_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output);
            let outputs = node.outputs();
            for row in computation_io_side_row_divider_indices(output_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = outputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_preview_image_content(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, src: &str, label_fill: canvas::Color, bg: canvas::Color) {
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
    fn paint_preview_content(
        &self,
        scene: &mut canvas::Scene,
        cam: &canvas::camera::Camera,
        viewport: &canvas::camera::Viewport,
        node: &DagNodeSpec,
        content: &DagPreviewContent,
        expanded: &BTreeSet<String>,
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

    fn paint_io_widget_channel_borders(scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, px: f64, chrome_stroke: f64, stroke: canvas::Color) {
        use canvas::{Line, Point, Stroke};
        let (name_left, top, name_right, bottom) = io_widget_name_column_bounds(node, px);
        let stroke_style = Stroke::new(chrome_stroke);
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_left, top), Point::new(name_left, bottom)));
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_right, top), Point::new(name_right, bottom)));
    }

    fn node_label_text(node: &DagNodeSpec, lod: DagDrawLod) -> Option<&str> {
        if matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            return None;
        }
        let text = match lod.node_label() {
            DagNodeLabel::None => return None,
            DagNodeLabel::Abbreviation => node.abbreviation.trim(),
            DagNodeLabel::Name => node.name.trim(),
        };
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }

    fn node_caption_delegated_to_js_overlay(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        Self::node_label_text(node, lod).is_some()
    }

    fn should_paint_node_lod_icon(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        if !lod.node_icon_visible() {
            return false;
        }
        !uses_computation_layout(&node.kind) || !lod.shows_computation_layout()
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_node_lod_icon(&self, scene: &mut canvas::Scene, lod: DagDrawLod, center_screen: canvas::Point, node: &DagNodeSpec, zoom: f64, fg: canvas::Color, bg: canvas::Color) {
        if !Self::should_paint_node_lod_icon(node, lod) {
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

    fn paint_cluster_affordances(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, paint_px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
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
    fn paint_computation_node_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::Point;
        let (label_x, label_y) = computation_name_world_center(node, label, px, cam.zoom);
        let anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_horizontal(scene, anchor, label, px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_slider_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        Self::paint_computation_node_name(scene, cam, viewport, node, label, px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_io_widget_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, lod: DagDrawLod, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::Point;
        if lod.node_label() == DagNodeLabel::None && !lod.shows_controls() {
            return;
        }
        let (label_x, label_y) = io_widget_label_center(node);
        let name_anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_vertical(scene, name_anchor, label, px, label_fill, label_halo);
    }

    fn paint_node_name_horizontal(scene: &mut canvas::Scene, center_screen: canvas::Point, name: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
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
    fn paint_computing_border_arc(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color, start_t: f64, dashed: bool) {
        use canvas::{BezPath, Stroke};
        const SEGMENTS: usize = 40;
        const ARC_FRACTION: f64 = 0.24;
        let mut path = BezPath::new();
        for i in 0..=SEGMENTS {
            let local = i as f64 / SEGMENTS as f64;
            let t = (start_t + local * ARC_FRACTION).fract();
            let p = Self::rect_perimeter_point(rect, t);
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

    fn paint_computing_active_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_active_anim_phase.get(), false);
    }

    fn paint_computing_stale_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_stale_anim_phase.get(), true);
    }

    fn paint_eval_status_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color, dashed: bool) {
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, color, 0.0, dashed);
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
    fn paint_note_caret_bar(scene: &mut canvas::Scene, aff: &canvas::Affine, node: &DagNodeSpec, caret_byte: usize, text: &str, font_px: f64, fill: canvas::Color, zoom: f64) {
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

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_node_visual(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod, lod_index: usize, node: &DagNodeSpec, chrome: DagNodePaintChrome) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::FillRule;
        use canvas::{Point, Rect, Stroke};

        let theme = &self.canvas_theme;
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
            DagNodeEvalStatusKind::Computing => self.paint_computing_active_border(scene, aff, &rect, cam.zoom, theme.node_stroke_computing),
            DagNodeEvalStatusKind::Stale | DagNodeEvalStatusKind::Queued => self.paint_computing_stale_border(scene, aff, &rect, cam.zoom, theme.node_stroke_stale),
            DagNodeEvalStatusKind::Error => self.paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_error, false),
            DagNodeEvalStatusKind::Blocked => self.paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_blocked, true),
            DagNodeEvalStatusKind::Ok => {}
        }
        let layout_px = dag_label_layout_px();
        let paint_px = dag_label_paint_px(cam.zoom, lod_index);
        let chrome_stroke = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let center_screen = world_to_screen(cam, viewport, Point::new(node.x, node.y));
        let node_fill = fill.unwrap_or(theme.node_fill);
        if !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            self.paint_node_lod_icon(scene, lod, center_screen, node, cam.zoom, label_fill, node_fill);
        }
        let label_text = Self::node_label_text(node, lod);
        let caption_on_overlay = Self::node_caption_delegated_to_js_overlay(node, lod);
        if lod.node_label_is_horizontal() {
            if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                Self::paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
            }
        } else {
            match &node.kind {
                DagNodeKind::Computation { .. } | DagNodeKind::Cluster { .. } => {
                    if lod.shows_computation_layout() {
                        let channel_row_pick = lod.uses_channel_row_pick();
                        if channel_row_pick {
                            self.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
                        }
                        Self::paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_chrome_stroke);
                        self.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_chrome_stroke, stroke, label_fill, channel_row_pick);
                        if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                            Self::paint_computation_node_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                        }
                        if matches!(node.kind, DagNodeKind::Cluster { .. }) {
                            Self::paint_cluster_affordances(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                        }
                        if let DagNodeKind::Computation { variadic_inputs, variadic_outputs, .. } = &node.kind {
                            if cam.zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD && (*variadic_inputs || *variadic_outputs) {
                                Self::paint_variadic_plus_controls(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                            }
                        }
                    }
                }
                DagNodeKind::Slider { value, .. } => {
                    if let Some(label) = label_text.filter(|_| lod.shows_controls() && !caption_on_overlay) {
                        Self::paint_slider_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                    }
                    // 🎚️ Track + knob live on the HTML `GraphSliderOverlays` control — painting them here
                    // doubles a pixelated GPU ghost above the crisp DOM slider. Only the left value readout
                    // stays on the GPU so the overlay can stay track-bounds-aligned (`showValue={false}`).
                    if lod.shows_controls() {
                        let value_text = format!("{value:.1}");
                        let value_px = paint_px * 0.9;
                        let (vx, vy) = slider_value_world_center(node, &value_text, value_px, cam.zoom);
                        let value_pos = world_to_screen(cam, viewport, Point::new(vx, vy));
                        append_label(scene, &value_text, value_pos, value_px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Select { options, selected, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
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
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
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
                    if lod.shows_detail_text() || lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let font_px = paint_px * 1.05;
                        let text_x = x0 + DAG_PREVIEW_PAD;
                        let max_w = (x1 - text_x).max(1.0);
                        let display = if text.is_empty() { "…" } else { text.as_str() };
                        let shown = truncate_label_to_fit_width(display, max_w, font_px);
                        let pos = world_to_screen(cam, viewport, Point::new(text_x, (y0 + y1) * 0.5));
                        append_label(scene, &shown, pos, font_px, label_fill, label_halo);
                        if let Some(edit) = &self.editing_note {
                            if edit.node_id == node.id && self.caret_visible {
                                Self::paint_note_caret_bar(scene, aff, node, edit.caret, text, font_px, label_fill, cam.zoom);
                            }
                        }
                    }
                }
                DagNodeKind::Image { src, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let frame = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &frame);
                    }
                    if lod.shows_detail_text() || lod.shows_controls() {
                        self.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, theme.raster_clear);
                    }
                }
                DagNodeKind::Preview { content, expanded, .. } => {
                    if lod.shows_detail_text() || lod.shows_controls() {
                        self.paint_preview_content(scene, cam, viewport, node, content, expanded, paint_px, label_fill, label_halo, theme.raster_clear);
                    }
                }
                DagNodeKind::Action { label, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
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
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
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
                        Self::paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
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

    pub fn paint_scene(&self, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use canvas::camera::{camera_content_affine, Camera as CanvasCamera, Viewport};
        use canvas::FillRule;
        use canvas::{Circle, Rect, Stroke};

        let theme = &self.canvas_theme;
        self.tick_computing_animation();
        let cam = CanvasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let lod = self.draw_lod_for_frame();
        let lod_index = dag_lod_index(cam.zoom);
        let lod_index_i8 = lod_index as i8;
        let prev_lod = self.last_logged_lod.get();
        if prev_lod != lod_index_i8 {
            self.last_logged_lod.set(lod_index_i8);
            dag_debug_log(&format!("[DEBUG] dag draw lod={} zoom={:.3} icon={} label={:?}", lod.label(), cam.zoom, lod.node_icon_visible(), lod.node_label()));
        }
        self.paint_lod_grid(scene, &cam, &viewport, lod);
        let snap = self.engine.render_snapshot();
        let edge_stroke = dag_world_stroke(lod.edge_stroke_screen_px(), cam.zoom);
        for &eid in self.engine.edges.keys() {
            let (is_selected, is_highlighted, is_hovered) = self.edge_interaction_chrome(eid);
            let stroke_c = dag_edge_body_stroke(theme, false, is_selected, is_highlighted, is_hovered);
            let style = self.edge_route_style.get(&eid).copied().unwrap_or_default();
            match style {
                EdgeRouteStyle::Bezier => {
                    if let Some(curve) = self.engine.edge_curve(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &curve);
                    }
                }
                EdgeRouteStyle::SharpSz => {
                    if let Some(path) = self.edge_sharp_path(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &path);
                    }
                }
            }
        }
        if let Some(preview) = snap.pending_edge {
            scene.stroke(&Stroke::new(edge_stroke), aff, dag_edge_body_stroke(theme, false, true, false, false), None, &preview);
        }
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let paint_snap_handle = |scene: &mut canvas::Scene, hid: &HandleId, center: &canvas::Point, shape_filter: Option<PortShape>| {
            if self.handle_port_visible.get(hid) == Some(&false) {
                return;
            }
            let shape = self.handle_port_shape.get(hid).copied().unwrap_or_default();
            if let Some(filter) = shape_filter {
                if shape != filter {
                    return;
                }
            }
            if !lod.shows_handles() && shape != PortShape::Triangle {
                return;
            }
            let node_id = self.engine.handles.get(hid).map(|handle| handle.node_id);
            let is_dimmed = node_id.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = self.handle_interaction_chrome(*hid);
            let fill = dag_handle_body_fill(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let stroke_c = dag_handle_body_stroke(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let chrome = is_dimmed || is_selected || is_highlighted || is_hovered;
            let outward = node_id.and_then(|nid| self.engine.nodes.get(&nid).and_then(|node| handle_outward_at_node_rim(*center, node.center, node.shape, node.radius, node.width, node.height)));
            if let Some(out) = outward {
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &self.handle_cap_fill_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &self.handle_cap_stroke_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
            } else {
                let circle = Circle::new(*center, DAG_HANDLE_WORLD_RADIUS);
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &circle);
            }
        };
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Semicircle));
        }
        let paint_minimap_node = |scene: &mut canvas::Scene, idx: usize, fixture_node: &DagNodeSpec| {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let node = node.as_ref();
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.node_interaction_chrome(nid));
            if let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) {
                scene.fill(FillRule::NonZero, aff, fill, None, &rect);
            }
        };
        if lod == DagDrawLod::Minimap {
            for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
                let engine_nid = self.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if !chrome {
                    paint_minimap_node(scene, idx, fixture_node);
                }
            }
            for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
                let engine_nid = self.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if chrome {
                    paint_minimap_node(scene, idx, fixture_node);
                }
            }
        } else {
            for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
                let node = self.node_spec_for_paint(idx, fixture_node);
                let node = node.as_ref();
                let engine_nid = self.engine_node_id_for_index(idx);
                let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
                let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.node_interaction_chrome(nid));
                let eval_status = engine_nid.and_then(|nid| self.node_eval_status.get(&nid).copied()).unwrap_or(DagNodeEvalStatusKind::Ok);
                self.paint_node_visual(scene, &aff, &cam, &viewport, lod, lod_index, node, DagNodePaintChrome { is_dimmed, is_selected, is_highlighted, is_hovered, eval_status, body_fill_alpha: 255, ghost_tint: false });
            }
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            self.paint_node_visual(scene, &aff, &cam, &viewport, lod, lod_index, ghost, DagNodePaintChrome::ghost_preview());
        }
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Triangle));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            let paint_ghost_handles = lod.shows_handles() || ghost.inputs().iter().any(|port| port.shape == PortShape::Triangle) || ghost.outputs().iter().any(|port| port.shape == PortShape::Triangle);
            if paint_ghost_handles {
                self.paint_node_handles_for_spec(scene, &aff, &cam, ghost, &DagNodePaintChrome::ghost_preview());
            }
        }
        self.paint_minimap_widget(scene, viewport_w, viewport_h);
    }
}
// #endregion 🔖️DagHost

// #region 🔖️WasmSession
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this session bridge is browser-only
// (attaches an `HtmlCanvasElement`), so it is narrowed to exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_session {
    use super::*;
    use semio_framework_async::browser::future_to_promise;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;

    struct DagSessionInner {
        host: DagHost,
        gpu: canvas::gpu_session::CanvasGpuSession,
        width: u32,
        height: u32,
        dpr: f64,
    }

    #[wasm_bindgen]
    pub struct DagSession {
        state: Rc<RefCell<DagSessionInner>>,
    }

    #[wasm_bindgen]
    impl DagSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self { state: Rc::new(RefCell::new(DagSessionInner { host: DagHost::default_demo(), gpu: canvas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
        }

        #[wasm_bindgen(js_name = loadFixtureJson)]
        pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
            let host = DagHost::load_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = nodeOverlaysJson)]
        pub fn node_overlays_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
        pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = attachCanvas)]
        pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
            let inner = self.state.clone();
            let lw = logical_w.max(1);
            let lh = logical_h.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            future_to_promise(async move {
                let (render_ctx, renderer, surface) = canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                g.width = lw;
                g.height = lh;
                g.dpr = dpr;
                g.host.set_viewport(lw, lh, dpr);
                g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
                Ok(JsValue::UNDEFINED)
            })
        }

        #[wasm_bindgen(js_name = gpuReady)]
        pub fn gpu_ready(&self) -> bool {
            self.state.borrow().gpu.gpu_ready()
        }

        #[wasm_bindgen(js_name = setSize)]
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            let mut inner = self.state.borrow_mut();
            inner.width = width.max(1);
            inner.height = height.max(1);
            inner.dpr = dpr.max(1.0);
            let (w, h, d) = (inner.width, inner.height, inner.dpr);
            inner.host.set_viewport(w, h, d);
            let pw = ((w as f64 * d).round() as u32).max(1);
            let ph = ((h as f64 * d).round() as u32).max(1);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = setCamera)]
        pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
            self.state.borrow_mut().host.set_camera(x, y, zoom);
        }

        #[wasm_bindgen(js_name = setWheelZoomActive)]
        pub fn set_wheel_zoom_active(&self, active: bool) {
            self.state.borrow_mut().host.set_wheel_zoom_active(active);
        }

        #[wasm_bindgen(js_name = lodScaleJson)]
        pub fn lod_scale_json(&self) -> String {
            dag_lod_scale_json()
        }

        #[wasm_bindgen(js_name = setAutomaticLod)]
        pub fn set_automatic_lod(&self, enabled: bool) {
            self.state.borrow_mut().host.set_automatic_lod(enabled);
        }

        #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
        pub fn set_forced_draw_lod_label(&self, label: &str) {
            self.state.borrow_mut().host.set_forced_draw_lod_label(label);
        }

        #[wasm_bindgen(js_name = drawLodLabel)]
        pub fn draw_lod_label(&self) -> String {
            self.state.borrow().host.draw_lod_label().to_string()
        }

        #[wasm_bindgen(js_name = pointerDown)]
        pub fn pointer_down(&self, x: f64, y: f64, extend: bool) {
            self.state.borrow_mut().host.pointer_down(x, y, extend);
        }

        #[wasm_bindgen(js_name = pointerMove)]
        pub fn pointer_move(&self, x: f64, y: f64) {
            self.state.borrow_mut().host.pointer_move(x, y);
        }

        #[wasm_bindgen(js_name = pointerUp)]
        pub fn pointer_up(&self, x: f64, y: f64) {
            self.state.borrow_mut().host.pointer_up(x, y);
        }

        #[wasm_bindgen(js_name = takePendingOpenInstanceId)]
        pub fn take_pending_open_instance_id(&self) -> Option<String> {
            dag_take_pending_open_instance_id(&mut self.state.borrow_mut().host)
        }

        #[wasm_bindgen(js_name = screenToWorld)]
        pub fn screen_to_world(&self, x: f64, y: f64) -> js_sys::Array {
            let (wx, wy) = dag_screen_to_world(&self.state.borrow().host, x, y);
            let out = js_sys::Array::new();
            out.push(&JsValue::from_f64(wx));
            out.push(&JsValue::from_f64(wy));
            out
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
            let opts = if options_json.trim().is_empty() { DagLayoutOptions::default() } else { dsl::os_pack::json::from_json_str(options_json).unwrap_or_default() };
            self.state.borrow_mut().host.reorganize(&opts).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = setCanvasThemeJson)]
        pub fn set_canvas_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let mut scene = canvas::Scene::new();
            let clear = inner.host.canvas_theme.raster_clear;
            inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            let scene = canvas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
            inner.gpu.render_frame(&scene, clear)
        }
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use wasm_session::DagSession;
// #endregion 🔖️WasmSession

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

//#region 🔖️WasmBridge
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this is a browser-only
// wasm-bindgen bridge, so it is narrowed to exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DagSnapshotVcs {
        store: RefCell<DagStore>,
    }

    #[wasm_bindgen]
    impl DagSnapshotVcs {
        /// 🌐️ Constructs the VCS bridge without synchronously blocking the browser host callback.
        #[wasm_bindgen(js_name = create)]
        pub async fn create() -> Result<DagSnapshotVcs, JsValue> {
            let store = create_dag_store("dag", empty_dag_document()).await.map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub async fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            let mut store = self.store.try_borrow_mut().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.dispatch_text(command_text).await.map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub async fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            let mut store = self.store.try_borrow_mut().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.dispatch_binary(command_bytes).await.map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = snapshotJson)]
        pub async fn snapshot_json(&self) -> Result<String, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub async fn envelope_json(&self) -> Result<String, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            store.envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub async fn generation(&self) -> Result<u32, JsValue> {
            let store = self.store.try_borrow().map_err(|_| JsValue::from_str("DAG VCS operation already in progress"))?;
            Ok(store.generation() as u32)
        }
    }
}
//#endregion 🔖️WasmBridge
