//! 🧭️ Grid step, port keys and tree toggle of the DAG model: the world grid stroke step, snapshot edge ids, port endpoint parts and keys, the preview tree path toggle and the board engine alias. See [`super`].

use super::*;

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
