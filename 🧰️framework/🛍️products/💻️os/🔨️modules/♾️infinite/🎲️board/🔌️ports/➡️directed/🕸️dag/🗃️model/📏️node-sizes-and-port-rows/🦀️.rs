//! 📏️ Node sizes and port rows of the DAG model: the layout constants, the channel row side, the computation and widget sizes, the row and connector bounds and the port centres. See [`super`].

use super::*;

pub(super) const DAG_NODE_EDGE_INSET: f64 = ui_styling::metrics::dag::NODE_EDGE_INSET;

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

pub(super) fn io_widget_label_center(node: &DagNodeSpec) -> (f64, f64) {
    let hh = node.height * 0.5;
    (node.x, node.y - hh * 0.2)
}

pub(super) fn channel_row_center_y(node_y: f64, node_height: f64, row_index: usize) -> f64 {
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

pub(super) fn computation_port_center_y(node: &DagNodeSpec, port_index: usize) -> f64 {
    channel_row_center_y(node.y, node.height, port_index + DAG_COMPUTATION_HEADER_ROWS)
}

fn proportional_port_center_y(node: &DagNodeSpec, port_index: usize, count: usize) -> f64 {
    let hh = node.height * 0.5;
    let t = (port_index as f64 + 0.5) / count.max(1) as f64;
    node.y - hh + t * node.height
}

pub(super) fn port_center_y(node: &DagNodeSpec, port_index: usize, count: usize) -> f64 {
    if uses_computation_layout(&node.kind) {
        computation_port_center_y(node, port_index)
    } else {
        proportional_port_center_y(node, port_index, count)
    }
}
