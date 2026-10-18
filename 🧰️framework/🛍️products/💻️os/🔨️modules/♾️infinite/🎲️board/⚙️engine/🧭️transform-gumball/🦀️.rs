//! 🧭️ Transform gumball of the board engine: the ring constants, gumball flags and the pivot, rotation and snap helpers. See [`super`].

use super::*;

// #region 🔖️TransformGumball
pub const TRANSFORM_RING_GAP_PX: f64 = ui_styling::metrics::board::TRANSFORM_RING_GAP_PX;

pub const TRANSFORM_RING_MIN_RADIUS_PX: f64 = ui_styling::metrics::board::TRANSFORM_RING_MIN_RADIUS_PX;

pub const TRANSFORM_RING_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::TRANSFORM_RING_HIT_TOLERANCE_PX;

pub const TRANSFORM_ROTATE_SNAP_RADIANS: f64 = ui_styling::metrics::board::TRANSFORM_ROTATE_SNAP_DEGREES * std::f64::consts::PI / 180.0;

/// 🕹️ Which selection-gumball handles a board surface offers. `move` is the board's native node
/// drag, `rotate` the ring around the selection centroid; scale is deliberately absent — a board
/// node's size comes from its kind catalog, never from a free drag (same law as puzzle-3d's gumball).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransformGumballFlags {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for TransformGumballFlags {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}

/// 📐️ The arithmetic mean of the selected node centres — the pivot both the in-canvas rotate
/// preview and the guest's `rotateSelection` reducer turn about, so the preview and the committed
/// document agree exactly. Locked members still count toward the pivot, only their positions stay.
pub fn transform_pivot_of(centers: &[Point]) -> Option<Point> {
    if centers.is_empty() {
        return None;
    }
    let count = centers.len() as f64;
    let sum = centers.iter().fold((0.0, 0.0), |(x, y), p| (x + p.x, y + p.y));
    Some(Point::new(sum.0 / count, sum.1 / count))
}

/// 🔄️ Turns `point` about `pivot` by `radians` counter-clockwise in board world space.
pub fn rotate_point_about(pivot: Point, point: Point, radians: f64) -> Point {
    let (sin, cos) = radians.sin_cos();
    let dx = point.x - pivot.x;
    let dy = point.y - pivot.y;
    Point::new(pivot.x + dx * cos - dy * sin, pivot.y + dx * sin + dy * cos)
}

/// ⭕️ World radius of the rotate ring: the farthest selected corner plus a constant screen gap, so
/// the grab ring keeps the same pixel clearance at every zoom.
pub fn transform_ring_radius_world(pivot: Point, corners: &[Point], zoom: f64) -> f64 {
    let zoom = if zoom.is_finite() && zoom > 1e-9 { zoom } else { 1.0 };
    let reach = corners.iter().fold(0.0_f64, |far, corner| far.max(distance_between(pivot, *corner)));
    (reach + TRANSFORM_RING_GAP_PX / zoom).max(TRANSFORM_RING_MIN_RADIUS_PX / zoom)
}

/// 🎯️ Whether `point` lies on the rotate ring band — the gumball outranks nodes, handles and edges,
/// so this is asked before any other hit test while the ring is drawn.
pub fn transform_ring_hit(pivot: Point, radius_world: f64, zoom: f64, point: Point) -> bool {
    let zoom = if zoom.is_finite() && zoom > 1e-9 { zoom } else { 1.0 };
    (distance_between(pivot, point) - radius_world).abs() <= TRANSFORM_RING_HIT_TOLERANCE_PX / zoom
}

/// 📐️ Signed delta between two ring grabs, normalized to `(-π, π]` so a drag across the seam does
/// not jump a full turn.
pub fn transform_ring_angle_delta(pivot: Point, from: Point, to: Point) -> f64 {
    let start = (from.y - pivot.y).atan2(from.x - pivot.x);
    let end = (to.y - pivot.y).atan2(to.x - pivot.x);
    let mut delta = end - start;
    while delta > std::f64::consts::PI {
        delta -= std::f64::consts::TAU;
    }
    while delta <= -std::f64::consts::PI {
        delta += std::f64::consts::TAU;
    }
    delta
}

/// 🧲️ Quantizes a live rotation to the snap step when the grid-snap modifier is on.
pub fn snap_transform_angle(radians: f64, snap: bool) -> f64 {
    if !snap || !radians.is_finite() {
        return radians;
    }
    (radians / TRANSFORM_ROTATE_SNAP_RADIANS).round() * TRANSFORM_ROTATE_SNAP_RADIANS
}
// #endregion 🔖️TransformGumball
