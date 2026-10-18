//! 🎚️ Cluster, slider and widget bounds of the DAG model: the cluster explode hit, slider tracks, select, note, preview and action control bounds and the I/O node handle angles. See [`super`].

use super::*;

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
