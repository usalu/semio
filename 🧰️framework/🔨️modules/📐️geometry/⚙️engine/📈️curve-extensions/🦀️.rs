//! 📈️ Curve extensions of the geometry primitives: the cubic bezier evaluation, split, arc length and nearest parameter and the segment and circle line intersections. See [`super`].

use super::*;

// #region 🔖️CurveExtensions
// 🧮️ Cubic-bezier subdivision, length, nearest-point, and segment/circle intersection.

/// 📐️ De Casteljau evaluation of a cubic bezier at parameter `t`.
pub fn cubic_point_at(c: CubicBez, t: f64) -> Point {
    c.eval(clamp_f64(t, 0.0, 1.0))
}

pub(super) fn lerp_point(a: Point, b: Point, t: f64) -> Point {
    Point::new(a.x() + (b.x() - a.x()) * t, a.y() + (b.y() - a.y()) * t)
}

/// ✂️ Splits a cubic bezier at parameter `t` via De Casteljau subdivision.
pub fn cubic_split(c: CubicBez, t: f64) -> (CubicBez, CubicBez) {
    let (p0, p1, p2, p3) = (c.p0(), c.p1(), c.p2(), c.p3());
    let a = lerp_point(p0, p1, t);
    let b = lerp_point(p1, p2, t);
    let cc = lerp_point(p2, p3, t);
    let d = lerp_point(a, b, t);
    let e = lerp_point(b, cc, t);
    let f = lerp_point(d, e, t);
    (CubicBez::new(p0, a, d, f), CubicBez::new(f, e, cc, p3))
}

/// 📏️ Polyline-approximated arc length of a cubic bezier.
pub fn cubic_arc_length(c: CubicBez, segments: usize) -> f64 {
    let n = segments.max(1);
    let mut total = 0.0;
    let mut previous = c.eval(0.0);
    for i in 1..=n {
        let t = i as f64 / n as f64;
        let next = c.eval(t);
        total += distance_between(previous, next);
        previous = next;
    }
    total
}

/// 🎯️ Parameter `t` of the sampled point on a cubic bezier nearest to `point`.
pub fn cubic_nearest_t(point: Point, c: CubicBez, segments: usize) -> f64 {
    let n = segments.max(1);
    let mut best_t = 0.0;
    let mut best_dist = f64::INFINITY;
    for i in 0..=n {
        let t = i as f64 / n as f64;
        let d = distance_between(point, c.eval(t));
        if d < best_dist {
            best_dist = d;
            best_t = t;
        }
    }
    best_t
}

/// 📍️ Intersection point of two line segments, if any (parametric line-line solve).
pub fn segment_intersection(a0: Point, a1: Point, b0: Point, b1: Point) -> Option<Point> {
    let (x1, y1, x2, y2) = (a0.x(), a0.y(), a1.x(), a1.y());
    let (x3, y3, x4, y4) = (b0.x(), b0.y(), b1.x(), b1.y());
    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denom.abs() <= f64::EPSILON {
        return None;
    }
    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / denom;
    if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) {
        return None;
    }
    Some(Point::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
}

/// ⭕️ Up to two intersection points of a circle and a line segment.
pub fn circle_line_intersections(center: Point, r: f64, p0: Point, p1: Point) -> Vec<Point> {
    let d = Vec2::new(p1.x() - p0.x(), p1.y() - p0.y());
    let f = Vec2::new(p0.x() - center.x(), p0.y() - center.y());
    let a = d.dot(d);
    if a <= f64::EPSILON {
        return Vec::new();
    }
    let b = 2.0 * f.dot(d);
    let c = f.dot(f) - r * r;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return Vec::new();
    }
    let sq = disc.sqrt();
    let mut out = Vec::new();
    for t in [(-b - sq) / (2.0 * a), (-b + sq) / (2.0 * a)] {
        if (0.0..=1.0).contains(&t) {
            out.push(Point::new(p0.x() + t * d.x(), p0.y() + t * d.y()));
        }
    }
    out
}
// #endregion 🔖️CurveExtensions
