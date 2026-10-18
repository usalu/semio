//! 🧮️ Computation columns and fit of the DAG model: the variadic plus threshold, label and divider columns, row divider spans and indices, name centres, row counts, node fitting, insert positions and I/O port angles. See [`super`].

use super::*;

pub const DAG_VARIADIC_PLUS_ZOOM_THRESHOLD: f64 = 1.5;

pub(super) fn computation_input_label_x(node: &DagNodeSpec) -> f64 {
    node.x - node.width * 0.5 + DAG_NODE_EDGE_INSET
}

pub(super) fn computation_output_label_x(node: &DagNodeSpec, label: &str, px: f64) -> f64 {
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

pub(super) fn io_widget_name_column_bounds(node: &DagNodeSpec, px: f64) -> (f64, f64, f64, f64) {
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

pub(super) fn port_angle_on_side(index: usize, count: usize, left: bool) -> f64 {
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
    use semio_framework_graph_drawing::routing::rectangle_handle_angle_toward;
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
    use semio_framework_graph_drawing::routing::rectangle_handle_angle_toward;
    let hw = node.width * 0.5;
    let count = if left { node.inputs().len() } else { node.outputs().len() };
    let port_y = port_center_y(node, port_index, count);
    let port_x = if left { node.x - hw } else { node.x + hw };
    rectangle_handle_angle_toward(Point::new(node.x, node.y), node.width, node.height, Point::new(port_x, port_y))
}
