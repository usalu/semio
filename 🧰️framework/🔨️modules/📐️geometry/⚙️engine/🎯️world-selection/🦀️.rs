//! 🎯️ World selection of the geometry primitives: the world box, its inflation, containment and intersection tests and the cubic bezier selection helpers. See [`super`].

use crate::{CubicBez, Point, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct WorldBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

pub fn inflate_world_box(b: WorldBox, pad: f64) -> WorldBox {
    WorldBox { min_x: b.min_x - pad, min_y: b.min_y - pad, max_x: b.max_x + pad, max_y: b.max_y + pad }
}

pub fn world_boxes_overlap(a: WorldBox, b: WorldBox) -> bool {
    a.min_x <= b.max_x && a.max_x >= b.min_x && a.min_y <= b.max_y && a.max_y >= b.min_y
}

pub fn world_box_contains_point(b: WorldBox, p: Point) -> bool {
    p.x >= b.min_x && p.x <= b.max_x && p.y >= b.min_y && p.y <= b.max_y
}

pub fn world_box_contains_box(outer: WorldBox, inner: WorldBox) -> bool {
    inner.min_x >= outer.min_x && inner.max_x <= outer.max_x && inner.min_y >= outer.min_y && inner.max_y <= outer.max_y
}

fn world_box_corners(b: WorldBox) -> [Point; 4] {
    [Point::new(b.min_x, b.min_y), Point::new(b.max_x, b.min_y), Point::new(b.max_x, b.max_y), Point::new(b.min_x, b.max_y)]
}

pub fn world_box_from_points(points: &[Point]) -> Option<WorldBox> {
    if points.is_empty() {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for p in points {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }
    Some(WorldBox { min_x, min_y, max_x, max_y })
}

pub fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[j];
        let crosses = (a.y > point.y) != (b.y > point.y);
        if crosses && point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x {
            inside = !inside;
        }
        j = i;
    }
    inside
}

    /// 🔲️ Includes a polygon's boundary in selection containment.
    pub fn polygon_covers_point(polygon: &[Point], point: Point) -> bool {
        polygon.len() >= 3 && (point_in_polygon(point, polygon) || (0..polygon.len()).any(|i| point_on_segment(point, polygon[i], polygon[(i + 1) % polygon.len()])))
    }

    /// 📏️ Encloses every interval of a segment, including concave polygon crossings.
    pub fn polygon_contains_segment(polygon: &[Point], start: Point, end: Point) -> bool {
        if !polygon_covers_point(polygon, start) || !polygon_covers_point(polygon, end) {
            return false;
        }
        let delta = end - start;
        let length_squared = delta.dot(delta);
        if length_squared <= 1e-18 {
            return true;
        }
        let mut parameters = vec![0.0, 1.0];
        for i in 0..polygon.len() {
            let a = polygon[i];
            let b = polygon[(i + 1) % polygon.len()];
            let edge = b - a;
            let offset = a - start;
            let cross = |left: Vec2, right: Vec2| left.x * right.y - left.y * right.x;
            let denominator = cross(delta, edge);
            if denominator.abs() > 1e-12 {
                let t = cross(offset, edge) / denominator;
                let u = cross(offset, delta) / denominator;
                if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
                    parameters.push(t);
                }
            } else if cross(offset, delta).abs() <= 1e-9 {
                parameters.extend([offset.dot(delta) / length_squared, (b - start).dot(delta) / length_squared].into_iter().filter(|t| (0.0..=1.0).contains(t)));
            }
        }
        parameters.sort_by(f64::total_cmp);
        parameters.windows(2).all(|interval| polygon_covers_point(polygon, start + delta * ((interval[0] + interval[1]) * 0.5)))
    }

    /// 🧩️ Encloses an entire outline instead of its bounds or representative point.
    pub fn polygon_contains_polygon(selection: &[Point], outline: &[Point]) -> bool {
        outline.len() >= 3 && (0..outline.len()).all(|i| polygon_contains_segment(selection, outline[i], outline[(i + 1) % outline.len()]))
    }

    /// 🕳️ Crosses a filled polygon region while excluding selections wholly inside holes.
    pub fn polygon_region_intersects_polygon(outline: &[Point], holes: &[Vec<Point>], selection: &[Point]) -> bool {
        if outline.len() < 3 || selection.len() < 3 {
            return false;
        }
        if (0..outline.len()).any(|i| segment_intersects_polygon(outline[i], outline[(i + 1) % outline.len()], selection)) {
            return true;
        }
        if holes.iter().any(|hole| (0..hole.len()).any(|i| segment_intersects_polygon(hole[i], hole[(i + 1) % hole.len()], selection))) {
            return true;
        }
        selection.iter().any(|&point| polygon_covers_point(outline, point) && !holes.iter().any(|hole| point_in_polygon(point, hole) && !polygon_boundary_contains_point(hole, point)))
    }

    fn polygon_boundary_contains_point(polygon: &[Point], point: Point) -> bool {
        (0..polygon.len()).any(|i| point_on_segment(point, polygon[i], polygon[(i + 1) % polygon.len()]))
    }

fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
    const EPS: f64 = 1e-9;
    point.x >= start.x.min(end.x) - EPS
        && point.x <= start.x.max(end.x) + EPS
        && point.y >= start.y.min(end.y) - EPS
        && point.y <= start.y.max(end.y) + EPS
        && ((end.x - start.x) * (point.y - start.y) - (end.y - start.y) * (point.x - start.x)).abs() <= EPS
}

fn orientation(a: Point, b: Point, c: Point) -> i8 {
    let v = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);
    if v > 1e-9 {
        1
    } else if v < -1e-9 {
        -1
    } else {
        0
    }
}

fn segments_intersect(a0: Point, a1: Point, b0: Point, b1: Point) -> bool {
    let o1 = orientation(a0, a1, b0);
    let o2 = orientation(a0, a1, b1);
    let o3 = orientation(b0, b1, a0);
    let o4 = orientation(b0, b1, a1);
    if o1 != o2 && o3 != o4 {
        return true;
    }
    point_on_segment(b0, a0, a1) || point_on_segment(b1, a0, a1) || point_on_segment(a0, b0, b1) || point_on_segment(a1, b0, b1)
}

fn world_box_edges(box_: WorldBox) -> [(Point, Point); 4] {
    let [a, b, c, d] = world_box_corners(box_);
    [(a, b), (b, c), (c, d), (d, a)]
}

pub fn segment_intersects_world_box(start: Point, end: Point, box_: WorldBox) -> bool {
    if world_box_contains_point(box_, start) || world_box_contains_point(box_, end) {
        return true;
    }
    let edges = world_box_edges(box_);
    for (a, b) in edges {
        if segments_intersect(start, end, a, b) {
            return true;
        }
    }
    false
}

fn polygon_segments(polygon: &[Point]) -> Vec<(Point, Point)> {
    if polygon.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(polygon.len());
    for i in 0..polygon.len() {
        out.push((polygon[i], polygon[(i + 1) % polygon.len()]));
    }
    out
}

pub fn polygon_contains_world_box(polygon: &[Point], box_: WorldBox) -> bool {
    polygon_contains_polygon(polygon, &world_box_corners(box_))
}

pub fn polygon_intersects_world_box(polygon: &[Point], box_: WorldBox) -> bool {
    let corners = world_box_corners(box_);
    for p in corners {
        if point_in_polygon(p, polygon) {
            return true;
        }
    }
    for &p in polygon {
        if world_box_contains_point(box_, p) {
            return true;
        }
    }
    let segments = polygon_segments(polygon);
    for (s, e) in segments {
        if segment_intersects_world_box(s, e, box_) {
            return true;
        }
    }
    false
}

pub fn segment_intersects_polygon(start: Point, end: Point, polygon: &[Point]) -> bool {
    if polygon.len() < 3 {
            return false;
        }
        if polygon_covers_point(polygon, start) || polygon_covers_point(polygon, end) {
        return true;
    }
    for i in 0..polygon.len() {
            let (a, b) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        if segments_intersect(start, end, a, b) {
            return true;
        }
    }
    false
}

pub fn cubic_bezier_axis_bounds(c: CubicBez) -> WorldBox {
    let xs = [c.p0.x, c.p1.x, c.p2.x, c.p3.x];
    let ys = [c.p0.y, c.p1.y, c.p2.y, c.p3.y];
    WorldBox {
        min_x: xs.iter().copied().fold(f64::INFINITY, f64::min),
        max_x: xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        min_y: ys.iter().copied().fold(f64::INFINITY, f64::min),
        max_y: ys.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    }
}

pub fn cubic_bezier_point(c: CubicBez, t: f64) -> Point {
    c.eval(t.clamp(0.0, 1.0))
}
