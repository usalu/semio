//! ➰️ Closed bulged loops: `Vec<Vertex>` where vertex `i` carries the bulge of the segment to vertex `i + 1` (the last one closes to the first).
//!
//! Area, signed area, centroid, perimeter, bounds, orientation, point containment, flattening to a polygon within a chord tolerance, transforms and a mitered raw offset.
//! Containment and area are exact for arcs (circular-segment corrections, no flattening).

use crate::bulge::{intersect, nearest_intersection, BulgeSeg, Extent};
use crate::vector::{cross, LENGTH_EPS};
use crate::{Affine, Point, Rect};

/// 📍️ One loop vertex: a point and the bulge of the segment leaving it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub point: Point,
    pub bulge: f64,
}

impl Vertex {
    /// 📍️ Vertex with an explicit bulge.
    pub fn new(point: Point, bulge: f64) -> Self {
        Self { point, bulge }
    }

    /// 📍️ Vertex leaving along a straight segment.
    pub fn corner(x: f64, y: f64) -> Self {
        Self { point: Point::new(x, y), bulge: 0.0 }
    }
}

/// 🧭️ Where a point lies relative to a loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Containment {
    Inside,
    Boundary,
    Outside,
}

/// 🔷️ Closed loop from straight polygon points.
pub fn from_polygon(points: &[Point]) -> Vec<Vertex> {
    points.iter().map(|&point| Vertex { point, bulge: 0.0 }).collect()
}

/// ✏️ Segments of the closed loop in order.
pub fn segments(vertices: &[Vertex]) -> Vec<BulgeSeg> {
    let n = vertices.len();
    (0..n).map(|i| BulgeSeg { start: vertices[i].point, end: vertices[(i + 1) % n].point, bulge: vertices[i].bulge }).collect()
}

/// 📐️ Signed area (positive counter-clockwise): shoelace plus the circular-segment areas.
pub fn signed_area(vertices: &[Vertex]) -> f64 {
    let n = vertices.len();
    if n < 2 {
        return 0.0;
    }
    segments(vertices).iter().map(|s| 0.5 * (s.start.x * s.end.y - s.end.x * s.start.y) + s.segment_area()).sum()
}

/// 📐️ Absolute area.
pub fn area(vertices: &[Vertex]) -> f64 {
    signed_area(vertices).abs()
}

/// 📏️ Perimeter (sum of arc and line lengths).
pub fn perimeter(vertices: &[Vertex]) -> f64 {
    segments(vertices).iter().map(BulgeSeg::length).sum()
}

/// ⚖️ Area centroid (vertex average for zero-area loops).
pub fn centroid(vertices: &[Vertex]) -> Point {
    if vertices.is_empty() {
        return Point::ZERO;
    }
    let area = signed_area(vertices);
    if area.abs() <= f64::EPSILON {
        let n = vertices.len() as f64;
        return Point::new(vertices.iter().map(|v| v.point.x).sum::<f64>() / n, vertices.iter().map(|v| v.point.y).sum::<f64>() / n);
    }
    let (mut mx, mut my) = (0.0, 0.0);
    for s in segments(vertices) {
        let w = s.start.x * s.end.y - s.end.x * s.start.y;
        mx += (s.start.x + s.end.x) * w / 6.0;
        my += (s.start.y + s.end.y) * w / 6.0;
        let moment = s.segment_first_moment();
        mx += moment.x;
        my += moment.y;
    }
    Point::new(mx / area, my / area)
}

/// 🧮️ Tight bounds including arc extremes; `None` for an empty loop.
pub fn bounds(vertices: &[Vertex]) -> Option<Rect> {
    segments(vertices).iter().map(BulgeSeg::bounds).reduce(|a, b| Rect::new(a.x0().min(b.x0()), a.y0().min(b.y0()), a.x1().max(b.x1()), a.y1().max(b.y1())))
}

/// 🧭️ `true` for counter-clockwise loops (positive signed area).
pub fn is_ccw(vertices: &[Vertex]) -> bool {
    signed_area(vertices) > 0.0
}

/// 🔄️ Same loop traversed backwards (bulges move to the new leaving vertex and flip sign).
pub fn reversed(vertices: &[Vertex]) -> Vec<Vertex> {
    let n = vertices.len();
    (0..n).map(|i| Vertex { point: vertices[(n - i) % n].point, bulge: -vertices[(n - i - 1) % n].bulge }).collect()
}

/// 🧭️ The loop itself when counter-clockwise, otherwise its reverse.
pub fn ccw(vertices: &[Vertex]) -> Vec<Vertex> {
    if is_ccw(vertices) {
        vertices.to_vec()
    } else {
        reversed(vertices)
    }
}

/// 🔁️ Image under a similarity transform; a mirror flips the bulges and the orientation.
pub fn transformed(vertices: &[Vertex], affine: Affine) -> Vec<Vertex> {
    let c = affine.as_coeffs();
    let sign = if c[0] * c[3] - c[1] * c[2] < 0.0 { -1.0 } else { 1.0 };
    vertices.iter().map(|v| Vertex { point: affine * v.point, bulge: v.bulge * sign }).collect()
}

fn chord_winding(points: &[Point], p: Point) -> i32 {
    let n = points.len();
    let mut winding = 0;
    for i in 0..n {
        let (a, b) = (points[i], points[(i + 1) % n]);
        let side = cross(b - a, p - a);
        let side = if side != 0.0 { side } else { b.x - a.x };
        if a.y <= p.y {
            if b.y > p.y && side > 0.0 {
                winding += 1;
            }
        } else if b.y <= p.y && side < 0.0 {
            winding -= 1;
        }
    }
    winding
}

/// 🧭️ Winding number of the loop around `p` (exact for arcs); a point exactly on a chord is resolved as if nudged infinitesimally towards +y. Meaningful when `p` is not on the boundary.
pub fn winding(vertices: &[Vertex], p: Point) -> i32 {
    let points: Vec<Point> = vertices.iter().map(|v| v.point).collect();
    let mut total = chord_winding(&points, p);
    for s in segments(vertices) {
        if let Some(center) = s.center() {
            let d = s.end - s.start;
            let side = cross(d, p - s.start);
            let side = if side != 0.0 { side } else if d.x != 0.0 { d.x } else { -d.y };
            let arc_side = side * s.bulge.signum() < 0.0;
            if arc_side && (p - center).hypot() < s.radius() {
                total += s.bulge.signum() as i32;
            }
        }
    }
    total
}

/// 🧭️ Containment with an explicit boundary tolerance.
pub fn locate(vertices: &[Vertex], p: Point, eps: f64) -> Containment {
    if segments(vertices).iter().any(|s| s.closest(p).distance <= eps) {
        return Containment::Boundary;
    }
    if winding(vertices, p) != 0 {
        Containment::Inside
    } else {
        Containment::Outside
    }
}

/// 🧭️ `true` when `p` is strictly inside (boundary within 1 nm counts as outside).
pub fn contains(vertices: &[Vertex], p: Point) -> bool {
    locate(vertices, p, LENGTH_EPS) == Containment::Inside
}

/// 🔷️ Polygon approximation (no repeated closing point) whose arcs deviate at most `tolerance` from the true curve.
pub fn flatten(vertices: &[Vertex], tolerance: f64) -> Vec<Point> {
    let mut out = Vec::new();
    for s in segments(vertices) {
        out.push(s.start);
        let mut tail = Vec::new();
        s.flatten_into(tolerance, &mut tail);
        tail.pop();
        out.extend(tail);
    }
    out
}

/// ↔️ Mitered raw offset: positive `distance` grows the loop (outward), negative shrinks it, regardless of orientation.
/// Each segment is offset exactly (lines shift, arcs change radius), neighbours are joined at the intersection of the infinite carriers; where the miter exceeds `miter_limit * |distance|` the corner is bevelled.
/// `None` when a segment collapses or the loop inverts. The result is not cleaned of self-intersections; use polygon offset (`semio-framework-2d`) for general cleanup.
pub fn offset(vertices: &[Vertex], distance: f64, miter_limit: f64) -> Option<Vec<Vertex>> {
    let n = vertices.len();
    if n < 2 {
        return None;
    }
    let left = if is_ccw(vertices) { -distance } else { distance };
    let source = segments(vertices);
    let shifted: Vec<BulgeSeg> = source.iter().map(|s| s.offset(left)).collect::<Option<_>>()?;
    let corners: Vec<Option<Point>> = (0..n)
        .map(|i| {
            let tip = nearest_intersection(&shifted[i], &shifted[(i + 1) % n], Extent::Unbounded, source[i].end);
            tip.filter(|m| (*m - source[i].end).hypot() <= miter_limit * left.abs().max(LENGTH_EPS))
        })
        .collect();
    let mut out = Vec::new();
    for i in 0..n {
        let before = (i + n - 1) % n;
        let start = corners[before].unwrap_or(shifted[i].start);
        let end = corners[i].unwrap_or(shifted[i].end);
        let trimmed = shifted[i].retarget(start, end);
        out.push(Vertex { point: trimmed.start, bulge: trimmed.bulge });
        if corners[i].is_none() {
            out.push(Vertex { point: trimmed.end, bulge: 0.0 });
        }
    }
    let grows = distance >= 0.0;
    let (before, after) = (signed_area(vertices).abs(), signed_area(&out).abs());
    if (grows && after < before) || (!grows && after > before) || signed_area(&out).signum() != signed_area(vertices).signum() {
        return None;
    }
    Some(out)
}

/// ✂️ Self-intersection points of a loop (segment pairs that are not neighbours), for validity checks.
pub fn self_intersections(vertices: &[Vertex]) -> Vec<Point> {
    let segs = segments(vertices);
    let n = segs.len();
    let mut out = Vec::new();
    for i in 0..n {
        for j in i + 2..n {
            if i == 0 && j == n - 1 {
                continue;
            }
            for hit in intersect(&segs[i], &segs[j], Extent::Bounded) {
                out.push(hit.point);
            }
        }
    }
    out
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
