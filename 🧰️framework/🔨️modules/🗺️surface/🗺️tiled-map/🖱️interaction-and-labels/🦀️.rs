//! 🖱️ Map interaction plan, geometry hit-test helpers and screen-space label declutter.
//!
//! Plans are produced and committed by [`super::MapHost`]; the geometry helpers back its feature picking.

use super::*;

// #region 🔖️MapContent
//#region 🔖️InteractionPlan
#[derive(Clone, Copy, Debug)]
pub enum MapInteractionIntent {
    Wheel { sx: f64, sy: f64, delta_y: f64 },
    PointerDown { sx: f64, sy: f64, button: u8 },
    PointerMove { sx: f64, sy: f64 },
    PointerUp { sx: f64, sy: f64 },
}

#[derive(Clone, Debug)]
pub struct MapInteractionPlan {
    pub(super) revision: u64,
    pub(super) expected_camera: camera::Camera,
    pub(super) expected_interaction: MapInteraction,
    pub(super) next_camera: camera::Camera,
    pub(super) next_interaction: MapInteraction,
    pub(super) emit_camera: bool,
}

impl MapInteractionPlan {
    pub fn camera(&self) -> [f64; 3] {
        [self.next_camera.x, self.next_camera.y, self.next_camera.zoom]
    }
}

pub(super) fn map_camera_matches(left: &camera::Camera, right: &camera::Camera) -> bool {
    left.x.to_bits() == right.x.to_bits() && left.y.to_bits() == right.y.to_bits() && left.zoom.to_bits() == right.zoom.to_bits()
}

pub(super) fn map_interaction_matches(left: &MapInteraction, right: &MapInteraction) -> bool {
    match (left, right) {
        (MapInteraction::None, MapInteraction::None) => true,
        (MapInteraction::Pan { origin: left_origin, start_screen: left_start }, MapInteraction::Pan { origin: right_origin, start_screen: right_start }) => {
            map_camera_matches(left_origin, right_origin) && left_start.x.to_bits() == right_start.x.to_bits() && left_start.y.to_bits() == right_start.y.to_bits()
        }
        _ => false,
    }
}
//#endregion 🔖️InteractionPlan

pub(super) fn map_point_segment_distance(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len2 = dx * dx + dy * dy;
    if len2 < 1e-12 {
        return ((px - x0).powi(2) + (py - y0).powi(2)).sqrt();
    }
    let t = ((px - x0) * dx + (py - y0) * dy) / len2;
    let t = t.clamp(0.0, 1.0);
    let qx = x0 + t * dx;
    let qy = y0 + t * dy;
    ((px - qx).powi(2) + (py - qy).powi(2)).sqrt()
}

fn map_segments_intersect_rect(start: Point, end: Point, min: Point, max: Point) -> bool {
    let Point { x: x0, y: y0 } = start;
    let Point { x: x1, y: y1 } = end;
    let Point { x: min_x, y: min_y } = min;
    let Point { x: max_x, y: max_y } = max;
    if (x0 >= min_x && x0 <= max_x && y0 >= min_y && y0 <= max_y) || (x1 >= min_x && x1 <= max_x && y1 >= min_y && y1 <= max_y) {
        return true;
    }
    let edges = [(min_x, min_y, max_x, min_y), (max_x, min_y, max_x, max_y), (max_x, max_y, min_x, max_y), (min_x, max_y, min_x, min_y)];
    for (ax, ay, bx, by) in edges {
        if map_segments_intersect(start, end, Point::new(ax, ay), Point::new(bx, by)) {
            return true;
        }
    }
    false
}

fn map_segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let Point { x: ax, y: ay } = a;
    let Point { x: bx, y: by } = b;
    let Point { x: cx, y: cy } = c;
    let Point { x: dx, y: dy } = d;
    fn orient(px: f64, py: f64, qx: f64, qy: f64, rx: f64, ry: f64) -> f64 {
        (qy - py) * (rx - qx) - (qx - px) * (ry - qy)
    }
    fn on_segment(px: f64, py: f64, qx: f64, qy: f64, rx: f64, ry: f64) -> bool {
        rx <= px.max(qx) + 1e-9 && rx + 1e-9 >= px.min(qx) && ry <= py.max(qy) + 1e-9 && ry + 1e-9 >= py.min(qy)
    }
    let o1 = orient(ax, ay, bx, by, cx, cy);
    let o2 = orient(ax, ay, bx, by, dx, dy);
    let o3 = orient(cx, cy, dx, dy, ax, ay);
    let o4 = orient(cx, cy, dx, dy, bx, by);
    if o1 * o2 < 0.0 && o3 * o4 < 0.0 {
        return true;
    }
    if o1.abs() < 1e-9 && on_segment(ax, ay, bx, by, cx, cy) {
        return true;
    }
    if o2.abs() < 1e-9 && on_segment(ax, ay, bx, by, dx, dy) {
        return true;
    }
    if o3.abs() < 1e-9 && on_segment(cx, cy, dx, dy, ax, ay) {
        return true;
    }
    if o4.abs() < 1e-9 && on_segment(cx, cy, dx, dy, bx, by) {
        return true;
    }
    false
}

pub(super) fn map_polyline_intersects_rect(points: &[Point], min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> bool {
    for p in points {
        if p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y {
            return true;
        }
    }
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        if map_segments_intersect_rect(a, b, Point::new(min_x, min_y), Point::new(max_x, max_y)) {
            return true;
        }
    }
    false
}

pub(super) fn map_point_in_polygon(px: f64, py: f64, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    if n < 3 {
        return false;
    }
    let mut j = n - 1;
    for i in 0..n {
        let pi = polygon[i];
        let pj = polygon[j];
        let intersect = (pi.y > py) != (pj.y > py) && px < (pj.x - pi.x) * (py - pi.y) / (pj.y - pi.y + 1e-12) + pi.x;
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

pub(super) fn map_polyline_intersects_polygon(points: &[Point], polygon: &[Point]) -> bool {
    for p in points {
        if map_point_in_polygon(p.x, p.y, polygon) {
            return true;
        }
    }
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let n = polygon.len();
        for i in 0..n {
            let c = polygon[i];
            let d = polygon[(i + 1) % n];
            if map_segments_intersect(a, b, c, d) {
                return true;
            }
        }
    }
    false
}

// #region 🔖️LabelDeclutter
pub(super) struct LabelDeclutter {
    cell: f64,
    cols: usize,
    rows: usize,
    mask: Vec<bool>,
    count: usize,
    max_count: usize,
    width: f64,
    height: f64,
}

impl LabelDeclutter {
    pub(super) fn for_viewport(viewport: &camera::Viewport, cell_px: f64, max_count: usize) -> Self {
        let width = viewport.width.max(1) as f64;
        let height = viewport.height.max(1) as f64;
        let cell = cell_px.max(12.0);
        let cols = ((width / cell).ceil() as usize).max(1) + 1;
        let rows = ((height / cell).ceil() as usize).max(1) + 1;
        Self { cell, cols, rows, mask: vec![false; cols * rows], count: 0, max_count: max_count.max(1), width, height }
    }

    fn estimate_box(label: &str, px: f64, origin: Point) -> (f64, f64, f64, f64) {
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let w = (label.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::MAP_WIDTH_MIN, ui_styling::metrics::label::MAP_WIDTH_MAX);
        let h = (px * 1.6 + pad * 2.0).clamp(14.0, 96.0);
        let x = origin.x;
        let y = origin.y - px * 0.85;
        (x, y, w, h)
    }

    pub(super) fn try_place(&mut self, label: &str, origin: Point, px: f64) -> bool {
        if self.count >= self.max_count {
            return false;
        }
        let (x, y, w, h) = Self::estimate_box(label, px, origin);
        if x + w < 0.0 || y + h < 0.0 || x > self.width || y > self.height {
            return false;
        }
        let cx0 = (x / self.cell).floor().max(0.0) as usize;
        let cy0 = (y / self.cell).floor().max(0.0) as usize;
        let cx1 = ((x + w) / self.cell).ceil().min(self.width) as usize;
        let cy1 = ((y + h) / self.cell).ceil().min(self.height) as usize;
        let cx1 = cx1.min(self.cols.saturating_sub(1));
        let cy1 = cy1.min(self.rows.saturating_sub(1));
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                if self.mask[cy * self.cols + cx] {
                    return false;
                }
            }
        }
        for cy in cy0..=cy1 {
            for cx in cx0..=cx1 {
                self.mask[cy * self.cols + cx] = true;
            }
        }
        self.count += 1;
        true
    }
}
// #endregion 🔖️LabelDeclutter
// #endregion 🔖️MapContent
